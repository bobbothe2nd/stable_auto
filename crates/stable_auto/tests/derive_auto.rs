use stable_auto::{auto_leaf, auto_opaque, auto_trait, Auto};

#[auto_trait]
trait Marker {}

auto_leaf! {
    impl Marker for u16 {}
    impl Marker for u32 {}
    impl Marker for usize {}
    impl Marker for PhantomMarked {}
    impl<T> Marker for core::mem::ManuallyDrop<T> {}
    impl Marker for fn(u8) -> u16 {}
}

#[auto_opaque]
struct PhantomNotMarked;

#[derive(Auto)]
enum PhantomMarked {}

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

#[derive(Auto)]
enum EnumAuto3 {
    _A { _a: ImplAuto2, _b: u16, _c: usize },

    _B,

    _C(u32),
}

#[derive(Auto)]
union UnionAuto3 {
    _a: core::mem::ManuallyDrop<ImplAuto2>,
    _b: u16,
    #[cfg(target_arch = "x86_64")]
    _c: u32,
    #[cfg(not(target_arch = "x86_64"))]
    _c: usize,
}

#[derive(Auto)]
struct ParseFn {
    _f: fn(u8) -> u16,
    _leaf: stable_auto::Leaf<fn(u8) -> u16>,
}

fn if_auto<T: Marker>() {}

#[test]
fn auto_leaf() {
    if_auto::<u32>();
    if_auto::<PhantomMarked>();
    if_auto::<ImplAuto1>();
    if_auto::<ImplAuto2>();
    if_auto::<ImplAuto3>();
    if_auto::<EnumAuto3>();
    if_auto::<UnionAuto3>();
    if_auto::<ParseFn>();

    let _ = PhantomNotMarked;
    let _ = ImplAuto1(0).0;
}
