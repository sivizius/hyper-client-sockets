#[cfg(test)]
mod tests;

use std::{
    ffi::OsString,
    io::Result as IoResult,
    os::unix::{ffi::OsStringExt as _, net::SocketAddr as UnixSocketAddr},
    path::{Path, PathBuf},
};

#[cfg(target_os = "android")]
use std::os::android::net::SocketAddrExt as _;

#[cfg(target_os = "linux")]
use std::os::linux::net::SocketAddrExt as _;

use hex::{encode, FromHex};
use http::uri::{InvalidUri, Uri};

use super::io_input_err;

/// An extension trait for hyper URI that allows constructing a hex-encoded Firecracker socket URI.
pub trait FirecrackerUri {
    /// Create a new Firecracker URI with the given host socket path, guest port and in-socket URL.
    fn firecracker<P: AsRef<Path>, S: AsRef<str>>(
        host_socket_path: P,
        guest_port: u32,
        url: S,
    ) -> Result<Uri, InvalidUri>;

    /// Create a new Firecracker URI with the given host socket address, guest port and in-socket URL.
    fn firecracker_addr<S: AsRef<str>>(
        host_socket_addr: &UnixSocketAddr,
        guest_port: u32,
        url: S,
    ) -> Result<Uri, InvalidUri>;

    /// Deconstruct this Firecracker URI into its host socket path and guest port.
    fn parse_firecracker(&self) -> IoResult<(PathBuf, u32)>;

    /// Deconstruct this Firecracker URI into its host socket address and guest port.
    fn parse_firecracker_addr(&self) -> IoResult<(UnixSocketAddr, u32)> {
        let (path, port) = self.parse_firecracker()?;

        #[cfg(any(target_os = "android", target_os = "linux"))]
        let from_abstract_name = |err| {
            let octets = path.as_os_str().as_encoded_bytes();
            match octets.split_first() {
                Some((0, name)) => UnixSocketAddr::from_abstract_name(name),
                _ => Err(err),
            }
        };

        #[cfg(not(any(target_os = "android", target_os = "linux")))]
        let from_abstract_name = |err| Err(err);

        UnixSocketAddr::from_pathname(&path)
            .or_else(from_abstract_name)
            .map(|address| (address, port))
    }
}

fn from_octets<S>(prefix: &str, octets: &[u8], guest_port: u32, url: S) -> Result<Uri, InvalidUri>
where
    S: AsRef<str>,
{
    let host = encode(octets);
    let guest_port = encode(guest_port.to_string());
    let authority = format!("{prefix}{host}{:02x}{guest_port}", b':');
    let path_and_query = url.as_ref().trim_start_matches('/');
    let uri_str = format!("fc://{authority}/{path_and_query}");
    uri_str.parse()
}

impl FirecrackerUri for Uri {
    fn firecracker<P, S>(host_socket_path: P, guest_port: u32, url: S) -> Result<Uri, InvalidUri>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let octets = host_socket_path.as_ref().as_os_str().as_encoded_bytes();
        from_octets("", octets, guest_port, url)
    }

    fn firecracker_addr<S>(host_socket_addr: &UnixSocketAddr, guest_port: u32, url: S) -> Result<Uri, InvalidUri>
    where
        S: AsRef<str>,
    {
        let (prefix, octets) = match host_socket_addr.as_pathname() {
            Some(host_socket_path) => ("", host_socket_path.as_os_str().as_encoded_bytes()),

            None => {
                #[cfg(any(target_os = "android", target_os = "linux"))]
                let octets = host_socket_addr.as_abstract_name().unwrap_or_default();

                #[cfg(not(any(target_os = "android", target_os = "linux")))]
                let octets = &[];

                // Unnamed Unix Domain sockets are encoded as `00`:
                ("00", octets)
            }
        };
        from_octets(prefix, octets, guest_port, url)
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
