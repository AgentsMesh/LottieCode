//! Shape 块、几何、Trim 解析。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError};
use crate::token::TokenKind;

use super::Parser;

impl Parser {
    /// `shape name { rect|ellipse|path { } | path = "..." | attr | animate | trim }`
    pub(crate) fn parse_shape(&mut self) -> Result<ShapeDecl, LottieError> {
        let start_span = self.current().span;
        self.expect(&TokenKind::Shape)?;
        let name = self.expect_ident()?;
        self.expect(&TokenKind::LBrace)?;

        let mut geometries = Vec::new();
        let mut attributes = Vec::new();
        let mut animations = Vec::new();
        let mut trim = None;
        let mut masks = Vec::new();
        let mut groups: Vec<ShapeGroupDecl> = Vec::new();

        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            // 识别 `group [name] { ... }` 子块（用 ident "group" + LBrace 模式，免占用 keyword）
            if let TokenKind::Ident(s) = &self.current().kind {
                if s == "group" {
                    let look_ahead_brace = self.tokens.get(self.pos + 1).map(|t| matches!(t.kind, TokenKind::LBrace | TokenKind::Ident(_)));
                    if look_ahead_brace == Some(true) {
                        groups.push(self.parse_shape_group()?);
                        continue;
                    }
                }
            }
            match &self.current().kind {
                TokenKind::Rect => geometries.push(self.parse_rect_block()?),
                TokenKind::Ellipse => geometries.push(self.parse_ellipse_block()?),
                TokenKind::PolyStar => geometries.push(self.parse_polystar_block()?),
                TokenKind::Path => geometries.push(self.parse_path_decl()?),
                TokenKind::Animate => animations.push(self.parse_animate()?),
                TokenKind::Mask => masks.push(self.parse_mask_block()?),
                TokenKind::Trim => {
                    if trim.is_some() {
                        return Err(LottieError::new(
                            ErrorKind::DuplicateDefinition,
                            "shape 内重复的 `trim` 块",
                            Some(self.current().span),
                        ));
                    }
                    trim = Some(self.parse_trim_block()?);
                }
                TokenKind::Fill | TokenKind::Stroke | TokenKind::Ident(_) => {
                    attributes.push(self.parse_attribute()?);
                }
                _ => {
                    return Err(LottieError::new(
                        ErrorKind::UnexpectedToken,
                        format!("shape 内意外的 {}", self.current().kind),
                        Some(self.current().span),
                    ));
                }
            }
        }
        self.expect(&TokenKind::RBrace)?;

        Ok(ShapeDecl {
            name,
            geometries,
            attributes,
            animations,
            trim,
            masks,
            groups,
            span: start_span,
        })
    }

    /// `group [name] { rect|ellipse|path { } | attr }` —— 子 group 块。

    pub(crate) fn parse_rect_block(&mut self) -> Result<GeometryDecl, LottieError> {
        let start_span = self.current().span;
        self.expect(&TokenKind::Rect)?;
        let attributes = self.parse_brace_attributes()?;
        Ok(GeometryDecl::Rect {
            attributes,
            span: start_span,
        })
    }

    pub(crate) fn parse_ellipse_block(&mut self) -> Result<GeometryDecl, LottieError> {
        let start_span = self.current().span;
        self.expect(&TokenKind::Ellipse)?;
        let attributes = self.parse_brace_attributes()?;
        Ok(GeometryDecl::Ellipse {
            attributes,
            span: start_span,
        })
    }

    pub(crate) fn parse_polystar_block(&mut self) -> Result<GeometryDecl, LottieError> {
        let start_span = self.current().span;
        self.expect(&TokenKind::PolyStar)?;
        let attributes = self.parse_brace_attributes()?;
        Ok(GeometryDecl::PolyStar {
            attributes,
            span: start_span,
        })
    }

    /// 同时支持 `path "..."`、`path = "..."`、`path { d = "..." }` 三种写法。
    pub(crate) fn parse_path_decl(&mut self) -> Result<GeometryDecl, LottieError> {
        let start_span = self.current().span;
        self.expect(&TokenKind::Path)?;

        let attributes = match &self.current().kind {
            TokenKind::Equals => {
                self.advance();
                let value = self.parse_expression()?;
                vec![Attribute {
                    key: "d".to_string(),
                    value,
                    span: start_span,
                }]
            }
            TokenKind::StringLit(_) => {
                let value = self.parse_expression()?;
                vec![Attribute {
                    key: "d".to_string(),
                    value,
                    span: start_span,
                }]
            }
            TokenKind::LBrace => self.parse_brace_attributes()?,
            _ => {
                return Err(LottieError::new(
                    ErrorKind::ExpectedToken,
                    format!("`path` 后期望 `=`、字符串或 `{{`，但遇到 {}", self.current().kind),
                    Some(self.current().span),
                ));
            }
        };

        Ok(GeometryDecl::Path {
            attributes,
            span: start_span,
        })
    }

    fn parse_trim_block(&mut self) -> Result<TrimDecl, LottieError> {
        let start_span = self.current().span;
        self.expect(&TokenKind::Trim)?;
        let attributes = self.parse_brace_attributes()?;
        Ok(TrimDecl {
            attributes,
            span: start_span,
        })
    }
}
