#[cfg(feature = "firecracker")]
#[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
pub mod firecracker;

#[cfg(feature = "vsock")]
#[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
pub mod vsock;

#[allow(unused)]
macro_rules! hyper_io_by_deref {
    ($ty:ty) => {
        const _: () = {
            use std::{
                io::{IoSlice, Result},
                pin::Pin,
                task::{Context, Poll},
            };

            use hyper::rt::{Read, ReadBufCursor, Write};

            impl Read for $ty {
                #[inline(always)]
                fn poll_read(
                    self: Pin<&mut Self>,
                    context: &mut Context<'_>,
                    cursor: ReadBufCursor<'_>,
                ) -> Poll<Result<()>> {
                    Pin::new(&mut self.get_mut().0).poll_read(context, cursor)
                }
            }

            impl Write for $ty {
                #[inline(always)]
                fn poll_write(self: Pin<&mut Self>, context: &mut Context<'_>, buffer: &[u8]) -> Poll<Result<usize>> {
                    Pin::new(&mut self.get_mut().0).poll_write(context, buffer)
                }

                #[inline(always)]
                fn poll_flush(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Result<()>> {
                    Pin::new(&mut self.get_mut().0).poll_flush(context)
                }

                #[inline(always)]
                fn poll_shutdown(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Result<()>> {
                    Pin::new(&mut self.get_mut().0).poll_shutdown(context)
                }

                #[inline(always)]
                fn is_write_vectored(&self) -> bool {
                    self.0.is_write_vectored()
                }

                #[inline(always)]
                fn poll_write_vectored(
                    self: Pin<&mut Self>,
                    context: &mut Context<'_>,
                    bufs: &[IoSlice<'_>],
                ) -> Poll<Result<usize>> {
                    Pin::new(&mut self.get_mut().0).poll_write_vectored(context, bufs)
                }
            }
        };
    };
}

#[allow(unused)]
macro_rules! hyper_util_connection_by_deref {
    ($ty:ty) => {
        const _: () = {
            use std::ops::Deref as _;

            use hyper_util::client::legacy::connect::{Connected, Connection};

            impl Connection for $ty {
                fn connected(&self) -> Connected {
                    self.deref().connected()
                }
            }
        };
    };
}

#[allow(unused)]
macro_rules! hyper_util_connection_default {
    ($ty:ty) => {
        const _: () = {
            use hyper_util::client::legacy::connect::{Connected, Connection};

            impl Connection for $ty {
                fn connected(&self) -> Connected {
                    Connected::new()
                }
            }
        };
    };
}

#[allow(unused)]
pub(crate) use {hyper_io_by_deref, hyper_util_connection_by_deref, hyper_util_connection_default};
