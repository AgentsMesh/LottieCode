//! 递归下降 parser —— Token 流转 AST。
//!
//! 模块拆分：
//! - composition: composition / token 顶层
//! - shape:       shape 块、几何、trim
//! - animate:     animate 块、keyframe、easing
//! - expr:        表达式、数组、对象、属性

pub mod animate;
pub mod asset;
pub mod component;
pub mod composition;
pub mod expr;
pub mod precomp;
pub mod shape;
pub mod shape_blocks;
pub mod solid_null;
pub mod text;

use crate::ast::Program;
use crate::error::{ErrorKind, LottieError};
use crate::token::{Token, TokenKind};

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// 顶层入口：解析整个 token 流为 Program。
    pub fn parse(&mut self) -> Result<Program, LottieError> {
        let mut program = Program::default();

        while !self.is_at_end() {
            match &self.current().kind {
                TokenKind::Composition => {
                    if program.composition.is_some() {
                        return Err(LottieError::new(
                            ErrorKind::DuplicateDefinition,
                            "重复的 `composition` 声明",
                            Some(self.current().span),
                        )
                        .with_hint("一个文件只允许一个顶层 composition"));
                    }
                    program.composition = Some(self.parse_composition()?);
                }
                TokenKind::Token => program.tokens.push(self.parse_token_group()?),
                TokenKind::Component => program.components.push(self.parse_component()?),
                TokenKind::Asset => program.assets.push(self.parse_asset()?),
                TokenKind::Precomp => program.precomps.push(self.parse_precomp_decl()?),
                TokenKind::Include => program.includes.push(self.parse_include()?),
                TokenKind::Slots => {
                    let slot_attrs = self.parse_slots_block()?;
                    program.slots.extend(slot_attrs);
                }
                TokenKind::Eof => break,
                _ => {
                    return Err(LottieError::new(
                        ErrorKind::UnexpectedToken,
                        format!("顶层意外的 {}", self.current().kind),
                        Some(self.current().span),
                    )
                    .with_hint("期望 `composition` / `token` / `component` / `precomp` / `asset` / `slots` / `include`"));
                }
            }
        }

        Ok(program)
    }

    // -- 通用辅助 --

    pub(crate) fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.current().kind == TokenKind::Eof
    }

    pub(crate) fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        if !self.is_at_end() {
            self.pos += 1;
        }
        tok
    }

    pub(crate) fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.current().kind) == std::mem::discriminant(kind)
    }

    pub(crate) fn expect(&mut self, kind: &TokenKind) -> Result<&Token, LottieError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(LottieError::new(
                ErrorKind::ExpectedToken,
                format!("期望 {kind}，但遇到 {}", self.current().kind),
                Some(self.current().span),
            ))
        }
    }

    pub(crate) fn expect_string(&mut self) -> Result<String, LottieError> {
        match &self.current().kind {
            TokenKind::StringLit(s) => {
                let s = s.clone();
                self.advance();
                Ok(s)
            }
            _ => Err(LottieError::new(
                ErrorKind::ExpectedToken,
                format!("期望字符串，但遇到 {}", self.current().kind),
                Some(self.current().span),
            )),
        }
    }

    pub(crate) fn expect_ident(&mut self) -> Result<String, LottieError> {
        match &self.current().kind {
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(LottieError::new(
                ErrorKind::ExpectedToken,
                format!("期望标识符，但遇到 {}", self.current().kind),
                Some(self.current().span),
            )),
        }
    }

    /// `slots { name = value ... }` 顶层块。
    fn parse_slots_block(&mut self) -> Result<Vec<crate::ast::Attribute>, LottieError> {
        self.expect(&TokenKind::Slots)?;
        self.expect(&TokenKind::LBrace)?;
        let mut entries = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            entries.push(self.parse_attribute()?);
        }
        self.expect(&TokenKind::RBrace)?;
        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_src(src: &str) -> Result<Program, LottieError> {
        let mut l = Lexer::new(src);
        let toks = l.tokenize()?;
        Parser::new(toks).parse()
    }

    #[test]
    fn parse_empty_composition() {
        let p = parse_src(r#"composition "x" { width=10 height=10 }"#).unwrap();
        assert!(p.composition.is_some());
        assert_eq!(p.composition.unwrap().name, "x");
    }

    #[test]
    fn duplicate_composition_errors() {
        let err = parse_src(
            r#"composition "a" { } composition "b" { }"#,
        )
        .unwrap_err();
        assert_eq!(err.kind, ErrorKind::DuplicateDefinition);
    }

    #[test]
    fn parse_token_group() {
        let p = parse_src(r#"token color { primary = #FF0000 }"#).unwrap();
        assert_eq!(p.tokens.len(), 1);
        assert_eq!(p.tokens[0].name, "color");
        assert_eq!(p.tokens[0].entries.len(), 1);
    }

    #[test]
    fn unexpected_top_level_token_errors() {
        let err = parse_src("rect { }").unwrap_err();
        assert_eq!(err.kind, ErrorKind::UnexpectedToken);
    }

    #[test]
    fn parse_include_directive() {
        let p = parse_src(r#"include "tokens.lc""#).unwrap();
        assert_eq!(p.includes.len(), 1);
        assert_eq!(p.includes[0].path, "tokens.lc");
    }
}
