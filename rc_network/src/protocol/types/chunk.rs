use crate::protocol::data::read_types::{read_unsignedbyte, read_ushort, read_varint};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

#[derive(Debug)]
pub struct NetworkChunk {
    pub block_count: u16,
    pub bits_per_block: u8,
    pub data: [[[u32; 16]; 16]; 16],
}

impl NetworkChunk {
    pub fn deserialize<T: Read>(buf: &mut T, len: i32, debug: bool) -> Vec<NetworkChunk> {
        let chunks_count = bits_set(len);
        let mut chunks = Vec::with_capacity(chunks_count as usize);

        for _ in 0..chunks_count {
            //if debug {
            chunks.push(NetworkChunk::load(buf, debug));
            // } else {
            //     chunks.push(NetworkChunk {
            //         block_count: 0,
            //         bits_per_block: 0,
            //         data: [[[0; 16]; 16]; 16],
            //     });
            // }
        }

        chunks
    }

    // https://web.archive.org/web/20201111224656/https://wiki.vg/Chunk_Format#Data_structure
    pub fn load<T: Read>(stream: &mut T, debug: bool) -> NetworkChunk {
        let block_count = read_ushort(stream);

        let mut bits_per_block = read_unsignedbyte(stream);

        if bits_per_block <= 4 {
            bits_per_block = 4;
        }

        // Indirect method
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

        let longs_len = read_varint(stream);

        if debug {
            log!("Bits: {}", bits_per_block);
            log!("Palette: {:?}", palette);
            log!("Data Size: {:?} bits", longs_len * 64);
            assert_eq!((longs_len * 64) / bits_per_block as i32, 4096)
        }

        let mut data_vec = Vec::new();
        for _ in 0..longs_len {
            data_vec.push(stream.read_u64::<BigEndian>().unwrap());
        }

        let mut data = BitStreamReader::new(data_vec.clone(), bits_per_block);
        let mut i = 0;

        let mut block_map: [[[u32; 16]; 16]; 16] = [[[0; 16]; 16]; 16];

        // Convert into 3d block map
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    let val = data.get(i);
                    i += 1;

                    let id = if palette.is_some() {
                        let palette = palette.as_ref().unwrap();
                        if val >= palette.len() as usize {
                            log_error!(format!(
                                "Value ({}) out of range of palette ({})",
                                val,
                                palette.len()
                            ));
                            0
                        } else {
                            if *palette.get(val as usize).unwrap() >= 3363 {
                                1397
                            } else {
                                *palette.get(val as usize).unwrap()
                            }
                        }
                    } else {
                        val as i32
                    };

                    block_map[x][y][z] = id as u32;
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

fn bits_set(mut value: i32) -> i32 {
    let mut count = 0;
    while value > 0 {
        if (value & 1) == 1 {
            count += 1;
        }
        value >>= 1;
    }

    count
}

fn get_bits(num: u64, start_bit: u8, end_bit: u8) -> u64 {
    let len = end_bit - start_bit;
    let mask = u64::max_value() >> (63 - len) << (63 - end_bit);
    (num & mask).reverse_bits()
}

pub struct BitStreamReader {
    data: Vec<u64>,
    bit_size: u8,
}

impl BitStreamReader {
    pub fn new(data: Vec<u64>, bit_size: u8) -> BitStreamReader {
        BitStreamReader { data, bit_size }
    }

    pub fn get(&self, i: usize) -> usize {
        let section_start = i * self.bit_size as usize;

        let long_index = section_start / 64;
        let long_offset = section_start % 64;

        let mask = (1 << self.bit_size) - 1;

        let data_index = (section_start + self.bit_size as usize - 1) / 64;

        if data_index == long_index {
            ((self.data[long_index as usize] >> long_offset) & mask) as usize
        } else {
            (((self.data[long_index] >> long_offset) | (self.data[data_index] << 64 - long_offset))
                & mask) as usize
        }
    }
}
