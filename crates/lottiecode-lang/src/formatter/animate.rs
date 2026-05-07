//! Animate / Keyframe / Geometry 块格式化。

use crate::ast::*;

use super::expr::format_expr;

pub fn format_animate(buf: &mut String, a: &AnimateDecl, indent: usize) {
    let pad = "    ".repeat(indent);
    buf.push_str(&format!("{pad}animate {} {{\n", a.property));
    for kf in &a.keyframes {
        let hold = if kf.hold { "hold " } else { "" };
        let ease_str = match &kf.easing {
            None => String::new(),
            Some(EasingExpr::Named(n)) => format!(" ease = {n}"),
            Some(EasingExpr::Cubic(c)) => {
                format!(" ease = cubic({}, {}, {}, {})", c[0], c[1], c[2], c[3])
            }
        };
        buf.push_str(&format!(
            "{pad}    {hold}{}: {}{ease_str}\n",
            format_expr(&kf.time),
            format_expr(&kf.value)
        ));
    }
    let suffix = if a.looped { " loop" } else { "" };
    buf.push_str(&format!("{pad}}}{suffix}\n"));
}

pub fn format_geometry(buf: &mut String, g: &GeometryDecl, indent: usize) {
    let pad = "    ".repeat(indent);
    let (name, attrs) = match g {
        GeometryDecl::Rect { attributes, .. } => ("rect", attributes),
        GeometryDecl::Ellipse { attributes, .. } => ("ellipse", attributes),
        GeometryDecl::PolyStar { attributes, .. } => ("polystar", attributes),
        GeometryDecl::Path { attributes, .. } => {
            if attrs_has_only_d(attributes) {
                if let Some(d) = attributes.iter().find(|a| a.key == "d") {
                    buf.push_str(&format!("{pad}path = {}\n", format_expr(&d.value)));
                    return;
                }
            }
            ("path", attributes)
        }
    };
    buf.push_str(&format!("{pad}{name} {{ "));
    let parts: Vec<String> = attrs
        .iter()
        .map(|a| format!("{} = {}", a.key, format_expr(&a.value)))
        .collect();
    buf.push_str(&parts.join(", "));
    buf.push_str(" }\n");
}

fn attrs_has_only_d(attrs: &[Attribute]) -> bool {
    attrs.len() == 1 && attrs[0].key == "d"
}
