//! Layer 元数据解析：parent / blend / matte。

use super::transform_attrs::AttrIndex;
use crate::ast::Expression;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::{BlendMode, MatteMode};

pub fn parse_parent(attrs: &AttrIndex) -> Option<String> {
    attrs.resolved("parent").and_then(|e| match e {
        Expression::Ident(s, _) => Some(s.clone()),
        _ => None,
    })
}

pub fn parse_blend(attrs: &AttrIndex) -> Result<BlendMode> {
    let Some(e) = attrs.resolved("blend") else { return Ok(BlendMode::Normal) };
    match e {
        Expression::Ident(s, _) => match blend_mode_id(s.as_str()) {
            Some(m) => Ok(m),
            None => Err(LottieError::new(
                ErrorKind::InvalidValue,
                format!(
                    "未知 blend 模式 `{s}`，可选: normal/multiply/screen/overlay/darken/lighten/color-dodge/color-burn/hard-light/soft-light/difference/exclusion/hue/saturation/color/luminosity"
                ),
                Some(e.span()),
            )),
        },
        _ => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            "blend 取值需为标识符（如 multiply / screen）",
            Some(e.span()),
        )),
    }
}

/// 命名混合模式 → BlendMode。返回 None 表示未知名字。
pub fn blend_mode_id(name: &str) -> Option<BlendMode> {
    Some(match name {
        "normal" => BlendMode::Normal,
        "multiply" => BlendMode::Multiply,
        "screen" => BlendMode::Screen,
        "overlay" => BlendMode::Overlay,
        "darken" => BlendMode::Darken,
        "lighten" => BlendMode::Lighten,
        "color-dodge" => BlendMode::ColorDodge,
        "color-burn" => BlendMode::ColorBurn,
        "hard-light" => BlendMode::HardLight,
        "soft-light" => BlendMode::SoftLight,
        "difference" => BlendMode::Difference,
        "exclusion" => BlendMode::Exclusion,
        "hue" => BlendMode::Hue,
        "saturation" => BlendMode::Saturation,
        "color" => BlendMode::Color,
        "luminosity" => BlendMode::Luminosity,
        _ => return None,
    })
}

pub fn parse_matte_source(attrs: &AttrIndex) -> bool {
    matches!(
        attrs.resolved("matte-source"),
        Some(Expression::BoolLit(true, _))
    )
}

pub fn parse_matte(attrs: &AttrIndex) -> Result<Option<(String, MatteMode)>> {
    let Some(e) = attrs.resolved("matte") else { return Ok(None) };
    match e {
        Expression::Ident(s, _) => Ok(Some((s.clone(), MatteMode::Alpha))),
        Expression::Object(items, _) => {
            let mut name = None;
            let mut mode = MatteMode::Alpha;
            for a in items {
                match (a.key.as_str(), &a.value) {
                    ("source", Expression::Ident(n, _)) => name = Some(n.clone()),
                    ("mode", Expression::Ident(m, sp)) => {
                        mode = matte_mode_id(m).map_err(|e| e.with_hint(format!("位置 {:?}", sp)))?;
                    }
                    (other, _) => {
                        return Err(LottieError::new(
                            ErrorKind::InvalidValue,
                            format!("matte 不支持的属性 `{other}`，可选: source / mode"),
                            Some(a.span),
                        ));
                    }
                }
            }
            Ok(name.map(|n| (n, mode)))
        }
        _ => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            "matte 取值需为标识符或 `{ source = ..., mode = ... }`",
            Some(e.span()),
        )),
    }
}

fn matte_mode_id(name: &str) -> Result<MatteMode> {
    Ok(match name {
        "alpha" => MatteMode::Alpha,
        "alpha-inverted" | "alpha-inv" => MatteMode::AlphaInv,
        "luma" => MatteMode::Luma,
        "luma-inverted" | "luma-inv" => MatteMode::LumaInv,
        _ => {
            return Err(LottieError::new(
                ErrorKind::InvalidValue,
                format!(
                    "未知 matte 模式 `{name}`，可选: alpha / alpha-inv / luma / luma-inv"
                ),
                None,
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_blend_modes_resolve() {
        for m in ["normal", "multiply", "luminosity"] {
            assert!(blend_mode_id(m).is_some());
        }
    }

    #[test]
    fn unknown_blend_mode_returns_none() {
        assert_eq!(blend_mode_id("unknown-mode"), None);
    }

    #[test]
    fn matte_modes_distinct_with_aliases() {
        assert!(matches!(matte_mode_id("alpha").unwrap(), MatteMode::Alpha));
        assert!(matches!(matte_mode_id("alpha-inv").unwrap(), MatteMode::AlphaInv));
        assert!(matches!(matte_mode_id("alpha-inverted").unwrap(), MatteMode::AlphaInv));
        assert!(matches!(matte_mode_id("luma").unwrap(), MatteMode::Luma));
        assert!(matches!(matte_mode_id("luma-inv").unwrap(), MatteMode::LumaInv));
        assert!(matte_mode_id("nope").is_err());
    }
}
