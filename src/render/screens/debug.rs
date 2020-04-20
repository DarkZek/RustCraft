use crate::render::RenderState;
use wgpu::{CommandEncoder, SwapChainOutput};
use wgpu_glyph::{Section, GlyphBrushBuilder, Scale, Layout, HorizontalAlign, VerticalAlign, BuiltInLineBreaker};
use systemstat::Platform;

pub fn draw_debug_screen(render: &mut RenderState, encoder: &mut CommandEncoder, frame: &SwapChainOutput) {

    let ui_scale = ((render.size.width as f32 / render.size.height as f32) / 2.0) * (0.04 * render.size.height as f32);

    let mut brush = GlyphBrushBuilder::using_font_bytes(&render.font).expect("Error in font")
        .texture_filter_method(wgpu::FilterMode::Nearest)
        .build(&render.device, wgpu::TextureFormat::Bgra8UnormSrgb);

    let stats_left: [(&str, String); 7] = [("Rustcraft {}", String::from("0.01.01")),
        ("{} FPS ", render.fps.to_string()),
        ("{} Vertices", render.services.chunk.vertices.clone().to_string()),
        ("Render Distance {}", render.services.settings.render_distance.to_string()),
        ("X: {}", render.camera.eye.x.to_string()),
        ("Y {}", render.camera.eye.y.to_string()),
        ("Z {}", render.camera.eye.z.to_string())];

    let mem = match render.system_info.memory() {
        Ok(mem) => {
            format!("?/{}MB", (mem.total.0 as f32 * 0.000001 as f32).round())
        }
        Err(_) => String::from("?")
    };

    let stats_right: [(&str, String); 2] = [("Rust {}", String::from("64bit")),
        ("Mem: {} ", mem)];

    for (i, section) in stats_left.iter().enumerate() {

        let text = section.0.replace("{}", &section.1);

        brush.queue(Section {
            text: &text,
            color: [1.0, 1.0, 1.0, 1.0],
            screen_position: (ui_scale / 2.0, ui_scale * (i as f32 + 0.5)),
            scale: Scale { x: ui_scale, y: ui_scale },
            ..Section::default()
        });
    }

    for (i, section) in stats_right.iter().enumerate() {

        let text = section.0.replace("{}", &section.1);

        brush.queue(Section {
            text: &text,
            color: [1.0, 1.0, 1.0, 1.0],
            screen_position: (render.size.width as f32 - (ui_scale / 2.0), ui_scale * (i as f32 + 0.5)),
            scale: Scale { x: ui_scale, y: ui_scale },
            layout: Layout::SingleLine {h_align: HorizontalAlign::Right, v_align: VerticalAlign::Center, line_breaker: BuiltInLineBreaker::UnicodeLineBreaker},
            ..Section::default()
        });
    }

    if let Err(e) = brush.draw_queued(
        &render.device,
        encoder,
        &frame.view,
        render.size.width,
        render.size.height,
    ) {
        log_error!("Failed to draw frame! {}", e);
    }
}