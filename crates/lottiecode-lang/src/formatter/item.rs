//! CompositionItem 格式化（layer 级条目）。

use crate::ast::*;

use super::animate::{format_animate, format_geometry};
use super::expr::format_expr;

pub fn format_item(buf: &mut String, item: &CompositionItem, indent: usize) {
    let pad = "    ".repeat(indent);
    match item {
        CompositionItem::Shape(sh) => format_shape(buf, sh, indent, &pad),
        CompositionItem::Text(td) => format_text(buf, td, indent, &pad),
        CompositionItem::Use(usd) => format_use(buf, usd, &pad),
        CompositionItem::Image(img) => format_simple_block(buf, "image", &img.name, &img.attributes, &pad),
        CompositionItem::Precomp(plyr) => {
            format_simple_block(buf, "precomp", &plyr.name, &plyr.attributes, &pad)
        }
        CompositionItem::Solid(s) => format_simple_block(buf, "solid", &s.name, &s.attributes, &pad),
        CompositionItem::Controller(c) => {
            format_simple_block(buf, "controller", &c.name, &c.attributes, &pad)
        }
        CompositionItem::Camera(cam) => {
            format_simple_block(buf, "camera", &cam.name, &cam.attributes, &pad)
        }
    }
}

fn format_shape(buf: &mut String, sh: &ShapeDecl, indent: usize, pad: &str) {
    buf.push_str(&format!("{pad}shape {} {{\n", sh.name));
    for geo in &sh.geometries {
        format_geometry(buf, geo, indent + 1);
    }
    for attr in &sh.attributes {
        buf.push_str(&format!("{pad}    {} = {}\n", attr.key, format_expr(&attr.value)));
    }
    for anim in &sh.animations {
        format_animate(buf, anim, indent + 1);
    }
    if let Some(t) = &sh.trim {
        buf.push_str(&format!("{pad}    trim {{\n"));
        for a in &t.attributes {
            buf.push_str(&format!(
                "{pad}        {} = {}\n",
                a.key,
                format_expr(&a.value)
            ));
        }
        buf.push_str(&format!("{pad}    }}\n"));
    }
    buf.push_str(&format!("{pad}}}\n"));
}

fn format_text(buf: &mut String, td: &TextDecl, indent: usize, pad: &str) {
    buf.push_str(&format!("{pad}text \"{}\" {{\n", td.content));
    for a in &td.attributes {
        buf.push_str(&format!("{pad}    {} = {}\n", a.key, format_expr(&a.value)));
    }
    for an in &td.animations {
        format_animate(buf, an, indent + 1);
    }
    buf.push_str(&format!("{pad}}}\n"));
}

fn format_use(buf: &mut String, usd: &UseDecl, pad: &str) {
    let args: Vec<String> = usd
        .args
        .iter()
        .map(|a| format!("{} = {}", a.key, format_expr(&a.value)))
        .collect();
    if args.is_empty() {
        buf.push_str(&format!("{pad}use {}\n", usd.component_name));
    } else {
        buf.push_str(&format!(
            "{pad}use {}({})\n",
            usd.component_name,
            args.join(", ")
        ));
    }
}

fn format_simple_block(buf: &mut String, kw: &str, name: &str, attrs: &[Attribute], pad: &str) {
    buf.push_str(&format!("{pad}{kw} {name} {{\n"));
    for a in attrs {
        buf.push_str(&format!("{pad}    {} = {}\n", a.key, format_expr(&a.value)));
    }
    buf.push_str(&format!("{pad}}}\n"));
}
