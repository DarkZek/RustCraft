use crate::protocol::data::read_types::{
    read_bool, read_float, read_int, read_long, read_string, read_varint,
};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use crate::protocol::types::slot::Slot;
use std::io::Cursor;

#[derive(Debug)]
pub struct AdvancementsPacket {
    pub clear: bool,
    pub mappings: Vec<(String, Advancement)>,
    pub identifiers: Vec<String>,
    pub progress: Vec<(String, AdvancementProgress)>,
}

impl ClientBoundPacketType for AdvancementsPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let clear = read_bool(buf);
        let mappings_len = read_varint(buf);
        let mut mappings = Vec::new();

        for _ in 0..mappings_len {
            let key = read_string(buf);
            let parent = if read_bool(buf) {
                Some(read_string(buf))
            } else {
                None
            };
            let display_data = if read_bool(buf) {
                Some(read_advancement_display(buf))
            } else {
                None
            };

            let criteria_len = read_varint(buf);
            let mut criteria = Vec::new();
            for _ in 0..criteria_len {
                criteria.push(read_string(buf));
            }

            let requirements_len = read_varint(buf);
            let mut requirements = Vec::new();
            for _ in 0..requirements_len {
                let subrequirements_len = read_varint(buf);
                let mut subrequirements = Vec::new();
                for _ in 0..subrequirements_len {
                    subrequirements.push(read_string(buf));
                }
                requirements.push(subrequirements);
            }

            mappings.push((
                key,
                Advancement {
                    parent,
                    display_data,
                    criteria,
                    requirements,
                },
            ))
        }

        let identifiers_len = read_varint(buf);
        let mut identifiers = Vec::new();
        for _ in 0..identifiers_len {
            identifiers.push(read_string(buf));
        }

        let progress_len = read_varint(buf);
        let mut progress = Vec::new();
        for _ in 0..progress_len {
            let key = read_string(buf);

            let size = read_varint(buf);
            let mut data = Vec::new();
            for _ in 0..size {
                let id = read_string(buf);
                let achieved = read_bool(buf);
                let date_achieved = if achieved { Some(read_long(buf)) } else { None };

                data.push((id, (achieved, date_achieved)));
            }

            progress.push((key, AdvancementProgress { data: vec![] }));
        }

        Box::new(AdvancementsPacket {
            clear,
            mappings,
            identifiers,
            progress,
        })
    }
}

fn read_advancement_display(buf: &mut Cursor<Vec<u8>>) -> AdvancementDisplay {
    let title = read_string(buf);
    let description = read_string(buf);
    let icon = Slot::deserialize(buf);
    let frame_type = read_varint(buf);
    let flags = read_int(buf);
    let background_texture = if flags & 0b1 == 1 {
        Some(read_string(buf))
    } else {
        None
    };
    let x = read_float(buf);
    let y = read_float(buf);
    AdvancementDisplay {
        title,
        description,
        icon,
        frame_type,
        flags,
        background_texture,
        x,
        y,
    }
}

#[derive(Debug)]
pub struct Advancement {
    parent: Option<String>,
    display_data: Option<AdvancementDisplay>,
    criteria: Vec<String>,
    requirements: Vec<Vec<String>>,
}

#[derive(Debug)]
pub struct AdvancementDisplay {
    title: String,
    description: String,
    icon: Slot,
    frame_type: i32,
    flags: i32,
    background_texture: Option<String>,
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub struct AdvancementProgress {
    data: Vec<(String, (bool, Option<i64>))>,
}
