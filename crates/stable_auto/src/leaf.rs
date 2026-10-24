use crate::{Auto, AllImpl};

use core::{
    cell::{BorrowError, BorrowMutError, Cell, Ref, RefCell, RefMut, UnsafeCell},
    any::TypeId,
    ascii::EscapeDefault,
    str::{
        Bytes, CharIndices, Chars, MatchIndices, RMatchIndices, RMatches, RSplit, RSplitN, RSplitTerminator,
        Utf8Chunk, Utf8Chunks, Utf8Error, EncodeUtf16, EscapeUnicode, Lines, Split, SplitAsciiWhitespace, SplitN, SplitTerminator,
        SplitWhitespace, EscapeDebug as StrDebug, EscapeDefault as StrDefault,
    },
    array::{IntoIter, TryFromSliceError},
    char::{EscapeDefault as CharDefault, EscapeDebug as CharDebug, EscapeUnicode as CharUnicode, ToLowercase, ToUppercase, DecodeUtf16, DecodeUtf16Error, CharTryFromError, TryFromCharError},
    convert::Infallible,
    marker::{PhantomPinned, PhantomData},
    num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, Wrapping},
    mem::{Discriminant, ManuallyDrop, MaybeUninit},
    ptr::NonNull,
    cmp::{Ordering, Reverse},
    ops::{Bound, ControlFlow, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    range::{
        legacy::{Range as LegacyRange, RangeFrom as LegacyRangeFrom, RangeInclusive as LegacyRangeInclusive, RangeToInclusive as LegacyRangeToInclusive},
        Range as Range2, RangeFrom as RangeFrom2, RangeFromIter, RangeFull as RangeFull2, RangeInclusive as RangeInclusive2, RangeInclusiveIter, RangeIter, RangeTo as RangeTo2, RangeToInclusive as RangeToInclusive2,
    },
    future::{Pending, PollFn, Ready},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    hash::BuildHasherDefault,
    time::{Duration, TryFromFloatSecsError},
    slice,
    fmt,
    iter,
};

impl_auto_leaf!(
    f32 f64

    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize

    NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize
    NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize

    DecodeUtf16Error EscapeDefault
    str Bytes<'_> Chars<'_> CharIndices<'_> EscapeUnicode<'_> StrDebug<'_> StrDefault<'_> EncodeUtf16<'_> Utf8Chunks<'_> Utf8Error Lines<'_> SplitAsciiWhitespace<'_> SplitWhitespace<'_> Utf8Chunk<'_>
    char CharDebug CharDefault CharUnicode ToLowercase ToUppercase CharTryFromError TryFromCharError

    TypeId Infallible PhantomPinned

    BorrowError BorrowMutError

    core::alloc::Layout core::alloc::LayoutError
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

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m128, __m128d, __m128i,
    __m256, __m256d, __m256i,
};
#[cfg(target_arch = "x86_64")]
impl_auto_leaf!(
    __m128 __m128d __m128i
    __m256 __m256d __m256i
);

#[cfg(u16_simd)]
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{__m128bh, __m256bh, __m512bh};
#[cfg(u16_simd)]
#[cfg(target_arch = "x86_64")]
impl_auto_leaf!(__m128bh __m256bh __m512bh);

#[cfg(f16_simd)]
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{__m128h, __m256h, __m512h};
#[cfg(f16_simd)]
#[cfg(target_arch = "x86_64")]
impl_auto_leaf!(__m128h __m256h __m512h);

#[cfg(avx512_types)]
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{__m512, __m512d, __m512i};
#[cfg(avx512_types)]
#[cfg(target_arch = "x86_64")]
impl_auto_leaf!(__m512 __m512d __m512i);
