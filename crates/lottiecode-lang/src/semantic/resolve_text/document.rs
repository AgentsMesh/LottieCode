//! 文本 document 字段解析（字体 / 颜色 / 描边 / 对齐）。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

use crate::semantic::resolve_shape::transform_attrs::AttrIndex;
use crate::semantic::util::*;

pub fn build_text_document(
    attrs: &AttrIndex,
    fonts: &mut Vec<IrFont>,
    span: crate::token::Span,
) -> Result<IrTextDocument> {
    let font_family = font_family(attrs)?;
    let weight = font_weight(attrs);
    let font_name = format!("{font_family}-{}", capitalize(&weight));
    register_font(fonts, &font_family, &weight);

    let font_size = attrs
        .resolved("size")
        .map(|e| expect_number(e, span))
        .transpose()?
        .unwrap_or(16.0);

    let fill_color = parse_fill_color(attrs, span)?;
    let line_height = attrs
        .resolved("line-height")
        .map(|e| expect_number(e, span))
        .transpose()?;
    let tracking = attrs
        .resolved("tracking")
        .map(|e| expect_number(e, span))
        .transpose()?
        .unwrap_or(0.0);
    let justify = parse_justify(attrs);
    let (stroke_color, stroke_width) = parse_stroke(attrs, span);

    Ok(IrTextDocument {
        text: String::new(),
        font_family,
        font_name,
        font_size,
        fill_color,
        line_height,
        tracking,
        justify,
        stroke_color,
        stroke_width,
    })
}

fn font_family(attrs: &AttrIndex) -> Result<String> {
    Ok(attrs
        .resolved("font")
        .map(extract_string)
        .transpose()?
        .unwrap_or_else(|| "Arial".to_string()))
}

fn font_weight(attrs: &AttrIndex) -> String {
    attrs
        .resolved("weight")
        .and_then(|e| match e {
            Expression::Ident(s, _) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "Regular".to_string())
}

fn register_font(fonts: &mut Vec<IrFont>, family: &str, weight: &str) {
    let style = capitalize(weight);
    let name = format!("{family}-{style}");
    if !fonts.iter().any(|f| f.name == name) {
        fonts.push(IrFont {
            family: family.to_string(),
            style,
            name,
        });
    }
}

fn parse_fill_color(attrs: &AttrIndex, span: crate::token::Span) -> Result<[f64; 3]> {
    if let Some(e) = attrs.resolved("color") {
        let rgba = match e {
            Expression::ColorLit(hex, sp) => parse_color(hex, *sp)?,
            _ => {
                return Err(LottieError::new(
                    ErrorKind::TypeMismatch,
                    "text.color 需为颜色字面量",
                    Some(span),
                ));
            }
        };
        Ok([rgba[0], rgba[1], rgba[2]])
    } else {
        Ok([0.0, 0.0, 0.0])
    }
}

fn parse_justify(attrs: &AttrIndex) -> u8 {
    attrs
        .resolved("align")
        .and_then(|e| match e {
            Expression::Ident(s, _) => Some(s.as_str()),
            _ => None,
        })
        .map(|s| match s {
            "left" => 0,
            "right" => 1,
            "center" => 2,
            _ => 0,
        })
        .unwrap_or(0)
}

fn parse_stroke(
    attrs: &AttrIndex,
    span: crate::token::Span,
) -> (Option<[f64; 3]>, Option<f64>) {
    let color = attrs
        .resolved("stroke-color")
        .and_then(|e| match e {
            Expression::ColorLit(hex, sp) => parse_color(hex, *sp).ok(),
            _ => None,
        })
        .map(|rgba| [rgba[0], rgba[1], rgba[2]]);
    let width = attrs
        .resolved("stroke-width")
        .and_then(|e| expect_number(e, span).ok());
    (color, width)
}

fn extract_string(expr: &Expression) -> Result<String> {
    match expr {
        Expression::StringLit(s, _) => Ok(s.clone()),
        Expression::Ident(s, _) => Ok(s.clone()),
        _ => Err(LottieError::new(
            ErrorKind::TypeMismatch,
            "期望字符串或标识符",
            Some(expr.span()),
        )),
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
