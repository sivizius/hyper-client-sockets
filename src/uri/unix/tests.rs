use std::{fmt::Debug, os::unix::net::SocketAddr as UnixSocketAddr, path::PathBuf};

#[cfg(target_os = "android")]
use std::os::android::net::SocketAddrExt as _;

#[cfg(target_os = "linux")]
use std::os::linux::net::SocketAddrExt as _;

use super::UnixUri as _;
use hyper::Uri;

fn assert_debug_eq<E, T>(expected: E, value: T)
where
    E: Debug,
    T: Debug,
{
    assert_eq!(format!("{expected:?}"), format!("{value:?}"))
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn decode_abstract_name_address() {
    // TODO: Randomise:
    let abstract_name = "abstract";
    let path_and_query = "/route";

    let uri = {
        let uri_str = format!("unix://00{}{path_and_query}", hex::encode(abstract_name));
        uri_str.parse::<Uri>().unwrap()
    };

    let expected = UnixSocketAddr::from_abstract_name(abstract_name).unwrap();

    assert_debug_eq(expected, uri.parse_unix_addr().unwrap());
}

#[test]
fn decode_abstract_name_path() {
    // TODO: Randomise:
    let abstract_name = "\0abstract";
    let path_and_query = "/route";

    let uri = {
        let uri_str = format!("unix://{}{path_and_query}", hex::encode(abstract_name));
        uri_str.parse::<Uri>().unwrap()
    };

    let expected = PathBuf::from(abstract_name);

    assert_eq!(expected, uri.parse_unix().unwrap());
}

#[test]
fn decode_pathname_address() {
    // TODO: Randomise:
    let path = "/tmp/socket.sock";
    let path_and_query = "/route";

    let uri = {
        let uri_str = format!("unix://{}{path_and_query}", hex::encode(path));
        uri_str.parse::<Uri>().unwrap()
    };

    let expected = UnixSocketAddr::from_pathname(path).unwrap();

    assert_debug_eq(expected, uri.parse_unix_addr().unwrap());
}

#[test]
fn decode_pathname_path() {
    // TODO: Randomise:
    let path = "/tmp/socket.sock";
    let path_and_query = "/route";

    let uri = {
        let uri_str = format!("unix://{}{path_and_query}", hex::encode(path));
        uri_str.parse::<Uri>().unwrap()
    };

    let expected = PathBuf::from(path);

    assert_eq!(expected, uri.parse_unix().unwrap());
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn encode_abstract_name_as_address() {
    // TODO: Randomise:
    let abstract_name = "abstract";
    let path_and_query = "/route";

    let address = UnixSocketAddr::from_abstract_name(abstract_name).unwrap();

    let expected = {
        let uri_str = format!("unix://00{}{path_and_query}", hex::encode(abstract_name));
        uri_str.parse::<Uri>().unwrap()
    };

    assert_eq!(expected, Uri::unix_addr(&address, path_and_query).unwrap());
}

#[test]
fn encode_abstract_name_as_path() {
    // TODO: Randomise:
    let abstract_name = "\0abstract";
    let path_and_query = "/route";

    let expected = {
        let uri_str = format!("unix://{}{path_and_query}", hex::encode(abstract_name));
        uri_str.parse::<Uri>().unwrap()
    };

    assert_eq!(expected, Uri::unix(abstract_name, path_and_query).unwrap());
}

#[test]
fn encode_pathname_as_address() {
    // TODO: Randomise:
    let path = "/tmp/socket.sock";
    let path_and_query = "/route";

    let address = UnixSocketAddr::from_pathname(path).unwrap();

    let expected = {
        let uri_str = format!("unix://{}{path_and_query}", hex::encode(path));
        uri_str.parse::<Uri>().unwrap()
    };

    assert_eq!(expected, Uri::unix_addr(&address, path_and_query).unwrap());
}

#[test]
fn encode_pathname_as_path() {
    // TODO: Randomise:
    let path = "/tmp/socket.sock";
    let path_and_query = "/route";

    let expected = {
        let uri_str = format!("unix://{}{path_and_query}", hex::encode(path));
        uri_str.parse::<Uri>().unwrap()
    };

    assert_eq!(expected, Uri::unix(path, path_and_query).unwrap());
}
