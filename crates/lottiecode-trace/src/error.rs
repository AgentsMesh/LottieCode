//! lottiecode-trace 错误类型。

use std::fmt;

#[derive(Debug)]
pub enum TraceError {
    /// 文件 I/O 失败（读取位图失败、找不到文件等）。
    Io(String),
    /// 位图解码失败（不支持的格式 / 损坏文件）。
    InvalidImage(String),
    /// vtracer 内部失败（聚类 / 路径追踪报错）。
    VtracerFailed(String),
    /// 用户传入的配置非法（mode/color_mode 字符串不识别等）。
    BadConfig(String),
}

impl fmt::Display for TraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceError::Io(m) => write!(f, "I/O 错误：{m}"),
            TraceError::InvalidImage(m) => write!(f, "位图解码失败：{m}"),
            TraceError::VtracerFailed(m) => write!(f, "矢量化失败：{m}"),
            TraceError::BadConfig(m) => write!(f, "配置错误：{m}"),
        }
    }
}

impl std::error::Error for TraceError {}

pub type Result<T> = std::result::Result<T, TraceError>;
