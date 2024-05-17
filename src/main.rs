use anyhow::{Context, Result};
use image::ExtendedColorType;
use std::env;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl From<u32> for Color {
    fn from(v: u32) -> Self {
        Color {
            b: (v & 0xff) as u8,
            g: (v >> 8 & 0xff) as u8,
            r: (v >> 16 & 0xff) as u8,
        }
    }
}

impl From<&[u8]> for Color {
    fn from(v: &[u8]) -> Self {
        Color {
            r: v[0],
            g: v[1],
            b: v[2],
        }
    }
}

struct QuantizationError {
    r: f32,
    g: f32,
    b: f32,
}

fn map_to_palette(orig: Color, palette: &[Color]) -> (&Color, QuantizationError) {
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

    let image = image::io::Reader::open(input)
        .context("open image {input} for reading")?
        .decode()
        .context("decode input image {input}")?
        .into_rgb8();
    let (width, height) = image.dimensions();
    let mut buffer = image.into_raw();

    // https://www.androidarts.com/palette/16pal.htm
    let palette = [
        Color::from(0x000000),
        Color::from(0x9D9D9D),
        Color::from(0xFFFFFF),
        Color::from(0xBE2633),
        Color::from(0xE06F8B),
        Color::from(0x493C2B),
        Color::from(0xA46422),
        Color::from(0xEB8931),
        Color::from(0xF7E26B),
        Color::from(0x2F484E),
        Color::from(0x44891A),
        Color::from(0xA3CE27),
        Color::from(0x1B2632),
        Color::from(0x005784),
        Color::from(0x31A2F2),
        Color::from(0xB2DCEF),
    ];

    let floyd_steinberg = [0.0, 0.0, 7.0 / 16.0, 3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0];

    for cy in 0..height {
        for cx in 0..width {
            let i = ((cy * width + cx) * 3) as usize;
            let (new, qe) = map_to_palette(Color::from(&buffer[i..i + 3]), &palette);
            buffer[i] = new.r;
            buffer[i + 1] = new.g;
            buffer[i + 2] = new.b;

            for dy in 0..=1 {
                for dx in -1..=1 {
                    let x = cx as isize + dx;
                    let y = cy + dy;
                    if x < 0 || x >= width as isize || y >= height {
                        continue;
                    }

                    let i = ((y * width + x as u32) * 3) as usize;
                    let di = ((dy * 3) + (1_isize + dx) as u32) as usize;

                    buffer[i] = (buffer[i] as f32 + qe.r * floyd_steinberg[di])
                        .round()
                        .clamp(0.0, 255.0) as u8;
                    buffer[i + 1] = (buffer[i + 1] as f32 + (qe.g * floyd_steinberg[di]))
                        .round()
                        .clamp(0.0, 255.0) as u8;
                    buffer[i + 2] = (buffer[i + 2] as f32 + (qe.b * floyd_steinberg[di]))
                        .round()
                        .clamp(0.0, 255.0) as u8;
                }
            }
        }
    }

    image::save_buffer(output, &buffer, width, height, ExtendedColorType::Rgb8)
        .context("save image to {output}")?;

    Ok(())
}
