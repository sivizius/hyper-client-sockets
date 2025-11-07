use std::{
    io::Result as IoResult,
    path::{Path, PathBuf},
};

use hex::{encode, FromHex};
use http::uri::{InvalidUri, Uri};

use super::io_input_err;

/// An extension trait for hyper URI that allows constructing a hex-encoded Firecracker socket URI.
pub trait FirecrackerUri {
    /// Create a new Firecracker URI with the given host socket path, guest port and in-socket URL.
    fn firecracker(
        host_socket_path: impl AsRef<Path>,
        guest_port: u32,
        url: impl AsRef<str>,
    ) -> Result<Uri, InvalidUri>;

    /// Deconstruct this Firecracker URI into its host socket path and guest port.
    fn parse_firecracker(&self) -> IoResult<(PathBuf, u32)>;
}

impl FirecrackerUri for Uri {
    fn firecracker(
        host_socket_path: impl AsRef<Path>,
        guest_port: u32,
        url: impl AsRef<str>,
    ) -> Result<Uri, InvalidUri> {
        let host = host_socket_path.as_ref().to_string_lossy().to_string();
        let authority = encode(format!("{host}:{guest_port}"));
        let path_and_query = url.as_ref().trim_start_matches('/');
        let uri_str = format!("fc://{authority}/{path_and_query}");
        uri_str.parse()
    }

    fn parse_firecracker(&self) -> IoResult<(PathBuf, u32)> {
        if self.scheme_str() == Some("fc") {
            let host = self.host().ok_or_else(|| io_input_err("URI host must be present"))?;
            let hex_decoded = Vec::from_hex(host).map_err(|_| io_input_err("URI host must be hex"))?;
            let full_str = String::from_utf8_lossy(&hex_decoded).into_owned();
            let splits = full_str
                .split_once(':')
                .ok_or_else(|| io_input_err("URI host could not be split in halves with a ."))?;
            let host_socket_path = PathBuf::from(splits.0);
            let guest_port = splits
                .1
                .parse::<u32>()
                .map_err(|_| io_input_err("URI guest port could not converted to u32"))?;

            Ok((host_socket_path, guest_port))
        } else {
            Err(io_input_err("URI scheme on a Firecracker socket must be fc://"))
        }
    }
}
