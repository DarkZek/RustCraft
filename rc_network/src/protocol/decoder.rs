/*
   This packet decoder is designed to be used only when the connection is in Play mode,
   and should be able to accept any incoming connections and store its information.
*/

use crate::protocol::packet::Packet;
use crate::protocol::read_types::{
    read_bool, read_bytearray, read_float, read_int, read_intarray, read_long, read_string,
    read_unsignedbyte, read_varint, read_varintarray,
};
use crate::protocol::types::chunk::NetworkChunk;
use crate::protocol::types::{PVarType, PVarTypeTemplate};
use crate::stream::NetworkStream;
use nbt::Blob;

pub struct PacketDecoder;

impl PacketDecoder {
    pub fn decode(stream: &mut NetworkStream) -> Packet {
        // Get length of packet
        let len = read_varint(stream);

        stream.reset_byte_counter();

        // Get id of packet
        let packet_id = read_varint(stream);

        let packet_definition = match get_packet_definition(packet_id) {
            Some(value) => value,
            None => {
                //println!("Packet ID: 0x{:x} not implemented, ignoring", packet_id);
                PacketDefinition::blank()
            }
        };

        let mut data = Vec::new();

        //println!("{} Bytes - {}", len, packet_definition.name);

        for (i, (name, var_type)) in packet_definition.data.iter().enumerate() {
            // Check if this one has any conditions
            for condition in packet_definition.conditions {
                if i == condition.0 as usize {
                    // This is a conditional, get value it refers to
                    let val: &PVarType = data.get(condition.1 as usize).unwrap();

                    let valid = match val {
                        PVarType::Boolean(val) => val.clone(),
                        _ => false,
                    };

                    if !valid {
                        continue;
                    }
                }
            }

            match var_type {
                PVarTypeTemplate::UnsignedByte => {
                    let result = read_unsignedbyte(stream);
                    data.push(PVarType::UnsignedByte(result));
                }
                PVarTypeTemplate::VarInt => {
                    let result = read_varint(stream);
                    data.push(PVarType::VarInt(result));
                }
                PVarTypeTemplate::String => {
                    let result = read_string(stream);
                    data.push(PVarType::String(result.clone()));
                }
                PVarTypeTemplate::VarIntByteArray => {
                    let result = read_varintarray(stream);
                    data.push(PVarType::VarIntByteArray(result.clone()));
                }
                PVarTypeTemplate::Int => {
                    let result = read_int(stream);
                    data.push(PVarType::Int(result));
                }
                PVarTypeTemplate::Long => {
                    let result = read_long(stream);
                    data.push(PVarType::Long(result));
                }
                PVarTypeTemplate::Boolean => {
                    let result = read_bool(stream);
                    data.push(PVarType::Boolean(result));
                }
                PVarTypeTemplate::Float => {
                    let result = read_float(stream);
                    data.push(PVarType::Float(result));
                }
                PVarTypeTemplate::ByteArray => {
                    let remaining_len = len - stream.get_bytes_read() as i64;
                    let result = read_bytearray(stream, remaining_len as u16);
                    data.push(PVarType::ByteArray(result.clone()));
                }
                PVarTypeTemplate::NBT => {
                    data.push(PVarType::NBT(Blob::from_reader(stream).unwrap()));
                }
                PVarTypeTemplate::IntArray(len) => {
                    data.push(PVarType::IntArray(read_intarray(
                        stream,
                        len.clone() as usize,
                    )));
                }
                PVarTypeTemplate::NBTArray => {}
                PVarTypeTemplate::ChunkData(chunks_bitmask) => {
                    let chunk_len = match data.get(chunks_bitmask.clone()).unwrap() {
                        PVarType::VarInt(val) => val,
                        _ => panic!(),
                    };

                    data.push(PVarType::ChunkData(NetworkChunk::load_arr(
                        stream,
                        chunk_len.clone(),
                    )));
                }
            }
        }

        if len - stream.get_bytes_read() as i64 > 0 {
            println!("Remaining Length: {}", len - stream.get_bytes_read() as i64);
        }

        while len - stream.get_bytes_read() as i64 > 0 {
            read_unsignedbyte(stream);
        }

        Packet {
            id: packet_id as u16,
            len: len as u32,
            tokens: data,
        }
    }
}

fn get_packet_definition(id: i64) -> Option<PacketDefinition> {
    for packet in &PACKET_DEFINITIONS {
        if packet.id == id {
            return Some(*packet);
        }
    }

    None
}

const PACKET_DEFINITIONS: [PacketDefinition; 7] = [
    PacketDefinition {
        name: "Join Game",
        id: 0x26,
        data: &[
            ("Entity ID", PVarTypeTemplate::Int),
            ("Gamemode", PVarTypeTemplate::UnsignedByte),
            ("Dimension", PVarTypeTemplate::Int),
            ("Hashed Seed", PVarTypeTemplate::Long),
            ("Max Players", PVarTypeTemplate::UnsignedByte),
            ("Level Type", PVarTypeTemplate::String),
            ("View Distance", PVarTypeTemplate::VarInt),
            ("Reduced Debug Info", PVarTypeTemplate::Boolean),
            ("Enable Respawn Screen", PVarTypeTemplate::Boolean),
        ],
        conditions: &[],
    },
    PacketDefinition {
        name: "Plugin Message",
        id: 0x19,
        data: &[
            // Bit mask. 0x08: damage disabled (god mode), 0x04: can fly, 0x02: is flying, 0x01: is Creative
            ("Channel", PVarTypeTemplate::String),
            ("Data", PVarTypeTemplate::ByteArray),
        ],
        conditions: &[],
    },
    PacketDefinition {
        name: "Server Difficulty",
        id: 0xe,
        data: &[
            // Bit mask. 0x08: damage disabled (god mode), 0x04: can fly, 0x02: is flying, 0x01: is Creative
            ("Difficulty", PVarTypeTemplate::UnsignedByte),
            ("Difficulty locked", PVarTypeTemplate::Boolean),
        ],
        conditions: &[],
    },
    PacketDefinition {
        name: "Player Abilities",
        id: 0x32,
        data: &[
            // Bit mask. 0x08: Creative Mode (god mode), 0x04: can fly, 0x02: is flying, 0x01: is Creative
            ("Flags", PVarTypeTemplate::UnsignedByte),
            ("Flying Speed", PVarTypeTemplate::Float),
            ("Field of View Modifier", PVarTypeTemplate::Float),
        ],
        conditions: &[],
    },
    PacketDefinition {
        name: "Held Item Change",
        id: 0x40,
        data: &[("Slot", PVarTypeTemplate::UnsignedByte)],
        conditions: &[],
    },
    PacketDefinition {
        name: "Chunk Data",
        id: 0x22,
        data: &[
            ("Chunk X", PVarTypeTemplate::Int),
            ("Chunk Z", PVarTypeTemplate::Int),
            ("Full chunk", PVarTypeTemplate::Boolean),
            ("Primary Bit Mask", PVarTypeTemplate::VarInt),
            ("Heightmaps", PVarTypeTemplate::NBT),
            ("Biomes", PVarTypeTemplate::IntArray(1024)),
            ("Data", PVarTypeTemplate::ChunkData(3)),
            ("Number of block entities", PVarTypeTemplate::VarInt),
            //("Block entities",              PVarType::NBTArray(String::new()))
        ],
        conditions: &[
            // Only read biomes if Full Chunk is true
            (5, 2),
        ],
    },
    PacketDefinition {
        name: "Keep Alive",
        id: 0x21,
        data: &[("Keep Alive ID", PVarTypeTemplate::Long)],
        conditions: &[],
    },
];

#[derive(Copy, Clone)]
pub struct PacketDefinition {
    name: &'static str,
    id: i64,
    data: &'static [(&'static str, PVarTypeTemplate)],
    conditions: &'static [(u32, u32)],
}

impl PacketDefinition {
    pub fn blank() -> PacketDefinition {
        PacketDefinition {
            name: "",
            id: 0,
            data: &[],
            conditions: &[],
        }
    }
}
