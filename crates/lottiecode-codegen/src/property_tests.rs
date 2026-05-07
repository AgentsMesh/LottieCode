use super::*;

#[test]
fn static_scalar_emits_a_zero() {
    let v = AnimatableValue::Static(42.0);
    let out = emit_scalar(&v);
    assert_eq!(out["a"], 0);
    assert_eq!(out["k"], 42.0);
}

#[test]
fn animated_scalar_emits_a_one_with_keyframes() {
    let kfs = vec![
        Keyframe {
            frame: 0.0,
            value: 0.0,
            easing: Easing::Linear,
            hold: false,
            spatial_in: None,
            spatial_out: None,
        },
        Keyframe {
            frame: 30.0,
            value: 100.0,
            easing: Easing::Cubic([0.4, 0.0, 0.2, 1.0]),
            hold: false,
            spatial_in: None,
            spatial_out: None,
        },
    ];
    let out = emit_scalar(&AnimatableValue::Animated(kfs));
    assert_eq!(out["a"], 1);
    let arr = out["k"].as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["t"], 0.0);
    assert_eq!(arr[1]["t"], 30.0);
}

#[test]
fn expression_emits_x_field() {
    let v = AnimatableValue::Expression {
        default: 50.0_f64,
        expr: "time*100".to_string(),
    };
    let out = emit_scalar(&v);
    assert_eq!(out["a"], 0);
    assert_eq!(out["k"], 50.0);
    assert_eq!(out["x"], "time*100");
}

#[test]
fn static_xy_emits_array() {
    let v = AnimatableValue::Static([1.5, 2.5]);
    let out = emit_xy(&v);
    assert_eq!(out["k"], json!([1.5, 2.5]));
}

#[test]
fn static_color_emits_array() {
    let v = AnimatableValue::Static([1.0, 0.5, 0.0, 1.0]);
    let out = emit_color(&v);
    assert_eq!(out["k"], json!([1.0, 0.5, 0.0, 1.0]));
}

#[test]
fn hold_keyframe_marked() {
    let kfs = vec![
        Keyframe {
            frame: 0.0,
            value: 0.0,
            easing: Easing::Linear,
            hold: true,
            spatial_in: None,
            spatial_out: None,
        },
        Keyframe {
            frame: 30.0,
            value: 100.0,
            easing: Easing::Linear,
            hold: false,
            spatial_in: None,
            spatial_out: None,
        },
    ];
    let out = emit_scalar(&AnimatableValue::Animated(kfs));
    assert_eq!(out["k"][0]["h"], 1);
}

#[test]
fn spatial_tangent_emitted_for_xyz() {
    let kfs = vec![
        Keyframe {
            frame: 0.0,
            value: [0.0, 0.0, 0.0],
            easing: Easing::Linear,
            hold: false,
            spatial_in: Some([1.0, 2.0, 0.0]),
            spatial_out: Some([3.0, 4.0, 0.0]),
        },
        Keyframe {
            frame: 30.0,
            value: [10.0, 10.0, 0.0],
            easing: Easing::Linear,
            hold: false,
            spatial_in: None,
            spatial_out: None,
        },
    ];
    let out = emit_xyz(&AnimatableValue::Animated(kfs));
    assert_eq!(out["k"][0]["ti"], json!([1.0, 2.0, 0.0]));
    assert_eq!(out["k"][0]["to"], json!([3.0, 4.0, 0.0]));
}
