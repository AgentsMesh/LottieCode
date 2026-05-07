//! `lc syntax` —— 输出完整 DSL 语法供 LLM 上下文使用。
//!
//! 文档分章节存放在子模块的 `SECTION` 常量中，按固定顺序拼接打印。
//! 命名 easing 列表从 `lottiecode_motion::easing::ALL_NAMED` 动态注入，
//! 保证文档与代码不会漂移。

mod animate;
mod layers;
mod lexical;
mod pitfalls;
mod shape;
mod top_level;

use lottiecode_motion::easing as ease_lib;

/// 全部章节按顺序输出。`run()` 是 CLI 入口，无副作用。
pub fn run() -> Result<(), String> {
    println!("{}", header());
    println!("{}", lexical::SECTION);
    println!("{}", top_level::SECTION);
    println!("{}", layers::SECTION);
    println!("{}", shape::SECTION);
    println!("{}", animate::SECTION);
    println!("{}", named_easings());
    println!("{}", pitfalls::SECTION);
    Ok(())
}

fn header() -> &'static str {
    "# LottieCode DSL 语法参考

LottieCode（lc）是把 Motion DSL 编译成 Lottie JSON 的语言。
一个文件 = 一个 Program，包含若干顶层声明。

阅读顺序：词法 → 顶层声明 → 层类型 → shape 详解 → animate → 命名 easing → 易错点。
本文档列出的所有字段均经过 lexer / parser / semantic 层验证；不在此处的字段会被拒绝。
"
}

fn named_easings() -> String {
    let mut s = String::from("## 命名 Easing\n\n关键帧 `ease = <name>` 接受以下命名曲线：\n\n```\n");
    for name in ease_lib::ALL_NAMED {
        s.push_str("  ");
        s.push_str(name);
        s.push('\n');
    }
    s.push_str("```\n\n自定义曲线：`ease = cubic(x1, y1, x2, y2)` —— 4 个数字，控制点坐标。\n\n");
    s.push_str("**ease 语义**：写在「源帧」表示离开此帧到下一帧的曲线；最后一个关键帧的 `ease` 没有意义（被忽略）。\n");
    s
}
