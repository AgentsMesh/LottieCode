//! 诊断错误 —— rustc 风格源码上下文输出
//!
//! 参考 VEAC `error.rs`：错误带 Span，可格式化出带下划线的源码上下文。

use crate::token::Span;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LottieError {
    pub kind: ErrorKind,
    pub message: String,
    pub span: Option<Span>,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    // -- Lexer --
    UnexpectedChar,
    UnterminatedString,
    InvalidNumber,
    InvalidTimeLiteral,
    InvalidColor,

    // -- Parser --
    UnexpectedToken,
    ExpectedToken,
    ExpectedBlock,
    ExpectedExpression,

    // -- Semantic --
    UndefinedToken,
    UndefinedComponent,
    UndefinedShape,
    UndefinedEasing,
    DuplicateDefinition,
    TypeMismatch,
    InvalidTimeRange,
    InvalidValue,
    NoComposition,
    KeyframeOutOfOrder,
    IncludeFileNotFound,

    // -- Codegen --
    CodegenFailure,
}

impl LottieError {
    pub fn new(kind: ErrorKind, message: impl Into<String>, span: Option<Span>) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// 输出带源码上下文的诊断（rustc 风格）。
    pub fn format(&self, source: &str, filename: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!("error[{:?}]: {}\n", self.kind, self.message));

        if let Some(span) = &self.span {
            out.push_str(&format!("  --> {}:{}:{}\n", filename, span.line, span.col));

            let lines: Vec<&str> = source.lines().collect();
            if span.line > 0 && span.line <= lines.len() {
                let line_str = lines[span.line - 1];
                let line_num = format!("{}", span.line);
                let padding = " ".repeat(line_num.len());

                out.push_str(&format!("{padding} |\n"));
                out.push_str(&format!("{line_num} | {line_str}\n"));

                let underline_start = if span.col > 0 { span.col - 1 } else { 0 };
                let underline_len = (span.end.saturating_sub(span.start)).max(1);
                out.push_str(&format!(
                    "{padding} | {}{}\n",
                    " ".repeat(underline_start),
                    "^".repeat(underline_len)
                ));
            }
        }

        if let Some(hint) = &self.hint {
            out.push_str(&format!("  = help: {hint}\n"));
        }

        out
    }
}

impl fmt::Display for LottieError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LottieError {}

pub type Result<T> = std::result::Result<T, LottieError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_without_hint() {
        let e = LottieError::new(ErrorKind::TypeMismatch, "bad", None);
        assert!(e.hint.is_none());
        assert_eq!(e.kind, ErrorKind::TypeMismatch);
    }

    #[test]
    fn with_hint_attaches_help_line() {
        let e = LottieError::new(ErrorKind::InvalidValue, "x", None).with_hint("try y");
        assert_eq!(e.hint.as_deref(), Some("try y"));
    }

    #[test]
    fn format_includes_kind_and_filename() {
        let e = LottieError::new(
            ErrorKind::InvalidColor,
            "无效颜色",
            Some(Span::new(0, 5, 1, 1)),
        );
        let out = e.format("hello world", "main.lc");
        assert!(out.contains("error[InvalidColor]"));
        assert!(out.contains("main.lc:1:1"));
    }

    #[test]
    fn format_includes_underline() {
        let e = LottieError::new(
            ErrorKind::UnexpectedToken,
            "意外",
            Some(Span::new(0, 1, 1, 1)),
        );
        let out = e.format("x", "f");
        assert!(out.contains("^"));
    }

    #[test]
    fn format_includes_hint_when_set() {
        let e = LottieError::new(ErrorKind::InvalidValue, "x", None).with_hint("提示");
        let out = e.format("", "f");
        assert!(out.contains("help: 提示"));
    }

    #[test]
    fn display_outputs_message() {
        let e = LottieError::new(ErrorKind::TypeMismatch, "bad", None);
        assert_eq!(format!("{e}"), "bad");
    }
}
