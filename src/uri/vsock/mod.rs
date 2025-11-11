#[cfg(test)]
mod tests;

use std::io::Result as IoResult;

use hex::{encode, FromHex};
use http::uri::{InvalidUri, Uri};
use vsock::VsockAddr;

use super::io_input_err;

/// An extension trait for hyper URI that allows constructing a hex-encoded VSOCK socket URI.
pub trait VsockUri {
    /// Create a new VSOCK URI with the given address and in-socket URL.
    fn vsock<S: AsRef<str>>(addr: VsockAddr, url: S) -> Result<Uri, InvalidUri>;

    /// Deconstruct this VSOCK URI into its address.
    fn parse_vsock(&self) -> IoResult<VsockAddr>;
}

impl VsockUri for Uri {
    fn vsock<S>(addr: VsockAddr, url: S) -> Result<Uri, InvalidUri>
    where
        S: AsRef<str>,
    {
        let authority = encode(format!("{}.{}", addr.cid(), addr.port()));
        let path_and_query = url.as_ref().trim_start_matches('/');
        let uri_str = format!("vsock://{authority}/{path_and_query}");
        uri_str.parse()
    }

    fn parse_vsock(&self) -> IoResult<VsockAddr> {
        if self.scheme_str() == Some("vsock") {
            match self.host() {
                Some(host) => {
                    let full_str = Vec::from_hex(host)
                        .map_err(|_| io_input_err("URI host must be hex"))
                        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())?;
                    let splits = full_str.split_once('.').ok_or_else(|| {
                        io_input_err("URI host could not be split at . into 2 slices (CID, then port)")
                    })?;
                    let cid = splits
                        .0
                        .parse::<u32>()
                        .map_err(|_| io_input_err("First split of URI (CID) can't be parsed"))?;
                    let port = splits
                        .1
                        .parse::<u32>()
                        .map_err(|_| io_input_err("Second split of URI (port) can't be parsed"))?;

                    Ok(VsockAddr::new(cid, port))
                }
                None => Err(io_input_err("URI host must be present")),
            }
        } else {
            Err(io_input_err("URI scheme on a VSOCK socket must be vsock://"))
        }
    }
}
