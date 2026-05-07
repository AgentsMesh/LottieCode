# LottieCode 架构提案

> 起草日期：2026-05-06
> 背景：直接让 Agent 写 Lottie JSON 效果不佳——字段缩写（`gr/tr/sh/fl/...`）、`{a, k}` 包装、Bezier 切线、关键帧编排、组与 Transform 1:1 强约束都是常见错误源。
> 思路：参考 `Works/VEAC` 与 `Works/Pastel` 的 **DSL → IR → Compile** 流水线，把「写 Lottie」变成「写代码」。

---

## 1. 核心理念：Animation as Code

| 难点（直写 JSON） | DSL 化后 |
|---|---|
| 字段名 1–2 字符缩写难记 | 用语义化关键字（`group`、`fill`、`stroke`、`trim` …） |
| `{a:0, k:V}` / `{a:1, k:[…]}` 包装重复 | 静态值直接写，动画用 `animate { 0s: …; 1s: … }` 块 |
| Bezier 缓动 `{i:{x,y}, o:{x,y}}` 不直观 | 用命名 easing：`ease-in-out`、`spring(0.3, 0.7)` |
| 「每个 group 末尾必须 tr」类强约束 | 编译器自动注入 `tr`，DSL 表达「shape + fill + stroke」即可 |
| Path 的 `v/i/o/c` 顶点+切线 | 接受 SVG path data（`M0 0 L10 0 …`）或预设几何 |
| 颜色 `[r,g,b,a]` 0–1 浮点 | 接受 `#RRGGBB[AA]` / `rgb(…)` / token 引用 |
| 时间用「帧」做 in/out，不直观 | 同时支持 `0s / 500ms / 30f`（VEAC 已实践） |
| 父子 `parent: ind` 数字索引 | 使用名称引用 `parent = card1` |

**编译器吃掉所有缩写、包装、约束、索引——Agent 只描述「要什么」**。

---

## 2. 流水线对标

```
                    VEAC                          Pastel                       LottieCode (新)
源文件             .veac                         .pastel                      .lottie-code  / .lc
顶层概念           project + timeline + tracks   canvas + frame tree          composition + layer tree + timeline
中间表示           IrProgram                     IrDocument                   IrAnimation
后端目标           FFmpeg 命令行                  Skia 渲染 + HTML/React/CSS   Lottie JSON / dotLottie / SVG / MP4
LLM-friendly 命令  veac syntax / plan            pastel syntax / inspect      lc syntax / plan / inspect
验证命令           veac check                    pastel check                 lc check
```

### 2.1 模块分布（与 VEAC 4-crate 对齐）

| Crate | 职责 | 参考 |
|---|---|---|
| `lottiecode-lang` | lexer / parser / ast / semantic / resolve / ir | VEAC `veac-lang` |
| `lottiecode-codegen` | IR → Lottie JSON / dotLottie / SVG | VEAC `veac-codegen`、Pastel `pastel-codegen` |
| `lottiecode-runtime`（可选） | headless 渲染：调用 lottie-web/Skottie 出 PNG/GIF/MP4 | VEAC `veac-runtime` |
| `lottiecode-cli` | CLI 入口 | VEAC/Pastel `*-cli` |

> MVP 只做前两个 + CLI；runtime 留给后续。

---

## 3. DSL 草案

### 3.1 Hello World

```
composition "hello" {
    width  = 400
    height = 400
    fps    = 60
    duration = 1.5s   // 编译为 op = 90 帧
}

shape circle {
    ellipse { size = [200, 200] }
    fill    = #6366F1
    position = [200, 200]

    animate scale {
        0s: [0, 0]
        0.5s: [110, 110] ease = ease-out
        1s:   [100, 100] ease = ease-in-out
    }
    animate opacity {
        0s: 0
        0.3s: 100
    }
}
```

编译产物：1 个 `ty=4` Shape Layer，含 1 个 `gr`（内含 `el` + `fl` + `tr`），`ks.s` 与 `ks.o` 为动画化属性。

### 3.2 Trim Path 勾选动画（对照 sample-success.json）

```
composition "check" {
    width = 64  height = 64  fps = 60  duration = 1.5s
}

token color {
    success = #1EBF75
}

shape check {
    path = "M9.25 -6 L-2.75 6 L-9.25 -0.5"
    stroke = { color = color.success, width = 3, cap = round, join = round }
    position = [32, 32]
    scale    = 33.333

    animate trim-end {
        0.73s: 0     // 静默
        1.5s:  100   ease = ease-out
    }
}

shape ring {
    ellipse { size = [53, 53] }
    stroke = { color = color.success, width = 3 }
    position = [32, 32]
    scale    = 33.333

    animate trim {
        0s:    { start = 0, end = 0,   offset = -360 }
        1.28s: { start = 0, end = 100, offset = 0    } ease = cubic(0.66, 0, 0.34, 1)
    }
}
```

编译产物完全对应 `sample-success.json` 的两个 Shape Layer + Trim Path + Stroke。

### 3.3 复用与组合

```
include "../tokens/brand.lc"

component pulse-dot(color: color, delay: time = 0s) {
    shape dot {
        ellipse { size = [20, 20] }
        fill = color
        animate scale {
            0s + delay:   [50, 50]
            0.5s + delay: [120, 120]
            1s + delay:   [50, 50]
        } loop
    }
}

composition "loader" {
    width = 200  height = 60  fps = 30  duration = 1s

    layout horizontal gap=20 align=center {
        pulse-dot(color = brand.primary)
        pulse-dot(color = brand.primary, delay = 0.1s)
        pulse-dot(color = brand.primary, delay = 0.2s)
    }
}
```

`component` / `include` / `token` 三件套与 Pastel 一致。`layout` 不是 Lottie 原生概念，但可在编译期展开为绝对坐标（After Effects 也是这么做的）。

### 3.4 高级特性骨架

```
// 关键帧动画的多段 keyframe
animate position {
    0s:   [0, 0]
    0.5s: [100, 50]    ease = ease-out
    1s:   [200, 0]     ease = ease-in
    hold 0.2s          // h:1
    1.5s: [200, 100]
}

// 文字层
text "Hello" {
    font   = "Inter"
    weight = bold
    size   = 32
    color  = #FFFFFF
    align  = center
}

// 预合成
precomp "card" using card-comp {
    position = [100, 100]
    duration = 2s
}

// 父子关系（按名引用而非索引）
shape head { parent = body  ... }

// 遮罩
shape masked-area {
    rect { size = [200, 200] }
    fill = #FFFFFF
    mask = circle      // 引用上面定义的 circle，编译为 tt/tp
}
```

---

## 4. IR 设计要点

### 4.1 类型安全

参考 Pastel 的 `IrNodeData` enum：

```rust
pub enum IrLayerData {
    Shape(ShapeLayerData),
    Text(TextLayerData),
    Image(ImageLayerData),
    Solid(SolidLayerData),
    Null(NullLayerData),
    Precomp(PrecompLayerData),
}

pub struct ShapeElement {
    // gr/rc/el/sr/sh/fl/st/gf/gs/tr/tm/rd/rp/mm/op
    pub kind: ShapeKind,
    // 仅本变体的字段
    pub data: ShapeElementData,
}
```

### 4.2 属性容器

```rust
pub enum AnimatableValue<T> {
    Static(T),
    Animated(Vec<Keyframe<T>>),
}

pub struct Keyframe<T> {
    pub time_frame: f64,        // 已统一到帧（DSL 写 s/ms 编译期换算）
    pub value: T,
    pub easing: Easing,
    pub hold: bool,
    // 仅 Position 类型需要的空间切线
    pub spatial_in: Option<[f64; 3]>,
    pub spatial_out: Option<[f64; 3]>,
}

pub enum Easing {
    Linear,
    EaseIn, EaseOut, EaseInOut,
    Cubic([f64; 4]),     // 编译为 i.x/i.y/o.x/o.y
}
```

> 直接对应 sample 实测的 `t/s/i/o/h/to/ti` 字段。

### 4.3 编译时验证（在 IR 之前）

| 错误类型 | DSL 阶段 | Lottie JSON 阶段 |
|---|---|---|
| 颜色超界 / 引用未定义 token | 立即报错 | 静默接受 |
| 关键帧时间倒序 | 立即报错 | 渲染异常 |
| `parent` 名引用错 | 立即报错 | 索引错位 |
| Group 内缺 Transform | 编译期自动补 | 必须手写 |
| Trim Path 在错误位置 | 检查 modifier 顺序 | 容易错 |
| 时间超出 composition 时长 | 警告 | 静默丢帧 |

---

## 5. CLI 设计（参考 VEAC/Pastel）

```bash
lc check  main.lc                 # 仅验证
lc plan   main.lc                 # 显示节点树 / IR 概览
lc fmt    main.lc                 # 格式化
lc inspect main.lc --json         # 输出 IR JSON（debug 用）
lc build  main.lc -o out.json     # 编译为 Lottie JSON
lc build  main.lc -o out.lottie   # 编译为 dotLottie zip
lc build  main.lc -o out.svg      # 静态首帧 SVG（runtime 可选）
lc syntax                         # 输出完整 DSL 语法供 LLM 上下文
lc preview main.lc                # 起本地服务 + lottie-web 预览
```

> `lc syntax` 与 `lc plan` 是 LLM 友好的关键命令——VEAC/Pastel 都已验证。

---

## 6. MVP 路线图

### 阶段 1：核心编译链（2 周内可见 demo）

- [x] **研究**：Lottie 格式（已完成 `research/docs/lottie-format.md`）+ 11 样本
- [ ] **lang**：lexer / parser / AST / 基础 semantic（不含 include / component）
- [ ] **IR**：覆盖 Shape Layer + 基础 Shape（rect/ellipse/path）+ fill/stroke + transform + 静态属性
- [ ] **codegen**：IR → Lottie JSON
- [ ] **CLI**：`check / build`
- [ ] **验收**：能从 DSL 编译出与 `sample-success.json` 等价的 JSON，并在 lottie-web 中正常渲染

### 阶段 2：动画与组合（4 周）

- [ ] 动画：`animate { keyframes }` + 命名 easing + Bezier 切线
- [ ] Trim Path、Repeater、Rounded Corners 修饰器
- [ ] Gradient Fill / Stroke
- [ ] 父子关系（parent = name）
- [ ] `component` + `include` + `token`
- [ ] CLI：`fmt / plan / inspect / syntax`

### 阶段 3：扩展（按需）

- [ ] Text Layer + Fonts
- [ ] Mask + Track Matte
- [ ] Image Asset 与 dotLottie 打包
- [ ] Precomp Layer
- [ ] Effect 通用框架
- [ ] runtime：调用 lottie-web headless 出 GIF/MP4

### 阶段 4：质量（持续）

- [ ] 真实样本回归（用 11 样本反向 decompile 成 DSL，再编回 JSON 比对）
- [ ] 设计 lint：检测时长过长 / 关键帧密度过高 / 未使用 token 等
- [ ] LLM 测试：用 prompt 集合驱动 Agent 生成 DSL，统计编译成功率与渲染正确率

---

## 7. 工程纪律（沿用 VEAC/Pastel 经验）

- **单文件不超过 200 行**（Pastel 强制）
- **测试与示例并重**：每个 IR 类型至少 1 个 example，每个 codegen 路径至少 1 个 fixture
- **中文回复**（继承用户偏好）
- **不主动提交/推分支**（继承用户偏好）
- **fixtures 双向**：`fixtures/{name}/source.lc` + `expected.json`，验证 deterministic 输出

---

## 8. 风险与未决问题

| 项 | 备注 |
|---|---|
| **DSL 表达力 vs 简洁性** | 太自由 → Agent 难掌握；太死板 → 限制设计师。建议 MVP 先窄后宽 |
| **关键帧时间单位** | 内部统一帧；DSL 同时接受 `s/ms/f`，由 fps 换算 |
| **Bezier 路径输入** | 优先支持 SVG path data；顶点+切线模式作为 escape hatch |
| **layout 抽象** | flex/grid 是 After Effects 没有的概念；编译期展开为绝对坐标 |
| **double-encoding 颜色** | DSL 用 0–255 / hex；IR 内部统一 0–1 浮点 |
| **是否要做 dotLottie** | MVP 后期，使用纯 Rust zip + miniz 即可 |
| **是否兼容 Lottie 1.0 / Bodymovin 5.x 双轨** | 默认输出 Bodymovin 5.7 兼容 JSON（lottie-web 主流），同时通过 `--spec 1.0` 切换 |

---

## 9. 立刻可做的下一步

1. 与用户确认本提案是否符合期望（DSL 风格、crate 切分、MVP 范围）
2. 初始化 Cargo workspace + 4 个 crate 骨架
3. 选定 1–2 个最简样本（如 `sample-success.json`）作为「首个端到端目标」
4. 起草 `lottiecode-lang` 的 token 与 AST 草案，并产出第一个示例 `examples/check/main.lc`

---

## 10. 参考材料索引

- 研究文档：`research/docs/lottie-format.md`（385 行）
- 样本：`research/samples/*.json`（11 个）
- 父目录工程：
  - `Works/VEAC` — DSL → FFmpeg 流水线，4 个 crate，可直接借鉴 lang/codegen/cli 结构
  - `Works/Pastel` — DSL → Skia + 多种 codegen，5 个 crate，IR 类型设计可直接映射到 Lottie
