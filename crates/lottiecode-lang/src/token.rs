//! Token 与 Span —— 词法单元定义
//!
//! 参考 VEAC 风格，TokenKind 覆盖 DSL 全部关键字、字面量、标点。

use std::fmt;

/// 源码位置，用于错误报告。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, col: usize) -> Self {
        Self { start, end, line, col }
    }

    /// 占位 Span，用于内部生成的节点。
    pub fn dummy() -> Self {
        Self { start: 0, end: 0, line: 0, col: 0 }
    }
}

/// LottieCode DSL 的所有 Token 类型。
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // -- 顶层关键字 --
    Composition,
    Token,
    Slots,
    Shape,
    Text,
    Image,
    Precomp,
    Asset,
    Mask,
    Solid,
    Controller,
    Camera,
    Component,
    Use,
    Include,
    Animate,
    Loop,
    Hold,
    Typewriter,

    // -- 几何关键字 --
    Rect,
    Ellipse,
    Path,
    PolyStar,

    // -- 样式关键字 --
    Fill,
    Stroke,
    Trim,

    // -- 标识符与字面量 --
    Ident(String),
    StringLit(String),
    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),

    // -- 时间字面量 --
    TimeSec(f64),     // 0.5s
    TimeMs(f64),      // 500ms
    TimeFrames(u64),  // 30f

    // -- 颜色字面量 --
    ColorLit(String), // #RRGGBB 或 #RRGGBBAA

    // -- 标点 --
    LBrace,    // {
    RBrace,    // }
    LParen,    // (
    RParen,    // )
    LBracket,  // [
    RBracket,  // ]
    Equals,    // =
    Comma,     // ,
    Dot,       // .
    Colon,     // :
    Plus,      // +

    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Composition => write!(f, "composition"),
            TokenKind::Token => write!(f, "token"),
            TokenKind::Slots => write!(f, "slots"),
            TokenKind::Shape => write!(f, "shape"),
            TokenKind::Text => write!(f, "text"),
            TokenKind::Image => write!(f, "image"),
            TokenKind::Precomp => write!(f, "precomp"),
            TokenKind::Asset => write!(f, "asset"),
            TokenKind::Mask => write!(f, "mask"),
            TokenKind::Solid => write!(f, "solid"),
            TokenKind::Controller => write!(f, "controller"),
            TokenKind::Camera => write!(f, "camera"),
            TokenKind::Component => write!(f, "component"),
            TokenKind::Use => write!(f, "use"),
            TokenKind::Include => write!(f, "include"),
            TokenKind::Animate => write!(f, "animate"),
            TokenKind::Loop => write!(f, "loop"),
            TokenKind::Hold => write!(f, "hold"),
            TokenKind::Typewriter => write!(f, "typewriter"),
            TokenKind::Rect => write!(f, "rect"),
            TokenKind::Ellipse => write!(f, "ellipse"),
            TokenKind::Path => write!(f, "path"),
            TokenKind::PolyStar => write!(f, "polystar"),
            TokenKind::Fill => write!(f, "fill"),
            TokenKind::Stroke => write!(f, "stroke"),
            TokenKind::Trim => write!(f, "trim"),
            TokenKind::Ident(s) => write!(f, "标识符 `{s}`"),
            TokenKind::StringLit(s) => write!(f, "字符串 \"{s}\""),
            TokenKind::IntLit(n) => write!(f, "整数 {n}"),
            TokenKind::FloatLit(n) => write!(f, "浮点 {n}"),
            TokenKind::BoolLit(b) => write!(f, "布尔 {b}"),
            TokenKind::TimeSec(v) => write!(f, "{v}s"),
            TokenKind::TimeMs(v) => write!(f, "{v}ms"),
            TokenKind::TimeFrames(n) => write!(f, "{n}f"),
            TokenKind::ColorLit(c) => write!(f, "颜色 #{c}"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::Equals => write!(f, "="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Eof => write!(f, "文件结束"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// 把保留字字符串映射到 TokenKind。
pub(crate) fn keyword(s: &str) -> Option<TokenKind> {
    Some(match s {
        "composition" => TokenKind::Composition,
        "token" => TokenKind::Token,
        "slots" => TokenKind::Slots,
        "shape" => TokenKind::Shape,
        "text" => TokenKind::Text,
        "image" => TokenKind::Image,
        "precomp" => TokenKind::Precomp,
        "asset" => TokenKind::Asset,
        "mask" => TokenKind::Mask,
        "solid" => TokenKind::Solid,
        "controller" => TokenKind::Controller,
        "camera" => TokenKind::Camera,
        "component" => TokenKind::Component,
        "use" => TokenKind::Use,
        "include" => TokenKind::Include,
        "animate" => TokenKind::Animate,
        "loop" => TokenKind::Loop,
        "hold" => TokenKind::Hold,
        "typewriter" => TokenKind::Typewriter,
        "rect" => TokenKind::Rect,
        "ellipse" => TokenKind::Ellipse,
        "path" => TokenKind::Path,
        "polystar" => TokenKind::PolyStar,
        "fill" => TokenKind::Fill,
        "stroke" => TokenKind::Stroke,
        "trim" => TokenKind::Trim,
        "true" => TokenKind::BoolLit(true),
        "false" => TokenKind::BoolLit(false),
        _ => return None,
    })
}


#[cfg(test)]
#[path = "token_tests.rs"]
mod tests;
