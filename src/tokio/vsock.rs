use std::{
    io::{Read as _, Result, Write as _},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    os::fd::{AsRawFd as _, FromRawFd as _, IntoRawFd as _, OwnedFd},
    pin::Pin,
    task::{Context, Poll},
};

use hyper::rt::{Read, ReadBufCursor, Write};
use tokio::io::unix::AsyncFd;
use vsock::{VsockAddr, VsockStream};

use crate::utils::vsock::{check_connection, raw_connect, try_advance_cursor, try_poll_write};

pub type TokioVsockIoInner = AsyncFd<VsockStream>;

/// IO object representing an active VSOCK connection controlled via a Tokio [`AsyncFd`].
/// This is internally a reimplementation of a relevant part of the tokio-vsock crate.
pub struct TokioVsockIo(TokioVsockIoInner);

impl TokioVsockIo {
    pub(super) async fn connect(addr: VsockAddr) -> Result<Self> {
        let socket = raw_connect(addr)?;
        let async_fd = AsyncFd::new(unsafe { OwnedFd::from_raw_fd(socket) })?;

        loop {
            let connection_check = {
                let mut guard = async_fd.writable().await?;
                guard.try_io(|fd| check_connection(fd.as_raw_fd()))
            };

            break match connection_check {
                Ok(Ok(_)) => {
                    let raw_fd = async_fd.into_inner().into_raw_fd();
                    let inner = unsafe { VsockStream::from_raw_fd(raw_fd) };
                    AsyncFd::new(inner).map(Self)
                }
                Ok(Err(err)) => Err(err),
                Err(_would_block) => continue,
            };
        }
    }

    fn try_poll_read(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
        cursor: &mut ReadBufCursor<'_>,
    ) -> Option<Poll<Result<()>>> {
        match self.0.poll_read_ready(context) {
            Poll::Ready(Ok(mut guard)) => {
                // TODO: Once https://github.com/rust-lang/rust/issues/63569 is stable, use `assume_init_mut`:
                let buffer = unsafe { &mut *(cursor.as_mut() as *mut [MaybeUninit<u8>] as *mut [u8]) };
                let amount = guard.try_io(|inner| inner.get_ref().read(buffer)).ok()?;
                try_advance_cursor(cursor, amount)
            }
            other => Some(other.map_ok(|_| ())),
        }
    }

    fn try_poll_write(self: Pin<&mut Self>, context: &mut Context<'_>, buffer: &[u8]) -> Option<Poll<Result<usize>>> {
        match self.0.poll_write_ready(context) {
            Poll::Ready(Ok(mut guard)) => try_poll_write(guard.try_io(|inner| inner.get_ref().write(buffer)).ok()?),
            other => Some(other.map_ok(|_| 0)),
        }
    }
}

impl Deref for TokioVsockIo {
    type Target = TokioVsockIoInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TokioVsockIo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Read for TokioVsockIo {
    #[inline(always)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        mut cursor: ReadBufCursor<'_>,
    ) -> Poll<Result<()>> {
        loop {
            if let Some(poll_result) = self.as_mut().try_poll_read(context, &mut cursor) {
                break poll_result;
            }
        }
    }
}

impl Write for TokioVsockIo {
    #[inline(always)]
    fn poll_write(mut self: Pin<&mut Self>, context: &mut Context<'_>, buffer: &[u8]) -> Poll<Result<usize>> {
        loop {
            if let Some(poll_result) = self.as_mut().try_poll_write(context, buffer) {
                break poll_result;
            }
        }
    }

    #[inline(always)]
    fn poll_flush(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    #[inline(always)]
    fn poll_shutdown(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }
}
