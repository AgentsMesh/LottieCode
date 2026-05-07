//! AST —— 抽象语法树根

pub mod animate;
pub mod asset;
pub mod component;
pub mod composition;
pub mod expr;
pub mod mask;
pub mod precomp;
pub mod shape;
pub mod solid_null;
pub mod text;

pub use animate::*;
pub use asset::*;
pub use component::*;
pub use composition::*;
pub use expr::*;
pub use mask::*;
pub use precomp::*;
pub use shape::*;
pub use solid_null::*;
pub use text::*;

use crate::token::Span;

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub includes: Vec<IncludeDecl>,
    pub composition: Option<CompositionDecl>,
    pub tokens: Vec<TokenGroupDecl>,
    pub components: Vec<ComponentDecl>,
    pub assets: Vec<AssetDecl>,
    pub precomps: Vec<PrecompDecl>,
    /// 顶层 `slots { name = value ... }`（Lottie 1.0 外部可注入参数）。
    pub slots: Vec<Attribute>,
}

/// 块内的 `key = value` 属性。
#[derive(Debug, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: Expression,
    pub span: Span,
}
