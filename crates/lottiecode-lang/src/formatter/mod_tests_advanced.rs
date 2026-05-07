//! formatter::mod_tests::advanced

use crate::ast::Program;
use crate::formatter::format;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn parse(src: &str) -> Program {
    let mut l = Lexer::new(src);
    let toks = l.tokenize().unwrap();
    Parser::new(toks).parse().unwrap()
}


#[test]
fn format_polystar_and_path_geometry() {
    let p = parse(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s1 {
                polystar { points = 5, outer-radius = 30, inner-radius = 12, type = star }
                fill = #FFD700
                position = [50, 50]
            }
            shape s2 {
                path = "M0 0 L20 0 L10 20"
                stroke = { color = #000000 width = 2 cap = round join = round }
                position = [50, 50]
            }
        }"#,
    );
    let out = format(&p);
    assert!(out.contains("polystar"));
    assert!(out.contains("path"));
}

#[test]
fn format_includes_token_groups_round_trip() {
    let src = r#"
include "a.lc"

token color {
brand = #FF0000
}

composition "x" {
width = 10
height = 10
fps = 30
duration = 1s
}
"#;
    let p1 = parse(src);
    let formatted = format(&p1);
    let p2 = parse(&formatted);
    assert_eq!(p1.tokens.len(), p2.tokens.len());
    assert_eq!(p1.includes.len(), p2.includes.len());
}

#[test]
fn format_keyframe_with_named_easing_and_hold() {
    let p = parse(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect { size = [40, 40] }
                fill = #000000
                position = [50, 50]
                animate scale {
                    0s:   [0, 0, 100]
                    0.3s: [0, 0, 100]      hold
                    0.5s: [120, 120, 100]  ease = ease-out-back
                    1s:   [100, 100, 100]
                }
            }
        }"#,
    );
    let out = format(&p);
    assert!(out.contains("ease = ease-out-back"));
    assert!(out.contains("hold"));
}

#[test]
fn format_keyframe_with_cubic_bezier() {
    let p = parse(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size = [10, 10] } fill = #000000 position = [5, 5]
                animate opacity {
                    0s: 0
                    1s: 100  ease = cubic(0.4, 0, 0.2, 1)
                }
            }
        }"#,
    );
    let out = format(&p);
    assert!(out.contains("cubic("));
}

#[test]
fn format_trim_block() {
    let p = parse(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                ellipse { size = [80, 80] }
                stroke = { color = #000000 width = 2 cap = round join = round }
                position = [50, 50]
                trim {
                    start = 0
                    end = 50
                }
            }
        }"#,
    );
    let out = format(&p);
    assert!(out.contains("trim {"));
}
