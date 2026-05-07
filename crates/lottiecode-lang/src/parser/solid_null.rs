//! Solid / Controller (Null) 层解析。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError};
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    /// `solid name { color = #..., size = [w, h], position = [...], animate ... }`
    pub(crate) fn parse_solid(&mut self) -> Result<SolidDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Solid)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::LBrace)?;
        let mut attributes = Vec::new();
        let mut animations = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            match &self.current().kind {
                TokenKind::Animate => animations.push(self.parse_animate()?),
                _ => attributes.push(self.parse_attribute()?),
            }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(SolidDecl {
            name,
            attributes,
            animations,
            span,
        })
    }

    /// `controller name { position = [...], animate ... }` —— null layer。
    pub(crate) fn parse_controller(&mut self) -> Result<ControllerDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Controller)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::LBrace)?;
        let mut attributes = Vec::new();
        let mut animations = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            match &self.current().kind {
                TokenKind::Animate => animations.push(self.parse_animate()?),
                _ => attributes.push(self.parse_attribute()?),
            }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(ControllerDecl {
            name,
            attributes,
            animations,
            span,
        })
    }

    /// `camera name { perspective = N, position = [x,y,z], ... }`
    pub(crate) fn parse_camera(&mut self) -> Result<CameraDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Camera)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::LBrace)?;
        let mut attributes = Vec::new();
        let mut animations = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            match &self.current().kind {
                TokenKind::Animate => animations.push(self.parse_animate()?),
                _ => attributes.push(self.parse_attribute()?),
            }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(CameraDecl {
            name,
            attributes,
            animations,
            span,
        })
    }
}
