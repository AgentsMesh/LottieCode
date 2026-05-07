//! CLI 各 subcommand 的集成测试 —— 用临时文件触发 run() 函数。

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// 在 std::env::temp_dir() 下生成唯一目录并回调；测试结束自动清理。
pub(super) fn with_tmpdir<F: FnOnce(&PathBuf)>(f: F) {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let path = std::env::temp_dir().join(format!(
        "lottiecode-test-{}-{}",
        std::process::id(),
        id
    ));
    fs::create_dir_all(&path).unwrap();
    f(&path);
    let _ = fs::remove_dir_all(&path);
}

pub(super) const SAMPLE_DSL: &str = r#"composition "demo" {
    width=100 height=100 fps=30 duration=1s
    shape s {
        rect { size=[40, 40] }
        fill = #FF0000
        position = [50, 50]
    }
}"#;

// ============================================================
// check
// ============================================================

#[test]
fn check_succeeds_on_valid_file() {
    with_tmpdir(|dir| {
        let path = dir.join("ok.lc");
        fs::write(&path, SAMPLE_DSL).unwrap();
        crate::commands::check::run(&path).unwrap();
    });
}

#[test]
fn check_reports_lex_error() {
    with_tmpdir(|dir| {
        let path = dir.join("bad.lc");
        fs::write(&path, "composition \"x\" { @ }").unwrap();
        let err = crate::commands::check::run(&path).unwrap_err();
        assert!(err.contains("error") || err.contains("意外") || err.contains("UnexpectedChar"));
    });
}

#[test]
fn check_missing_file_errors() {
    let err = crate::commands::check::run(&PathBuf::from("/nonexistent/file.lc")).unwrap_err();
    assert!(err.contains("无法读取") || err.contains("No such"));
}


mod build_cmd;
mod fixtures;
mod fmt_cmd;
mod inspect_plan_syntax;
mod roundtrip;
