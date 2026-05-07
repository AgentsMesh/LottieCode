//! Precomp（预合成）AST 节点。
//!
//! 设计：
//! - 顶层 `precomp <name> { width=N height=N fps=N duration=Ns ... shape ... }` 定义一个嵌套合成（asset）
//! - 在 composition / component 内 `precomp <inst-name> { asset = <ref>, position = [x,y] }` 引用

use super::animate::AnimateDecl;
use super::composition::CompositionItem;
use super::Attribute;
use crate::token::Span;

/// 顶层 `precomp <name> { ... }` —— 定义一个可复用的嵌套合成。
#[derive(Debug, Clone)]
pub struct PrecompDecl {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub items: Vec<CompositionItem>,
    pub span: Span,
}

/// 层内 `precomp <inst-name> { asset = ref, position = [x,y], animate ... }` —— 引用一个 PrecompDecl。
#[derive(Debug, Clone)]
pub struct PrecompLayerDecl {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub animations: Vec<AnimateDecl>,
    pub span: Span,
}
