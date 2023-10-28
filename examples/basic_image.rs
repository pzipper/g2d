use futures::executor::block_on;
use g2d::{Handle, Texture};
use image::RgbaImage;

async fn run() {
    let handle = g2d::WindowlessHandle::new().await.unwrap();

    let data = vec![255; 4 * 16 * 16]; // filled with white

    let texture = handle
        .make_texture(g2d::Dimension::new(16, 16), &data)
        .unwrap();

    // Load the pixels from the texture
    let pixels = texture.canvas().pixels().await.unwrap();

    RgbaImage::from_raw(16, 16, pixels.into_vec())
        .unwrap()
        .save("test.png")
        .unwrap();
}

fn main() {
    block_on(run());
}
