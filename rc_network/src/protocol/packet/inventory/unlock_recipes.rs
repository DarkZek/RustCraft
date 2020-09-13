use crate::protocol::packet::PacketType;
use crate::protocol::data::read_types::{read_unsignedbyte, read_bool, read_int, read_float, read_double, read_varint, read_string};
use std::io::{Cursor};

#[derive(Debug)]
pub struct UnlockRecipesPacket {
    pub action: i64,
    pub crafting_recipe_book_open: bool,
    pub crafting_recipe_book_filter_active: bool,
    pub smelting_recipe_book_open: bool,
    pub smelting_recipe_book_filter_active: bool,
    pub recipe_ids: Vec<String>,
    pub recipe_ids_init: Option<Vec<String>>,
}

impl PacketType for UnlockRecipesPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let action = read_varint(buf);
        let crafting_recipe_book_open = read_bool(buf);
        let crafting_recipe_book_filter_active = read_bool(buf);
        let smelting_recipe_book_open = read_bool(buf);
        let smelting_recipe_book_filter_active = read_bool(buf);
        let mut recipe_ids = Vec::new();
        let mut recipe_ids_init = Vec::new();

        let len = read_varint(buf);
        for _ in 0..len {
            recipe_ids.push(read_string(buf));
        }

        let len = read_varint(buf);
        for _ in 0..len {
            recipe_ids_init.push(read_string(buf));
        }

        Box::new(UnlockRecipesPacket {
            action,
            crafting_recipe_book_open,
            crafting_recipe_book_filter_active,
            smelting_recipe_book_open,
            smelting_recipe_book_filter_active,
            recipe_ids,
            recipe_ids_init: Some(recipe_ids_init)
        })
    }
}
