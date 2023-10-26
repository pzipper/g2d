//! Writes to the pixels of a texture.
//!
//! This example performs the following operations:
//! - Creates a blank texture on the GPU
//! - Pulls the pixels from that texture onto the CPU
//! - Fills the texture with white pixels.
//! - Fills

use std::time::Instant;

use futures::executor::block_on;
use g2d::math::{Dimension, Rgba};
use image::RgbaImage;

async fn run() {
    let context = g2d::Context::new().await;

    // Create the blank texture
    let texture = context.make_blank_texture(Dimension::new(16, 16));
    let graphics = texture.graphics();

    let time_start = Instant::now();

    let data = [255; 16 * 16 * Rgba::SIZE]; // blank white texture

    graphics.overwrite_pixel_data(&data);

    // // Write pixels
    // graphics
    //     .update_pixels(|mut pixels| {
    //         pixels.rows_mut().for_each(|row| row.fill(255))

    //         // pixels.set_color(0, 0, Rgba::BLACK);
    //         // pixels.set_color(1, 0, Rgba::WHITE);
    //         // pixels.set_color(2, 0, Rgba::BLACK);
    //         // pixels.set_color(1, 1, Rgba::BLACK);

    //         // // Fill with white
    //         // pixels.update_pixels().enumerate().for_each(|(idx, pixel)| {
    //         //     println!("TEST: {} (color: {:?}", idx, pixel);
    //         //     *pixel = Rgba::WHITE
    //         // })

    //         // for pixel in pixels.pixels() {
    //         //     println!("{:?}", pixel);
    //         // }
    //     })
    //     .await;

    // Read pixels
    let pixels = graphics.pixels().await;

    let time_elapsed = time_start.elapsed();
    println!(
        "Elapsed: {}ms",
        time_elapsed.as_nanos() as f64 / 1_000_000f64
    );

    // Write image
    RgbaImage::from_raw(16, 16, pixels.to_vec())
        .unwrap()
        .save("test.png")
        .unwrap();
}

fn main() {
    block_on(run());
}
