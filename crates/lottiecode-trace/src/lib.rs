//! lottiecode-trace —— 位图 → SVG / lc DSL 矢量化器。
//!
//! 基于 [vtracer](https://crates.io/crates/vtracer) 的薄壳，把它的 SVG 输出再翻译为
//! LottieCode DSL，让用户通过 `lc trace foo.png -o out.lc` 一条命令完成
//! 「位图 → 矢量 → Lottie」的链路。
//!
//! # 公开 API
//! - [`trace_to_svg`]：位图文件 → SVG 字符串
//! - [`trace_to_lc`]：位图文件 → lc DSL 字符串
//! - [`TraceConfig`]：旋钮（mode / color_mode / filter_speckle / color_precision）
//! - [`TraceError`] / [`Result`]：统一错误类型

mod config;
mod error;
mod lc_emit;
mod svg_emit;

use std::path::Path;

pub use config::TraceConfig;
pub use error::{Result, TraceError};

/// 入口：位图文件 → SVG 字符串。
pub fn trace_to_svg(input: &Path, cfg: &TraceConfig) -> Result<String> {
    svg_emit::raster_to_svg_string(input, cfg)
}

/// 入口：位图文件 → lc DSL 字符串。
pub fn trace_to_lc(input: &Path, cfg: &TraceConfig) -> Result<String> {
    let svg = svg_emit::raster_to_svg_string(input, cfg)?;
    let label = input
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("input");
    lc_emit::svg_to_lc(&svg, label)
}
