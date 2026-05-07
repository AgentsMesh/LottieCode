//! lottiecode-lang —— DSL 编译前端
//!
//! 流水线：lexer → parser → ast → semantic → ir
//! IR 完全 resolved，可直接交给 codegen 输出 Lottie JSON。

pub mod ast;
pub mod error;
pub mod formatter;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod token;

#[cfg(test)]
mod e2e_tests;

#[cfg(test)]
mod tests {
    use super::lexer::Lexer;
    use super::parser::Parser;
    use super::semantic::SemanticAnalyzer;
    use super::token::TokenKind;

    /// 把源码全程跑通 lex + parse + semantic，返回 IR 或可读错误。
    fn compile(src: &str) -> Result<crate::ir::IrAnimation, String> {
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().map_err(|e| e.format(src, "<test>"))?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| e.format(src, "<test>"))?;
        SemanticAnalyzer::analyze(&program).map_err(|e| e.format(src, "<test>"))
    }

    #[test]
    fn lex_keywords_and_literals() {
        let mut l = Lexer::new(r#"composition "x" { width=100 } #FF8800 1.5s 30f"#);
        let toks = l.tokenize().unwrap();
        let kinds: Vec<_> = toks.into_iter().map(|t| t.kind).collect();
        assert!(matches!(kinds[0], TokenKind::Composition));
        assert!(matches!(kinds[1], TokenKind::StringLit(ref s) if s == "x"));
        assert!(kinds.iter().any(|k| matches!(k, TokenKind::ColorLit(s) if s == "FF8800")));
        assert!(kinds.iter().any(|k| matches!(k, TokenKind::TimeSec(v) if (*v - 1.5).abs() < 1e-9)));
        assert!(kinds.iter().any(|k| matches!(k, TokenKind::TimeFrames(30))));
    }

    #[test]
    fn lex_hyphenated_ident() {
        let mut l = Lexer::new("ease-out-back trim-end");
        let toks = l.tokenize().unwrap();
        assert!(matches!(&toks[0].kind, TokenKind::Ident(s) if s == "ease-out-back"));
        assert!(matches!(&toks[1].kind, TokenKind::Ident(s) if s == "trim-end"));
    }

    #[test]
    fn parse_minimal_composition() {
        let ir = compile(
            r#"composition "demo" { width=100 height=100 fps=30 duration=1s }"#,
        )
        .unwrap();
        assert_eq!(ir.name, "demo");
        assert_eq!(ir.width, 100);
        assert_eq!(ir.fps, 30.0);
        assert_eq!(ir.out_frame, 30.0);
    }

    #[test]
    fn semantic_token_resolution() {
        let ir = compile(
            r#"
            token color { brand = #FF0000 }
            composition "t" {
                width=10 height=10 fps=30 duration=1s
                shape s {
                    rect { size=[10,10] }
                    fill = color.brand
                }
            }
            "#,
        )
        .unwrap();
        assert_eq!(ir.layers.len(), 1);
    }

    #[test]
    fn semantic_keyframe_order_check() {
        let err = compile(
            r#"
            composition "x" {
                width=10 height=10 fps=30 duration=1s
                shape s {
                    rect { size=[10,10] }
                    fill = #FF0000
                    animate opacity {
                        1s: 100
                        0s: 0
                    }
                }
            }
            "#,
        )
        .unwrap_err();
        assert!(err.contains("KeyframeOutOfOrder") || err.contains("严格递增"));
    }

    #[test]
    fn missing_composition_errors() {
        let err = compile(r#"token x { y = 1 }"#).unwrap_err();
        assert!(err.contains("composition"));
    }
}
