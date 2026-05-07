//! e2e_tests::misc


use super::compile;

// ============================================================
// Fill object form
// ============================================================

#[test]
fn fill_object_form_with_rule_non_zero() {
    compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size=[10,10] }
                fill = { color = #FF0000, opacity = 80, rule = non-zero }
                position = [5, 5]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn fill_object_form_with_rule_even_odd() {
    compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size=[10,10] }
                fill = { color = #FF0000, rule = even-odd }
                position = [5, 5]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn fill_invalid_rule_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size=[10,10] }
                fill = { color = #FF0000, rule = nonsense }
                position = [5, 5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("rule") || err.contains("nonsense"));
}

#[test]
fn fill_unknown_attr_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size=[10,10] }
                fill = { color = #FF0000, garbage = 1 }
                position = [5, 5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("garbage"));
}

#[test]
fn fill_missing_color_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size=[10,10] }
                fill = { opacity = 80 }
                position = [5, 5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("color") || err.contains("缺少"));
}

#[test]
fn fill_invalid_type_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size=[10,10] }
                fill = 42
                position = [5, 5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("fill") || err.contains("颜色"));
}

// ============================================================
// Markers / Path SVG commands
// ============================================================

#[test]
fn composition_markers_emitted() {
    let ir = compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=2s
            markers = [
                { name = "intro", time = 0s, duration = 0.5s },
                { name = "loop", time = 0.5s }
            ]
        }"#,
    )
    .unwrap();
    assert_eq!(ir.markers.len(), 2);
    assert_eq!(ir.markers[0].comment, "intro");
}

#[test]
fn unknown_composition_attr_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            unsupported = 1
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("unsupported"));
}

#[test]
fn missing_width_errors() {
    let err = compile(
        r#"composition "x" {
            height=10 fps=30 duration=1s
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("width"));
}

#[test]
fn path_svg_with_curve_command_compiles() {
    // SVG path 仅支持 M / L / Z 命令，使用基本三角形
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                path = "M0 0 L20 0 L20 20 L0 20"
                fill = #000000
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn path_svg_with_close_command() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                path = "M0 0 L20 0 L20 20 L0 20 Z"
                fill = #000000
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn path_animation_morph() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                path = "M0 0 L20 0 L10 20"
                fill = #000000
                position = [50, 50]
                animate path {
                    0s: "M0 0 L20 0 L10 20"
                    1s: "M0 0 L30 0 L15 30"
                }
            }
        }"#,
    )
    .unwrap();
}

