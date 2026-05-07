//! `lc build` —— 编译为 Lottie JSON 或 dotLottie zip。

use std::fs;
use std::path::{Path, PathBuf};

use lottiecode_codegen::{dotlottie, generate_compact, generate_string};

use crate::pipeline::compile_file;

pub fn run(file: &Path, output: Option<&Path>, compact: bool) -> Result<(), String> {
    let ir = compile_file(file)?;

    let out_path: PathBuf = match output {
        Some(p) => p.to_path_buf(),
        None => file.with_extension("json"),
    };

    if out_path.extension().and_then(|s| s.to_str()) == Some("lottie") {
        let bytes = dotlottie::pack(&ir).map_err(|e| format!("dotLottie 打包失败：{e}"))?;
        fs::write(&out_path, bytes)
            .map_err(|e| format!("无法写入 {}: {e}", out_path.display()))?;
        println!("✔ 已生成 dotLottie {}", out_path.display());
        return Ok(());
    }

    let json = if compact {
        generate_compact(&ir)
    } else {
        generate_string(&ir)
    };
    fs::write(&out_path, json)
        .map_err(|e| format!("无法写入 {}: {e}", out_path.display()))?;
    println!("✔ 已生成 {}", out_path.display());
    Ok(())
}
