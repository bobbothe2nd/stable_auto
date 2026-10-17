use stable_auto::{__stable_auto_macro_inner, AllImpl, Auto};

trait Marker {}

impl __auto_Marker_inner for u16 {}
impl __auto_Marker_inner for u32 {}
impl __auto_Marker_inner for usize {}

__stable_auto_macro_inner!(@auto_trait Marker __auto_Marker_inner);

struct ImplAuto1;

unsafe impl Auto for ImplAuto1 {
    type Fields = AllImpl<u32>;
}

struct ImplAuto2;

unsafe impl Auto for ImplAuto2 {
    type Fields = (AllImpl<u32>, AllImpl<usize>);
}

struct ImplAuto3;

unsafe impl Auto for ImplAuto3 {
    type Fields = ((AllImpl<u32>, AllImpl<u16>), AllImpl<usize>);
}

fn if_auto<T: Marker>() {}

#[test]
fn impl_auto() {
    if_auto::<u32>();
    if_auto::<ImplAuto1>();
    if_auto::<ImplAuto2>();
    if_auto::<ImplAuto3>();
}
