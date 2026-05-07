//! e2e_tests::errors_basic


use super::compile;

// ============================================================
// 错误：keyframe 类型 / token 引用
// ============================================================

#[test]
fn animate_unknown_property_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size=[10,10] } fill=#000000 position=[5,5]
                animate weird-prop { 0s: 0  1s: 100 }
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("weird-prop") || err.contains("不支持"));
}

#[test]
fn duplicate_composition_at_top_level_errors() {
    let err = compile(
        r#"composition "a" { width=10 height=10 fps=30 duration=1s }
        composition "b" { width=10 height=10 fps=30 duration=1s }"#,
    )
    .unwrap_err();
    assert!(err.contains("composition") || err.contains("重复"));
}

#[test]
fn duplicate_component_errors() {
    let err = compile(
        r#"
        component dup() { shape s { rect{size=[10,10]} fill=#000 position=[0,0] } }
        component dup() { shape s { rect{size=[10,10]} fill=#000 position=[0,0] } }
        composition "x" { width=10 height=10 fps=30 duration=1s }
        "#,
    )
    .unwrap_err();
    assert!(err.contains("dup") || err.contains("重复"));
}

// ============================================================
// Modifier 错误路径
// ============================================================

#[test]
fn repeater_non_object_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { rect { size=[10,10] } fill=#000 repeater = 5 position=[5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("repeater"));
}

#[test]
fn repeater_unknown_attr_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect { size=[10,10] } fill=#000000
                repeater = { copies=2, offset=0, garbage=1 }
                position=[5,5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("repeater") && (err.contains("garbage") || err.contains("不支持")));
}

#[test]
fn merge_non_ident_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { rect{size=[10,10]} fill=#000 merge = 42 position=[5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("merge"));
}

#[test]
fn offset_path_non_object_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { rect{size=[10,10]} fill=#000 offset-path = 5 position=[5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("offset-path") || err.contains("offset"));
}

#[test]
fn offset_path_unknown_attr_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                rect{size=[10,10]} fill=#000000
                offset-path = { amount=5, garbage=1 }
                position=[5,5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("不支持"));
}

#[test]
fn pucker_invalid_type_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { polystar { points=5, outer-radius=10, inner-radius=5, type=star } fill=#000 pucker = "x" position=[5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("pucker"));
}

#[test]
fn twist_invalid_type_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { rect{size=[10,10]} fill=#000 twist = "x" position=[5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("twist"));
}

#[test]
fn zigzag_non_object_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { rect{size=[10,10]} fill=#000 zigzag = 5 position=[5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("zigzag"));
}

