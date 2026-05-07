//! cli_tests::fmt_cmd

use std::fs;
use std::path::PathBuf;

use super::{with_tmpdir, SAMPLE_DSL};

// ============================================================
// fmt
// ============================================================

#[test]
fn fmt_to_stdout_does_not_modify_file() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(&src, SAMPLE_DSL).unwrap();
        let before = fs::read_to_string(&src).unwrap();
        crate::commands::fmt::run(&src, false).unwrap();
        let after = fs::read_to_string(&src).unwrap();
        assert_eq!(before, after);
    });
}

#[test]
fn fmt_write_replaces_file_with_canonical() {
    with_tmpdir(|dir| {
        let src = dir.join("a.lc");
        fs::write(&src, "composition \"d\"{width=10 height=10}").unwrap();
        crate::commands::fmt::run(&src, true).unwrap();
        let after = fs::read_to_string(&src).unwrap();
        // 规范化后应有缩进与换行
        assert!(after.contains("    "));
        assert!(after.contains("\n"));
    });
}

#[test]
fn fmt_propagates_parse_error() {
    with_tmpdir(|dir| {
        let src = dir.join("bad.lc");
        fs::write(&src, "this is not valid").unwrap();
        let err = crate::commands::fmt::run(&src, false).unwrap_err();
        assert!(!err.is_empty());
    });
}

