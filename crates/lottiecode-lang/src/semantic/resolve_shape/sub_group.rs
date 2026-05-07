//! Shape 内显式 `group { ... }` 子块解析。

use crate::ast::*;
use crate::error::Result;
use crate::ir::*;

use super::fill_stroke::*;
use super::geometry::resolve_geometry;
use super::transform_attrs::index_attrs;
use crate::semantic::resolve_token::TokenMap;
use crate::semantic::util::*;

/// 把显式 `group { ... }` 子块编译为 IrGroup。
pub fn build_sub_group(
    sub: &ShapeGroupDecl,
    tokens: &TokenMap,
) -> Result<IrGroup> {
    let attrs = index_attrs(&sub.attributes, tokens)?;

    let mut geometries: Vec<IrGeometry> = Vec::new();
    for geo in &sub.geometries {
        geometries.push(resolve_geometry(geo, tokens)?);
    }

    let mut styles: Vec<IrStyle> = Vec::new();
    if let Some(stroke) = attrs.find("stroke") {
        let resolved = attrs.resolved("stroke").unwrap();
        styles.push(build_stroke(resolved, stroke.span, &[], 30.0, tokens)?);
    }
    if let Some(fill) = attrs.find("fill") {
        let resolved = attrs.resolved("fill").unwrap();
        styles.push(build_fill(resolved, fill.span, &[], 30.0, tokens)?);
    }

    let mut tr = IrShapeTransform::default();
    if let Some(p) = attrs.resolved("position") {
        tr.position = AnimatableValue::Static(expect_xy(p, sub.span)?);
    }
    if let Some(a) = attrs.resolved("anchor") {
        tr.anchor = AnimatableValue::Static(expect_xy(a, sub.span)?);
    }
    if let Some(s_v) = attrs.resolved("scale") {
        if let Some(n) = s_v.as_f64() {
            tr.scale = AnimatableValue::Static([n, n]);
        } else {
            tr.scale = AnimatableValue::Static(expect_xy(s_v, sub.span)?);
        }
    }
    if let Some(r) = attrs.resolved("rotation") {
        tr.rotation = AnimatableValue::Static(expect_number(r, sub.span)?);
    }
    if let Some(o) = attrs.resolved("opacity") {
        tr.opacity = AnimatableValue::Static(expect_number(o, sub.span)?);
    }

    Ok(IrGroup {
        name: sub.name.clone().unwrap_or_else(|| "group".to_string()),
        geometries,
        modifiers: Vec::new(),
        styles,
        repeater: None,
        transform: tr,
    })
}
