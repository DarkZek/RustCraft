use bevy::prelude::*;
use rc_client::game::blocks::deserialisation::BlockStatesFile;
use rc_client::game::blocks::loader::BlockStateAssetLoader;
use rc_client::game::blocks::states::BlockStates;

pub struct BlockStatesPlugin;

impl Plugin for BlockStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BlockStatesFile>()
            .init_asset_loader::<BlockStateAssetLoader>()
            .add_systems(Startup, create_block_states)
            .insert_resource(BlockStates::new());
    }
}

pub fn create_block_states(server: Res<AssetServer>, mut states: ResMut<BlockStates>) {
    states.asset = Some(server.load("game/state.blocks"));
}
