//! Shape 详解章节：几何 / 样式 / 修饰器 / 子 group / mask / effects。

pub const SECTION: &str = r#"## Shape 块详解

```
shape <name> {
    // 通用 layer 属性（见「层类型 → 通用 layer 属性」）

    // 几何（可多个，从前到后渲染）
    <geometry>

    // 样式（应用到 shape 内全部几何）
    fill   = ...
    stroke = ...

    // 路径变形器（可叠加）
    pucker | twist | zigzag | rounded-corners | offset-path | merge

    // 复制器
    repeater = { ... }

    // 裁剪
    trim { ... }

    // 子 group（迷你 shape）
    group [<name>] { ... }

    // 蒙版（可多个）
    mask { ... }

    // 效果
    shadow = ...
    blur   = <number>

    // 动画
    animate <prop> { ... }
}
```

### 几何（geometry）

```
rect     { size = [w, h]  position = [x, y]  radius = N }     // size 必填
ellipse  { size = [w, h]  position = [x, y] }                  // size 必填
polystar { points = N  outer-radius = N                        // outer-radius 必填
           inner-radius = N         (默认 outer-radius * 0.5)
           outer-roundness = N      (默认 0)
           inner-roundness = N      (默认 0)
           type = star | polygon    (默认 star)
           rotation = N             (默认 0)
           position = [x, y] }
path     = "M0 0 L10 10 ..."                                   // 简写：等号 + SVG d
path     "M0 0 L10 10"                                         // 简写：无等号
path     { d = "M..." }                                        // 等价
path     { v = [[x,y], ...]                                    // Bezier 顶点
           in  = [[x,y], ...]                                  // in tangents（同长度）
           out = [[x,y], ...]                                  // out tangents（同长度）
           closed = true }                                     // 默认 false
```

### fill —— 颜色 / 渐变

```
fill = #RRGGBB[AA]                                             // 简写
fill = { color = #..., opacity = 0..100, rule = non-zero | even-odd }
fill = { gradient = linear | radial,
         start    = [x, y],
         end      = [x, y],
         colors   = [#..., #..., ...],     // ≥ 2 个
         opacity  = 0..100 }
```

### stroke —— 必须是对象

```
stroke = { color    = #...,                  // 或 gradient = ...
           width    = N,
           cap      = butt | round | square, // 默认 round
           join     = miter | round | bevel, // 默认 round
           miter-limit = N,                  // 默认 4
           opacity  = 0..100,
           dash     = [N, N, ...],           // 数字数组
           dash-offset = N }
```

### 路径变形器

```
pucker         = N                  // 或 { amount = N }，范围常用 -200..200
twist          = N                  // 或 { angle = N, center = [x, y] }
zigzag         = { amplitude = N, frequency = N, type = corner | smooth }
rounded-corners = N                 // 圆角半径
offset-path    = { amount = N, join = miter | round | bevel, miter-limit = N }
merge          = merge | add | subtract | intersect | exclude
```

### 复制器

```
repeater = { copies   = N,                   // 必填
             offset   = N,                   // 默认 0
             position = [x, y],              // 默认 [0,0]
             rotation = N,                   // 默认 0
             scale    = N | [sx, sy],        // 默认 [100,100]
             composite = above | below }     // 默认 above
```

### 裁剪 trim

```
trim { start = N  end = N  offset = N }     // 静态：N ∈ 0..100
animate trim-start  { ... }                  // 动画化某一项
animate trim-end    { ... }
animate trim-offset { ... }
animate trim        { 0s: { start=0, end=0, offset=0 }
                      1s: { start=0, end=100, offset=0 } }   // 三字段同步
```

### 子 group

```
group [<name>] {
    rect | ellipse | polystar | path { ... }
    fill   = ...
    stroke = ...
    position = [x, y]   anchor = [x, y]
    scale = ...   rotation = N   opacity = N
}
```

子 group 没有 modifiers / repeater / animate（如需动画请放到 shape 顶层或单独建 shape）。

### Mask

```
mask {
    mode    = add | subtract | intersect | lighten | darken | difference | none   // 默认 add
    path    = "M..."                  // SVG d 字符串，必填
    invert  = true                    // 默认 false
    opacity = 0..100                  // 默认 100
    expand  = N                       // 默认 0
    animate path { 0s: "M..."  1s: "M..." }   // path 可动画
}
```

### Effects

```
shadow = [dx, dy, blur, #RRGGBBAA]                         // 简写：4 元数组
shadow = { color = #..., direction = N, distance = N,
           softness = N, opacity = 0..100 }                // 对象写法
blur   = <number>                                          // 高斯模糊半径
```
"#;
