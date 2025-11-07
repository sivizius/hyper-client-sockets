use std::{future::Future, marker::PhantomData, pin::Pin, task::Poll};

use http::Uri;

use super::ConnectableIo;
use crate::{uri::VsockUri, Backend};

/// A hyper-util connector that accepts hex-encoded VSOCK URIs and
/// uses them to connect to VSOCK sockets via the given [`Backend`].
#[derive(Debug, Default, Clone)]
pub struct VsockConnector<B: Backend> {
    marker: PhantomData<B>,
}

impl<B: Backend> VsockConnector<B> {
    pub const fn new() -> Self {
        Self { marker: PhantomData }
    }
}

impl<B: Backend> tower_service::Service<Uri> for VsockConnector<B> {
    type Response = ConnectableIo<B::VsockIo>;

    type Error = std::io::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline(always)]
    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline(always)]
    fn call(&mut self, uri: Uri) -> Self::Future {
        Box::pin(async move {
            let addr = uri.parse_vsock()?;
            let io = B::connect_to_vsock_socket(addr).await?;
            Ok(ConnectableIo(io))
        })
    }
}
