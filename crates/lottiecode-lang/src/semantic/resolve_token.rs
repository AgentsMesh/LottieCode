//! Token 引用解析与值规范化。

use std::collections::HashMap;

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};

pub type TokenMap = HashMap<String, Expression>;

/// 把所有 `token group_name { k = v }` 展平为 `group_name.k → v`。
/// 把所有 `token group_name { k = v }` 展平为 `group_name.k → v`。
/// slots 也走同样路径，namespace 名固定为 "slot"。
pub fn build_token_map(groups: &[TokenGroupDecl]) -> TokenMap {
    let mut map = HashMap::new();
    for g in groups {
        for entry in &g.entries {
            map.insert(format!("{}.{}", g.name, entry.key), entry.value.clone());
        }
    }
    map
}

/// 把 slots 加入 token_map，namespace = "slot"。
pub fn add_slots_to_map(map: &mut TokenMap, slots: &[Attribute]) {
    for s in slots {
        map.insert(format!("slot.{}", s.key), s.value.clone());
    }
}

/// 把表达式中的 `Path(["color","success"])` 替换为 token 实际值。
/// 其他表达式保持不变（递归处理 Array/Object）。
pub fn resolve_value(expr: &Expression, tokens: &TokenMap) -> Result<Expression> {
    match expr {
        Expression::Path(parts, span) => {
            let key = parts.join(".");
            tokens.get(&key).cloned().ok_or_else(|| {
                LottieError::new(
                    ErrorKind::UndefinedToken,
                    format!("未定义的 token `{key}`"),
                    Some(*span),
                )
            })
        }
        Expression::Array(items, span) => {
            let resolved: Result<Vec<_>> = items.iter().map(|e| resolve_value(e, tokens)).collect();
            Ok(Expression::Array(resolved?, *span))
        }
        Expression::Object(attrs, span) => {
            let resolved: Result<Vec<_>> = attrs
                .iter()
                .map(|a| {
                    Ok(Attribute {
                        key: a.key.clone(),
                        value: resolve_value(&a.value, tokens)?,
                        span: a.span,
                    })
                })
                .collect();
            Ok(Expression::Object(resolved?, *span))
        }
        _ => Ok(expr.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Span;

    fn sp() -> Span {
        Span::new(0, 0, 1, 1)
    }

    fn group(name: &str, entries: Vec<(&str, Expression)>) -> TokenGroupDecl {
        TokenGroupDecl {
            name: name.to_string(),
            entries: entries
                .into_iter()
                .map(|(k, v)| Attribute { key: k.to_string(), value: v, span: sp() })
                .collect(),
            span: sp(),
        }
    }

    #[test]
    fn build_map_flattens_namespaced_keys() {
        let g = group("color", vec![("brand", Expression::ColorLit("FF0000".into(), sp()))]);
        let m = build_token_map(&[g]);
        assert!(m.contains_key("color.brand"));
    }

    #[test]
    fn add_slots_uses_slot_namespace() {
        let mut m = HashMap::new();
        let slots = vec![Attribute {
            key: "primary".into(),
            value: Expression::IntLit(1, sp()),
            span: sp(),
        }];
        add_slots_to_map(&mut m, &slots);
        assert!(m.contains_key("slot.primary"));
    }

    #[test]
    fn resolve_path_substitutes_value() {
        let g = group("ease", vec![("fast", Expression::FloatLit(0.25, sp()))]);
        let m = build_token_map(&[g]);
        let path = Expression::Path(vec!["ease".into(), "fast".into()], sp());
        let resolved = resolve_value(&path, &m).unwrap();
        assert!(matches!(resolved, Expression::FloatLit(v, _) if (v - 0.25).abs() < 1e-9));
    }

    #[test]
    fn resolve_undefined_token_errors() {
        let m = HashMap::new();
        let path = Expression::Path(vec!["x".into(), "y".into()], sp());
        let err = resolve_value(&path, &m).unwrap_err();
        assert_eq!(err.kind, ErrorKind::UndefinedToken);
    }

    #[test]
    fn resolve_recurses_into_arrays() {
        let g = group("size", vec![("w", Expression::IntLit(100, sp()))]);
        let m = build_token_map(&[g]);
        let arr = Expression::Array(
            vec![
                Expression::Path(vec!["size".into(), "w".into()], sp()),
                Expression::IntLit(200, sp()),
            ],
            sp(),
        );
        let resolved = resolve_value(&arr, &m).unwrap();
        if let Expression::Array(items, _) = resolved {
            assert!(matches!(items[0], Expression::IntLit(100, _)));
            assert!(matches!(items[1], Expression::IntLit(200, _)));
        } else {
            panic!("expected array");
        }
    }

    #[test]
    fn non_path_expressions_pass_through() {
        let m = HashMap::new();
        let lit = Expression::IntLit(42, sp());
        let resolved = resolve_value(&lit, &m).unwrap();
        assert!(matches!(resolved, Expression::IntLit(42, _)));
    }
}
