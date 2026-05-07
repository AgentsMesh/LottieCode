//! Formatter —— AST → 规范化 DSL 文本。
//!
//! 用于 `lc fmt` 命令；保证 round-trip 后语义不变。
//! 子模块按职责拆分：
//! - `expr` —— 表达式与 KV 序列化
//! - `animate` —— animate / 几何块
//! - `composition` —— 顶层块（token / component / composition / precomp）
//! - `item` —— layer 级条目分发

mod animate;
mod composition;
mod expr;
mod item;

use crate::ast::*;

use composition::{format_component, format_composition, format_token_group};

pub fn format(program: &Program) -> String {
    let mut buf = String::new();

    // includes
    for inc in &program.includes {
        buf.push_str(&format!("include \"{}\"\n", inc.path));
    }
    if !program.includes.is_empty() {
        buf.push('\n');
    }

    // tokens
    for tg in &program.tokens {
        format_token_group(&mut buf, tg);
        buf.push('\n');
    }

    // components
    for c in &program.components {
        format_component(&mut buf, c);
        buf.push('\n');
    }

    // composition
    if let Some(comp) = &program.composition {
        format_composition(&mut buf, comp);
    }

    buf
}


#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
