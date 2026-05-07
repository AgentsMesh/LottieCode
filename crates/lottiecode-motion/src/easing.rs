//! 命名 easing —— DSL 设计师词汇 → 三次贝塞尔控制点。
//!
//! 这是 Motion Design DSL 的核心：让 agent 写 `ease-out-back` 而不是
//! `cubic(0.34, 1.56, 0.64, 1.0)`，把"曲线设计"沉淀到标准库。

/// 把命名 easing 解析为 cubic 控制点 `[ix, iy, ox, oy]`。
/// 未识别返回 `None`，调用方应回报 `UndefinedEasing` 错误。
pub fn resolve(name: &str) -> Option<[f64; 4]> {
    Some(match name {
        // -- 基础 --
        "linear" => [0.0, 0.0, 1.0, 1.0],
        "ease" => [0.25, 0.1, 0.25, 1.0],
        "ease-in" => [0.42, 0.0, 1.0, 1.0],
        "ease-out" => [0.0, 0.0, 0.58, 1.0],
        "ease-in-out" => [0.42, 0.0, 0.58, 1.0],
        // -- back（带回弹） --
        "ease-in-back" => [0.36, 0.0, 0.66, -0.56],
        "ease-out-back" => [0.34, 1.56, 0.64, 1.0],
        "ease-in-out-back" => [0.68, -0.6, 0.32, 1.6],
        // -- expo（指数） --
        "ease-in-expo" => [0.7, 0.0, 0.84, 0.0],
        "ease-out-expo" => [0.16, 1.0, 0.3, 1.0],
        "ease-in-out-expo" => [0.87, 0.0, 0.13, 1.0],
        // -- circ（圆） --
        "ease-in-circ" => [0.55, 0.0, 1.0, 0.45],
        "ease-out-circ" => [0.0, 0.55, 0.45, 1.0],
        // -- Material Motion 标准 --
        "standard" => [0.4, 0.0, 0.2, 1.0],
        "emphasized" => [0.2, 0.0, 0.0, 1.0],
        "decelerate" => [0.0, 0.0, 0.2, 1.0],
        "accelerate" => [0.4, 0.0, 1.0, 1.0],
        // -- Spring 风味（cubic 近似，无真实物理） --
        "spring" => [0.34, 1.56, 0.64, 1.0],
        "spring-snappy" => [0.5, 1.8, 0.5, 1.0],
        "spring-gentle" => [0.25, 1.2, 0.5, 1.0],
        "spring-wobbly" => [0.34, 1.8, 0.4, 0.95],
        // -- bounce 风味 --
        "bounce-out" => [0.34, 1.56, 0.64, 1.0],
        _ => return None,
    })
}

/// 列出所有支持的命名 easing —— 供 `lc syntax` 命令输出供 LLM 参考。
pub const ALL_NAMED: &[&str] = &[
    "linear",
    "ease",
    "ease-in",
    "ease-out",
    "ease-in-out",
    "ease-in-back",
    "ease-out-back",
    "ease-in-out-back",
    "ease-in-expo",
    "ease-out-expo",
    "ease-in-out-expo",
    "ease-in-circ",
    "ease-out-circ",
    "standard",
    "emphasized",
    "decelerate",
    "accelerate",
    "spring",
    "spring-snappy",
    "spring-gentle",
    "spring-wobbly",
    "bounce-out",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_names_resolve() {
        for name in ALL_NAMED {
            assert!(resolve(name).is_some(), "{name} 应能解析");
        }
    }

    #[test]
    fn unknown_returns_none() {
        assert!(resolve("not-a-real-easing").is_none());
    }

    #[test]
    fn linear_returns_diagonal() {
        assert_eq!(resolve("linear"), Some([0.0, 0.0, 1.0, 1.0]));
    }

    #[test]
    fn ease_out_back_overshoots() {
        let c = resolve("ease-out-back").unwrap();
        assert!(c[1] > 1.0, "ease-out-back 应在 P1.y 处过冲");
    }

    #[test]
    fn ease_in_back_undershoots() {
        let c = resolve("ease-in-back").unwrap();
        assert!(c[3] < 0.0, "ease-in-back 应在 P2.y 处下冲");
    }

    #[test]
    fn material_motion_standard() {
        assert_eq!(resolve("standard"), Some([0.4, 0.0, 0.2, 1.0]));
    }

    #[test]
    fn all_named_list_consistent() {
        for name in ALL_NAMED {
            assert!(
                resolve(name).is_some(),
                "ALL_NAMED 包含 `{name}` 但 resolve 返回 None"
            );
        }
    }

    #[test]
    fn case_sensitive() {
        assert!(resolve("LINEAR").is_none());
        assert!(resolve("Linear").is_none());
    }
}
