//! Simulating `auto trait`s on stable Rust through blanket `impl`s

#![forbid(missing_docs)]
#![no_std]

use core::marker::{PhantomData, PhantomPinned};

pub use stable_auto_macros::{auto_leaf, auto_opaque, auto_trait};

mod private {
    pub trait SealedTuple {}
    impl<A> SealedTuple for super::AllImpl<A> {}
    impl<A> SealedTuple for super::Leaf<A> {}
    impl SealedTuple for () {}
    impl<A, B> SealedTuple for (A, B) {}
}

/// Represents a leaf node in auto trait solving
///
/// Use this when you want to use an auto type to be used with multiple other types.
pub struct Leaf<T = ()>(PhantomData<T>);

/// Represents an opaque wrapper in auto trait solving
///
/// While mostly an implementation detail, this is used when a type is a always an auto leaf node.
pub struct AllImpl<T = ()>(PhantomData<T>);

/// A trait to automatically implement other traits if all fields implement them.
///
/// Prefer to use `#[derive(Auto)]` instead of implementing manually.
/// Manual implementation is meant as an advanced (and unsafe) escape hatch if you want it to represent something other than it's fields/`Self`.
///
/// This means, if `Send2` is a `#[auto_trait]`, these would be equal:
///
/// ```rust
/// # use core::cell::UnsafeCell;
/// struct Foo {
///     a: UnsafeCell<u32>,
/// }
///
/// unsafe impl Send for Foo {}
/// ```
///
/// and:
///
/// ```rust
/// # use stable_auto::*; use core::cell::UnsafeCell; #[auto_trait] trait Send2 {}
/// struct Foo {
///     a: UnsafeCell<u32>,
/// }
///
/// unsafe impl Auto for Foo {
///     type Fields = stable_auto::AllImpl<u32>;
/// }
/// ```
///
/// # Safety
///
/// An `Auto` implementation must choose `Fields` such that every safety-relevant property that is supposed to propagate through the auto trait is correctly represented by `Fields`.
pub unsafe trait Auto {
    /// Must represent all the unique types contained within this type; else unsound
    type Fields: private::SealedTuple;
}

unsafe impl<T> Auto for AllImpl<T> {
    type Fields = Self;
}

unsafe impl<T> Auto for Leaf<T> {
    type Fields = Self;
}

unsafe impl Auto for () {
    type Fields = Self;
}

unsafe impl<A: Auto, B: Auto> Auto for (A, B) {
    type Fields = (AllImpl<A>, AllImpl<B>);
}

unsafe impl<T: Auto, const N: usize> Auto for [T; N] {
    type Fields = AllImpl<Self>;
}

macro_rules! impl_auto_leaf {
    ($(<$($generics:ident),*> $ty:ty)*) => {$(
        unsafe impl <$($generics: Auto),*> Auto for $ty {
            type Fields = AllImpl<Self>;
        }
    )*};

    ($($ty:ty)*) => {$(
        unsafe impl Auto for $ty {
            type Fields = AllImpl<Self>;
        }
    )*};
}

#[rustfmt::skip]
mod leaf {
    use super::*;

    use core::num::{
        NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    };

    impl_auto_leaf!(
        f32 f64
        PhantomPinned
    );

    impl_auto_leaf!(
        u8 u16 u32 u64 u128 usize
        i8 i16 i32 i64 i128 isize
        NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize
        NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize
    );

    impl_auto_leaf!(
        <T> core::num::Wrapping<T>
        <T> core::num::Saturating<T>
        <T> core::mem::MaybeUninit<T>
        <T> core::mem::ManuallyDrop<T>
        <T, E> core::result::Result<T, E>
        <T> core::option::Option<T>
        <T> PhantomData<T>
        <T> &[T]
    );

    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::{
        __m128, __m128bh, __m128d, __m128h, __m128i,
        __m256, __m256bh, __m256d, __m256h, __m256i,
        __m512, __m512bh, __m512d, __m512h, __m512i,
    };

    #[cfg(target_arch = "x86_64")]
    impl_auto_leaf!(
        __m128 __m128bh __m128d __m128h __m128i
        __m256 __m256bh __m256d __m256h __m256i
        __m512 __m512bh __m512d __m512h __m512i
    );
}
