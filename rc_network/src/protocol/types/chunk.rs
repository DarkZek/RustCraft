use crate::protocol::data::read_types::{read_short, read_unsignedbyte, read_varint};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

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

    // https://web.archive.org/web/20201111224656/https://wiki.vg/Chunk_Format#Data_structure
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

        // Technically we should use longs here, but bytes are so much easier to work with.
        let longs_len = read_varint(stream);

        let mut data = Vec::new();
        for _ in 0..longs_len {
            data.push(stream.read_u64::<BigEndian>().unwrap().reverse_bits());
        }

        let mut data = BitStreamReader::new(data);

        let mut block_map: [[[u32; 16]; 16]; 16] = [[[0; 16]; 16]; 16];

        // Convert into 3d block map
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    let val = data.get(bits_per_block);

                    let id = if palette.is_some() {
                        let palette = palette.as_ref().unwrap();
                        if val >= palette.len() as u64 {
                            println!(
                                "Value ({}) out of range of palette ({})",
                                val,
                                palette.len()
                            );
                            return NetworkChunk {
                                block_count,
                                bits_per_block,
                                data: [[[0; 16]; 16]; 16],
                            };
                        } else {
                            palette.get(val as usize).unwrap().clone()
                        }
                    } else {
                        val as i64
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

struct BitStreamReader {
    data: Vec<u64>,
    bit_number: u64,
}

impl BitStreamReader {
    pub fn new(data: Vec<u64>) -> BitStreamReader {
        BitStreamReader {
            data,
            bit_number: 0,
        }
    }

    pub fn remaining(&self) -> usize {
        self.bit_number as usize - (self.data.len() * 64)
    }

    pub fn get(&mut self, bits: u8) -> u64 {
        if (self.bit_number as f32 + bits as f32) / 64.0 > self.data.len() as f32 {
            panic!("Read too much!");
        }

        // Get bits from current byte
        let byte = (self.bit_number as f32 / 64.0).floor() as u32;
        let bit = self.bit_number as u8 % 64;
        let end_bit = if bit + bits >= 64 { 63 } else { bit + bits - 1 };

        let value = *self.data.get(byte as usize).clone().unwrap();
        let mut out = get_bits(value, bit as u8, end_bit as u8);

        out >>= bit;

        self.bit_number += bits as u64;

        out
    }
}

fn get_bits(num: u64, start_bit: u8, end_bit: u8) -> u64 {
    let len = end_bit - start_bit;
    let mask = u64::max_value() >> (63 - len) << (63 - end_bit);
    (num & mask).reverse_bits()
}
