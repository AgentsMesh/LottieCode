use super::*;

#[test]
fn span_new_fields() {
    let s = Span::new(5, 10, 2, 3);
    assert_eq!(s.start, 5);
    assert_eq!(s.end, 10);
    assert_eq!(s.line, 2);
    assert_eq!(s.col, 3);
}

#[test]
fn span_dummy_zero() {
    let s = Span::dummy();
    assert_eq!(s.start, 0);
    assert_eq!(s.line, 0);
}

#[test]
fn keyword_composition_recognized() {
    assert!(matches!(keyword("composition"), Some(TokenKind::Composition)));
}

#[test]
fn keyword_bool_literals_handled() {
    assert!(matches!(keyword("true"), Some(TokenKind::BoolLit(true))));
    assert!(matches!(keyword("false"), Some(TokenKind::BoolLit(false))));
}

#[test]
fn keyword_unknown_returns_none() {
    assert!(keyword("not-a-keyword").is_none());
    assert!(keyword("").is_none());
}

#[test]
fn token_new_keeps_kind_and_span() {
    let span = Span::new(0, 1, 1, 1);
    let t = Token::new(TokenKind::Composition, span);
    assert!(matches!(t.kind, TokenKind::Composition));
    assert_eq!(t.span.start, 0);
}

#[test]
fn display_all_token_kinds() {
    let kinds = vec![
        TokenKind::Composition,
        TokenKind::Token,
        TokenKind::Slots,
        TokenKind::Shape,
        TokenKind::Text,
        TokenKind::Image,
        TokenKind::Precomp,
        TokenKind::Asset,
        TokenKind::Mask,
        TokenKind::Solid,
        TokenKind::Controller,
        TokenKind::Camera,
        TokenKind::Component,
        TokenKind::Use,
        TokenKind::Include,
        TokenKind::Animate,
        TokenKind::Loop,
        TokenKind::Hold,
        TokenKind::Typewriter,
        TokenKind::Rect,
        TokenKind::Ellipse,
        TokenKind::Path,
        TokenKind::PolyStar,
        TokenKind::Fill,
        TokenKind::Stroke,
        TokenKind::Trim,
        TokenKind::Ident("foo".into()),
        TokenKind::StringLit("hi".into()),
        TokenKind::IntLit(42),
        TokenKind::FloatLit(1.5),
        TokenKind::BoolLit(true),
        TokenKind::TimeSec(1.0),
        TokenKind::TimeMs(500.0),
        TokenKind::TimeFrames(30),
        TokenKind::ColorLit("FF0000".into()),
        TokenKind::LBrace,
        TokenKind::RBrace,
        TokenKind::LParen,
        TokenKind::RParen,
        TokenKind::LBracket,
        TokenKind::RBracket,
        TokenKind::Equals,
        TokenKind::Comma,
        TokenKind::Dot,
        TokenKind::Colon,
        TokenKind::Plus,
        TokenKind::Eof,
    ];
    for k in kinds {
        let s = format!("{k}");
        assert!(!s.is_empty(), "{k:?} 应有非空 Display");
    }
}

#[test]
fn keyword_recognizes_all_remaining() {
    for kw in [
        "slots",
        "text",
        "image",
        "precomp",
        "asset",
        "mask",
        "solid",
        "controller",
        "camera",
        "component",
        "use",
        "include",
        "animate",
        "loop",
        "hold",
        "typewriter",
        "rect",
        "ellipse",
        "path",
        "polystar",
        "fill",
        "stroke",
        "trim",
    ] {
        assert!(keyword(kw).is_some(), "`{kw}` 应被识别");
    }
}
