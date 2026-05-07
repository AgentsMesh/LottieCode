//! Layer-level transform 属性解析。

use std::collections::HashMap;

use crate::ast::*;
use crate::error::Result;
use crate::ir::*;

use crate::semantic::resolve_animate::*;
use crate::semantic::resolve_token::*;
use crate::semantic::util::*;

/// 按 key 索引 attribute（保持原 attribute 引用以拿 span）。
pub struct AttrIndex<'a> {
    map: HashMap<String, &'a Attribute>,
    /// 已解析的 value 缓存，确保 token 已替换。
    pub resolved: HashMap<String, Expression>,
}

impl<'a> AttrIndex<'a> {
    pub fn find(&self, key: &str) -> Option<&'a Attribute> {
        self.map.get(key).copied()
    }

    pub fn resolved(&self, key: &str) -> Option<&Expression> {
        self.resolved.get(key)
    }
}

/// 把 `[Attribute]` 索引化，并对每个 value 做 token 解析。
pub fn index_attrs<'a>(attrs: &'a [Attribute], tokens: &TokenMap) -> Result<AttrIndex<'a>> {
    let mut map = HashMap::new();
    let mut resolved = HashMap::new();
    for a in attrs {
        map.insert(a.key.clone(), a);
        resolved.insert(a.key.clone(), resolve_value(&a.value, tokens)?);
    }
    Ok(AttrIndex { map, resolved })
}

/// 把 layer-level transform 属性应用到 IrTransform。
pub fn apply_transform_attrs(tr: &mut IrTransform, attrs: &AttrIndex) -> Result<()> {
    if let Some(a) = attrs.find("position") {
        let v = attrs.resolved("position").unwrap();
        tr.position = AnimatableValue::Static(expect_xyz(v, a.span)?);
    }
    if let Some(a) = attrs.find("anchor") {
        let v = attrs.resolved("anchor").unwrap();
        tr.anchor = AnimatableValue::Static(expect_xyz(v, a.span)?);
    }
    if let Some(a) = attrs.find("scale") {
        let v = attrs.resolved("scale").unwrap();
        tr.scale = AnimatableValue::Static(scale_to_xyz(v, a.span)?);
    }
    if let Some(a) = attrs.find("rotation") {
        let v = attrs.resolved("rotation").unwrap();
        tr.rotation = AnimatableValue::Static(expect_number(v, a.span)?);
    }
    if let Some(a) = attrs.find("opacity") {
        let v = attrs.resolved("opacity").unwrap();
        tr.opacity = AnimatableValue::Static(expect_number(v, a.span)?);
    }
    if let Some(a) = attrs.find("skew") {
        let v = attrs.resolved("skew").unwrap();
        tr.skew = AnimatableValue::Static(expect_number(v, a.span)?);
    }
    if let Some(a) = attrs.find("skew-axis") {
        let v = attrs.resolved("skew-axis").unwrap();
        tr.skew_axis = AnimatableValue::Static(expect_number(v, a.span)?);
    }
    Ok(())
}

/// 把 `animate <prop>` 应用到 IrTransform。
pub fn apply_transform_animations(
    tr: &mut IrTransform,
    animations: &[AnimateDecl],
    fps: f64,
    tokens: &TokenMap,
) -> Result<()> {
    for anim in animations {
        match anim.property.as_str() {
            "position" => {
                tr.position = animate_to_value(anim, fps, tokens, conv_xyz)?;
            }
            "anchor" => {
                tr.anchor = animate_to_value(anim, fps, tokens, conv_xyz)?;
            }
            "scale" => {
                tr.scale = animate_to_value(anim, fps, tokens, conv_scale_xyz)?;
            }
            "rotation" => {
                tr.rotation = animate_to_value(anim, fps, tokens, conv_scalar)?;
            }
            "opacity" => {
                tr.opacity = animate_to_value(anim, fps, tokens, conv_scalar)?;
            }
            "skew" => {
                tr.skew = animate_to_value(anim, fps, tokens, conv_scalar)?;
            }
            "skew-axis" => {
                tr.skew_axis = animate_to_value(anim, fps, tokens, conv_scalar)?;
            }
            // trim-* / trim 在 build_trim 里处理；fill/stroke 颜色动画在 build_fill/stroke 里处理
            "trim" | "trim-start" | "trim-end" | "trim-offset" => {}
            "fill" | "fill-color" | "stroke-color" => {}
            "path" => {} // path morph 动画在 resolve_shape 里处理
            other => {
                return Err(crate::error::LottieError::new(
                    crate::error::ErrorKind::InvalidValue,
                    format!("不支持的 animate 属性 `{other}`"),
                    Some(anim.span),
                )
                .with_hint("已知：position / scale / rotation / opacity / anchor / trim*"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Span;

    fn sp() -> Span {
        Span::new(0, 0, 1, 1)
    }

    fn attr(key: &str, value: Expression) -> Attribute {
        Attribute { key: key.to_string(), value, span: sp() }
    }

    #[test]
    fn index_attrs_resolves_token_paths() {
        let mut tokens: TokenMap = HashMap::new();
        tokens.insert("size.w".into(), Expression::IntLit(100, sp()));
        let attrs = vec![attr("width", Expression::Path(vec!["size".into(), "w".into()], sp()))];
        let idx = index_attrs(&attrs, &tokens).unwrap();
        let v = idx.resolved("width").unwrap();
        assert!(matches!(v, Expression::IntLit(100, _)));
    }

    #[test]
    fn apply_position_static() {
        let attrs = vec![attr(
            "position",
            Expression::Array(
                vec![Expression::IntLit(50, sp()), Expression::IntLit(60, sp())],
                sp(),
            ),
        )];
        let idx = index_attrs(&attrs, &HashMap::new()).unwrap();
        let mut tr = IrTransform::default();
        apply_transform_attrs(&mut tr, &idx).unwrap();
        if let AnimatableValue::Static(p) = &tr.position {
            assert_eq!(*p, [50.0, 60.0, 0.0]);
        } else {
            panic!("expected static position");
        }
    }

    #[test]
    fn unknown_animate_property_errors() {
        let anim = AnimateDecl {
            property: "fart".into(),
            keyframes: vec![],
            looped: false,
            span: sp(),
        };
        let mut tr = IrTransform::default();
        let err =
            apply_transform_animations(&mut tr, &[anim], 30.0, &HashMap::new()).unwrap_err();
        assert_eq!(err.kind, crate::error::ErrorKind::InvalidValue);
    }

    #[test]
    fn scale_scalar_broadcasts_to_xyz() {
        let attrs = vec![attr("scale", Expression::IntLit(50, sp()))];
        let idx = index_attrs(&attrs, &HashMap::new()).unwrap();
        let mut tr = IrTransform::default();
        apply_transform_attrs(&mut tr, &idx).unwrap();
        if let AnimatableValue::Static(s) = &tr.scale {
            assert_eq!(*s, [50.0, 50.0, 100.0]);
        } else {
            panic!("expected static scale");
        }
    }
}
