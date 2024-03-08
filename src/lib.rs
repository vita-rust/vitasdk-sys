#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]
#![allow(
    nonstandard_style,
    clippy::useless_transmute,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::len_without_is_empty
)]

mod ctypes {
    pub use ::core::ffi::c_void;

    pub type c_char = i8;
    pub type c_double = f64;
    pub type c_float = f32;
    pub type c_int = i32;
    pub type c_long = i32;
    pub type c_longlong = i64;
    pub type c_schar = i8;
    pub type c_short = i16;
    pub type c_uchar = u8;
    pub type c_uint = u32;
    pub type c_ulong = u32;
    pub type c_ulonglong = u64;
    pub type c_ushort = u16;
}

pub use ctypes::*;

#[cfg(test)]
mod tests;

#[cfg(any(not(feature = "bindgen"), docsrs))]
include!("bindings.rs");

#[cfg(all(feature = "bindgen", not(docsrs)))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
