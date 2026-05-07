//! e2e_tests::geom_grad

use super::{build, compile, shape_items};

// ============================================================
// Shape geometry (rc/el/sr/sh)
// ============================================================

#[test]
fn polystar_emits_sr() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                polystar { points = 5, outer-radius = 30, inner-radius = 12, type = star }
                fill = #FFD700
                position = [50, 50]
            }
        }"#,
    );
    let items = shape_items(&json, 0);
    assert!(items.iter().any(|x| x["ty"] == "sr"));
}

#[test]
fn path_emits_sh_with_bezier() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                path = "M0 0 L20 0 L10 20"
                stroke = { color = #000000 width = 2 cap = round join = round }
                position = [50, 50]
            }
        }"#,
    );
    let items = shape_items(&json, 0);
    let path = items.iter().find(|x| x["ty"] == "sh").expect("path emitted");
    assert!(path["ks"]["k"]["v"].is_array());
}

// ============================================================
// Gradient fill / stroke (gf / gs)
// ============================================================

#[test]
fn linear_gradient_fill_emits_gf() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [60, 60] }
                fill = {
                    gradient = linear
                    start    = [-30, 0]
                    end      = [30, 0]
                    colors   = [#FF0000, #0000FF]
                }
                position = [50, 50]
            }
        }"#,
    );
    let items = shape_items(&json, 0);
    let gf = items.iter().find(|x| x["ty"] == "gf").expect("gf");
    assert_eq!(gf["t"], 1);
}

#[test]
fn radial_gradient_fill_t_is_2() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                ellipse { size = [60, 60] }
                fill = {
                    gradient = radial
                    start    = [0, 0]
                    end      = [30, 0]
                    colors   = [#FF0000, #FFFF00, #0000FF]
                }
                position = [50, 50]
            }
        }"#,
    );
    let items = shape_items(&json, 0);
    let gf = items.iter().find(|x| x["ty"] == "gf").expect("gf");
    assert_eq!(gf["t"], 2);
}

#[test]
fn gradient_stroke_emits_gs() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [60, 60] }
                stroke = {
                    gradient = linear
                    start    = [-30, 0]
                    end      = [30, 0]
                    colors   = [#FF0000, #0000FF]
                    width    = 4
                    cap      = round
                    join     = round
                }
                position = [50, 50]
            }
        }"#,
    );
    let items = shape_items(&json, 0);
    let gs = items.iter().find(|x| x["ty"] == "gs").expect("gs");
    assert_eq!(gs["t"], 1);
    assert_eq!(gs["lc"], 2);
}

