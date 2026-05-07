//! 易错点 / 枚举值速查。

pub const SECTION: &str = r#"## 枚举值速查

**blend 模式**：
`normal` / `multiply` / `screen` / `overlay` / `darken` / `lighten`
/ `color-dodge` / `color-burn` / `hard-light` / `soft-light`
/ `difference` / `exclusion` / `hue` / `saturation` / `color` / `luminosity`

**matte 模式**：
`alpha` / `alpha-inv`（同 `alpha-inverted`） / `luma` / `luma-inv`（同 `luma-inverted`）

**mask 模式**：
`add` / `subtract` / `intersect` / `lighten` / `darken` / `difference` / `none`

**fill rule**：`non-zero` / `even-odd`

**stroke cap**：`butt` / `round` / `square`（默认 `round`）
**stroke join / merge join**：`miter` / `round` / `bevel`（默认 `round`，merge 为 `miter`）

**polystar type**：`star` / `polygon`

**zigzag type**：`corner` / `smooth`

**merge mode**：`merge` / `add` / `subtract` / `intersect` / `exclude`

**repeater composite**：`above` / `below`

**text align**：`left` / `center` / `right`

## 易错点

1. **scale / opacity / trim 都是 0..100 百分比**，不要写 `1.0` / `0.5`。`scale = 100` 才是原大小。
2. **scale 单值会广播**：`scale = 50` ≡ `[50, 50, 100]`；2D 数组 z 默认补 100（identity），不是 0。
3. **stroke 必须写对象**，不能用 `stroke = #FF0000` 简写；fill 才支持颜色简写。
4. **fill 对象必须含 color 或 gradient**；stroke 对象必须含 color 或 gradient + width。
5. **path 动画（morph）的关键帧 SVG 字符串顶点数必须一致**，否则 Lottie 无法补间。
6. **path 的 SVG d** 推荐只用 `M / L / Z`；其它命令（H/V）不保证支持。
7. **Bezier 字面量** 的 `v` / `in` / `out` 三个数组必须等长（每个顶点配一对切线）。
8. **关键帧 ease 写在「源帧」**：从此帧出发到下一帧的曲线。最后一帧的 ease 没意义。
9. **重复属性以最后一次为准**：相同 key 写多次时，后写的覆盖先写的（HashMap 索引）。
10. **layer 默认入出场 `(0, comp.duration)`**；要错位入场必须显式 `start = 0.3s`。
11. **time 单位**：写明 `s` / `ms` / `f` 最稳；`f` 在不同 fps 下含义不同（`30f` @ fps=60 = 0.5s）。
12. **token 引用必须先声明**，否则编译报 `UndefinedToken`；slot / component-arg 同理。
13. **shape 至少要有一个几何或子 group**，否则编译时该 shape 被丢弃。
14. **rect 必填 `size`、ellipse 必填 `size`、polystar 必填 `outer-radius`、path 必填 `d` 或 `v`。**
15. **mask 必填 `path`**（字符串），mode 默认 `add`。
16. **precomp / image layer 必填 `asset`**，且引用的 id 必须先声明。
17. **solid 必填 `color` 与 `size`**。
18. **component 内嵌 `use` 可递归展开**，但仍受限于必填参数检查。
19. **loop 写在 animate 块的 `}` 之后**，不是块内字段；写在块内会被当成属性名而报错。
20. **animate 中未列入支持表的 property 会被拒绝**（如 `animate width { ... }` 报 InvalidValue）。

## 完整微示例

```
token color { brand = #6366F1 }

component dot(cx, cy = 30) {
    shape d {
        ellipse  { size = [20, 20] }
        fill     = color.brand
        position = [cx, cy]
        animate scale {
            0s:   [60, 60]
            0.5s: [120, 120] ease = ease-out-back
            1s:   [60, 60]
        } loop
    }
}

composition "loading-dots" {
    width = 200  height = 60  fps = 30  duration = 1s

    use dot(cx = 60)
    use dot(cx = 100)
    use dot(cx = 140)
}
```
"#;
