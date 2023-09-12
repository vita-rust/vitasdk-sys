/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

#[repr(C)]
pub struct SceSblDmac5EncDecParam {
    pub src: *const crate::ctypes::c_void,
    pub dst: *mut crate::ctypes::c_void,
    pub length: SceSize,
    pub key: *const crate::ctypes::c_void,
    pub keysize: SceSize,
    pub iv: *mut crate::ctypes::c_void,
}
#[repr(C)]
pub struct SceSblDmac5HashTransformContext {
    pub state: [SceUInt32; 8usize],
    pub length: SceUInt64,
}
#[repr(C)]
pub struct SceSblDmac5HashTransformParam {
    pub src: *const crate::ctypes::c_void,
    pub dst: *mut crate::ctypes::c_void,
    pub length: SceSize,
    pub key: *const crate::ctypes::c_void,
    pub keysize: SceSize,
    pub ctx: *mut crate::ctypes::c_void,
}
extern "C" {
    pub fn sceSblDmac5EncDec(
        param: *mut SceSblDmac5EncDecParam,
        command: SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceSblDmac5HashTransform(
        param: *mut SceSblDmac5HashTransformParam,
        command: SceUInt32,
        extra: SceUInt32,
    ) -> crate::ctypes::c_int;
}
