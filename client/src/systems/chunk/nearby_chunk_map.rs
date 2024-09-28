use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use crate::systems::chunk::nearby_cache::NearbyChunkCache;
use rc_shared::CHUNK_SIZE;
use rc_shared::helpers::global_to_local_position;
use crate::systems::chunk::condensed_spacial_data::CondensedSpacialData;
use crate::systems::chunk::data::ChunkData;

/// Stores information for each block in a 3x3 of chunk data centered on `position`
#[derive(Deserialize, Serialize)]
pub struct NearbyChunkMap<T: Sized + Copy> {
    position: Vector3<i32>,
    pub data: CondensedSpacialData<T>
}

impl<T: Default + Copy> Default for NearbyChunkMap<T> {
    fn default() -> Self {
        NearbyChunkMap {
            position: Default::default(),
            data: CondensedSpacialData::new(CHUNK_SIZE * 3)
        }
    }
}

impl<T: Default + Copy> NearbyChunkMap<T> {
    pub fn new_empty(position: Vector3<i32>) -> Self {
        NearbyChunkMap {
            position,
            data: CondensedSpacialData::new(CHUNK_SIZE * 3)
        }
    }
}

impl<T: Copy> NearbyChunkMap<T> {
    pub fn new(position: Vector3<i32>, data: CondensedSpacialData<T>) -> Self {
        NearbyChunkMap {
            position,
            data,
        }
    }

    pub fn for_each<F>(&self, mut cb: F) where F: FnMut(NearbyChunkItem<T>) {
        for chunk_x in (self.position.x - 1)..=(self.position.x + 1) {
            for chunk_y in (self.position.y - 1)..=(self.position.y + 1) {
                for chunk_z in (self.position.z - 1)..=(self.position.z + 1) {
                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            for z in 0..CHUNK_SIZE {
                                let world_position =
                                    (Vector3::new(chunk_x, chunk_y, chunk_z) * CHUNK_SIZE as i32) +
                                        Vector3::new(x, y, z).cast::<i32>();

                                let index = world_to_index_unchecked(world_position, self.position);

                                cb(NearbyChunkItem {
                                    data: self.data.get(index).unwrap(),
                                    world_position,
                                    chunk_position: Vector3::new(chunk_x, chunk_y, chunk_z),
                                    block_position: Vector3::new(x, y, z),
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn for_each_mut<F>(&mut self, mut cb: F) where F: FnMut(NearbyChunkItemMut<T>) {
        for chunk_x in (self.position.x - 1)..=(self.position.x + 1) {
            for chunk_y in (self.position.y - 1)..=(self.position.y + 1) {
                for chunk_z in (self.position.z - 1)..=(self.position.z + 1) {
                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            for z in 0..CHUNK_SIZE {
                                let world_position =
                                    (Vector3::new(chunk_x, chunk_y, chunk_z) * CHUNK_SIZE as i32) +
                                        Vector3::new(x, y, z).cast::<i32>();

                                let index = world_to_index_unchecked(world_position, self.position);

                                cb(NearbyChunkItemMut {
                                    data: self.data.get_mut(index).unwrap(),
                                    world_position,
                                    chunk_position: Vector3::new(chunk_x, chunk_y, chunk_z),
                                    block_position: Vector3::new(x, y, z),
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn for_each_mut_with_chunks<F>(&mut self, nearby_chunk_cache: &NearbyChunkCache, mut cb: F) where F: FnMut(NearbyChunkItemWithChunkMut<T>) {
        for chunk_x in (self.position.x - 1)..=(self.position.x + 1) {
            for chunk_y in (self.position.y - 1)..=(self.position.y + 1) {
                for chunk_z in (self.position.z - 1)..=(self.position.z + 1) {

                    let chunk = nearby_chunk_cache.get_chunk(Vector3::new(chunk_x, chunk_y, chunk_z));

                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            for z in 0..CHUNK_SIZE {
                                let world_position =
                                    (Vector3::new(chunk_x, chunk_y, chunk_z) * CHUNK_SIZE as i32) +
                                        Vector3::new(x, y, z).cast::<i32>();

                                let index = world_to_index_unchecked(world_position, self.position);

                                cb(NearbyChunkItemWithChunkMut {
                                    data: self.data.get_mut(index).unwrap(),
                                    world_position,
                                    chunk_position: Vector3::new(chunk_x, chunk_y, chunk_z),
                                    block_position: Vector3::new(x, y, z),
                                    chunk
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_position(&self, world_position: Vector3<i32>) -> Option<&T> {
        let index = world_to_index(world_position, self.position);

        if let Some(index) = index {
            self.data
                .get(index)

        } else {
            None
        }
    }

    pub fn get_position_mut(&mut self, world_position: Vector3<i32>) -> Option<&mut T> {
        let index = world_to_index(world_position, self.position);

        if let Some(index) = index {
            self.data
                .get_mut(index)

        } else {
            None
        }
    }

    pub unsafe fn get_position_unchecked(&self, world_position: Vector3<i32>) -> NearbyChunkItem<T> {
        // Localize
        let index = world_to_index(world_position, self.position).unwrap();

        let (chunk_position, block_position) = global_to_local_position(world_position);

        NearbyChunkItem {
            data: self.data.get(index).unwrap(),
            world_position,
            chunk_position,
            block_position,
        }
    }

    pub unsafe fn get_position_mut_unchecked(&mut self, world_position: Vector3<i32>) -> NearbyChunkItem<T> {
        // Localize
        let index = world_to_index(world_position, self.position).unwrap();

        let (chunk_position, block_position) = global_to_local_position(world_position);

        NearbyChunkItem {
            data: self.data.get_mut(index).unwrap(),
            world_position,
            chunk_position,
            block_position,
        }
    }
}

#[inline]
fn world_to_index(world_position: Vector3<i32>, index_center: Vector3<i32>) -> Option<Vector3<usize>> {
    // Localize
    let relative_position = world_position - (index_center * CHUNK_SIZE as i32);

    // Break into components
    let (mut chunk_position, block_position) = global_to_local_position(relative_position);

    // Move to make array start at zero
    chunk_position += Vector3::new(1,1,1);

    if chunk_position.x < 0 || chunk_position.x >= 3 ||
        chunk_position.y < 0 || chunk_position.y >= 3 ||
        chunk_position.z < 0 || chunk_position.z >= 3 {
        return None;
    }

    Some(Vector3::new(
        chunk_position.x as usize * CHUNK_SIZE + block_position.x,
        chunk_position.y as usize * CHUNK_SIZE + block_position.y,
        chunk_position.z as usize * CHUNK_SIZE + block_position.z,
    ))
}

#[inline]
fn world_to_index_unchecked(world_position: Vector3<i32>, index_center: Vector3<i32>) -> Vector3<usize> {
    // Localize
    let relative_position = world_position - (index_center * CHUNK_SIZE as i32);

    // Break into components
    let (mut chunk_position, block_position) = global_to_local_position(relative_position);

    // Move to make array start at zero
    chunk_position += Vector3::new(1,1,1);

    Vector3::new(
        chunk_position.x as usize * CHUNK_SIZE + block_position.x,
        chunk_position.y as usize * CHUNK_SIZE + block_position.y,
        chunk_position.z as usize * CHUNK_SIZE + block_position.z,
    )
}

#[derive(Copy, Clone, Debug)]
pub struct NearbyChunkItem<'a, T> {
    pub data: &'a T,
    pub world_position: Vector3<i32>,
    pub chunk_position: Vector3<i32>,
    pub block_position: Vector3<usize>
}

#[derive(Debug)]
pub struct NearbyChunkItemMut<'a, T> {
    pub data: &'a mut T,
    pub world_position: Vector3<i32>,
    pub chunk_position: Vector3<i32>,
    pub block_position: Vector3<usize>
}

#[derive(Debug)]
pub struct NearbyChunkItemWithChunkMut<'a, T> {
    pub data: &'a mut T,
    pub world_position: Vector3<i32>,
    pub chunk_position: Vector3<i32>,
    pub block_position: Vector3<usize>,
    pub chunk: Option<&'a ChunkData>
}

#[cfg(test)]
mod tests {
    use nalgebra::{ Vector3};
    use crate::systems::chunk::condensed_spacial_data::CondensedSpacialData;
    use crate::systems::chunk::nearby_chunk_map::NearbyChunkMap;

    #[test]
    fn check_positions() {
        let mut data: NearbyChunkMap<bool> = NearbyChunkMap::new(
            Vector3::new(1, 0, 0),
            CondensedSpacialData::from_slice([[[false; 48]; 48]; 48])
        );

        *data.get_position_mut(Vector3::new(17, 0, 0)).unwrap() = true;

        let asserted = false;
        data.for_each(|t| {
            if t.world_position == Vector3::new(17, 0, 0) {
                assert_eq!(*t.data, true);
            } else {
                assert_eq!(*t.data, false);
            }
        });

        assert_eq!(asserted, true);
    }

    #[test]
    fn boundary_testing() {
        let mut data: NearbyChunkMap<bool> = NearbyChunkMap::new(
            Vector3::new(0, 0, 0),
            CondensedSpacialData::from_slice([[[false; 48]; 48]; 48])
        );

        assert_eq!(data.get_position_mut(Vector3::new(-16, 0, 0)).is_some(), true);
        assert_eq!(data.get_position_mut(Vector3::new(-17, 0, 0)).is_some(), false);
    }
}