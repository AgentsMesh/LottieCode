//! 路径变形器：pucker / twist / zigzag。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;
use crate::semantic::util::*;
use crate::token::Span;

use super::identifier_str;

/// `pucker = N` 或 `pucker = { amount = N }`，amount 范围通常 -200..200。
pub fn build_pucker(value: &Expression, span: Span) -> Result<IrPathModifier> {
    let amount = match value {
        Expression::IntLit(n, _) => *n as f64,
        Expression::FloatLit(v, _) => *v,
        Expression::Object(attrs, _) => {
            let mut a = 0.0;
            for at in attrs {
                if at.key == "amount" {
                    a = expect_number(&at.value, at.span)?;
                }
            }
            a
        }
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "pucker 需为数字或对象",
                Some(span),
            ))
        }
    };
    Ok(IrPathModifier::PuckerBloat { amount: s(amount) })
}

pub fn build_twist(value: &Expression, span: Span) -> Result<IrPathModifier> {
    let (angle, center) = match value {
        Expression::IntLit(n, _) => (*n as f64, [0.0, 0.0]),
        Expression::FloatLit(v, _) => (*v, [0.0, 0.0]),
        Expression::Object(attrs, _) => {
            let mut angle = 0.0;
            let mut center = [0.0, 0.0];
            for a in attrs {
                match a.key.as_str() {
                    "angle" => angle = expect_number(&a.value, a.span)?,
                    "center" => center = expect_xy(&a.value, a.span)?,
                    _ => {}
                }
            }
            (angle, center)
        }
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "twist 需为数字或对象",
                Some(span),
            ))
        }
    };
    Ok(IrPathModifier::Twist {
        angle: s(angle),
        center: s(center),
    })
}

pub fn build_zigzag(value: &Expression, span: Span) -> Result<IrPathModifier> {
    let attrs = match value {
        Expression::Object(a, _) => a,
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "zigzag 必须是对象",
                Some(span),
            ))
        }
    };
    let mut amplitude = 5.0;
    let mut frequency = 8.0;
    let mut kind = ZigzagKind::Corner;
    for a in attrs {
        match a.key.as_str() {
            "amplitude" => amplitude = expect_number(&a.value, a.span)?,
            "frequency" => frequency = expect_number(&a.value, a.span)?,
            "type" => {
                kind = match identifier_str(&a.value) {
                    Some("corner") | Some("corners") => ZigzagKind::Corner,
                    Some("smooth") => ZigzagKind::Smooth,
                    _ => ZigzagKind::Corner,
                }
            }
            _ => {}
        }
    }
    Ok(IrPathModifier::Zigzag {
        amplitude: s(amplitude),
        frequency: s(frequency),
        kind,
    })
}
