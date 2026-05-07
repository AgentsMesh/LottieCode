//! 资源 base64 嵌入工具（避免引入第三方依赖）。

use crate::error::{ErrorKind, LottieError, Result};
use crate::token::Span;

pub fn embed_if_needed(path: &str, embed: bool, span: Span) -> Result<(String, bool)> {
    if embed && !path.starts_with("data:") {
        embed_local_file(path, span)
    } else if path.starts_with("data:") {
        Ok((path.to_string(), true))
    } else {
        Ok((path.to_string(), false))
    }
}

fn embed_local_file(path: &str, span: Span) -> Result<(String, bool)> {
    use std::io::Read;
    match std::fs::File::open(path) {
        Ok(mut f) => {
            let mut buf = Vec::new();
            f.read_to_end(&mut buf).ok();
            let mime = mime_for(path);
            let b64 = base64_encode(&buf);
            Ok((format!("data:{};base64,{}", mime, b64), true))
        }
        Err(_) => Err(LottieError::new(
            ErrorKind::InvalidValue,
            format!("无法读取 `{}` 用于 embed", path),
            Some(span),
        )),
    }
}

fn mime_for(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else {
        "application/octet-stream"
    }
}

fn base64_encode(input: &[u8]) -> String {
    const ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        out.push(ALPHA[(b0 >> 2) as usize] as char);
        out.push(ALPHA[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            out.push(ALPHA[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(ALPHA[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    fn sp() -> Span {
        Span::new(0, 0, 1, 1)
    }

    fn tmp_path(label: &str, ext: &str) -> std::path::PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        std::env::temp_dir().join(format!(
            "lc-embed-{}-{}-{}.{}",
            std::process::id(),
            label,
            id,
            ext
        ))
    }

    #[test]
    fn data_url_passthrough() {
        let (out, embedded) =
            embed_if_needed("data:image/png;base64,xyz", false, sp()).unwrap();
        assert!(embedded);
        assert_eq!(out, "data:image/png;base64,xyz");
    }

    #[test]
    fn external_path_when_not_embedded() {
        let (out, embedded) = embed_if_needed("logo.png", false, sp()).unwrap();
        assert!(!embedded);
        assert_eq!(out, "logo.png");
    }

    #[test]
    fn embed_actual_png_file() {
        let path = tmp_path("png", "png");
        fs::write(&path, b"PNGDATA").unwrap();
        let (data_url, embedded) = embed_if_needed(path.to_str().unwrap(), true, sp()).unwrap();
        assert!(embedded);
        assert!(data_url.starts_with("data:image/png;base64,"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn embed_jpeg_uses_jpeg_mime() {
        let path = tmp_path("jpg", "jpg");
        fs::write(&path, b"JPG").unwrap();
        let (data_url, _) = embed_if_needed(path.to_str().unwrap(), true, sp()).unwrap();
        assert!(data_url.contains("image/jpeg"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn embed_gif_mime() {
        let path = tmp_path("gif", "gif");
        fs::write(&path, b"G").unwrap();
        let (data_url, _) = embed_if_needed(path.to_str().unwrap(), true, sp()).unwrap();
        assert!(data_url.contains("image/gif"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn embed_webp_mime() {
        let path = tmp_path("webp", "webp");
        fs::write(&path, b"WEBP").unwrap();
        let (data_url, _) = embed_if_needed(path.to_str().unwrap(), true, sp()).unwrap();
        assert!(data_url.contains("image/webp"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn embed_svg_mime() {
        let path = tmp_path("svg", "svg");
        fs::write(&path, b"<svg/>").unwrap();
        let (data_url, _) = embed_if_needed(path.to_str().unwrap(), true, sp()).unwrap();
        assert!(data_url.contains("image/svg+xml"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn embed_unknown_extension_falls_back_to_octet_stream() {
        let path = tmp_path("xyz", "xyz");
        fs::write(&path, b"x").unwrap();
        let (data_url, _) = embed_if_needed(path.to_str().unwrap(), true, sp()).unwrap();
        assert!(data_url.contains("application/octet-stream"));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn embed_missing_file_errors() {
        let err =
            embed_if_needed("/path/that/does/not/exist.png", true, sp()).unwrap_err();
        assert_eq!(err.kind, ErrorKind::InvalidValue);
    }

    #[test]
    fn base64_encode_known_vector() {
        // RFC 4648 测试向量
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
    }
}
