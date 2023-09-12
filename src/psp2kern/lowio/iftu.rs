/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub mod SceIftuErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IFTU_ERROR_INVALID_PLANE: Type = 2151614208;
    pub const SCE_IFTU_ERROR_INVALID_PARAM: Type = 2151614209;
    pub const SCE_IFTU_ERROR_INVALID_PIXELFORMAT: Type = 2151614211;
    pub const SCE_IFTU_ERROR_PLANE_BUSY: Type = 2151614212;
}
pub mod SceIftuPixelformat {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_IFTU_PIXELFORMAT_BGR565: Type = 1;
    pub const SCE_IFTU_PIXELFORMAT_RGB565: Type = 2;
    pub const SCE_IFTU_PIXELFORMAT_BGRA5551: Type = 4;
    pub const SCE_IFTU_PIXELFORMAT_RGBA5551: Type = 8;
    pub const SCE_IFTU_PIXELFORMAT_BGRX8888: Type = 16;
    pub const SCE_IFTU_PIXELFORMAT_RGBX8888: Type = 32;
    pub const SCE_IFTU_PIXELFORMAT_BGRA1010102: Type = 64;
    pub const SCE_IFTU_PIXELFORMAT_RGBA1010102: Type = 128;
    pub const SCE_IFTU_PIXELFORMAT_BGRP: Type = 256;
    pub const SCE_IFTU_PIXELFORMAT_RGBX8888_MULT: Type = 4096;
    pub const SCE_IFTU_PIXELFORMAT_BGRX8888_MULT: Type = 8192;
    pub const SCE_IFTU_PIXELFORMAT_RGBA1010102_MULT: Type = 16384;
    pub const SCE_IFTU_PIXELFORMAT_BGRA1010102_MULT: Type = 32768;
    pub const SCE_IFTU_PIXELFORMAT_NV12: Type = 65536;
    pub const SCE_IFTU_PIXELFORMAT_YUV420: Type = 131072;
    pub const SCE_IFTU_PIXELFORMAT_YUV422: Type = 2097152;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIftuCscParams {
    pub post_add_0: crate::ctypes::c_uint,
    pub post_add_1_2: crate::ctypes::c_uint,
    pub post_clamp_max_0: crate::ctypes::c_uint,
    pub post_clamp_min_0: crate::ctypes::c_uint,
    pub post_clamp_max_1_2: crate::ctypes::c_uint,
    pub post_clamp_min_1_2: crate::ctypes::c_uint,
    pub ctm: [[crate::ctypes::c_uint; 3usize]; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIftuConvParams {
    pub size: crate::ctypes::c_uint,
    pub unk04: crate::ctypes::c_uint,
    pub csc_params1: *mut SceIftuCscParams,
    pub csc_params2: *mut SceIftuCscParams,
    pub csc_control: crate::ctypes::c_uint,
    pub unk14: crate::ctypes::c_uint,
    pub unk18: crate::ctypes::c_uint,
    pub unk1C: crate::ctypes::c_uint,
    pub alpha: crate::ctypes::c_uint,
    pub unk24: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIftuFrameBuf {
    pub pixelformat: crate::ctypes::c_uint,
    pub width: crate::ctypes::c_uint,
    pub height: crate::ctypes::c_uint,
    pub leftover_stride: crate::ctypes::c_uint,
    pub leftover_align: crate::ctypes::c_uint,
    pub paddr0: crate::ctypes::c_uint,
    pub paddr1: crate::ctypes::c_uint,
    pub paddr2: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIftuPlaneState {
    pub fb: SceIftuFrameBuf,
    pub unk20: crate::ctypes::c_uint,
    pub src_x: crate::ctypes::c_uint,
    pub src_y: crate::ctypes::c_uint,
    pub src_w: crate::ctypes::c_uint,
    pub src_h: crate::ctypes::c_uint,
    pub dst_x: crate::ctypes::c_uint,
    pub dst_y: crate::ctypes::c_uint,
    pub dst_w: crate::ctypes::c_uint,
    pub dst_h: crate::ctypes::c_uint,
    pub vtop_padding: crate::ctypes::c_uint,
    pub vbot_padding: crate::ctypes::c_uint,
    pub hleft_padding: crate::ctypes::c_uint,
    pub hright_padding: crate::ctypes::c_uint,
}
extern "C" {
    pub fn ksceIftuCsc(
        dst: *mut SceIftuFrameBuf,
        src: *mut SceIftuPlaneState,
        params: *mut SceIftuConvParams,
    ) -> crate::ctypes::c_int;
}
