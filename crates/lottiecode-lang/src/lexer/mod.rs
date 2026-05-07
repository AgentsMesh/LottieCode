//! Lexer —— 词法分析入口
//!
//! 参考 VEAC 风格：手写状态机 + Span 携带。
//! LottieCode 与 VEAC 的差异：
//! - 标识符可含连字符（`ease-out-back` / `trim-end`）
//! - 新增 `[ ] : +` 标点

pub mod scan;

use crate::error::{ErrorKind, LottieError};
use crate::token::{Span, Token, TokenKind};

pub struct Lexer<'a> {
    pub(crate) _source: &'a str,
    pub(crate) chars: Vec<char>,
    pub(crate) pos: usize,
    pub(crate) line: usize,
    pub(crate) col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            _source: source,
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    /// 完整扫描，返回所有 token 或第一个错误。
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LottieError> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token()?;
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }

    pub(crate) fn current(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    pub(crate) fn advance(&mut self) -> Option<char> {
        let ch = self.current()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    pub(crate) fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub(crate) fn span_from(&self, start: usize, start_line: usize, start_col: usize) -> Span {
        Span::new(start, self.pos, start_line, start_col)
    }

    pub(crate) fn next_token(&mut self) -> Result<Token, LottieError> {
        self.skip_whitespace();

        let start = self.pos;
        let start_line = self.line;
        let start_col = self.col;

        let ch = match self.current() {
            Some(ch) => ch,
            None => {
                return Ok(Token::new(
                    TokenKind::Eof,
                    self.span_from(start, start_line, start_col),
                ));
            }
        };

        if ch == '/' && self.peek() == Some('/') {
            return self.lex_line_comment();
        }
        if ch == '"' {
            return self.lex_string(start, start_line, start_col);
        }
        if ch == '#' {
            return self.lex_color(start, start_line, start_col);
        }
        if ch.is_ascii_digit() {
            return self.lex_number(start, start_line, start_col);
        }
        if ch == '-' && self.peek().is_some_and(|c| c.is_ascii_digit() || c == '.') {
            return self.lex_negative_number(start, start_line, start_col);
        }
        if ch.is_ascii_alphabetic() || ch == '_' {
            return self.lex_ident_or_keyword(start, start_line, start_col);
        }

        let kind = match ch {
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '=' => TokenKind::Equals,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ':' => TokenKind::Colon,
            '+' => TokenKind::Plus,
            _ => {
                return Err(LottieError::new(
                    ErrorKind::UnexpectedChar,
                    format!("意外字符 `{ch}`"),
                    Some(self.span_from(start, start_line, start_col)),
                ));
            }
        };

        self.advance();
        Ok(Token::new(
            kind,
            self.span_from(start, start_line, start_col),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_emits_only_eof() {
        let mut l = Lexer::new("");
        let toks = l.tokenize().unwrap();
        assert_eq!(toks.len(), 1);
        assert_eq!(toks[0].kind, TokenKind::Eof);
    }

    #[test]
    fn whitespace_skipped() {
        let mut l = Lexer::new("   \n\t  ");
        let toks = l.tokenize().unwrap();
        assert_eq!(toks.len(), 1);
        assert_eq!(toks[0].kind, TokenKind::Eof);
    }

    #[test]
    fn line_comment_skipped() {
        let mut l = Lexer::new("// comment\n42");
        let toks = l.tokenize().unwrap();
        assert!(matches!(toks[0].kind, TokenKind::IntLit(42)));
    }

    #[test]
    fn punctuation_tokens() {
        let mut l = Lexer::new("{ } ( ) [ ] = , . : +");
        let toks = l.tokenize().unwrap();
        let kinds: Vec<_> = toks.into_iter().map(|t| t.kind).collect();
        assert_eq!(kinds[0], TokenKind::LBrace);
        assert_eq!(kinds[1], TokenKind::RBrace);
        assert_eq!(kinds[6], TokenKind::Equals);
        assert_eq!(kinds[7], TokenKind::Comma);
    }

    #[test]
    fn unexpected_char_errors() {
        let mut l = Lexer::new("@");
        let err = l.tokenize().unwrap_err();
        assert_eq!(err.kind, ErrorKind::UnexpectedChar);
    }

    #[test]
    fn span_tracks_line_and_col() {
        let mut l = Lexer::new("foo\nbar");
        let toks = l.tokenize().unwrap();
        assert_eq!(toks[0].span.line, 1);
        assert_eq!(toks[1].span.line, 2);
    }
}
