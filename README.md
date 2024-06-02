# img_dither

**img_dither** is a simple Rust implementation of image dithering through error diffusion. It was developed as part of a service to display images on a custom 7-color epaper picture frame.

## Disclaimer
While there are comprehensive dithering implementations available, **img_dither** was created for educational purposes and tailored to specific requirements. It is not recommended for production use. However, if you are interested in learning the fundamentals of error diffusion-based dithering, this codebase could be valuable.

## Building

To build the project, use `cargo` to install dependencies and generate the executable:

```shell
cargo build --release
```

## Running

The executable produced by `cargo build` requires two arguments: the **input image** and the **output image**. The palette and diffusion matrix are currently hardcoded within the executable.

Example usage:

```shell
target/release/img_dither --load examples/penguins.jpg --lanczos 832 1152 --dither --save dithered_resized.png
```
