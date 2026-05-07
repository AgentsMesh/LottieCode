//! Component 实例化：参数替换 + 复制 component 内 items。
//!
//! 实例化流程：
//! 1. 查找 component
//! 2. 构建参数 map：use 提供的优先，component 默认值兜底
//! 3. 在 component.items 的所有表达式中递归替换参数引用

mod substitute;

use std::collections::HashMap;

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};

use super::resolve_token::TokenMap;
use substitute::*;

/// 实例化一个 use，返回展开后的 CompositionItem 列表（已扁平化所有嵌套 use）。
pub fn instantiate(
    usd: &UseDecl,
    components: &HashMap<String, ComponentDecl>,
    tokens: &TokenMap,
) -> Result<Vec<CompositionItem>> {
    let component = components.get(&usd.component_name).ok_or_else(|| {
        LottieError::new(
            ErrorKind::UndefinedComponent,
            format!("未定义的 component `{}`", usd.component_name),
            Some(usd.span),
        )
    })?;
    let args = build_arg_map(usd, component)?;

    let mut out = Vec::new();
    for item in &component.items {
        expand_item(item, &args, components, tokens, &mut out)?;
    }
    Ok(out)
}

fn build_arg_map(
    usd: &UseDecl,
    component: &ComponentDecl,
) -> Result<HashMap<String, Expression>> {
    let mut args: HashMap<String, Expression> = HashMap::new();
    for arg in &usd.args {
        args.insert(arg.key.clone(), arg.value.clone());
    }
    for param in &component.params {
        if args.contains_key(&param.name) {
            continue;
        }
        match &param.default {
            Some(default) => {
                args.insert(param.name.clone(), default.clone());
            }
            None => {
                return Err(LottieError::new(
                    ErrorKind::InvalidValue,
                    format!("use {} 缺少必需参数 `{}`", usd.component_name, param.name),
                    Some(usd.span),
                ));
            }
        }
    }
    Ok(args)
}

fn expand_item(
    item: &CompositionItem,
    args: &HashMap<String, Expression>,
    components: &HashMap<String, ComponentDecl>,
    tokens: &TokenMap,
    out: &mut Vec<CompositionItem>,
) -> Result<()> {
    match item {
        CompositionItem::Shape(sh) => out.push(CompositionItem::Shape(substitute_shape(sh, args))),
        CompositionItem::Text(td) => out.push(CompositionItem::Text(substitute_text(td, args))),
        CompositionItem::Image(img) => out.push(CompositionItem::Image(ImageLayerDecl {
            name: img.name.clone(),
            attributes: img.attributes.iter().map(|a| substitute_attr(a, args)).collect(),
            span: img.span,
        })),
        CompositionItem::Precomp(plyr) => out.push(CompositionItem::Precomp(PrecompLayerDecl {
            name: plyr.name.clone(),
            attributes: plyr.attributes.iter().map(|a| substitute_attr(a, args)).collect(),
            animations: plyr.animations.iter().map(|an| substitute_animate(an, args)).collect(),
            span: plyr.span,
        })),
        CompositionItem::Solid(s) => out.push(CompositionItem::Solid(SolidDecl {
            name: s.name.clone(),
            attributes: s.attributes.iter().map(|a| substitute_attr(a, args)).collect(),
            animations: s.animations.iter().map(|an| substitute_animate(an, args)).collect(),
            span: s.span,
        })),
        CompositionItem::Controller(c) => out.push(CompositionItem::Controller(ControllerDecl {
            name: c.name.clone(),
            attributes: c.attributes.iter().map(|a| substitute_attr(a, args)).collect(),
            animations: c.animations.iter().map(|an| substitute_animate(an, args)).collect(),
            span: c.span,
        })),
        CompositionItem::Camera(cam) => out.push(CompositionItem::Camera(CameraDecl {
            name: cam.name.clone(),
            attributes: cam.attributes.iter().map(|a| substitute_attr(a, args)).collect(),
            animations: cam.animations.iter().map(|an| substitute_animate(an, args)).collect(),
            span: cam.span,
        })),
        CompositionItem::Use(nested) => {
            let nested_substituted = UseDecl {
                component_name: nested.component_name.clone(),
                args: nested.args.iter().map(|a| substitute_attr(a, args)).collect(),
                span: nested.span,
            };
            out.extend(instantiate(&nested_substituted, components, tokens)?);
        }
    }
    Ok(())
}
