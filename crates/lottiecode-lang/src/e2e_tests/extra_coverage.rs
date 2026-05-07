//! 补充覆盖：spatial tangents、blend mode、matte modes、modifier 覆盖更多分支。

use crate::ir::{IrLayerKind, IrPathModifier};

use super::{compile, has_modifier};

#[test]
fn spatial_tangents_in_keyframes() {
    let ir = compile(
        r#"composition "x" {
            width=200 height=200 fps=30 duration=1s
            shape s {
                rect { size=[10,10] }
                fill = #000000
                position = [50, 50]
                animate position {
                    0s:  [0, 0]    to=[20, 0]
                    1s:  [100, 0]  ti=[-20, 0]
                }
            }
        }"#,
    )
    .unwrap();
    assert_eq!(ir.layers.len(), 1);
}

#[test]
fn matte_alpha_inverted_mode() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape src { ellipse{size=[80,80]} fill=#FFFFFF position=[50,50] matte-source=true }
            shape dst { rect{size=[80,80]} fill=#FF0000 position=[50,50] matte=alpha-inv }
        }"#,
    )
    .unwrap();
}

#[test]
fn matte_luma_mode() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape src { ellipse{size=[80,80]} fill=#FFFFFF position=[50,50] matte-source=true }
            shape dst { rect{size=[80,80]} fill=#FF0000 position=[50,50] matte=luma }
        }"#,
    )
    .unwrap();
}

#[test]
fn matte_object_form_with_mode() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape src { ellipse{size=[80,80]} fill=#FFFFFF position=[50,50] matte-source=true }
            shape dst { rect{size=[80,80]} fill=#FF0000 position=[50,50] matte={ source=src, mode=luma-inv } }
        }"#,
    )
    .unwrap();
}

#[test]
fn mask_subtract_inverted() {
    let ir = compile(
        r#"composition "x" {
            width=200 height=200 fps=30 duration=1s
            shape s {
                rect { size=[150,150] }
                fill = #6366F1
                position = [100, 100]
                mask {
                    mode = subtract
                    inverted = true
                    path = "M-50 0 L0 -50 L50 0 L0 50"
                    opacity = 80
                    expand = 5
                }
            }
        }"#,
    )
    .unwrap();
    assert_eq!(ir.layers.len(), 1);
}

#[test]
fn mask_lighten_darken_difference_modes() {
    for mode in ["lighten", "darken", "difference"] {
        let src = format!(
            r#"composition "x" {{
                width=100 height=100 fps=30 duration=1s
                shape s {{
                    rect{{size=[60,60]}} fill=#000000 position=[50,50]
                    mask {{ mode = {mode}, path = "M-30 0 L0 -30 L30 0 L0 30", opacity = 100 }}
                }}
            }}"#
        );
        compile(&src).unwrap_or_else(|e| panic!("{mode}: {e}"));
    }
}

#[test]
fn pucker_object_form() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                polystar { points=5, outer-radius=30, inner-radius=15, type=star }
                fill = #FF00FF
                pucker = { amount = -25 }
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
    if let IrLayerKind::Shape(layer) = &ir.layers[0].kind {
        assert!(has_modifier(layer, |m| matches!(m, IrPathModifier::PuckerBloat { .. })));
    }
}

#[test]
fn twist_scalar_form_uses_origin() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s { rect{size=[40,40]} fill=#000000 twist = 30.0 position=[50,50] }
        }"#,
    )
    .unwrap();
}

#[test]
fn zigzag_smooth_type() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size=[60,60] } fill=#000000
                zigzag = { amplitude = 4, frequency = 6, type = smooth }
                position=[50,50]
            }
        }"#,
    )
    .unwrap();
}

