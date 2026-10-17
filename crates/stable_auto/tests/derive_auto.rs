use stable_auto::{Auto, auto_trait, auto_opaque, impl_auto};

#[auto_trait]
trait Marker {}

#[auto_opaque]
struct PhantomNotMarked;

impl_auto! {
    impl Marker for u16 {}
    impl Marker for u32 {}
    impl Marker for usize {}
}

#[derive(Auto)]
struct ImplAuto1 {
    _a: u32,
}

#[derive(Auto)]
struct ImplAuto2 {
    _a: u32,
    _b: usize,
}

#[derive(Auto)]
struct ImplAuto3 {
    _a: u32,
    _b: u16,
    _c: usize,
}

fn if_auto<T: Marker>() {}

#[test]
fn impl_auto() {
    if_auto::<u32>();
    if_auto::<ImplAuto1>();
    if_auto::<ImplAuto2>();
    if_auto::<ImplAuto3>();
}
