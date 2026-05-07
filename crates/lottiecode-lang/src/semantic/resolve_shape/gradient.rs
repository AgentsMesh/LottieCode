//! 渐变填充 / 描边解析 —— 提取自 fill_stroke.rs。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;
use crate::token::Span;

use crate::semantic::util::*;

pub(super) fn build_gradient_fill(attrs: &[Attribute], span: Span) -> Result<IrStyle> {
    let (kind, start, end, stops, opacity) = parse_gradient_common(attrs, span)?;
    Ok(IrStyle::GradientFill {
        kind,
        start: s(start),
        end: s(end),
        stops,
        opacity: s(opacity),
    })
}

pub(super) fn build_gradient_stroke(attrs: &[Attribute], span: Span) -> Result<IrStyle> {
    let (kind, start, end, stops, opacity) = parse_gradient_common(attrs, span)?;

    let mut width = 1.0;
    let mut line_cap = LineCap::Round;
    let mut line_join = LineJoin::Round;
    let mut miter_limit = 4.0;
    for a in attrs {
        match a.key.as_str() {
            "width" => width = expect_number(&a.value, a.span)?,
            "cap" => {
                line_cap = match super::fill_stroke::identifier_str(&a.value) {
                    Some("butt") => LineCap::Butt,
                    Some("round") => LineCap::Round,
                    Some("square") => LineCap::Square,
                    _ => LineCap::Round,
                }
            }
            "join" => {
                line_join = match super::fill_stroke::identifier_str(&a.value) {
                    Some("miter") => LineJoin::Miter,
                    Some("round") => LineJoin::Round,
                    Some("bevel") => LineJoin::Bevel,
                    _ => LineJoin::Round,
                }
            }
            "miter-limit" => miter_limit = expect_number(&a.value, a.span)?,
            _ => {}
        }
    }

    Ok(IrStyle::GradientStroke {
        kind,
        start: s(start),
        end: s(end),
        stops,
        opacity: s(opacity),
        width: s(width),
        line_cap,
        line_join,
        miter_limit,
    })
}

fn parse_gradient_common(
    attrs: &[Attribute],
    span: Span,
) -> Result<(GradientKind, [f64; 2], [f64; 2], Vec<GradientStop>, f64)> {
    let mut kind = GradientKind::Linear;
    let mut start = [0.0, 0.0];
    let mut end = [100.0, 0.0];
    let mut colors: Option<Vec<[f64; 4]>> = None;
    let mut opacity = 100.0;

    for a in attrs {
        match a.key.as_str() {
            "gradient" => {
                kind = match super::fill_stroke::identifier_str(&a.value) {
                    Some("linear") => GradientKind::Linear,
                    Some("radial") => GradientKind::Radial,
                    _ => {
                        return Err(LottieError::new(
                            ErrorKind::InvalidValue,
                            "gradient 取值需为 linear 或 radial",
                            Some(a.span),
                        ))
                    }
                }
            }
            "start" => start = expect_xy(&a.value, a.span)?,
            "end" => end = expect_xy(&a.value, a.span)?,
            "colors" => {
                let arr = match &a.value {
                    Expression::Array(items, _) => items,
                    _ => {
                        return Err(LottieError::new(
                            ErrorKind::TypeMismatch,
                            "gradient.colors 需为数组",
                            Some(a.span),
                        ))
                    }
                };
                colors = Some(
                    arr.iter()
                        .map(super::fill_stroke::extract_color)
                        .collect::<Result<Vec<_>>>()?,
                );
            }
            "opacity" => opacity = expect_number(&a.value, a.span)?,
            _ => {}
        }
    }

    let colors = colors.ok_or_else(|| {
        LottieError::new(ErrorKind::InvalidValue, "gradient 缺少 colors", Some(span))
    })?;
    if colors.len() < 2 {
        return Err(LottieError::new(
            ErrorKind::InvalidValue,
            "gradient.colors 至少需要 2 个颜色",
            Some(span),
        ));
    }

    let n = colors.len();
    let stops: Vec<GradientStop> = colors
        .into_iter()
        .enumerate()
        .map(|(i, c)| GradientStop {
            offset: i as f64 / (n - 1) as f64,
            color: c,
        })
        .collect();

    Ok((kind, start, end, stops, opacity))
}
