//! IR Layer —— 单个图层。

use super::enums::{BlendMode, MatteMode};
use super::mask::IrMask;
use super::property::AnimatableValue;
use super::shape::IrShapeLayer;
use super::text::IrTextData;
use super::transform::IrTransform;

#[derive(Debug, Clone)]
pub struct IrLayer {
    pub name: String,
    pub kind: IrLayerKind,
    pub transform: IrTransform,
    /// Lottie `ip`：layer 在 composition 时间轴上出现的帧。
    pub in_frame: f64,
    /// Lottie `op`：layer 在 composition 时间轴上消失的帧。
    pub out_frame: f64,
    /// Lottie `st`：layer 内部时间起点偏移（关键帧的 t 字段以此为零点）。默认 0。
    pub start_time: f64,
    /// Lottie `sr`：layer 时间倍率。1.0 = 正常速度，0.5 = 半速。默认 1.0。
    pub time_stretch: f64,
    pub parent: Option<String>,
    pub masks: Vec<IrMask>,
    pub blend_mode: BlendMode,
    pub matte_source: bool,
    pub matte: Option<(String, MatteMode)>,
    pub effects: Vec<IrEffect>,
}

#[derive(Debug, Clone)]
pub enum IrEffect {
    /// 投影。Lottie ty=25。
    DropShadow {
        color: AnimatableValue<[f64; 4]>,
        opacity: AnimatableValue<f64>,    // 0-255 (Lottie convention)
        direction: AnimatableValue<f64>,  // 角度 deg
        distance: AnimatableValue<f64>,
        softness: AnimatableValue<f64>,
    },
    /// 高斯模糊。Lottie ty=29。
    GaussianBlur {
        blurriness: AnimatableValue<f64>,
        direction: AnimatableValue<f64>,  // 0=horizontal+vertical
        repeat_edge_pixels: bool,
    },
}

#[derive(Debug, Clone)]
pub enum IrLayerKind {
    Shape(IrShapeLayer),
    Text(IrTextData),
    Image {
        asset_id: String,
    },
    Precomp {
        asset_id: String,
        width: u32,
        height: u32,
        /// Lottie `tm` 时间重映射：`AnimatableValue<f64>`，单位为秒。`None` 表示不重映射。
        time_remap: Option<AnimatableValue<f64>>,
    },
    /// `ty:1` Solid Layer —— 纯色矩形。
    Solid {
        color: [f64; 4],
        width: u32,
        height: u32,
    },
    /// `ty:3` Null Layer —— 不可见控制器，常用于 parenting 锚点。
    Null,
    /// `ty:6` Audio Layer。
    Audio {
        asset_id: String,
        volume: f64,
    },
    /// `ty:15` Data Source Layer。
    Data {
        asset_id: String,
    },
    /// `ty:13` Camera Layer。
    Camera {
        perspective: AnimatableValue<f64>,
    },
}
