/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

#[repr(C)]
pub struct SceKernelSemaOptParam {
    pub size: SceSize,
}
#[repr(C)]
pub struct SceKernelSemaInfo {
    pub size: SceSize,
    pub semaId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: SceUInt,
    pub initCount: crate::ctypes::c_int,
    pub currentCount: crate::ctypes::c_int,
    pub maxCount: crate::ctypes::c_int,
    pub numWaitThreads: crate::ctypes::c_int,
}
extern "C" {
    pub fn sceKernelCreateSema(
        name: *const crate::ctypes::c_char,
        attr: SceUInt,
        initVal: crate::ctypes::c_int,
        maxVal: crate::ctypes::c_int,
        option: *mut SceKernelSemaOptParam,
    ) -> SceUID;
}
extern "C" {
    pub fn sceKernelDeleteSema(semaid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelSignalSema(
        semaid: SceUID,
        signal: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelWaitSema(
        semaid: SceUID,
        signal: crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelWaitSemaCB(
        semaid: SceUID,
        signal: crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPollSema(semaid: SceUID, signal: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelCancelSema(
        semaid: SceUID,
        setCount: crate::ctypes::c_int,
        numWaitThreads: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelGetSemaInfo(
        semaid: SceUID,
        info: *mut SceKernelSemaInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelOpenSema(name: *const crate::ctypes::c_char) -> SceUID;
}
extern "C" {
    pub fn sceKernelCloseSema(semaid: SceUID) -> crate::ctypes::c_int;
}
