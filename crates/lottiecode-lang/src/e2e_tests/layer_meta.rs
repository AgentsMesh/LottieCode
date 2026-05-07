//! e2e_tests::layer_meta


use super::compile;

// ============================================================
// 转换：parent / matte / blend-mode / 3D
// ============================================================

#[test]
fn parent_layer_resolution() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            controller anchor { position = [50, 50, 0] }
            shape s {
                rect { size = [20, 20] }
                fill = #FF0000
                position = [0, 0]
                parent = anchor
            }
        }"#,
    )
    .unwrap();
    assert_eq!(ir.layers[1].parent.as_deref(), Some("anchor"));
}

#[test]
fn blend_modes_supported() {
    for mode in ["normal", "multiply", "screen", "overlay", "lighten", "darken"] {
        let src = format!(
            r#"composition "x" {{
                width=100 height=100 fps=30 duration=1s
                shape s {{
                    rect {{ size = [40, 40] }}
                    fill = #FF0000
                    position = [50, 50]
                    blend = {mode}
                }}
            }}"#
        );
        compile(&src).unwrap_or_else(|e| panic!("{mode}: {e}"));
    }
}

#[test]
fn matte_modes_supported() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape mask-source {
                ellipse { size = [80, 80] }
                fill = #FFFFFF
                position = [50, 50]
                matte-source = true
            }
            shape body {
                rect { size = [80, 80] }
                fill = #FF0000
                position = [50, 50]
                matte = alpha
            }
        }"#,
    )
    .unwrap();
    assert_eq!(ir.layers.len(), 2);
}

// ============================================================
// 时间字面量与 token slots
// ============================================================

#[test]
fn slots_resolution() {
    let ir = compile(
        r#"
        slots {
            primary = #6366F1
        }
        composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = slot.primary
                position = [50, 50]
            }
        }
        "#,
    )
    .unwrap();
    assert_eq!(ir.slots.len(), 1);
}

#[test]
fn duration_in_milliseconds() {
    let ir = compile(
        r#"composition "x" {
            width=10 height=10 fps=60 duration=500ms
        }"#,
    )
    .unwrap();
    assert_eq!(ir.out_frame, 30.0);
}

#[test]
fn duration_in_frames() {
    let ir = compile(
        r#"composition "x" {
            width=10 height=10 fps=60 duration=120f
        }"#,
    )
    .unwrap();
    assert_eq!(ir.out_frame, 120.0);
}

