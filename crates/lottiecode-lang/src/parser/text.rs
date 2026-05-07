//! Text 块解析。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError};
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    /// `text "content" { ... }`
    pub(crate) fn parse_text(&mut self) -> Result<TextDecl, LottieError> {
        let span = self.current().span;
        self.expect(&TokenKind::Text)?;
        let content = self.expect_string()?;
        self.expect(&TokenKind::LBrace)?;

        let mut attributes = Vec::new();
        let mut animations = Vec::new();
        let mut typewriter: Option<Vec<Keyframe>> = None;
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            match &self.current().kind {
                TokenKind::Animate => animations.push(self.parse_animate()?),
                TokenKind::Typewriter => {
                    self.advance();
                    self.expect(&TokenKind::LBrace)?;
                    let mut kfs = Vec::new();
                    while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
                        kfs.push(self.parse_keyframe()?);
                    }
                    self.expect(&TokenKind::RBrace)?;
                    typewriter = Some(kfs);
                }
                TokenKind::Fill | TokenKind::Stroke | TokenKind::Ident(_) => {
                    attributes.push(self.parse_attribute()?);
                }
                _ => {
                    return Err(LottieError::new(
                        ErrorKind::UnexpectedToken,
                        format!("text 内意外的 {}", self.current().kind),
                        Some(self.current().span),
                    ));
                }
            }
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(TextDecl {
            content,
            attributes,
            animations,
            typewriter,
            span,
        })
    }
}
