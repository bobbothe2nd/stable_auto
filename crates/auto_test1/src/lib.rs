use stable_auto::{AllImpl, Auto, Leaf};

pub trait Marker {}

impl __auto_Marker_inner for u16 {}
impl __auto_Marker_inner for u32 {}
impl __auto_Marker_inner for usize {}

impl<T> Marker for T
where
    T: Auto,
    T::Fields: Auto,
    <T::Fields as Auto>::Fields: __auto_Marker_inner,
{
}

#[allow(non_camel_case_types)]
trait __auto_Marker_inner {}

impl<T> __auto_Marker_inner for AllImpl<T>
where 
    T: __auto_Marker_inner,
{}

impl<T> __auto_Marker_inner for Leaf<T>
where 
    T: Marker,
{}

impl<A, B> __auto_Marker_inner for (A, B)
where
    A: Auto,
    B: Auto,
    A::Fields: Marker,
    B::Fields: Marker,
{
}
