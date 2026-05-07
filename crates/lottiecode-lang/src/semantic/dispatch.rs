//! Composition 内 items 调度 —— 把每种 CompositionItem 转 IrLayer。

use std::collections::HashMap;

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

use super::resolve_component::instantiate;
use super::resolve_image::resolve_image_layer;
use super::resolve_precomp::resolve_precomp_layer;
use super::resolve_shape::resolve_shape;
use super::resolve_solid_null::{resolve_camera, resolve_controller, resolve_solid};
use super::resolve_text::resolve_text;
use super::resolve_token::TokenMap;

/// 把 CompositionItem 列表转为 IrLayer 列表。
pub(crate) fn dispatch_items(
    items: &[CompositionItem],
    component_map: &HashMap<String, ComponentDecl>,
    asset_map: &HashMap<String, IrAsset>,
    token_map: &TokenMap,
    fps: f64,
    out_frame: f64,
    fonts: &mut Vec<IrFont>,
) -> Result<Vec<IrLayer>> {
    let mut layers = Vec::new();
    for item in items {
        dispatch_one(
            item,
            component_map,
            asset_map,
            token_map,
            fps,
            out_frame,
            fonts,
            &mut layers,
        )?;
    }
    Ok(layers)
}

fn dispatch_one(
    item: &CompositionItem,
    component_map: &HashMap<String, ComponentDecl>,
    asset_map: &HashMap<String, IrAsset>,
    token_map: &TokenMap,
    fps: f64,
    out_frame: f64,
    fonts: &mut Vec<IrFont>,
    layers: &mut Vec<IrLayer>,
) -> Result<()> {
    match item {
        CompositionItem::Shape(sh) => {
            layers.push(resolve_shape(sh, token_map, fps, out_frame)?);
        }
        CompositionItem::Text(td) => {
            layers.push(resolve_text(td, token_map, fps, out_frame, fonts)?);
        }
        CompositionItem::Image(img) => {
            layers.push(resolve_image_layer(img, asset_map, token_map, out_frame)?);
        }
        CompositionItem::Precomp(plyr) => {
            layers.push(resolve_precomp_layer(plyr, asset_map, token_map, out_frame, fps)?);
        }
        CompositionItem::Solid(s) => {
            layers.push(resolve_solid(s, token_map, fps, out_frame)?);
        }
        CompositionItem::Controller(c) => {
            layers.push(resolve_controller(c, token_map, fps, out_frame)?);
        }
        CompositionItem::Camera(cam) => {
            layers.push(resolve_camera(cam, token_map, fps, out_frame)?);
        }
        CompositionItem::Use(usd) => {
            let expanded = instantiate(usd, component_map, token_map)?;
            for sub in expanded {
                if matches!(sub, CompositionItem::Use(_)) {
                    return Err(LottieError::new(
                        ErrorKind::InvalidValue,
                        "instantiate 返回了未展开的 use",
                        Some(usd.span),
                    ));
                }
                dispatch_one(
                    &sub,
                    component_map,
                    asset_map,
                    token_map,
                    fps,
                    out_frame,
                    fonts,
                    layers,
                )?;
            }
        }
    }
    Ok(())
}
