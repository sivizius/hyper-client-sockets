#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod utils;

#[cfg(any(feature = "firecracker", feature = "unix"))]
use std::path::Path;

#[cfg(feature = "vsock")]
use vsock::VsockAddr;

#[cfg(any(feature = "firecracker", feature = "unix", feature = "vsock"))]
use std::{future::Future, io::Result};

#[cfg(any(feature = "firecracker", feature = "unix", feature = "vsock"))]
use hyper::rt::{Read, Write};

#[cfg(feature = "async-io-backend")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-io-backend")))]
pub mod async_io;

#[cfg(feature = "tokio-backend")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-backend")))]
pub mod tokio;

#[cfg(feature = "hyper-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "hyper-util")))]
pub mod connector;

#[cfg(feature = "hyper-util")]
#[cfg_attr(docsrs, doc(cfg(feature = "hyper-util")))]
pub mod uri;

/// A [`Backend`] is a runtime- and reactor-agnostic way to use hyper client-side with various types of sockets.
pub trait Backend: Clone {
    /// An IO object representing a connected Firecracker socket (a specialized Unix Domain socket).
    #[cfg(feature = "firecracker")]
    #[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
    type FirecrackerIo: Read + Write + Send + Unpin;

    /// An IO object representing a connected Unix Domain socket.
    #[cfg(feature = "unix")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unix")))]
    type UnixIo: Read + Write + Send + Unpin;

    /// An IO object representing a connected VSOCK socket.
    #[cfg(feature = "vsock")]
    #[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
    type VsockIo: Read + Write + Send + Unpin;

    /// Connect to a Firecracker socket at the given [`Path`],
    /// establishing a tunnel to the given guest VSOCK port.
    #[cfg(feature = "firecracker")]
    #[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
    fn connect_to_firecracker_socket(
        host_socket_path: &Path,
        guest_port: u32,
    ) -> impl Future<Output = Result<Self::FirecrackerIo>> + Send;

    /// Connect to a Unix Domain socket at the given [`Path`].
    #[cfg(feature = "unix")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unix")))]
    fn connect_to_unix_socket(socket_path: &Path) -> impl Future<Output = Result<Self::UnixIo>> + Send;

    /// Connect to a VSOCK socket at the given address.
    #[cfg(feature = "vsock")]
    #[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
    fn connect_to_vsock_socket(addr: VsockAddr) -> impl Future<Output = Result<Self::VsockIo>> + Send;
}
