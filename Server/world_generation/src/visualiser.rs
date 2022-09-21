use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::fs::FileType;
use image::{GenericImageView, ImageBuffer, Rgba, RgbImage};
use crate::constants::{CHUNK_SIZE, VERTICAL_CHUNKS};
use crate::helper::Map;

pub struct MapVisualiser;

impl MapVisualiser {

    /// Clears the cache directory
    pub fn prepare() {
        // Delete debug folder
        fs::remove_dir_all("./debug/");
        // Delete images folder
        fs::remove_dir_all("./images/");

        // Recreate it
        fs::create_dir("./debug/").unwrap();

        // Recreate it
        fs::create_dir("./images/").unwrap();
    }

    /// Combines all the images into one large map
    pub fn finish() {
        let mut images: HashMap<String, Vec<[i32; 2]>> = HashMap::new();

        // Collate images into images hashmap
        for file in fs::read_dir("./debug/").unwrap() {
            if let Ok(file) = file {
                let name = file.file_name().into_string().unwrap();

                // All the files to be collated start with x
                if !name.starts_with(":") {
                    continue;
                }

                // Add image
                let mut parts = name.split(":").skip(1);

                let x = parts.next().unwrap().parse::<i32>().unwrap();
                let z = parts.next().unwrap().parse::<i32>().unwrap();
                let name = parts.next().unwrap();

                if images.contains_key(name) {
                    images.get_mut(name).unwrap().push([x, z]);
                } else {
                    images.insert(name.to_string(), vec![[x, z]]);
                }
            }
        }

        // Combine them
        for (name, mut chunks) in images {
            //
            chunks.sort_by(|a, b| {
                match a[1].cmp(&b[1]) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => {
                        a[0].cmp(&b[0])
                    }
                    Ordering::Greater => Ordering::Greater
                }
            });

            let min = chunks.get(0).unwrap();
            let max = chunks.get(chunks.len() - 1).unwrap();

            let size = [max[0] - min[0] + 1, max[1] - min[1] + 1];

            // Create image
            let mut img = ImageBuffer::new(size[0] as u32 * CHUNK_SIZE as u32, size[1] as u32 * CHUNK_SIZE as u32);

            // Go through and add the images
            for location in chunks {
                let name = format!("./debug/:{}:{}:{}:.png", location[0], location[1], name);

                let sub_image = image::open(name).unwrap();

                for x in 0..sub_image.width() {
                    for y in 0..sub_image.height() {
                        img.put_pixel((location[0] * CHUNK_SIZE as i32) as u32 + x, (location[1] * CHUNK_SIZE as i32) as u32 + y, sub_image.get_pixel(x, y));

                        if x == 0 || y == 0 {
                            img.put_pixel((location[0] * CHUNK_SIZE as i32) as u32 + x, (location[1] * CHUNK_SIZE as i32) as u32 + y, Rgba::from([0, 0, 0, 255]));
                        }
                    }
                }
            }

            // Write
            img.save(format!("./images/{}.png", name)).unwrap();
        }

        // Delete debug folder
        fs::remove_dir_all("./debug/");
    }

    pub fn visualise_f64_map11(data: [[f64; CHUNK_SIZE]; CHUNK_SIZE], location: &[i32; 2], name: &str) {
        let img = ImageBuffer::from_fn(CHUNK_SIZE as u32, CHUNK_SIZE as u32, |x, y| {
            image::Luma([((data[x as usize][y as usize] + 1.0) * (255.0 / 2.0)) as u8])
        });

        img.save(format!("./debug/:{}:{}:{}:.png", location[0], location[1], name)).unwrap();
    }

    pub fn visualise_f64_map1(data: [[f64; CHUNK_SIZE]; CHUNK_SIZE], location: &[i32; 2], name: &str) {
        let img = ImageBuffer::from_fn(CHUNK_SIZE as u32, CHUNK_SIZE as u32, |x, y| {
            image::Luma([(data[x as usize][y as usize] * 255.0) as u8])
        });

        img.save(format!("./debug/:{}:{}:{}:.png", location[0], location[1], name)).unwrap();
    }

    pub fn visualise_f64_map_heightmap(data: [[u32; CHUNK_SIZE]; CHUNK_SIZE], location: &[i32; 2], name: &str) {
        let img = ImageBuffer::from_fn(CHUNK_SIZE as u32, CHUNK_SIZE as u32, |x, y| {
            image::Luma([(data[x as usize][y as usize].map(60, 200, 0, 255)) as u8])
        });

        img.save(format!("./debug/:{}:{}:{}:.png", location[0], location[1], name)).unwrap();
    }
}