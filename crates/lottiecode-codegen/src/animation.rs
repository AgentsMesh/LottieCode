//! Animation 顶层 → Lottie JSON 根对象。
//!
//! ## AE 字段顺序契约
//! - 根对象：`v, fr, ip, op, w, h, nm, ddd, assets, layers, markers, [fonts], [slots]`
//! - precomp asset：`id, [fr], layers`（fr 仅在与顶层 fps 不同时输出）
//! - image asset：`id, w, h, u, p, e`

use serde_json::{json, Value};

use lottiecode_lang::ir::{IrAnimation, IrAsset, IrFont, IrMarker};

use crate::layer::{build_name_index, emit_layer};

pub fn emit_animation(ir: &IrAnimation) -> Value {
    let name_index = build_name_index(&ir.layers);
    let layers: Vec<Value> = ir
        .layers
        .iter()
        .enumerate()
        .map(|(i, l)| emit_layer(l, (i + 1) as i64, &name_index))
        .collect();

    let mut root = serde_json::Map::new();
    root.insert("v".to_string(), json!(ir.version));
    root.insert("fr".to_string(), json!(ir.fps));
    root.insert("ip".to_string(), json!(ir.in_frame));
    root.insert("op".to_string(), json!(ir.out_frame));
    root.insert("w".to_string(), json!(ir.width));
    root.insert("h".to_string(), json!(ir.height));
    root.insert("nm".to_string(), json!(ir.name));
    root.insert("ddd".to_string(), json!(0));
    root.insert("assets".to_string(), json!(emit_assets(&ir.assets, ir.fps)));
    root.insert("layers".to_string(), json!(layers));
    root.insert("markers".to_string(), json!(emit_markers(&ir.markers)));

    if !ir.fonts.is_empty() {
        root.insert("fonts".to_string(), emit_fonts(&ir.fonts));
    }

    if !ir.slots.is_empty() {
        root.insert("slots".to_string(), emit_slots(&ir.slots));
    }
    if ir.has_3d {
        root.insert("ddd".to_string(), json!(1));
    }

    Value::Object(root)
}

fn emit_markers(markers: &[IrMarker]) -> Vec<Value> {
    markers
        .iter()
        .map(|m| {
            json!({
                "tm": m.time_frame,
                "cm": m.comment,
                "dr": m.duration,
            })
        })
        .collect()
}

fn emit_slots(slots: &[lottiecode_lang::ir::IrSlot]) -> Value {
    let mut obj = serde_json::Map::new();
    for s in slots {
        let entry = match &s.kind {
            lottiecode_lang::ir::IrSlotKind::Color(c) => json!({
                "p": { "a": 0, "k": c.to_vec() }, "t": 1
            }),
            lottiecode_lang::ir::IrSlotKind::Number(n) => json!({
                "p": { "a": 0, "k": n }, "t": 0
            }),
            lottiecode_lang::ir::IrSlotKind::String(s) => json!({
                "p": s, "t": 5
            }),
        };
        obj.insert(s.name.clone(), entry);
    }
    Value::Object(obj)
}

fn emit_assets(assets: &[IrAsset], top_fps: f64) -> Vec<Value> {
    assets
        .iter()
        .map(|a| match a {
            IrAsset::Image {
                id,
                width,
                height,
                path,
                embedded,
            } => {
                if *embedded {
                    json!({
                        "id": id,
                        "w": width,
                        "h": height,
                        "u": "",
                        "p": path,
                        "e": 1,
                    })
                } else {
                    let (u, p) = match path.rsplit_once('/') {
                        Some((dir, file)) => (format!("{dir}/"), file.to_string()),
                        None => (String::new(), path.clone()),
                    };
                    json!({
                        "id": id,
                        "w": width,
                        "h": height,
                        "u": u,
                        "p": p,
                        "e": 0,
                    })
                }
            }
            IrAsset::Precomp {
                id,
                fps,
                layers,
                ..
            } => {
                let sub_index = build_name_index(layers);
                let layer_values: Vec<Value> = layers
                    .iter()
                    .enumerate()
                    .map(|(i, l)| emit_layer(l, (i + 1) as i64, &sub_index))
                    .collect();
                let mut obj = serde_json::Map::new();
                obj.insert("id".to_string(), json!(id));
                // fr 仅当与顶层 fps 不同时输出（AE 习惯：继承时省略）。
                if (fps - top_fps).abs() > 1e-9 {
                    obj.insert("fr".to_string(), json!(fps));
                }
                obj.insert("layers".to_string(), json!(layer_values));
                Value::Object(obj)
            }
            IrAsset::Sound {
                id,
                path,
                embedded,
            } => {
                if *embedded {
                    json!({ "id": id, "u": "", "p": path, "e": 1 })
                } else {
                    let (u, p) = match path.rsplit_once('/') {
                        Some((dir, file)) => (format!("{dir}/"), file.to_string()),
                        None => (String::new(), path.clone()),
                    };
                    json!({ "id": id, "u": u, "p": p, "e": 0 })
                }
            }
            IrAsset::Data {
                id,
                path,
                embedded,
            } => {
                if *embedded {
                    json!({ "id": id, "u": "", "p": path, "e": 1, "t": 3 })
                } else {
                    let (u, p) = match path.rsplit_once('/') {
                        Some((dir, file)) => (format!("{dir}/"), file.to_string()),
                        None => (String::new(), path.clone()),
                    };
                    json!({ "id": id, "u": u, "p": p, "e": 0, "t": 3 })
                }
            }
        })
        .collect()
}

fn emit_fonts(fonts: &[IrFont]) -> Value {
    let list: Vec<Value> = fonts
        .iter()
        .map(|f| {
            json!({
                "fName": f.name,
                "fFamily": f.family,
                "fStyle": f.style,
                "fWeight": "",
                "fPath": "",
                "fClass": "",
                "origin": 0,
                "ascent": 75.0,
            })
        })
        .collect();
    json!({ "list": list })
}


#[cfg(test)]
#[path = "animation_tests.rs"]
mod tests;
