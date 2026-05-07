//! shape 样式 + modifier 输出：fill / stroke / trim / merge mode 名映射。
//!
//! 几何元素（path/rect/ellipse/polystar）见 `geometry` 模块。

use serde_json::Value;

use crate::animate::emit_anim_block;
use crate::format::{fmt_color, fmt_num, pad};

pub fn emit_fill(s: &Value, indent: usize) -> String {
    let p = pad(indent);
    let color = s
        .get("c")
        .and_then(|c| c.get("k"))
        .and_then(|k| k.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect::<Vec<_>>())
        .unwrap_or_default();
    format!("{p}fill = {}", fmt_color(&color))
}

pub fn emit_stroke(s: &Value, indent: usize) -> String {
    let p = pad(indent);
    let color = s
        .get("c")
        .and_then(|c| c.get("k"))
        .and_then(|k| k.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect::<Vec<_>>())
        .unwrap_or_default();
    let width = s
        .get("w")
        .and_then(|w| w.get("k"))
        .and_then(|k| k.as_f64())
        .unwrap_or(1.0);
    let cap = match s.get("lc").and_then(|v| v.as_i64()).unwrap_or(2) {
        1 => "butt",
        3 => "square",
        _ => "round",
    };
    let join = match s.get("lj").and_then(|v| v.as_i64()).unwrap_or(2) {
        1 => "miter",
        3 => "bevel",
        _ => "round",
    };
    format!(
        "{p}stroke = {{ color = {}, width = {}, cap = {}, join = {} }}",
        fmt_color(&color),
        fmt_num(width),
        cap,
        join
    )
}

/// trim 输出：动画三字段产生独立 animate 块；静态部分合并到一个 `trim {}` 行。
pub fn emit_trim(tm: &Value, fps: f64, indent: usize) -> String {
    let p = pad(indent);
    let mut blocks = Vec::new();
    let mut statics = Vec::new();
    for (key, anim_name, static_name) in &[
        ("s", "trim-start", "start"),
        ("e", "trim-end", "end"),
        ("o", "trim-offset", "offset"),
    ] {
        let Some(field) = tm.get(*key) else { continue };
        let is_animated = field.get("a").and_then(|v| v.as_i64()) == Some(1);
        if is_animated {
            blocks.push(emit_anim_block(anim_name, field, fps, indent, anim_name));
        } else if let Some(n) = field.get("k").and_then(|v| v.as_f64()) {
            statics.push(format!("{} = {}", static_name, fmt_num(n)));
        }
    }
    if !statics.is_empty() {
        blocks.push(format!("{p}trim {{ {} }}", statics.join(", ")));
    }
    blocks.join("\n")
}

pub fn merge_mode_name(mode: i64) -> &'static str {
    match mode {
        2 => "add",
        3 => "subtract",
        4 => "intersect",
        5 => "exclude",
        _ => "merge",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn emit_fill_outputs_color() {
        let s = json!({ "c": { "k": [1.0, 0.0, 0.0, 1.0] } });
        assert!(emit_fill(&s, 4).contains("fill = #FF0000"));
    }

    #[test]
    fn emit_stroke_outputs_full_object() {
        let s = json!({
            "c": { "k": [0.0, 0.0, 0.0, 1.0] },
            "w": { "k": 2.0 }, "lc": 2, "lj": 1,
        });
        let out = emit_stroke(&s, 0);
        assert!(out.contains("color = #000000"));
        assert!(out.contains("width = 2"));
        assert!(out.contains("cap = round"));
        assert!(out.contains("join = miter"));
    }
}
