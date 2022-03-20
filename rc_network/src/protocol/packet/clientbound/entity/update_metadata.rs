use crate::protocol::data::read_types::read_varint;
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use crate::protocol::types::entity_metadata::EntityMetadata;
use std::io::Cursor;

#[derive(Debug)]
pub struct EntityUpdateMetadataPacket {
    pub entity_id: i32,
    pub entity_metadata: EntityMetadata,
}

impl ClientBoundPacketType for EntityUpdateMetadataPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let entity_id = read_varint(buf);
        let entity_metadata = EntityMetadata::deserialize(buf);

        Box::new(EntityUpdateMetadataPacket {
            entity_id,
            entity_metadata,
        })
    }
}
