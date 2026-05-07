//! Text Layer codegen —— ty=5 + t.d.k 字段 + typewriter 动画器。

use serde_json::{json, Value};

use lottiecode_lang::ir::{AnimatableValue, IrTextData, IrTextDocument};

use crate::property::*;

pub fn emit_text_data(text: &IrTextData) -> Value {
    let document = match &text.document {
        AnimatableValue::Static(d) => d,
        AnimatableValue::Animated(_) | AnimatableValue::Expression { .. } => {
            unimplemented!("文本内容动画化暂不支持")
        }
    };

    // Typewriter animator：用 character-based ramp-up selector + opacity prop
    let animators = match &text.typewriter {
        Some(amount_anim) => vec![json!({
            "s": {
                "t": 0,
                "xe": { "a": 0, "k": 0 },
                "ne": { "a": 0, "k": 0 },
                // amount 从 keyframes 来：值 0 = 0% 字符显示，100 = 全部显示
                "a": emit_scalar(amount_anim),
                "b": 1,        // based on character
                "rn": 0,
                "sh": 1,       // ramp up
                "o": { "a": 0, "k": 0 },
                "r": 1,
            },
            "a": {
                // 选中字符 opacity 从 100 减到 0（即"未到达"的字符不可见）
                "o": { "a": 0, "k": -100 }
            },
            "nm": "Typewriter",
        })],
        None => vec![],
    };

    json!({
        "d": {
            "k": [{
                "t": 0,
                "s": emit_text_document(document),
            }]
        },
        "p": {},
        "m": { "g": 1, "a": { "a": 0, "k": [0, 0] } },
        "a": animators
    })
}

fn emit_text_document(d: &IrTextDocument) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert("t".to_string(), json!(d.text.clone()));
    obj.insert("f".to_string(), json!(d.font_name));
    obj.insert("s".to_string(), json!(d.font_size));
    obj.insert("fc".to_string(), json!(d.fill_color.to_vec()));
    obj.insert("j".to_string(), json!(d.justify));
    obj.insert("tr".to_string(), json!(d.tracking));
    let lh = d.line_height.unwrap_or(d.font_size * 1.2);
    obj.insert("lh".to_string(), json!(lh));
    obj.insert("ls".to_string(), json!(0));
    if let (Some(sc), Some(sw)) = (d.stroke_color, d.stroke_width) {
        obj.insert("sc".to_string(), json!(sc.to_vec()));
        obj.insert("sw".to_string(), json!(sw));
        obj.insert("of".to_string(), json!(true));
    }
    Value::Object(obj)
}
