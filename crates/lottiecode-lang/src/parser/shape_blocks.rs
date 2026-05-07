//! Shape 子块（group / mask）解析。

use crate::ast::*;
use crate::error::LottieError;
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    pub(crate) fn parse_shape_group(&mut self) -> Result<ShapeGroupDecl, LottieError> {
        let span = self.current().span;
        self.advance(); // 消耗 ident "group"
        let name = if let TokenKind::Ident(_) = &self.current().kind {
            Some(self.expect_ident()?)
        } else {
            None
        };
        self.expect(&TokenKind::LBrace)?;
        let mut geometries = Vec::new();
        let mut attributes = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            match &self.current().kind {
                TokenKind::Rect => geometries.push(self.parse_rect_block()?),
                TokenKind::Ellipse => geometries.push(self.parse_ellipse_block()?),
                TokenKind::PolyStar => geometries.push(self.parse_polystar_block()?),
                TokenKind::Path => geometries.push(self.parse_path_decl()?),
                _ => attributes.push(self.parse_attribute()?),
            }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(ShapeGroupDecl {
            name,
            geometries,
            attributes,
            span,
        })
    }

    pub(crate) fn parse_mask_block(&mut self) -> Result<MaskDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Mask)?;
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
        Ok(MaskDecl {
            attributes,
            animations,
            span,
        })
    }
}
