use std::{
    io::Result,
    ops::{Deref, DerefMut},
    path::Path,
};

use hyper_util::rt::TokioIo;
use tokio::net::UnixStream;

use crate::utils::hyper_io_by_deref;

pub type TokioUnixIoInner = TokioIo<UnixStream>;

#[derive(Debug)]
pub struct TokioUnixIo(pub TokioUnixIoInner);

impl TokioUnixIo {
    pub(super) async fn connect<P>(socket_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        UnixStream::connect(socket_path).await.map(TokioIo::new).map(Self)
    }
}

impl Deref for TokioUnixIo {
    type Target = TokioUnixIoInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TokioUnixIo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TokioUnixIoInner> for TokioUnixIo {
    fn from(inner: TokioUnixIoInner) -> Self {
        Self(inner)
    }
}

impl From<TokioUnixIo> for TokioUnixIoInner {
    fn from(TokioUnixIo(inner): TokioUnixIo) -> Self {
        inner
    }
}

hyper_io_by_deref!(TokioUnixIo);
