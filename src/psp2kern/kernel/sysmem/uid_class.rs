/* automatically generated by rust-bindgen 0.68.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::core::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub type SceClassCallback = ::core::option::Option<
    unsafe extern "C" fn(item: *mut crate::ctypes::c_void) -> crate::ctypes::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceClass {
    pub next: *mut SceClass,
    pub root: *mut SceClass,
    pub prev: *mut SceClass,
    pub name: *const crate::ctypes::c_char,
    pub uidclass: *mut SceClass,
    pub attributes: crate::ctypes::c_uint,
    pub itemsize: crate::ctypes::c_ushort,
    pub itemsize_aligned: crate::ctypes::c_ushort,
    pub unk1C: crate::ctypes::c_uint,
    pub create_cb: SceClassCallback,
    pub destroy_cb: SceClassCallback,
    pub magic: crate::ctypes::c_uint,
}
#[repr(C)]
pub struct SceObjectBase {
    pub __bindgen_anon_1: SceObjectBase__bindgen_ty_1,
    pub data: __IncompleteArrayField<u32>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceObjectBase__bindgen_ty_1 {
    pub sce_reserved: [u32; 2usize],
    pub __bindgen_anon_1: SceObjectBase__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceObjectBase__bindgen_ty_1__bindgen_ty_1 {
    pub object: *mut crate::ctypes::c_void,
    pub sce_class: *mut SceClass,
}
extern "C" {
    pub fn ksceKernelGetUidClass() -> *mut SceClass;
}
extern "C" {
    pub fn ksceKernelGetUidDLinkClass() -> *mut SceClass;
}
extern "C" {
    pub fn ksceKernelGetUidHeapClass() -> *mut SceClass;
}
extern "C" {
    pub fn ksceKernelGetUidMemBlockClass() -> *mut SceClass;
}
extern "C" {
    pub fn ksceKernelCreateClass(
        cls: *mut SceClass,
        name: *const crate::ctypes::c_char,
        uidclass: *mut crate::ctypes::c_void,
        itemsize: SceSize,
        create: SceClassCallback,
        destroy: SceClassCallback,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelFindClassByName(
        name: *const crate::ctypes::c_char,
        cls: *mut *mut SceClass,
    ) -> crate::ctypes::c_int;
}
