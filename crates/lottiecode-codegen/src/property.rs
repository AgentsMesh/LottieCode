use serde_json::{json, Value};

use lottiecode_lang::ir::{AnimatableValue, Easing, Keyframe};

pub fn emit_scalar(v: &AnimatableValue<f64>) -> Value {
    match v {
        AnimatableValue::Static(x) => json!({ "a": 0, "k": x }),
        AnimatableValue::Animated(kfs) => json!({ "a": 1, "k": emit_keyframes_scalar(kfs) }),
        AnimatableValue::Expression { default, expr } => {
            json!({ "a": 0, "k": default, "x": expr })
        }
    }
}

pub fn emit_xy(v: &AnimatableValue<[f64; 2]>) -> Value {
    match v {
        AnimatableValue::Static(arr) => json!({ "a": 0, "k": arr.to_vec() }),
        AnimatableValue::Animated(kfs) => json!({ "a": 1, "k": emit_keyframes_array(kfs) }),
        AnimatableValue::Expression { default, expr } => {
            json!({ "a": 0, "k": default.to_vec(), "x": expr })
        }
    }
}

pub fn emit_xyz(v: &AnimatableValue<[f64; 3]>) -> Value {
    match v {
        AnimatableValue::Static(arr) => json!({ "a": 0, "k": arr.to_vec() }),
        AnimatableValue::Animated(kfs) => json!({ "a": 1, "k": emit_keyframes_array(kfs) }),
        AnimatableValue::Expression { default, expr } => {
            json!({ "a": 0, "k": default.to_vec(), "x": expr })
        }
    }
}

pub fn emit_color(v: &AnimatableValue<[f64; 4]>) -> Value {
    match v {
        AnimatableValue::Static(arr) => json!({ "a": 0, "k": arr.to_vec() }),
        AnimatableValue::Animated(kfs) => json!({ "a": 1, "k": emit_keyframes_array4(kfs) }),
        AnimatableValue::Expression { default, expr } => {
            json!({ "a": 0, "k": default.to_vec(), "x": expr })
        }
    }
}

fn emit_keyframes_scalar(kfs: &[Keyframe<f64>]) -> Vec<Value> {
    let mut out = Vec::with_capacity(kfs.len());
    for (i, kf) in kfs.iter().enumerate() {
        let mut obj = serde_json::Map::new();
        // 源帧的 i/o 取「下一帧」的 easing：DSL 把 ease 写在目标帧上。
        if i + 1 < kfs.len() {
            insert_easing(&mut obj, kfs[i + 1].easing);
        }
        obj.insert("t".to_string(), json!(kf.frame));
        obj.insert("s".to_string(), json!([kf.value]));
        if kf.hold {
            obj.insert("h".to_string(), json!(1));
        }
        out.push(Value::Object(obj));
    }
    out
}

fn emit_keyframes_array<const N: usize>(kfs: &[Keyframe<[f64; N]>]) -> Vec<Value> {
    let mut out = Vec::with_capacity(kfs.len());
    for (i, kf) in kfs.iter().enumerate() {
        let mut obj = serde_json::Map::new();
        if i + 1 < kfs.len() {
            insert_easing(&mut obj, kfs[i + 1].easing);
        }
        obj.insert("t".to_string(), json!(kf.frame));
        obj.insert("s".to_string(), json!(kf.value.to_vec()));
        if kf.hold {
            obj.insert("h".to_string(), json!(1));
        }
        if let Some(ti) = kf.spatial_in {
            obj.insert("ti".to_string(), json!(ti.to_vec()));
        }
        if let Some(to) = kf.spatial_out {
            obj.insert("to".to_string(), json!(to.to_vec()));
        }
        out.push(Value::Object(obj));
    }
    out
}

fn emit_keyframes_array4(kfs: &[Keyframe<[f64; 4]>]) -> Vec<Value> {
    let mut out = Vec::with_capacity(kfs.len());
    for (i, kf) in kfs.iter().enumerate() {
        let mut obj = serde_json::Map::new();
        if i + 1 < kfs.len() {
            insert_easing(&mut obj, kfs[i + 1].easing);
        }
        obj.insert("t".to_string(), json!(kf.frame));
        obj.insert("s".to_string(), json!(kf.value.to_vec()));
        if kf.hold {
            obj.insert("h".to_string(), json!(1));
        }
        out.push(Value::Object(obj));
    }
    out
}

fn insert_easing(obj: &mut serde_json::Map<String, Value>, easing: Easing) {
    let c = easing.as_cubic();
    // c = [P1.x, P1.y, P2.x, P2.y]；Lottie o = 起始切线 (P1)，i = 终点切线 (P2)
    obj.insert("o".to_string(), json!({ "x": [c[0]], "y": [c[1]] }));
    obj.insert("i".to_string(), json!({ "x": [c[2]], "y": [c[3]] }));
}


#[cfg(test)]
#[path = "property_tests.rs"]
mod tests;
