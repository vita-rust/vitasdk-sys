#![no_std]
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_safety_doc,
    clippy::useless_transmute,
    clippy::too_many_arguments,
    clippy::len_without_is_empty,
    clippy::module_inception
)]
pub mod ctypes;
pub mod psp2;
pub mod psp2common;
pub mod psp2kern;
pub mod vitasdk;
