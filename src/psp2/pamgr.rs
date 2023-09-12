/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPaTraceBufferParam {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPaGpuSampledData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPaGpuTraceParam {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPaCounterTraceParam {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPaArmTraceParam {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sceKernelPaGetTraceBufferSize(type_: SceUInt32) -> SceSize;
}
extern "C" {
    pub fn sceKernelPaGetIoBaseAddress() -> SceUInt32;
}
extern "C" {
    pub fn sceKernelPaGetTimebaseFrequency() -> SceUInt32;
}
extern "C" {
    pub fn sceKernelPaGetTraceBufferStatus() -> SceUInt32;
}
extern "C" {
    pub fn sceKernelPaGetWritePointer() -> SceUInt32;
}
extern "C" {
    pub fn sceKernelPaGetTimebaseValue() -> SceUInt64;
}
extern "C" {
    pub fn _sceKernelPaAddArmTraceByKey(
        key: crate::ctypes::c_int,
        param: *const SceKernelPaArmTraceParam,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn _sceKernelPaAddCounterTraceByKey(
        key: crate::ctypes::c_int,
        param: *const SceKernelPaCounterTraceParam,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn _sceKernelPaAddGpuTraceByKey(
        key: crate::ctypes::c_int,
        param: *const SceKernelPaGpuTraceParam,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn _sceKernelPaGetGpuSampledData(
        data: *mut SceKernelPaGpuSampledData,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn _sceKernelPaSetupTraceBufferByKey(
        key: crate::ctypes::c_int,
        param: *const SceKernelPaTraceBufferParam,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPaInsertBookmark(
        fifo: SceUInt32,
        channel: SceUInt32,
        data: SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPaRegister() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPaRemoveArmTraceByKey(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPaRemoveCounterTraceByKey(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPaRemoveGpuTraceByKey(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPaSetBookmarkChannelEnableByKey(
        key: crate::ctypes::c_int,
        fifo: SceUInt32,
        mask: SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPaStartByKey(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPaStopByKey(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPaUnregister(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPerfArmPmonClose() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPerfArmPmonOpen() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPerfArmPmonReset(threadId: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPerfArmPmonSelectEvent(
        threadId: SceUID,
        counter: SceUInt32,
        eventCode: SceUInt8,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPerfArmPmonSetCounterValue(
        threadId: SceUID,
        counter: SceUInt32,
        value: SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPerfArmPmonStart(threadId: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelPerfArmPmonStop(threadId: SceUID) -> crate::ctypes::c_int;
}
