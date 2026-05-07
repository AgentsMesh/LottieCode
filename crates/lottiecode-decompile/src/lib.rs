//! lottiecode-decompile —— Lottie JSON → LottieCode DSL
//!
//! 反向编译流水线：解析 Lottie JSON → 输出 DSL 字符串。
//!
//! # 关键能力
//! - **多 group 处理**：layer 含多个 top-level group 时输出 `group g0 {} group g1 {}` 子块
//! - **反 AE 模式检测**：单 group 但 `group.tr.p ≠ layer.ks.a` 时显式输出 group 块（保留偏移）
//! - **spatial tangents**：position 关键帧的 `to`/`ti` 还原为 DSL `to=[...] ti=[...]` 语法
//! - **easing 推断**：从下一帧的 `o`/`i` 推断 cubic 控制点（DSL 把 ease 写在源帧上）
//! - **precomp 嵌套**：保留 precomp asset → composition 引用结构
//!
//! # 不支持
//! - text layer（ty=5）的 t.d 文档完整解析
//! - image / solid / null layer
//! - effects / masks
//! - gradient fill / stroke
//!
//! 这些是后续迭代项；当前覆盖足以反编译 Phase 1-3 的形状层主流场景。

mod animate;
mod animation;
mod error;
mod format;
mod geometry;
mod layer;
mod layer_helpers;
mod shape;
mod transform;

pub use error::{DecompileError, Result};

use serde_json::Value;

/// 入口：把 Lottie JSON Value 反编译为 DSL 字符串。
pub fn decompile(json: &Value) -> Result<String> {
    animation::emit_animation(json)
}

/// 便利入口：从 JSON 字符串直接反编译。
pub fn decompile_str(s: &str) -> Result<String> {
    let json: Value = serde_json::from_str(s).map_err(|e| DecompileError::InvalidJson(e.to_string()))?;
    decompile(&json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn decompile_minimal_animation() {
        let root = json!({
            "v": "5.7.5", "fr": 30, "ip": 0, "op": 30,
            "w": 100, "h": 100, "nm": "test",
            "assets": [], "layers": []
        });
        let dsl = decompile(&root).unwrap();
        assert!(dsl.contains("composition \"test\""));
    }

    #[test]
    fn decompile_str_handles_invalid_json() {
        let err = decompile_str("not json").unwrap_err();
        matches!(err, DecompileError::InvalidJson(_));
    }
}
