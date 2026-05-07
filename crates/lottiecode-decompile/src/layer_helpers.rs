//! Layer 内部 helper：shape 元素收集 + 反 AE 模式检测。

use serde_json::Value;

#[derive(Default)]
pub struct ShapeElems<'a> {
    pub paths: Vec<&'a Value>,
    pub rects: Vec<&'a Value>,
    pub ellipses: Vec<&'a Value>,
    pub polystars: Vec<&'a Value>,
    pub fills: Vec<&'a Value>,
    pub strokes: Vec<&'a Value>,
    pub trims: Vec<&'a Value>,
    pub merges: Vec<&'a Value>,
}

pub fn collect_shape_elems(layer: &Value) -> ShapeElems<'_> {
    let mut out = ShapeElems::default();
    if let Some(arr) = layer.get("shapes").and_then(|v| v.as_array()) {
        collect_into(arr, &mut out);
    }
    out
}

fn collect_into<'a>(items: &'a [Value], out: &mut ShapeElems<'a>) {
    for it in items {
        match it.get("ty").and_then(|v| v.as_str()) {
            Some("sh") => out.paths.push(it),
            Some("rc") => out.rects.push(it),
            Some("el") => out.ellipses.push(it),
            Some("sr") => out.polystars.push(it),
            Some("fl") => out.fills.push(it),
            Some("st") => out.strokes.push(it),
            Some("tm") => out.trims.push(it),
            Some("mm") => out.merges.push(it),
            _ => {
                if let Some(sub) = it.get("it").and_then(|v| v.as_array()) {
                    collect_into(sub, out);
                }
            }
        }
    }
}

pub fn top_level_groups(layer: &Value) -> Vec<&Value> {
    layer
        .get("shapes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter(|s| s.get("ty").and_then(|t| t.as_str()) == Some("gr")).collect())
        .unwrap_or_default()
}

/// 反 AE 模式检测：单 group 且 group.tr.p ≠ layer.ks.a → 返回 group.p。
/// 用于发现 `Layer 7` 那种反例（layer.a=[0,0] 但 group.p=[-150,-81]）。
pub fn group_offset_vs_anchor(layer: &Value) -> Option<[f64; 2]> {
    let groups = top_level_groups(layer);
    if groups.len() != 1 { return None; }
    let tr = groups[0]
        .get("it")?
        .as_array()?
        .iter()
        .find(|it| it.get("ty").and_then(|v| v.as_str()) == Some("tr"))?;
    let p_field = tr.get("p")?;
    if p_field.get("a").and_then(|v| v.as_i64()) != Some(0) { return None; }
    let gp_arr = p_field.get("k")?.as_array()?;
    let gp: Vec<f64> = gp_arr.iter().filter_map(|v| v.as_f64()).take(2).collect();
    if gp.len() != 2 { return None; }
    let la_arr = layer.get("ks")?.get("a")?.get("k")?.as_array()?;
    let la: Vec<f64> = la_arr.iter().filter_map(|v| v.as_f64()).take(2).collect();
    let la_x = la.first().copied().unwrap_or(0.0);
    let la_y = la.get(1).copied().unwrap_or(0.0);
    if (gp[0] - la_x).abs() < 1e-6 && (gp[1] - la_y).abs() < 1e-6 {
        None
    } else {
        Some([gp[0], gp[1]])
    }
}
