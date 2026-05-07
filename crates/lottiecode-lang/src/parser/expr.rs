//! 表达式、属性、数组、对象的解析。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    /// `key = value` —— shape/geometry/composition 块通用属性。
    /// 所有保留字都允许作为属性名（关键字仅在块上下文中才作为关键字识别）。
    pub(crate) fn parse_attribute(&mut self) -> Result<Attribute, LottieError> {
        let start_span = self.current().span;
        let key = match keyword_to_name(&self.current().kind) {
            Some(name) => {
                self.advance();
                name
            }
            None => {
                return Err(LottieError::new(
                    ErrorKind::ExpectedToken,
                    format!("期望属性名，但遇到 {}", self.current().kind),
                    Some(self.current().span),
                ));
            }
        };
        self.expect(&TokenKind::Equals)?;
        let value = self.parse_expression()?;
        Ok(Attribute {
            key,
            value,
            span: start_span,
        })
    }

    /// 解析 `{ key = val ... }` 形式的属性集合（用于几何块、对象字面量等）。
    pub(crate) fn parse_brace_attributes(&mut self) -> Result<Vec<Attribute>, LottieError> {
        self.expect(&TokenKind::LBrace)?;
        let mut attrs = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            attrs.push(self.parse_attribute()?);
            // 对象内允许逗号分隔
            if self.check(&TokenKind::Comma) {
                self.advance();
            }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(attrs)
    }

    pub(crate) fn parse_expression(&mut self) -> Result<Expression, LottieError> {
        let tok = self.current().clone();
        let span = tok.span;
        match tok.kind {
            TokenKind::StringLit(s) => {
                self.advance();
                Ok(Expression::StringLit(s, span))
            }
            TokenKind::IntLit(n) => {
                self.advance();
                Ok(Expression::IntLit(n, span))
            }
            TokenKind::FloatLit(n) => {
                self.advance();
                Ok(Expression::FloatLit(n, span))
            }
            TokenKind::BoolLit(b) => {
                self.advance();
                Ok(Expression::BoolLit(b, span))
            }
            TokenKind::TimeSec(v) => {
                self.advance();
                Ok(Expression::TimeSec(v, span))
            }
            TokenKind::TimeMs(v) => {
                self.advance();
                Ok(Expression::TimeMs(v, span))
            }
            TokenKind::TimeFrames(n) => {
                self.advance();
                Ok(Expression::TimeFrames(n, span))
            }
            TokenKind::ColorLit(c) => {
                self.advance();
                Ok(Expression::ColorLit(c, span))
            }
            TokenKind::Ident(_) => self.parse_ident_or_path(),
            TokenKind::LBracket => self.parse_array(span),
            TokenKind::LBrace => {
                let attrs = self.parse_brace_attributes()?;
                Ok(Expression::Object(attrs, span))
            }
            _ => Err(LottieError::new(
                ErrorKind::ExpectedExpression,
                format!("期望表达式，但遇到 {}", tok.kind),
                Some(span),
            )),
        }
    }

    /// 标识符或点路径：`bold` / `color.success` / `tokens.duration.fast`。
    fn parse_ident_or_path(&mut self) -> Result<Expression, LottieError> {
        let span = self.current().span;
        let first = self.expect_ident()?;
        if !self.check(&TokenKind::Dot) {
            return Ok(Expression::Ident(first, span));
        }
        let mut parts = vec![first];
        while self.check(&TokenKind::Dot) {
            self.advance();
            parts.push(self.expect_ident()?);
        }
        Ok(Expression::Path(parts, span))
    }

    fn parse_array(&mut self, span: crate::token::Span) -> Result<Expression, LottieError> {
        self.expect(&TokenKind::LBracket)?;
        let mut items = Vec::new();
        while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
            items.push(self.parse_expression()?);
            if self.check(&TokenKind::Comma) {
                self.advance();
            }
        }
        self.expect(&TokenKind::RBracket)?;
        Ok(Expression::Array(items, span))
    }
}

/// 用于消除「未使用 import」警告：Token 用于公共字段。
#[allow(dead_code)]
fn _kept_for_signature(_t: &Token) {}

/// 把 keyword token 还原为字符串名（用于允许保留字作为属性名）。
fn keyword_to_name(kind: &TokenKind) -> Option<String> {
    Some(match kind {
        TokenKind::Ident(s) => s.clone(),
        TokenKind::Composition => "composition".into(),
        TokenKind::Token => "token".into(),
        TokenKind::Slots => "slots".into(),
        TokenKind::Shape => "shape".into(),
        TokenKind::Text => "text".into(),
        TokenKind::Image => "image".into(),
        TokenKind::Precomp => "precomp".into(),
        TokenKind::Asset => "asset".into(),
        TokenKind::Mask => "mask".into(),
        TokenKind::Solid => "solid".into(),
        TokenKind::Controller => "controller".into(),
        TokenKind::Camera => "camera".into(),
        TokenKind::Component => "component".into(),
        TokenKind::Use => "use".into(),
        TokenKind::Include => "include".into(),
        TokenKind::Animate => "animate".into(),
        TokenKind::Loop => "loop".into(),
        TokenKind::Hold => "hold".into(),
        TokenKind::Typewriter => "typewriter".into(),
        TokenKind::Rect => "rect".into(),
        TokenKind::Ellipse => "ellipse".into(),
        TokenKind::Path => "path".into(),
        TokenKind::PolyStar => "polystar".into(),
        TokenKind::Fill => "fill".into(),
        TokenKind::Stroke => "stroke".into(),
        TokenKind::Trim => "trim".into(),
        _ => return None,
    })
}
