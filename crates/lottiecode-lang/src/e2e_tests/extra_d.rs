//! e2e_tests::extra_d

use crate::ir::IrLayerKind;

use super::compile;

#[test]
fn merge_object_form_with_mode_attr() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect{size=[40,40] position=[-10,0]}
                rect{size=[40,40] position=[10,0]}
                fill = #000000
                merge = { mode = intersect }
                position=[50,50]
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn repeater_composite_above_below() {
    for mode in ["above", "below"] {
        let src = format!(
            r#"composition "x" {{
                width=200 height=100 fps=30 duration=1s
                shape s {{
                    rect{{size=[10,10]}} fill=#000000
                    repeater = {{ copies=3, offset=0, position=[15,0], composite={mode}, scale=[100,100] }}
                    position=[50,50]
                }}
            }}"#
        );
        compile(&src).unwrap_or_else(|e| panic!("{mode}: {e}"));
    }
}

#[test]
fn precomp_layer_with_animation_transforms() {
    let ir = compile(
        r#"
        precomp tile {
            width=50 height=50 fps=30 duration=1s
            shape s { rect{size=[50,50]} fill=#FF0000 position=[25,25] }
        }
        composition "x" {
            width=200 height=100 fps=30 duration=1s
            precomp t1 {
                asset = tile
                position = [50, 50]
                animate position {
                    0s: [50, 50, 0]
                    1s: [150, 50, 0]
                }
                animate scale {
                    0s: [50, 50, 100]
                    1s: [120, 120, 100]
                }
                animate opacity {
                    0s: 0
                    1s: 100
                }
            }
        }"#,
    )
    .unwrap();
    assert!(matches!(ir.layers[0].kind, IrLayerKind::Precomp { .. }));
}

#[test]
fn precomp_layer_with_time_remap() {
    compile(
        r#"
        precomp clip {
            width=100 height=100 fps=30 duration=2s
            shape s { rect{size=[100,100]} fill=#000000 position=[50,50] }
        }
        composition "x" {
            width=100 height=100 fps=30 duration=1s
            precomp inst {
                asset = clip
                position = [50, 50]
                animate time-remap { 0s: 0  1s: 2s }
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn component_with_text_image_solid_layers() {
    compile(
        r#"
        asset logo { image = "logo.png" width = 50 height = 50 }
        component banner(w = 200) {
            shape bg { rect{size=[w, 60]} fill=#6366F1 position=[0, 0] }
            text "Hi" {
                font = "Arial"
                size = 18
                color = #FFFFFF
                position = [0, 0]
            }
            image my_logo {
                asset = logo
                position = [0, 0]
            }
            solid backdrop {
                color = #000000
                size = [50, 50]
            }
            controller anchor { position = [0, 0, 0] }
        }
        composition "x" {
            width=400 height=200 fps=30 duration=1s
            use banner
        }"#,
    )
    .unwrap();
}
