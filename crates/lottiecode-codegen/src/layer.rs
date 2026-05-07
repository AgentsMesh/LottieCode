//! Layer → Lottie layer 对象。
//!
//! ## AE 字段顺序契约
//!
//! 输出与 After Effects 保持一致的顺序，便于与 sample 做 byte-level diff：
//! ```text
//! ddd, ind, ty, nm, sr, ks, ao, [shapes/refId/t/sw/...], ip, op, st, bm,
//! [hasMask, masksProperties], [ef], [parent], [td], [tt, tp]
//! ```
//! 启用 serde_json 的 `preserve_order` feature，依赖 Map::insert 顺序输出。
//! 任何字段顺序调整都必须维持本契约——若 player 严格按字段顺序解析（lottie-web
//! 优化解析器、移动端等），乱序可能导致渲染失败。
//!
//! ## ks（layer transform）契约
//!
//! 顺序：`o, r, p, a, s, [sk], [sa]`。sk/sa 默认 0 时省略（AE 习惯）。

use std::collections::HashMap;

use serde_json::{json, Value};

use lottiecode_lang::ir::{AnimatableValue, IrLayer, IrLayerKind, IrTransform};

use crate::effect::emit_effects;
use crate::mask::emit_masks;
use crate::property::*;
use crate::shape::emit_shape_layer;
use crate::text::emit_text_data;

/// 编译期建立 layer 名 → ind 映射，便于 parent 字段输出。
pub fn build_name_index(layers: &[IrLayer]) -> HashMap<String, i64> {
    let mut map = HashMap::new();
    for (i, l) in layers.iter().enumerate() {
        map.insert(l.name.clone(), (i + 1) as i64);
    }
    map
}

pub fn emit_layer(layer: &IrLayer, ind: i64, name_index: &HashMap<String, i64>) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert("ddd".to_string(), json!(0));
    obj.insert("ind".to_string(), json!(ind));
    obj.insert("ty".to_string(), json!(layer_type_id(&layer.kind)));
    obj.insert("nm".to_string(), json!(layer.name.clone()));
    obj.insert("sr".to_string(), json!(layer.time_stretch));
    obj.insert("ks".to_string(), emit_transform(&layer.transform));
    obj.insert("ao".to_string(), json!(0));

    // 紧跟 ao 后输出 layer kind 特有字段（与 AE 输出顺序一致：shapes/refId 在 ip 之前）
    insert_kind_payload(&mut obj, &layer.kind);

    obj.insert("ip".to_string(), json!(layer.in_frame));
    obj.insert("op".to_string(), json!(layer.out_frame));
    obj.insert("st".to_string(), json!(layer.start_time));
    obj.insert("bm".to_string(), json!(layer.blend_mode.as_lottie()));

    if !layer.masks.is_empty() {
        obj.insert("hasMask".to_string(), json!(true));
        obj.insert("masksProperties".to_string(), json!(emit_masks(&layer.masks)));
    }
    if !layer.effects.is_empty() {
        obj.insert("ef".to_string(), json!(emit_effects(&layer.effects)));
    }
    if let Some(p) = &layer.parent {
        if let Some(p_ind) = name_index.get(p) {
            obj.insert("parent".to_string(), json!(p_ind));
        }
    }
    if layer.matte_source {
        obj.insert("td".to_string(), json!(1));
    }
    if let Some((source_name, mode)) = &layer.matte {
        obj.insert("tt".to_string(), json!(mode.as_lottie()));
        if let Some(src_ind) = name_index.get(source_name) {
            obj.insert("tp".to_string(), json!(src_ind));
        }
    }

    Value::Object(obj)
}

fn layer_type_id(kind: &IrLayerKind) -> u8 {
    match kind {
        IrLayerKind::Precomp { .. } => 0,
        IrLayerKind::Solid { .. } => 1,
        IrLayerKind::Image { .. } => 2,
        IrLayerKind::Null => 3,
        IrLayerKind::Shape(_) => 4,
        IrLayerKind::Text(_) => 5,
        IrLayerKind::Audio { .. } => 6,
        IrLayerKind::Camera { .. } => 13,
        IrLayerKind::Data { .. } => 15,
    }
}

fn insert_kind_payload(
    obj: &mut serde_json::Map<String, Value>,
    kind: &IrLayerKind,
) {
    match kind {
        IrLayerKind::Shape(shape_layer) => {
            // anchor 已在 semantic 层 promote 到 group.tr.position，此处直接输出。
            obj.insert("shapes".to_string(), json!(emit_shape_layer(shape_layer)));
        }
        IrLayerKind::Text(td) => {
            obj.insert("t".to_string(), emit_text_data(td));
        }
        IrLayerKind::Image { asset_id } => {
            obj.insert("refId".to_string(), json!(asset_id));
        }
        IrLayerKind::Precomp { asset_id, width, height, time_remap } => {
            obj.insert("refId".to_string(), json!(asset_id));
            obj.insert("w".to_string(), json!(width));
            obj.insert("h".to_string(), json!(height));
            if let Some(tm) = time_remap {
                obj.insert("tm".to_string(), emit_scalar(tm));
            }
        }
        IrLayerKind::Solid { color, width, height } => {
            obj.insert("sw".to_string(), json!(width));
            obj.insert("sh".to_string(), json!(height));
            let r = (color[0] * 255.0).round() as u8;
            let g = (color[1] * 255.0).round() as u8;
            let b = (color[2] * 255.0).round() as u8;
            obj.insert("sc".to_string(), json!(format!("#{:02X}{:02X}{:02X}", r, g, b)));
        }
        IrLayerKind::Null => {}
        IrLayerKind::Audio { asset_id, volume } => {
            obj.insert("refId".to_string(), json!(asset_id));
            obj.insert("au".to_string(), json!({ "lv": { "a": 0, "k": [volume] } }));
        }
        IrLayerKind::Data { asset_id } => {
            obj.insert("refId".to_string(), json!(asset_id));
        }
        IrLayerKind::Camera { perspective } => {
            obj.insert("pe".to_string(), emit_scalar(perspective));
        }
    }
}

pub fn emit_transform(tr: &IrTransform) -> Value {
    let mut m = serde_json::Map::new();
    m.insert("o".to_string(), emit_scalar(&tr.opacity));
    m.insert("r".to_string(), emit_scalar(&tr.rotation));
    m.insert("p".to_string(), emit_xyz(&tr.position));
    m.insert("a".to_string(), emit_xyz(&tr.anchor));
    m.insert("s".to_string(), emit_xyz(&tr.scale));
    if !is_zero_scalar(&tr.skew) {
        m.insert("sk".to_string(), emit_scalar(&tr.skew));
    }
    if !is_zero_scalar(&tr.skew_axis) {
        m.insert("sa".to_string(), emit_scalar(&tr.skew_axis));
    }
    Value::Object(m)
}

fn is_zero_scalar(v: &AnimatableValue<f64>) -> bool {
    matches!(v, AnimatableValue::Static(x) if x.abs() < 1e-9)
}
