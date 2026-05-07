//! Lottie ks (transform) → DSL 静态属性 + 动画块。
//!
//! 输出顺序：position → anchor → scale → rotation → opacity（与 IR 一致）。
//! 静态值直接 inline 输出 `dsl = value`；动画值产生 `animate dsl { ... }` 块。

use serde_json::Value;

use crate::animate::emit_anim_block;
use crate::format::{fmt_num, fmt_xy, pad};

/// 返回 (静态属性行, 动画块行)。
pub fn emit_transform(ks: &Value, fps: f64, indent: usize) -> (Vec<String>, Vec<String>) {
    let mut statics = Vec::new();
    let mut animated = Vec::new();
    let mapping: &[(&str, &str)] = &[
        ("p", "position"),
        ("a", "anchor"),
        ("s", "scale"),
        ("r", "rotation"),
        ("o", "opacity"),
    ];
    for (key, dsl) in mapping {
        let Some(prop) = ks.get(*key) else { continue };
        let is_animated = prop.get("a").and_then(|v| v.as_i64()) == Some(1);
        if is_animated {
            animated.push(emit_anim_block(dsl, prop, fps, indent, dsl));
        } else if let Some(line) = emit_static(prop, dsl, indent) {
            statics.push(line);
        }
    }
    (statics, animated)
}

/// 把静态 ks 字段输出为 DSL `name = value`。返回 None 表示数据结构异常。
fn emit_static(prop: &Value, dsl_name: &str, indent: usize) -> Option<String> {
    let k = prop.get("k")?;
    let p = pad(indent);
    if let Some(arr) = k.as_array() {
        let nums: Vec<f64> = arr.iter().filter_map(|v| v.as_f64()).collect();
        let stripped = strip_z(&nums, dsl_name);
        if stripped.len() == 1 {
            Some(format!("{p}{dsl_name} = {}", fmt_num(stripped[0])))
        } else {
            Some(format!("{p}{dsl_name} = {}", fmt_xy(&stripped)))
        }
    } else if let Some(n) = k.as_f64() {
        Some(format!("{p}{dsl_name} = {}", fmt_num(n)))
    } else {
        None
    }
}

/// 把 [x, y, 0] 中的 z=0 维去掉（避免误触发 3D）。仅 position / anchor 安全做。
/// scale 不能剥（z=0 会被解析为 [x,y,0] 即 0% z 缩放破坏 identity）。
pub fn strip_z(vals: &[f64], dsl_name: &str) -> Vec<f64> {
    if vals.len() == 3 && (dsl_name == "position" || dsl_name == "anchor") && vals[2].abs() < 1e-9 {
        vals[..2].to_vec()
    } else {
        vals.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn static_position_strips_z() {
        let ks = json!({
            "p": { "a": 0, "k": [100.0, 50.0, 0.0] },
            "s": { "a": 0, "k": [80.0, 80.0, 100.0] },
        });
        let (s, a) = emit_transform(&ks, 30.0, 4);
        assert!(a.is_empty());
        assert!(s.iter().any(|l| l.contains("position = [100, 50]")));
        // scale 保留 z
        assert!(s.iter().any(|l| l.contains("scale = [80, 80, 100]")));
    }

    #[test]
    fn animated_position_produces_block() {
        let ks = json!({
            "p": {
                "a": 1,
                "k": [
                    { "t": 0.0, "s": [0.0, 0.0] },
                    { "t": 30.0, "s": [100.0, 100.0] },
                ]
            }
        });
        let (s, a) = emit_transform(&ks, 30.0, 0);
        assert!(s.is_empty());
        assert_eq!(a.len(), 1);
        assert!(a[0].contains("animate position"));
    }
}
