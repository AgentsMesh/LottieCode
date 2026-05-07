use super::*;
use std::fs;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn tmp_dir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let p = std::env::temp_dir().join(format!(
        "lc-incl-{}-{}-{}",
        std::process::id(),
        label,
        id
    ));
    fs::create_dir_all(&p).unwrap();
    p
}

fn parse(src: &str) -> Program {
    let mut l = Lexer::new(src);
    let toks = l.tokenize().unwrap();
    Parser::new(toks).parse().unwrap()
}

#[test]
fn include_merges_tokens_and_components() {
    let dir = tmp_dir("merge");
    fs::write(
        dir.join("tokens.lc"),
        r#"token color { brand = #FF0000 }"#,
    )
    .unwrap();
    fs::write(
        dir.join("comps.lc"),
        r#"component dot() {
            shape s { ellipse { size = [10, 10] } fill = #000000 position = [0, 0] }
        }"#,
    )
    .unwrap();
    let main = format!(
        r#"include "tokens.lc"
        include "comps.lc"
        composition "x" {{ width=10 height=10 fps=30 duration=1s }}"#
    );
    let mut prog = parse(&main);
    load_includes(&mut prog, &dir).unwrap();
    assert_eq!(prog.tokens.len(), 1);
    assert_eq!(prog.components.len(), 1);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn include_dedupes_via_canonicalize() {
    let dir = tmp_dir("dedupe");
    fs::write(dir.join("a.lc"), r#"token color { brand = #FF0000 }"#).unwrap();
    let main = r#"include "a.lc"
    include "a.lc"
    composition "x" { width=10 height=10 fps=30 duration=1s }"#;
    let mut prog = parse(main);
    load_includes(&mut prog, &dir).unwrap();
    assert_eq!(prog.tokens.len(), 1);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn include_recursive_chain() {
    let dir = tmp_dir("chain");
    fs::write(dir.join("base.lc"), r#"token base { x = 1 }"#).unwrap();
    fs::write(
        dir.join("mid.lc"),
        r#"include "base.lc"
        token mid { y = 2 }"#,
    )
    .unwrap();
    let main = r#"include "mid.lc"
    composition "x" { width=10 height=10 fps=30 duration=1s }"#;
    let mut prog = parse(main);
    load_includes(&mut prog, &dir).unwrap();
    assert_eq!(prog.tokens.len(), 2);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn include_missing_file_errors() {
    let dir = tmp_dir("missing");
    let main = r#"include "no-such.lc"
    composition "x" { width=10 height=10 fps=30 duration=1s }"#;
    let mut prog = parse(main);
    let err = load_includes(&mut prog, &dir).unwrap_err();
    assert_eq!(err.kind, ErrorKind::IncludeFileNotFound);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn included_composition_is_ignored() {
    let dir = tmp_dir("ignored");
    fs::write(
        dir.join("other.lc"),
        r#"composition "ignored" { width=999 height=999 fps=30 duration=1s }"#,
    )
    .unwrap();
    let main = r#"include "other.lc"
    composition "main" { width=100 height=100 fps=30 duration=1s }"#;
    let mut prog = parse(main);
    load_includes(&mut prog, &dir).unwrap();
    assert_eq!(prog.composition.unwrap().name, "main");
    let _ = fs::remove_dir_all(&dir);
}
