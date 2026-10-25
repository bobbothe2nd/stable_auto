#[cfg(any(target_arch = "x86", doc))]
pub mod x86 {
}

#[cfg(any(target_arch = "x86_64", doc))]
pub mod x86_64 {
    use core::arch::x86_64::{
        __m128, __m128d, __m128i,
        __m256, __m256d, __m256i,
    };

    impl_auto_leaf!(
        __m128 __m128d __m128i
        __m256 __m256d __m256i
    );

    #[cfg(u16_simd)]
    use core::arch::x86_64::{__m128bh, __m256bh, __m512bh};
    #[cfg(u16_simd)]
    impl_auto_leaf!(__m128bh __m256bh __m512bh);

    #[cfg(f16_simd)]
    use core::arch::x86_64::{__m128h, __m256h, __m512h};
    #[cfg(f16_simd)]
    impl_auto_leaf!(__m128h __m256h __m512h);

    #[cfg(avx512_types)]
    use core::arch::x86_64::{__m512, __m512d, __m512i};
    #[cfg(avx512_types)]
    impl_auto_leaf!(__m512 __m512d __m512i);
}

#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", doc))]
pub mod aarch64 {
}

#[cfg(any(target_arch = "wasm32", doc))]
pub mod wasm32 {
}
