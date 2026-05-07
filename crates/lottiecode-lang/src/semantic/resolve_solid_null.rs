//! Solid / Controller (Null) 层 → IrLayer。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

use super::resolve_shape::transform_attrs::*;
use super::resolve_shape::{blend_mode_id, parse_blend};
use super::resolve_token::*;
use super::util::*;

pub fn resolve_solid(solid: &SolidDecl, tokens: &TokenMap, fps: f64, out_frame: f64) -> Result<IrLayer> {
    let attrs = index_attrs(&solid.attributes, tokens)?;

    let color = attrs
        .resolved("color")
        .ok_or_else(|| {
            LottieError::new(ErrorKind::InvalidValue, "solid 缺少 color", Some(solid.span))
        })
        .and_then(|e| match e {
            Expression::ColorLit(hex, sp) => parse_color(hex, *sp),
            _ => Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "solid.color 需为颜色字面量",
                Some(solid.span),
            )),
        })?;

    let size = attrs
        .resolved("size")
        .map(|e| expect_xy(e, solid.span))
        .ok_or_else(|| {
            LottieError::new(ErrorKind::InvalidValue, "solid 缺少 size", Some(solid.span))
        })??;

    let mut layer_tr = IrTransform::default();
    apply_transform_attrs(&mut layer_tr, &attrs)?;
    apply_transform_animations(&mut layer_tr, &solid.animations, fps, tokens)?;

    let parent = parent_of(&attrs);
    let blend = parse_blend(&attrs)?;
    let _ = blend_mode_id;

    Ok(IrLayer {
        name: solid.name.clone(),
        kind: IrLayerKind::Solid {
            color,
            width: size[0] as u32,
            height: size[1] as u32,
        },
        transform: layer_tr,
        in_frame: 0.0,
        start_time: 0.0,
        time_stretch: 1.0,
        out_frame,
        parent,
        masks: Vec::new(),
        blend_mode: blend,
        matte_source: false,
        matte: None,
        effects: Vec::new(),
    })
}

pub fn resolve_controller(
    ctl: &ControllerDecl,
    tokens: &TokenMap,
    fps: f64,
    out_frame: f64,
) -> Result<IrLayer> {
    let attrs = index_attrs(&ctl.attributes, tokens)?;
    let mut layer_tr = IrTransform::default();
    apply_transform_attrs(&mut layer_tr, &attrs)?;
    apply_transform_animations(&mut layer_tr, &ctl.animations, fps, tokens)?;

    Ok(IrLayer {
        name: ctl.name.clone(),
        kind: IrLayerKind::Null,
        transform: layer_tr,
        in_frame: 0.0,
        start_time: 0.0,
        time_stretch: 1.0,
        out_frame,
        parent: parent_of(&attrs),
        masks: Vec::new(),
        blend_mode: crate::ir::BlendMode::Normal,
        matte_source: false,
        matte: None,
        effects: Vec::new(),
    })
}

fn parent_of(attrs: &AttrIndex) -> Option<String> {
    attrs.resolved("parent").and_then(|e| match e {
        Expression::Ident(s, _) => Some(s.clone()),
        _ => None,
    })
}

pub fn resolve_camera(
    cam: &CameraDecl,
    tokens: &TokenMap,
    fps: f64,
    out_frame: f64,
) -> Result<IrLayer> {
    let attrs = index_attrs(&cam.attributes, tokens)?;
    let perspective = attrs
        .resolved("perspective")
        .map(|e| expect_number(e, cam.span))
        .transpose()?
        .unwrap_or(800.0);

    let mut layer_tr = IrTransform::default();
    apply_transform_attrs(&mut layer_tr, &attrs)?;
    apply_transform_animations(&mut layer_tr, &cam.animations, fps, tokens)?;
    layer_tr.three_d = true;

    Ok(IrLayer {
        name: cam.name.clone(),
        kind: IrLayerKind::Camera {
            perspective: AnimatableValue::Static(perspective),
        },
        transform: layer_tr,
        in_frame: 0.0,
        start_time: 0.0,
        time_stretch: 1.0,
        out_frame,
        parent: parent_of(&attrs),
        masks: Vec::new(),
        blend_mode: crate::ir::BlendMode::Normal,
        matte_source: false,
        matte: None,
        effects: Vec::new(),
    })
}
