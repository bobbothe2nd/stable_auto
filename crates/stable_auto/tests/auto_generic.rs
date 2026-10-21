use core::marker::PhantomData;

use stable_auto::{auto_leaf, auto_opaque, auto_trait};

#[auto_trait]
trait Marker<T = u8>: Clone
where
    Self: Copy,
{
}

auto_leaf! {
    impl Marker for u32 {}
    impl Marker for usize {}
    impl Marker for PhantomMarked<'_> {}
}

/*
mod impls {
    stable_auto::auto_leaf! {
        // parse paths!!
        impl super::Marker for u16 {}
    }
}
*/

#[auto_opaque]
struct PhantomNotMarked;

#[auto_opaque]
#[derive(Clone, Copy)]
struct PhantomMarked<'a, T: Clone = u8, const N: usize = 32>
where
    T: Send,
{
    _marker: PhantomData<&'a (T, [T; N])>,
}

/*
#[derive(Auto)]
struct ImplAuto3<'a, Y: Clone = usize>
where
    T: Send,
{
    _a: &'a ImplAuto2,
    _b: [u16; 3],
    _c: T,
}
*/

fn if_auto<T: Marker>() {}

#[test]
fn auto_leaf() {
    if_auto::<u32>();
    if_auto::<PhantomMarked>();
    let _ = PhantomNotMarked;
    let _ = *&PhantomMarked::<'_, u8, 32> {
        _marker: PhantomData,
    };
}
