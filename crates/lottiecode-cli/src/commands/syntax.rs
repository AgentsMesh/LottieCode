//! `lc syntax` —— 输出完整 DSL 语法供 LLM 上下文使用。

use lottiecode_motion::easing as ease_lib;

const REFERENCE: &str = r#"# LottieCode DSL 语法参考

## 顶层结构

```
composition "name" {
    width    = <int>
    height   = <int>
    fps      = <int>
    duration = <time>     // 0.5s / 500ms / 30f

    shape ... { ... }
}

token <group> {
    <key> = <value>
    ...
}
```

## Shape 块

```
shape <name> {
    // 几何（任选其一）：
    rect    { size = [w, h]  position = [x, y]  radius = <n> }
    ellipse { size = [w, h]  position = [x, y] }
    path    = "M9 -6 L-3 6 L-9 0"     // 仅 M / L / Z
    // 或：path { d = "..." }

    // Layer 级 transform：
    position = [x, y]                  // 或 [x, y, z]
    anchor   = [x, y]
    scale    = <n> 或 [sx, sy]         // 单值时两轴相同
    rotation = <度数>
    opacity  = <0..100>

    // 样式：
    fill   = #RRGGBB[AA]                            // 简写
    fill   = { color = #..., opacity = N, rule = non-zero|even-odd }
    stroke = { color = #..., width = N, cap = butt|round|square,
               join = miter|round|bevel, opacity = N }

    // Trim Path：
    trim { start = N  end = N  offset = N }         // 静态
    animate trim-end { 0s: 0  1s: 100 ease = ... }  // 动画化

    // 动画化 transform：
    animate scale     { 0s: 0  0.5s: 110 ease = ease-out-back  1s: 100 }
    animate position  { 0s: [0,0]  1s: [100, 50] }
    animate opacity   { 0s: 0  0.3s: 100 }
    // 同样可作用于 anchor / rotation
}
```

## 关键帧 ease 语义

`ease = X` 写在「源帧」上，描述「离开此帧到下一帧」的缓动曲线。
最后一帧的 `ease` 没有意义（被忽略）。

## 命名 Easing 列表
"#;

pub fn run() -> Result<(), String> {
    print!("{REFERENCE}");
    println!();
    println!("```");
    for name in ease_lib::ALL_NAMED {
        println!("  {name}");
    }
    println!("```");
    println!();
    println!("自定义曲线：`ease = cubic(x1, y1, x2, y2)`");
    Ok(())
}
