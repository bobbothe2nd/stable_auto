extern crate rust_ver;

use rust_ver::{build::cfg_rust_threshold, ver::Version};

fn main() {
    cfg_rust_threshold("weak_ptr", Version::new(1, 4, 0)).unwrap();

    cfg_rust_threshold("pin", Version::new(1, 33, 0)).unwrap();

    cfg_rust_threshold("char_escape_debug", Version::new(1, 20, 0)).unwrap();
    cfg_rust_threshold("decode_utf16", Version::new(1, 9, 0)).unwrap();
    cfg_rust_threshold("u8_from_char", Version::new(1, 59, 0)).unwrap();

    cfg_rust_threshold("try_from", Version::new(1, 34, 0)).unwrap();
    cfg_rust_threshold("try_borrow", Version::new(1, 13, 0)).unwrap();

    cfg_rust_threshold("saturating_type", Version::new(1, 74, 0)).unwrap();

    cfg_rust_threshold("manually_drop", Version::new(1, 20, 0)).unwrap();
    cfg_rust_threshold("maybe_uninit", Version::new(1, 36, 0)).unwrap();

    cfg_rust_threshold("nonzero", Version::new(1, 28, 0)).unwrap();
    cfg_rust_threshold("signed_nonzero", Version::new(1, 34, 0)).unwrap();

    cfg_rust_threshold("layout", Version::new(1, 28, 0)).unwrap();
    cfg_rust_threshold("layout_error", Version::new(1, 50, 0)).unwrap();

    cfg_rust_threshold("cstr", Version::new(1, 64, 0)).unwrap();
    cfg_rust_threshold("cstr_until_nul", Version::new(1, 69, 0)).unwrap();

    cfg_rust_threshold("once_cell", Version::new(1, 70, 0)).unwrap();
    cfg_rust_threshold("lazy_cell", Version::new(1, 80, 0)).unwrap();

    cfg_rust_threshold("rust_simd", Version::new(1, 27, 0)).unwrap();
    cfg_rust_threshold("avx512_types", Version::new(1, 72, 0)).unwrap();
    cfg_rust_threshold("u16_simd", Version::new(1, 89, 0)).unwrap();
    cfg_rust_threshold("f16_simd", Version::new(1, 94, 0)).unwrap();

    cfg_rust_threshold("boxed_arr_iter", Version::new(1, 99, 0)).unwrap();
}
