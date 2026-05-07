//! e2e_tests::layers

use crate::ir::IrLayerKind;

use super::compile;

// ============================================================
// Solid / Controller / Camera 层
// ============================================================

#[test]
fn solid_layer_compiles() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            solid bg {
                color = #FF0000
                size = [100, 100]
            }
        }"#,
    )
    .unwrap();
    assert_eq!(ir.layers.len(), 1);
    assert!(matches!(ir.layers[0].kind, IrLayerKind::Solid { .. }));
}

#[test]
fn controller_layer_compiles() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            controller anchor {
                position = [50, 50, 0]
            }
        }"#,
    )
    .unwrap();
    assert!(matches!(ir.layers[0].kind, IrLayerKind::Null));
}

#[test]
fn camera_layer_compiles() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            camera cam1 {
                position = [50, 50, -200]
                perspective = 500
            }
        }"#,
    )
    .unwrap();
    assert!(matches!(ir.layers[0].kind, IrLayerKind::Camera { .. }));
}

// ============================================================
// Text 层
// ============================================================

#[test]
fn text_layer_with_font_and_color() {
    let ir = compile(
        r#"composition "x" {
            width=200 height=100 fps=30 duration=1s
            text "Hello" {
                font = "Arial"
                weight = Bold
                size = 32
                color = #FFFFFF
                align = center
                tracking = 5
                line-height = 40
                position = [100, 50]
            }
        }"#,
    )
    .unwrap();
    assert_eq!(ir.layers.len(), 1);
    assert_eq!(ir.fonts.len(), 1);
    assert!(matches!(ir.layers[0].kind, IrLayerKind::Text(_)));
}

#[test]
fn text_layer_with_stroke() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=50 fps=30 duration=1s
            text "Outline" {
                font = "Arial"
                size = 24
                color = #000000
                stroke-color = #FF0000
                stroke-width = 2
                position = [50, 25]
            }
        }"#,
    )
    .unwrap();
    assert_eq!(ir.layers.len(), 1);
}

#[test]
fn text_align_modes() {
    for (mode, _) in [("left", 0), ("right", 1), ("center", 2)] {
        let src = format!(
            r#"composition "x" {{
                width=100 height=50 fps=30 duration=1s
                text "T" {{
                    font = "Arial"
                    size = 16
                    color = #000000
                    align = {mode}
                    position = [50, 25]
                }}
            }}"#
        );
        compile(&src).unwrap();
    }
}

