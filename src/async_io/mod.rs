#[cfg(feature = "firecracker")]
#[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
pub mod firecracker;

#[cfg(feature = "unix")]
#[cfg_attr(docsrs, doc(cfg(feature = "unix")))]
pub mod unix;

#[cfg(feature = "vsock")]
#[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
pub mod vsock;

#[cfg(any(feature = "firecracker", feature = "unix", feature = "vsock"))]
use std::io::Result;

#[cfg(any(feature = "firecracker", feature = "unix"))]
use std::path::Path;

#[cfg(feature = "vsock")]
use ::vsock::VsockAddr;

#[cfg(feature = "firecracker")]
#[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
pub use self::firecracker::{AsyncFirecrackerIo, AsyncFirecrackerIoInner};

#[cfg(feature = "unix")]
#[cfg_attr(docsrs, doc(cfg(feature = "unix")))]
pub use self::unix::{AsyncUnixIo, AsyncUnixIoInner};

#[cfg(feature = "vsock")]
#[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
pub use self::vsock::{AsyncVsockIo, AsyncVsockIoInner};

use crate::Backend;

/// [`Backend`] for hyper-client-sockets that is implemented via the async-io crate's reactor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AsyncIoBackend;

impl Backend for AsyncIoBackend {
    #[cfg(feature = "firecracker")]
    #[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
    type FirecrackerIo = AsyncFirecrackerIo;

    #[cfg(feature = "unix")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unix")))]
    type UnixIo = AsyncUnixIo;

    #[cfg(feature = "vsock")]
    #[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
    type VsockIo = AsyncVsockIo;

    #[cfg(feature = "firecracker")]
    #[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
    async fn connect_to_firecracker_socket(host_socket_path: &Path, guest_port: u32) -> Result<Self::FirecrackerIo> {
        Self::FirecrackerIo::connect(host_socket_path, guest_port).await
    }

    #[cfg(feature = "unix")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unix")))]
    async fn connect_to_unix_socket(socket_path: &Path) -> Result<Self::UnixIo> {
        Self::UnixIo::connect(socket_path).await
    }

    #[cfg(feature = "vsock")]
    #[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
    async fn connect_to_vsock_socket(addr: VsockAddr) -> Result<Self::VsockIo> {
        Self::VsockIo::connect(addr).await
    }
}
