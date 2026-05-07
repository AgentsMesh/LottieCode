//! Mask AST 节点。

use super::animate::AnimateDecl;
use super::Attribute;
use crate::token::Span;

/// `mask { mode = ..., path = "...", opacity = ..., invert = false, expand = 0, animate path { ... } }`
#[derive(Debug, Clone)]
pub struct MaskDecl {
    pub attributes: Vec<Attribute>,
    pub animations: Vec<AnimateDecl>,
    pub span: Span,
}
