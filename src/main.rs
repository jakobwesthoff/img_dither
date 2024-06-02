mod color;
mod dithering;
mod image_handling;
mod resizing;

use image_handling::*;
// use resizing::resize_lanczos;

use dithering::{dither_image, DitheringKernel, PALETTE16};

use anyhow::{Context, Result};
use std::env;

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

    // let smaller = resize_lanczos(&image, image.width / 8, image.height / 8, 3f64)?;
    // let bigger = resize_lanczos(&smaller, smaller.width * 8, smaller.height * 8, 3f64)?;

    // save_image_to_file(&smaller, "./smaller.png")?;
    // save_image_to_file(&bigger, "./bigger.png")?;

    let dithered = dither_image(&image, &DitheringKernel::floyd_steinberg(), &PALETTE16);
    save_image_to_file(&dithered, output).with_context(|| format!("saving image to {output}"))?;

    Ok(())
}
