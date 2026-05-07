//! e2e_tests::errors_styles


use super::compile;

// ============================================================
// Gradient 错误路径
// ============================================================

#[test]
fn gradient_unknown_kind_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect{size=[10,10]}
                fill = { gradient = spiral, start=[0,0], end=[10,0], colors=[#FF0000,#0000FF] }
                position=[5,5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("gradient") || err.contains("linear"));
}

#[test]
fn gradient_too_few_colors_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect{size=[10,10]}
                fill = { gradient = linear, start=[0,0], end=[10,0], colors=[#FF0000] }
                position=[5,5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("colors") || err.contains("2 个"));
}

// ============================================================
// Stroke 错误路径
// ============================================================

#[test]
fn stroke_non_object_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { rect{size=[10,10]} stroke = #FF0000 position=[5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("stroke"));
}

#[test]
fn stroke_unknown_attr_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect{size=[10,10]}
                stroke = { color=#000, width=2, garbage=1 }
                position=[5,5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("garbage"));
}

#[test]
fn stroke_missing_color_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect{size=[10,10]}
                stroke = { width = 2 }
                position=[5,5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("stroke") || err.contains("color"));
}

// ============================================================
// Effects 错误路径
// ============================================================

#[test]
fn shadow_non_array_object_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { rect{size=[10,10]} fill=#000 shadow = 5 position=[5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("shadow"));
}

#[test]
fn shadow_array_too_short_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { rect{size=[10,10]} fill=#000 shadow = [4, 4] position=[5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("shadow") || err.contains("blur"));
}

#[test]
fn blur_non_number_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { rect{size=[10,10]} fill=#000 blur = "x" position=[5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("blur") || err.contains("数字"));
}

// ============================================================
