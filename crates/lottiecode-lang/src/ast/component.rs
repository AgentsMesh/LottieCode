//! component / use / include AST 节点。

use super::composition::CompositionItem;
use super::expr::Expression;
use super::Attribute;
use crate::token::Span;

#[derive(Debug, Clone)]
pub struct IncludeDecl {
    pub path: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ComponentDecl {
    pub name: String,
    pub params: Vec<ComponentParam>,
    pub items: Vec<CompositionItem>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ComponentParam {
    pub name: String,
    pub default: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct UseDecl {
    pub component_name: String,
    pub args: Vec<Attribute>,
    pub span: Span,
}