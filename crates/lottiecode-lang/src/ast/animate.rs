//! Animate 块、关键帧、Easing。

use super::expr::Expression;
use crate::token::Span;

#[derive(Debug, Clone)]
pub struct AnimateDecl {
    /// "scale" / "opacity" / "position" / "trim-end" / "trim" / ...
    pub property: String,
    pub keyframes: Vec<Keyframe>,
    pub looped: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: Expression,
    pub value: Expression,
    pub easing: Option<EasingExpr>,
    pub hold: bool,
    /// 仅 position 有意义。
    pub spatial_to: Option<Expression>,
    pub spatial_ti: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum EasingExpr {
    /// `ease-out-back`、`linear` 等命名 easing。
    Named(String),
    /// `cubic(x1, y1, x2, y2)`。
    Cubic([f64; 4]),
}
