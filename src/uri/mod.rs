#[cfg(test)]
mod tests;

#[cfg(feature = "firecracker")]
#[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
pub mod firecracker;

#[cfg(feature = "unix")]
#[cfg_attr(docsrs, doc(cfg(feature = "unix")))]
pub mod unix;

#[cfg(feature = "vsock")]
#[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
pub mod vsock;

#[cfg(any(feature = "firecracker", feature = "unix", feature = "vsock"))]
use std::io::{Error, ErrorKind};

#[cfg(feature = "firecracker")]
#[cfg_attr(docsrs, doc(cfg(feature = "firecracker")))]
pub use self::firecracker::FirecrackerUri;

#[cfg(feature = "unix")]
#[cfg_attr(docsrs, doc(cfg(feature = "unix")))]
pub use self::unix::UnixUri;

#[cfg(feature = "vsock")]
#[cfg_attr(docsrs, doc(cfg(feature = "vsock")))]
pub use self::vsock::VsockUri;

#[cfg(any(feature = "firecracker", feature = "unix", feature = "vsock"))]
#[inline(always)]
fn io_input_err(detail: &str) -> Error {
    Error::new(ErrorKind::InvalidInput, detail)
}
