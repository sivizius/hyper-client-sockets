use std::{
    io::Result,
    ops::{Deref, DerefMut},
    path::Path,
};

use hyper_util::rt::TokioIo;
use tokio::{
    io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader},
    net::UnixStream,
};

use crate::utils::{
    firecracker::{format_request, parse_connection_response},
    hyper_io_by_deref,
};

pub type TokioFirecrackerIoInner = TokioIo<UnixStream>;

pub struct TokioFirecrackerIo(TokioIo<UnixStream>);

impl TokioFirecrackerIo {
    pub(super) async fn connect<P>(host_socket_path: P, guest_port: u32) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut stream = UnixStream::connect(host_socket_path).await?;
        stream.write_all(format_request(guest_port).as_bytes()).await?;
        let response = BufReader::new(&mut stream).lines().next_line().await;

        parse_connection_response(stream, response).map(TokioIo::new).map(Self)
    }
}

impl Deref for TokioFirecrackerIo {
    type Target = TokioFirecrackerIoInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TokioFirecrackerIo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

hyper_io_by_deref!(TokioFirecrackerIo);
