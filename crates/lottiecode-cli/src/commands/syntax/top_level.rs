//! 顶层声明章节：composition / precomp / token / slots / asset / include / component。

pub const SECTION: &str = r#"## 顶层声明

```
program := { include | token | slots | asset | precomp | component | composition }*
```

- 每个文件**至多一个 composition**（重复声明报错）。
- 其他声明可任意多个。
- 顺序无关，但建议把 token / asset / component 放在 composition 之前以便阅读。

### 1) composition —— 最终输出的合成

```
composition "<name>" {
    width    = <int>      // 必填
    height   = <int>      // 必填
    fps      = <number>   // 默认 30
    duration = <time>     // 必填，例 1.5s / 1500ms / 45f

    <layer> ...           // 见「层类型」
}
```

### 2) precomp —— 嵌套合成（可被实例化为 layer）

```
precomp <name> {
    width    = <int>      // 必填
    height   = <int>      // 必填
    fps      = <number>
    duration = <time>
    <layer> ...
}
```

实例化：在 composition / 别的 precomp 内 `precomp <inst-name> { asset = <name> ... }`。

### 3) token —— 设计 token 命名空间

```
token <group> {
    <key> = <value>       // 任意 expression
    ...
}
```

引用：`<group>.<key>`。例 `token color { primary = #6366F1 }` → `fill = color.primary`。

### 4) slots —— Lottie 1.0 外部参数注入

```
slots {
    <key> = <default-value>
}
```

引用：`slot.<key>`。运行时可被 player 覆盖。

### 5) include —— 包含外部文件

```
include "path/to/file.lc"
```

被 include 的文件**不允许包含 composition**，只能定义 token / asset / component / slots / precomp。
路径相对于当前 .lc 文件目录。

### 6) asset —— 图像 / 音频 / 数据资源

```
asset <id> { image = "path.png"  width = N  height = N  embed = true }   // 图像
asset <id> { sound = "audio.mp3" embed = true }                          // 音频
asset <id> { data  = "blob.json" embed = true }                          // 数据
```

- `embed = true` 表示把文件读入并 base64 内联到 JSON。
- Precomp asset **不**用 `asset` 关键字声明；用上面的顶层 `precomp`。

### 7) component —— 可复用图层模板

```
component <name>(<param> [= <default>], ...) {
    <layer> ...           // 任意 layer，参数在 body 内以标识符引用
}
```

- 没默认值的参数为必填。
- component body 只能含 layer，不能再嵌套 token / composition。
- 实例化：在 composition / precomp / 别的 component 内 `use <name>(arg = ..., ...)`，展开为多个 layer。
"#;
