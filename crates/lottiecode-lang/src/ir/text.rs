//! IR Text Layer 数据。

use super::property::AnimatableValue;

/// Lottie Text Document（对应 `t.d.k[].s`）。
#[derive(Debug, Clone)]
pub struct IrTextDocument {
    pub text: String,
    pub font_family: String,
    /// Lottie 字体引用名（Family-Style，如 "Inter-Bold"），与 IrFont.name 一致。
    pub font_name: String,
    pub font_size: f64,
    pub fill_color: [f64; 3],
    pub line_height: Option<f64>,
    pub tracking: f64,
    pub justify: u8,
    pub stroke_color: Option<[f64; 3]>,
    pub stroke_width: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct IrTextData {
    pub document: AnimatableValue<IrTextDocument>,
    /// Typewriter selector amount 动画（0-100）。
    pub typewriter: Option<AnimatableValue<f64>>,
    /// 字符级偏移：被 selector 选中的字符相对于原位置的偏移。
    pub char_position: Option<[f64; 2]>,
    pub char_scale: Option<[f64; 2]>,
    pub char_opacity: Option<f64>,
    pub char_rotation: Option<f64>,
}
