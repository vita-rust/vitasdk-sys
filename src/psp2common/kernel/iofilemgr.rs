/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

pub const SCE_CST_MODE: u32 = 1;
pub const SCE_CST_SIZE: u32 = 4;
pub const SCE_CST_CT: u32 = 8;
pub const SCE_CST_AT: u32 = 16;
pub const SCE_CST_MT: u32 = 32;
#[repr(C)]
pub struct SceIoDevInfo {
    pub max_size: SceOff,
    pub free_size: SceOff,
    pub cluster_size: SceSize,
    pub unk: *mut crate::ctypes::c_void,
}
pub mod SceIoMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_O_RDONLY: Type = 1;
    pub const SCE_O_WRONLY: Type = 2;
    pub const SCE_O_RDWR: Type = 3;
    pub const SCE_O_NBLOCK: Type = 4;
    pub const SCE_O_DIROPEN: Type = 8;
    pub const SCE_O_RDLOCK: Type = 16;
    pub const SCE_O_WRLOCK: Type = 32;
    pub const SCE_O_APPEND: Type = 256;
    pub const SCE_O_CREAT: Type = 512;
    pub const SCE_O_TRUNC: Type = 1024;
    pub const SCE_O_EXCL: Type = 2048;
    pub const SCE_O_SCAN: Type = 4096;
    pub const SCE_O_RCOM: Type = 8192;
    pub const SCE_O_NOBUF: Type = 16384;
    pub const SCE_O_NOWAIT: Type = 32768;
    pub const SCE_O_FDEXCL: Type = 16777216;
    pub const SCE_O_PWLOCK: Type = 33554432;
    pub const SCE_O_FGAMEDATA: Type = 1073741824;
}
pub mod SceIoSeekMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SEEK_SET: Type = 0;
    pub const SCE_SEEK_CUR: Type = 1;
    pub const SCE_SEEK_END: Type = 2;
}
pub mod SceIoDevType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_DEV_TYPE_NULL: Type = 0;
    pub const SCE_DEV_TYPE_CHAR: Type = 1;
    pub const SCE_DEV_TYPE_BLOCK: Type = 4;
    pub const SCE_DEV_TYPE_FS: Type = 16;
    pub const SCE_DEV_TYPE_ALIAS: Type = 32;
    pub const SCE_DEV_TYPE_MOUNTPT: Type = 64;
}
pub mod SceIoAccessMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_S_IXUSR: Type = 64;
    pub const SCE_S_IWUSR: Type = 128;
    pub const SCE_S_IRUSR: Type = 256;
    pub const SCE_S_IRWXU: Type = 448;
    pub const SCE_S_IXGRP: Type = 0;
    pub const SCE_S_IWGRP: Type = 0;
    pub const SCE_S_IRGRP: Type = 0;
    pub const SCE_S_IRWXG: Type = 0;
    pub const SCE_S_IXSYS: Type = 1;
    pub const SCE_S_IWSYS: Type = 2;
    pub const SCE_S_IRSYS: Type = 4;
    pub const SCE_S_IRWXS: Type = 7;
    pub const SCE_S_IXOTH: Type = 1;
    pub const SCE_S_IWOTH: Type = 2;
    pub const SCE_S_IROTH: Type = 4;
    pub const SCE_S_IRWXO: Type = 7;
    pub const SCE_S_ISVTX: Type = 0;
    pub const SCE_S_ISGID: Type = 0;
    pub const SCE_S_ISUID: Type = 0;
    pub const SCE_S_IFDIR: Type = 4096;
    pub const SCE_S_IFREG: Type = 8192;
    pub const SCE_S_IFLNK: Type = 16384;
    pub const SCE_S_IFMT: Type = 61440;
}
pub mod SceIoFileMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SO_IXOTH: Type = 1;
    pub const SCE_SO_IWOTH: Type = 2;
    pub const SCE_SO_IROTH: Type = 4;
    pub const SCE_SO_IFLNK: Type = 8;
    pub const SCE_SO_IFDIR: Type = 16;
    pub const SCE_SO_IFREG: Type = 32;
    pub const SCE_SO_IFMT: Type = 56;
}
#[repr(C)]
pub struct SceIoStat {
    pub st_mode: SceMode,
    pub st_attr: crate::ctypes::c_uint,
    pub st_size: SceOff,
    pub st_ctime: SceDateTime,
    pub st_atime: SceDateTime,
    pub st_mtime: SceDateTime,
    pub st_private: [crate::ctypes::c_uint; 6usize],
}
#[repr(C)]
pub struct SceIoDirent {
    pub d_stat: SceIoStat,
    pub d_name: [crate::ctypes::c_char; 256usize],
    pub d_private: *mut crate::ctypes::c_void,
    pub dummy: crate::ctypes::c_int,
}
