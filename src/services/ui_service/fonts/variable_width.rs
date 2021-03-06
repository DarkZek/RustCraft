use crate::services::asset_service::atlas::{ATLAS_HEIGHT, ATLAS_WIDTH};
use crate::services::asset_service::AssetService;
use crate::services::ui_service::fonts::FONT_TEXTURE_SIZE;
use image::GenericImageView;

/// Create a map of the widths of each character so we can display them nicely on the screen without them being monospaced.
pub fn generate_variable_width_map(assets: &AssetService) -> [u8; 127] {
    let mut ascii_atlas_uv = assets
        .atlas_index
        .as_ref()
        .unwrap()
        .get("font/ascii")
        .unwrap()
        .clone();

    let mut image = assets.atlas_image.as_ref().unwrap().clone();

    ascii_atlas_uv.multiply(ATLAS_WIDTH as f32, ATLAS_HEIGHT as f32);

    // Reduce the size so we can focus on only the ascii texture
    image = image.crop(
        ascii_atlas_uv.u_min as u32,
        ascii_atlas_uv.v_min as u32,
        ascii_atlas_uv.width() as u32,
        ascii_atlas_uv.height() as u32,
    );

    let letter_size = ascii_atlas_uv.width() as i32 / FONT_TEXTURE_SIZE as i32;

    let mut width = [0; 127];

    // Loop over letters and compile list of widths
    for i in 0..127 {
        let row = (i as f32 / FONT_TEXTURE_SIZE).floor() as i32;
        let column = i as i32 % FONT_TEXTURE_SIZE as i32;

        let mut rightmost_pixel = (letter_size as f32 * 0.75) as i32;

        // Scan from right to left looking for the rightmost non transparent pixel
        'checker: for x in (0..letter_size).rev() {
            // Scan whole column
            for y in 0..letter_size {
                let pixel = image.get_pixel(
                    (column * letter_size) as u32 + x as u32,
                    (row * letter_size) as u32 + y as u32,
                );

                // Check if its transparent
                if pixel.0[3] != 0 {
                    rightmost_pixel = x;
                    break 'checker;
                }
            }
        }

        width[i] = ((rightmost_pixel as f32 / letter_size as f32) * 255.0) as u8;
    }

    // Make space width smaller
    width[32] /= 3;

    width
}
