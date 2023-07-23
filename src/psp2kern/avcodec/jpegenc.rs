/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub type SceJpegEncoderContext = *mut crate::ctypes::c_void;
pub mod SceJpegEncErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_JPEGENC_ERROR_IMAGE_SIZE: Type = 2154103296;
    pub const SCE_JPEGENC_ERROR_INSUFFICIENT_BUFFER: Type = 2154103297;
    pub const SCE_JPEGENC_ERROR_INVALID_COMPRATIO: Type = 2154103298;
    pub const SCE_JPEGENC_ERROR_INVALID_PIXELFORMAT: Type = 2154103299;
    pub const SCE_JPEGENC_ERROR_INVALID_HEADER_MODE: Type = 2154103300;
    pub const SCE_JPEGENC_ERROR_INVALID_POINTER: Type = 2154103301;
    pub const SCE_JPEGENC_ERROR_NOT_PHY_CONTINUOUS_MEMORY: Type = 2154103302;
}
pub mod SceJpegEncoderPixelFormat {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_JPEGENC_PIXELFORMAT_ARGB8888: Type = 0;
    pub const SCE_JPEGENC_PIXELFORMAT_YCBCR420: Type = 8;
    pub const SCE_JPEGENC_PIXELFORMAT_YCBCR422: Type = 9;
    pub const SCE_JPEGENC_PIXELFORMAT_CSC_ARGB_YCBCR: Type = 16;
}
pub mod SceJpegEncoderHeaderMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_JPEGENC_HEADER_MODE_JPEG: Type = 0;
    pub const SCE_JPEGENC_HEADER_MODE_MJPEG: Type = 1;
}
extern "C" {
    pub fn ksceJpegEncoderInit(
        context: SceJpegEncoderContext,
        inWidth: crate::ctypes::c_int,
        inHeight: crate::ctypes::c_int,
        pixelformat: SceJpegEncoderPixelFormat::Type,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceJpegEncoderEnd(context: SceJpegEncoderContext) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceJpegEncoderEncode(
        context: SceJpegEncoderContext,
        inBuffer: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceJpegEncoderSetCompressionRatio(
        context: SceJpegEncoderContext,
        ratio: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceJpegEncoderSetOutputAddr(
        context: SceJpegEncoderContext,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceJpegEncoderCsc(
        context: SceJpegEncoderContext,
        outBuffer: *mut crate::ctypes::c_void,
        inBuffer: *const crate::ctypes::c_void,
        inPitch: crate::ctypes::c_int,
        inPixelFormat: SceJpegEncoderPixelFormat::Type,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceJpegEncoderGetContextSize() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceJpegEncoderSetValidRegion(
        context: SceJpegEncoderContext,
        inWidth: crate::ctypes::c_int,
        inHeight: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceJpegEncoderSetHeaderMode(
        context: SceJpegEncoderContext,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
