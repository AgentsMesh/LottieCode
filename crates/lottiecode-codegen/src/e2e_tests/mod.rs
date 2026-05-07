//! Codegen 端到端测试：compile DSL → IR → JSON。

mod extra;
mod geom_grad;
mod layers;
mod mask_effect;
mod misc;
mod modifiers;

use lottiecode_lang::ir::IrAnimation;
use lottiecode_lang::lexer::Lexer;
use lottiecode_lang::parser::Parser;
use lottiecode_lang::semantic::SemanticAnalyzer;

use crate::generate;

pub(crate) fn compile(src: &str) -> IrAnimation {
    let mut l = Lexer::new(src);
    let toks = l.tokenize().unwrap();
    let prog = Parser::new(toks).parse().unwrap();
    SemanticAnalyzer::analyze(&prog).unwrap()
}

pub(crate) fn build(src: &str) -> serde_json::Value {
    generate(&compile(src))
}

pub(crate) fn shape_items(json: &serde_json::Value, layer_idx: usize) -> Vec<serde_json::Value> {
    let mut out = Vec::new();
    let layer = &json["layers"][layer_idx];
    if let Some(shapes) = layer["shapes"].as_array() {
        for sh in shapes {
            collect_shape_items(sh, &mut out);
        }
    }
    out
}

fn collect_shape_items(node: &serde_json::Value, out: &mut Vec<serde_json::Value>) {
    out.push(node.clone());
    if let Some(items) = node["it"].as_array() {
        for it in items {
            collect_shape_items(it, out);
        }
    }
}
