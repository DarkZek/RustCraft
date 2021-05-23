use serde_json::Value;

// Manages all blocks
pub struct BlockManager {}

impl BlockManager {
    pub fn new() -> BlockManager {
        let data = get_data();

        BlockManager {}
    }
}

fn get_data() -> Value {
    let data = include_str!("../../../../../RustCraft/assets/blocks.json");
    serde_json::from_str(data).unwrap()
}
