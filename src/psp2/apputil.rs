/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub mod SceAppUtilSaveDataRemoveMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_APPUTIL_SAVEDATA_DATA_REMOVE_MODE_DEFAULT: Type = 0;
    pub const SCE_APPUTIL_SAVEDATA_DATA_REMOVE_MODE_NO_SLOT: Type = 1;
}
pub use self::SceAppUtilSaveDataRemoveMode::Type as SceAppUtilSaveDataDataRemoveMode;
pub mod SceAppUtilSaveDataSaveMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_APPUTIL_SAVEDATA_DATA_SAVE_MODE_FILE: Type = 0;
    pub const SCE_APPUTIL_SAVEDATA_DATA_SAVE_MODE_DIRECTORY: Type = 2;
}
pub use self::SceAppUtilSaveDataSaveMode::Type as SceAppUtilSaveDataDataSaveMode;
pub mod SceAppUtilErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_APPUTIL_ERROR_PARAMETER: Type = 2148533760;
    pub const SCE_APPUTIL_ERROR_NOT_INITIALIZED: Type = 2148533761;
    pub const SCE_APPUTIL_ERROR_NO_MEMORY: Type = 2148533762;
    pub const SCE_APPUTIL_ERROR_BUSY: Type = 2148533763;
    pub const SCE_APPUTIL_ERROR_NOT_MOUNTED: Type = 2148533764;
    pub const SCE_APPUTIL_ERROR_NO_PERMISSION: Type = 2148533765;
    pub const SCE_APPUTIL_ERROR_PASSCODE_MISMATCH: Type = 2148533766;
    pub const SCE_APPUTIL_ERROR_APPEVENT_PARSE_INVALID_DATA: Type = 2148533792;
    pub const SCE_APPUTIL_ERROR_SAVEDATA_SLOT_EXISTS: Type = 2148533824;
    pub const SCE_APPUTIL_ERROR_SAVEDATA_SLOT_NOT_FOUND: Type = 2148533825;
    pub const SCE_APPUTIL_ERROR_SAVEDATA_NO_SPACE_QUOTA: Type = 2148533826;
    pub const SCE_APPUTIL_ERROR_SAVEDATA_NO_SPACE_FS: Type = 2148533827;
    pub const SCE_APPUTIL_ERROR_DRM_NO_ENTITLEMENT: Type = 2148533856;
    pub const SCE_APPUTIL_ERROR_PHOTO_DEVICE_NOT_FOUND: Type = 2148533888;
    pub const SCE_APPUTIL_ERROR_MUSIC_DEVICE_NOT_FOUND: Type = 2148533893;
    pub const SCE_APPUTIL_ERROR_MOUNT_LIMIT_OVER: Type = 2148533894;
    pub const SCE_APPUTIL_ERROR_STACKSIZE_TOO_SHORT: Type = 2148533920;
}
pub type SceAppUtilBootAttribute = crate::ctypes::c_uint;
pub type SceAppUtilAppEventType = crate::ctypes::c_uint;
pub type SceAppUtilSaveDataSlotId = crate::ctypes::c_uint;
pub type SceAppUtilSaveDataSlotStatus = crate::ctypes::c_uint;
pub type SceAppUtilAppParamId = crate::ctypes::c_uint;
pub type SceAppUtilBgdlStatusType = crate::ctypes::c_uint;
#[repr(C)]
pub struct SceAppUtilBgdlStatus {
    pub type_: SceAppUtilBgdlStatusType,
    pub addcontNumReady: SceUInt32,
    pub addcontNumNotReady: SceUInt32,
    pub licenseReady: SceUInt32,
    pub reserved: [SceChar8; 28usize],
}
#[repr(C)]
pub struct SceAppUtilInitParam {
    pub workBufSize: SceSize,
    pub reserved: [u8; 60usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilBootParam {
    pub attr: SceAppUtilBootAttribute,
    pub appVersion: crate::ctypes::c_uint,
    pub reserved: [u8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataMountPoint {
    pub data: [u8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilAppEventParam {
    pub type_: SceAppUtilAppEventType,
    pub dat: [u8; 1024usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilMountPoint {
    pub data: [i8; 16usize],
}
#[repr(C)]
pub struct SceAppUtilSaveDataSlotEmptyParam {
    pub title: *mut SceWChar16,
    pub iconPath: *mut crate::ctypes::c_char,
    pub iconBuf: *mut crate::ctypes::c_void,
    pub iconBufSize: SceSize,
    pub reserved: [u8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataSlot {
    pub id: SceAppUtilSaveDataSlotId,
    pub status: SceAppUtilSaveDataSlotStatus,
    pub userParam: crate::ctypes::c_int,
    pub emptyParam: *mut SceAppUtilSaveDataSlotEmptyParam,
}
#[repr(C)]
pub struct SceAppUtilSaveDataSlotParam {
    pub status: SceAppUtilSaveDataSlotStatus,
    pub title: [SceWChar16; 32usize],
    pub subTitle: [SceWChar16; 64usize],
    pub detail: [SceWChar16; 256usize],
    pub iconPath: [crate::ctypes::c_char; 64usize],
    pub userParam: crate::ctypes::c_int,
    pub sizeKB: SceSize,
    pub modifiedTime: SceDateTime,
    pub reserved: [u8; 48usize],
}
#[repr(C)]
pub struct SceAppUtilSaveDataSaveItem {
    pub dataPath: *const crate::ctypes::c_char,
    pub buf: *const crate::ctypes::c_void,
    pub pad: u32,
    pub offset: SceOff,
    pub mode: crate::ctypes::c_int,
    pub reserved: [u8; 36usize],
}
#[repr(C)]
pub struct SceAppUtilSaveDataFile {
    pub filePath: *const crate::ctypes::c_char,
    pub buf: *mut crate::ctypes::c_void,
    pub bufSize: SceSize,
    pub offset: SceOff,
    pub mode: crate::ctypes::c_uint,
    pub progDelta: crate::ctypes::c_uint,
    pub reserved: [u8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataFileSlot {
    pub id: crate::ctypes::c_uint,
    pub slotParam: *mut SceAppUtilSaveDataSlotParam,
    pub reserved: [u8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataRemoveItem {
    pub dataPath: *const crate::ctypes::c_char,
    pub mode: crate::ctypes::c_int,
    pub reserved: [u8; 36usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilStoreBrowseParam {
    pub type_: crate::ctypes::c_uint,
    pub id: *const crate::ctypes::c_char,
}
#[repr(C)]
pub struct SceAppUtilWebBrowserParam {
    pub str_: *const crate::ctypes::c_char,
    pub strlen: SceSize,
    pub launchMode: crate::ctypes::c_uint,
    pub reserved: crate::ctypes::c_uint,
}
extern "C" {
    pub fn sceAppUtilInit(
        initParam: *mut SceAppUtilInitParam,
        bootParam: *mut SceAppUtilBootParam,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilShutdown() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilReceiveAppEvent(
        eventParam: *mut SceAppUtilAppEventParam,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilAppEventParseLiveArea(
        eventParam: *const SceAppUtilAppEventParam,
        buffer: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilSaveDataSlotCreate(
        slotId: crate::ctypes::c_uint,
        param: *mut SceAppUtilSaveDataSlotParam,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilSaveDataSlotDelete(
        slotId: crate::ctypes::c_uint,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilSaveDataSlotSetParam(
        slotId: crate::ctypes::c_uint,
        param: *mut SceAppUtilSaveDataSlotParam,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilSaveDataSlotGetParam(
        slotId: crate::ctypes::c_uint,
        param: *mut SceAppUtilSaveDataSlotParam,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilSaveDataDataSave(
        slot: *mut SceAppUtilSaveDataFileSlot,
        files: *mut SceAppUtilSaveDataFile,
        fileNum: crate::ctypes::c_uint,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
        requiredSizeKB: *mut SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilSaveDataDataRemove(
        slot: *mut SceAppUtilSaveDataFileSlot,
        files: *mut SceAppUtilSaveDataRemoveItem,
        fileNum: crate::ctypes::c_uint,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilMusicMount() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilMusicUmount() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilPhotoMount() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilPhotoUmount() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilCacheMount() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilCacheUmount() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilSystemParamGetInt(
        paramId: crate::ctypes::c_uint,
        value: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilSystemParamGetString(
        paramId: crate::ctypes::c_uint,
        buf: *mut SceChar8,
        bufSize: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilAppParamGetInt(
        paramId: SceAppUtilAppParamId,
        value: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilSaveSafeMemory(
        buf: *mut crate::ctypes::c_void,
        bufSize: SceSize,
        offset: SceOff,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilLoadSafeMemory(
        buf: *mut crate::ctypes::c_void,
        bufSize: SceSize,
        offset: SceOff,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilStoreBrowse(param: *mut SceAppUtilStoreBrowseParam) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilBgdlGetStatus(stat: *mut SceAppUtilBgdlStatus) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceAppUtilLaunchWebBrowser(
        param: *mut SceAppUtilWebBrowserParam,
    ) -> crate::ctypes::c_int;
}
