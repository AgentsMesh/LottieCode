//! IR Transform —— 图层与 Group 的变换。

use super::property::AnimatableValue;

/// 图层 Transform（Lottie 的 `ks`）。
#[derive(Debug, Clone)]
pub struct IrTransform {
    pub anchor: AnimatableValue<[f64; 3]>,
    pub position: AnimatableValue<[f64; 3]>,
    pub scale: AnimatableValue<[f64; 3]>,
    pub rotation: AnimatableValue<f64>,
    pub opacity: AnimatableValue<f64>,
    pub skew: AnimatableValue<f64>,
    pub skew_axis: AnimatableValue<f64>,
    pub rx: AnimatableValue<f64>,
    pub ry: AnimatableValue<f64>,
    pub rz: AnimatableValue<f64>,
    pub orientation: AnimatableValue<[f64; 3]>,
    pub three_d: bool,
}

impl Default for IrTransform {
    fn default() -> Self {
        Self {
            anchor: AnimatableValue::Static([0.0, 0.0, 0.0]),
            position: AnimatableValue::Static([0.0, 0.0, 0.0]),
            scale: AnimatableValue::Static([100.0, 100.0, 100.0]),
            rotation: AnimatableValue::Static(0.0),
            opacity: AnimatableValue::Static(100.0),
            skew: AnimatableValue::Static(0.0),
            skew_axis: AnimatableValue::Static(0.0),
            rx: AnimatableValue::Static(0.0),
            ry: AnimatableValue::Static(0.0),
            rz: AnimatableValue::Static(0.0),
            orientation: AnimatableValue::Static([0.0, 0.0, 0.0]),
            three_d: false,
        }
    }
}

/// Shape 内 Group 的 Transform（2D，结构与 IrTransform 类似但维度不同）。
#[derive(Debug, Clone)]
pub struct IrShapeTransform {
    pub anchor: AnimatableValue<[f64; 2]>,
    pub position: AnimatableValue<[f64; 2]>,
    pub scale: AnimatableValue<[f64; 2]>,
    pub rotation: AnimatableValue<f64>,
    pub opacity: AnimatableValue<f64>,
    pub skew: AnimatableValue<f64>,
    pub skew_axis: AnimatableValue<f64>,
}

impl Default for IrShapeTransform {
    fn default() -> Self {
        Self {
            anchor: AnimatableValue::Static([0.0, 0.0]),
            position: AnimatableValue::Static([0.0, 0.0]),
            scale: AnimatableValue::Static([100.0, 100.0]),
            rotation: AnimatableValue::Static(0.0),
            opacity: AnimatableValue::Static(100.0),
            skew: AnimatableValue::Static(0.0),
            skew_axis: AnimatableValue::Static(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_transform_default_identity() {
        let t = IrTransform::default();
        if let AnimatableValue::Static(s) = t.scale {
            assert_eq!(s, [100.0, 100.0, 100.0]);
        }
        if let AnimatableValue::Static(o) = t.opacity {
            assert_eq!(o, 100.0);
        }
        assert!(!t.three_d);
    }

    #[test]
    fn shape_transform_default_identity() {
        let t = IrShapeTransform::default();
        if let AnimatableValue::Static(s) = t.scale {
            assert_eq!(s, [100.0, 100.0]);
        }
        if let AnimatableValue::Static(p) = t.position {
            assert_eq!(p, [0.0, 0.0]);
        }
    }

    #[test]
    fn layer_transform_3d_fields_present() {
        let t = IrTransform::default();
        if let AnimatableValue::Static(o) = t.orientation {
            assert_eq!(o, [0.0, 0.0, 0.0]);
        } else {
            panic!("expected static orientation");
        }
    }
}
