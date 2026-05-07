//! Composition 与 Token 块的 AST 节点。

use super::component::UseDecl;
use super::precomp::PrecompLayerDecl;
use super::shape::ShapeDecl;
use super::solid_null::{CameraDecl, ControllerDecl, SolidDecl};
use super::text::TextDecl;
use super::Attribute;
use crate::token::Span;

#[derive(Debug, Clone)]
pub enum CompositionItem {
    Shape(ShapeDecl),
    Text(TextDecl),
    Image(ImageLayerDecl),
    Precomp(PrecompLayerDecl),
    Solid(SolidDecl),
    Controller(ControllerDecl),
    Camera(CameraDecl),
    Use(UseDecl),
}

#[derive(Debug, Clone)]
pub struct CompositionDecl {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub items: Vec<CompositionItem>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct TokenGroupDecl {
    pub name: String,
    pub entries: Vec<Attribute>,
    pub span: Span,
}

/// `image my-logo { asset = logo, position = [...] }`
#[derive(Debug, Clone)]
pub struct ImageLayerDecl {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}
