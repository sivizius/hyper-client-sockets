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

/// An extension trait for a URI that allows constructing a hex-encoded Unix Domain socket URI.
pub trait UnixUri {
    /// Create a new Unix URI with the given socket path and in-socket URI.
    fn unix<P: AsRef<Path>, S: AsRef<str>>(socket_path: P, url: S) -> Result<Uri, InvalidUri>;

    /// Create a new Unix URI with the given socket address and in-socket URI.
    fn unix_addr<S: AsRef<str>>(socket_addr: &UnixSocketAddr, url: S) -> Result<Uri, InvalidUri>;

    /// Try to deconstruct this Unix URI's socket path.
    fn parse_unix(&self) -> IoResult<PathBuf>;

    /// Try to deconstruct this Unix URI's socket address.
    fn parse_unix_addr(&self) -> IoResult<UnixSocketAddr> {
        let path = self.parse_unix()?;

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

        UnixSocketAddr::from_pathname(&path).or_else(from_abstract_name)
    }
}

fn from_octets<S>(prefix: &str, octets: &[u8], url: S) -> Result<Uri, InvalidUri>
where
    S: AsRef<str>,
{
    let authority = encode(octets);
    let path_and_query = url.as_ref().trim_start_matches('/');
    let uri_str = format!("unix://{prefix}{authority}/{path_and_query}");
    uri_str.parse()
}

impl UnixUri for Uri {
    fn unix<P, S>(socket_path: P, url: S) -> Result<Uri, InvalidUri>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let octets = socket_path.as_ref().as_os_str().as_encoded_bytes();
        from_octets("", octets, url)
    }

    fn unix_addr<S>(socket_addr: &UnixSocketAddr, url: S) -> Result<Uri, InvalidUri>
    where
        S: AsRef<str>,
    {
        let (prefix, octets) = match socket_addr.as_pathname() {
            Some(socket_path) => ("", socket_path.as_os_str().as_encoded_bytes()),

            None => {
                #[cfg(any(target_os = "android", target_os = "linux"))]
                let octets = socket_addr.as_abstract_name().unwrap_or_default();

                #[cfg(not(any(target_os = "android", target_os = "linux")))]
                let octets = &[];

                // Unnamed Unix Domain sockets are encoded as `00`:
                ("00", octets)
            }
        };
        from_octets(prefix, octets, url)
    }

    fn parse_unix(&self) -> IoResult<PathBuf> {
        if self.scheme_str() == Some("unix") {
            match self.host() {
                Some(host) => {
                    let octets =
                        Vec::from_hex(host).map_err(|_| io_input_err("URI host must be hexadecimal encoded"))?;
                    Ok(OsString::from_vec(octets).into())
                }
                None => Err(io_input_err("URI host must be present")),
            }
        } else {
            Err(io_input_err("URI scheme on a Unix Domain socket must be unix://"))
        }
    }
}
