use crate::protocol::data::read_types::{read_bool, read_int, read_varint};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use crate::protocol::types::chunk::NetworkChunk;
use nbt::Blob;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct ChunkDataPacket {
    pub x: i32,
    pub z: i32,
    // True = New chunk, False = Existing chunk
    pub new: bool,
    pub primary_bit_mask: i32,
    pub heightmaps: Blob,
    pub biomes: Option<Vec<i32>>,
    pub data: Vec<NetworkChunk>,
    pub block_entities: Vec<Blob>,
}

impl ClientBoundPacketType for ChunkDataPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let x = read_int(buf);
        let z = read_int(buf);

        let new = read_bool(buf);
        let primary_bit_mask = read_varint(buf);

        let heightmaps = Blob::from_reader(buf).unwrap();
        let biomes = if new {
            let mut arr = Vec::new();
            for _ in 0..1024 {
                arr.push(read_int(buf));
            }
            Some(arr)
        } else {
            None
        };

        let data_len = read_varint(buf);
        let mut data = vec![0u8; data_len as usize];

        buf.read_exact(&mut data).unwrap();

        let mut chunk_data = Cursor::new(data);
        let chunks = NetworkChunk::deserialize(&mut chunk_data, primary_bit_mask, false); //x == -4 && z == -3);

        let block_entities_len = read_varint(buf);
        let mut block_entities = Vec::new();

        for _ in 0..block_entities_len {
            block_entities.push(Blob::from_reader(buf).unwrap());
        }

        Box::new(ChunkDataPacket {
            x,
            z,
            new,
            primary_bit_mask,
            heightmaps,
            biomes,
            data: chunks,
            block_entities,
        })
    }
}
