use crate::{AllImpl, Auto};

use std::{
    io::{
        BufReader, BufWriter, Bytes, Chain, Cursor, Empty, Error, ErrorKind, IntoInnerError, IoSlice, IoSliceMut, LineWriter, Lines, Repeat,
        Sink, Split, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock, Take, Write,
    },
    fs::{DirBuilder, DirEntry, File, FileType, OpenOptions, Metadata, ReadDir, Permissions},
    sync::{
        Barrier, BarrierWaitResult, Condvar, RwLock, RwLockReadGuard, RwLockWriteGuard, Mutex, MutexGuard, Once, WaitTimeoutResult, PoisonError,
        mpsc::{IntoIter as MpscIntoIter, Iter as MpscIter, TryIter as MpscTryIter, TryRecvError, TrySendError, Receiver, RecvError, RecvTimeoutError, SendError, Sender, SyncSender},
    },
    net::{Incoming, Shutdown, TcpListener, TcpStream, UdpSocket},
    time::{Instant, SystemTime, SystemTimeError},
    thread::{LocalKey, Builder as ThrBuild, AccessError, Thread, ThreadId, JoinHandle},
    alloc::System,
    collections::{
        hash_map::{HashMap, DefaultHasher, Drain as MapDrain, Entry, VacantEntry, OccupiedEntry, Values, ValuesMut, Keys, IntoIter as MapIntoIter, Iter as MapIter, IterMut as MapIterMut},
        hash_set::{HashSet, Difference, Drain as SetDrain, SymmetricDifference, Intersection, IntoIter as SetIntoIter, Iter as SetIter, Union},
    },
};

impl_auto_leaf!(
    Instant SystemTime SystemTimeError

    ThrBuild AccessError Thread ThreadId

    System DefaultHasher

    Barrier BarrierWaitResult Condvar WaitTimeoutResult Once

    DirBuilder DirEntry File FileType OpenOptions Metadata ReadDir Permissions

    TcpListener TcpStream UdpSocket Incoming<'_> Shutdown

    TryRecvError RecvError RecvTimeoutError

    Empty Error ErrorKind IoSlice<'_> IoSliceMut<'_> Repeat Sink Stderr StderrLock<'_> Stdin StdinLock<'_> Stdout StdoutLock<'_>
);

impl_auto_leaf!(
    <T> LocalKey<T>
    <T> JoinHandle<T>

    <T, S> HashSet<T, S>
    <K> SetIntoIter<K>
    <K> SetIter<'_, K>
    <K> SetDrain<'_, K>
    <T, S> Difference<'_, T, S>
    <T, S> SymmetricDifference<'_, T, S>
    <T, S> Intersection<'_, T, S>
    <T, S> Union<'_, T, S>

    <K, V, S> HashMap<K, V, S>
    <K, V> MapDrain<'_, K, V>
    <K, V> MapIntoIter<K, V>
    <K, V> MapIter<'_, K, V>
    <K, V> MapIterMut<'_, K, V>
    <K, V> Entry<'_, K, V>
    <K, V> VacantEntry<'_, K, V>
    <K, V> OccupiedEntry<'_, K, V>
    <K, V> Values<'_, K, V>
    <K, V> ValuesMut<'_, K, V>
    <K, V> Keys<'_, K, V>

    <T> PoisonError<T>
    <T> Mutex<T>
    <T> MutexGuard<'_, T>
    <T> RwLock<T>
    <T> RwLockReadGuard<'_, T>
    <T> RwLockWriteGuard<'_, T>

    <T> MpscIntoIter<T>
    <T> MpscIter<'_, T>
    <T> MpscTryIter<'_, T>
    <T> Sender<T>
    <T> SyncSender<T>
    <T> SendError<T>
    <T> TrySendError<T>
    <T> Receiver<T>

    <R: ?Sized> BufReader<R>
    <R> Bytes<R>
    <R> Lines<R>
    <T, U> Chain<T, U>
    <T> Cursor<T>
    <W> IntoInnerError<W>
    <T> Take<T>
    <B> Split<B>
);

unsafe impl<R: Write + ?Sized> Auto for BufWriter<R> {
    type Fields = AllImpl<Self>;
}
unsafe impl<R: Write + ?Sized> Auto for LineWriter<R> {
    type Fields = AllImpl<Self>;
}

#[cfg(anonymous_pipe)]
impl_auto_leaf!(std::io::PipeReader std::io::PipeWriter);
#[cfg(panic_write)]
impl_auto_leaf!(std::io::WriterPanicked);

#[cfg(hash_extract_if)]
impl_auto_leaf!(
    <K, F> std::collections::hash_set::ExtractIf<'_, K, F>
    <K, V, F> std::collections::hash_map::ExtractIf<'_, K, V, F>
);

#[cfg(into_valkey)]
impl_auto_leaf!(
    <K, V> std::collections::hash_map::IntoValues<K, V>
    <K, V> std::collections::hash_map::IntoKeys<K, V>
);

#[cfg(scoped_thread)]
impl_auto_leaf!(std::thread::Scope<'_, '_>);
#[cfg(scoped_thread)]
impl_auto_leaf!(<T> std::thread::ScopedJoinHandle<'_, T>);

#[cfg(backtrace)]
impl_auto_leaf!(std::backtrace::Backtrace std::backtrace::BacktraceStatus);

#[cfg(file_times)]
impl_auto_leaf!(std::fs::FileTimes);

#[cfg(once_cell)]
impl_auto_leaf!(<T> std::sync::OnceLock<T>);

#[cfg(once_poison)]
impl_auto_leaf!(std::sync::OnceState);

#[cfg(lazy_cell)]
impl_auto_leaf!(<T> std::sync::LazyLock<T>);
