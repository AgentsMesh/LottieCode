//! 位图加载 + 调 vtracer 得到 SVG 字符串。

use std::path::Path;

use crate::config::TraceConfig;
use crate::error::{Result, TraceError};

/// 读取位图文件，调用 vtracer 矢量化，返回 SVG XML 字符串。
pub fn raster_to_svg_string(input: &Path, cfg: &TraceConfig) -> Result<String> {
    let dynamic = image::open(input).map_err(|e| match e {
        image::ImageError::IoError(io) => TraceError::Io(format!("{}: {io}", input.display())),
        other => TraceError::InvalidImage(format!("{}: {other}", input.display())),
    })?;

    // vtracer 要求 4 字节/像素 的 ColorImage。
    let rgba = dynamic.to_rgba8();
    let (width, height) = (rgba.width() as usize, rgba.height() as usize);
    let img = vtracer::ColorImage {
        pixels: rgba.into_raw(),
        width,
        height,
    };

    let svg_file = vtracer::convert(img, cfg.to_vtracer()).map_err(TraceError::VtracerFailed)?;

    // SvgFile 实现了 Display，直接 to_string 得到完整 XML。
    Ok(svg_file.to_string())
}
