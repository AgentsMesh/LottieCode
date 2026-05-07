//! Precomp 层级实例 `precomp <name> { asset = <id>, ... }` → IrLayer。

use std::collections::HashMap;

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

use crate::semantic::resolve_animate::animate_to_value;
use crate::semantic::resolve_shape::transform_attrs::*;
use crate::semantic::resolve_token::*;

/// 把 `precomp <inst> { asset = ref, ... animate ... }` 转为 IrLayer。
pub fn resolve_precomp_layer(
    plyr: &PrecompLayerDecl,
    asset_map: &HashMap<String, IrAsset>,
    tokens: &TokenMap,
    out_frame: f64,
    fps: f64,
) -> Result<IrLayer> {
    let attrs = index_attrs(&plyr.attributes, tokens)?;

    let asset_name = resolve_asset_ref(&attrs, plyr.span)?;
    let (width, height) = lookup_precomp_dims(asset_map, &asset_name, plyr.span)?;

    let mut layer_tr = IrTransform::default();
    apply_transform_attrs(&mut layer_tr, &attrs)?;
    let transform_anims: Vec<AnimateDecl> = plyr
        .animations
        .iter()
        .filter(|a| a.property != "time-remap" && a.property != "time_remap")
        .cloned()
        .collect();
    apply_transform_animations(&mut layer_tr, &transform_anims, fps, tokens)?;

    let time_remap = parse_time_remap(&plyr.animations, fps, tokens)?;

    Ok(IrLayer {
        name: plyr.name.clone(),
        kind: IrLayerKind::Precomp {
            asset_id: asset_name,
            width,
            height,
            time_remap,
        },
        transform: layer_tr,
        in_frame: 0.0,
        start_time: 0.0,
        time_stretch: 1.0,
        out_frame,
        parent: None,
        masks: Vec::new(),
        blend_mode: crate::ir::BlendMode::Normal,
        matte_source: false,
        matte: None,
        effects: Vec::new(),
    })
}

fn resolve_asset_ref(attrs: &AttrIndex, span: crate::token::Span) -> Result<String> {
    let asset_ref = attrs.resolved("asset").ok_or_else(|| {
        LottieError::new(ErrorKind::InvalidValue, "precomp 层缺少 asset", Some(span))
    })?;
    match asset_ref {
        Expression::Ident(s, _) => Ok(s.clone()),
        Expression::Path(parts, _) => Ok(parts.join(".")),
        _ => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            "precomp.asset 需为标识符",
            Some(span),
        )),
    }
}

fn lookup_precomp_dims(
    asset_map: &HashMap<String, IrAsset>,
    name: &str,
    span: crate::token::Span,
) -> Result<(u32, u32)> {
    match asset_map.get(name) {
        Some(IrAsset::Precomp { width, height, .. }) => Ok((*width, *height)),
        Some(_) => Err(LottieError::new(
            ErrorKind::InvalidValue,
            format!("`{name}` 不是 precomp asset"),
            Some(span),
        )),
        None => Err(LottieError::new(
            ErrorKind::InvalidValue,
            format!("未定义的 precomp `{name}`"),
            Some(span),
        )),
    }
}

fn parse_time_remap(
    animations: &[AnimateDecl],
    fps: f64,
    tokens: &TokenMap,
) -> Result<Option<AnimatableValue<f64>>> {
    animations
        .iter()
        .find(|a| a.property == "time-remap" || a.property == "time_remap")
        .map(|anim| {
            animate_to_value(anim, fps, tokens, |expr| match expr {
                Expression::TimeSec(v, _) => Ok(*v),
                Expression::TimeMs(v, _) => Ok(*v / 1000.0),
                Expression::IntLit(n, _) => Ok(*n as f64),
                Expression::FloatLit(v, _) => Ok(*v),
                _ => Err(LottieError::new(
                    ErrorKind::TypeMismatch,
                    "time-remap 关键帧值需为时间或数字",
                    Some(expr.span()),
                )),
            })
        })
        .transpose()
}
