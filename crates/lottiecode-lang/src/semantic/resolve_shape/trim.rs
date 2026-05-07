//! TrimPath 解析（trim 块 + animate trim*）。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

use super::transform_attrs::index_attrs;
use crate::semantic::resolve_animate::{animate_to_value, conv_scalar};
use crate::semantic::resolve_token::*;
use crate::semantic::util::*;

pub fn build_trim(
    trim: &Option<TrimDecl>,
    animations: &[AnimateDecl],
    fps: f64,
    tokens: &TokenMap,
) -> Result<Option<IrPathModifier>> {
    let mut start: AnimatableValue<f64> = s(0.0);
    let mut end: AnimatableValue<f64> = s(100.0);
    let mut offset: AnimatableValue<f64> = s(0.0);
    let mut has_any = false;

    if let Some(td) = trim {
        let attrs = index_attrs(&td.attributes, tokens)?;
        if let Some(a) = attrs.find("start") {
            start = s(expect_number(&a.value, a.span)?);
            has_any = true;
        }
        if let Some(a) = attrs.find("end") {
            end = s(expect_number(&a.value, a.span)?);
            has_any = true;
        }
        if let Some(a) = attrs.find("offset") {
            offset = s(expect_number(&a.value, a.span)?);
            has_any = true;
        }
    }

    for anim in animations {
        match anim.property.as_str() {
            "trim-start" => {
                start = animate_to_value(anim, fps, tokens, conv_scalar)?;
                has_any = true;
            }
            "trim-end" => {
                end = animate_to_value(anim, fps, tokens, conv_scalar)?;
                has_any = true;
            }
            "trim-offset" => {
                offset = animate_to_value(anim, fps, tokens, conv_scalar)?;
                has_any = true;
            }
            "trim" => {
                expand_trim_combined(anim, fps, tokens, &mut start, &mut end, &mut offset)?;
                has_any = true;
            }
            _ => {}
        }
    }

    if !has_any {
        return Ok(None);
    }

    Ok(Some(IrPathModifier::TrimPath {
        start,
        end,
        offset,
        mode: TrimMode::Simultaneously,
    }))
}

/// `animate trim { time: { start=N, end=N, offset=N } }` 同步动画三字段。
fn expand_trim_combined(
    anim: &AnimateDecl,
    fps: f64,
    tokens: &TokenMap,
    start: &mut AnimatableValue<f64>,
    end: &mut AnimatableValue<f64>,
    offset: &mut AnimatableValue<f64>,
) -> Result<()> {
    let extract = |field: &str| -> Result<AnimatableValue<f64>> {
        animate_to_value(anim, fps, tokens, |expr| {
            let attrs = match expr {
                Expression::Object(a, _) => a,
                _ => {
                    return Err(LottieError::new(
                        ErrorKind::TypeMismatch,
                        "animate trim 的关键帧值需是对象 `{ start=N, end=N, offset=N }`",
                        Some(expr.span()),
                    ))
                }
            };
            for a in attrs {
                if a.key == field {
                    return expect_number(&a.value, a.span);
                }
            }
            Ok(match field {
                "start" => 0.0,
                "end" => 100.0,
                "offset" => 0.0,
                _ => 0.0,
            })
        })
    };

    *start = extract("start")?;
    *end = extract("end")?;
    *offset = extract("offset")?;
    Ok(())
}
