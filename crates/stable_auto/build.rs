extern crate rust_ver;

use rust_ver::{build::cfg_rust_threshold, ver::Version};

fn main() {
    cfg_rust_threshold(
        "rust_simd",
        Version::new(1, 27, 0),
    ).unwrap();

    cfg_rust_threshold(
        "avx512_types",
        Version::new(1, 72, 0),
    ).unwrap();

    cfg_rust_threshold(
        "u16_simd",
        Version::new(1, 89, 0),
    ).unwrap();

    cfg_rust_threshold(
        "f16_simd",
        Version::new(1, 94, 0),
    ).unwrap();

    cfg_rust_threshold(
        "maybe_uninit",
        Version::new(1, 36, 0),
    ).unwrap();

    cfg_rust_threshold(
        "saturating_type",
        Version::new(1, 74, 0),
    ).unwrap();

    cfg_rust_threshold(
        "signed_nonzero",
        Version::new(1, 34, 0),
    ).unwrap();
}
