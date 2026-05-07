//! 表达式与子结构上的参数替换 helpers。

use std::collections::HashMap;

use crate::ast::*;

/// 把表达式中的 `Ident(name)` 替换为参数对应值；其他递归处理。
pub fn substitute_expr(expr: &Expression, args: &HashMap<String, Expression>) -> Expression {
    match expr {
        Expression::Ident(name, _) => args.get(name).cloned().unwrap_or_else(|| expr.clone()),
        Expression::Array(items, span) => Expression::Array(
            items.iter().map(|e| substitute_expr(e, args)).collect(),
            *span,
        ),
        Expression::Object(attrs, span) => Expression::Object(
            attrs.iter().map(|a| substitute_attr(a, args)).collect(),
            *span,
        ),
        _ => expr.clone(),
    }
}

pub fn substitute_attr(a: &Attribute, args: &HashMap<String, Expression>) -> Attribute {
    Attribute {
        key: a.key.clone(),
        value: substitute_expr(&a.value, args),
        span: a.span,
    }
}

pub fn substitute_animate(an: &AnimateDecl, args: &HashMap<String, Expression>) -> AnimateDecl {
    AnimateDecl {
        property: an.property.clone(),
        keyframes: an
            .keyframes
            .iter()
            .map(|kf| Keyframe {
                time: substitute_expr(&kf.time, args),
                value: substitute_expr(&kf.value, args),
                easing: kf.easing.clone(),
                hold: kf.hold,
                spatial_to: kf.spatial_to.as_ref().map(|e| substitute_expr(e, args)),
                spatial_ti: kf.spatial_ti.as_ref().map(|e| substitute_expr(e, args)),
                span: kf.span,
            })
            .collect(),
        looped: an.looped,
        span: an.span,
    }
}

pub fn substitute_geometry(g: &GeometryDecl, args: &HashMap<String, Expression>) -> GeometryDecl {
    let map_attrs = |attrs: &[Attribute]| -> Vec<Attribute> {
        attrs.iter().map(|a| substitute_attr(a, args)).collect()
    };
    match g {
        GeometryDecl::Rect { attributes, span } => GeometryDecl::Rect {
            attributes: map_attrs(attributes),
            span: *span,
        },
        GeometryDecl::Ellipse { attributes, span } => GeometryDecl::Ellipse {
            attributes: map_attrs(attributes),
            span: *span,
        },
        GeometryDecl::PolyStar { attributes, span } => GeometryDecl::PolyStar {
            attributes: map_attrs(attributes),
            span: *span,
        },
        GeometryDecl::Path { attributes, span } => GeometryDecl::Path {
            attributes: map_attrs(attributes),
            span: *span,
        },
    }
}

pub fn substitute_shape(sh: &ShapeDecl, args: &HashMap<String, Expression>) -> ShapeDecl {
    ShapeDecl {
        name: sh.name.clone(),
        geometries: sh.geometries.iter().map(|g| substitute_geometry(g, args)).collect(),
        attributes: sh.attributes.iter().map(|a| substitute_attr(a, args)).collect(),
        animations: sh.animations.iter().map(|an| substitute_animate(an, args)).collect(),
        trim: sh.trim.as_ref().map(|t| TrimDecl {
            attributes: t.attributes.iter().map(|a| substitute_attr(a, args)).collect(),
            span: t.span,
        }),
        masks: sh
            .masks
            .iter()
            .map(|m| MaskDecl {
                attributes: m.attributes.iter().map(|a| substitute_attr(a, args)).collect(),
                animations: m.animations.iter().map(|an| substitute_animate(an, args)).collect(),
                span: m.span,
            })
            .collect(),
        groups: sh
            .groups
            .iter()
            .map(|g| ShapeGroupDecl {
                name: g.name.clone(),
                geometries: g.geometries.iter().map(|geo| substitute_geometry(geo, args)).collect(),
                attributes: g.attributes.iter().map(|a| substitute_attr(a, args)).collect(),
                span: g.span,
            })
            .collect(),
        span: sh.span,
    }
}

pub fn substitute_text(td: &TextDecl, args: &HashMap<String, Expression>) -> TextDecl {
    TextDecl {
        content: td.content.clone(),
        attributes: td.attributes.iter().map(|a| substitute_attr(a, args)).collect(),
        animations: td.animations.iter().map(|an| substitute_animate(an, args)).collect(),
        typewriter: td.typewriter.clone(),
        span: td.span,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Span;

    fn sp() -> Span {
        Span::new(0, 0, 1, 1)
    }

    #[test]
    fn substitute_ident_with_arg() {
        let mut args = HashMap::new();
        args.insert("color".into(), Expression::ColorLit("FF0000".into(), sp()));
        let expr = Expression::Ident("color".into(), sp());
        let out = substitute_expr(&expr, &args);
        assert!(matches!(out, Expression::ColorLit(_, _)));
    }

    #[test]
    fn substitute_unknown_ident_unchanged() {
        let args = HashMap::new();
        let expr = Expression::Ident("foo".into(), sp());
        let out = substitute_expr(&expr, &args);
        assert!(matches!(out, Expression::Ident(s, _) if s == "foo"));
    }

    #[test]
    fn substitute_recurses_into_array() {
        let mut args = HashMap::new();
        args.insert("w".into(), Expression::IntLit(100, sp()));
        let arr = Expression::Array(
            vec![
                Expression::Ident("w".into(), sp()),
                Expression::IntLit(50, sp()),
            ],
            sp(),
        );
        if let Expression::Array(items, _) = substitute_expr(&arr, &args) {
            assert!(matches!(items[0], Expression::IntLit(100, _)));
            assert!(matches!(items[1], Expression::IntLit(50, _)));
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn substitute_recurses_into_object() {
        let mut args = HashMap::new();
        args.insert("h".into(), Expression::IntLit(200, sp()));
        let obj = Expression::Object(
            vec![Attribute {
                key: "height".into(),
                value: Expression::Ident("h".into(), sp()),
                span: sp(),
            }],
            sp(),
        );
        if let Expression::Object(attrs, _) = substitute_expr(&obj, &args) {
            assert!(matches!(attrs[0].value, Expression::IntLit(200, _)));
        } else {
            panic!("expected object");
        }
    }

    #[test]
    fn substitute_literal_unchanged() {
        let args = HashMap::new();
        let lit = Expression::IntLit(42, sp());
        let out = substitute_expr(&lit, &args);
        assert!(matches!(out, Expression::IntLit(42, _)));
    }
}
