use std::{
    fs::File,
    io::{ErrorKind, Read as _, Result, Write as _},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    os::fd::{AsRawFd as _, FromRawFd as _, IntoRawFd as _},
    pin::Pin,
    task::{Context, Poll},
};

use async_io::Async;
use hyper::rt::{Read, ReadBufCursor, Write};
use vsock::VsockAddr;

use crate::utils::{
    hyper_util_connection_default,
    vsock::{check_connection, raw_connect, try_advance_cursor, try_poll_write},
};

pub type AsyncVsockIoInner = Async<File>;

#[derive(Debug)]
pub struct AsyncVsockIo(pub AsyncVsockIoInner);

impl AsyncVsockIo {
    pub(super) async fn connect(addr: VsockAddr) -> Result<Self> {
        let socket = raw_connect(addr)?;
        let async_fd = Async::new(unsafe { File::from_raw_fd(socket) })?;

        loop {
            let connection_check = async_fd.write_with(|fd| check_connection(fd.as_raw_fd()));

            break match connection_check.await {
                Ok(_) => {
                    let raw_fd = async_fd.into_inner()?.into_raw_fd();
                    let inner = unsafe { File::from_raw_fd(raw_fd) };
                    Async::new(inner).map(Self)
                }
                Err(err) => match err.kind() {
                    ErrorKind::Interrupted | ErrorKind::WouldBlock => continue,
                    _ => Err(err),
                },
            };
        }
    }

    fn try_poll_read(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
        cursor: &mut ReadBufCursor<'_>,
    ) -> Option<Poll<Result<()>>> {
        match self.0.poll_readable(context) {
            Poll::Ready(Ok(_)) => {
                // TODO: Once https://github.com/rust-lang/rust/issues/63569 is stable, use `assume_init_mut`:
                let buffer = unsafe { &mut *(cursor.as_mut() as *mut [MaybeUninit<u8>] as *mut [u8]) };
                let amount = self.0.get_ref().read(buffer);
                try_advance_cursor(cursor, amount)
            }
            other => Some(other),
        }
    }

    fn try_poll_write(self: Pin<&mut Self>, context: &mut Context<'_>, buffer: &[u8]) -> Option<Poll<Result<usize>>> {
        match self.0.poll_writable(context) {
            Poll::Ready(Ok(_)) => try_poll_write(self.0.get_ref().write(buffer)),
            other => Some(other.map_ok(|_| 0)),
        }
    }
}

impl Deref for AsyncVsockIo {
    type Target = AsyncVsockIoInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AsyncVsockIo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<AsyncVsockIoInner> for AsyncVsockIo {
    fn from(inner: AsyncVsockIoInner) -> Self {
        Self(inner)
    }
}

impl From<AsyncVsockIo> for AsyncVsockIoInner {
    fn from(AsyncVsockIo(inner): AsyncVsockIo) -> Self {
        inner
    }
}

impl Read for AsyncVsockIo {
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

impl Write for AsyncVsockIo {
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

hyper_util_connection_default!(AsyncVsockIo);
