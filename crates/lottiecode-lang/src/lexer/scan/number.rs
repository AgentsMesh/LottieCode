//! 数字字面量与时间后缀（s / ms / f）扫描。

use crate::error::{ErrorKind, LottieError};
use crate::token::{Span, Token, TokenKind};

use crate::lexer::Lexer;

impl Lexer<'_> {
    pub(crate) fn lex_number(
        &mut self,
        start: usize,
        sl: usize,
        sc: usize,
    ) -> Result<Token, LottieError> {
        let num_start = self.pos;
        let mut has_dot = false;
        while let Some(ch) = self.current() {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' && !has_dot && self.peek().is_some_and(|n| n.is_ascii_digit()) {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }
        let num_str: String = self.chars[num_start..self.pos].iter().collect();
        let span = self.span_from(start, sl, sc);
        self.try_lex_time_suffix(&num_str, has_dot, span)
    }

    fn try_lex_time_suffix(
        &mut self,
        num_str: &str,
        has_dot: bool,
        span: Span,
    ) -> Result<Token, LottieError> {
        if let Some(ch) = self.current() {
            if ch == 's' && !self.peek().is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') {
                self.advance();
                let val: f64 = num_str.parse().map_err(|_| invalid_time(num_str, "s", span))?;
                return Ok(Token::new(TokenKind::TimeSec(val), span));
            }
            if ch == 'm' && self.peek() == Some('s') {
                self.advance();
                self.advance();
                let val: f64 = num_str
                    .parse()
                    .map_err(|_| invalid_time(num_str, "ms", span))?;
                return Ok(Token::new(TokenKind::TimeMs(val), span));
            }
            if ch == 'f'
                && !has_dot
                && !self.peek().is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
            {
                self.advance();
                let val: u64 = num_str.parse().map_err(|_| invalid_time(num_str, "f", span))?;
                return Ok(Token::new(TokenKind::TimeFrames(val), span));
            }
        }

        if has_dot {
            let val: f64 = num_str.parse().map_err(|_| {
                LottieError::new(
                    ErrorKind::InvalidNumber,
                    format!("无效浮点字面量 `{num_str}`"),
                    Some(span),
                )
            })?;
            Ok(Token::new(TokenKind::FloatLit(val), span))
        } else {
            let val: i64 = num_str.parse().map_err(|_| {
                LottieError::new(
                    ErrorKind::InvalidNumber,
                    format!("无效整数字面量 `{num_str}`"),
                    Some(span),
                )
            })?;
            Ok(Token::new(TokenKind::IntLit(val), span))
        }
    }

    pub(crate) fn lex_negative_number(
        &mut self,
        start: usize,
        sl: usize,
        sc: usize,
    ) -> Result<Token, LottieError> {
        self.advance();
        let tok = self.lex_number(start, sl, sc)?;
        let negated = match tok.kind {
            TokenKind::IntLit(n) => TokenKind::IntLit(-n),
            TokenKind::FloatLit(n) => TokenKind::FloatLit(-n),
            TokenKind::TimeSec(n) => TokenKind::TimeSec(-n),
            TokenKind::TimeMs(n) => TokenKind::TimeMs(-n),
            other => other,
        };
        Ok(Token::new(negated, tok.span))
    }
}

fn invalid_time(num: &str, suffix: &str, span: Span) -> LottieError {
    LottieError::new(
        ErrorKind::InvalidTimeLiteral,
        format!("无效时间字面量 `{num}{suffix}`"),
        Some(span),
    )
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
    fn integer_literal() {
        assert!(matches!(first("42"), TokenKind::IntLit(42)));
    }

    #[test]
    fn float_literal() {
        assert!(matches!(first("3.14"), TokenKind::FloatLit(v) if (v - 3.14).abs() < 1e-9));
    }

    #[test]
    fn time_seconds() {
        assert!(matches!(first("1.5s"), TokenKind::TimeSec(v) if (v - 1.5).abs() < 1e-9));
    }

    #[test]
    fn time_milliseconds() {
        assert!(matches!(first("250ms"), TokenKind::TimeMs(v) if (v - 250.0).abs() < 1e-9));
    }

    #[test]
    fn time_frames() {
        assert!(matches!(first("30f"), TokenKind::TimeFrames(30)));
    }

    #[test]
    fn negative_int() {
        assert!(matches!(first("-7"), TokenKind::IntLit(-7)));
    }

    #[test]
    fn negative_float_seconds() {
        assert!(matches!(first("-0.5s"), TokenKind::TimeSec(v) if (v + 0.5).abs() < 1e-9));
    }
}
