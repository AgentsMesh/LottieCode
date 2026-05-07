//! Lexer 各 token 类型的扫描方法分发。
//!
//! 子模块按 token 类型拆分，各自向 `Lexer` 添加扫描方法。

mod color;
mod ident;
mod number;
mod string;

use crate::error::LottieError;
use crate::token::Token;

use super::Lexer;

impl Lexer<'_> {
    pub(crate) fn lex_line_comment(&mut self) -> Result<Token, LottieError> {
        self.advance();
        self.advance();
        while let Some(ch) = self.current() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
        self.next_token()
    }
}
