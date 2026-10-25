# Stable Auto (`stable_auto`)

This crate offers 4 macros, 1 trait, and 2 types for one goal: Implementing traits that aren't even in your dependency graph for your types using auto traits on stable rust.

Much like unstable `auto trait`s, this crate implements traits for types containing only other types which implement that trait.

## Usage

```rust
use stable_auto::{auto_leaf, auto_opaque, auto_trait, Auto};

#[auto_trait]
trait Marker<T = u8>: Clone
where
    Self: Copy,
{
}

auto_leaf! {
    impl Marker for u16 {}
    impl Marker for u32 {}
    impl Marker for usize {}
    impl<T: Marker, const N: usize> Marker for [T; N] {}
    impl Marker for PhantomMarked<'_> {}
}

#[auto_opaque]
struct PhantomNotMarked;

#[auto_opaque]
#[derive(Clone, Copy)]
struct PhantomMarked<'a, T: Clone = u8, const N: usize = 32>
where
    T: Send,
{
    _marker: core::marker::PhantomData<&'a (T, [T; N])>,
}

#[derive(Auto)]
struct ImplAuto3<'a, T: Clone = usize>
where
    T: Send,
{
    _a: PhantomMarked<'a>,
    _b: [u16; 3],
    _c: T,
}
```
