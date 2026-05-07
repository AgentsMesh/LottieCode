//! AnimateDecl → AnimatableValue<T>。
//!
//! ease 写在源帧（DSL 与 Web 动画直觉一致），与 Lottie keyframe 的 `i/o` 对齐。

use lottiecode_motion::easing as ease_lib;

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::{AnimatableValue, Easing, Keyframe};

use super::util::*;
use super::resolve_token::*;

pub fn animate_to_value<T, F>(
    anim: &AnimateDecl,
    fps: f64,
    tokens: &TokenMap,
    convert: F,
) -> Result<AnimatableValue<T>>
where
    T: Clone,
    F: Fn(&Expression) -> Result<T>,
{
    let mut keyframes = Vec::with_capacity(anim.keyframes.len());

    for kf in &anim.keyframes {
        let frame = expect_frames(&kf.time, kf.span, fps)?;
        let value_expr = resolve_value(&kf.value, tokens)?;
        let value = convert(&value_expr)?;
        let easing = match &kf.easing {
            None => Easing::Linear,
            Some(EasingExpr::Cubic(c)) => Easing::Cubic(*c),
            Some(EasingExpr::Named(name)) => match ease_lib::resolve(name) {
                Some(c) => Easing::Cubic(c),
                None => {
                    return Err(LottieError::new(
                        ErrorKind::UndefinedEasing,
                        format!("未知 easing `{name}`"),
                        Some(kf.span),
                    )
                    .with_hint("查看 lc syntax 输出的命名 easing 列表"))
                }
            },
        };

        let spatial_out = match &kf.spatial_to {
            Some(e) => Some(extract_spatial(e, kf.span)?),
            None => None,
        };
        let spatial_in = match &kf.spatial_ti {
            Some(e) => Some(extract_spatial(e, kf.span)?),
            None => None,
        };

        keyframes.push(Keyframe {
            frame,
            value,
            easing,
            hold: kf.hold,
            spatial_in,
            spatial_out,
        });
    }

    for w in keyframes.windows(2) {
        if w[1].frame < w[0].frame {
            return Err(LottieError::new(
                ErrorKind::KeyframeOutOfOrder,
                "关键帧时间必须严格递增",
                Some(anim.span),
            ));
        }
    }

    if keyframes.len() < 2 {
        return Err(LottieError::new(
            ErrorKind::InvalidValue,
            "动画至少需要 2 个关键帧",
            Some(anim.span),
        ));
    }

    Ok(AnimatableValue::Animated(keyframes))
}

pub fn conv_scalar(expr: &Expression) -> Result<f64> {
    expect_number(expr, expr.span())
}

#[allow(dead_code)]
pub fn conv_xy(expr: &Expression) -> Result<[f64; 2]> {
    expect_xy(expr, expr.span())
}

pub fn conv_xyz(expr: &Expression) -> Result<[f64; 3]> {
    expect_xyz(expr, expr.span())
}

pub fn conv_scale_xyz(expr: &Expression) -> Result<[f64; 3]> {
    scale_to_xyz(expr, expr.span())
}

#[allow(dead_code)]
pub fn conv_color(expr: &Expression) -> Result<[f64; 4]> {
    match expr {
        Expression::ColorLit(hex, span) => parse_color(hex, *span),
        _ => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            "期望颜色字面量（`#RRGGBB` 或 `#RRGGBBAA`）",
            Some(expr.span()),
        )),
    }
}

#[allow(dead_code)]
pub fn conv_scale_xy(expr: &Expression) -> Result<[f64; 2]> {
    if let Some(v) = expr.as_f64() {
        return Ok([v, v]);
    }
    expect_xy(expr, expr.span())
}

#[allow(dead_code)]
pub fn statify<T: Clone>(v: T) -> AnimatableValue<T> {
    s(v)
}

fn extract_spatial(expr: &Expression, span: crate::token::Span) -> Result<[f64; 3]> {
    let arr = expr.as_f64_array().ok_or_else(|| {
        LottieError::new(ErrorKind::TypeMismatch, "to/ti 需为数字数组", Some(span))
    })?;
    Ok(match arr.len() {
        2 => [arr[0], arr[1], 0.0],
        3 => [arr[0], arr[1], arr[2]],
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "to/ti 需为 2 或 3 元素数组",
                Some(span),
            ))
        }
    })
}


#[cfg(test)]
#[path = "resolve_animate_tests.rs"]
mod tests;
