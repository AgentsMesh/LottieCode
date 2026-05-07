//! IrGeometry → Lottie shape 对象（rc / el / sr / sh）。
//!
//! ## AE 字段顺序契约
//! - rc/el/sr：`ty, d, p, s, [r/...], nm, hd`
//! - sh（path）：`ty, d, ks, nm, hd`
//! - bezier ks.k：`i, o, v, c`（lottie-web 严格按此顺序解析路径数据，乱序会导致空 path）

use serde_json::{json, Value};

use lottiecode_lang::ir::{AnimatableValue, BezierPath, IrGeometry};

use crate::property::*;

pub fn emit_geometry(geo: &IrGeometry) -> Value {
    match geo {
        IrGeometry::Rectangle { position, size, radius, direction } => json!({
            "ty": "rc",
            "d": direction.as_lottie(),
            "p": emit_xy(position),
            "s": emit_xy(size),
            "r": emit_scalar(radius),
            "nm": "Rectangle",
            "hd": false,
        }),
        IrGeometry::Ellipse { position, size, direction } => json!({
            "ty": "el",
            "d": direction.as_lottie(),
            "p": emit_xy(position),
            "s": emit_xy(size),
            "nm": "Ellipse",
            "hd": false,
        }),
        IrGeometry::PolyStar {
            position, rotation, points, outer_radius, outer_roundness,
            inner_radius, inner_roundness, star_type, direction,
        } => json!({
            "ty": "sr",
            "d": direction.as_lottie(),
            "p": emit_xy(position),
            "r": emit_scalar(rotation),
            "pt": emit_scalar(points),
            "or": emit_scalar(outer_radius),
            "os": emit_scalar(outer_roundness),
            "ir": emit_scalar(inner_radius),
            "is": emit_scalar(inner_roundness),
            "sy": star_type.as_lottie(),
            "nm": "PolyStar",
            "hd": false,
        }),
        IrGeometry::Path { bezier, direction } => json!({
            "ty": "sh",
            "d": direction.as_lottie(),
            "ks": emit_bezier_value(bezier),
            "nm": "Path",
            "hd": false,
        }),
    }
}

fn emit_bezier_value(bezier: &AnimatableValue<BezierPath>) -> Value {
    match bezier {
        AnimatableValue::Static(b) => json!({ "a": 0, "k": bezier_obj(b) }),
        AnimatableValue::Animated(kfs) => {
            let mut keyframes = Vec::with_capacity(kfs.len());
            for (i, kf) in kfs.iter().enumerate() {
                let mut obj = serde_json::Map::new();
                if i + 1 < kfs.len() {
                    let c = kfs[i + 1].easing.as_cubic();
                    obj.insert("o".to_string(), json!({"x": [c[0]], "y": [c[1]]}));
                    obj.insert("i".to_string(), json!({"x": [c[2]], "y": [c[3]]}));
                }
                obj.insert("t".to_string(), json!(kf.frame));
                obj.insert("s".to_string(), json!([bezier_obj(&kf.value)]));
                keyframes.push(Value::Object(obj));
            }
            json!({ "a": 1, "k": keyframes })
        }
        AnimatableValue::Expression { default, expr } => json!({
            "a": 0, "k": bezier_obj(default), "x": expr,
        }),
    }
}

pub(crate) fn bezier_obj(b: &BezierPath) -> Value {
    json!({
        "i": b.in_tangents.iter().map(|p| p.to_vec()).collect::<Vec<_>>(),
        "o": b.out_tangents.iter().map(|p| p.to_vec()).collect::<Vec<_>>(),
        "v": b.vertices.iter().map(|p| p.to_vec()).collect::<Vec<_>>(),
        "c": b.closed,
    })
}
