mod color;

use color::Color;

use anyhow::{Context, Result};
use image::ExtendedColorType;
use std::env;

struct QuantizationError {
    r: f32,
    g: f32,
    b: f32,
}

fn map_to_palette<'a>(orig: &Color, palette: &'a [Color]) -> (&'a Color, QuantizationError) {
    let mut min_distance = f32::INFINITY;
    let mut color = &palette[0];

    for candidate in palette {
        let distance = (orig.r as f32 - candidate.r as f32).powi(2)
            + (orig.g as f32 - candidate.g as f32).powi(2)
            + (orig.b as f32 - candidate.b as f32).powi(2);

        if distance < min_distance {
            color = &candidate;
            min_distance = distance;
        }
    }

    let qe = QuantizationError {
        r: orig.r as f32 - color.r as f32,
        g: orig.g as f32 - color.g as f32,
        b: orig.b as f32 - color.b as f32,
    };

    (color, qe)
}

struct Image {
    data: Vec<Color>,
    width: usize,
    height: usize,
}

struct DitheringKernel {
    matrix: Vec<f32>,
    width: usize,
    height: usize,
}

fn dither(image: &Image, kernel: &DitheringKernel, palette: &[Color]) -> Image {
    let mut dithered = vec![Color::from(0xff); image.width * image.height];
    dithered.clone_from_slice(image.data.as_ref());

    for cy in 0..image.height {
        for cx in 0..image.width {
            let i = cy * image.width + cx;
            let (new, qe) = map_to_palette(&dithered[i], palette);
            dithered[i] = *new;

            for ky in 0..kernel.height {
                for kx in 0..kernel.width {
                    let dy = ky as isize - (kernel.height as isize / 2);
                    let dx = kx as isize - (kernel.width as isize / 2);

                    let x = cx as isize + dx;
                    let y = cy as isize + dy;
                    if x < 0 || x >= image.width as isize || y < 0 || y >= image.height as isize {
                        continue;
                    }

                    let i = y as usize * image.width + x as usize;
                    let ki = ky * kernel.width + kx;

                    dithered[i].r = (dithered[i].r as f32 + qe.r * kernel.matrix[ki])
                        .round()
                        .clamp(0.0, 255.0) as u8;
                    dithered[i].g = (dithered[i].g as f32 + qe.g * kernel.matrix[ki])
                        .round()
                        .clamp(0.0, 255.0) as u8;
                    dithered[i].b = (dithered[i].b as f32 + qe.b * kernel.matrix[ki])
                        .round()
                        .clamp(0.0, 255.0) as u8;
                }
            }
        }
    }
    Image {
        data: dithered,
        width: image.width,
        height: image.height,
    }
}

fn load_image_from_file(input: &str) -> Result<Image> {
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

fn save_image_to_file(image: &Image, output: &str) -> Result<()> {
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

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let (command, args) = args
        .split_first()
        .expect("expecting to always be provided at least the command within args.");

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {command} <input-image> <output-image>");
        std::process::exit(1);
    }

    let input = &args[0];
    let output = &args[1];

    let image = load_image_from_file(input).with_context(|| format!("loading image {input}"))?;

    // https://www.androidarts.com/palette/16pal.htm
    let palette = [
        Color::from(0x000000FF),
        Color::from(0x9D9D9DFF),
        Color::from(0xFFFFFFFF),
        Color::from(0xBE2633FF),
        Color::from(0xE06F8BFF),
        Color::from(0x493C2BFF),
        Color::from(0xA46422FF),
        Color::from(0xEB8931FF),
        Color::from(0xF7E26BFF),
        Color::from(0x2F484EFF),
        Color::from(0x44891AFF),
        Color::from(0xA3CE27FF),
        Color::from(0x1B2632FF),
        Color::from(0x005784FF),
        Color::from(0x31A2F2FF),
        Color::from(0xB2DCEFFF),
    ];

    let floyd_steinberg = DitheringKernel {
        #[rustfmt::skip]
        matrix: vec![
            0.0,        0.0,        0.0,
            0.0,        0.0,        7.0 / 16.0,
            3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0,
        ],
        width: 3,
        height: 3,
    };

    let dithered = dither(&image, &floyd_steinberg, &palette);

    save_image_to_file(&dithered, output).with_context(|| format!("saving image to {output}"))?;

    Ok(())
}
