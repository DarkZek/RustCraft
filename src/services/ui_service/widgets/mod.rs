use crate::render::vertices::UIVertex;
use crate::services::asset_service::atlas::ATLAS_LOOKUPS;
use crate::services::ui_service::draw::draw_sprite;
use crate::services::ui_service::fonts::{FontsManager, TextView};
use crate::services::ui_service::meshdata::UIMeshData;
use crate::services::ui_service::{ObjectAlignment, Positioning};
use std::collections::HashMap;
use wgpu::Device;
use winit::dpi::PhysicalSize;

/// Image Manager is a subsystem of the User Interface Service.
/// It's job is to manage images and allow other services to easily create new images on the screen as well as update and delete them.
pub struct WidgetManager {
    widgets: HashMap<usize, Widget>,
    pub model: UIMeshData,
    pub size: PhysicalSize<u32>,
}

impl WidgetManager {
    pub fn new(size: PhysicalSize<u32>) -> WidgetManager {
        WidgetManager {
            widgets: HashMap::new(),
            model: UIMeshData::new(),
            size,
        }
    }

    pub fn create_widget(&mut self) -> WidgetBuilder {
        WidgetBuilder {
            widget: Some(Widget {
                fullscreen: false,
                ty: WidgetType::BUTTON(ButtonState::PRESSED, None),
                pos: [0.0, 0.0],
                scale: [450.0, 40.0],
                alignment: ObjectAlignment::Center,
            }),
            manager: self,
        }
    }

    pub fn add_widget(&mut self, image: Widget, device: &Device) -> WidgetView {
        let id = rand::random::<usize>();
        self.widgets.insert(id, image);

        self.build(device);

        WidgetView { id }
    }

    pub fn build(&mut self, device: &Device) {
        self.model.clear();

        for (id, widget) in self.widgets.iter() {
            draw_widget(
                widget.ty,
                &mut self.model.total_vertices,
                &mut self.model.total_indices,
                widget.pos,
                widget.scale,
                None,
                widget.alignment,
            );
        }

        self.model.build_buf(device);
    }
}

fn draw_widget<'a>(
    ty: WidgetType,
    vertices: &'a mut Vec<UIVertex>,
    indices: &'a mut Vec<u16>,
    pos: [f32; 2],
    scale: [f32; 2],
    color: Option<[f32; 4]>,
    alignment: ObjectAlignment,
) {
    match ty {
        WidgetType::BUTTON(state, _) => {
            draw_button(vertices, indices, pos, scale, color, state, alignment)
        }
    }
}

static BUTTON_TEXTURE_HEIGHT: f32 = 40.0;
static BUTTON_TEXTURE_EDGE_WIDTH: f32 = 10.0;
static BUTTON_TEXTURE_CENTER_WIDTH: f32 = 60.0;

fn draw_button<'a>(
    vertices: &'a mut Vec<UIVertex>,
    indices: &'a mut Vec<u16>,
    mut pos: [f32; 2],
    scale: [f32; 2],
    color: Option<[f32; 4]>,
    state: ButtonState,
    alignment: ObjectAlignment,
) {
    let err = ATLAS_LOOKUPS.get().unwrap().get("mcv3/error").unwrap();
    let (atlas_edge, atlas_center) = match state {
        ButtonState::NORMAL => (
            "gui/widgets/button_pressed/edge",
            "gui/widgets/button_pressed/center",
        ),
        ButtonState::PRESSED => (
            "gui/widgets/button_pressed/edge",
            "gui/widgets/button_pressed/center",
        ),
        ButtonState::DISABLED => (
            "gui/widgets/button_pressed/edge",
            "gui/widgets/button_pressed/center",
        ),
    };

    pos[0] -= scale[0] / 2.0;

    let atlas_edge = ATLAS_LOOKUPS.get().unwrap().get(atlas_edge).unwrap_or(err);
    let atlas_center = ATLAS_LOOKUPS
        .get()
        .unwrap()
        .get(atlas_center)
        .unwrap_or(err);

    // Left edge
    draw_sprite(
        vertices,
        indices,
        pos,
        [BUTTON_TEXTURE_EDGE_WIDTH, BUTTON_TEXTURE_HEIGHT],
        atlas_edge.clone(),
        color.clone(),
    );

    // Right Edge
    draw_sprite(
        vertices,
        indices,
        [pos[0] + scale[0] - (BUTTON_TEXTURE_EDGE_WIDTH), pos[1]],
        [-BUTTON_TEXTURE_EDGE_WIDTH, BUTTON_TEXTURE_HEIGHT],
        atlas_edge.clone(),
        color.clone(),
    );

    let mut x = pos[0] + BUTTON_TEXTURE_EDGE_WIDTH;

    while x + BUTTON_TEXTURE_CENTER_WIDTH < pos[0] + scale[0] {
        // Central bit Edge
        draw_sprite(
            vertices,
            indices,
            [x, pos[1]],
            [BUTTON_TEXTURE_CENTER_WIDTH, BUTTON_TEXTURE_HEIGHT],
            atlas_center.clone(),
            color.clone(),
        );

        x += BUTTON_TEXTURE_CENTER_WIDTH;
    }
}

pub struct WidgetBuilder<'a> {
    widget: Option<Widget>,
    manager: &'a mut WidgetManager,
}

impl WidgetBuilder<'_> {
    pub fn build(mut self, device: &Device) -> WidgetView {
        self.manager.add_widget(self.widget.take().unwrap(), device)
    }
    pub fn set_fullscreen(mut self, fullscreen: bool) -> Self {
        self.widget.as_mut().unwrap().fullscreen = fullscreen;
        self
    }
    pub fn set_type(mut self, ty: WidgetType) -> Self {
        self.widget.as_mut().unwrap().ty = ty;
        self
    }
    pub fn set_button(
        mut self,
        state: ButtonState,
        text: &str,
        manager: &mut FontsManager,
    ) -> Self {
        let pos = self.widget.as_ref().unwrap().pos;

        //pos[0] += (self.widget.as_ref().unwrap().scale[0]) / 2.0;

        let text = manager
            .create_text()
            .set_text_alignment(ObjectAlignment::Center)
            .set_offset(pos)
            .set_positioning(Positioning::Relative)
            .set_background(false)
            .set_text(text)
            .build();

        self.widget.as_mut().unwrap().ty = WidgetType::BUTTON(state, Some(text));
        self
    }
}

pub struct Widget {
    fullscreen: bool,
    ty: WidgetType,
    pos: [f32; 2],
    scale: [f32; 2],
    alignment: ObjectAlignment,
}

#[derive(Copy, Clone)]
pub enum WidgetType {
    BUTTON(ButtonState, Option<TextView>),
}

#[derive(Copy, Clone)]
pub enum ButtonState {
    NORMAL,
    PRESSED,
    DISABLED,
}

pub struct WidgetView {
    id: usize,
}
