//! Image Layer + Asset 解析。
//!
//! - layer：`image foo { asset = id, position = ... }` → IrLayer
//! - asset：顶层 `asset img { image = "...", embed = true }` → IrAsset
//! - embed：本地文件读 → base64 内联

mod asset;
mod embed;

use std::collections::HashMap;

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

use super::resolve_shape::transform_attrs::*;
use super::resolve_token::*;

pub use asset::resolve_asset;

pub fn resolve_image_layer(
    image: &ImageLayerDecl,
    asset_map: &HashMap<String, IrAsset>,
    tokens: &TokenMap,
    out_frame: f64,
) -> Result<IrLayer> {
    let attrs = index_attrs(&image.attributes, tokens)?;
    let asset_name = resolve_asset_ref(&attrs, image.span)?;

    if !asset_map.contains_key(&asset_name) {
        return Err(LottieError::new(
            ErrorKind::InvalidValue,
            format!("未定义的 asset `{asset_name}`"),
            Some(image.span),
        ));
    }

    let mut layer_tr = IrTransform::default();
    apply_transform_attrs(&mut layer_tr, &attrs)?;

    Ok(IrLayer {
        name: image.name.clone(),
        kind: IrLayerKind::Image { asset_id: asset_name },
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
        LottieError::new(ErrorKind::InvalidValue, "image 层缺少 asset", Some(span))
    })?;
    match asset_ref {
        Expression::Ident(s, _) => Ok(s.clone()),
        Expression::Path(parts, _) => Ok(parts.join(".")),
        _ => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            "image.asset 需为标识符",
            Some(span),
        )),
    }
}
