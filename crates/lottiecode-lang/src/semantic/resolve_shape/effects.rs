//! Effect 解析（DropShadow / GaussianBlur）。

use crate::ast::Expression;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::IrEffect;

use super::transform_attrs::AttrIndex;
use crate::semantic::resolve_token::TokenMap;
use crate::semantic::util::*;

pub fn build_effects(attrs: &AttrIndex, _tokens: &TokenMap) -> Result<Vec<IrEffect>> {
    let mut out = Vec::new();
    if let Some(shadow) = attrs.resolved("shadow") {
        out.push(parse_drop_shadow(shadow)?);
    }
    if let Some(blur) = attrs.resolved("blur") {
        out.push(parse_blur(blur)?);
    }
    Ok(out)
}

fn parse_drop_shadow(expr: &Expression) -> Result<IrEffect> {
    match expr {
        Expression::Array(items, span) => parse_shadow_array(items, *span),
        Expression::Object(attrs, _) => parse_shadow_object(attrs),
        _ => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            "shadow 需为数组 `[dx, dy, blur, color]` 或对象",
            Some(expr.span()),
        )),
    }
}

fn parse_shadow_array(items: &[Expression], span: crate::token::Span) -> Result<IrEffect> {
    let mut nums = Vec::new();
    let mut color = [0.0, 0.0, 0.0, 1.0];
    for it in items {
        match it {
            Expression::IntLit(n, _) => nums.push(*n as f64),
            Expression::FloatLit(n, _) => nums.push(*n),
            Expression::ColorLit(hex, sp) => color = parse_color(hex, *sp)?,
            _ => {}
        }
    }
    if nums.len() < 3 {
        return Err(LottieError::new(
            ErrorKind::InvalidValue,
            "shadow 需为 [x, y, blur, color]",
            Some(span),
        ));
    }
    let (dx, dy, softness) = (nums[0], nums[1], nums[2]);
    let direction = dy.atan2(dx).to_degrees() + 90.0;
    let distance = (dx * dx + dy * dy).sqrt();
    Ok(IrEffect::DropShadow {
        color: crate::semantic::util::s(color),
        opacity: crate::semantic::util::s(color[3] * 255.0),
        direction: crate::semantic::util::s(direction),
        distance: crate::semantic::util::s(distance),
        softness: crate::semantic::util::s(softness),
    })
}

fn parse_shadow_object(attrs: &[crate::ast::Attribute]) -> Result<IrEffect> {
    let mut color = [0.0, 0.0, 0.0, 1.0];
    let mut opacity = 127.5;
    let mut direction = 135.0;
    let mut distance = 5.0;
    let mut softness = 5.0;
    for a in attrs {
        match a.key.as_str() {
            "color" => {
                if let Expression::ColorLit(hex, sp) = &a.value {
                    color = parse_color(hex, *sp)?;
                }
            }
            "opacity" => opacity = expect_number(&a.value, a.span)? * 2.55,
            "direction" | "angle" => direction = expect_number(&a.value, a.span)?,
            "distance" => distance = expect_number(&a.value, a.span)?,
            "softness" | "blur" => softness = expect_number(&a.value, a.span)?,
            _ => {}
        }
    }
    Ok(IrEffect::DropShadow {
        color: crate::semantic::util::s(color),
        opacity: crate::semantic::util::s(opacity),
        direction: crate::semantic::util::s(direction),
        distance: crate::semantic::util::s(distance),
        softness: crate::semantic::util::s(softness),
    })
}

fn parse_blur(expr: &Expression) -> Result<IrEffect> {
    let blurriness = match expr {
        Expression::IntLit(n, _) => *n as f64,
        Expression::FloatLit(n, _) => *n,
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "blur 需为数字",
                Some(expr.span()),
            ))
        }
    };
    Ok(IrEffect::GaussianBlur {
        blurriness: crate::semantic::util::s(blurriness),
        direction: crate::semantic::util::s(0.0),
        repeat_edge_pixels: true,
    })
}
