//! 通用辅助：值校验、颜色解析、时间换算。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::AnimatableValue;
use crate::token::Span;

pub fn expect_uint(expr: &Expression, span: Span) -> Result<u32> {
    match expr {
        Expression::IntLit(n, _) if *n >= 0 => Ok(*n as u32),
        _ => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            "期望非负整数",
            Some(span),
        )),
    }
}

pub fn expect_number(expr: &Expression, span: Span) -> Result<f64> {
    expr.as_f64().ok_or_else(|| {
        LottieError::new(ErrorKind::TypeMismatch, "期望数字", Some(span))
    })
}

pub fn expect_seconds(expr: &Expression, span: Span, fps: f64) -> Result<f64> {
    match expr {
        Expression::TimeSec(v, _) => Ok(*v),
        Expression::TimeMs(v, _) => Ok(*v / 1000.0),
        Expression::TimeFrames(n, _) => Ok(*n as f64 / fps),
        Expression::IntLit(n, _) => Ok(*n as f64),
        Expression::FloatLit(v, _) => Ok(*v),
        _ => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            "期望时间字面量（如 1.5s / 500ms / 30f）",
            Some(span),
        )),
    }
}

pub fn expect_frames(expr: &Expression, span: Span, fps: f64) -> Result<f64> {
    let secs = expect_seconds(expr, span, fps)?;
    Ok((secs * fps).round())
}

pub fn parse_color(hex: &str, span: Span) -> Result<[f64; 4]> {
    if hex.len() != 6 && hex.len() != 8 {
        return Err(LottieError::new(
            ErrorKind::InvalidColor,
            format!("无效颜色 `#{hex}`"),
            Some(span),
        ));
    }
    let parse = |i: usize| -> Result<f64> {
        let s = &hex[i..i + 2];
        u8::from_str_radix(s, 16)
            .map(|v| v as f64 / 255.0)
            .map_err(|_| {
                LottieError::new(
                    ErrorKind::InvalidColor,
                    format!("颜色十六进制片段 `{s}` 无效"),
                    Some(span),
                )
            })
    };
    let r = parse(0)?;
    let g = parse(2)?;
    let b = parse(4)?;
    let a = if hex.len() == 8 { parse(6)? } else { 1.0 };
    Ok([r, g, b, a])
}

pub fn expect_xy(expr: &Expression, span: Span) -> Result<[f64; 2]> {
    let arr = expr.as_f64_array().ok_or_else(|| {
        LottieError::new(ErrorKind::TypeMismatch, "期望 `[x, y]` 数组", Some(span))
    })?;
    if arr.len() != 2 {
        return Err(LottieError::new(
            ErrorKind::TypeMismatch,
            format!("期望 2 个元素的数组，实际 {}", arr.len()),
            Some(span),
        ));
    }
    Ok([arr[0], arr[1]])
}

pub fn expect_xyz(expr: &Expression, span: Span) -> Result<[f64; 3]> {
    let arr = expr.as_f64_array().ok_or_else(|| {
        LottieError::new(ErrorKind::TypeMismatch, "期望坐标数组", Some(span))
    })?;
    match arr.len() {
        2 => Ok([arr[0], arr[1], 0.0]),
        3 => Ok([arr[0], arr[1], arr[2]]),
        n => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            format!("期望 2 或 3 个元素的坐标数组，实际 {n}"),
            Some(span),
        )),
    }
}

/// `33.333` → `[33.333, 33.333, 100.0]`：scale 单值广播。
/// `[80, 80]` 2D 数组 → `[80, 80, 100]`：z 默认 identity（不是 0）。
pub fn scale_to_xyz(expr: &Expression, span: Span) -> Result<[f64; 3]> {
    if let Some(v) = expr.as_f64() {
        return Ok([v, v, 100.0]);
    }
    let arr = expr.as_f64_array().ok_or_else(|| {
        LottieError::new(ErrorKind::TypeMismatch, "期望 scale 数组", Some(span))
    })?;
    match arr.len() {
        2 => Ok([arr[0], arr[1], 100.0]),
        3 => Ok([arr[0], arr[1], arr[2]]),
        n => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            format!("scale 期望 2 或 3 元素数组，实际 {n}"),
            Some(span),
        )),
    }
}

pub fn s<T>(v: T) -> AnimatableValue<T> {
    AnimatableValue::Static(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Span;

    fn sp() -> Span {
        Span::new(0, 0, 1, 1)
    }

    #[test]
    fn parse_color_six_digits() {
        let rgba = parse_color("FF0000", sp()).unwrap();
        assert!((rgba[0] - 1.0).abs() < 1e-9);
        assert!(rgba[1].abs() < 1e-9);
        assert!(rgba[2].abs() < 1e-9);
        assert!((rgba[3] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn parse_color_eight_digits_alpha() {
        let rgba = parse_color("00FF0080", sp()).unwrap();
        assert!((rgba[3] - 0.5019607843).abs() < 1e-3);
    }

    #[test]
    fn parse_color_invalid_length() {
        assert!(parse_color("ABC", sp()).is_err());
    }

    #[test]
    fn parse_color_invalid_hex() {
        assert!(parse_color("ZZZZZZ", sp()).is_err());
    }

    #[test]
    fn expect_uint_rejects_negative() {
        let expr = Expression::IntLit(-1, sp());
        assert!(expect_uint(&expr, sp()).is_err());
    }

    #[test]
    fn expect_seconds_handles_frames() {
        let expr = Expression::TimeFrames(60, sp());
        let secs = expect_seconds(&expr, sp(), 30.0).unwrap();
        assert!((secs - 2.0).abs() < 1e-9);
    }

    #[test]
    fn expect_seconds_handles_ms() {
        let expr = Expression::TimeMs(500.0, sp());
        let secs = expect_seconds(&expr, sp(), 30.0).unwrap();
        assert!((secs - 0.5).abs() < 1e-9);
    }

    #[test]
    fn expect_xy_validates_array_size() {
        let bad = Expression::Array(vec![Expression::IntLit(1, sp())], sp());
        assert!(expect_xy(&bad, sp()).is_err());
    }

    #[test]
    fn expect_xyz_pads_2d_to_3d() {
        let v = Expression::Array(
            vec![Expression::IntLit(1, sp()), Expression::IntLit(2, sp())],
            sp(),
        );
        assert_eq!(expect_xyz(&v, sp()).unwrap(), [1.0, 2.0, 0.0]);
    }

    #[test]
    fn scale_to_xyz_broadcasts_scalar() {
        let v = Expression::IntLit(50, sp());
        assert_eq!(scale_to_xyz(&v, sp()).unwrap(), [50.0, 50.0, 100.0]);
    }
}
