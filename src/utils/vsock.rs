use std::{
    io::{Error, ErrorKind, Result},
    os::fd::RawFd,
    task::Poll,
};

use hyper::rt::ReadBufCursor;
use vsock::VsockAddr;

pub fn check_connection(raw_fd: RawFd) -> Result<()> {
    let mut sock_err: libc::c_int = 0;
    let mut sock_err_len: libc::socklen_t = size_of::<libc::c_int>() as libc::socklen_t;
    let err = unsafe {
        libc::getsockopt(
            raw_fd,
            libc::SOL_SOCKET,
            libc::SO_ERROR,
            &mut sock_err as *mut _ as *mut libc::c_void,
            &mut sock_err_len as *mut libc::socklen_t,
        )
    };

    if err < 0 {
        Err(Error::last_os_error())
    } else if sock_err != 0 {
        Err(Error::from_raw_os_error(sock_err))
    } else {
        Ok(())
    }
}

pub fn raw_connect(addr: VsockAddr) -> Result<RawFd> {
    let socket = unsafe { libc::socket(libc::AF_VSOCK, libc::SOCK_STREAM, 0) };
    if socket < 0 {
        Err(Error::last_os_error())
    } else if unsafe { libc::fcntl(socket, libc::F_SETFL, libc::O_NONBLOCK | libc::O_CLOEXEC) } < 0 {
        let _ = unsafe { libc::close(socket) };
        Err(Error::last_os_error())
    } else {
        let addr = &addr as *const _ as *const libc::sockaddr;
        let addrlen = size_of::<libc::sockaddr_vm>() as libc::socklen_t;
        if unsafe { libc::connect(socket, addr, addrlen) } >= 0 {
            Ok(socket)
        } else {
            let err = Error::last_os_error();
            match err.raw_os_error() {
                Some(libc::EINPROGRESS) => Ok(socket),
                Some(_os_err) => {
                    let _ = unsafe { libc::close(socket) };
                    Err(err)
                }
                None => unreachable!(),
            }
        }
    }
}

pub fn try_advance_cursor(cursor: &mut ReadBufCursor<'_>, amount: Result<usize>) -> Option<Poll<Result<()>>> {
    match amount {
        Ok(amount) => {
            unsafe {
                cursor.advance(amount);
            }
            Some(Poll::Ready(Ok(())))
        }
        Err(err) => match err.kind() {
            ErrorKind::Interrupted | ErrorKind::WouldBlock => None,
            _ => Some(Poll::Ready(Err(err))),
        },
    }
}

pub fn try_poll_write(amount: Result<usize>) -> Option<Poll<Result<usize>>> {
    match amount {
        Err(err) if matches!(err.kind(), ErrorKind::Interrupted | ErrorKind::WouldBlock) => None,
        other => Some(Poll::Ready(other)),
    }
}
