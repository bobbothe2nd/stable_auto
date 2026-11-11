#[cfg(target_arch = "x86")]
pub mod x86 {
    use core::arch::x86::{
        __m128, __m128d, __m128i,
        __m256, __m256d, __m256i,
    };

    impl_auto_leaf!(
        __m128 __m128d __m128i
        __m256 __m256d __m256i
    );

    #[cfg(u16_simd)]
    use core::arch::x86::{__m128bh, __m256bh, __m512bh};
    #[cfg(u16_simd)]
    impl_auto_leaf!(__m128bh __m256bh __m512bh);

    #[cfg(f16_simd)]
    use core::arch::x86::{__m128h, __m256h, __m512h};
    #[cfg(f16_simd)]
    impl_auto_leaf!(__m128h __m256h __m512h);

    #[cfg(avx512_types)]
    use core::arch::x86::{__m512, __m512d, __m512i};
    #[cfg(avx512_types)]
    impl_auto_leaf!(__m512 __m512d __m512i);
}

#[cfg(target_arch = "x86_64")]
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

#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
pub mod aarch64 {
    #[cfg(neon_intr)]
    use core::arch::aarch64::*;
    #[cfg(neon_intr)]
    impl_auto_leaf!(
        float32x2_t float32x2x2_t float32x2x3_t float32x2x4_t float32x4_t
        float32x4x2_t float32x4x3_t float32x4x4_t float64x1_t float64x1x2_t
        float64x1x3_t float64x1x4_t float64x2_t float64x2x2_t float64x2x3_t
        float64x2x4_t int16x4_t int16x4x2_t int16x4x3_t int16x4x4_t int16x8_t
        int16x8x2_t int16x8x3_t int16x8x4_t int32x2_t int32x2x2_t int32x2x3_t
        int32x2x4_t int32x4_t int32x4x2_t int32x4x3_t int32x4x4_t int64x1_t
        int64x1x2_t int64x1x3_t int64x1x4_t int64x2_t int64x2x2_t int64x2x3_t
        int64x2x4_t int8x16_t int8x16x2_t int8x16x3_t int8x16x4_t int8x8_t
        int8x8x2_t int8x8x3_t int8x8x4_t poly16x4_t poly16x4x2_t poly16x4x3_t
        poly16x4x4_t poly16x8_t poly16x8x2_t poly16x8x3_t poly16x8x4_t
        poly64x1_t poly64x1x2_t poly64x1x3_t poly64x1x4_t poly64x2_t
        poly64x2x2_t poly64x2x3_t poly64x2x4_t poly8x16_t poly8x16x2_t
        poly8x16x3_t poly8x16x4_t poly8x8_t poly8x8x2_t poly8x8x3_t poly8x8x4_t
        uint16x4_t uint16x4x2_t uint16x4x3_t uint16x4x4_t uint16x8_t
        uint16x8x2_t uint16x8x3_t uint16x8x4_t uint32x2_t uint32x2x2_t
        uint32x2x3_t uint32x2x4_t uint32x4_t uint32x4x2_t uint32x4x3_t
        uint32x4x4_t uint64x1_t uint64x1x2_t uint64x1x3_t uint64x1x4_t
        uint64x2_t uint64x2x2_t uint64x2x3_t uint64x2x4_t uint8x16_t
        uint8x16x2_t uint8x16x3_t uint8x16x4_t uint8x8_t uint8x8x2_t
        uint8x8x3_t uint8x8x4_t
    );
}

#[cfg(target_family = "wasm")]
pub mod wasm32 {
    #[cfg(wasm_simd)]
    impl_auto_leaf!(core::arch::wasm32::v128);
}
