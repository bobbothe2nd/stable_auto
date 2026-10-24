use crate::{AllImpl, Auto};

use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    rc::Rc,
    string::String,
    sync::Arc,
    vec::{Drain, ExtractIf, IntoIter as VecIter, Splice, Vec},
    collections::{
        TryReserveError,
        binary_heap::{BinaryHeap, Drain as HeapDrain, IntoIter as HeapIter, Iter, PeekMut},
        btree_map,
        btree_set::{BTreeSet, Difference, ExtractIf as SetExt, Range, Intersection, IntoIter as SetIntoIter, Iter as SetIter, Union, SymmetricDifference},
        linked_list::{ExtractIf as ExtractDrain, IntoIter as ExtractIntoIter, Iter as ExtractIter, IterMut as ExtractIterMut},
        vec_deque::{Drain as DeqDrain, IntoIter as DeqIntoIter, Iter as DeqIter, IterMut as DeqIterMut},
    },
};

impl_auto_leaf!(String);
impl_auto_leaf!(
    <T> Vec<T>
    <T> Drain<'_, T>
    <T, F> ExtractIf<'_, T, F>
    <T> VecIter<T>
    <T: ?Sized> Box<T>
    <T: ?Sized> Rc<T>
    <T: ?Sized> Arc<T>
);

unsafe impl<I: Iterator> Auto for Splice<'_, I> {
    type Fields = AllImpl<Self>;
}
unsafe impl<T: ?Sized + ToOwned> Auto for Cow<'_, T> {
    type Fields = AllImpl<Self>;
}

#[cfg(boxed_arr_iter)]
unsafe impl<T, const N: usize> Auto for alloc::boxed::BoxedArrayIntoIter<T, N> {
    type Fields = AllImpl<Self>;
}

impl_auto_leaf!(
    <T: ?Sized> alloc::rc::Weak<T>
    <T: ?Sized> alloc::sync::Weak<T>
);

#[cfg(cstr)]
use alloc::ffi::{CString, NulError, FromVecWithNulError, IntoStringError};
#[cfg(cstr)]
impl_auto_leaf!(CString NulError FromVecWithNulError IntoStringError);
