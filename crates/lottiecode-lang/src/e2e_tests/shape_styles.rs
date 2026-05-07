//! e2e_tests::shape_styles

use crate::ir::{IrLayerKind, IrStyle};

use super::{compile, has_style};

// ============================================================
// Shape: Stroke / Gradient / Modifiers
// ============================================================

#[test]
fn stroke_with_dash_pattern() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [50, 50] }
                stroke = {
                    color = #000000
                    width = 2
                    cap = round
                    join = miter
                    miter-limit = 4
                    dash = [5, 3, 2, 3]
                    dash-offset = 1
                }
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
    assert_eq!(ir.layers.len(), 1);
    if let IrLayerKind::Shape(layer) = &ir.layers[0].kind {
        assert!(has_style(layer, |s| matches!(s, IrStyle::Stroke { .. })));
    }
}

#[test]
fn linear_gradient_fill_compiles() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [80, 80] }
                fill = {
                    gradient = linear
                    start    = [-40, 0]
                    end      = [40, 0]
                    colors   = [#FF0000, #0000FF]
                }
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
    if let IrLayerKind::Shape(layer) = &ir.layers[0].kind {
        assert!(
            has_style(layer, |s| matches!(s, IrStyle::GradientFill { .. })),
            "expected GradientFill"
        );
    }
}

#[test]
fn radial_gradient_with_stops() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                ellipse { size = [80, 80] }
                fill = {
                    gradient = radial
                    start    = [0, 0]
                    end      = [40, 0]
                    colors   = [#FF0000, #FFFF00, #00FF00]
                }
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
    assert_eq!(ir.layers.len(), 1);
}

#[test]
fn gradient_stroke_compiles() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [80, 80] }
                stroke = {
                    gradient = linear
                    start    = [-40, 0]
                    end      = [40, 0]
                    colors   = [#FF0000, #0000FF]
                    width    = 4
                    cap      = round
                    join     = round
                }
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
    if let IrLayerKind::Shape(layer) = &ir.layers[0].kind {
        assert!(has_style(layer, |s| matches!(s, IrStyle::GradientStroke { .. })));
    }
}

#[test]
fn repeater_modifier() {
    compile(
        r#"composition "x" {
            width=200 height=100 fps=30 duration=1s
            shape s {
                rect { size = [10, 10] }
                fill = #000000
                repeater = {
                    copies = 8
                    offset = 0
                    position = [20, 0]
                    rotation = 15
                    scale = 95
                }
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn merge_modes_supported() {
    for mode in ["add", "subtract", "intersect", "exclude", "merge"] {
        let src = format!(
            r#"composition "x" {{
                width=100 height=100 fps=30 duration=1s
                shape s {{
                    rect {{ size = [40, 40] position = [-10, 0] }}
                    rect {{ size = [40, 40] position = [10, 0] }}
                    fill = #000000
                    merge = {mode}
                    position = [50, 50]
                }}
            }}"#
        );
        compile(&src).unwrap_or_else(|e| panic!("{mode}: {e}"));
    }
}

