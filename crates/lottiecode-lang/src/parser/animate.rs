//! Animate 块、关键帧、Easing 解析。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError};
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    /// `animate <prop-name> { time: value [ease=...] [...] } [loop]`
    pub(crate) fn parse_animate(&mut self) -> Result<AnimateDecl, LottieError> {
        let start_span = self.current().span;
        self.expect(&TokenKind::Animate)?;
        // 属性名可以是普通标识符或形状关键字（trim/fill/stroke 等可作为属性引用）
        let property = match &self.current().kind {
            TokenKind::Ident(s) => {
                let s = s.clone();
                self.advance();
                s
            }
            TokenKind::Trim => {
                self.advance();
                "trim".to_string()
            }
            TokenKind::Fill => {
                self.advance();
                "fill".to_string()
            }
            TokenKind::Stroke => {
                self.advance();
                "stroke".to_string()
            }
            TokenKind::Path => {
                self.advance();
                "path".to_string()
            }
            _ => {
                return Err(LottieError::new(
                    ErrorKind::ExpectedToken,
                    format!("期望动画属性名，但遇到 {}", self.current().kind),
                    Some(self.current().span),
                ));
            }
        };
        self.expect(&TokenKind::LBrace)?;

        let mut keyframes = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            keyframes.push(self.parse_keyframe()?);
        }
        self.expect(&TokenKind::RBrace)?;

        // 可选 `loop` 修饰符
        let looped = if self.check(&TokenKind::Loop) {
            self.advance();
            true
        } else {
            false
        };

        Ok(AnimateDecl {
            property,
            keyframes,
            looped,
            span: start_span,
        })
    }

    /// `<time> : <value> [ease = <easing>]`
    /// 也支持开头的 `hold` 关键字：`hold 0.2s` 表示在该时间保持当前值（h:1）。
    pub(crate) fn parse_keyframe(&mut self) -> Result<Keyframe, LottieError> {
        let start_span = self.current().span;
        let hold = if self.check(&TokenKind::Hold) {
            self.advance();
            true
        } else {
            false
        };

        let time = self.parse_expression()?;
        self.expect(&TokenKind::Colon)?;
        let value = self.parse_expression()?;

        let easing = if matches!(&self.current().kind, TokenKind::Ident(s) if s == "ease") {
            self.advance();
            self.expect(&TokenKind::Equals)?;
            Some(self.parse_easing()?)
        } else {
            None
        };

        // 可选的空间切线：`to = [x, y]` 或 `ti = [x, y]`
        let mut spatial_to = None;
        let mut spatial_ti = None;
        loop {
            let key = match &self.current().kind {
                TokenKind::Ident(s) if s == "to" => Some("to"),
                TokenKind::Ident(s) if s == "ti" => Some("ti"),
                _ => None,
            };
            match key {
                Some("to") => {
                    self.advance();
                    self.expect(&TokenKind::Equals)?;
                    spatial_to = Some(self.parse_expression()?);
                }
                Some("ti") => {
                    self.advance();
                    self.expect(&TokenKind::Equals)?;
                    spatial_ti = Some(self.parse_expression()?);
                }
                _ => break,
            }
        }

        Ok(Keyframe {
            time,
            value,
            easing,
            hold,
            spatial_to,
            spatial_ti,
            span: start_span,
        })
    }

    /// `ease-out-back` 或 `cubic(0.4, 0, 0.6, 1)`。
    fn parse_easing(&mut self) -> Result<EasingExpr, LottieError> {
        match &self.current().kind {
            TokenKind::Ident(name) => {
                let name = name.clone();
                let span = self.current().span;
                self.advance();
                if name == "cubic" {
                    self.expect(&TokenKind::LParen)?;
                    let a = self.parse_expression()?.as_f64().ok_or_else(|| {
                        LottieError::new(ErrorKind::TypeMismatch, "cubic 第 1 参数应为数字", Some(span))
                    })?;
                    self.expect(&TokenKind::Comma)?;
                    let b = self.parse_expression()?.as_f64().ok_or_else(|| {
                        LottieError::new(ErrorKind::TypeMismatch, "cubic 第 2 参数应为数字", Some(span))
                    })?;
                    self.expect(&TokenKind::Comma)?;
                    let c = self.parse_expression()?.as_f64().ok_or_else(|| {
                        LottieError::new(ErrorKind::TypeMismatch, "cubic 第 3 参数应为数字", Some(span))
                    })?;
                    self.expect(&TokenKind::Comma)?;
                    let d = self.parse_expression()?.as_f64().ok_or_else(|| {
                        LottieError::new(ErrorKind::TypeMismatch, "cubic 第 4 参数应为数字", Some(span))
                    })?;
                    self.expect(&TokenKind::RParen)?;
                    Ok(EasingExpr::Cubic([a, b, c, d]))
                } else {
                    Ok(EasingExpr::Named(name))
                }
            }
            _ => Err(LottieError::new(
                ErrorKind::ExpectedExpression,
                format!("期望 easing 名称或 `cubic(...)`，但遇到 {}", self.current().kind),
                Some(self.current().span),
            )),
        }
    }
}
