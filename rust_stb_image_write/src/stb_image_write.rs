/* automatically generated by rust-bindgen 0.69.4 */

extern "C" {
    pub fn stbi_write_png(
        filename: *const ::std::os::raw::c_char,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        comp: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
        stride_in_bytes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_write_bmp(
        filename: *const ::std::os::raw::c_char,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        comp: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_write_tga(
        filename: *const ::std::os::raw::c_char,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        comp: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_write_hdr(
        filename: *const ::std::os::raw::c_char,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        comp: ::std::os::raw::c_int,
        data: *const f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_write_jpg(
        filename: *const ::std::os::raw::c_char,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        comp: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
        quality: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub type stbi_write_func = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
    ),
>;
extern "C" {
    pub fn stbi_write_png_to_func(
        func: stbi_write_func,
        context: *mut ::std::os::raw::c_void,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        comp: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
        stride_in_bytes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_write_bmp_to_func(
        func: stbi_write_func,
        context: *mut ::std::os::raw::c_void,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        comp: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_write_tga_to_func(
        func: stbi_write_func,
        context: *mut ::std::os::raw::c_void,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        comp: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_write_hdr_to_func(
        func: stbi_write_func,
        context: *mut ::std::os::raw::c_void,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        comp: ::std::os::raw::c_int,
        data: *const f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_write_jpg_to_func(
        func: stbi_write_func,
        context: *mut ::std::os::raw::c_void,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        comp: ::std::os::raw::c_int,
        data: *const ::std::os::raw::c_void,
        quality: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stbi_flip_vertically_on_write(flip_boolean: ::std::os::raw::c_int);
}
