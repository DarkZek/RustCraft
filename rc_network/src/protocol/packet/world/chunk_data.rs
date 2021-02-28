use crate::protocol::data::read_types::{read_bool, read_int, read_unsignedbyte, read_varint};
use crate::protocol::packet::PacketType;
use crate::protocol::types::chunk::NetworkChunk;
use nbt::Blob;
use std::io::Cursor;

#[derive(Debug)]
pub struct ChunkDataPacket {
    pub x: i32,
    pub z: i32,
    // True = New chunk, False = Existing chunk
    pub full_chunk: bool,
    pub primary_bit_mask: i64,
    pub heightmaps: Blob,
    pub biomes: Option<Vec<i32>>,
    pub data: Vec<NetworkChunk>,
    pub block_entities: Vec<Blob>,
}

impl PacketType for ChunkDataPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let x = read_int(buf);
        let z = read_int(buf);

        let full_chunk = read_bool(buf);
        let primary_bit_mask = read_varint(buf);

        let heightmaps = Blob::from_reader(buf).unwrap();
        let biomes = if full_chunk {
            let mut arr = Vec::new();
            for _ in 0..1024 {
                arr.push(read_int(buf));
            }
            Some(arr)
        } else {
            None
        };

        let data_len = read_varint(buf);
        let mut data = Vec::new();

        for _ in 0..data_len {
            data.push(read_unsignedbyte(buf));
        }

        let mut chunk_data = Cursor::new(data);
        let chunks = NetworkChunk::deserialize(&mut chunk_data, primary_bit_mask);

        let block_entities_len = read_varint(buf);
        let mut block_entities = Vec::new();

        for _ in 0..block_entities_len {
            block_entities.push(Blob::from_reader(buf).unwrap());
        }

        Box::new(ChunkDataPacket {
            x,
            z,
            full_chunk,
            primary_bit_mask,
            heightmaps,
            biomes,
            data: chunks,
            block_entities,
        })
    }
}
