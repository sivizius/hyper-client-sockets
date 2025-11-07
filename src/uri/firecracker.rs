use std::{
    ffi::OsString,
    io::Result as IoResult,
    os::unix::ffi::OsStringExt as _,
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
        let host = encode(host_socket_path.as_ref().as_os_str().as_encoded_bytes());
        let guest_port = encode(guest_port.to_string());
        let authority = format!("{host}{:02x}{guest_port}", b':');
        let path_and_query = url.as_ref().trim_start_matches('/');
        let uri_str = format!("fc://{authority}/{path_and_query}");
        uri_str.parse()
    }

    fn parse_firecracker(&self) -> IoResult<(PathBuf, u32)> {
        if self.scheme_str() == Some("fc") {
            let host_hex = self.host().ok_or_else(|| io_input_err("URI host must be present"))?;
            let mut host_octets =
                Vec::from_hex(host_hex).map_err(|_| io_input_err("URI host must be hexadecimal encoded"))?;

            let colon_pos = host_octets
                .iter()
                .rposition(|octet| *octet == b':')
                .ok_or_else(|| io_input_err("URI host does not encode port"))?;

            let guest_port = String::from_utf8(host_octets.split_off(colon_pos))
                .map_err(|_| io_input_err("URI guest port is not valid UTF8"))?
                .split_at(1)
                .1
                .parse::<u32>()
                .map_err(|_| io_input_err("URI guest port could not be parsed"))?;

            let host_socket_path = OsString::from_vec(host_octets).into();

            Ok((host_socket_path, guest_port))
        } else {
            Err(io_input_err("URI scheme on a Firecracker socket must be fc://"))
        }
    }
}
