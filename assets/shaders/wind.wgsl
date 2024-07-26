fn get_wind(world_position: vec4<f32>, time: f32) -> vec4<f32>{
    var offsetX = world_position.x + time;
    var offsetZ = world_position.z + time;

    var idk = cos((world_position.x * world_position.z) + time / 10.0) * 1.0;

    return vec4<f32>(sin(offsetX) * 0.05, 0.0, sin(offsetZ + 0.5) * 0.03, 0.0) * idk;
}