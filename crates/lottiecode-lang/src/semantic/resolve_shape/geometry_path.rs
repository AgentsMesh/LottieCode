//! Path 几何解析：SVG d 字符串 + Bezier 字面量。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::IrGeometry;

use super::path_parse::parse_svg_path;
use super::transform_attrs::{index_attrs, AttrIndex};
use crate::semantic::resolve_token::TokenMap;

pub fn resolve_path(
    attributes: &[Attribute],
    span: crate::token::Span,
    tokens: &TokenMap,
) -> Result<IrGeometry> {
    let attrs = index_attrs(attributes, tokens)?;
    if attrs.find("v").is_some() || attrs.find("vertices").is_some() {
        return resolve_bezier_literal(&attrs, span);
    }
    let d = attrs
        .find("d")
        .ok_or_else(|| LottieError::new(ErrorKind::InvalidValue, "path 缺少 d 或 v", Some(span)))?
        .value
        .clone();
    let path_str = match d {
        Expression::StringLit(s, _) => s,
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                "path 的 d 必须是字符串",
                Some(span),
            ))
        }
    };
    Ok(IrGeometry::Path {
        bezier: crate::ir::AnimatableValue::Static(parse_svg_path(&path_str, span)?),
        direction: crate::ir::PathDirection::Normal,
    })
}

fn resolve_bezier_literal(
    attrs: &AttrIndex,
    span: crate::token::Span,
) -> Result<IrGeometry> {
    let v_expr = attrs.resolved("v").or(attrs.resolved("vertices")).unwrap();
    let vertices = expect_xy_array(v_expr, span, "v")?;
    let n = vertices.len();
    if n < 2 {
        return Err(LottieError::new(
            ErrorKind::InvalidValue,
            "path.v 至少需要 2 个顶点",
            Some(span),
        ));
    }
    let in_tangents = match attrs.resolved("in").or(attrs.resolved("in_tangents")) {
        Some(e) => expect_xy_array(e, span, "in")?,
        None => vec![[0.0, 0.0]; n],
    };
    let out_tangents = match attrs.resolved("out").or(attrs.resolved("out_tangents")) {
        Some(e) => expect_xy_array(e, span, "out")?,
        None => vec![[0.0, 0.0]; n],
    };
    if in_tangents.len() != n || out_tangents.len() != n {
        return Err(LottieError::new(
            ErrorKind::InvalidValue,
            "path 的 in/out 切线数必须与 v 顶点数一致",
            Some(span),
        ));
    }
    let closed = matches!(attrs.resolved("closed"), Some(Expression::BoolLit(true, _)));
    Ok(IrGeometry::Path {
        bezier: crate::ir::AnimatableValue::Static(crate::ir::BezierPath {
            vertices,
            in_tangents,
            out_tangents,
            closed,
        }),
        direction: crate::ir::PathDirection::Normal,
    })
}

fn expect_xy_array(expr: &Expression, span: crate::token::Span, key: &str) -> Result<Vec<[f64; 2]>> {
    let items = match expr {
        Expression::Array(items, _) => items,
        _ => {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                format!("path.{key} 需为嵌套数组 [[x,y], ...]"),
                Some(span),
            ))
        }
    };
    let mut out = Vec::with_capacity(items.len());
    for it in items {
        let pair = it.as_f64_array().ok_or_else(|| {
            LottieError::new(
                ErrorKind::TypeMismatch,
                format!("path.{key} 元素需为 [x, y] 数字数组"),
                Some(span),
            )
        })?;
        if pair.len() != 2 {
            return Err(LottieError::new(
                ErrorKind::TypeMismatch,
                format!("path.{key} 元素需为 2 元素数组，实际 {}", pair.len()),
                Some(span),
            ));
        }
        out.push([pair[0], pair[1]]);
    }
    Ok(out)
}
