//! 颜色字面量 `#RRGGBB` / `#RRGGBBAA` 扫描。

use crate::error::{ErrorKind, LottieError};
use crate::token::{Token, TokenKind};

use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn lex_color(
        &mut self,
        start: usize,
        sl: usize,
        sc: usize,
    ) -> Result<Token, LottieError> {
        self.advance();
        let hex_start = self.pos;
        while let Some(ch) = self.current() {
            if ch.is_ascii_hexdigit() {
                self.advance();
            } else {
                break;
            }
        }
        let hex: String = self.chars[hex_start..self.pos].iter().collect();
        if hex.len() != 6 && hex.len() != 8 {
            return Err(LottieError::new(
                ErrorKind::InvalidColor,
                format!("无效颜色字面量 `#{hex}`，需要 6 或 8 位十六进制"),
                Some(self.span_from(start, sl, sc)),
            )
            .with_hint("使用格式 #RRGGBB 或 #RRGGBBAA"));
        }
        Ok(Token::new(
            TokenKind::ColorLit(hex),
            self.span_from(start, sl, sc),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ErrorKind;
    use crate::lexer::Lexer;
    use crate::token::TokenKind;

    #[test]
    fn rgb_six_digits() {
        let mut l = Lexer::new("#FF8800");
        let toks = l.tokenize().unwrap();
        assert!(matches!(&toks[0].kind, TokenKind::ColorLit(s) if s == "FF8800"));
    }

    #[test]
    fn rgba_eight_digits() {
        let mut l = Lexer::new("#11223344");
        let toks = l.tokenize().unwrap();
        assert!(matches!(&toks[0].kind, TokenKind::ColorLit(s) if s == "11223344"));
    }

    #[test]
    fn invalid_length_errors() {
        let mut l = Lexer::new("#ABC");
        let err = l.tokenize().unwrap_err();
        assert_eq!(err.kind, ErrorKind::InvalidColor);
    }

    #[test]
    fn lowercase_hex_accepted() {
        let mut l = Lexer::new("#abcdef");
        let toks = l.tokenize().unwrap();
        assert!(matches!(&toks[0].kind, TokenKind::ColorLit(s) if s == "abcdef"));
    }
}
