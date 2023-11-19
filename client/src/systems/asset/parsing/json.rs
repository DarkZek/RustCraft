use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use std::marker::PhantomData;

#[derive(Default)]
pub struct JsonAssetLoader<T> {
    _marker: PhantomData<T>,
}

impl<T> AssetLoader for JsonAssetLoader<T>
where
    for<'a> T: serde::Deserialize<'a> + Asset,
{
    type Asset = T;
    type Settings = ();
    type Error = serde_json::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<T, serde_json::Error>> {
        Box::pin(async move {
            let mut data = Vec::new();
            reader.read_to_end(&mut data).await.unwrap();
            let custom_asset: T = serde_json::from_slice::<T>(&data)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
