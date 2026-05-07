//! IR —— 完全 resolved 的中间表示
//!
//! 所有 token 引用已展开、命名 easing 已转 cubic、时间已统一为帧。
//! IR 由 codegen 直接消费输出 Lottie JSON。

pub mod asset;
pub mod enums;
pub mod layer;
pub mod mask;
pub mod property;
pub mod shape;
pub mod text;
pub mod transform;

pub use asset::*;
pub use enums::*;
pub use layer::*;
pub use mask::*;
pub use property::*;
pub use shape::*;
pub use text::*;
pub use transform::*;

#[derive(Debug, Clone)]
pub struct IrAnimation {
    pub name: String,
    pub version: String,
    pub fps: f64,
    pub in_frame: f64,
    pub out_frame: f64,
    pub width: u32,
    pub height: u32,
    pub layers: Vec<IrLayer>,
    pub assets: Vec<IrAsset>,
    pub fonts: Vec<IrFont>,
    pub markers: Vec<IrMarker>,
    /// Lottie 1.0 顶层 slots：`name → SlotEntry`。
    pub slots: Vec<IrSlot>,
    /// 是否包含 3D 内容（顶层 ddd 标志）。
    pub has_3d: bool,
}

/// Slot 条目：name → 静态值（color / number / string）。
#[derive(Debug, Clone)]
pub struct IrSlot {
    pub name: String,
    pub kind: IrSlotKind,
}

#[derive(Debug, Clone)]
pub enum IrSlotKind {
    Color([f64; 4]),
    Number(f64),
    String(String),
}

/// Lottie 顶层 fonts.list 条目。
#[derive(Debug, Clone)]
pub struct IrFont {
    pub family: String,
    pub style: String,
    pub name: String,
}

/// `markers` 顶层数组项 —— 时间标记，用于 lottie-web `goToAndStop("name")`。
#[derive(Debug, Clone)]
pub struct IrMarker {
    pub time_frame: f64,
    pub comment: String,
    pub duration: f64,
}
