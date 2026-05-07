//! `lc plan` —— 树形打印 IR 结构（不输出 JSON）。

use std::path::Path;

use lottiecode_lang::ir::*;

use crate::pipeline::compile_file;

pub fn run(file: &Path) -> Result<(), String> {
    let ir = compile_file(file)?;
    println!("composition \"{}\"", ir.name);
    println!("  画布     {} × {}", ir.width, ir.height);
    println!("  fps      {}", ir.fps);
    println!("  时长     {} 帧 ({:.3}s)", ir.out_frame, ir.out_frame / ir.fps);
    println!("  图层数   {}", ir.layers.len());
    for (i, layer) in ir.layers.iter().enumerate() {
        println!("    [{}] {}", i + 1, layer.name);
        match &layer.kind {
            IrLayerKind::Shape(shape_layer) => {
                print_shape_layer(shape_layer, 6);
            }
            IrLayerKind::Text(_) => println!("      (text)"),
            IrLayerKind::Image { asset_id } => println!("      (image asset={asset_id})"),
            IrLayerKind::Precomp { asset_id, width, height, .. } => {
                println!("      (precomp asset={asset_id} {width}×{height})")
            }
            IrLayerKind::Solid { width, height, .. } => println!("      (solid {width}×{height})"),
            IrLayerKind::Null => println!("      (null controller)"),
            IrLayerKind::Audio { asset_id, .. } => println!("      (audio asset={asset_id})"),
            IrLayerKind::Data { asset_id } => println!("      (data asset={asset_id})"),
            IrLayerKind::Camera { .. } => println!("      (camera)"),
        }
    }
    Ok(())
}

fn print_shape_layer(layer: &IrShapeLayer, indent: usize) {
    let pad = " ".repeat(indent);
    for g in &layer.groups {
        println!(
            "{pad}gr \"{}\" ({} geometries, {} modifiers, {} styles)",
            g.name,
            g.geometries.len(),
            g.modifiers.len(),
            g.styles.len()
        );
        for geo in &g.geometries {
            print_geometry(geo, indent + 2);
        }
        for m in &g.modifiers {
            print_modifier(m, indent + 2);
        }
        for st in &g.styles {
            print_style(st, indent + 2);
        }
        if g.repeater.is_some() {
            println!("{}rp", " ".repeat(indent + 2));
        }
    }
    if let Some(tm) = &layer.layer_trim {
        print_modifier(tm, indent);
    }
}

fn print_geometry(g: &IrGeometry, indent: usize) {
    let pad = " ".repeat(indent);
    match g {
        IrGeometry::Rectangle { .. } => println!("{pad}rc"),
        IrGeometry::Ellipse { .. } => println!("{pad}el"),
        IrGeometry::PolyStar { .. } => println!("{pad}sr"),
        IrGeometry::Path { bezier, .. } => {
            let n = match bezier {
                AnimatableValue::Static(b) => b.vertices.len(),
                AnimatableValue::Animated(kfs) => kfs.first().map(|k| k.value.vertices.len()).unwrap_or(0),
                AnimatableValue::Expression { default, .. } => default.vertices.len(),
            };
            println!("{pad}sh ({} 顶点)", n);
        }
    }
}

fn print_modifier(m: &IrPathModifier, indent: usize) {
    let pad = " ".repeat(indent);
    match m {
        IrPathModifier::TrimPath { .. } => println!("{pad}tm"),
        IrPathModifier::RoundedCorners { .. } => println!("{pad}rd"),
        IrPathModifier::Merge { .. } => println!("{pad}mm"),
        IrPathModifier::OffsetPath { .. } => println!("{pad}op"),
        IrPathModifier::PuckerBloat { .. } => println!("{pad}pb"),
        IrPathModifier::Twist { .. } => println!("{pad}tw"),
        IrPathModifier::Zigzag { .. } => println!("{pad}zz"),
    }
}

fn print_style(st: &IrStyle, indent: usize) {
    let pad = " ".repeat(indent);
    match st {
        IrStyle::Fill { .. } => println!("{pad}fl"),
        IrStyle::Stroke { width, .. } => {
            let w = if let AnimatableValue::Static(v) = width { *v } else { -1.0 };
            println!("{pad}st (width={w})");
        }
        IrStyle::GradientFill { .. } => println!("{pad}gf"),
        IrStyle::GradientStroke { .. } => println!("{pad}gs"),
    }
}
