use nalgebra::Vector3;
use rc_shared::CHUNK_SIZE;

#[inline]
pub fn global_to_local_position(vector: Vector3<i32>) -> (Vector3<i32>, Vector3<usize>) {
    // Locate block
    let inner_loc = Vector3::new(
        ((vector.x as usize % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
        ((vector.y as usize % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
        ((vector.z as usize % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
    );

    // Locate chunk
    let chunk_loc = Vector3::new(
        (vector.x as f32 / CHUNK_SIZE as f32).floor() as i32,
        (vector.y as f32 / CHUNK_SIZE as f32).floor() as i32,
        (vector.z as f32 / CHUNK_SIZE as f32).floor() as i32,
    );

    (chunk_loc, inner_loc)
}
