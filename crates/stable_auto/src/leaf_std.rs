use crate::{AllImpl, Auto};

use std::{
    backtrace::{Backtrace, BacktraceStatus},
    io,
    fs,
    sync,
    net,
    time::{Instant, SystemTime, SystemTimeError},
    thread::{LocalKey, Result as ThrRes, Builder as ThrBuild, AccessError, Thread, ThreadId, JoinHandle, Scope, ScopedJoinHandle},
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
