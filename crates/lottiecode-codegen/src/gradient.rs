//! 渐变填充 / 描边 codegen。

use serde_json::{json, Value};

use lottiecode_lang::ir::{AnimatableValue, GradientKind, GradientStop, LineCap, LineJoin};

use crate::property::*;

/// 把渐变停止点扁平化为 Lottie 数组：`[off1, r1, g1, b1, ..., offN, alphaN, ...]`。
pub fn flatten_stops(stops: &[GradientStop]) -> Vec<f64> {
    let mut out = Vec::new();
    for stop in stops {
        out.push(stop.offset);
        out.push(stop.color[0]);
        out.push(stop.color[1]);
        out.push(stop.color[2]);
    }
    for stop in stops {
        out.push(stop.offset);
        out.push(stop.color[3]);
    }
    out
}

pub fn emit_gradient_fill(
    kind: &GradientKind,
    start: &AnimatableValue<[f64; 2]>,
    end: &AnimatableValue<[f64; 2]>,
    stops: &[GradientStop],
    opacity: &AnimatableValue<f64>,
) -> Value {
    json!({
        "ty": "gf",
        "o": emit_scalar(opacity),
        "g": { "p": stops.len(), "k": { "a": 0, "k": flatten_stops(stops) } },
        "s": emit_xy(start),
        "e": emit_xy(end),
        "t": kind.as_lottie(),
        "r": 1,
        "bm": 0,
        "nm": "Gradient Fill",
        "hd": false,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn emit_gradient_stroke(
    kind: &GradientKind,
    start: &AnimatableValue<[f64; 2]>,
    end: &AnimatableValue<[f64; 2]>,
    stops: &[GradientStop],
    opacity: &AnimatableValue<f64>,
    width: &AnimatableValue<f64>,
    line_cap: LineCap,
    line_join: LineJoin,
    miter_limit: f64,
) -> Value {
    json!({
        "ty": "gs",
        "o": emit_scalar(opacity),
        "g": { "p": stops.len(), "k": { "a": 0, "k": flatten_stops(stops) } },
        "s": emit_xy(start),
        "e": emit_xy(end),
        "t": kind.as_lottie(),
        "w": emit_scalar(width),
        "lc": line_cap.as_lottie(),
        "lj": line_join.as_lottie(),
        "ml": miter_limit,
        "bm": 0,
        "nm": "Gradient Stroke",
        "hd": false,
    })
}
