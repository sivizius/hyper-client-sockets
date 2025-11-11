use std::{
    io::Result,
    ops::{Deref, DerefMut},
    os::unix::net::UnixStream,
    path::Path,
};

use async_io::Async;
use smol_hyper::rt::FuturesIo;

use crate::utils::hyper_io_by_deref;

pub type AsyncUnixIoInner = FuturesIo<Async<UnixStream>>;

#[derive(Debug)]
pub struct AsyncUnixIo(pub AsyncUnixIoInner);

impl AsyncUnixIo {
    pub(super) async fn connect<P>(socket_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Async::<UnixStream>::connect(socket_path)
            .await
            .map(FuturesIo::new)
            .map(Self)
    }
}

impl Deref for AsyncUnixIo {
    type Target = AsyncUnixIoInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AsyncUnixIo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<AsyncUnixIoInner> for AsyncUnixIo {
    fn from(inner: AsyncUnixIoInner) -> Self {
        Self(inner)
    }
}

impl From<AsyncUnixIo> for AsyncUnixIoInner {
    fn from(AsyncUnixIo(inner): AsyncUnixIo) -> Self {
        inner
    }
}

hyper_io_by_deref!(AsyncUnixIo);
