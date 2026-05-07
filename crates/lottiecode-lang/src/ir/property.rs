//! 属性值 —— 静态或动画化。
//!
//! `Static(v)` → Lottie `{a:0, k:v}`；`Animated(...)` → `{a:1, k:[...]}`；
//! `Expression{default,expr}` → `{a:0, k:default, x:expr}`。

#[derive(Debug, Clone)]
pub enum AnimatableValue<T> {
    Static(T),
    Animated(Vec<Keyframe<T>>),
    Expression { default: T, expr: String },
}

impl<T: Clone> AnimatableValue<T> {
    pub fn map_static<U>(self, f: impl Fn(T) -> U) -> AnimatableValue<U>
    where
        U: Clone,
    {
        match self {
            AnimatableValue::Static(v) => AnimatableValue::Static(f(v)),
            AnimatableValue::Animated(kfs) => AnimatableValue::Animated(
                kfs.into_iter()
                    .map(|kf| Keyframe {
                        frame: kf.frame,
                        value: f(kf.value),
                        easing: kf.easing,
                        hold: kf.hold,
                        spatial_in: kf.spatial_in,
                        spatial_out: kf.spatial_out,
                    })
                    .collect(),
            ),
            AnimatableValue::Expression { default, expr } => AnimatableValue::Expression {
                default: f(default),
                expr,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Keyframe<T> {
    pub frame: f64,
    pub value: T,
    pub easing: Easing,
    pub hold: bool,
    pub spatial_in: Option<[f64; 3]>,
    pub spatial_out: Option<[f64; 3]>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    Cubic([f64; 4]),
}

impl Default for Easing {
    fn default() -> Self {
        Easing::Linear
    }
}

impl Easing {
    pub fn as_cubic(&self) -> [f64; 4] {
        match self {
            Easing::Linear => [0.0, 0.0, 1.0, 1.0],
            Easing::Cubic(c) => *c,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_as_cubic() {
        assert_eq!(Easing::Linear.as_cubic(), [0.0, 0.0, 1.0, 1.0]);
    }

    #[test]
    fn cubic_as_cubic_passthrough() {
        let c = [0.34, 1.56, 0.64, 1.0];
        assert_eq!(Easing::Cubic(c).as_cubic(), c);
    }

    #[test]
    fn easing_default_is_linear() {
        assert_eq!(Easing::default(), Easing::Linear);
    }

    #[test]
    fn map_static_transforms_value() {
        let v: AnimatableValue<f64> = AnimatableValue::Static(2.0);
        let mapped = v.map_static(|x| x * 10.0);
        assert!(matches!(mapped, AnimatableValue::Static(v) if (v - 20.0).abs() < 1e-9));
    }

    #[test]
    fn map_static_preserves_animated_keyframe_metadata() {
        let kf = Keyframe {
            frame: 5.0,
            value: 1.0,
            easing: Easing::Linear,
            hold: true,
            spatial_in: None,
            spatial_out: None,
        };
        let v: AnimatableValue<f64> = AnimatableValue::Animated(vec![kf]);
        let mapped = v.map_static(|x| x + 0.5);
        if let AnimatableValue::Animated(kfs) = mapped {
            assert_eq!(kfs.len(), 1);
            assert!((kfs[0].value - 1.5).abs() < 1e-9);
            assert!(kfs[0].hold);
            assert_eq!(kfs[0].frame, 5.0);
        } else {
            panic!("expected Animated variant");
        }
    }

    #[test]
    fn map_static_preserves_expression() {
        let v: AnimatableValue<f64> = AnimatableValue::Expression {
            default: 0.5,
            expr: "time".to_string(),
        };
        let mapped = v.map_static(|x| x * 100.0);
        match mapped {
            AnimatableValue::Expression { default, expr } => {
                assert!((default - 50.0).abs() < 1e-9);
                assert_eq!(expr, "time");
            }
            _ => panic!("expected Expression variant"),
        }
    }
}
