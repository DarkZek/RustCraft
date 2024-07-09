use crate::block::deserialisation::BlockStatesFile;
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};

#[derive(Default)]
pub struct BlockStateAssetLoader;

impl AssetLoader for BlockStateAssetLoader {
    type Asset = BlockStatesFile;
    type Settings = ();
    type Error = serde_json::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, serde_json::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await.unwrap();

        let states = match serde_json::from_slice(&bytes) {
            Ok(val) => val,
            Err(e) => panic!("Invalid block states json {:?}", e), // TODO: Handle this better
        };

        Ok(states)
    }

    fn extensions(&self) -> &[&str] {
        &["blocks"]
    }
}
