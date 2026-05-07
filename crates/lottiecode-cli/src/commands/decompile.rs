//! `lc decompile` —— Lottie JSON → DSL（实验性反向编译）。

use std::path::Path;

pub fn run(file: &Path, output: Option<&Path>) -> Result<(), String> {
    let raw = std::fs::read_to_string(file)
        .map_err(|e| format!("无法读取 {}: {e}", file.display()))?;
    let dsl = lottiecode_decompile::decompile_str(&raw)
        .map_err(|e| format!("反编译失败：{e}"))?;
    match output {
        Some(out) => {
            std::fs::write(out, &dsl).map_err(|e| format!("写入 {} 失败：{e}", out.display()))?;
            println!("✔ 已生成 {}", out.display());
        }
        None => println!("{dsl}"),
    }
    Ok(())
}
