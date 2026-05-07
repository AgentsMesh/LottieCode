//! e2e_tests::shape_modifiers


use super::compile;

#[test]
fn merge_invalid_mode_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size = [10, 10] }
                fill = #000000
                merge = nope
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("merge") || err.contains("nope"));
}

#[test]
fn offset_path_modifier() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #000000
                offset-path = {
                    amount = 10
                    join = round
                    miter-limit = 4
                }
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn pucker_bloat_modifier() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                polystar { points = 5, outer-radius = 30, inner-radius = 15, type = star }
                fill = #FF00FF
                pucker = -50
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn twist_modifier_with_object() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #000000
                twist = { angle = 45, center = [0, 0] }
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn twist_modifier_scalar_shorthand() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #000000
                twist = 30
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn zigzag_modifier() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [60, 60] }
                fill = #000000
                zigzag = { amplitude = 5, frequency = 8, type = corner }
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn rounded_corners_modifier() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #000000
                rounded-corners = 8
                position = [50, 50]
            }
        }"#,
    )
    .unwrap();
}

