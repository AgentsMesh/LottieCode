//! e2e_tests::layers

use super::{build, compile, shape_items};

// ============================================================
// Layer kinds (text/image/precomp/solid/null/camera)
// ============================================================

#[test]
fn text_layer_emits_ty5_with_t_block() {
    let json = build(
        r#"composition "x" {
            width=200 height=100 fps=30 duration=1s
            text "Hello" {
                font = "Inter"
                weight = Bold
                size = 24
                color = #FFFFFF
                align = center
                position = [100, 50]
            }
        }"#,
    );
    assert_eq!(json["layers"][0]["ty"], 5);
    assert!(json["layers"][0]["t"]["d"]["k"][0]["s"]["t"].is_string());
    assert_eq!(json["fonts"]["list"][0]["fFamily"], "Inter");
}

#[test]
fn image_layer_emits_ty2_with_refid() {
    let json = build(
        r#"
        asset logo { image = "logo.png" width = 100 height = 100 }
        composition "x" {
            width=200 height=200 fps=30 duration=1s
            image i { asset = logo position = [100, 100] }
        }"#,
    );
    assert_eq!(json["layers"][0]["ty"], 2);
    assert_eq!(json["layers"][0]["refId"], "logo");
    assert_eq!(json["assets"][0]["id"], "logo");
}

#[test]
fn precomp_layer_emits_ty0_with_dimensions() {
    let json = build(
        r#"
        precomp card {
            width = 100 height = 50 fps = 30 duration = 1s
            shape bg { rect { size = [100, 50] } fill = #000000 position = [50, 25] }
        }
        composition "x" {
            width=200 height=100 fps=30 duration=1s
            precomp p { asset = card position = [100, 50] }
        }"#,
    );
    assert_eq!(json["layers"][0]["ty"], 0);
    assert_eq!(json["layers"][0]["w"], 100);
    assert_eq!(json["layers"][0]["h"], 50);
    assert_eq!(json["assets"][0]["id"], "card");
}

#[test]
fn solid_layer_emits_ty1_with_sc() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            solid bg {
                color = #FF0000
                size = [100, 100]
            }
        }"#,
    );
    assert_eq!(json["layers"][0]["ty"], 1);
    assert_eq!(json["layers"][0]["sw"], 100);
    assert_eq!(json["layers"][0]["sh"], 100);
    assert!(json["layers"][0]["sc"].as_str().unwrap().starts_with("#"));
}

#[test]
fn controller_emits_ty3_null() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            controller anchor { position = [50, 50, 0] }
        }"#,
    );
    assert_eq!(json["layers"][0]["ty"], 3);
}

#[test]
fn camera_emits_ty13_with_perspective() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            camera cam { position = [50, 50, -200] perspective = 500 }
        }"#,
    );
    assert_eq!(json["layers"][0]["ty"], 13);
    assert_eq!(json["layers"][0]["pe"]["k"], 500.0);
}

// ============================================================
// Mask 输出
// ============================================================

#[test]
fn mask_emits_masksProperties_and_hasMask() {
    let json = build(
        r#"composition "x" {
            width=200 height=200 fps=30 duration=1s
            shape s {
                rect { size = [150, 150] }
                fill = #6366F1
                position = [100, 100]
                mask {
                    mode = add
                    path = "M-50 0 L0 -50 L50 0 L0 50"
                    opacity = 100
                }
            }
        }"#,
    );
    let layer = &json["layers"][0];
    assert_eq!(layer["hasMask"], true);
    assert!(layer["masksProperties"].is_array());
    assert_eq!(layer["masksProperties"][0]["mode"], "a");
}

