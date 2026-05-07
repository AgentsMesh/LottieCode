//! AE 约定：layer.anchor 与 group.tr.position 同步。
//!
//! 当 group.tr.position 是默认 [0,0] 时，把 layer.anchor 推到 group，
//! 让 path 顶点 [0,0] 经过两层变换后落在 layer.position（AE 标准）。
//! 显式偏移（如 DSL `group g0 { position = [...] }`）被尊重，不覆盖。

use crate::ir::{AnimatableValue, IrGroup, Keyframe};

pub fn promote_layer_anchor_to_groups(
    groups: &mut [IrGroup],
    layer_anchor: &AnimatableValue<[f64; 3]>,
) {
    for g in groups {
        if is_zero_position(&g.transform.position) {
            g.transform.position = anchor_3d_to_2d(layer_anchor);
        }
    }
}

fn is_zero_position(p: &AnimatableValue<[f64; 2]>) -> bool {
    matches!(p, AnimatableValue::Static(v) if v[0].abs() < 1e-9 && v[1].abs() < 1e-9)
}

fn anchor_3d_to_2d(a: &AnimatableValue<[f64; 3]>) -> AnimatableValue<[f64; 2]> {
    match a {
        AnimatableValue::Static(v) => AnimatableValue::Static([v[0], v[1]]),
        AnimatableValue::Animated(kfs) => {
            let mapped: Vec<Keyframe<[f64; 2]>> = kfs
                .iter()
                .map(|kf| Keyframe {
                    frame: kf.frame,
                    value: [kf.value[0], kf.value[1]],
                    easing: kf.easing,
                    hold: kf.hold,
                    spatial_in: kf.spatial_in,
                    spatial_out: kf.spatial_out,
                })
                .collect();
            AnimatableValue::Animated(mapped)
        }
        AnimatableValue::Expression { default, expr } => AnimatableValue::Expression {
            default: [default[0], default[1]],
            expr: expr.clone(),
        },
    }
}
