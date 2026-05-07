//! component / use / include 解析。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError};
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    /// `include "path/to/file.lc"`
    pub(crate) fn parse_include(&mut self) -> Result<IncludeDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Include)?;
        let path = self.expect_string()?;
        Ok(IncludeDecl { path, span })
    }

    /// `component name(p1, p2 = default) { items... }`
    pub(crate) fn parse_component(&mut self) -> Result<ComponentDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Component)?;
        let name = self.expect_ident()?;

        // 参数列表（可选）
        let params = if self.check(&TokenKind::LParen) {
            self.advance();
            let mut params = Vec::new();
            while !self.check(&TokenKind::RParen) && !self.is_at_end() {
                params.push(self.parse_component_param()?);
                if self.check(&TokenKind::Comma) {
                    self.advance();
                }
            }
            self.expect(&TokenKind::RParen)?;
            params
        } else {
            Vec::new()
        };

        // body
        self.expect(&TokenKind::LBrace)?;
        let mut items = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            match &self.current().kind {
                TokenKind::Shape => items.push(CompositionItem::Shape(self.parse_shape()?)),
                TokenKind::Text => items.push(CompositionItem::Text(self.parse_text()?)),
                TokenKind::Image => items.push(CompositionItem::Image(self.parse_image_layer()?)),
                TokenKind::Precomp => {
                    items.push(CompositionItem::Precomp(self.parse_precomp_layer()?))
                }
                TokenKind::Solid => items.push(CompositionItem::Solid(self.parse_solid()?)),
                TokenKind::Controller => {
                    items.push(CompositionItem::Controller(self.parse_controller()?))
                }
                TokenKind::Camera => items.push(CompositionItem::Camera(self.parse_camera()?)),
                TokenKind::Use => items.push(CompositionItem::Use(self.parse_use()?)),
                _ => {
                    return Err(LottieError::new(
                        ErrorKind::UnexpectedToken,
                        format!("component 内意外的 {}", self.current().kind),
                        Some(self.current().span),
                    )
                    .with_hint("component 块允许 `shape` / `text` / `image` / `precomp` / `solid` / `controller` / `camera` / `use`"));
                }
            }
        }
        self.expect(&TokenKind::RBrace)?;

        Ok(ComponentDecl {
            name,
            params,
            items,
            span,
        })
    }

    fn parse_component_param(&mut self) -> Result<ComponentParam, LottieError> {
        let span = self.current().span;
        let name = self.expect_ident()?;
        let default = if self.check(&TokenKind::Equals) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(ComponentParam {
            name,
            default,
            span,
        })
    }

    /// `use name(arg1 = val1, arg2 = val2)`
    pub(crate) fn parse_use(&mut self) -> Result<UseDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Use)?;
        let component_name = self.expect_ident()?;

        let mut args = Vec::new();
        if self.check(&TokenKind::LParen) {
            self.advance();
            while !self.check(&TokenKind::RParen) && !self.is_at_end() {
                let arg_span = self.current().span;
                let key = self.expect_ident()?;
                self.expect(&TokenKind::Equals)?;
                let value = self.parse_expression()?;
                args.push(Attribute {
                    key,
                    value,
                    span: arg_span,
                });
                if self.check(&TokenKind::Comma) {
                    self.advance();
                }
            }
            self.expect(&TokenKind::RParen)?;
        }

        Ok(UseDecl {
            component_name,
            args,
            span,
        })
    }
}
