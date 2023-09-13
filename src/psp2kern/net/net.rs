/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2common::net::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

extern "C" {
    pub fn ksceNetSocket(
        name: *const crate::ctypes::c_char,
        domain: crate::ctypes::c_int,
        type_: crate::ctypes::c_int,
        protocol: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetAccept(
        s: crate::ctypes::c_int,
        addr: *mut SceNetSockaddr,
        addrlen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetBind(
        s: crate::ctypes::c_int,
        addr: *const SceNetSockaddr,
        addrlen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetConnect(
        s: crate::ctypes::c_int,
        name: *const SceNetSockaddr,
        namelen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetListen(
        s: crate::ctypes::c_int,
        backlog: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetRecvfrom(
        s: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        flags: crate::ctypes::c_int,
        from: *mut SceNetSockaddr,
        fromlen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetSendto(
        s: crate::ctypes::c_int,
        msg: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        flags: crate::ctypes::c_int,
        to: *const SceNetSockaddr,
        tolen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetSetsockopt(
        s: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
        optname: crate::ctypes::c_int,
        optval: *const crate::ctypes::c_void,
        optlen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceNetClose(s: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
