extern crate stable_auto;

use stable_auto::{AllImpl, Auto, Leaf};

trait Marker {}

impl __auto_Marker_inner for u16 {}
impl __auto_Marker_inner for u32 {}
impl __auto_Marker_inner for usize {}

impl<T> Marker for T
where
    T: Auto,
    T::Fields: __auto_Marker_inner,
{
}

#[allow(non_camel_case_types)]
trait __auto_Marker_inner {}

impl<T: __auto_Marker_inner> __auto_Marker_inner for AllImpl<T> {}

impl<T: Marker> __auto_Marker_inner for Leaf<T> {}

impl<A, B> __auto_Marker_inner for (A, B)
where
    A: Auto,
    B: Auto,
    A::Fields: Marker,
    B::Fields: Marker,
{
}

struct ImplAuto1;

unsafe impl Auto for ImplAuto1 {
    type Fields = Leaf<u32>;
}

struct ImplAuto2;

unsafe impl Auto for ImplAuto2 {
    type Fields = (Leaf<u32>, Leaf<usize>);
}

struct ImplAuto3;

unsafe impl Auto for ImplAuto3 {
    type Fields = ((Leaf<ImplAuto2>, Leaf<u16>), Leaf<usize>);
}

fn if_auto<T: Marker>() {}

#[test]
fn impl_auto() {
    if_auto::<u32>();
    if_auto::<ImplAuto1>();
    if_auto::<ImplAuto2>();
    if_auto::<ImplAuto3>();
}
