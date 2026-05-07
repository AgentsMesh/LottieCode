//! 表达式与字面量。

use super::Attribute;
use crate::token::Span;

/// LottieCode 中所有可能的表达式形式。
#[derive(Debug, Clone)]
pub enum Expression {
    /// `"..."`
    StringLit(String, Span),
    /// 整数：`100`
    IntLit(i64, Span),
    /// 浮点：`33.333`
    FloatLit(f64, Span),
    /// 布尔：`true` / `false`
    BoolLit(bool, Span),
    /// `0.5s`
    TimeSec(f64, Span),
    /// `300ms`
    TimeMs(f64, Span),
    /// `30f`
    TimeFrames(u64, Span),
    /// `#RRGGBB[AA]`
    ColorLit(String, Span),
    /// 单标识符：`bold` / `round` / `center`（用于枚举值）。
    Ident(String, Span),
    /// 点路径：`color.success` / `tokens.duration.fast`。
    Path(Vec<String>, Span),
    /// `[100, 200]` 或 `[100, 200, 0]`。
    Array(Vec<Expression>, Span),
    /// 对象：`{ color = #FF0000, width = 3 }`。
    Object(Vec<Attribute>, Span),
    /// `expr("var $bm_rt = ...")` —— AE 风格表达式字符串。
    Expr(String, Span),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::StringLit(_, s)
            | Expression::IntLit(_, s)
            | Expression::FloatLit(_, s)
            | Expression::BoolLit(_, s)
            | Expression::TimeSec(_, s)
            | Expression::TimeMs(_, s)
            | Expression::TimeFrames(_, s)
            | Expression::ColorLit(_, s)
            | Expression::Ident(_, s)
            | Expression::Path(_, s)
            | Expression::Array(_, s)
            | Expression::Object(_, s)
            | Expression::Expr(_, s) => *s,
        }
    }

    /// 把任意时间字面量转为「秒」（用于 keyframe 时间换算）。
    pub fn as_seconds(&self) -> Option<f64> {
        match self {
            Expression::TimeSec(v, _) => Some(*v),
            Expression::TimeMs(v, _) => Some(*v / 1000.0),
            Expression::TimeFrames(_, _) => None, // 需要 fps 换算，由 semantic 处理
            _ => None,
        }
    }

    /// 把数值字面量转为 `f64`。
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Expression::IntLit(v, _) => Some(*v as f64),
            Expression::FloatLit(v, _) => Some(*v),
            _ => None,
        }
    }

    /// 把数组字面量转为 `Vec<f64>`（仅对全数值数组有效）。
    pub fn as_f64_array(&self) -> Option<Vec<f64>> {
        if let Expression::Array(items, _) = self {
            items.iter().map(|e| e.as_f64()).collect()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sp() -> Span {
        Span::new(0, 0, 1, 1)
    }

    #[test]
    fn span_extracts_uniformly() {
        for expr in [
            Expression::StringLit("x".into(), sp()),
            Expression::IntLit(0, sp()),
            Expression::FloatLit(0.0, sp()),
            Expression::BoolLit(true, sp()),
            Expression::TimeSec(1.0, sp()),
            Expression::TimeMs(100.0, sp()),
            Expression::TimeFrames(30, sp()),
            Expression::ColorLit("FF".into(), sp()),
            Expression::Ident("x".into(), sp()),
            Expression::Path(vec!["a".into()], sp()),
            Expression::Array(vec![], sp()),
            Expression::Object(vec![], sp()),
            Expression::Expr("e".into(), sp()),
        ] {
            assert_eq!(expr.span(), sp());
        }
    }

    #[test]
    fn as_seconds_handles_time_units() {
        assert_eq!(Expression::TimeSec(1.5, sp()).as_seconds(), Some(1.5));
        assert_eq!(Expression::TimeMs(500.0, sp()).as_seconds(), Some(0.5));
        assert_eq!(Expression::TimeFrames(30, sp()).as_seconds(), None);
        assert_eq!(Expression::IntLit(1, sp()).as_seconds(), None);
    }

    #[test]
    fn as_f64_int_and_float() {
        assert_eq!(Expression::IntLit(42, sp()).as_f64(), Some(42.0));
        assert_eq!(Expression::FloatLit(1.5, sp()).as_f64(), Some(1.5));
        assert_eq!(Expression::StringLit("x".into(), sp()).as_f64(), None);
    }

    #[test]
    fn as_f64_array_all_numeric() {
        let arr = Expression::Array(
            vec![Expression::IntLit(1, sp()), Expression::FloatLit(2.5, sp())],
            sp(),
        );
        assert_eq!(arr.as_f64_array(), Some(vec![1.0, 2.5]));
    }

    #[test]
    fn as_f64_array_rejects_mixed() {
        let arr = Expression::Array(
            vec![Expression::IntLit(1, sp()), Expression::Ident("x".into(), sp())],
            sp(),
        );
        assert_eq!(arr.as_f64_array(), None);
    }

    #[test]
    fn as_f64_array_rejects_non_array() {
        assert_eq!(Expression::IntLit(0, sp()).as_f64_array(), None);
    }
}
