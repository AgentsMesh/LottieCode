//! IR Shape 元素 —— 形状层内部所有种类。
//!
//! 设计：按职责分类，渲染顺序由类型系统强制：
//!   geometry → modifier → style → repeater
//! 不再用大 enum 平铺，避免 build 顺序硬编码。

use super::enums::*;
use super::property::AnimatableValue;
use super::transform::IrShapeTransform;

/// 几何源（产生路径的元素）。Lottie ty=rc/el/sr/sh。
#[derive(Debug, Clone)]
pub enum IrGeometry {
    Rectangle {
        position: AnimatableValue<[f64; 2]>,
        size: AnimatableValue<[f64; 2]>,
        radius: AnimatableValue<f64>,
        direction: PathDirection,
    },
    Ellipse {
        position: AnimatableValue<[f64; 2]>,
        size: AnimatableValue<[f64; 2]>,
        direction: PathDirection,
    },
    PolyStar {
        position: AnimatableValue<[f64; 2]>,
        rotation: AnimatableValue<f64>,
        points: AnimatableValue<f64>,
        outer_radius: AnimatableValue<f64>,
        outer_roundness: AnimatableValue<f64>,
        inner_radius: AnimatableValue<f64>,
        inner_roundness: AnimatableValue<f64>,
        star_type: StarType,
        direction: PathDirection,
    },
    Path {
        bezier: AnimatableValue<BezierPath>,
        direction: PathDirection,
    },
}

/// 路径变形器（作用于上方所有 path）。Lottie ty=tm/rd/mm/op/pb/tw/zz。
#[derive(Debug, Clone)]
pub enum IrPathModifier {
    TrimPath {
        start: AnimatableValue<f64>,
        end: AnimatableValue<f64>,
        offset: AnimatableValue<f64>,
        mode: TrimMode,
    },
    RoundedCorners {
        radius: AnimatableValue<f64>,
    },
    Merge {
        mode: MergeMode,
    },
    OffsetPath {
        amount: AnimatableValue<f64>,
        line_join: LineJoin,
        miter_limit: AnimatableValue<f64>,
    },
    PuckerBloat {
        amount: AnimatableValue<f64>,
    },
    Twist {
        angle: AnimatableValue<f64>,
        center: AnimatableValue<[f64; 2]>,
    },
    Zigzag {
        amplitude: AnimatableValue<f64>,
        frequency: AnimatableValue<f64>,
        kind: ZigzagKind,
    },
}

/// 样式（填充 / 描边）。Lottie ty=fl/st/gf/gs。
#[derive(Debug, Clone)]
pub enum IrStyle {
    Fill {
        color: AnimatableValue<[f64; 4]>,
        opacity: AnimatableValue<f64>,
        rule: FillRule,
    },
    Stroke {
        color: AnimatableValue<[f64; 4]>,
        opacity: AnimatableValue<f64>,
        width: AnimatableValue<f64>,
        line_cap: LineCap,
        line_join: LineJoin,
        miter_limit: f64,
        dash: Vec<f64>,
        dash_offset: f64,
    },
    GradientFill {
        kind: GradientKind,
        start: AnimatableValue<[f64; 2]>,
        end: AnimatableValue<[f64; 2]>,
        stops: Vec<GradientStop>,
        opacity: AnimatableValue<f64>,
    },
    GradientStroke {
        kind: GradientKind,
        start: AnimatableValue<[f64; 2]>,
        end: AnimatableValue<[f64; 2]>,
        stops: Vec<GradientStop>,
        opacity: AnimatableValue<f64>,
        width: AnimatableValue<f64>,
        line_cap: LineCap,
        line_join: LineJoin,
        miter_limit: f64,
    },
}

/// Repeater 复制器（作用于整组）。Lottie ty=rp。
#[derive(Debug, Clone)]
pub struct IrRepeater {
    pub copies: AnimatableValue<f64>,
    pub offset: AnimatableValue<f64>,
    pub composite: CompositeOrder,
    pub transform: IrShapeTransform,
}

/// Group 一个完整渲染单元：geometry → modifier → style → repeater + 自身 transform。
#[derive(Debug, Clone)]
pub struct IrGroup {
    pub name: String,
    pub geometries: Vec<IrGeometry>,
    pub modifiers: Vec<IrPathModifier>,
    pub styles: Vec<IrStyle>,
    pub repeater: Option<IrRepeater>,
    pub transform: IrShapeTransform,
}

/// Shape layer 的内容：多个 group + layer-level trim sibling。
#[derive(Debug, Clone, Default)]
pub struct IrShapeLayer {
    pub groups: Vec<IrGroup>,
    /// 提到 group 外作 layer-level sibling 的 trim（与 Lottie 设计师工具一致）。
    pub layer_trim: Option<IrPathModifier>,
}

#[derive(Debug, Clone, Copy)]
pub enum GradientKind {
    Linear, // t=1
    Radial, // t=2
}

impl GradientKind {
    pub fn as_lottie(&self) -> u8 {
        match self {
            GradientKind::Linear => 1,
            GradientKind::Radial => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GradientStop {
    pub offset: f64,
    pub color: [f64; 4],
}

#[derive(Debug, Clone, Default)]
pub struct BezierPath {
    pub vertices: Vec<[f64; 2]>,
    pub in_tangents: Vec<[f64; 2]>,
    pub out_tangents: Vec<[f64; 2]>,
    pub closed: bool,
}
