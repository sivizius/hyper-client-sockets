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

pub struct AsyncUnixIo(AsyncUnixIoInner);

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

hyper_io_by_deref!(AsyncUnixIo);
