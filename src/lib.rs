#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]
#![allow(
    nonstandard_style,
    clippy::useless_transmute,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::len_without_is_empty
)]

#[cfg(test)]
mod tests;

#[cfg(any(not(feature = "bindgen"), docsrs))]
include!("bindings.rs");

#[cfg(all(feature = "bindgen", not(docsrs)))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
