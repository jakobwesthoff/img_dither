# rust_stb_image_write

**rust_stb_image_write** is a minimal rust wrapper around the `stb_image_write.h` image writing implementation from the [stb collection](https://github.com/nothings/stb).

**WIP:** At this current point in time the wrapper is not completed, but only provides access to a limited amount of functions, especially needed for another project of mine. Once the wrapper is finished it will be published to crates.io as well. Until then, it lives as part of the `img_dither` project, where it was initially needed for.

## Building

To build the project, use `cargo` to install dependencies and generate the executable:

```shell
cargo build --release
```

## Usage

Import the needed library functions and use them as with any other rust library.

```rust
use rust_stb_image_write::stbi_write_png_to_file;

stbi_write_png_to_file(output, image.width, image.height, &buffer)
```
