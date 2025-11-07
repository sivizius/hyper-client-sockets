use std::io::{Error, ErrorKind, Result};

pub fn format_request(guest_port: u32) -> String {
    format!("CONNECT {guest_port}\n")
}

pub fn parse_connection_response<S>(stream: S, response: Result<Option<String>>) -> Result<S> {
    match response {
        Ok(Some(line)) => {
            if line.starts_with("OK") {
                Ok(stream)
            } else {
                Err(Error::new(
                    ErrorKind::ConnectionRefused,
                    "Firecracker refused to establish a tunnel to the given guest port",
                ))
            }
        }
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            "Could not read Firecracker response",
        )),
    }
}
