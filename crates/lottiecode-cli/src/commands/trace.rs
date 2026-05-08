//! `lc trace` —— 位图（PNG/JPG/...）矢量化为 SVG 或 lc DSL（基于 vtracer）。

use std::path::{Path, PathBuf};

use lottiecode_trace::TraceConfig;

/// 输出格式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Format {
    Svg,
    Lc,
}

#[allow(clippy::too_many_arguments)]
pub fn run(
    file: &Path,
    output: Option<&Path>,
    stdout: bool,
    as_format: Option<&str>,
    mode: &str,
    color_mode: &str,
    filter_speckle: usize,
    color_precision: i32,
) -> Result<(), String> {
    let cfg = TraceConfig::from_strs(mode, color_mode, filter_speckle, color_precision)
        .map_err(|e| e.to_string())?;
    let format = pick_format(output, stdout, as_format)?;

    let body = match format {
        Format::Svg => lottiecode_trace::trace_to_svg(file, &cfg),
        Format::Lc => lottiecode_trace::trace_to_lc(file, &cfg),
    }
    .map_err(|e| e.to_string())?;

    write_or_print(output, stdout, &body)
}

/// 根据 `-o` 后缀或 `--stdout --as` 决定输出格式；其它情况报错。
fn pick_format(output: Option<&Path>, stdout: bool, as_format: Option<&str>) -> Result<Format, String> {
    if stdout {
        let kind = as_format.ok_or_else(|| {
            "使用 --stdout 时必须配合 --as svg|lc 指定格式".to_string()
        })?;
        return parse_format(kind);
    }
    let out = output.ok_or_else(|| {
        "请用 -o foo.svg / -o foo.lc 指定输出，或 --stdout --as svg|lc".to_string()
    })?;
    let ext = out
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase());
    match ext.as_deref() {
        Some("svg") => Ok(Format::Svg),
        Some("lc") => Ok(Format::Lc),
        Some(other) => Err(format!("不识别的输出后缀 .{other}，仅支持 .svg / .lc")),
        None => Err("输出路径缺少 .svg / .lc 后缀".into()),
    }
}

fn parse_format(s: &str) -> Result<Format, String> {
    match s.to_ascii_lowercase().as_str() {
        "svg" => Ok(Format::Svg),
        "lc" => Ok(Format::Lc),
        other => Err(format!("--as 仅支持 svg | lc，收到：{other}")),
    }
}

fn write_or_print(output: Option<&Path>, stdout: bool, body: &str) -> Result<(), String> {
    if stdout {
        print!("{body}");
        return Ok(());
    }
    // pick_format 已经保证此分支下 output 非空。
    let out: PathBuf = output
        .ok_or_else(|| "内部错误：缺少输出路径".to_string())?
        .to_path_buf();
    std::fs::write(&out, body).map_err(|e| format!("写入 {} 失败：{e}", out.display()))?;
    println!("✔ 已生成 {}", out.display());
    Ok(())
}
