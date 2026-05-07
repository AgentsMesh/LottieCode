//! Asset 顶层声明：image / precomp 资源。

use super::Attribute;
use crate::token::Span;

/// `asset name { image = "path", width = N, height = N }`
/// 或 `asset name { precomp { ... } }`（Phase 3 仅支持 image）。
#[derive(Debug, Clone)]
pub struct AssetDecl {
    pub name: String,
    pub kind: AssetKind,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetKind {
    Image,
    Precomp,
    Sound,
    Data,
}
