//! TextDecl → IrLayer。

mod document;

use crate::ast::*;
use crate::error::Result;
use crate::ir::*;

use super::resolve_animate::animate_to_value;
use super::resolve_shape::transform_attrs::*;
use super::resolve_token::*;
use super::util::expect_xy;
use super::util::expect_number;

use document::build_text_document;

pub fn resolve_text(
    td: &TextDecl,
    tokens: &TokenMap,
    fps: f64,
    out_frame: f64,
    fonts: &mut Vec<IrFont>,
) -> Result<IrLayer> {
    let attrs = index_attrs(&td.attributes, tokens)?;

    let mut layer_tr = IrTransform::default();
    apply_transform_attrs(&mut layer_tr, &attrs)?;
    apply_transform_animations(&mut layer_tr, &td.animations, fps, tokens)?;

    let document = build_text_document(&attrs, fonts, td.span)?;
    let typewriter = build_typewriter(td, fps, tokens)?;

    Ok(IrLayer {
        name: format!("Text: {}", &td.content[..td.content.len().min(20)]),
        kind: IrLayerKind::Text(IrTextData {
            document: AnimatableValue::Static(IrTextDocument {
                text: td.content.clone(),
                ..document
            }),
            typewriter,
            char_position: attrs
                .resolved("char-position")
                .and_then(|e| expect_xy(e, td.span).ok()),
            char_scale: attrs.resolved("char-scale").and_then(|e| {
                if let Some(n) = e.as_f64() {
                    Some([n, n])
                } else {
                    expect_xy(e, td.span).ok()
                }
            }),
            char_opacity: attrs
                .resolved("char-opacity")
                .and_then(|e| expect_number(e, td.span).ok()),
            char_rotation: attrs
                .resolved("char-rotation")
                .and_then(|e| expect_number(e, td.span).ok()),
        }),
        transform: layer_tr,
        in_frame: 0.0,
        start_time: 0.0,
        time_stretch: 1.0,
        out_frame,
        parent: None,
        masks: Vec::new(),
        blend_mode: crate::ir::BlendMode::Normal,
        matte_source: false,
        matte: None,
        effects: Vec::new(),
    })
}

fn build_typewriter(
    td: &TextDecl,
    fps: f64,
    tokens: &TokenMap,
) -> Result<Option<AnimatableValue<f64>>> {
    let Some(kfs) = &td.typewriter else {
        return Ok(None);
    };
    if kfs.is_empty() {
        return Ok(None);
    }
    let dummy = AnimateDecl {
        property: "typewriter".to_string(),
        keyframes: kfs.clone(),
        looped: false,
        span: td.span,
    };
    Ok(Some(animate_to_value(&dummy, fps, tokens, |expr| {
        match expr {
            Expression::IntLit(n, _) => Ok(*n as f64),
            Expression::FloatLit(v, _) => Ok(*v),
            _ => Err(crate::error::LottieError::new(
                crate::error::ErrorKind::TypeMismatch,
                "typewriter 关键帧值需为 0-100 数字",
                Some(expr.span()),
            )),
        }
    })?))
}
