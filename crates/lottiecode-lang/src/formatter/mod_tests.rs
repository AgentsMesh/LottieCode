use super::*;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn parse(src: &str) -> Program {
    let mut l = Lexer::new(src);
    let toks = l.tokenize().unwrap();
    Parser::new(toks).parse().unwrap()
}

#[test]
fn format_minimal_composition() {
    let p = parse(r#"composition "demo" { width=100 height=100 }"#);
    let out = format(&p);
    assert!(out.contains("composition \"demo\""));
    assert!(out.contains("width = 100"));
}

#[test]
fn format_round_trip_preserves_semantics() {
    let src = r#"
token color { brand = #FF0000 }
composition "x" {
width = 100
height = 100
fps = 30
duration = 1s
shape s {
    rect { size = [10, 10] }
    fill = color.brand
}
}
"#;
    let p1 = parse(src);
    let formatted = format(&p1);
    let p2 = parse(&formatted);
    assert_eq!(
        p1.composition.as_ref().unwrap().name,
        p2.composition.as_ref().unwrap().name
    );
    assert_eq!(p1.tokens.len(), p2.tokens.len());
}

#[test]
fn format_includes_emit_first() {
    let p = parse(r#"include "tokens.lc" composition "x" {}"#);
    let out = format(&p);
    let inc_pos = out.find("include").unwrap();
    let comp_pos = out.find("composition").unwrap();
    assert!(inc_pos < comp_pos);
}

#[test]
fn empty_program_formats_empty() {
    let p = Program::default();
    assert_eq!(format(&p), "");
}

#[test]
fn format_text_layer() {
    let p = parse(
        r#"composition "x" {
            width=200 height=100 fps=30 duration=1s
            text "Hi" {
                font = "Arial"
                size = 24
                color = #FFFFFF
                position = [100, 50]
                animate opacity { 0s: 0  1s: 100 }
            }
        }"#,
    );
    let out = format(&p);
    assert!(out.contains("text \"Hi\""));
    assert!(out.contains("animate opacity"));
}

#[test]
fn format_image_layer() {
    let p = parse(
        r#"
        asset logo { image = "logo.png" width = 10 height = 10 }
        composition "x" {
            width=200 height=200 fps=30 duration=1s
            image foo { asset = logo position = [100, 100] }
        }"#,
    );
    let out = format(&p);
    assert!(out.contains("image foo"));
}

#[test]
fn format_solid_layer() {
    let p = parse(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            solid bg { color = #FF0000 size = [100, 100] }
        }"#,
    );
    let out = format(&p);
    assert!(out.contains("solid bg"));
}

#[test]
fn format_controller_camera_use() {
    let p = parse(
        r#"
        component dot(x = 0) {
            shape s { ellipse { size = [10, 10] } fill = #000000 position = [x, 0] }
        }
        composition "x" {
            width=100 height=100 fps=30 duration=1s
            controller anchor { position = [50, 50, 0] }
            camera cam { position = [50, 50, -200] }
            use dot(x = 30)
        }"#,
    );
    let out = format(&p);
    assert!(out.contains("controller anchor"));
    assert!(out.contains("camera cam"));
    assert!(out.contains("use dot"));
    assert!(out.contains("component dot"));
}

#[path = "mod_tests_advanced.rs"]
mod advanced;
