//! Lottie 顶层 → DSL composition + precomp 块。

use serde_json::Value;

use crate::error::{DecompileError, Result};
use crate::format::{fmt_num, fmt_seconds, pad};
use crate::layer::emit_shape_layer;
use crate::transform::emit_transform;

pub fn emit_animation(root: &Value) -> Result<String> {
    let fps = root
        .get("fr")
        .and_then(|v| v.as_f64())
        .ok_or(DecompileError::MissingField("fr"))?;
    let in_frame = root.get("ip").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let out_frame = root
        .get("op")
        .and_then(|v| v.as_f64())
        .ok_or(DecompileError::MissingField("op"))?;
    let width = root.get("w").and_then(|v| v.as_u64()).unwrap_or(0);
    let height = root.get("h").and_then(|v| v.as_u64()).unwrap_or(0);
    let duration_sec = (out_frame - in_frame) / fps;
    let comp_name = root.get("nm").and_then(|v| v.as_str()).unwrap_or("untitled");

    let mut out = Vec::new();

    // precomp assets（先输出，因为 composition 中的 precomp inst 引用它们）
    let mut precomp_id = String::new();
    if let Some(assets) = root.get("assets").and_then(|v| v.as_array()) {
        for asset in assets {
            if asset.get("layers").is_some() {
                let id = asset.get("id").and_then(|v| v.as_str()).unwrap_or("comp_0");
                precomp_id = id.to_string();
                emit_precomp_asset(&mut out, asset, fps, out_frame, width, height, duration_sec);
                out.push(String::new());
            }
        }
    }

    // 顶层 composition
    out.push(format!("composition \"{}\" {{", comp_name));
    out.push(format!("    width    = {width}"));
    out.push(format!("    height   = {height}"));
    out.push(format!("    fps      = {}", fmt_num(fps)));
    out.push(format!("    duration = {}s", fmt_seconds(out_frame - in_frame, fps)));
    out.push(String::new());
    if let Some(layers) = root.get("layers").and_then(|v| v.as_array()) {
        for (i, layer) in layers.iter().enumerate() {
            match layer.get("ty").and_then(|v| v.as_i64()) {
                Some(0) => out.push(emit_precomp_inst(layer, &precomp_id, fps, 4)),
                Some(4) => out.push(emit_shape_layer(layer, fps, i, out_frame, 4)),
                _ => {} // 其他 layer 类型暂未覆盖（text/image/...，待扩展）
            }
            out.push(String::new());
        }
    }
    out.push("}".to_string());
    Ok(out.join("\n"))
}

fn emit_precomp_asset(
    out: &mut Vec<String>,
    asset: &Value,
    fps: f64,
    out_frame: f64,
    width: u64,
    height: u64,
    duration_sec: f64,
) {
    let id = asset.get("id").and_then(|v| v.as_str()).unwrap_or("comp_0");
    let asset_fps = asset.get("fr").and_then(|v| v.as_f64()).unwrap_or(fps);
    out.push(format!("precomp {id} {{"));
    out.push(format!("    width    = {width}"));
    out.push(format!("    height   = {height}"));
    out.push(format!("    fps      = {}", fmt_num(asset_fps)));
    out.push(format!("    duration = {}s", fmt_num(duration_sec)));
    out.push(String::new());
    if let Some(layers) = asset.get("layers").and_then(|v| v.as_array()) {
        for (i, layer) in layers.iter().enumerate() {
            if layer.get("ty").and_then(|v| v.as_i64()) == Some(4) {
                out.push(emit_shape_layer(layer, fps, i, out_frame, 4));
                out.push(String::new());
            }
        }
    }
    out.push("}".to_string());
}

fn emit_precomp_inst(layer: &Value, asset_id: &str, fps: f64, indent: usize) -> String {
    let p = pad(indent);
    let inner = indent + 4;
    let mut lines = vec![format!("{p}precomp inst {{"), format!("{}asset = {asset_id}", pad(inner))];
    if let Some(ks) = layer.get("ks") {
        let (statics, animated) = emit_transform(ks, fps, inner);
        lines.extend(statics);
        lines.extend(animated);
    }
    lines.push(format!("{p}}}"));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn empty_animation_emits_composition() {
        let root = json!({
            "v": "5.7.5", "fr": 30.0, "ip": 0.0, "op": 30.0,
            "w": 100, "h": 100, "nm": "test",
            "assets": [], "layers": []
        });
        let out = emit_animation(&root).unwrap();
        assert!(out.contains("composition \"test\""));
        assert!(out.contains("width    = 100"));
        assert!(out.contains("fps      = 30"));
    }

    #[test]
    fn missing_required_field_errors() {
        let root = json!({ "v": "5.7.5" });
        assert!(emit_animation(&root).is_err());
    }
}
