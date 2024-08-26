use wasm_bindgen::prelude::*;
use image::{ImageBuffer, Rgba};
use rusttype::{Font, Scale, point};
use base64::encode;
use rand::Rng;

#[wasm_bindgen]
pub fn generate_image_from_text(text: &str) -> String {
    let width = 300;
    let height = 100;

    // Load a font
    let font_data = include_bytes!("../Panchang-Extrabold.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    // Set font size
    let scale = Scale::uniform(45.0);

    // Measure the text
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0)).collect();
    let width_of_text: i32 = if let Some(last_glyph) = glyphs.last() {
        last_glyph.position().x as i32 + last_glyph.unpositioned().h_metrics().advance_width as i32
    } else {
        0
    };

    let height_of_text = v_metrics.ascent - v_metrics.descent;

    // Calculate position for centered text
    let start_x = (width as i32 - width_of_text) / 2;
    let start_y = ((height as f32 + height_of_text) / 2.0 - v_metrics.descent) as i32 - 18;

    // Create a new image buffer
    let mut image = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));

    // Draw the text
    for glyph in font.layout(text, scale, point(start_x as f32, start_y as f32)) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let pixel_x = x + bounding_box.min.x as u32;
                let pixel_y = y + bounding_box.min.y as u32;
                if pixel_x < width && pixel_y < height {
                    let pixel = image.get_pixel_mut(pixel_x, pixel_y);
                    *pixel = image::Rgba([
                        (255.0 * (1.0 - v)) as u8, 
                        (255.0 * (1.0 - v)) as u8, 
                        (255.0 * (1.0 - v)) as u8, 
                        255
                    ]);
                }
            });
        }
    }

    // Add noise to the image (random pixels)
    add_noise(&mut image, width, height);

    // Convert the image to PNG format
    let mut png_data = Vec::new();
    image::codecs::png::PngEncoder::new(&mut png_data)
        .encode(&image, width, height, image::ColorType::Rgba8)
        .unwrap();

    // Encode the PNG data to Base64
    let base64_png = encode(&png_data);

    // Return the Base64-encoded PNG with the proper data URI scheme
    format!("data:image/png;base64,{}", base64_png)
}

fn add_noise(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, width: u32, height: u32) {
    let mut rng = rand::thread_rng();
    let noise_amount = 5000; // Number of noise pixels

    for _ in 0..noise_amount {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let pixel = image.get_pixel_mut(x, y);
        let noise_value = rng.gen_range(0..=255) as u8;

        *pixel = Rgba([noise_value, noise_value, noise_value, 255]);
    }

    // Optionally, add some random lines for more complexity
    for _ in 0..15 {
        let x1 = rng.gen_range(0..width);
        let y1 = rng.gen_range(0..height);
        let x2 = rng.gen_range(0..width);
        let y2 = rng.gen_range(0..height);

        draw_line(image, x1, y1, x2, y2, Rgba([0, 0, 0, 255]));
    }
}

fn draw_line(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x0: u32, y0: u32, x1: u32, y1: u32, color: Rgba<u8>) {
    let dx = (x1 as i32 - x0 as i32).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 as i32 - y0 as i32).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;

    loop {
        if x0 >= 0 && y0 >= 0 && (x0 as u32) < image.width() && (y0 as u32) < image.height() {
            let pixel = image.get_pixel_mut(x0 as u32, y0 as u32);
            *pixel = color;
        }

        if x0 == x1 as i32 && y0 == y1 as i32 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}