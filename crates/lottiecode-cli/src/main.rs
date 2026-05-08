//! lottiecode-cli —— 命令行入口

mod commands;
mod pipeline;

#[cfg(test)]
mod cli_tests;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lc", version, about = "LottieCode —— Motion DSL 编译器")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 仅验证 DSL 文件是否合法。
    Check {
        file: PathBuf,
    },
    /// 编译为 Lottie JSON。
    Build {
        file: PathBuf,
        /// 输出路径（默认 file 同名 .json）。
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// 紧凑输出（去除空白）。
        #[arg(long)]
        compact: bool,
    },
    /// 树形打印 IR。
    Plan {
        file: PathBuf,
    },
    /// 输出完整 DSL 语法参考（供 LLM 上下文）。
    Syntax,
    /// 格式化 DSL 源文件。
    Fmt {
        file: PathBuf,
        /// 直接写回文件（默认输出到 stdout）。
        #[arg(short, long)]
        write: bool,
    },
    /// 检视编译后的 IR。
    Inspect {
        file: PathBuf,
        /// 输出格式化的 Lottie JSON。
        #[arg(long)]
        json: bool,
    },
    /// 把 Lottie JSON 反编译为 DSL（实验性）。
    Decompile {
        file: PathBuf,
        /// 输出路径（默认 stdout）。
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// 位图矢量化为 SVG / lc DSL（基于 vtracer）。
    Trace {
        /// 输入位图（PNG / JPG / BMP / GIF）。
        file: PathBuf,
        /// 输出路径；后缀决定格式（.svg | .lc）。与 --stdout 互斥。
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// 输出到 stdout（必须同时指定 --as）。
        #[arg(long)]
        stdout: bool,
        /// 与 --stdout 配对：svg | lc。
        #[arg(long = "as", value_parser = ["svg", "lc"])]
        as_format: Option<String>,
        /// 路径简化模式：spline | polygon | none。
        #[arg(long, default_value = "spline")]
        mode: String,
        /// 颜色模式：color | binary。
        #[arg(long = "color-mode", default_value = "color")]
        color_mode: String,
        /// 过滤小于该面积平方根的杂点。
        #[arg(long = "filter-speckle", default_value_t = 4)]
        filter_speckle: usize,
        /// 颜色聚类精度（位深，1-8）。
        #[arg(long = "color-precision", default_value_t = 6)]
        color_precision: i32,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Check { file } => commands::check::run(&file),
        Command::Build {
            file,
            output,
            compact,
        } => commands::build::run(&file, output.as_deref(), compact),
        Command::Plan { file } => commands::plan::run(&file),
        Command::Syntax => commands::syntax::run(),
        Command::Fmt { file, write } => commands::fmt::run(&file, write),
        Command::Inspect { file, json } => commands::inspect::run(&file, json),
        Command::Decompile { file, output } => commands::decompile::run(&file, output.as_deref()),
        Command::Trace {
            file,
            output,
            stdout,
            as_format,
            mode,
            color_mode,
            filter_speckle,
            color_precision,
        } => commands::trace::run(
            &file,
            output.as_deref(),
            stdout,
            as_format.as_deref(),
            &mode,
            &color_mode,
            filter_speckle,
            color_precision,
        ),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}
