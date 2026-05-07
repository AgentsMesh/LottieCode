//! e2e_tests::bezier_literal —— path Bezier 字面量解析。

use crate::ir::{IrGeometry, IrLayerKind};

use super::compile;

#[test]
fn path_bezier_literal_with_v_in_out() {
    let ir = compile(
        r#"composition "x" {
            width=200 height=200 fps=30 duration=1s
            shape s {
                path {
                    v = [[-50, 0], [50, 0], [0, 50]]
                    in = [[0, 0], [10, -5], [-10, 5]]
                    out = [[10, -5], [-10, -5], [0, 0]]
                    closed = true
                }
                fill = #FF0000
                position = [100, 100]
            }
        }"#,
    )
    .unwrap();
    if let IrLayerKind::Shape(layer) = &ir.layers[0].kind {
        let path = layer
            .groups
            .iter()
            .flat_map(|g| g.geometries.iter())
            .find(|g| matches!(g, IrGeometry::Path { .. }));
        if let Some(IrGeometry::Path { bezier, .. }) = path {
            if let crate::ir::AnimatableValue::Static(b) = bezier {
                assert_eq!(b.vertices.len(), 3);
                assert_eq!(b.vertices[0], [-50.0, 0.0]);
                assert_eq!(b.in_tangents[1], [10.0, -5.0]);
                assert!(b.closed);
            } else {
                panic!("expected Static bezier");
            }
        } else {
            panic!("path not found");
        }
    }
}

#[test]
fn path_bezier_default_tangents_zero() {
    compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                path {
                    v = [[0, 0], [10, 0], [10, 10]]
                    closed = false
                }
                fill = #000000
                position = [5, 5]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn path_bezier_v_alias_vertices() {
    compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                path {
                    vertices = [[0, 0], [10, 0]]
                    in_tangents = [[0, 0], [0, 0]]
                    out_tangents = [[0, 0], [0, 0]]
                }
                fill = #000000
                position = [5, 5]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn path_bezier_too_few_vertices_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { path { v = [[0, 0]] } fill = #000000 position = [5,5] }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("v") || err.contains("顶点"));
}

#[test]
fn path_bezier_tangent_count_mismatch_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s {
                path {
                    v = [[0, 0], [10, 0], [10, 10]]
                    in = [[0, 0], [0, 0]]
                }
                fill = #000000
                position = [5,5]
            }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("in") || err.contains("切线"));
}

#[test]
fn path_still_supports_svg_d() {
    compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            shape s { path { d = "M0 0 L10 0 L10 10" } fill = #000000 position=[5,5] }
        }"#,
    )
    .unwrap();
}
