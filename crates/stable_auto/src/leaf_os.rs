#[cfg(target_family = "unix")]
mod unix {
    // use std::os::unix;
}

#[cfg(all(target_family = "unix", target_vendor = "apple"))]
mod darwin {
    // use std::os::darwin;
}

#[cfg(target_os = "linux")]
mod linux {
    // use std::os::linux;
}

#[cfg(all(target_family = "wasm", any(target_env = "p1", target_env = "p2", target_env = "p3")))]
mod wasi {
    // use std::os::wasi;
}

#[cfg(all(target_family = "wasm", target_env = "p2"))]
mod wasip2 {
    // use std::os::wasip2;
}

#[cfg(target_family = "windows")]
mod windows {
    use std::os::windows::{
        ffi::EncodeWide,
        io::{BorrowedHandle, BorrowedSocket, HandleOrInvalid, HandleOrNull, InvalidHandleError, NullHandleError, OwnedHandle, OwnedSocket},
    };

    impl_auto_leaf!(
        EncodeWide<'_> BorrowedHandle<'_> BorrowedSocket<'_> HandleOrInvalid HandleOrNull InvalidHandleError NullHandleError OwnedHandle OwnedSocket
    );
}

#[cfg(target_os = "aix")]
mod aix {
    // use std::os::aix;
}

#[cfg(target_os = "android")]
mod android {
    // use std::os::android;
}

#[cfg(target_os = "cygwin")]
mod cygwin {
    // use std::os::cygwin;
}

#[cfg(target_os = "dragonfly")]
mod dragonfly {
    // use std::os::dragonfly;
}

#[cfg(target_os = "emscripten")]
mod emscripten {
    // use std::os::emscripten;
}

#[cfg(target_os = "espidf")]
mod espidf {
    // use std::os::espidf;
}

#[cfg(all(target_vendor = "fortanix", target_env = "sgx"))]
mod fortanix_sgx {
    // use std::os::fortanix_sgx;
}

#[cfg(target_os = "freebsd")]
mod freebsd {
    // use std::os::freebsd;
}

#[cfg(target_os = "fuchsia")]
mod fuchsia {
    // use std::os::fuchsia;
}

#[cfg(target_os = "haiku")]
mod haiku {
    // use std::os::haiku;
}

#[cfg(target_os = "hermit")]
mod hermit {
    // use std::os::hermit;
}

#[cfg(target_os = "horizon")]
mod horizon {
    // use std::os::horizon;
}

#[cfg(target_os = "hurd")]
mod hurd {
    // use std::os::hurd;
}

#[cfg(target_os = "illumos")]
mod illumos {
    // use std::os::illumos;
}

#[cfg(target_os = "ios")]
mod ios {
    // use std::os::ios;
}

#[cfg(target_os = "l4re")]
mod l4re {
    // use std::os::l4re;
}

#[cfg(target_os = "macos")]
mod macos {
    // use std::os::macos;
}

#[cfg(target_os = "motor")]
mod motor {
    // use std::os::motor;
}

#[cfg(target_os = "netbsd")]
mod netbsd {
    // use std::os::netbsd;
}

#[cfg(any(target_os = "nto", target_os = "qnx"))]
mod nto {
    // use std::os::nto;
}

#[cfg(target_os = "nuttx")]
mod nuttx {
    // use std::os::nuttx;
}

#[cfg(target_os = "openbsd")]
mod openbsd {
    // use std::os::openbsd;
}

#[cfg(target_os = "redox")]
mod redox {
    // use std::os::redox;
}

#[cfg(target_os = "rtems")]
mod rtems {
    // use std::os::rtems;
}

#[cfg(target_os = "solaris")]
mod solaris {
    // use std::os::solaris;
}

#[cfg(target_os = "solid_asp3")]
mod solid {
    // use std::os::solid;
}

#[cfg(target_os = "trusty")]
mod trusty {
    // use std::os::trusty;
}

#[cfg(target_os = "uefi")]
mod uefi {
    // use std::os::uefi;
}

#[cfg(target_os = "vita")]
mod vita {
    // use std::os::vita;
}

#[cfg(target_os = "vxworks")]
mod vxworks {
    // use std::os::vxworks;
}

#[cfg(target_os = "xous")]
mod xous {
    // use std::os::xous;
}
