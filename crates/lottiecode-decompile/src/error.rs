//! Decompile 错误。

use std::fmt;

#[derive(Debug)]
pub enum DecompileError {
    /// JSON 解析失败。
    InvalidJson(String),
    /// 必需字段缺失。
    MissingField(&'static str),
    /// 字段类型不符合预期。
    TypeMismatch(&'static str),
}

impl fmt::Display for DecompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecompileError::InvalidJson(m) => write!(f, "JSON 解析失败：{m}"),
            DecompileError::MissingField(name) => write!(f, "缺少必需字段 `{name}`"),
            DecompileError::TypeMismatch(name) => write!(f, "字段 `{name}` 类型错误"),
        }
    }
}

impl std::error::Error for DecompileError {}

pub type Result<T> = std::result::Result<T, DecompileError>;
