use nalgebra::Vector2;
use crate::CHUNK_SIZE;

// Chunk data, with the ability to index slightly outside the bounds
pub struct RelativeChunkFlatMap<T: Sized + Copy> {
    position: Vector2<i32>,
    expansion: usize,
    pub data: Vec<Vec<T>>
}

impl<T: Default + Copy> RelativeChunkFlatMap<T> {
    pub fn new_empty(position: Vector2<i32>, expansion: usize) -> Self {
        RelativeChunkFlatMap {
            position,
            expansion,
            data: vec![vec![T::default(); CHUNK_SIZE + (expansion * 2)]; CHUNK_SIZE + (expansion * 2)],
        }
    }
}

impl<T: Copy> RelativeChunkFlatMap<T> {
    pub fn get(&self, position: [i32; 2]) -> Option<&T> {
        let position = Vector2::new(position[0], position[1]) - self.position +
            Vector2::new(self.expansion as i32, self.expansion as i32);

        if let Some(position) = position.try_cast::<usize>() {
            self.data.get(position.x)
                .map(|y| {
                    y.get(position.y)
                })
                .flatten()
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, position: [i32; 2]) -> Option<&mut T> {
        let position = Vector2::new(position[0], position[1]) - self.position +
            Vector2::new(self.expansion as i32, self.expansion as i32);

        if let Some(position) = position.try_cast::<usize>() {
            self.data.get_mut(position.x)
                .map(|y| {
                    y.get_mut(position.y)
                })
                .flatten()
        } else {
            None
        }
    }
    pub fn set(&mut self, position: [i32; 2], value: T) {
        if let Some(entry) = self.get_mut(position) {
            *entry = value;
        }
    }
}



#[cfg(test)]
mod tests {

    use nalgebra::{ Vector2 };
    use crate::relative_chunk_flat_map::RelativeChunkFlatMap;

    #[test]
    fn check_positions() {
        let chunk_map: RelativeChunkFlatMap<bool> = RelativeChunkFlatMap::new_empty(Vector2::new(1,2), 1);

        assert!(chunk_map.get([1, 2]).is_some());
        assert!(chunk_map.get([0, 2]).is_some());
        assert!(chunk_map.get([0, 1]).is_some());
        assert!(chunk_map.get([5, 5]).is_some());

        assert!(chunk_map.get([0, 0]).is_none());
        assert!(chunk_map.get([1, -2]).is_none());
    }
}