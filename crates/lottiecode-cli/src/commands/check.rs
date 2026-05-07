//! `lc check` —— 仅做语法语义验证。

use std::path::Path;

use crate::pipeline::compile_file;

pub fn run(file: &Path) -> Result<(), String> {
    compile_file(file)?;
    println!("✔ {} 验证通过", file.display());
    Ok(())
}
