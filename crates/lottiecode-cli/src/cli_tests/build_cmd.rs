//! cli_tests::build_cmd

use std::fs;
use std::path::PathBuf;

use super::{with_tmpdir, SAMPLE_DSL};

// ============================================================
// build —— JSON / compact / .lottie
// ============================================================

#[test]
fn build_emits_pretty_json() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        let dst = dir.join("a.json");
        fs::write(&src, SAMPLE_DSL).unwrap();
        crate::commands::build::run(&src, Some(&dst), false).unwrap();
        let out = fs::read_to_string(&dst).unwrap();
        assert!(out.contains("\n  \""), "pretty 应有缩进");
        assert!(out.contains("\"v\""));
    });
}

#[test]
fn build_compact_has_no_indentation() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        let dst = dir.join("a.json");
        fs::write(&src, SAMPLE_DSL).unwrap();
        crate::commands::build::run(&src, Some(&dst), true).unwrap();
        let out = fs::read_to_string(&dst).unwrap();
        assert!(!out.contains("\n  \""), "compact 不应有缩进");
    });
}

#[test]
fn build_default_output_path_uses_json_extension() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(&src, SAMPLE_DSL).unwrap();
        crate::commands::build::run(&src, None, false).unwrap();
        assert!(dir.join("a.json").exists());
    });
}

#[test]
fn build_dotlottie_zip_when_extension_is_lottie() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        let dst = dir.join("a.lottie");
        fs::write(&src, SAMPLE_DSL).unwrap();
        crate::commands::build::run(&src, Some(&dst), false).unwrap();
        let bytes = fs::read(&dst).unwrap();
        assert!(bytes.starts_with(b"PK"), "应为 zip 格式");
    });
}

#[test]
fn build_propagates_compile_error() {
    with_tmpdir(|dir| {
        let src = dir.join("bad.lc");
        fs::write(&src, "composition \"x\" { @ }").unwrap();
        let err = crate::commands::build::run(&src, None, false).unwrap_err();
        assert!(err.contains("error") || err.contains("意外"));
    });
}

