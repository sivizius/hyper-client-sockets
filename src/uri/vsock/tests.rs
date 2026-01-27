use hyper::Uri;
use nix::sys::socket::VsockAddr;

use super::VsockUri as _;

#[test]
fn decode() {
    // TODO: Randomise:
    let cid = 10;
    let port = 20;
    let path_and_query = "/route";

    let uri = {
        let formatted = format!("{cid}.{port}");
        let uri_str = format!("vsock://{}{path_and_query}", hex::encode(formatted));
        uri_str.parse::<Uri>().unwrap()
    };

    let expected = VsockAddr::new(cid, port);

    assert_eq!(expected, uri.parse_vsock().unwrap());
}

#[test]
fn encode() {
    // TODO: Randomise:
    let cid = 10;
    let port = 20;
    let path_and_query = "/route";

    let address = VsockAddr::new(cid, port);

    let expected = {
        let formatted = format!("{cid}.{port}");
        let uri_str = format!("vsock://{}{path_and_query}", hex::encode(formatted));
        uri_str.parse::<Uri>().unwrap()
    };

    assert_eq!(expected, Uri::vsock(address, path_and_query).unwrap());
}
