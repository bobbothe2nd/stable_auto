pub trait Marker {}

impl __auto_Marker_inner for u16 {}
impl __auto_Marker_inner for u32 {}
impl __auto_Marker_inner for usize {}

impl<T> Marker for T
where
    T: ::stable_auto::Auto,
    T::Fields: ::stable_auto::Auto,
    <T::Fields as ::stable_auto::Auto>::Fields: __auto_Marker_inner,
{
}

#[allow(non_camel_case_types)]
trait __auto_Marker_inner {}

impl<T> __auto_Marker_inner for ::stable_auto::AllImpl<T> where T: __auto_Marker_inner {}

impl<T> __auto_Marker_inner for ::stable_auto::Leaf<T> where T: Marker {}

impl<A, B> __auto_Marker_inner for (A, B)
where
    A: ::stable_auto::Auto,
    B: ::stable_auto::Auto,
    A::Fields: Marker,
    B::Fields: Marker,
{
}
