use crate::systems::asset::atlas::ResourcePackData;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use fnv::{FnvBuildHasher, FnvHashMap};
use image::{DynamicImage, ImageFormat};

use std::collections::HashMap;

use bevy::asset::io::Reader;
use bevy::log::error;
use std::io::{Cursor, Read};

use zip::ZipArchive;

#[derive(Default)]
pub struct ResourcePackAssetLoader;

impl AssetLoader for ResourcePackAssetLoader {
    type Asset = ResourcePackData;
    type Settings = ();
    type Error = serde_json::Error; // TODO: Come and fix these error types

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<ResourcePackData, serde_json::Error>> {
        Box::pin(async move {
            let mut data = Vec::new();
            reader.read_to_end(&mut data).await.unwrap();
            let mut archive = ZipArchive::new(Cursor::new(data.as_slice())).unwrap();

            let data = load_resources(&mut archive);

            Ok(ResourcePackData::new(data))
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
