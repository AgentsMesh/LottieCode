//! 字符串字面量扫描。

use crate::error::{ErrorKind, LottieError};
use crate::token::{Token, TokenKind};

use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn lex_string(
        &mut self,
        start: usize,
        sl: usize,
        sc: usize,
    ) -> Result<Token, LottieError> {
        self.advance();
        let content_start = self.pos;
        loop {
            match self.current() {
                None => {
                    return Err(LottieError::new(
                        ErrorKind::UnterminatedString,
                        "未闭合的字符串字面量",
                        Some(self.span_from(start, sl, sc)),
                    )
                    .with_hint("用 `\"` 闭合字符串"));
                }
                Some('"') => {
                    let content: String = self.chars[content_start..self.pos].iter().collect();
                    self.advance();
                    return Ok(Token::new(
                        TokenKind::StringLit(content),
                        self.span_from(start, sl, sc),
                    ));
                }
                Some('\\') => {
                    self.advance();
                    self.advance();
                }
                Some(_) => {
                    self.advance();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::TokenKind;

    #[test]
    fn simple_string() {
        let mut l = Lexer::new(r#""hello""#);
        let toks = l.tokenize().unwrap();
        assert!(matches!(&toks[0].kind, TokenKind::StringLit(s) if s == "hello"));
    }

    #[test]
    fn empty_string() {
        let mut l = Lexer::new(r#""""#);
        let toks = l.tokenize().unwrap();
        assert!(matches!(&toks[0].kind, TokenKind::StringLit(s) if s.is_empty()));
    }

    #[test]
    fn unterminated_string_errors() {
        let mut l = Lexer::new(r#""no-end"#);
        let err = l.tokenize().unwrap_err();
        assert_eq!(err.kind, crate::error::ErrorKind::UnterminatedString);
    }

    #[test]
    fn unicode_string() {
        let mut l = Lexer::new(r#""中文 emoji""#);
        let toks = l.tokenize().unwrap();
        assert!(matches!(&toks[0].kind, TokenKind::StringLit(s) if s == "中文 emoji"));
    }
}
