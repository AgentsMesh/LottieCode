//! Expression / Attribute 格式化辅助。

use crate::ast::*;

pub fn format_expr(e: &Expression) -> String {
    match e {
        Expression::StringLit(s, _) => format!("\"{s}\""),
        Expression::IntLit(n, _) => n.to_string(),
        Expression::FloatLit(n, _) => n.to_string(),
        Expression::BoolLit(b, _) => b.to_string(),
        Expression::TimeSec(v, _) => format!("{v}s"),
        Expression::TimeMs(v, _) => format!("{v}ms"),
        Expression::TimeFrames(n, _) => format!("{n}f"),
        Expression::ColorLit(hex, _) => format!("#{hex}"),
        Expression::Ident(s, _) => s.clone(),
        Expression::Path(parts, _) => parts.join("."),
        Expression::Array(items, _) => {
            let parts: Vec<String> = items.iter().map(format_expr).collect();
            format!("[{}]", parts.join(", "))
        }
        Expression::Object(attrs, _) => {
            let parts: Vec<String> = attrs
                .iter()
                .map(|a| format!("{} = {}", a.key, format_expr(&a.value)))
                .collect();
            format!("{{ {} }}", parts.join(", "))
        }
        Expression::Expr(s, _) => format!("expr({:?})", s),
    }
}
