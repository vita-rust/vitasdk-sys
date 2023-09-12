/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceArmCpuRegisters {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
    pub cpsr: u32,
    pub fpscr: u32,
}
pub type ArmCpuRegisters = SceArmCpuRegisters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceThreadCpuRegisters {
    pub __bindgen_anon_1: SceThreadCpuRegisters__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceThreadCpuRegisters__bindgen_ty_1 {
    pub __bindgen_anon_1: SceThreadCpuRegisters__bindgen_ty_1__bindgen_ty_1,
    pub entry: [SceArmCpuRegisters; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceThreadCpuRegisters__bindgen_ty_1__bindgen_ty_1 {
    pub user: SceArmCpuRegisters,
    pub kernel: SceArmCpuRegisters,
}
pub type ThreadCpuRegisters = SceThreadCpuRegisters;
#[repr(C)]
pub struct SceKernelThreadContextInfo {
    pub process_id: ScePID,
    pub thread_id: SceUID,
}
extern "C" {
    pub fn ksceKernelGetThreadIdList(
        pid: SceUID,
        ids: *mut SceUID,
        n: crate::ctypes::c_int,
        copy_count: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetThreadCpuRegisters(
        thid: SceUID,
        registers: *mut SceThreadCpuRegisters,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetThreadContextInfo(
        pInfo: *mut SceKernelThreadContextInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelChangeThreadSuspendStatus(
        thid: SceUID,
        status: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetUserThreadId(thid: SceUID) -> SceUID;
}
extern "C" {
    pub fn ksceKernelIsThreadDebugSuspended(thid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDebugSuspendThread(
        thid: SceUID,
        status: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDebugResumeThread(
        thid: SceUID,
        status: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetThreadInfoForDebugger(
        thid: SceUID,
        a2: crate::ctypes::c_int,
        pInfo: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelGetVfpRegisterForDebugger(
        thid: SceUID,
        pVfpRegister: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
#[repr(C)]
pub struct SceKernelFaultingProcessInfo {
    pub pid: SceUID,
    pub faultingThreadId: SceUID,
}
