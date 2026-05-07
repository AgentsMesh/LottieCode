//! Modifier 修饰器分发：路径操作 + 路径变形器。

mod deformers;
mod path_ops;

use crate::ast::Expression;

pub use deformers::{build_pucker, build_twist, build_zigzag};
pub use path_ops::{build_merge, build_offset_path, build_repeater};

pub(super) fn identifier_str(expr: &Expression) -> Option<&str> {
    match expr {
        Expression::Ident(s, _) => Some(s.as_str()),
        _ => None,
    }
}
