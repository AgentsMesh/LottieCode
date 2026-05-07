//! 格式化辅助：xy / color / safe_name / 缩进 / 数字。
//!
//! 数字格式化策略：去除冗余 0（如 `1.0` → `1`，`100.5` → `100.5`），与 Python `:g` 一致。

pub fn fmt_num(v: f64) -> String {
    if !v.is_finite() {
        return v.to_string();
    }
    if v.fract() == 0.0 && v.abs() < 1e15 {
        return format!("{}", v as i64);
    }
    // 6 位有效数字（与 Python `:g` 一致）：fps=29.970001 → 29.97
    let abs = v.abs();
    let magnitude = abs.log10().floor() as i32;
    let decimals = ((6 - magnitude - 1).max(0) as usize).min(15);
    let s = format!("{:.*}", decimals, v);
    let trimmed = s.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() || trimmed == "-" {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn fmt_xy(pt: &[f64]) -> String {
    let xs: Vec<String> = pt.iter().map(|v| fmt_num(*v)).collect();
    format!("[{}]", xs.join(", "))
}

pub fn fmt_xy_array(pts: &[Vec<f64>]) -> String {
    let parts: Vec<String> = pts.iter().map(|p| fmt_xy(p)).collect();
    format!("[{}]", parts.join(", "))
}

pub fn fmt_color(c: &[f64]) -> String {
    let r = (c.first().copied().unwrap_or(0.0) * 255.0).round() as u32;
    let g = (c.get(1).copied().unwrap_or(0.0) * 255.0).round() as u32;
    let b = (c.get(2).copied().unwrap_or(0.0) * 255.0).round() as u32;
    let alpha = c.get(3).copied().unwrap_or(1.0);
    if alpha < 1.0 - 1e-6 {
        let a = (alpha * 255.0).round() as u32;
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    } else {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

/// 把任意 nm 转为合法 DSL 标识符：保留字母/数字/下划线，其余替换为 `_`，去除前后下划线。
pub fn safe_name(nm: &str, fallback_idx: usize) -> String {
    let cleaned: String = nm
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
        .collect();
    let trimmed = cleaned.trim_matches('_');
    if trimmed.is_empty() {
        format!("layer{fallback_idx}")
    } else {
        trimmed.to_string()
    }
}

/// 生成指定深度的空格缩进。
pub fn pad(indent: usize) -> String {
    " ".repeat(indent)
}

/// 把 Lottie 时间帧值转为秒并格式化（4 位小数，去尾零）。
pub fn fmt_seconds(frame: f64, fps: f64) -> String {
    let sec = frame / fps;
    let s = format!("{sec:.4}");
    let trimmed = s.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_num_strips_trailing_zero() {
        assert_eq!(fmt_num(1.0), "1");
        assert_eq!(fmt_num(100.5), "100.5");
        assert_eq!(fmt_num(0.0), "0");
        assert_eq!(fmt_num(-0.5), "-0.5");
    }

    #[test]
    fn fmt_num_uses_6_significant_digits() {
        // 与 Python `:g` 一致：fps=29.970001 → 29.97
        assert_eq!(fmt_num(29.970001), "29.97");
        assert_eq!(fmt_num(0.001234567), "0.00123457");
    }

    #[test]
    fn fmt_xy_outputs_all_dims() {
        assert_eq!(fmt_xy(&[1.0, 2.0]), "[1, 2]");
        assert_eq!(fmt_xy(&[100.5, -3.25]), "[100.5, -3.25]");
        assert_eq!(fmt_xy(&[80.0, 80.0, 100.0]), "[80, 80, 100]");
    }

    #[test]
    fn fmt_color_omits_full_alpha() {
        assert_eq!(fmt_color(&[1.0, 0.0, 0.0, 1.0]), "#FF0000");
        assert_eq!(fmt_color(&[0.0, 1.0, 0.0]), "#00FF00");
        assert_eq!(fmt_color(&[1.0, 1.0, 1.0, 0.5]), "#FFFFFF80");
    }

    #[test]
    fn safe_name_handles_empty_and_special() {
        assert_eq!(safe_name("", 3), "layer3");
        assert_eq!(safe_name("Shape Layer 1", 0), "Shape_Layer_1");
        assert_eq!(safe_name("___", 5), "layer5");
        assert_eq!(safe_name("___abc___", 0), "abc");
    }

    #[test]
    fn fmt_seconds_uses_fps() {
        assert_eq!(fmt_seconds(60.0, 30.0), "2");
        assert_eq!(fmt_seconds(15.0, 30.0), "0.5");
    }
}
