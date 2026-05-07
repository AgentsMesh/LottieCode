//! e2e_tests::extra_b

use super::compile;


#[test]
fn component_with_precomp_and_camera() {
    compile(
        r#"
        precomp tile {
            width=50 height=50 fps=30 duration=1s
            shape s { rect{size=[50,50]} fill=#FF0000 position=[25,25] }
        }
        component scene() {
            precomp t { asset = tile position=[25, 25] }
            camera cam { position = [50, 50, -200] perspective = 500 }
        }
        composition "x" {
            width=200 height=200 fps=30 duration=1s
            width=200 height=200 fps=30 duration=1s
            use scene
        }"#,
    )
    .unwrap();
}

#[test]
fn component_use_arg_via_object_path() {
    compile(
        r#"
        token color {
            brand = #FF0000
            accent = #00FF00
        }
        component dot(c, x = 50) {
            shape s { ellipse{size=[10,10]} fill=c position=[x, 50] }
        }
        composition "x" {
            width=200 height=100 fps=30 duration=1s
            use dot(c = color.brand, x = 50)
            use dot(c = color.accent, x = 150)
        }"#,
    )
    .unwrap();
}

#[test]
fn animate_keyframe_with_loop_flag() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=2s
            shape s {
                rect{size=[40,40]} fill=#000000 position=[50,50]
                animate rotation {
                    0s: 0
                    2s: 360 ease = linear
                } loop
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn animate_anchor_skew_skew_axis() {
    compile(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect{size=[40,40]} fill=#000000 position=[50,50]
                animate anchor   { 0s: [0,0,0]  1s: [10,10,0] }
                animate skew     { 0s: 0  1s: 30 }
                animate skew-axis { 0s: 0  1s: 90 }
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn nested_component_use_in_component() {
    compile(
        r#"
        component dot(x = 0) {
            shape s { ellipse{size=[10,10]} fill=#000000 position=[x, 0] }
        }
        component row(y = 0) {
            use dot(x = 30)
            use dot(x = 60)
            use dot(x = 90)
            shape bg { rect{size=[200,5]} fill=#CCCCCC position=[100, y] }
        }
        composition "x" {
            width=200 height=200 fps=30 duration=1s
            use row(y = 100)
        }"#,
    )
    .unwrap();
}

#[test]
fn animated_mask_path_morph() {
    compile(
        r#"composition "x" {
            width=200 height=200 fps=30 duration=1s
            shape s {
                rect{size=[150,150]} fill=#000000 position=[100,100]
                mask {
                    mode = add
                    path = "M-50 0 L0 -50 L50 0 L0 50"
                    opacity = 100
                    animate path {
                        0s: "M-50 0 L0 -50 L50 0 L0 50"
                        1s: "M-30 0 L0 -30 L30 0 L0 30"
                    }
                }
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn animated_mask_opacity_and_expand() {
    compile(
        r#"composition "x" {
            width=200 height=200 fps=30 duration=1s
            shape s {
                rect{size=[150,150]} fill=#000000 position=[100,100]
                mask {
                    mode = add
                    path = "M-50 0 L0 -50 L50 0 L0 50"
                    opacity = 100
                    expand = 0
                }
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn text_with_typewriter_block() {
    compile(
        r#"composition "x" {
            width=400 height=100 fps=30 duration=2s
            text "Typing..." {
                font = "Arial"
                size = 24
                color = #FFFFFF
                position = [200, 50]
                typewriter {
                    0s: 0
                    1.5s: 100
                }
            }
        }"#,
    )
    .unwrap();
}

