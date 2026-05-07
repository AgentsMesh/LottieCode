//! dotLottie 打包 —— 输出 .lottie zip。
//!
//! .lottie 格式 = zip 含：
//! - manifest.json
//! - animations/{id}.json

use std::io::Write;

use serde_json::json;
use zip::write::FileOptions;
use zip::CompressionMethod;

use lottiecode_lang::ir::IrAnimation;

use crate::generate_compact;

/// 把 IR 打包为 dotLottie zip 字节。
pub fn pack(ir: &IrAnimation) -> std::io::Result<Vec<u8>> {
    let buf = Vec::new();
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(buf));

    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    // manifest.json
    zip.start_file("manifest.json", options)?;
    let manifest = json!({
        "version": "1.0",
        "generator": "lottiecode",
        "author": "LottieCode",
        "animations": [
            {
                "id": ir.name,
                "loop": false,
                "autoplay": true,
            }
        ]
    });
    zip.write_all(serde_json::to_string(&manifest)?.as_bytes())?;

    // animations/{name}.json
    zip.start_file(format!("animations/{}.json", ir.name), options)?;
    zip.write_all(generate_compact(ir).as_bytes())?;

    let cursor = zip.finish()?;
    Ok(cursor.into_inner())
}
