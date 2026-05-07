//! Stroke 解析（描边 + dash + miter）。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;
use crate::token::Span;

use crate::semantic::resolve_animate::{animate_to_value, conv_color, conv_scalar};
use crate::semantic::resolve_shape::gradient::build_gradient_stroke;
use crate::semantic::resolve_token::TokenMap;
use crate::semantic::util::*;

use super::common::{extract_color, identifier_str};

pub fn build_stroke(
    value: &Expression,
    span: Span,
    animations: &[AnimateDecl],
    fps: f64,
    tokens: &TokenMap,
) -> Result<IrStyle> {
    let attrs = match value {
        Expression::Object(a, _) => a,
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "stroke 必须是对象 `{ color=..., width=..., cap=..., join=... }`",
                Some(span),
            ))
        }
    };

    if attrs.iter().any(|a| a.key == "gradient") {
        return build_gradient_stroke(attrs, span);
    }

    let mut state = StrokeBuilder::default();
    for a in attrs {
        state.set(a)?;
    }
    let color_anim = animations.iter().find(|a| a.property == "stroke-color" || a.property == "stroke");
    let opacity_anim = animations.iter().find(|a| a.property == "stroke-opacity");
    let width_anim = animations.iter().find(|a| a.property == "stroke-width");
    state.build(span, color_anim, opacity_anim, width_anim, fps, tokens)
}

struct StrokeBuilder {
    color: Option<[f64; 4]>,
    opacity: f64,
    width: f64,
    line_cap: LineCap,
    line_join: LineJoin,
    miter_limit: f64,
    dash: Vec<f64>,
    dash_offset: f64,
}

impl Default for StrokeBuilder {
    fn default() -> Self {
        Self {
            color: None,
            opacity: 100.0,
            width: 1.0,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            miter_limit: 4.0,
            dash: Vec::new(),
            dash_offset: 0.0,
        }
    }
}

impl StrokeBuilder {
    fn set(&mut self, a: &Attribute) -> Result<()> {
        match a.key.as_str() {
            "color" => self.color = Some(extract_color(&a.value)?),
            "opacity" => self.opacity = expect_number(&a.value, a.span)?,
            "width" => self.width = expect_number(&a.value, a.span)?,
            "cap" => {
                self.line_cap = match identifier_str(&a.value) {
                    Some("butt") => LineCap::Butt,
                    Some("round") => LineCap::Round,
                    Some("square") => LineCap::Square,
                    _ => LineCap::Round,
                }
            }
            "join" => {
                self.line_join = match identifier_str(&a.value) {
                    Some("miter") => LineJoin::Miter,
                    Some("round") => LineJoin::Round,
                    Some("bevel") => LineJoin::Bevel,
                    _ => LineJoin::Round,
                }
            }
            "miter-limit" => self.miter_limit = expect_number(&a.value, a.span)?,
            "dash" => {
                self.dash = a.value.as_f64_array().ok_or_else(|| {
                    LottieError::new(
                        ErrorKind::TypeMismatch,
                        "stroke.dash 需为数字数组",
                        Some(a.span),
                    )
                })?;
            }
            "dash-offset" => self.dash_offset = expect_number(&a.value, a.span)?,
            other => {
                return Err(LottieError::new(
                    ErrorKind::InvalidValue,
                    format!("stroke 不支持的属性 `{other}`"),
                    Some(a.span),
                ));
            }
        }
        Ok(())
    }

    fn build(
        self,
        span: Span,
        color_anim: Option<&AnimateDecl>,
        opacity_anim: Option<&AnimateDecl>,
        width_anim: Option<&AnimateDecl>,
        fps: f64,
        tokens: &TokenMap,
    ) -> Result<IrStyle> {
        let color_value = match color_anim {
            Some(anim) => animate_to_value(anim, fps, tokens, conv_color)?,
            None => {
                let c = self.color.ok_or_else(|| {
                    LottieError::new(ErrorKind::InvalidValue, "stroke 缺少 color", Some(span))
                })?;
                s(c)
            }
        };
        let opacity_value = match opacity_anim {
            Some(anim) => animate_to_value(anim, fps, tokens, conv_scalar)?,
            None => s(self.opacity),
        };
        let width_value = match width_anim {
            Some(anim) => animate_to_value(anim, fps, tokens, conv_scalar)?,
            None => s(self.width),
        };
        Ok(IrStyle::Stroke {
            color: color_value,
            opacity: opacity_value,
            width: width_value,
            line_cap: self.line_cap,
            line_join: self.line_join,
            miter_limit: self.miter_limit,
            dash: self.dash,
            dash_offset: self.dash_offset,
        })
    }
}
