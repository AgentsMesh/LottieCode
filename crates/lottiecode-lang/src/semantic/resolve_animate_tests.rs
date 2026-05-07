use super::*;
use crate::ast::Keyframe as AstKeyframe;
use crate::token::Span;
use std::collections::HashMap;

fn sp() -> Span {
    Span::new(0, 0, 1, 1)
}

fn kf(time: Expression, value: Expression) -> AstKeyframe {
    AstKeyframe {
        time,
        value,
        easing: None,
        hold: false,
        spatial_to: None,
        spatial_ti: None,
        span: sp(),
    }
}

fn anim(kfs: Vec<AstKeyframe>) -> AnimateDecl {
    AnimateDecl {
        property: "opacity".into(),
        keyframes: kfs,
        looped: false,
        span: sp(),
    }
}

#[test]
fn at_least_two_keyframes_required() {
    let a = anim(vec![kf(
        Expression::TimeSec(0.0, sp()),
        Expression::IntLit(0, sp()),
    )]);
    let err = animate_to_value(&a, 30.0, &HashMap::new(), conv_scalar).unwrap_err();
    assert_eq!(err.kind, ErrorKind::InvalidValue);
}

#[test]
fn keyframe_order_strictly_increasing() {
    let a = anim(vec![
        kf(Expression::TimeSec(1.0, sp()), Expression::IntLit(100, sp())),
        kf(Expression::TimeSec(0.5, sp()), Expression::IntLit(0, sp())),
    ]);
    let err = animate_to_value(&a, 30.0, &HashMap::new(), conv_scalar).unwrap_err();
    assert_eq!(err.kind, ErrorKind::KeyframeOutOfOrder);
}

#[test]
fn unknown_named_easing_errors() {
    let a = anim(vec![
        AstKeyframe {
            time: Expression::TimeSec(0.0, sp()),
            value: Expression::IntLit(0, sp()),
            easing: Some(EasingExpr::Named("not-real".into())),
            hold: false,
            spatial_to: None,
            spatial_ti: None,
            span: sp(),
        },
        kf(Expression::TimeSec(1.0, sp()), Expression::IntLit(1, sp())),
    ]);
    let err = animate_to_value(&a, 30.0, &HashMap::new(), conv_scalar).unwrap_err();
    assert_eq!(err.kind, ErrorKind::UndefinedEasing);
}

#[test]
fn frame_conversion_uses_fps() {
    let a = anim(vec![
        kf(Expression::TimeSec(0.0, sp()), Expression::IntLit(0, sp())),
        kf(Expression::TimeSec(2.0, sp()), Expression::IntLit(100, sp())),
    ]);
    let v = animate_to_value(&a, 30.0, &HashMap::new(), conv_scalar).unwrap();
    if let AnimatableValue::Animated(kfs) = v {
        assert_eq!(kfs[0].frame, 0.0);
        assert_eq!(kfs[1].frame, 60.0);
    } else {
        panic!("expected animated");
    }
}

#[test]
fn conv_color_rejects_non_color() {
    assert!(conv_color(&Expression::IntLit(0, sp())).is_err());
}

#[test]
fn conv_color_accepts_hex_lit() {
    let v = conv_color(&Expression::ColorLit("FF0000".into(), sp())).unwrap();
    assert!((v[0] - 1.0).abs() < 1e-9);
}
