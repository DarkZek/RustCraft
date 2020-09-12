use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_bool, read_int};
use std::io::{Cursor, Seek, Read};
use crate::protocol::types::entity_metadata::EntityMetadata;

#[derive(Debug)]
pub struct EntityUpdateMetadataPacket {
    pub entity_id: i32,
    pub entity_metadata: EntityMetadata
}

impl PacketType for EntityUpdateMetadataPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_int(buf);
        let entity_metadata = EntityMetadata::deserialize(buf);

        Box::new(EntityUpdateMetadataPacket {
            entity_id,
            entity_metadata
        })
    }
}
