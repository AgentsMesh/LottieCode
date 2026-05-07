//! Lottie spec 中的有限枚举集中定义。
//!
//! 每个 enum 提供 `as_lottie() -> i64`（spec 数字）。codegen 直接序列化数字；
//! IR 层面用 enum 编译期挡住非法值（如 `mode: 9`）。

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PathDirection {
    #[default]
    Normal,
    Reverse,
}

impl PathDirection {
    pub fn as_lottie(self) -> i64 {
        match self {
            PathDirection::Normal => 1,
            PathDirection::Reverse => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StarType {
    #[default]
    Star,
    Polygon,
}

impl StarType {
    pub fn as_lottie(self) -> i64 { match self { StarType::Star => 1, StarType::Polygon => 2 } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TrimMode {
    #[default]
    Simultaneously,
    Individually,
}

impl TrimMode {
    pub fn as_lottie(self) -> i64 {
        match self { TrimMode::Simultaneously => 1, TrimMode::Individually => 2 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MergeMode {
    #[default]
    Merge,
    Add,
    Subtract,
    Intersect,
    Exclude,
}

impl MergeMode {
    pub fn as_lottie(self) -> i64 {
        match self {
            MergeMode::Merge => 1, MergeMode::Add => 2, MergeMode::Subtract => 3,
            MergeMode::Intersect => 4, MergeMode::Exclude => 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineCap {
    Butt,
    #[default]
    Round,
    Square,
}

impl LineCap {
    pub fn as_lottie(self) -> i64 {
        match self { LineCap::Butt => 1, LineCap::Round => 2, LineCap::Square => 3 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineJoin {
    Miter,
    #[default]
    Round,
    Bevel,
}

impl LineJoin {
    pub fn as_lottie(self) -> i64 {
        match self { LineJoin::Miter => 1, LineJoin::Round => 2, LineJoin::Bevel => 3 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FillRule {
    #[default]
    NonZero,
    EvenOdd,
}

impl FillRule {
    pub fn as_lottie(self) -> i64 { match self { FillRule::NonZero => 1, FillRule::EvenOdd => 2 } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompositeOrder {
    #[default]
    Above,
    Below,
}

impl CompositeOrder {
    pub fn as_lottie(self) -> i64 {
        match self { CompositeOrder::Above => 1, CompositeOrder::Below => 2 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ZigzagKind {
    #[default]
    Corner,
    Smooth,
}

impl ZigzagKind {
    pub fn as_lottie(self) -> i64 {
        match self { ZigzagKind::Corner => 1, ZigzagKind::Smooth => 2 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlendMode {
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl BlendMode {
    pub fn as_lottie(self) -> i64 { self as i64 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatteMode {
    #[default]
    None,
    Alpha,
    AlphaInv,
    Luma,
    LumaInv,
}

impl MatteMode {
    pub fn as_lottie(self) -> i64 { self as i64 }
}
