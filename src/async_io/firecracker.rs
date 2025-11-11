use std::{
    io::Result,
    ops::{Deref, DerefMut},
    os::unix::net::UnixStream,
    path::Path,
};

use async_io::Async;
use futures_lite::{io::BufReader, AsyncBufReadExt as _, AsyncWriteExt as _, StreamExt as _};
use smol_hyper::rt::FuturesIo;

use crate::utils::{
    firecracker::{format_request, parse_connection_response},
    hyper_io_by_deref,
};

pub type AsyncFirecrackerIoInner = FuturesIo<Async<UnixStream>>;

#[derive(Debug)]
pub struct AsyncFirecrackerIo(pub AsyncFirecrackerIoInner);

impl AsyncFirecrackerIo {
    pub(super) async fn connect<P>(host_socket_path: P, guest_port: u32) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut stream = Async::<UnixStream>::connect(host_socket_path).await?;
        stream.write_all(format_request(guest_port).as_bytes()).await?;
        let response = BufReader::new(&mut stream).lines().next().await.transpose();
        parse_connection_response(stream, response)
            .map(FuturesIo::new)
            .map(Self)
    }
}

impl Deref for AsyncFirecrackerIo {
    type Target = AsyncFirecrackerIoInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AsyncFirecrackerIo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<AsyncFirecrackerIoInner> for AsyncFirecrackerIo {
    fn from(inner: AsyncFirecrackerIoInner) -> Self {
        Self(inner)
    }
}

impl From<AsyncFirecrackerIo> for AsyncFirecrackerIoInner {
    fn from(AsyncFirecrackerIo(inner): AsyncFirecrackerIo) -> Self {
        inner
    }
}

hyper_io_by_deref!(AsyncFirecrackerIo);
