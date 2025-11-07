use std::{
    future::Future,
    io::Error,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use http::Uri;
use tower_service::Service;

use super::ConnectableIo;
use crate::{uri::FirecrackerUri, Backend};

/// A hyper-util connector that accepts hex-encoded Firecracker URIs and uses them to connect
/// to Firecracker sockets via the given [Backend].
#[derive(Debug, Default, Clone)]
pub struct FirecrackerConnector<B: Backend> {
    marker: PhantomData<B>,
}

impl<B: Backend> FirecrackerConnector<B> {
    pub const fn new() -> Self {
        Self { marker: PhantomData }
    }
}

impl<B: Backend> Service<Uri> for FirecrackerConnector<B> {
    type Response = ConnectableIo<B::FirecrackerIo>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline(always)]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline(always)]
    fn call(&mut self, uri: Uri) -> Self::Future {
        Box::pin(async move {
            let (host_socket_path, guest_port) = uri.parse_firecracker()?;
            let io = B::connect_to_firecracker_socket(&host_socket_path, guest_port).await?;
            Ok(ConnectableIo(io))
        })
    }
}
