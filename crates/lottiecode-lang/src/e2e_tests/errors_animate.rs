//! e2e_tests::errors_animate


use super::compile;

// resolve_animate conv_* 错误
// ============================================================

#[test]
fn animate_color_invalid_value_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size=[10,10] }
                position = [5, 5]
                fill = { color = #000000, opacity = 100 }
                stroke = { color = #000000, width = 1 }
                animate fill-color {
                    0s: 5
                    1s: 10
                }
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("颜色") || err.contains("color"));
}

#[test]
fn animate_position_invalid_value_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size=[10,10] }
                fill = #000000
                position = [5, 5]
                animate position {
                    0s: "x"
                    1s: "y"
                }
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("position") || err.contains("数字") || err.contains("数组"));
}

// ============================================================
