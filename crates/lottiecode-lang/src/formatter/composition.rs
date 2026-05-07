//! Composition / Component / Precomp 顶层块格式化。

use crate::ast::*;

use super::expr::format_expr;
use super::item::format_item;

pub fn format_token_group(buf: &mut String, tg: &TokenGroupDecl) {
    buf.push_str(&format!("token {} {{\n", tg.name));
    for entry in &tg.entries {
        buf.push_str(&format!("    {} = {}\n", entry.key, format_expr(&entry.value)));
    }
    buf.push_str("}\n");
}

pub fn format_component(buf: &mut String, c: &ComponentDecl) {
    let params: Vec<String> = c
        .params
        .iter()
        .map(|p| match &p.default {
            Some(d) => format!("{} = {}", p.name, format_expr(d)),
            None => p.name.clone(),
        })
        .collect();
    if params.is_empty() {
        buf.push_str(&format!("component {} {{\n", c.name));
    } else {
        buf.push_str(&format!("component {}({}) {{\n", c.name, params.join(", ")));
    }
    for item in &c.items {
        format_item(buf, item, 1);
    }
    buf.push_str("}\n");
}

pub fn format_composition(buf: &mut String, comp: &CompositionDecl) {
    buf.push_str(&format!("composition \"{}\" {{\n", comp.name));
    for attr in &comp.attributes {
        buf.push_str(&format!("    {} = {}\n", attr.key, format_expr(&attr.value)));
    }
    if !comp.attributes.is_empty() && !comp.items.is_empty() {
        buf.push('\n');
    }
    for item in &comp.items {
        format_item(buf, item, 1);
    }
    buf.push_str("}\n");
}

