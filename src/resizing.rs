use crate::color::Color;
use crate::Image;
use anyhow::Result;

fn sinc(x: f64) -> f64 {
    if x.abs() < f64::EPSILON {
        1f64
    } else {
        x.sin() / x
    }
}

fn lanczos(x: f64, a: f64) -> f64 {
    if -a < x || x < a {
        sinc(x) * sinc(x / a)
    } else {
        0f64
    }
}

fn lanczos2d(x: f64, y: f64, a: f64) -> f64 {
    lanczos(x, a) * lanczos(y, a)
}

pub fn resize_lanczos(image: &Image, new_width: usize, new_height: usize, a: f64) -> Result<Image> {
    let mut new_image = vec![Color::from_rgb(0, 0, 0); new_width * new_height];
    for ny in 0..new_height {
        let oy = ny as f64 * image.height as f64 / new_height as f64;
        for nx in 0..new_width {
            let ox = nx as f64 * image.width as f64 / new_width as f64;

            let mut new_r = 0f64;
            let mut new_g = 0f64;
            let mut new_b = 0f64;
            let mut sum_weights = 0f64;

            for iy in (oy.floor() - a + 1f64) as isize..(oy.floor() + a) as isize {
                for ix in (ox.floor() - a + 1f64) as isize..(ox.floor() + a) as isize {
                    if iy < 0 || iy >= image.height as isize || ix < 0 || ix >= image.width as isize {
                        continue;
                    }

                    let weight = lanczos2d(ox - ix as f64, oy - iy as f64, a);
                    sum_weights += weight;
                    let color = image.data[iy as usize * image.width as usize + ix as usize];
                    new_r += color.r as f64 * weight;
                    new_g += color.g as f64 * weight;
                    new_b += color.b as f64 * weight;
                }
            }

            new_image[ny * new_width + nx].r =
                (new_r / sum_weights).round().clamp(0f64, 255f64) as u8;
            new_image[ny * new_width + nx].g =
                (new_g / sum_weights).round().clamp(0f64, 255f64) as u8;
            new_image[ny * new_width + nx].b =
                (new_b / sum_weights).round().clamp(0f64, 255f64) as u8;
        }
    }

    Ok(Image {
        data: new_image,
        width: new_width,
        height: new_height,
    })
}
