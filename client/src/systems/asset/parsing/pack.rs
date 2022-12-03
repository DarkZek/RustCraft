use crate::systems::asset::atlas::ResourcePackData;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use fnv::{FnvBuildHasher, FnvHashMap};
use image::{DynamicImage, ImageFormat};

use std::collections::HashMap;

use bevy::prelude::error;
use std::io::{Cursor, Read};

use zip::ZipArchive;

#[derive(Default)]
pub struct ResourcePackAssetLoader;

impl AssetLoader for ResourcePackAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut archive = ZipArchive::new(Cursor::new(bytes))?;

            let data = load_resources(&mut archive);

            load_context.set_default_asset(LoadedAsset::new(ResourcePackData::new(data)));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["pack"]
    }
}

fn load_resources(
    archive: &mut ZipArchive<Cursor<&[u8]>>,
) -> HashMap<String, DynamicImage, FnvBuildHasher> {
    let mut out = FnvHashMap::default();

    for i in 0..archive.len() {
        let mut item = archive.by_index(i).unwrap();

        if item.is_file() && item.name().ends_with(".png") {
            let mut data: Vec<u8> = Vec::new();
            if let Err(e) = item.read_to_end(&mut data) {
                error!("Error reading resource {} - {}", item.name(), e);
                continue;
            }

            match image::load_from_memory_with_format(data.as_slice(), ImageFormat::Png) {
                Ok(img) => {
                    out.insert(item.name().to_string(), img);
                }
                Err(e) => {
                    error!("Error reading resource {} - {}", item.name(), e);
                }
            };
        }
    }

    out
}
