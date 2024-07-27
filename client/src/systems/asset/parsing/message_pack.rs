use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext};
use std::marker::PhantomData;

#[derive(Default)]
pub struct MessagePackAssetLoader<T> {
    _marker: PhantomData<T>,
}

impl<T> AssetLoader for MessagePackAssetLoader<T>
where
    for<'a> T: serde::Deserialize<'a> + Asset,
{
    type Asset = T;
    type Settings = ();
    type Error = rmp_serde::decode::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<T, rmp_serde::decode::Error> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data).await.unwrap();
        let custom_asset: T = rmp_serde::from_slice::<T>(&data)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["mpk"]
    }
}
