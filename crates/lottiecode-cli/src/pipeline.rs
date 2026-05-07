//! 编译流水线：源文件 → IR。

use std::fs;
use std::path::Path;

use lottiecode_lang::ir::IrAnimation;
use lottiecode_lang::lexer::Lexer;
use lottiecode_lang::parser::Parser;
use lottiecode_lang::semantic::{load_includes, SemanticAnalyzer};

/// 把 DSL 源码字符串编译为 IR（不处理 include）。
#[allow(dead_code)]
pub fn compile_source(source: &str, filename: &str) -> Result<IrAnimation, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| e.format(source, filename))?;

    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| e.format(source, filename))?;

    let ir = SemanticAnalyzer::analyze(&program).map_err(|e| e.format(source, filename))?;
    Ok(ir)
}

/// 读取文件并编译（处理 include 链）。
pub fn compile_file(path: &Path) -> Result<IrAnimation, String> {
    let source = fs::read_to_string(path)
        .map_err(|e| format!("无法读取 {}: {e}", path.display()))?;
    let filename = path.to_string_lossy();

    let mut lexer = Lexer::new(&source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| e.format(&source, &filename))?;

    let mut parser = Parser::new(tokens);
    let mut program = parser
        .parse()
        .map_err(|e| e.format(&source, &filename))?;

    let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
    load_includes(&mut program, base_dir).map_err(|e| e.format(&source, &filename))?;

    let ir = SemanticAnalyzer::analyze(&program).map_err(|e| e.format(&source, &filename))?;
    Ok(ir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_minimal_source_succeeds() {
        let src = r#"composition "x" { width=100 height=100 fps=30 duration=1s }"#;
        let ir = compile_source(src, "<test>").unwrap();
        assert_eq!(ir.name, "x");
        assert_eq!(ir.fps, 30.0);
    }

    #[test]
    fn compile_propagates_lex_errors() {
        let src = r#"composition "missing #ZZZZZZ"#;
        let err = compile_source(src, "<test>").unwrap_err();
        assert!(err.contains("error") || err.contains("错误") || !err.is_empty());
    }

    #[test]
    fn compile_full_shape_pipeline() {
        let src = r#"
composition "demo" {
    width=100 height=100 fps=30 duration=1s
    shape s {
        rect { size=[10,10] position=[0,0] }
        fill = #FF0000
    }
}
"#;
        let ir = compile_source(src, "<test>").unwrap();
        assert_eq!(ir.layers.len(), 1);
    }

    #[test]
    fn compile_with_token_resolution() {
        let src = r#"
token color { brand = #FF0000 }
composition "x" {
    width=10 height=10 fps=30 duration=1s
    shape s {
        rect { size=[10,10] }
        fill = color.brand
    }
}
"#;
        assert!(compile_source(src, "<test>").is_ok());
    }

    #[test]
    fn missing_required_field_errors() {
        let src = r#"composition "x" { width=100 fps=30 duration=1s }"#;
        let err = compile_source(src, "<test>").unwrap_err();
        assert!(err.contains("height"), "expected height-required hint, got: {err}");
    }
}
