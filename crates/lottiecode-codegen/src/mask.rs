//! Mask codegen —— layer.masksProperties + hasMask。

use serde_json::{json, Value};

use lottiecode_lang::ir::{AnimatableValue, BezierPath, IrMask};

use crate::property::*;

pub fn emit_masks(masks: &[IrMask]) -> Vec<Value> {
    masks.iter().map(emit_one).collect()
}

fn emit_one(m: &IrMask) -> Value {
    json!({
        "mode": m.mode.as_str(),
        "inv": m.inverted,
        "pt": emit_path(&m.path),
        "o": emit_scalar(&m.opacity),
        "x": emit_scalar(&m.expand),
        "nm": "Mask",
    })
}

fn bezier_obj(b: &BezierPath) -> Value {
    json!({
        "i": b.in_tangents.iter().map(|p| p.to_vec()).collect::<Vec<_>>(),
        "o": b.out_tangents.iter().map(|p| p.to_vec()).collect::<Vec<_>>(),
        "v": b.vertices.iter().map(|p| p.to_vec()).collect::<Vec<_>>(),
        "c": b.closed,
    })
}

fn emit_path(path: &AnimatableValue<BezierPath>) -> Value {
    match path {
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
                keyframes.push(serde_json::Value::Object(obj));
            }
            json!({ "a": 1, "k": keyframes })
        }
        AnimatableValue::Expression { default, expr } => json!({
            "a": 0,
            "k": bezier_obj(default),
            "x": expr,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lottiecode_lang::ir::MaskMode;

    fn rect_path() -> BezierPath {
        BezierPath {
            vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]],
            in_tangents: vec![[0.0, 0.0]; 4],
            out_tangents: vec![[0.0, 0.0]; 4],
            closed: true,
        }
    }

    #[test]
    fn mask_emits_mode_string() {
        let m = IrMask {
            mode: MaskMode::Add,
            inverted: false,
            path: AnimatableValue::Static(rect_path()),
            opacity: AnimatableValue::Static(100.0),
            expand: AnimatableValue::Static(0.0),
        };
        let out = emit_masks(&[m]);
        assert_eq!(out[0]["mode"], "a");
        assert_eq!(out[0]["inv"], false);
    }

    #[test]
    fn mask_path_emits_bezier_obj() {
        let m = IrMask {
            mode: MaskMode::Subtract,
            inverted: true,
            path: AnimatableValue::Static(rect_path()),
            opacity: AnimatableValue::Static(80.0),
            expand: AnimatableValue::Static(2.0),
        };
        let out = emit_masks(&[m]);
        assert_eq!(out[0]["mode"], "s");
        assert_eq!(out[0]["inv"], true);
        assert_eq!(out[0]["pt"]["k"]["c"], true);
        assert_eq!(out[0]["pt"]["k"]["v"][0], serde_json::json!([0.0, 0.0]));
    }

    #[test]
    fn all_mask_modes_distinct() {
        use std::collections::HashSet;
        let modes = [
            MaskMode::None,
            MaskMode::Add,
            MaskMode::Subtract,
            MaskMode::Intersect,
            MaskMode::Lighten,
            MaskMode::Darken,
            MaskMode::Difference,
        ];
        let s: HashSet<&str> = modes.iter().map(|m| m.as_str()).collect();
        assert_eq!(s.len(), modes.len());
    }
}
