//! lottiecode-codegen —— IR → Lottie JSON

mod animation;
pub mod dotlottie;
mod effect;
pub mod fixture_compare;
mod gradient;
mod layer;
mod lottie;
mod mask;
mod property;
mod shape;
mod shape_geometry;
mod shape_modifier;
mod shape_style;
mod style;
mod text;

#[cfg(test)]
mod e2e_tests;

use lottiecode_lang::ir::IrAnimation;

/// 顶层入口：把 IR 转为可序列化的 `serde_json::Value`。
pub fn generate(ir: &IrAnimation) -> serde_json::Value {
    animation::emit_animation(ir)
}

/// 把 IR 转为格式化字符串（带 2 空格缩进）。
pub fn generate_string(ir: &IrAnimation) -> String {
    serde_json::to_string_pretty(&generate(ir)).expect("Lottie JSON 序列化不应失败")
}

/// 紧凑模式（无空格，用于生产环境）。
pub fn generate_compact(ir: &IrAnimation) -> String {
    serde_json::to_string(&generate(ir)).expect("Lottie JSON 序列化不应失败")
}

#[cfg(test)]
mod tests {
    use lottiecode_lang::ir::*;
    use lottiecode_lang::lexer::Lexer;
    use lottiecode_lang::parser::Parser;
    use lottiecode_lang::semantic::SemanticAnalyzer;

    use super::*;

    fn empty_ir() -> IrAnimation {
        IrAnimation {
            name: "test".to_string(),
            version: "5.7.5".to_string(),
            fps: 30.0,
            in_frame: 0.0,
            out_frame: 30.0,
            width: 100,
            height: 100,
            layers: Vec::new(),
            assets: Vec::new(),
            fonts: Vec::new(),
            markers: Vec::new(),
            slots: Vec::new(),
            has_3d: false,
        }
    }

    fn compile(src: &str) -> IrAnimation {
        let mut l = Lexer::new(src);
        let toks = l.tokenize().unwrap();
        let prog = Parser::new(toks).parse().unwrap();
        SemanticAnalyzer::analyze(&prog).unwrap()
    }

    #[test]
    fn empty_animation_has_required_fields() {
        let json = generate(&empty_ir());
        assert_eq!(json["v"], "5.7.5");
        assert_eq!(json["fr"], 30.0);
        assert_eq!(json["w"], 100);
        assert_eq!(json["h"], 100);
        assert!(json["assets"].is_array());
        assert!(json["layers"].is_array());
    }

    #[test]
    fn shape_layer_outputs_ty4() {
        let mut ir = empty_ir();
        ir.layers.push(IrLayer {
            name: "test".into(),
            kind: IrLayerKind::Shape(IrShapeLayer::default()),
            transform: IrTransform::default(),
            in_frame: 0.0,
        start_time: 0.0,
        time_stretch: 1.0,
            out_frame: 30.0,
            parent: None,
            masks: Vec::new(),
            blend_mode: BlendMode::Normal,
            matte_source: false,
            matte: None,
            effects: Vec::new(),
        });
        let json = generate(&ir);
        assert_eq!(json["layers"][0]["ty"], 4);
    }

    #[test]
    fn static_value_emits_a_zero() {
        let mut ir = empty_ir();
        ir.layers.push(IrLayer {
            name: "x".into(),
            kind: IrLayerKind::Shape(IrShapeLayer::default()),
            transform: IrTransform::default(),
            in_frame: 0.0,
        start_time: 0.0,
        time_stretch: 1.0,
            out_frame: 30.0,
            parent: None,
            masks: Vec::new(),
            blend_mode: BlendMode::Normal,
            matte_source: false,
            matte: None,
            effects: Vec::new(),
        });
        let json = generate(&ir);
        assert_eq!(json["layers"][0]["ks"]["o"]["a"], 0);
        assert_eq!(json["layers"][0]["ks"]["o"]["k"], 100.0);
    }

    #[test]
    fn end_to_end_rectangle_with_fill() {
        let ir = compile(
            r#"composition "demo" {
                width=100 height=100 fps=30 duration=1s
                shape s {
                    rect { size=[20,20] position=[0,0] }
                    fill = #FF0000
                }
            }"#,
        );
        let json = generate(&ir);
        assert_eq!(json["w"], 100);
        assert_eq!(json["layers"][0]["ty"], 4);
        let shapes = json["layers"][0]["shapes"].as_array().unwrap();
        assert_eq!(shapes[0]["ty"], "gr");
        let group_items = shapes[0]["it"].as_array().unwrap();
        assert!(group_items.iter().any(|x| x["ty"] == "rc"));
        assert!(group_items.iter().any(|x| x["ty"] == "fl"));
        assert!(group_items.iter().any(|x| x["ty"] == "tr"));
    }

    #[test]
    fn end_to_end_animated_opacity() {
        let ir = compile(
            r#"composition "demo" {
                width=10 height=10 fps=60 duration=1s
                shape s {
                    rect { size=[10,10] }
                    fill = #000000
                    animate opacity {
                        0s: 0
                        0.5s: 100
                    }
                }
            }"#,
        );
        let json = generate(&ir);
        assert_eq!(json["layers"][0]["ks"]["o"]["a"], 1);
        let kfs = json["layers"][0]["ks"]["o"]["k"].as_array().unwrap();
        assert_eq!(kfs.len(), 2);
        assert_eq!(kfs[0]["t"], 0.0);
        assert_eq!(kfs[1]["t"], 30.0);
    }

    #[test]
    fn generate_string_is_pretty() {
        let s = generate_string(&empty_ir());
        assert!(s.contains("  "), "pretty output should have indentation");
    }

    #[test]
    fn generate_compact_is_compact() {
        let s = generate_compact(&empty_ir());
        assert!(!s.contains("\n  "), "compact output should not be indented");
    }
}
