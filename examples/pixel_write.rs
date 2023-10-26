//! Writes to the pixels of a texture.
//!
//! This example performs the following operations:
//! - Creates a blank texture on the GPU
//! - Pulls the pixels from that texture onto the CPU
//! - Fills the texture with white pixels.
//! - Fills

use futures::executor::block_on;
use g2d::math::Dimension;
use image::RgbaImage;

async fn run() {
    let context = g2d::Context::new().await;

    // Create the blank texture
    let texture = context.make_blank_texture(Dimension::new(16, 16));
    let graphics = texture.graphics();

    // Write pixels
    graphics
        .pixels_mut(|mut pixels| {
            // fill with white
            pixels.rows_mut().for_each(|row| row.fill(255));
        })
        .await;

    // Read pixels
    let pixels = graphics.pixels_mut(|pixels| pixels.to_vec()).await;

    // Write image
    RgbaImage::from_raw(16, 16, pixels)
        .unwrap()
        .save("test.png")
        .unwrap();
}

fn main() {
    block_on(run());
}
