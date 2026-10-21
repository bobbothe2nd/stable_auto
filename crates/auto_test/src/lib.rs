use stable_auto::{Auto, Leaf};

pub struct ImplAuto1;

unsafe impl Auto for ImplAuto1 {
    type Fields = Leaf<u32>;
}

pub struct ImplAuto2;

// explicit impls unsupported, must implement `Auto` instead
// impl Marker for ImplAuto2 {}

unsafe impl Auto for ImplAuto2 {
    type Fields = (Leaf<Leaf<Leaf<u16>>>, usize);
}

pub struct ImplAuto3;

unsafe impl Auto for ImplAuto3 {
    type Fields = ((ImplAuto2, u16), usize);
}

pub struct ImplAuto4;

unsafe impl Auto for ImplAuto4 {
    type Fields = ((Leaf<ImplAuto2>, Leaf<ImplAuto3>), Leaf<ImplAuto1>);
}

pub struct ImplAuto5;

unsafe impl Auto for ImplAuto5 {
    type Fields = (
        (
            (Leaf<Leaf<Leaf<ImplAuto4>>>, Leaf<ImplAuto2>),
            Leaf<ImplAuto3>,
        ),
        Leaf<ImplAuto1>,
    );
}

#[test]
fn impl_auto() {
    use auto_test1::Marker;

    fn if_auto<T: Marker>() {}

    if_auto::<u32>();
    if_auto::<ImplAuto1>();
    if_auto::<ImplAuto2>();
    if_auto::<ImplAuto3>();
    if_auto::<ImplAuto4>();
    if_auto::<ImplAuto5>();
}
