use crate::protocol::read_types::{
    read_longarray, read_short, read_unsignedbyte, read_varint, read_varintarray, read_varlongarray,
};
use crate::stream::NetworkStream;

pub struct NetworkChunk {
    pub block_count: i16,
    pub bits_per_block: u8,
    pub data: [[[u32; 16]; 16]; 16],
}

impl NetworkChunk {
    pub fn load_arr(stream: &mut NetworkStream, len: i64) -> Vec<NetworkChunk> {
        let chunks_count = bits_set(len);

        let mut chunks = Vec::with_capacity(chunks_count as usize);

        // for i in 0..chunks_count {
        //     chunks.push(NetworkChunk::load(stream));
        // }
        chunks.push(NetworkChunk::load(stream));

        chunks
    }

    //https://wiki.vg/Chunk_Format#Data_structure
    pub fn load(stream: &mut NetworkStream) -> NetworkChunk {
        let block_count = read_short(stream);
        let mut bits_per_block = read_unsignedbyte(stream);

        let mut output = Vec::new();
        let mut current_number = 0;
        let mut bits = 0;

        let mut read_bytes = stream.get_bytes_read();
        let data = read_longarray(stream, ((16 * 16 * 16 * bits_per_block as i64) / 64) as u16);
        //let data = read_varlongarray(stream);
        read_bytes = stream.get_bytes_read() - read_bytes;
        println!(
            "Actual longs: {}, Expected Longs: {}",
            data.len(),
            (16 * 16 * 16 * bits_per_block as i64) / 64
        );
        println!(
            "Bytes I Read: {}, Bytes I Should Have Read: {} B: {}",
            read_bytes,
            (16 * 16 * 16 * bits_per_block as i64) / 8,
            bits_per_block
        );

        // Read compacted chunk block data
        for mut byte in data {
            for i in 0..64 {
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
