//! e2e_tests::extra_c

use super::compile;

#[test]
fn text_with_char_animator_attrs() {
    compile(
        r#"composition "x" {
            width=400 height=100 fps=30 duration=1s
            text "Wave" {
                font = "Arial"
                size = 32
                color = #FF0000
                position = [200, 50]
                char-position = [0, -10]
                char-scale = 50
                char-opacity = 50
                char-rotation = 15
            }
        }"#,
    )
    .unwrap();
}

#[test]
fn precomp_decl_with_use_inside() {
    compile(
        r#"
        component dot() {
            shape s { ellipse{size=[10,10]} fill=#FF0000 position=[5,5] }
        }
        precomp tile {
            width = 50 height = 50 fps = 30 duration = 1s
            use dot
            use dot
        }
        composition "x" {
            width=100 height=100 fps=30 duration=1s
            precomp t { asset = tile position=[50,50] }
        }"#,
    )
    .unwrap();
}

#[test]
fn precomp_decl_with_text_image_solid_layers() {
    compile(
        r#"
        asset img1 { image = "x.png" width = 20 height = 20 }
        precomp scene {
            width = 200 height = 100 fps = 30 duration = 1s
            shape bg { rect{size=[200,100]} fill=#000000 position=[100,50] }
            text "Hello" { font="Arial" size=24 color=#FFFFFF position=[100,50] }
            image i { asset=img1 position=[10, 10] }
            solid panel { color=#FF0000 size=[50, 50] }
            controller anchor { position=[50, 50, 0] }
            camera cam { position=[100, 50, -200] }
        }
        composition "x" {
            width=200 height=100 fps=30 duration=1s
            precomp s { asset = scene position=[100, 50] }
        }"#,
    )
    .unwrap();
}

#[test]
fn component_with_animate_block_substitutes_keyframes() {
    compile(
        r#"
        component pulse(start_x = 0, end_x = 100) {
            shape s {
                rect { size = [10, 10] }
                fill = #FF0000
                position = [start_x, 50]
                animate position {
                    0s: [start_x, 50]
                    1s: [end_x, 50]
                }
                animate scale {
                    0s: [50, 50, 100]
                    0.5s: [120, 120, 100]
                    1s: [100, 100, 100]
                }
                animate opacity {
                    0s: 0
                    0.5s: 100
                    1s: 0
                }
            }
        }
        composition "x" {
            width=200 height=100 fps=30 duration=1s
            use pulse(start_x = 50, end_x = 150)
        }"#,
    )
    .unwrap();
}

#[test]
fn component_with_trim_block_substitutes() {
    compile(
        r#"
        component circle(r = 30) {
            shape s {
                ellipse { size = [r, r] }
                stroke = { color = #000000, width = 2, cap = round, join = round }
                position = [50, 50]
                trim {
                    start = 0
                    end = 75
                }
            }
        }
        composition "x" {
            width=100 height=100 fps=30 duration=1s
            use circle(r = 60)
        }"#,
    )
    .unwrap();
}

#[test]
fn component_with_mask_block_substitutes() {
    compile(
        r#"
        component reveal() {
            shape s {
                rect { size=[100,100] } fill=#000000 position=[50,50]
                mask {
                    mode = add
                    path = "M-30 0 L0 -30 L30 0 L0 30"
                    opacity = 100
                }
            }
        }
        composition "x" {
            width=100 height=100 fps=30 duration=1s
            use reveal
        }"#,
    )
    .unwrap();
}
