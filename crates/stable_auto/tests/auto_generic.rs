use core::marker::PhantomData;

use stable_auto::{auto_leaf, auto_opaque, auto_trait, Auto};

#[auto_trait]
trait Marker<T = u8>: Clone
where
    Self: Copy,
{
}

auto_leaf! {
    impl Marker for u32 {}
    impl Marker for usize {}
    impl<T: Marker> Marker for &T {}
    impl<T: Marker, const N: usize> Marker for [T; N] {}
}

mod impls {
    stable_auto::auto_leaf! {
        impl super::Marker for u16 {}
        impl super::Marker for super::PhantomMarked<'_> {}
    }
}

#[auto_opaque]
struct PhantomNotMarked;

#[repr(C)]
#[derive(Debug)]
#[auto_opaque]
#[derive(Clone, Copy)]
struct PhantomMarked<'a, T: Clone = u8, const N: usize = 32>
where
    T: Send,
{
    _marker: PhantomData<&'a (T, [T; N])>,
}

#[derive(Debug)]
#[derive(Auto, Clone)]
#[repr(C)]
struct ImplAuto3<'a, T: Clone = usize>
where
    T: Send,
{
    _a: &'a PhantomMarked<'a>,
    _b: [u16; 3],
    _c: T,
}

#[derive(Auto)]
#[repr(u16)]
enum EnumReprU16 {
    This,
    That,
}

fn if_auto<T: Marker>() {}

#[test]
fn auto_leaf() {
    if_auto::<u32>();
    if_auto::<PhantomMarked>();
    if_auto::<ImplAuto3>();
    if_auto::<EnumReprU16>();
    let _ = (EnumReprU16::That, EnumReprU16::This);
    let _ = PhantomNotMarked;
    let _ = *&PhantomMarked::<'_, u8, 32> {
        _marker: PhantomData,
    };
}
