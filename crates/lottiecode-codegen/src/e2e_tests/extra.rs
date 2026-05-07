//! 补充覆盖：sound / data / audio asset、各种动画化属性、字体边界。

use super::{build, compile};

#[test]
fn sound_asset_external_path_split() {
    let ir = compile(
        r#"
        asset bgm { sound = "audio/track.mp3" }
        composition "x" { width=10 height=10 fps=30 duration=1s }
        "#,
    );
    let json = crate::generate(&ir);
    assert_eq!(json["assets"][0]["u"], "audio/");
    assert_eq!(json["assets"][0]["p"], "track.mp3");
}

#[test]
fn data_asset_external_path() {
    let ir = compile(
        r#"
        asset cfg { data = "config/app.json" }
        composition "x" { width=10 height=10 fps=30 duration=1s }
        "#,
    );
    let json = crate::generate(&ir);
    assert_eq!(json["assets"][0]["t"], 3);
    assert_eq!(json["assets"][0]["u"], "config/");
}

#[test]
fn animated_position_emits_a_one() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect{size=[10,10]} fill=#000000 position=[50,50]
                animate position { 0s: [0, 0]  1s: [100, 0] }
            }
        }"#,
    );
    assert_eq!(json["layers"][0]["ks"]["p"]["a"], 1);
    let kfs = json["layers"][0]["ks"]["p"]["k"].as_array().unwrap();
    assert_eq!(kfs.len(), 2);
}

#[test]
fn animated_color_in_fill() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect{size=[40,40]} fill=#FF0000 position=[50,50]
                animate fill-color {
                    0s: #FF0000
                    1s: #0000FF
                }
            }
        }"#,
    );
    let layer = &json["layers"][0];
    let shapes = layer["shapes"].as_array().unwrap();
    let group = &shapes[0];
    let items = group["it"].as_array().unwrap();
    let fill = items.iter().find(|x| x["ty"] == "fl").unwrap();
    assert_eq!(fill["c"]["a"], 1);
}

#[test]
fn animated_scale_with_easing() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s {
                rect{size=[10,10]} fill=#000000 position=[50,50]
                animate scale {
                    0s: [50, 50, 100]
                    1s: [120, 120, 100]  ease = ease-out-back
                }
            }
        }"#,
    );
    assert_eq!(json["layers"][0]["ks"]["s"]["a"], 1);
    let kfs = json["layers"][0]["ks"]["s"]["k"].as_array().unwrap();
    assert!(kfs[0]["i"]["x"].as_array().unwrap()[0].as_f64().unwrap() > 0.0);
}

#[test]
fn fonts_emit_font_path_class_weight_defaults() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            text "T" { font = "Roboto"  size = 16  color = #000000  position = [50, 50] }
        }"#,
    );
    let font0 = &json["fonts"]["list"][0];
    assert_eq!(font0["fFamily"], "Roboto");
    assert_eq!(font0["fStyle"], "Regular");
    assert_eq!(font0["fName"], "Roboto-Regular");
    assert_eq!(font0["origin"], 0);
}

#[test]
fn duplicate_font_with_same_family_dedupes() {
    let json = build(
        r#"composition "x" {
            width=200 height=100 fps=30 duration=1s
            text "A" { font = "Inter"  size = 12  color = #000000  position = [50, 50] }
            text "B" { font = "Inter"  size = 14  color = #000000  position = [150, 50] }
        }"#,
    );
    let list = json["fonts"]["list"].as_array().unwrap();
    assert_eq!(list.len(), 1, "same family/weight should dedupe");
}

#[test]
fn distinct_weights_create_separate_fonts() {
    let json = build(
        r#"composition "x" {
            width=200 height=100 fps=30 duration=1s
            text "A" { font="Inter" weight=Bold size=16 color=#000000 position=[50,50] }
            text "B" { font="Inter" weight=Light size=16 color=#000000 position=[150,50] }
        }"#,
    );
    let list = json["fonts"]["list"].as_array().unwrap();
    assert_eq!(list.len(), 2);
}

#[test]
fn audio_layer_with_volume() {
    let json = build(
        r#"
        asset bgm { sound = "data:audio/mp3;base64,xxx" }
        composition "x" {
            width=10 height=10 fps=30 duration=1s
        }"#,
    );
    assert_eq!(json["assets"][0]["e"], 1);
}

#[test]
fn markers_emit_top_level() {
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=2s
            markers = [
                { name = "intro", time = 0s, duration = 0.5s }
            ]
        }"#,
    );
    let markers = json["markers"].as_array().unwrap();
    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0]["cm"], "intro");
}

#[test]
fn slots_emit_top_level() {
    let json = build(
        r#"
        slots {
            primary = #6366F1
        }
        composition "x" {
            width=100 height=100 fps=30 duration=1s
            shape s { rect{size=[40,40]} fill=slot.primary position=[50,50] }
        }"#,
    );
    assert!(json["slots"].is_object());
}

#[test]
fn three_d_layer_emits_ddd_one() {
    // camera 层会触发 three_d=true → ddd=1
    let json = build(
        r#"composition "x" {
            width=100 height=100 fps=30 duration=1s
            camera cam { position = [50, 50, -200] perspective = 500 }
            shape s { rect{size=[40,40]} fill=#000000 position=[50,50] }
        }"#,
    );
    assert_eq!(json["ddd"], 1);
}
