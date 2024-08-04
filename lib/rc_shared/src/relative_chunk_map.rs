use nalgebra::Vector3;
use crate::CHUNK_SIZE;

// Chunk data, with the ability to index slightly outside the bounds
pub struct RelativeChunkMap<T: Sized + Copy> {
    position: Vector3<i32>,
    expansion: usize,
    pub data: Vec<Vec<Vec<T>>>
}

impl<T: Default + Copy> RelativeChunkMap<T> {
    pub fn new_empty(position: Vector3<i32>, expansion: usize) -> Self {
        RelativeChunkMap {
            position,
            expansion,
            data: vec![vec![vec![T::default(); CHUNK_SIZE + (expansion * 2)]; CHUNK_SIZE + (expansion * 2)]; CHUNK_SIZE + (expansion * 2)],
        }
    }
}

impl<T: Copy> RelativeChunkMap<T> {
    pub fn get(&self, position: Vector3<i32>) -> Option<&T> {
        let position = position - self.position +
            Vector3::new(self.expansion as i32, self.expansion as i32, self.expansion as i32);

        if let Some(position) = position.try_cast::<usize>() {
            self.data.get(position.x)
                .map(|y| {
                    y.get(position.y)
                        .map(|z| z.get(position.z))
                })
                .flatten()
                .flatten()
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, position: [i32; 3]) -> Option<&mut T> {
        let position = Vector3::new(position[0], position[1], position[2]) - self.position +
            Vector3::new(self.expansion as i32, self.expansion as i32, self.expansion as i32);

        if let Some(position) = position.try_cast::<usize>() {
            self.data.get_mut(position.x)
                .map(|y| {
                    y.get_mut(position.y)
                        .map(|z| z.get_mut(position.z))
                })
                .flatten()
                .flatten()
        } else {
            None
        }
    }
    pub fn set(&mut self, position: [i32; 3], value: T) {
        if let Some(entry) = self.get_mut(position) {
            *entry = value;
        }
    }
}



#[cfg(test)]
mod tests {
    
    use nalgebra::{ Vector3};
    use crate::relative_chunk_map::RelativeChunkMap;

    #[test]
    fn check_positions() {
        let chunk_map: RelativeChunkMap<bool> = RelativeChunkMap::new_empty(Vector3::new(1,2,2), 1);

        assert!(chunk_map.get(Vector3::new(1, 2, 2)).is_some());
        assert!(chunk_map.get(Vector3::new(0, 2, 2)).is_some());
        assert!(chunk_map.get(Vector3::new(0, 1, 2)).is_some());

        assert!(chunk_map.get(Vector3::new(0, 0, 2)).is_none());
        assert!(chunk_map.get(Vector3::new(1, 2, 20)).is_none());
    }
}