use anyhow::Context;

use pathfinder_export::{Export, FileFormat};
use pathfinder_geometry::{rect::RectF, vector::vec2f};
use pathfinder_renderer::scene::Scene;
use rex::{
    font::FontContext,
    layout::{engine, Grid, Layout, LayoutSettings, Style},
    parser::{nodes::Color, parse, ParseNode},
    render::{Renderer, SceneWrapper},
    RGBA,
};

pub fn get_svg(input: &str, color: &str) -> anyhow::Result<String> {
    let parsed = parse(&input).ok().context("failed to parse TeX input")?;
    let rgba = RGBA::from_name(color)
        .or_else(|| -> Option<_> {
            let hex = color.trim_start_matches('#');
            let r: u8 = u8::from_str_radix(hex.get(0..2)?, 16).ok()?;
            let g: u8 = u8::from_str_radix(hex.get(2..4)?, 16).ok()?;
            let b: u8 = u8::from_str_radix(hex.get(4..6)?, 16).ok()?;
            let a: u8 = u8::from_str_radix(hex.get(6..8).unwrap_or("ff"), 16).ok()?;
            Some(RGBA(r, g, b, a))
        })
        .unwrap_or(RGBA(0, 0, 0, 255));
    let styled = ParseNode::Color(Color {
        color: rgba,
        inner: parsed,
    });
    let font = font::parse(include_bytes!("../rex-xits.otf"))
        .ok()
        .context("failed to parse font")?
        .downcast_box()
        .ok()
        .context("failed to downcast font")?;

    let mut grid = Grid::new();

    let ctx = FontContext::new(&font);
    let layout_settings = LayoutSettings::new(&ctx, 500.0, Style::Display);
    let node = engine::layout(&[styled], layout_settings)
        .ok()
        .context("failed to generate layout")?
        .as_node();

    grid.insert(0, 0, node);
    let mut layout = Layout::new();
    layout.add_node(grid.build());

    let renderer = Renderer::new();
    let (x0, y0, x1, y1) = renderer.size(&layout);
    let mut scene = Scene::new();
    scene.set_view_box(RectF::from_points(
        vec2f(x0 as f32, y0 as f32),
        vec2f(x1 as f32, y1 as f32),
    ));
    let mut backend = SceneWrapper::new(&mut scene);
    renderer.render(&layout, &mut backend);

    let mut buf = Vec::new();
    scene.export(&mut buf, FileFormat::SVG)?;
    let svg = String::from_utf8(buf)?;
    Ok(svg)
}
