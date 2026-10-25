//! Simulating `auto trait`s on stable Rust through blanket `impl`s

#![forbid(missing_docs)]
#![no_std]

#![allow(clippy::incompatible_msrv)]

extern crate stable_auto_macros;
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use core::marker::PhantomData;

pub use stable_auto_macros::{auto_leaf, auto_opaque, auto_trait, Auto};

mod private {
    pub trait SealedTuple {}
    impl<A: ?Sized> SealedTuple for super::AllImpl<A> {}
    impl<A: ?Sized> SealedTuple for super::Leaf<A> {}
    impl SealedTuple for () {}
    impl<A, B> SealedTuple for (A, B) {}
}

/// Represents a leaf node in auto trait solving
///
/// Use this when you want to use an auto type to be used with multiple other types.
pub struct Leaf<T: ?Sized = ()>(PhantomData<T>);

/// Represents an opaque wrapper in auto trait solving
///
/// While mostly an implementation detail, this is used when a type is a always an auto leaf node.
pub struct AllImpl<T: ?Sized = ()>(PhantomData<T>);

/// A trait to automatically implement other traits if all fields implement them.
///
/// Prefer to use `#[derive(Auto)]` instead of implementing manually.
/// Manual implementation is meant as an advanced (and unsafe) escape hatch if you want it to represent something other than it's fields/`Self`.
///
/// This means, if `Send2` is a `#[auto_trait]`, these would be equal:
///
/// ```rust
/// # use std::cell::UnsafeCell;
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
/// # use stable_auto::*; use std::cell::UnsafeCell;
/// #[auto_trait]
/// unsafe trait Send2 {}
///
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
///
/// `type Fields = AllImpl<Self>` is always safe.
pub unsafe trait Auto {
    /// Must represent all the unique types contained within this type; else unsound
    type Fields: private::SealedTuple;
}

unsafe impl<T: ?Sized> Auto for AllImpl<T> {
    type Fields = Self;
}

unsafe impl<T: ?Sized> Auto for Leaf<T> {
    type Fields = Self;
}

unsafe impl<T: ?Sized> Auto for &T {
    type Fields = AllImpl<Self>;
}

unsafe impl<T: ?Sized> Auto for &mut T {
    type Fields = AllImpl<Self>;
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
    (<P: Pattern> $($ty:ident$(<$lifetime:lifetime>)?)*) => {$(
        impl_auto_leaf!($ty<$($lifetime)?, &str>);
        impl_auto_leaf!($ty<$($lifetime)?, char>);
        impl_auto_leaf!($ty<$($lifetime)?, &[char]>);
        unsafe impl<P: FnMut(char) -> bool> Auto for $ty<$($lifetime)?, P> {
            type Fields = AllImpl<Self>;
        }
    )*};

    ($(<$($generics:ident$(: ?$Sized:ident)?),*> $ty:ty)*) => {$(
        unsafe impl <$($generics: Auto $(+ ?$Sized)?),*> Auto for $ty {
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
mod leaf;

#[cfg(feature = "alloc")]
#[rustfmt::skip]
mod leaf_alloc;

#[cfg(feature = "std")]
#[rustfmt::skip]
mod leaf_std;
