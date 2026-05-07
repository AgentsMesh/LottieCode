//! e2e_tests::mask_effect

use super::{build, compile, shape_items};

// ============================================================
// Effects (DropShadow / GaussianBlur)
// ============================================================

#[test]
fn drop_shadow_emits_ef_ty25() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #FFFFFF
                shadow = [4, 4, 8, #00000080]
                position = [50, 50]
            }
        }"#,
    );
    let effects = json["layers"][0]["ef"].as_array().unwrap();
    assert_eq!(effects[0]["ty"], 25);
}

#[test]
fn gaussian_blur_emits_ef_ty29() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #FFFFFF
                blur = 8
                position = [50, 50]
            }
        }"#,
    );
    let effects = json["layers"][0]["ef"].as_array().unwrap();
    assert_eq!(effects[0]["ty"], 29);
}

// ============================================================
// Parent / blend mode / matte
// ============================================================

#[test]
fn parent_field_resolves_to_index() {
    let json = build(
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
    );
    assert_eq!(json["layers"][1]["parent"], 1);
}

#[test]
fn blend_mode_field() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #FF0000
                position = [50, 50]
                blend = multiply
            }
        }"#,
    );
    assert_eq!(json["layers"][0]["bm"], 1);
}

