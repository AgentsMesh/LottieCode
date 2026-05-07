//! cli_tests::inspect_plan_syntax

use std::fs;
use std::path::PathBuf;

use super::{with_tmpdir, SAMPLE_DSL};

// ============================================================
// inspect / plan / syntax
// ============================================================

#[test]
fn inspect_default_uses_debug_format() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(&src, SAMPLE_DSL).unwrap();
        crate::commands::inspect::run(&src, false).unwrap();
    });
}

#[test]
fn inspect_json_mode_runs_codegen() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(&src, SAMPLE_DSL).unwrap();
        crate::commands::inspect::run(&src, true).unwrap();
    });
}

#[test]
fn inspect_propagates_compile_error() {
    let err = crate::commands::inspect::run(&PathBuf::from("/no.lc"), false).unwrap_err();
    assert!(err.contains("无法读取") || err.contains("No such"));
}

#[test]
fn plan_prints_layer_tree() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        let multi_layer = r#"composition "demo" {
            width=200 height=200 fps=30 duration=2s
            shape rect-layer {
                rect { size=[40, 40] }
                fill = #FF0000
                position = [50, 50]
            }
            shape ellipse-layer {
                ellipse { size=[40, 40] }
                fill = #00FF00
                position = [150, 50]
            }
            shape star-layer {
                polystar { points = 5, outer-radius = 20, inner-radius = 8, type = star }
                fill = #FFD700
                position = [100, 150]
            }
        }"#;
        fs::write(&src, multi_layer).unwrap();
        crate::commands::plan::run(&src).unwrap();
    });
}

#[test]
fn plan_handles_text_layer() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(
            &src,
            r#"composition "x" {
                width=200 height=100 fps=30 duration=1s
                text "Hi" { font="Arial" size=24 color=#FFFFFF position=[100,50] }
            }"#,
        )
        .unwrap();
        crate::commands::plan::run(&src).unwrap();
    });
}

#[test]
fn plan_handles_solid_and_null_layers() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(
            &src,
            r#"composition "x" {
                width=100 height=100 fps=30 duration=1s
                solid bg { color=#FF0000 size=[100,100] }
                controller anchor { position=[50,50,0] }
                camera cam { position=[50,50,-200] perspective=500 }
            }"#,
        )
        .unwrap();
        crate::commands::plan::run(&src).unwrap();
    });
}

#[test]
fn plan_handles_image_and_precomp_layers() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(
            &src,
            r#"
            asset logo { image="x.png" width=10 height=10 }
            precomp card {
                width=50 height=50 fps=30 duration=1s
                shape s { rect{size=[50,50]} fill=#000000 position=[25,25] }
            }
            composition "x" {
                width=200 height=200 fps=30 duration=1s
                image i { asset=logo position=[100, 100] }
                precomp p { asset=card position=[50, 50] }
            }
            "#,
        )
        .unwrap();
        crate::commands::plan::run(&src).unwrap();
    });
}

#[test]
fn syntax_runs_without_error() {
    crate::commands::syntax::run().unwrap();
}

#[test]
fn plan_handles_camera_layer() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(
            &src,
            r#"composition "x" {
                width=200 height=200 fps=30 duration=1s
                camera cam { position=[100, 100, -200] perspective=500 }
                shape s { rect{size=[40,40]} fill=#FF0000 position=[100,100] }
            }"#,
        )
        .unwrap();
        crate::commands::plan::run(&src).unwrap();
    });
}

#[test]
fn plan_handles_groups_in_shape() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(
            &src,
            r#"composition "x" {
                width=100 height=100 fps=30 duration=1s
                shape s {
                    polystar { points=5, outer-radius=30, inner-radius=15, type=star }
                    fill=#FFD700
                    stroke = { color=#000000 width=2 cap=round join=round }
                    position=[50,50]
                    repeater = { copies=4, offset=0, rotation=90, position=[0,0], scale=80 }
                }
            }"#,
        )
        .unwrap();
        crate::commands::plan::run(&src).unwrap();
    });
}

#[test]
fn inspect_text_layer() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(
            &src,
            r#"composition "x" {
                width=100 height=100 fps=30 duration=1s
                text "Hi" { font="Arial" size=18 color=#000000 position=[50,50] }
            }"#,
        )
        .unwrap();
        crate::commands::inspect::run(&src, true).unwrap();
    });
}
