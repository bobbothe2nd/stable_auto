/*
use stable_auto::{Auto, auto_trait, auto_opaque, auto_leaf};

#[auto_trait]
trait Marker {}

auto_leaf! {
    impl Marker for u16 {}
    impl Marker for u32 {}
    impl Marker for usize {}
    impl Marker for PhantomMarked {}
}

#[auto_opaque]
struct PhantomNotMarked;

#[auto_opaque]
struct PhantomMarked;

#[derive(Auto)]
struct ImplAuto1(u32);

#[derive(Auto)]
struct ImplAuto2 {
    _a: u32,
    _b: usize,
}

#[derive(Auto)]
struct ImplAuto3 {
    _a: ImplAuto2,
    _b: u16,
    _c: usize,
}

fn if_auto<T: Marker>() {}

#[test]
fn auto_leaf() {
    if_auto::<u32>();
    if_auto::<PhantomMarked>();
    if_auto::<ImplAuto1>();
    if_auto::<ImplAuto2>();
    if_auto::<ImplAuto3>();
}
*/
