use crate::{Auto, AllImpl};

use core::{
    cell::{BorrowError, BorrowMutError, Cell, Ref, RefCell, RefMut, UnsafeCell},
    any::TypeId,
    ascii::EscapeDefault,
    str::{
        Bytes, CharIndices, Chars, MatchIndices, RMatchIndices, RMatches, RSplit, RSplitN, RSplitTerminator,
        Utf8Error, EncodeUtf16, EscapeUnicode, Lines, Split, SplitAsciiWhitespace, SplitN, SplitTerminator,
        SplitWhitespace, EscapeDebug as StrDebug, EscapeDefault as StrDefault,
    },
    array::TryFromSliceError,
    char::{EscapeDefault as CharDefault, EscapeDebug as CharDebug, EscapeUnicode as CharUnicode, ToLowercase, ToUppercase, DecodeUtf16, DecodeUtf16Error, CharTryFromError},
    convert::Infallible,
    marker::{PhantomPinned, PhantomData},
    num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, Wrapping},
    mem::{Discriminant, ManuallyDrop, MaybeUninit},
    ptr::NonNull,
    cmp::{Ordering, Reverse},
    ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    future::{Pending, Ready},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    hash::BuildHasherDefault,
    time::Duration,
    slice,
    fmt::{Alignment, Arguments, DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple, Error, Formatter},
    iter,
};

impl_auto_leaf!(
    f32 f64

    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize

    NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize
    NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize

    DecodeUtf16Error EscapeDefault
    str Bytes<'_> Chars<'_> CharIndices<'_> EscapeUnicode<'_> StrDebug<'_> StrDefault<'_> EncodeUtf16<'_> Utf8Error Lines<'_> SplitAsciiWhitespace<'_> SplitWhitespace<'_>
    char CharDebug CharDefault CharUnicode ToLowercase ToUppercase CharTryFromError

    TypeId Infallible PhantomPinned

    BorrowError BorrowMutError

    core::alloc::Layout core::alloc::LayoutError

    Ordering

    Context<'_> RawWaker RawWakerVTable Waker

    AddrParseError IpAddr Ipv4Addr Ipv6Addr SocketAddr SocketAddrV4 SocketAddrV6

    Alignment Arguments<'_> DebugList<'_, '_> DebugMap<'_, '_> DebugSet<'_, '_> DebugStruct<'_, '_> DebugTuple<'_, '_> Error Formatter<'_>

    Duration

    TryFromSliceError

    RangeFull
);

impl_auto_leaf!(
    <T> Wrapping<T>
    <T, E> Result<T, E>
    <T> Option<T>
    <T> PhantomData<T>
    <T> UnsafeCell<T>
    <T> Cell<T>
    <T> RefCell<T>
    <T> Ref<'_, T>
    <T> RefMut<'_, T>
    <T> Discriminant<T>
    <T> ManuallyDrop<T>
    <T> MaybeUninit<T>
    <T> NonNull<T>
    <T> Reverse<T>
    <T> BuildHasherDefault<T>
    <T> Pending<T>
    <T> Ready<T>
    <T> Poll<T>
    <T> Bound<T>
    <T> Range<T>
    <T> RangeTo<T>
    <T> RangeFrom<T>
    <T> RangeToInclusive<T>
    <T> RangeInclusive<T>
    <T> [T]
);

impl_auto_leaf!(
    <P: Pattern>
    MatchIndices<'_> Split<'_> SplitN<'_> SplitTerminator<'_> RMatchIndices<'_> RMatches<'_> RSplit<'_> RSplitN<'_> RSplitTerminator<'_>
);

unsafe impl<T: Iterator<Item = u16>> Auto for DecodeUtf16<T> {
    type Fields = AllImpl<Self>;
}

#[cfg(target_has_atomic = "8")]
use core::sync::atomic::{AtomicBool, AtomicI8, AtomicU8};
#[cfg(target_has_atomic = "8")]
impl_auto_leaf!(AtomicBool AtomicU8 AtomicI8);

#[cfg(target_has_atomic = "16")]
use core::sync::atomic::{AtomicI16, AtomicU16};
#[cfg(target_has_atomic = "16")]
impl_auto_leaf!(AtomicU16 AtomicI16);

#[cfg(target_has_atomic = "32")]
use core::sync::atomic::{AtomicI32, AtomicU32};
#[cfg(target_has_atomic = "32")]
impl_auto_leaf!(AtomicU32 AtomicI32);

#[cfg(target_has_atomic = "64")]
use core::sync::atomic::{AtomicI64, AtomicU64};
#[cfg(target_has_atomic = "64")]
impl_auto_leaf!(AtomicU64 AtomicI64);

#[cfg(target_has_atomic = "ptr")]
use core::sync::atomic::{AtomicIsize, AtomicUsize, AtomicPtr};
#[cfg(target_has_atomic = "ptr")]
impl_auto_leaf!(AtomicUsize AtomicIsize);
#[cfg(target_has_atomic = "ptr")]
impl_auto_leaf!(<T> AtomicPtr<T>);

#[cfg(utf8_chunks)]
impl_auto_leaf!(core::str::Utf8Chunks<'_> core::str::Utf8Chunk<'_>);

#[cfg(u8_from_char)]
impl_auto_leaf!(core::char::TryFromCharError);

#[cfg(duration_checked_float)]
impl_auto_leaf!(core::time::TryFromFloatSecsError);

#[cfg(fmt_from_fn)]
impl_auto_leaf!(<T> core::fmt::FromFn<T>);

#[cfg(poll_fn)]
impl_auto_leaf!(<T> core::future::PollFn<T>);

#[cfg(split_inclusive)]
use core::str::SplitInclusive;
#[cfg(split_inclusive)]
impl_auto_leaf!(<P: Pattern> SplitInclusive<'_>);

#[cfg(once_cell)]
impl_auto_leaf!(<T> core::cell::OnceCell<T>);
#[cfg(lazy_cell)]
impl_auto_leaf!(<T> core::cell::LazyCell<T>);

#[cfg(saturating_type)]
impl_auto_leaf!(<T> core::num::Saturating<T>);

#[cfg(cstr)]
impl_auto_leaf!(
    core::ffi::CStr
    core::ffi::FromBytesWithNulError
);
#[cfg(cstr_until_nul)]
impl_auto_leaf!(core::ffi::FromBytesUntilNulError);

#[cfg(new_range)]
use core::range::{
    Range as Range2, RangeFrom as RangeFrom2, RangeToInclusive as RangeToInclusive2, RangeFromIter, RangeIter,
};
#[cfg(new_range)]
impl_auto_leaf!(
    <T> RangeIter<T>
    <T> RangeFromIter<T>
    <T> Range2<T>
    <T> RangeFrom2<T>
    <T> RangeToInclusive2<T>
);

#[cfg(new_range_inclusive)]
use core::range::{
    RangeInclusive as RangeInclusive2, RangeInclusiveIter,
};
#[cfg(new_range_inclusive)]
impl_auto_leaf!(
    <T> RangeInclusive2<T>
    <T> RangeInclusiveIter<T>
);

#[cfg(num_buffer)]
use core::fmt::NumBuffer;
#[cfg(num_buffer)]
impl_auto_leaf!(
    NumBuffer<u8> NumBuffer<u16> NumBuffer<u32> NumBuffer<u64> NumBuffer<u128> NumBuffer<usize>
    NumBuffer<i8> NumBuffer<i16> NumBuffer<i32> NumBuffer<i64> NumBuffer<i128> NumBuffer<isize>
);

#[cfg(control_flow)]
use core::ops::ControlFlow;
#[cfg(control_flow)]
impl_auto_leaf!(
    <B, C> ControlFlow<B, C>
);

#[cfg(arr_iter)]
use core::array::IntoIter;
#[cfg(arr_iter)]
unsafe impl<T, const N: usize> Auto for IntoIter<T, N> {
    type Fields = AllImpl<Self>;
}
