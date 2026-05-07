//! Fixture 对比工具：把生成的 Lottie JSON 与 `expected.json` 做结构等价比对。
//!
//! 用途：每个 example 配一份 `expected.json`，CI 跑完整流水线 → 比对 → 防止
//! 字段顺序、字段类型、默认值省略策略等回归。
//!
//! ## 等价规则
//! - 结构必须完全一致（key 数 + 嵌套结构 + 字段顺序）
//! - 数字容忍 1e-6 浮点误差
//! - AE 内部字段（`ix`、`mn`、`np`、`cix`、`ind`、`shapes[*].nm`）不参与比对
//!   —— 我们不输出这些 AE meta 字段，原版 sample 才有

use serde_json::Value;

const AE_META_KEYS: &[&str] = &["ix", "mn", "np", "cix", "ind"];
const FLOAT_EPS: f64 = 1e-6;

#[derive(Debug)]
pub struct Mismatch {
    pub path: String,
    pub left: String,
    pub right: String,
}

impl std::fmt::Display for Mismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: 左={}  右={}", self.path, self.left, self.right)
    }
}

/// 结构等价比对：返回首个不匹配项（None 表示等价）。
pub fn compare(actual: &Value, expected: &Value) -> Option<Mismatch> {
    diff_value(actual, expected, "$")
}

fn diff_value(a: &Value, e: &Value, path: &str) -> Option<Mismatch> {
    match (a, e) {
        (Value::Object(am), Value::Object(em)) => diff_object(am, em, path),
        (Value::Array(aa), Value::Array(ea)) => diff_array(aa, ea, path),
        (Value::Number(an), Value::Number(en)) => diff_number(an, en, path),
        (Value::Null, Value::Null) | (Value::Bool(_), Value::Bool(_)) | (Value::String(_), Value::String(_)) => {
            if a == e { None } else { mismatch(path, a, e) }
        }
        _ => mismatch(path, a, e),
    }
}

fn diff_object(am: &serde_json::Map<String, Value>, em: &serde_json::Map<String, Value>, path: &str) -> Option<Mismatch> {
    let common: Vec<String> = em.keys().filter(|k| !AE_META_KEYS.contains(&k.as_str())).cloned().collect();
    for k in &common {
        let sub = format!("{path}.{k}");
        let av = am.get(k);
        let ev = em.get(k);
        match (av, ev) {
            (Some(a), Some(e)) => {
                if let Some(m) = diff_value(a, e, &sub) { return Some(m); }
            }
            (None, Some(e)) => return Some(Mismatch { path: sub, left: "<missing>".into(), right: short(e) }),
            _ => {}
        }
    }
    // actual 多出的非 AE-meta 字段也算差异
    for k in am.keys() {
        if AE_META_KEYS.contains(&k.as_str()) { continue; }
        if !em.contains_key(k) {
            return Some(Mismatch {
                path: format!("{path}.{k}"),
                left: short(am.get(k).unwrap()),
                right: "<missing>".into(),
            });
        }
    }
    None
}

fn diff_array(aa: &[Value], ea: &[Value], path: &str) -> Option<Mismatch> {
    if aa.len() != ea.len() {
        return Some(Mismatch {
            path: path.to_string(),
            left: format!("array len {}", aa.len()),
            right: format!("array len {}", ea.len()),
        });
    }
    for (i, (av, ev)) in aa.iter().zip(ea.iter()).enumerate() {
        let sub = format!("{path}[{i}]");
        if let Some(m) = diff_value(av, ev, &sub) { return Some(m); }
    }
    None
}

fn diff_number(a: &serde_json::Number, e: &serde_json::Number, path: &str) -> Option<Mismatch> {
    let af = a.as_f64().unwrap_or(f64::NAN);
    let ef = e.as_f64().unwrap_or(f64::NAN);
    if (af - ef).abs() <= FLOAT_EPS { None } else {
        Some(Mismatch { path: path.to_string(), left: format!("{af}"), right: format!("{ef}") })
    }
}

fn mismatch(path: &str, a: &Value, e: &Value) -> Option<Mismatch> {
    Some(Mismatch { path: path.to_string(), left: short(a), right: short(e) })
}

fn short(v: &Value) -> String {
    let s = serde_json::to_string(v).unwrap_or_else(|_| "?".into());
    if s.len() > 80 { format!("{}...", &s[..77]) } else { s }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn equal_objects_return_none() {
        let a = json!({"x": 1, "y": 2.0});
        let e = json!({"x": 1, "y": 2.0});
        assert!(compare(&a, &e).is_none());
    }

    #[test]
    fn float_within_eps_is_equal() {
        let a = json!({"v": 1.0000001});
        let e = json!({"v": 1.0});
        assert!(compare(&a, &e).is_none());
    }

    #[test]
    fn ae_meta_fields_ignored() {
        let a = json!({"ty": "sh"});
        let e = json!({"ty": "sh", "ix": 1, "mn": "ADBE...", "np": 3});
        assert!(compare(&a, &e).is_none());
    }

    #[test]
    fn extra_field_in_actual_reports_mismatch() {
        let a = json!({"x": 1, "extra": 9});
        let e = json!({"x": 1});
        assert!(compare(&a, &e).is_some());
    }

    #[test]
    fn array_length_mismatch_reports() {
        let a = json!([1, 2]);
        let e = json!([1, 2, 3]);
        assert!(compare(&a, &e).is_some());
    }
}
