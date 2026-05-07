//! Shape 块：几何 + 样式 + Trim。

use super::animate::AnimateDecl;
use super::mask::MaskDecl;
use super::Attribute;
use crate::token::Span;

#[derive(Debug, Clone)]
pub struct ShapeDecl {
    pub name: String,
    pub geometries: Vec<GeometryDecl>,
    pub attributes: Vec<Attribute>,
    pub animations: Vec<AnimateDecl>,
    pub trim: Option<TrimDecl>,
    pub masks: Vec<MaskDecl>,
    /// 显式 `group { ... }` 子块。每个为 layer 内独立 group（带自己的 transform）。
    pub groups: Vec<ShapeGroupDecl>,
    pub span: Span,
}

/// `group [name] { rect/ellipse/path { } fill = ... position = ... }`
#[derive(Debug, Clone)]
pub struct ShapeGroupDecl {
    pub name: Option<String>,
    pub geometries: Vec<GeometryDecl>,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum GeometryDecl {
    Rect { attributes: Vec<Attribute>, span: Span },
    Ellipse { attributes: Vec<Attribute>, span: Span },
    PolyStar { attributes: Vec<Attribute>, span: Span },
    Path { attributes: Vec<Attribute>, span: Span },
}

impl GeometryDecl {
    pub fn span(&self) -> Span {
        match self {
            GeometryDecl::Rect { span, .. }
            | GeometryDecl::Ellipse { span, .. }
            | GeometryDecl::PolyStar { span, .. }
            | GeometryDecl::Path { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrimDecl {
    pub attributes: Vec<Attribute>,
    pub span: Span,
}
