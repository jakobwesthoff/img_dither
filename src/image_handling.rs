use image::ExtendedColorType;
use anyhow::{Context, Result};

use crate::color::Color;

pub struct Image {
    pub data: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

pub fn load_image_from_file(input: &str) -> Result<Image> {
    let image = image::io::Reader::open(input)
        .context("open image {input} for reading")?
        .decode()
        .context("decode input image {input}")?
        .into_rgb8();
    let (width, height) = image.dimensions();
    let buffer = image.into_raw();
    let mut data = vec![Color::from(0xff); width as usize * height as usize];
    for i in 0..buffer.len() / 3 {
        data[i].r = buffer[i * 3];
        data[i].g = buffer[i * 3 + 1];
        data[i].b = buffer[i * 3 + 2];
    }
    Ok(Image {
        width: width as usize,
        height: height as usize,
        data,
    })
}

pub fn save_image_to_file(image: &Image, output: &str) -> Result<()> {
    let mut buffer = vec![0u8; image.width * image.height * 3];
    for i in 0..image.width * image.height {
        buffer[i * 3] = image.data[i].r;
        buffer[i * 3 + 1] = image.data[i].g;
        buffer[i * 3 + 2] = image.data[i].b;
    }

    image::save_buffer(
        output,
        &buffer,
        image.width as u32,
        image.height as u32,
        ExtendedColorType::Rgb8,
    )
    .context("save image to {output}")?;

    Ok(())
}
