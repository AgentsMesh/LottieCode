use super::*;
use lottiecode_lang::ir::*;

fn empty() -> IrAnimation {
    IrAnimation {
        name: "t".into(),
        version: "5.7.5".into(),
        fps: 60.0,
        in_frame: 0.0,
        out_frame: 60.0,
        width: 200,
        height: 100,
        layers: vec![],
        assets: vec![],
        fonts: vec![],
        markers: vec![],
        slots: vec![],
        has_3d: false,
    }
}

#[test]
fn root_fields_emitted() {
    let json = emit_animation(&empty());
    assert_eq!(json["v"], "5.7.5");
    assert_eq!(json["fr"], 60.0);
    assert_eq!(json["w"], 200);
    assert_eq!(json["h"], 100);
    assert_eq!(json["ddd"], 0);
    assert!(json["layers"].is_array());
}

#[test]
fn has_3d_sets_ddd_one() {
    let mut ir = empty();
    ir.has_3d = true;
    let json = emit_animation(&ir);
    assert_eq!(json["ddd"], 1);
}

#[test]
fn fonts_block_omitted_when_empty() {
    let json = emit_animation(&empty());
    assert!(json.get("fonts").is_none());
}

#[test]
fn fonts_block_present_when_non_empty() {
    let mut ir = empty();
    ir.fonts.push(IrFont {
        family: "Inter".into(),
        style: "Regular".into(),
        name: "Inter-Regular".into(),
    });
    let json = emit_animation(&ir);
    assert_eq!(json["fonts"]["list"][0]["fFamily"], "Inter");
}

#[test]
fn precomp_asset_emits_layer_array() {
    let mut ir = empty();
    ir.assets.push(IrAsset::Precomp {
        id: "card".into(),
        width: 100,
        height: 100,
        fps: 30.0,
        layers: vec![],
    });
    let json = emit_animation(&ir);
    assert_eq!(json["assets"][0]["id"], "card");
    // precomp fps (30) != top fps (60)，应输出 fr。
    assert_eq!(json["assets"][0]["fr"], 30.0);
}

#[test]
fn precomp_asset_omits_fr_when_matches_top_fps() {
    let mut ir = empty();
    ir.assets.push(IrAsset::Precomp {
        id: "inherit".into(),
        width: 100,
        height: 100,
        fps: 60.0, // 与顶层 fps 一致 → 省略 fr（AE 习惯）
        layers: vec![],
    });
    let json = emit_animation(&ir);
    assert!(json["assets"][0].get("fr").is_none());
}

#[test]
fn image_asset_external_path_split() {
    let mut ir = empty();
    ir.assets.push(IrAsset::Image {
        id: "img1".into(),
        width: 32,
        height: 32,
        path: "images/logo.png".into(),
        embedded: false,
    });
    let json = emit_animation(&ir);
    assert_eq!(json["assets"][0]["u"], "images/");
    assert_eq!(json["assets"][0]["p"], "logo.png");
    assert_eq!(json["assets"][0]["e"], 0);
}

#[test]
fn image_asset_embedded_keeps_full_path() {
    let mut ir = empty();
    ir.assets.push(IrAsset::Image {
        id: "img1".into(),
        width: 32,
        height: 32,
        path: "data:image/png;base64,xxx".into(),
        embedded: true,
    });
    let json = emit_animation(&ir);
    assert_eq!(json["assets"][0]["u"], "");
    assert_eq!(json["assets"][0]["e"], 1);
}
