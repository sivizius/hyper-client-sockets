use std::path::PathBuf;

use hyper::Uri;
use vsock::VsockAddr;

use crate::uri::{FirecrackerUri, UnixUri, VsockUri};

#[test]
fn unix_uri_with_pathname_should_be_constructed_correctly() {
    let uri_str = format!("unix://{}/route", hex::encode("/tmp/socket.sock"));
    assert_eq!(
        Uri::unix("/tmp/socket.sock", "/route").unwrap(),
        uri_str.parse::<Uri>().unwrap()
    );
}

#[test]
fn unix_uri_with_abstract_name_should_be_constructed_correctly() {
    let uri_str = format!("unix://00{}/route", hex::encode("/tmp/socket.sock"));
    assert_eq!(
        Uri::unix("\0/tmp/socket.sock", "/route").unwrap(),
        uri_str.parse::<Uri>().unwrap()
    );
}

#[test]
fn unix_uri_with_pathname_should_be_deconstructed_correctly() {
    let uri = format!("unix://{}/route", hex::encode("/tmp/socket.sock"));
    assert_eq!(
        uri.parse::<Uri>().unwrap().parse_unix().unwrap(),
        PathBuf::from("/tmp/socket.sock")
    );
}

#[test]
fn unix_uri_with_abstract_name_should_be_deconstructed_correctly() {
    let uri = format!("unix://00{}/route", hex::encode("/tmp/socket.sock"));
    assert_eq!(
        uri.parse::<Uri>().unwrap().parse_unix().unwrap(),
        PathBuf::from("\0/tmp/socket.sock")
    );
}

#[test]
fn vsock_uri_should_be_constructed_correctly() {
    let uri = format!("vsock://{}/route", hex::encode("10.20"));
    assert_eq!(
        uri.parse::<Uri>().unwrap(),
        Uri::vsock(VsockAddr::new(10, 20), "/route").unwrap()
    );
}

#[test]
fn vsock_uri_should_be_deconstructed_correctly() {
    let uri = format!("vsock://{}/route", hex::encode("10.20"))
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.parse_vsock().unwrap(), VsockAddr::new(10, 20));
}

#[test]
fn firecracker_uri_with_pathname_should_be_constructed_correctly() {
    let uri_str = format!("fc://{}/route", hex::encode("/tmp/socket.sock:1000"));
    assert_eq!(
        Uri::firecracker("/tmp/socket.sock", 1000, "/route").unwrap(),
        uri_str.parse::<Uri>().unwrap()
    );
}

#[test]
fn firecracker_uri_with_abstract_name_should_be_constructed_correctly() {
    let uri_str = format!("fc://00{}/route", hex::encode("/tmp/socket.sock:1000"));
    assert_eq!(
        Uri::firecracker("\0/tmp/socket.sock", 1000, "/route").unwrap(),
        uri_str.parse::<Uri>().unwrap()
    );
}

#[test]
fn firecracker_uri_with_pathname_should_be_deconstructed_correctly() {
    let uri = format!("fc://{}/route", hex::encode("/tmp/socket.sock:1000"))
        .parse::<Uri>()
        .unwrap();
    let (socket_path, port) = uri.parse_firecracker().unwrap();
    assert_eq!(socket_path, PathBuf::from("/tmp/socket.sock"));
    assert_eq!(port, 1000);
}

#[test]
fn firecracker_uri_with_abstract_name_should_be_deconstructed_correctly() {
    let uri = format!("fc://00{}/route", hex::encode("/tmp/socket.sock:1000"))
        .parse::<Uri>()
        .unwrap();
    let (socket_path, port) = uri.parse_firecracker().unwrap();
    assert_eq!(socket_path, PathBuf::from("\0/tmp/socket.sock"));
    assert_eq!(port, 1000);
}
