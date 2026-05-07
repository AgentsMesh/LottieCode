//! 端到端 DSL 测试：覆盖各 layer 类型 / shape 元素 / modifier / 错误路径。
//!
//! 这些测试目的是把 semantic 层 0% 覆盖的子模块拉起来——
//! 通过完整的 lex+parse+semantic 流水线触发各分支。

mod assets;
mod bezier_literal;
mod effects_trim;
mod errors_animate;
mod errors_basic;
mod errors_styles;
mod extra_b;
mod extra_c;
mod extra_coverage;
mod extra_d;
mod group_block;
mod layer_meta;
mod layers;
mod misc;
mod shape_modifiers;
mod shape_styles;

use crate::ir::{IrAnimation, IrGeometry, IrGroup, IrPathModifier, IrShapeLayer, IrStyle};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;

pub(crate) fn compile(src: &str) -> Result<IrAnimation, String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize().map_err(|e| e.format(src, "<test>"))?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| e.format(src, "<test>"))?;
    SemanticAnalyzer::analyze(&program).map_err(|e| e.format(src, "<test>"))
}

/// 测试 helper：检查 shape layer 内是否有指定 geometry 类型。
pub(crate) fn has_geometry(layer: &IrShapeLayer, pred: impl Fn(&IrGeometry) -> bool) -> bool {
    layer.groups.iter().any(|g| g.geometries.iter().any(&pred))
}

/// 测试 helper：检查 shape layer 内是否有指定 modifier。
pub(crate) fn has_modifier(layer: &IrShapeLayer, pred: impl Fn(&IrPathModifier) -> bool) -> bool {
    layer.groups.iter().any(|g| g.modifiers.iter().any(&pred))
        || layer.layer_trim.as_ref().map(&pred).unwrap_or(false)
}

/// 测试 helper：检查 shape layer 内是否有指定 style。
pub(crate) fn has_style(layer: &IrShapeLayer, pred: impl Fn(&IrStyle) -> bool) -> bool {
    layer.groups.iter().any(|g| g.styles.iter().any(&pred))
}

/// 测试 helper：返回所有 group。
pub(crate) fn groups(layer: &IrShapeLayer) -> &[IrGroup] {
    &layer.groups
}
