use crate::systems::asset::atlas::resource_packs::ResourcePack;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use fnv::{FnvBuildHasher, FnvHashMap};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rc_shared::atlas::{TextureAtlas, TextureAtlasIndex, TextureAtlasTrait};

use std::collections::HashMap;

use std::sync::{OnceLock, RwLock, RwLockReadGuard};
use bevy::prelude::{Assets, error, Image, ResMut};

pub const ATLAS_WIDTH: u32 = 4096 / 4;
pub const ATLAS_HEIGHT: u32 = 4096 / 4;

    /// Generate a a new texture atlas from a list of textures and a resources directory
pub fn new_atlas(
    _resource_pack: &ResourcePack,
    textures: &mut HashMap<String, DynamicImage, FnvBuildHasher>,
    assets: &mut ResMut<Assets<Image>>,
) -> TextureAtlas {
    let mut atlas_index: HashMap<String, TextureAtlasIndex, FnvBuildHasher> =
        FnvHashMap::default();
    let mut atlas_img = None;

    // If reading cache didnt work then remake it
    if atlas_img.is_none() {
        let mut textures = sort_textures(textures);

        // Add error texture
        textures.push((
            String::from("game/error"),
            DynamicImage::ImageRgba8(gen_invalid_texture()),
        ));

        let atlas = generate_atlas(textures, &mut atlas_index);

        // if settings.atlas_cache_writing {
        //     write_cached_atlas(
        //         &path,
        //         &atlas_path,
        //         &atlas_index_path,
        //         &atlas_info_path,
        //         &atlas_index,
        //         zip_name,
        //         &atlas,
        //         resource_pack,
        //     );
        // }

        atlas_img = Some(DynamicImage::ImageRgba8(atlas));
    }

    let atlas_img = atlas_img.unwrap();

    let image = Image::new(
        Extent3d {
            width: atlas_img.width(),
            height: atlas_img.height(),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        atlas_img.into_bytes(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all()
    );

    let image = assets.add(image);

    TextureAtlas {
        image,
        index: atlas_index,
    }
}

fn generate_atlas(
    textures: Vec<(String, DynamicImage)>,
    atlas_index: &mut HashMap<String, TextureAtlasIndex, FnvBuildHasher>,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut atlas: ImageBuffer<Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::new(ATLAS_WIDTH, ATLAS_HEIGHT);

    // Stores the ID of the lowest texture id on this row
    let mut texture_id = 0;

    let mut current_texture_id = 0;

    // Stores the x index that the textures start at
    let mut texture_numbers_x = Vec::new();

    // Stores the working Y
    let mut current_y = 0;

    for (x, y, pixel) in atlas.enumerate_pixels_mut() {
        // Generate the row info
        if current_y <= y {
            texture_id += texture_numbers_x.len();
            texture_numbers_x.clear();

            // We're done!
            if textures.len() <= texture_id {
                break;
            }

            // Stores the filled space of this atlas row
            let mut row_width = 0;
            let row_height = textures.get(texture_id).unwrap().1.height();

            // Stores the texture relative we're looking at compared to the texture_id
            let mut relative_texture_index = 0;

            while textures.len() > (relative_texture_index + texture_id) {
                // Add to row if theres space
                let (name, img) = textures.get(relative_texture_index + texture_id).unwrap();
                let width = img.width();

                if (row_width + width) <= ATLAS_WIDTH {
                    texture_numbers_x.push(row_width + width - 1);

                    // Generate a list of locations that our textures exist inside of the site atlas texture. These are in the form 1/(X POS) because this is how it's expected in the shaders.
                    atlas_index.insert(
                        name.split('.').next().unwrap().to_string(),
                        TextureAtlasIndex::new(
                            (row_width as f32) / ATLAS_WIDTH as f32,
                            ((row_width + width) as f32) / ATLAS_WIDTH as f32,
                            ((current_y + row_height - img.height()) as f32) / ATLAS_HEIGHT as f32,
                            ((current_y + row_height) as f32) / ATLAS_HEIGHT as f32,
                        ),
                    );
                } else {
                    break;
                }

                row_width += width;
                relative_texture_index += 1;
            }

            // Update y
            current_y += row_height;

            if current_y > ATLAS_HEIGHT {
                error!("Atlas too small! Not all textures could fit in");
                break;
            }
        }

        // Reset current texture after x row
        if x == 0 {
            current_texture_id = 0;
        }

        // Check if there is any more textures to draw this row
        if texture_numbers_x.len() <= current_texture_id {
            *pixel = image::Rgba([0, 0, 0, 255]);
            continue;
        }

        // Check if we can more to drawing the next texture yet
        if texture_numbers_x.get(current_texture_id).unwrap() < &x {
            current_texture_id += 1;
        }

        // Check if there is any more textures this row
        if texture_numbers_x.len() <= current_texture_id {
            *pixel = image::Rgba([255, 0, 255, 255]);
            continue;
        }

        // Get the pixel
        match textures.get(texture_id + current_texture_id as usize) {
            Some((_, image)) => {
                let tex_x =
                    x - (texture_numbers_x.get(current_texture_id).unwrap() - (image.width() - 1));

                if current_y - image.height() > y {
                    *pixel = image::Rgba([255, 0, 0, 255]);
                } else {
                    let tex_y = image.height() + y - current_y;

                    if tex_y <= image.height() {
                        *pixel = image.get_pixel(tex_x, tex_y);
                    } else {
                        *pixel = image::Rgba([255, 0, 0, 255]);
                    }
                }
            }
            None => {
                *pixel = image::Rgba([255, 255, 0, 255]);
            }
        }
    }

    atlas
}

fn gen_invalid_texture() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut buffer = ImageBuffer::new(2, 2);
    for (x, y, p) in buffer.enumerate_pixels_mut() {
        *p = invalid_texture(x, y, 2);
    }
    buffer
}

fn invalid_texture(x: u32, y: u32, texture_size: u32) -> Rgba<u8> {
    let relative_x = ((x as f32 + 1.0) / (texture_size as f32 / 2.0)).ceil();
    let relative_y = ((y as f32 + 1.0) / (texture_size as f32 / 2.0)).ceil();
    let purple = (relative_x + relative_y) % 2.0 == 0.0;
    if purple {
        image::Rgba([255, 0, 255, 255])
    } else {
        image::Rgba([0, 0, 0, 255])
    }
}

fn sort_textures(
    textures: &mut HashMap<String, DynamicImage, FnvBuildHasher>,
) -> Vec<(String, DynamicImage)> {
    // Create a new array we can sort by
    let mut buckets = FnvHashMap::default();
    let mut out = Vec::new();

    for (name, texture) in textures.into_iter() {
        if !buckets.contains_key(&texture.height()) {
            // Add new bucket
            buckets.insert(texture.height(), vec![name.clone()]);
        } else {
            // Add to existing bucket
            buckets
                .get_mut(&texture.height())
                .unwrap()
                .push(name.clone());
        }
    }

    let mut ordered: Vec<&u32> = buckets.keys().collect();
    ordered.sort();
    ordered.reverse();

    for size in ordered {
        let bucket = buckets.get(size).unwrap();

        for texture_name in bucket {
            let texture = textures.remove(texture_name).unwrap();

            out.push((texture_name.clone(), texture));
        }
    }
    out
}
