//! `lc inspect` —— 输出编译后的 IR（调试用）。
//!
//! 默认 Debug 风格，`--json` 输出 codegen 产出的 Lottie JSON（已格式化）。

use std::path::Path;

use lottiecode_codegen::generate_string;

use crate::pipeline::compile_file;

pub fn run(file: &Path, json: bool) -> Result<(), String> {
    let ir = compile_file(file)?;
    if json {
        println!("{}", generate_string(&ir));
    } else {
        println!("{ir:#?}");
    }
    Ok(())
}
