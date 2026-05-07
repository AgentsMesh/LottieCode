//! 顶层 precomp 声明 → IrAsset::Precomp。

use std::collections::HashMap;

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

use crate::semantic::resolve_image::resolve_image_layer;
use crate::semantic::resolve_shape::resolve_shape;
use crate::semantic::resolve_solid_null;
use crate::semantic::resolve_text::resolve_text;
use crate::semantic::resolve_token::*;
use crate::semantic::util::*;
use crate::semantic::CompositionMeta;

use super::layer::resolve_precomp_layer;

pub fn parse_precomp_meta(decl: &PrecompDecl, tokens: &TokenMap) -> Result<CompositionMeta> {
    let mut width: Option<u32> = None;
    let mut height: Option<u32> = None;
    let mut fps: f64 = 30.0;
    let mut duration_sec: f64 = 1.0;

    for attr in &decl.attributes {
        let v = resolve_value(&attr.value, tokens)?;
        match attr.key.as_str() {
            "width" => width = Some(expect_uint(&v, attr.span)?),
            "height" => height = Some(expect_uint(&v, attr.span)?),
            "fps" => fps = expect_number(&v, attr.span)?,
            "duration" => duration_sec = expect_seconds(&v, attr.span, fps)?,
            _ => {
                return Err(LottieError::new(
                    ErrorKind::InvalidValue,
                    format!("precomp 不支持的属性 `{}`", attr.key),
                    Some(attr.span),
                ));
            }
        }
    }

    Ok(CompositionMeta {
        width: width.unwrap_or(100),
        height: height.unwrap_or(100),
        fps,
        out_frame: (duration_sec * fps).round(),
    })
}

/// 把顶层 `precomp` 编译为一个 IrAsset::Precomp。
pub fn resolve_precomp_decl(
    decl: &PrecompDecl,
    tokens: &TokenMap,
    component_map: &HashMap<String, ComponentDecl>,
    asset_map: &HashMap<String, IrAsset>,
    fonts: &mut Vec<IrFont>,
) -> Result<IrAsset> {
    let CompositionMeta { width, height, fps, out_frame } = parse_precomp_meta(decl, tokens)?;

    let mut layers = Vec::new();
    for item in &decl.items {
        if let CompositionItem::Use(usd) = item {
            let expanded = crate::semantic::resolve_component::instantiate(usd, component_map, tokens)?;
            for sub in expanded {
                if matches!(sub, CompositionItem::Use(_)) {
                    return Err(LottieError::new(
                        ErrorKind::InvalidValue,
                        "instantiate 返回了未展开的 use",
                        Some(usd.span),
                    ));
                }
                layers.push(resolve_inner(&sub, tokens, asset_map, fonts, fps, out_frame)?);
            }
        } else {
            layers.push(resolve_inner(item, tokens, asset_map, fonts, fps, out_frame)?);
        }
    }

    Ok(IrAsset::Precomp {
        id: decl.name.clone(),
        width,
        height,
        fps,
        layers,
    })
}

/// 解析单个非 Use 条目为一个 IrLayer。
fn resolve_inner(
    item: &CompositionItem,
    tokens: &TokenMap,
    asset_map: &HashMap<String, IrAsset>,
    fonts: &mut Vec<IrFont>,
    fps: f64,
    out_frame: f64,
) -> Result<IrLayer> {
    match item {
        CompositionItem::Shape(sh) => resolve_shape(sh, tokens, fps, out_frame),
        CompositionItem::Text(td) => resolve_text(td, tokens, fps, out_frame, fonts),
        CompositionItem::Image(img) => resolve_image_layer(img, asset_map, tokens, out_frame),
        CompositionItem::Precomp(plyr) => {
            resolve_precomp_layer(plyr, asset_map, tokens, out_frame, fps)
        }
        CompositionItem::Solid(s) => resolve_solid_null::resolve_solid(s, tokens, fps, out_frame),
        CompositionItem::Controller(c) => {
            resolve_solid_null::resolve_controller(c, tokens, fps, out_frame)
        }
        CompositionItem::Camera(cam) => {
            resolve_solid_null::resolve_camera(cam, tokens, fps, out_frame)
        }
        CompositionItem::Use(_) => unreachable!("Use items should be expanded by caller"),
    }
}
