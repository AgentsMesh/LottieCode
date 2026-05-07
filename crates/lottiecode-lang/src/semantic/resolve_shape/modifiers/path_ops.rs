//! 形状路径操作：repeater / merge / offset-path。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;
use crate::semantic::util::*;
use crate::token::Span;

use super::identifier_str;

/// `repeater = { copies = N, offset = N, transform = { p=..., r=..., s=... } }`
pub fn build_repeater(value: &Expression, span: Span) -> Result<IrRepeater> {
    let attrs = expect_object(value, span, "repeater 必须是对象")?;
    let mut copies = 1.0;
    let mut offset = 0.0;
    let mut composite = CompositeOrder::Above;
    let mut transform = IrShapeTransform::default();

    for a in attrs {
        match a.key.as_str() {
            "copies" => copies = expect_number(&a.value, a.span)?,
            "offset" => offset = expect_number(&a.value, a.span)?,
            "composite" => {
                composite = match identifier_str(&a.value) {
                    Some("below") => CompositeOrder::Below,
                    Some("above") => CompositeOrder::Above,
                    _ => CompositeOrder::Above,
                }
            }
            "position" => transform.position = s(expect_xy(&a.value, a.span)?),
            "rotation" => transform.rotation = s(expect_number(&a.value, a.span)?),
            "scale" => {
                if let Some(n) = a.value.as_f64() {
                    transform.scale = s([n, n]);
                } else {
                    transform.scale = s(expect_xy(&a.value, a.span)?);
                }
            }
            "opacity-start" | "opacity-end" => {
                let _ = expect_number(&a.value, a.span)?;
            }
            _ => {
                return Err(LottieError::new(
                    ErrorKind::InvalidValue,
                    format!("repeater 不支持的属性 `{}`", a.key),
                    Some(a.span),
                ))
            }
        }
    }
    Ok(IrRepeater {
        copies: s(copies),
        offset: s(offset),
        composite,
        transform,
    })
}

/// `merge = add|subtract|intersect|exclude` 或 `merge = { mode = ... }`
pub fn build_merge(value: &Expression, span: Span) -> Result<IrPathModifier> {
    let mode_str = match value {
        Expression::Ident(s, _) => s.as_str(),
        Expression::Object(attrs, _) => {
            let mode_attr = attrs.iter().find(|a| a.key == "mode").ok_or_else(|| {
                LottieError::new(ErrorKind::InvalidValue, "merge 缺少 mode", Some(span))
            })?;
            match &mode_attr.value {
                Expression::Ident(s, _) => s.as_str(),
                _ => {
                    return Err(LottieError::new(
                        ErrorKind::TypeMismatch,
                        "merge.mode 需为标识符",
                        Some(mode_attr.span),
                    ))
                }
            }
        }
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "merge 必须是标识符或对象",
                Some(span),
            ))
        }
    };
    let mode = match mode_str {
        "merge" => MergeMode::Merge,
        "add" => MergeMode::Add,
        "subtract" => MergeMode::Subtract,
        "intersect" => MergeMode::Intersect,
        "exclude" => MergeMode::Exclude,
        other => {
            return Err(LottieError::new(
                ErrorKind::InvalidValue,
                format!("merge.mode 不支持 `{other}`"),
                Some(span),
            )
            .with_hint("可选：merge / add / subtract / intersect / exclude"))
        }
    };
    Ok(IrPathModifier::Merge { mode })
}

/// `offset-path = { amount = N, join = miter|round|bevel, miter-limit = N }`
pub fn build_offset_path(value: &Expression, span: Span) -> Result<IrPathModifier> {
    let attrs = expect_object(value, span, "offset-path 必须是对象")?;
    let mut amount = 0.0;
    let mut line_join = LineJoin::Miter;
    let mut miter_limit = 4.0;
    for a in attrs {
        match a.key.as_str() {
            "amount" => amount = expect_number(&a.value, a.span)?,
            "join" => {
                line_join = match identifier_str(&a.value) {
                    Some("miter") => LineJoin::Miter,
                    Some("round") => LineJoin::Round,
                    Some("bevel") => LineJoin::Bevel,
                    _ => LineJoin::Miter,
                }
            }
            "miter-limit" => miter_limit = expect_number(&a.value, a.span)?,
            _ => {
                return Err(LottieError::new(
                    ErrorKind::InvalidValue,
                    format!("offset-path 不支持的属性 `{}`", a.key),
                    Some(a.span),
                ))
            }
        }
    }
    Ok(IrPathModifier::OffsetPath {
        amount: s(amount),
        line_join,
        miter_limit: s(miter_limit),
    })
}

fn expect_object<'a>(value: &'a Expression, span: Span, msg: &str) -> Result<&'a [Attribute]> {
    match value {
        Expression::Object(a, _) => Ok(a),
        _ => Err(LottieError::new(ErrorKind::TypeMismatch, msg, Some(span))),
    }
}
