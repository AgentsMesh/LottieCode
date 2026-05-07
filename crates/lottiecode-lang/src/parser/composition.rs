//! 顶层 composition / token 块解析。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError};
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    /// `composition "name" { attr ... shape ... use ... }`
    pub(crate) fn parse_composition(&mut self) -> Result<CompositionDecl, LottieError> {
        let start_span = self.current().span;
        self.expect(&TokenKind::Composition)?;
        let name = self.expect_string()?;
        self.expect(&TokenKind::LBrace)?;

        let mut attributes = Vec::new();
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
                TokenKind::Ident(_) => attributes.push(self.parse_attribute()?),
                _ => {
                    return Err(LottieError::new(
                        ErrorKind::UnexpectedToken,
                        format!("composition 内意外的 {}", self.current().kind),
                        Some(self.current().span),
                    )
                    .with_hint("composition 块内允许属性、`shape`、`text`、`image`、`precomp`、`solid`、`controller`、`camera`、`use`"));
                }
            }
        }

        self.expect(&TokenKind::RBrace)?;
        Ok(CompositionDecl {
            name,
            attributes,
            items,
            span: start_span,
        })
    }

    /// `token group_name { key = value ... }`
    pub(crate) fn parse_token_group(&mut self) -> Result<TokenGroupDecl, LottieError> {
        let start_span = self.current().span;
        self.expect(&TokenKind::Token)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::LBrace)?;

        let mut entries = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            entries.push(self.parse_attribute()?);
        }
        self.expect(&TokenKind::RBrace)?;

        Ok(TokenGroupDecl {
            name,
            entries,
            span: start_span,
        })
    }
}
