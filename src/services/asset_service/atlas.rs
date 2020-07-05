use crate::block::Block;
use crate::services::asset_service::{AssetService, ResourcePack};
use crate::services::settings_service::SettingsService;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::time::SystemTime;
use wgpu::{Device, Queue, Sampler, Texture, BufferUsage, CompareFunction, TextureDataLayout};

pub type TextureAtlasIndex = ([f32; 2], [f32; 2]);

pub const ATLAS_WIDTH: u32 = 4096;
pub const ATLAS_HEIGHT: u32 = (4096.0 * 2.0) as u32;

impl AssetService {
    /// Generate a a new texture atlas from a list of textures and a resources directory
    pub fn generate_texture_atlas(
        resource_pack: &mut ResourcePack,
        device: &Device,
        queue: &mut Queue,
        settings: &SettingsService,
    ) -> (
        DynamicImage,
        Texture,
        HashMap<String, TextureAtlasIndex>,
        Sampler,
    ) {
        let textures = sort_textures(&mut resource_pack.textures);

        let start_time = SystemTime::now();

        //Create buffer
        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: ATLAS_WIDTH,
                height: ATLAS_HEIGHT,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let mut atlas_index: HashMap<String, TextureAtlasIndex> = HashMap::new();
        let mut atlas_img = None;

        if settings.atlas_cache_reading {
            match load_cached_atlas(&settings) {
                Ok((img, index)) => {
                    atlas_img = Some(img);
                    atlas_index = index;
                    log!(
                        "Loading cached texture atlas took: {}ms",
                        start_time.elapsed().unwrap().as_millis()
                    );
                }
                Err(e) => log_error!("Error loading cached atlas info {}", e),
            }
        }

        // If reading cache didnt work then remake it
        if atlas_img.is_none() {
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
                        let (name, img) =
                            textures.get(relative_texture_index + texture_id).unwrap();
                        let width = img.width();

                        if (row_width + width) <= ATLAS_WIDTH {
                            texture_numbers_x.push(row_width + width - 1);

                            // Generate a list of locations that our textures exist inside of the main atlas texture. These are in the form 1/(X POS) because this is how it's expected in the shaders.
                            atlas_index.insert(
                                name.clone(),
                                (
                                    [
                                        (row_width as f32) / ATLAS_WIDTH as f32,
                                        ((current_y + row_height - img.height()) as f32)
                                            / ATLAS_HEIGHT as f32,
                                    ],
                                    [
                                        ((row_width + width) as f32) / ATLAS_WIDTH as f32,
                                        ((current_y + row_height) as f32) / ATLAS_HEIGHT as f32,
                                    ],
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
                        log_error!("Atlas too small! Not all textures could fit in");
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
                        let tex_x = x
                            - (texture_numbers_x.get(current_texture_id).unwrap()
                                - (image.width() - 1));

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

            if settings.debug_atlas {
                for (_, coord) in atlas_index.iter() {
                    let x = (coord.0[0] * ATLAS_WIDTH as f32) as u32;
                    let y = (coord.0[1] * ATLAS_HEIGHT as f32) as u32;

                    atlas.put_pixel(x, y, image::Rgba([255, 255, 0, 255]));
                    atlas.put_pixel(x, y + 1, image::Rgba([255, 255, 0, 255]));
                    atlas.put_pixel(x + 1, y, image::Rgba([255, 255, 0, 255]));

                    let x = (coord.1[0] * ATLAS_WIDTH as f32) as u32;
                    let y = (coord.1[1] * ATLAS_HEIGHT as f32) as u32;

                    if atlas.dimensions().0 == x {
                        continue;
                    }

                    atlas.put_pixel(x, y, image::Rgba([255, 255, 0, 255]));
                    atlas.put_pixel(x, y - 1, image::Rgba([255, 255, 0, 255]));
                    atlas.put_pixel(x - 1, y, image::Rgba([255, 255, 0, 255]));
                }
            }

            if settings.atlas_cache_writing {
                if let Err(e) = atlas.save(format!("{}resources/atlas.png", settings.path)) {
                    log_error!("Failed to cache atlas image: {}", e);
                }

                let result = serde_json::to_string(&atlas_index).unwrap();

                match File::create(format!("{}resources/atlas_index.json", settings.path)) {
                    Ok(mut atlas_index_file) => {
                        if let Err(e) = atlas_index_file.write_all(result.as_bytes()) {
                            log_error!("Error writing texture atlas index: {}", e);
                        }
                    }
                    Err(e) => {
                        log_error!("Failed to cache atlas index: {}", e);
                    }
                }
            }

            atlas_img = Some(DynamicImage::ImageRgba8(atlas));
            log!(
                "Generating texture atlas took: {}ms",
                start_time.elapsed().unwrap().as_millis()
            );
        }

        let atlas_img = atlas_img.unwrap();
        let diffuse_rgba = atlas_img.as_rgba8().unwrap();
        let dimensions = diffuse_rgba.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };

        let diffuse_buffer =
            device.create_buffer_with_data(&diffuse_rgba, BufferUsage::COPY_SRC);

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Add it to buffer
        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &diffuse_buffer,
                layout: TextureDataLayout {
                    offset: 0,
                    bytes_per_row: 4 * size.width,
                    rows_per_image: size.height,
                }
            },
            wgpu::TextureCopyView {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );

        queue.submit(Some(encoder.finish()));

        let diffuse_sampler_descriptor = wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: Some(CompareFunction::Always),
            anisotropy_clamp: None,
            _non_exhaustive: Default::default()
        };

        let diffuse_sampler = device.create_sampler(&diffuse_sampler_descriptor);

        (atlas_img, diffuse_texture, atlas_index, diffuse_sampler)
    }
}

#[allow(dead_code)]
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

fn sort_textures(textures: &mut HashMap<String, DynamicImage>) -> Vec<(String, DynamicImage)> {
    // Create a new array we can sort by
    let mut buckets = HashMap::new();
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

        //TODO: Remove once we have array of texture atlases up and running
        if size > &512 {
            continue;
        }

        for texture_name in bucket {
            let texture = textures.remove(texture_name).unwrap();

            out.push((texture_name.clone(), texture));
        }
    }
    out
}

pub fn load_cached_atlas(
    settings: &SettingsService,
) -> Result<(DynamicImage, HashMap<String, TextureAtlasIndex>), Box<dyn std::error::Error>> {
    let img = image::open(format!("{}resources/atlas.png", settings.path))?;

    let mut index_file = File::open(format!("{}resources/atlas_index.json", settings.path))?;
    let mut data = Vec::new();
    index_file.read_to_end(&mut data)?;

    let index = serde_json::from_slice::<HashMap<String, TextureAtlasIndex>>(data.as_slice())?;

    Ok((img, index))
}

pub fn atlas_update_blocks(mapping: &HashMap<String, TextureAtlasIndex>, blocks: &mut Vec<Block>) {
    for mut block in blocks.iter_mut() {
        for (i, tex) in block.raw_texture_names.iter().enumerate() {
            match mapping.get(*tex) {
                Some(map) => {
                    block.texture_atlas_lookups[i] = map.clone();
                }
                None => {
                    log_error!("No mapping found for {}", tex);
                }
            }
        }
    }
}
