use crate::item::deserialisation::ItemStatesFile;
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};

#[derive(Default)]
pub struct ItemStateAssetLoader;

impl AssetLoader for ItemStateAssetLoader {
    type Asset = ItemStatesFile;
    type Settings = ();
    type Error = serde_json::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, serde_json::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await.unwrap();

            let states = match serde_json::from_slice(&bytes) {
                Ok(val) => val,
                Err(e) => panic!("Invalid type states json {:?}", e), // TODO: Handle this better
            };

            Ok(states)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["items"]
    }
}
