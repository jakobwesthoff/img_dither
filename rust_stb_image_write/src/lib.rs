mod stb_image_write;
use stb_image_write::stbi_write_png;
use std::ffi::CString;
use std::os::raw::c_void;

pub fn stbi_write_png_to_file(path: &str, width: usize, height: usize, data: &[u8]) -> bool {
    let path_cstr = CString::new(path).unwrap();
    unsafe {
        let result = stbi_write_png(
            path_cstr.as_ptr(),
            width as i32,
            height as i32,
            3,
            data.as_ptr() as *const c_void,
            0,
        );
        result != 0
    }
}
