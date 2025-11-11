use std::{fmt::Debug, os::unix::net::SocketAddr as UnixSocketAddr, path::PathBuf};

#[cfg(target_os = "android")]
use std::os::android::net::SocketAddrExt as _;

#[cfg(target_os = "linux")]
use std::os::linux::net::SocketAddrExt as _;

use super::FirecrackerUri as _;
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
    let port = 1000;
    let path_and_query = "/route";

    let uri = {
        let formatted = format!("{abstract_name}:{port}");
        let uri_str = format!("fc://00{}{path_and_query}", hex::encode(formatted));
        uri_str.parse::<Uri>().unwrap()
    };

    let expected = {
        let address = UnixSocketAddr::from_abstract_name(abstract_name).unwrap();
        (address, port)
    };

    assert_debug_eq(expected, uri.parse_firecracker_addr().unwrap());
}

#[test]
fn decode_abstract_name_path() {
    // TODO: Randomise:
    let abstract_name = "\0abstract";
    let port = 1000;
    let path_and_query = "/route";

    let uri = {
        let formatted = format!("{abstract_name}:{port}");
        let uri_str = format!("fc://{}{path_and_query}", hex::encode(formatted));
        uri_str.parse::<Uri>().unwrap()
    };

    let expected = {
        let path = PathBuf::from(abstract_name);
        (path, port)
    };

    assert_eq!(expected, uri.parse_firecracker().unwrap());
}

#[test]
fn decode_pathname_address() {
    // TODO: Randomise:
    let path = "/tmp/socket.sock";
    let port = 1000;
    let path_and_query = "/route";

    let uri = {
        let formatted = format!("{path}:{port}");
        let uri_str = format!("fc://{}{path_and_query}", hex::encode(formatted));
        uri_str.parse::<Uri>().unwrap()
    };

    let expected = {
        let address = UnixSocketAddr::from_pathname(path).unwrap();
        (address, port)
    };

    assert_debug_eq(expected, uri.parse_firecracker_addr().unwrap());
}

#[test]
fn decode_pathname_path() {
    // TODO: Randomise:
    let path = "/tmp/socket.sock";
    let port = 1000;
    let path_and_query = "/route";

    let uri = {
        let formatted = format!("{path}:{port}");
        let uri_str = format!("fc://{}{path_and_query}", hex::encode(formatted));
        uri_str.parse::<Uri>().unwrap()
    };

    let expected = {
        let path = PathBuf::from(path);
        (path, port)
    };

    assert_eq!(expected, uri.parse_firecracker().unwrap());
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn encode_abstract_name_as_address() {
    // TODO: Randomise:
    let abstract_name = "abstract";
    let port = 1000;
    let path_and_query = "/route";

    let address = UnixSocketAddr::from_abstract_name(abstract_name).unwrap();

    let expected = {
        let formatted = format!("{abstract_name}:{port}");
        let uri_str = format!("fc://00{}{path_and_query}", hex::encode(formatted));
        uri_str.parse::<Uri>().unwrap()
    };

    assert_eq!(expected, Uri::firecracker_addr(&address, port, path_and_query).unwrap());
}

#[test]
fn encode_abstract_name_as_path() {
    // TODO: Randomise:
    let abstract_name = "\0abstract";
    let port = 1000;
    let path_and_query = "/route";

    let expected = {
        let formatted = format!("{abstract_name}:{port}");
        let uri_str = format!("fc://{}{path_and_query}", hex::encode(formatted));
        uri_str.parse::<Uri>().unwrap()
    };

    assert_eq!(expected, Uri::firecracker(abstract_name, port, path_and_query).unwrap());
}

#[test]
fn encode_pathname_as_address() {
    // TODO: Randomise:
    let path = "/tmp/socket.sock";
    let port = 1000;
    let path_and_query = "/route";

    let address = UnixSocketAddr::from_pathname(path).unwrap();

    let expected = {
        let formatted = format!("{path}:{port}");
        let uri_str = format!("fc://{}{path_and_query}", hex::encode(formatted));
        uri_str.parse::<Uri>().unwrap()
    };

    assert_eq!(expected, Uri::firecracker_addr(&address, port, path_and_query).unwrap());
}

#[test]
fn encode_pathname_as_path() {
    // TODO: Randomise:
    let path = "/tmp/socket.sock";
    let port = 1000;
    let path_and_query = "/route";

    let expected = {
        let formatted = format!("{path}:{port}");
        let uri_str = format!("fc://{}{path_and_query}", hex::encode(formatted));
        uri_str.parse::<Uri>().unwrap()
    };

    assert_eq!(expected, Uri::firecracker(path, port, path_and_query).unwrap());
}
