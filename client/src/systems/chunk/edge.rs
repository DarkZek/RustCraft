use nalgebra::Vector3;
use crate::systems::chunk::ChunkSystem;
use crate::systems::chunk::flags::ChunkFlagsBitMap;

impl ChunkSystem {
    pub fn recompute_on_edge(&mut self, position: Vector3<i32>) {
        if self.chunks.get(&position).is_none() {
            return
        }

        for x in (position.x - 1)..=(position.x + 1) {
            for y in (position.y - 1)..=(position.y + 1) {
                for z in (position.z - 1)..=(position.z + 1) {
                    if self.chunks.get(&Vector3::new(x, y, z)).is_none() {
                        self.chunks.get_mut(&position).unwrap().flags.add_flag(ChunkFlagsBitMap::AtEdge);
                        return;
                    }
                }
            }
        }

        self.chunks.get_mut(&position).unwrap().flags.remove_flag(ChunkFlagsBitMap::AtEdge);
    }
}