// A set of definitions useful for bare metal platforms, where the LwIP stack is compiled with the "lwip_"-prefixed function variants
// The ESP-IDF SDK of Espressif is one such example

use libc::*;

extern "C" {
    fn lwip_accept(
        s: libc::c_int,
        addr: *mut libc::sockaddr,
        addrlen: *mut libc::socklen_t,
    ) -> libc::c_int;
    fn lwip_bind(
        s: libc::c_int,
        name: *const libc::sockaddr,
        namelen: libc::socklen_t,
    ) -> libc::c_int;
    fn lwip_shutdown(s: libc::c_int, how: libc::c_int) -> libc::c_int;

    fn lwip_getpeername(
        s: libc::c_int,
        name: *const libc::sockaddr,
        namelen: libc::socklen_t,
    ) -> libc::c_int;
    fn lwip_getsockname(
        s: libc::c_int,
        name: *const libc::sockaddr,
        namelen: libc::socklen_t,
    ) -> libc::c_int;

    fn lwip_setsockopt(
        s: libc::c_int,
        level: libc::c_int,
        optname: libc::c_int,
        opval: *const libc::c_void,
        optlen: libc::socklen_t,
    ) -> libc::c_int;
    fn lwip_getsockopt(
        s: libc::c_int,
        level: libc::c_int,
        optname: libc::c_int,
        opval: *mut libc::c_void,
        optlen: *mut libc::socklen_t,
    ) -> libc::c_int;

    fn lwip_close(s: libc::c_int) -> libc::c_int;

    fn lwip_connect(
        s: libc::c_int,
        name: *const libc::sockaddr,
        namelen: libc::socklen_t,
    ) -> libc::c_int;
    fn lwip_listen(s: libc::c_int, backlog: libc::c_int) -> libc::c_int;

    fn lwip_recvmsg(
        sockfd: libc::c_int,
        msg: *mut libc::msghdr,
        flags: libc::c_int,
    ) -> libc::size_t;
    fn lwip_recv(
        s: libc::c_int,
        mem: *mut libc::c_void,
        len: libc::size_t,
        flags: libc::c_int,
    ) -> libc::size_t;
    fn lwip_recvfrom(
        s: libc::c_int,
        mem: *mut libc::c_void,
        len: libc::size_t,
        flags: libc::c_int,
        from: *mut libc::sockaddr,
        fromlen: *mut libc::socklen_t,
    ) -> libc::size_t;

    fn lwip_send(
        s: libc::c_int,
        dataptr: *const libc::c_void,
        size: libc::size_t,
        flags: libc::c_int,
    ) -> libc::size_t;
    fn lwip_sendmsg(
        s: libc::c_int,
        message: *const libc::msghdr,
        flags: libc::c_int,
    ) -> libc::size_t;
    fn lwip_sendto(
        s: libc::c_int,
        dataptr: *const libc::c_void,
        size: libc::size_t,
        flags: libc::c_int,
        to: *const libc::sockaddr,
        tolen: libc::socklen_t,
    ) -> libc::size_t;

    fn lwip_socket(domain: libc::c_int, typest: libc::c_int, protocol: libc::c_int) -> libc::c_int;
    fn lwip_select(
        maxfdp1: libc::c_int,
        readset: *mut libc::fd_set,
        writeset: *mut libc::fd_set,
        exceptset: *mut libc::fd_set,
        timeout: *mut libc::timeval,
    ) -> libc::c_int;
    fn lwip_ioctl(s: libc::c_int, cmd: libc::c_long, argp: *mut libc::c_void) -> libc::c_int;

    fn lwip_gethostbyname_r(
        name: *const libc::c_char,
        ret: *mut libc::hostent,
        buf: *mut libc::c_char,
        buflen: libc::size_t,
        result: *mut *mut libc::hostent,
        h_errnop: *mut libc::c_int,
    ) -> libc::c_int;
    fn lwip_gethostbyname(name: *const libc::c_char) -> *mut libc::hostent;

    fn lwip_freeaddrinfo(ai: *mut libc::addrinfo) -> libc::c_void;
    fn lwip_getaddrinfo(
        nodename: *const libc::c_char,
        servname: *const libc::c_char,
        hints: *const libc::addrinfo,
        res: *mut *mut libc::addrinfo,
    ) -> libc::c_int;
}

#[no_mangle]
pub extern "C" fn accept(
    s: libc::c_int,
    addr: *mut libc::sockaddr,
    addrlen: *mut libc::socklen_t,
) -> libc::c_int {
    unsafe { lwip_accept(s, addr, addrlen) }
}
#[no_mangle]
pub extern "C" fn bind(
    s: libc::c_int,
    name: *const libc::sockaddr,
    namelen: libc::socklen_t,
) -> libc::c_int {
    unsafe { lwip_bind(s, name, namelen) }
}
#[no_mangle]
pub extern "C" fn shutdown(s: libc::c_int, how: libc::c_int) -> libc::c_int {
    unsafe { lwip_shutdown(s, how) }
}

#[no_mangle]
pub extern "C" fn getpeername(
    s: libc::c_int,
    name: *const libc::sockaddr,
    namelen: libc::socklen_t,
) -> libc::c_int {
    unsafe { lwip_getpeername(s, name, namelen) }
}
#[no_mangle]
pub extern "C" fn getsockname(
    s: libc::c_int,
    name: *const libc::sockaddr,
    namelen: libc::socklen_t,
) -> libc::c_int {
    unsafe { lwip_getsockname(s, name, namelen) }
}

#[no_mangle]
pub extern "C" fn setsockopt(
    s: libc::c_int,
    level: libc::c_int,
    optname: libc::c_int,
    opval: *const libc::c_void,
    optlen: libc::socklen_t,
) -> libc::c_int {
    unsafe { lwip_setsockopt(s, level, optname, opval, optlen) }
}
#[no_mangle]
pub extern "C" fn getsockopt(
    s: libc::c_int,
    level: libc::c_int,
    optname: libc::c_int,
    opval: *mut libc::c_void,
    optlen: *mut libc::socklen_t,
) -> libc::c_int {
    unsafe { lwip_getsockopt(s, level, optname, opval, optlen) }
}

#[no_mangle]
pub extern "C" fn closesocket(s: libc::c_int) -> libc::c_int {
    unsafe { lwip_close(s) }
}

#[no_mangle]
pub extern "C" fn connect(
    s: libc::c_int,
    name: *const libc::sockaddr,
    namelen: libc::socklen_t,
) -> libc::c_int {
    unsafe { lwip_connect(s, name, namelen) }
}
#[no_mangle]
pub extern "C" fn listen(s: libc::c_int, backlog: libc::c_int) -> libc::c_int {
    unsafe { lwip_listen(s, backlog) }
}

#[no_mangle]
pub extern "C" fn recvmsg(
    sockfd: libc::c_int,
    msg: *mut libc::msghdr,
    flags: libc::c_int,
) -> libc::size_t {
    unsafe { lwip_recvmsg(sockfd, msg, flags) }
}
#[no_mangle]
pub extern "C" fn recv(
    s: libc::c_int,
    mem: *mut libc::c_void,
    len: libc::size_t,
    flags: libc::c_int,
) -> libc::size_t {
    unsafe { lwip_recv(s, mem, len, flags) }
}
#[no_mangle]
pub extern "C" fn recvfrom(
    s: libc::c_int,
    mem: *mut libc::c_void,
    len: libc::size_t,
    flags: libc::c_int,
    from: *mut libc::sockaddr,
    fromlen: *mut libc::socklen_t,
) -> libc::size_t {
    unsafe { lwip_recvfrom(s, mem, len, flags, from, fromlen) }
}

#[no_mangle]
pub extern "C" fn send(
    s: libc::c_int,
    dataptr: *const libc::c_void,
    size: libc::size_t,
    flags: libc::c_int,
) -> libc::size_t {
    unsafe { lwip_send(s, dataptr, size, flags) }
}
#[no_mangle]
pub extern "C" fn sendmsg(
    s: libc::c_int,
    message: *const libc::msghdr,
    flags: libc::c_int,
) -> libc::size_t {
    unsafe { lwip_sendmsg(s, message, flags) }
}
#[no_mangle]
pub extern "C" fn sendto(
    s: libc::c_int,
    dataptr: *const libc::c_void,
    size: libc::size_t,
    flags: libc::c_int,
    to: *const libc::sockaddr,
    tolen: libc::socklen_t,
) -> libc::size_t {
    unsafe { lwip_sendto(s, dataptr, size, flags, to, tolen) }
}

#[no_mangle]
pub extern "C" fn socket(
    domain: libc::c_int,
    typest: libc::c_int,
    protocol: libc::c_int,
) -> libc::c_int {
    unsafe { lwip_socket(domain, typest, protocol) }
}
// Defined by ESP-IDF #[no_mangle] pub extern "C" fn select(maxfdp1: libc::c_int, readset: *mut libc::fd_set, writeset: *mut libc::fd_set, exceptset: *mut libc::fd_set, timeout: *mut libc::timeval) -> libc::c_int {unsafe {lwip_select(maxfdp1, readset, writeset, exceptset, timeout)}}
#[no_mangle]
pub extern "C" fn ioctlsocket(
    s: libc::c_int,
    cmd: libc::c_long,
    argp: *mut libc::c_void,
) -> libc::c_int {
    unsafe { lwip_ioctl(s, cmd, argp) }
}

#[no_mangle]
pub extern "C" fn gethostbyname_r(
    name: *const libc::c_char,
    ret: *mut libc::hostent,
    buf: *mut libc::c_char,
    buflen: libc::size_t,
    result: *mut *mut libc::hostent,
    h_errnop: *mut libc::c_int,
) -> libc::c_int {
    unsafe { lwip_gethostbyname_r(name, ret, buf, buflen, result, h_errnop) }
}
#[no_mangle]
pub extern "C" fn gethostbyname(name: *const libc::c_char) -> *mut libc::hostent {
    unsafe { lwip_gethostbyname(name) }
}

#[no_mangle]
pub extern "C" fn freeaddrinfo(ai: *mut libc::addrinfo) -> libc::c_void {
    unsafe { lwip_freeaddrinfo(ai) }
}
#[no_mangle]
pub extern "C" fn getaddrinfo(
    nodename: *const libc::c_char,
    servname: *const libc::c_char,
    hints: *const libc::addrinfo,
    res: *mut *mut libc::addrinfo,
) -> libc::c_int {
    unsafe { lwip_getaddrinfo(nodename, servname, hints, res) }
}
