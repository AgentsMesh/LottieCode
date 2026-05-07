//! 顶层 `asset` 声明 → IrAsset。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

use crate::semantic::resolve_shape::transform_attrs::index_attrs;
use crate::semantic::resolve_token::TokenMap;
use crate::semantic::util::*;

use super::embed::embed_if_needed;

/// 资源声明分发。Precomp asset 由顶层 `precomp` 关键字处理，这里报错引导。
pub fn resolve_asset(decl: &AssetDecl, tokens: &TokenMap) -> Result<IrAsset> {
    match decl.kind {
        AssetKind::Image => resolve_image_asset(decl, tokens),
        AssetKind::Sound => resolve_sound_asset(decl, tokens),
        AssetKind::Data => resolve_data_asset(decl, tokens),
        AssetKind::Precomp => Err(LottieError::new(
            ErrorKind::InvalidValue,
            "Precomp asset 应通过 `precomp` 顶层关键字声明",
            Some(decl.span),
        )),
    }
}

fn resolve_image_asset(decl: &AssetDecl, tokens: &TokenMap) -> Result<IrAsset> {
    let attrs = index_attrs(&decl.attributes, tokens)?;
    let path = expect_string_attr(&attrs, "image", "asset.image", decl.span)?;
    let width = attrs
        .resolved("width")
        .map(|e| expect_uint(e, decl.span))
        .transpose()?
        .unwrap_or(100);
    let height = attrs
        .resolved("height")
        .map(|e| expect_uint(e, decl.span))
        .transpose()?
        .unwrap_or(100);
    let embed = is_embed(&attrs);
    let (final_path, embedded) = embed_if_needed(&path, embed, decl.span)?;
    Ok(IrAsset::Image {
        id: decl.name.clone(),
        width,
        height,
        path: final_path,
        embedded,
    })
}

fn resolve_sound_asset(decl: &AssetDecl, tokens: &TokenMap) -> Result<IrAsset> {
    let attrs = index_attrs(&decl.attributes, tokens)?;
    let path = match attrs.resolved("sound").or(attrs.resolved("audio")) {
        Some(Expression::StringLit(s, _)) => s.clone(),
        _ => {
            return Err(LottieError::new(
                ErrorKind::InvalidValue,
                "asset.sound 需为字符串路径",
                Some(decl.span),
            ))
        }
    };
    let embed = is_embed(&attrs);
    let (final_path, embedded) = embed_if_needed(&path, embed, decl.span)?;
    Ok(IrAsset::Sound {
        id: decl.name.clone(),
        path: final_path,
        embedded,
    })
}

fn resolve_data_asset(decl: &AssetDecl, tokens: &TokenMap) -> Result<IrAsset> {
    let attrs = index_attrs(&decl.attributes, tokens)?;
    let path = expect_string_attr(&attrs, "data", "asset.data", decl.span)?;
    let embed = is_embed(&attrs);
    let (final_path, embedded) = embed_if_needed(&path, embed, decl.span)?;
    Ok(IrAsset::Data {
        id: decl.name.clone(),
        path: final_path,
        embedded,
    })
}

fn expect_string_attr(
    attrs: &crate::semantic::resolve_shape::transform_attrs::AttrIndex,
    key: &str,
    label: &str,
    span: crate::token::Span,
) -> Result<String> {
    match attrs.resolved(key) {
        Some(Expression::StringLit(s, _)) => Ok(s.clone()),
        _ => Err(LottieError::new(
            ErrorKind::InvalidValue,
            format!("{label} 需为字符串路径"),
            Some(span),
        )),
    }
}

fn is_embed(attrs: &crate::semantic::resolve_shape::transform_attrs::AttrIndex) -> bool {
    matches!(attrs.resolved("embed"), Some(Expression::BoolLit(true, _)))
}
