use crate::{AllImpl, Auto};

use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    rc::{self, Rc},
    string::String,
    sync::{self, Arc},
    vec::{Drain, IntoIter as VecIter, Splice, Vec},
    collections::{
        binary_heap::{BinaryHeap, Drain as HeapDrain, IntoIter as HeapIntoIter, Iter as HeapIter, PeekMut},
        btree_map::{BTreeMap, Entry, ExtractIf as MapExt, VacantEntry, OccupiedEntry, IntoIter as MapIntoIter, IntoKeys, IntoValues, Iter as MapIter, IterMut as MapIterMut, Keys, Values, ValuesMut, Range, RangeMut},
        btree_set::{BTreeSet, Difference, Range as SetRange, Intersection, IntoIter as SetIntoIter, Iter as SetIter, Union, SymmetricDifference},
        linked_list::{LinkedList, IntoIter as LinkedIntoIter, Iter as LinkedIter, IterMut as LinkedIterMut},
        vec_deque::{VecDeque, Drain as DeqDrain, IntoIter as DeqIntoIter, Iter as DeqIter, IterMut as DeqIterMut},
    },
};

impl_auto_leaf!(String);
impl_auto_leaf!(
    <T: ?Sized> Box<T>
    <T: ?Sized> Rc<T>
    <T: ?Sized> Arc<T>
    <T: ?Sized> rc::Weak<T>
    <T: ?Sized> sync::Weak<T>

    <T> Vec<T>
    <T> Drain<'_, T>
    <T> VecIter<T>

    <T> VecDeque<T>
    <T> DeqDrain<'_, T>
    <T> DeqIter<'_, T>
    <T> DeqIterMut<'_, T>
    <T> DeqIntoIter<T>

    <T> BinaryHeap<T>
    <T> HeapDrain<'_, T>
    <T> HeapIter<'_, T>
    <T> HeapIntoIter<T>

    <T> LinkedList<T>
    <T> LinkedIter<'_, T>
    <T> LinkedIterMut<'_, T>
    <T> LinkedIntoIter<T>

    <T> BTreeSet<T>
    <T> SetRange<'_, T>
    <T> SetIter<'_, T>
    <T> SetIntoIter<T>
    <T> Union<'_, T>
    <T> Difference<'_, T>
    <T> SymmetricDifference<'_, T>
    <T> Intersection<'_, T>

    <K, V> BTreeMap<K, V>
    <K, V> Entry<'_, K, V>
    <K, V, R, F> MapExt<'_, K, V, R, F>
    <K, V> VacantEntry<'_, K, V>
    <K, V> OccupiedEntry<'_, K, V>
    <K, V> MapIter<'_, K, V>
    <K, V> MapIterMut<'_, K, V>
    <K, V> MapIntoIter<K, V>
    <K, V> IntoKeys<K, V>
    <K, V> IntoValues<K, V>
    <K, V> Keys<'_, K, V>
    <K, V> Values<'_, K, V>
    <K, V> ValuesMut<'_, K, V>
    <K, V> Range<'_, K, V>
    <K, V> RangeMut<'_, K, V>
);

unsafe impl<I: Iterator> Auto for Splice<'_, I> {
    type Fields = AllImpl<Self>;
}
unsafe impl<T: Ord> Auto for PeekMut<'_, T> {
    type Fields = AllImpl<Self>;
}
unsafe impl<T: ?Sized + ToOwned> Auto for Cow<'_, T> {
    type Fields = AllImpl<Self>;
}

#[cfg(try_reserve)]
impl_auto_leaf!(alloc::collections::TryReserveError);

#[cfg(extract_if)]
impl_auto_leaf!(
    <T, F> alloc::vec::ExtractIf<'_, T, F>
    <T, F> alloc::collections::linked_list::ExtractIf<'_, T, F>
);
#[cfg(btree_extract_if)]
impl_auto_leaf!(
    <T, R, F> alloc::collections::btree_set::ExtractIf<'_, T, R, F>
);

#[cfg(boxed_arr_iter)]
unsafe impl<T, const N: usize> Auto for alloc::boxed::BoxedArrayIntoIter<T, N> {
    type Fields = AllImpl<Self>;
}

#[cfg(cstr)]
use alloc::ffi::{CString, NulError, FromVecWithNulError, IntoStringError};
#[cfg(cstr)]
impl_auto_leaf!(CString NulError FromVecWithNulError IntoStringError);
