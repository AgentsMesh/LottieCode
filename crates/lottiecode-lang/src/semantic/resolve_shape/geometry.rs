//! Geometry 元素解析（rect / ellipse / polystar / path）→ IrGeometry。

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

use super::geometry_path::resolve_path;
use super::transform_attrs::index_attrs;
use crate::semantic::resolve_token::*;
use crate::semantic::util::*;

pub fn resolve_geometry(geo: &GeometryDecl, tokens: &TokenMap) -> Result<IrGeometry> {
    match geo {
        GeometryDecl::Rect { attributes, span } => {
            let attrs = index_attrs(attributes, tokens)?;
            let position = attrs
                .find("position")
                .map(|a| expect_xy(&a.value, a.span))
                .transpose()?
                .unwrap_or([0.0, 0.0]);
            let size = attrs
                .find("size")
                .map(|a| expect_xy(&a.value, a.span))
                .ok_or_else(|| {
                    LottieError::new(ErrorKind::InvalidValue, "rect 缺少 size", Some(*span))
                })??;
            let radius = attrs
                .find("radius")
                .map(|a| expect_number(&a.value, a.span))
                .transpose()?
                .unwrap_or(0.0);
            Ok(IrGeometry::Rectangle {
                position: s(position),
                size: s(size),
                radius: s(radius),
                direction: PathDirection::Normal,
            })
        }
        GeometryDecl::Ellipse { attributes, span } => {
            let attrs = index_attrs(attributes, tokens)?;
            let position = attrs
                .find("position")
                .map(|a| expect_xy(&a.value, a.span))
                .transpose()?
                .unwrap_or([0.0, 0.0]);
            let size = attrs
                .find("size")
                .map(|a| expect_xy(&a.value, a.span))
                .ok_or_else(|| {
                    LottieError::new(ErrorKind::InvalidValue, "ellipse 缺少 size", Some(*span))
                })??;
            Ok(IrGeometry::Ellipse {
                position: s(position),
                size: s(size),
                direction: PathDirection::Normal,
            })
        }
        GeometryDecl::PolyStar { attributes, span } => resolve_polystar(attributes, *span, tokens),
        GeometryDecl::Path { attributes, span } => resolve_path(attributes, *span, tokens),
    }
}

fn resolve_polystar(
    attributes: &[Attribute],
    span: crate::token::Span,
    tokens: &TokenMap,
) -> Result<IrGeometry> {
    let attrs = index_attrs(attributes, tokens)?;
    let position = attrs
        .find("position")
        .map(|a| expect_xy(&a.value, a.span))
        .transpose()?
        .unwrap_or([0.0, 0.0]);
    let points = attrs
        .find("points")
        .map(|a| expect_number(&a.value, a.span))
        .transpose()?
        .unwrap_or(5.0);
    let outer_radius = attrs
        .find("outer-radius")
        .map(|a| expect_number(&a.value, a.span))
        .ok_or_else(|| {
            LottieError::new(
                ErrorKind::InvalidValue,
                "polystar 缺少 outer-radius",
                Some(span),
            )
        })??;
    let inner_radius = attrs
        .find("inner-radius")
        .map(|a| expect_number(&a.value, a.span))
        .transpose()?
        .unwrap_or(outer_radius * 0.5);
    let outer_roundness = attrs
        .find("outer-roundness")
        .map(|a| expect_number(&a.value, a.span))
        .transpose()?
        .unwrap_or(0.0);
    let inner_roundness = attrs
        .find("inner-roundness")
        .map(|a| expect_number(&a.value, a.span))
        .transpose()?
        .unwrap_or(0.0);
    let rotation = attrs
        .find("rotation")
        .map(|a| expect_number(&a.value, a.span))
        .transpose()?
        .unwrap_or(0.0);
    let star_type = attrs
        .find("type")
        .and_then(|a| match &a.value {
            Expression::Ident(s, _) if s == "polygon" => Some(StarType::Polygon),
            Expression::Ident(s, _) if s == "star" => Some(StarType::Star),
            _ => None,
        })
        .unwrap_or(StarType::Star);
    Ok(IrGeometry::PolyStar {
        position: s(position),
        rotation: s(rotation),
        points: s(points),
        outer_radius: s(outer_radius),
        outer_roundness: s(outer_roundness),
        inner_radius: s(inner_radius),
        inner_roundness: s(inner_roundness),
        star_type,
        direction: PathDirection::Normal,
    })
}
