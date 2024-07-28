use std::mem;
use std::mem::MaybeUninit;
use nalgebra::Vector3;
use rc_shared::CHUNK_SIZE;
use rc_shared::helpers::global_to_local_position;

/// Stores information for each block in a 3x3 of chunk data centered on `position`
struct NearbyChunkMap<T: Sized + Copy> {
    position: Vector3<i32>,
    data: [[[T; CHUNK_SIZE * 3]; CHUNK_SIZE * 3]; CHUNK_SIZE * 3]
}

impl<T: Default + Copy> Default for NearbyChunkMap<T> {
    fn default() -> Self {
        NearbyChunkMap {
            position: Default::default(),
            data: [[[T::default(); CHUNK_SIZE * 3]; CHUNK_SIZE * 3]; CHUNK_SIZE * 3],
        }
    }
}

impl<T: Copy> NearbyChunkMap<T> {
    pub fn new(position: Vector3<i32>, data: [[[T; CHUNK_SIZE * 3]; CHUNK_SIZE * 3]; CHUNK_SIZE * 3]) -> Self {
        NearbyChunkMap {
            position,
            data,
        }
    }

    pub fn get_iter_mut(&self) -> NearbyChunkIterator<T> {

        return unimplemented!();
        NearbyChunkIterator::new(&self)
    }

    pub fn iter(&mut self, cb: &dyn Fn(&mut T)) {
        for chunk_x in (self.position.x - 1)..=(self.position.x + 1) {
            for chunk_y in (self.position.y - 1)..=(self.position.y + 1) {
                for chunk_z in (self.position.z - 1)..=(self.position.z + 1) {
                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            for z in 0..CHUNK_SIZE {
                                let world_position =
                                    (Vector3::new(chunk_x, chunk_y, chunk_z) * CHUNK_SIZE as i32) +
                                        Vector3::new(x, y, z).cast::<i32>();

                                cb()
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_position_mut(&mut self, world_position: Vector3<i32>) -> Option<&mut T> {

        let index = world_to_index(world_position, self.position);

        self.data
            .get_mut(index.x).map(|y|
        y.get_mut(index.y).map(|z| z.get_mut(index.z))
        )
            .flatten()
            .flatten()
    }

    pub unsafe fn get_position_unchecked(&self, world_position: Vector3<i32>) -> NearbyChunkItem<T> {
        // Localize
        let index = world_to_index(world_position, self.position);

        NearbyChunkItem {
            data: &self.data[index.x][index.y][index.z],
            world_position,
        }
    }
}

fn world_to_index(world_position: Vector3<i32>, index_center: Vector3<i32>) -> Vector3<usize> {
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
pub struct NearbyChunkItem<T> {
    data: T,
    world_position: Vector3<i32>
}

pub struct NearbyChunkIterator<'a, T: Copy> {
    entries: [NearbyChunkItem<'a, T>; (CHUNK_SIZE * 3) * (CHUNK_SIZE * 3) * (CHUNK_SIZE * 3)],
    i: usize
}

impl<'a, T: Copy> NearbyChunkIterator<'a, T> {
    pub fn new(
        data: &'a NearbyChunkMap<T>
    ) -> NearbyChunkIterator<T> {

        return unimplemented!();

        println!("WHAT");

        let chunk_pos = data.position;

        let mut entries = unsafe {
            [const { MaybeUninit::uninit() as MaybeUninit<NearbyChunkItem<T>> }; (CHUNK_SIZE * 3) * (CHUNK_SIZE * 3) * (CHUNK_SIZE * 3)]
        };

        let mut i = 0;



        println!("WHAT");

        let entries = unsafe {
            mem::transmute::<_, [NearbyChunkItem<T>; (CHUNK_SIZE * 3) * (CHUNK_SIZE * 3) * (CHUNK_SIZE * 3)]>(entries)
        };

        Self {
            entries,
            i: 0
        }
    }
}

impl<'a, T: Copy> Iterator for NearbyChunkIterator<'a, T> {
    type Item = NearbyChunkItem<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.i == (CHUNK_SIZE*3)*(CHUNK_SIZE*3)*(CHUNK_SIZE*3) {
            return None;
        }

        let result = self.entries[self.i];

        self.i += 1;

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use bevy::tasks::futures_lite::StreamExt;
    use nalgebra::{Vector, Vector3};
    use crate::systems::chunk::nearby_chunk_map::NearbyChunkMap;

    #[test]
    fn benchmark_chunk_building() {
        let mut data: NearbyChunkMap<bool> = NearbyChunkMap::new(
            Vector3::new(1, 0, 0),
            [[[false; 48]; 48]; 48]
        );

        println!("hmm {:?}", unsafe { data.get_position_unchecked(Vector3::new(17, 0, 0)) });

        *data.get_position_mut(Vector3::new(17, 0, 0)).unwrap() = true;

        println!("hmm {:?}", unsafe { data.get_position_unchecked(Vector3::new(17, 0, 0)) });

        data.get_iter_mut();

        return

        data.get_iter_mut().for_each(|mut t| {

            if t.world_position == Vector3::new(17, 0, 0) {
                println!("{:?}", t.data);
            }

            println!("?? {:?}", t.data);

        });


    }
}