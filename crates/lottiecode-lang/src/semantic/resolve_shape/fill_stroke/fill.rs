//! Fill 解析（颜色简写 + 对象形式）。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;
use crate::token::Span;

use crate::semantic::resolve_animate::{animate_to_value, conv_color, conv_scalar};
use crate::semantic::resolve_shape::gradient::build_gradient_fill;
use crate::semantic::resolve_token::TokenMap;
use crate::semantic::util::*;

use super::common::{extract_color, identifier_str};

pub fn build_fill(
    value: &Expression,
    span: Span,
    animations: &[AnimateDecl],
    fps: f64,
    tokens: &TokenMap,
) -> Result<IrStyle> {
    let color_anim = find_anim(animations, &["fill", "fill-color"]);
    let opacity_anim = find_anim(animations, &["fill-opacity"]);
    let static_color = match value {
        Expression::ColorLit(hex, sp) => Some(parse_color(hex, *sp)?),
        Expression::Object(attrs, _) => {
            if attrs.iter().any(|a| a.key == "gradient") {
                return build_gradient_fill(attrs, span);
            }
            return build_fill_object(attrs, span, color_anim, opacity_anim, fps, tokens);
        }
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "fill 必须是颜色字面量或对象",
                Some(span),
            ))
        }
    };

    let color_value = match color_anim {
        Some(anim) => animate_to_value(anim, fps, tokens, conv_color)?,
        None => s(static_color.unwrap_or([0.0, 0.0, 0.0, 1.0])),
    };
    let opacity_value = match opacity_anim {
        Some(anim) => animate_to_value(anim, fps, tokens, conv_scalar)?,
        None => s(100.0),
    };
    Ok(IrStyle::Fill {
        color: color_value,
        opacity: opacity_value,
        rule: FillRule::NonZero,
    })
}

fn build_fill_object(
    attrs: &[Attribute],
    span: Span,
    color_anim: Option<&AnimateDecl>,
    opacity_anim: Option<&AnimateDecl>,
    fps: f64,
    tokens: &TokenMap,
) -> Result<IrStyle> {
    let mut color: Option<[f64; 4]> = None;
    let mut opacity: f64 = 100.0;
    let mut rule = FillRule::NonZero;
    for a in attrs {
        match a.key.as_str() {
            "color" => color = Some(extract_color(&a.value)?),
            "opacity" => opacity = expect_number(&a.value, a.span)?,
            "rule" => {
                rule = match identifier_str(&a.value) {
                    Some("non-zero") => FillRule::NonZero,
                    Some("even-odd") => FillRule::EvenOdd,
                    _ => {
                        return Err(LottieError::new(
                            ErrorKind::InvalidValue,
                            "fill.rule 取值需为 non-zero 或 even-odd",
                            Some(a.span),
                        ))
                    }
                };
            }
            other => {
                return Err(LottieError::new(
                    ErrorKind::InvalidValue,
                    format!("fill 不支持的属性 `{other}`"),
                    Some(a.span),
                ));
            }
        }
    }
    let color_value = match color_anim {
        Some(anim) => animate_to_value(anim, fps, tokens, conv_color)?,
        None => {
            let c = color.ok_or_else(|| {
                LottieError::new(ErrorKind::InvalidValue, "fill 缺少 color", Some(span))
            })?;
            s(c)
        }
    };
    let opacity_value = match opacity_anim {
        Some(anim) => animate_to_value(anim, fps, tokens, conv_scalar)?,
        None => s(opacity),
    };
    Ok(IrStyle::Fill {
        color: color_value,
        opacity: opacity_value,
        rule,
    })
}

fn find_anim<'a>(animations: &'a [AnimateDecl], names: &[&str]) -> Option<&'a AnimateDecl> {
    animations.iter().find(|a| names.contains(&a.property.as_str()))
}
