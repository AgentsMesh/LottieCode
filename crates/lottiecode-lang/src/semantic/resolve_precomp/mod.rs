//! Precomp 解析 —— 顶层 PrecompDecl → IrAsset::Precomp + 层内 PrecompLayerDecl → IrLayer。

mod decl;
mod layer;

pub use decl::resolve_precomp_decl;
pub use layer::resolve_precomp_layer;
