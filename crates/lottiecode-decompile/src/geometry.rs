//! shape 几何输出：rect / ellipse / polystar / path。
//!
//! 这些是产生路径的元素（`ty=rc/el/sr/sh`），与 styles/modifiers 区分。

use serde_json::Value;

use crate::format::{fmt_num, fmt_xy, fmt_xy_array, pad};

/// 把 sh (path) 元素转为 DSL `path { v=... in=... out=... closed=... }`。
/// Animated path（morph）：取第一帧作为静态体（path 动画的完整还原是后续 TODO）。
pub fn emit_path_geometry(sh: &Value, indent: usize) -> String {
    let p = pad(indent);
    let inner = pad(indent + 4);
    let Some(ks) = sh.get("ks") else {
        return format!("{p}path {{ }}");
    };
    let bezier = extract_bezier(ks);
    let Some(k) = bezier else {
        return format!("{p}path {{ }}");
    };
    let v = collect_xy_array(k.get("v"));
    let i = collect_xy_array(k.get("i"));
    let o = collect_xy_array(k.get("o"));
    let closed = k.get("c").and_then(|x| x.as_bool()).unwrap_or(false);
    if v.is_empty() {
        return format!("{p}path {{ }}");
    }
    format!(
        "{p}path {{\n{inner}v = {}\n{inner}in = {}\n{inner}out = {}\n{inner}closed = {}\n{p}}}",
        fmt_xy_array(&v),
        fmt_xy_array(&i),
        fmt_xy_array(&o),
        if closed { "true" } else { "false" },
    )
}

/// 抽取 path bezier 对象：静态 ks.k 直接返回；animated 取第一帧的 s[0]。
fn extract_bezier(ks: &Value) -> Option<&Value> {
    let k = ks.get("k")?;
    if ks.get("a").and_then(|v| v.as_i64()) == Some(1) {
        // animated: k 是 keyframe 数组，每帧 s 是 [bezier_obj]
        k.as_array()?.first()?.get("s")?.as_array()?.first()
    } else {
        Some(k)
    }
}

pub fn emit_rect(rc: &Value, indent: usize) -> String {
    let p = pad(indent);
    let inner = pad(indent + 4);
    let pos = static_xy(rc.get("p"));
    let size = static_xy(rc.get("s"));
    let r = static_num(rc.get("r"));
    let mut lines = vec![format!("{p}rect {{")];
    lines.push(format!("{inner}size = {}", fmt_xy(&size)));
    if pos.iter().any(|v| v.abs() > 1e-9) {
        lines.push(format!("{inner}position = {}", fmt_xy(&pos)));
    }
    if r.abs() > 1e-9 {
        lines.push(format!("{inner}radius = {}", fmt_num(r)));
    }
    lines.push(format!("{p}}}"));
    lines.join("\n")
}

pub fn emit_ellipse(el: &Value, indent: usize) -> String {
    let p = pad(indent);
    let inner = pad(indent + 4);
    let pos = static_xy(el.get("p"));
    let size = static_xy(el.get("s"));
    let mut lines = vec![format!("{p}ellipse {{")];
    lines.push(format!("{inner}size = {}", fmt_xy(&size)));
    if pos.iter().any(|v| v.abs() > 1e-9) {
        lines.push(format!("{inner}position = {}", fmt_xy(&pos)));
    }
    lines.push(format!("{p}}}"));
    lines.join("\n")
}

pub fn emit_polystar(sr: &Value, indent: usize) -> String {
    let p = pad(indent);
    let inner = pad(indent + 4);
    let pos = static_xy(sr.get("p"));
    let pt = static_num(sr.get("pt"));
    let or_ = static_num(sr.get("or"));
    let ir_ = static_num(sr.get("ir"));
    let os = static_num(sr.get("os"));
    let is_ = static_num(sr.get("is"));
    let r = static_num(sr.get("r"));
    let sy = sr.get("sy").and_then(|v| v.as_i64()).unwrap_or(1);
    let kind = if sy == 2 { "polygon" } else { "star" };
    let mut lines = vec![format!("{p}polystar {{")];
    lines.push(format!("{inner}points = {}", fmt_num(pt)));
    lines.push(format!("{inner}outer-radius = {}", fmt_num(or_)));
    lines.push(format!("{inner}inner-radius = {}", fmt_num(ir_)));
    if os.abs() > 1e-9 {
        lines.push(format!("{inner}outer-roundness = {}", fmt_num(os)));
    }
    if is_.abs() > 1e-9 {
        lines.push(format!("{inner}inner-roundness = {}", fmt_num(is_)));
    }
    if r.abs() > 1e-9 {
        lines.push(format!("{inner}rotation = {}", fmt_num(r)));
    }
    if pos.iter().any(|v| v.abs() > 1e-9) {
        lines.push(format!("{inner}position = {}", fmt_xy(&pos)));
    }
    lines.push(format!("{inner}type = {kind}"));
    lines.push(format!("{p}}}"));
    lines.join("\n")
}

fn static_xy(field: Option<&Value>) -> Vec<f64> {
    field
        .and_then(|f| f.get("k"))
        .and_then(|k| k.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect())
        .unwrap_or_else(|| vec![0.0, 0.0])
}

fn static_num(field: Option<&Value>) -> f64 {
    field
        .and_then(|f| f.get("k"))
        .and_then(|k| k.as_f64())
        .unwrap_or(0.0)
}

fn collect_xy_array(v: Option<&Value>) -> Vec<Vec<f64>> {
    let Some(arr) = v.and_then(|x| x.as_array()) else { return Vec::new() };
    arr.iter()
        .filter_map(|p| p.as_array())
        .map(|p| p.iter().filter_map(|x| x.as_f64()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn emit_ellipse_outputs_size_only_when_position_zero() {
        let el = json!({"p":{"k":[0.0,0.0]},"s":{"k":[40.0,40.0]}});
        let out = emit_ellipse(&el, 0);
        assert!(out.contains("size = [40, 40]"));
        assert!(!out.contains("position"));
    }

    #[test]
    fn emit_rect_with_radius() {
        let rc = json!({"p":{"k":[5.0,5.0]},"s":{"k":[20.0,30.0]},"r":{"k":4.0}});
        let out = emit_rect(&rc, 0);
        assert!(out.contains("size = [20, 30]"));
        assert!(out.contains("position = [5, 5]"));
        assert!(out.contains("radius = 4"));
    }

    #[test]
    fn emit_polystar_with_star_type() {
        let sr = json!({
            "p":{"k":[0.0,0.0]},"pt":{"k":5.0},"or":{"k":50.0},"ir":{"k":25.0},
            "os":{"k":0.0},"is":{"k":0.0},"r":{"k":0.0},"sy":1
        });
        let out = emit_polystar(&sr, 0);
        assert!(out.contains("points = 5"));
        assert!(out.contains("outer-radius = 50"));
        assert!(out.contains("type = star"));
    }
}
