//! e2e_tests::group_block —— shape 内多 group。

use crate::ir::IrLayerKind;

use super::compile;

#[test]
fn shape_with_two_groups() {
    let ir = compile(
        r#"composition "x" {
            width=200 height=200 fps=30 duration=1s
            shape multi {
                position = [100, 100]
                group dot1 {
                    ellipse { size = [10, 10] }
                    fill = #FF0000
                    position = [-30, 0]
                }
                group dot2 {
                    ellipse { size = [10, 10] }
                    fill = #00FF00
                    position = [30, 0]
                }
            }
        }"#,
    )
    .unwrap();
    if let IrLayerKind::Shape(layer) = &ir.layers[0].kind {
        assert_eq!(layer.groups.len(), 2);
    }
}

#[test]
fn shape_with_implicit_main_group_and_sub_groups() {
    let ir = compile(
        r#"composition "x" {
            width=200 height=200 fps=30 duration=1s
            shape composite {
                rect { size = [80, 80] }
                fill = #6366F1
                position = [100, 100]
                group accent {
                    ellipse { size = [16, 16] }
                    fill = #FFFFFF
                    position = [30, -30]
                }
            }
        }"#,
    )
    .unwrap();
    if let IrLayerKind::Shape(layer) = &ir.layers[0].kind {
        assert_eq!(layer.groups.len(), 2);
    }
}

#[test]
fn empty_shape_with_only_groups() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape just_groups {
                position = [50, 50]
                group a { ellipse { size = [10, 10] } fill = #000000 }
                group b { ellipse { size = [10, 10] } fill = #FFFFFF position = [20, 0] }
            }
        }"#,
    )
    .unwrap();
}
