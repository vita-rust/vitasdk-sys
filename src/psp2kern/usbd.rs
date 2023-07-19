/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::vitasdk::build_utils::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
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
pub const SCE_USBD_PROBE_SUCCEEDED: u32 = 0;
pub const SCE_USBD_PROBE_FAILED: i32 = -1;
pub const SCE_USBD_ATTACH_SUCCEEDED: u32 = 0;
pub const SCE_USBD_ATTACH_FAILED: i32 = -1;
pub const SCE_USBD_DETACH_SUCCEEDED: u32 = 0;
pub const SCE_USBD_DETACH_FAILED: i32 = -1;
pub const SCE_USBD_MAX_LS_CONTROL_PACKET_SIZE: u32 = 8;
pub const SCE_USBD_MAX_FS_CONTROL_PACKET_SIZE: u32 = 64;
pub const SCE_USBD_MAX_ISOCHRONOUS_PACKET_SIZE: u32 = 1023;
pub const SCE_USBD_MAX_LS_INTERRUPT_PACKET_SIZE: u32 = 8;
pub const SCE_USBD_MAX_FS_INTERRUPT_PACKET_SIZE: u32 = 64;
pub const SCE_USBD_MAX_BULK_PACKET_SIZE: u32 = 64;
pub const SCE_USBD_FEATURE_ENDPOINT_HALT: u32 = 0;
pub const SCE_USBD_FEATURE_DEVICE_REMOTE_WAKEUP: u32 = 1;
pub const SCE_USBD_CLASS_PER_INTERFACE: u32 = 0;
pub const SCE_USBD_CLASS_AUDIO: u32 = 1;
pub const SCE_USBD_CLASS_COMMUNICATIONS: u32 = 2;
pub const SCE_USBD_CLASS_HID: u32 = 3;
pub const SCE_USBD_CLASS_MONITOR: u32 = 4;
pub const SCE_USBD_CLASS_PHYSICAL: u32 = 5;
pub const SCE_USBD_CLASS_POWER: u32 = 6;
pub const SCE_USBD_CLASS_PRINTER: u32 = 7;
pub const SCE_USBD_CLASS_STORAGE: u32 = 8;
pub const SCE_USBD_CLASS_HUB: u32 = 9;
pub const SCE_USBD_CLASS_DATA: u32 = 10;
pub const SCE_USBD_CLASS_VENDOR_SPECIFIC: u32 = 255;
pub const SCE_USBD_CONFIGURATION_RESERVED_ZERO: u32 = 31;
pub const SCE_USBD_CONFIGURATION_REMOTE_WAKEUP: u32 = 32;
pub const SCE_USBD_CONFIGURATION_SELF_POWERED: u32 = 64;
pub const SCE_USBD_CONFIGURATION_RESERVED_ONE: u32 = 128;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_BITS: u32 = 3;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_SHIFT: u32 = 0;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_CONTROL: u32 = 0;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_ISOCHRONOUS: u32 = 1;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_BULK: u32 = 2;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_INTERRUPT: u32 = 3;
pub const SCE_USBD_ENDPOINT_NUMBER_BITS: u32 = 31;
pub const SCE_USBD_ENDPOINT_NUMBER_SHIFT: u32 = 0;
pub const SCE_USBD_ENDPOINT_DIRECTION_BITS: u32 = 128;
pub const SCE_USBD_ENDPOINT_DIRECTION_SHIFT: u32 = 7;
pub const SCE_USBD_ENDPOINT_DIRECTION_OUT: u32 = 0;
pub const SCE_USBD_ENDPOINT_DIRECTION_IN: u32 = 128;
pub const SCE_USBD_DEVICE_SPEED_LS: u32 = 0;
pub const SCE_USBD_DEVICE_SPEED_FS: u32 = 1;
pub const SCE_USBD_DEVICE_SPEED_HS: u32 = 2;
pub const OHCI_CC_NO_ERROR: u32 = 0;
pub const OHCI_CC_CRC: u32 = 1;
pub const OHCI_CC_BIT_STUFFING: u32 = 2;
pub const OHCI_CC_DATA_TOGGLE_MISMATCH: u32 = 3;
pub const OHCI_CC_STALL: u32 = 4;
pub const OHCI_CC_DEVICE_NOT_RESPONDING: u32 = 5;
pub const OHCI_CC_PID_CHECK_FAILURE: u32 = 6;
pub const OHCI_CC_UNEXPECTED_PID: u32 = 7;
pub const OHCI_CC_DATA_OVERRUN: u32 = 8;
pub const OHCI_CC_DATA_UNDERRUN: u32 = 9;
pub const OHCI_CC_BUFFER_OVERRUN: u32 = 12;
pub const OHCI_CC_BUFFER_UNDERRUN: u32 = 13;
pub const OHCI_CC_NOT_ACCESSED1: u32 = 14;
pub const OHCI_CC_NOT_ACCESSED2: u32 = 15;
pub const EHCI_CC_MISSED_MICRO_FRAME: u32 = 16;
pub const EHCI_CC_XACT: u32 = 32;
pub const EHCI_CC_BABBLE: u32 = 48;
pub const EHCI_CC_DATABUF: u32 = 64;
pub const EHCI_CC_HALTED: u32 = 80;
pub const USBD_CC_NOERR: u32 = 0;
pub const USBD_CC_MISSED_MICRO_FRAME: u32 = 1;
pub const USBD_CC_XACTERR: u32 = 2;
pub const USBD_CC_BABBLE: u32 = 4;
pub const USBD_CC_DATABUF: u32 = 8;
pub mod SceUsbdDescriptorType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_USBD_DESCRIPTOR_DEVICE: Type = 1;
    pub const SCE_USBD_DESCRIPTOR_CONFIGURATION: Type = 2;
    pub const SCE_USBD_DESCRIPTOR_STRING: Type = 3;
    pub const SCE_USBD_DESCRIPTOR_INTERFACE: Type = 4;
    pub const SCE_USBD_DESCRIPTOR_ENDPOINT: Type = 5;
    pub const SCE_USBD_DESCRIPTOR_DEVICE_QUALIFIER: Type = 6;
    pub const SCE_USBD_DESCRIPTOR_OTHER_SPEED: Type = 7;
    pub const SCE_USBD_DESCRIPTOR_INTERFACE_POWER: Type = 8;
    pub const SCE_USBD_DESCRIPTOR_OTG: Type = 9;
    pub const SCE_USBD_DESCRIPTOR_HID: Type = 33;
    pub const SCE_USBD_DESCRIPTOR_REPORT: Type = 34;
}
pub mod SceUsbdErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_USBD_ERROR_NOT_INITIALIZED: Type = 2149842945;
    pub const SCE_USBD_ERROR_ALREADY_INITIALIZED: Type = 2149842946;
    pub const SCE_USBD_ERROR_INVALID_PARAM: Type = 2149842947;
    pub const SCE_USBD_ERROR_PIPE_NOT_FOUND: Type = 2149842948;
    pub const SCE_USBD_ERROR_NO_MEMORY: Type = 2149842949;
    pub const SCE_USBD_ERROR_DEVICE_NOT_FOUND: Type = 2149842950;
    pub const SCE_USBD_ERROR_80240007: Type = 2149842951;
    pub const SCE_USBD_ERROR_80240009: Type = 2149842953;
    pub const SCE_USBD_ERROR_8024000A: Type = 2149842954;
    pub const SCE_USBD_ERROR_FATAL: Type = 2149843199;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDeviceDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubclass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdConfigurationDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub MaxPower: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdInterfaceDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubclass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdEndpointDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct SceUsbdStringDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bString: __IncompleteArrayField<u8>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdHidSubDescriptorInfo {
    pub bDescriptorType: u8,
    pub wDescriptorLength0: u8,
    pub wDescriptorLength1: u8,
}
#[repr(C)]
#[derive(Debug)]
pub struct SceUsbdHidDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdHID0: u8,
    pub bcdHID1: u8,
    pub bCountryCode: u8,
    pub bNumDescriptors: u8,
    pub SubDescriptorInfo: __IncompleteArrayField<SceUsbdHidSubDescriptorInfo>,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDeviceAddress {
    pub unk0: u32,
    pub unk1: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDriver {
    pub name: *const crate::ctypes::c_char,
    pub probe: ::core::option::Option<
        unsafe extern "C" fn(device_id: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
    pub attach: ::core::option::Option<
        unsafe extern "C" fn(device_id: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
    pub detach: ::core::option::Option<
        unsafe extern "C" fn(device_id: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdCompositeDriver {
    pub name: *const crate::ctypes::c_char,
    pub probe: ::core::option::Option<
        unsafe extern "C" fn(
            device_id: crate::ctypes::c_int,
            desc: *mut SceUsbdEndpointDescriptor,
        ) -> crate::ctypes::c_int,
    >,
    pub attach: ::core::option::Option<
        unsafe extern "C" fn(device_id: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
    pub detach: ::core::option::Option<
        unsafe extern "C" fn(device_id: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDeviceRequest {
    pub bmRequestType: u8,
    pub bRequest: u8,
    pub wValue: u16,
    pub wIndex: u16,
    pub wLength: u16,
}
pub mod SceUsbdReqtype {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_USBD_REQTYPE_DIR_BITS: Type = 128;
    pub const SCE_USBD_REQTYPE_DIR_TO_DEVICE: Type = 0;
    pub const SCE_USBD_REQTYPE_DIR_TO_HOST: Type = 128;
    pub const SCE_USBD_REQTYPE_TYPE_BITS: Type = 96;
    pub const SCE_USBD_REQTYPE_TYPE_STANDARD: Type = 0;
    pub const SCE_USBD_REQTYPE_TYPE_CLASS: Type = 32;
    pub const SCE_USBD_REQTYPE_TYPE_VENDOR: Type = 64;
    pub const SCE_USBD_REQTYPE_TYPE_RESERVED: Type = 96;
    pub const SCE_USBD_REQTYPE_RECIP_BITS: Type = 31;
    pub const SCE_USBD_REQTYPE_RECIP_DEVICE: Type = 0;
    pub const SCE_USBD_REQTYPE_RECIP_INTERFACE: Type = 1;
    pub const SCE_USBD_REQTYPE_RECIP_ENDPOINT: Type = 2;
    pub const SCE_USBD_REQTYPE_RECIP_OTHER: Type = 3;
}
pub mod SceUsbdRequest {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_USBD_REQUEST_GET_STATUS: Type = 0;
    pub const SCE_USBD_REQUEST_CLEAR_FEATURE: Type = 1;
    pub const SCE_USBD_REQUEST_SET_FEATURE: Type = 3;
    pub const SCE_USBD_REQUEST_SET_ADDRESS: Type = 5;
    pub const SCE_USBD_REQUEST_GET_DESCRIPTOR: Type = 6;
    pub const SCE_USBD_REQUEST_SET_DESCRIPTOR: Type = 7;
    pub const SCE_USBD_REQUEST_GET_CONFIGURATION: Type = 8;
    pub const SCE_USBD_REQUEST_SET_CONFIGURATION: Type = 9;
    pub const SCE_USBD_REQUEST_GET_INTERFACE: Type = 10;
    pub const SCE_USBD_REQUEST_SET_INTERFACE: Type = 11;
    pub const SCE_USBD_REQUEST_SYNCH_FRAME: Type = 12;
}
#[repr(C)]
#[repr(align(2))]
#[derive(Debug, Copy, Clone)]
pub struct ksceUsbdIsochPswLen {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
}
impl ksceUsbdIsochPswLen {
    #[inline]
    pub fn len(&self) -> u16 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 12u8) as u16) }
    }
    #[inline]
    pub fn set_len(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub fn PSW(&self) -> u16 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 4u8) as u16) }
    }
    #[inline]
    pub fn set_PSW(&mut self, val: u16) {
        unsafe {
            let val: u16 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(len: u16, PSW: u16) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 12u8, {
            let len: u16 = unsafe { ::core::mem::transmute(len) };
            len as u64
        });
        __bindgen_bitfield_unit.set(12usize, 4u8, {
            let PSW: u16 = unsafe { ::core::mem::transmute(PSW) };
            PSW as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ksceUsbdIsochTransfer {
    pub buffer_base: *mut crate::ctypes::c_void,
    pub relative_start_frame: i32,
    pub num_packets: i32,
    pub packets: [ksceUsbdIsochPswLen; 8usize],
}
extern "C" {
    pub fn ksceUsbdRegisterDriver(driver: *const SceUsbdDriver) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdRegisterCompositeLdd(
        driver: *const SceUsbdCompositeDriver,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdUnregisterDriver(driver: *const SceUsbdDriver) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdUnregisterCompositeLdd(
        driver: *const SceUsbdCompositeDriver,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdScanStaticDescriptor(
        device_id: SceUID,
        start: *mut crate::ctypes::c_void,
        type_: SceUsbdDescriptorType::Type,
    ) -> *mut crate::ctypes::c_void;
}
extern "C" {
    pub fn ksceUsbdOpenPipe(
        device_id: crate::ctypes::c_int,
        endpoint: *mut SceUsbdEndpointDescriptor,
    ) -> SceUID;
}
extern "C" {
    pub fn ksceUsbdClosePipe(pipe_id: SceUID) -> crate::ctypes::c_int;
}
pub type ksceUsbdDoneCallback = ::core::option::Option<
    unsafe extern "C" fn(result: i32, count: i32, arg: *mut crate::ctypes::c_void),
>;
pub type ksceUsbdIsochDoneCallback = ::core::option::Option<
    unsafe extern "C" fn(
        result: i32,
        req: *mut ksceUsbdIsochTransfer,
        arg: *mut crate::ctypes::c_void,
    ),
>;
extern "C" {
    pub fn ksceUsbdControlTransfer(
        pipe_id: SceUID,
        req: *const SceUsbdDeviceRequest,
        buffer: *mut crate::ctypes::c_uchar,
        cb: ksceUsbdDoneCallback,
        user_data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdInterruptTransfer(
        pipe_id: SceUID,
        buffer: *mut crate::ctypes::c_uchar,
        length: SceSize,
        cb: ksceUsbdDoneCallback,
        user_data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdIsochronousTransfer(
        pipe_id: SceUID,
        transfer: *mut ksceUsbdIsochTransfer,
        cb: ksceUsbdIsochDoneCallback,
        user_data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdBulkTransfer(
        pipe_id: SceUID,
        buffer: *mut crate::ctypes::c_uchar,
        length: crate::ctypes::c_uint,
        cb: ksceUsbdDoneCallback,
        user_data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdBulkTransfer2(
        pipe_id: crate::ctypes::c_int,
        buffer: *mut crate::ctypes::c_uchar,
        length: crate::ctypes::c_uint,
        cb: ksceUsbdDoneCallback,
        user_data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdGetDeviceLocation(device_id: SceUID, location: *mut u8) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdSuspend(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdSuspendPhase2(
        port: crate::ctypes::c_int,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdResume(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdHostStop(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdHostStart(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbdGetDeviceSpeed(
        device_id: crate::ctypes::c_int,
        speed: *mut u8,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbd_05073925(
        device_id: SceUID,
        unk1: *mut crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUsbd_7938DAC7(pipe_id: SceUID) -> crate::ctypes::c_int;
}
