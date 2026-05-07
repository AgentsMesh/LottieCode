//! e2e_tests::effects_trim


use super::compile;

// ============================================================
// Mask / Effects
// ============================================================

#[test]
fn shape_mask_with_path() {
    compile(
        r#"composition "x" {
            width=200 height=200 fps=30 duration=1s
            shape s {
                rect { size = [150, 150] }
                fill = #6366F1
                position = [100, 100]
                mask {
                    mode = add
                    path = "M-50 0 L0 -50 L50 0 L0 50"
                    opacity = 100
                }
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn drop_shadow_effect() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #FFFFFF
                shadow = [4, 4, 8, #00000080]
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn drop_shadow_effect_object_form() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #FFFFFF
                shadow = {
                    color = #000000
                    opacity = 50
                    direction = 135
                    distance = 4
                    softness = 8
                }
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn gaussian_blur_effect() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #FFFFFF
                blur = 12
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

// ============================================================
// Trim Path
// ============================================================

#[test]
fn trim_block_with_static_values() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                ellipse { size = [80, 80] }
                stroke = { color = #000000 width = 2 cap = round join = round }
                position = [50, 50]
                trim {
                    start = 0
                    end = 50
                    offset = 0
                }
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn trim_animation_via_animate_block() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                ellipse { size = [80, 80] }
                stroke = { color = #000000 width = 2 cap = round join = round }
                position = [50, 50]
                animate trim {
                    0s: { start = 0, end = 0,   offset = 0 }
                    1s: { start = 0, end = 100, offset = 0 } ease = ease-out
                }
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn trim_end_animate_shorthand() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                stroke = { color = #000000 width = 2 cap = round join = round }
                position = [50, 50]
                animate trim-end {
                    0s: 0
                    1s: 100  ease = ease-out
                }
            }
        }"#,
    )
    .unwrap();
}

