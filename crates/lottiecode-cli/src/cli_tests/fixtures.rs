//! Fixture 回归测试：每个 example 编译后与 `expected.json` 做结构等价比对。
//!
//! 防御目标：字段顺序漂移、字段类型回退（如 enum → 整数→ 浮点）、
//! 默认值省略策略变化。任何会导致 player 行为差异的变更都应被这层捕获。

use std::path::PathBuf;

use lottiecode_codegen::fixture_compare::compare;

const EXAMPLES: &[&str] = &[
    "01-check",
    "02-loader",
    "04-text-banner",
    "05-gradient-card",
    "06-stars",
    "07-mask-reveal",
    "08-precomp-card",
    "09-typewriter",
    "10-path-morph",
    "11-shadow-blur",
    "12-deformers",
    "13-repeater-fan",
    "14-gaming-restore",
];

fn examples_root() -> PathBuf {
    // Bazel test：runfiles 暴露 examples 在 $TEST_SRCDIR/_main/examples
    if let Ok(srcdir) = std::env::var("TEST_SRCDIR") {
        let p = PathBuf::from(srcdir).join("_main").join("examples");
        if p.is_dir() { return p; }
    }
    // bazel run 或本地：BUILD_WORKSPACE_DIRECTORY 指向 repo 根
    if let Ok(ws) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
        let p = PathBuf::from(ws).join("examples");
        if p.is_dir() { return p; }
    }
    // cargo test fallback
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    here.ancestors()
        .find(|p| p.join("examples").is_dir())
        .map(|p| p.join("examples"))
        .expect("找不到 examples 目录")
}

fn compile_to_json(lc_path: &std::path::Path) -> serde_json::Value {
    let ir = crate::pipeline::compile_file(lc_path).expect("编译失败");
    lottiecode_codegen::generate(&ir)
}

#[test]
fn all_examples_match_fixtures() {
    let examples_dir = examples_root();

    let mut failures = Vec::new();
    for example in EXAMPLES {
        let lc_path = examples_dir.join(example).join("main.lc");
        let expected_path = examples_dir.join(example).join("expected.json");
        if !lc_path.exists() || !expected_path.exists() {
            failures.push(format!("{example}: 缺少 main.lc 或 expected.json"));
            continue;
        }
        let actual = compile_to_json(&lc_path);
        let expected_raw = std::fs::read_to_string(&expected_path).unwrap();
        let expected: serde_json::Value = serde_json::from_str(&expected_raw).unwrap();
        if let Some(m) = compare(&actual, &expected) {
            failures.push(format!("{example}: {m}"));
        }
    }
    assert!(failures.is_empty(), "fixture 不匹配:\n  {}", failures.join("\n  "));
}
