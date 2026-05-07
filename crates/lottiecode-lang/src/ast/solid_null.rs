//! Solid 与 Null 层 AST。

use super::animate::AnimateDecl;
use super::Attribute;
use crate::token::Span;

/// `solid name { color = #..., size = [w, h], position = [x, y] ... }`
#[derive(Debug, Clone)]
pub struct SolidDecl {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub animations: Vec<AnimateDecl>,
    pub span: Span,
}

/// `controller name { position = [...], animate ... }` —— Null Layer / 控制器。
#[derive(Debug, Clone)]
pub struct ControllerDecl {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub animations: Vec<AnimateDecl>,
    pub span: Span,
}

/// `camera name { perspective = N, position = [x,y,z], rotation-x = N ... }`
#[derive(Debug, Clone)]
pub struct CameraDecl {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub animations: Vec<AnimateDecl>,
    pub span: Span,
}
