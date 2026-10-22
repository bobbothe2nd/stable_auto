use super::{Auto, AllImpl, PhantomData};

use core::{
    cell::{BorrowError, BorrowMutError, Cell, Ref, RefCell, RefMut, UnsafeCell},
    any::TypeId,
    ascii::EscapeDefault,
    char::{EscapeDefault as CharDefault, EscapeUnicode as CharUnicode, ToLowercase as CharLower, ToUppercase as CharUpper},
    convert::Infallible,
};

impl_auto_leaf!(
    f32 f64

    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize

    str EscapeDefault CharDefault CharUnicode CharLower CharUpper

    TypeId Infallible
);

impl_auto_leaf!(
    <T> core::num::Wrapping<T>
    <T, E> Result<T, E>
    <T> Option<T>
    <T> PhantomData<T>
    <T> UnsafeCell<T>
    <T> Cell<T>
    <T> RefCell<T>
    <T> Ref<'_, T>
    <T> RefMut<'_, T>
    <T> [T]
);

#[cfg(decode_utf16)]
use core::char::{DecodeUtf16, DecodeUtf16Error};
#[cfg(decode_utf16)]
unsafe impl<T: Iterator<Item = u16>> Auto for DecodeUtf16<T> {
    type Fields = AllImpl<Self>;
}
#[cfg(decode_utf16)]
impl_auto_leaf!(DecodeUtf16Error);

#[cfg(char_escape_debug)]
impl_auto_leaf!(core::char::EscapeDebug);
#[cfg(u8_from_char)]
impl_auto_leaf!(core::char::TryFromCharError);

#[cfg(try_from)]
use core::char::CharTryFromError;
#[cfg(try_from)]
impl_auto_leaf!(CharTryFromError);

#[cfg(pin)]
impl_auto_leaf!(core::marker::PhantomPinned);

#[cfg(layout)]
impl_auto_leaf!(core::alloc::Layout);
#[cfg(layout_error)]
impl_auto_leaf!(core::alloc::LayoutError);

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

#[cfg(nonzero)]
use core::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize};
#[cfg(nonzero)]
impl_auto_leaf!(NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize);

#[cfg(signed_nonzero)]
use core::num::{NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize};
#[cfg(signed_nonzero)]
impl_auto_leaf!(NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize);

#[cfg(once_cell)]
impl_auto_leaf!(<T> core::cell::OnceCell<T>);
#[cfg(lazy_cell)]
impl_auto_leaf!(<T> core::cell::LazyCell<T>);

#[cfg(try_borrow)]
impl_auto_leaf!(BorrowError BorrowMutError);

#[cfg(manually_drop)]
impl_auto_leaf!(<T> core::mem::ManuallyDrop<T>);
#[cfg(maybe_uninit)]
impl_auto_leaf!(<T> core::mem::MaybeUninit<T>);

#[cfg(saturating_type)]
impl_auto_leaf!(<T> core::num::Saturating<T>);

#[cfg(rust_simd)]
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m128, __m128d, __m128i,
    __m256, __m256d, __m256i,
};
#[cfg(rust_simd)]
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

#[cfg(cstr)]
impl_auto_leaf!(
    core::ffi::CStr
    core::ffi::FromBytesWithNulError
);
#[cfg(cstr_until_nul)]
impl_auto_leaf!(core::ffi::FromBytesUntilNulError);

#[cfg(feature = "alloc")]
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    rc::Rc,
    string::String,
    sync::Arc,
    vec::{Drain, ExtractIf, IntoIter, Splice, Vec},
};
#[cfg(feature = "alloc")]
impl_auto_leaf!(String);
#[cfg(feature = "alloc")]
impl_auto_leaf!(
    <T> Vec<T>
    <T> Drain<'_, T>
    <T, F> ExtractIf<'_, T, F>
    <T> IntoIter<T>
    <T: ?Sized> Box<T>
    <T: ?Sized> Rc<T>
    <T: ?Sized> Arc<T>
);

#[cfg(feature = "alloc")]
unsafe impl<I: Iterator> Auto for Splice<'_, I> {
    type Fields = AllImpl<Self>;
}
#[cfg(feature = "alloc")]
unsafe impl<T: ?Sized + ToOwned> Auto for Cow<'_, T> {
    type Fields = AllImpl<Self>;
}

#[cfg(all(feature = "alloc", boxed_arr_iter))]
unsafe impl<T, const N: usize> Auto for alloc::boxed::BoxedArrayIntoIter<T, N> {
    type Fields = AllImpl<Self>;
}

#[cfg(all(feature = "alloc", weak_ptr))]
impl_auto_leaf!(
    <T: ?Sized> alloc::rc::Weak<T>
    <T: ?Sized> alloc::sync::Weak<T>
);

#[cfg(all(feature = "alloc", cstr))]
use alloc::ffi::{CString, NulError, FromVecWithNulError, IntoStringError};
#[cfg(all(feature = "alloc", cstr))]
impl_auto_leaf!(CString NulError FromVecWithNulError IntoStringError);
