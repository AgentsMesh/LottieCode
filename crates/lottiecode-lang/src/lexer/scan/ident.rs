//! 标识符 / 关键字扫描。
//!
//! LottieCode 允许标识符内部含 `-`（如 `ease-out-back`），
//! 但 `-` 后必须紧跟字母/数字，否则当作其他符号。

use crate::error::LottieError;
use crate::token::{keyword, Token, TokenKind};

use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn lex_ident_or_keyword(
        &mut self,
        start: usize,
        sl: usize,
        sc: usize,
    ) -> Result<Token, LottieError> {
        let ident_start = self.pos;
        while let Some(ch) = self.current() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.advance();
            } else if ch == '-' && self.peek().is_some_and(|c| c.is_ascii_alphanumeric()) {
                self.advance();
            } else {
                break;
            }
        }
        let ident: String = self.chars[ident_start..self.pos].iter().collect();
        let kind = keyword(&ident).unwrap_or(TokenKind::Ident(ident));
        Ok(Token::new(kind, self.span_from(start, sl, sc)))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::TokenKind;

    fn first(src: &str) -> TokenKind {
        let mut l = Lexer::new(src);
        l.tokenize().unwrap().into_iter().next().unwrap().kind
    }

    #[test]
    fn plain_ident() {
        assert!(matches!(first("foo"), TokenKind::Ident(s) if s == "foo"));
    }

    #[test]
    fn ident_with_underscore() {
        assert!(matches!(first("my_var"), TokenKind::Ident(s) if s == "my_var"));
    }

    #[test]
    fn hyphen_continues_ident() {
        assert!(matches!(first("ease-out"), TokenKind::Ident(s) if s == "ease-out"));
    }

    #[test]
    fn keyword_recognized() {
        assert!(matches!(first("composition"), TokenKind::Composition));
    }

    #[test]
    fn shape_keyword_recognized() {
        assert!(matches!(first("shape"), TokenKind::Shape));
    }
}
