use crate::protocol::data::read_types::{read_longarray, read_short, read_unsignedbyte, read_long};
use std::io::{Read};

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
        println!("Block count: {}", block_count);
        let mut bits_per_block = read_unsignedbyte(stream);
        println!("Bits per block: {}", bits_per_block);

        if bits_per_block <=4 {
            bits_per_block = 4;
        }

        let mut output = Vec::new();
        let mut current_number = 0;
        let mut bits = 0;

        let longs_len = read_long(stream);
        println!("Longs: {} {}", 16*16*16, longs_len);

        let data = read_longarray(stream, ((16 * 16 * 16 * bits_per_block as i64) / 64) as u16);

        println!(
            "Actual longs: {}, Expected Longs: {}",
            data.len(),
            (16 * 16 * 16 * bits_per_block as i64) / 64
        );
        // println!(
        //     "Bytes I Read: {}, Bytes I Should Have Read: {} B: {}",
        //     read_bytes,
        //     (16 * 16 * 16 * bits_per_block as i64) / 8,
        //     bits_per_block
        // );

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
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    block_map[x][y][z] = if output.get(i).unwrap() != &0 { 1 } else { 0 };
                    i += 1;
                }
            }
        }

        //println!("Len: {}", output.len());

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
