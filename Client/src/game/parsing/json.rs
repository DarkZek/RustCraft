use bevy::asset::{Asset, AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use std::marker::PhantomData;

#[derive(Default)]
pub struct JsonAssetLoader<T> {
    _marker: PhantomData<T>,
}

impl<T> AssetLoader for JsonAssetLoader<T>
where
    for<'a> T: serde::Deserialize<'a> + Asset,
{
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset: T = serde_json::from_slice::<T>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
