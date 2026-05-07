//! IR Mask —— 图层级遮罩。

use super::property::AnimatableValue;
use super::shape::BezierPath;

#[derive(Debug, Clone, Copy)]
pub enum MaskMode {
    None,
    Add,
    Subtract,
    Intersect,
    Lighten,
    Darken,
    Difference,
}

impl MaskMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            MaskMode::None => "n",
            MaskMode::Add => "a",
            MaskMode::Subtract => "s",
            MaskMode::Intersect => "i",
            MaskMode::Lighten => "l",
            MaskMode::Darken => "d",
            MaskMode::Difference => "f",
        }
    }
}

#[derive(Debug, Clone)]
pub struct IrMask {
    pub mode: MaskMode,
    pub inverted: bool,
    pub path: AnimatableValue<BezierPath>,
    pub opacity: AnimatableValue<f64>,
    pub expand: AnimatableValue<f64>,
}
