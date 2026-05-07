//! Text 块 AST。

use super::animate::{AnimateDecl, Keyframe};
use super::Attribute;
use crate::token::Span;

/// `text "..." { font = ... size = ... ... typewriter { 0s: 0  2s: 100 } }`
#[derive(Debug, Clone)]
pub struct TextDecl {
    pub content: String,
    pub attributes: Vec<Attribute>,
    pub animations: Vec<AnimateDecl>,
    /// Typewriter（逐字出现）效果。值范围 0-100，表示已显示的字符百分比。
    pub typewriter: Option<Vec<Keyframe>>,
    pub span: Span,
}
