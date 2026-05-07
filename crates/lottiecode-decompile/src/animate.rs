//! Lottie 关键帧数组 → DSL `animate { ... }` 块。
//!
//! 关键功能：
//! - 时间换算：`t` 帧 / fps → 秒
//! - easing：从下一帧的 `o`/`i` 推断 cubic（DSL 把 ease 写在源帧上）
//! - spatial tangents：position 关键帧的 `to`/`ti` 还原为 `to=[...] ti=[...]`

use serde_json::Value;

use crate::format::{fmt_num, fmt_seconds, fmt_xy, pad};
use crate::transform::strip_z;

pub fn emit_anim_block(prop: &str, ks: &Value, fps: f64, indent: usize, dsl_name_for_strip: &str) -> String {
    let p = pad(indent);
    let kfs = ks.get("k").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let mut lines = vec![format!("{p}animate {prop} {{")];
    let inner = pad(indent + 4);
    for (i, kf) in kfs.iter().enumerate() {
        let t_frame = kf.get("t").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let v_str = format_keyframe_value(kf, dsl_name_for_strip);
        let ease_str = if i + 1 < kfs.len() {
            kf_ease(kf)
        } else {
            String::new()
        };
        let spatial_str = format_spatial_tangents(kf);
        lines.push(format!(
            "{inner}{}s: {v_str}{ease_str}{spatial_str}",
            fmt_seconds(t_frame, fps)
        ));
    }
    lines.push(format!("{p}}}"));
    lines.join("\n")
}

fn format_keyframe_value(kf: &Value, dsl_name: &str) -> String {
    let s = kf.get("s");
    match s {
        Some(Value::Array(arr)) => {
            let nums: Vec<f64> = arr.iter().filter_map(|v| v.as_f64()).collect();
            if nums.len() == 1 {
                fmt_num(nums[0])
            } else {
                let stripped = strip_z(&nums, dsl_name);
                fmt_xy(&stripped)
            }
        }
        Some(Value::Number(n)) => fmt_num(n.as_f64().unwrap_or(0.0)),
        _ => "0".to_string(),
    }
}

fn kf_ease(kf: &Value) -> String {
    let o = kf.get("o");
    let i = kf.get("i");
    let (Some(o), Some(i)) = (o, i) else { return String::new() };
    let ox = first_num(o.get("x"));
    let oy = first_num(o.get("y"));
    let ix = first_num(i.get("x"));
    let iy = first_num(i.get("y"));
    let (Some(ox), Some(oy), Some(ix), Some(iy)) = (ox, oy, ix, iy) else {
        return String::new();
    };
    // linear (o.x=0, o.y=0, i.x=1, i.y=1) 省略
    if ox.abs() < 1e-6 && oy.abs() < 1e-6 && (ix - 1.0).abs() < 1e-6 && (iy - 1.0).abs() < 1e-6 {
        return String::new();
    }
    format!(
        "  ease = cubic({}, {}, {}, {})",
        fmt_num(ox), fmt_num(oy), fmt_num(ix), fmt_num(iy)
    )
}

fn format_spatial_tangents(kf: &Value) -> String {
    let to = kf.get("to").and_then(|v| v.as_array());
    let ti = kf.get("ti").and_then(|v| v.as_array());
    let mut parts = Vec::new();
    if let Some(to) = to {
        let nums: Vec<f64> = to.iter().filter_map(|v| v.as_f64()).collect();
        if !is_zero(&nums) {
            parts.push(format!("to={}", fmt_xy(&nums)));
        }
    }
    if let Some(ti) = ti {
        let nums: Vec<f64> = ti.iter().filter_map(|v| v.as_f64()).collect();
        if !is_zero(&nums) {
            parts.push(format!("ti={}", fmt_xy(&nums)));
        }
    }
    if parts.is_empty() { String::new() } else { format!("  {}", parts.join(" ")) }
}

fn first_num(v: Option<&Value>) -> Option<f64> {
    let v = v?;
    if let Some(arr) = v.as_array() {
        arr.first().and_then(|x| x.as_f64())
    } else {
        v.as_f64()
    }
}

fn is_zero(nums: &[f64]) -> bool {
    nums.iter().all(|n| n.abs() < 1e-6)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn linear_easing_omitted() {
        let kf = json!({
            "t": 0.0, "s": [0.0, 0.0],
            "o": { "x": [0.0], "y": [0.0] },
            "i": { "x": [1.0], "y": [1.0] }
        });
        assert_eq!(kf_ease(&kf), "");
    }

    #[test]
    fn cubic_easing_emitted() {
        let kf = json!({
            "t": 0.0, "s": [0.0],
            "o": { "x": [0.4], "y": [0.0] },
            "i": { "x": [0.2], "y": [1.0] }
        });
        assert!(kf_ease(&kf).contains("ease = cubic(0.4, 0, 0.2, 1)"));
    }

    #[test]
    fn spatial_tangents_emitted_when_nonzero() {
        let kf = json!({
            "t": 0.0, "s": [0.0, 0.0],
            "to": [10.0, 0.0], "ti": [-5.0, 0.0]
        });
        let s = format_spatial_tangents(&kf);
        assert!(s.contains("to=[10, 0]"));
        assert!(s.contains("ti=[-5, 0]"));
    }

    #[test]
    fn zero_spatial_tangents_omitted() {
        let kf = json!({
            "t": 0.0, "s": [0.0, 0.0],
            "to": [0.0, 0.0], "ti": [0.0, 0.0]
        });
        assert_eq!(format_spatial_tangents(&kf), "");
    }
}
