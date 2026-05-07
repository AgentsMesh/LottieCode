//! 层类型章节：所有可放进 composition / precomp / component 的 layer。

pub const SECTION: &str = r#"## 层类型

可在 `composition` / `precomp` / `component` 内出现的 layer：

| 关键字                              | 类型           | 必填字段          |
|-------------------------------------|----------------|-------------------|
| `shape <name> { ... }`              | 矢量形状层     | 至少一个几何或子 group |
| `text "<content>" { ... }`          | 文本层         | 无（默认 Arial Regular 16） |
| `image <name> { asset = <id>, ... }`| 位图层         | `asset` |
| `precomp <inst> { asset = <id>, ... }` | 嵌套合成实例 | `asset` |
| `solid <name> { color, size, ... }` | 实色层         | `color`, `size` |
| `controller <name> { ... }`         | Null 控制层    | 无 |
| `camera <name> { ... }`             | 摄像机层       | 无（默认 perspective=800） |
| `use <component>(args)`             | 组件展开       | 取决于 component |

### 通用 layer 属性

所有 layer（除 `use`）都接受这些字段：

| key             | 类型                      | 默认            | 说明 |
|-----------------|---------------------------|-----------------|------|
| `position`      | `[x,y]` 或 `[x,y,z]`       | `[0,0,0]`       | 位置 |
| `anchor`        | 同上                       | `[0,0,0]`       | 锚点 |
| `scale`         | 数字 / `[sx,sy]` / `[sx,sy,sz]` | `[100,100,100]` | 单值广播；2D 数组 z 自动补 100 |
| `rotation`      | 度数                        | 0               | |
| `opacity`       | `0..100`                   | 100             | |
| `skew`          | 度数                        | 0               | |
| `skew-axis`     | 度数                        | 0               | |
| `start` / `in`  | time                       | 0               | layer 入场时间，同义 |
| `end` / `out`   | time                       | comp.duration   | layer 出场时间，同义 |
| `delay`         | time                       | 0               | start_time 偏移 |
| `speed`         | 数字                        | 1.0             | 时间拉伸（speed=2 → 2 倍速）|
| `parent`        | ident                      | 无              | 父 layer 名 |
| `blend`         | enum                       | `normal`        | 混合模式（见易错点） |
| `matte`         | ident 或 `{source, mode}`   | 无              | 蒙版源；mode = alpha / alpha-inv / luma / luma-inv |
| `matte-source`  | bool                       | false           | 标记自己为蒙版源 |

### text 块

```
text "<content>" {
    font   = "Arial"             // 默认 "Arial"
    weight = Regular             // 标识符；默认 Regular（如 Bold）
    size   = <number>            // 默认 16
    color  = #RRGGBB             // 默认 #000000
    align  = left | center | right    // 默认 left
    line-height = <number>       // 可选
    tracking    = <number>       // 字距，默认 0
    stroke-color = #RRGGBB
    stroke-width = <number>

    // 通用 transform 属性（position / anchor / scale / rotation / opacity / parent ...）

    // 字符级 motion（静态值）
    char-position = [x, y]
    char-scale    = N 或 [sx, sy]
    char-opacity  = 0..100
    char-rotation = N

    // 打字机：0..100 控制可见字符比例，可作为关键帧动画
    typewriter {
        <time>: <0..100> [ease = ...]
        ...
    }

    animate <prop> { ... }
}
```

### image / precomp 实例 / solid / controller / camera

```
image <name> {
    asset = <asset-id>           // 必填，引用顶层 asset
    // 通用 transform 属性
}

precomp <inst-name> {
    asset = <precomp-id>         // 必填，引用顶层 precomp
    // 通用 transform 属性
    animate <prop> { ... }
    animate time-remap { 0s: 0  2s: 1.0 }   // 时间重映射，仅 precomp 实例
}

solid <name> {
    color = #RRGGBB              // 必填
    size  = [w, h]               // 必填
    // 通用 transform / animate
}

controller <name> {
    // 仅 transform / animate；用作 parent 控制点（Null Layer）
}

camera <name> {
    perspective = <number>       // 默认 800
    // 通用 transform / animate（自动 3D）
}

use <component-name>(arg1 = <value>, arg2 = <value>, ...)
```
"#;
