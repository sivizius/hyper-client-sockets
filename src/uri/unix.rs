use std::{
    io::Result as IoResult,
    path::{Path, PathBuf},
};

use hex::{encode, FromHex};
use http::uri::{InvalidUri, Uri};

use super::io_input_err;

/// An extension trait for a URI that allows constructing a hex-encoded Unix Domain socket URI.
pub trait UnixUri {
    /// Create a new Unix URI with the given socket path and in-socket URI.
    fn unix(socket_path: impl AsRef<Path>, url: impl AsRef<str>) -> Result<Uri, InvalidUri>;

    /// Try to deconstruct this Unix URI's socket path.
    fn parse_unix(&self) -> IoResult<PathBuf>;
}

impl UnixUri for Uri {
    fn unix(socket_path: impl AsRef<Path>, url: impl AsRef<str>) -> Result<Uri, InvalidUri> {
        let authority = encode(socket_path.as_ref().to_string_lossy().to_string());
        let path_and_query = url.as_ref().trim_start_matches('/');
        let uri_str = format!("unix://{authority}/{path_and_query}");
        uri_str.parse()
    }

    fn parse_unix(&self) -> IoResult<PathBuf> {
        if self.scheme_str() == Some("unix") {
            match self.host() {
                Some(host) => {
                    let bytes = Vec::from_hex(host).map_err(|_| io_input_err("URI host must be hex"))?;
                    Ok(PathBuf::from(String::from_utf8_lossy(&bytes).into_owned()))
                }
                None => Err(io_input_err("URI host must be present")),
            }
        } else {
            Err(io_input_err("URI scheme on a Unix Domain socket must be unix://"))
        }
    }
}
