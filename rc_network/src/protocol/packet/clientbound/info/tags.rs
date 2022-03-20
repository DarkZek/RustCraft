use crate::protocol::data::read_types::{read_string, read_varint};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use std::io::Cursor;

#[derive(Debug)]
pub struct TagsPacket {
    pub block_tags: Vec<Tag>,
    pub item_tags: Vec<Tag>,
    pub fluid_tags: Vec<Tag>,
    pub entity_tags: Vec<Tag>,
}

impl ClientBoundPacketType for TagsPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let mut block_tags = Vec::new();
        let mut item_tags = Vec::new();
        let mut fluid_tags = Vec::new();
        let mut entity_tags = Vec::new();

        // Blocks, Items, Fluids, Entities
        for i in 0..4 {
            let tags_len = read_varint(buf);
            let mut tags = Vec::new();

            for _ in 0..tags_len {
                let name = read_string(buf);
                let count = read_varint(buf);
                let mut entries = Vec::new();

                for _ in 0..count {
                    entries.push(read_varint(buf));
                }

                tags.push(Tag {
                    name,
                    count,
                    entries,
                })
            }

            match i {
                0 => block_tags = (*tags).to_vec(),
                1 => item_tags = (*tags).to_vec(),
                2 => fluid_tags = (*tags).to_vec(),
                3 => entity_tags = (*tags).to_vec(),
                _ => {}
            }
        }

        Box::new(TagsPacket {
            block_tags,
            item_tags,
            fluid_tags,
            entity_tags,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    name: String,
    count: i32,
    entries: Vec<i32>,
}
