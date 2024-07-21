fn get_wind(world_position: vec4<f32>, time: f32) -> vec4<f32>{
    var offsetX = world_position.x + time;
    var offsetZ = world_position.z + time;

    return vec4<f32>(sin(offsetX) * 0.05, 0.0, sin(offsetZ + 0.5) * 0.03, 0.0);
}