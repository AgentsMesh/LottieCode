//! IrPathModifier → Lottie shape 对象（tm / rd / mm / op / pb / tw / zz）。
//!
//! ## AE 字段顺序契约（每种 modifier 通用）
//! `ty, <type-specific fields>, nm, hd`

use serde_json::{json, Value};

use lottiecode_lang::ir::IrPathModifier;

use crate::property::*;

pub fn emit_modifier(m: &IrPathModifier) -> Value {
    match m {
        IrPathModifier::TrimPath { start, end, offset, mode } => json!({
            "ty": "tm",
            "s": emit_scalar(start),
            "e": emit_scalar(end),
            "o": emit_scalar(offset),
            "m": mode.as_lottie(),
            "nm": "Trim Paths",
            "hd": false,
        }),
        IrPathModifier::RoundedCorners { radius } => json!({
            "ty": "rd",
            "r": emit_scalar(radius),
            "nm": "Rounded Corners",
            "hd": false,
        }),
        IrPathModifier::Merge { mode } => json!({
            "ty": "mm",
            "mm": mode.as_lottie(),
            "nm": "Merge Paths",
            "hd": false,
        }),
        IrPathModifier::OffsetPath { amount, line_join, miter_limit } => json!({
            "ty": "op",
            "a": emit_scalar(amount),
            "lj": line_join.as_lottie(),
            "ml": emit_scalar(miter_limit),
            "nm": "Offset Path",
            "hd": false,
        }),
        IrPathModifier::PuckerBloat { amount } => json!({
            "ty": "pb",
            "a": emit_scalar(amount),
            "nm": "Pucker - Bloat",
            "hd": false,
        }),
        IrPathModifier::Twist { angle, center } => json!({
            "ty": "tw",
            "a": emit_scalar(angle),
            "c": emit_xy(center),
            "nm": "Twist",
            "hd": false,
        }),
        IrPathModifier::Zigzag { amplitude, frequency, kind } => json!({
            "ty": "zz",
            "s": emit_scalar(amplitude),
            "r": emit_scalar(frequency),
            "pt": kind.as_lottie(),
            "nm": "Zig Zag",
            "hd": false,
        }),
    }
}
