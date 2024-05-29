fn main() {
    println!("cargo::rerun-if-changed=vendor/stb_image_write.c");

    cc::Build::new()
        .file("vendor/stb_image_write.c")
        .define("STB_IMAGE_WRITE_IMPLEMENTATION", None)
        .compile("stb-image-write");
}
