# Lottie 文件格式研究

> 研究日期：2026-05-06
> 研究目标：为 LottieCode 项目奠定格式基础——明确字段定义、层级关系、属性结构与开源样本来源。

---

## 1. 概览

**Lottie** 是一种基于 JSON 的矢量动画文件格式，由 Hernan Torrisi 通过 After Effects 的 Bodymovin 插件发明，2024 年成立 Lottie Animation Community (LAC) 进行标准化。
- 官方注册媒体类型：`video/lottie+json`，扩展名 `.lot`（实际生态以 `.json` 为主）
- 现行规范：[Lottie Spec 1.0](https://lottie.github.io/lottie-spec/1.0/)
- 人类可读文档：[Lottie Docs (LottieFiles)](https://lottiefiles.github.io/lottie-docs/)
- JSON Schema：<https://lottiefiles.github.io/lottie-docs/schema/>

**派生格式 dotLottie (.lottie)**：用 deflate 压缩的 zip 包，内含 manifest + lottie JSON + 资源（图片/字体），LottieFiles 主推。

**核心特点**：
- 字段名采用 1–2 字符缩写以减小体积（不可读但紧凑）
- 布尔值用整数 `0/1` 表示
- 向量值用数组表示，`[x, y]` 或 `[x, y, z]`
- 颜色为归一化 RGBA：`[r, g, b, a]`，分量取值 0–1
- 属性可在「静态」与「动画化」两种形态间切换

---

## 2. 顶层 Animation 对象

| 字段 | 含义 | 类型 |
|---|---|---|
| `v` | Bodymovin 版本（如 `5.7.5`） | string |
| `ver` | 规范版本（MMmmpp 格式） | int（1.0 引入） |
| `nm` | 动画名 | string |
| `fr` | 帧率 (frames per second) | number |
| `ip` | In Point — 起始帧 | number |
| `op` | Out Point — 结束帧（动画时长 = (op - ip) / fr 秒） | number |
| `w` | 画布宽度 | int |
| `h` | 画布高度 | int |
| `ddd` | 是否包含 3D 层（0/1） | int |
| `assets` | 资源数组（precomp/image/sound/data） | array |
| `layers` | 主合成图层数组 | array |
| `markers` | 时间标记数组 | array |
| `fonts` | 字体定义（`{list: [...]}`） | object |
| `chars` | 字符数据（用于内嵌字形） | array |
| `slots` | 1.0 新增，外部可注入的占位属性 | object |
| `meta` | 作者/关键字等元数据 | object |

**样本验证**：所有 11 个采集样本均含 `v / fr / ip / op / w / h / layers`，其中仅 `sample-player.json` 包含 `fonts.list` 与 `markers`。

---

## 3. Layer 层级

### 3.1 层类型枚举（`ty`）

| ty | 类型 | 关键专用字段 |
|---|---|---|
| 0 | Precomposition（预合成） | `refId`, `w`, `h`, `tm` (时间重映射) |
| 1 | Solid（纯色） | `sw`, `sh`, `sc`（十六进制色） |
| 2 | Image（图像） | `refId` |
| 3 | Null（空） | 仅作变换控制器 |
| 4 | Shape（矢量形状） | `shapes` |
| 5 | Text（文字） | `t`（Text Data） |
| 6 | Audio（音频） | `refId`, `au.lv` |
| 13 | Camera（相机） | `pe`（透视距离） |
| 15 | Data（数据源） | `refId` |

### 3.2 通用字段

| 字段 | 全称 | 说明 |
|---|---|---|
| `nm` | Name | 可读名 |
| `mn` | Match Name | 表达式引用名 |
| `ind` | Index | 层索引（父子关系基准） |
| `parent` | Parent Index | 指向父层的 `ind` |
| `sr` | Time Stretch | 时间拉伸因子 |
| `ip / op` | In/Out Point | 该层起止帧 |
| `st` | Start Time | 时间偏移 |
| `ddd` | 3D | 该层是否启用 3D |
| `hd` | Hidden | 隐藏 |
| `bm` | Blend Mode | 混合模式索引 |

### 3.3 视觉层附加字段

| 字段 | 说明 |
|---|---|
| `ks` | Transform 对象 |
| `ao` | Auto-orient (沿路径自旋) |
| `tt` | Track Matte 模式 |
| `tp` | Matte 父层索引 |
| `td` | Track Matte target 标志 |
| `hasMask` / `masksProperties` | 遮罩开关 + 遮罩列表 |
| `ef` | Effect 列表 |
| `mb` | Motion Blur |
| `ct` | Collapse Transform |

### 3.4 Transform (`ks`) 对象

| 字段 | 含义 | 备注 |
|---|---|---|
| `a` | Anchor Point | Position 类型 |
| `p` | Position | Position；可拆分 (`s:true, x:{}, y:{}`) |
| `r` | Rotation (deg) | Scalar |
| `s` | Scale | Vector，`[100,100]` = 1× |
| `o` | Opacity | Scalar，0–100 |
| `sk` | Skew | Scalar |
| `sa` | Skew Axis | Scalar |
| `rx/ry/rz` | 拆分旋转 | 3D 时使用 |
| `or` | Orientation | 3D 时使用 |

---

## 4. Shape 元素

### 4.1 类型（`ty` 字符串）

| ty | 名称 | 类别 |
|---|---|---|
| `gr` | Group | 容器 |
| `rc` | Rectangle | 几何 |
| `el` | Ellipse | 几何 |
| `sr` | PolyStar | 几何 |
| `sh` | Path（自由 Bezier） | 几何 |
| `fl` | Fill | 样式 |
| `st` | Stroke | 样式 |
| `gf` | Gradient Fill | 样式 |
| `gs` | Gradient Stroke | 样式 |
| `tr` | Transform | 变换（每个 `gr` 末尾通常带一个） |
| `tm` | Trim Path | 修饰器 |
| `rd` | Rounded Corners | 修饰器 |
| `rp` | Repeater | 修饰器 |
| `mm` | Merge | 修饰器 |
| `op` | Offset Path | 修饰器 |

**样本统计（11 文件聚合）**：
```
gr=705  tr=705  sh=614  st=423  fl=237  tm=162
gf=62   el=54   rc=39   sr=12   mm=3
```
> Group 与 Transform 数量相等，符合「每个 group 末尾一个 tr」的约定。

### 4.2 关键字段速查

- **Rectangle**：`p`（中心位置）、`s`（宽高 Vector）、`r`（圆角半径 Scalar）、`d`（绘制方向）
- **Ellipse**：`p`、`s`、`d`
- **PolyStar**：`p`、`pt`（顶点数）、`r`（旋转）、`or/ir`（外/内半径）、`os/is`（外/内圆度）、`sy`（1=星 / 2=多边形）
- **Path (`sh`)**：`ks`（Bezier，见下文）
- **Fill**：`c`（颜色）、`o`（不透明度）、`r`（填充规则 1=非零 / 2=奇偶）
- **Stroke**：`c`、`o`、`w`（宽度）、`lc`（端点 1/2/3）、`lj`（连接 1/2/3）、`ml`（米接限制）、`d`（虚线模式）
- **Gradient Fill**：`g.p`（颜色停止数）、`g.k`（动画化的 stops）、`s`（起点）、`e`（终点）、`t`（1=线性 2=径向 3=圆锥）、`h/a`（高光长度/角度）
- **Trim Path**：`s`/`e`（起止 0–100）、`o`（偏移）、`m`（1=平行 / 2=顺序）

### 4.3 Bezier Path（`ks` 内部）

```json
{
  "i": [[0,0],[0,0],[0,0]],         // 各顶点的入切线（相对顶点偏移）
  "o": [[0,0],[0,0],[0,0]],         // 各顶点的出切线（相对偏移）
  "v": [[9.25,-6],[-2.75,6],[-9.25,-0.5]],  // 顶点绝对坐标
  "c": false                         // 是否闭合
}
```

---

## 5. Property（属性）结构

属性是 Lottie 中所有可动画化值的统一容器。

### 5.1 静态 vs 动画

```jsonc
// 静态
{ "a": 0, "k": 100,        "ix": 11 }
{ "a": 0, "k": [255,128,0,1] }
// 动画
{ "a": 1, "k": [ {keyframe1}, {keyframe2}, ... ], "ix": 6 }
```
- `a` — animated 标志（0/1）
- `k` — 值或关键帧数组
- `ix` — 在表达式中按索引引用
- `x` — 表达式字符串（仍存在于 35 处样本中）

### 5.2 Keyframe 字段

| 字段 | 含义 | 备注 |
|---|---|---|
| `t` | 帧号（必须升序） | 必填 |
| `s` | 该关键帧的起始值 | 必填（数组/对象/标量） |
| `e` | 终止值（已废弃，应取下一个 keyframe 的 `s`） | 仍出现 |
| `i` | In Tangent（缓动控制点 `{x, y}`） | 范围 0–1 |
| `o` | Out Tangent（缓动控制点 `{x, y}`） | 范围 0–1 |
| `h` | Hold（=1 表示保持不插值） | 0/1 |
| `to` | 空间切线 — 出方向（仅 Position） | `[x,y,z]` |
| `ti` | 空间切线 — 入方向（仅 Position） | `[x,y,z]` |
| `n` | 缓动名（lottie-web 旧字段，可忽略） | 已废弃 |

**实测频率（11 样本）**：`t=5517, s=5395, i=4969, o=4969, to/ti=2236, n/e=133`。

### 5.3 Bezier 缓动示例

```json
{
  "i": {"x":[0.667], "y":[1]},
  "o": {"x":[0.333], "y":[0]},
  "t": 30,
  "s": [100, 100, 100]
}
```

### 5.4 拆分位置（Split Position）

```json
"p": {
  "s": true,
  "x": { "a": 0, "k": 100 },
  "y": { "a": 0, "k": 200 }
}
```
> 11 样本中未出现，但规范允许，需实现兼容。

### 5.5 颜色与渐变

- **Color**：`[r, g, b, a]`，0–1 浮点
- **Gradient.k**：扁平数组：`[offset, r, g, b, offset, r, g, b, ..., offsetA, alpha, offsetA, alpha]`
- 动画化时仍用统一 `{a, k}` 包装

---

## 6. Mask 与 Matte

**Mask**（同层内自包含）：
| 字段 | 含义 |
|---|---|
| `pt` | 路径（动画化 Bezier） |
| `o` | 不透明度 |
| `mode` | `n`/`a`/`s`/`i`/`l`/`d`/`f`（None/Add/Subtract/Intersect/Lighten/Darken/Difference） |
| `inv` | 反转 |
| `x` | 扩展量 |

**Track Matte**（用另一层做遮罩）：通过被遮罩层的 `tt`（matte 模式）和 `tp`（matte 源层 `ind`）配合，遮罩层设 `td=1`。

> 11 样本中无 mask 实例，但需在解析器中预留。

---

## 7. Effect

通用结构：

| 字段 | 含义 |
|---|---|
| `ty` | 效果类型 |
| `ix` | 索引 |
| `nm/mn` | 名称 |
| `np` | 参数数量 |
| `ef` | 参数数组（每项含 `ty/nm/mn/v`） |
| `en` | 启用 0/1 |

常见 `ty`：5=Custom, 20=Tint, 21=Fill, 22=Stroke, 25=DropShadow, 29=GaussianBlur, 30=Twirl, 33=Spherize。

> 样本仅出现 5 (Custom Effect) × 5 处。

---

## 8. Text Layer

`t` 对象包含：
- `t.d` — 文档（含动画化 keyframes，每个 `s` 是 Text Document）
- `t.a` — 动画器数组（基于范围的文字动画）
- `t.m` — 对齐配置（`g`/`a`）
- `t.p` — 文字沿路径选项

**Text Document (`t.d.k[].s`)** 字段（实测）：
| 字段 | 含义 |
|---|---|
| `t` | 文本字符串 |
| `f` | 字体 family |
| `fc` | 填充颜色 [r,g,b] |
| `s` | 字号 |
| `lh` | 行高 |
| `ls` | 基线偏移 |
| `tr` | 字距 |
| `j` | 对齐 (0=左 1=右 2=中 3=两端) |

**Fonts**（顶层）：
```json
{"list": [{
  "fName": "Teko-Bold", "fFamily": "Teko", "fStyle": "Bold",
  "fPath": "https://fonts.googleapis.com/css?family=Teko",
  "origin": 0, "ascent": 66.899
}]}
```

---

## 9. Asset

按字段组合判断类型：

- **Precomposition Asset**：含 `id`、`layers`，可选 `fr`、`xt`
- **Image Asset**：含 `id`、`w`、`h`、`p`（文件名或 dataURL）、`u`（路径前缀）、`e`（1=embedded base64）
- **Sound Asset**：含 `id`、`p`、`u`、`e`
- **Data Source**：含 `id`、`p`、`u`、`e`、`t=3`

**样本统计**：13 个 precomp / 0 个 image —— 表明纯矢量动画里 image asset 较少见，但在含位图的动画里会大量出现。

---

## 10. Marker

```json
[
  {"tm": 0,  "cm": "label", "dr": 0},
  {"tm": 22, "cm": "1",      "dr": 0}
]
```
- `tm`：帧号
- `cm`：注释/标签
- `dr`：持续帧数

---

## 11. 开源样本目录

存放路径：`research/samples/`

| 文件 | 体积 | 来源 | 特征 |
|---|---:|---|---|
| `sample-success.json` | 2 KB | LottieFiles 官方包 | 基础勾选动画，2 个 Shape Layer，含 Trim Path 缓动 |
| `sample-classic2.json` | 6 KB | lottie-player demo | Loading Ring，4 个 Shape Layer，无 asset |
| `sample-loading.json` | 8 KB | LottieFiles 官方包 | Paperplane，使用 Precomp，含 `to/ti` 空间切线 |
| `sample-classic.json` | 30 KB | LottieFiles | H_Icon_04，含 Precomp，Bodymovin 4.7.1 旧版 |
| `sample-react.json` | 33 KB | lottie-js demo | Heart eyes burst |
| `sample-player.json` | 41 KB | lottie-player demo | 含 Text Layer + fonts.list + markers |
| `sample-data1.json` | 80 KB | LottieFiles | Desktop 场景，9 个 Precomp 拆分 |
| `sample-loader3.json` | 98 KB | LottieFiles | Rocket，13 个 Precomp |
| `sample-nuxt.json` | 154 KB | LottieFiles | Comp 1 |
| `sample-data2.json` | 343 KB | LottieFiles | Background，114 个 Shape Layer 大型动画 |
| `sample-confetti.json` | 614 KB | LottieFiles | Confetti，51 层（1 Null + 50 Shape） |

样本聚合统计：
- 形状类型：`gr/tr/sh/st/fl/tm/gf/el/rc/sr/mm` 全部覆盖
- 关键帧字段：`t/s/i/o/to/ti/n/e` 全部出现
- 表达式属性：35 处
- Parent 引用：60 处
- 3D 层、Mask、Image Asset、Hold 关键帧、Split Position、Auto-Orient：未出现（需补充对应样本以完整测试解析器）

---

## 12. 关键参考资源

**规范与文档**：
- [Lottie Spec 1.0 — 官方规范](https://lottie.github.io/lottie-spec/1.0/)
- [Lottie Spec 单页版](https://lottie.github.io/lottie-spec/1.0/single-page/)
- [Lottie Docs — 详细教程](https://lottiefiles.github.io/lottie-docs/)
- [JSON Schema](https://lottiefiles.github.io/lottie-docs/schema/)
- [GitHub: lottie/lottie-spec](https://github.com/lottie/lottie-spec)
- [Wikipedia 条目](https://en.wikipedia.org/wiki/Lottie_(file_format))
- [dotLottie 介绍](https://dotlottie.io/intro/)

**工具与库**：
- [airbnb/lottie-web](https://github.com/airbnb/lottie-web) — 浏览器渲染器
- [LottieFiles/lottie-js](https://github.com/LottieFiles/lottie-js) — JS 对象模型
- [LottieFiles/lottie-player](https://github.com/LottieFiles/lottie-player) — Web Component 播放器
- [marciogranzotto/lottie-tools](https://github.com/marciogranzotto/lottie-tools) — 开源在线编辑器
- [Bodymovin AE 插件](https://aescripts.com/bodymovin/)

**资源仓库**：
- [LottieFiles 免费动画](https://lottiefiles.com/free-animations/open-source)
- [LottieFiles GitHub 组织](https://github.com/lottiefiles)
- [awesome-lottie 资源汇总](https://github.com/LottieFiles/awesome-lottie)
- [Lottie Animation Community 主站](https://lottie.github.io/)

---

## 13. 给 LottieCode 项目的实现建议

1. **解析器分层**：先实现 Animation → Layer → Shape → Property 四层数据模型，再处理 Mask/Effect/Text 等扩展
2. **Property 抽象**：把 `{a, k, ix}` 包装成统一对象，按 `a` 分发到 `StaticValue` / `AnimatedValue<T>` 子类，T 包括 Scalar/Vector2/Vector3/Color/Bezier/Gradient
3. **关键帧采样**：内置三次贝塞尔缓动求值器（`i.x/i.y/o.x/o.y`），位置采样还需空间切线 `to/ti`
4. **Shape 渲染顺序**：组内自下而上：Path → Modifier (`tm/rd/rp`) → Style (`fl/st/gf/gs`) → Transform (`tr`)
5. **缩写映射表**：维护一份字段全称映射，调试与导出可读模型时使用
6. **覆盖测试集**：现样本缺 Mask、Image Asset、3D、Split Position、Hold、表达式重型场景——后续补样本以保证解析器健壮性
7. **优先目标格式**：聚焦 Lottie Spec 1.0 + lottie-web 5.x 兼容子集；dotLottie 解压可作为后续增量
