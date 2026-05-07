//! Mask 块解析。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

use super::path_parse::parse_svg_path;
use super::transform_attrs::{index_attrs, AttrIndex};
use crate::semantic::resolve_token::*;
use crate::semantic::util::*;

pub fn build_masks(masks: &[MaskDecl], tokens: &TokenMap, fps: f64) -> Result<Vec<IrMask>> {
    let mut out = Vec::with_capacity(masks.len());
    for m in masks {
        let attrs = index_attrs(&m.attributes, tokens)?;
        out.push(build_one(&attrs, m, fps, tokens)?);
    }
    Ok(out)
}

fn build_one(attrs: &AttrIndex, m: &MaskDecl, fps: f64, tokens: &TokenMap) -> Result<IrMask> {
    let span = m.span;
    let mode_str = attrs
        .resolved("mode")
        .and_then(|e| match e {
            Expression::Ident(s, _) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or("add");
    let mode = match mode_str {
        "none" => MaskMode::None,
        "add" => MaskMode::Add,
        "subtract" => MaskMode::Subtract,
        "intersect" => MaskMode::Intersect,
        "lighten" => MaskMode::Lighten,
        "darken" => MaskMode::Darken,
        "difference" => MaskMode::Difference,
        other => {
            return Err(LottieError::new(
                ErrorKind::InvalidValue,
                format!("mask.mode 不支持 `{other}`"),
                Some(span),
            ));
        }
    };

    let inverted = attrs
        .resolved("invert")
        .and_then(|e| match e {
            Expression::BoolLit(b, _) => Some(*b),
            _ => None,
        })
        .unwrap_or(false);

    let path_d = attrs
        .resolved("path")
        .ok_or_else(|| LottieError::new(ErrorKind::InvalidValue, "mask 缺少 path", Some(span)))?;
    let static_path = match path_d {
        Expression::StringLit(s, sp) => parse_svg_path(s, *sp)?,
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "mask.path 必须是字符串",
                Some(span),
            ))
        }
    };

    // 检查 mask 是否有 path 动画
    let path_value = if let Some(path_anim) = m.animations.iter().find(|a| a.property == "path") {
        crate::semantic::resolve_animate::animate_to_value(path_anim, fps, tokens, |expr| {
            match expr {
                Expression::StringLit(s, sp) => parse_svg_path(s, *sp),
                _ => Err(LottieError::new(
                    ErrorKind::TypeMismatch,
                    "mask path 动画值需为 SVG 路径字符串",
                    Some(expr.span()),
                )),
            }
        })?
    } else {
        AnimatableValue::Static(static_path)
    };

    let opacity = attrs
        .resolved("opacity")
        .map(|e| expect_number(e, span))
        .transpose()?
        .unwrap_or(100.0);

    let expand = attrs
        .resolved("expand")
        .map(|e| expect_number(e, span))
        .transpose()?
        .unwrap_or(0.0);

    Ok(IrMask {
        mode,
        inverted,
        path: path_value,
        opacity: AnimatableValue::Static(opacity),
        expand: AnimatableValue::Static(expand),
    })
}
