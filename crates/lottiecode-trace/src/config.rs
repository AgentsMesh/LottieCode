//! 公开的 TraceConfig + 转换到 vtracer::Config。
//!
//! 仅暴露 4 个最常用旋钮（mode / color_mode / filter_speckle / color_precision），
//! 其他字段沿用 vtracer 默认（见 vtracer::Config::default()）。

use crate::error::{Result, TraceError};
use visioncortex::PathSimplifyMode;
use vtracer::{ColorMode, Config as VConfig};

#[derive(Debug, Clone)]
pub struct TraceConfig {
    pub mode: PathSimplifyMode,
    pub color_mode: ColorMode,
    pub filter_speckle: usize,
    pub color_precision: i32,
}

impl Default for TraceConfig {
    fn default() -> Self {
        // 与 vtracer 默认一致：彩色 + Spline + 4 + 6
        TraceConfig {
            mode: PathSimplifyMode::Spline,
            color_mode: ColorMode::Color,
            filter_speckle: 4,
            color_precision: 6,
        }
    }
}

impl TraceConfig {
    /// 从字符串构造，便于 CLI 直接透传 clap 解析出的 `--mode` / `--color-mode`。
    pub fn from_strs(
        mode: &str,
        color_mode: &str,
        filter_speckle: usize,
        color_precision: i32,
    ) -> Result<Self> {
        let mode = match mode.to_ascii_lowercase().as_str() {
            "spline" => PathSimplifyMode::Spline,
            "polygon" => PathSimplifyMode::Polygon,
            "none" => PathSimplifyMode::None,
            other => {
                return Err(TraceError::BadConfig(format!(
                    "--mode 仅支持 spline | polygon | none，收到：{other}"
                )));
            }
        };
        let color_mode = match color_mode.to_ascii_lowercase().as_str() {
            "color" => ColorMode::Color,
            "binary" => ColorMode::Binary,
            other => {
                return Err(TraceError::BadConfig(format!(
                    "--color-mode 仅支持 color | binary，收到：{other}"
                )));
            }
        };
        Ok(TraceConfig {
            mode,
            color_mode,
            filter_speckle,
            color_precision,
        })
    }

    /// 把公开的 TraceConfig 投射到 vtracer 的内部 Config。其余字段沿用 vtracer 默认。
    pub(crate) fn to_vtracer(&self) -> VConfig {
        let mut v = VConfig::default();
        v.mode = self.mode;
        v.color_mode = match self.color_mode {
            ColorMode::Color => ColorMode::Color,
            ColorMode::Binary => ColorMode::Binary,
        };
        v.filter_speckle = self.filter_speckle;
        v.color_precision = self.color_precision;
        v
    }
}
