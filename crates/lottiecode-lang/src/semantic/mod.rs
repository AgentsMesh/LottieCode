//! Semantic 分析 —— AST → IR。
//!
//! 顶层流水线：tokens + slots → components → assets → meta → dispatch → markers/has_3d。

mod anchor_promote;
mod dispatch;
mod meta;
mod resolve_animate;
mod resolve_component;
mod resolve_image;
mod resolve_include;
mod resolve_precomp;
mod resolve_shape;
mod resolve_solid_null;
mod resolve_text;
mod resolve_token;
mod util;

pub use resolve_include::load_includes;

use std::collections::HashMap;

use dispatch::dispatch_items;
use meta::{build_component_map, build_markers, build_slots, parse_composition_meta, CompositionMeta};
use resolve_image::resolve_asset;
use resolve_precomp::resolve_precomp_decl;
use resolve_token::{add_slots_to_map, build_token_map};

use crate::ast::*;
use crate::error::{ErrorKind, LottieError, Result};
use crate::ir::*;

pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    pub fn analyze(program: &Program) -> Result<IrAnimation> {
        let composition = program.composition.as_ref().ok_or_else(|| {
            LottieError::new(ErrorKind::NoComposition, "缺少顶层 `composition` 声明", None)
                .with_hint("文件至少需要一个 `composition \"name\" { ... }`")
        })?;

        let mut token_map = build_token_map(&program.tokens);
        add_slots_to_map(&mut token_map, &program.slots);
        let component_map = build_component_map(&program.components)?;

        let mut assets: Vec<IrAsset> = Vec::new();
        let mut asset_map: HashMap<String, IrAsset> = HashMap::new();
        let mut fonts: Vec<IrFont> = Vec::new();

        for a in &program.assets {
            if asset_map.contains_key(&a.name) {
                return Err(LottieError::new(
                    ErrorKind::DuplicateDefinition,
                    format!("asset `{}` 重复定义", a.name),
                    Some(a.span),
                ));
            }
            let ia = resolve_asset(a, &token_map)?;
            asset_map.insert(a.name.clone(), ia.clone());
            assets.push(ia);
        }
        for p in &program.precomps {
            if asset_map.contains_key(&p.name) {
                return Err(LottieError::new(
                    ErrorKind::DuplicateDefinition,
                    format!("precomp `{}` 与已存在的 asset/precomp 同名", p.name),
                    Some(p.span),
                ));
            }
            let ia = resolve_precomp_decl(p, &token_map, &component_map, &asset_map, &mut fonts)?;
            asset_map.insert(p.name.clone(), ia.clone());
            assets.push(ia);
        }

        let CompositionMeta { width, height, fps, out_frame } =
            parse_composition_meta(composition, &token_map)?;

        let layers = dispatch_items(
            &composition.items,
            &component_map,
            &asset_map,
            &token_map,
            fps,
            out_frame,
            &mut fonts,
        )?;

        let has_3d = layers.iter().any(|l| l.transform.three_d);

        Ok(IrAnimation {
            name: composition.name.clone(),
            version: "5.7.5".to_string(),
            fps,
            in_frame: 0.0,
            out_frame,
            width,
            height,
            layers,
            assets,
            fonts,
            markers: build_markers(composition, fps)?,
            slots: build_slots(&program.slots)?,
            has_3d,
        })
    }
}
