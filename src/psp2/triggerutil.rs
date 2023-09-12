/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::kernel::clib::*;
#[allow(unused_imports)]
use crate::psp2::rtc::*;
#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub const SCE_TRIGGER_UTIL_VERSION: u32 = 52428800;
pub mod SceTriggerUtilDays {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_TRIGGER_UTIL_SUNDAY: Type = 1;
    pub const SCE_TRIGGER_UTIL_MONDAY: Type = 2;
    pub const SCE_TRIGGER_UTIL_TUESDAY: Type = 4;
    pub const SCE_TRIGGER_UTIL_WEDNESDAY: Type = 8;
    pub const SCE_TRIGGER_UTIL_THURSDAY: Type = 16;
    pub const SCE_TRIGGER_UTIL_FRIDAY: Type = 32;
    pub const SCE_TRIGGER_UTIL_SATURDAY: Type = 64;
}
pub mod SceTriggerUtilErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_TRIGGER_UTIL_ERROR_BUSY: Type = 2148546048;
    pub const SCE_TRIGGER_UTIL_ERROR_NOT_FOUND_USER: Type = 2148546065;
    pub const SCE_TRIGGER_UTIL_ERROR_NOT_FOUND_SYSTEM: Type = 2148546068;
    pub const SCE_TRIGGER_UTIL_ERROR_NOT_REGISTERED: Type = 2148546081;
    pub const SCE_TRIGGER_UTIL_ERROR_EVENT_TYPE_MISMATCH: Type = 2148546084;
    pub const SCE_TRIGGER_UTIL_ERROR_INVALID_ARG: Type = 2148546144;
}
#[repr(C)]
pub struct SceTriggerUtilEventParamDaily {
    pub ver: SceUInt32,
    pub extraParam1: SceInt16,
    pub extraParam2: SceInt16,
    pub triggerTime: SceInt32,
    pub repeatDays: SceUInt16,
    pub reserved: [SceChar8; 64usize],
}
#[repr(C)]
pub struct SceTriggerUtilEventParamOneTime {
    pub ver: SceUInt32,
    pub triggerTime: SceRtcTick,
    pub extraParam1: SceUInt8,
    pub extraParam2: SceUInt8,
    pub reserved: [SceChar8; 68usize],
}
#[repr(C)]
pub struct SceTriggerUtilUserAppInfo {
    pub name: [SceWChar16; 52usize],
    pub iconPath: [SceChar8; 1024usize],
    pub unk: crate::ctypes::c_short,
}
#[repr(C)]
pub struct SceTriggerUtilSystemAppInfo {
    pub name: [SceWChar16; 256usize],
    pub iconPath: [SceChar8; 1024usize],
    pub reserved: [crate::ctypes::c_char; 2usize],
}
extern "C" {
    pub fn sceTriggerUtilRegisterDailyEvent(
        titleid: *const crate::ctypes::c_char,
        param: *const SceTriggerUtilEventParamDaily,
        eventId: crate::ctypes::c_int,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceTriggerUtilRegisterOneTimeEvent(
        titleid: *const crate::ctypes::c_char,
        param: *const SceTriggerUtilEventParamOneTime,
        eventId: crate::ctypes::c_int,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceTriggerUtilUnregisterDailyEvent(
        eventId: crate::ctypes::c_int,
        a2: crate::ctypes::c_int,
        a3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceTriggerUtilUnregisterOneTimeEvent(
        eventId: crate::ctypes::c_int,
        a2: crate::ctypes::c_int,
        a3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceTriggerUtilGetAutoStartStatus(
        status: *mut crate::ctypes::c_int,
        a2: crate::ctypes::c_int,
        a3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceTriggerUtilGetOneTimeEventInfo(
        eventId: crate::ctypes::c_int,
        triggerTime: *mut SceRtcTick,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceTriggerUtilGetDailyEventInfo(
        eventId: crate::ctypes::c_int,
        param: *mut SceTriggerUtilEventParamDaily,
        a5: crate::ctypes::c_int,
        a6: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceTriggerUtilGetUserAppInfo(
        titleid: *const crate::ctypes::c_char,
        appInfo: *mut SceTriggerUtilUserAppInfo,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceTriggerUtilGetRegisteredUserTitleIdList(
        titleIdBuffer: *mut crate::ctypes::c_char,
        numOfIds: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceTriggerUtilGetSystemAppInfo(
        titleid: *const crate::ctypes::c_char,
        appInfo: *mut SceTriggerUtilSystemAppInfo,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceTriggerUtilGetRegisteredSystemTitleIdList(
        buffer: *mut crate::ctypes::c_char,
        numOfIds: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
