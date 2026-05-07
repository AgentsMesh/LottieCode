//! e2e_tests::modifiers

use super::{build, compile, shape_items};

// ============================================================
// Modifier dispatch (rd / rp / mm / op / pb / tw / zz)
// ============================================================

#[test]
fn rounded_corners_emits_rd() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #000000
                rounded-corners = 8
                position = [50, 50]
            }
        }"#,
    );
    assert!(shape_items(&json, 0).iter().any(|x| x["ty"] == "rd"));
}

#[test]
fn repeater_emits_rp() {
    let json = build(
        r#"composition "x" {
            width=200 height=100 fps=30 duration=1s
            shape s {
                rect { size = [10, 10] }
                fill = #000000
                repeater = { copies = 5, offset = 0, position = [20, 0], rotation = 0, scale = 100 }
                position = [50, 50]
            }
        }"#,
    );
    let items = shape_items(&json, 0);
    let rp = items.iter().find(|x| x["ty"] == "rp").expect("rp");
    assert_eq!(rp["c"]["k"], 5.0);
}

#[test]
fn merge_emits_mm() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] position = [-10, 0] }
                rect { size = [40, 40] position = [10, 0] }
                fill = #000000
                merge = subtract
                position = [50, 50]
            }
        }"#,
    );
    let items = shape_items(&json, 0);
    let mm = items.iter().find(|x| x["ty"] == "mm").expect("mm");
    assert_eq!(mm["mm"], 3);
}

#[test]
fn offset_path_emits_op() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #000000
                offset-path = { amount = 10, join = round, miter-limit = 4 }
                position = [50, 50]
            }
        }"#,
    );
    assert!(shape_items(&json, 0).iter().any(|x| x["ty"] == "op"));
}

#[test]
fn pucker_emits_pb() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                polystar { points = 5, outer-radius = 30, inner-radius = 15, type = star }
                fill = #FF00FF
                pucker = -50
                position = [50, 50]
            }
        }"#,
    );
    assert!(shape_items(&json, 0).iter().any(|x| x["ty"] == "pb"));
}

#[test]
fn twist_emits_tw() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #000000
                twist = { angle = 45, center = [0, 0] }
                position = [50, 50]
            }
        }"#,
    );
    assert!(shape_items(&json, 0).iter().any(|x| x["ty"] == "tw"));
}

#[test]
fn zigzag_emits_zz() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [60, 60] }
                fill = #000000
                zigzag = { amplitude = 5, frequency = 8, type = corner }
                position = [50, 50]
            }
        }"#,
    );
    assert!(shape_items(&json, 0).iter().any(|x| x["ty"] == "zz"));
}

