//! Fill / Stroke 解析 —— 简写直接颜色或对象形式。
//! 渐变变体在 gradient.rs。

mod common;
mod fill;
mod stroke;

pub use common::{extract_color, identifier_str};
pub use fill::build_fill;
pub use stroke::build_stroke;
