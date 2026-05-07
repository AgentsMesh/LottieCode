//! Precomp 解析：顶层定义 + 层内引用。

use crate::ast::*;
use crate::error::LottieError;
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    /// 顶层 `precomp <name> { width=N ... shape ... }`
    pub(crate) fn parse_precomp_decl(&mut self) -> Result<PrecompDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Precomp)?;
        let name = self.expect_ident()?;
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
                        crate::error::ErrorKind::UnexpectedToken,
                        format!("precomp 内意外的 {}", self.current().kind),
                        Some(self.current().span),
                    ));
                }
            }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(PrecompDecl {
            name,
            attributes,
            items,
            span,
        })
    }

    /// 层内 `precomp <inst-name> { asset = ref, position = [x,y], animate ... }`
    pub(crate) fn parse_precomp_layer(&mut self) -> Result<PrecompLayerDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Precomp)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::LBrace)?;

        let mut attributes = Vec::new();
        let mut animations = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            match &self.current().kind {
                TokenKind::Animate => animations.push(self.parse_animate()?),
                _ => {
                    attributes.push(self.parse_attribute()?);
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    }
                }
            }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(PrecompLayerDecl {
            name,
            attributes,
            animations,
            span,
        })
    }
}
