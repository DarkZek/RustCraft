use crate::protocol::data::read_types::{read_float, read_string, read_varint};
use crate::protocol::packet::clientbound::ClientBoundPacketType;
use crate::protocol::types::ingredient::Ingredient;
use crate::protocol::types::slot::Slot;
use std::io::Cursor;

#[derive(Debug)]
pub struct DeclareRecipesPacket {
    pub recipes: Vec<CraftingRecipe>,
}

impl ClientBoundPacketType for DeclareRecipesPacket {
    fn deserialize(buf: &mut Cursor<Vec<u8>>) -> Box<Self> {
        let num_recipes = read_varint(buf);
        let mut recipes = Vec::new();

        for _ in 0..num_recipes {
            let identifier = read_string(buf);
            let id = read_string(buf);

            let data = match identifier.as_str() {
                "minecraft:crafting_shapeless" => {
                    let group = read_string(buf);
                    let ingredient_count = read_varint(buf);
                    let mut ingredients = Vec::new();

                    for _ in 0..ingredient_count {
                        ingredients.push(Ingredient::deserialize(buf));
                    }

                    let result = Slot::deserialize(buf);

                    Some(CraftingRecipeData::CraftingShapeless(
                        group,
                        ingredients,
                        result,
                    ))
                }
                "minecraft:crafting_shaped" => {
                    let width = read_varint(buf);
                    let height = read_varint(buf);
                    let group = read_string(buf);

                    let mut ingredients = Vec::new();

                    for _ in 0..(width * height) {
                        ingredients.push(Ingredient::deserialize(buf));
                    }

                    let result = Slot::deserialize(buf);

                    Some(CraftingRecipeData::CraftingShaped(
                        width,
                        height,
                        group,
                        ingredients,
                        result,
                    ))
                }
                "minecraft:smelting"
                | "minecraft:blasting"
                | "minecraft:smoking"
                | "minecraft:campfire_cooking" => {
                    let group = read_string(buf);
                    let ingredient = Ingredient::deserialize(buf);
                    let result = Slot::deserialize(buf);
                    let experience = read_float(buf);
                    let cooking_time = read_varint(buf);

                    Some(CraftingRecipeData::SmeltingBlastingSmokingCampfireCooking(
                        group,
                        ingredient,
                        result,
                        experience,
                        cooking_time,
                    ))
                }
                "minecraft:stonecutting" => {
                    let group = read_string(buf);
                    let ingredient = Ingredient::deserialize(buf);
                    let result = Slot::deserialize(buf);

                    Some(CraftingRecipeData::Stonecutting(group, ingredient, result))
                }
                _ => None,
            };

            recipes.push(CraftingRecipe {
                identifier,
                id,
                data,
            })
        }

        Box::new(DeclareRecipesPacket { recipes })
    }
}

#[derive(Debug)]
pub struct CraftingRecipe {
    identifier: String,
    id: String,
    data: Option<CraftingRecipeData>,
}

// https://wiki.vg/Protocol#Server_Difficulty
#[derive(Debug)]
pub enum CraftingRecipeData {
    CraftingShapeless(String, Vec<Ingredient>, Slot),
    CraftingShaped(i32, i32, String, Vec<Ingredient>, Slot),
    SmeltingBlastingSmokingCampfireCooking(String, Ingredient, Slot, f32, i32),
    Stonecutting(String, Ingredient, Slot),
}
