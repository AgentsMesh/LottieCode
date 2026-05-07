//! Layer → DSL `shape` / `precomp inst` 块。
//!
//! ## 决策树
//! 1. 多 group：每个 group 独立 DSL `group g0 {} group g1 {}` 子块（共享 layer 属性）
//! 2. 单 group + group.tr.p ≠ layer.ks.a（反 AE 模式）：用 `group g0 {}` 显式包裹
//! 3. 单 group + group.tr.p == layer.ks.a：扁平输出（默认 anchor 推断）
//!
//! 反 AE 模式检测：原版 Layer 7 `layer.a=[0,0]` 但 `group.p=[-150,-81]`，
//! 必须保留 group 块否则 D-pad 圆环位置错位。

use serde_json::Value;

use crate::format::{fmt_seconds, pad, safe_name};
use crate::geometry::{emit_ellipse, emit_path_geometry, emit_polystar, emit_rect};
use crate::layer_helpers::{collect_shape_elems, group_offset_vs_anchor, top_level_groups};
use crate::shape::{emit_fill, emit_stroke, emit_trim, merge_mode_name};
use crate::transform::emit_transform;

pub fn emit_shape_layer(layer: &Value, fps: f64, idx: usize, total_op: f64, indent: usize) -> String {
    let top_groups: Vec<&Value> = top_level_groups(layer);
    if top_groups.len() > 1 {
        emit_multi_group(layer, &top_groups, fps, idx, indent, total_op)
    } else {
        emit_single_group(layer, fps, idx, total_op, indent)
    }
}

fn emit_single_group(layer: &Value, fps: f64, idx: usize, total_op: f64, indent: usize) -> String {
    let p = pad(indent);
    let nm = safe_name(layer.get("nm").and_then(|v| v.as_str()).unwrap_or(""), idx);
    let elems = collect_shape_elems(layer);
    let extra_offset = group_offset_vs_anchor(layer);
    let mut lines = vec![format!("{p}shape {nm} {{")];
    let inner = indent + 4;

    // AE 模式：直接扁平输出 path/rect/ellipse/polystar/fill/stroke
    if extra_offset.is_none() {
        for sh in &elems.paths { lines.push(emit_path_geometry(sh, inner)); }
        for rc in &elems.rects { lines.push(emit_rect(rc, inner)); }
        for el in &elems.ellipses { lines.push(emit_ellipse(el, inner)); }
        for sr in &elems.polystars { lines.push(emit_polystar(sr, inner)); }
        for s in &elems.fills { lines.push(emit_fill(s, inner)); }
        for s in &elems.strokes { lines.push(emit_stroke(s, inner)); }
    }
    if let Some(mm) = elems.merges.first() {
        let mode_n = mm.get("mm").and_then(|v| v.as_i64()).unwrap_or(1);
        lines.push(format!("{}merge = {}", pad(inner), merge_mode_name(mode_n)));
    }

    // layer-level transform / 时间属性
    if let Some(ks) = layer.get("ks") {
        let (statics, animated) = emit_transform(ks, fps, inner);
        lines.extend(statics);
        lines.extend(animated);
    }
    append_time_attrs(&mut lines, layer, fps, inner, total_op);

    // 反 AE 模式：path/fill/stroke 下沉到 group 块
    if let Some(off) = extra_offset {
        let g = pad(inner);
        let gi = inner + 4;
        lines.push(format!("{g}group g0 {{"));
        for sh in &elems.paths { lines.push(emit_path_geometry(sh, gi)); }
        for rc in &elems.rects { lines.push(emit_rect(rc, gi)); }
        for el in &elems.ellipses { lines.push(emit_ellipse(el, gi)); }
        for sr in &elems.polystars { lines.push(emit_polystar(sr, gi)); }
        for s in &elems.fills { lines.push(emit_fill(s, gi)); }
        for s in &elems.strokes { lines.push(emit_stroke(s, gi)); }
        lines.push(format!("{}position = [{}, {}]", pad(gi), crate::format::fmt_num(off[0]), crate::format::fmt_num(off[1])));
        lines.push(format!("{g}}}"));
    }

    for tm in &elems.trims {
        let block = emit_trim(tm, fps, inner);
        if !block.is_empty() { lines.push(block); }
    }
    lines.push(format!("{p}}}"));
    lines.join("\n")
}

fn emit_multi_group(layer: &Value, groups: &[&Value], fps: f64, idx: usize, indent: usize, total_op: f64) -> String {
    let p = pad(indent);
    let nm = safe_name(layer.get("nm").and_then(|v| v.as_str()).unwrap_or(""), idx);
    let inner = indent + 4;
    let mut lines = vec![format!("{p}shape {nm} {{")];
    if let Some(ks) = layer.get("ks") {
        let (statics, animated) = emit_transform(ks, fps, inner);
        lines.extend(statics);
        lines.extend(animated);
    }
    append_time_attrs(&mut lines, layer, fps, inner, total_op);
    for (gi, g) in groups.iter().enumerate() {
        let g_indent = inner;
        let inner2 = g_indent + 4;
        lines.push(format!("{}group g{gi} {{", pad(g_indent)));
        for it in g.get("it").and_then(|v| v.as_array()).into_iter().flatten() {
            match it.get("ty").and_then(|v| v.as_str()) {
                Some("sh") => lines.push(emit_path_geometry(it, inner2)),
                Some("rc") => lines.push(emit_rect(it, inner2)),
                Some("el") => lines.push(emit_ellipse(it, inner2)),
                Some("sr") => lines.push(emit_polystar(it, inner2)),
                Some("fl") => lines.push(emit_fill(it, inner2)),
                Some("st") => lines.push(emit_stroke(it, inner2)),
                Some("tr") => {
                    if let Some(p_arr) = it.get("p").and_then(|p| p.get("k")).and_then(|k| k.as_array()) {
                        let xy: Vec<f64> = p_arr.iter().filter_map(|v| v.as_f64()).take(2).collect();
                        if xy.len() == 2 {
                            lines.push(format!(
                                "{}position = [{}, {}]",
                                pad(inner2),
                                crate::format::fmt_num(xy[0]),
                                crate::format::fmt_num(xy[1])
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
        lines.push(format!("{}}}", pad(g_indent)));
    }
    lines.push(format!("{p}}}"));
    lines.join("\n")
}

fn append_time_attrs(lines: &mut Vec<String>, layer: &Value, fps: f64, indent: usize, total_op: f64) {
    let p = pad(indent);
    let ip = layer.get("ip").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let op = layer.get("op").and_then(|v| v.as_f64()).unwrap_or(total_op);
    let st = layer.get("st").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let sr = layer.get("sr").and_then(|v| v.as_f64()).unwrap_or(1.0);
    if ip > 0.001 {
        lines.push(format!("{p}start = {}s", fmt_seconds(ip, fps)));
    }
    if op < total_op - 0.001 {
        lines.push(format!("{p}end = {}s", fmt_seconds(op, fps)));
    }
    if st.abs() > 0.001 && (st - ip).abs() > 0.001 {
        lines.push(format!("{p}delay = {}s", fmt_seconds(st, fps)));
    }
    if (sr - 1.0).abs() > 0.001 && sr.abs() > 1e-9 {
        lines.push(format!("{p}speed = {}", crate::format::fmt_num(1.0 / sr)));
    }
}
