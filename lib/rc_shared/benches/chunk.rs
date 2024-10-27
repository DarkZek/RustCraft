use rc_shared::chunk::RawChunkData;

pub fn get_chunk() -> RawChunkData {
    let mut data = [[[0; 16]; 16]; 16];

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                data[x][y][z] = if y == 15 {
                    0
                } else if y == 14 {
                    4
                } else if y == 14 {
                    3
                } else {
                    1
                }
            }
        }
    }

    data
}