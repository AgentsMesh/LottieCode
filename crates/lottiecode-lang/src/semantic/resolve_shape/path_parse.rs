//! SVG path data → BezierPath。
//!
//! Phase 1 仅支持 M / L / Z 三种命令（直线段）。
//! Phase 2 起扩展 C / Q / S 等贝塞尔命令。

use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::BezierPath;
use crate::token::Span;

pub fn parse_svg_path(s: &str, span: Span) -> Result<BezierPath> {
    // 预处理：在 M/L/Z 等命令字母前后插入空格，再 split。
    let mut spaced = String::with_capacity(s.len() * 2);
    for ch in s.chars() {
        if "MmLlZz".contains(ch) {
            spaced.push(' ');
            spaced.push(ch);
            spaced.push(' ');
        } else {
            spaced.push(ch);
        }
    }

    let mut bezier = BezierPath::default();
    let tokens: Vec<&str> = spaced.split_whitespace().collect();
    let mut i = 0;
    let mut cur_cmd = ' ';

    while i < tokens.len() {
        let t = tokens[i];
        // 命令字母（M/L/Z）
        if t.len() == 1 && "MmLlZz".contains(t) {
            cur_cmd = t.chars().next().unwrap();
            i += 1;
            if cur_cmd == 'Z' || cur_cmd == 'z' {
                bezier.closed = true;
            }
            continue;
        }

        match cur_cmd {
            'M' | 'L' => {
                if i + 1 >= tokens.len() {
                    return Err(LottieError::new(
                        ErrorKind::InvalidValue,
                        format!("SVG path `{cur_cmd}` 命令需要两个数字"),
                        Some(span),
                    ));
                }
                let x: f64 = tokens[i].parse().map_err(|_| invalid_num(tokens[i], span))?;
                let y: f64 = tokens[i + 1].parse().map_err(|_| invalid_num(tokens[i + 1], span))?;
                bezier.vertices.push([x, y]);
                bezier.in_tangents.push([0.0, 0.0]);
                bezier.out_tangents.push([0.0, 0.0]);
                i += 2;
            }
            'm' | 'l' => {
                if i + 1 >= tokens.len() {
                    return Err(LottieError::new(
                        ErrorKind::InvalidValue,
                        format!("SVG path `{cur_cmd}` 命令需要两个数字"),
                        Some(span),
                    ));
                }
                let dx: f64 = tokens[i].parse().map_err(|_| invalid_num(tokens[i], span))?;
                let dy: f64 = tokens[i + 1].parse().map_err(|_| invalid_num(tokens[i + 1], span))?;
                let last = bezier.vertices.last().copied().unwrap_or([0.0, 0.0]);
                bezier.vertices.push([last[0] + dx, last[1] + dy]);
                bezier.in_tangents.push([0.0, 0.0]);
                bezier.out_tangents.push([0.0, 0.0]);
                i += 2;
            }
            _ => {
                return Err(LottieError::new(
                    ErrorKind::InvalidValue,
                    format!(
                        "SVG path 不支持的命令 `{cur_cmd}`（Phase 1 仅支持 M/L/Z）"
                    ),
                    Some(span),
                ));
            }
        }
    }

    if bezier.vertices.is_empty() {
        return Err(LottieError::new(
            ErrorKind::InvalidValue,
            "SVG path 至少需要一个 M 命令",
            Some(span),
        ));
    }

    Ok(bezier)
}

fn invalid_num(s: &str, span: Span) -> LottieError {
    LottieError::new(
        ErrorKind::InvalidValue,
        format!("SVG path 中的非法数字 `{s}`"),
        Some(span),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_check() {
        let p = parse_svg_path("M9.25 -6 L-2.75 6 L-9.25 -0.5", Span::dummy()).unwrap();
        assert_eq!(p.vertices.len(), 3);
        assert_eq!(p.vertices[0], [9.25, -6.0]);
        assert_eq!(p.vertices[2], [-9.25, -0.5]);
        assert!(!p.closed);
    }

    #[test]
    fn close_command_marks_closed() {
        let p = parse_svg_path("M0 0 L10 0 L10 10 Z", Span::dummy()).unwrap();
        assert!(p.closed);
        assert_eq!(p.vertices.len(), 3);
    }

    #[test]
    fn lowercase_close_also_works() {
        let p = parse_svg_path("M0 0 L10 0 z", Span::dummy()).unwrap();
        assert!(p.closed);
    }

    #[test]
    fn relative_move_l_lowercase() {
        let p = parse_svg_path("M10 10 l5 0 l0 5", Span::dummy()).unwrap();
        assert_eq!(p.vertices.len(), 3);
        assert_eq!(p.vertices[1], [15.0, 10.0]);
        assert_eq!(p.vertices[2], [15.0, 15.0]);
    }

    #[test]
    fn empty_path_errors() {
        let err = parse_svg_path("", Span::dummy()).unwrap_err();
        assert_eq!(err.kind, ErrorKind::InvalidValue);
    }

    #[test]
    fn unsupported_command_errors() {
        let err = parse_svg_path("M0 0 C 1 1 2 2 3 3", Span::dummy()).unwrap_err();
        assert!(err.message.contains("C") || err.message.contains("不支持"));
    }

    #[test]
    fn truncated_command_errors() {
        let err = parse_svg_path("M0", Span::dummy()).unwrap_err();
        assert!(err.message.contains("M") || err.message.contains("两个数字"));
    }

    #[test]
    fn invalid_number_errors() {
        let err = parse_svg_path("M0 abc", Span::dummy()).unwrap_err();
        assert!(err.message.contains("非法数字") || err.message.contains("abc"));
    }

    #[test]
    fn no_vertices_errors() {
        let err = parse_svg_path("Z", Span::dummy()).unwrap_err();
        assert!(err.message.contains("M"));
    }
}
