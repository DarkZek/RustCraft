use bevy::prelude::Component;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::ShaderType;

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PostProcessSettings {
    pub intensity: f32,
    // WebGL2 structs must be 16 byte aligned.
    #[cfg(feature = "webgl2")]
    pub _webgl2_padding: Vec3,
}