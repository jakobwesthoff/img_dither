use super::Image;
use crate::color::Color;

pub struct DitheringKernel {
    matrix: Vec<f32>,
    width: usize,
    height: usize,
}

impl DitheringKernel {
    pub fn floyd_steinberg() -> Self {
        DitheringKernel {
            #[rustfmt::skip]
            matrix: vec![
                0.0,        0.0,        0.0,
                0.0,        0.0,        7.0 / 16.0,
                3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0,
            ],
            width: 3,
            height: 3,
        }
    }
}

// https://www.androidarts.com/palette/16pal.htm
pub const PALETTE16: [Color; 16] = [
    Color::from_u32_rgba(0x000000FF),
    Color::from_u32_rgba(0x9D9D9DFF),
    Color::from_u32_rgba(0xFFFFFFFF),
    Color::from_u32_rgba(0xBE2633FF),
    Color::from_u32_rgba(0xE06F8BFF),
    Color::from_u32_rgba(0x493C2BFF),
    Color::from_u32_rgba(0xA46422FF),
    Color::from_u32_rgba(0xEB8931FF),
    Color::from_u32_rgba(0xF7E26BFF),
    Color::from_u32_rgba(0x2F484EFF),
    Color::from_u32_rgba(0x44891AFF),
    Color::from_u32_rgba(0xA3CE27FF),
    Color::from_u32_rgba(0x1B2632FF),
    Color::from_u32_rgba(0x005784FF),
    Color::from_u32_rgba(0x31A2F2FF),
    Color::from_u32_rgba(0xB2DCEFFF),
];

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

pub fn dither_image(image: &Image, kernel: &DitheringKernel, palette: &[Color]) -> Image {
    let mut dithered = vec![Color::BLACK; image.width * image.height];
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
