use rand::Rng;
use std::fs;
use image::{RgbImage, ImageBuffer, imageops::resize};

fn main() {
    //Height and Width of the avatar to be created
    const WIDTH: u32 = 16;
    const HEIGHT: u32 = 16;

    //Generate color for avatar
    let color = generate_color();

    //Generate avatar
    let avatar: RgbImage = generate_avatar(color, HEIGHT, WIDTH);

    //Resize avatar to fit in a 256x256 image
    let resized_avatar = resize(&avatar, WIDTH * 16, HEIGHT * 16, image::imageops::FilterType::Nearest);

    //Create output directory
    fs::create_dir_all("./output").expect("Failed to create output directory");

    //Save avatar
    resized_avatar.save("./output/avatar.png").unwrap();
}

//Generates a random color in RGB format as a tuple
fn generate_color() -> (u8, u8, u8) {
    //Generate random RGB color
    let r = rand::thread_rng().gen_range(0..=u8::MAX);
    let g = rand::thread_rng().gen_range(0..=u8::MAX);
    let b = rand::thread_rng().gen_range(0..=u8::MAX);
    (r, g, b)
}

//Generates an RGBImage using the given color and dimensions
fn generate_avatar(color: (u8, u8, u8), h: u32, w: u32) -> RgbImage {
    //Create a new image buffer
    let mut img: RgbImage = ImageBuffer::new(w, h);

    //Loop through image pixels and set them to the given color or white
    for (_x, _y, pixel) in img.enumerate_pixels_mut() {
        let random = rand::thread_rng().gen_range(0..=6); //random number to decide if pixel is filled or not
        if random > 2 {
            *pixel = image::Rgb([color.0, color.1, color.2]); //colored pixel
        } else {
            *pixel = image::Rgb([255, 255, 255]); //white pixel
        }
    }
    img
}