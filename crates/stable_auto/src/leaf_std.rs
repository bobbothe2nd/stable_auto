use crate::{AllImpl, Auto};

use std::{
    backtrace::{Backtrace, BacktraceStatus},
    os,
    io,
    fs,
    sync::{Barrier, BarrierWaitResult, Condvar, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard, Mutex, MutexGuard, Once, TryLockResult, WaitTimeoutResult, PoisonError, mpsc},
    net::{AddrParseError, Incoming, IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, SocketAddrV6, TcpListener, TcpStream, UdpSocket},
    time::{Instant, SystemTime, SystemTimeError},
    thread::{LocalKey, Result as ThrRes, Builder as ThrBuild, AccessError, Thread, ThreadId, JoinHandle, Scope, ScopedJoinHandle},
    alloc::System,
};

impl_auto_leaf!(
    Backtrace BacktraceStatus

    Instant SystemTime SystemTimeError

    ThrBuild AccessError Thread ThreadId Scope<'_, '_>
);

impl_auto_leaf!(
    <T> LocalKey<T>
    <T> ThrRes<T>
    <T> JoinHandle<T>
    <T> ScopedJoinHandle<'_, T>
);

#[cfg(once_cell)]
use std::sync::OnceLock;

#[cfg(once_poison)]
use std::sync::OnceState;

#[cfg(lazy_cell)]
use std::sync::LazyLock;
