/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2common::kernel::threadmgr::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

extern "C" {
    pub fn ksceKernelGetSemaInfo(
        semaid: SceUID,
        info: *mut SceKernelSemaInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelCreateSema(
        name: *const crate::ctypes::c_char,
        attr: SceUInt,
        initVal: crate::ctypes::c_int,
        maxVal: crate::ctypes::c_int,
        option: *mut SceKernelSemaOptParam,
    ) -> SceUID;
}
extern "C" {
    pub fn ksceKernelDeleteSema(semaid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelSignalSema(
        semaid: SceUID,
        signal: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelWaitSema(
        semaid: SceUID,
        signal: crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelPollSema(semaid: SceUID, signal: crate::ctypes::c_int)
        -> crate::ctypes::c_int;
}
