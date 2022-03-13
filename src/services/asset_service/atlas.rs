use crate::render::device::get_device;
use crate::services::asset_service::{AssetService, ResourcePack};
use crate::services::settings_service::SettingsService;
use core::num::NonZeroU32;
use fnv::{FnvBuildHasher, FnvHashMap};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rc_ui::atlas::TextureAtlasIndex;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{Read, Write};
use std::lazy::SyncOnceCell;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BufferUsages, ImageDataLayout, Queue, Sampler, Texture, TextureAspect, TextureFormat};

// TODO: Refactor

pub const ATLAS_WIDTH: u32 = 4096;
pub const ATLAS_HEIGHT: u32 = (4096.0 * 2.0) as u32;

// Create global atlas lookup variable
pub static ATLAS_LOOKUPS: SyncOnceCell<HashMap<String, TextureAtlasIndex, FnvBuildHasher>> =
    SyncOnceCell::new();

impl AssetService {
    /// Generate a a new texture atlas from a list of textures and a resources directory
    pub fn generate_texture_atlas(
        resource_pack: &mut ResourcePack,
        zip_name: &str,
        queue: &mut Queue,
        settings: &SettingsService,
    ) -> (
        DynamicImage,
        Texture,
        HashMap<String, TextureAtlasIndex, FnvBuildHasher>,
        Sampler,
    ) {
        let start_time = SystemTime::now();

        // Get paths
        let mut path = settings.path.clone();
        path.push("cache");

        let mut atlas_path = path.clone();
        atlas_path.push("atlas");
        atlas_path.set_extension("png");

        let mut atlas_index_path = path.clone();
        atlas_index_path.push("atlas_index");
        atlas_index_path.set_extension("json");

        let mut atlas_info_path = path.clone();
        atlas_info_path.push("atlas_info");

        //Create buffer
        let diffuse_texture = get_device().create_texture(&wgpu::TextureDescriptor {
            label: Some("Asset Service Texture Atlas Texture"),
            size: wgpu::Extent3d {
                width: ATLAS_WIDTH,
                height: ATLAS_HEIGHT,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        let mut atlas_index: HashMap<String, TextureAtlasIndex, FnvBuildHasher> =
            FnvHashMap::default();
        let mut atlas_img = None;

        if settings.atlas_cache_reading {
            // Check if they're the same resource pack
            if let Some((image, map)) = load_cached_atlas(
                &atlas_path,
                &atlas_info_path,
                &atlas_index_path,
                zip_name,
                resource_pack,
                settings,
            ) {
                atlas_index = map;
                atlas_img = Some(image);
                log!(
                    "Loading cached texture atlas took: {}ms",
                    start_time.elapsed().unwrap().as_millis()
                );
            }
        }

        // If reading cache didnt work then remake it
        if atlas_img.is_none() {
            let mut textures = sort_textures(&mut resource_pack.textures);

            // Add error texture
            textures.push((
                String::from("mcv3/error"),
                DynamicImage::ImageRgba8(gen_invalid_texture()),
            ));

            let atlas = generate_atlas(textures, &mut atlas_index);

            if settings.atlas_cache_writing {
                write_cached_atlas(
                    &path,
                    &atlas_path,
                    &atlas_index_path,
                    &atlas_info_path,
                    &atlas_index,
                    zip_name,
                    &atlas,
                    resource_pack,
                );
            }

            atlas_img = Some(DynamicImage::ImageRgba8(atlas));

            log!(
                "Generating texture atlas took: {}ms",
                start_time.elapsed().unwrap().as_millis()
            );
        }

        let diffuse_sampler = process_image(atlas_img.as_mut().unwrap(), &diffuse_texture, queue);

        ATLAS_LOOKUPS
            .set(atlas_index.clone())
            .expect("Atlas failed to setup");

        if settings.debug_atlas {
            let mut atlas_info_path = path.clone();
            atlas_info_path.push("atlas_states");

            if let Ok(mut file) = File::create(atlas_info_path) {
                file.write_all(format!("{:?}", atlas_index).as_bytes())
                    .unwrap();
            }
        }

        (
            atlas_img.unwrap(),
            diffuse_texture,
            atlas_index,
            diffuse_sampler,
        )
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

                    // Generate a list of locations that our textures exist inside of the src atlas texture. These are in the form 1/(X POS) because this is how it's expected in the shaders.
                    atlas_index.insert(
                        name.clone(),
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

fn process_image(
    atlas_img: &mut DynamicImage,
    diffuse_texture: &Texture,
    queue: &mut Queue,
) -> Sampler {
    let diffuse_rgba = atlas_img.as_rgba8().unwrap();
    let dimensions = diffuse_rgba.dimensions();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    let diffuse_buffer = get_device().create_buffer_init(&BufferInitDescriptor {
        label: Some("Asset Service Texture Atlas Buffer"),
        contents: &diffuse_rgba,
        usage: BufferUsages::COPY_SRC,
    });

    let mut encoder = get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Asset Service Texture Atlas Command Encoder"),
    });

    // Add it to buffer
    encoder.copy_buffer_to_texture(
        wgpu::ImageCopyBuffer {
            buffer: &diffuse_buffer,
            layout: ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(NonZeroU32::try_from(4 * size.width).unwrap()),
                rows_per_image: Some(NonZeroU32::try_from(size.height).unwrap()),
            },
        },
        wgpu::ImageCopyTexture {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        size,
    );

    queue.submit(Some(encoder.finish()));

    let diffuse_sampler_descriptor = wgpu::SamplerDescriptor {
        label: Some("Asset Service Texture Atlas Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare: None,
        anisotropy_clamp: None,
        border_color: None,
    };

    let diffuse_sampler = get_device().create_sampler(&diffuse_sampler_descriptor);

    diffuse_sampler
}

fn write_cached_atlas(
    path: &PathBuf,
    atlas_path: &PathBuf,
    atlas_index_path: &PathBuf,
    atlas_info_path: &PathBuf,
    atlas_index: &HashMap<String, TextureAtlasIndex, FnvBuildHasher>,
    zip_name: &str,
    atlas: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    resource_pack: &mut ResourcePack,
) {
    if !path.as_path().is_dir() {
        if let Result::Err(error) = std::fs::create_dir(path.clone()) {
            log_error!("Failed to create cache directory: {}", error)
        }
    }

    if let Err(e) = atlas.save(atlas_path) {
        log_error!("Failed to cache atlas image: {}", e);
    }

    let result = serde_json::to_string(&atlas_index).unwrap();

    match File::create(atlas_index_path) {
        Ok(mut atlas_index_file) => {
            if let Err(e) = atlas_index_file.write_all(result.as_bytes()) {
                log_error!("Error writing texture atlas index: {}", e);
            }
        }
        Err(e) => {
            log_error!("Failed to cache atlas index: {}", e);
        }
    }

    match File::create(&atlas_info_path) {
        Ok(mut file) => {
            let output = format!(
                "{}\n{}",
                zip_name,
                resource_pack
                    .modified
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            if let Result::Err(err) = file.write_all(output.as_bytes()) {
                log_error!("Error while writing atlas_info file {}", err)
            }
        }
        Err(e) => {
            log_error!("Failed to cache atlas info: {}", e);
        }
    }
}

fn load_cached_atlas_content(
    _settings: &SettingsService,
    atlas_path: &PathBuf,
    atlas_index_path: &PathBuf,
) -> Result<
    (
        DynamicImage,
        HashMap<String, TextureAtlasIndex, FnvBuildHasher>,
    ),
    Box<dyn std::error::Error>,
> {
    let img = image::open(atlas_path)?;

    let mut index_file = File::open(atlas_index_path)?;
    let mut data = Vec::new();
    index_file.read_to_end(&mut data)?;

    let index = serde_json::from_slice::<HashMap<String, TextureAtlasIndex, FnvBuildHasher>>(
        data.as_slice(),
    )?;

    Ok((img, index))
}

fn load_cached_atlas(
    atlas_path: &PathBuf,
    atlas_info_path: &PathBuf,
    atlas_index_path: &PathBuf,
    zip_name: &str,
    resource_pack: &ResourcePack,
    settings: &SettingsService,
) -> Option<(
    DynamicImage,
    HashMap<String, TextureAtlasIndex, FnvBuildHasher>,
)> {
    let pack_details_match = {
        match File::open(&atlas_info_path) {
            Ok(mut file) => {
                let mut info = String::new();
                if let Result::Err(err) = file.read_to_string(&mut info) {
                    log_error!("Error reading atlas_info file {}", err);
                }
                let (name, time) = info.split_at(info.find("\n").unwrap_or(0));
                let time = time.trim().parse::<u64>().unwrap();
                name == zip_name
                    && resource_pack
                        .modified
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                        == time
            }
            Err(_) => false,
        }
    };

    if pack_details_match {
        match load_cached_atlas_content(&settings, &atlas_path, &atlas_index_path) {
            Ok((img, index)) => {
                return Some((img, index));
            }
            Err(e) => log_error!("Error loading cached atlas info {}", e),
        }
    }

    None
}
