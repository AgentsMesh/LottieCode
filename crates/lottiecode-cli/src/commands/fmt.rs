//! `lc fmt` —— 格式化 DSL 源文件。

use std::fs;
use std::path::Path;

use lottiecode_lang::formatter::format;
use lottiecode_lang::lexer::Lexer;
use lottiecode_lang::parser::Parser;

pub fn run(file: &Path, write: bool) -> Result<(), String> {
    let source = fs::read_to_string(file)
        .map_err(|e| format!("无法读取 {}: {e}", file.display()))?;
    let filename = file.to_string_lossy();

    let mut lexer = Lexer::new(&source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| e.format(&source, &filename))?;
    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| e.format(&source, &filename))?;

    let formatted = format(&program);
    if write {
        fs::write(file, formatted)
            .map_err(|e| format!("无法写回 {}: {e}", file.display()))?;
        println!("✔ 已格式化 {}", file.display());
    } else {
        print!("{formatted}");
    }
    Ok(())
}
