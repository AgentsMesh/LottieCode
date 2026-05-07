//! Effect codegen —— DropShadow / GaussianBlur 输出 ef 数组。

use serde_json::{json, Value};

use lottiecode_lang::ir::IrEffect;

use crate::property::{emit_color, emit_scalar};

pub fn emit_effects(effects: &[IrEffect]) -> Vec<Value> {
    effects
        .iter()
        .enumerate()
        .map(|(i, e)| emit_one(e, (i + 1) as i64))
        .collect()
}

fn emit_one(effect: &IrEffect, ix: i64) -> Value {
    match effect {
        IrEffect::DropShadow { color, opacity, direction, distance, softness } => json!({
            "ty": 25,
            "nm": "Drop Shadow",
            "ix": ix,
            "np": 7,
            "mn": "ADBE Drop Shadow",
            "ef": [
                { "ty": 2, "nm": "Shadow Color", "mn": "ADBE Drop Shadow-0001", "ix": 1, "v": emit_color(color) },
                { "ty": 0, "nm": "Opacity",      "mn": "ADBE Drop Shadow-0002", "ix": 2, "v": emit_scalar(opacity) },
                { "ty": 0, "nm": "Direction",    "mn": "ADBE Drop Shadow-0003", "ix": 3, "v": emit_scalar(direction) },
                { "ty": 0, "nm": "Distance",     "mn": "ADBE Drop Shadow-0004", "ix": 4, "v": emit_scalar(distance) },
                { "ty": 0, "nm": "Softness",     "mn": "ADBE Drop Shadow-0005", "ix": 5, "v": emit_scalar(softness) },
                { "ty": 7, "nm": "Shadow Only",  "mn": "ADBE Drop Shadow-0006", "ix": 6, "v": { "a": 0, "k": 0 } },
            ]
        }),
        IrEffect::GaussianBlur { blurriness, direction, repeat_edge_pixels } => json!({
            "ty": 29,
            "nm": "Gaussian Blur",
            "ix": ix,
            "np": 5,
            "mn": "ADBE Gaussian Blur 2",
            "ef": [
                { "ty": 0, "nm": "Blurriness",        "mn": "ADBE Gaussian Blur 2-0001", "ix": 1, "v": emit_scalar(blurriness) },
                { "ty": 7, "nm": "Blur Dimensions",   "mn": "ADBE Gaussian Blur 2-0002", "ix": 2, "v": emit_scalar(direction) },
                { "ty": 7, "nm": "Repeat Edge Pixels","mn": "ADBE Gaussian Blur 2-0003", "ix": 3, "v": { "a": 0, "k": if *repeat_edge_pixels {1} else {0} } },
            ]
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lottiecode_lang::ir::AnimatableValue;

    fn s<T>(v: T) -> AnimatableValue<T> {
        AnimatableValue::Static(v)
    }

    #[test]
    fn drop_shadow_outputs_ty_25() {
        let e = IrEffect::DropShadow {
            color: s([0.0, 0.0, 0.0, 1.0]),
            opacity: s(50.0),
            direction: s(135.0),
            distance: s(5.0),
            softness: s(3.0),
        };
        let out = emit_effects(&[e]);
        assert_eq!(out[0]["ty"], 25);
        assert_eq!(out[0]["nm"], "Drop Shadow");
        let params = out[0]["ef"].as_array().unwrap();
        assert_eq!(params.len(), 6);
    }

    #[test]
    fn gaussian_blur_outputs_ty_29() {
        let e = IrEffect::GaussianBlur {
            blurriness: s(8.0),
            direction: s(0.0),
            repeat_edge_pixels: true,
        };
        let out = emit_effects(&[e]);
        assert_eq!(out[0]["ty"], 29);
    }

    #[test]
    fn effect_indices_are_one_based() {
        let effects = vec![
            IrEffect::GaussianBlur {
                blurriness: s(1.0),
                direction: s(0.0),
                repeat_edge_pixels: false,
            },
            IrEffect::GaussianBlur {
                blurriness: s(2.0),
                direction: s(0.0),
                repeat_edge_pixels: false,
            },
        ];
        let out = emit_effects(&effects);
        assert_eq!(out[0]["ix"], 1);
        assert_eq!(out[1]["ix"], 2);
    }

    #[test]
    fn empty_effects_emit_empty_vec() {
        assert!(emit_effects(&[]).is_empty());
    }
}
