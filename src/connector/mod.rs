#[cfg(feature = "firecracker")]
#[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
pub mod firecracker;

#[cfg(feature = "unix")]
#[cfg_attr(docsrs, doc(cfg(feature = "unix")))]
pub mod unix;

#[cfg(feature = "vsock")]
#[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
pub mod vsock;

use std::{
    io::{IoSlice, Result},
    pin::Pin,
    task::{Context, Poll},
};

use hyper::rt::{Read, ReadBufCursor, Write};
use hyper_util::client::legacy::connect::{Connected, Connection};

#[cfg(feature = "firecracker")]
#[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
pub use self::firecracker::FirecrackerConnector;

#[cfg(feature = "unix")]
#[cfg_attr(docsrs, doc(cfg(feature = "unix")))]
pub use self::unix::UnixConnector;

#[cfg(feature = "vsock")]
#[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
pub use self::vsock::VsockConnector;

/// This is an internal wrapper over an IO type that implements [`Write`] and
/// [`Read`] that also implements [`Connection`] to achieve compatibility with hyper-util.
pub struct ConnectableIo<IO>(IO);

impl<IO: Write + Read + Send + Unpin> Write for ConnectableIo<IO> {
    #[inline(always)]
    fn poll_write(self: Pin<&mut Self>, ctx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        Pin::new(&mut self.get_mut().0).poll_write(ctx, buf)
    }

    #[inline(always)]
    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_flush(ctx)
    }

    #[inline(always)]
    fn poll_shutdown(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_shutdown(ctx)
    }

    #[inline(always)]
    fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    #[inline(always)]
    fn poll_write_vectored(self: Pin<&mut Self>, ctx: &mut Context<'_>, bufs: &[IoSlice<'_>]) -> Poll<Result<usize>> {
        Pin::new(&mut self.get_mut().0).poll_write_vectored(ctx, bufs)
    }
}

impl<IO: Write + Read + Send + Unpin> Read for ConnectableIo<IO> {
    #[inline(always)]
    fn poll_read(self: Pin<&mut Self>, ctx: &mut Context<'_>, buf: ReadBufCursor<'_>) -> Poll<Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_read(ctx, buf)
    }
}

impl<IO: Write + Read + Send + Unpin> Connection for ConnectableIo<IO> {
    fn connected(&self) -> Connected {
        Connected::new()
    }
}
