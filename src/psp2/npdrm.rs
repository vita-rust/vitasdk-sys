/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::npdrm::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

extern "C" {
    pub fn _sceNpDrmGetRifName(
        rif_name: *mut crate::ctypes::c_char,
        aid: u64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn _sceNpDrmGetFixedRifName(
        rif_name: *mut crate::ctypes::c_char,
        aid: u64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn _sceNpDrmCheckActData(
        act_type: *mut crate::ctypes::c_int,
        version_flag: *mut crate::ctypes::c_int,
        account_id: *mut SceUInt64,
        act_exp_time: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn _sceNpDrmGetRifNameForInstall(
        rif_name: *mut crate::ctypes::c_char,
        rif_data: *const crate::ctypes::c_void,
        unk: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePsmDrmGetRifKey(
        license_buf: *const ScePsmDrmLicense,
        keydata: *mut crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
