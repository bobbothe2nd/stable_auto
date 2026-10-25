use crate::{AllImpl, Auto};

use std::{
    io,
    fs,
    sync::{Barrier, BarrierWaitResult, Condvar, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard, Mutex, MutexGuard, Once, TryLockResult, WaitTimeoutResult, PoisonError, mpsc},
    net::{AddrParseError, Incoming, IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, SocketAddrV6, TcpListener, TcpStream, UdpSocket},
    time::{Instant, SystemTime, SystemTimeError},
    thread::{LocalKey, Builder as ThrBuild, AccessError, Thread, ThreadId, JoinHandle},
    alloc::System,
    collections::{
        HashMap, HashSet,
        hash_map::{DefaultHasher, Drain as MapDrain, Entry, ExtractIf as MapExt, VacantEntry, Values, ValuesMut, Keys, IntoIter as MapIntoIter, IntoKeys, IntoValues, Iter as MapIter, IterMut},
        hash_set::{Difference, Drain as SetExt, ExtractIf, SymmetricDifference, Intersection, IntoIter as SetIntoIter, Iter as SetIter, Union},
    },
};

impl_auto_leaf!(
    Instant SystemTime SystemTimeError

    ThrBuild AccessError Thread ThreadId

    System
);

impl_auto_leaf!(
    <T> LocalKey<T>
    <T> JoinHandle<T>
);

#[cfg(scoped_thread)]
impl_auto_leaf!(std::thread::Scope<'_, '_>);
#[cfg(scoped_thread)]
impl_auto_leaf!(<T> std::thread::ScopedJoinHandle<'_, T>);

#[cfg(backtrace)]
impl_auto_leaf!(std::backtrace::Backtrace std::backtrace::BacktraceStatus);

#[cfg(once_cell)]
use std::sync::OnceLock;

#[cfg(once_poison)]
use std::sync::OnceState;

#[cfg(lazy_cell)]
use std::sync::LazyLock;
