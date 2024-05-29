mod color;
mod dithering;
mod image_handling;

use image_handling::*;

use color::Color;
use dithering::{DitheringKernel, dither_image};

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

    let dithered = dither_image(&image, &DitheringKernel::floyd_steinberg(), &palette);

    save_image_to_file(&dithered, output).with_context(|| format!("saving image to {output}"))?;

    Ok(())
}
