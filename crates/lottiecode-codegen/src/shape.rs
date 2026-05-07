//! IrShapeLayer / IrGroup → Lottie shapes 数组。
//!
//! ## 渲染顺序（group.it 数组）
//! 类型系统强制：geometries → modifiers → styles → repeater → auto transform。
//! 该顺序模拟 AE shape stack 自上而下：geometry 产生路径 → modifier 变形 → style 着色 → repeater 复制。
//!
//! ## AE 字段顺序契约
//! - 整个 shape layer：先输出所有 group，layer-level trim 作 group 外的 sibling 追加。
//! - group：`ty, it, nm, bm, hd`（it 数组内部按上述渲染顺序）。
//! - tr（group 末尾 transform）：`ty, p, a, s, r, o, sk, sa, nm`（与 layer.ks 顺序不同）。

use serde_json::{json, Value};

use lottiecode_lang::ir::{
    IrGroup, IrPathModifier, IrRepeater, IrShapeLayer, IrShapeTransform, IrStyle,
};

use crate::property::*;
use crate::shape_geometry::emit_geometry;
use crate::shape_modifier::emit_modifier;
use crate::shape_style::emit_style;

/// 把 IrShapeLayer 转为 Lottie `shapes` 数组。
/// layer-level trim 作 group 外 sibling（与 Lottie 设计师工具一致）。
pub fn emit_shape_layer(layer: &IrShapeLayer) -> Vec<Value> {
    let mut out: Vec<Value> = layer.groups.iter().map(emit_group).collect();
    if let Some(tm) = &layer.layer_trim {
        out.push(emit_modifier(tm));
    }
    out
}

pub fn emit_group(g: &IrGroup) -> Value {
    let mut items: Vec<Value> = Vec::new();
    for geo in &g.geometries {
        items.push(emit_geometry(geo));
    }
    for m in &g.modifiers {
        items.push(emit_modifier(m));
    }
    for st in &g.styles {
        items.push(emit_style(st));
    }
    if let Some(rp) = &g.repeater {
        items.push(emit_repeater(rp));
    }
    items.push(emit_shape_transform(&g.transform));
    json!({
        "ty": "gr",
        "it": items,
        "nm": g.name,
        "bm": 0,
        "hd": false,
    })
}

fn emit_repeater(rp: &IrRepeater) -> Value {
    json!({
        "ty": "rp",
        "c": emit_scalar(&rp.copies),
        "o": emit_scalar(&rp.offset),
        "m": rp.composite.as_lottie(),
        "tr": emit_shape_transform(&rp.transform),
        "nm": "Repeater",
        "hd": false,
    })
}

/// Group 末尾的 Transform（`tr`）—— 由 codegen 自动注入。
pub fn emit_shape_transform(tr: &IrShapeTransform) -> Value {
    json!({
        "ty": "tr",
        "p": emit_xy(&tr.position),
        "a": emit_xy(&tr.anchor),
        "s": emit_xy(&tr.scale),
        "r": emit_scalar(&tr.rotation),
        "o": emit_scalar(&tr.opacity),
        "sk": emit_scalar(&tr.skew),
        "sa": emit_scalar(&tr.skew_axis),
        "nm": "Transform",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use lottiecode_lang::ir::*;

    fn st<T>(v: T) -> AnimatableValue<T> {
        AnimatableValue::Static(v)
    }

    fn group_with(geometries: Vec<IrGeometry>, styles: Vec<IrStyle>) -> IrGroup {
        IrGroup {
            name: "g".into(),
            geometries,
            modifiers: Vec::new(),
            styles,
            repeater: None,
            transform: IrShapeTransform::default(),
        }
    }

    #[test]
    fn group_emits_gr_with_geometry_and_style() {
        let g = group_with(
            vec![IrGeometry::Rectangle {
                position: st([0.0, 0.0]),
                size: st([100.0, 50.0]),
                radius: st(5.0),
                direction: PathDirection::Normal,
            }],
            vec![IrStyle::Fill {
                color: st([1.0, 0.0, 0.0, 1.0]),
                opacity: st(100.0),
                rule: FillRule::NonZero,
            }],
        );
        let j = emit_group(&g);
        assert_eq!(j["ty"], "gr");
        let items = j["it"].as_array().unwrap();
        assert_eq!(items[0]["ty"], "rc");
        assert_eq!(items[1]["ty"], "fl");
        assert_eq!(items[2]["ty"], "tr");
    }

    #[test]
    fn shape_layer_with_layer_trim_appends_sibling() {
        let layer = IrShapeLayer {
            groups: vec![group_with(
                vec![IrGeometry::Ellipse {
                    position: st([0.0, 0.0]),
                    size: st([10.0, 10.0]),
                    direction: PathDirection::Normal,
                }],
                vec![],
            )],
            layer_trim: Some(IrPathModifier::TrimPath {
                start: st(0.0),
                end: st(50.0),
                offset: st(0.0),
                mode: TrimMode::Simultaneously,
            }),
        };
        let v = emit_shape_layer(&layer);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0]["ty"], "gr");
        assert_eq!(v[1]["ty"], "tm");
    }
}
