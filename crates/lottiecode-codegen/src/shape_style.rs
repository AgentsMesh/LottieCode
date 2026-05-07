//! IrStyle → Lottie shape 对象（fl / st / gf / gs）。
//!
//! ## AE 字段顺序契约
//! - fl：`ty, c, o, r, bm, nm, hd`
//! - st：`ty, c, o, w, lc, lj, ml, [d (dash)], bm, nm, hd`

use serde_json::{json, Value};

use lottiecode_lang::ir::IrStyle;

use crate::gradient::{emit_gradient_fill, emit_gradient_stroke};
use crate::property::*;

pub fn emit_style(st: &IrStyle) -> Value {
    match st {
        IrStyle::Fill { color, opacity, rule } => json!({
            "ty": "fl",
            "c": emit_color(color),
            "o": emit_scalar(opacity),
            "r": rule.as_lottie(),
            "bm": 0,
            "nm": "Fill",
            "hd": false,
        }),
        IrStyle::Stroke {
            color, opacity, width, line_cap, line_join, miter_limit, dash, dash_offset,
        } => emit_stroke_value(color, opacity, width, *line_cap, *line_join, *miter_limit, dash, *dash_offset),
        IrStyle::GradientFill { kind, start, end, stops, opacity } => {
            emit_gradient_fill(kind, start, end, stops, opacity)
        }
        IrStyle::GradientStroke {
            kind, start, end, stops, opacity, width, line_cap, line_join, miter_limit,
        } => emit_gradient_stroke(
            kind, start, end, stops, opacity, width, *line_cap, *line_join, *miter_limit,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn emit_stroke_value(
    color: &lottiecode_lang::ir::AnimatableValue<[f64; 4]>,
    opacity: &lottiecode_lang::ir::AnimatableValue<f64>,
    width: &lottiecode_lang::ir::AnimatableValue<f64>,
    line_cap: lottiecode_lang::ir::LineCap,
    line_join: lottiecode_lang::ir::LineJoin,
    miter_limit: f64,
    dash: &[f64],
    dash_offset: f64,
) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert("ty".to_string(), json!("st"));
    obj.insert("c".to_string(), emit_color(color));
    obj.insert("o".to_string(), emit_scalar(opacity));
    obj.insert("w".to_string(), emit_scalar(width));
    obj.insert("lc".to_string(), json!(line_cap.as_lottie()));
    obj.insert("lj".to_string(), json!(line_join.as_lottie()));
    obj.insert("ml".to_string(), json!(miter_limit));
    obj.insert("bm".to_string(), json!(0));
    obj.insert("nm".to_string(), json!("Stroke"));
    obj.insert("hd".to_string(), json!(false));
    if !dash.is_empty() {
        let mut entries = vec![json!({
            "n": "o", "nm": "offset", "v": { "a": 0, "k": dash_offset }
        })];
        for (i, v) in dash.iter().enumerate() {
            let n = if i % 2 == 0 { "d" } else { "g" };
            let nm = if i % 2 == 0 { "dash" } else { "gap" };
            entries.push(json!({ "n": n, "nm": nm, "v": { "a": 0, "k": v } }));
        }
        obj.insert("d".to_string(), json!(entries));
    }
    Value::Object(obj)
}
