//! 顶层属性解析：composition meta / markers / slots / components map。

use std::collections::HashMap;

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;
use crate::token::Span;

use super::resolve_token::*;
use super::util::*;

/// composition 元数据：画布尺寸 + 时间。
#[derive(Debug, Clone)]
pub(crate) struct CompositionMeta {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub out_frame: f64,
}

pub(crate) fn parse_composition_meta(
    comp: &CompositionDecl,
    tokens: &TokenMap,
) -> Result<CompositionMeta> {
    let mut width: Option<u32> = None;
    let mut height: Option<u32> = None;
    let mut fps: f64 = 30.0;
    let mut duration_sec: f64 = 1.0;

    for attr in &comp.attributes {
        let v = resolve_value(&attr.value, tokens)?;
        match attr.key.as_str() {
            "width" => width = Some(expect_uint(&v, attr.span)?),
            "height" => height = Some(expect_uint(&v, attr.span)?),
            "fps" => fps = expect_number(&v, attr.span)?,
            "duration" => duration_sec = expect_seconds(&v, attr.span, fps)?,
            "markers" => {} // 由 build_markers 处理
            _ => {
                return Err(LottieError::new(
                    ErrorKind::InvalidValue,
                    format!("composition 不支持的属性 `{}`", attr.key),
                    Some(attr.span),
                )
                .with_hint("已知属性：width / height / fps / duration / markers"))
            }
        }
    }

    let width = width.ok_or_else(|| missing("width", comp.span))?;
    let height = height.ok_or_else(|| missing("height", comp.span))?;
    let out_frame = (duration_sec * fps).round();

    Ok(CompositionMeta { width, height, fps, out_frame })
}

pub(crate) fn build_component_map(
    components: &[ComponentDecl],
) -> Result<HashMap<String, ComponentDecl>> {
    let mut map = HashMap::new();
    for c in components {
        if map.insert(c.name.clone(), c.clone()).is_some() {
            return Err(LottieError::new(
                ErrorKind::DuplicateDefinition,
                format!("重复的 component `{}`", c.name),
                Some(c.span),
            ));
        }
    }
    Ok(map)
}

pub(crate) fn build_markers(comp: &CompositionDecl, fps: f64) -> Result<Vec<IrMarker>> {
    for a in &comp.attributes {
        if a.key == "markers" {
            if let Expression::Array(items, _) = &a.value {
                return parse_marker_array(items, fps);
            }
        }
    }
    Ok(Vec::new())
}

fn parse_marker_array(items: &[Expression], fps: f64) -> Result<Vec<IrMarker>> {
    let mut out = Vec::new();
    for it in items {
        if let Expression::Object(fields, _) = it {
            let mut name = String::new();
            let mut time = 0.0;
            let mut duration = 0.0;
            for f in fields {
                match f.key.as_str() {
                    "name" | "label" | "comment" => {
                        if let Expression::StringLit(s, _) = &f.value {
                            name = s.clone();
                        }
                    }
                    "time" | "at" => time = expect_seconds(&f.value, f.span, fps)?,
                    "duration" => duration = expect_seconds(&f.value, f.span, fps)?,
                    _ => {}
                }
            }
            out.push(IrMarker {
                time_frame: (time * fps).round(),
                comment: name,
                duration: (duration * fps).round(),
            });
        }
    }
    Ok(out)
}

pub(crate) fn build_slots(slots: &[Attribute]) -> Result<Vec<IrSlot>> {
    let mut out = Vec::new();
    for a in slots {
        let kind = match &a.value {
            Expression::ColorLit(hex, sp) => IrSlotKind::Color(parse_color(hex, *sp)?),
            Expression::IntLit(n, _) => IrSlotKind::Number(*n as f64),
            Expression::FloatLit(v, _) => IrSlotKind::Number(*v),
            Expression::StringLit(s, _) => IrSlotKind::String(s.clone()),
            _ => continue,
        };
        out.push(IrSlot {
            name: a.key.clone(),
            kind,
        });
    }
    Ok(out)
}

fn missing(field: &str, span: Span) -> LottieError {
    LottieError::new(
        ErrorKind::InvalidValue,
        format!("composition 缺少必需属性 `{field}`"),
        Some(span),
    )
}
