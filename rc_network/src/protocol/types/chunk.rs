use crate::protocol::data::read_types::{read_longarray, read_short, read_unsignedbyte, read_long, read_varint};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct NetworkChunk {
    pub block_count: i16,
    pub bits_per_block: u8,
    pub data: [[[u32; 16]; 16]; 16],
}

impl NetworkChunk {
    pub fn deserialize<T: Read>(buf: &mut T, len: i64) -> Vec<NetworkChunk> {
        let chunks_count = bits_set(len);
        let mut chunks = Vec::with_capacity(chunks_count as usize);

        for _ in 0..chunks_count {
            chunks.push(NetworkChunk::load(buf));
        }

        chunks
    }

    // https://wiki.vg/Chunk_Format#Data_structure
    pub fn load<T: Read>(stream: &mut T) -> NetworkChunk {
        let block_count = read_short(stream);

        let mut bits_per_block = read_unsignedbyte(stream);

        if bits_per_block <= 4 {
            bits_per_block = 4;
        }

        let palette = if bits_per_block < 9 {
            let len = read_varint(stream);
            let mut maps = Vec::new();
            for _ in 0..len {
                maps.push(read_varint(stream));
            }
            Some(maps)
        } else {
            None
        };

        let mut output = Vec::new();
        let mut current_number = 0;
        let mut bits = 0;

        let longs_len = read_varint(stream);

        let data = read_longarray(stream, longs_len as u16);

        // Read compacted chunk block data
        for mut byte in data {
            for _ in 0..64 {
                if byte & 0b10000000_00000000_00000000_00000000 != 0 {
                    current_number ^= 0b1;
                };

                byte <<= 1;

                bits += 1;

                if bits == bits_per_block as u32 {
                    bits = 0;
                    output.push(current_number);
                    current_number = 0;
                }

                current_number <<= 1
            }
        }

        let mut block_map: [[[u32; 16]; 16]; 16] = [[[0; 16]; 16]; 16];
        let mut i = 0;
        // Convert into 3d block map
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    let id = if palette.is_some() {
                        let palette = palette.as_ref().unwrap();
                        if output.get(i).unwrap() >= &(palette.len() as i32) {
                            0
                        } else {
                            palette.get(*output.get(i).unwrap() as usize).unwrap().clone()
                        }
                    } else {
                        output.get(i).unwrap().clone() as i64
                    };
                    block_map[x][y][z] = if id == 0 { 0 } else { ((id % 6) + 1) as u32 };
                    i += 1;
                }
            }
        }

        NetworkChunk {
            block_count,
            bits_per_block,
            data: block_map,
        }
    }
}

fn bits_set(mut value: i64) -> i32 {
    let mut count = 0;
    while value > 0 {
        if (value & 1) == 1 {
            count += 1;
        }
        value >>= 1;
    }

    count
}
