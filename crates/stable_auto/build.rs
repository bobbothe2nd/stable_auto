extern crate rust_ver;

use rust_ver::{build::cfg_rust_threshold, ver::Version};

fn main() {
    cfg_rust_threshold("split_inclusive", Version::new(1, 51, 0)).unwrap();
    cfg_rust_threshold("u8_from_char", Version::new(1, 59, 0)).unwrap();

    cfg_rust_threshold("saturating_type", Version::new(1, 74, 0)).unwrap();

    cfg_rust_threshold("num_buffer", Version::new(1, 98, 0)).unwrap();

    cfg_rust_threshold("cstr", Version::new(1, 64, 0)).unwrap();
    cfg_rust_threshold("cstr_until_nul", Version::new(1, 69, 0)).unwrap();

    cfg_rust_threshold("once_poison", Version::new(1, 51, 0)).unwrap();
    cfg_rust_threshold("once_cell", Version::new(1, 70, 0)).unwrap();
    cfg_rust_threshold("lazy_cell", Version::new(1, 80, 0)).unwrap();

    cfg_rust_threshold("avx512_types", Version::new(1, 72, 0)).unwrap();
    cfg_rust_threshold("u16_simd", Version::new(1, 89, 0)).unwrap();
    cfg_rust_threshold("f16_simd", Version::new(1, 94, 0)).unwrap();

    cfg_rust_threshold("arr_iter", Version::new(1, 51, 0)).unwrap();
    cfg_rust_threshold("boxed_arr_iter", Version::new(1, 99, 0)).unwrap();

    cfg_rust_threshold("control_flow", Version::new(1, 55, 0)).unwrap();

    cfg_rust_threshold("new_range", Version::new(1, 96, 0)).unwrap();
    cfg_rust_threshold("new_range_inclusive", Version::new(1, 95, 0)).unwrap();

    cfg_rust_threshold("poll_fn", Version::new(1, 64, 0)).unwrap();
}
