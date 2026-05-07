//! e2e_tests::assets

use crate::ir::{IrAsset, IrLayerKind};

use super::compile;

// ============================================================
// Image / Asset / Embed
// ============================================================

#[test]
fn image_layer_with_asset() {
    let ir = compile(
        r#"
        asset logo {
            image = "logo.png"
            width = 100
            height = 100
        }
        composition "x" {
            width=200 height=200 fps=30 duration=1s
            image my-logo {
                asset = logo
                position = [100, 100]
            }
        }
        "#,
    )
    .unwrap();
    assert_eq!(ir.layers.len(), 1);
    assert_eq!(ir.assets.len(), 1);
    assert!(matches!(ir.layers[0].kind, IrLayerKind::Image { .. }));
}

#[test]
fn data_url_image_treated_embedded() {
    let ir = compile(
        r#"
        asset png {
            image = "data:image/png;base64,iVBOR"
            width = 1
            height = 1
        }
        composition "x" {
            width=10 height=10 fps=30 duration=1s
            image i { asset = png }
        }
        "#,
    )
    .unwrap();
    assert_eq!(ir.assets.len(), 1);
}

#[test]
fn sound_asset_compiles() {
    let ir = compile(
        r#"
        asset bgm { sound = "music.mp3" }
        composition "x" { width=10 height=10 fps=30 duration=1s }
        "#,
    )
    .unwrap();
    assert_eq!(ir.assets.len(), 1);
}

#[test]
fn data_asset_compiles() {
    let ir = compile(
        r#"
        asset cfg { data = "config.json" }
        composition "x" { width=10 height=10 fps=30 duration=1s }
        "#,
    )
    .unwrap();
    assert_eq!(ir.assets.len(), 1);
}

#[test]
fn missing_image_asset_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            image foo { asset = nonexistent }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("nonexistent") || err.contains("asset"));
}

// ============================================================
// Precomp 顶层 + 实例
// ============================================================

#[test]
fn precomp_decl_and_layer_instance() {
    let ir = compile(
        r#"
        precomp card {
            width = 200
            height = 100
            fps = 30
            duration = 1s

            shape bg {
                rect { size = [200, 100] }
                fill = #FF0000
                position = [100, 50]
            }
        }

        composition "x" {
            width = 400
            height = 200
            fps = 30
            duration = 1s

            precomp inst { asset = card position = [200, 100] }
        }
        "#,
    )
    .unwrap();
    assert_eq!(ir.assets.len(), 1);
    assert!(matches!(&ir.assets[0], IrAsset::Precomp { .. }));
    assert!(matches!(ir.layers[0].kind, IrLayerKind::Precomp { .. }));
}

#[test]
fn precomp_layer_missing_asset_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            precomp inst { asset = no-such }
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("no-such") || err.contains("precomp"));
}

// ============================================================
// Component / Use 实例化
// ============================================================

#[test]
fn component_with_default_param_instantiates() {
    let ir = compile(
        r#"
        component dot(color = #FF0000, x = 50) {
            shape s {
                ellipse { size = [10, 10] }
                fill = color
                position = [x, 50]
            }
        }
        composition "x" {
            width=200 height=100 fps=30 duration=1s
            use dot
            use dot(x = 100)
            use dot(color = #00FF00, x = 150)
        }
        "#,
    )
    .unwrap();
    assert_eq!(ir.layers.len(), 3);
}

#[test]
fn use_undefined_component_errors() {
    let err = compile(
        r#"composition "x" {
            width=10 height=10 fps=30 duration=1s
            use missing
        }"#,
    )
    .unwrap_err();
    assert!(err.contains("missing"));
}

#[test]
fn use_missing_required_param_errors() {
    let err = compile(
        r#"
        component requires-x(x) {
            shape s { rect { size = [10, 10] } fill = #000000 position = [x, 0] }
        }
        composition "x" {
            width=10 height=10 fps=30 duration=1s
            use requires-x
        }
        "#,
    )
    .unwrap_err();
    assert!(err.contains("x") || err.contains("参数"));
}

