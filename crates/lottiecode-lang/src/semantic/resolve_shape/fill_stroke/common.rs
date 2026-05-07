//! Fill / Stroke 共享辅助：颜色提取 + 标识符匹配。

use crate::ast::Expression;
use crate::error::{ErrorKind, LottieError, Result};
use crate::semantic::util::parse_color;

pub fn extract_color(expr: &Expression) -> Result<[f64; 4]> {
    match expr {
        Expression::ColorLit(hex, span) => parse_color(hex, *span),
        _ => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            "期望颜色字面量",
            Some(expr.span()),
        )),
    }
}

pub fn identifier_str(expr: &Expression) -> Option<&str> {
    match expr {
        Expression::Ident(s, _) => Some(s.as_str()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Span;

    fn sp() -> Span {
        Span::new(0, 0, 1, 1)
    }

    #[test]
    fn extract_color_accepts_hex() {
        let v = extract_color(&Expression::ColorLit("00FF00".into(), sp())).unwrap();
        assert!(v[1] > 0.99);
    }

    #[test]
    fn extract_color_rejects_other() {
        assert!(extract_color(&Expression::IntLit(0, sp())).is_err());
    }

    #[test]
    fn identifier_str_returns_ident() {
        assert_eq!(
            identifier_str(&Expression::Ident("round".into(), sp())),
            Some("round")
        );
    }

    #[test]
    fn identifier_str_returns_none_for_others() {
        assert_eq!(identifier_str(&Expression::IntLit(0, sp())), None);
    }
}
