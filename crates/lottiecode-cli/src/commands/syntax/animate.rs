//! Animate 章节：关键帧、属性表、loop / hold / 空间切线。

pub const SECTION: &str = r#"## Animate

```
animate <property> {
    <time>:        <value> [ease = <easing>] [to = [x,y]] [ti = [x,y]]
    [hold] <time>: <value> [ease = <easing>]
    ...
} [loop]
```

### 关键帧字段

- `<time>`：时间字面量（`0.5s` / `500ms` / `30f`）或纯数字（按秒）。
- `<value>`：与 property 类型匹配（数字 / 数组 / 颜色 / 字符串）。
- `ease = <name>` 或 `ease = cubic(x1, y1, x2, y2)`：写在「源帧」表示离开此帧到下一帧的曲线。**最后一帧的 ease 被忽略。**
- `hold <time>: <value>`：关键帧前置 `hold`，到达此帧时直接跳变（h:1），不插值到下一帧。
- `to = [x, y]` / `ti = [x, y]`：仅 spatial 属性（如 position）的运动切线，控制路径弧度。

### 块尾 `loop`

```
animate scale {
    0s: [60, 60]
    1s: [120, 120] ease = ease-out
} loop
```

`loop` 写在 `}` 之后，标记这条动画无限循环；具体由播放端解释。

### 支持的 property

| property                                | 值类型                  | 备注 |
|-----------------------------------------|-------------------------|------|
| `position` / `anchor`                   | `[x,y]` 或 `[x,y,z]`     | 允许 `to` / `ti` 切线 |
| `scale`                                 | 数字 / `[sx,sy]` / `[sx,sy,sz]` | 单值广播 |
| `rotation` / `skew` / `skew-axis`       | 数字（度数）              | |
| `opacity`                               | 0..100                   | |
| `path`                                  | 字符串（SVG d）           | path morph：关键帧间顶点数必须一致 |
| `fill` / `fill-color`                   | 颜色字面量                | 等价；写其一即可 |
| `fill-opacity`                          | 0..100                   | |
| `stroke` / `stroke-color`               | 颜色字面量                | 等价 |
| `stroke-opacity`                        | 0..100                   | |
| `stroke-width`                          | 数字                      | |
| `trim-start` / `trim-end` / `trim-offset` | 0..100                  | |
| `trim`                                  | `{ start, end, offset }` | 三字段同步动画 |
| `time-remap`                            | time 或数字               | 仅 precomp 实例 layer |
| `typewriter`                            | 0..100                   | 仅 text 块的 typewriter 子块 |

### 微示例

```
// position 动画 + 命名 ease
animate position {
    0s:    [0, 0]
    0.6s:  [200, 100] ease = ease-out-back
    1.2s:  [200, 100]
}

// trim 动画（rotation + 描边裁剪 = loader 经典组合）
animate trim-end {
    0s:   0
    0.6s: 100  ease = ease-in-out
}

// hold + 跳变
animate opacity {
    0s:   0
    0.3s: 0  hold       // 持续 0.3s 不变
    0.6s: 100  ease = ease-out
}

// path morph（顶点数必须一致）
animate path {
    0s: "M0 -50 L50 30 L0 30 L-50 30"
    1s: "M-50 -50 L50 -50 L50 50 L-50 50"  ease = ease-in-out
}
```
"#;
