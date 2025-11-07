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
use crate::{uri::UnixUri, Backend};

/// A hyper-util connector that accepts hex-encoded Unix URIs and
/// uses them to connect to Unix Domain sockets via the given [`Backend`].
#[derive(Debug, Default, Clone)]
pub struct UnixConnector<B: Backend> {
    marker: PhantomData<B>,
}

impl<B: Backend> UnixConnector<B> {
    pub const fn new() -> Self {
        Self { marker: PhantomData }
    }
}

impl<B: Backend> Service<Uri> for UnixConnector<B> {
    type Response = ConnectableIo<B::UnixIo>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    #[inline(always)]
    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline(always)]
    fn call(&mut self, uri: Uri) -> Self::Future {
        Box::pin(async move {
            let socket_path = uri.parse_unix()?;
            let io = B::connect_to_unix_socket(&socket_path).await?;
            Ok(ConnectableIo(io))
        })
    }
}
