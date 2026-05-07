//! Shape AST → IrLayer 主流程。

mod effects;
mod fill_stroke;
mod geometry;
mod gradient;
mod layer_meta;
mod masks;
mod modifiers;
mod path_parse;
mod geometry_path;
mod sub_group;
pub(crate) mod transform_attrs;
mod trim;

use crate::ast::*;
use crate::error::Result;
use crate::ir::*;

use super::resolve_token::*;
use super::util::*;
use effects::build_effects;
use fill_stroke::*;
use geometry::resolve_geometry;
use layer_meta::*;
use masks::build_masks;
use modifiers::*;
use path_parse::parse_svg_path;
use sub_group::build_sub_group;
use transform_attrs::*;
use trim::build_trim;

pub use layer_meta::{blend_mode_id, parse_blend};

pub fn resolve_shape(
    shape: &ShapeDecl,
    tokens: &TokenMap,
    fps: f64,
    out_frame: f64,
) -> Result<IrLayer> {
    let attrs = index_attrs(&shape.attributes, tokens)?;

    let mut layer_tr = IrTransform::default();
    apply_transform_attrs(&mut layer_tr, &attrs)?;
    apply_transform_animations(&mut layer_tr, &shape.animations, fps, tokens)?;

    let in_frame = parse_layer_time(&attrs, "in", fps)?
        .or(parse_layer_time(&attrs, "start", fps)?)
        .unwrap_or(0.0);
    let layer_out = parse_layer_time(&attrs, "out", fps)?
        .or(parse_layer_time(&attrs, "end", fps)?)
        .unwrap_or(out_frame);
    let start_time = parse_layer_time(&attrs, "delay", fps)?.unwrap_or(0.0);
    let time_stretch = attrs
        .find("speed")
        .map(|a| expect_number(&a.value, a.span))
        .transpose()?
        .map(|v| if v.abs() < 1e-9 { 1.0 } else { 1.0 / v })
        .unwrap_or(1.0);

    let main = build_main_group(shape, &attrs, tokens, fps)?;
    let layer_trim = build_trim(&shape.trim, &shape.animations, fps, tokens)?;

    let mut groups: Vec<IrGroup> = Vec::new();
    if !main.is_empty() {
        groups.push(IrGroup {
            name: shape.name.clone(),
            geometries: main.geometries,
            modifiers: main.modifiers,
            styles: main.styles,
            repeater: main.repeater,
            transform: IrShapeTransform::default(),
        });
    }
    for sub in &shape.groups {
        groups.push(build_sub_group(sub, tokens)?);
    }

    // AE 约定：layer.anchor 自动同步到 group.tr.position（仅当 group 未显式指定 position）。
    crate::semantic::anchor_promote::promote_layer_anchor_to_groups(&mut groups, &layer_tr.anchor);

    Ok(IrLayer {
        name: shape.name.clone(),
        kind: IrLayerKind::Shape(IrShapeLayer { groups, layer_trim }),
        transform: layer_tr,
        in_frame,
        out_frame: layer_out,
        start_time,
        time_stretch,
        parent: parse_parent(&attrs),
        masks: build_masks(&shape.masks, tokens, fps)?,
        blend_mode: parse_blend(&attrs)?,
        matte_source: parse_matte_source(&attrs),
        matte: parse_matte(&attrs)?,
        effects: build_effects(&attrs, tokens)?,
    })
}

fn parse_layer_time(attrs: &AttrIndex, key: &str, fps: f64) -> Result<Option<f64>> {
    match attrs.find(key) {
        Some(a) => Ok(Some(expect_frames(&a.value, a.span, fps)?)),
        None => Ok(None),
    }
}

/// 主 group 中间结果（按职责分类，渲染顺序由字段位置保证）。
struct MainGroup {
    geometries: Vec<IrGeometry>,
    modifiers: Vec<IrPathModifier>,
    styles: Vec<IrStyle>,
    repeater: Option<IrRepeater>,
}

impl MainGroup {
    fn is_empty(&self) -> bool {
        self.geometries.is_empty()
            && self.modifiers.is_empty()
            && self.styles.is_empty()
            && self.repeater.is_none()
    }
}

/// 把 shape 顶层（隐式主 group）的 geometry / modifier / style / repeater 解析出来。
fn build_main_group(
    shape: &ShapeDecl,
    attrs: &AttrIndex,
    tokens: &TokenMap,
    fps: f64,
) -> Result<MainGroup> {
    let mut geometries: Vec<IrGeometry> = Vec::new();
    for geo in &shape.geometries {
        geometries.push(resolve_geometry(geo, tokens)?);
    }
    apply_path_morph(&mut geometries, shape, fps, tokens)?;

    let mut modifiers: Vec<IrPathModifier> = Vec::new();
    if let Some(a) = attrs.find("merge") {
        modifiers.push(build_merge(attrs.resolved("merge").unwrap(), a.span)?);
    }
    if let Some(a) = attrs.find("offset-path") {
        modifiers.push(build_offset_path(attrs.resolved("offset-path").unwrap(), a.span)?);
    }
    if let Some(a) = attrs.find("pucker") {
        modifiers.push(build_pucker(attrs.resolved("pucker").unwrap(), a.span)?);
    }
    if let Some(a) = attrs.find("twist") {
        modifiers.push(build_twist(attrs.resolved("twist").unwrap(), a.span)?);
    }
    if let Some(a) = attrs.find("zigzag") {
        modifiers.push(build_zigzag(attrs.resolved("zigzag").unwrap(), a.span)?);
    }
    if let Some(rd) = attrs.find("rounded-corners") {
        let radius = expect_number(attrs.resolved("rounded-corners").unwrap(), rd.span)?;
        modifiers.push(IrPathModifier::RoundedCorners { radius: s(radius) });
    }

    let mut styles: Vec<IrStyle> = Vec::new();
    if let Some(stroke) = attrs.find("stroke") {
        let resolved = attrs.resolved("stroke").unwrap();
        styles.push(build_stroke(resolved, stroke.span, &shape.animations, fps, tokens)?);
    }
    if let Some(fill) = attrs.find("fill") {
        let resolved = attrs.resolved("fill").unwrap();
        styles.push(build_fill(resolved, fill.span, &shape.animations, fps, tokens)?);
    }

    let repeater = if let Some(a) = attrs.find("repeater") {
        Some(build_repeater(attrs.resolved("repeater").unwrap(), a.span)?)
    } else {
        None
    };

    Ok(MainGroup { geometries, modifiers, styles, repeater })
}

fn apply_path_morph(
    geometries: &mut [IrGeometry],
    shape: &ShapeDecl,
    fps: f64,
    tokens: &TokenMap,
) -> Result<()> {
    let Some(anim) = shape.animations.iter().find(|a| a.property == "path") else {
        return Ok(());
    };
    let new_bezier = crate::semantic::resolve_animate::animate_to_value(anim, fps, tokens, |expr| {
        match expr {
            Expression::StringLit(s, sp) => parse_svg_path(s, *sp),
            _ => Err(crate::error::LottieError::new(
                crate::error::ErrorKind::TypeMismatch,
                "path 动画值需为 SVG 路径字符串",
                Some(expr.span()),
            )),
        }
    })?;
    for g in geometries.iter_mut() {
        if let IrGeometry::Path { bezier, .. } = g {
            *bezier = new_bezier.clone();
            break;
        }
    }
    Ok(())
}
