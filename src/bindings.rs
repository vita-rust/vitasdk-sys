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
pub type _bindgen_ty_1 = crate::ctypes::c_uint;
pub type _bindgen_ty_2 = crate::ctypes::c_uint;
pub use self::SceAppUtilSaveDataRemoveMode as SceAppUtilSaveDataDataRemoveMode;
pub use self::SceAppUtilSaveDataSaveMode as SceAppUtilSaveDataDataSaveMode;
pub use self::SceHttpStatusCode as SceHttpStatuscode;
pub use self::SceHttpVersion as SceHttpHttpVersion;
pub use self::SceImeDialogErrorCode as SceImeDialogError;
pub use self::SceNotificationUitlErrorCode as SceNotificationUtilErrorCode;
pub use self::_ScePerfArmPmonEventCode as ScePerfArmPmonEventCode;
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: crate::ctypes::c_longlong,
    pub __clang_max_align_nonce2: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MsgPipeRecvData {
    pub message: *mut crate::ctypes::c_void,
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MsgPipeSendData {
    pub message: *const crate::ctypes::c_void,
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicExportParam {
    pub reserved: [crate::ctypes::c_char; 128usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PhotoExportParam {
    pub version: crate::ctypes::c_int,
    pub photoTitle: *const SceWChar32,
    pub gameTitle: *const SceWChar32,
    pub gameComment: *const SceWChar32,
    pub reserved: [crate::ctypes::c_int; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAesContext {
    pub data: [u8; 976usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAllocOpt {
    pub size: SceSize,
    pub data04: SceSize,
    pub align: SceSize,
    pub data0C: crate::ctypes::c_int,
    pub data10: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrAppInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrAppState {
    pub systemEventNum: SceUInt32,
    pub appEventNum: SceUInt32,
    pub isSystemUiOverlaid: SceBool,
    pub reserved: [SceUInt8; 116usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrBudgetInfo {
    pub size: crate::ctypes::c_int,
    pub app_mode: crate::ctypes::c_int,
    pub unk0: crate::ctypes::c_int,
    pub total_user_rw_mem: crate::ctypes::c_uint,
    pub free_user_rw: crate::ctypes::c_uint,
    pub extra_mem_allowed: SceBool,
    pub unk1: crate::ctypes::c_int,
    pub total_extra_mem: crate::ctypes::c_uint,
    pub free_extra_mem: crate::ctypes::c_uint,
    pub unk2: [crate::ctypes::c_int; 2usize],
    pub total_phycont_mem: crate::ctypes::c_uint,
    pub free_phycont_mem: crate::ctypes::c_uint,
    pub unk3: [crate::ctypes::c_int; 10usize],
    pub total_cdram_mem: crate::ctypes::c_uint,
    pub free_cdram_mem: crate::ctypes::c_uint,
    pub reserved: [crate::ctypes::c_int; 9usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrCoredumpState {
    pub pid: SceUID,
    pub process_state: crate::ctypes::c_int,
    pub progress: crate::ctypes::c_int,
    pub is_coredump_completed: crate::ctypes::c_int,
    pub data_0x10: crate::ctypes::c_int,
    pub path_len: SceSize,
    pub path: [crate::ctypes::c_char; 1024usize],
    pub data_0x418: crate::ctypes::c_int,
    pub data_0x41C: crate::ctypes::c_int,
    pub data_0x420: crate::ctypes::c_int,
    pub data_0x424: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrDrmOpenParam {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrExecOptParam {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrLaunchAppOptParam {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrLaunchParam {
    pub size: SceSize,
    pub unk_4: crate::ctypes::c_uint,
    pub unk_8: crate::ctypes::c_uint,
    pub unk_C: crate::ctypes::c_uint,
    pub unk_10: crate::ctypes::c_uint,
    pub unk_14: crate::ctypes::c_uint,
    pub unk_18: crate::ctypes::c_uint,
    pub unk_1C: crate::ctypes::c_uint,
    pub unk_20: crate::ctypes::c_uint,
    pub unk_24: crate::ctypes::c_uint,
    pub unk_28: crate::ctypes::c_uint,
    pub unk_2C: crate::ctypes::c_uint,
    pub unk_30: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrLoadExecOptParam {
    pub reserved: [crate::ctypes::c_int; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrSaveDataData {
    pub size: crate::ctypes::c_int,
    pub slotId: crate::ctypes::c_uint,
    pub slotParam: *mut SceAppUtilSaveDataSlotParam,
    pub reserved: [u8; 32usize],
    pub files: *mut SceAppUtilSaveDataFile,
    pub fileNum: crate::ctypes::c_int,
    pub mountPoint: SceAppUtilSaveDataMountPoint,
    pub requiredSizeKB: *mut crate::ctypes::c_uint,
    pub unk_0x48: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrSaveDataDataDelete {
    pub size: crate::ctypes::c_int,
    pub slotId: crate::ctypes::c_uint,
    pub slotParam: *mut SceAppUtilSaveDataSlotParam,
    pub reserved: [u8; 32usize],
    pub files: *mut SceAppUtilSaveDataFile,
    pub fileNum: crate::ctypes::c_int,
    pub mountPoint: SceAppUtilSaveDataMountPoint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrSaveDataSlot {
    pub size: crate::ctypes::c_int,
    pub slotId: crate::ctypes::c_uint,
    pub slotParam: SceAppUtilSaveDataSlotParam,
    pub reserved: [u8; 116usize],
    pub mountPoint: SceAppUtilSaveDataMountPoint,
    pub reserved2: [u8; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrSaveDataSlotDelete {
    pub size: crate::ctypes::c_int,
    pub slotId: crate::ctypes::c_uint,
    pub mountPoint: SceAppUtilSaveDataMountPoint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppMgrSystemEvent {
    pub systemEvent: crate::ctypes::c_int,
    pub reserved: [u8; 60usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilAppEventParam {
    pub type_: SceAppUtilAppEventType,
    pub dat: [u8; 1024usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilBgdlStatus {
    pub type_: SceAppUtilBgdlStatusType,
    pub addcontNumReady: SceUInt32,
    pub addcontNumNotReady: SceUInt32,
    pub licenseReady: SceUInt32,
    pub reserved: [SceChar8; 28usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilBootParam {
    pub attr: SceAppUtilBootAttribute,
    pub appVersion: crate::ctypes::c_uint,
    pub reserved: [u8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilInitParam {
    pub workBufSize: SceSize,
    pub reserved: [u8; 60usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilMountPoint {
    pub data: [i8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataFile {
    pub filePath: *const crate::ctypes::c_char,
    pub buf: *mut crate::ctypes::c_void,
    pub bufSize: SceSize,
    pub offset: SceOff,
    pub mode: crate::ctypes::c_uint,
    pub progDelta: crate::ctypes::c_uint,
    pub reserved: [u8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataFileSlot {
    pub id: crate::ctypes::c_uint,
    pub slotParam: *mut SceAppUtilSaveDataSlotParam,
    pub reserved: [u8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataMountPoint {
    pub data: [u8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataRemoveItem {
    pub dataPath: *const crate::ctypes::c_char,
    pub mode: crate::ctypes::c_int,
    pub reserved: [u8; 36usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataSaveItem {
    pub dataPath: *const crate::ctypes::c_char,
    pub buf: *const crate::ctypes::c_void,
    pub pad: u32,
    pub offset: SceOff,
    pub mode: crate::ctypes::c_int,
    pub reserved: [u8; 36usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataSlot {
    pub id: SceAppUtilSaveDataSlotId,
    pub status: SceAppUtilSaveDataSlotStatus,
    pub userParam: crate::ctypes::c_int,
    pub emptyParam: *mut SceAppUtilSaveDataSlotEmptyParam,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataSlotEmptyParam {
    pub title: *mut SceWChar16,
    pub iconPath: *mut crate::ctypes::c_char,
    pub iconBuf: *mut crate::ctypes::c_void,
    pub iconBufSize: SceSize,
    pub reserved: [u8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilSaveDataSlotParam {
    pub status: SceAppUtilSaveDataSlotStatus,
    pub title: [SceWChar16; 32usize],
    pub subTitle: [SceWChar16; 64usize],
    pub detail: [SceWChar16; 256usize],
    pub iconPath: [crate::ctypes::c_char; 64usize],
    pub userParam: crate::ctypes::c_int,
    pub sizeKB: SceSize,
    pub modifiedTime: SceDateTime,
    pub reserved: [u8; 48usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilStoreBrowseParam {
    pub type_: crate::ctypes::c_uint,
    pub id: *const crate::ctypes::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAppUtilWebBrowserParam {
    pub str_: *const crate::ctypes::c_char,
    pub strlen: SceSize,
    pub launchMode: crate::ctypes::c_uint,
    pub reserved: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceArmCpuRegisters {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
    pub cpsr: u32,
    pub fpscr: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAtracContentInfo {
    pub size: SceUInt32,
    pub atracType: SceUInt32,
    pub channel: SceUInt32,
    pub samplingRate: SceUInt32,
    pub endSample: SceInt32,
    pub loopStartSample: SceInt32,
    pub loopEndSample: SceInt32,
    pub bitRate: SceUInt32,
    pub fixedEncBlockSize: SceUInt32,
    pub fixedEncBlockSample: SceUInt32,
    pub frameSample: SceUInt32,
    pub loopBlockOffset: SceUInt32,
    pub loopBlockSize: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAtracDecoderGroup {
    pub size: SceUInt32,
    pub wordLength: SceUInt32,
    pub totalCh: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAtracStreamInfo {
    pub size: SceUInt32,
    pub pWritePosition: *mut SceUChar8,
    pub readPosition: SceUInt32,
    pub writableSize: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudiodecCtrl {
    pub size: SceUInt32,
    pub handle: SceInt32,
    pub pEs: *mut SceUInt8,
    pub inputEsSize: SceUInt32,
    pub maxEsSize: SceUInt32,
    pub pPcm: *mut crate::ctypes::c_void,
    pub outputPcmSize: SceUInt32,
    pub maxPcmSize: SceUInt32,
    pub wordLength: SceUInt32,
    pub pInfo: *mut SceAudiodecInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceAudiodecInfo {
    pub size: SceUInt32,
    pub at9: SceAudiodecInfoAt9,
    pub mp3: SceAudiodecInfoMp3,
    pub aac: SceAudiodecInfoAac,
    pub celp: SceAudiodecInfoCelp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudiodecInfoAac {
    pub size: SceUInt32,
    pub isAdts: SceUInt32,
    pub ch: SceUInt32,
    pub samplingRate: SceUInt32,
    pub isSbr: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudiodecInfoAt9 {
    pub size: SceUInt32,
    pub configData: [SceUInt8; 4usize],
    pub ch: SceUInt32,
    pub bitRate: SceUInt32,
    pub samplingRate: SceUInt32,
    pub superFrameSize: SceUInt32,
    pub framesInSuperFrame: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudiodecInfoCelp {
    pub size: SceUInt32,
    pub excitationMode: SceUInt32,
    pub samplingRate: SceUInt32,
    pub bitRate: SceUInt32,
    pub lostCount: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudiodecInfoMp3 {
    pub size: SceUInt32,
    pub ch: SceUInt32,
    pub version: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudiodecInitChParam {
    pub size: SceUInt32,
    pub totalCh: SceUInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceAudiodecInitParam {
    pub size: SceUInt32,
    pub at9: SceAudiodecInitChParam,
    pub mp3: SceAudiodecInitStreamParam,
    pub aac: SceAudiodecInitStreamParam,
    pub celp: SceAudiodecInitStreamParam,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudiodecInitStreamParam {
    pub size: SceUInt32,
    pub totalStreams: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudioencCtrl {
    pub size: SceSize,
    pub handle: crate::ctypes::c_int,
    pub pInputPcm: *mut crate::ctypes::c_void,
    pub inputPcmSize: SceSize,
    pub maxPcmSize: SceSize,
    pub pOutputEs: *mut crate::ctypes::c_void,
    pub outputEsSize: SceSize,
    pub maxEsSize: SceSize,
    pub wordLength: SceSize,
    pub pInfo: *mut SceAudioencInfo,
    pub pOptInfo: *mut SceAudioencOptInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceAudioencInfo {
    pub size: SceSize,
    pub celp: SceAudioencInfoCelp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudioencInfoCelp {
    pub size: SceSize,
    pub excitationMode: crate::ctypes::c_uint,
    pub samplingRate: crate::ctypes::c_uint,
    pub bitRate: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceAudioencInitParam {
    pub size: SceSize,
    pub celp: SceAudioencInitStreamParam,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudioencInitStreamParam {
    pub size: SceSize,
    pub totalStreams: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceAudioencOptInfo {
    pub size: SceSize,
    pub celp: SceAudioencOptInfoCelp,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAudioencOptInfoCelp {
    pub size: SceSize,
    pub header: [u8; 32usize],
    pub headerSize: SceSize,
    pub encoderVersion: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAuthInfo {
    pub unk_0x00: SceUInt32,
    pub self_type: SceUInt32,
    pub request: SceSelfAuthInfo,
    pub response: SceSelfAuthInfo,
    pub media_type: SceUInt32,
    pub unk_0x12C: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvcdecArrayPicture {
    pub numOfOutput: u32,
    pub numOfElm: u32,
    pub pPicture: *mut *mut SceAvcdecPicture,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvcdecAu {
    pub pts: SceVideodecTimeStamp,
    pub dts: SceVideodecTimeStamp,
    pub es: SceAvcdecBuf,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvcdecBuf {
    pub pBuf: *mut crate::ctypes::c_void,
    pub size: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvcdecCtrl {
    pub handle: u32,
    pub frameBuf: SceAvcdecBuf,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvcdecDecoderInfo {
    pub frameMemSize: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceAvcdecFrame {
    pub pixelType: u32,
    pub framePitch: u32,
    pub frameWidth: u32,
    pub frameHeight: u32,
    pub horizontalSize: u32,
    pub verticalSize: u32,
    pub frameCropLeftOffset: u32,
    pub frameCropRightOffset: u32,
    pub frameCropTopOffset: u32,
    pub frameCropBottomOffset: u32,
    pub opt: SceAvcdecFrameOption,
    pub pPicture: [*mut crate::ctypes::c_void; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceAvcdecFrameOption {
    pub reserved: [u8; 16usize],
    pub rgba: SceAvcdecFrameOptionRGBA,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvcdecFrameOptionRGBA {
    pub alpha: u8,
    pub cscCoefficient: u8,
    pub reserved: [u8; 14usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvcdecInfo {
    pub numUnitsInTick: u32,
    pub timeScale: u32,
    pub fixedFrameRateFlag: u8,
    pub aspectRatioIdc: u8,
    pub sarWidth: u16,
    pub sarHeight: u16,
    pub colourPrimaries: u8,
    pub transferCharacteristics: u8,
    pub matrixCoefficients: u8,
    pub videoFullRangeFlag: u8,
    pub picStruct: [u8; 2usize],
    pub ctType: u8,
    pub pts: SceVideodecTimeStamp,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceAvcdecPicture {
    pub size: u32,
    pub frame: SceAvcdecFrame,
    pub info: SceAvcdecInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvcdecQueryDecoderInfo {
    pub horizontal: u32,
    pub vertical: u32,
    pub numOfRefFrames: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvPlayerAudio {
    pub channelCount: u16,
    pub reserved: [u8; 2usize],
    pub sampleRate: u32,
    pub size: u32,
    pub languageCode: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvPlayerEventReplacement {
    pub objectPointer: *mut crate::ctypes::c_void,
    pub eventCallback: SceAvPlayerEventCallback,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvPlayerFileReplacement {
    pub objectPointer: *mut crate::ctypes::c_void,
    pub open: SceAvPlayerOpenFile,
    pub close: SceAvPlayerCloseFile,
    pub readOffset: SceAvPlayerReadOffsetFile,
    pub size: SceAvPlayerSizeFile,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceAvPlayerFrameInfo {
    pub pData: *mut u8,
    pub reserved: u32,
    pub timeStamp: u64,
    pub details: SceAvPlayerStreamDetails,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvPlayerInitData {
    pub memoryReplacement: SceAvPlayerMemReplacement,
    pub fileReplacement: SceAvPlayerFileReplacement,
    pub eventReplacement: SceAvPlayerEventReplacement,
    pub debugLevel: i32,
    pub basePriority: u32,
    pub numOutputVideoFrameBuffers: i32,
    pub autoStart: SceBool,
    pub reserved: [u8; 3usize],
    pub defaultLanguage: *const crate::ctypes::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvPlayerMemReplacement {
    pub objectPointer: *mut crate::ctypes::c_void,
    pub allocate: SceAvPlayerAlloc,
    pub deallocate: SceAvPlayerFree,
    pub allocateTexture: SceAvPlayerAllocFrame,
    pub deallocateTexture: SceAvPlayerFreeFrame,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceAvPlayerStreamDetails {
    pub reserved: [u32; 4usize],
    pub audio: SceAvPlayerAudio,
    pub video: SceAvPlayerVideo,
    pub subs: SceAvPlayerTimedText,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceAvPlayerStreamInfo {
    pub type_: u32,
    pub reserved: u32,
    pub details: SceAvPlayerStreamDetails,
    pub duration: u64,
    pub startTime: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvPlayerTextPosition {
    pub top: u16,
    pub left: u16,
    pub bottom: u16,
    pub right: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvPlayerTimedText {
    pub languageCode: u32,
    pub textSize: u16,
    pub fontSize: u16,
    pub position: SceAvPlayerTextPosition,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceAvPlayerVideo {
    pub width: u32,
    pub height: u32,
    pub aspectRatio: f32,
    pub languageCode: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceBtEvent {
    pub __bindgen_anon_1: SceBtEvent__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceBtEvent__bindgen_ty_1 {
    pub data: [crate::ctypes::c_uchar; 16usize],
    pub __bindgen_anon_1: SceBtEvent__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceBtEvent__bindgen_ty_1__bindgen_ty_1 {
    pub id: crate::ctypes::c_uchar,
    pub unk1: crate::ctypes::c_uchar,
    pub unk2: crate::ctypes::c_ushort,
    pub unk3: crate::ctypes::c_uint,
    pub mac0: crate::ctypes::c_uint,
    pub mac1: crate::ctypes::c_uint,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _SceBtHidRequest {
    pub unk00: u32,
    pub unk04: u32,
    pub type_: u8,
    pub unk09: u8,
    pub unk0A: u8,
    pub unk0B: u8,
    pub buffer: *mut crate::ctypes::c_void,
    pub length: u32,
    pub next: *mut _SceBtHidRequest,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceBtRegisteredInfo {
    pub mac: [crate::ctypes::c_uchar; 6usize],
    pub unk0: crate::ctypes::c_ushort,
    pub bt_class: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_uint,
    pub unk2: crate::ctypes::c_uint,
    pub vid: crate::ctypes::c_ushort,
    pub pid: crate::ctypes::c_ushort,
    pub unk3: crate::ctypes::c_uint,
    pub unk4: crate::ctypes::c_uint,
    pub name: [crate::ctypes::c_char; 128usize],
    pub unk5: [crate::ctypes::c_uchar; 96usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCameraInfo {
    pub size: SceSize,
    pub priority: u16,
    pub format: u16,
    pub resolution: u16,
    pub framerate: u16,
    pub width: u16,
    pub height: u16,
    pub range: u16,
    pub pad: u16,
    pub sizeIBase: SceSize,
    pub sizeUBase: SceSize,
    pub sizeVBase: SceSize,
    pub pIBase: *mut crate::ctypes::c_void,
    pub pUBase: *mut crate::ctypes::c_void,
    pub pVBase: *mut crate::ctypes::c_void,
    pub pitch: u16,
    pub buffer: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCameraRead {
    pub size: SceSize,
    pub mode: crate::ctypes::c_int,
    pub pad: crate::ctypes::c_int,
    pub status: crate::ctypes::c_int,
    pub frame: u64,
    pub timestamp: u64,
    pub sizeIBase: SceSize,
    pub sizeUBase: SceSize,
    pub sizeVBase: SceSize,
    pub pIBase: *mut crate::ctypes::c_void,
    pub pUBase: *mut crate::ctypes::c_void,
    pub pVBase: *mut crate::ctypes::c_void,
}
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
#[derive(Debug, Copy, Clone)]
pub struct SceClibMspaceStats {
    pub capacity: SceSize,
    pub unk: SceSize,
    pub peak_in_use: SceSize,
    pub current_in_use: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCommonDialogColor {
    pub r: SceUInt8,
    pub g: SceUInt8,
    pub b: SceUInt8,
    pub a: SceUInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCommonDialogConfigParam {
    pub sdkVersion: SceUInt32,
    pub language: SceSystemParamLang,
    pub enterButtonAssign: SceSystemParamEnterButtonAssign,
    pub reserved: [SceUInt8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCommonDialogInfobarParam {
    pub visibility: SceInt32,
    pub color: SceInt32,
    pub transparency: SceInt32,
    pub reserved: [SceUInt8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCommonDialogParam {
    pub infobarParam: *mut SceCommonDialogInfobarParam,
    pub bgColor: *mut SceCommonDialogColor,
    pub dimmerColor: *mut SceCommonDialogColor,
    pub reserved: [SceUInt8; 60usize],
    pub magic: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCommonDialogRenderTargetInfo {
    pub depthSurfaceData: ScePVoid,
    pub colorSurfaceData: ScePVoid,
    pub surfaceType: SceGxmColorSurfaceType,
    pub colorFormat: SceGxmColorFormat,
    pub width: SceUInt32,
    pub height: SceUInt32,
    pub strideInPixels: SceUInt32,
    pub reserved: [SceUInt8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCommonDialogUpdateParam {
    pub renderTarget: SceCommonDialogRenderTargetInfo,
    pub displaySyncObject: *mut SceGxmSyncObject,
    pub reserved: [SceUInt8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCompatCdram {
    pub cached_cdram: *mut crate::ctypes::c_void,
    pub uncached_cdram: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceConsoleId {
    pub unk: u16,
    pub company_code: u16,
    pub product_code: u16,
    pub product_sub_code: u16,
    pub __bindgen_anon_1: SceConsoleId__bindgen_ty_1,
    pub unk3: [u8; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceConsoleId__bindgen_ty_1 {
    pub __bindgen_anon_1: SceConsoleId__bindgen_ty_1__bindgen_ty_1,
    pub chassis_check: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SceConsoleId__bindgen_ty_1__bindgen_ty_1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
}
impl SceConsoleId__bindgen_ty_1__bindgen_ty_1 {
    #[inline]
    pub fn unk2(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_unk2(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn factory_code(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 6u8) as u8) }
    }
    #[inline]
    pub fn set_factory_code(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(unk2: u8, factory_code: u8) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let unk2: u8 = unsafe { ::core::mem::transmute(unk2) };
            unk2 as u64
        });
        __bindgen_bitfield_unit.set(2usize, 6u8, {
            let factory_code: u8 = unsafe { ::core::mem::transmute(factory_code) };
            factory_code as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCoredumpTriggerParam {
    pub size: SceSize,
    pub data_0x04: crate::ctypes::c_int,
    pub data_0x08: crate::ctypes::c_int,
    pub data_0x0C: crate::ctypes::c_int,
    pub data_0x10: crate::ctypes::c_int,
    pub titleid_len: SceSize,
    pub titleid: *const crate::ctypes::c_char,
    pub app_name_len: SceSize,
    pub app_name: *const crate::ctypes::c_char,
    pub data_0x24: crate::ctypes::c_int,
    pub data_0x28: crate::ctypes::c_int,
    pub crash_thid: SceUID,
    pub data_0x30: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCorelockContext {
    pub lock: crate::ctypes::c_int,
    pub core_count: i16,
    pub last_wait_core: i16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCtrlActuator {
    pub small: crate::ctypes::c_uchar,
    pub large: crate::ctypes::c_uchar,
    pub unk: [u8; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCtrlData {
    pub timeStamp: u64,
    pub buttons: crate::ctypes::c_uint,
    pub lx: crate::ctypes::c_uchar,
    pub ly: crate::ctypes::c_uchar,
    pub rx: crate::ctypes::c_uchar,
    pub ry: crate::ctypes::c_uchar,
    pub up: u8,
    pub right: u8,
    pub down: u8,
    pub left: u8,
    pub lt: u8,
    pub rt: u8,
    pub l1: u8,
    pub r1: u8,
    pub triangle: u8,
    pub circle: u8,
    pub cross: u8,
    pub square: u8,
    pub reserved: [u8; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCtrlDataPsp {
    pub TimeStamp: crate::ctypes::c_uint,
    pub Buttons: crate::ctypes::c_uint,
    pub Lx: crate::ctypes::c_uchar,
    pub Ly: crate::ctypes::c_uchar,
    pub Rx: crate::ctypes::c_uchar,
    pub Ry: crate::ctypes::c_uchar,
    pub Rsrv: [crate::ctypes::c_uchar; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCtrlPortInfo {
    pub port: [u8; 5usize],
    pub unk: [u8; 11usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCtrlRapidFireRule {
    pub Mask: crate::ctypes::c_uint,
    pub Trigger: crate::ctypes::c_uint,
    pub Target: crate::ctypes::c_uint,
    pub Delay: crate::ctypes::c_uint,
    pub Make: crate::ctypes::c_uint,
    pub Break: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceCtrlVirtualControllerDriver {
    pub readButtons: ::core::option::Option<
        unsafe extern "C" fn(
            port: crate::ctypes::c_int,
            pad_data: *mut SceCtrlData,
            count: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub setActuator: ::core::option::Option<
        unsafe extern "C" fn(
            port: crate::ctypes::c_int,
            pState: *const SceCtrlActuator,
        ) -> crate::ctypes::c_int,
    >,
    pub getBatteryInfo: ::core::option::Option<
        unsafe extern "C" fn(
            port: crate::ctypes::c_int,
            batt: *mut SceUInt8,
        ) -> crate::ctypes::c_int,
    >,
    pub disconnect: ::core::option::Option<
        unsafe extern "C" fn(port: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
    pub setTurnOffInterval: ::core::option::Option<
        unsafe extern "C" fn(port: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
    pub getActiveControllerPort:
        ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub changePortAssign: ::core::option::Option<
        unsafe extern "C" fn(
            port1: crate::ctypes::c_int,
            port2: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub unk0: ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub getControllerPortInfo: ::core::option::Option<
        unsafe extern "C" fn(info: *mut SceCtrlPortInfo) -> crate::ctypes::c_int,
    >,
    pub setLightBar: ::core::option::Option<
        unsafe extern "C" fn(
            port: crate::ctypes::c_int,
            r: SceUInt8,
            g: SceUInt8,
            b: SceUInt8,
        ) -> crate::ctypes::c_int,
    >,
    pub resetLightBar: ::core::option::Option<
        unsafe extern "C" fn(port: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
    pub unk1: ::core::option::Option<
        unsafe extern "C" fn(port: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
    pub singleControllerMode: ::core::option::Option<
        unsafe extern "C" fn(port: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceDateTime {
    pub year: crate::ctypes::c_ushort,
    pub month: crate::ctypes::c_ushort,
    pub day: crate::ctypes::c_ushort,
    pub hour: crate::ctypes::c_ushort,
    pub minute: crate::ctypes::c_ushort,
    pub second: crate::ctypes::c_ushort,
    pub microsecond: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceDeflatePartialInputParam {
    pub size: u32,
    pub pBufEnd: *const crate::ctypes::c_void,
    pub cookie: *mut crate::ctypes::c_void,
    pub SceDeflateDecompressPartialInputCallback: ::core::option::Option<
        unsafe extern "C" fn(
            param: *mut SceDeflatePartialInputParam,
            outputsize: u32,
        ) -> *const crate::ctypes::c_void,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceDipsw {
    pub cp_timestamp_1: u32,
    pub cp_version: u16,
    pub cp_build_id: u16,
    pub cp_timestamp_2: u32,
    pub aslr_seed: u32,
    pub sce_sdk_flags: u32,
    pub shell_flags: u32,
    pub debug_control_flags: u32,
    pub system_control_flags: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceDisplayFrameBuf {
    pub size: SceSize,
    pub base: *mut crate::ctypes::c_void,
    pub pitch: crate::ctypes::c_uint,
    pub pixelformat: crate::ctypes::c_uint,
    pub width: crate::ctypes::c_uint,
    pub height: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceDisplayFrameBufInfo {
    pub size: SceSize,
    pub pid: SceUID,
    pub vblankcount: crate::ctypes::c_uint,
    pub paddr: usize,
    pub framebuf: SceDisplayFrameBuf,
    pub resolution: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceExcpmgrData {
    pub nestedExceptionCount: [crate::ctypes::c_int; 4usize],
    pub unused: [crate::ctypes::c_int; 4usize],
    pub ExcpStackTop: [*mut crate::ctypes::c_void; 4usize],
    pub ExcpStackBottom: [*mut crate::ctypes::c_void; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceExcpmgrExceptionContext {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub sp: u32,
    pub lr: u32,
    pub address_of_faulting_instruction: u32,
    pub ExceptionKind: SceExcpKind,
    pub SPSR: u32,
    pub CPACR: u32,
    pub FPSCR: u32,
    pub FPEXC: u32,
    pub CONTEXTIDR: u32,
    pub TPIDRURW: u32,
    pub TPIDRURO: u32,
    pub TPIDRPRW: u32,
    pub TTBR1: u32,
    pub unused68: u32,
    pub DACR: u32,
    pub DFSR: u32,
    pub IFSR: u32,
    pub DFAR: u32,
    pub IFAR: u32,
    pub PAR: u32,
    pub TEEHBR: u32,
    pub PMCR: u32,
    pub PMCNTENSET: u32,
    pub PMCNTENSET_2: u32,
    pub PMSELR: u32,
    pub PMCCNTR: u32,
    pub PMUSERENR: u32,
    pub PMXEVTYPER0: u32,
    pub PMXEVCNTR0: u32,
    pub PMXEVTYPER1: u32,
    pub PMXEVCNTR1: u32,
    pub PMXEVTYPER2: u32,
    pub PMXEVCNTR2: u32,
    pub PMXEVTYPER3: u32,
    pub PMXEVCNTR3: u32,
    pub PMXEVTYPER4: u32,
    pub PMXEVCNTR4: u32,
    pub PMXEVTYPER5: u32,
    pub PMXEVCNTR5: u32,
    pub unusedD0: u32,
    pub unkD4: u32,
    pub DBGSCRext: u32,
    pub unusedDC: [u32; 9usize],
    pub VFP_registers: [u64; 32usize],
    pub unk200: [u32; 128usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFColor {
    pub r: SceFloat,
    pub g: SceFloat,
    pub b: SceFloat,
    pub a: SceFloat,
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct SceFiber {
    pub reserved: [crate::ctypes::c_char; 128usize],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct SceFiberInfo {
    pub entry: SceFiberEntry,
    pub argOnInitialize: SceUInt32,
    pub addrContext: *mut crate::ctypes::c_void,
    pub sizeContext: SceSize,
    pub name: [crate::ctypes::c_char; 32usize],
    pub padding: [crate::ctypes::c_uint; 80usize],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct SceFiberOptParam {
    pub reserved: [crate::ctypes::c_char; 128usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFiosDHOpenSyncSyscallArgs {
    pub to_order: SceUInt8,
    pub padding: [crate::ctypes::c_int; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFiosGetListSyscallArgs {
    pub out_ids: *mut SceFiosKernelOverlayID,
    pub data_0x04: crate::ctypes::c_int,
    pub data_0x08: crate::ctypes::c_int,
    pub data_0x0C: SceSize,
    pub data_0x10: crate::ctypes::c_int,
    pub data_0x14: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFiosKernelOverlay {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFiosNativeDirEntry {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFiosNativeStat {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFiosOverlay {
    pub type_: u8,
    pub order: u8,
    pub dst_len: u16,
    pub src_len: u16,
    pub unk2: u16,
    pub pid: SceUID,
    pub id: SceFiosOverlayID,
    pub dst: [crate::ctypes::c_char; 292usize],
    pub src: [crate::ctypes::c_char; 292usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFiosResolveSyncSyscallArgs {
    pub out_path: *mut crate::ctypes::c_char,
    pub data_0x04: crate::ctypes::c_int,
    pub data_0x08: crate::ctypes::c_int,
    pub data_0x0C: crate::ctypes::c_int,
    pub data_0x10: crate::ctypes::c_int,
    pub data_0x14: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFiosResolveWithRangeSyncSyscallArgs {
    pub out_path: *mut crate::ctypes::c_char,
    pub data_0x04: crate::ctypes::c_int,
    pub data_0x08: SceUInt8,
    pub data_0x09: SceUInt8,
    pub data_0x0C: crate::ctypes::c_int,
    pub data_0x10: crate::ctypes::c_int,
    pub data_0x14: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFMatrix2 {
    pub x: SceFVector2,
    pub y: SceFVector2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFMatrix3 {
    pub x: SceFVector3,
    pub y: SceFVector3,
    pub z: SceFVector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFMatrix4 {
    pub x: SceFVector4,
    pub y: SceFVector4,
    pub z: SceFVector4,
    pub w: SceFVector4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFontCharInfo {
    pub bitmapWidth: crate::ctypes::c_uint,
    pub bitmapHeight: crate::ctypes::c_uint,
    pub bitmapLeft: crate::ctypes::c_uint,
    pub bitmapTop: crate::ctypes::c_uint,
    pub sfp26Width: crate::ctypes::c_uint,
    pub sfp26Height: crate::ctypes::c_uint,
    pub sfp26Ascender: crate::ctypes::c_int,
    pub sfp26Descender: crate::ctypes::c_int,
    pub sfp26BearingHX: crate::ctypes::c_int,
    pub sfp26BearingHY: crate::ctypes::c_int,
    pub sfp26BearingVX: crate::ctypes::c_int,
    pub sfp26BearingVY: crate::ctypes::c_int,
    pub sfp26AdvanceH: crate::ctypes::c_int,
    pub sfp26AdvanceV: crate::ctypes::c_int,
    pub shadowFlags: crate::ctypes::c_short,
    pub shadowId: crate::ctypes::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFontGlyphImage {
    pub pixelFormat: crate::ctypes::c_uint,
    pub xPos64: crate::ctypes::c_int,
    pub yPos64: crate::ctypes::c_int,
    pub bufWidth: crate::ctypes::c_ushort,
    pub bufHeight: crate::ctypes::c_ushort,
    pub bytesPerLine: crate::ctypes::c_ushort,
    pub pad: crate::ctypes::c_ushort,
    pub bufferPtr: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFontImageRect {
    pub width: crate::ctypes::c_ushort,
    pub height: crate::ctypes::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFontInfo {
    pub maxGlyphWidthI: crate::ctypes::c_uint,
    pub maxGlyphHeightI: crate::ctypes::c_uint,
    pub maxGlyphAscenderI: crate::ctypes::c_uint,
    pub maxGlyphDescenderI: crate::ctypes::c_uint,
    pub maxGlyphLeftXI: crate::ctypes::c_uint,
    pub maxGlyphBaseYI: crate::ctypes::c_uint,
    pub minGlyphCenterXI: crate::ctypes::c_uint,
    pub maxGlyphTopYI: crate::ctypes::c_uint,
    pub maxGlyphAdvanceXI: crate::ctypes::c_uint,
    pub maxGlyphAdvanceYI: crate::ctypes::c_uint,
    pub maxGlyphWidthF: f32,
    pub maxGlyphHeightF: f32,
    pub maxGlyphAscenderF: f32,
    pub maxGlyphDescenderF: f32,
    pub maxGlyphLeftXF: f32,
    pub maxGlyphBaseYF: f32,
    pub minGlyphCenterXF: f32,
    pub maxGlyphTopYF: f32,
    pub maxGlyphAdvanceXF: f32,
    pub maxGlyphAdvanceYF: f32,
    pub maxGlyphWidth: crate::ctypes::c_short,
    pub maxGlyphHeight: crate::ctypes::c_short,
    pub charMapLength: crate::ctypes::c_uint,
    pub shadowMapLength: crate::ctypes::c_uint,
    pub fontStyle: SceFontStyle,
    pub BPP: u8,
    pub pad: [u8; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFontNewLibParams {
    pub userData: *mut crate::ctypes::c_void,
    pub numFonts: crate::ctypes::c_uint,
    pub cacheData: *mut crate::ctypes::c_void,
    pub allocFunc: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut crate::ctypes::c_void,
            arg2: crate::ctypes::c_uint,
        ) -> *mut crate::ctypes::c_void,
    >,
    pub freeFunc: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut crate::ctypes::c_void, arg2: *mut crate::ctypes::c_void),
    >,
    pub openFunc: *mut crate::ctypes::c_void,
    pub closeFunc: *mut crate::ctypes::c_void,
    pub readFunc: *mut crate::ctypes::c_void,
    pub seekFunc: *mut crate::ctypes::c_void,
    pub errorFunc: *mut crate::ctypes::c_void,
    pub ioFinishFunc: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFontStyle {
    pub fontH: f32,
    pub fontV: f32,
    pub fontHRes: f32,
    pub fontVRes: f32,
    pub fontWeight: f32,
    pub fontFamily: crate::ctypes::c_ushort,
    pub fontStyle: crate::ctypes::c_ushort,
    pub fontStyleSub: crate::ctypes::c_ushort,
    pub fontLanguage: crate::ctypes::c_ushort,
    pub fontRegion: crate::ctypes::c_ushort,
    pub fontCountry: crate::ctypes::c_ushort,
    pub fontName: [crate::ctypes::c_char; 64usize],
    pub fontFileName: [crate::ctypes::c_char; 64usize],
    pub fontAttributes: crate::ctypes::c_uint,
    pub fontExpire: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFPlane {
    pub a: SceFloat,
    pub b: SceFloat,
    pub c: SceFloat,
    pub d: SceFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFQuaternion {
    pub x: SceFloat,
    pub y: SceFloat,
    pub z: SceFloat,
    pub w: SceFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFVector2 {
    pub x: SceFloat,
    pub y: SceFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFVector3 {
    pub x: SceFloat,
    pub y: SceFloat,
    pub z: SceFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFVector4 {
    pub x: SceFloat,
    pub y: SceFloat,
    pub z: SceFloat,
    pub w: SceFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGpsDeviceInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGpsPositionData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGpsSatelliteData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGpsStatus {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceGUIDKernelCreateOpt {
    pub __bindgen_anon_1: SceGUIDKernelCreateOpt__bindgen_ty_1,
    pub field_4: SceUInt32,
    pub field_8: SceUInt32,
    pub pid: SceUInt32,
    pub field_10: SceUInt32,
    pub field_14: SceUInt32,
    pub field_18: SceUInt32,
    pub field_1C: SceUInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceGUIDKernelCreateOpt__bindgen_ty_1 {
    pub flags: SceUInt32,
    pub attr: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmAuxiliarySurface {
    pub colorFormat: u32,
    pub type_: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub data: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmBlendInfo {
    pub colorMask: u8,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
}
impl SceGxmBlendInfo {
    #[inline]
    pub fn colorFunc(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_colorFunc(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn alphaFunc(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_alphaFunc(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn colorSrc(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_colorSrc(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn colorDst(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_colorDst(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn alphaSrc(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_alphaSrc(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn alphaDst(&self) -> u8 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(20usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_alphaDst(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(20usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        colorFunc: u8,
        alphaFunc: u8,
        colorSrc: u8,
        colorDst: u8,
        alphaSrc: u8,
        alphaDst: u8,
    ) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let colorFunc: u8 = unsafe { ::core::mem::transmute(colorFunc) };
            colorFunc as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let alphaFunc: u8 = unsafe { ::core::mem::transmute(alphaFunc) };
            alphaFunc as u64
        });
        __bindgen_bitfield_unit.set(8usize, 4u8, {
            let colorSrc: u8 = unsafe { ::core::mem::transmute(colorSrc) };
            colorSrc as u64
        });
        __bindgen_bitfield_unit.set(12usize, 4u8, {
            let colorDst: u8 = unsafe { ::core::mem::transmute(colorDst) };
            colorDst as u64
        });
        __bindgen_bitfield_unit.set(16usize, 4u8, {
            let alphaSrc: u8 = unsafe { ::core::mem::transmute(alphaSrc) };
            alphaSrc as u64
        });
        __bindgen_bitfield_unit.set(20usize, 4u8, {
            let alphaDst: u8 = unsafe { ::core::mem::transmute(alphaDst) };
            alphaDst as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceGxmColorSurface {
    pub pbeSidebandWord: crate::ctypes::c_uint,
    pub pbeEmitWords: [crate::ctypes::c_uint; 6usize],
    pub outputRegisterSize: crate::ctypes::c_uint,
    pub backgroundTex: SceGxmTexture,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmCommandList {
    pub words: [u32; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmContext {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmContextParams {
    pub hostMem: *mut crate::ctypes::c_void,
    pub hostMemSize: SceSize,
    pub vdmRingBufferMem: *mut crate::ctypes::c_void,
    pub vdmRingBufferMemSize: SceSize,
    pub vertexRingBufferMem: *mut crate::ctypes::c_void,
    pub vertexRingBufferMemSize: SceSize,
    pub fragmentRingBufferMem: *mut crate::ctypes::c_void,
    pub fragmentRingBufferMemSize: SceSize,
    pub fragmentUsseRingBufferMem: *mut crate::ctypes::c_void,
    pub fragmentUsseRingBufferMemSize: SceSize,
    pub fragmentUsseRingBufferOffset: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmDeferredContextParams {
    pub hostMem: *mut crate::ctypes::c_void,
    pub hostMemSize: SceSize,
    pub vdmCallback: ::core::option::Option<
        unsafe extern "C" fn(
            args: *mut crate::ctypes::c_void,
            requestedSize: SceSize,
            size: *mut SceSize,
        ) -> *mut crate::ctypes::c_void,
    >,
    pub vertexCallback: ::core::option::Option<
        unsafe extern "C" fn(
            args: *mut crate::ctypes::c_void,
            requestedSize: SceSize,
            size: *mut SceSize,
        ) -> *mut crate::ctypes::c_void,
    >,
    pub fragmentCallback: ::core::option::Option<
        unsafe extern "C" fn(
            args: *mut crate::ctypes::c_void,
            requestedSize: SceSize,
            size: *mut SceSize,
        ) -> *mut crate::ctypes::c_void,
    >,
    pub callbackData: *mut crate::ctypes::c_void,
    pub vdmRingBufferMem: *mut crate::ctypes::c_void,
    pub vdmRingBufferMemSize: SceSize,
    pub vertexRingBufferMem: *mut crate::ctypes::c_void,
    pub vertexRingBufferMemSize: SceSize,
    pub fragmentRingBufferMem: *mut crate::ctypes::c_void,
    pub fragmentRingBufferMemSize: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmDepthStencilSurface {
    pub zlsControl: crate::ctypes::c_uint,
    pub depthData: *mut crate::ctypes::c_void,
    pub stencilData: *mut crate::ctypes::c_void,
    pub backgroundDepth: f32,
    pub backgroundControl: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmFragmentProgram {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmInitializeParams {
    pub flags: crate::ctypes::c_uint,
    pub displayQueueMaxPendingCount: crate::ctypes::c_uint,
    pub displayQueueCallback: SceGxmDisplayQueueCallback,
    pub displayQueueCallbackDataSize: crate::ctypes::c_uint,
    pub parameterBufferSize: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmNotification {
    pub address: *mut crate::ctypes::c_uint,
    pub value: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmPrecomputedDraw {
    pub data: [crate::ctypes::c_uint; 11usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmPrecomputedFragmentState {
    pub data: [crate::ctypes::c_uint; 9usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmPrecomputedVertexState {
    pub data: [crate::ctypes::c_uint; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmProgram {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmProgramParameter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmRegisteredProgram {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmRenderTarget {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmRenderTargetParams {
    pub flags: u32,
    pub width: u16,
    pub height: u16,
    pub scenesPerFrame: u16,
    pub multisampleMode: u16,
    pub multisampleLocations: u32,
    pub driverMemBlock: SceUID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmShaderPatcher {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmShaderPatcherParams {
    pub userData: *mut crate::ctypes::c_void,
    pub hostAllocCallback: SceGxmShaderPatcherHostAllocCallback,
    pub hostFreeCallback: SceGxmShaderPatcherHostFreeCallback,
    pub bufferAllocCallback: SceGxmShaderPatcherBufferAllocCallback,
    pub bufferFreeCallback: SceGxmShaderPatcherBufferFreeCallback,
    pub bufferMem: *mut crate::ctypes::c_void,
    pub bufferMemSize: SceSize,
    pub vertexUsseAllocCallback: SceGxmShaderPatcherUsseAllocCallback,
    pub vertexUsseFreeCallback: SceGxmShaderPatcherUsseFreeCallback,
    pub vertexUsseMem: *mut crate::ctypes::c_void,
    pub vertexUsseMemSize: SceSize,
    pub vertexUsseOffset: crate::ctypes::c_uint,
    pub fragmentUsseAllocCallback: SceGxmShaderPatcherUsseAllocCallback,
    pub fragmentUsseFreeCallback: SceGxmShaderPatcherUsseFreeCallback,
    pub fragmentUsseMem: *mut crate::ctypes::c_void,
    pub fragmentUsseMemSize: SceSize,
    pub fragmentUsseOffset: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmSyncObject {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceGxmTexture {
    pub __bindgen_anon_1: SceGxmTexture__bindgen_ty_1,
    pub __bindgen_anon_2: SceGxmTexture__bindgen_ty_2,
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
impl SceGxmTexture {
    #[inline]
    pub fn lod_min0(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_lod_min0(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn data_addr(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 30u8) as u32) }
    }
    #[inline]
    pub fn set_data_addr(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 30u8, val as u64)
        }
    }
    #[inline]
    pub fn palette_addr(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(32usize, 26u8) as u32) }
    }
    #[inline]
    pub fn set_palette_addr(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(32usize, 26u8, val as u64)
        }
    }
    #[inline]
    pub fn lod_min1(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(58usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_lod_min1(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(58usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn swizzle_format(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(60usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_swizzle_format(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(60usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn normalize_mode(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(63usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_normalize_mode(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(63usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        lod_min0: u32,
        data_addr: u32,
        palette_addr: u32,
        lod_min1: u32,
        swizzle_format: u32,
        normalize_mode: u32,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let lod_min0: u32 = unsafe { ::core::mem::transmute(lod_min0) };
            lod_min0 as u64
        });
        __bindgen_bitfield_unit.set(2usize, 30u8, {
            let data_addr: u32 = unsafe { ::core::mem::transmute(data_addr) };
            data_addr as u64
        });
        __bindgen_bitfield_unit.set(32usize, 26u8, {
            let palette_addr: u32 = unsafe { ::core::mem::transmute(palette_addr) };
            palette_addr as u64
        });
        __bindgen_bitfield_unit.set(58usize, 2u8, {
            let lod_min1: u32 = unsafe { ::core::mem::transmute(lod_min1) };
            lod_min1 as u64
        });
        __bindgen_bitfield_unit.set(60usize, 3u8, {
            let swizzle_format: u32 = unsafe { ::core::mem::transmute(swizzle_format) };
            swizzle_format as u64
        });
        __bindgen_bitfield_unit.set(63usize, 1u8, {
            let normalize_mode: u32 = unsafe { ::core::mem::transmute(normalize_mode) };
            normalize_mode as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceGxmTexture__bindgen_ty_1 {
    pub generic: SceGxmTexture__bindgen_ty_1__bindgen_ty_1,
    pub linear_strided: SceGxmTexture__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmTexture__bindgen_ty_1__bindgen_ty_1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl SceGxmTexture__bindgen_ty_1__bindgen_ty_1 {
    #[inline]
    pub fn unk0(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_unk0(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn stride_ext(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_stride_ext(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn vaddr_mode(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_vaddr_mode(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn uaddr_mode(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_uaddr_mode(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn mip_filter(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_mip_filter(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn min_filter(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(10usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_min_filter(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(10usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn mag_filter(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_mag_filter(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn unk1(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(14usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_unk1(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(14usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn mip_count(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(17usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_mip_count(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(17usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn lod_bias(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(21usize, 6u8) as u32) }
    }
    #[inline]
    pub fn set_lod_bias(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(21usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub fn gamma_mode(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(27usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_gamma_mode(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(27usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn unk2(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(29usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_unk2(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(29usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn format0(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_format0(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        unk0: u32,
        stride_ext: u32,
        vaddr_mode: u32,
        uaddr_mode: u32,
        mip_filter: u32,
        min_filter: u32,
        mag_filter: u32,
        unk1: u32,
        mip_count: u32,
        lod_bias: u32,
        gamma_mode: u32,
        unk2: u32,
        format0: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let unk0: u32 = unsafe { ::core::mem::transmute(unk0) };
            unk0 as u64
        });
        __bindgen_bitfield_unit.set(1usize, 2u8, {
            let stride_ext: u32 = unsafe { ::core::mem::transmute(stride_ext) };
            stride_ext as u64
        });
        __bindgen_bitfield_unit.set(3usize, 3u8, {
            let vaddr_mode: u32 = unsafe { ::core::mem::transmute(vaddr_mode) };
            vaddr_mode as u64
        });
        __bindgen_bitfield_unit.set(6usize, 3u8, {
            let uaddr_mode: u32 = unsafe { ::core::mem::transmute(uaddr_mode) };
            uaddr_mode as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let mip_filter: u32 = unsafe { ::core::mem::transmute(mip_filter) };
            mip_filter as u64
        });
        __bindgen_bitfield_unit.set(10usize, 2u8, {
            let min_filter: u32 = unsafe { ::core::mem::transmute(min_filter) };
            min_filter as u64
        });
        __bindgen_bitfield_unit.set(12usize, 2u8, {
            let mag_filter: u32 = unsafe { ::core::mem::transmute(mag_filter) };
            mag_filter as u64
        });
        __bindgen_bitfield_unit.set(14usize, 3u8, {
            let unk1: u32 = unsafe { ::core::mem::transmute(unk1) };
            unk1 as u64
        });
        __bindgen_bitfield_unit.set(17usize, 4u8, {
            let mip_count: u32 = unsafe { ::core::mem::transmute(mip_count) };
            mip_count as u64
        });
        __bindgen_bitfield_unit.set(21usize, 6u8, {
            let lod_bias: u32 = unsafe { ::core::mem::transmute(lod_bias) };
            lod_bias as u64
        });
        __bindgen_bitfield_unit.set(27usize, 2u8, {
            let gamma_mode: u32 = unsafe { ::core::mem::transmute(gamma_mode) };
            gamma_mode as u64
        });
        __bindgen_bitfield_unit.set(29usize, 2u8, {
            let unk2: u32 = unsafe { ::core::mem::transmute(unk2) };
            unk2 as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let format0: u32 = unsafe { ::core::mem::transmute(format0) };
            format0 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmTexture__bindgen_ty_1__bindgen_ty_2 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl SceGxmTexture__bindgen_ty_1__bindgen_ty_2 {
    #[inline]
    pub fn unk0(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_unk0(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn stride_ext(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_stride_ext(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn vaddr_mode(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_vaddr_mode(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn uaddr_mode(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_uaddr_mode(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn stride_low(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(9usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_stride_low(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(9usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn mag_filter(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_mag_filter(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn unk1(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(14usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_unk1(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(14usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn stride(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(17usize, 10u8) as u32) }
    }
    #[inline]
    pub fn set_stride(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(17usize, 10u8, val as u64)
        }
    }
    #[inline]
    pub fn gamma_mode(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(27usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_gamma_mode(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(27usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn unk2(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(29usize, 2u8) as u32) }
    }
    #[inline]
    pub fn set_unk2(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(29usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn format0(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_format0(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        unk0: u32,
        stride_ext: u32,
        vaddr_mode: u32,
        uaddr_mode: u32,
        stride_low: u32,
        mag_filter: u32,
        unk1: u32,
        stride: u32,
        gamma_mode: u32,
        unk2: u32,
        format0: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let unk0: u32 = unsafe { ::core::mem::transmute(unk0) };
            unk0 as u64
        });
        __bindgen_bitfield_unit.set(1usize, 2u8, {
            let stride_ext: u32 = unsafe { ::core::mem::transmute(stride_ext) };
            stride_ext as u64
        });
        __bindgen_bitfield_unit.set(3usize, 3u8, {
            let vaddr_mode: u32 = unsafe { ::core::mem::transmute(vaddr_mode) };
            vaddr_mode as u64
        });
        __bindgen_bitfield_unit.set(6usize, 3u8, {
            let uaddr_mode: u32 = unsafe { ::core::mem::transmute(uaddr_mode) };
            uaddr_mode as u64
        });
        __bindgen_bitfield_unit.set(9usize, 3u8, {
            let stride_low: u32 = unsafe { ::core::mem::transmute(stride_low) };
            stride_low as u64
        });
        __bindgen_bitfield_unit.set(12usize, 2u8, {
            let mag_filter: u32 = unsafe { ::core::mem::transmute(mag_filter) };
            mag_filter as u64
        });
        __bindgen_bitfield_unit.set(14usize, 3u8, {
            let unk1: u32 = unsafe { ::core::mem::transmute(unk1) };
            unk1 as u64
        });
        __bindgen_bitfield_unit.set(17usize, 10u8, {
            let stride: u32 = unsafe { ::core::mem::transmute(stride) };
            stride as u64
        });
        __bindgen_bitfield_unit.set(27usize, 2u8, {
            let gamma_mode: u32 = unsafe { ::core::mem::transmute(gamma_mode) };
            gamma_mode as u64
        });
        __bindgen_bitfield_unit.set(29usize, 2u8, {
            let unk2: u32 = unsafe { ::core::mem::transmute(unk2) };
            unk2 as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let format0: u32 = unsafe { ::core::mem::transmute(format0) };
            format0 as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceGxmTexture__bindgen_ty_2 {
    pub generic2: SceGxmTexture__bindgen_ty_2__bindgen_ty_1,
    pub swizzled_cube: SceGxmTexture__bindgen_ty_2__bindgen_ty_2,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmTexture__bindgen_ty_2__bindgen_ty_1 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl SceGxmTexture__bindgen_ty_2__bindgen_ty_1 {
    #[inline]
    pub fn height(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 12u8) as u32) }
    }
    #[inline]
    pub fn set_height(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub fn width(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 12u8) as u32) }
    }
    #[inline]
    pub fn set_width(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub fn base_format(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(24usize, 5u8) as u32) }
    }
    #[inline]
    pub fn set_base_format(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(24usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn type_(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(29usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(29usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        height: u32,
        width: u32,
        base_format: u32,
        type_: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 12u8, {
            let height: u32 = unsafe { ::core::mem::transmute(height) };
            height as u64
        });
        __bindgen_bitfield_unit.set(12usize, 12u8, {
            let width: u32 = unsafe { ::core::mem::transmute(width) };
            width as u64
        });
        __bindgen_bitfield_unit.set(24usize, 5u8, {
            let base_format: u32 = unsafe { ::core::mem::transmute(base_format) };
            base_format as u64
        });
        __bindgen_bitfield_unit.set(29usize, 3u8, {
            let type_: u32 = unsafe { ::core::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmTexture__bindgen_ty_2__bindgen_ty_2 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl SceGxmTexture__bindgen_ty_2__bindgen_ty_2 {
    #[inline]
    pub fn height_pot(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_height_pot(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved0(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 12u8) as u32) }
    }
    #[inline]
    pub fn set_reserved0(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 12u8, val as u64)
        }
    }
    #[inline]
    pub fn width_pot(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_width_pot(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn reserved1(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(20usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_reserved1(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(20usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn base_format(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(24usize, 5u8) as u32) }
    }
    #[inline]
    pub fn set_base_format(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(24usize, 5u8, val as u64)
        }
    }
    #[inline]
    pub fn type_(&self) -> u32 {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(29usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(29usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        height_pot: u32,
        reserved0: u32,
        width_pot: u32,
        reserved1: u32,
        base_format: u32,
        type_: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let height_pot: u32 = unsafe { ::core::mem::transmute(height_pot) };
            height_pot as u64
        });
        __bindgen_bitfield_unit.set(4usize, 12u8, {
            let reserved0: u32 = unsafe { ::core::mem::transmute(reserved0) };
            reserved0 as u64
        });
        __bindgen_bitfield_unit.set(16usize, 4u8, {
            let width_pot: u32 = unsafe { ::core::mem::transmute(width_pot) };
            width_pot as u64
        });
        __bindgen_bitfield_unit.set(20usize, 4u8, {
            let reserved1: u32 = unsafe { ::core::mem::transmute(reserved1) };
            reserved1 as u64
        });
        __bindgen_bitfield_unit.set(24usize, 5u8, {
            let base_format: u32 = unsafe { ::core::mem::transmute(base_format) };
            base_format as u64
        });
        __bindgen_bitfield_unit.set(29usize, 3u8, {
            let type_: u32 = unsafe { ::core::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmValidRegion {
    pub xMax: u32,
    pub yMax: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmVertexAttribute {
    pub streamIndex: u16,
    pub offset: u16,
    pub format: u8,
    pub componentCount: u8,
    pub regIndex: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmVertexProgram {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxmVertexStream {
    pub stride: u16,
    pub indexSource: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxtHeader {
    pub tag: u32,
    pub version: u32,
    pub numTextures: u32,
    pub dataOffset: u32,
    pub dataSize: u32,
    pub numP4Palettes: u32,
    pub numP8Palettes: u32,
    pub pad: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceGxtTextureInfo {
    pub dataOffset: u32,
    pub dataSize: u32,
    pub paletteIndex: u32,
    pub flags: u32,
    pub type_: u32,
    pub format: u32,
    pub width: u16,
    pub height: u16,
    pub mipCount: u8,
    pub pad: [u8; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceHidKeyboardReport {
    pub reserved: SceUInt8,
    pub modifiers: [SceUInt8; 2usize],
    pub keycodes: [SceUInt8; 6usize],
    pub reserved2: [SceUInt8; 7usize],
    pub timestamp: SceUInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceHidMouseReport {
    pub buttons: SceUInt8,
    pub reserved: SceUInt8,
    pub rel_x: SceInt16,
    pub rel_y: SceInt16,
    pub wheel: SceInt8,
    pub tilt: SceInt8,
    pub timestamp: SceUInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceHttpMemoryPoolStats {
    pub poolSize: crate::ctypes::c_uint,
    pub maxInuseSize: crate::ctypes::c_uint,
    pub currentInuseSize: crate::ctypes::c_uint,
    pub reserved: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceHttpsCaList {
    pub caCerts: *mut *mut crate::ctypes::c_void,
    pub caNum: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceHttpsData {
    pub ptr: *mut crate::ctypes::c_char,
    pub size: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceHttpUriElement {
    pub opaque: crate::ctypes::c_int,
    pub scheme: *mut crate::ctypes::c_char,
    pub username: *mut crate::ctypes::c_char,
    pub password: *mut crate::ctypes::c_char,
    pub hostname: *mut crate::ctypes::c_char,
    pub path: *mut crate::ctypes::c_char,
    pub query: *mut crate::ctypes::c_char,
    pub fragment: *mut crate::ctypes::c_char,
    pub port: crate::ctypes::c_ushort,
    pub reserved: [crate::ctypes::c_uchar; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceI2cDebugHandlers {
    pub size: crate::ctypes::c_uint,
    pub write_start: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            addr: crate::ctypes::c_int,
            buffer: *mut crate::ctypes::c_uchar,
            size: crate::ctypes::c_int,
        ),
    >,
    pub write_end: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            error: crate::ctypes::c_int,
            result: crate::ctypes::c_int,
        ),
    >,
    pub read_start: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            addr: crate::ctypes::c_int,
            buffer: *mut crate::ctypes::c_uchar,
            size: crate::ctypes::c_int,
        ),
    >,
    pub read_end: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            error: crate::ctypes::c_int,
            result: crate::ctypes::c_int,
        ),
    >,
    pub write_read_start: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            write_addr: crate::ctypes::c_int,
            write_buffer: *mut crate::ctypes::c_uchar,
            write_size: crate::ctypes::c_int,
            read_addr: crate::ctypes::c_uint,
            read_buffer: *mut crate::ctypes::c_uchar,
            read_size: crate::ctypes::c_int,
        ),
    >,
    pub write_read_end: ::core::option::Option<
        unsafe extern "C" fn(
            bus: crate::ctypes::c_int,
            error: crate::ctypes::c_int,
            result: crate::ctypes::c_int,
        ),
    >,
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIMatrix2 {
    pub x: SceIVector2,
    pub y: SceIVector2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIMatrix3 {
    pub x: SceIVector3,
    pub y: SceIVector3,
    pub z: SceIVector3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIMatrix4 {
    pub x: SceIVector4,
    pub y: SceIVector4,
    pub z: SceIVector4,
    pub w: SceIVector4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceImeCaret {
    pub x: SceUInt32,
    pub y: SceUInt32,
    pub height: SceUInt32,
    pub index: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceImeDialogParam {
    pub sdkVersion: SceUInt32,
    pub inputMethod: SceUInt32,
    pub supportedLanguages: SceUInt64,
    pub languagesForced: SceBool,
    pub type_: SceUInt32,
    pub option: SceUInt32,
    pub filter: SceImeTextFilter,
    pub dialogMode: SceUInt32,
    pub textBoxMode: SceUInt32,
    pub title: *const SceWChar16,
    pub maxTextLength: SceUInt32,
    pub initialText: *mut SceWChar16,
    pub inputTextBuffer: *mut SceWChar16,
    pub commonParam: SceCommonDialogParam,
    pub enterLabel: SceUChar8,
    pub reserved: [SceChar8; 35usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceImeDialogResult {
    pub result: SceInt32,
    pub button: SceInt32,
    pub reserved: [SceChar8; 28usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceImeEditText {
    pub preeditIndex: SceUInt32,
    pub preeditLength: SceUInt32,
    pub caretIndex: SceUInt32,
    pub str_: *mut SceWChar16,
    pub editIndex: SceUInt32,
    pub editLengthChange: SceInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceImeEventData {
    pub id: SceUInt32,
    pub param: SceImeEventParam,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceImeEventParam {
    pub rect: SceImeRect,
    pub text: SceImeEditText,
    pub caretIndex: SceUInt32,
    pub reserved: [SceUChar8; 40usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceImeParam {
    pub sdkVersion: SceUInt32,
    pub inputMethod: SceUInt32,
    pub supportedLanguages: SceUInt64,
    pub languagesForced: SceBool,
    pub type_: SceUInt32,
    pub option: SceUInt32,
    pub work: *mut crate::ctypes::c_void,
    pub arg: *mut crate::ctypes::c_void,
    pub handler: SceImeEventHandler,
    pub filter: SceImeTextFilter,
    pub initialText: *mut SceWChar16,
    pub maxTextLength: SceUInt32,
    pub inputTextBuffer: *mut SceWChar16,
    pub enterLabel: SceUChar8,
    pub reserved: [SceUChar8; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceImePreeditGeometry {
    pub x: SceUInt32,
    pub y: SceUInt32,
    pub height: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceImeRect {
    pub x: SceUInt32,
    pub y: SceUInt32,
    pub width: SceUInt32,
    pub height: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIncomingDialogParam {
    pub sdkVersion: SceInt32,
    pub audioPath: [SceChar8; 128usize],
    pub titleid: [SceChar8; 16usize],
    pub unk_BC: SceInt32,
    pub dialogTimer: SceUInt32,
    pub reserved1: [SceChar8; 62usize],
    pub buttonRightText: [SceWChar16; 31usize],
    pub separator0: SceInt16,
    pub buttonLeftText: [SceWChar16; 31usize],
    pub separator1: SceInt16,
    pub dialogText: [SceWChar16; 128usize],
    pub separator2: SceInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIoDevInfo {
    pub max_size: SceOff,
    pub free_size: SceOff,
    pub cluster_size: SceSize,
    pub unk: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIoDirent {
    pub d_stat: SceIoStat,
    pub d_name: [crate::ctypes::c_char; 256usize],
    pub d_private: *mut crate::ctypes::c_void,
    pub dummy: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIoFdInfo {
    pub fd: SceUID,
    pub pid: SceUID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIofileInfo {
    pub path: [crate::ctypes::c_char; 1024usize],
    pub path2: [crate::ctypes::c_char; 1024usize],
    pub pid: SceUID,
    pub data_0x804: crate::ctypes::c_int,
    pub data_0x808: crate::ctypes::c_int,
    pub data_0x80C: crate::ctypes::c_int,
    pub data_0x810: crate::ctypes::c_int,
    pub data_0x814: crate::ctypes::c_int,
    pub data_0x818: crate::ctypes::c_int,
    pub data_0x81C: crate::ctypes::c_int,
    pub data_0x820: crate::ctypes::c_int,
    pub data_0x824: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub struct SceIVector2 {
    pub x: SceInt,
    pub y: SceInt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIVector3 {
    pub x: SceInt,
    pub y: SceInt,
    pub z: SceInt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceIVector4 {
    pub x: SceInt,
    pub y: SceInt,
    pub z: SceInt,
    pub w: SceInt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceJpegEncoderInitParam {
    pub size: SceSize,
    pub inWidth: crate::ctypes::c_int,
    pub inHeight: crate::ctypes::c_int,
    pub pixelFormat: crate::ctypes::c_int,
    pub outBuffer: *mut crate::ctypes::c_void,
    pub outSize: SceSize,
    pub option: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceJpegMJpegInitParam {
    pub size: SceSize,
    pub decoderCount: SceInt32,
    pub options: SceInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceJpegOutputInfo {
    pub colorSpace: SceInt32,
    pub width: SceUInt16,
    pub height: SceUInt16,
    pub outputSize: SceUInt32,
    pub unk_0xc: SceUInt32,
    pub unk_0x10: SceUInt32,
    pub pitch: [SceJpegPitch; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceJpegPitch {
    pub x: SceUInt32,
    pub y: SceUInt32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SceKblParam {
    pub version: u16,
    pub size: u16,
    pub current_fw_version: u32,
    pub factory_fw_version: u32,
    pub unk_C: u32,
    pub unk_10: u32,
    pub unk_14: [u8; 12usize],
    pub qa_flags: [u8; 16usize],
    pub boot_flags: [u8; 16usize],
    pub dipsw: SceDipsw,
    pub dram: SceKernelPARange,
    pub unk_68: u32,
    pub boot_type_indicator_1: u32,
    pub openpsid: SceOpenPsId,
    pub secure_kernel: SceKernelPARange,
    pub context_auth_sm: SceKernelPARange,
    pub kprx_auth_sm: SceKernelPARange,
    pub prog_rvk: SceKernelPARange,
    pub pscode: ScePsCode,
    pub __stack_chk_guard: u32,
    pub unk_AC: u32,
    pub session_id: [u8; 16usize],
    pub unk_C0: u32,
    pub wakeup_factor: u32,
    pub unk_C8: u32,
    pub hold_ctrl: u32,
    pub resume_context_addr: u32,
    pub hardware_info: u32,
    pub boot_type_indicator_2: u32,
    pub unk_DC: u32,
    pub unk_E0: u32,
    pub unk_E4: u32,
    pub hardware_flags: [u8; 16usize],
    pub bootldr_revision: u32,
    pub magic: u32,
    pub coredump_session_key: [u8; 32usize],
    pub unused: [u8; 224usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelAddrPair {
    pub addr: u32,
    pub length: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelAllocMemBlockKernelOpt {
    pub size: SceSize,
    pub field_4: SceUInt32,
    pub attr: SceUInt32,
    pub field_C: SceUInt32,
    pub paddr: SceUInt32,
    pub alignment: SceSize,
    pub extraLow: SceUInt32,
    pub extraHigh: SceUInt32,
    pub mirror_blockid: SceUInt32,
    pub pid: SceUID,
    pub paddr_list: *mut SceKernelPaddrList,
    pub field_2C: SceUInt32,
    pub field_30: SceUInt32,
    pub field_34: SceUInt32,
    pub field_38: SceUInt32,
    pub field_3C: SceUInt32,
    pub field_40: SceUInt32,
    pub field_44: SceUInt32,
    pub field_48: SceUInt32,
    pub field_4C: SceUInt32,
    pub field_50: SceUInt32,
    pub field_54: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelAllocMemBlockOpt {
    pub size: SceSize,
    pub attr: SceUInt32,
    pub alignment: SceSize,
    pub uidBaseBlock: SceUInt32,
    pub strBaseBlockName: *const crate::ctypes::c_char,
    pub flags: crate::ctypes::c_int,
    pub reserved: [crate::ctypes::c_int; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelBootArgs {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelCallbackInfo {
    pub size: SceSize,
    pub callbackId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub threadId: SceUID,
    pub callback: SceKernelCallbackFunction,
    pub common: *mut crate::ctypes::c_void,
    pub notifyCount: crate::ctypes::c_int,
    pub notifyArg: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelCondInfo {
    pub size: SceSize,
    pub condId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: SceUInt,
    pub mutexId: SceUID,
    pub numWaitThreads: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelCondOptParam {
    pub size: SceSize,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SceKernelDebugEventLog {
    pub size: SceSize,
    pub data_0x04: crate::ctypes::c_int,
    pub titleid: [crate::ctypes::c_char; 12usize],
    pub flags: crate::ctypes::c_int,
    pub ppid: SceUID,
    pub data_0x1C: SceUID,
    pub rsvd: [crate::ctypes::c_int; 4usize],
    pub time: SceUInt64,
    pub data_0x38: crate::ctypes::c_int,
    pub item_size: SceSize,
    pub __bindgen_anon_1: SceKernelDebugEventLog__bindgen_ty_1,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDebugEventLog1 {
    pub data_0x40: crate::ctypes::c_int,
    pub pid: SceUID,
    pub budget_type: crate::ctypes::c_int,
    pub data_0x4C: crate::ctypes::c_int,
    pub titleid: [crate::ctypes::c_char; 12usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDebugEventLog2 {
    pub data_0x40: crate::ctypes::c_int,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDebugEventLog3 {
    pub data_0x40: crate::ctypes::c_int,
    pub ip1: [crate::ctypes::c_char; 16usize],
    pub ip2: [crate::ctypes::c_char; 16usize],
    pub ip3: [crate::ctypes::c_char; 16usize],
    pub ip4: [crate::ctypes::c_char; 16usize],
    pub ip5: [crate::ctypes::c_char; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceKernelDebugEventLog__bindgen_ty_1 {
    pub type1: SceKernelDebugEventLog1,
    pub type2: SceKernelDebugEventLog2,
    pub type3: SceKernelDebugEventLog3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelDebugInfo {
    pub __bindgen_anon_1: SceKernelDebugInfo__bindgen_ty_1,
    pub func: *const crate::ctypes::c_char,
    pub line: SceUInt32,
    pub file: *const crate::ctypes::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceKernelDebugInfo__bindgen_ty_1 {
    pub __bindgen_anon_1: SceKernelDebugInfo__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: SceKernelDebugInfo__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDebugInfo__bindgen_ty_1__bindgen_ty_1 {
    pub fileHash: SceUInt32,
    pub lineHash: SceUInt32,
    pub funcHash: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDebugInfo__bindgen_ty_1__bindgen_ty_2 {
    pub hex_value0_hi: SceUInt32,
    pub hex_value0_lo: SceUInt32,
    pub hex_value1: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDmaOpChainParam {
    pub size: SceSize,
    pub coherencyMask: SceUInt32,
    pub setValue: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDmaOpDirectParam {
    pub src: *const crate::ctypes::c_void,
    pub dst: *mut crate::ctypes::c_void,
    pub len: SceUInt32,
    pub cmd: SceUInt32,
    pub blockSize: SceUInt32,
    pub coherencyMask: SceUInt32,
    pub setValue: SceUInt32,
    pub encDec: SceKernelDmaOpEncDec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDmaOpEncDec {
    pub keyring: SceUInt32,
    pub iv: *mut crate::ctypes::c_void,
    pub ivCoherencyMask: SceUInt32,
    pub reserved: SceUInt32,
    pub key: [SceUInt8; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDmaOpEncDecChainParam {
    pub header: SceKernelDmaOpChainParam,
    pub encDec: SceKernelDmaOpEncDec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelDmaOpTag {
    pub src: *const crate::ctypes::c_void,
    pub dst: *mut crate::ctypes::c_void,
    pub len: SceUInt32,
    pub cmd: SceUInt32,
    pub keyring: SceUInt32,
    pub iv: *mut crate::ctypes::c_void,
    pub blockSize: SceUInt32,
    pub pNext: *mut SceKernelDmaOpTag,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelEventFlagInfo {
    pub size: SceSize,
    pub evfId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: SceUInt,
    pub initPattern: SceUInt,
    pub currentPattern: SceUInt,
    pub numWaitThreads: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelEventFlagOptParam {
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelFastMutex {
    pub data: [SceUInt8; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelFaultingProcessInfo {
    pub pid: SceUID,
    pub faultingThreadId: SceUID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelFreeMemorySizeInfo {
    pub size: crate::ctypes::c_int,
    pub size_user: crate::ctypes::c_int,
    pub size_cdram: crate::ctypes::c_int,
    pub size_phycont: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelHeapCreateOpt {
    pub size: SceSize,
    pub __bindgen_anon_1: SceKernelHeapCreateOpt__bindgen_ty_1,
    pub field_8: SceUInt32,
    pub field_C: SceUInt32,
    pub memtype: SceUInt32,
    pub field_14: SceUInt32,
    pub field_18: SceUInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceKernelHeapCreateOpt__bindgen_ty_1 {
    pub attr: SceUInt32,
    pub uselock: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelIntrOptHandlers {
    pub size: u32,
    pub pre_register_subintr_cb: *mut SceKernelIntrOptHandlersCb3,
    pub post_register_subintr_cb: *mut SceKernelIntrOptHandlersCb3,
    pub release_subintr_cb: *mut SceKernelIntrOptHandlersCb1,
    pub fptr0: *mut SceKernelIntrOptHandlersCb1,
    pub enable_subintr_cb: *mut SceKernelIntrOptHandlersCb1,
    pub disable_subintr_cb: *mut SceKernelIntrOptHandlersCb1,
    pub fptr3: *mut SceKernelIntrOptHandlersCb2,
    pub fptr4: *mut SceKernelIntrOptHandlersCb1,
    pub fptr5: *mut SceKernelIntrOptHandlersCb1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelIntrOptParam {
    pub size: u32,
    pub num: u32,
    pub handlers: *mut SceKernelIntrOptHandlers,
    pub unk_C: u32,
    pub unk_10: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLibraryInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLMOption {
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLoadModuleOption {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLwCondInfo {
    pub size: SceSize,
    pub lwcond_id: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: crate::ctypes::c_int,
    pub work: *mut SceKernelLwCondWork,
    pub lwmutex: *mut SceKernelLwMutexWork,
    pub num_wait_threads: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLwCondOptParam {
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLwCondWork {
    pub data: [SceInt32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLwMutexInfo {
    pub size: SceSize,
    pub mtxid: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: crate::ctypes::c_int,
    pub work: *mut SceKernelLwMutexWork,
    pub init_count: crate::ctypes::c_int,
    pub current_count: crate::ctypes::c_int,
    pub current_owner_id: SceUID,
    pub num_wait_threads: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLwMutexOptParam {
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelLwMutexWork {
    pub data: [SceInt64; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelMemBlockInfo {
    pub size: SceSize,
    pub mappedBase: *mut crate::ctypes::c_void,
    pub mappedSize: SceSize,
    pub memoryType: crate::ctypes::c_int,
    pub access: SceUInt32,
    pub type_: SceKernelMemBlockType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelModuleInfo {
    pub size: SceSize,
    pub modid: SceUID,
    pub modattr: u16,
    pub modver: [u8; 2usize],
    pub module_name: [crate::ctypes::c_char; 28usize],
    pub unk28: SceUInt,
    pub start_entry: *mut crate::ctypes::c_void,
    pub stop_entry: *mut crate::ctypes::c_void,
    pub exit_entry: *mut crate::ctypes::c_void,
    pub exidx_top: *mut crate::ctypes::c_void,
    pub exidx_btm: *mut crate::ctypes::c_void,
    pub extab_top: *mut crate::ctypes::c_void,
    pub extab_btm: *mut crate::ctypes::c_void,
    pub tlsInit: *mut crate::ctypes::c_void,
    pub tlsInitSize: SceSize,
    pub tlsAreaSize: SceSize,
    pub path: [crate::ctypes::c_char; 256usize],
    pub segments: [SceKernelSegmentInfo; 4usize],
    pub state: SceUInt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelModuleLibraryInfo {
    pub size: SceSize,
    pub library_id: SceUID,
    pub libnid: u32,
    pub version: u16,
    pub flags: u16,
    pub entry_num_function: u16,
    pub entry_num_variable: u16,
    pub unk_0x14: u16,
    pub unk_0x16: u16,
    pub library_name: [crate::ctypes::c_char; 256usize],
    pub number_of_imported: SceSize,
    pub modid2: SceUID,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelModuleListInfo {
    pub size: SceSize,
    pub modid: SceUID,
    pub version: u32,
    pub module_version: u32,
    pub unk10: u32,
    pub unk14: *mut crate::ctypes::c_void,
    pub unk18: u32,
    pub unk1C: *mut crate::ctypes::c_void,
    pub unk20: *mut crate::ctypes::c_void,
    pub module_name: [crate::ctypes::c_char; 28usize],
    pub unk40: u32,
    pub unk44: u32,
    pub nid: u32,
    pub segments_num: SceSize,
    pub __bindgen_anon_1: SceKernelModuleListInfo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceKernelModuleListInfo__bindgen_ty_1 {
    pub seg1: SceKernelModuleListInfo__bindgen_ty_1__bindgen_ty_1,
    pub seg2: SceKernelModuleListInfo__bindgen_ty_1__bindgen_ty_2,
    pub seg3: SceKernelModuleListInfo__bindgen_ty_1__bindgen_ty_3,
    pub seg4: SceKernelModuleListInfo__bindgen_ty_1__bindgen_ty_4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelModuleListInfo__bindgen_ty_1__bindgen_ty_1 {
    pub SegmentInfo: [SceKernelSegmentInfo2; 1usize],
    pub addr: [u32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelModuleListInfo__bindgen_ty_1__bindgen_ty_2 {
    pub SegmentInfo: [SceKernelSegmentInfo2; 2usize],
    pub addr: [u32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelModuleListInfo__bindgen_ty_1__bindgen_ty_3 {
    pub SegmentInfo: [SceKernelSegmentInfo2; 3usize],
    pub addr: [u32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelModuleListInfo__bindgen_ty_1__bindgen_ty_4 {
    pub SegmentInfo: [SceKernelSegmentInfo2; 4usize],
    pub addr: [u32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelModuleName {
    pub s: [crate::ctypes::c_char; 28usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelMppInfo {
    pub size: SceSize,
    pub mppId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: SceUInt,
    pub bufSize: crate::ctypes::c_int,
    pub freeSize: crate::ctypes::c_int,
    pub numSendWaitThreads: crate::ctypes::c_int,
    pub numReceiveWaitThreads: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelMsgPipeInfo {
    pub size: SceSize,
    pub msgpipe_id: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: crate::ctypes::c_int,
    pub buffer_size: SceSize,
    pub free_size: SceSize,
    pub num_send_wait_threads: crate::ctypes::c_int,
    pub num_receive_wait_threads: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelMutexInfo {
    pub size: SceSize,
    pub mutexId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: SceUInt,
    pub initCount: crate::ctypes::c_int,
    pub currentCount: crate::ctypes::c_int,
    pub currentOwnerId: SceUID,
    pub numWaitThreads: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelMutexOptParam {
    pub size: SceSize,
    pub ceilingPriority: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelOpenPsId {
    pub id: [crate::ctypes::c_char; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPaArmTraceParam {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPaCounterTraceParam {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPaGpuSampledData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPaGpuTraceParam {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPARange {
    pub addr: u32,
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPaTraceBufferParam {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelPAVector {
    pub size: SceSize,
    pub __bindgen_anon_1: SceKernelPAVector__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceKernelPAVector__bindgen_ty_1 {
    pub __bindgen_anon_1: SceKernelPAVector__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: SceKernelPAVector__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPAVector__bindgen_ty_1__bindgen_ty_1 {
    pub ranges_size: u32,
    pub data_in_vector: u32,
    pub count: u32,
    pub ranges: *mut SceKernelPARange,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelPAVector__bindgen_ty_1__bindgen_ty_2 {
    pub list_size: u32,
    pub ret_length: u32,
    pub ret_count: u32,
    pub list: *mut SceKernelAddrPair,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelProcessContext {
    pub TTBR1: SceUInt32,
    pub DACR: SceUInt32,
    pub CONTEXTIDR: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelProcessInfo {
    pub size: SceSize,
    pub pid: SceUID,
    pub unk1: crate::ctypes::c_int,
    pub unk2: crate::ctypes::c_int,
    pub unk3: crate::ctypes::c_int,
    pub ppid: SceUID,
    pub unk: [crate::ctypes::c_int; 52usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelRWLockInfo {
    pub size: SceSize,
    pub rwLockId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: SceUInt32,
    pub lockCount: SceInt32,
    pub writeOwnerId: SceUID,
    pub numReadWaitThreads: SceUInt32,
    pub numWriteWaitThreads: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelRWLockOptParam {
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSegmentInfo {
    pub size: SceSize,
    pub perms: SceUInt,
    pub vaddr: *mut crate::ctypes::c_void,
    pub memsz: SceSize,
    pub filesz: SceSize,
    pub res: SceUInt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSegmentInfo2 {
    pub size: SceSize,
    pub perm: crate::ctypes::c_int,
    pub vaddr: *mut crate::ctypes::c_void,
    pub memsz: u32,
    pub unk_10: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSemaInfo {
    pub size: SceSize,
    pub semaId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: SceUInt,
    pub initCount: crate::ctypes::c_int,
    pub currentCount: crate::ctypes::c_int,
    pub maxCount: crate::ctypes::c_int,
    pub numWaitThreads: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSemaOptParam {
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSysrootSelfInfo {
    pub size: SceSize,
    pub self_data: *mut crate::ctypes::c_void,
    pub self_size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSystemInfo {
    pub size: SceSize,
    pub activeCpuMask: SceUInt32,
    pub cpuInfo: [SceKernelSystemInfo__bindgen_ty_1; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSystemInfo__bindgen_ty_1 {
    pub idleClock: SceKernelSysClock,
    pub comesOutOfIdleCount: SceUInt32,
    pub threadSwitchCount: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelSystemSwVersion {
    pub size: SceSize,
    pub versionString: [crate::ctypes::c_char; 28usize],
    pub version: SceUInt,
    pub unk_24: SceUInt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelThreadContextInfo {
    pub process_id: ScePID,
    pub thread_id: SceUID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelThreadInfo {
    pub size: SceSize,
    pub processId: SceUID,
    pub name: [crate::ctypes::c_char; 32usize],
    pub attr: SceUInt32,
    pub status: SceUInt32,
    pub entry: SceKernelThreadEntry,
    pub stack: *mut crate::ctypes::c_void,
    pub stackSize: SceInt32,
    pub initPriority: SceInt32,
    pub currentPriority: SceInt32,
    pub initCpuAffinityMask: SceInt32,
    pub currentCpuAffinityMask: SceInt32,
    pub currentCpuId: SceInt32,
    pub lastExecutedCpuId: SceInt32,
    pub waitType: SceUInt32,
    pub waitId: SceUID,
    pub exitStatus: SceInt32,
    pub runClocks: SceKernelSysClock,
    pub intrPreemptCount: SceUInt32,
    pub threadPreemptCount: SceUInt32,
    pub threadReleaseCount: SceUInt32,
    pub changeCpuCount: SceInt32,
    pub fNotifyCallback: SceInt32,
    pub reserved: SceInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelThreadOptParam {
    pub size: SceSize,
    pub attr: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelThreadRunStatus {
    pub size: SceSize,
    pub cpuInfo: [SceKernelThreadRunStatus__bindgen_ty_1; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelThreadRunStatus__bindgen_ty_1 {
    pub processId: SceUID,
    pub threadId: SceUID,
    pub priority: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelTimeval {
    pub sec: SceInt32,
    pub usec: SceInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelTimezone {
    pub value: SceUInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelULMOption {
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelUnloadModuleOption {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceKernelVARange {
    pub addr: u32,
    pub size: SceSize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceLocationHeadingInfo {
    pub trueHeading: SceFloat32,
    pub headingVectorX: SceFloat32,
    pub headingVectorY: SceFloat32,
    pub headingVectorZ: SceFloat32,
    pub reserve: SceFloat32,
    pub reserve2: SceFloat32,
    pub timestamp: SceRtcTick,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceLocationLocationInfo {
    pub latitude: SceDouble64,
    pub longitude: SceDouble64,
    pub altitude: SceDouble64,
    pub accuracy: SceFloat32,
    pub reserve: SceFloat32,
    pub direction: SceFloat32,
    pub speed: SceFloat32,
    pub timestamp: SceRtcTick,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceLocationPermissionInfo {
    pub parentalstatus: SceLocationPermissionStatus,
    pub mainstatus: SceLocationPermissionStatus,
    pub applicationstatus: SceLocationPermissionApplicationStatus,
    pub unk_0x0C: crate::ctypes::c_int,
    pub unk_0x10: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevAccCalibData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevCalibrationData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevCalibrationHeader {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevDeviceInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevDeviceLocation {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevGyroBiasData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevGyroCalibData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDeviceLocation {
    pub accelerometer: SceFVector3,
    pub gyro: SceFVector3,
    pub reserved: [u8; 24usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevMagnCalibData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionDevModeInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionSensorState {
    pub accelerometer: SceFVector3,
    pub gyro: SceFVector3,
    pub reserved1: [u8; 12usize],
    pub timestamp: crate::ctypes::c_uint,
    pub counter: crate::ctypes::c_uint,
    pub reserved2: [u8; 4usize],
    pub hostTimestamp: SceULong64,
    pub unknown: u8,
    pub reserved3: [u8; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMotionState {
    pub timestamp: crate::ctypes::c_uint,
    pub acceleration: SceFVector3,
    pub angularVelocity: SceFVector3,
    pub reserved1: [u8; 12usize],
    pub deviceQuat: SceFQuaternion,
    pub rotationMatrix: SceFMatrix4,
    pub nedMatrix: SceFMatrix4,
    pub reserved2: [u8; 4usize],
    pub basicOrientation: SceFVector3,
    pub hostTimestamp: SceULong64,
    pub reserved3: [u8; 36usize],
    pub magFieldStability: u8,
    pub unknown: u8,
    pub reserved4: [u8; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMsgDialogButtonsParam {
    pub msg1: *const crate::ctypes::c_char,
    pub fontSize1: SceInt32,
    pub msg2: *const crate::ctypes::c_char,
    pub fontSize2: SceInt32,
    pub msg3: *const crate::ctypes::c_char,
    pub fontSize3: SceInt32,
    pub reserved: [SceChar8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMsgDialogErrorCodeParam {
    pub errorCode: SceInt32,
    pub reserved: [SceChar8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMsgDialogParam {
    pub sdkVersion: SceUInt32,
    pub commonParam: SceCommonDialogParam,
    pub mode: SceInt32,
    pub userMsgParam: *mut SceMsgDialogUserMessageParam,
    pub sysMsgParam: *mut SceMsgDialogSystemMessageParam,
    pub errorCodeParam: *mut SceMsgDialogErrorCodeParam,
    pub progBarParam: *mut SceMsgDialogProgressBarParam,
    pub flag: SceInt32,
    pub reserved: [SceChar8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMsgDialogProgressBarParam {
    pub barType: SceInt32,
    pub sysMsgParam: SceMsgDialogSystemMessageParam,
    pub msg: *const SceChar8,
    pub reserved: [SceInt32; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMsgDialogResult {
    pub mode: SceInt32,
    pub result: SceInt32,
    pub buttonId: SceInt32,
    pub reserved: [SceChar8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMsgDialogSystemMessageParam {
    pub sysMsgType: SceInt32,
    pub value: SceInt32,
    pub reserved: [SceChar8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMsgDialogUserMessageParam {
    pub buttonType: SceInt32,
    pub msg: *const SceChar8,
    pub buttonParam: *mut SceMsgDialogButtonsParam,
    pub reserved: [SceChar8; 28usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceMsInfo {
    pub unk_0x00: crate::ctypes::c_int,
    pub unk_0x04: crate::ctypes::c_int,
    pub nbytes: SceUInt64,
    pub nbytes2: SceUInt64,
    pub sector_size: SceUInt32,
    pub unk_0x1C: crate::ctypes::c_int,
    pub fs_offset: SceUInt32,
    pub unk_0x24: SceUInt32,
    pub unk_0x28: SceUInt32,
    pub unk_0x2C: SceUInt32,
    pub id: [SceUInt8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocctlAdhocId {
    pub type_: crate::ctypes::c_int,
    pub data: [SceChar8; 9usize],
    pub padding: [SceUChar8; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocctlBSSId {
    pub data: [SceUChar8; 6usize],
    pub padding: [SceUChar8; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocctlGroupName {
    pub data: [SceChar8; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocctlNickname {
    pub data: [SceChar8; 128usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocctlParameter {
    pub channel: crate::ctypes::c_int,
    pub groupName: SceNetAdhocctlGroupName,
    pub nickname: SceNetAdhocctlNickname,
    pub bssid: SceNetAdhocctlBSSId,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocctlPeerInfo {
    pub next: *mut SceNetAdhocctlPeerInfo,
    pub nickname: SceNetAdhocctlNickname,
    pub macAddr: SceNetEtherAddr,
    pub padding: [SceUChar8; 6usize],
    pub lastRecv: SceUInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocPdpStat {
    pub next: *mut SceNetAdhocPdpStat,
    pub id: crate::ctypes::c_int,
    pub laddr: SceNetEtherAddr,
    pub lport: SceUShort16,
    pub rcv_sb_cc: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocPollSd {
    pub id: crate::ctypes::c_int,
    pub events: crate::ctypes::c_int,
    pub revents: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetAdhocPtpStat {
    pub next: *mut SceNetAdhocPtpStat,
    pub id: crate::ctypes::c_int,
    pub laddr: SceNetEtherAddr,
    pub paddr: SceNetEtherAddr,
    pub lport: SceUShort16,
    pub pport: SceUShort16,
    pub snd_sb_cc: crate::ctypes::c_uint,
    pub rcv_sb_cc: crate::ctypes::c_uint,
    pub state: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetCheckDialogAgeRestriction {
    pub countryCode: [crate::ctypes::c_char; 2usize],
    pub age: SceInt8,
    pub padding: SceInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetCheckDialogParam {
    pub sdkVersion: SceUInt32,
    pub commonParam: SceCommonDialogParam,
    pub mode: SceInt32,
    pub npCommunicationId: SceNpCommunicationId,
    pub ps3ConnectParam: *mut SceNetCheckDialogPS3ConnectParam,
    pub groupName: *mut SceNetAdhocctlGroupName,
    pub timeoutUs: SceUInt32,
    pub defaultAgeRestriction: SceInt8,
    pub padding: [SceInt8; 3usize],
    pub ageRestrictionCount: SceInt32,
    pub ageRestriction: *const SceNetCheckDialogAgeRestriction,
    pub reserved: [SceUInt8; 104usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetCheckDialogPS3ConnectInfo {
    pub inaddr: SceNetInAddr,
    pub nickname: [SceUInt8; 128usize],
    pub macAddress: [SceUInt8; 6usize],
    pub reserved: [SceUInt8; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetCheckDialogPS3ConnectParam {
    pub action: SceInt32,
    pub ssid: [crate::ctypes::c_char; 33usize],
    pub wpaKey: [crate::ctypes::c_char; 65usize],
    pub titleId: [crate::ctypes::c_char; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetCheckDialogResult {
    pub result: SceInt32,
    pub psnModeSucceeded: SceBool,
    pub reserved: [SceUInt8; 124usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetCtlAdhocPeerInfo {
    pub next: *mut SceNetCtlAdhocPeerInfo,
    pub inet_addr: SceNetInAddr,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceNetCtlInfo {
    pub cnf_name: [crate::ctypes::c_char; 65usize],
    pub device: crate::ctypes::c_uint,
    pub ether_addr: SceNetEtherAddr,
    pub mtu: crate::ctypes::c_uint,
    pub link: crate::ctypes::c_uint,
    pub bssid: SceNetEtherAddr,
    pub ssid: [crate::ctypes::c_char; 33usize],
    pub wifi_security: crate::ctypes::c_uint,
    pub rssi_dbm: crate::ctypes::c_uint,
    pub rssi_percentage: crate::ctypes::c_uint,
    pub channel: crate::ctypes::c_uint,
    pub ip_config: crate::ctypes::c_uint,
    pub dhcp_hostname: [crate::ctypes::c_char; 256usize],
    pub pppoe_auth_name: [crate::ctypes::c_char; 128usize],
    pub ip_address: [crate::ctypes::c_char; 16usize],
    pub netmask: [crate::ctypes::c_char; 16usize],
    pub default_route: [crate::ctypes::c_char; 16usize],
    pub primary_dns: [crate::ctypes::c_char; 16usize],
    pub secondary_dns: [crate::ctypes::c_char; 16usize],
    pub http_proxy_config: crate::ctypes::c_uint,
    pub http_proxy_server: [crate::ctypes::c_char; 256usize],
    pub http_proxy_port: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetCtlNatInfo {
    pub size: crate::ctypes::c_uint,
    pub stun_status: crate::ctypes::c_int,
    pub nat_type: crate::ctypes::c_int,
    pub mapped_addr: SceNetInAddr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetDnsInfo {
    pub dns_addr: [SceNetInAddr; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetEmulationData {
    pub drop_rate: crate::ctypes::c_ushort,
    pub drop_duration: crate::ctypes::c_ushort,
    pub pass_duration: crate::ctypes::c_ushort,
    pub delay_time: crate::ctypes::c_ushort,
    pub delay_jitter: crate::ctypes::c_ushort,
    pub order_rate: crate::ctypes::c_ushort,
    pub order_delay_time: crate::ctypes::c_ushort,
    pub duplication_rate: crate::ctypes::c_ushort,
    pub bps_limit: crate::ctypes::c_uint,
    pub lower_size_limit: crate::ctypes::c_ushort,
    pub upper_size_limit: crate::ctypes::c_ushort,
    pub system_policy_pattern: crate::ctypes::c_uint,
    pub game_policy_pattern: crate::ctypes::c_uint,
    pub policy_flags: [crate::ctypes::c_ushort; 64usize],
    pub reserved: [crate::ctypes::c_uchar; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetEmulationParam {
    pub version: crate::ctypes::c_ushort,
    pub option_number: crate::ctypes::c_ushort,
    pub current_version: crate::ctypes::c_ushort,
    pub result: crate::ctypes::c_ushort,
    pub flags: crate::ctypes::c_uint,
    pub reserved1: crate::ctypes::c_uint,
    pub send: SceNetEmulationData,
    pub recv: SceNetEmulationData,
    pub seed: crate::ctypes::c_uint,
    pub reserved: [crate::ctypes::c_uchar; 44usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceNetEpollData {
    pub ptr: *mut crate::ctypes::c_void,
    pub fd: crate::ctypes::c_int,
    pub u32_: crate::ctypes::c_uint,
    pub u64_: crate::ctypes::c_ulonglong,
    pub ext: SceNetEpollDataExt,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetEpollDataExt {
    pub id: crate::ctypes::c_int,
    pub u32_: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetEpollEvent {
    pub events: crate::ctypes::c_uint,
    pub reserved: crate::ctypes::c_uint,
    pub system: SceNetEpollSystemData,
    pub data: SceNetEpollData,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetEpollSystemData {
    pub system: [crate::ctypes::c_uint; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetEtherAddr {
    pub data: [crate::ctypes::c_uchar; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetFdSet {
    pub bits: [crate::ctypes::c_uint; 32usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetIcmpHeader {
    pub type_: crate::ctypes::c_uchar,
    pub code: crate::ctypes::c_uchar,
    pub checksum: crate::ctypes::c_ushort,
    pub un: SceNetIcmpHeaderUnion,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetIcmpHeaderEcho {
    pub id: crate::ctypes::c_ushort,
    pub sequence: crate::ctypes::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetIcmpHeaderFrag {
    pub unused: crate::ctypes::c_ushort,
    pub mtu: crate::ctypes::c_ushort,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceNetIcmpHeaderUnion {
    pub echo: SceNetIcmpHeaderEcho,
    pub gateway: crate::ctypes::c_uint,
    pub frag: SceNetIcmpHeaderFrag,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetInAddr {
    pub s_addr: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetInitParam {
    pub memory: *mut crate::ctypes::c_void,
    pub size: crate::ctypes::c_int,
    pub flags: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetIovec {
    pub iov_base: *mut crate::ctypes::c_void,
    pub iov_len: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetIpHeader {
    pub un: SceNetIpHeaderUnion,
    pub ip_tos: crate::ctypes::c_uchar,
    pub ip_len: crate::ctypes::c_ushort,
    pub ip_id: crate::ctypes::c_ushort,
    pub ip_off: crate::ctypes::c_ushort,
    pub ip_ttl: crate::ctypes::c_uchar,
    pub ip_p: crate::ctypes::c_uchar,
    pub ip_sum: crate::ctypes::c_ushort,
    pub ip_src: SceNetInAddr,
    pub ip_dst: SceNetInAddr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetIpHeaderIpVerHl {
    pub hl: crate::ctypes::c_uchar,
    pub ver: crate::ctypes::c_uchar,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceNetIpHeaderUnion {
    pub ip_ver_hl: SceNetIpHeaderIpVerHl,
    pub ver_hl: crate::ctypes::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetIpMreq {
    pub imr_multiaddr: SceNetInAddr,
    pub imr_interface: SceNetInAddr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetLinger {
    pub l_onoff: crate::ctypes::c_int,
    pub l_linger: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetMsghdr {
    pub msg_name: *mut crate::ctypes::c_void,
    pub msg_namelen: crate::ctypes::c_uint,
    pub msg_iov: *mut SceNetIovec,
    pub msg_iovlen: crate::ctypes::c_int,
    pub msg_control: *mut crate::ctypes::c_void,
    pub msg_controllen: crate::ctypes::c_uint,
    pub msg_flags: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetResolverParam {
    pub allocate: SceNetResolverFunctionAllocate,
    pub free: SceNetResolverFunctionFree,
    pub user: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetSockaddr {
    pub sa_len: crate::ctypes::c_uchar,
    pub sa_family: crate::ctypes::c_uchar,
    pub sa_data: [crate::ctypes::c_char; 14usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetSockaddrIn {
    pub sin_len: crate::ctypes::c_uchar,
    pub sin_family: crate::ctypes::c_uchar,
    pub sin_port: crate::ctypes::c_ushort,
    pub sin_addr: SceNetInAddr,
    pub sin_vport: crate::ctypes::c_ushort,
    pub sin_zero: [crate::ctypes::c_char; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetSockInfo {
    pub name: [crate::ctypes::c_char; 32usize],
    pub pid: crate::ctypes::c_int,
    pub s: crate::ctypes::c_int,
    pub socket_type: crate::ctypes::c_char,
    pub policy: crate::ctypes::c_char,
    pub reserved16: crate::ctypes::c_short,
    pub recv_queue_length: crate::ctypes::c_int,
    pub send_queue_length: crate::ctypes::c_int,
    pub local_adr: SceNetInAddr,
    pub remote_adr: SceNetInAddr,
    pub local_port: crate::ctypes::c_ushort,
    pub remote_port: crate::ctypes::c_ushort,
    pub local_vport: crate::ctypes::c_ushort,
    pub remote_vport: crate::ctypes::c_ushort,
    pub state: crate::ctypes::c_int,
    pub flags: crate::ctypes::c_int,
    pub reserved: [crate::ctypes::c_int; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetStatisticsInfo {
    pub kernel_mem_free_size: crate::ctypes::c_int,
    pub kernel_mem_free_min: crate::ctypes::c_int,
    pub packet_count: crate::ctypes::c_int,
    pub packet_qos_count: crate::ctypes::c_int,
    pub libnet_mem_free_size: crate::ctypes::c_int,
    pub libnet_mem_free_min: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNetSyscallParameter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsBufferInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsCallbackInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsCallbackListInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsParamsDescriptor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsPatchSetupInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsRackDescription {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsSystemInitParams {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsVoiceDefinition {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNgsVoicePreset {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNotificationUtilProgressFinishParam {
    pub notificationText: [SceWChar16; 63usize],
    pub separator0: SceInt16,
    pub notificationSubText: [SceWChar16; 63usize],
    pub separator1: SceInt16,
    pub path: [SceChar8; 1000usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNotificationUtilProgressInitParam {
    pub notificationText: [SceWChar16; 63usize],
    pub separator0: SceInt16,
    pub notificationSubText: [SceWChar16; 63usize],
    pub separator1: SceInt16,
    pub unk: [SceChar8; 998usize],
    pub unk_4EC: SceInt32,
    pub eventHandler: SceNotificationUtilProgressEventHandler,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNotificationUtilProgressUpdateParam {
    pub notificationText: [SceWChar16; 63usize],
    pub separator0: SceInt16,
    pub notificationSubText: [SceWChar16; 63usize],
    pub separator1: SceInt16,
    pub targetProgress: SceFloat,
    pub reserved: [SceChar8; 56usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNpCommunicationId {
    pub data: [crate::ctypes::c_char; 9usize],
    pub term: crate::ctypes::c_char,
    pub num: SceUChar8,
    pub dummy: crate::ctypes::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNpDrmActivationData {
    pub act_type: SceInt16,
    pub version_flag: SceInt16,
    pub version: SceInt32,
    pub account_id: SceUInt64,
    pub primary_key_table: [[SceUInt8; 16usize]; 128usize],
    pub unk1: [SceUInt8; 64usize],
    pub openpsid: [SceUInt8; 16usize],
    pub unk2: [SceUInt8; 16usize],
    pub unk3: [SceUInt8; 16usize],
    pub secondary_key_table: [[SceUInt8; 16usize]; 101usize],
    pub rsa_signature: [SceUInt8; 256usize],
    pub unk_sigmature: [SceUInt8; 64usize],
    pub ecdsa_signature: [SceUInt8; 40usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceNpDrmLicense {
    pub version: SceInt16,
    pub version_flags: SceInt16,
    pub license_type: SceInt16,
    pub license_flags: SceInt16,
    pub account_id: SceUInt64,
    pub content_id: [crate::ctypes::c_char; 48usize],
    pub key_table: [crate::ctypes::c_char; 16usize],
    pub key1: [crate::ctypes::c_char; 16usize],
    pub start_time: SceInt64,
    pub expiration_time: SceInt64,
    pub ecdsa_signature: [crate::ctypes::c_char; 40usize],
    pub flags: SceInt64,
    pub key2: [crate::ctypes::c_char; 16usize],
    pub unk_0xB0: [crate::ctypes::c_char; 16usize],
    pub open_psid: [crate::ctypes::c_char; 16usize],
    pub unk_0xD0: [crate::ctypes::c_char; 16usize],
    pub cmd56_handshake_part: [crate::ctypes::c_char; 20usize],
    pub debug_upgradable: crate::ctypes::c_int,
    pub unk_0xF8: crate::ctypes::c_int,
    pub sku_flag: crate::ctypes::c_int,
    pub rsa_signature: [crate::ctypes::c_char; 256usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _sceNpDrmPackageDecrypt {
    pub offset: SceOff,
    pub identifier: crate::ctypes::c_uint,
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceOpenPsId {
    pub open_psid: [u8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePafDateTime {
    pub data: SceDateTime,
    pub data_0x10: crate::ctypes::c_int,
    pub data_0x14: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePafHeapContext {
    pub vtable: *mut crate::ctypes::c_void,
    pub heap: *mut crate::ctypes::c_void,
    pub membase: *mut crate::ctypes::c_void,
    pub size: SceSize,
    pub name: [crate::ctypes::c_char; 32usize],
    pub is_import_membase: SceChar8,
    pub is_skip_debug_msg: SceChar8,
    pub data_0x32: crate::ctypes::c_char,
    pub data_0x33: crate::ctypes::c_char,
    pub data_0x34: crate::ctypes::c_int,
    pub lw_mtx: SceKernelLwMutexWork,
    pub memblk_id: SceUID,
    pub mode: SceInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePafHeapOpt {
    pub a1: crate::ctypes::c_int,
    pub a2: crate::ctypes::c_int,
    pub is_skip_debug_msg: SceChar8,
    pub a3: [crate::ctypes::c_char; 3usize],
    pub mode: SceInt32,
    pub a5: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePafSha1Context {
    pub h: [u32; 5usize],
    pub unk: [crate::ctypes::c_char; 84usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePfsRndDriveId {
    pub drive_id: [crate::ctypes::c_char; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePortabilityData {
    pub msg_size: SceSize,
    pub msg: [u8; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceProcEventHandler {
    pub size: SceSize,
    pub create: ::core::option::Option<
        unsafe extern "C" fn(
            pid: SceUID,
            a2: *mut SceProcEventInvokeParam2,
            a3: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub exit: ::core::option::Option<
        unsafe extern "C" fn(
            pid: SceUID,
            a2: *mut SceProcEventInvokeParam1,
            a3: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub kill: ::core::option::Option<
        unsafe extern "C" fn(
            pid: SceUID,
            a2: *mut SceProcEventInvokeParam1,
            a3: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub stop: ::core::option::Option<
        unsafe extern "C" fn(
            pid: SceUID,
            event_type: crate::ctypes::c_int,
            a3: *mut SceProcEventInvokeParam1,
            a4: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub start: ::core::option::Option<
        unsafe extern "C" fn(
            pid: SceUID,
            event_type: crate::ctypes::c_int,
            a3: *mut SceProcEventInvokeParam1,
            a4: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub switch_process: ::core::option::Option<
        unsafe extern "C" fn(
            event_id: crate::ctypes::c_int,
            event_type: crate::ctypes::c_int,
            a3: *mut SceProcEventInvokeParam2,
            a4: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceProcEventInvokeParam1 {
    pub size: SceSize,
    pub unk_0x04: crate::ctypes::c_int,
    pub unk_0x08: crate::ctypes::c_int,
    pub unk_0x0C: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceProcEventInvokeParam2 {
    pub size: SceSize,
    pub pid: SceUID,
    pub unk_0x08: crate::ctypes::c_int,
    pub unk_0x0C: crate::ctypes::c_int,
    pub unk_0x10: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePromoterUtilityImportParams {
    pub path: [crate::ctypes::c_char; 128usize],
    pub titleid: [crate::ctypes::c_char; 12usize],
    pub type_: ScePromoterUtilityPackageType,
    pub attribute: u32,
    pub reserved: [crate::ctypes::c_char; 28usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePromoterUtilityLAUpdate {
    pub titleid: [crate::ctypes::c_char; 12usize],
    pub path: [crate::ctypes::c_char; 128usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsCode {
    pub company_code: u16,
    pub product_code: u16,
    pub product_sub_code: u16,
    pub factory_code: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePsmDrmLicense {
    pub magic: [crate::ctypes::c_char; 8usize],
    pub unk1: SceUInt32,
    pub unk2: SceUInt32,
    pub account_id: SceUInt64,
    pub unk3: SceUInt32,
    pub unk4: SceUInt32,
    pub start_time: SceUInt64,
    pub expiration_time: SceUInt64,
    pub activation_checksum: [SceUInt8; 32usize],
    pub content_id: [crate::ctypes::c_char; 48usize],
    pub unk5: [SceUInt8; 128usize],
    pub unk6: [SceUInt8; 32usize],
    pub key: [SceUInt8; 16usize],
    pub signature: [SceUInt8; 464usize],
    pub rsa_signature: [SceUInt8; 256usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePssCryptoHandle {
    pub fd: SceUID,
    pub unk1: u32,
    pub size: SceSize,
    pub unk3: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfCacheKey {
    pub keyValue0: crate::ctypes::c_int,
    pub keyValue1: crate::ctypes::c_int,
    pub keyValue2: f32,
    pub keyValue3: f32,
    pub keyValue4: f32,
    pub keyValue5: f32,
    pub keyValue6: f32,
    pub keyValue7: f32,
    pub keyValue8: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfCacheSystemInterface {
    pub cacheInstance: *mut ScePvfPointer,
    pub lockFunc: ScePvfFontCacheLockFunc,
    pub unlockFunc: ScePvfFontChcheUnlockFunc,
    pub findFunc: ScePvfFontChcheFindFunc,
    pub writeKeyValueToCacheFunc: ScePvfFontChcheWriteKeyValueToCacheFunc,
    pub write0ToCacheFunc: ScePvfFontChcheWriteToCacheFunc,
    pub write1ToCacheFunc: ScePvfFontChcheWriteToCacheFunc,
    pub read0FromCacheFunc: ScePvfFontChcheReadFromCacheFunc,
    pub read1FromCacheFunc: ScePvfFontChcheReadFromCacheFunc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfCharInfo {
    pub bitmapWidth: ScePvfU32,
    pub bitmapHeight: ScePvfU32,
    pub bitmapPitch: ScePvfU32,
    pub bitmapLeft: ScePvfS32,
    pub bitmapTop: ScePvfS32,
    pub glyphMetrics: ScePvfIGlyphMetricsInfo,
    pub reserved0: [ScePvfU8; 2usize],
    pub reserved1: ScePvfU16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfFGlyphMetricsInfo {
    pub width: ScePvfFloat32,
    pub height: ScePvfFloat32,
    pub ascender: ScePvfFloat32,
    pub descender: ScePvfFloat32,
    pub horizontalBearingX: ScePvfFloat32,
    pub horizontalBearingY: ScePvfFloat32,
    pub verticalBearingX: ScePvfFloat32,
    pub verticalBearingY: ScePvfFloat32,
    pub horizontalAdvance: ScePvfFloat32,
    pub verticalAdvance: ScePvfFloat32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfFKerningInfo {
    pub xOffset: ScePvfFloat32,
    pub yOffset: ScePvfFloat32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfFontInfo {
    pub maxIGlyphMetrics: ScePvfIGlyphMetricsInfo,
    pub maxFGlyphMetrics: ScePvfFGlyphMetricsInfo,
    pub numChars: ScePvfU32,
    pub fontStyleInfo: ScePvfFontStyleInfo,
    pub reserved: [ScePvfU8; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfFontStyleInfo {
    pub weight: ScePvfFloat32,
    pub familyCode: ScePvfU16,
    pub style: ScePvfU16,
    pub subStyle: ScePvfU16,
    pub languageCode: ScePvfU16,
    pub regionCode: ScePvfU16,
    pub countryCode: ScePvfU16,
    pub fontName: [ScePvfU8; 64usize],
    pub styleName: [ScePvfU8; 64usize],
    pub fileName: [ScePvfU8; 64usize],
    pub extraAttributes: ScePvfU32,
    pub expireDate: ScePvfU32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfIGlyphMetricsInfo {
    pub width64: ScePvfU32,
    pub height64: ScePvfU32,
    pub ascender64: ScePvfS32,
    pub descender64: ScePvfS32,
    pub horizontalBearingX64: ScePvfS32,
    pub horizontalBearingY64: ScePvfS32,
    pub verticalBearingX64: ScePvfS32,
    pub verticalBearingY64: ScePvfS32,
    pub horizontalAdvance64: ScePvfS32,
    pub verticalAdvance64: ScePvfS32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfIKerningInfo {
    pub xOffset64: ScePvfS32,
    pub yOffset64: ScePvfS32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfInitRec {
    pub userData: ScePvfPointer,
    pub maxNumFonts: ScePvfU32,
    pub cache: *mut ScePvfCacheSystemInterface,
    pub reserved: ScePvfPointer,
    pub allocFunc: ScePvfAllocFunc,
    pub reallocFunc: ScePvfReallocFunc,
    pub freeFunc: ScePvfFreeFunc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfIrect {
    pub width: ScePvfU16,
    pub height: ScePvfU16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfKerningInfo {
    pub iKerningInfo: ScePvfIKerningInfo,
    pub fKerningInfo: ScePvfFKerningInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfRect {
    pub width: ScePvfU32,
    pub height: ScePvfU32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePvfUserImageBufferRec {
    pub pixelFormat: ScePvfU32,
    pub xPos64: ScePvfS32,
    pub yPos64: ScePvfS32,
    pub rect: ScePvfIrect,
    pub bytesPerLine: ScePvfU16,
    pub reserved: ScePvfU16,
    pub buffer: *mut ScePvfU8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryFrame {
    pub header: SceRazorGpuLiveEntryHeader,
    pub start_time: u64,
    pub duration: u32,
    pub frame_number: u32,
    pub gpu_activity_duration_time: u32,
    pub reserved: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryHeader {
    pub entry_size: u16,
    pub entry_type: u16,
    pub reserved: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryJob {
    pub header: SceRazorGpuLiveEntryHeader,
    pub start_time: u64,
    pub end_time: u64,
    pub type_: u8,
    pub core: i8,
    pub scene_index: u16,
    pub frame_number: u32,
    pub job_values: SceRazorGpuLiveEntryJobValues,
    pub process_id: u32,
    pub unk: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryJobFragmentValues1 {
    pub usse_fragment_processing_percent: f32,
    pub usse_dependent_texture_reads_percent: f32,
    pub usse_non_dependent_texture_reads_percent: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryJobFragmentValues2 {
    pub rasterized_pixels_before_hsr_num: u32,
    pub rasterized_output_pixels_num: u32,
    pub rasterized_output_samples_num: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryJobFragmentValues3 {
    pub isp_parameter_fetches_mem_reads: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryJobValues {
    pub vertex_values_type1: SceRazorGpuLiveEntryJobVertexValues1,
    pub fragment_values_type1: SceRazorGpuLiveEntryJobFragmentValues1,
    pub vertex_values_type2: SceRazorGpuLiveEntryJobVertexValues2,
    pub fragment_values_type2: SceRazorGpuLiveEntryJobFragmentValues2,
    pub vertex_values_type3: SceRazorGpuLiveEntryJobVertexValues3,
    pub fragment_values_type3: SceRazorGpuLiveEntryJobFragmentValues3,
    pub reserved: [crate::ctypes::c_int; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryJobVertexValues1 {
    pub usse_vertex_processing_percent: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryJobVertexValues2 {
    pub vdm_primitives_input_num: u32,
    pub mte_primitives_output_num: u32,
    pub vdm_vertices_input_num: u32,
    pub mte_vertices_output_num: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryJobVertexValues3 {
    pub tiling_accelerated_mem_writes: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveEntryParameterBuffer {
    pub header: SceRazorGpuLiveEntryHeader,
    pub peak_usage_timestamp: u64,
    pub peak_usage_value: u32,
    pub partial_render: u8,
    pub vertex_job_paused: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRazorGpuLiveResultInfo {
    pub result_data: *mut crate::ctypes::c_void,
    pub entry_count: SceSize,
    pub overflow_count: SceSize,
    pub buffer_size: SceSize,
    pub start_time: u64,
    pub end_time: u64,
    pub metric_group: u32,
    pub reserved: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceRtcTick {
    pub tick: SceUInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSblDmac5EncDecParam {
    pub src: *const crate::ctypes::c_void,
    pub dst: *mut crate::ctypes::c_void,
    pub length: SceSize,
    pub key: *const crate::ctypes::c_void,
    pub keysize: SceSize,
    pub iv: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSblDmac5HashTransformContext {
    pub state: [SceUInt32; 8usize],
    pub length: SceUInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSblDmac5HashTransformParam {
    pub src: *const crate::ctypes::c_void,
    pub dst: *mut crate::ctypes::c_void,
    pub length: SceSize,
    pub key: *const crate::ctypes::c_void,
    pub keysize: SceSize,
    pub ctx: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSblRsaDataParam {
    pub data: *mut crate::ctypes::c_void,
    pub size: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSblRsaPrivateKeyParam {
    pub unk_0x00: crate::ctypes::c_int,
    pub unk_0x04: crate::ctypes::c_int,
    pub unk_0x08: crate::ctypes::c_int,
    pub unk_0x0C: crate::ctypes::c_int,
    pub p: *mut crate::ctypes::c_void,
    pub q: *mut crate::ctypes::c_void,
    pub dp: *mut crate::ctypes::c_void,
    pub dq: *mut crate::ctypes::c_void,
    pub qp: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSblRsaPublicKeyParam {
    pub n: *const crate::ctypes::c_void,
    pub k: *const crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSblSmCommPair {
    pub data_00: crate::ctypes::c_int,
    pub data_04: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceScreenShotParam {
    pub photoTitle: *const SceWChar32,
    pub gameTitle: *const SceWChar32,
    pub gameComment: *const SceWChar32,
    pub reserved: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSelfAuthInfo {
    pub program_authority_id: SceUInt64,
    pub padding: [u8; 8usize],
    pub capability: [u8; 32usize],
    pub attribute: [u8; 32usize],
    pub secret: SceSharedSecret,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSha1Context {
    pub h: [u32; 5usize],
    pub usRemains: u16,
    pub usComputed: u16,
    pub ullTotalLen: u64,
    pub buf: [crate::ctypes::c_char; 64usize],
    pub result: [crate::ctypes::c_char; 20usize],
    pub pad: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSha224Context {
    pub h: [u32; 8usize],
    pub pad: u32,
    pub usRemains: u16,
    pub usComputed: u16,
    pub ullTotalLen: u64,
    pub buf: [crate::ctypes::c_char; 64usize],
    pub result: [crate::ctypes::c_char; 28usize],
    pub pad2: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSha256Context {
    pub h: [u32; 8usize],
    pub pad: u32,
    pub usRemains: u16,
    pub usComputed: u16,
    pub ullTotalLen: u64,
    pub buf: [crate::ctypes::c_char; 64usize],
    pub result: [crate::ctypes::c_char; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceShaccCgCallbackList {
    pub openFile: SceShaccCgCallbackOpenFile,
    pub releaseFile: SceShaccCgCallbackReleaseFile,
    pub locateFile: SceShaccCgCallbackLocateFile,
    pub absolutePath: SceShaccCgCallbackAbsolutePath,
    pub releaseFileName: SceShaccCgCallbackReleaseFileName,
    pub fileDate: SceShaccCgCallbackFileDate,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceShaccCgCompileOptions {
    pub mainSourceFile: *const crate::ctypes::c_char,
    pub targetProfile: SceShaccCgTargetProfile,
    pub entryFunctionName: *const crate::ctypes::c_char,
    pub searchPathCount: SceUInt32,
    pub searchPaths: *const *const crate::ctypes::c_char,
    pub macroDefinitionCount: SceUInt32,
    pub macroDefinitions: *const *const crate::ctypes::c_char,
    pub includeFileCount: SceUInt32,
    pub includeFiles: *const *const crate::ctypes::c_char,
    pub suppressedWarningsCount: SceUInt32,
    pub suppressedWarnings: *const SceUInt32,
    pub locale: SceShaccCgLocale,
    pub useFx: SceInt32,
    pub noStdlib: SceInt32,
    pub optimizationLevel: SceInt32,
    pub useFastmath: SceInt32,
    pub useFastprecision: SceInt32,
    pub useFastint: SceInt32,
    pub field_48: crate::ctypes::c_int,
    pub warningsAsErrors: SceInt32,
    pub performanceWarnings: SceInt32,
    pub warningLevel: SceInt32,
    pub pedantic: SceInt32,
    pub pedanticError: SceInt32,
    pub field_60: crate::ctypes::c_int,
    pub field_64: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceShaccCgCompileOutput {
    pub programData: *const u8,
    pub programSize: SceUInt32,
    pub diagnosticCount: SceInt32,
    pub diagnostics: *const SceShaccCgDiagnosticMessage,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceShaccCgDiagnosticMessage {
    pub level: SceShaccCgDiagnosticLevel,
    pub code: SceUInt32,
    pub location: *const SceShaccCgSourceLocation,
    pub message: *const crate::ctypes::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceShaccCgSourceFile {
    pub fileName: *const crate::ctypes::c_char,
    pub text: *const crate::ctypes::c_char,
    pub size: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceShaccCgSourceLocation {
    pub file: *const SceShaccCgSourceFile,
    pub lineNumber: SceUInt32,
    pub columnNumber: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSharedFbInfo {
    pub fb_base: *mut crate::ctypes::c_void,
    pub fb_size: crate::ctypes::c_int,
    pub fb_base2: *mut crate::ctypes::c_void,
    pub unk0: [crate::ctypes::c_int; 6usize],
    pub stride: crate::ctypes::c_int,
    pub width: crate::ctypes::c_int,
    pub height: crate::ctypes::c_int,
    pub unk1: crate::ctypes::c_int,
    pub index: crate::ctypes::c_int,
    pub unk2: [crate::ctypes::c_int; 4usize],
    pub vsync: crate::ctypes::c_int,
    pub unk3: [crate::ctypes::c_int; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSharedSecret {
    pub shared_secret_0: [u8; 16usize],
    pub klicensee: [u8; 16usize],
    pub shared_secret_2: [u8; 16usize],
    pub shared_secret_3_0: u32,
    pub shared_secret_3_1: u32,
    pub shared_secret_3_2: u32,
    pub shared_secret_3_3: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceShellUtilLaunchAppParam {
    pub cmd: *const crate::ctypes::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSqliteMallocMethods {
    pub xMalloc: ::core::option::Option<
        unsafe extern "C" fn(arg1: crate::ctypes::c_int) -> *mut crate::ctypes::c_void,
    >,
    pub xRealloc: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut crate::ctypes::c_void,
            arg2: crate::ctypes::c_int,
        ) -> *mut crate::ctypes::c_void,
    >,
    pub xFree: ::core::option::Option<unsafe extern "C" fn(arg1: *mut crate::ctypes::c_void)>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSslMemoryPoolStats {
    pub poolSize: crate::ctypes::c_uint,
    pub maxInuseSize: crate::ctypes::c_uint,
    pub currentInuseSize: crate::ctypes::c_uint,
    pub reserved: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSysconDebugHandlers {
    pub size: crate::ctypes::c_int,
    pub start: ::core::option::Option<unsafe extern "C" fn(packet: *mut SceSysconPacket)>,
    pub end: ::core::option::Option<unsafe extern "C" fn(packet: *mut SceSysconPacket)>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSysconPacket {
    pub next: *mut SceSysconPacket,
    pub status: crate::ctypes::c_uint,
    pub semaId: SceUID,
    pub unk: crate::ctypes::c_uint,
    pub tx: [crate::ctypes::c_uchar; 32usize],
    pub rx: [crate::ctypes::c_uchar; 32usize],
    pub unk1: [crate::ctypes::c_uint; 4usize],
    pub callback: ::core::option::Option<
        unsafe extern "C" fn(
            packet: *mut SceSysconPacket,
            argp: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub argp: *mut crate::ctypes::c_void,
    pub time: crate::ctypes::c_uint,
    pub unk2: [crate::ctypes::c_uint; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSysmoduleOpt {
    pub flags: crate::ctypes::c_int,
    pub result: *mut crate::ctypes::c_int,
    pub unused: [crate::ctypes::c_int; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSysrootDbgpHandler {
    pub size: SceSize,
    pub unk_0x04: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
            a4: crate::ctypes::c_int,
        ),
    >,
    pub unk_0x08: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
            a4: crate::ctypes::c_int,
        ),
    >,
    pub unk_0x0C: ::core::option::Option<unsafe extern "C" fn(a1: crate::ctypes::c_int)>,
    pub unk_0x10: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
            a4: crate::ctypes::c_int,
        ),
    >,
    pub unk_0x14: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
            a4: crate::ctypes::c_int,
        ),
    >,
    pub unk_0x18: ::core::option::Option<
        unsafe extern "C" fn(pid: SceUID, modid: SceUID, flags: crate::ctypes::c_int, time: u64),
    >,
    pub unk_0x1C: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
        ),
    >,
    pub unk_0x20: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
        ),
    >,
    pub unk_0x24: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
        ),
    >,
    pub unk_0x28:
        ::core::option::Option<unsafe extern "C" fn(pid: SceUID, modid: SceUID, time: u64)>,
    pub unk_0x2C:
        ::core::option::Option<unsafe extern "C" fn(pid: SceUID, modid: SceUID, time: u64)>,
    pub unk_0x30: ::core::option::Option<unsafe extern "C" fn(pid: SceUID) -> crate::ctypes::c_int>,
    pub unk_0x34: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub unk_0x38: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub unk_0x3C: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub unk_0x40: ::core::option::Option<
        unsafe extern "C" fn(
            pid: SceUID,
            some_flag: *mut crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub unk_0x44: ::core::option::Option<
        unsafe extern "C" fn(
            pid: SceUID,
            modid: SceUID,
            flags: crate::ctypes::c_int,
            time: u64,
        ) -> crate::ctypes::c_int,
    >,
    pub unk_0x48: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub unk_0x4C: ::core::option::Option<unsafe extern "C" fn()>,
    pub unk_0x50: ::core::option::Option<unsafe extern "C" fn()>,
    pub unk_0x54: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
            a4: crate::ctypes::c_int,
            a5: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub unk_0x58: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSysrootForDriver_733C243E_struct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSysrootForKernel_D29BCA77_struct {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSysrootModulemgrHandlers {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSysrootProcessHandler {
    pub size: SceSize,
    pub unk_4: ::core::option::Option<
        unsafe extern "C" fn(pid: SceUID, modid: SceUID, flags: crate::ctypes::c_int, time: u64),
    >,
    pub exit: ::core::option::Option<
        unsafe extern "C" fn(pid: SceUID, flags: crate::ctypes::c_int, time: u64),
    >,
    pub kill: ::core::option::Option<unsafe extern "C" fn(pid: SceUID)>,
    pub unk_10: ::core::option::Option<unsafe extern "C" fn(pid: SceUID, modid: SceUID, time: u64)>,
    pub unk_14: ::core::option::Option<unsafe extern "C" fn(pid: SceUID, modid: SceUID, time: u64)>,
    pub unk_18: ::core::option::Option<unsafe extern "C" fn(pid: SceUID, modid: SceUID, time: u64)>,
    pub on_process_created: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub unk_20: ::core::option::Option<unsafe extern "C" fn(pid: SceUID, modid: SceUID, time: u64)>,
    pub unk_24: ::core::option::Option<
        unsafe extern "C" fn(pid: SceUID, modid: SceUID, flags: crate::ctypes::c_int, time: u64),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceThreadCpuRegisters {
    pub __bindgen_anon_1: SceThreadCpuRegisters__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceThreadCpuRegisters__bindgen_ty_1 {
    pub __bindgen_anon_1: SceThreadCpuRegisters__bindgen_ty_1__bindgen_ty_1,
    pub entry: [SceArmCpuRegisters; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceThreadCpuRegisters__bindgen_ty_1__bindgen_ty_1 {
    pub user: SceArmCpuRegisters,
    pub kernel: SceArmCpuRegisters,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceTouchData {
    pub timeStamp: SceUInt64,
    pub status: SceUInt32,
    pub reportNum: SceUInt32,
    pub report: [SceTouchReport; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceTouchPanelInfo {
    pub minAaX: SceInt16,
    pub minAaY: SceInt16,
    pub maxAaX: SceInt16,
    pub maxAaY: SceInt16,
    pub minDispX: SceInt16,
    pub minDispY: SceInt16,
    pub maxDispX: SceInt16,
    pub maxDispY: SceInt16,
    pub minForce: SceUInt8,
    pub maxForce: SceUInt8,
    pub reserved: [SceUInt8; 30usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceTouchReport {
    pub id: SceUInt8,
    pub force: SceUInt8,
    pub x: SceInt16,
    pub y: SceInt16,
    pub reserved: [SceUInt8; 8usize],
    pub info: SceUInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceTriggerUtilEventParamDaily {
    pub ver: SceUInt32,
    pub extraParam1: SceInt16,
    pub extraParam2: SceInt16,
    pub triggerTime: SceInt32,
    pub repeatDays: SceUInt16,
    pub reserved: [SceChar8; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceTriggerUtilEventParamOneTime {
    pub ver: SceUInt32,
    pub triggerTime: SceRtcTick,
    pub extraParam1: SceUInt8,
    pub extraParam2: SceUInt8,
    pub reserved: [SceChar8; 68usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceTriggerUtilSystemAppInfo {
    pub name: [SceWChar16; 256usize],
    pub iconPath: [SceChar8; 1024usize],
    pub reserved: [crate::ctypes::c_char; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceTriggerUtilUserAppInfo {
    pub name: [SceWChar16; 52usize],
    pub iconPath: [SceChar8; 1024usize],
    pub unk: crate::ctypes::c_short,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdConfigDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub wTotalLength: crate::ctypes::c_ushort,
    pub bNumInterfaces: crate::ctypes::c_uchar,
    pub bConfigurationValue: crate::ctypes::c_uchar,
    pub iConfiguration: crate::ctypes::c_uchar,
    pub bmAttributes: crate::ctypes::c_uchar,
    pub bMaxPower: crate::ctypes::c_uchar,
    pub settings: *mut SceUdcdInterfaceSettings,
    pub extra: *mut crate::ctypes::c_uchar,
    pub extraLength: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdConfiguration {
    pub configDescriptors: *mut SceUdcdConfigDescriptor,
    pub settings: *mut SceUdcdInterfaceSettings,
    pub interfaceDescriptors: *mut SceUdcdInterfaceDescriptor,
    pub endpointDescriptors: *mut SceUdcdEndpointDescriptor,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDeviceDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub bcdUSB: crate::ctypes::c_ushort,
    pub bDeviceClass: crate::ctypes::c_uchar,
    pub bDeviceSubClass: crate::ctypes::c_uchar,
    pub bDeviceProtocol: crate::ctypes::c_uchar,
    pub bMaxPacketSize0: crate::ctypes::c_uchar,
    pub idVendor: crate::ctypes::c_ushort,
    pub idProduct: crate::ctypes::c_ushort,
    pub bcdDevice: crate::ctypes::c_ushort,
    pub iManufacturer: crate::ctypes::c_uchar,
    pub iProduct: crate::ctypes::c_uchar,
    pub iSerialNumber: crate::ctypes::c_uchar,
    pub bNumConfigurations: crate::ctypes::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDeviceInfo {
    pub info: [crate::ctypes::c_uchar; 64usize],
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDeviceQualifierDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub bcdUSB: crate::ctypes::c_ushort,
    pub bDeviceClass: crate::ctypes::c_uchar,
    pub bDeviceSubClass: crate::ctypes::c_uchar,
    pub bDeviceProtocol: crate::ctypes::c_uchar,
    pub bMaxPacketSize0: crate::ctypes::c_uchar,
    pub bNumConfigurations: crate::ctypes::c_uchar,
    pub bReserved: crate::ctypes::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDeviceRequest {
    pub endpoint: *mut SceUdcdEndpoint,
    pub data: *mut crate::ctypes::c_void,
    pub attributes: crate::ctypes::c_uint,
    pub size: crate::ctypes::c_int,
    pub isControlRequest: crate::ctypes::c_int,
    pub onComplete: ::core::option::Option<unsafe extern "C" fn(req: *mut SceUdcdDeviceRequest)>,
    pub transmitted: crate::ctypes::c_int,
    pub returnCode: crate::ctypes::c_int,
    pub next: *mut SceUdcdDeviceRequest,
    pub unused: *mut crate::ctypes::c_void,
    pub physicalAddress: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDeviceState {
    pub unk_00: crate::ctypes::c_int,
    pub state: crate::ctypes::c_int,
    pub cable: crate::ctypes::c_int,
    pub connection: crate::ctypes::c_int,
    pub use_usb_charging: crate::ctypes::c_int,
    pub unk_14: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDriver {
    pub driverName: *const crate::ctypes::c_char,
    pub numEndpoints: crate::ctypes::c_int,
    pub endpoints: *mut SceUdcdEndpoint,
    pub interface: *mut SceUdcdInterface,
    pub descriptor_hi: *mut SceUdcdDeviceDescriptor,
    pub configuration_hi: *mut SceUdcdConfiguration,
    pub descriptor: *mut SceUdcdDeviceDescriptor,
    pub configuration: *mut SceUdcdConfiguration,
    pub stringDescriptors: *mut SceUdcdStringDescriptor,
    pub stringDescriptorProduct: *mut SceUdcdStringDescriptor,
    pub stringDescriptorSerial: *mut SceUdcdStringDescriptor,
    pub processRequest: ::core::option::Option<
        unsafe extern "C" fn(
            recipient: crate::ctypes::c_int,
            arg: crate::ctypes::c_int,
            req: *mut SceUdcdEP0DeviceRequest,
            user_data: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub changeSetting: ::core::option::Option<
        unsafe extern "C" fn(
            interfaceNumber: crate::ctypes::c_int,
            alternateSetting: crate::ctypes::c_int,
            bus: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub attach: ::core::option::Option<
        unsafe extern "C" fn(
            usb_version: crate::ctypes::c_int,
            user_data: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub detach: ::core::option::Option<unsafe extern "C" fn(user_data: *mut crate::ctypes::c_void)>,
    pub configure: ::core::option::Option<
        unsafe extern "C" fn(
            usb_version: crate::ctypes::c_int,
            desc_count: crate::ctypes::c_int,
            settings: *mut SceUdcdInterfaceSettings,
            user_data: *mut crate::ctypes::c_void,
        ),
    >,
    pub start: ::core::option::Option<
        unsafe extern "C" fn(
            size: crate::ctypes::c_int,
            args: *mut crate::ctypes::c_void,
            user_data: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub stop: ::core::option::Option<
        unsafe extern "C" fn(
            size: crate::ctypes::c_int,
            args: *mut crate::ctypes::c_void,
            user_data: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub user_data: *mut crate::ctypes::c_void,
    pub bus: crate::ctypes::c_int,
    pub link: *mut SceUdcdDriver,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDriverName {
    pub size: crate::ctypes::c_int,
    pub name: [crate::ctypes::c_char; 32usize],
    pub flags: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdEndpoint {
    pub direction: crate::ctypes::c_int,
    pub driverEndpointNumber: crate::ctypes::c_int,
    pub endpointNumber: crate::ctypes::c_int,
    pub transmittedBytes: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdEndpointDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub bEndpointAddress: crate::ctypes::c_uchar,
    pub bmAttributes: crate::ctypes::c_uchar,
    pub wMaxPacketSize: crate::ctypes::c_ushort,
    pub bInterval: crate::ctypes::c_uchar,
    pub extra: *mut crate::ctypes::c_uchar,
    pub extraLength: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdEP0DeviceRequest {
    pub bmRequestType: crate::ctypes::c_uchar,
    pub bRequest: crate::ctypes::c_uchar,
    pub wValue: crate::ctypes::c_ushort,
    pub wIndex: crate::ctypes::c_ushort,
    pub wLength: crate::ctypes::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdInterface {
    pub expectNumber: crate::ctypes::c_int,
    pub interfaceNumber: crate::ctypes::c_int,
    pub numInterfaces: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdInterfaceDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub bInterfaceNumber: crate::ctypes::c_uchar,
    pub bAlternateSetting: crate::ctypes::c_uchar,
    pub bNumEndpoints: crate::ctypes::c_uchar,
    pub bInterfaceClass: crate::ctypes::c_uchar,
    pub bInterfaceSubClass: crate::ctypes::c_uchar,
    pub bInterfaceProtocol: crate::ctypes::c_uchar,
    pub iInterface: crate::ctypes::c_uchar,
    pub endpoints: *mut SceUdcdEndpointDescriptor,
    pub extra: *mut crate::ctypes::c_uchar,
    pub extraLength: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdInterfaceSettings {
    pub descriptors: *mut SceUdcdInterfaceDescriptor,
    pub alternateSetting: crate::ctypes::c_uint,
    pub numDescriptors: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdStringDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub bString: [crate::ctypes::c_short; 31usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdWaitParam {
    pub unk_00: crate::ctypes::c_int,
    pub status: crate::ctypes::c_int,
    pub unk_08: crate::ctypes::c_int,
    pub unk_0C: crate::ctypes::c_int,
    pub unk_10: crate::ctypes::c_int,
    pub driverName: *const crate::ctypes::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUIDSysrootObject {
    pub object: *mut crate::ctypes::c_void,
    pub sce_class: *mut SceClass,
    pub size: SceSize,
    pub magic1: SceUInt32,
    pub cpu_intr: crate::ctypes::c_int,
    pub boot_alloc_memory_size: SceSize,
    pub boot_alloc_memory: *mut crate::ctypes::c_void,
    pub unk_0x1C: crate::ctypes::c_int,
    pub unk_0x20: *mut crate::ctypes::c_void,
    pub boot_flags: SceUInt32,
    pub status: SceUInt32,
    pub corelock_context: SceCorelockContext,
    pub unk_0x34: crate::ctypes::c_int,
    pub unk_0x38: crate::ctypes::c_int,
    pub unk_0x3C: *mut crate::ctypes::c_void,
    pub unk_0x40: *mut crate::ctypes::c_void,
    pub unk_0x44: *mut crate::ctypes::c_void,
    pub unk_0x48: *mut crate::ctypes::c_void,
    pub unk_0x4C: *mut crate::ctypes::c_void,
    pub unk_0x50: *mut crate::ctypes::c_void,
    pub unk_0x54: crate::ctypes::c_int,
    pub unk_0x58: *mut crate::ctypes::c_void,
    pub unk_0x5C: crate::ctypes::c_int,
    pub unk_0x60: crate::ctypes::c_int,
    pub unk_0x64: crate::ctypes::c_int,
    pub unk_0x68: crate::ctypes::c_int,
    pub kbl_param: *mut SceKblParam,
    pub boot_args: *mut SceKernelBootArgs,
    pub soc_revision: SceUInt32,
    pub unk_0x78: SceUInt32,
    pub soc_revision2: SceUInt32,
    pub model_info: SceUInt32,
    pub model_capability: SceUInt32,
    pub longtime5base: *mut crate::ctypes::c_void,
    pub cpu_intr_for_init_callback: crate::ctypes::c_int,
    pub init_callback_registable_base_number: SceUInt32,
    pub init_callback_slot: [[SceUIDSysrootObject__bindgen_ty_1; 8usize]; 9usize],
    pub funcThreadMgrStartAfterProcess:
        ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub funcIofilemgrStart: ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub unk_0x2DC: *mut crate::ctypes::c_void,
    pub unk_0x2E0: *mut crate::ctypes::c_void,
    pub sysroot_names: [*mut crate::ctypes::c_char; 9usize],
    pub this_object_uid: SceUID,
    pub unk_0x30C: *mut crate::ctypes::c_void,
    pub unk_0x310: *mut crate::ctypes::c_void,
    pub unk_0x314: *mut crate::ctypes::c_void,
    pub VBAR: *mut crate::ctypes::c_void,
    pub MVBAR: *mut crate::ctypes::c_void,
    pub unk_0x320: *mut crate::ctypes::c_void,
    pub unk_func_0x324: *mut *mut crate::ctypes::c_void,
    pub funcGetThreadId: ::core::option::Option<unsafe extern "C" fn() -> SceUID>,
    pub funcThreadFunction3: ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub funcGetProcessId: ::core::option::Option<unsafe extern "C" fn() -> SceUID>,
    pub funcThreadFunction4: ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub funcThreadFunction5: ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub funcThreadFunction6: ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub funcThreadFunction7: ::core::option::Option<unsafe extern "C" fn()>,
    pub funcThreadFunction8: ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub funcThreadFunction9: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub funcThreadFunction10: ::core::option::Option<
        unsafe extern "C" fn(a1: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
    pub modulemgr_handlers: *mut SceSysrootModulemgrHandlers,
    pub unk_0x354: crate::ctypes::c_int,
    pub processmgr_callbacks1: *mut SceSysrootForKernel_D29BCA77_struct,
    pub processmgr_callbacks2: *mut SceSysrootForDriver_733C243E_struct,
    pub unk_func_0x360: *mut crate::ctypes::c_void,
    pub funcGetBusError: ::core::option::Option<
        unsafe extern "C" fn(dst: *mut crate::ctypes::c_void, len: u32) -> crate::ctypes::c_int,
    >,
    pub funcAppMgrFunction1: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
            a3: crate::ctypes::c_int,
            a4: crate::ctypes::c_int,
            a5: crate::ctypes::c_int,
            a6: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub funcGetShellPid: ::core::option::Option<unsafe extern "C" fn() -> SceUID>,
    pub unk_func_0x370: *mut crate::ctypes::c_void,
    pub unk_func_0x374: *mut crate::ctypes::c_void,
    pub unk_func_0x378: *mut crate::ctypes::c_void,
    pub unk_0x37C: crate::ctypes::c_int,
    pub unk_func_0x380: *mut crate::ctypes::c_void,
    pub unk_func_0x384: ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub unk_func_0x388: ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub unk_func_0x38C: *mut crate::ctypes::c_void,
    pub unk_func_0x390: *mut crate::ctypes::c_void,
    pub funcHasNpTestFlag: ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>,
    pub unk_func_0x398: *mut crate::ctypes::c_void,
    pub funcLedSetMode: ::core::option::Option<
        unsafe extern "C" fn(
            led: crate::ctypes::c_int,
            mode: crate::ctypes::c_int,
            led_configuration: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub cached_sm_info_ranges: *mut SceKernelVARange,
    pub funcGetFunctionNameByNID: ::core::option::Option<
        unsafe extern "C" fn(
            funcnid: SceNID,
            name: *mut *const crate::ctypes::c_char,
        ) -> crate::ctypes::c_int,
    >,
    pub unk_0x3A8: *mut crate::ctypes::c_void,
    pub unk_0x3AC: *mut crate::ctypes::c_void,
    pub unk_struct_0x3B0: *mut crate::ctypes::c_void,
    pub unk_0x3B4: *mut crate::ctypes::c_void,
    pub unk_0x3B8: *mut crate::ctypes::c_void,
    pub unk_0x3BC: *mut crate::ctypes::c_void,
    pub unk_0x3C0: *mut crate::ctypes::c_void,
    pub process_handler: *mut SceSysrootProcessHandler,
    pub dbgp_handler: *mut SceSysrootDbgpHandler,
    pub unk_func_0x3CC: *mut crate::ctypes::c_void,
    pub unk_func_0x3D0: *mut crate::ctypes::c_void,
    pub unk_func_0x3D4: *mut crate::ctypes::c_void,
    pub unk_func_0x3D8: *mut crate::ctypes::c_void,
    pub unk_func_0x3DC: *mut crate::ctypes::c_void,
    pub unk_func_0x3E0: *mut crate::ctypes::c_void,
    pub funcAppMgrFunction3: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub funcAppMgrFunction4: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            a2: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub funcAppMgrFunction5: ::core::option::Option<
        unsafe extern "C" fn(a1: crate::ctypes::c_int) -> crate::ctypes::c_int,
    >,
    pub unk_func_0x3F0: *mut crate::ctypes::c_void,
    pub unk_struct_0x3F4: *mut crate::ctypes::c_void,
    pub unk_struct_0x3F8: *mut crate::ctypes::c_void,
    pub unk_struct_0x3FC: *mut crate::ctypes::c_void,
    pub unk_data_0x400: *mut crate::ctypes::c_void,
    pub unk_func_0x404: *mut crate::ctypes::c_void,
    pub unk_func_0x408: *mut crate::ctypes::c_void,
    pub unk_func_0x40C: *mut crate::ctypes::c_void,
    pub unk_func_0x410: *mut crate::ctypes::c_void,
    pub unk_func_0x414: *mut crate::ctypes::c_void,
    pub magic2: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUIDSysrootObject__bindgen_ty_1 {
    pub callback: ::core::option::Option<
        unsafe extern "C" fn(
            a1: crate::ctypes::c_int,
            args: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub args: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbAudioInDeviceInfo {
    pub vendor: u16,
    pub product: u16,
    pub _reserved: [SceUInt32; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbAudioInDeviceListItem {
    pub device_id: SceUInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdAttachCompositeParam {
    pub driver_id: u32,
    pub bus: u32,
    pub device: u32,
    pub unk3: u32,
    pub unk4: u32,
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
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDeviceAddress {
    pub unk0: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_ushort,
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
pub struct SceUsbdDeviceInfo {
    pub port: crate::ctypes::c_uint,
    pub device_num: crate::ctypes::c_uint,
    pub unk3: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDevicePipe {
    pub device_id: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_uint,
    pub unk2: crate::ctypes::c_uint,
    pub unk3: crate::ctypes::c_uint,
    pub unk4: crate::ctypes::c_uint,
    pub unk5: crate::ctypes::c_uint,
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
pub struct SceUsbdHidDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdHID0: u8,
    pub bcdHID1: u8,
    pub bCountryCode: u8,
    pub bNumDescriptors: u8,
    pub SubDescriptorInfo: __IncompleteArrayField<SceUsbdHidSubDescriptorInfo>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdHidSubDescriptorInfo {
    pub bDescriptorType: u8,
    pub wDescriptorLength0: u8,
    pub wDescriptorLength1: u8,
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
pub struct SceUsbdIsochTransfer {
    pub unk0: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_uint,
    pub unk2: crate::ctypes::c_uint,
    pub unk3: crate::ctypes::c_uint,
    pub unk4: crate::ctypes::c_uint,
    pub unk5: crate::ctypes::c_uint,
    pub unk6: crate::ctypes::c_uint,
    pub unk7: crate::ctypes::c_uint,
    pub unk8: crate::ctypes::c_uint,
    pub unk9: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdIsochTransferStatus {
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: *mut usize,
    pub unk3: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdReceiveEvent {
    pub unk0: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_uint,
    pub unk2: crate::ctypes::c_uint,
    pub unk3: crate::ctypes::c_uint,
    pub unk4: crate::ctypes::c_uint,
    pub unk5: crate::ctypes::c_uint,
    pub transfer_id: crate::ctypes::c_uint,
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
pub struct SceUsbdTransferData {
    pub pipe: crate::ctypes::c_uint,
    pub data: *const crate::ctypes::c_void,
    pub data_size: crate::ctypes::c_uint,
    pub transferred: *mut crate::ctypes::c_void,
    pub timeout: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdTransferStatus {
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SceVideodecQueryInitInfo {
    pub reserved: [u8; 32usize],
    pub hwAvc: SceVideodecQueryInitInfoHwAvcdec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceVideodecQueryInitInfoHwAvcdec {
    pub size: u32,
    pub horizontal: u32,
    pub vertical: u32,
    pub numOfRefFrames: u32,
    pub numOfStreams: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceVideodecTimeStamp {
    pub upper: u32,
    pub lower: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SulphaNgsModuleQuery {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SulphaNgsRegistration {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VideoExportInputParam {
    pub path: [crate::ctypes::c_char; 1024usize],
    pub reserved: [crate::ctypes::c_char; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VideoExportOutputParam {
    pub path: [crate::ctypes::c_char; 1024usize],
    pub reserved: [crate::ctypes::c_char; 8usize],
}
pub const BOOT_INTF_SUBCLASS: u32 = 1;
pub const BOOT_PROTOCOL: SceUdcdProtocol = 0;
pub const EHCI_CC_BABBLE: u32 = 48;
pub const EHCI_CC_DATABUF: u32 = 64;
pub const EHCI_CC_HALTED: u32 = 80;
pub const EHCI_CC_MISSED_MICRO_FRAME: u32 = 16;
pub const EHCI_CC_XACT: u32 = 32;
pub const HID_DESCRIPTOR_HID: SceUdcdHidDescriptor = 33;
pub const HID_DESCRIPTOR_REPORT: SceUdcdHidDescriptor = 34;
pub const HID_DESRIPTOR_PHY: SceUdcdHidDescriptor = 35;
pub const HID_INTF: u32 = 3;
pub const HID_PROTOCOL_KEYBOARD: SceUdcdHidProtocol = 1;
pub const HID_PROTOCOL_MOUSE: SceUdcdHidProtocol = 2;
pub const HID_PROTOCOL_NONE: SceUdcdHidProtocol = 0;
pub const HID_REQUEST_GET_IDLE: SceUdcdHidRequest = 2;
pub const HID_REQUEST_GET_PROTOCOL: SceUdcdHidRequest = 3;
pub const HID_REQUEST_GET_REPORT: SceUdcdHidRequest = 1;
pub const HID_REQUEST_SET_IDLE: SceUdcdHidRequest = 10;
pub const HID_REQUEST_SET_PROTOCOL: SceUdcdHidRequest = 11;
pub const HID_REQUEST_SET_REPORT: SceUdcdHidRequest = 9;
pub const OHCI_CC_BIT_STUFFING: u32 = 2;
pub const OHCI_CC_BUFFER_OVERRUN: u32 = 12;
pub const OHCI_CC_BUFFER_UNDERRUN: u32 = 13;
pub const OHCI_CC_CRC: u32 = 1;
pub const OHCI_CC_DATA_OVERRUN: u32 = 8;
pub const OHCI_CC_DATA_TOGGLE_MISMATCH: u32 = 3;
pub const OHCI_CC_DATA_UNDERRUN: u32 = 9;
pub const OHCI_CC_DEVICE_NOT_RESPONDING: u32 = 5;
pub const OHCI_CC_NO_ERROR: u32 = 0;
pub const OHCI_CC_NOT_ACCESSED1: u32 = 14;
pub const OHCI_CC_NOT_ACCESSED2: u32 = 15;
pub const OHCI_CC_PID_CHECK_FAILURE: u32 = 6;
pub const OHCI_CC_STALL: u32 = 4;
pub const OHCI_CC_UNEXPECTED_PID: u32 = 7;
pub const __PSP2FILEHASH__: u32 = 0;
pub const PSP2_SDK_VERSION: u32 = 56033297;
pub const RPT_PROTOCOL: SceUdcdProtocol = 1;
pub const SCE_APPMGR_APPLICATION_MODE_A: SceAppMgrApplicationMode = 2;
pub const SCE_APPMGR_APPLICATION_MODE_B: SceAppMgrApplicationMode = 3;
pub const SCE_APPMGR_APPLICATION_MODE_C: SceAppMgrApplicationMode = 4;
pub const SCE_APPMGR_ERROR_BGM_PORT_BUSY: SceAppMgrErrorCode = 2155884544;
pub const SCE_APPMGR_ERROR_BUSY: SceAppMgrErrorCode = 2155880448;
pub const SCE_APPMGR_ERROR_INVALID: SceAppMgrErrorCode = 2155880474;
pub const SCE_APPMGR_ERROR_INVALID_SELF_PATH: SceAppMgrErrorCode = 2155880478;
pub const SCE_APPMGR_ERROR_NOEXEC: SceAppMgrErrorCode = 2155880479;
pub const SCE_APPMGR_ERROR_NULL_POINTER: SceAppMgrErrorCode = 2155880470;
pub const SCE_APPMGR_ERROR_STATE: SceAppMgrErrorCode = 2155880467;
pub const SCE_APPMGR_ERROR_TOO_LONG_ARGV: SceAppMgrErrorCode = 2155880477;
pub const SCE_APPMGR_INFOBAR_COLOR_BLACK: SceAppMgrInfoBarColor = 0;
pub const SCE_APPMGR_INFOBAR_COLOR_WHITE: SceAppMgrInfoBarColor = 1;
pub const SCE_APPMGR_INFOBAR_TRANSPARENCY_OPAQUE: SceAppMgrInfoBarTransparency = 0;
pub const SCE_APPMGR_INFOBAR_TRANSPARENCY_TRANSLUCENT: SceAppMgrInfoBarTransparency = 1;
pub const SCE_APPMGR_INFOBAR_VISIBILITY_INVISIBLE: SceAppMgrInfoBarVisibility = 0;
pub const SCE_APPMGR_INFOBAR_VISIBILITY_VISIBLE: SceAppMgrInfoBarVisibility = 1;
pub const SCE_APPMGR_MAX_APP_NAME_LENGTH: u32 = 31;
pub const SCE_APPMGR_SYSTEMEVENT_ON_NP_MESSAGE_ARRIVED: SceAppMgrSystemEventType = 268435461;
pub const SCE_APPMGR_SYSTEMEVENT_ON_RESUME: SceAppMgrSystemEventType = 268435459;
pub const SCE_APPMGR_SYSTEMEVENT_ON_STORE_PURCHASE: SceAppMgrSystemEventType = 268435460;
pub const SCE_APPMGR_SYSTEMEVENT_ON_STORE_REDEMPTION: SceAppMgrSystemEventType = 268435462;
pub const SCE_APPUTIL_ERROR_APPEVENT_PARSE_INVALID_DATA: SceAppUtilErrorCode = 2148533792;
pub const SCE_APPUTIL_ERROR_BUSY: SceAppUtilErrorCode = 2148533763;
pub const SCE_APPUTIL_ERROR_DRM_NO_ENTITLEMENT: SceAppUtilErrorCode = 2148533856;
pub const SCE_APPUTIL_ERROR_MOUNT_LIMIT_OVER: SceAppUtilErrorCode = 2148533894;
pub const SCE_APPUTIL_ERROR_MUSIC_DEVICE_NOT_FOUND: SceAppUtilErrorCode = 2148533893;
pub const SCE_APPUTIL_ERROR_NO_MEMORY: SceAppUtilErrorCode = 2148533762;
pub const SCE_APPUTIL_ERROR_NO_PERMISSION: SceAppUtilErrorCode = 2148533765;
pub const SCE_APPUTIL_ERROR_NOT_INITIALIZED: SceAppUtilErrorCode = 2148533761;
pub const SCE_APPUTIL_ERROR_NOT_MOUNTED: SceAppUtilErrorCode = 2148533764;
pub const SCE_APPUTIL_ERROR_PARAMETER: SceAppUtilErrorCode = 2148533760;
pub const SCE_APPUTIL_ERROR_PASSCODE_MISMATCH: SceAppUtilErrorCode = 2148533766;
pub const SCE_APPUTIL_ERROR_PHOTO_DEVICE_NOT_FOUND: SceAppUtilErrorCode = 2148533888;
pub const SCE_APPUTIL_ERROR_SAVEDATA_NO_SPACE_FS: SceAppUtilErrorCode = 2148533827;
pub const SCE_APPUTIL_ERROR_SAVEDATA_NO_SPACE_QUOTA: SceAppUtilErrorCode = 2148533826;
pub const SCE_APPUTIL_ERROR_SAVEDATA_SLOT_EXISTS: SceAppUtilErrorCode = 2148533824;
pub const SCE_APPUTIL_ERROR_SAVEDATA_SLOT_NOT_FOUND: SceAppUtilErrorCode = 2148533825;
pub const SCE_APPUTIL_ERROR_STACKSIZE_TOO_SHORT: SceAppUtilErrorCode = 2148533920;
pub const SCE_APPUTIL_SAVEDATA_DATA_REMOVE_MODE_DEFAULT: SceAppUtilSaveDataRemoveMode = 0;
pub const SCE_APPUTIL_SAVEDATA_DATA_REMOVE_MODE_NO_SLOT: SceAppUtilSaveDataRemoveMode = 1;
pub const SCE_APPUTIL_SAVEDATA_DATA_SAVE_MODE_DIRECTORY: SceAppUtilSaveDataSaveMode = 2;
pub const SCE_APPUTIL_SAVEDATA_DATA_SAVE_MODE_FILE: SceAppUtilSaveDataSaveMode = 0;
pub const SCE_ATRAC_ALIGNMENT_SIZE: u32 = 256;
pub const SCE_ATRAC_AT9_MAX_CH_IN_DECODER: u32 = 2;
pub const SCE_ATRAC_AT9_MAX_FRAME_SAMPLES: u32 = 256;
pub const SCE_ATRAC_AT9_MAX_OUTPUT_FRAMES: u32 = 8;
pub const SCE_ATRAC_AT9_MAX_TOTAL_CH: u32 = 16;
pub const SCE_ATRAC_AT9_MIN_LOOP_SAMPLES: u32 = 3072;
pub const SCE_ATRAC_DECODER_STATUS_ALL_DATA_IS_ON_MEMORY: SceAtracDecoderStatus = 2;
pub const SCE_ATRAC_DECODER_STATUS_ALL_DATA_WAS_DECODED: SceAtracDecoderStatus = 1;
pub const SCE_ATRAC_DECODER_STATUS_LOOP_PART_IS_ON_MEMORY: SceAtracDecoderStatus = 8;
pub const SCE_ATRAC_DECODER_STATUS_NONLOOP_PART_IS_ON_MEMORY: SceAtracDecoderStatus = 4;
pub const SCE_ATRAC_ERROR_ADDED_DATA_IS_TOO_BIG: SceAtracErrorCode = 2153971731;
pub const SCE_ATRAC_ERROR_ALL_DATA_WAS_DECODED: SceAtracErrorCode = 2153971729;
pub const SCE_ATRAC_ERROR_ALREADY_CREATED: SceAtracErrorCode = 2153971718;
pub const SCE_ATRAC_ERROR_DATA_SHORTAGE_IN_BUFFER: SceAtracErrorCode = 2153971728;
pub const SCE_ATRAC_ERROR_INVALID_ALIGNMENT: SceAtracErrorCode = 2153971717;
pub const SCE_ATRAC_ERROR_INVALID_DATA: SceAtracErrorCode = 2153971722;
pub const SCE_ATRAC_ERROR_INVALID_HANDLE: SceAtracErrorCode = 2153971724;
pub const SCE_ATRAC_ERROR_INVALID_LOOP_NUM: SceAtracErrorCode = 2153971760;
pub const SCE_ATRAC_ERROR_INVALID_LOOP_STATUS: SceAtracErrorCode = 2153971735;
pub const SCE_ATRAC_ERROR_INVALID_MAX_OUTPUT_SAMPLES: SceAtracErrorCode = 2153971730;
pub const SCE_ATRAC_ERROR_INVALID_POINTER: SceAtracErrorCode = 2153971712;
pub const SCE_ATRAC_ERROR_INVALID_SAMPLE: SceAtracErrorCode = 2153971733;
pub const SCE_ATRAC_ERROR_INVALID_SIZE: SceAtracErrorCode = 2153971713;
pub const SCE_ATRAC_ERROR_INVALID_TOTAL_CH: SceAtracErrorCode = 2153971716;
pub const SCE_ATRAC_ERROR_INVALID_TYPE: SceAtracErrorCode = 2153971715;
pub const SCE_ATRAC_ERROR_INVALID_WORD_LENGTH: SceAtracErrorCode = 2153971714;
pub const SCE_ATRAC_ERROR_MAIN_BUFFER_SIZE_IS_TOO_SMALL: SceAtracErrorCode = 2153971726;
pub const SCE_ATRAC_ERROR_NEED_SUB_BUFFER: SceAtracErrorCode = 2153971732;
pub const SCE_ATRAC_ERROR_NO_NEED_SUB_BUFFER: SceAtracErrorCode = 2153971734;
pub const SCE_ATRAC_ERROR_NOT_CREATED: SceAtracErrorCode = 2153971719;
pub const SCE_ATRAC_ERROR_READ_SIZE_IS_TOO_SMALL: SceAtracErrorCode = 2153971723;
pub const SCE_ATRAC_ERROR_READ_SIZE_OVER_BUFFER: SceAtracErrorCode = 2153971725;
pub const SCE_ATRAC_ERROR_REMAIN_VALID_HANDLE: SceAtracErrorCode = 2153971736;
pub const SCE_ATRAC_ERROR_SHORTAGE_OF_CH: SceAtracErrorCode = 2153971720;
pub const SCE_ATRAC_ERROR_SUB_BUFFER_SIZE_IS_TOO_SMALL: SceAtracErrorCode = 2153971727;
pub const SCE_ATRAC_ERROR_UNSUPPORTED_DATA: SceAtracErrorCode = 2153971721;
pub const SCE_ATRAC_INFINITE_LOOP_NUM: i32 = -1;
pub const SCE_ATRAC_INFINITE_SAMPLES: i32 = -1;
pub const SCE_ATRAC_LOOP_STATUS_NON_RESETABLE_PART: SceAtracLoopStatus = 0;
pub const SCE_ATRAC_LOOP_STATUS_RESETABLE_PART: SceAtracLoopStatus = 1;
pub const SCE_ATRAC_MAX_OUTPUT_SAMPLES: u32 = 2048;
pub const SCE_ATRAC_TYPE_AT9: u32 = 8195;
pub const SCE_ATRAC_WORD_LENGTH_16BITS: u32 = 16;
pub const SCE_AUDIO_ALC_MODE1: SceAudioOutAlcMode = 1;
pub const SCE_AUDIO_ALC_MODE_MAX: SceAudioOutAlcMode = 2;
pub const SCE_AUDIO_ALC_OFF: SceAudioOutAlcMode = 0;
pub const SCE_AUDIODEC_AAC_ERROR_INVALID_CH: SceAudiodecErrorCode = 2155819008;
pub const SCE_AUDIODEC_AAC_EXTRA_ACCESS_SIZE: u32 = 0;
pub const SCE_AUDIODEC_AAC_MAX_CH_IN_DECODER: u32 = 2;
pub const SCE_AUDIODEC_AAC_MAX_ES_SIZE: u32 = 1536;
pub const SCE_AUDIODEC_AAC_MAX_NFRAMES: u32 = 1;
pub const SCE_AUDIODEC_AAC_MAX_NSTREAMS: u32 = 6;
pub const SCE_AUDIODEC_AAC_MAX_SAMPLES: u32 = 2048;
pub const SCE_AUDIODEC_AAC_MAX_STREAMS: u32 = 8;
pub const SCE_AUDIODEC_ALIGNMENT_SIZE: u32 = 256;
pub const SCE_AUDIODEC_AT9_ERROR_INVALID_CONFIG: SceAudiodecErrorCode = 2155814912;
pub const SCE_AUDIODEC_AT9_EXTRA_ACCESS_SIZE: u32 = 0;
pub const SCE_AUDIODEC_AT9_MAX_CH_IN_DECODER: u32 = 2;
pub const SCE_AUDIODEC_AT9_MAX_CH_IN_LIBRARY: u32 = 16;
pub const SCE_AUDIODEC_AT9_MAX_ES_SIZE: u32 = 1024;
pub const SCE_AUDIODEC_AT9_MAX_NFRAMES: u32 = 8;
pub const SCE_AUDIODEC_AT9_MAX_NSTREAMS: u32 = 6;
pub const SCE_AUDIODEC_AT9_MAX_SAMPLES: u32 = 256;
pub const SCE_AUDIODEC_CELP_BIT_RATE_10700BPS: SceAudiodecCelpBitrate = 10700;
pub const SCE_AUDIODEC_CELP_BIT_RATE_11800BPS: SceAudiodecCelpBitrate = 11800;
pub const SCE_AUDIODEC_CELP_BIT_RATE_12200BPS: SceAudiodecCelpBitrate = 12200;
pub const SCE_AUDIODEC_CELP_BIT_RATE_3850BPS: SceAudiodecCelpBitrate = 3850;
pub const SCE_AUDIODEC_CELP_BIT_RATE_4650BPS: SceAudiodecCelpBitrate = 4650;
pub const SCE_AUDIODEC_CELP_BIT_RATE_5700BPS: SceAudiodecCelpBitrate = 5700;
pub const SCE_AUDIODEC_CELP_BIT_RATE_6600BPS: SceAudiodecCelpBitrate = 6600;
pub const SCE_AUDIODEC_CELP_BIT_RATE_7300BPS: SceAudiodecCelpBitrate = 7300;
pub const SCE_AUDIODEC_CELP_BIT_RATE_8700BPS: SceAudiodecCelpBitrate = 8700;
pub const SCE_AUDIODEC_CELP_BIT_RATE_9900BPS: SceAudiodecCelpBitrate = 9900;
pub const SCE_AUDIODEC_CELP_ERROR_INVALID_CONFIG: SceAudiodecErrorCode = 2155821056;
pub const SCE_AUDIODEC_CELP_EXTRA_ACCESS_SIZE: u32 = 0;
pub const SCE_AUDIODEC_CELP_MAX_CH_IN_DECODER: u32 = 1;
pub const SCE_AUDIODEC_CELP_MAX_ES_SIZE: u32 = 24;
pub const SCE_AUDIODEC_CELP_MAX_NFRAMES: u32 = 1;
pub const SCE_AUDIODEC_CELP_MAX_NSTREAMS: u32 = 7;
pub const SCE_AUDIODEC_CELP_MAX_SAMPLES: u32 = 320;
pub const SCE_AUDIODEC_CELP_MAX_STREAMS: u32 = 8;
pub const SCE_AUDIODEC_CELP_MPE: u32 = 0;
pub const SCE_AUDIODEC_CELP_SAMPLING_RATE_8KHZ: u32 = 8000;
pub const SCE_AUDIODEC_ERROR_A_HANDLE_IN_USE: SceAudiodecErrorCode = 2155806726;
pub const SCE_AUDIODEC_ERROR_ALL_HANDLES_IN_USE: SceAudiodecErrorCode = 2155806727;
pub const SCE_AUDIODEC_ERROR_ALREADY_INITIALIZED: SceAudiodecErrorCode = 2155806723;
pub const SCE_AUDIODEC_ERROR_API_FAIL: SceAudiodecErrorCode = 2155806720;
pub const SCE_AUDIODEC_ERROR_BUSY: SceAudiodecErrorCode = 2155806739;
pub const SCE_AUDIODEC_ERROR_CH_SHORTAGE: SceAudiodecErrorCode = 2155806731;
pub const SCE_AUDIODEC_ERROR_DIFFERENT_TYPES: SceAudiodecErrorCode = 2155806737;
pub const SCE_AUDIODEC_ERROR_INVALID_HANDLE: SceAudiodecErrorCode = 2155806729;
pub const SCE_AUDIODEC_ERROR_INVALID_INIT_PARAM: SceAudiodecErrorCode = 2155806722;
pub const SCE_AUDIODEC_ERROR_INVALID_NFRAMES: SceAudiodecErrorCode = 2155806735;
pub const SCE_AUDIODEC_ERROR_INVALID_NSTREAMS: SceAudiodecErrorCode = 2155806736;
pub const SCE_AUDIODEC_ERROR_INVALID_PTR: SceAudiodecErrorCode = 2155806728;
pub const SCE_AUDIODEC_ERROR_INVALID_SIZE: SceAudiodecErrorCode = 2155806733;
pub const SCE_AUDIODEC_ERROR_INVALID_TYPE: SceAudiodecErrorCode = 2155806721;
pub const SCE_AUDIODEC_ERROR_INVALID_WORD_LENGTH: SceAudiodecErrorCode = 2155806732;
pub const SCE_AUDIODEC_ERROR_NOT_HANDLE_IN_USE: SceAudiodecErrorCode = 2155806730;
pub const SCE_AUDIODEC_ERROR_NOT_INITIALIZED: SceAudiodecErrorCode = 2155806725;
pub const SCE_AUDIODEC_ERROR_OUT_OF_MEMORY: SceAudiodecErrorCode = 2155806724;
pub const SCE_AUDIODEC_ERROR_SAME_HANDLES: SceAudiodecErrorCode = 2155806738;
pub const SCE_AUDIODEC_ERROR_UNSUPPORTED: SceAudiodecErrorCode = 2155806734;
pub const SCE_AUDIODEC_MP3_ERROR_INVALID_CH: SceAudiodecErrorCode = 2155816960;
pub const SCE_AUDIODEC_MP3_ERROR_INVALID_MPEG_VERSION: SceAudiodecErrorCode = 2155816961;
pub const SCE_AUDIODEC_MP3_EXTRA_ACCESS_SIZE: u32 = 0;
pub const SCE_AUDIODEC_MP3_MAX_CH_IN_DECODER: u32 = 2;
pub const SCE_AUDIODEC_MP3_MAX_ES_SIZE: u32 = 1441;
pub const SCE_AUDIODEC_MP3_MAX_NFRAMES: u32 = 1;
pub const SCE_AUDIODEC_MP3_MAX_NSTREAMS: u32 = 6;
pub const SCE_AUDIODEC_MP3_MAX_SAMPLES: u32 = 1152;
pub const SCE_AUDIODEC_MP3_MAX_STREAMS: u32 = 8;
pub const SCE_AUDIODEC_MP3_MPEG_VERSION_1: SceAudiodecMpegVersion = 3;
pub const SCE_AUDIODEC_MP3_MPEG_VERSION_2: SceAudiodecMpegVersion = 2;
pub const SCE_AUDIODEC_MP3_MPEG_VERSION_2_5: SceAudiodecMpegVersion = 0;
pub const SCE_AUDIODEC_MP3_MPEG_VERSION_RESERVED: SceAudiodecMpegVersion = 1;
pub const SCE_AUDIODEC_TYPE_AAC: SceAudiodecType = 4101;
pub const SCE_AUDIODEC_TYPE_AT9: SceAudiodecType = 4099;
pub const SCE_AUDIODEC_TYPE_CELP: SceAudiodecType = 4102;
pub const SCE_AUDIODEC_TYPE_MP3: SceAudiodecType = 4100;
pub const SCE_AUDIODEC_WORD_LENGTH_16BITS: u32 = 16;
pub const SCE_AUDIOENC_CELP_BIT_RATE_10700BPS: SceAudioencCelpBitrate = 10700;
pub const SCE_AUDIOENC_CELP_BIT_RATE_11800BPS: SceAudioencCelpBitrate = 11800;
pub const SCE_AUDIOENC_CELP_BIT_RATE_12200BPS: SceAudioencCelpBitrate = 12200;
pub const SCE_AUDIOENC_CELP_BIT_RATE_3850BPS: SceAudioencCelpBitrate = 3850;
pub const SCE_AUDIOENC_CELP_BIT_RATE_4650BPS: SceAudioencCelpBitrate = 4650;
pub const SCE_AUDIOENC_CELP_BIT_RATE_5700BPS: SceAudioencCelpBitrate = 5700;
pub const SCE_AUDIOENC_CELP_BIT_RATE_6600BPS: SceAudioencCelpBitrate = 6600;
pub const SCE_AUDIOENC_CELP_BIT_RATE_7300BPS: SceAudioencCelpBitrate = 7300;
pub const SCE_AUDIOENC_CELP_BIT_RATE_8700BPS: SceAudioencCelpBitrate = 8700;
pub const SCE_AUDIOENC_CELP_BIT_RATE_9900BPS: SceAudioencCelpBitrate = 9900;
pub const SCE_AUDIOENC_CELP_ERROR_INVALID_CONFIG: SceAudioencCelpErrorCode = 2156269569;
pub const SCE_AUDIOENC_CELP_MAX_ES_SIZE: u32 = 24;
pub const SCE_AUDIOENC_CELP_MAX_SAMPLES: u32 = 320;
pub const SCE_AUDIOENC_CELP_MAX_STREAMS: u32 = 1;
pub const SCE_AUDIOENC_CELP_MPE: u32 = 0;
pub const SCE_AUDIOENC_CELP_SAMPLING_RATE_8KHZ: u32 = 8000;
pub const SCE_AUDIOENC_ERROR_A_HANDLE_IN_USE: SceAudioencErrorCode = 2156265478;
pub const SCE_AUDIOENC_ERROR_ALL_HANDLES_IN_USE: SceAudioencErrorCode = 2156265479;
pub const SCE_AUDIOENC_ERROR_ALREADY_INITIALIZED: SceAudioencErrorCode = 2156265475;
pub const SCE_AUDIOENC_ERROR_API_FAIL: SceAudioencErrorCode = 2156265472;
pub const SCE_AUDIOENC_ERROR_CH_SHORTAGE: SceAudioencErrorCode = 2156265483;
pub const SCE_AUDIOENC_ERROR_INVALID_ALIGNMENT: SceAudioencErrorCode = 2156265486;
pub const SCE_AUDIOENC_ERROR_INVALID_HANDLE: SceAudioencErrorCode = 2156265481;
pub const SCE_AUDIOENC_ERROR_INVALID_INIT_PARAM: SceAudioencErrorCode = 2156265474;
pub const SCE_AUDIOENC_ERROR_INVALID_PTR: SceAudioencErrorCode = 2156265480;
pub const SCE_AUDIOENC_ERROR_INVALID_SIZE: SceAudioencErrorCode = 2156265485;
pub const SCE_AUDIOENC_ERROR_INVALID_TYPE: SceAudioencErrorCode = 2156265473;
pub const SCE_AUDIOENC_ERROR_INVALID_WORD_LENGTH: SceAudioencErrorCode = 2156265484;
pub const SCE_AUDIOENC_ERROR_NOT_HANDLE_IN_USE: SceAudioencErrorCode = 2156265482;
pub const SCE_AUDIOENC_ERROR_NOT_INITIALIZED: SceAudioencErrorCode = 2156265477;
pub const SCE_AUDIOENC_ERROR_OUT_OF_MEMORY: SceAudioencErrorCode = 2156265476;
pub const SCE_AUDIOENC_ERROR_UNSUPPORTED: SceAudioencErrorCode = 2156265487;
pub const SCE_AUDIOENC_TYPE_CELP: u32 = 8198;
pub const SCE_AUDIOENC_WORD_LENGTH_16BITS: u32 = 16;
pub const SCE_AUDIO_IN_ERROR_BUSY: SceAudioInErrorCode = 2149974282;
pub const SCE_AUDIO_IN_ERROR_FATAL: SceAudioInErrorCode = 2149974272;
pub const SCE_AUDIO_IN_ERROR_INVALID_PARAMETER: SceAudioInErrorCode = 2149974283;
pub const SCE_AUDIO_IN_ERROR_INVALID_POINTER: SceAudioInErrorCode = 2149974277;
pub const SCE_AUDIO_IN_ERROR_INVALID_PORT: SceAudioInErrorCode = 2149974273;
pub const SCE_AUDIO_IN_ERROR_INVALID_PORT_PARAM: SceAudioInErrorCode = 2149974278;
pub const SCE_AUDIO_IN_ERROR_INVALID_PORT_TYPE: SceAudioInErrorCode = 2149974276;
pub const SCE_AUDIO_IN_ERROR_INVALID_SAMPLE_FREQ: SceAudioInErrorCode = 2149974275;
pub const SCE_AUDIO_IN_ERROR_INVALID_SIZE: SceAudioInErrorCode = 2149974274;
pub const SCE_AUDIO_IN_ERROR_NOT_OPENED: SceAudioInErrorCode = 2149974281;
pub const SCE_AUDIO_IN_ERROR_OUT_OF_MEMORY: SceAudioInErrorCode = 2149974280;
pub const SCE_AUDIO_IN_ERROR_PORT_FULL: SceAudioInErrorCode = 2149974279;
pub const SCE_AUDIO_IN_GETSTATUS_MUTE: SceAudioInParam = 1;
pub const SCE_AUDIO_IN_PARAM_FORMAT_S16_MONO: SceAudioInParam = 0;
pub const SCE_AUDIO_IN_PORT_TYPE_RAW: SceAudioInPortType = 2;
pub const SCE_AUDIO_IN_PORT_TYPE_VOICE: SceAudioInPortType = 0;
pub const SCE_AUDIO_MAX_LEN: u32 = 65472;
pub const SCE_AUDIO_MIN_LEN: u32 = 64;
pub const SCE_AUDIO_OUT_CONFIG_TYPE_FREQ: SceAudioOutConfigType = 1;
pub const SCE_AUDIO_OUT_CONFIG_TYPE_LEN: SceAudioOutConfigType = 0;
pub const SCE_AUDIO_OUT_CONFIG_TYPE_MODE: SceAudioOutConfigType = 2;
pub const SCE_AUDIO_OUT_ERROR_BUSY: SceAudioOutErrorCode = 2149974018;
pub const SCE_AUDIO_OUT_ERROR_INVALID_CONF_TYPE: SceAudioOutErrorCode = 2149974028;
pub const SCE_AUDIO_OUT_ERROR_INVALID_FORMAT: SceAudioOutErrorCode = 2149974023;
pub const SCE_AUDIO_OUT_ERROR_INVALID_FX_TYPE: SceAudioOutErrorCode = 2149974027;
pub const SCE_AUDIO_OUT_ERROR_INVALID_POINTER: SceAudioOutErrorCode = 2149974020;
pub const SCE_AUDIO_OUT_ERROR_INVALID_PORT: SceAudioOutErrorCode = 2149974019;
pub const SCE_AUDIO_OUT_ERROR_INVALID_PORT_TYPE: SceAudioOutErrorCode = 2149974026;
pub const SCE_AUDIO_OUT_ERROR_INVALID_SAMPLE_FREQ: SceAudioOutErrorCode = 2149974024;
pub const SCE_AUDIO_OUT_ERROR_INVALID_SIZE: SceAudioOutErrorCode = 2149974022;
pub const SCE_AUDIO_OUT_ERROR_INVALID_VOLUME: SceAudioOutErrorCode = 2149974025;
pub const SCE_AUDIO_OUT_ERROR_NOT_OPENED: SceAudioOutErrorCode = 2149974017;
pub const SCE_AUDIO_OUT_ERROR_OUT_OF_MEMORY: SceAudioOutErrorCode = 2149974029;
pub const SCE_AUDIO_OUT_ERROR_PORT_FULL: SceAudioOutErrorCode = 2149974021;
pub const SCE_AUDIO_OUT_MAX_VOL: u32 = 32768;
pub const SCE_AUDIO_OUT_MODE_MONO: SceAudioOutMode = 0;
pub const SCE_AUDIO_OUT_MODE_STEREO: SceAudioOutMode = 1;
pub const SCE_AUDIO_OUT_PARAM_FORMAT_S16_MONO: SceAudioOutParam = 0;
pub const SCE_AUDIO_OUT_PARAM_FORMAT_S16_STEREO: SceAudioOutParam = 1;
pub const SCE_AUDIO_OUT_PORT_TYPE_BGM: SceAudioOutPortType = 1;
pub const SCE_AUDIO_OUT_PORT_TYPE_MAIN: SceAudioOutPortType = 0;
pub const SCE_AUDIO_OUT_PORT_TYPE_VOICE: SceAudioOutPortType = 2;
pub const SCE_AUDIO_VOLUME_0DB: u32 = 32768;
pub const SCE_AUDIO_VOLUME_FLAG_L_CH: SceAudioOutChannelFlag = 1;
pub const SCE_AUDIO_VOLUME_FLAG_R_CH: SceAudioOutChannelFlag = 2;
pub const SCE_AVCDEC_ERROR_ALREADY_USED: SceAvcdecErrorCode = 2153906184;
pub const SCE_AVCDEC_ERROR_ES_BUFFER_FULL: SceAvcdecErrorCode = 2153906186;
pub const SCE_AVCDEC_ERROR_INITIALIZE: SceAvcdecErrorCode = 2153906187;
pub const SCE_AVCDEC_ERROR_INVALID_ARGUMENT_SIZE: SceAvcdecErrorCode = 2153906190;
pub const SCE_AVCDEC_ERROR_INVALID_COLOR_FORMAT: SceAvcdecErrorCode = 2153906182;
pub const SCE_AVCDEC_ERROR_INVALID_PARAM: SceAvcdecErrorCode = 2153906178;
pub const SCE_AVCDEC_ERROR_INVALID_POINTER: SceAvcdecErrorCode = 2153906185;
pub const SCE_AVCDEC_ERROR_INVALID_STATE: SceAvcdecErrorCode = 2153906180;
pub const SCE_AVCDEC_ERROR_INVALID_STREAM: SceAvcdecErrorCode = 2153906189;
pub const SCE_AVCDEC_ERROR_INVALID_TYPE: SceAvcdecErrorCode = 2153906177;
pub const SCE_AVCDEC_ERROR_NOT_INITIALIZE: SceAvcdecErrorCode = 2153906188;
pub const SCE_AVCDEC_ERROR_NOT_PHY_CONTINUOUS_MEMORY: SceAvcdecErrorCode = 2153906183;
pub const SCE_AVCDEC_ERROR_OUT_OF_MEMORY: SceAvcdecErrorCode = 2153906179;
pub const SCE_AVCDEC_ERROR_UNSUPPORT_IMAGE_SIZE: SceAvcdecErrorCode = 2153906181;
pub const SCE_AVCDEC_PIXELFORMAT_RGBA5551: SceAvcdecPixelFormat = 2;
pub const SCE_AVCDEC_PIXELFORMAT_RGBA565: SceAvcdecPixelFormat = 1;
pub const SCE_AVCDEC_PIXELFORMAT_RGBA8888: SceAvcdecPixelFormat = 0;
pub const SCE_AVCDEC_PIXELFORMAT_YUV420_PACKED_RASTER: SceAvcdecPixelFormat = 32;
pub const SCE_AVCDEC_PIXELFORMAT_YUV420_RASTER: SceAvcdecPixelFormat = 16;
pub const SCE_AVCONFIG_COLOR_SPACE_MODE_DEFAULT: SceAVConfigColorSpaceMode = 0;
pub const SCE_AVCONFIG_COLOR_SPACE_MODE_HIGH_CONTRAST: SceAVConfigColorSpaceMode = 1;
pub const SCE_AVPLAYER_AUDIO: SceAvPlayerStreamType = 1;
pub const SCE_AVPLAYER_ERROR_INVALID_PARAM: SceAvPlayerErrorCode = 2154430465;
pub const SCE_AVPLAYER_ERROR_OUT_OF_MEMORY: SceAvPlayerErrorCode = 2154430467;
pub const SCE_AVPLAYER_TIMEDTEXT: SceAvPlayerStreamType = 2;
pub const SCE_AVPLAYER_TRICK_SPEED_FAST_FORWARD_16X: SceAvPlayerTrickSpeeds = 1600;
pub const SCE_AVPLAYER_TRICK_SPEED_FAST_FORWARD_2X: SceAvPlayerTrickSpeeds = 200;
pub const SCE_AVPLAYER_TRICK_SPEED_FAST_FORWARD_32X: SceAvPlayerTrickSpeeds = 3200;
pub const SCE_AVPLAYER_TRICK_SPEED_FAST_FORWARD_4X: SceAvPlayerTrickSpeeds = 400;
pub const SCE_AVPLAYER_TRICK_SPEED_FAST_FORWARD_8X: SceAvPlayerTrickSpeeds = 800;
pub const SCE_AVPLAYER_TRICK_SPEED_NORMAL: SceAvPlayerTrickSpeeds = 100;
pub const SCE_AVPLAYER_TRICK_SPEED_REWIND_16X: SceAvPlayerTrickSpeeds = -1600;
pub const SCE_AVPLAYER_TRICK_SPEED_REWIND_32X: SceAvPlayerTrickSpeeds = -3200;
pub const SCE_AVPLAYER_TRICK_SPEED_REWIND_8X: SceAvPlayerTrickSpeeds = -800;
pub const SCE_AVPLAYER_VIDEO: SceAvPlayerStreamType = 0;
pub const SCE_BGAPP_UTIL_ERROR_INVALID_ARG: SceBgAppUtilErrorCode = 2148558082;
pub const SCE_BT_ERROR_ACL_TX_BUF_OVERFLOW: SceBtErrorCode = 2150566659;
pub const SCE_BT_ERROR_ACL_TX_CB_OVERFLOW: SceBtErrorCode = 2150566660;
pub const SCE_BT_ERROR_ATT_APPLICATION_ERROR_HI: SceBtErrorCode = 2150573055;
pub const SCE_BT_ERROR_ATT_APPLICATION_ERROR_LO: SceBtErrorCode = 2150572928;
pub const SCE_BT_ERROR_ATT_ATTRIBUTE_NOT_FOUND: SceBtErrorCode = 2150572810;
pub const SCE_BT_ERROR_ATT_ATTRIBUTE_NOT_LONG: SceBtErrorCode = 2150572811;
pub const SCE_BT_ERROR_ATT_BASE: SceBtErrorCode = 2150572800;
pub const SCE_BT_ERROR_ATT_INSUFFICIENT_AUTHENTICATION: SceBtErrorCode = 2150572805;
pub const SCE_BT_ERROR_ATT_INSUFFICIENT_AUTHORIZATION: SceBtErrorCode = 2150572808;
pub const SCE_BT_ERROR_ATT_INSUFFICIENT_ENCRYPTION: SceBtErrorCode = 2150572815;
pub const SCE_BT_ERROR_ATT_INSUFFICIENT_ENCRYPTION_KEY_SIZE: SceBtErrorCode = 2150572812;
pub const SCE_BT_ERROR_ATT_INSUFFICIENT_RESOURCES: SceBtErrorCode = 2150572817;
pub const SCE_BT_ERROR_ATT_INVALID_ATTRIBUTE_VALUE_LENGTH: SceBtErrorCode = 2150572813;
pub const SCE_BT_ERROR_ATT_INVALID_HANDLE: SceBtErrorCode = 2150572801;
pub const SCE_BT_ERROR_ATT_INVALID_OFFSET: SceBtErrorCode = 2150572807;
pub const SCE_BT_ERROR_ATT_INVALID_PDU: SceBtErrorCode = 2150572804;
pub const SCE_BT_ERROR_ATT_NOT_YET: SceBtErrorCode = 2150574599;
pub const SCE_BT_ERROR_ATT_PEPARE_QUEUE_FULL: SceBtErrorCode = 2150572809;
pub const SCE_BT_ERROR_ATT_READ_INVALID_INTERNAL: SceBtErrorCode = 2150574595;
pub const SCE_BT_ERROR_ATT_READ_INVALID_LENGTH: SceBtErrorCode = 2150574594;
pub const SCE_BT_ERROR_ATT_READ_INVALID_NO: SceBtErrorCode = 2150574593;
pub const SCE_BT_ERROR_ATT_READ_NOT_PERMITTED: SceBtErrorCode = 2150572802;
pub const SCE_BT_ERROR_ATT_REQUEST_NOT_SUPPORTED: SceBtErrorCode = 2150572806;
pub const SCE_BT_ERROR_ATT_UNLIKELY_ERROR: SceBtErrorCode = 2150572814;
pub const SCE_BT_ERROR_ATT_UNSUPPORTED_GROUP_TYPE: SceBtErrorCode = 2150572816;
pub const SCE_BT_ERROR_ATT_WRITE_INVALID_INTERNAL: SceBtErrorCode = 2150574598;
pub const SCE_BT_ERROR_ATT_WRITE_INVALID_LENGTH: SceBtErrorCode = 2150574597;
pub const SCE_BT_ERROR_ATT_WRITE_INVALID_NO: SceBtErrorCode = 2150574596;
pub const SCE_BT_ERROR_ATT_WRITE_NOT_PERMITTED: SceBtErrorCode = 2150572803;
pub const SCE_BT_ERROR_AUDIO_COMBI_NOT_FOUND: SceBtErrorCode = 2150567190;
pub const SCE_BT_ERROR_AUDIO_FREQ_IS_LE: SceBtErrorCode = 2150572553;
pub const SCE_BT_ERROR_AUDIO_FREQ_NOT_CONNECTED: SceBtErrorCode = 2150567183;
pub const SCE_BT_ERROR_AUDIO_INTERNAL_1: SceBtErrorCode = 2150567192;
pub const SCE_BT_ERROR_AUDIO_RECV_BAD_TYPE: SceBtErrorCode = 2150567186;
pub const SCE_BT_ERROR_AUDIO_RECV_BUSY: SceBtErrorCode = 2150567194;
pub const SCE_BT_ERROR_AUDIO_RECV_INVALID_LENGTH: SceBtErrorCode = 2150567178;
pub const SCE_BT_ERROR_AUDIO_RECV_IS_LE: SceBtErrorCode = 2150572552;
pub const SCE_BT_ERROR_AUDIO_RECV_NO_CAP: SceBtErrorCode = 2150567197;
pub const SCE_BT_ERROR_AUDIO_RECV_NOT_CONNECTED: SceBtErrorCode = 2150567176;
pub const SCE_BT_ERROR_AUDIO_RECV_NOT_STARTED: SceBtErrorCode = 2150567177;
pub const SCE_BT_ERROR_AUDIO_RECV_SERV_FAILED: SceBtErrorCode = 2150567198;
pub const SCE_BT_ERROR_AUDIO_SEND_BAD_TYPE: SceBtErrorCode = 2150567185;
pub const SCE_BT_ERROR_AUDIO_SEND_BUSY: SceBtErrorCode = 2150567193;
pub const SCE_BT_ERROR_AUDIO_SEND_INVALID_LENGTH: SceBtErrorCode = 2150567175;
pub const SCE_BT_ERROR_AUDIO_SEND_IS_LE: SceBtErrorCode = 2150572551;
pub const SCE_BT_ERROR_AUDIO_SEND_NO_CAP: SceBtErrorCode = 2150567195;
pub const SCE_BT_ERROR_AUDIO_SEND_NO_CP: SceBtErrorCode = 2150567184;
pub const SCE_BT_ERROR_AUDIO_SEND_NO_L2C: SceBtErrorCode = 2150567191;
pub const SCE_BT_ERROR_AUDIO_SEND_NOT_CONNECTED: SceBtErrorCode = 2150567173;
pub const SCE_BT_ERROR_AUDIO_SEND_NOT_STARTED: SceBtErrorCode = 2150567174;
pub const SCE_BT_ERROR_AUDIO_SEND_SERV_FAILED: SceBtErrorCode = 2150567196;
pub const SCE_BT_ERROR_AUDIO_START_INVALID_SERV: SceBtErrorCode = 2150567179;
pub const SCE_BT_ERROR_AUDIO_START_IS_LE: SceBtErrorCode = 2150572549;
pub const SCE_BT_ERROR_AUDIO_START_NO_CAP: SceBtErrorCode = 2150567170;
pub const SCE_BT_ERROR_AUDIO_START_NOT_CONNECTED: SceBtErrorCode = 2150567169;
pub const SCE_BT_ERROR_AUDIO_START_SERV_FAILED: SceBtErrorCode = 2150567181;
pub const SCE_BT_ERROR_AUDIO_STOP_INVALID_SERV: SceBtErrorCode = 2150567180;
pub const SCE_BT_ERROR_AUDIO_STOP_IS_LE: SceBtErrorCode = 2150572550;
pub const SCE_BT_ERROR_AUDIO_STOP_NO_CAP: SceBtErrorCode = 2150567172;
pub const SCE_BT_ERROR_AUDIO_STOP_NOT_CONNECTED: SceBtErrorCode = 2150567171;
pub const SCE_BT_ERROR_AUDIO_STOP_SERV_FAILED: SceBtErrorCode = 2150567182;
pub const SCE_BT_ERROR_AVCTP_IS_LE: SceBtErrorCode = 2150572547;
pub const SCE_BT_ERROR_AVCTP_NOT_CONNECTED: SceBtErrorCode = 2150565380;
pub const SCE_BT_ERROR_AVCTP_OPEN_NO_L2C: SceBtErrorCode = 2150565377;
pub const SCE_BT_ERROR_AVCTP_READ_NO_VOLUME: SceBtErrorCode = 2150565384;
pub const SCE_BT_ERROR_AVCTP_SEND_BUSY: SceBtErrorCode = 2150565381;
pub const SCE_BT_ERROR_AVCTP_SEND_NO_L2C: SceBtErrorCode = 2150565379;
pub const SCE_BT_ERROR_AVCTP_SEND_NO_PRESS: SceBtErrorCode = 2150565382;
pub const SCE_BT_ERROR_AVCTP_SEND_NO_RELEASE: SceBtErrorCode = 2150565383;
pub const SCE_BT_ERROR_AVCTP_SEND_NOT_RUBY: SceBtErrorCode = 2150565385;
pub const SCE_BT_ERROR_AVDTP_CLOSE_BAD_SERV: SceBtErrorCode = 2150565122;
pub const SCE_BT_ERROR_AVDTP_CLOSE_BAD_STATE: SceBtErrorCode = 2150565123;
pub const SCE_BT_ERROR_AVDTP_OPEN_NO_L2C: SceBtErrorCode = 2150565121;
pub const SCE_BT_ERROR_AVDTP_RECONF_BAD_SERV: SceBtErrorCode = 2150565129;
pub const SCE_BT_ERROR_AVDTP_RECONF_BAD_STATE: SceBtErrorCode = 2150565130;
pub const SCE_BT_ERROR_AVDTP_SEND_BAD_STATE: SceBtErrorCode = 2150565128;
pub const SCE_BT_ERROR_AVDTP_START_BAD_SERV: SceBtErrorCode = 2150565124;
pub const SCE_BT_ERROR_AVDTP_START_BAD_STATE: SceBtErrorCode = 2150565125;
pub const SCE_BT_ERROR_AVDTP_STOP_BAD_SERV: SceBtErrorCode = 2150565126;
pub const SCE_BT_ERROR_AVDTP_STOP_BAD_STATE: SceBtErrorCode = 2150565127;
pub const SCE_BT_ERROR_AVRCP_INVALID_PLAY_STATUS: SceBtErrorCode = 2150568450;
pub const SCE_BT_ERROR_AVRCP_TOO_LONG_TITLE: SceBtErrorCode = 2150568449;
pub const SCE_BT_ERROR_CB_NOT_REGISTERED: SceBtErrorCode = 2150566915;
pub const SCE_BT_ERROR_CB_OVERFLOW: SceBtErrorCode = 2150566916;
pub const SCE_BT_ERROR_CB_TOO_MANY: SceBtErrorCode = 2150566914;
pub const SCE_BT_ERROR_CONF_BT_INACTIVE: SceBtErrorCode = 2150569219;
pub const SCE_BT_ERROR_CONF_CANT_DISABLE_FUNCTION: SceBtErrorCode = 2150569224;
pub const SCE_BT_ERROR_CONF_CANT_ENABLE_FUNCTION: SceBtErrorCode = 2150569223;
pub const SCE_BT_ERROR_CONF_CANT_ENTER: SceBtErrorCode = 2150569248;
pub const SCE_BT_ERROR_CONF_CARD_NOT_FOUND: SceBtErrorCode = 2150569221;
pub const SCE_BT_ERROR_CONF_DISABLE_SUBINTR: SceBtErrorCode = 2150569228;
pub const SCE_BT_ERROR_CONF_ENABLE_SUBINTR: SceBtErrorCode = 2150569227;
pub const SCE_BT_ERROR_CONF_FUNCTION_NOT_FOUND: SceBtErrorCode = 2150569222;
pub const SCE_BT_ERROR_CONF_INVALID_VALUE: SceBtErrorCode = 2150569218;
pub const SCE_BT_ERROR_CONF_NOT_READY: SceBtErrorCode = 2150569217;
pub const SCE_BT_ERROR_CONF_OFF_TIMEOUT: SceBtErrorCode = 2150569230;
pub const SCE_BT_ERROR_CONF_ON_TIMEOUT: SceBtErrorCode = 2150569229;
pub const SCE_BT_ERROR_CONF_REGISTER_SUBINTR_HANDLER: SceBtErrorCode = 2150569225;
pub const SCE_BT_ERROR_CONF_RELEASE_SUBINTR_HANDLER: SceBtErrorCode = 2150569226;
pub const SCE_BT_ERROR_CONF_SUSPEND_TIMEOUT: SceBtErrorCode = 2150569231;
pub const SCE_BT_ERROR_CONF_TIMEOUT: SceBtErrorCode = 2150569220;
pub const SCE_BT_ERROR_CONNECT_START_BUSY: SceBtErrorCode = 2150564356;
pub const SCE_BT_ERROR_CONNECT_START_CONNECTED: SceBtErrorCode = 2150564362;
pub const SCE_BT_ERROR_CONNECT_START_DELETING: SceBtErrorCode = 2150564366;
pub const SCE_BT_ERROR_CONNECT_START_IS_LE: SceBtErrorCode = 2150572546;
pub const SCE_BT_ERROR_CONNECT_START_NO_REG: SceBtErrorCode = 2150564354;
pub const SCE_BT_ERROR_CONNECT_START_NOT_CONNECTABLE: SceBtErrorCode = 2150564355;
pub const SCE_BT_ERROR_CONNECT_START_REG_FULL: SceBtErrorCode = 2150564361;
pub const SCE_BT_ERROR_CONNECT_START_TOO_MANY: SceBtErrorCode = 2150564365;
pub const SCE_BT_ERROR_COPYIN_FAILED: SceBtErrorCode = 2150566665;
pub const SCE_BT_ERROR_COPYOUT_FAILED: SceBtErrorCode = 2150566666;
pub const SCE_BT_ERROR_DISCONNECT_START_NOT_CONNECTED: SceBtErrorCode = 2150564357;
pub const SCE_BT_ERROR_GATT_BUSY: SceBtErrorCode = 2150573059;
pub const SCE_BT_ERROR_GATT_DISCONNECT: SceBtErrorCode = 2150573066;
pub const SCE_BT_ERROR_GATT_ENTER: SceBtErrorCode = 2150573068;
pub const SCE_BT_ERROR_GATT_INVALID_FLAGS: SceBtErrorCode = 2150573061;
pub const SCE_BT_ERROR_GATT_INVALID_HANDLE: SceBtErrorCode = 2150573060;
pub const SCE_BT_ERROR_GATT_INVALID_NO: SceBtErrorCode = 2150573057;
pub const SCE_BT_ERROR_GATT_INVALID_SIZE: SceBtErrorCode = 2150573062;
pub const SCE_BT_ERROR_GATT_NOT_CONNECTED: SceBtErrorCode = 2150573058;
pub const SCE_BT_ERROR_GATT_NOT_LE: SceBtErrorCode = 2150573063;
pub const SCE_BT_ERROR_GATT_NOT_YET: SceBtErrorCode = 2150573065;
pub const SCE_BT_ERROR_GATT_TOO_BIG_BUFFER: SceBtErrorCode = 2150573069;
pub const SCE_BT_ERROR_GATT_TOO_BIG_RECORD: SceBtErrorCode = 2150573064;
pub const SCE_BT_ERROR_GATT_TSLEEP: SceBtErrorCode = 2150573067;
pub const SCE_BT_ERROR_GET_DEBUG_INFO_INVALID_ARGUMENT: SceBtErrorCode = 2150572293;
pub const SCE_BT_ERROR_GET_DEBUG_INFO_INVALID_REQUEST: SceBtErrorCode = 2150572289;
pub const SCE_BT_ERROR_GET_DEBUG_INFO_INVALID_SIZE: SceBtErrorCode = 2150572291;
pub const SCE_BT_ERROR_GET_DEBUG_INFO_INVALID_STATE: SceBtErrorCode = 2150572295;
pub const SCE_BT_ERROR_GET_DEBUG_INFO_NOT_CONNECTED: SceBtErrorCode = 2150572297;
pub const SCE_BT_ERROR_GET_JACK_STATUS_NOT_CONNECTED: SceBtErrorCode = 2150576897;
pub const SCE_BT_ERROR_GET_NAME_NO_DEVICE: SceBtErrorCode = 2150566913;
pub const SCE_BT_ERROR_HCI_TX_OVERFLOW: SceBtErrorCode = 2150566658;
pub const SCE_BT_ERROR_HID_CLOSE_NO_L2C: SceBtErrorCode = 2150566146;
pub const SCE_BT_ERROR_HID_INVALID_BUFFER_ADDRESS: SceBtErrorCode = 2150566151;
pub const SCE_BT_ERROR_HID_INVALID_IDLE: SceBtErrorCode = 2150566153;
pub const SCE_BT_ERROR_HID_INVALID_LENGTH: SceBtErrorCode = 2150566157;
pub const SCE_BT_ERROR_HID_INVALID_PROTOCOL: SceBtErrorCode = 2150566152;
pub const SCE_BT_ERROR_HID_INVALID_REPORT_ID: SceBtErrorCode = 2150566158;
pub const SCE_BT_ERROR_HID_INVALID_REQUEST_TYPE: SceBtErrorCode = 2150566150;
pub const SCE_BT_ERROR_HID_IS_LE: SceBtErrorCode = 2150572548;
pub const SCE_BT_ERROR_HID_NO_CAP: SceBtErrorCode = 2150566149;
pub const SCE_BT_ERROR_HID_NOT_CONNECTED: SceBtErrorCode = 2150566148;
pub const SCE_BT_ERROR_HID_NOT_YET: SceBtErrorCode = 2150566156;
pub const SCE_BT_ERROR_HID_OPEN_NO_L2C: SceBtErrorCode = 2150566145;
pub const SCE_BT_ERROR_HID_OVERWRITE_REQ: SceBtErrorCode = 2150566159;
pub const SCE_BT_ERROR_HID_RECV_INVALID_LENGTH: SceBtErrorCode = 2150567426;
pub const SCE_BT_ERROR_HID_RECV_NOT_CONNECTED: SceBtErrorCode = 2150567425;
pub const SCE_BT_ERROR_HID_SEND_NO_L2C: SceBtErrorCode = 2150566147;
pub const SCE_BT_ERROR_INQUIRY_START_BUSY: SceBtErrorCode = 2150564353;
pub const SCE_BT_ERROR_JEDI_SNIFF_NOT_CONNECTED: SceBtErrorCode = 2150577665;
pub const SCE_BT_ERROR_JEDI_SNIFF_NOT_JEDI: SceBtErrorCode = 2150577666;
pub const SCE_BT_ERROR_JEDI_VOLUME_GAIN_NOT_CONNECTED: SceBtErrorCode = 2150577409;
pub const SCE_BT_ERROR_JUMBO_UNLOCK_NOT_OWNER: SceBtErrorCode = 2150566657;
pub const SCE_BT_ERROR_KPROC_CREATE: SceBtErrorCode = 2150572034;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_BUSY: SceBtErrorCode = 2150574349;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_CONNECTED: SceBtErrorCode = 2150574347;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_FULL: SceBtErrorCode = 2150574348;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_INTERVAL_MAX: SceBtErrorCode = 2150574342;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_INTERVAL_MIN: SceBtErrorCode = 2150574341;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_LATENCY: SceBtErrorCode = 2150574343;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_OWN_ADDRESS_TYPE: SceBtErrorCode = 2150574340;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_PEER_ADDRESS: SceBtErrorCode = 2150574339;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_SCAN_INTERVAL: SceBtErrorCode = 2150574337;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_SCAN_WINDOW: SceBtErrorCode = 2150574338;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_INVALID_TIMEOUT: SceBtErrorCode = 2150574344;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_REG_ERROR: SceBtErrorCode = 2150574346;
pub const SCE_BT_ERROR_LE_CREATE_CONNECTION_REG_FULL: SceBtErrorCode = 2150574345;
pub const SCE_BT_ERROR_LE_GET_ADVERTISING_NOT_FOUND: SceBtErrorCode = 2150574081;
pub const SCE_BT_ERROR_LE_NOT_SUPPORTED: SceBtErrorCode = 2150574849;
pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_DIRECT_ADDRESS: SceBtErrorCode = 2150573575;
pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_DIRECT_ADDRESS_TYPE: SceBtErrorCode = 2150573574;
pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_INTERVAL_MAX: SceBtErrorCode = 2150573572;
pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_INTERVAL_MIN: SceBtErrorCode = 2150573571;
pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_LENGTH: SceBtErrorCode = 2150573570;
pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_OWN_ADDRESS_TYPE: SceBtErrorCode = 2150573573;
pub const SCE_BT_ERROR_LE_SET_ADVERTISING_INVALID_REQ: SceBtErrorCode = 2150573569;
pub const SCE_BT_ERROR_LE_SET_SCAN_INVALID_INTERVAL: SceBtErrorCode = 2150573827;
pub const SCE_BT_ERROR_LE_SET_SCAN_INVALID_LENGTH: SceBtErrorCode = 2150573826;
pub const SCE_BT_ERROR_LE_SET_SCAN_INVALID_OWN_ADDRESS_TYPE: SceBtErrorCode = 2150573829;
pub const SCE_BT_ERROR_LE_SET_SCAN_INVALID_REQ: SceBtErrorCode = 2150573825;
pub const SCE_BT_ERROR_LE_SET_SCAN_INVALID_WINDOW: SceBtErrorCode = 2150573828;
pub const SCE_BT_ERROR_NOTIMP: SceBtErrorCode = 2150572033;
pub const SCE_BT_ERROR_NOT_READY: SceBtErrorCode = 2150576641;
pub const SCE_BT_ERROR_NOT_SUPPORTED_DEVICE: SceBtErrorCode = 2150577155;
pub const SCE_BT_ERROR_PAIRING_OOB_CAN_NOT_DISCONNECT: SceBtErrorCode = 2150576387;
pub const SCE_BT_ERROR_PAIRING_OOB_FULL: SceBtErrorCode = 2150576386;
pub const SCE_BT_ERROR_PAIRING_OOB_INTERNAL_ERROR: SceBtErrorCode = 2150576388;
pub const SCE_BT_ERROR_PAIRING_OOB_TIMEOUT: SceBtErrorCode = 2150576385;
pub const SCE_BT_ERROR_PIN_INVALID_LENGTH: SceBtErrorCode = 2150564358;
pub const SCE_BT_ERROR_PIN_IS_LE: SceBtErrorCode = 2150572545;
pub const SCE_BT_ERROR_REG_CANNOT_LOAD: SceBtErrorCode = 2150564106;
pub const SCE_BT_ERROR_REG_CANNOT_OPEN: SceBtErrorCode = 2150564107;
pub const SCE_BT_ERROR_REG_CANNOT_READ: SceBtErrorCode = 2150564108;
pub const SCE_BT_ERROR_REG_CANNOT_WRITE: SceBtErrorCode = 2150564109;
pub const SCE_BT_ERROR_REG_DELETE_CONNECTING: SceBtErrorCode = 2150564105;
pub const SCE_BT_ERROR_REG_DELETE_NO_ENTRY: SceBtErrorCode = 2150564098;
pub const SCE_BT_ERROR_REG_GET_HID_DESC_NO_REG: SceBtErrorCode = 2150564103;
pub const SCE_BT_ERROR_REG_GET_HID_DESC_TOO_SHORT: SceBtErrorCode = 2150564104;
pub const SCE_BT_ERROR_REG_NOT_READY: SceBtErrorCode = 2150564097;
pub const SCE_BT_ERROR_REG_SET_HID_DESC_BAD_ARG: SceBtErrorCode = 2150564101;
pub const SCE_BT_ERROR_REG_SET_HID_DESC_NO_REG: SceBtErrorCode = 2150564102;
pub const SCE_BT_ERROR_REG_UPDATE_CANNOT_SAVE: SceBtErrorCode = 2150564099;
pub const SCE_BT_ERROR_SDIO_DISABLE_FUNCTION: SceBtErrorCode = 2150572041;
pub const SCE_BT_ERROR_SDIO_ENABLE_FUNCTION: SceBtErrorCode = 2150572040;
pub const SCE_BT_ERROR_SDIO_GET_FUNCTION: SceBtErrorCode = 2150572035;
pub const SCE_BT_ERROR_SDIO_LOCK: SceBtErrorCode = 2150572038;
pub const SCE_BT_ERROR_SDIO_READ_DIR: SceBtErrorCode = 2150572044;
pub const SCE_BT_ERROR_SDIO_READ_FIX: SceBtErrorCode = 2150572046;
pub const SCE_BT_ERROR_SDIO_REGISTER_INTR_HANDLER: SceBtErrorCode = 2150572036;
pub const SCE_BT_ERROR_SDIO_SET_BLOCK_LEN: SceBtErrorCode = 2150572042;
pub const SCE_BT_ERROR_SDIO_SET_BUS_SPEED: SceBtErrorCode = 2150572043;
pub const SCE_BT_ERROR_SDIO_UNLOCK: SceBtErrorCode = 2150572039;
pub const SCE_BT_ERROR_SDIO_UNREGISTER_INTR_HANDLER: SceBtErrorCode = 2150572037;
pub const SCE_BT_ERROR_SDIO_WRITE_DIR: SceBtErrorCode = 2150572045;
pub const SCE_BT_ERROR_SDIO_WRITE_FIX: SceBtErrorCode = 2150572047;
pub const SCE_BT_ERROR_SDP_OPEN_NO_L2C: SceBtErrorCode = 2150564360;
pub const SCE_BT_ERROR_SET_DEBUG_INFO_INVALID_ARGUMENT: SceBtErrorCode = 2150572294;
pub const SCE_BT_ERROR_SET_DEBUG_INFO_INVALID_REQUEST: SceBtErrorCode = 2150572290;
pub const SCE_BT_ERROR_SET_DEBUG_INFO_INVALID_SIZE: SceBtErrorCode = 2150572292;
pub const SCE_BT_ERROR_SET_DEBUG_INFO_INVALID_STATE: SceBtErrorCode = 2150572296;
pub const SCE_BT_ERROR_SET_DEBUG_INFO_NOT_CONNECTED: SceBtErrorCode = 2150572298;
pub const SCE_BT_ERROR_SM_INVALID_CONFIRM_REPLY: SceBtErrorCode = 2150573317;
pub const SCE_BT_ERROR_SM_INVALID_KEY_LENGTH: SceBtErrorCode = 2150573313;
pub const SCE_BT_ERROR_SM_NO_REQ: SceBtErrorCode = 2150573315;
pub const SCE_BT_ERROR_SM_NOT_DIGIT: SceBtErrorCode = 2150573314;
pub const SCE_BT_ERROR_SM_NOT_NEEDED_PIN: SceBtErrorCode = 2150573316;
pub const SCE_BT_ERROR_TIMER_CANCEL_BAD_ID: SceBtErrorCode = 2150566661;
pub const SCE_BT_ERROR_TIMER_CANCEL_NOT_INITIALIZED: SceBtErrorCode = 2150566662;
pub const SCE_BT_ERROR_TIMER_SET_NO_SPACE: SceBtErrorCode = 2150566664;
pub const SCE_BT_ERROR_TIMER_SET_NOT_INITIALIZED: SceBtErrorCode = 2150566663;
pub const SCE_BT_ERROR_TOO_MANY_CONNECTION: SceBtErrorCode = 2150577153;
pub const SCE_BT_ERROR_TOO_MANY_HID: SceBtErrorCode = 2150577154;
pub const SCE_BT_ERROR_TSLEEP: SceBtErrorCode = 2150572048;
pub const SCE_BT_ERROR_USER_CONFIRM_NOT_CONNECTED: SceBtErrorCode = 2150564359;
pub const SCE_CAMERA_ANTIFLICKER_50HZ: SceCameraAntiFlicker = 2;
pub const SCE_CAMERA_ANTIFLICKER_60HZ: SceCameraAntiFlicker = 3;
pub const SCE_CAMERA_ANTIFLICKER_AUTO: SceCameraAntiFlicker = 1;
pub const SCE_CAMERA_BACKLIGHT_OFF: SceCameraBacklight = 0;
pub const SCE_CAMERA_BACKLIGHT_ON: SceCameraBacklight = 1;
pub const SCE_CAMERA_DEVICE_BACK: SceCameraDevice = 1;
pub const SCE_CAMERA_DEVICE_FRONT: SceCameraDevice = 0;
pub const SCE_CAMERA_EFFECT_BLACKWHITE: SceCameraEffect = 2;
pub const SCE_CAMERA_EFFECT_BLUE: SceCameraEffect = 4;
pub const SCE_CAMERA_EFFECT_GREEN: SceCameraEffect = 6;
pub const SCE_CAMERA_EFFECT_NEGATIVE: SceCameraEffect = 1;
pub const SCE_CAMERA_EFFECT_NORMAL: SceCameraEffect = 0;
pub const SCE_CAMERA_EFFECT_RED: SceCameraEffect = 5;
pub const SCE_CAMERA_EFFECT_SEPIA: SceCameraEffect = 3;
pub const SCE_CAMERA_ERROR_ALREADY_INIT: SceCameraErrorCode = 2150498305;
pub const SCE_CAMERA_ERROR_ALREADY_OPEN: SceCameraErrorCode = 2150498307;
pub const SCE_CAMERA_ERROR_ALREADY_READ: SceCameraErrorCode = 2150498319;
pub const SCE_CAMERA_ERROR_ALREADY_START: SceCameraErrorCode = 2150498309;
pub const SCE_CAMERA_ERROR_ATTRIBUTE_UNKNOWN: SceCameraErrorCode = 2150498316;
pub const SCE_CAMERA_ERROR_BAD_FRAMERATE: SceCameraErrorCode = 2150498313;
pub const SCE_CAMERA_ERROR_DATA_RANGE_UNKNOWN: SceCameraErrorCode = 2150498321;
pub const SCE_CAMERA_ERROR_EXCLUSIVE: SceCameraErrorCode = 2150498315;
pub const SCE_CAMERA_ERROR_FATAL: SceCameraErrorCode = 2150498559;
pub const SCE_CAMERA_ERROR_FORMAT_UNKNOWN: SceCameraErrorCode = 2150498311;
pub const SCE_CAMERA_ERROR_MAX_PROCESS: SceCameraErrorCode = 2150498317;
pub const SCE_CAMERA_ERROR_NOT_ACTIVE: SceCameraErrorCode = 2150498318;
pub const SCE_CAMERA_ERROR_NOT_INIT: SceCameraErrorCode = 2150498306;
pub const SCE_CAMERA_ERROR_NOT_MOUNTED: SceCameraErrorCode = 2150498320;
pub const SCE_CAMERA_ERROR_NOT_OPEN: SceCameraErrorCode = 2150498308;
pub const SCE_CAMERA_ERROR_NOT_START: SceCameraErrorCode = 2150498310;
pub const SCE_CAMERA_ERROR_OTHER_ALREADY_START: SceCameraErrorCode = 2150498322;
pub const SCE_CAMERA_ERROR_PARAM: SceCameraErrorCode = 2150498304;
pub const SCE_CAMERA_ERROR_RESOLUTION_UNKNOWN: SceCameraErrorCode = 2150498312;
pub const SCE_CAMERA_ERROR_TIMEOUT: SceCameraErrorCode = 2150498314;
pub const SCE_CAMERA_EV_NEGATIVE_10: SceCameraExposureCompensation = -10;
pub const SCE_CAMERA_EV_NEGATIVE_13: SceCameraExposureCompensation = -13;
pub const SCE_CAMERA_EV_NEGATIVE_15: SceCameraExposureCompensation = -15;
pub const SCE_CAMERA_EV_NEGATIVE_17: SceCameraExposureCompensation = -17;
pub const SCE_CAMERA_EV_NEGATIVE_20: SceCameraExposureCompensation = -20;
pub const SCE_CAMERA_EV_NEGATIVE_3: SceCameraExposureCompensation = -3;
pub const SCE_CAMERA_EV_NEGATIVE_5: SceCameraExposureCompensation = -5;
pub const SCE_CAMERA_EV_NEGATIVE_7: SceCameraExposureCompensation = -7;
pub const SCE_CAMERA_EV_POSITIVE_0: SceCameraExposureCompensation = 0;
pub const SCE_CAMERA_EV_POSITIVE_10: SceCameraExposureCompensation = 10;
pub const SCE_CAMERA_EV_POSITIVE_13: SceCameraExposureCompensation = 13;
pub const SCE_CAMERA_EV_POSITIVE_15: SceCameraExposureCompensation = 15;
pub const SCE_CAMERA_EV_POSITIVE_17: SceCameraExposureCompensation = 17;
pub const SCE_CAMERA_EV_POSITIVE_20: SceCameraExposureCompensation = 20;
pub const SCE_CAMERA_EV_POSITIVE_3: SceCameraExposureCompensation = 3;
pub const SCE_CAMERA_EV_POSITIVE_5: SceCameraExposureCompensation = 5;
pub const SCE_CAMERA_EV_POSITIVE_7: SceCameraExposureCompensation = 7;
pub const SCE_CAMERA_FORMAT_ABGR: SceCameraFormat = 5;
pub const SCE_CAMERA_FORMAT_ARGB: SceCameraFormat = 4;
pub const SCE_CAMERA_FORMAT_INVALID: SceCameraFormat = 0;
pub const SCE_CAMERA_FORMAT_RAW8: SceCameraFormat = 6;
pub const SCE_CAMERA_FORMAT_YUV420_PLANE: SceCameraFormat = 3;
pub const SCE_CAMERA_FORMAT_YUV422_PACKED: SceCameraFormat = 2;
pub const SCE_CAMERA_FORMAT_YUV422_PLANE: SceCameraFormat = 1;
pub const SCE_CAMERA_FRAMERATE_10_FPS: SceCameraFrameRate = 10;
pub const SCE_CAMERA_FRAMERATE_120_FPS: SceCameraFrameRate = 120;
pub const SCE_CAMERA_FRAMERATE_15_FPS: SceCameraFrameRate = 15;
pub const SCE_CAMERA_FRAMERATE_20_FPS: SceCameraFrameRate = 20;
pub const SCE_CAMERA_FRAMERATE_30_FPS: SceCameraFrameRate = 30;
pub const SCE_CAMERA_FRAMERATE_3_FPS: SceCameraFrameRate = 3;
pub const SCE_CAMERA_FRAMERATE_5_FPS: SceCameraFrameRate = 5;
pub const SCE_CAMERA_FRAMERATE_60_FPS: SceCameraFrameRate = 60;
pub const SCE_CAMERA_FRAMERATE_7_FPS: SceCameraFrameRate = 7;
pub const SCE_CAMERA_GAIN_1: SceCameraGain = 1;
pub const SCE_CAMERA_GAIN_10: SceCameraGain = 10;
pub const SCE_CAMERA_GAIN_11: SceCameraGain = 11;
pub const SCE_CAMERA_GAIN_12: SceCameraGain = 12;
pub const SCE_CAMERA_GAIN_13: SceCameraGain = 13;
pub const SCE_CAMERA_GAIN_14: SceCameraGain = 14;
pub const SCE_CAMERA_GAIN_15: SceCameraGain = 15;
pub const SCE_CAMERA_GAIN_16: SceCameraGain = 16;
pub const SCE_CAMERA_GAIN_2: SceCameraGain = 2;
pub const SCE_CAMERA_GAIN_3: SceCameraGain = 3;
pub const SCE_CAMERA_GAIN_4: SceCameraGain = 4;
pub const SCE_CAMERA_GAIN_5: SceCameraGain = 5;
pub const SCE_CAMERA_GAIN_6: SceCameraGain = 6;
pub const SCE_CAMERA_GAIN_7: SceCameraGain = 7;
pub const SCE_CAMERA_GAIN_8: SceCameraGain = 8;
pub const SCE_CAMERA_GAIN_9: SceCameraGain = 9;
pub const SCE_CAMERA_GAIN_AUTO: SceCameraGain = 0;
pub const SCE_CAMERA_ISO_100: SceCameraISO = 100;
pub const SCE_CAMERA_ISO_200: SceCameraISO = 200;
pub const SCE_CAMERA_ISO_400: SceCameraISO = 400;
pub const SCE_CAMERA_ISO_AUTO: SceCameraISO = 1;
pub const SCE_CAMERA_NIGHTMODE_LESS10: SceCameraNightmode = 1;
pub const SCE_CAMERA_NIGHTMODE_LESS100: SceCameraNightmode = 2;
pub const SCE_CAMERA_NIGHTMODE_OFF: SceCameraNightmode = 0;
pub const SCE_CAMERA_NIGHTMODE_OVER100: SceCameraNightmode = 3;
pub const SCE_CAMERA_PRIORITY_EXCLUSIVE: SceCameraPriority = 1;
pub const SCE_CAMERA_PRIORITY_SHARE: SceCameraPriority = 0;
pub const SCE_CAMERA_RESOLUTION_0_0: SceCameraResolution = 0;
pub const SCE_CAMERA_RESOLUTION_160_120: SceCameraResolution = 3;
pub const SCE_CAMERA_RESOLUTION_176_144: SceCameraResolution = 5;
pub const SCE_CAMERA_RESOLUTION_320_240: SceCameraResolution = 2;
pub const SCE_CAMERA_RESOLUTION_352_288: SceCameraResolution = 4;
pub const SCE_CAMERA_RESOLUTION_480_272: SceCameraResolution = 6;
pub const SCE_CAMERA_RESOLUTION_640_360: SceCameraResolution = 8;
pub const SCE_CAMERA_RESOLUTION_640_480: SceCameraResolution = 1;
pub const SCE_CAMERA_REVERSE_FLIP: SceCameraReverse = 2;
pub const SCE_CAMERA_REVERSE_MIRROR: SceCameraReverse = 1;
pub const SCE_CAMERA_REVERSE_MIRROR_FLIP: SceCameraReverse = 3;
pub const SCE_CAMERA_REVERSE_OFF: SceCameraReverse = 0;
pub const SCE_CAMERA_SATURATION_0: SceCameraSaturation = 0;
pub const SCE_CAMERA_SATURATION_10: SceCameraSaturation = 10;
pub const SCE_CAMERA_SATURATION_20: SceCameraSaturation = 20;
pub const SCE_CAMERA_SATURATION_30: SceCameraSaturation = 30;
pub const SCE_CAMERA_SATURATION_40: SceCameraSaturation = 40;
pub const SCE_CAMERA_SATURATION_5: SceCameraSaturation = 5;
pub const SCE_CAMERA_SHARPNESS_100: SceCameraSharpness = 1;
pub const SCE_CAMERA_SHARPNESS_200: SceCameraSharpness = 2;
pub const SCE_CAMERA_SHARPNESS_300: SceCameraSharpness = 3;
pub const SCE_CAMERA_SHARPNESS_400: SceCameraSharpness = 4;
pub const SCE_CAMERA_WB_AUTO: SceCameraWhiteBalance = 0;
pub const SCE_CAMERA_WB_CWF: SceCameraWhiteBalance = 2;
pub const SCE_CAMERA_WB_DAY: SceCameraWhiteBalance = 1;
pub const SCE_CAMERA_WB_SLSA: SceCameraWhiteBalance = 4;
pub const SCE_COMMON_DIALOG_ERROR_BUSY: SceCommonDialogErrorCode = 2147615745;
pub const SCE_COMMON_DIALOG_ERROR_GXM_IS_UNINITIALIZED: SceCommonDialogErrorCode = 2147615798;
pub const SCE_COMMON_DIALOG_ERROR_ILLEGAL_CALLER_THREAD: SceCommonDialogErrorCode = 2147615750;
pub const SCE_COMMON_DIALOG_ERROR_IME_IN_USE: SceCommonDialogErrorCode = 2147615792;
pub const SCE_COMMON_DIALOG_ERROR_INVALID_ARGUMENT: SceCommonDialogErrorCode = 2147615747;
pub const SCE_COMMON_DIALOG_ERROR_INVALID_BG_COLOR: SceCommonDialogErrorCode = 2147615796;
pub const SCE_COMMON_DIALOG_ERROR_INVALID_COLOR_FORMAT: SceCommonDialogErrorCode = 2147615776;
pub const SCE_COMMON_DIALOG_ERROR_INVALID_DIMMER_COLOR: SceCommonDialogErrorCode = 2147615797;
pub const SCE_COMMON_DIALOG_ERROR_INVALID_ENTER_BUTTON_ASSIGN: SceCommonDialogErrorCode =
    2147615794;
pub const SCE_COMMON_DIALOG_ERROR_INVALID_INFOBAR_PARAM: SceCommonDialogErrorCode = 2147615795;
pub const SCE_COMMON_DIALOG_ERROR_INVALID_LANGUAGE: SceCommonDialogErrorCode = 2147615793;
pub const SCE_COMMON_DIALOG_ERROR_INVALID_SURFACE_RESOLUTION: SceCommonDialogErrorCode = 2147615777;
pub const SCE_COMMON_DIALOG_ERROR_INVALID_SURFACE_STRIDE: SceCommonDialogErrorCode = 2147615778;
pub const SCE_COMMON_DIALOG_ERROR_INVALID_SURFACE_TYPE: SceCommonDialogErrorCode = 2147615779;
pub const SCE_COMMON_DIALOG_ERROR_NOT_AVAILABLE: SceCommonDialogErrorCode = 2147615752;
pub const SCE_COMMON_DIALOG_ERROR_NOT_CONFIGURED: SceCommonDialogErrorCode = 2147615751;
pub const SCE_COMMON_DIALOG_ERROR_NOT_FINISHED: SceCommonDialogErrorCode = 2147615760;
pub const SCE_COMMON_DIALOG_ERROR_NOT_IN_USE: SceCommonDialogErrorCode = 2147615761;
pub const SCE_COMMON_DIALOG_ERROR_NOT_RUNNING: SceCommonDialogErrorCode = 2147615748;
pub const SCE_COMMON_DIALOG_ERROR_NOT_SUPPORTED: SceCommonDialogErrorCode = 2147615749;
pub const SCE_COMMON_DIALOG_ERROR_NULL: SceCommonDialogErrorCode = 2147615746;
pub const SCE_COMMON_DIALOG_ERROR_UNEXPECTED_FATAL: SceCommonDialogErrorCode = 2147615871;
pub const SCE_COMMON_DIALOG_ERROR_WITHIN_SCENE: SceCommonDialogErrorCode = 2147615780;
pub const SCE_COMMON_DIALOG_MAGIC_NUMBER: u32 = 3234963721;
pub const SCE_COMMON_DIALOG_RESULT_ABORTED: SceCommonDialogResult = 2;
pub const SCE_COMMON_DIALOG_RESULT_OK: SceCommonDialogResult = 0;
pub const SCE_COMMON_DIALOG_RESULT_USER_CANCELED: SceCommonDialogResult = 1;
pub const SCE_COMMON_DIALOG_STATUS_FINISHED: SceCommonDialogStatus = 2;
pub const SCE_COMMON_DIALOG_STATUS_NONE: SceCommonDialogStatus = 0;
pub const SCE_COMMON_DIALOG_STATUS_RUNNING: SceCommonDialogStatus = 1;
pub const SCE_COMPAT_CACHE_INVALIDATE: SceCompatCacheMode = 1;
pub const SCE_COMPAT_CACHE_NONE: SceCompatCacheMode = 0;
pub const SCE_COMPAT_CACHE_WRITEBACK: SceCompatCacheMode = 2;
pub const SCE_COMPAT_PERIPHERAL_HPREMOTE_IS_HEADPHONE_EXIST: SceCompatPeripheralMode = 4;
pub const SCE_COMPAT_PERIPHERAL_POWER_GET_BATTERY_LIFETIME: SceCompatPeripheralMode = 2;
pub const SCE_COMPAT_PERIPHERAL_POWER_GET_BATTERY_PERCENT: SceCompatPeripheralMode = 3;
pub const SCE_COMPAT_PERIPHERAL_POWER_IS_SUSPEND_REQUIRED: SceCompatPeripheralMode = 1;
pub const SCE_CST_AT: u32 = 16;
pub const SCE_CST_CT: u32 = 8;
pub const SCE_CST_MODE: u32 = 1;
pub const SCE_CST_MT: u32 = 32;
pub const SCE_CST_SIZE: u32 = 4;
pub const SCE_CTRL_CIRCLE: SceCtrlButtons = 8192;
pub const SCE_CTRL_CROSS: SceCtrlButtons = 16384;
pub const SCE_CTRL_DOWN: SceCtrlButtons = 64;
pub const SCE_CTRL_ERROR_FATAL: SceCtrlErrorCode = 2150891775;
pub const SCE_CTRL_ERROR_INVALID_ARG: SceCtrlErrorCode = 2150891521;
pub const SCE_CTRL_ERROR_INVALID_MODE: SceCtrlErrorCode = 2150891554;
pub const SCE_CTRL_ERROR_NO_DEVICE: SceCtrlErrorCode = 2150891552;
pub const SCE_CTRL_ERROR_NOT_SUPPORTED: SceCtrlErrorCode = 2150891553;
pub const SCE_CTRL_ERROR_PRIV_REQUIRED: SceCtrlErrorCode = 2150891522;
pub const SCE_CTRL_HEADPHONE: SceCtrlButtons = 524288;
pub const SCE_CTRL_INTERCEPTED: SceCtrlButtons = 65536;
pub const SCE_CTRL_L1: SceCtrlButtons = 1024;
pub const SCE_CTRL_L2: SceCtrlButtons = 256;
pub const SCE_CTRL_L3: SceCtrlButtons = 2;
pub const SCE_CTRL_LEFT: SceCtrlButtons = 128;
pub const SCE_CTRL_LTRIGGER: SceCtrlButtons = 256;
pub const SCE_CTRL_MODE_ANALOG: SceCtrlPadInputMode = 1;
pub const SCE_CTRL_MODE_ANALOG_WIDE: SceCtrlPadInputMode = 2;
pub const SCE_CTRL_MODE_DIGITAL: SceCtrlPadInputMode = 0;
pub const SCE_CTRL_POWER: SceCtrlButtons = 1073741824;
pub const SCE_CTRL_PSBUTTON: SceCtrlButtons = 65536;
pub const SCE_CTRL_R1: SceCtrlButtons = 2048;
pub const SCE_CTRL_R2: SceCtrlButtons = 512;
pub const SCE_CTRL_R3: SceCtrlButtons = 4;
pub const SCE_CTRL_RIGHT: SceCtrlButtons = 32;
pub const SCE_CTRL_RTRIGGER: SceCtrlButtons = 512;
pub const SCE_CTRL_SELECT: SceCtrlButtons = 1;
pub const SCE_CTRL_SQUARE: SceCtrlButtons = 32768;
pub const SCE_CTRL_START: SceCtrlButtons = 8;
pub const SCE_CTRL_TRIANGLE: SceCtrlButtons = 4096;
pub const SCE_CTRL_TYPE_DS3: SceCtrlExternalInputMode = 4;
pub const SCE_CTRL_TYPE_DS4: SceCtrlExternalInputMode = 8;
pub const SCE_CTRL_TYPE_PHY: SceCtrlExternalInputMode = 1;
pub const SCE_CTRL_TYPE_UNPAIRED: SceCtrlExternalInputMode = 0;
pub const SCE_CTRL_TYPE_VIRT: SceCtrlExternalInputMode = 2;
pub const SCE_CTRL_UP: SceCtrlButtons = 16;
pub const SCE_CTRL_VOLDOWN: SceCtrlButtons = 2097152;
pub const SCE_CTRL_VOLUP: SceCtrlButtons = 1048576;
pub const SCE_CTYPE_CONTROL: SceCTypeFlag = 8;
pub const SCE_CTYPE_HEX_CASE: SceCTypeFlag = 64;
pub const SCE_CTYPE_INVISIBLE: SceCTypeFlag = 32;
pub const SCE_CTYPE_LOWERCASE: SceCTypeFlag = 2;
pub const SCE_CTYPE_NONE: SceCTypeFlag = 0;
pub const SCE_CTYPE_NUMBER: SceCTypeFlag = 4;
pub const SCE_CTYPE_SYMBOL: SceCTypeFlag = 16;
pub const SCE_CTYPE_UPPERCASE: SceCTypeFlag = 1;
pub const SCE_DBG_BREAK_ON_ASSERT: u32 = 1;
pub const SCE_DBG_LOG_LEVEL_DEBUG: SceDbgLogLevel = 1;
pub const SCE_DBG_LOG_LEVEL_ERROR: SceDbgLogLevel = 4;
pub const SCE_DBG_LOG_LEVEL_INFO: SceDbgLogLevel = 2;
pub const SCE_DBG_LOG_LEVEL_TRACE: SceDbgLogLevel = 0;
pub const SCE_DBG_LOG_LEVEL_WARNING: SceDbgLogLevel = 3;
pub const SCE_DBG_NUM_LOG_LEVELS: SceDbgLogLevel = 5;
pub const SCE_DEV_TYPE_ALIAS: SceIoDevType = 32;
pub const SCE_DEV_TYPE_BLOCK: SceIoDevType = 4;
pub const SCE_DEV_TYPE_CHAR: SceIoDevType = 1;
pub const SCE_DEV_TYPE_FS: SceIoDevType = 16;
pub const SCE_DEV_TYPE_MOUNTPT: SceIoDevType = 64;
pub const SCE_DEV_TYPE_NULL: SceIoDevType = 0;
pub const SCE_DISPLAY_ERROR_INVALID_ADDR: SceDisplayErrorCode = 2150170626;
pub const SCE_DISPLAY_ERROR_INVALID_HEAD: SceDisplayErrorCode = 2150170624;
pub const SCE_DISPLAY_ERROR_INVALID_PITCH: SceDisplayErrorCode = 2150170628;
pub const SCE_DISPLAY_ERROR_INVALID_PIXELFORMAT: SceDisplayErrorCode = 2150170627;
pub const SCE_DISPLAY_ERROR_INVALID_RESOLUTION: SceDisplayErrorCode = 2150170629;
pub const SCE_DISPLAY_ERROR_INVALID_UPDATETIMING: SceDisplayErrorCode = 2150170630;
pub const SCE_DISPLAY_ERROR_INVALID_VALUE: SceDisplayErrorCode = 2150170625;
pub const SCE_DISPLAY_ERROR_NO_FRAME_BUFFER: SceDisplayErrorCode = 2150170631;
pub const SCE_DISPLAY_ERROR_NO_OUTPUT_SIGNAL: SceDisplayErrorCode = 2150170633;
pub const SCE_DISPLAY_ERROR_NO_PIXEL_DATA: SceDisplayErrorCode = 2150170632;
pub const SCE_DISPLAY_ERROR_OK: SceDisplayErrorCode = 0;
pub const SCE_DISPLAY_PIXELFORMAT_A2B10G10R10: SceDisplayPixelFormat = 1619001344;
pub const SCE_DISPLAY_PIXELFORMAT_A8B8G8R8: SceDisplayPixelFormat = 0;
pub const __SCE_DISPLAY_SETBUF: SceDisplaySetBufSync = 4294967295;
pub const SCE_DISPLAY_SETBUF_IMMEDIATE: SceDisplaySetBufSync = 0;
pub const SCE_DISPLAY_SETBUF_NEXTFRAME: SceDisplaySetBufSync = 1;
pub const SCE_DSI_ERROR_HEAD_NOT_ENABLED: SceDsiErrorCode = 2151613955;
pub const SCE_DSI_ERROR_INVALID_HEAD: SceDsiErrorCode = 2151613952;
pub const SCE_DSI_ERROR_INVALID_PARAM: SceDsiErrorCode = 2151613954;
pub const SCE_DSI_ERROR_INVALID_STATE: SceDsiErrorCode = 2151613953;
pub const SCE_DSI_HEAD_HDMI: SceDsiHead = 1;
pub const SCE_DSI_HEAD_OLED_LCD: SceDsiHead = 0;
pub const SCE_ERROR_NET_ADHOC_ALREADY_CREATED: ScePspnetAdhocErrorCode = 2151745306;
pub const SCE_ERROR_NET_ADHOC_ALREADY_INITIALIZED: ScePspnetAdhocErrorCode = 2151745299;
pub const SCE_ERROR_NET_ADHOC_BUSY: ScePspnetAdhocErrorCode = 2151745300;
pub const SCE_ERROR_NET_ADHOC_CONNECTION_REFUSED: ScePspnetAdhocErrorCode = 2151745304;
pub const SCE_ERROR_NET_ADHOCCTL_ALREADY_INITIALIZED: ScePspnetAdhocctlErrorCode = 2151746311;
pub const SCE_ERROR_NET_ADHOCCTL_INVALID_ARG: ScePspnetAdhocctlErrorCode = 2151746308;
pub const SCE_ERROR_NET_ADHOCCTL_NOT_INITIALIZED: ScePspnetAdhocctlErrorCode = 2151746312;
pub const SCE_ERROR_NET_ADHOC_DISCONNECTED: ScePspnetAdhocErrorCode = 2151745292;
pub const SCE_ERROR_NET_ADHOC_EXCEPTION_EVENT: ScePspnetAdhocErrorCode = 2151745303;
pub const SCE_ERROR_NET_ADHOC_INVALID_ADDR: ScePspnetAdhocErrorCode = 2151745282;
pub const SCE_ERROR_NET_ADHOC_INVALID_ALIGNMENT: ScePspnetAdhocErrorCode = 2151745309;
pub const SCE_ERROR_NET_ADHOC_INVALID_ARG: ScePspnetAdhocErrorCode = 2151745297;
pub const SCE_ERROR_NET_ADHOC_INVALID_BUFLEN: ScePspnetAdhocErrorCode = 2151745284;
pub const SCE_ERROR_NET_ADHOC_INVALID_DATALEN: ScePspnetAdhocErrorCode = 2151745285;
pub const SCE_ERROR_NET_ADHOC_INVALID_PORT: ScePspnetAdhocErrorCode = 2151745283;
pub const SCE_ERROR_NET_ADHOC_INVALID_SOCKET_ID: ScePspnetAdhocErrorCode = 2151745281;
pub const SCE_ERROR_NET_ADHOC_NO_ENTRY: ScePspnetAdhocErrorCode = 2151745302;
pub const SCE_ERROR_NET_ADHOC_NOT_CONNECTED: ScePspnetAdhocErrorCode = 2151745291;
pub const SCE_ERROR_NET_ADHOC_NOT_CREATED: ScePspnetAdhocErrorCode = 2151745308;
pub const SCE_ERROR_NET_ADHOC_NOT_ENOUGH_SPACE: ScePspnetAdhocErrorCode = 2151745286;
pub const SCE_ERROR_NET_ADHOC_NOT_IN_GAMEMODE: ScePspnetAdhocErrorCode = 2151745307;
pub const SCE_ERROR_NET_ADHOC_NOT_INITIALIZED: ScePspnetAdhocErrorCode = 2151745298;
pub const SCE_ERROR_NET_ADHOC_NOT_LISTENED: ScePspnetAdhocErrorCode = 2151745294;
pub const SCE_ERROR_NET_ADHOC_NOT_OPENED: ScePspnetAdhocErrorCode = 2151745293;
pub const SCE_ERROR_NET_ADHOC_PORT_IN_USE: ScePspnetAdhocErrorCode = 2151745290;
pub const SCE_ERROR_NET_ADHOC_PORT_NOT_AVAIL: ScePspnetAdhocErrorCode = 2151745296;
pub const SCE_ERROR_NET_ADHOC_SOCKET_ALERTED: ScePspnetAdhocErrorCode = 2151745288;
pub const SCE_ERROR_NET_ADHOC_SOCKET_DELETED: ScePspnetAdhocErrorCode = 2151745287;
pub const SCE_ERROR_NET_ADHOC_SOCKET_ID_NOT_AVAIL: ScePspnetAdhocErrorCode = 2151745295;
pub const SCE_ERROR_NET_ADHOC_THREAD_ABORTED: ScePspnetAdhocErrorCode = 2151745305;
pub const SCE_ERROR_NET_ADHOC_TIMEOUT: ScePspnetAdhocErrorCode = 2151745301;
pub const SCE_ERROR_NET_ADHOC_WOULD_BLOCK: ScePspnetAdhocErrorCode = 2151745289;
pub const SCE_EVENT_OPENABLE: SceEventFlagAttributes = 128;
pub const SCE_EVENT_THREAD_FIFO: SceEventFlagAttributes = 0;
pub const SCE_EVENT_THREAD_PRIO: SceEventFlagAttributes = 8192;
pub const SCE_EVENT_WAITAND: SceEventFlagWaitTypes = 0;
pub const SCE_EVENT_WAITCLEAR: SceEventFlagWaitTypes = 2;
pub const SCE_EVENT_WAITCLEAR_PAT: SceEventFlagWaitTypes = 4;
pub const SCE_EVENT_WAITMULTIPLE: SceEventFlagAttributes = 4096;
pub const SCE_EVENT_WAITOR: SceEventFlagWaitTypes = 1;
pub const SCE_EVENT_WAITSINGLE: SceEventFlagAttributes = 0;
pub const SCE_EXCP_DABT: SceExcpKind = 4;
pub const SCE_EXCP_FIQ: SceExcpKind = 7;
pub const SCE_EXCP_IRQ: SceExcpKind = 6;
pub const SCE_EXCPMGR_EXCEPTION_HANDLED: SceExcpHandlingCode = 0;
pub const SCE_EXCPMGR_EXCEPTION_HANDLING_CODE_2: SceExcpHandlingCode = 2;
pub const SCE_EXCPMGR_EXCEPTION_HANDLING_CODE_4: SceExcpHandlingCode = 4;
pub const SCE_EXCPMGR_EXCEPTION_NOT_HANDLED: SceExcpHandlingCode = 1;
pub const SCE_EXCPMGR_EXCEPTION_NOT_HANDLED_FATAL: SceExcpHandlingCode = 3;
pub const SCE_EXCP_PABT: SceExcpKind = 3;
pub const SCE_EXCP_RESET: SceExcpKind = 0;
pub const SCE_EXCP_SVC: SceExcpKind = 2;
pub const SCE_EXCP_UNDEF_INSTRUCTION: SceExcpKind = 1;
pub const SCE_EXCP_UNUSED: SceExcpKind = 5;
pub const SCE_FALSE: _bindgen_ty_1 = 0;
pub const SCE_FIBER_ERROR_AGAIN: SceFiberErrorCode = 2153316360;
pub const SCE_FIBER_ERROR_ALIGNMENT: SceFiberErrorCode = 2153316354;
pub const SCE_FIBER_ERROR_BUSY: SceFiberErrorCode = 2153316359;
pub const SCE_FIBER_ERROR_FATAL: SceFiberErrorCode = 2153316361;
pub const SCE_FIBER_ERROR_INVALID: SceFiberErrorCode = 2153316356;
pub const SCE_FIBER_ERROR_NULL: SceFiberErrorCode = 2153316353;
pub const SCE_FIBER_ERROR_PERMISSION: SceFiberErrorCode = 2153316357;
pub const SCE_FIBER_ERROR_RANGE: SceFiberErrorCode = 2153316355;
pub const SCE_FIBER_ERROR_STATE: SceFiberErrorCode = 2153316358;
pub const SCE_FIOS2_OVERLAY_PATH_MAX_LENGTH: u32 = 291;
pub const SCE_FIOS2_OVERLAY_PATH_SIZE: u32 = 292;
pub const SCE_FIOS_OVERLAY_TYPE_NEWER: SceFiosOverlayType = 2;
pub const SCE_FIOS_OVERLAY_TYPE_OPAQUE: SceFiosOverlayType = 0;
pub const SCE_FIOS_OVERLAY_TYPE_TRANSLUCENT: SceFiosOverlayType = 1;
pub const SCE_FIOS_OVERLAY_TYPE_WRITABLE: SceFiosOverlayType = 3;
pub const SCE_FONT_ERROR_HANDLER_OPEN_FAILED: SceFontErrorCode = 2152071173;
pub const SCE_FONT_ERROR_INVALID_FONT_DATA: SceFontErrorCode = 2152071178;
pub const SCE_FONT_ERROR_INVALID_LIBID: SceFontErrorCode = 2152071170;
pub const SCE_FONT_ERROR_INVALID_PARAMETER: SceFontErrorCode = 2152071171;
pub const SCE_FONT_ERROR_OUT_OF_MEMORY: SceFontErrorCode = 2152071169;
pub const SCE_FONT_ERROR_TOO_MANY_OPEN_FONTS: SceFontErrorCode = 2152071177;
pub const SCE_FONT_FAMILY_DEFAULT: SceFontFamilyCode = 0;
pub const SCE_FONT_FAMILY_ROUNDED: SceFontFamilyCode = 3;
pub const SCE_FONT_FAMILY_SANS_SERIF: SceFontFamilyCode = 1;
pub const SCE_FONT_FAMILY_SERIF: SceFontFamilyCode = 2;
pub const SCE_FONT_LANGUAGE_CHINESE: SceFontLanguageCode = 4;
pub const SCE_FONT_LANGUAGE_CJK: SceFontLanguageCode = 5;
pub const SCE_FONT_LANGUAGE_DEFAULT: SceFontLanguageCode = 0;
pub const SCE_FONT_LANGUAGE_JAPANESE: SceFontLanguageCode = 1;
pub const SCE_FONT_LANGUAGE_KOREAN: SceFontLanguageCode = 3;
pub const SCE_FONT_LANGUAGE_LATIN: SceFontLanguageCode = 2;
pub const SCE_FONT_PIXELFORMAT_24: SceFontPixelFormatCode = 3;
pub const SCE_FONT_PIXELFORMAT_32: SceFontPixelFormatCode = 4;
pub const SCE_FONT_PIXELFORMAT_4: SceFontPixelFormatCode = 0;
pub const SCE_FONT_PIXELFORMAT_4_REV: SceFontPixelFormatCode = 1;
pub const SCE_FONT_PIXELFORMAT_8: SceFontPixelFormatCode = 2;
pub const SCE_FONT_STYLE_B: SceFontStyleCode = 104;
pub const SCE_FONT_STYLE_BLACK: SceFontStyleCode = 7;
pub const SCE_FONT_STYLE_BLACK_ITALIC: SceFontStyleCode = 8;
pub const SCE_FONT_STYLE_BOLD: SceFontStyleCode = 5;
pub const SCE_FONT_STYLE_BOLD_ITALIC: SceFontStyleCode = 6;
pub const SCE_FONT_STYLE_DB: SceFontStyleCode = 103;
pub const SCE_FONT_STYLE_DEFAULT: SceFontStyleCode = 0;
pub const SCE_FONT_STYLE_EB: SceFontStyleCode = 105;
pub const SCE_FONT_STYLE_ITALIC: SceFontStyleCode = 2;
pub const SCE_FONT_STYLE_L: SceFontStyleCode = 101;
pub const SCE_FONT_STYLE_M: SceFontStyleCode = 102;
pub const SCE_FONT_STYLE_NARROW: SceFontStyleCode = 3;
pub const SCE_FONT_STYLE_NARROW_ITALIC: SceFontStyleCode = 4;
pub const SCE_FONT_STYLE_REGULAR: SceFontStyleCode = 1;
pub const SCE_FONT_STYLE_UB: SceFontStyleCode = 106;
pub const SCE_GPIO_ERROR_INVALID_BUS: SceGpioErrorCode = 2151612672;
pub const SCE_GPIO_ERROR_INVALID_MODE: SceGpioErrorCode = 2151612674;
pub const SCE_GPIO_ERROR_INVALID_PORT: SceGpioErrorCode = 2151612673;
pub const SCE_GPIO_PORT_MASK_LED_GAMECARD: SceGpioPortMasks = 64;
pub const SCE_GPIO_PORT_MASK_LED_PS_BUTTON: SceGpioPortMasks = 128;
pub const SCE_GPIO_PORT_MODE_INPUT: SceGpioPortMode = 1;
pub const SCE_GPIO_PORT_MODE_OUTPUT: SceGpioPortMode = 0;
pub const SCE_GUID_ALL_PROCESS: _bindgen_ty_2 = 65569;
pub const SCE_GUID_CLEANER_THREAD: _bindgen_ty_2 = 65559;
pub const SCE_GUID_DUMMY_PROCESS_GAME: _bindgen_ty_2 = 65555;
pub const SCE_GUID_DUMMY_PROCESS_SYSTEM: _bindgen_ty_2 = 65557;
pub const SCE_GUID_GLOBAL_WORK_QUEUE: _bindgen_ty_2 = 65571;
pub const SCE_GUID_GUID_ENTRY_HEAP: _bindgen_ty_2 = 65539;
pub const SCE_GUID_IDLE_THREAD_0: _bindgen_ty_2 = 65561;
pub const SCE_GUID_IDLE_THREAD_1: _bindgen_ty_2 = 65563;
pub const SCE_GUID_IDLE_THREAD_2: _bindgen_ty_2 = 65565;
pub const SCE_GUID_IDLE_THREAD_3: _bindgen_ty_2 = 65567;
pub const SCE_GUID_KERNEL_ADDRESS_SPACE: _bindgen_ty_2 = 65543;
pub const SCE_GUID_KERNEL_FIXED_HEAP_128B: _bindgen_ty_2 = 65595;
pub const SCE_GUID_KERNEL_FIXED_HEAP_16B: _bindgen_ty_2 = 65587;
pub const SCE_GUID_KERNEL_FIXED_HEAP_256B: _bindgen_ty_2 = 65597;
pub const SCE_GUID_KERNEL_FIXED_HEAP_32B: _bindgen_ty_2 = 65589;
pub const SCE_GUID_KERNEL_FIXED_HEAP_48B: _bindgen_ty_2 = 65591;
pub const SCE_GUID_KERNEL_FIXED_HEAP_512B: _bindgen_ty_2 = 65599;
pub const SCE_GUID_KERNEL_FIXED_HEAP_64B: _bindgen_ty_2 = 65593;
pub const SCE_GUID_KERNEL_FIXED_HEAP_L2_OBJECT: _bindgen_ty_2 = 65601;
pub const SCE_GUID_KERNEL_HEAP: _bindgen_ty_2 = 65547;
pub const SCE_GUID_KERNEL_HEAP_TOOL: _bindgen_ty_2 = 65579;
pub const SCE_GUID_KERNEL_HEAP_UNCACHE: _bindgen_ty_2 = 65549;
pub const SCE_GUID_KERNEL_PROCESS_ID: _bindgen_ty_2 = 65541;
pub const SCE_GUID_PART_CDRAM: _bindgen_ty_2 = 65607;
pub const SCE_GUID_PART_IO: _bindgen_ty_2 = 65605;
pub const SCE_GUID_PART_KMP: _bindgen_ty_2 = 65615;
pub const SCE_GUID_PART_ROOT: _bindgen_ty_2 = 65545;
pub const SCE_GUID_PART_ROOT_TOOL: _bindgen_ty_2 = 65621;
pub const SCE_GUID_PART_ROOT_TOOL_UNCACHE: _bindgen_ty_2 = 65623;
pub const SCE_GUID_PART_ROOT_UNCACHE: _bindgen_ty_2 = 65613;
pub const SCE_GUID_PART_ROOT_UNCACHE_GPU_GAME: _bindgen_ty_2 = 65617;
pub const SCE_GUID_PART_TMP: _bindgen_ty_2 = 65611;
pub const SCE_GUID_PART_TMP_FS_GAME: _bindgen_ty_2 = 65619;
pub const SCE_GUID_PART_USER_SHARED: _bindgen_ty_2 = 65603;
pub const SCE_GUID_PHYMEM_PART_KERNEL: _bindgen_ty_2 = 65551;
pub const SCE_GUID_PHYMEM_PART_TOOL: _bindgen_ty_2 = 65553;
pub const SCE_GUID_PROCESS_BUDGET_FULL_GAME: _bindgen_ty_2 = 65629;
pub const SCE_GUID_PROCESS_BUDGET_KERNEL: _bindgen_ty_2 = 65625;
pub const SCE_GUID_PROCESS_BUDGET_SHELL: _bindgen_ty_2 = 65627;
pub const SCE_GUID_RESERVED_29: _bindgen_ty_2 = 65577;
pub const SCE_GUID_RESERVED_2D: _bindgen_ty_2 = 65581;
pub const SCE_GUID_RESERVED_2F: _bindgen_ty_2 = 65583;
pub const SCE_GUID_RESERVED_31: _bindgen_ty_2 = 65585;
pub const SCE_GUID_RESERVED_49: _bindgen_ty_2 = 65609;
pub const SCE_GUID_RESERVED_5F: _bindgen_ty_2 = 65631;
pub const SCE_GUID_SYSROOT: _bindgen_ty_2 = 65537;
pub const SCE_GUID_THREAD_ID_PROCESS_ALL: _bindgen_ty_2 = 65575;
pub const SCE_GUID_THREAD_ID_SYSTEM_ALL: _bindgen_ty_2 = 65573;
pub const SCE_GXM_ATTRIBUTE_FORMAT_F16: SceGxmAttributeFormat = 8;
pub const SCE_GXM_ATTRIBUTE_FORMAT_F32: SceGxmAttributeFormat = 9;
pub const SCE_GXM_ATTRIBUTE_FORMAT_S16: SceGxmAttributeFormat = 3;
pub const SCE_GXM_ATTRIBUTE_FORMAT_S16N: SceGxmAttributeFormat = 7;
pub const SCE_GXM_ATTRIBUTE_FORMAT_S8: SceGxmAttributeFormat = 1;
pub const SCE_GXM_ATTRIBUTE_FORMAT_S8N: SceGxmAttributeFormat = 5;
pub const SCE_GXM_ATTRIBUTE_FORMAT_U16: SceGxmAttributeFormat = 2;
pub const SCE_GXM_ATTRIBUTE_FORMAT_U16N: SceGxmAttributeFormat = 6;
pub const SCE_GXM_ATTRIBUTE_FORMAT_U8: SceGxmAttributeFormat = 0;
pub const SCE_GXM_ATTRIBUTE_FORMAT_U8N: SceGxmAttributeFormat = 4;
pub const SCE_GXM_ATTRIBUTE_FORMAT_UNTYPED: SceGxmAttributeFormat = 10;
pub const SCE_GXM_BLEND_FACTOR_DST_ALPHA: SceGxmBlendFactor = 8;
pub const SCE_GXM_BLEND_FACTOR_DST_ALPHA_SATURATE: SceGxmBlendFactor = 11;
pub const SCE_GXM_BLEND_FACTOR_DST_COLOR: SceGxmBlendFactor = 6;
pub const SCE_GXM_BLEND_FACTOR_ONE: SceGxmBlendFactor = 1;
pub const SCE_GXM_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: SceGxmBlendFactor = 9;
pub const SCE_GXM_BLEND_FACTOR_ONE_MINUS_DST_COLOR: SceGxmBlendFactor = 7;
pub const SCE_GXM_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: SceGxmBlendFactor = 5;
pub const SCE_GXM_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: SceGxmBlendFactor = 3;
pub const SCE_GXM_BLEND_FACTOR_SRC_ALPHA: SceGxmBlendFactor = 4;
pub const SCE_GXM_BLEND_FACTOR_SRC_ALPHA_SATURATE: SceGxmBlendFactor = 10;
pub const SCE_GXM_BLEND_FACTOR_SRC_COLOR: SceGxmBlendFactor = 2;
pub const SCE_GXM_BLEND_FACTOR_ZERO: SceGxmBlendFactor = 0;
pub const SCE_GXM_BLEND_FUNC_ADD: SceGxmBlendFunc = 1;
pub const SCE_GXM_BLEND_FUNC_MAX: SceGxmBlendFunc = 5;
pub const SCE_GXM_BLEND_FUNC_MIN: SceGxmBlendFunc = 4;
pub const SCE_GXM_BLEND_FUNC_NONE: SceGxmBlendFunc = 0;
pub const SCE_GXM_BLEND_FUNC_REVERSE_SUBTRACT: SceGxmBlendFunc = 3;
pub const SCE_GXM_BLEND_FUNC_SUBTRACT: SceGxmBlendFunc = 2;
pub const SCE_GXM_COLOR_BASE_FORMAT_F11F11F10: SceGxmColorBaseFormat = 553648128;
pub const SCE_GXM_COLOR_BASE_FORMAT_F16: SceGxmColorBaseFormat = 4026531840;
pub const SCE_GXM_COLOR_BASE_FORMAT_F16F16: SceGxmColorBaseFormat = 8388608;
pub const SCE_GXM_COLOR_BASE_FORMAT_F16F16F16F16: SceGxmColorBaseFormat = 16777216;
pub const SCE_GXM_COLOR_BASE_FORMAT_F32: SceGxmColorBaseFormat = 276824064;
pub const SCE_GXM_COLOR_BASE_FORMAT_F32F32: SceGxmColorBaseFormat = 285212672;
pub const SCE_GXM_COLOR_BASE_FORMAT_S16: SceGxmColorBaseFormat = 545259520;
pub const SCE_GXM_COLOR_BASE_FORMAT_S16S16: SceGxmColorBaseFormat = 813694976;
pub const SCE_GXM_COLOR_BASE_FORMAT_S5S5U6: SceGxmColorBaseFormat = 2692743168;
pub const SCE_GXM_COLOR_BASE_FORMAT_S8: SceGxmColorBaseFormat = 2424307712;
pub const SCE_GXM_COLOR_BASE_FORMAT_S8S8: SceGxmColorBaseFormat = 3229614080;
pub const SCE_GXM_COLOR_BASE_FORMAT_S8S8S8S8: SceGxmColorBaseFormat = 3766484992;
pub const SCE_GXM_COLOR_BASE_FORMAT_SE5M9M9M9: SceGxmColorBaseFormat = 822083584;
pub const SCE_GXM_COLOR_BASE_FORMAT_U16: SceGxmColorBaseFormat = 1082130432;
pub const SCE_GXM_COLOR_BASE_FORMAT_U16U16: SceGxmColorBaseFormat = 1350565888;
pub const SCE_GXM_COLOR_BASE_FORMAT_U1U5U5U5: SceGxmColorBaseFormat = 1073741824;
pub const SCE_GXM_COLOR_BASE_FORMAT_U2F10F10F10: SceGxmColorBaseFormat = 1090519040;
pub const SCE_GXM_COLOR_BASE_FORMAT_U2U10U10U10: SceGxmColorBaseFormat = 1619001344;
pub const SCE_GXM_COLOR_BASE_FORMAT_U4U4U4U4: SceGxmColorBaseFormat = 1342177280;
pub const SCE_GXM_COLOR_BASE_FORMAT_U5U6U5: SceGxmColorBaseFormat = 805306368;
pub const SCE_GXM_COLOR_BASE_FORMAT_U8: SceGxmColorBaseFormat = 2155872256;
pub const SCE_GXM_COLOR_BASE_FORMAT_U8S8S8U8: SceGxmColorBaseFormat = 3498049536;
pub const SCE_GXM_COLOR_BASE_FORMAT_U8U3U3U2: SceGxmColorBaseFormat = 1610612736;
pub const SCE_GXM_COLOR_BASE_FORMAT_U8U8: SceGxmColorBaseFormat = 2961178624;
pub const SCE_GXM_COLOR_BASE_FORMAT_U8U8U8: SceGxmColorBaseFormat = 268435456;
pub const SCE_GXM_COLOR_BASE_FORMAT_U8U8U8U8: SceGxmColorBaseFormat = 0;
pub const SCE_GXM_COLOR_FORMAT_A1R5G5B5: SceGxmColorFormat = 1074790400;
pub const SCE_GXM_COLOR_FORMAT_A4R4G4B4: SceGxmColorFormat = 1343225856;
pub const SCE_GXM_COLOR_FORMAT_A8: SceGxmColorFormat = 2156920832;
pub const SCE_GXM_COLOR_FORMAT_A8B8G8R8: SceGxmColorFormat = 0;
pub const SCE_GXM_COLOR_FORMAT_A8R8G8B8: SceGxmColorFormat = 1048576;
pub const SCE_GXM_COLOR_FORMAT_F10F10F10U2_BGRA: SceGxmColorFormat = 1093664768;
pub const SCE_GXM_COLOR_FORMAT_F10F10F10U2_RGBA: SceGxmColorFormat = 1092616192;
pub const SCE_GXM_COLOR_FORMAT_F10F11F11_BGR: SceGxmColorFormat = 553648128;
pub const SCE_GXM_COLOR_FORMAT_F11F11F10_RGB: SceGxmColorFormat = 554696704;
pub const SCE_GXM_COLOR_FORMAT_F16F16F16F16_ABGR: SceGxmColorFormat = 16777216;
pub const SCE_GXM_COLOR_FORMAT_F16F16F16F16_ARGB: SceGxmColorFormat = 17825792;
pub const SCE_GXM_COLOR_FORMAT_F16F16F16F16_BGRA: SceGxmColorFormat = 19922944;
pub const SCE_GXM_COLOR_FORMAT_F16F16F16F16_RGBA: SceGxmColorFormat = 18874368;
pub const SCE_GXM_COLOR_FORMAT_F16F16_GR: SceGxmColorFormat = 8388608;
pub const SCE_GXM_COLOR_FORMAT_F16F16_RG: SceGxmColorFormat = 9437184;
pub const SCE_GXM_COLOR_FORMAT_F16_G: SceGxmColorFormat = 4027580416;
pub const SCE_GXM_COLOR_FORMAT_F16_R: SceGxmColorFormat = 4026531840;
pub const SCE_GXM_COLOR_FORMAT_F32F32_GR: SceGxmColorFormat = 285212672;
pub const SCE_GXM_COLOR_FORMAT_F32F32_RG: SceGxmColorFormat = 286261248;
pub const SCE_GXM_COLOR_FORMAT_F32_R: SceGxmColorFormat = 276824064;
pub const SCE_GXM_COLOR_FORMAT_R5G6B5: SceGxmColorFormat = 806354944;
pub const SCE_GXM_COLOR_FORMAT_S16_G: SceGxmColorFormat = 546308096;
pub const SCE_GXM_COLOR_FORMAT_S16_R: SceGxmColorFormat = 545259520;
pub const SCE_GXM_COLOR_FORMAT_S16S16_GR: SceGxmColorFormat = 813694976;
pub const SCE_GXM_COLOR_FORMAT_S16S16_RG: SceGxmColorFormat = 814743552;
pub const SCE_GXM_COLOR_FORMAT_S5S5U6_RGB: SceGxmColorFormat = 2693791744;
pub const SCE_GXM_COLOR_FORMAT_S8_A: SceGxmColorFormat = 2425356288;
pub const SCE_GXM_COLOR_FORMAT_S8_R: SceGxmColorFormat = 2424307712;
pub const SCE_GXM_COLOR_FORMAT_S8S8_AR: SceGxmColorFormat = 3232759808;
pub const SCE_GXM_COLOR_FORMAT_S8S8_GR: SceGxmColorFormat = 3229614080;
pub const SCE_GXM_COLOR_FORMAT_S8S8_RA: SceGxmColorFormat = 3231711232;
pub const SCE_GXM_COLOR_FORMAT_S8S8_RG: SceGxmColorFormat = 3230662656;
pub const SCE_GXM_COLOR_FORMAT_S8S8S8S8_ABGR: SceGxmColorFormat = 3766484992;
pub const SCE_GXM_COLOR_FORMAT_S8S8S8S8_ARGB: SceGxmColorFormat = 3767533568;
pub const SCE_GXM_COLOR_FORMAT_S8S8S8S8_BGRA: SceGxmColorFormat = 3769630720;
pub const SCE_GXM_COLOR_FORMAT_S8S8S8S8_RGBA: SceGxmColorFormat = 3768582144;
pub const SCE_GXM_COLOR_FORMAT_S8S8U8U8_BGRA: SceGxmColorFormat = 3501195264;
pub const SCE_GXM_COLOR_FORMAT_SE5M9M9M9_BGR: SceGxmColorFormat = 822083584;
pub const SCE_GXM_COLOR_FORMAT_SE5M9M9M9_RGB: SceGxmColorFormat = 823132160;
pub const SCE_GXM_COLOR_FORMAT_U10U10U10U2_BGRA: SceGxmColorFormat = 1622147072;
pub const SCE_GXM_COLOR_FORMAT_U10U10U10U2_RGBA: SceGxmColorFormat = 1621098496;
pub const SCE_GXM_COLOR_FORMAT_U16_G: SceGxmColorFormat = 1083179008;
pub const SCE_GXM_COLOR_FORMAT_U16_R: SceGxmColorFormat = 1082130432;
pub const SCE_GXM_COLOR_FORMAT_U16U16_GR: SceGxmColorFormat = 1350565888;
pub const SCE_GXM_COLOR_FORMAT_U16U16_RG: SceGxmColorFormat = 1351614464;
pub const SCE_GXM_COLOR_FORMAT_U1U5U5U5_ABGR: SceGxmColorFormat = 1073741824;
pub const SCE_GXM_COLOR_FORMAT_U1U5U5U5_ARGB: SceGxmColorFormat = 1074790400;
pub const SCE_GXM_COLOR_FORMAT_U2F10F10F10_ABGR: SceGxmColorFormat = 1090519040;
pub const SCE_GXM_COLOR_FORMAT_U2F10F10F10_ARGB: SceGxmColorFormat = 1091567616;
pub const SCE_GXM_COLOR_FORMAT_U2U10U10U10_ABGR: SceGxmColorFormat = 1619001344;
pub const SCE_GXM_COLOR_FORMAT_U2U10U10U10_ARGB: SceGxmColorFormat = 1620049920;
pub const SCE_GXM_COLOR_FORMAT_U4U4U4U4_ABGR: SceGxmColorFormat = 1342177280;
pub const SCE_GXM_COLOR_FORMAT_U4U4U4U4_ARGB: SceGxmColorFormat = 1343225856;
pub const SCE_GXM_COLOR_FORMAT_U4U4U4U4_BGRA: SceGxmColorFormat = 1345323008;
pub const SCE_GXM_COLOR_FORMAT_U4U4U4U4_RGBA: SceGxmColorFormat = 1344274432;
pub const SCE_GXM_COLOR_FORMAT_U5U5U5U1_BGRA: SceGxmColorFormat = 1076887552;
pub const SCE_GXM_COLOR_FORMAT_U5U5U5U1_RGBA: SceGxmColorFormat = 1075838976;
pub const SCE_GXM_COLOR_FORMAT_U5U6U5_BGR: SceGxmColorFormat = 805306368;
pub const SCE_GXM_COLOR_FORMAT_U5U6U5_RGB: SceGxmColorFormat = 806354944;
pub const SCE_GXM_COLOR_FORMAT_U6S5S5_BGR: SceGxmColorFormat = 2692743168;
pub const SCE_GXM_COLOR_FORMAT_U8_A: SceGxmColorFormat = 2156920832;
pub const SCE_GXM_COLOR_FORMAT_U8_R: SceGxmColorFormat = 2155872256;
pub const SCE_GXM_COLOR_FORMAT_U8S8S8U8_ABGR: SceGxmColorFormat = 3498049536;
pub const SCE_GXM_COLOR_FORMAT_U8S8S8U8_RGBA: SceGxmColorFormat = 3500146688;
pub const SCE_GXM_COLOR_FORMAT_U8U3U3U2_ARGB: SceGxmColorFormat = 1610612736;
pub const SCE_GXM_COLOR_FORMAT_U8U8_AR: SceGxmColorFormat = 2964324352;
pub const SCE_GXM_COLOR_FORMAT_U8U8_GR: SceGxmColorFormat = 2961178624;
pub const SCE_GXM_COLOR_FORMAT_U8U8_RA: SceGxmColorFormat = 2963275776;
pub const SCE_GXM_COLOR_FORMAT_U8U8_RG: SceGxmColorFormat = 2962227200;
pub const SCE_GXM_COLOR_FORMAT_U8U8S8S8_ARGB: SceGxmColorFormat = 3499098112;
pub const SCE_GXM_COLOR_FORMAT_U8U8U8_BGR: SceGxmColorFormat = 268435456;
pub const SCE_GXM_COLOR_FORMAT_U8U8U8_RGB: SceGxmColorFormat = 269484032;
pub const SCE_GXM_COLOR_FORMAT_U8U8U8U8_ABGR: SceGxmColorFormat = 0;
pub const SCE_GXM_COLOR_FORMAT_U8U8U8U8_ARGB: SceGxmColorFormat = 1048576;
pub const SCE_GXM_COLOR_FORMAT_U8U8U8U8_BGRA: SceGxmColorFormat = 3145728;
pub const SCE_GXM_COLOR_FORMAT_U8U8U8U8_RGBA: SceGxmColorFormat = 2097152;
pub const SCE_GXM_COLOR_MASK_A: SceGxmColorMask = 1;
pub const SCE_GXM_COLOR_MASK_ALL: SceGxmColorMask = 15;
pub const SCE_GXM_COLOR_MASK_B: SceGxmColorMask = 8;
pub const SCE_GXM_COLOR_MASK_G: SceGxmColorMask = 4;
pub const SCE_GXM_COLOR_MASK_NONE: SceGxmColorMask = 0;
pub const SCE_GXM_COLOR_MASK_R: SceGxmColorMask = 2;
pub const SCE_GXM_COLOR_SURFACE_ALIGNMENT: u32 = 4;
pub const SCE_GXM_COLOR_SURFACE_DITHER_DISABLED: SceGxmColorSurfaceDitherMode = 0;
pub const SCE_GXM_COLOR_SURFACE_DITHER_ENABLED: SceGxmColorSurfaceDitherMode = 8;
pub const SCE_GXM_COLOR_SURFACE_GAMMA_BGR: SceGxmColorSurfaceGammaMode = 4096;
pub const SCE_GXM_COLOR_SURFACE_GAMMA_GR: SceGxmColorSurfaceGammaMode = 12288;
pub const SCE_GXM_COLOR_SURFACE_GAMMA_NONE: SceGxmColorSurfaceGammaMode = 0;
pub const SCE_GXM_COLOR_SURFACE_GAMMA_R: SceGxmColorSurfaceGammaMode = 4096;
pub const SCE_GXM_COLOR_SURFACE_LINEAR: SceGxmColorSurfaceType = 0;
pub const SCE_GXM_COLOR_SURFACE_SCALE_MSAA_DOWNSCALE: SceGxmColorSurfaceScaleMode = 1;
pub const SCE_GXM_COLOR_SURFACE_SCALE_NONE: SceGxmColorSurfaceScaleMode = 0;
pub const SCE_GXM_COLOR_SURFACE_SWIZZLED: SceGxmColorSurfaceType = 134217728;
pub const SCE_GXM_COLOR_SURFACE_TILED: SceGxmColorSurfaceType = 67108864;
pub const SCE_GXM_COLOR_SWIZZLE1_A: SceGxmColorSwizzle1Mode = 1048576;
pub const SCE_GXM_COLOR_SWIZZLE1_G: SceGxmColorSwizzle1Mode = 1048576;
pub const SCE_GXM_COLOR_SWIZZLE1_R: SceGxmColorSwizzle1Mode = 0;
pub const SCE_GXM_COLOR_SWIZZLE2_AR: SceGxmColorSwizzle2Mode = 3145728;
pub const SCE_GXM_COLOR_SWIZZLE2_GR: SceGxmColorSwizzle2Mode = 0;
pub const SCE_GXM_COLOR_SWIZZLE2_RA: SceGxmColorSwizzle2Mode = 2097152;
pub const SCE_GXM_COLOR_SWIZZLE2_RG: SceGxmColorSwizzle2Mode = 1048576;
pub const SCE_GXM_COLOR_SWIZZLE3_BGR: SceGxmColorSwizzle3Mode = 0;
pub const SCE_GXM_COLOR_SWIZZLE3_RGB: SceGxmColorSwizzle3Mode = 1048576;
pub const SCE_GXM_COLOR_SWIZZLE4_ABGR: SceGxmColorSwizzle4Mode = 0;
pub const SCE_GXM_COLOR_SWIZZLE4_ARGB: SceGxmColorSwizzle4Mode = 1048576;
pub const SCE_GXM_COLOR_SWIZZLE4_BGRA: SceGxmColorSwizzle4Mode = 3145728;
pub const SCE_GXM_COLOR_SWIZZLE4_RGBA: SceGxmColorSwizzle4Mode = 2097152;
pub const SCE_GXM_CULL_CCW: SceGxmCullMode = 2;
pub const SCE_GXM_CULL_CW: SceGxmCullMode = 1;
pub const SCE_GXM_CULL_NONE: SceGxmCullMode = 0;
pub const SCE_GXM_DEFAULT_FRAGMENT_RING_BUFFER_SIZE: u32 = 524288;
pub const SCE_GXM_DEFAULT_FRAGMENT_USSE_RING_BUFFER_SIZE: u32 = 16384;
pub const SCE_GXM_DEFAULT_PARAMETER_BUFFER_SIZE: u32 = 16777216;
pub const SCE_GXM_DEFAULT_VDM_RING_BUFFER_SIZE: u32 = 131072;
pub const SCE_GXM_DEFAULT_VERTEX_RING_BUFFER_SIZE: u32 = 2097152;
pub const SCE_GXM_DEPTH_FUNC_ALWAYS: SceGxmDepthFunc = 29360128;
pub const SCE_GXM_DEPTH_FUNC_EQUAL: SceGxmDepthFunc = 8388608;
pub const SCE_GXM_DEPTH_FUNC_GREATER: SceGxmDepthFunc = 16777216;
pub const SCE_GXM_DEPTH_FUNC_GREATER_EQUAL: SceGxmDepthFunc = 25165824;
pub const SCE_GXM_DEPTH_FUNC_LESS: SceGxmDepthFunc = 4194304;
pub const SCE_GXM_DEPTH_FUNC_LESS_EQUAL: SceGxmDepthFunc = 12582912;
pub const SCE_GXM_DEPTH_FUNC_NEVER: SceGxmDepthFunc = 0;
pub const SCE_GXM_DEPTH_FUNC_NOT_EQUAL: SceGxmDepthFunc = 20971520;
pub const SCE_GXM_DEPTH_STENCIL_FORCE_LOAD_DISABLED: SceGxmDepthStencilForceLoadMode = 0;
pub const SCE_GXM_DEPTH_STENCIL_FORCE_LOAD_ENABLED: SceGxmDepthStencilForceLoadMode = 2;
pub const SCE_GXM_DEPTH_STENCIL_FORCE_STORE_DISABLED: SceGxmDepthStencilForceStoreMode = 0;
pub const SCE_GXM_DEPTH_STENCIL_FORCE_STORE_ENABLED: SceGxmDepthStencilForceStoreMode = 4;
pub const SCE_GXM_DEPTH_STENCIL_FORMAT_D16: SceGxmDepthStencilFormat = 38027264;
pub const SCE_GXM_DEPTH_STENCIL_FORMAT_DF32: SceGxmDepthStencilFormat = 278528;
pub const SCE_GXM_DEPTH_STENCIL_FORMAT_DF32M: SceGxmDepthStencilFormat = 835584;
pub const SCE_GXM_DEPTH_STENCIL_FORMAT_DF32M_S8: SceGxmDepthStencilFormat = 974848;
pub const SCE_GXM_DEPTH_STENCIL_FORMAT_DF32_S8: SceGxmDepthStencilFormat = 417792;
pub const SCE_GXM_DEPTH_STENCIL_FORMAT_S8: SceGxmDepthStencilFormat = 139264;
pub const SCE_GXM_DEPTH_STENCIL_FORMAT_S8D24: SceGxmDepthStencilFormat = 19292160;
pub const SCE_GXM_DEPTHSTENCIL_SURFACE_ALIGNMENT: u32 = 16;
pub const SCE_GXM_DEPTH_STENCIL_SURFACE_LINEAR: SceGxmDepthStencilSurfaceType = 0;
pub const SCE_GXM_DEPTH_STENCIL_SURFACE_TILED: SceGxmDepthStencilSurfaceType = 69632;
pub const SCE_GXM_DEPTH_WRITE_DISABLED: SceGxmDepthWriteMode = 1048576;
pub const SCE_GXM_DEPTH_WRITE_ENABLED: SceGxmDepthWriteMode = 0;
pub const SCE_GXM_EDGE_ENABLE_01: SceGxmEdgeEnableFlags = 256;
pub const SCE_GXM_EDGE_ENABLE_12: SceGxmEdgeEnableFlags = 512;
pub const SCE_GXM_EDGE_ENABLE_20: SceGxmEdgeEnableFlags = 1024;
pub const SCE_GXM_ERROR_ALREADY_INITIALIZED: SceGxmErrorCode = 2153447425;
pub const SCE_GXM_ERROR_DRIVER: SceGxmErrorCode = 2153447447;
pub const SCE_GXM_ERROR_INVALID_ALIGNMENT: SceGxmErrorCode = 2153447429;
pub const SCE_GXM_ERROR_INVALID_AUXILIARY_SURFACE: SceGxmErrorCode = 2153447443;
pub const SCE_GXM_ERROR_INVALID_INDEX_COUNT: SceGxmErrorCode = 2153447437;
pub const SCE_GXM_ERROR_INVALID_POINTER: SceGxmErrorCode = 2153447428;
pub const SCE_GXM_ERROR_INVALID_POLYGON_MODE: SceGxmErrorCode = 2153447438;
pub const SCE_GXM_ERROR_INVALID_PRECOMPUTED_DRAW: SceGxmErrorCode = 2153447444;
pub const SCE_GXM_ERROR_INVALID_PRECOMPUTED_FRAGMENT_STATE: SceGxmErrorCode = 2153447446;
pub const SCE_GXM_ERROR_INVALID_PRECOMPUTED_VERTEX_STATE: SceGxmErrorCode = 2153447445;
pub const SCE_GXM_ERROR_INVALID_SAMPLER_RESULT_TYPE_COMPONENT_COUNT: SceGxmErrorCode = 2153447440;
pub const SCE_GXM_ERROR_INVALID_SAMPLER_RESULT_TYPE_PRECISION: SceGxmErrorCode = 2153447439;
pub const SCE_GXM_ERROR_INVALID_TEXTURE: SceGxmErrorCode = 2153447448;
pub const SCE_GXM_ERROR_INVALID_TEXTURE_DATA_POINTER: SceGxmErrorCode = 2153447449;
pub const SCE_GXM_ERROR_INVALID_TEXTURE_PALETTE_POINTER: SceGxmErrorCode = 2153447450;
pub const SCE_GXM_ERROR_INVALID_VALUE: SceGxmErrorCode = 2153447427;
pub const SCE_GXM_ERROR_NOT_WITHIN_SCENE: SceGxmErrorCode = 2153447430;
pub const SCE_GXM_ERROR_NULL_PROGRAM: SceGxmErrorCode = 2153447432;
pub const SCE_GXM_ERROR_OUT_OF_MEMORY: SceGxmErrorCode = 2153447426;
pub const SCE_GXM_ERROR_OUT_OF_RENDER_TARGETS: SceGxmErrorCode = 2153447463;
pub const SCE_GXM_ERROR_PATCHER_INTERNAL: SceGxmErrorCode = 2153447434;
pub const SCE_GXM_ERROR_PROGRAM_IN_USE: SceGxmErrorCode = 2153447436;
pub const SCE_GXM_ERROR_RESERVE_FAILED: SceGxmErrorCode = 2153447435;
pub const SCE_GXM_ERROR_UNIFORM_BUFFER_NOT_RESERVED: SceGxmErrorCode = 2153447441;
pub const SCE_GXM_ERROR_UNINITIALIZED: SceGxmErrorCode = 2153447424;
pub const SCE_GXM_ERROR_UNSUPPORTED: SceGxmErrorCode = 2153447433;
pub const SCE_GXM_ERROR_WITHIN_SCENE: SceGxmErrorCode = 2153447431;
pub const SCE_GXM_FRAGMENT_PROGRAM: SceGxmProgramType = 1;
pub const SCE_GXM_FRAGMENT_PROGRAM_DISABLED: SceGxmFragmentProgramMode = 2097152;
pub const SCE_GXM_FRAGMENT_PROGRAM_ENABLED: SceGxmFragmentProgramMode = 0;
pub const SCE_GXM_INDEX_FORMAT_U16: SceGxmIndexFormat = 0;
pub const SCE_GXM_INDEX_FORMAT_U32: SceGxmIndexFormat = 16777216;
pub const SCE_GXM_INDEX_SOURCE_INDEX_16BIT: SceGxmIndexSource = 0;
pub const SCE_GXM_INDEX_SOURCE_INDEX_32BIT: SceGxmIndexSource = 1;
pub const SCE_GXM_INDEX_SOURCE_INSTANCE_16BIT: SceGxmIndexSource = 2;
pub const SCE_GXM_INDEX_SOURCE_INSTANCE_32BIT: SceGxmIndexSource = 3;
pub const SCE_GXM_LINE_FILL_LAST_PIXEL_DISABLED: SceGxmLineFillLastPixelMode = 0;
pub const SCE_GXM_LINE_FILL_LAST_PIXEL_ENABLED: SceGxmLineFillLastPixelMode = 524288;
pub const SCE_GXM_MAX_AUXILIARY_SURFACES: u32 = 3;
pub const SCE_GXM_MAX_TEXTURE_UNITS: u32 = 16;
pub const SCE_GXM_MAX_UNIFORM_BUFFERS: u32 = 14;
pub const SCE_GXM_MAX_VERTEX_ATTRIBUTES: u32 = 16;
pub const SCE_GXM_MAX_VERTEX_STREAMS: u32 = 16;
pub const SCE_GXM_MEMORY_ATTRIB_READ: SceGxmMemoryAttribFlags = 1;
pub const SCE_GXM_MEMORY_ATTRIB_RW: SceGxmMemoryAttribFlags = 3;
pub const SCE_GXM_MEMORY_ATTRIB_WRITE: SceGxmMemoryAttribFlags = 2;
pub const SCE_GXM_MIDSCENE_PRESERVE_DEFAULT_UNIFORM_BUFFERS: SceGxmMidSceneFlags = 1;
pub const SCE_GXM_MINIMUM_CONTEXT_HOST_MEM_SIZE: u32 = 2048;
pub const SCE_GXM_MULTISAMPLE_2X: SceGxmMultisampleMode = 1;
pub const SCE_GXM_MULTISAMPLE_4X: SceGxmMultisampleMode = 2;
pub const SCE_GXM_MULTISAMPLE_NONE: SceGxmMultisampleMode = 0;
pub const SCE_GXM_OUTPUT_REGISTER_FORMAT_CHAR4: SceGxmOutputRegisterFormat = 2;
pub const SCE_GXM_OUTPUT_REGISTER_FORMAT_DECLARED: SceGxmOutputRegisterFormat = 0;
pub const SCE_GXM_OUTPUT_REGISTER_FORMAT_FLOAT: SceGxmOutputRegisterFormat = 8;
pub const SCE_GXM_OUTPUT_REGISTER_FORMAT_FLOAT2: SceGxmOutputRegisterFormat = 7;
pub const SCE_GXM_OUTPUT_REGISTER_FORMAT_HALF2: SceGxmOutputRegisterFormat = 6;
pub const SCE_GXM_OUTPUT_REGISTER_FORMAT_HALF4: SceGxmOutputRegisterFormat = 5;
pub const SCE_GXM_OUTPUT_REGISTER_FORMAT_SHORT2: SceGxmOutputRegisterFormat = 4;
pub const SCE_GXM_OUTPUT_REGISTER_FORMAT_UCHAR4: SceGxmOutputRegisterFormat = 1;
pub const SCE_GXM_OUTPUT_REGISTER_FORMAT_USHORT2: SceGxmOutputRegisterFormat = 3;
pub const SCE_GXM_OUTPUT_REGISTER_SIZE_32BIT: SceGxmOutputRegisterSize = 0;
pub const SCE_GXM_OUTPUT_REGISTER_SIZE_64BIT: SceGxmOutputRegisterSize = 1;
pub const SCE_GXM_PALETTE_ALIGNMENT: u32 = 64;
pub const SCE_GXM_PARAMETER_CATEGORY_ATTRIBUTE: SceGxmParameterCategory = 0;
pub const SCE_GXM_PARAMETER_CATEGORY_AUXILIARY_SURFACE: SceGxmParameterCategory = 3;
pub const SCE_GXM_PARAMETER_CATEGORY_SAMPLER: SceGxmParameterCategory = 2;
pub const SCE_GXM_PARAMETER_CATEGORY_UNIFORM: SceGxmParameterCategory = 1;
pub const SCE_GXM_PARAMETER_CATEGORY_UNIFORM_BUFFER: SceGxmParameterCategory = 4;
pub const SCE_GXM_PARAMETER_SEMANTIC_ATTR: SceGxmParameterSemantic = 1;
pub const SCE_GXM_PARAMETER_SEMANTIC_BCOL: SceGxmParameterSemantic = 2;
pub const SCE_GXM_PARAMETER_SEMANTIC_BINORMAL: SceGxmParameterSemantic = 3;
pub const SCE_GXM_PARAMETER_SEMANTIC_BLENDINDICES: SceGxmParameterSemantic = 4;
pub const SCE_GXM_PARAMETER_SEMANTIC_BLENDWEIGHT: SceGxmParameterSemantic = 5;
pub const SCE_GXM_PARAMETER_SEMANTIC_COLOR: SceGxmParameterSemantic = 6;
pub const SCE_GXM_PARAMETER_SEMANTIC_DIFFUSE: SceGxmParameterSemantic = 7;
pub const SCE_GXM_PARAMETER_SEMANTIC_FOGCOORD: SceGxmParameterSemantic = 8;
pub const SCE_GXM_PARAMETER_SEMANTIC_NONE: SceGxmParameterSemantic = 0;
pub const SCE_GXM_PARAMETER_SEMANTIC_NORMAL: SceGxmParameterSemantic = 9;
pub const SCE_GXM_PARAMETER_SEMANTIC_POINTSIZE: SceGxmParameterSemantic = 10;
pub const SCE_GXM_PARAMETER_SEMANTIC_POSITION: SceGxmParameterSemantic = 11;
pub const SCE_GXM_PARAMETER_SEMANTIC_SPECULAR: SceGxmParameterSemantic = 12;
pub const SCE_GXM_PARAMETER_SEMANTIC_TANGENT: SceGxmParameterSemantic = 13;
pub const SCE_GXM_PARAMETER_SEMANTIC_TEXCOORD: SceGxmParameterSemantic = 14;
pub const SCE_GXM_PARAMETER_TYPE_AGGREGATE: SceGxmParameterType = 9;
pub const SCE_GXM_PARAMETER_TYPE_C10: SceGxmParameterType = 2;
pub const SCE_GXM_PARAMETER_TYPE_F16: SceGxmParameterType = 1;
pub const SCE_GXM_PARAMETER_TYPE_F32: SceGxmParameterType = 0;
pub const SCE_GXM_PARAMETER_TYPE_S16: SceGxmParameterType = 6;
pub const SCE_GXM_PARAMETER_TYPE_S32: SceGxmParameterType = 4;
pub const SCE_GXM_PARAMETER_TYPE_S8: SceGxmParameterType = 8;
pub const SCE_GXM_PARAMETER_TYPE_U16: SceGxmParameterType = 5;
pub const SCE_GXM_PARAMETER_TYPE_U32: SceGxmParameterType = 3;
pub const SCE_GXM_PARAMETER_TYPE_U8: SceGxmParameterType = 7;
pub const SCE_GXM_PASS_TYPE_DEPTH_REPLACE: SceGxmPassType = 167772160;
pub const SCE_GXM_PASS_TYPE_DISCARD: SceGxmPassType = 67108864;
pub const SCE_GXM_PASS_TYPE_MASK_UPDATE: SceGxmPassType = 100663296;
pub const SCE_GXM_PASS_TYPE_OPAQUE: SceGxmPassType = 0;
pub const SCE_GXM_PASS_TYPE_TRANSLUCENT: SceGxmPassType = 33554432;
pub const SCE_GXM_POLYGON_MODE_LINE: SceGxmPolygonMode = 32768;
pub const SCE_GXM_POLYGON_MODE_POINT: SceGxmPolygonMode = 98304;
pub const SCE_GXM_POLYGON_MODE_POINT_01UV: SceGxmPolygonMode = 131072;
pub const SCE_GXM_POLYGON_MODE_POINT_10UV: SceGxmPolygonMode = 65536;
pub const SCE_GXM_POLYGON_MODE_TRIANGLE_FILL: SceGxmPolygonMode = 0;
pub const SCE_GXM_POLYGON_MODE_TRIANGLE_LINE: SceGxmPolygonMode = 163840;
pub const SCE_GXM_POLYGON_MODE_TRIANGLE_POINT: SceGxmPolygonMode = 196608;
pub const SCE_GXM_PRECOMPUTED_DRAW_WORD_COUNT: SceGxmPrecomputedWordCount = 11;
pub const SCE_GXM_PRECOMPUTED_FRAGMENT_STATE_WORD_COUNT: SceGxmPrecomputedWordCount = 9;
pub const SCE_GXM_PRECOMPUTED_VERTEX_STATE_WORD_COUNT: SceGxmPrecomputedWordCount = 7;
pub const SCE_GXM_PRIMITIVE_LINES: SceGxmPrimitiveType = 67108864;
pub const SCE_GXM_PRIMITIVE_POINTS: SceGxmPrimitiveType = 134217728;
pub const SCE_GXM_PRIMITIVE_TRIANGLE_EDGES: SceGxmPrimitiveType = 335544320;
pub const SCE_GXM_PRIMITIVE_TRIANGLE_FAN: SceGxmPrimitiveType = 268435456;
pub const SCE_GXM_PRIMITIVE_TRIANGLES: SceGxmPrimitiveType = 0;
pub const SCE_GXM_PRIMITIVE_TRIANGLE_STRIP: SceGxmPrimitiveType = 201326592;
pub const SCE_GXM_REGION_CLIP_ALL: SceGxmRegionClipMode = 1073741824;
pub const SCE_GXM_REGION_CLIP_INSIDE: SceGxmRegionClipMode = 3221225472;
pub const SCE_GXM_REGION_CLIP_NONE: SceGxmRegionClipMode = 0;
pub const SCE_GXM_REGION_CLIP_OUTSIDE: SceGxmRegionClipMode = 2147483648;
pub const SCE_GXM_RENDER_TARGET_CUSTOM_MULTISAMPLE_LOCATIONS: SceGxmRenderTargetFlags = 1;
pub const SCE_GXM_SCENE_FRAGMENT_SET_DEPENDENCY: SceGxmSceneFlags = 1;
pub const SCE_GXM_SCENE_FRAGMENT_TRANSFER_SYNC: SceGxmSceneFlags = 4;
pub const SCE_GXM_SCENE_VERTEX_TRANSFER_SYNC: SceGxmSceneFlags = 8;
pub const SCE_GXM_SCENE_VERTEX_WAIT_FOR_DEPENDENCY: SceGxmSceneFlags = 2;
pub const SCE_GXM_STENCIL_FUNC_ALWAYS: SceGxmStencilFunc = 234881024;
pub const SCE_GXM_STENCIL_FUNC_EQUAL: SceGxmStencilFunc = 67108864;
pub const SCE_GXM_STENCIL_FUNC_GREATER: SceGxmStencilFunc = 134217728;
pub const SCE_GXM_STENCIL_FUNC_GREATER_EQUAL: SceGxmStencilFunc = 201326592;
pub const SCE_GXM_STENCIL_FUNC_LESS: SceGxmStencilFunc = 33554432;
pub const SCE_GXM_STENCIL_FUNC_LESS_EQUAL: SceGxmStencilFunc = 100663296;
pub const SCE_GXM_STENCIL_FUNC_NEVER: SceGxmStencilFunc = 0;
pub const SCE_GXM_STENCIL_FUNC_NOT_EQUAL: SceGxmStencilFunc = 167772160;
pub const SCE_GXM_STENCIL_OP_DECR: SceGxmStencilOp = 4;
pub const SCE_GXM_STENCIL_OP_DECR_WRAP: SceGxmStencilOp = 7;
pub const SCE_GXM_STENCIL_OP_INCR: SceGxmStencilOp = 3;
pub const SCE_GXM_STENCIL_OP_INCR_WRAP: SceGxmStencilOp = 6;
pub const SCE_GXM_STENCIL_OP_INVERT: SceGxmStencilOp = 5;
pub const SCE_GXM_STENCIL_OP_KEEP: SceGxmStencilOp = 0;
pub const SCE_GXM_STENCIL_OP_REPLACE: SceGxmStencilOp = 2;
pub const SCE_GXM_STENCIL_OP_ZERO: SceGxmStencilOp = 1;
pub const SCE_GXM_TEXTURE_ADDR_CLAMP: SceGxmTextureAddrMode = 2;
pub const SCE_GXM_TEXTURE_ADDR_CLAMP_FULL_BORDER: SceGxmTextureAddrMode = 5;
pub const SCE_GXM_TEXTURE_ADDR_CLAMP_HALF_BORDER: SceGxmTextureAddrMode = 7;
pub const SCE_GXM_TEXTURE_ADDR_CLAMP_IGNORE_BORDER: SceGxmTextureAddrMode = 6;
pub const SCE_GXM_TEXTURE_ADDR_MIRROR: SceGxmTextureAddrMode = 1;
pub const SCE_GXM_TEXTURE_ADDR_MIRROR_CLAMP: SceGxmTextureAddrMode = 3;
pub const SCE_GXM_TEXTURE_ADDR_REPEAT: SceGxmTextureAddrMode = 0;
pub const SCE_GXM_TEXTURE_ADDR_REPEAT_IGNORE_BORDER: SceGxmTextureAddrMode = 4;
pub const SCE_GXM_TEXTURE_ALIGNMENT: u32 = 16;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_F11F11F10: SceGxmTextureBaseFormat = 436207616;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_F16: SceGxmTextureBaseFormat = 184549376;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_F16F16: SceGxmTextureBaseFormat = 285212672;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_F16F16F16F16: SceGxmTextureBaseFormat = 452984832;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_F32: SceGxmTextureBaseFormat = 301989888;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_F32F32: SceGxmTextureBaseFormat = 503316480;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_F32M: SceGxmTextureBaseFormat = 318767104;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_P4: SceGxmTextureBaseFormat = 2483027968;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_P8: SceGxmTextureBaseFormat = 2499805184;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_PVRT2BPP: SceGxmTextureBaseFormat = 2147483648;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_PVRT4BPP: SceGxmTextureBaseFormat = 2164260864;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_PVRTII2BPP: SceGxmTextureBaseFormat = 2181038080;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_PVRTII4BPP: SceGxmTextureBaseFormat = 2197815296;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_S16: SceGxmTextureBaseFormat = 167772160;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_S16S16: SceGxmTextureBaseFormat = 268435456;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_S16S16S16S16: SceGxmTextureBaseFormat = 486539264;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_S32: SceGxmTextureBaseFormat = 402653184;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_S5S5U6: SceGxmTextureBaseFormat = 100663296;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_S8: SceGxmTextureBaseFormat = 16777216;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_S8S8: SceGxmTextureBaseFormat = 134217728;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_S8S8S8: SceGxmTextureBaseFormat = 2566914048;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_S8S8S8S8: SceGxmTextureBaseFormat = 218103808;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_SBC4: SceGxmTextureBaseFormat = 2298478592;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_SBC5: SceGxmTextureBaseFormat = 2332033024;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_SE5M9M9M9: SceGxmTextureBaseFormat = 419430400;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U16: SceGxmTextureBaseFormat = 150994944;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U16U16: SceGxmTextureBaseFormat = 251658240;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U16U16U16U16: SceGxmTextureBaseFormat = 469762048;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U1U5U5U5: SceGxmTextureBaseFormat = 67108864;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U2F10F10F10: SceGxmTextureBaseFormat = 2583691264;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U2U10U10U10: SceGxmTextureBaseFormat = 234881024;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U32: SceGxmTextureBaseFormat = 385875968;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U32U32: SceGxmTextureBaseFormat = 520093696;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U4U4U4U4: SceGxmTextureBaseFormat = 33554432;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U5U6U5: SceGxmTextureBaseFormat = 83886080;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U8: SceGxmTextureBaseFormat = 0;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U8U3U3U2: SceGxmTextureBaseFormat = 50331648;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U8U8: SceGxmTextureBaseFormat = 117440512;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U8U8U8: SceGxmTextureBaseFormat = 2550136832;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_U8U8U8U8: SceGxmTextureBaseFormat = 201326592;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_UBC1: SceGxmTextureBaseFormat = 2231369728;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_UBC2: SceGxmTextureBaseFormat = 2248146944;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_UBC3: SceGxmTextureBaseFormat = 2264924160;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_UBC4: SceGxmTextureBaseFormat = 2281701376;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_UBC5: SceGxmTextureBaseFormat = 2315255808;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_X8S8S8U8: SceGxmTextureBaseFormat = 335544320;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_X8U24: SceGxmTextureBaseFormat = 352321536;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_YUV420P2: SceGxmTextureBaseFormat = 2415919104;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_YUV420P3: SceGxmTextureBaseFormat = 2432696320;
pub const SCE_GXM_TEXTURE_BASE_FORMAT_YUV422: SceGxmTextureBaseFormat = 2449473536;
pub const SCE_GXM_TEXTURE_CUBE: SceGxmTextureType = 1073741824;
pub const SCE_GXM_TEXTURE_CUBE_ARBITRARY: SceGxmTextureType = 3758096384;
pub const SCE_GXM_TEXTURE_FILTER_LINEAR: SceGxmTextureFilter = 1;
pub const SCE_GXM_TEXTURE_FILTER_MIPMAP_LINEAR: SceGxmTextureFilter = 2;
pub const SCE_GXM_TEXTURE_FILTER_MIPMAP_POINT: SceGxmTextureFilter = 3;
pub const SCE_GXM_TEXTURE_FILTER_POINT: SceGxmTextureFilter = 0;
pub const SCE_GXM_TEXTURE_FORMAT_A16: SceGxmTextureFormat = 151019520;
pub const SCE_GXM_TEXTURE_FORMAT_A1R5G5B5: SceGxmTextureFormat = 67112960;
pub const SCE_GXM_TEXTURE_FORMAT_A4R4G4B4: SceGxmTextureFormat = 33558528;
pub const SCE_GXM_TEXTURE_FORMAT_A8: SceGxmTextureFormat = 24576;
pub const SCE_GXM_TEXTURE_FORMAT_A8B8G8R8: SceGxmTextureFormat = 201326592;
pub const SCE_GXM_TEXTURE_FORMAT_A8L8: SceGxmTextureFormat = 117448704;
pub const SCE_GXM_TEXTURE_FORMAT_A8R8G8B8: SceGxmTextureFormat = 201330688;
pub const SCE_GXM_TEXTURE_FORMAT_AF16: SceGxmTextureFormat = 184573952;
pub const SCE_GXM_TEXTURE_FORMAT_AF16LF16: SceGxmTextureFormat = 285220864;
pub const SCE_GXM_TEXTURE_FORMAT_AF32M: SceGxmTextureFormat = 318791680;
pub const SCE_GXM_TEXTURE_FORMAT_D16: SceGxmTextureFormat = 150994944;
pub const SCE_GXM_TEXTURE_FORMAT_DF32M: SceGxmTextureFormat = 318767104;
pub const SCE_GXM_TEXTURE_FORMAT_F10F10F10U2_BGRA: SceGxmTextureFormat = 2583703552;
pub const SCE_GXM_TEXTURE_FORMAT_F10F10F10U2_RGBA: SceGxmTextureFormat = 2583699456;
pub const SCE_GXM_TEXTURE_FORMAT_F10F10F10X2_BGR1: SceGxmTextureFormat = 2583719936;
pub const SCE_GXM_TEXTURE_FORMAT_F10F10F10X2_RGB1: SceGxmTextureFormat = 2583715840;
pub const SCE_GXM_TEXTURE_FORMAT_F10F11F11_BGR: SceGxmTextureFormat = 436207616;
pub const SCE_GXM_TEXTURE_FORMAT_F11F11F10_RGB: SceGxmTextureFormat = 436211712;
pub const SCE_GXM_TEXTURE_FORMAT_F16_000R: SceGxmTextureFormat = 184553472;
pub const SCE_GXM_TEXTURE_FORMAT_F16_0RRR: SceGxmTextureFormat = 184565760;
pub const SCE_GXM_TEXTURE_FORMAT_F16_111R: SceGxmTextureFormat = 184557568;
pub const SCE_GXM_TEXTURE_FORMAT_F16_1RRR: SceGxmTextureFormat = 184569856;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16_00GR: SceGxmTextureFormat = 285216768;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16_00RG: SceGxmTextureFormat = 285233152;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16F16F16_ABGR: SceGxmTextureFormat = 452984832;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16F16F16_ARGB: SceGxmTextureFormat = 452988928;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16F16F16_BGRA: SceGxmTextureFormat = 452997120;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16F16F16_RGBA: SceGxmTextureFormat = 452993024;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16F16X16_BGR1: SceGxmTextureFormat = 453013504;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16F16X16_RGB1: SceGxmTextureFormat = 453009408;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16_GR: SceGxmTextureFormat = 285212672;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16_GRGR: SceGxmTextureFormat = 285229056;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16_GRRR: SceGxmTextureFormat = 285220864;
pub const SCE_GXM_TEXTURE_FORMAT_F16F16_RGGG: SceGxmTextureFormat = 285224960;
pub const SCE_GXM_TEXTURE_FORMAT_F16_R: SceGxmTextureFormat = 184549376;
pub const SCE_GXM_TEXTURE_FORMAT_F16_R000: SceGxmTextureFormat = 184573952;
pub const SCE_GXM_TEXTURE_FORMAT_F16_R111: SceGxmTextureFormat = 184578048;
pub const SCE_GXM_TEXTURE_FORMAT_F16_RRRR: SceGxmTextureFormat = 184561664;
pub const SCE_GXM_TEXTURE_FORMAT_F32_000R: SceGxmTextureFormat = 301993984;
pub const SCE_GXM_TEXTURE_FORMAT_F32_0RRR: SceGxmTextureFormat = 302006272;
pub const SCE_GXM_TEXTURE_FORMAT_F32_111R: SceGxmTextureFormat = 301998080;
pub const SCE_GXM_TEXTURE_FORMAT_F32_1RRR: SceGxmTextureFormat = 302010368;
pub const SCE_GXM_TEXTURE_FORMAT_F32F32_00GR: SceGxmTextureFormat = 503320576;
pub const SCE_GXM_TEXTURE_FORMAT_F32F32_00RG: SceGxmTextureFormat = 503336960;
pub const SCE_GXM_TEXTURE_FORMAT_F32F32_GR: SceGxmTextureFormat = 503316480;
pub const SCE_GXM_TEXTURE_FORMAT_F32F32_GRGR: SceGxmTextureFormat = 503332864;
pub const SCE_GXM_TEXTURE_FORMAT_F32F32_GRRR: SceGxmTextureFormat = 503324672;
pub const SCE_GXM_TEXTURE_FORMAT_F32F32_RGGG: SceGxmTextureFormat = 503328768;
pub const SCE_GXM_TEXTURE_FORMAT_F32M_000R: SceGxmTextureFormat = 318771200;
pub const SCE_GXM_TEXTURE_FORMAT_F32M_0RRR: SceGxmTextureFormat = 318783488;
pub const SCE_GXM_TEXTURE_FORMAT_F32M_111R: SceGxmTextureFormat = 318775296;
pub const SCE_GXM_TEXTURE_FORMAT_F32M_1RRR: SceGxmTextureFormat = 318787584;
pub const SCE_GXM_TEXTURE_FORMAT_F32M_R: SceGxmTextureFormat = 318767104;
pub const SCE_GXM_TEXTURE_FORMAT_F32M_R000: SceGxmTextureFormat = 318791680;
pub const SCE_GXM_TEXTURE_FORMAT_F32M_R111: SceGxmTextureFormat = 318795776;
pub const SCE_GXM_TEXTURE_FORMAT_F32M_RRRR: SceGxmTextureFormat = 318779392;
pub const SCE_GXM_TEXTURE_FORMAT_F32_R: SceGxmTextureFormat = 301989888;
pub const SCE_GXM_TEXTURE_FORMAT_F32_R000: SceGxmTextureFormat = 302014464;
pub const SCE_GXM_TEXTURE_FORMAT_F32_R111: SceGxmTextureFormat = 302018560;
pub const SCE_GXM_TEXTURE_FORMAT_F32_RRRR: SceGxmTextureFormat = 302002176;
pub const SCE_GXM_TEXTURE_FORMAT_G8R8: SceGxmTextureFormat = 117444608;
pub const SCE_GXM_TEXTURE_FORMAT_GF16RF16: SceGxmTextureFormat = 285216768;
pub const SCE_GXM_TEXTURE_FORMAT_L16: SceGxmTextureFormat = 151015424;
pub const SCE_GXM_TEXTURE_FORMAT_L8: SceGxmTextureFormat = 20480;
pub const SCE_GXM_TEXTURE_FORMAT_L8A8: SceGxmTextureFormat = 117452800;
pub const SCE_GXM_TEXTURE_FORMAT_LF16: SceGxmTextureFormat = 184569856;
pub const SCE_GXM_TEXTURE_FORMAT_LF16AF16: SceGxmTextureFormat = 285224960;
pub const SCE_GXM_TEXTURE_FORMAT_LF32M: SceGxmTextureFormat = 318787584;
pub const SCE_GXM_TEXTURE_FORMAT_P4_1BGR: SceGxmTextureFormat = 2483044352;
pub const SCE_GXM_TEXTURE_FORMAT_P4_1RGB: SceGxmTextureFormat = 2483048448;
pub const SCE_GXM_TEXTURE_FORMAT_P4_ABGR: SceGxmTextureFormat = 2483027968;
pub const SCE_GXM_TEXTURE_FORMAT_P4_ARGB: SceGxmTextureFormat = 2483032064;
pub const SCE_GXM_TEXTURE_FORMAT_P4_BGR1: SceGxmTextureFormat = 2483056640;
pub const SCE_GXM_TEXTURE_FORMAT_P4_BGRA: SceGxmTextureFormat = 2483040256;
pub const SCE_GXM_TEXTURE_FORMAT_P4_RGB1: SceGxmTextureFormat = 2483052544;
pub const SCE_GXM_TEXTURE_FORMAT_P4_RGBA: SceGxmTextureFormat = 2483036160;
pub const SCE_GXM_TEXTURE_FORMAT_P8_1BGR: SceGxmTextureFormat = 2499821568;
pub const SCE_GXM_TEXTURE_FORMAT_P8_1RGB: SceGxmTextureFormat = 2499825664;
pub const SCE_GXM_TEXTURE_FORMAT_P8_ABGR: SceGxmTextureFormat = 2499805184;
pub const SCE_GXM_TEXTURE_FORMAT_P8_ARGB: SceGxmTextureFormat = 2499809280;
pub const SCE_GXM_TEXTURE_FORMAT_P8_BGR1: SceGxmTextureFormat = 2499833856;
pub const SCE_GXM_TEXTURE_FORMAT_P8_BGRA: SceGxmTextureFormat = 2499817472;
pub const SCE_GXM_TEXTURE_FORMAT_P8_RGB1: SceGxmTextureFormat = 2499829760;
pub const SCE_GXM_TEXTURE_FORMAT_P8_RGBA: SceGxmTextureFormat = 2499813376;
pub const SCE_GXM_TEXTURE_FORMAT_PVRT2BPP: SceGxmTextureFormat = 2147483648;
pub const SCE_GXM_TEXTURE_FORMAT_PVRT2BPP_1BGR: SceGxmTextureFormat = 2147500032;
pub const SCE_GXM_TEXTURE_FORMAT_PVRT2BPP_ABGR: SceGxmTextureFormat = 2147483648;
pub const SCE_GXM_TEXTURE_FORMAT_PVRT4BPP: SceGxmTextureFormat = 2164260864;
pub const SCE_GXM_TEXTURE_FORMAT_PVRT4BPP_1BGR: SceGxmTextureFormat = 2164277248;
pub const SCE_GXM_TEXTURE_FORMAT_PVRT4BPP_ABGR: SceGxmTextureFormat = 2164260864;
pub const SCE_GXM_TEXTURE_FORMAT_PVRTII2BPP: SceGxmTextureFormat = 2181038080;
pub const SCE_GXM_TEXTURE_FORMAT_PVRTII2BPP_1BGR: SceGxmTextureFormat = 2181054464;
pub const SCE_GXM_TEXTURE_FORMAT_PVRTII2BPP_ABGR: SceGxmTextureFormat = 2181038080;
pub const SCE_GXM_TEXTURE_FORMAT_PVRTII4BPP: SceGxmTextureFormat = 2197815296;
pub const SCE_GXM_TEXTURE_FORMAT_PVRTII4BPP_1BGR: SceGxmTextureFormat = 2197831680;
pub const SCE_GXM_TEXTURE_FORMAT_PVRTII4BPP_ABGR: SceGxmTextureFormat = 2197815296;
pub const SCE_GXM_TEXTURE_FORMAT_R16: SceGxmTextureFormat = 150999040;
pub const SCE_GXM_TEXTURE_FORMAT_R5G6B5: SceGxmTextureFormat = 83890176;
pub const SCE_GXM_TEXTURE_FORMAT_R8: SceGxmTextureFormat = 4096;
pub const SCE_GXM_TEXTURE_FORMAT_RF16: SceGxmTextureFormat = 184553472;
pub const SCE_GXM_TEXTURE_FORMAT_RF32M: SceGxmTextureFormat = 318771200;
pub const SCE_GXM_TEXTURE_FORMAT_S16_000R: SceGxmTextureFormat = 167776256;
pub const SCE_GXM_TEXTURE_FORMAT_S16_0RRR: SceGxmTextureFormat = 167788544;
pub const SCE_GXM_TEXTURE_FORMAT_S16_111R: SceGxmTextureFormat = 167780352;
pub const SCE_GXM_TEXTURE_FORMAT_S16_1RRR: SceGxmTextureFormat = 167792640;
pub const SCE_GXM_TEXTURE_FORMAT_S16_R: SceGxmTextureFormat = 167772160;
pub const SCE_GXM_TEXTURE_FORMAT_S16_R000: SceGxmTextureFormat = 167796736;
pub const SCE_GXM_TEXTURE_FORMAT_S16_R111: SceGxmTextureFormat = 167800832;
pub const SCE_GXM_TEXTURE_FORMAT_S16_RRRR: SceGxmTextureFormat = 167784448;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16_00GR: SceGxmTextureFormat = 268439552;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16_00RG: SceGxmTextureFormat = 268455936;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16_GR: SceGxmTextureFormat = 268435456;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16_GRGR: SceGxmTextureFormat = 268451840;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16_GRRR: SceGxmTextureFormat = 268443648;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16_RGGG: SceGxmTextureFormat = 268447744;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16S16S16_ABGR: SceGxmTextureFormat = 486539264;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16S16S16_ARGB: SceGxmTextureFormat = 486543360;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16S16S16_BGRA: SceGxmTextureFormat = 486551552;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16S16S16_RGBA: SceGxmTextureFormat = 486547456;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16S16X16_BGR1: SceGxmTextureFormat = 486567936;
pub const SCE_GXM_TEXTURE_FORMAT_S16S16S16X16_RGB1: SceGxmTextureFormat = 486563840;
pub const SCE_GXM_TEXTURE_FORMAT_S32_000R: SceGxmTextureFormat = 402657280;
pub const SCE_GXM_TEXTURE_FORMAT_S32_0RRR: SceGxmTextureFormat = 402669568;
pub const SCE_GXM_TEXTURE_FORMAT_S32_111R: SceGxmTextureFormat = 402661376;
pub const SCE_GXM_TEXTURE_FORMAT_S32_1RRR: SceGxmTextureFormat = 402673664;
pub const SCE_GXM_TEXTURE_FORMAT_S32_R: SceGxmTextureFormat = 402653184;
pub const SCE_GXM_TEXTURE_FORMAT_S32_R000: SceGxmTextureFormat = 402677760;
pub const SCE_GXM_TEXTURE_FORMAT_S32_R111: SceGxmTextureFormat = 402681856;
pub const SCE_GXM_TEXTURE_FORMAT_S32_RRRR: SceGxmTextureFormat = 402665472;
pub const SCE_GXM_TEXTURE_FORMAT_S5S5U6_RGB: SceGxmTextureFormat = 100667392;
pub const SCE_GXM_TEXTURE_FORMAT_S8_000R: SceGxmTextureFormat = 16781312;
pub const SCE_GXM_TEXTURE_FORMAT_S8_0RRR: SceGxmTextureFormat = 16793600;
pub const SCE_GXM_TEXTURE_FORMAT_S8_111R: SceGxmTextureFormat = 16785408;
pub const SCE_GXM_TEXTURE_FORMAT_S8_1RRR: SceGxmTextureFormat = 16797696;
pub const SCE_GXM_TEXTURE_FORMAT_S8_R: SceGxmTextureFormat = 16777216;
pub const SCE_GXM_TEXTURE_FORMAT_S8_R000: SceGxmTextureFormat = 16801792;
pub const SCE_GXM_TEXTURE_FORMAT_S8_R111: SceGxmTextureFormat = 16805888;
pub const SCE_GXM_TEXTURE_FORMAT_S8_RRRR: SceGxmTextureFormat = 16789504;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8_00GR: SceGxmTextureFormat = 134221824;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8_00RG: SceGxmTextureFormat = 134238208;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8_GR: SceGxmTextureFormat = 134217728;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8_GRGR: SceGxmTextureFormat = 134234112;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8_GRRR: SceGxmTextureFormat = 134225920;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8_RGGG: SceGxmTextureFormat = 134230016;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8S8_BGR: SceGxmTextureFormat = 2566914048;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8S8_RGB: SceGxmTextureFormat = 2566918144;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8S8S8_ABGR: SceGxmTextureFormat = 218103808;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8S8S8_ARGB: SceGxmTextureFormat = 218107904;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8S8S8_BGRA: SceGxmTextureFormat = 218116096;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8S8S8_RGBA: SceGxmTextureFormat = 218112000;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8S8X8_BGR1: SceGxmTextureFormat = 218132480;
pub const SCE_GXM_TEXTURE_FORMAT_S8S8S8X8_RGB1: SceGxmTextureFormat = 218128384;
pub const SCE_GXM_TEXTURE_FORMAT_SBC4_000R: SceGxmTextureFormat = 2298482688;
pub const SCE_GXM_TEXTURE_FORMAT_SBC4_0RRR: SceGxmTextureFormat = 2298494976;
pub const SCE_GXM_TEXTURE_FORMAT_SBC4_111R: SceGxmTextureFormat = 2298486784;
pub const SCE_GXM_TEXTURE_FORMAT_SBC4_1RRR: SceGxmTextureFormat = 2298499072;
pub const SCE_GXM_TEXTURE_FORMAT_SBC4_R: SceGxmTextureFormat = 2298478592;
pub const SCE_GXM_TEXTURE_FORMAT_SBC4_R000: SceGxmTextureFormat = 2298503168;
pub const SCE_GXM_TEXTURE_FORMAT_SBC4_R111: SceGxmTextureFormat = 2298507264;
pub const SCE_GXM_TEXTURE_FORMAT_SBC4_RRRR: SceGxmTextureFormat = 2298490880;
pub const SCE_GXM_TEXTURE_FORMAT_SBC5_00GR: SceGxmTextureFormat = 2332037120;
pub const SCE_GXM_TEXTURE_FORMAT_SBC5_00RG: SceGxmTextureFormat = 2332053504;
pub const SCE_GXM_TEXTURE_FORMAT_SBC5_GR: SceGxmTextureFormat = 2332033024;
pub const SCE_GXM_TEXTURE_FORMAT_SBC5_GRGR: SceGxmTextureFormat = 2332049408;
pub const SCE_GXM_TEXTURE_FORMAT_SBC5_GRRR: SceGxmTextureFormat = 2332041216;
pub const SCE_GXM_TEXTURE_FORMAT_SBC5_RGGG: SceGxmTextureFormat = 2332045312;
pub const SCE_GXM_TEXTURE_FORMAT_SE5M9M9M9_BGR: SceGxmTextureFormat = 419430400;
pub const SCE_GXM_TEXTURE_FORMAT_SE5M9M9M9_RGB: SceGxmTextureFormat = 419434496;
pub const SCE_GXM_TEXTURE_FORMAT_U10U10U10U2_BGRA: SceGxmTextureFormat = 234893312;
pub const SCE_GXM_TEXTURE_FORMAT_U10U10U10U2_RGBA: SceGxmTextureFormat = 234889216;
pub const SCE_GXM_TEXTURE_FORMAT_U10U10U10X2_BGR1: SceGxmTextureFormat = 234909696;
pub const SCE_GXM_TEXTURE_FORMAT_U10U10U10X2_RGB1: SceGxmTextureFormat = 234905600;
pub const SCE_GXM_TEXTURE_FORMAT_U16_000R: SceGxmTextureFormat = 150999040;
pub const SCE_GXM_TEXTURE_FORMAT_U16_0RRR: SceGxmTextureFormat = 151011328;
pub const SCE_GXM_TEXTURE_FORMAT_U16_111R: SceGxmTextureFormat = 151003136;
pub const SCE_GXM_TEXTURE_FORMAT_U16_1RRR: SceGxmTextureFormat = 151015424;
pub const SCE_GXM_TEXTURE_FORMAT_U16_R: SceGxmTextureFormat = 150994944;
pub const SCE_GXM_TEXTURE_FORMAT_U16_R000: SceGxmTextureFormat = 151019520;
pub const SCE_GXM_TEXTURE_FORMAT_U16_R111: SceGxmTextureFormat = 151023616;
pub const SCE_GXM_TEXTURE_FORMAT_U16_RRRR: SceGxmTextureFormat = 151007232;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16_00GR: SceGxmTextureFormat = 251662336;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16_00RG: SceGxmTextureFormat = 251678720;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16_GR: SceGxmTextureFormat = 251658240;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16_GRGR: SceGxmTextureFormat = 251674624;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16_GRRR: SceGxmTextureFormat = 251666432;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16_RGGG: SceGxmTextureFormat = 251670528;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16U16U16_ABGR: SceGxmTextureFormat = 469762048;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16U16U16_ARGB: SceGxmTextureFormat = 469766144;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16U16U16_BGRA: SceGxmTextureFormat = 469774336;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16U16U16_RGBA: SceGxmTextureFormat = 469770240;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16U16X16_BGR1: SceGxmTextureFormat = 469790720;
pub const SCE_GXM_TEXTURE_FORMAT_U16U16U16X16_RGB1: SceGxmTextureFormat = 469786624;
pub const SCE_GXM_TEXTURE_FORMAT_U1U5U5U5_ABGR: SceGxmTextureFormat = 67108864;
pub const SCE_GXM_TEXTURE_FORMAT_U1U5U5U5_ARGB: SceGxmTextureFormat = 67112960;
pub const SCE_GXM_TEXTURE_FORMAT_U24X8_DS: SceGxmTextureFormat = 352325632;
pub const SCE_GXM_TEXTURE_FORMAT_U2F10F10F10_ABGR: SceGxmTextureFormat = 2583691264;
pub const SCE_GXM_TEXTURE_FORMAT_U2F10F10F10_ARGB: SceGxmTextureFormat = 2583695360;
pub const SCE_GXM_TEXTURE_FORMAT_U2U10U10U10_ABGR: SceGxmTextureFormat = 234881024;
pub const SCE_GXM_TEXTURE_FORMAT_U2U10U10U10_ARGB: SceGxmTextureFormat = 234885120;
pub const SCE_GXM_TEXTURE_FORMAT_U32_000R: SceGxmTextureFormat = 385880064;
pub const SCE_GXM_TEXTURE_FORMAT_U32_0RRR: SceGxmTextureFormat = 385892352;
pub const SCE_GXM_TEXTURE_FORMAT_U32_111R: SceGxmTextureFormat = 385884160;
pub const SCE_GXM_TEXTURE_FORMAT_U32_1RRR: SceGxmTextureFormat = 385896448;
pub const SCE_GXM_TEXTURE_FORMAT_U32_R: SceGxmTextureFormat = 385875968;
pub const SCE_GXM_TEXTURE_FORMAT_U32_R000: SceGxmTextureFormat = 385900544;
pub const SCE_GXM_TEXTURE_FORMAT_U32_R111: SceGxmTextureFormat = 385904640;
pub const SCE_GXM_TEXTURE_FORMAT_U32_RRRR: SceGxmTextureFormat = 385888256;
pub const SCE_GXM_TEXTURE_FORMAT_U32U32_00GR: SceGxmTextureFormat = 520097792;
pub const SCE_GXM_TEXTURE_FORMAT_U32U32_00RG: SceGxmTextureFormat = 520114176;
pub const SCE_GXM_TEXTURE_FORMAT_U32U32_GR: SceGxmTextureFormat = 520093696;
pub const SCE_GXM_TEXTURE_FORMAT_U32U32_GRGR: SceGxmTextureFormat = 520110080;
pub const SCE_GXM_TEXTURE_FORMAT_U32U32_GRRR: SceGxmTextureFormat = 520101888;
pub const SCE_GXM_TEXTURE_FORMAT_U32U32_RGGG: SceGxmTextureFormat = 520105984;
pub const SCE_GXM_TEXTURE_FORMAT_U4U4U4U4_ABGR: SceGxmTextureFormat = 33554432;
pub const SCE_GXM_TEXTURE_FORMAT_U4U4U4U4_ARGB: SceGxmTextureFormat = 33558528;
pub const SCE_GXM_TEXTURE_FORMAT_U4U4U4U4_BGRA: SceGxmTextureFormat = 33566720;
pub const SCE_GXM_TEXTURE_FORMAT_U4U4U4U4_RGBA: SceGxmTextureFormat = 33562624;
pub const SCE_GXM_TEXTURE_FORMAT_U4U4U4X4_BGR1: SceGxmTextureFormat = 33583104;
pub const SCE_GXM_TEXTURE_FORMAT_U4U4U4X4_RGB1: SceGxmTextureFormat = 33579008;
pub const SCE_GXM_TEXTURE_FORMAT_U5U5U5U1_BGRA: SceGxmTextureFormat = 67121152;
pub const SCE_GXM_TEXTURE_FORMAT_U5U5U5U1_RGBA: SceGxmTextureFormat = 67117056;
pub const SCE_GXM_TEXTURE_FORMAT_U5U5U5X1_BGR1: SceGxmTextureFormat = 67137536;
pub const SCE_GXM_TEXTURE_FORMAT_U5U5U5X1_RGB1: SceGxmTextureFormat = 67133440;
pub const SCE_GXM_TEXTURE_FORMAT_U5U6U5_BGR: SceGxmTextureFormat = 83886080;
pub const SCE_GXM_TEXTURE_FORMAT_U5U6U5_RGB: SceGxmTextureFormat = 83890176;
pub const SCE_GXM_TEXTURE_FORMAT_U6S5S5_BGR: SceGxmTextureFormat = 100663296;
pub const SCE_GXM_TEXTURE_FORMAT_U8_000R: SceGxmTextureFormat = 4096;
pub const SCE_GXM_TEXTURE_FORMAT_U8_0RRR: SceGxmTextureFormat = 16384;
pub const SCE_GXM_TEXTURE_FORMAT_U8_111R: SceGxmTextureFormat = 8192;
pub const SCE_GXM_TEXTURE_FORMAT_U8_1RRR: SceGxmTextureFormat = 20480;
pub const SCE_GXM_TEXTURE_FORMAT_U8_R: SceGxmTextureFormat = 0;
pub const SCE_GXM_TEXTURE_FORMAT_U8_R000: SceGxmTextureFormat = 24576;
pub const SCE_GXM_TEXTURE_FORMAT_U8_R111: SceGxmTextureFormat = 28672;
pub const SCE_GXM_TEXTURE_FORMAT_U8_RRRR: SceGxmTextureFormat = 12288;
pub const SCE_GXM_TEXTURE_FORMAT_U8U3U3U2_ARGB: SceGxmTextureFormat = 50331648;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8_00GR: SceGxmTextureFormat = 117444608;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8_00RG: SceGxmTextureFormat = 117460992;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8_GR: SceGxmTextureFormat = 117440512;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8_GRGR: SceGxmTextureFormat = 117456896;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8_GRRR: SceGxmTextureFormat = 117448704;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8_RGGG: SceGxmTextureFormat = 117452800;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8U8_BGR: SceGxmTextureFormat = 2550136832;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8U8_RGB: SceGxmTextureFormat = 2550140928;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8U8U8_ABGR: SceGxmTextureFormat = 201326592;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8U8U8_ARGB: SceGxmTextureFormat = 201330688;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8U8U8_BGRA: SceGxmTextureFormat = 201338880;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8U8U8_RGBA: SceGxmTextureFormat = 201334784;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8U8X8_BGR1: SceGxmTextureFormat = 201355264;
pub const SCE_GXM_TEXTURE_FORMAT_U8U8U8X8_RGB1: SceGxmTextureFormat = 201351168;
pub const SCE_GXM_TEXTURE_FORMAT_UBC1: SceGxmTextureFormat = 2231369728;
pub const SCE_GXM_TEXTURE_FORMAT_UBC1_1BGR: SceGxmTextureFormat = 2231386112;
pub const SCE_GXM_TEXTURE_FORMAT_UBC1_ABGR: SceGxmTextureFormat = 2231369728;
pub const SCE_GXM_TEXTURE_FORMAT_UBC2: SceGxmTextureFormat = 2248146944;
pub const SCE_GXM_TEXTURE_FORMAT_UBC2_1BGR: SceGxmTextureFormat = 2248163328;
pub const SCE_GXM_TEXTURE_FORMAT_UBC2_ABGR: SceGxmTextureFormat = 2248146944;
pub const SCE_GXM_TEXTURE_FORMAT_UBC3: SceGxmTextureFormat = 2264924160;
pub const SCE_GXM_TEXTURE_FORMAT_UBC3_1BGR: SceGxmTextureFormat = 2264940544;
pub const SCE_GXM_TEXTURE_FORMAT_UBC3_ABGR: SceGxmTextureFormat = 2264924160;
pub const SCE_GXM_TEXTURE_FORMAT_UBC4_000R: SceGxmTextureFormat = 2281705472;
pub const SCE_GXM_TEXTURE_FORMAT_UBC4_0RRR: SceGxmTextureFormat = 2281717760;
pub const SCE_GXM_TEXTURE_FORMAT_UBC4_111R: SceGxmTextureFormat = 2281709568;
pub const SCE_GXM_TEXTURE_FORMAT_UBC4_1RRR: SceGxmTextureFormat = 2281721856;
pub const SCE_GXM_TEXTURE_FORMAT_UBC4_R: SceGxmTextureFormat = 2281701376;
pub const SCE_GXM_TEXTURE_FORMAT_UBC4_R000: SceGxmTextureFormat = 2281725952;
pub const SCE_GXM_TEXTURE_FORMAT_UBC4_R111: SceGxmTextureFormat = 2281730048;
pub const SCE_GXM_TEXTURE_FORMAT_UBC4_RRRR: SceGxmTextureFormat = 2281713664;
pub const SCE_GXM_TEXTURE_FORMAT_UBC5_00GR: SceGxmTextureFormat = 2315259904;
pub const SCE_GXM_TEXTURE_FORMAT_UBC5_00RG: SceGxmTextureFormat = 2315276288;
pub const SCE_GXM_TEXTURE_FORMAT_UBC5_GR: SceGxmTextureFormat = 2315255808;
pub const SCE_GXM_TEXTURE_FORMAT_UBC5_GRGR: SceGxmTextureFormat = 2315272192;
pub const SCE_GXM_TEXTURE_FORMAT_UBC5_GRRR: SceGxmTextureFormat = 2315264000;
pub const SCE_GXM_TEXTURE_FORMAT_UBC5_RGGG: SceGxmTextureFormat = 2315268096;
pub const SCE_GXM_TEXTURE_FORMAT_UYVY422_CSC0: SceGxmTextureFormat = 2449481728;
pub const SCE_GXM_TEXTURE_FORMAT_UYVY422_CSC1: SceGxmTextureFormat = 2449498112;
pub const SCE_GXM_TEXTURE_FORMAT_VYUY: SceGxmTextureFormat = 2449485824;
pub const SCE_GXM_TEXTURE_FORMAT_VYUY422_CSC0: SceGxmTextureFormat = 2449485824;
pub const SCE_GXM_TEXTURE_FORMAT_VYUY422_CSC1: SceGxmTextureFormat = 2449502208;
pub const SCE_GXM_TEXTURE_FORMAT_X16F16F16F16_1BGR: SceGxmTextureFormat = 453001216;
pub const SCE_GXM_TEXTURE_FORMAT_X16F16F16F16_1RGB: SceGxmTextureFormat = 453005312;
pub const SCE_GXM_TEXTURE_FORMAT_X16S16S16S16_1BGR: SceGxmTextureFormat = 486555648;
pub const SCE_GXM_TEXTURE_FORMAT_X16S16S16S16_1RGB: SceGxmTextureFormat = 486559744;
pub const SCE_GXM_TEXTURE_FORMAT_X16U16U16U16_1BGR: SceGxmTextureFormat = 469778432;
pub const SCE_GXM_TEXTURE_FORMAT_X16U16U16U16_1RGB: SceGxmTextureFormat = 469782528;
pub const SCE_GXM_TEXTURE_FORMAT_X1U5U5U5_1BGR: SceGxmTextureFormat = 67125248;
pub const SCE_GXM_TEXTURE_FORMAT_X1U5U5U5_1RGB: SceGxmTextureFormat = 67129344;
pub const SCE_GXM_TEXTURE_FORMAT_X2F10F10F10_1BGR: SceGxmTextureFormat = 2583707648;
pub const SCE_GXM_TEXTURE_FORMAT_X2F10F10F10_1RGB: SceGxmTextureFormat = 2583711744;
pub const SCE_GXM_TEXTURE_FORMAT_X2U10U10U10_1BGR: SceGxmTextureFormat = 234897408;
pub const SCE_GXM_TEXTURE_FORMAT_X2U10U10U10_1RGB: SceGxmTextureFormat = 234901504;
pub const SCE_GXM_TEXTURE_FORMAT_X4U4U4U4_1BGR: SceGxmTextureFormat = 33570816;
pub const SCE_GXM_TEXTURE_FORMAT_X4U4U4U4_1RGB: SceGxmTextureFormat = 33574912;
pub const SCE_GXM_TEXTURE_FORMAT_X8S8S8S8_1BGR: SceGxmTextureFormat = 218120192;
pub const SCE_GXM_TEXTURE_FORMAT_X8S8S8S8_1RGB: SceGxmTextureFormat = 218124288;
pub const SCE_GXM_TEXTURE_FORMAT_X8S8S8U8_1BGR: SceGxmTextureFormat = 335544320;
pub const SCE_GXM_TEXTURE_FORMAT_X8U24_SD: SceGxmTextureFormat = 352321536;
pub const SCE_GXM_TEXTURE_FORMAT_X8U8S8S8_1RGB: SceGxmTextureFormat = 335548416;
pub const SCE_GXM_TEXTURE_FORMAT_X8U8U8U8_1BGR: SceGxmTextureFormat = 201342976;
pub const SCE_GXM_TEXTURE_FORMAT_X8U8U8U8_1RGB: SceGxmTextureFormat = 201347072;
pub const SCE_GXM_TEXTURE_FORMAT_YUV420P2_CSC0: SceGxmTextureFormat = 2415919104;
pub const SCE_GXM_TEXTURE_FORMAT_YUV420P2_CSC1: SceGxmTextureFormat = 2415927296;
pub const SCE_GXM_TEXTURE_FORMAT_YUV420P3_CSC0: SceGxmTextureFormat = 2432696320;
pub const SCE_GXM_TEXTURE_FORMAT_YUV420P3_CSC1: SceGxmTextureFormat = 2432704512;
pub const SCE_GXM_TEXTURE_FORMAT_YUYV422_CSC0: SceGxmTextureFormat = 2449473536;
pub const SCE_GXM_TEXTURE_FORMAT_YUYV422_CSC1: SceGxmTextureFormat = 2449489920;
pub const SCE_GXM_TEXTURE_FORMAT_YVU420P2_CSC0: SceGxmTextureFormat = 2415923200;
pub const SCE_GXM_TEXTURE_FORMAT_YVU420P2_CSC1: SceGxmTextureFormat = 2415931392;
pub const SCE_GXM_TEXTURE_FORMAT_YVU420P3_CSC0: SceGxmTextureFormat = 2432700416;
pub const SCE_GXM_TEXTURE_FORMAT_YVU420P3_CSC1: SceGxmTextureFormat = 2432708608;
pub const SCE_GXM_TEXTURE_FORMAT_YVYU: SceGxmTextureFormat = 2449477632;
pub const SCE_GXM_TEXTURE_FORMAT_YVYU422_CSC0: SceGxmTextureFormat = 2449477632;
pub const SCE_GXM_TEXTURE_FORMAT_YVYU422_CSC1: SceGxmTextureFormat = 2449494016;
pub const SCE_GXM_TEXTURE_GAMMA_BGR: SceGxmTextureGammaMode = 134217728;
pub const SCE_GXM_TEXTURE_GAMMA_GR: SceGxmTextureGammaMode = 402653184;
pub const SCE_GXM_TEXTURE_GAMMA_NONE: SceGxmTextureGammaMode = 0;
pub const SCE_GXM_TEXTURE_GAMMA_R: SceGxmTextureGammaMode = 134217728;
pub const SCE_GXM_TEXTURE_LINEAR: SceGxmTextureType = 1610612736;
pub const SCE_GXM_TEXTURE_LINEAR_STRIDED: SceGxmTextureType = 3221225472;
pub const SCE_GXM_TEXTURE_MIP_FILTER_DISABLED: SceGxmTextureMipFilter = 0;
pub const SCE_GXM_TEXTURE_MIP_FILTER_ENABLED: SceGxmTextureMipFilter = 512;
pub const SCE_GXM_TEXTURE_NORMALIZE_DISABLED: SceGxmTextureNormalizeMode = 0;
pub const SCE_GXM_TEXTURE_NORMALIZE_ENABLED: SceGxmTextureNormalizeMode = 2147483648;
pub const SCE_GXM_TEXTURE_SWIZZLE1_000R: SceGxmTextureSwizzle1Mode = 4096;
pub const SCE_GXM_TEXTURE_SWIZZLE1_0RRR: SceGxmTextureSwizzle1Mode = 16384;
pub const SCE_GXM_TEXTURE_SWIZZLE1_111R: SceGxmTextureSwizzle1Mode = 8192;
pub const SCE_GXM_TEXTURE_SWIZZLE1_1RRR: SceGxmTextureSwizzle1Mode = 20480;
pub const SCE_GXM_TEXTURE_SWIZZLE1_R: SceGxmTextureSwizzle1Mode = 0;
pub const SCE_GXM_TEXTURE_SWIZZLE1_R000: SceGxmTextureSwizzle1Mode = 24576;
pub const SCE_GXM_TEXTURE_SWIZZLE1_R111: SceGxmTextureSwizzle1Mode = 28672;
pub const SCE_GXM_TEXTURE_SWIZZLE1_RRRR: SceGxmTextureSwizzle1Mode = 12288;
pub const SCE_GXM_TEXTURE_SWIZZLE2_00GR: SceGxmTextureSwizzle2Mode = 4096;
pub const SCE_GXM_TEXTURE_SWIZZLE2_00RG: SceGxmTextureSwizzle2Mode = 20480;
pub const SCE_GXM_TEXTURE_SWIZZLE2_DS: SceGxmTextureSwizzle2ModeAlt = 4096;
pub const SCE_GXM_TEXTURE_SWIZZLE2_GR: SceGxmTextureSwizzle2Mode = 0;
pub const SCE_GXM_TEXTURE_SWIZZLE2_GRGR: SceGxmTextureSwizzle2Mode = 16384;
pub const SCE_GXM_TEXTURE_SWIZZLE2_GRRR: SceGxmTextureSwizzle2Mode = 8192;
pub const SCE_GXM_TEXTURE_SWIZZLE2_RGGG: SceGxmTextureSwizzle2Mode = 12288;
pub const SCE_GXM_TEXTURE_SWIZZLE2_SD: SceGxmTextureSwizzle2ModeAlt = 0;
pub const SCE_GXM_TEXTURE_SWIZZLE3_BGR: SceGxmTextureSwizzle3Mode = 0;
pub const SCE_GXM_TEXTURE_SWIZZLE3_RGB: SceGxmTextureSwizzle3Mode = 4096;
pub const SCE_GXM_TEXTURE_SWIZZLE4_1BGR: SceGxmTextureSwizzle4Mode = 16384;
pub const SCE_GXM_TEXTURE_SWIZZLE4_1RGB: SceGxmTextureSwizzle4Mode = 20480;
pub const SCE_GXM_TEXTURE_SWIZZLE4_ABGR: SceGxmTextureSwizzle4Mode = 0;
pub const SCE_GXM_TEXTURE_SWIZZLE4_ARGB: SceGxmTextureSwizzle4Mode = 4096;
pub const SCE_GXM_TEXTURE_SWIZZLE4_BGR1: SceGxmTextureSwizzle4Mode = 28672;
pub const SCE_GXM_TEXTURE_SWIZZLE4_BGRA: SceGxmTextureSwizzle4Mode = 12288;
pub const SCE_GXM_TEXTURE_SWIZZLE4_RGB1: SceGxmTextureSwizzle4Mode = 24576;
pub const SCE_GXM_TEXTURE_SWIZZLE4_RGBA: SceGxmTextureSwizzle4Mode = 8192;
pub const SCE_GXM_TEXTURE_SWIZZLED: SceGxmTextureType = 0;
pub const SCE_GXM_TEXTURE_SWIZZLED_ARBITRARY: SceGxmTextureType = 2684354560;
pub const SCE_GXM_TEXTURE_SWIZZLE_UYVY_CSC0: SceGxmTextureSwizzleYUV422Mode = 8192;
pub const SCE_GXM_TEXTURE_SWIZZLE_UYVY_CSC1: SceGxmTextureSwizzleYUV422Mode = 24576;
pub const SCE_GXM_TEXTURE_SWIZZLE_VYUY_CSC0: SceGxmTextureSwizzleYUV422Mode = 12288;
pub const SCE_GXM_TEXTURE_SWIZZLE_VYUY_CSC1: SceGxmTextureSwizzleYUV422Mode = 28672;
pub const SCE_GXM_TEXTURE_SWIZZLE_YUV_CSC0: SceGxmTextureSwizzleYUV420Mode = 0;
pub const SCE_GXM_TEXTURE_SWIZZLE_YUV_CSC1: SceGxmTextureSwizzleYUV420Mode = 8192;
pub const SCE_GXM_TEXTURE_SWIZZLE_YUYV_CSC0: SceGxmTextureSwizzleYUV422Mode = 0;
pub const SCE_GXM_TEXTURE_SWIZZLE_YUYV_CSC1: SceGxmTextureSwizzleYUV422Mode = 16384;
pub const SCE_GXM_TEXTURE_SWIZZLE_YVU_CSC0: SceGxmTextureSwizzleYUV420Mode = 4096;
pub const SCE_GXM_TEXTURE_SWIZZLE_YVU_CSC1: SceGxmTextureSwizzleYUV420Mode = 12288;
pub const SCE_GXM_TEXTURE_SWIZZLE_YVYU_CSC0: SceGxmTextureSwizzleYUV422Mode = 4096;
pub const SCE_GXM_TEXTURE_SWIZZLE_YVYU_CSC1: SceGxmTextureSwizzleYUV422Mode = 20480;
pub const SCE_GXM_TEXTURE_TILED: SceGxmTextureType = 2147483648;
pub const SCE_GXM_TILE_SHIFTX: u32 = 5;
pub const SCE_GXM_TILE_SHIFTY: u32 = 5;
pub const SCE_GXM_TILE_SIZEX: u32 = 32;
pub const SCE_GXM_TILE_SIZEY: u32 = 32;
pub const SCE_GXM_TRANSFER_COLORKEY_NONE: SceGxmTransferColorKeyMode = 0;
pub const SCE_GXM_TRANSFER_COLORKEY_PASS: SceGxmTransferColorKeyMode = 1;
pub const SCE_GXM_TRANSFER_COLORKEY_REJECT: SceGxmTransferColorKeyMode = 2;
pub const SCE_GXM_TRANSFER_FORMAT_RAW128: SceGxmTransferFormat = 1245184;
pub const SCE_GXM_TRANSFER_FORMAT_RAW16: SceGxmTransferFormat = 983040;
pub const SCE_GXM_TRANSFER_FORMAT_RAW32: SceGxmTransferFormat = 1114112;
pub const SCE_GXM_TRANSFER_FORMAT_RAW64: SceGxmTransferFormat = 1179648;
pub const SCE_GXM_TRANSFER_FORMAT_U1U5U5U5_ABGR: SceGxmTransferFormat = 131072;
pub const SCE_GXM_TRANSFER_FORMAT_U2U10U10U10_ABGR: SceGxmTransferFormat = 851968;
pub const SCE_GXM_TRANSFER_FORMAT_U4U4U4U4_ABGR: SceGxmTransferFormat = 65536;
pub const SCE_GXM_TRANSFER_FORMAT_U5U6U5_BGR: SceGxmTransferFormat = 196608;
pub const SCE_GXM_TRANSFER_FORMAT_U8_R: SceGxmTransferFormat = 0;
pub const SCE_GXM_TRANSFER_FORMAT_U8U8_GR: SceGxmTransferFormat = 262144;
pub const SCE_GXM_TRANSFER_FORMAT_U8U8U8_BGR: SceGxmTransferFormat = 327680;
pub const SCE_GXM_TRANSFER_FORMAT_U8U8U8U8_ABGR: SceGxmTransferFormat = 393216;
pub const SCE_GXM_TRANSFER_FORMAT_UYVY422: SceGxmTransferFormat = 589824;
pub const SCE_GXM_TRANSFER_FORMAT_VYUY422: SceGxmTransferFormat = 458752;
pub const SCE_GXM_TRANSFER_FORMAT_YUYV422: SceGxmTransferFormat = 655360;
pub const SCE_GXM_TRANSFER_FORMAT_YVYU422: SceGxmTransferFormat = 524288;
pub const SCE_GXM_TRANSFER_FRAGMENT_SYNC: SceGxmTransferFlags = 1;
pub const SCE_GXM_TRANSFER_LINEAR: SceGxmTransferType = 0;
pub const SCE_GXM_TRANSFER_SWIZZLED: SceGxmTransferType = 8388608;
pub const SCE_GXM_TRANSFER_TILED: SceGxmTransferType = 4194304;
pub const SCE_GXM_TRANSFER_VERTEX_SYNC: SceGxmTransferFlags = 2;
pub const SCE_GXM_TWO_SIDED_DISABLED: SceGxmTwoSidedMode = 0;
pub const SCE_GXM_TWO_SIDED_ENABLED: SceGxmTwoSidedMode = 2048;
pub const SCE_GXM_VERTEX_PROGRAM: SceGxmProgramType = 0;
pub const SCE_GXM_VIEWPORT_DISABLED: SceGxmViewportMode = 65536;
pub const SCE_GXM_VIEWPORT_ENABLED: SceGxmViewportMode = 0;
pub const SCE_GXM_VISIBILITY_TEST_DISABLED: SceGxmVisibilityTestMode = 0;
pub const SCE_GXM_VISIBILITY_TEST_ENABLED: SceGxmVisibilityTestMode = 16384;
pub const SCE_GXM_VISIBILITY_TEST_OP_INCREMENT: SceGxmVisibilityTestOp = 0;
pub const SCE_GXM_VISIBILITY_TEST_OP_SET: SceGxmVisibilityTestOp = 262144;
pub const SCE_GXM_WBUFFER_DISABLED: SceGxmWBufferMode = 0;
pub const SCE_GXM_WBUFFER_ENABLED: SceGxmWBufferMode = 16384;
pub const SCE_GXM_WCLAMP_MODE_DISABLED: SceGxmWClampMode = 0;
pub const SCE_GXM_WCLAMP_MODE_ENABLED: SceGxmWClampMode = 32768;
pub const SCE_GXM_YUV_PROFILE_BT601_FULL_RANGE: SceGxmYuvProfile = 2;
pub const SCE_GXM_YUV_PROFILE_BT601_STANDARD: SceGxmYuvProfile = 0;
pub const SCE_GXM_YUV_PROFILE_BT709_FULL_RANGE: SceGxmYuvProfile = 3;
pub const SCE_GXM_YUV_PROFILE_BT709_STANDARD: SceGxmYuvProfile = 1;
pub const SCE_GXT_ERROR_INVALID_ALIGNMENT: SceGxtErrorCode = 2153578498;
pub const SCE_GXT_ERROR_INVALID_POINTER: SceGxtErrorCode = 2153578497;
pub const SCE_GXT_ERROR_INVALID_VALUE: SceGxtErrorCode = 2153578496;
pub const SCE_HID_MAX_DEVICE_COUNT: u32 = 8;
pub const SCE_HID_MAX_REPORT: u32 = 16;
pub const SCE_HTTP_AUTH_BASIC: SceHttpAuthType = 0;
pub const SCE_HTTP_AUTH_DIGEST: SceHttpAuthType = 1;
pub const SCE_HTTP_AUTH_RESERVED0: SceHttpAuthType = 2;
pub const SCE_HTTP_AUTH_RESERVED1: SceHttpAuthType = 3;
pub const SCE_HTTP_AUTH_RESERVED2: SceHttpAuthType = 4;
pub const SCE_HTTP_DEFAULT_CONNECT_TIMEOUT: u32 = 30000000;
pub const SCE_HTTP_DEFAULT_RECV_BLOCK_SIZE: u32 = 1500;
pub const SCE_HTTP_DEFAULT_RECV_TIMEOUT: u32 = 120000000;
pub const SCE_HTTP_DEFAULT_REDIRECT_MAX: u32 = 6;
pub const SCE_HTTP_DEFAULT_RESOLVER_RETRY: u32 = 5;
pub const SCE_HTTP_DEFAULT_RESOLVER_TIMEOUT: u32 = 1000000;
pub const SCE_HTTP_DEFAULT_RESPONSE_HEADER_MAX: u32 = 5000;
pub const SCE_HTTP_DEFAULT_SEND_TIMEOUT: u32 = 120000000;
pub const SCE_HTTP_DEFAULT_TRY_AUTH_MAX: u32 = 6;
pub const SCE_HTTP_DISABLE: u32 = 0;
pub const SCE_HTTP_ENABLE: u32 = 1;
pub const SCE_HTTP_ERROR_ABORTED: SceHttpErrorCode = 2151878784;
pub const SCE_HTTP_ERROR_AFTER_SEND: SceHttpErrorCode = 2151878758;
pub const SCE_HTTP_ERROR_ALREADY_INITED: SceHttpErrorCode = 2151878688;
pub const SCE_HTTP_ERROR_BAD_RESPONSE: SceHttpErrorCode = 2151878756;
pub const SCE_HTTP_ERROR_BEFORE_INIT: SceHttpErrorCode = 2151878657;
pub const SCE_HTTP_ERROR_BEFORE_SEND: SceHttpErrorCode = 2151878757;
pub const SCE_HTTP_ERROR_BUSY: SceHttpErrorCode = 2151878689;
pub const SCE_HTTP_ERROR_CHUNK_ENC: SceHttpErrorCode = 2151878770;
pub const SCE_HTTP_ERROR_INVALID_ID: SceHttpErrorCode = 2151878912;
pub const SCE_HTTP_ERROR_INVALID_URL: SceHttpErrorCode = 2151886944;
pub const SCE_HTTP_ERROR_INVALID_VALUE: SceHttpErrorCode = 2151879166;
pub const SCE_HTTP_ERROR_INVALID_VERSION: SceHttpErrorCode = 2151878762;
pub const SCE_HTTP_ERROR_NETWORK: SceHttpErrorCode = 2151878755;
pub const SCE_HTTP_ERROR_NO_CONTENT_LENGTH: SceHttpErrorCode = 2151878769;
pub const SCE_HTTP_ERROR_NOT_FOUND: SceHttpErrorCode = 2151878693;
pub const SCE_HTTP_ERROR_NOT_IN_COM: SceHttpErrorCode = 2151878768;
pub const SCE_HTTP_ERROR_OUT_OF_MEMORY: SceHttpErrorCode = 2151878690;
pub const SCE_HTTP_ERROR_OUT_OF_SIZE: SceHttpErrorCode = 2151878916;
pub const SCE_HTTP_ERROR_PARSE_HTTP_INVALID_RESPONSE: SceHttpErrorCode = 2151882848;
pub const SCE_HTTP_ERROR_PARSE_HTTP_INVALID_VALUE: SceHttpErrorCode = 2151883262;
pub const SCE_HTTP_ERROR_PARSE_HTTP_NOT_FOUND: SceHttpErrorCode = 2151882789;
pub const SCE_HTTP_ERROR_READ_BY_HEAD_METHOD: SceHttpErrorCode = 2151878767;
pub const SCE_HTTP_ERROR_RESOLVER_EFORMAT: SceHttpErrorCode = 2151899141;
pub const SCE_HTTP_ERROR_RESOLVER_ENODNS: SceHttpErrorCode = 2151899138;
pub const SCE_HTTP_ERROR_RESOLVER_ENOHOST: SceHttpErrorCode = 2151899143;
pub const SCE_HTTP_ERROR_RESOLVER_ENORECORD: SceHttpErrorCode = 2151899146;
pub const SCE_HTTP_ERROR_RESOLVER_ENOSUPPORT: SceHttpErrorCode = 2151899140;
pub const SCE_HTTP_ERROR_RESOLVER_ENOTIMPLEMENTED: SceHttpErrorCode = 2151899144;
pub const SCE_HTTP_ERROR_RESOLVER_EPACKET: SceHttpErrorCode = 2151899137;
pub const SCE_HTTP_ERROR_RESOLVER_ESERVERFAILURE: SceHttpErrorCode = 2151899142;
pub const SCE_HTTP_ERROR_RESOLVER_ESERVERREFUSED: SceHttpErrorCode = 2151899145;
pub const SCE_HTTP_ERROR_RESOLVER_ETIMEDOUT: SceHttpErrorCode = 2151899139;
pub const SCE_HTTP_ERROR_SSL: SceHttpErrorCode = 2151878773;
pub const SCE_HTTP_ERROR_TIMEOUT: SceHttpErrorCode = 2151878760;
pub const SCE_HTTP_ERROR_TOO_LARGE_RESPONSE_HEADER: SceHttpErrorCode = 2151878771;
pub const SCE_HTTP_ERROR_UNKNOWN: SceHttpErrorCode = 2151878785;
pub const SCE_HTTP_ERROR_UNKNOWN_METHOD: SceHttpErrorCode = 2151878763;
pub const SCE_HTTP_ERROR_UNKNOWN_SCHEME: SceHttpErrorCode = 2151878753;
pub const SCE_HTTP_ERROR_UNKOWN_AUTH_TYPE: SceHttpErrorCode = 2151878761;
pub const SCE_HTTP_HEADER_ADD: SceHttpAddHeaderMode = 1;
pub const SCE_HTTP_HEADER_OVERWRITE: SceHttpAddHeaderMode = 0;
pub const SCE_HTTP_INVALID_ID: u32 = 0;
pub const SCE_HTTP_METHOD_CONNECT: SceHttpMethods = 7;
pub const SCE_HTTP_METHOD_DELETE: SceHttpMethods = 5;
pub const SCE_HTTP_METHOD_GET: SceHttpMethods = 0;
pub const SCE_HTTP_METHOD_HEAD: SceHttpMethods = 2;
pub const SCE_HTTP_METHOD_OPTIONS: SceHttpMethods = 3;
pub const SCE_HTTP_METHOD_POST: SceHttpMethods = 1;
pub const SCE_HTTP_METHOD_PUT: SceHttpMethods = 4;
pub const SCE_HTTP_METHOD_TRACE: SceHttpMethods = 6;
pub const SCE_HTTP_PASSWORD_MAX_SIZE: u32 = 256;
pub const SCE_HTTP_PROXY_AUTO: SceHttpProxyMode = 0;
pub const SCE_HTTP_PROXY_MANUAL: SceHttpProxyMode = 1;
pub const SCE_HTTPS_ERROR_CERT: SceHttpsErrorCode = 2151895136;
pub const SCE_HTTPS_ERROR_HANDSHAKE: SceHttpsErrorCode = 2151895137;
pub const SCE_HTTPS_ERROR_INTERNAL: SceHttpsErrorCode = 2151895139;
pub const SCE_HTTPS_ERROR_IO: SceHttpsErrorCode = 2151895138;
pub const SCE_HTTPS_ERROR_PROXY: SceHttpsErrorCode = 2151895140;
pub const SCE_HTTPS_ERROR_SSL_CN_CHECK: SceHttpsSslErrorCode = 4;
pub const SCE_HTTPS_ERROR_SSL_INTERNAL: SceHttpsSslErrorCode = 1;
pub const SCE_HTTPS_ERROR_SSL_INVALID_CERT: SceHttpsSslErrorCode = 2;
pub const SCE_HTTPS_ERROR_SSL_NOT_AFTER_CHECK: SceHttpsSslErrorCode = 8;
pub const SCE_HTTPS_ERROR_SSL_NOT_BEFORE_CHECK: SceHttpsSslErrorCode = 16;
pub const SCE_HTTPS_ERROR_SSL_UNKNOWN_CA: SceHttpsSslErrorCode = 32;
pub const SCE_HTTPS_FLAG_CLIENT_VERIFY: SceHttpsFlag = 2;
pub const SCE_HTTPS_FLAG_CN_CHECK: SceHttpsFlag = 4;
pub const SCE_HTTPS_FLAG_KNOWN_CA_CHECK: SceHttpsFlag = 32;
pub const SCE_HTTPS_FLAG_NOT_AFTER_CHECK: SceHttpsFlag = 8;
pub const SCE_HTTPS_FLAG_NOT_BEFORE_CHECK: SceHttpsFlag = 16;
pub const SCE_HTTPS_FLAG_SERVER_VERIFY: SceHttpsFlag = 1;
pub const SCE_HTTPS_SSLV2: SceHttpSslVersion = 1;
pub const SCE_HTTPS_SSLV23: SceHttpSslVersion = 0;
pub const SCE_HTTPS_SSLV3: SceHttpSslVersion = 2;
pub const SCE_HTTP_STATUS_CODE_ACCEPTED: SceHttpStatusCode = 202;
pub const SCE_HTTP_STATUS_CODE_BAD_GATEWAY: SceHttpStatusCode = 502;
pub const SCE_HTTP_STATUS_CODE_BAD_REQUEST: SceHttpStatusCode = 400;
pub const SCE_HTTP_STATUS_CODE_CONFLICT: SceHttpStatusCode = 409;
pub const SCE_HTTP_STATUS_CODE_CONTINUE: SceHttpStatusCode = 100;
pub const SCE_HTTP_STATUS_CODE_CREATED: SceHttpStatusCode = 201;
pub const SCE_HTTP_STATUS_CODE_EXPECTATION_FAILED: SceHttpStatusCode = 417;
pub const SCE_HTTP_STATUS_CODE_FAILED_DEPENDENCY: SceHttpStatusCode = 424;
pub const SCE_HTTP_STATUS_CODE_FORBIDDDEN: SceHttpStatusCode = 403;
pub const SCE_HTTP_STATUS_CODE_FOUND: SceHttpStatusCode = 302;
pub const SCE_HTTP_STATUS_CODE_GATEWAY_TIME_OUT: SceHttpStatusCode = 504;
pub const SCE_HTTP_STATUS_CODE_GONE: SceHttpStatusCode = 410;
pub const SCE_HTTP_STATUS_CODE_HTTP_VERSION_NOT_SUPPORTED: SceHttpStatusCode = 505;
pub const SCE_HTTP_STATUS_CODE_INSUFFICIENT_STORAGE: SceHttpStatusCode = 507;
pub const SCE_HTTP_STATUS_CODE_INTERNAL_SERVER_ERROR: SceHttpStatusCode = 500;
pub const SCE_HTTP_STATUS_CODE_LENGTH_REQUIRED: SceHttpStatusCode = 411;
pub const SCE_HTTP_STATUS_CODE_LOCKED: SceHttpStatusCode = 423;
pub const SCE_HTTP_STATUS_CODE_METHOD_NOT_ALLOWED: SceHttpStatusCode = 405;
pub const SCE_HTTP_STATUS_CODE_MOVED_PERMANENTLY: SceHttpStatusCode = 301;
pub const SCE_HTTP_STATUS_CODE_MULTIPLE_CHOICES: SceHttpStatusCode = 300;
pub const SCE_HTTP_STATUS_CODE_MULTI_STATUS: SceHttpStatusCode = 207;
pub const SCE_HTTP_STATUS_CODE_NO_CONTENT: SceHttpStatusCode = 204;
pub const SCE_HTTP_STATUS_CODE_NON_AUTHORITATIVE_INFORMATION: SceHttpStatusCode = 203;
pub const SCE_HTTP_STATUS_CODE_NOT_ACCEPTABLE: SceHttpStatusCode = 406;
pub const SCE_HTTP_STATUS_CODE_NOT_FOUND: SceHttpStatusCode = 404;
pub const SCE_HTTP_STATUS_CODE_NOT_IMPLEMENTED: SceHttpStatusCode = 501;
pub const SCE_HTTP_STATUS_CODE_NOT_MODIFIED: SceHttpStatusCode = 304;
pub const SCE_HTTP_STATUS_CODE_OK: SceHttpStatusCode = 200;
pub const SCE_HTTP_STATUS_CODE_PARTIAL_CONTENT: SceHttpStatusCode = 206;
pub const SCE_HTTP_STATUS_CODE_PAYMENT_REQUIRED: SceHttpStatusCode = 402;
pub const SCE_HTTP_STATUS_CODE_PRECONDITION_FAILED: SceHttpStatusCode = 412;
pub const SCE_HTTP_STATUS_CODE_PROCESSING: SceHttpStatusCode = 102;
pub const SCE_HTTP_STATUS_CODE_PROXY_AUTHENTICATION_REQUIRED: SceHttpStatusCode = 407;
pub const SCE_HTTP_STATUS_CODE_REQUEST_ENTITY_TOO_LARGE: SceHttpStatusCode = 413;
pub const SCE_HTTP_STATUS_CODE_REQUEST_RANGE_NOT_SATISFIBLE: SceHttpStatusCode = 416;
pub const SCE_HTTP_STATUS_CODE_REQUEST_TIME_OUT: SceHttpStatusCode = 408;
pub const SCE_HTTP_STATUS_CODE_REQUEST_URI_TOO_LARGE: SceHttpStatusCode = 414;
pub const SCE_HTTP_STATUS_CODE_RESET_CONTENT: SceHttpStatusCode = 205;
pub const SCE_HTTP_STATUS_CODE_SEE_OTHER: SceHttpStatusCode = 303;
pub const SCE_HTTP_STATUS_CODE_SERVICE_UNAVAILABLE: SceHttpStatusCode = 503;
pub const SCE_HTTP_STATUS_CODE_SWITCHING_PROTOCOLS: SceHttpStatusCode = 101;
pub const SCE_HTTP_STATUS_CODE_TEMPORARY_REDIRECT: SceHttpStatusCode = 307;
pub const SCE_HTTP_STATUS_CODE_UNAUTHORIZED: SceHttpStatusCode = 401;
pub const SCE_HTTP_STATUS_CODE_UNPROCESSABLE_ENTITY: SceHttpStatusCode = 422;
pub const SCE_HTTP_STATUS_CODE_UNSUPPORTED_MEDIA_TYPE: SceHttpStatusCode = 415;
pub const SCE_HTTP_STATUS_CODE_UPGRADE_REQUIRED: SceHttpStatusCode = 426;
pub const SCE_HTTP_STATUS_CODE_USE_PROXY: SceHttpStatusCode = 305;
pub const SCE_HTTPS_TLSV1: SceHttpSslVersion = 3;
pub const SCE_HTTP_URI_BUILD_WITH_ALL: SceHttpUriBuildType = 65535;
pub const SCE_HTTP_URI_BUILD_WITH_FRAGMENT: SceHttpUriBuildType = 128;
pub const SCE_HTTP_URI_BUILD_WITH_HOSTNAME: SceHttpUriBuildType = 2;
pub const SCE_HTTP_URI_BUILD_WITH_PASSWORD: SceHttpUriBuildType = 32;
pub const SCE_HTTP_URI_BUILD_WITH_PATH: SceHttpUriBuildType = 8;
pub const SCE_HTTP_URI_BUILD_WITH_PORT: SceHttpUriBuildType = 4;
pub const SCE_HTTP_URI_BUILD_WITH_QUERY: SceHttpUriBuildType = 64;
pub const SCE_HTTP_URI_BUILD_WITH_SCHEME: SceHttpUriBuildType = 1;
pub const SCE_HTTP_URI_BUILD_WITH_USERNAME: SceHttpUriBuildType = 16;
pub const SCE_HTTP_USERNAME_MAX_SIZE: u32 = 256;
pub const SCE_HTTP_VERSION_1_0: SceHttpVersion = 1;
pub const SCE_HTTP_VERSION_1_1: SceHttpVersion = 2;
pub const SCE_I2C_ERROR_INVALID_ADDR: SceI2cErrorCode = 2151613187;
pub const SCE_I2C_ERROR_INVALID_BUS: SceI2cErrorCode = 2151613184;
pub const SCE_I2C_ERROR_INVALID_SIZE: SceI2cErrorCode = 2151613186;
pub const SCE_IFTU_ERROR_INVALID_PARAM: SceIftuErrorCode = 2151614209;
pub const SCE_IFTU_ERROR_INVALID_PIXELFORMAT: SceIftuErrorCode = 2151614211;
pub const SCE_IFTU_ERROR_INVALID_PLANE: SceIftuErrorCode = 2151614208;
pub const SCE_IFTU_ERROR_PLANE_BUSY: SceIftuErrorCode = 2151614212;
pub const SCE_IFTU_PIXELFORMAT_BGR565: SceIftuPixelformat = 1;
pub const SCE_IFTU_PIXELFORMAT_BGRA1010102: SceIftuPixelformat = 64;
pub const SCE_IFTU_PIXELFORMAT_BGRA1010102_MULT: SceIftuPixelformat = 32768;
pub const SCE_IFTU_PIXELFORMAT_BGRA5551: SceIftuPixelformat = 4;
pub const SCE_IFTU_PIXELFORMAT_BGRP: SceIftuPixelformat = 256;
pub const SCE_IFTU_PIXELFORMAT_BGRX8888: SceIftuPixelformat = 16;
pub const SCE_IFTU_PIXELFORMAT_BGRX8888_MULT: SceIftuPixelformat = 8192;
pub const SCE_IFTU_PIXELFORMAT_NV12: SceIftuPixelformat = 65536;
pub const SCE_IFTU_PIXELFORMAT_RGB565: SceIftuPixelformat = 2;
pub const SCE_IFTU_PIXELFORMAT_RGBA1010102: SceIftuPixelformat = 128;
pub const SCE_IFTU_PIXELFORMAT_RGBA1010102_MULT: SceIftuPixelformat = 16384;
pub const SCE_IFTU_PIXELFORMAT_RGBA5551: SceIftuPixelformat = 8;
pub const SCE_IFTU_PIXELFORMAT_RGBX8888: SceIftuPixelformat = 32;
pub const SCE_IFTU_PIXELFORMAT_RGBX8888_MULT: SceIftuPixelformat = 4096;
pub const SCE_IFTU_PIXELFORMAT_YUV420: SceIftuPixelformat = 131072;
pub const SCE_IFTU_PIXELFORMAT_YUV422: SceIftuPixelformat = 2097152;
pub const SCE_IME_DIALOG_BUTTON_CLOSE: SceImeDialogButton = 1;
pub const SCE_IME_DIALOG_BUTTON_ENTER: SceImeDialogButton = 2;
pub const SCE_IME_DIALOG_BUTTON_NONE: SceImeDialogButton = 0;
pub const SCE_IME_DIALOG_DIALOG_MODE_DEFAULT: SceImeDialogDialogMode = 0;
pub const SCE_IME_DIALOG_DIALOG_MODE_WITH_CANCEL: SceImeDialogDialogMode = 1;
pub const SCE_IME_DIALOG_ERROR_INTERNAL: SceImeDialogErrorCode = 2148540418;
pub const SCE_IME_DIALOG_ERROR_INVALID_DIALOG_MODE: SceImeDialogErrorCode = 2148540419;
pub const SCE_IME_DIALOG_ERROR_INVALID_TEXT_BOX_MODE: SceImeDialogErrorCode = 2148540420;
pub const SCE_IME_DIALOG_ERROR_INVALID_TITLE: SceImeDialogErrorCode = 2148540421;
pub const SCE_IME_DIALOG_ERROR_PARAM: SceImeDialogErrorCode = 2148540417;
pub const SCE_IME_DIALOG_MAX_TEXT_LENGTH: u32 = 2048;
pub const SCE_IME_DIALOG_MAX_TITLE_LENGTH: u32 = 128;
pub const SCE_IME_DIALOG_TEXTBOX_MODE_DEFAULT: SceImeDialogTextboxMode = 0;
pub const SCE_IME_DIALOG_TEXTBOX_MODE_PASSWORD: SceImeDialogTextboxMode = 1;
pub const SCE_IME_DIALOG_TEXTBOX_MODE_WITH_CLEAR: SceImeDialogTextboxMode = 2;
pub const SCE_IME_ENTER_LABEL_DEFAULT: SceImeEnterLabel = 0;
pub const SCE_IME_ENTER_LABEL_GO: SceImeEnterLabel = 3;
pub const SCE_IME_ENTER_LABEL_SEARCH: SceImeEnterLabel = 2;
pub const SCE_IME_ENTER_LABEL_SEND: SceImeEnterLabel = 1;
pub const SCE_IME_ERROR_ALREADY_OPENED: SceImeErrorCode = 2148534016;
pub const SCE_IME_ERROR_CONNECTION_FAILED: SceImeErrorCode = 2148534021;
pub const SCE_IME_ERROR_INTERNAL: SceImeErrorCode = 2148534096;
pub const SCE_IME_ERROR_INVALID_ARG: SceImeErrorCode = 2148534039;
pub const SCE_IME_ERROR_INVALID_ENTER_LABEL: SceImeErrorCode = 2148534044;
pub const SCE_IME_ERROR_INVALID_HANDLER: SceImeErrorCode = 2148534040;
pub const SCE_IME_ERROR_INVALID_INPUT_METHOD: SceImeErrorCode = 2148534034;
pub const SCE_IME_ERROR_INVALID_INPUT_TEXT_BUFFER: SceImeErrorCode = 2148534042;
pub const SCE_IME_ERROR_INVALID_MAX_TEXT_LENGTH: SceImeErrorCode = 2148534041;
pub const SCE_IME_ERROR_INVALID_OPTION: SceImeErrorCode = 2148534037;
pub const SCE_IME_ERROR_INVALID_PARAM: SceImeErrorCode = 2148534019;
pub const SCE_IME_ERROR_INVALID_POINTER: SceImeErrorCode = 2148534018;
pub const SCE_IME_ERROR_INVALID_RESERVED: SceImeErrorCode = 2148534043;
pub const SCE_IME_ERROR_INVALID_SIZE: SceImeErrorCode = 2148534032;
pub const SCE_IME_ERROR_INVALID_SUPPORTED_LANGUAGES: SceImeErrorCode = 2148534035;
pub const SCE_IME_ERROR_INVALID_TEXT: SceImeErrorCode = 2148534022;
pub const SCE_IME_ERROR_INVALID_TYPE: SceImeErrorCode = 2148534036;
pub const SCE_IME_ERROR_INVALID_WORK: SceImeErrorCode = 2148534038;
pub const SCE_IME_ERROR_NO_MEMORY: SceImeErrorCode = 2148534020;
pub const SCE_IME_ERROR_NOT_OPENED: SceImeErrorCode = 2148534017;
pub const SCE_IME_ERROR_TOO_MANY_REQUESTS: SceImeErrorCode = 2148534023;
pub const SCE_IME_EVENT_CHANGE_SIZE: SceImeEvent = 3;
pub const SCE_IME_EVENT_OPEN: SceImeEvent = 0;
pub const SCE_IME_EVENT_PRESS_CLOSE: SceImeEvent = 4;
pub const SCE_IME_EVENT_PRESS_ENTER: SceImeEvent = 5;
pub const SCE_IME_EVENT_UPDATE_CARET: SceImeEvent = 2;
pub const SCE_IME_EVENT_UPDATE_TEXT: SceImeEvent = 1;
pub const SCE_IME_LANGUAGE_DANISH: SceImeLanguage = 1;
pub const SCE_IME_LANGUAGE_DUTCH: SceImeLanguage = 64;
pub const SCE_IME_LANGUAGE_ENGLISH: SceImeLanguage = 4;
pub const SCE_IME_LANGUAGE_ENGLISH_GB: SceImeLanguage = 262144;
pub const SCE_IME_LANGUAGE_FINNISH: SceImeLanguage = 2048;
pub const SCE_IME_LANGUAGE_FRENCH: SceImeLanguage = 16;
pub const SCE_IME_LANGUAGE_GERMAN: SceImeLanguage = 2;
pub const SCE_IME_LANGUAGE_ITALIAN: SceImeLanguage = 32;
pub const SCE_IME_LANGUAGE_JAPANESE: SceImeLanguage = 8192;
pub const SCE_IME_LANGUAGE_KOREAN: SceImeLanguage = 16384;
pub const SCE_IME_LANGUAGE_NORWEGIAN: SceImeLanguage = 128;
pub const SCE_IME_LANGUAGE_POLISH: SceImeLanguage = 256;
pub const SCE_IME_LANGUAGE_PORTUGUESE: SceImeLanguage = 512;
pub const SCE_IME_LANGUAGE_PORTUGUESE_BR: SceImeLanguage = 131072;
pub const SCE_IME_LANGUAGE_RUSSIAN: SceImeLanguage = 1024;
pub const SCE_IME_LANGUAGE_SIMPLIFIED_CHINESE: SceImeLanguage = 32768;
pub const SCE_IME_LANGUAGE_SPANISH: SceImeLanguage = 8;
pub const SCE_IME_LANGUAGE_SWEDISH: SceImeLanguage = 4096;
pub const SCE_IME_LANGUAGE_TRADITIONAL_CHINESE: SceImeLanguage = 65536;
pub const SCE_IME_LANGUAGE_TURKISH: SceImeLanguage = 524288;
pub const SCE_IME_MAX_PREEDIT_LENGTH: u32 = 30;
pub const SCE_IME_MAX_TEXT_LENGTH: u32 = 2048;
pub const SCE_IME_OPTION_MULTILINE: SceImeOption = 1;
pub const SCE_IME_OPTION_NO_ASSISTANCE: SceImeOption = 4;
pub const SCE_IME_OPTION_NO_AUTO_CAPITALIZATION: SceImeOption = 2;
pub const SCE_IME_TYPE_BASIC_LATIN: SceImeType = 1;
pub const SCE_IME_TYPE_DEFAULT: SceImeType = 0;
pub const SCE_IME_TYPE_EXTENDED_NUMBER: SceImeType = 3;
pub const SCE_IME_TYPE_MAIL: SceImeType = 5;
pub const SCE_IME_TYPE_NUMBER: SceImeType = 2;
pub const SCE_IME_TYPE_URL: SceImeType = 4;
pub const SCE_IME_WORK_BUFFER_SIZE: u32 = 20480;
pub const SCE_INCOMING_DIALOG_ACCEPTED: SceIncomingDialogStatus = 1;
pub const SCE_INCOMING_DIALOG_BUSY: SceIncomingDialogStatus = 5;
pub const SCE_INCOMING_DIALOG_CLOSED: SceIncomingDialogStatus = 4;
pub const SCE_INCOMINGDIALOG_ERROR_INVALID_ARG: SceIncomingDialogErrorCode = 2148557313;
pub const SCE_INCOMING_DIALOG_NOT_RUNNING: SceIncomingDialogStatus = 0;
pub const SCE_INCOMING_DIALOG_REJECTED: SceIncomingDialogStatus = 3;
pub const SCE_INCOMING_DIALOG_RUNNING: SceIncomingDialogStatus = 2;
pub const SCE_INCOMING_DIALOG_TIMEOUT: SceIncomingDialogStatus = 6;
pub const SCE_JPEG_ARM_OK: SceJpegArmErrorCode = 0;
pub const SCE_JPEGENCARM_DEFAULT_COMP_RATIO: u32 = 64;
pub const SCE_JPEGENCARM_ERROR_IMAGE_SIZE: SceJpegEncArmErrorCode = 2154103552;
pub const SCE_JPEGENCARM_ERROR_INSUFFICIENT_BUFFER: SceJpegEncArmErrorCode = 2154103553;
pub const SCE_JPEGENCARM_ERROR_INVALID_COMP_RATIO: SceJpegEncArmErrorCode = 2154103554;
pub const SCE_JPEGENCARM_ERROR_INVALID_HEADER_MODE: SceJpegEncArmErrorCode = 2154103556;
pub const SCE_JPEGENCARM_ERROR_INVALID_PIXELFORMAT: SceJpegEncArmErrorCode = 2154103555;
pub const SCE_JPEGENCARM_ERROR_INVALID_POINTER: SceJpegEncArmErrorCode = 2154103557;
pub const SCE_JPEGENCARM_HEADER_MODE_JPEG: SceJpegArmEncoderHeaderMode = 0;
pub const SCE_JPEGENCARM_HEADER_MODE_MJPEG: SceJpegArmEncoderHeaderMode = 1;
pub const SCE_JPEGENCARM_MAX_COMP_RATIO: u32 = 255;
pub const SCE_JPEGENCARM_MIN_COMP_RATIO: u32 = 1;
pub const SCE_JPEGENCARM_PIXELFORMAT_YCBCR420: SceJpegArmEncoderPixelFormat = 8;
pub const SCE_JPEGENCARM_PIXELFORMAT_YCBCR422: SceJpegArmEncoderPixelFormat = 9;
pub const SCE_JPEGENC_ERROR_IMAGE_SIZE: SceJpegEncErrorCode = 2154103296;
pub const SCE_JPEGENC_ERROR_INSUFFICIENT_BUFFER: SceJpegEncErrorCode = 2154103297;
pub const SCE_JPEGENC_ERROR_INVALID_COMPRATIO: SceJpegEncErrorCode = 2154103298;
pub const SCE_JPEGENC_ERROR_INVALID_HEADER_MODE: SceJpegEncErrorCode = 2154103300;
pub const SCE_JPEGENC_ERROR_INVALID_PIXELFORMAT: SceJpegEncErrorCode = 2154103299;
pub const SCE_JPEGENC_ERROR_INVALID_POINTER: SceJpegEncErrorCode = 2154103301;
pub const SCE_JPEGENC_ERROR_NOT_PHY_CONTINUOUS_MEMORY: SceJpegEncErrorCode = 2154103302;
pub const SCE_JPEGENC_HEADER_MODE_JPEG: SceJpegEncoderHeaderMode = 0;
pub const SCE_JPEGENC_HEADER_MODE_MJPEG: SceJpegEncoderHeaderMode = 1;
pub const SCE_JPEGENC_INIT_PARAM_OPTION_LPDDR2_MEMORY: SceJpegEncoderInitParamOption = 1;
pub const SCE_JPEGENC_INIT_PARAM_OPTION_NONE: SceJpegEncoderInitParamOption = 0;
pub const SCE_JPEGENC_PIXELFORMAT_ARGB8888: SceJpegEncoderPixelFormat = 0;
pub const SCE_JPEGENC_PIXELFORMAT_CSC_ARGB_YCBCR: SceJpegEncoderPixelFormat = 16;
pub const SCE_JPEGENC_PIXELFORMAT_YCBCR420: SceJpegEncoderPixelFormat = 8;
pub const SCE_JPEGENC_PIXELFORMAT_YCBCR422: SceJpegEncoderPixelFormat = 9;
pub const SCE_KERNEL_128KiB: u32 = 131072;
pub const SCE_KERNEL_128MiB: u32 = 134217728;
pub const SCE_KERNEL_16GiB: u64 = 17179869184;
pub const SCE_KERNEL_16KiB: u32 = 16384;
pub const SCE_KERNEL_16MiB: u32 = 16777216;
pub const SCE_KERNEL_1GiB: u32 = 1073741824;
pub const SCE_KERNEL_1KiB: u32 = 1024;
pub const SCE_KERNEL_1MiB: u32 = 1048576;
pub const SCE_KERNEL_256KiB: u32 = 262144;
pub const SCE_KERNEL_256MiB: u32 = 268435456;
pub const SCE_KERNEL_2GiB: u32 = 2147483648;
pub const SCE_KERNEL_2KiB: u32 = 2048;
pub const SCE_KERNEL_2MiB: u32 = 2097152;
pub const SCE_KERNEL_32GiB: u64 = 34359738368;
pub const SCE_KERNEL_32KiB: u32 = 32768;
pub const SCE_KERNEL_32MiB: u32 = 33554432;
pub const SCE_KERNEL_4GiB: u64 = 4294967296;
pub const SCE_KERNEL_4KiB: u32 = 4096;
pub const SCE_KERNEL_4MiB: u32 = 4194304;
pub const SCE_KERNEL_512KiB: u32 = 524288;
pub const SCE_KERNEL_512MiB: u32 = 536870912;
pub const SCE_KERNEL_64KiB: u32 = 65536;
pub const SCE_KERNEL_64MiB: u32 = 67108864;
pub const SCE_KERNEL_8GiB: u64 = 8589934592;
pub const SCE_KERNEL_8KiB: u32 = 8192;
pub const SCE_KERNEL_8MiB: u32 = 8388608;
pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_ALLOW_PARTIAL_OP: SceKernelAllocMemBlockAttr = 67108864;
pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_ALIGNMENT: SceKernelAllocMemBlockAttr = 4;
pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_MIRROR_BLOCKID: SceKernelAllocMemBlockAttr = 64;
pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_PADDR: SceKernelAllocMemBlockAttr = 2;
pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_PADDR_LIST: SceKernelAllocMemBlockAttr = 4096;
pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_HAS_PID: SceKernelAllocMemBlockAttr = 128;
pub const SCE_KERNEL_ALLOC_MEMBLOCK_ATTR_PHYCONT: SceKernelAllocMemBlockAttr = 2097152;
pub const SCE_KERNEL_ASSERT_LEVEL_0: SceKernelAssertLevel = 0;
pub const SCE_KERNEL_ASSERT_LEVEL_1: SceKernelAssertLevel = 1;
pub const SCE_KERNEL_ASSERT_LEVEL_2: SceKernelAssertLevel = 2;
pub const SCE_KERNEL_ATTR_OPENABLE: SceKernelWaitableAttribute = 128;
pub const SCE_KERNEL_ATTR_THREAD_FIFO: SceKernelWaitableAttribute = 0;
pub const SCE_KERNEL_ATTR_THREAD_PRIO: SceKernelWaitableAttribute = 8192;
pub const SCE_KERNEL_CPU_MASK_SYSTEM: u32 = 524288;
pub const SCE_KERNEL_CPU_MASK_USER_0: u32 = 65536;
pub const SCE_KERNEL_CPU_MASK_USER_1: u32 = 131072;
pub const SCE_KERNEL_CPU_MASK_USER_2: u32 = 262144;
pub const SCE_KERNEL_CPU_MASK_USER_ALL: u32 = 458752;
pub const SCE_KERNEL_DEBUG_INFO_FLAG_CORE: SceKernelDebugInfoFlags = 1;
pub const SCE_KERNEL_DEBUG_INFO_FLAG_FILE: SceKernelDebugInfoFlags = 8;
pub const SCE_KERNEL_DEBUG_INFO_FLAG_FUNC: SceKernelDebugInfoFlags = 2;
pub const SCE_KERNEL_DEBUG_INFO_FLAG_LINE: SceKernelDebugInfoFlags = 4;
pub const SCE_KERNEL_DEBUG_INFO_FLAG_NONE: SceKernelDebugInfoFlags = 0;
pub const SCE_KERNEL_DEBUG_LEVEL_ALWAYS: SceKernelDebugLevel = 0;
pub const SCE_KERNEL_DEBUG_LEVEL_DEBUG: SceKernelDebugLevel = 1;
pub const SCE_KERNEL_DEBUG_LEVEL_TRACE: SceKernelDebugLevel = 2;
pub const SCE_KERNEL_DMAC_BLOCKSIZE_DST_MASK: u32 = 4294901760;
pub const SCE_KERNEL_DMAC_BLOCKSIZE_DST_SHIFT: u32 = 16;
pub const SCE_KERNEL_DMAC_BLOCKSIZE_SRC_MASK: u32 = 65535;
pub const SCE_KERNEL_DMAC_BLOCKSIZE_SRC_SHIFT: u32 = 0;
pub const SCE_KERNEL_DMAC_CMD_COHERENT_DST: u32 = 33554432;
pub const SCE_KERNEL_DMAC_CMD_COHERENT_IV_READ: u32 = 67108864;
pub const SCE_KERNEL_DMAC_CMD_COHERENT_IV_WRITE: u32 = 134217728;
pub const SCE_KERNEL_DMAC_CMD_COHERENT_SRC: u32 = 16777216;
pub const SCE_KERNEL_DMAC_CMD_HASH_FINALIZE: u32 = 2048;
pub const SCE_KERNEL_DMAC_CMD_HASH_UPDATE: u32 = 1024;
pub const SCE_KERNEL_DMAC_CMD_KEYSIZE_128BIT: u32 = 256;
pub const SCE_KERNEL_DMAC_CMD_KEYSIZE_192BIT: u32 = 512;
pub const SCE_KERNEL_DMAC_CMD_KEYSIZE_256BIT: u32 = 768;
pub const SCE_KERNEL_DMAC_CMD_KEYSIZE_64BIT: u32 = 0;
pub const SCE_KERNEL_DMAC_CMD_OP_COPY: u32 = 0;
pub const SCE_KERNEL_DMAC_CMD_OP_DECRYPT_AES_CBC: u32 = 10;
pub const SCE_KERNEL_DMAC_CMD_OP_DECRYPT_AES_CTR: u32 = 18;
pub const SCE_KERNEL_DMAC_CMD_OP_DECRYPT_AES_ECB: u32 = 2;
pub const SCE_KERNEL_DMAC_CMD_OP_DECRYPT_DES_CBC: u32 = 74;
pub const SCE_KERNEL_DMAC_CMD_OP_DECRYPT_DES_ECB: u32 = 66;
pub const SCE_KERNEL_DMAC_CMD_OP_ENCRYPT_AES_CBC: u32 = 9;
pub const SCE_KERNEL_DMAC_CMD_OP_ENCRYPT_AES_CTR: u32 = 17;
pub const SCE_KERNEL_DMAC_CMD_OP_ENCRYPT_AES_ECB: u32 = 1;
pub const SCE_KERNEL_DMAC_CMD_OP_ENCRYPT_DES_CBC: u32 = 73;
pub const SCE_KERNEL_DMAC_CMD_OP_ENCRYPT_DES_ECB: u32 = 65;
pub const SCE_KERNEL_DMAC_CMD_OP_HASH_SHA1: u32 = 3;
pub const SCE_KERNEL_DMAC_CMD_OP_HASH_SHA224: u32 = 11;
pub const SCE_KERNEL_DMAC_CMD_OP_HASH_SHA256: u32 = 19;
pub const SCE_KERNEL_DMAC_CMD_OP_HMAC_SHA1: u32 = 35;
pub const SCE_KERNEL_DMAC_CMD_OP_HMAC_SHA224: u32 = 43;
pub const SCE_KERNEL_DMAC_CMD_OP_HMAC_SHA256: u32 = 51;
pub const SCE_KERNEL_DMAC_CMD_OP_RNG: u32 = 4;
pub const SCE_KERNEL_DMAC_CMD_OP_SET: u32 = 12;
pub const SCE_KERNEL_DMAC_CMD_USE_EXTERNAL_KEY: u32 = 128;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_DST_MASK: u32 = 261632;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_DST_SHIFT: u32 = 9;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_SRC_DST_MASK: u32 = 262143;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_SRC_MASK: u32 = 511;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_SRC_SHIFT: u32 = 0;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_UNK_MASK: u32 = 133955584;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_UNK_SHIFT: u32 = 18;
pub const SCE_KERNEL_DMAC_ID_DMAC01: SceKernelDmacId = 16;
pub const SCE_KERNEL_DMAC_ID_DMAC23: SceKernelDmacId = 17;
pub const SCE_KERNEL_DMAC_ID_DMAC4: SceKernelDmacId = 18;
pub const SCE_KERNEL_DMAC_ID_DMAC5: SceKernelDmacId = 19;
pub const SCE_KERNEL_DMAC_ID_DMAC6: SceKernelDmacId = 20;
pub const SCE_KERNEL_DMAC_IV_COHERENCY_MSK_READ_MASK: u32 = 511;
pub const SCE_KERNEL_DMAC_IV_COHERENCY_MSK_READ_SHIFT: u32 = 0;
pub const SCE_KERNEL_DMAC_IV_COHERENCY_MSK_RW_MASK: u32 = 33489407;
pub const SCE_KERNEL_DMAC_IV_COHERENCY_MSK_WRITE_MASK: u32 = 33488896;
pub const SCE_KERNEL_DMAC_IV_COHERENCY_MSK_WRITE_SHIFT: u32 = 16;
pub const SCE_KERNEL_DMAC_STAT_ABORTED: u32 = 2;
pub const SCE_KERNEL_DMAC_STAT_BUSY: u32 = 1;
pub const SCE_KERNEL_DMAC_STAT_ERROR_ILLEGAL_CONFIG: u32 = 262144;
pub const SCE_KERNEL_DMAC_STAT_ERROR_READ: u32 = 65536;
pub const SCE_KERNEL_DMAC_STAT_ERROR_TAG: u32 = 524288;
pub const SCE_KERNEL_DMAC_STAT_ERROR_WRITE: u32 = 131072;
pub const SCE_KERNEL_DMAC_STAT_ERROR_ZERO_BYTE: u32 = 1048576;
pub const SCE_KERNEL_DMA_OP_COMPLETE_CHAIN: SceKernelDmaOpFlag = 256;
pub const SCE_KERNEL_DMA_OP_PHYSICAL_ADDR: SceKernelDmaOpFlag = 0;
pub const SCE_KERNEL_DMA_OP_SYNC_POLL: SceKernelDmaOpSyncMode = 1;
pub const SCE_KERNEL_DMA_OP_SYNC_TIMED_WAIT: SceKernelDmaOpSyncMode = 3;
pub const SCE_KERNEL_DMA_OP_SYNC_WAIT: SceKernelDmaOpSyncMode = 2;
pub const SCE_KERNEL_DMA_OP_VIRTUAL_ADDR: SceKernelDmaOpFlag = 17;
pub const SCE_KERNEL_DMA_OP_VIRTUAL_DST_ADDR: SceKernelDmaOpFlag = 16;
pub const SCE_KERNEL_DMA_OP_VIRTUAL_SRC_ADDR: SceKernelDmaOpFlag = 1;
pub const SCE_KERNEL_ERROR_ADDRESS_SPACE_CANNOT_FIND_PARTITION_BY_ADDR: SceKernelErrorCode =
    2147633667;
pub const SCE_KERNEL_ERROR_ALARM_CAN_NOT_CANCEL: SceKernelErrorCode = 2147647682;
pub const SCE_KERNEL_ERROR_ALARM_ERROR: SceKernelErrorCode = 2147647680;
pub const SCE_KERNEL_ERROR_ALREADY_DEBUG_SUSPENDED: SceKernelErrorCode = 2147647535;
pub const SCE_KERNEL_ERROR_ALREADY_QUEUED: SceKernelErrorCode = 2147643905;
pub const SCE_KERNEL_ERROR_ALREADY_REGISTERED: SceKernelErrorCode = 2147647491;
pub const SCE_KERNEL_ERROR_ALREADY_SENT: SceKernelErrorCode = 2147647777;
pub const SCE_KERNEL_ERROR_AUTHFAIL: SceKernelErrorCode = 2147676160;
pub const SCE_KERNEL_ERROR_BLOCK_ERROR: SceKernelErrorCode = 2147632896;
pub const SCE_KERNEL_ERROR_BLOCK_IN_USE: SceKernelErrorCode = 2147632899;
pub const SCE_KERNEL_ERROR_CALLBACK_ERROR: SceKernelErrorCode = 2147647648;
pub const SCE_KERNEL_ERROR_CALLBACK_NOT_REGISTERED: SceKernelErrorCode = 2147647651;
pub const SCE_KERNEL_ERROR_CANCELING: SceKernelErrorCode = 2147643912;
pub const SCE_KERNEL_ERROR_CANNOT_ALLOC_BITHEAP: SceKernelErrorCode = 2147634433;
pub const SCE_KERNEL_ERROR_CANNOT_ALLOC_PHYMEMLOW: SceKernelErrorCode = 2147634177;
pub const SCE_KERNEL_ERROR_CANNOT_GROW_PHYMEMPART: SceKernelErrorCode = 2147631880;
pub const SCE_KERNEL_ERROR_CANNOT_RELEASE_EXCPHANDLER: SceKernelErrorCode = 2147643396;
pub const SCE_KERNEL_ERROR_CAN_NOT_SUSPEND: SceKernelErrorCode = 2147647531;
pub const SCE_KERNEL_ERROR_CAN_NOT_USE_VFP: SceKernelErrorCode = 2147647537;
pub const SCE_KERNEL_ERROR_CAN_NOT_WAIT: SceKernelErrorCode = 2147647492;
pub const SCE_KERNEL_ERROR_COND_ERROR: SceKernelErrorCode = 2147647904;
pub const SCE_KERNEL_ERROR_CP14_DISABLED: SceKernelErrorCode = 2147651592;
pub const SCE_KERNEL_ERROR_CPU_ERROR: SceKernelErrorCode = 2147622912;
pub const SCE_KERNEL_ERROR_DEBUG_ERROR: SceKernelErrorCode = 2147618816;
pub const SCE_KERNEL_ERROR_DELETED: SceKernelErrorCode = 2147647530;
pub const SCE_KERNEL_ERROR_DIFFERENT_UID_CLASS: SceKernelErrorCode = 2147647490;
pub const SCE_KERNEL_ERROR_DMACMGR_ERROR: SceKernelErrorCode = 2147643904;
pub const SCE_KERNEL_ERROR_DORMANT: SceKernelErrorCode = 2147647527;
pub const SCE_KERNEL_ERROR_DUPLICATE_NAME: SceKernelErrorCode = 2147634690;
pub const SCE_KERNEL_ERROR_ERROR: SceKernelErrorCode = 2147614721;
pub const SCE_KERNEL_ERROR_EVENT_COND: SceKernelErrorCode = 2147648002;
pub const SCE_KERNEL_ERROR_EVENT_ERROR: SceKernelErrorCode = 2147648000;
pub const SCE_KERNEL_ERROR_EVENT_NOT_SET: SceKernelErrorCode = 2147648066;
pub const SCE_KERNEL_ERROR_EVF_COND: SceKernelErrorCode = 2147647715;
pub const SCE_KERNEL_ERROR_EVF_ERROR: SceKernelErrorCode = 2147647712;
pub const SCE_KERNEL_ERROR_EVF_MULTI: SceKernelErrorCode = 2147647714;
pub const SCE_KERNEL_ERROR_EXCEEDED_MAX_PROCESSES: SceKernelErrorCode = 2147651593;
pub const SCE_KERNEL_ERROR_EXCPMGR_ERROR: SceKernelErrorCode = 2147643392;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_ALREADY_INITIALIZED: SceKernelErrorCode = 2147647847;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_ERROR: SceKernelErrorCode = 2147647840;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_FAILED_TO_OWN: SceKernelErrorCode = 2147647844;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_LOCK_OVF: SceKernelErrorCode = 2147647843;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_NOT_INITIALIZED: SceKernelErrorCode = 2147647848;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_NOT_OWNED: SceKernelErrorCode = 2147647845;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_OWNED: SceKernelErrorCode = 2147647846;
pub const SCE_KERNEL_ERROR_FAST_MUTEX_RECURSIVE: SceKernelErrorCode = 2147647842;
pub const SCE_KERNEL_ERROR_FIXEDHEAP_ERROR: SceKernelErrorCode = 2147632128;
pub const SCE_KERNEL_ERROR_FIXEDHEAP_ILLEGAL_INDEX: SceKernelErrorCode = 2147632130;
pub const SCE_KERNEL_ERROR_FIXEDHEAP_ILLEGAL_SIZE: SceKernelErrorCode = 2147632129;
pub const SCE_KERNEL_ERROR_FIXEDHEAP_INDEX_OVERFLOW: SceKernelErrorCode = 2147632131;
pub const SCE_KERNEL_ERROR_FIXEDHEAP_NO_CHUNK: SceKernelErrorCode = 2147632132;
pub const SCE_KERNEL_ERROR_FOUND_HANDLER: SceKernelErrorCode = 2147643657;
pub const SCE_KERNEL_ERROR_HEAPLIB_ERROR: SceKernelErrorCode = 2147633408;
pub const SCE_KERNEL_ERROR_HEAPLIB_NOMEM: SceKernelErrorCode = 2147633411;
pub const SCE_KERNEL_ERROR_HEAPLIB_VERIFY_ERROR: SceKernelErrorCode = 2147633412;
pub const SCE_KERNEL_ERROR_ILLEGAL_ADDR: SceKernelErrorCode = 2147614726;
pub const SCE_KERNEL_ERROR_ILLEGAL_ALIGNMENT: SceKernelErrorCode = 2147614727;
pub const SCE_KERNEL_ERROR_ILLEGAL_ATTR: SceKernelErrorCode = 2147614734;
pub const SCE_KERNEL_ERROR_ILLEGAL_BLOCK_ID: SceKernelErrorCode = 2147632897;
pub const SCE_KERNEL_ERROR_ILLEGAL_BLOCK_TYPE: SceKernelErrorCode = 2147632898;
pub const SCE_KERNEL_ERROR_ILLEGAL_CONTEXT: SceKernelErrorCode = 2147643649;
pub const SCE_KERNEL_ERROR_ILLEGAL_COUNT: SceKernelErrorCode = 2147614735;
pub const SCE_KERNEL_ERROR_ILLEGAL_CPU_AFFINITY_MASK: SceKernelErrorCode = 2147647525;
pub const SCE_KERNEL_ERROR_ILLEGAL_DIPSW_NUMBER: SceKernelErrorCode = 2147618817;
pub const SCE_KERNEL_ERROR_ILLEGAL_ELF_HEADER: SceKernelErrorCode = 2147635201;
pub const SCE_KERNEL_ERROR_ILLEGAL_EXCPCODE: SceKernelErrorCode = 2147643393;
pub const SCE_KERNEL_ERROR_ILLEGAL_EXCPHANDLER: SceKernelErrorCode = 2147643394;
pub const SCE_KERNEL_ERROR_ILLEGAL_HANDLER: SceKernelErrorCode = 2147643656;
pub const SCE_KERNEL_ERROR_ILLEGAL_HEAP_ID: SceKernelErrorCode = 2147633409;
pub const SCE_KERNEL_ERROR_ILLEGAL_INTRCODE: SceKernelErrorCode = 2147643650;
pub const SCE_KERNEL_ERROR_ILLEGAL_INTRFILTER: SceKernelErrorCode = 2147643654;
pub const SCE_KERNEL_ERROR_ILLEGAL_INTRPARAM: SceKernelErrorCode = 2147643651;
pub const SCE_KERNEL_ERROR_ILLEGAL_INTRPRIORITY: SceKernelErrorCode = 2147643652;
pub const SCE_KERNEL_ERROR_ILLEGAL_INTRTYPE: SceKernelErrorCode = 2147643655;
pub const SCE_KERNEL_ERROR_ILLEGAL_KERNEL_TLS_INDEX: SceKernelErrorCode = 2147647618;
pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_CODE: SceKernelErrorCode = 2147633924;
pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_REMAP_TYPE: SceKernelErrorCode = 2147633922;
pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_SIZE: SceKernelErrorCode = 2147633925;
pub const SCE_KERNEL_ERROR_ILLEGAL_MEMBLOCK_TYPE: SceKernelErrorCode = 2147633921;
pub const SCE_KERNEL_ERROR_ILLEGAL_MODE: SceKernelErrorCode = 2147614736;
pub const SCE_KERNEL_ERROR_ILLEGAL_OPEN_LIMIT: SceKernelErrorCode = 2147614737;
pub const SCE_KERNEL_ERROR_ILLEGAL_PARTITION_ID: SceKernelErrorCode = 2147633153;
pub const SCE_KERNEL_ERROR_ILLEGAL_PARTITION_INDEX: SceKernelErrorCode = 2147633154;
pub const SCE_KERNEL_ERROR_ILLEGAL_PATTERN: SceKernelErrorCode = 2147614733;
pub const SCE_KERNEL_ERROR_ILLEGAL_PERMISSION: SceKernelErrorCode = 2147614728;
pub const SCE_KERNEL_ERROR_ILLEGAL_PHYPAGE_STATUS: SceKernelErrorCode = 2147631873;
pub const SCE_KERNEL_ERROR_ILLEGAL_PRIORITY: SceKernelErrorCode = 2147647523;
pub const SCE_KERNEL_ERROR_ILLEGAL_SELF_HEADER: SceKernelErrorCode = 2147635202;
pub const SCE_KERNEL_ERROR_ILLEGAL_SIZE: SceKernelErrorCode = 2147614731;
pub const SCE_KERNEL_ERROR_ILLEGAL_STACK_SIZE: SceKernelErrorCode = 2147647524;
pub const SCE_KERNEL_ERROR_ILLEGAL_TARGET_CPU: SceKernelErrorCode = 2147643653;
pub const SCE_KERNEL_ERROR_ILLEGAL_THREAD_ID: SceKernelErrorCode = 2147647522;
pub const SCE_KERNEL_ERROR_ILLEGAL_THREAD_PARAM_COMBINATION: SceKernelErrorCode = 2147647526;
pub const SCE_KERNEL_ERROR_ILLEGAL_TYPE: SceKernelErrorCode = 2147614732;
pub const SCE_KERNEL_ERROR_ILLEGAL_USERMAP_SIZE: SceKernelErrorCode = 2147633926;
pub const SCE_KERNEL_ERROR_ILLEGAL_VIRPAGE_TYPE: SceKernelErrorCode = 2147632641;
pub const SCE_KERNEL_ERROR_INTRMGR_ERROR: SceKernelErrorCode = 2147643648;
pub const SCE_KERNEL_ERROR_INVALID_ADDRESS_SPACE_ID: SceKernelErrorCode = 2147633665;
pub const SCE_KERNEL_ERROR_INVALID_ARGUMENT: SceKernelErrorCode = 2147614725;
pub const SCE_KERNEL_ERROR_INVALID_ARGUMENT_SIZE: SceKernelErrorCode = 2147614729;
pub const SCE_KERNEL_ERROR_INVALID_BUDGET_ID: SceKernelErrorCode = 2147651590;
pub const SCE_KERNEL_ERROR_INVALID_BUDGET_SIZE: SceKernelErrorCode = 2147651591;
pub const SCE_KERNEL_ERROR_INVALID_CPU_AFFINITY: SceKernelErrorCode = 2147622916;
pub const SCE_KERNEL_ERROR_INVALID_FLAGS: SceKernelErrorCode = 2147614730;
pub const SCE_KERNEL_ERROR_INVALID_MEMORY_ACCESS: SceKernelErrorCode = 2147622917;
pub const SCE_KERNEL_ERROR_INVALID_MEMORY_ACCESS_PERMISSION: SceKernelErrorCode = 2147622918;
pub const SCE_KERNEL_ERROR_INVALID_PARTITION_INDEX: SceKernelErrorCode = 2147633666;
pub const SCE_KERNEL_ERROR_INVALID_PID: SceKernelErrorCode = 2147651585;
pub const SCE_KERNEL_ERROR_INVALID_PROCESS_CONTEXT: SceKernelErrorCode = 2147631105;
pub const SCE_KERNEL_ERROR_INVALID_PROCESS_STATUS: SceKernelErrorCode = 2147651588;
pub const SCE_KERNEL_ERROR_INVALID_PROCESS_TYPE: SceKernelErrorCode = 2147651586;
pub const SCE_KERNEL_ERROR_INVALID_SUBBUDGET_ID: SceKernelErrorCode = 2147631882;
pub const SCE_KERNEL_ERROR_INVALID_UID: SceKernelErrorCode = 2147632385;
pub const SCE_KERNEL_ERROR_INVALID_UID_CLASS: SceKernelErrorCode = 2147632392;
pub const SCE_KERNEL_ERROR_INVALID_UID_SUBCLASS: SceKernelErrorCode = 2147632393;
pub const SCE_KERNEL_ERROR_IO_ALIAS_USED: SceKernelErrorCode = 2147655683;
pub const SCE_KERNEL_ERROR_IO_DEL_DEV: SceKernelErrorCode = 2147655684;
pub const SCE_KERNEL_ERROR_IOFILEMGR_ERROR: SceKernelErrorCode = 2147655680;
pub const SCE_KERNEL_ERROR_IO_NAME_TOO_LONG: SceKernelErrorCode = 2147655681;
pub const SCE_KERNEL_ERROR_IO_REG_DEV: SceKernelErrorCode = 2147655682;
pub const SCE_KERNEL_ERROR_IO_WOULD_BLOCK: SceKernelErrorCode = 2147655685;
pub const SCE_KERNEL_ERROR_KERNEL_TLS_BUSY: SceKernelErrorCode = 2147647619;
pub const SCE_KERNEL_ERROR_KERNEL_TLS_ERROR: SceKernelErrorCode = 2147647616;
pub const SCE_KERNEL_ERROR_KERNEL_TLS_FULL: SceKernelErrorCode = 2147647617;
pub const SCE_KERNEL_ERROR_LIBRARYDB_NOENT: SceKernelErrorCode = 2147668096;
pub const SCE_KERNEL_ERROR_LIBRARYDB_NO_LIB: SceKernelErrorCode = 2147668097;
pub const SCE_KERNEL_ERROR_LIBRARYDB_NO_MOD: SceKernelErrorCode = 2147668098;
pub const SCE_KERNEL_ERROR_LOADCORE_ERROR: SceKernelErrorCode = 2147635200;
pub const SCE_KERNEL_ERROR_LW_COND_ERROR: SceKernelErrorCode = 2147647936;
pub const SCE_KERNEL_ERROR_LW_MUTEX_ERROR: SceKernelErrorCode = 2147647872;
pub const SCE_KERNEL_ERROR_LW_MUTEX_FAILED_TO_OWN: SceKernelErrorCode = 2147647877;
pub const SCE_KERNEL_ERROR_LW_MUTEX_LOCK_OVF: SceKernelErrorCode = 2147647875;
pub const SCE_KERNEL_ERROR_LW_MUTEX_NOT_OWNED: SceKernelErrorCode = 2147647878;
pub const SCE_KERNEL_ERROR_LW_MUTEX_RECURSIVE: SceKernelErrorCode = 2147647874;
pub const SCE_KERNEL_ERROR_LW_MUTEX_UNLOCK_UDF: SceKernelErrorCode = 2147647876;
pub const SCE_KERNEL_ERROR_MEMBLOCK_OVERFLOW: SceKernelErrorCode = 2147633931;
pub const SCE_KERNEL_ERROR_MEMBLOCK_RANGE_ERROR: SceKernelErrorCode = 2147633929;
pub const SCE_KERNEL_ERROR_MEMBLOCK_TYPE_FOR_KERNEL_PROCESS: SceKernelErrorCode = 2147633927;
pub const SCE_KERNEL_ERROR_MEMBLOCK_TYPE_FOR_UPDATER_OR_SAFEMODE: SceKernelErrorCode = 2147633930;
pub const SCE_KERNEL_ERROR_MMU_ILLEGAL_L1_TYPE: SceKernelErrorCode = 2147622913;
pub const SCE_KERNEL_ERROR_MMU_L2_INDEX_OVERFLOW: SceKernelErrorCode = 2147622914;
pub const SCE_KERNEL_ERROR_MMU_L2_SIZE_OVERFLOW: SceKernelErrorCode = 2147622915;
pub const SCE_KERNEL_ERROR_MODULEMGR_BUSY: SceKernelErrorCode = 2147667997;
pub const SCE_KERNEL_ERROR_MODULEMGR_CANNOT_EXPORT_LIB_TO_SHARED: SceKernelErrorCode = 2147667992;
pub const SCE_KERNEL_ERROR_MODULEMGR_ELINK: SceKernelErrorCode = 2147667995;
pub const SCE_KERNEL_ERROR_MODULEMGR_IN_USE: SceKernelErrorCode = 2147667970;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_LIB: SceKernelErrorCode = 2147667977;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_PROCESS_UID: SceKernelErrorCode = 2147667991;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_PROC_PARAM: SceKernelErrorCode = 2147667983;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_REF_INFO: SceKernelErrorCode = 2147667994;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_REL_INFO: SceKernelErrorCode = 2147667993;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_STUB: SceKernelErrorCode = 2147667978;
pub const SCE_KERNEL_ERROR_MODULEMGR_INVALID_TYPE: SceKernelErrorCode = 2147667981;
pub const SCE_KERNEL_ERROR_MODULEMGR_NAMETOOLONG: SceKernelErrorCode = 2147667999;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOENT: SceKernelErrorCode = 2147667996;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOEXEC: SceKernelErrorCode = 2147667998;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_FUNC_NID: SceKernelErrorCode = 2147667979;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_LIB: SceKernelErrorCode = 2147667971;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM: SceKernelErrorCode = 2147667976;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM_LIB: SceKernelErrorCode = 2147667973;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM_SELF: SceKernelErrorCode = 2147667975;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOMEM_STUB: SceKernelErrorCode = 2147667974;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_MOD: SceKernelErrorCode = 2147667985;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_MOD_ENTRY: SceKernelErrorCode = 2147667982;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_MODOBJ: SceKernelErrorCode = 2147667984;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_PROCESS: SceKernelErrorCode = 2147667986;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOT_STARTED: SceKernelErrorCode = 2147667989;
pub const SCE_KERNEL_ERROR_MODULEMGR_NOT_STOPPED: SceKernelErrorCode = 2147667990;
pub const SCE_KERNEL_ERROR_MODULEMGR_NO_VAR_NID: SceKernelErrorCode = 2147667980;
pub const SCE_KERNEL_ERROR_MODULEMGR_OLD_LIB: SceKernelErrorCode = 2147667987;
pub const SCE_KERNEL_ERROR_MODULEMGR_STARTED: SceKernelErrorCode = 2147667988;
pub const SCE_KERNEL_ERROR_MODULEMGR_START_FAILED: SceKernelErrorCode = 2147667968;
pub const SCE_KERNEL_ERROR_MODULEMGR_STOP_FAIL: SceKernelErrorCode = 2147667969;
pub const SCE_KERNEL_ERROR_MODULEMGR_SYSCALL_REG: SceKernelErrorCode = 2147667972;
pub const SCE_KERNEL_ERROR_MSG_PIPE_DELETED: SceKernelErrorCode = 2147648036;
pub const SCE_KERNEL_ERROR_MSG_PIPE_EMPTY: SceKernelErrorCode = 2147648035;
pub const SCE_KERNEL_ERROR_MSG_PIPE_ERROR: SceKernelErrorCode = 2147648032;
pub const SCE_KERNEL_ERROR_MSG_PIPE_FULL: SceKernelErrorCode = 2147648034;
pub const SCE_KERNEL_ERROR_MUTEX_ERROR: SceKernelErrorCode = 2147647808;
pub const SCE_KERNEL_ERROR_MUTEX_FAILED_TO_OWN: SceKernelErrorCode = 2147647813;
pub const SCE_KERNEL_ERROR_MUTEX_LOCK_OVF: SceKernelErrorCode = 2147647811;
pub const SCE_KERNEL_ERROR_MUTEX_NOT_OWNED: SceKernelErrorCode = 2147647814;
pub const SCE_KERNEL_ERROR_MUTEX_RECURSIVE: SceKernelErrorCode = 2147647810;
pub const SCE_KERNEL_ERROR_MUTEX_UNLOCK_UDF: SceKernelErrorCode = 2147647812;
pub const SCE_KERNEL_ERROR_NO_AUTH: SceKernelErrorCode = 2147676161;
pub const SCE_KERNEL_ERROR_NO_FREE_PHYSICAL_PAGE: SceKernelErrorCode = 2147631874;
pub const SCE_KERNEL_ERROR_NO_FREE_PHYSICAL_PAGE_CDRAM: SceKernelErrorCode = 2147631881;
pub const SCE_KERNEL_ERROR_NO_FREE_PHYSICAL_PAGE_UNIT: SceKernelErrorCode = 2147631875;
pub const SCE_KERNEL_ERROR_NO_FREE_TIMER: SceKernelErrorCode = 2147644161;
pub const SCE_KERNEL_ERROR_NO_IOADDR: SceKernelErrorCode = 2147631364;
pub const SCE_KERNEL_ERROR_NO_L2PAGETABLE: SceKernelErrorCode = 2147633155;
pub const SCE_KERNEL_ERROR_NO_MEMORY: SceKernelErrorCode = 2147643659;
pub const SCE_KERNEL_ERROR_NO_PHYADDR: SceKernelErrorCode = 2147631361;
pub const SCE_KERNEL_ERROR_NO_PHYMEMPART_CDRAM: SceKernelErrorCode = 2147631878;
pub const SCE_KERNEL_ERROR_NO_PHYMEMPART_LPDDR2: SceKernelErrorCode = 2147631877;
pub const SCE_KERNEL_ERROR_NO_PROCESS_DATA: SceKernelErrorCode = 2147651595;
pub const SCE_KERNEL_ERROR_NO_SUCH_NAME: SceKernelErrorCode = 2147634689;
pub const SCE_KERNEL_ERROR_NOSYS: SceKernelErrorCode = 2147614723;
pub const SCE_KERNEL_ERROR_NOT_DEBUG_SUSPENDED: SceKernelErrorCode = 2147647536;
pub const SCE_KERNEL_ERROR_NOT_DORMANT: SceKernelErrorCode = 2147647528;
pub const SCE_KERNEL_ERROR_NOTFOUND_EXCPHANDLER: SceKernelErrorCode = 2147643395;
pub const SCE_KERNEL_ERROR_NOTFOUND_HANDLER: SceKernelErrorCode = 2147643658;
pub const SCE_KERNEL_ERROR_NOTIFY_CALLBACK: SceKernelErrorCode = 2147647650;
pub const SCE_KERNEL_ERROR_NOT_IMPLEMENTED: SceKernelErrorCode = 2147614722;
pub const SCE_KERNEL_ERROR_NOT_INITIALIZED: SceKernelErrorCode = 2147643909;
pub const SCE_KERNEL_ERROR_NOT_KERNEL_UID: SceKernelErrorCode = 2147632391;
pub const SCE_KERNEL_ERROR_NOT_PHY_CONT_MEMBLOCK: SceKernelErrorCode = 2147633923;
pub const SCE_KERNEL_ERROR_NOT_PROCESS_UID: SceKernelErrorCode = 2147632390;
pub const SCE_KERNEL_ERROR_NOT_QUEUED: SceKernelErrorCode = 2147643906;
pub const SCE_KERNEL_ERROR_NOT_SETUP: SceKernelErrorCode = 2147643907;
pub const SCE_KERNEL_ERROR_NOT_SUSPENDED: SceKernelErrorCode = 2147647534;
pub const SCE_KERNEL_ERROR_NOT_UNDER_CONTROL: SceKernelErrorCode = 2147643911;
pub const SCE_KERNEL_ERROR_ONLY_DEVELOPMENT_MODE: SceKernelErrorCode = 2147614738;
pub const SCE_KERNEL_ERROR_ON_TRANSFERRING: SceKernelErrorCode = 2147643908;
pub const SCE_KERNEL_ERROR_OUT_OF_RANG: SceKernelErrorCode = 2147633410;
pub const SCE_KERNEL_ERROR_PA_ERROR: SceKernelErrorCode = 2147619072;
pub const SCE_KERNEL_ERROR_PA_INVALID_KEY: SceKernelErrorCode = 2147619074;
pub const SCE_KERNEL_ERROR_PA_INVALID_SIGNATURE: SceKernelErrorCode = 2147619076;
pub const SCE_KERNEL_ERROR_PA_KEY_IS_NOT_SHARED: SceKernelErrorCode = 2147619075;
pub const SCE_KERNEL_ERROR_PA_NOT_AVAILABLE: SceKernelErrorCode = 2147619073;
pub const SCE_KERNEL_ERROR_PARTITION_ERROR: SceKernelErrorCode = 2147633152;
pub const SCE_KERNEL_ERROR_PHYADDR_ERROR: SceKernelErrorCode = 2147631360;
pub const SCE_KERNEL_ERROR_PHYADDR_NOT_USED: SceKernelErrorCode = 2147631363;
pub const SCE_KERNEL_ERROR_PHYADDR_USED: SceKernelErrorCode = 2147631362;
pub const SCE_KERNEL_ERROR_PHYMEM_ERROR: SceKernelErrorCode = 2147631872;
pub const SCE_KERNEL_ERROR_PHYMEMPART_NOT_EMPTY: SceKernelErrorCode = 2147631876;
pub const SCE_KERNEL_ERROR_PHYMEMPART_OUT_OF_INDEX: SceKernelErrorCode = 2147631879;
pub const SCE_KERNEL_ERROR_PLS_FULL: SceKernelErrorCode = 2147651587;
pub const SCE_KERNEL_ERROR_PMON_ERROR: SceKernelErrorCode = 2147648128;
pub const SCE_KERNEL_ERROR_PMON_NOT_CPU_MODE: SceKernelErrorCode = 2147648130;
pub const SCE_KERNEL_ERROR_PMON_NOT_THREAD_MODE: SceKernelErrorCode = 2147648129;
pub const SCE_KERNEL_ERROR_PRELOAD_FAILED: SceKernelErrorCode = 2147668208;
pub const SCE_KERNEL_ERROR_PRELOAD_FIOS2_FAILED: SceKernelErrorCode = 2147668210;
pub const SCE_KERNEL_ERROR_PRELOAD_LIBC_FAILED: SceKernelErrorCode = 2147668209;
pub const SCE_KERNEL_ERROR_PROCESS_CALLBACK_NOTFOUND: SceKernelErrorCode = 2147651589;
pub const SCE_KERNEL_ERROR_PROCESS_CANNOT_REMAP_MEMBLOCK: SceKernelErrorCode = 2147633928;
pub const SCE_KERNEL_ERROR_PROCESS_EVENT_INHIBITED: SceKernelErrorCode = 2147651596;
pub const SCE_KERNEL_ERROR_PROCESSMGR_ERROR: SceKernelErrorCode = 2147651584;
pub const SCE_KERNEL_ERROR_PROCESS_REMAINING: SceKernelErrorCode = 2147651594;
pub const SCE_KERNEL_ERROR_RUNNING: SceKernelErrorCode = 2147647529;
pub const SCE_KERNEL_ERROR_RW_LOCK_ERROR: SceKernelErrorCode = 2147647968;
pub const SCE_KERNEL_ERROR_RW_LOCK_FAILED_TO_LOCK: SceKernelErrorCode = 2147647974;
pub const SCE_KERNEL_ERROR_RW_LOCK_FAILED_TO_UNLOCK: SceKernelErrorCode = 2147647975;
pub const SCE_KERNEL_ERROR_RW_LOCK_LOCK_OVF: SceKernelErrorCode = 2147647971;
pub const SCE_KERNEL_ERROR_RW_LOCK_NOT_OWNED: SceKernelErrorCode = 2147647972;
pub const SCE_KERNEL_ERROR_RW_LOCK_RECURSIVE: SceKernelErrorCode = 2147647970;
pub const SCE_KERNEL_ERROR_RW_LOCK_UNLOCK_UDF: SceKernelErrorCode = 2147647973;
pub const SCE_KERNEL_ERROR_SEMA_ERROR: SceKernelErrorCode = 2147647744;
pub const SCE_KERNEL_ERROR_SEMA_OVF: SceKernelErrorCode = 2147647747;
pub const SCE_KERNEL_ERROR_SEMA_ZERO: SceKernelErrorCode = 2147647746;
pub const SCE_KERNEL_ERROR_SIGNAL_ERROR: SceKernelErrorCode = 2147647776;
pub const SCE_KERNEL_ERROR_SIMPLE_EVENT_ERROR: SceKernelErrorCode = 2147648096;
pub const SCE_KERNEL_ERROR_SYSMEM_ADDRESS_SPACE_ERROR: SceKernelErrorCode = 2147633664;
pub const SCE_KERNEL_ERROR_SYSMEM_BITHEAP_ERROR: SceKernelErrorCode = 2147634432;
pub const SCE_KERNEL_ERROR_SYSMEM_CANNOT_ALLOCATE_UIDENTRY: SceKernelErrorCode = 2147632389;
pub const SCE_KERNEL_ERROR_SYSMEM_ERROR: SceKernelErrorCode = 2147631104;
pub const SCE_KERNEL_ERROR_SYSMEM_INVALID_UID_RANGE: SceKernelErrorCode = 2147632387;
pub const SCE_KERNEL_ERROR_SYSMEM_MEMBLOCK_ERROR: SceKernelErrorCode = 2147633920;
pub const SCE_KERNEL_ERROR_SYSMEM_NAMEHEAP_ERROR: SceKernelErrorCode = 2147634688;
pub const SCE_KERNEL_ERROR_SYSMEM_NO_VALID_UID: SceKernelErrorCode = 2147632388;
pub const SCE_KERNEL_ERROR_SYSMEM_PHYMEMLOW_ERROR: SceKernelErrorCode = 2147634176;
pub const SCE_KERNEL_ERROR_SYSMEM_UID_INVALID_ARGUMENT: SceKernelErrorCode = 2147632386;
pub const SCE_KERNEL_ERROR_SYSTIMER_ERROR: SceKernelErrorCode = 2147644160;
pub const SCE_KERNEL_ERROR_THREAD_ERROR: SceKernelErrorCode = 2147647520;
pub const SCE_KERNEL_ERROR_THREAD_EVENT_ERROR: SceKernelErrorCode = 2147647584;
pub const SCE_KERNEL_ERROR_THREADMGR_ERROR: SceKernelErrorCode = 2147647488;
pub const SCE_KERNEL_ERROR_THREAD_STOPPED: SceKernelErrorCode = 2147647532;
pub const SCE_KERNEL_ERROR_THREAD_SUSPENDED: SceKernelErrorCode = 2147647533;
pub const SCE_KERNEL_ERROR_TIMER_COUNTING: SceKernelErrorCode = 2147644163;
pub const SCE_KERNEL_ERROR_TIMER_ERROR: SceKernelErrorCode = 2147648064;
pub const SCE_KERNEL_ERROR_TIMER_NOT_ALLOCATED: SceKernelErrorCode = 2147644162;
pub const SCE_KERNEL_ERROR_TIMER_STOPPED: SceKernelErrorCode = 2147644164;
pub const SCE_KERNEL_ERROR_TRANSFERRED: SceKernelErrorCode = 2147643910;
pub const SCE_KERNEL_ERROR_UID_CANNOT_FIND_BY_NAME: SceKernelErrorCode = 2147632394;
pub const SCE_KERNEL_ERROR_UID_ERROR: SceKernelErrorCode = 2147632384;
pub const SCE_KERNEL_ERROR_UID_MAX_OPEN: SceKernelErrorCode = 2147632396;
pub const SCE_KERNEL_ERROR_UID_NAME_TOO_LONG: SceKernelErrorCode = 2147631106;
pub const SCE_KERNEL_ERROR_UID_NOT_VISIBLE: SceKernelErrorCode = 2147632395;
pub const SCE_KERNEL_ERROR_UID_RL_OVERFLOW: SceKernelErrorCode = 2147632397;
pub const SCE_KERNEL_ERROR_UNKNOWN_ALARM_ID: SceKernelErrorCode = 2147647681;
pub const SCE_KERNEL_ERROR_UNKNOWN_CALLBACK_ID: SceKernelErrorCode = 2147647649;
pub const SCE_KERNEL_ERROR_UNKNOWN_COND_ID: SceKernelErrorCode = 2147647905;
pub const SCE_KERNEL_ERROR_UNKNOWN_EVENT_ID: SceKernelErrorCode = 2147648001;
pub const SCE_KERNEL_ERROR_UNKNOWN_EVF_ID: SceKernelErrorCode = 2147647713;
pub const SCE_KERNEL_ERROR_UNKNOWN_FAST_MUTEX_ID: SceKernelErrorCode = 2147647841;
pub const SCE_KERNEL_ERROR_UNKNOWN_LW_COND_ID: SceKernelErrorCode = 2147647937;
pub const SCE_KERNEL_ERROR_UNKNOWN_LW_MUTEX_ID: SceKernelErrorCode = 2147647873;
pub const SCE_KERNEL_ERROR_UNKNOWN_MSG_PIPE_ID: SceKernelErrorCode = 2147648033;
pub const SCE_KERNEL_ERROR_UNKNOWN_MUTEX_ID: SceKernelErrorCode = 2147647809;
pub const SCE_KERNEL_ERROR_UNKNOWN_PHYMEMLOW_TYPE: SceKernelErrorCode = 2147634178;
pub const SCE_KERNEL_ERROR_UNKNOWN_RW_LOCK_ID: SceKernelErrorCode = 2147647969;
pub const SCE_KERNEL_ERROR_UNKNOWN_SEMA_ID: SceKernelErrorCode = 2147647745;
pub const SCE_KERNEL_ERROR_UNKNOWN_SIMPLE_EVENT_ID: SceKernelErrorCode = 2147648097;
pub const SCE_KERNEL_ERROR_UNKNOWN_THREAD_EVENT_ID: SceKernelErrorCode = 2147647585;
pub const SCE_KERNEL_ERROR_UNKNOWN_THREAD_ID: SceKernelErrorCode = 2147647521;
pub const SCE_KERNEL_ERROR_UNKNOWN_TIMER_ID: SceKernelErrorCode = 2147648065;
pub const SCE_KERNEL_ERROR_UNKNOWN_UID: SceKernelErrorCode = 2147647489;
pub const SCE_KERNEL_ERROR_UNKNOWN_WORK_QUEUE_ID: SceKernelErrorCode = 2147648257;
pub const SCE_KERNEL_ERROR_UNKNOWN_WORK_TASK_ID: SceKernelErrorCode = 2147648258;
pub const SCE_KERNEL_ERROR_UNSUP: SceKernelErrorCode = 2147614724;
pub const SCE_KERNEL_ERROR_VA2PA_FAULT: SceKernelErrorCode = 2147622919;
pub const SCE_KERNEL_ERROR_VA2PA_MAPPED: SceKernelErrorCode = 2147622920;
pub const SCE_KERNEL_ERROR_VALIDATION_CHECK_FAILED: SceKernelErrorCode = 2147622921;
pub const SCE_KERNEL_ERROR_VARANGE_IS_NOT_PHYSICAL_CONTINUOUS: SceKernelErrorCode = 2147631107;
pub const SCE_KERNEL_ERROR_VIRPAGE_ERROR: SceKernelErrorCode = 2147632640;
pub const SCE_KERNEL_ERROR_WAIT_CANCEL: SceKernelErrorCode = 2147647495;
pub const SCE_KERNEL_ERROR_WAIT_CANCEL_COND: SceKernelErrorCode = 2147647909;
pub const SCE_KERNEL_ERROR_WAIT_CANCEL_MUTEX: SceKernelErrorCode = 2147647907;
pub const SCE_KERNEL_ERROR_WAIT_DELETE: SceKernelErrorCode = 2147647494;
pub const SCE_KERNEL_ERROR_WAIT_DELETE_COND: SceKernelErrorCode = 2147647908;
pub const SCE_KERNEL_ERROR_WAIT_DELETE_LW_COND: SceKernelErrorCode = 2147647939;
pub const SCE_KERNEL_ERROR_WAIT_DELETE_LW_MUTEX: SceKernelErrorCode = 2147647938;
pub const SCE_KERNEL_ERROR_WAIT_DELETE_MUTEX: SceKernelErrorCode = 2147647906;
pub const SCE_KERNEL_ERROR_WAIT_TIMEOUT: SceKernelErrorCode = 2147647493;
pub const SCE_KERNEL_ERROR_WORK_QUEUE: SceKernelErrorCode = 2147648256;
pub const SCE_KERNEL_HEAP_ATTR_HAS_AUTO_EXTEND: SceKernelHeapAttr = 1;
pub const SCE_KERNEL_HEAP_ATTR_HAS_MEMORY_TYPE: SceKernelHeapAttr = 1024;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_CDRAM_L1WBWA_RW: u32 = 1077952518;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_CDRAM_RW: u32 = 1077968902;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_IO_NC_R: u32 = 537952260;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_IO_NC_RW: u32 = 537952262;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_IO_R: u32 = 537921540;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_IO_RW: u32 = 537921542;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_IO_SO_R: u32 = 537920004;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_IO_SO_RW: u32 = 537920006;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_KMP_GAME_RW: u32 = 3226521606;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_KMP_TOOL_RW: u32 = 3237007366;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_CDIALOG_R: u32 = 278974468;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_CDIALOG_RW: u32 = 278974470;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_DEVICE_RW: u32 = 270534662;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_GAME_RW: u32 = 273731590;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_L1WBWA_RW: u32 = 270548998;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_NC_R: u32 = 270565380;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_NC_RW: u32 = 270565382;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_PHYCONT_NC_R: u32 = 813727748;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_PHYCONT_NC_RW: u32 = 813727750;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_PHYCONT_R: u32 = 276877316;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_PHYCONT_RW: u32 = 276877318;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_R: u32 = 270585860;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_RW: u32 = 270585862;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_RX: u32 = 270585861;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_SO_RW: u32 = 270533126;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_UMAIN_NC_RW: u32 = 281051142;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_ROOT_UMAIN_RW: u32 = 281071622;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_RW: u32 = 270585862;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_RX: u32 = 270585861;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TMP_FS_GAME_NC_R: u32 = 320897028;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TMP_FS_GAME_NC_RW: u32 = 320897030;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TMP_FS_GAME_R: u32 = 304140292;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TMP_FS_GAME_RW: u32 = 304140294;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TMP_GAME_RW: u32 = 1615908870;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TMP_NC_R: u32 = 1612742660;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TMP_NC_RW: u32 = 1612742662;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TMP_R: u32 = 1612763140;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TMP_RW: u32 = 1612763142;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TOOL_NC_R: u32 = 284196868;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TOOL_NC_RW: u32 = 284196870;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TOOL_R: u32 = 284217348;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TOOL_RW: u32 = 284217350;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_TOOL_RX: u32 = 284217349;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_UNK_11208006_NC_RW: u32 = 287342598;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_UNK_60208014_NC_R: u32 = 1612742676;
pub const SCE_KERNEL_MEMBLOCK_TYPE_KERNEL_UNK_60208016_NC_RW: u32 = 1612742678;
pub const SCE_KERNEL_MEMBLOCK_TYPE_RW_UNK0: u32 = 1612763142;
pub const SCE_KERNEL_MEMBLOCK_TYPE_SHARED_RX: u32 = 59822160;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_NC_R: u32 = 237011008;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_NC_RW: u32 = 237011040;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_R: u32 = 237031488;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_RW: u32 = 237031520;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDIALOG_RX: u32 = 237031504;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_L1WBWA_RW: u32 = 155205728;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_R: u32 = 155222080;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_CDRAM_RW: u32 = 155222112;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_IO_DEVICE_R: u32 = 185600064;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_IO_DEVICE_RW: u32 = 185600096;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_IO_SO_R: u32 = 185598528;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_IO_SO_RW: u32 = 185598560;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_CDIALOG_NC_RW: u32 = 211845216;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_CDIALOG_RW: u32 = 211865696;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_DEVICE_RW: u32 = 203425888;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_GAME_RW: u32 = 206622816;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_NC_RW: u32 = 203456608;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_NC_R: u32 = 226525248;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_NC_RW: u32 = 226525280;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_R: u32 = 209768512;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_PHYCONT_RW: u32 = 209768544;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_R: u32 = 203477056;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_RW: u32 = 203477088;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_RX: u32 = 203477072;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_TOOL_NC_RW: u32 = 217088096;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_TOOL_RW: u32 = 217108576;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_MAIN_TOOL_RX: u32 = 217108560;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_RW: u32 = 203477088;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_RW_UNCACHE: u32 = 203456608;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_RX: u32 = 203477072;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_CDIALOG_R: u32 = 60870720;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_CDIALOG_RW: u32 = 60870752;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_CDIALOG_RX: u32 = 60870736;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_MAIN_R: u32 = 52482112;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_MAIN_RW: u32 = 52482144;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_MAIN_RX: u32 = 52482128;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_SHARED_R: u32 = 59822144;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_SHARED_RW: u32 = 59822176;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_SHARED_RX: u32 = 59822160;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_TOOL_R: u32 = 66113600;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_TOOL_RW: u32 = 66113632;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_SHARED_TOOL_RX: u32 = 66113616;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_TOOL_NC_RW: u32 = 217088096;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_TOOL_R: u32 = 149999680;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_TOOL_RW: u32 = 149999712;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_TOOL_RX: u32 = 149999696;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_TOOL_UNK_RW: u32 = 149999622;
pub const SCE_KERNEL_MEMBLOCK_TYPE_USER_UNK_0720D006_RW: u32 = 119590918;
pub const SCE_KERNEL_MEMORY_ACCESS_R: SceKernelMemoryAccessType = 4;
pub const SCE_KERNEL_MEMORY_ACCESS_W: SceKernelMemoryAccessType = 2;
pub const SCE_KERNEL_MEMORY_ACCESS_X: SceKernelMemoryAccessType = 1;
pub const SCE_KERNEL_MEMORY_REF_PERM_ANY: SceKernelMemoryRefPerm = 0;
pub const SCE_KERNEL_MEMORY_REF_PERM_KERN_R: SceKernelMemoryRefPerm = 16;
pub const SCE_KERNEL_MEMORY_REF_PERM_KERN_W: SceKernelMemoryRefPerm = 32;
pub const SCE_KERNEL_MEMORY_REF_PERM_KERN_X: SceKernelMemoryRefPerm = 64;
pub const SCE_KERNEL_MEMORY_REF_PERM_USER_R: SceKernelMemoryRefPerm = 1;
pub const SCE_KERNEL_MEMORY_REF_PERM_USER_W: SceKernelMemoryRefPerm = 2;
pub const SCE_KERNEL_MEMORY_REF_PERM_USER_X: SceKernelMemoryRefPerm = 4;
pub const SCE_KERNEL_MEMORY_TYPE_NORMAL: SceKernelMemoryType = 208;
pub const SCE_KERNEL_MEMORY_TYPE_NORMAL_NC: SceKernelMemoryType = 128;
pub const SCE_KERNEL_MODEL_VITA: SceKernelModel = 65536;
pub const SCE_KERNEL_MODEL_VITATV: SceKernelModel = 131072;
pub const SCE_KERNEL_MODULE_STATE_ENDED: SceKernelModuleState = 9;
pub const SCE_KERNEL_MODULE_STATE_READY: SceKernelModuleState = 2;
pub const SCE_KERNEL_MODULE_STATE_STARTED: SceKernelModuleState = 6;
pub const SCE_KERNEL_MUTEX_ATTR_CEILING: SceKernelMutexAttribute = 4;
pub const SCE_KERNEL_MUTEX_ATTR_RECURSIVE: SceKernelMutexAttribute = 2;
pub const SCE_KERNEL_OK: SceKernelErrorCode = 0;
pub const SCE_KERNEL_POWER_TICK_DEFAULT: SceKernelPowerTickType = 0;
pub const __SCE_KERNEL_POWER_TICK_DISABLE: SceKernelPowerTickType = 4294967295;
pub const SCE_KERNEL_POWER_TICK_DISABLE_AUTO_SUSPEND: SceKernelPowerTickType = 1;
pub const SCE_KERNEL_POWER_TICK_DISABLE_OLED_DIMMING: SceKernelPowerTickType = 6;
pub const SCE_KERNEL_POWER_TICK_DISABLE_OLED_OFF: SceKernelPowerTickType = 4;
pub const SCE_KERNEL_PRELOAD_INHIBIT_APPUTIL: SceKernelPreloadInhibit = 4194304;
pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBC: SceKernelPreloadInhibit = 65536;
pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBCDLG: SceKernelPreloadInhibit = 1048576;
pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBDBG: SceKernelPreloadInhibit = 131072;
pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBFIOS2: SceKernelPreloadInhibit = 2097152;
pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBPERF: SceKernelPreloadInhibit = 33554432;
pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBPVF: SceKernelPreloadInhibit = 16777216;
pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBSCEFT2: SceKernelPreloadInhibit = 8388608;
pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBSHELLSVC: SceKernelPreloadInhibit = 524288;
pub const SCE_KERNEL_PRELOAD_INHIBIT_NONE: SceKernelPreloadInhibit = 0;
pub const SCE_KERNEL_PROCESS_ID_SELF: u32 = 0;
pub const __SCE_KERNEL_PROCESS_PRIORITY_SYSTEM: SceKernelProcessPrioritySystem = 4294967295;
pub const SCE_KERNEL_PROCESS_PRIORITY_SYSTEM_DEFAULT: SceKernelProcessPrioritySystem = 96;
pub const SCE_KERNEL_PROCESS_PRIORITY_SYSTEM_HIGH: SceKernelProcessPrioritySystem = 32;
pub const SCE_KERNEL_PROCESS_PRIORITY_SYSTEM_LOW: SceKernelProcessPrioritySystem = 159;
pub const __SCE_KERNEL_PROCESS_PRIORITY_USER: SceKernelProcessPriorityUser = 4294967295;
pub const SCE_KERNEL_PROCESS_PRIORITY_USER_DEFAULT: SceKernelProcessPriorityUser = 96;
pub const SCE_KERNEL_PROCESS_PRIORITY_USER_HIGH: SceKernelProcessPriorityUser = 64;
pub const SCE_KERNEL_PROCESS_PRIORITY_USER_LOW: SceKernelProcessPriorityUser = 127;
pub const SCE_KERNEL_START_FAILED: u32 = 2;
pub const SCE_KERNEL_START_NO_RESIDENT: u32 = 1;
pub const SCE_KERNEL_START_RESIDENT: u32 = 0;
pub const SCE_KERNEL_START_SUCCESS: u32 = 0;
pub const SCE_KERNEL_STOP_CANCEL: u32 = 1;
pub const SCE_KERNEL_STOP_FAIL: u32 = 1;
pub const SCE_KERNEL_STOP_SUCCESS: u32 = 0;
pub const SCE_KERNEL_SYSROOT_SELF_INDEX_ENCDEC_W_PORTABILITY_SM: SceKernelSysrootSelfIndex = 2;
pub const SCE_KERNEL_SYSROOT_SELF_INDEX_GCAUTHMGR_SM: SceKernelSysrootSelfIndex = 0;
pub const SCE_KERNEL_SYSROOT_SELF_INDEX_RMAUTH_SM: SceKernelSysrootSelfIndex = 1;
pub const SCE_KERNEL_THREAD_CPU_AFFINITY_MASK_DEFAULT: u32 = 0;
pub const SCE_KERNEL_THREAD_ID_SELF: u32 = 0;
pub const SCE_KERNEL_TMID_Alarm: SceKernelIdListType = 10;
pub const SCE_KERNEL_TMID_Callback: SceKernelIdListType = 8;
pub const SCE_KERNEL_TMID_DelayThread: SceKernelIdListType = 65;
pub const SCE_KERNEL_TMID_DormantThread: SceKernelIdListType = 67;
pub const SCE_KERNEL_TMID_EventFlag: SceKernelIdListType = 3;
pub const SCE_KERNEL_TMID_Fpl: SceKernelIdListType = 6;
pub const SCE_KERNEL_TMID_Mbox: SceKernelIdListType = 4;
pub const SCE_KERNEL_TMID_Mpipe: SceKernelIdListType = 7;
pub const SCE_KERNEL_TMID_Semaphore: SceKernelIdListType = 2;
pub const SCE_KERNEL_TMID_SleepThread: SceKernelIdListType = 64;
pub const SCE_KERNEL_TMID_SuspendThread: SceKernelIdListType = 66;
pub const SCE_KERNEL_TMID_Thread: SceKernelIdListType = 1;
pub const SCE_KERNEL_TMID_ThreadEventHandler: SceKernelIdListType = 9;
pub const SCE_KERNEL_TMID_Vpl: SceKernelIdListType = 5;
pub const SCE_KERNEL_TMID_VTimer: SceKernelIdListType = 11;
pub const SCE_LOCATION_DATA_INVALID: f64 = -9999.0;
pub const __SCE_LOCATION_DIALOG_RESULT: SceLocationDialogResult = 4294967295;
pub const SCE_LOCATION_DIALOG_RESULT_DISABLE: SceLocationDialogResult = 1;
pub const SCE_LOCATION_DIALOG_RESULT_ENABLE: SceLocationDialogResult = 2;
pub const SCE_LOCATION_DIALOG_RESULT_NONE: SceLocationDialogResult = 0;
pub const __SCE_LOCATION_DIALOG_STATUS: SceLocationDialogStatus = 4294967295;
pub const SCE_LOCATION_DIALOG_STATUS_FINISHED: SceLocationDialogStatus = 2;
pub const SCE_LOCATION_DIALOG_STATUS_IDLE: SceLocationDialogStatus = 0;
pub const SCE_LOCATION_DIALOG_STATUS_RUNNING: SceLocationDialogStatus = 1;
pub const SCE_LOCATION_ERROR_DIALOG_RESULT_NONE: SceLocationErrorCode = 2148536844;
pub const SCE_LOCATION_ERROR_DISABLE_APPLICATION: SceLocationErrorCode = 2148536845;
pub const SCE_LOCATION_ERROR_FATAL_ERROR: SceLocationErrorCode = 2148537087;
pub const SCE_LOCATION_ERROR_FILE_IO: SceLocationErrorCode = 2148536962;
pub const SCE_LOCATION_ERROR_INVALID_ADDRESS: SceLocationErrorCode = 2148536836;
pub const SCE_LOCATION_ERROR_INVALID_FILE_FORMAT: SceLocationErrorCode = 2148536963;
pub const SCE_LOCATION_ERROR_INVALID_HANDLE: SceLocationErrorCode = 2148536837;
pub const SCE_LOCATION_ERROR_INVALID_HEADING_METHOD: SceLocationErrorCode = 2148536841;
pub const SCE_LOCATION_ERROR_INVALID_LOCATION_METHOD: SceLocationErrorCode = 2148536840;
pub const SCE_LOCATION_ERROR_INVALID_TITLE_ID: SceLocationErrorCode = 2148536966;
pub const SCE_LOCATION_ERROR_MULTIPLE_CALLBACK: SceLocationErrorCode = 2148536842;
pub const SCE_LOCATION_ERROR_MULTIPLE_CONFIRM: SceLocationErrorCode = 2148536846;
pub const SCE_LOCATION_ERROR_NO_MEMORY: SceLocationErrorCode = 2148536838;
pub const SCE_LOCATION_ERROR_NO_SERVER_MEMORY: SceLocationErrorCode = 2148536965;
pub const SCE_LOCATION_ERROR_NOT_RUNNING_CALLBACK: SceLocationErrorCode = 2148536843;
pub const SCE_LOCATION_ERROR_PROVIDER_UNAVAILABLE: SceLocationErrorCode = 2148536961;
pub const SCE_LOCATION_ERROR_TIME_OUT: SceLocationErrorCode = 2148536964;
pub const SCE_LOCATION_ERROR_TOO_MANY_HANDLES: SceLocationErrorCode = 2148536839;
pub const SCE_LOCATION_ERROR_UNAUTHORIZED: SceLocationErrorCode = 2148536960;
pub const __SCE_LOCATION_HMETHOD: SceLocationHeadingMethod = 4294967295;
pub const SCE_LOCATION_HMETHOD_AUTO: SceLocationHeadingMethod = 1;
pub const SCE_LOCATION_HMETHOD_CAMERA: SceLocationHeadingMethod = 4;
pub const SCE_LOCATION_HMETHOD_HORIZONTAL: SceLocationHeadingMethod = 3;
pub const SCE_LOCATION_HMETHOD_NONE: SceLocationHeadingMethod = 0;
pub const SCE_LOCATION_HMETHOD_VERTICAL: SceLocationHeadingMethod = 2;
pub const SCE_LOCATION_INFO_DENIED_BY_USER: SceLocationErrorCode = 2148536835;
pub const SCE_LOCATION_INFO_GET_LOCATION_CANCELED: SceLocationErrorCode = 2148536834;
pub const SCE_LOCATION_INFO_INSUFFICIENT_INFORMATION: SceLocationErrorCode = 2148536833;
pub const SCE_LOCATION_INFO_UNDETERMINED_LOCATION: SceLocationErrorCode = 2148536832;
pub const __SCE_LOCATION_LMETHOD: SceLocationLocationMethod = 4294967295;
pub const SCE_LOCATION_LMETHOD_3G: SceLocationLocationMethod = 4;
pub const SCE_LOCATION_LMETHOD_AGPS_AND_3G_AND_WIFI: SceLocationLocationMethod = 1;
pub const SCE_LOCATION_LMETHOD_GPS: SceLocationLocationMethod = 5;
pub const SCE_LOCATION_LMETHOD_GPS_AND_WIFI: SceLocationLocationMethod = 2;
pub const SCE_LOCATION_LMETHOD_NONE: SceLocationLocationMethod = 0;
pub const SCE_LOCATION_LMETHOD_WIFI: SceLocationLocationMethod = 3;
pub const __SCE_LOCATION_PERMISSION: SceLocationPermissionStatus = 4294967295;
pub const SCE_LOCATION_PERMISSION_ALLOW: SceLocationPermissionStatus = 1;
pub const __SCE_LOCATION_PERMISSION_APPLICATION: SceLocationPermissionApplicationStatus =
    4294967295;
pub const SCE_LOCATION_PERMISSION_APPLICATION_ALLOW: SceLocationPermissionApplicationStatus = 3;
pub const SCE_LOCATION_PERMISSION_APPLICATION_DENY: SceLocationPermissionApplicationStatus = 2;
pub const SCE_LOCATION_PERMISSION_APPLICATION_INIT: SceLocationPermissionApplicationStatus = 1;
pub const SCE_LOCATION_PERMISSION_APPLICATION_NONE: SceLocationPermissionApplicationStatus = 0;
pub const SCE_LOCATION_PERMISSION_DENY: SceLocationPermissionStatus = 0;
pub const SCE_LOCATION_SUCCESS: SceLocationErrorCode = 0;
pub const SCE_MOTION_ERROR_ALREADY_SAMPLING: SceMotionErrorCode = 2151023111;
pub const SCE_MOTION_ERROR_CALIB_READ_FAIL: SceMotionErrorCode = 2151023108;
pub const SCE_MOTION_ERROR_DATA_INVALID: SceMotionErrorCode = 2151023104;
pub const SCE_MOTION_ERROR_MEM_IN_USE: SceMotionErrorCode = 2151023112;
pub const SCE_MOTION_ERROR_NON_INIT_ERR: SceMotionErrorCode = 2151023106;
pub const SCE_MOTION_ERROR_NOT_SAMPLING: SceMotionErrorCode = 2151023110;
pub const SCE_MOTION_ERROR_OUT_OF_BOUNDS: SceMotionErrorCode = 2151023109;
pub const SCE_MOTION_ERROR_READING: SceMotionErrorCode = 2151023105;
pub const SCE_MOTION_ERROR_STATE_INVALID: SceMotionErrorCode = 2151023107;
pub const SCE_MOTION_MAGFIELD_STABLE: SceMotionMagFieldStability = 2;
pub const SCE_MOTION_MAGFIELD_UNSTABLE: SceMotionMagFieldStability = 0;
pub const SCE_MOTION_MAGFIELD_UNUSED: SceMotionMagFieldStability = 1;
pub const SCE_MSG_DIALOG_BUTTON_ID_BUTTON1: SceMsgDialogButtonId = 1;
pub const SCE_MSG_DIALOG_BUTTON_ID_BUTTON2: SceMsgDialogButtonId = 2;
pub const SCE_MSG_DIALOG_BUTTON_ID_BUTTON3: SceMsgDialogButtonId = 3;
pub const SCE_MSG_DIALOG_BUTTON_ID_INVALID: SceMsgDialogButtonId = 0;
pub const SCE_MSG_DIALOG_BUTTON_ID_NO: SceMsgDialogButtonId = 2;
pub const SCE_MSG_DIALOG_BUTTON_ID_OK: SceMsgDialogButtonId = 1;
pub const SCE_MSG_DIALOG_BUTTON_ID_RETRY: SceMsgDialogButtonId = 3;
pub const SCE_MSG_DIALOG_BUTTON_ID_YES: SceMsgDialogButtonId = 1;
pub const SCE_MSG_DIALOG_BUTTON_TYPE_3BUTTONS: SceMsgDialogButtonType = 5;
pub const SCE_MSG_DIALOG_BUTTON_TYPE_CANCEL: SceMsgDialogButtonType = 4;
pub const SCE_MSG_DIALOG_BUTTON_TYPE_NONE: SceMsgDialogButtonType = 2;
pub const SCE_MSG_DIALOG_BUTTON_TYPE_OK: SceMsgDialogButtonType = 0;
pub const SCE_MSG_DIALOG_BUTTON_TYPE_OK_CANCEL: SceMsgDialogButtonType = 3;
pub const SCE_MSG_DIALOG_BUTTON_TYPE_YESNO: SceMsgDialogButtonType = 1;
pub const SCE_MSG_DIALOG_ENV_FLAG_DEFAULT: SceMsgDialogEnvFlag = 0;
pub const SCE_MSG_DIALOG_ERROR_PARAM: SceMsgDialogErrorCode = 2148534785;
pub const SCE_MSG_DIALOG_FONT_SIZE_DEFAULT: SceMsgDialogFontSize = 0;
pub const SCE_MSG_DIALOG_FONT_SIZE_SMALL: SceMsgDialogFontSize = 1;
pub const SCE_MSG_DIALOG_MODE_ERROR_CODE: SceMsgDialogMode = 3;
pub const SCE_MSG_DIALOG_MODE_INVALID: SceMsgDialogMode = 0;
pub const SCE_MSG_DIALOG_MODE_PROGRESS_BAR: SceMsgDialogMode = 4;
pub const SCE_MSG_DIALOG_MODE_SYSTEM_MSG: SceMsgDialogMode = 2;
pub const SCE_MSG_DIALOG_MODE_USER_MSG: SceMsgDialogMode = 1;
pub const SCE_MSG_DIALOG_PROGRESSBAR_TARGET_BAR_DEFAULT: SceMsgDialogProgressBarTarget = 0;
pub const SCE_MSG_DIALOG_PROGRESSBAR_TYPE_PERCENTAGE: SceMsgDialogProgressBarType = 0;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_INVALID: SceMsgDialogSystemMessageType = 0;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_MAGNETIC_CALIBRATION: SceMsgDialogSystemMessageType = 3;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_NEED_MC_CONTINUE: SceMsgDialogSystemMessageType = 7;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_NEED_MC_OPERATION: SceMsgDialogSystemMessageType = 8;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_NOSPACE: SceMsgDialogSystemMessageType = 2;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_TRC_EMPTY_STORE: SceMsgDialogSystemMessageType = 103;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_TRC_MIC_DISABLED: SceMsgDialogSystemMessageType = 100;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_TRC_WIFI_REQUIRED_APPLICATION: SceMsgDialogSystemMessageType =
    102;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_TRC_WIFI_REQUIRED_OPERATION: SceMsgDialogSystemMessageType =
    101;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_WAIT: SceMsgDialogSystemMessageType = 1;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_WAIT_CANCEL: SceMsgDialogSystemMessageType = 6;
pub const SCE_MSG_DIALOG_SYSMSG_TYPE_WAIT_SMALL: SceMsgDialogSystemMessageType = 5;
pub const SCE_MSG_DIALOG_USER_MSG_SIZE: u32 = 512;
pub const SCE_NET_ADHOCCTL_ADHOCID_LEN: u32 = 9;
pub const SCE_NET_ADHOCCTL_ADHOCTYPE_PRODUCT_ID: ScePspnetAdhocctlAdhocType = 0;
pub const SCE_NET_ADHOCCTL_ADHOCTYPE_RESERVED: ScePspnetAdhocctlAdhocType = 1;
pub const SCE_NET_ADHOCCTL_ADHOCTYPE_SYSTEM: ScePspnetAdhocctlAdhocType = 2;
pub const SCE_NET_ADHOCCTL_BSSID_LEN: u32 = 6;
pub const SCE_NET_ADHOCCTL_GROUPNAME_LEN: u32 = 8;
pub const SCE_NET_ADHOCCTL_NICKNAME_LEN: u32 = 128;
pub const SCE_NET_ADHOC_EV_ACCEPT: ScePspnetAdhocEvent = 8;
pub const SCE_NET_ADHOC_EV_ALERT: ScePspnetAdhocEvent = 1024;
pub const SCE_NET_ADHOC_EV_CONNECT: ScePspnetAdhocEvent = 4;
pub const SCE_NET_ADHOC_EV_DELETE: ScePspnetAdhocEvent = 512;
pub const SCE_NET_ADHOC_EV_DISCONNECT: ScePspnetAdhocEvent = 2048;
pub const SCE_NET_ADHOC_EV_FLUSH: ScePspnetAdhocEvent = 16;
pub const SCE_NET_ADHOC_EV_INVALID: ScePspnetAdhocEvent = 256;
pub const SCE_NET_ADHOC_EV_RECV: ScePspnetAdhocEvent = 2;
pub const SCE_NET_ADHOC_EV_SEND: ScePspnetAdhocEvent = 1;
pub const SCE_NET_ADHOC_F_ALERTACCEPT: ScePspnetAdhocFlags = 256;
pub const SCE_NET_ADHOC_F_ALERTCONNECT: ScePspnetAdhocFlags = 128;
pub const SCE_NET_ADHOC_F_ALERTFLUSH: ScePspnetAdhocFlags = 512;
pub const SCE_NET_ADHOC_F_ALERTPOLL: ScePspnetAdhocFlags = 64;
pub const SCE_NET_ADHOC_F_ALERTRECV: ScePspnetAdhocFlags = 32;
pub const SCE_NET_ADHOC_F_ALERTSEND: ScePspnetAdhocFlags = 16;
pub const SCE_NET_ADHOC_F_NONBLOCK: ScePspnetAdhocFlags = 1;
pub const SCE_NET_ADHOC_PDP_MFS: u32 = 1444;
pub const SCE_NET_ADHOC_PDP_MTU: u32 = 65523;
pub const SCE_NET_ADHOC_PORT: u32 = 3658;
pub const SCE_NET_ADHOC_PTP_MSS: u32 = 1444;
pub const SCE_NET_ADHOC_PTP_STATE_CLOSED: ScePspnetAdhocPtpState = 0;
pub const SCE_NET_ADHOC_PTP_STATE_ESTABLISHED: ScePspnetAdhocPtpState = 4;
pub const SCE_NET_ADHOC_PTP_STATE_LISTEN: ScePspnetAdhocPtpState = 1;
pub const SCE_NET_ADHOC_PTP_STATE_SYN_RCVD: ScePspnetAdhocPtpState = 3;
pub const SCE_NET_ADHOC_PTP_STATE_SYN_SENT: ScePspnetAdhocPtpState = 2;
pub const SCE_NET_AF_INET: u32 = 2;
pub const SCE_NETCHECK_DIALOG_AGE_RESTRICTION_COUNT_MAX: u32 = 200;
pub const SCE_NETCHECK_DIALOG_COUNTRY_CODE_LEN: u32 = 2;
pub const SCE_NETCHECK_DIALOG_ERROR_INVALID_MODE: SceNetCheckDialoErrorCode = 2148535298;
pub const SCE_NETCHECK_DIALOG_ERROR_INVALID_PSPADHOC_PARAM: SceNetCheckDialoErrorCode = 2148535303;
pub const SCE_NETCHECK_DIALOG_ERROR_INVALID_TIMEOUT_PARAM: SceNetCheckDialoErrorCode = 2148535304;
pub const SCE_NETCHECK_DIALOG_ERROR_LACK_OF_LIBHTTP_POOL_SIZE: SceNetCheckDialoErrorCode =
    2148535299;
pub const SCE_NETCHECK_DIALOG_ERROR_LACK_OF_LIBSSL_POOL_SIZE: SceNetCheckDialoErrorCode =
    2148535300;
pub const SCE_NETCHECK_DIALOG_ERROR_LATEST_PATCH_PKG_EXIST: SceNetCheckDialoErrorCode = 2148535301;
pub const SCE_NETCHECK_DIALOG_ERROR_PARAM: SceNetCheckDialoErrorCode = 2148535297;
pub const SCE_NETCHECK_DIALOG_ERROR_PSN_AGE_RESTRICTION: SceNetCheckDialoErrorCode = 2148535305;
pub const SCE_NETCHECK_DIALOG_ERROR_SIGN_OUT: SceNetCheckDialoErrorCode = 2148535302;
pub const SCE_NETCHECK_DIALOG_INITIAL_AGE_RESTRICTION: i32 = -1;
pub const SCE_NETCHECK_DIALOG_LEAST_HTTP_POOL_SIZE: u32 = 36864;
pub const SCE_NETCHECK_DIALOG_LEAST_SSL_POOL_SIZE: u32 = 98304;
pub const SCE_NETCHECK_DIALOG_MODE_ADHOC_CONN: SceNetCheckDialogMode = 1;
pub const SCE_NETCHECK_DIALOG_MODE_INVALID: SceNetCheckDialogMode = 0;
pub const SCE_NETCHECK_DIALOG_MODE_PS3_CONNECT: SceNetCheckDialogMode = 4;
pub const SCE_NETCHECK_DIALOG_MODE_PSN: SceNetCheckDialogMode = 2;
pub const SCE_NETCHECK_DIALOG_MODE_PSN_ONLINE: SceNetCheckDialogMode = 3;
pub const SCE_NETCHECK_DIALOG_MODE_PSP_ADHOC_CONN: SceNetCheckDialogMode = 5;
pub const SCE_NETCHECK_DIALOG_MODE_PSP_ADHOC_CREATE: SceNetCheckDialogMode = 6;
pub const SCE_NETCHECK_DIALOG_MODE_PSP_ADHOC_JOIN: SceNetCheckDialogMode = 7;
pub const SCE_NETCHECK_DIALOG_PS3_CONNECT_ACTION_ENTER: SceNetCheckDialogPS3ConnectAction = 0;
pub const SCE_NETCHECK_DIALOG_PS3_CONNECT_ACTION_LEAVE: SceNetCheckDialogPS3ConnectAction = 1;
pub const SCE_NETCTL_INFO_CONFIG_NAME_LEN_MAX: u32 = 64;
pub const SCE_NETCTL_INFO_GET_BSSID: SceNetCtlInfoType = 6;
pub const SCE_NETCTL_INFO_GET_CHANNEL: SceNetCtlInfoType = 11;
pub const SCE_NETCTL_INFO_GET_CNF_NAME: SceNetCtlInfoType = 1;
pub const SCE_NETCTL_INFO_GET_DEFAULT_ROUTE: SceNetCtlInfoType = 17;
pub const SCE_NETCTL_INFO_GET_DEVICE: SceNetCtlInfoType = 2;
pub const SCE_NETCTL_INFO_GET_DHCP_HOSTNAME: SceNetCtlInfoType = 13;
pub const SCE_NETCTL_INFO_GET_ETHER_ADDR: SceNetCtlInfoType = 3;
pub const SCE_NETCTL_INFO_GET_HTTP_PROXY_CONFIG: SceNetCtlInfoType = 20;
pub const SCE_NETCTL_INFO_GET_HTTP_PROXY_PORT: SceNetCtlInfoType = 22;
pub const SCE_NETCTL_INFO_GET_HTTP_PROXY_SERVER: SceNetCtlInfoType = 21;
pub const SCE_NETCTL_INFO_GET_IP_ADDRESS: SceNetCtlInfoType = 15;
pub const SCE_NETCTL_INFO_GET_IP_CONFIG: SceNetCtlInfoType = 12;
pub const SCE_NETCTL_INFO_GET_LINK: SceNetCtlInfoType = 5;
pub const SCE_NETCTL_INFO_GET_MTU: SceNetCtlInfoType = 4;
pub const SCE_NETCTL_INFO_GET_NETMASK: SceNetCtlInfoType = 16;
pub const SCE_NETCTL_INFO_GET_PPPOE_AUTH_NAME: SceNetCtlInfoType = 14;
pub const SCE_NETCTL_INFO_GET_PRIMARY_DNS: SceNetCtlInfoType = 18;
pub const SCE_NETCTL_INFO_GET_RSSI_DBM: SceNetCtlInfoType = 9;
pub const SCE_NETCTL_INFO_GET_RSSI_PERCENTAGE: SceNetCtlInfoType = 10;
pub const SCE_NETCTL_INFO_GET_SECONDARY_DNS: SceNetCtlInfoType = 19;
pub const SCE_NETCTL_INFO_GET_SSID: SceNetCtlInfoType = 7;
pub const SCE_NETCTL_INFO_GET_WIFI_SECURITY: SceNetCtlInfoType = 8;
pub const SCE_NETCTL_INFO_SSID_LEN_MAX: u32 = 32;
pub const SCE_NETCTL_STATE_CONNECTED: SceNetCtlState = 3;
pub const SCE_NETCTL_STATE_CONNECTING: SceNetCtlState = 1;
pub const SCE_NETCTL_STATE_DISCONNECTED: SceNetCtlState = 0;
pub const SCE_NETCTL_STATE_FINALIZING: SceNetCtlState = 2;
pub const SCE_NET_DEBUG_NAME_LEN_MAX: u32 = 31;
pub const SCE_NET_DUMP_ABORT_FLAG_PRESERVATION: SceNetDumpType = 1;
pub const SCE_NET_DUMP_DONTWAIT: SceNetDumpType = 32;
pub const SCE_NET_DUMP_OVERFLOW: SceNetDumpType = 64;
pub const SCE_NET_DUMP_PEEK: SceNetDumpType = 16;
pub const SCE_NET_E2BIG: SceNetKernelErrorCode = 7;
pub const SCE_NET_EACCES: SceNetKernelErrorCode = 13;
pub const SCE_NET_EADDRINUSE: SceNetKernelErrorCode = 48;
pub const SCE_NET_EADDRNOTAVAIL: SceNetKernelErrorCode = 49;
pub const SCE_NET_EADHOC: SceNetKernelErrorCode = 160;
pub const SCE_NET_EAFNOSUPPORT: SceNetKernelErrorCode = 47;
pub const SCE_NET_EAGAIN: SceNetKernelErrorCode = 35;
pub const SCE_NET_EALREADY: SceNetKernelErrorCode = 37;
pub const SCE_NET_EAUTH: SceNetKernelErrorCode = 80;
pub const SCE_NET_EBADF: SceNetKernelErrorCode = 9;
pub const SCE_NET_EBADMSG: SceNetKernelErrorCode = 88;
pub const SCE_NET_EBADRPC: SceNetKernelErrorCode = 72;
pub const SCE_NET_EBUSY: SceNetKernelErrorCode = 16;
pub const SCE_NET_ECALLBACK: SceNetLibnetErrorCode = 203;
pub const SCE_NET_ECANCELED: SceNetKernelErrorCode = 87;
pub const SCE_NET_ECHILD: SceNetKernelErrorCode = 10;
pub const SCE_NET_ECONNABORTED: SceNetKernelErrorCode = 53;
pub const SCE_NET_ECONNREFUSED: SceNetKernelErrorCode = 61;
pub const SCE_NET_ECONNRESET: SceNetKernelErrorCode = 54;
pub const SCE_NET_EDEADLK: SceNetKernelErrorCode = 11;
pub const SCE_NET_EDESTADDRREQ: SceNetKernelErrorCode = 39;
pub const SCE_NET_EDISABLEDIF: SceNetKernelErrorCode = 161;
pub const SCE_NET_EDOM: SceNetKernelErrorCode = 33;
pub const SCE_NET_EDQUOT: SceNetKernelErrorCode = 69;
pub const SCE_NET_EEXIST: SceNetKernelErrorCode = 17;
pub const SCE_NET_EFAULT: SceNetKernelErrorCode = 14;
pub const SCE_NET_EFBIG: SceNetKernelErrorCode = 27;
pub const SCE_NET_EFTYPE: SceNetKernelErrorCode = 79;
pub const SCE_NET_EHOSTDOWN: SceNetKernelErrorCode = 64;
pub const SCE_NET_EHOSTUNREACH: SceNetKernelErrorCode = 65;
pub const SCE_NET_EIDRM: SceNetKernelErrorCode = 82;
pub const SCE_NET_EILSEQ: SceNetKernelErrorCode = 85;
pub const SCE_NET_EINPROGRESS: SceNetKernelErrorCode = 36;
pub const SCE_NET_EINTERNAL: SceNetLibnetErrorCode = 204;
pub const SCE_NET_EINTR: SceNetKernelErrorCode = 4;
pub const SCE_NET_EINVAL: SceNetKernelErrorCode = 22;
pub const SCE_NET_EIO: SceNetKernelErrorCode = 5;
pub const SCE_NET_EISCONN: SceNetKernelErrorCode = 56;
pub const SCE_NET_EISDIR: SceNetKernelErrorCode = 21;
pub const SCE_NET_ELOOP: SceNetKernelErrorCode = 62;
pub const SCE_NET_EMFILE: SceNetKernelErrorCode = 24;
pub const SCE_NET_EMLINK: SceNetKernelErrorCode = 31;
pub const SCE_NET_EMSGSIZE: SceNetKernelErrorCode = 40;
pub const SCE_NET_EMULATION_FLAG_ETH0: SceNetEmulationFlag = 1;
pub const SCE_NET_EMULATION_FLAG_WLAN0: SceNetEmulationFlag = 2;
pub const SCE_NET_ENAMETOOLONG: SceNetKernelErrorCode = 63;
pub const SCE_NET_ENEEDAUTH: SceNetKernelErrorCode = 81;
pub const SCE_NET_ENETDOWN: SceNetKernelErrorCode = 50;
pub const SCE_NET_ENETRESET: SceNetKernelErrorCode = 52;
pub const SCE_NET_ENETUNREACH: SceNetKernelErrorCode = 51;
pub const SCE_NET_ENFILE: SceNetKernelErrorCode = 23;
pub const SCE_NET_ENOBUFS: SceNetKernelErrorCode = 55;
pub const SCE_NET_ENODATA: SceNetKernelErrorCode = 89;
pub const SCE_NET_ENODEV: SceNetKernelErrorCode = 19;
pub const SCE_NET_ENOENT: SceNetKernelErrorCode = 2;
pub const SCE_NET_ENOEXEC: SceNetKernelErrorCode = 8;
pub const SCE_NET_ENOLCK: SceNetKernelErrorCode = 77;
pub const SCE_NET_ENOLIBMEM: SceNetLibnetErrorCode = 201;
pub const SCE_NET_ENOMEM: SceNetKernelErrorCode = 12;
pub const SCE_NET_ENOMSG: SceNetKernelErrorCode = 83;
pub const SCE_NET_ENOPROTOOPT: SceNetKernelErrorCode = 42;
pub const SCE_NET_ENOSPC: SceNetKernelErrorCode = 28;
pub const SCE_NET_ENOSR: SceNetKernelErrorCode = 90;
pub const SCE_NET_ENOSTR: SceNetKernelErrorCode = 91;
pub const SCE_NET_ENOSYS: SceNetKernelErrorCode = 78;
pub const SCE_NET_ENOTBLK: SceNetKernelErrorCode = 15;
pub const SCE_NET_ENOTCONN: SceNetKernelErrorCode = 57;
pub const SCE_NET_ENOTDIR: SceNetKernelErrorCode = 20;
pub const SCE_NET_ENOTEMPTY: SceNetKernelErrorCode = 66;
pub const SCE_NET_ENOTINIT: SceNetLibnetErrorCode = 200;
pub const SCE_NET_ENOTSOCK: SceNetKernelErrorCode = 38;
pub const SCE_NET_ENOTSUP: SceNetKernelErrorCode = 86;
pub const SCE_NET_ENOTTY: SceNetKernelErrorCode = 25;
pub const SCE_NET_ENXIO: SceNetKernelErrorCode = 6;
pub const SCE_NET_EOPNOTSUPP: SceNetKernelErrorCode = 45;
pub const SCE_NET_EOVERFLOW: SceNetKernelErrorCode = 84;
pub const SCE_NET_EPERM: SceNetKernelErrorCode = 1;
pub const SCE_NET_EPFNOSUPPORT: SceNetKernelErrorCode = 46;
pub const SCE_NET_EPIPE: SceNetKernelErrorCode = 32;
pub const SCE_NET_EPOLL_ABORT_FLAG_PRESERVATION: u32 = 1;
pub const SCE_NET_EPOLL_CTL_ADD: SceNetEpollControlFlag = 1;
pub const SCE_NET_EPOLL_CTL_DEL: SceNetEpollControlFlag = 3;
pub const SCE_NET_EPOLL_CTL_MOD: SceNetEpollControlFlag = 2;
pub const SCE_NET_EPOLLDESCID: SceNetEpollEventType = 65536;
pub const SCE_NET_EPOLLERR: SceNetEpollEventType = 8;
pub const SCE_NET_EPOLLHUP: SceNetEpollEventType = 16;
pub const SCE_NET_EPOLLIN: SceNetEpollEventType = 1;
pub const SCE_NET_EPOLLOUT: SceNetEpollEventType = 2;
pub const SCE_NET_EPROCLIM: SceNetKernelErrorCode = 67;
pub const SCE_NET_EPROCUNAVAIL: SceNetKernelErrorCode = 76;
pub const SCE_NET_EPROGMISMATCH: SceNetKernelErrorCode = 75;
pub const SCE_NET_EPROGUNAVAIL: SceNetKernelErrorCode = 74;
pub const SCE_NET_EPROTONOSUPPORT: SceNetKernelErrorCode = 43;
pub const SCE_NET_EPROTOTYPE: SceNetKernelErrorCode = 41;
pub const SCE_NET_ERANGE: SceNetKernelErrorCode = 34;
pub const SCE_NET_EREMOTE: SceNetKernelErrorCode = 71;
pub const SCE_NET_ERESUME: SceNetKernelErrorCode = 162;
pub const SCE_NET_ERETURN: SceNetLibnetErrorCode = 205;
pub const SCE_NET_EROFS: SceNetKernelErrorCode = 30;
pub const SCE_NET_ERPCMISMATCH: SceNetKernelErrorCode = 73;
pub const SCE_NET_ERROR_E2BIG: SceNetErrorCode = 2151743751;
pub const SCE_NET_ERROR_EACCES: SceNetErrorCode = 2151743757;
pub const SCE_NET_ERROR_EADDRINUSE: SceNetErrorCode = 2151743792;
pub const SCE_NET_ERROR_EADDRNOTAVAIL: SceNetErrorCode = 2151743793;
pub const SCE_NET_ERROR_EADHOC: SceNetErrorCode = 2151743904;
pub const SCE_NET_ERROR_EAFNOSUPPORT: SceNetErrorCode = 2151743791;
pub const SCE_NET_ERROR_EAGAIN: SceNetErrorCode = 2151743779;
pub const SCE_NET_ERROR_EALREADY: SceNetErrorCode = 2151743781;
pub const SCE_NET_ERROR_EAUTH: SceNetErrorCode = 2151743824;
pub const SCE_NET_ERROR_EBADF: SceNetErrorCode = 2151743753;
pub const SCE_NET_ERROR_EBADMSG: SceNetErrorCode = 2151743832;
pub const SCE_NET_ERROR_EBADRPC: SceNetErrorCode = 2151743816;
pub const SCE_NET_ERROR_EBUSY: SceNetErrorCode = 2151743760;
pub const SCE_NET_ERROR_ECALLBACK: SceNetErrorCode = 2151743947;
pub const SCE_NET_ERROR_ECANCELED: SceNetErrorCode = 2151743831;
pub const SCE_NET_ERROR_ECHILD: SceNetErrorCode = 2151743754;
pub const SCE_NET_ERROR_ECONNABORTED: SceNetErrorCode = 2151743797;
pub const SCE_NET_ERROR_ECONNREFUSED: SceNetErrorCode = 2151743805;
pub const SCE_NET_ERROR_ECONNRESET: SceNetErrorCode = 2151743798;
pub const SCE_NET_ERROR_EDEADLK: SceNetErrorCode = 2151743755;
pub const SCE_NET_ERROR_EDESTADDRREQ: SceNetErrorCode = 2151743783;
pub const SCE_NET_ERROR_EDISABLEDIF: SceNetErrorCode = 2151743905;
pub const SCE_NET_ERROR_EDOM: SceNetErrorCode = 2151743777;
pub const SCE_NET_ERROR_EDQUOT: SceNetErrorCode = 2151743813;
pub const SCE_NET_ERROR_EEXIST: SceNetErrorCode = 2151743761;
pub const SCE_NET_ERROR_EFAULT: SceNetErrorCode = 2151743758;
pub const SCE_NET_ERROR_EFBIG: SceNetErrorCode = 2151743771;
pub const SCE_NET_ERROR_EFTYPE: SceNetErrorCode = 2151743823;
pub const SCE_NET_ERROR_EHOSTDOWN: SceNetErrorCode = 2151743808;
pub const SCE_NET_ERROR_EHOSTUNREACH: SceNetErrorCode = 2151743809;
pub const SCE_NET_ERROR_EIDRM: SceNetErrorCode = 2151743826;
pub const SCE_NET_ERROR_EILSEQ: SceNetErrorCode = 2151743829;
pub const SCE_NET_ERROR_EINPROGRESS: SceNetErrorCode = 2151743780;
pub const SCE_NET_ERROR_EINTERNAL: SceNetErrorCode = 2151743948;
pub const SCE_NET_ERROR_EINTR: SceNetErrorCode = 2151743748;
pub const SCE_NET_ERROR_EINVAL: SceNetErrorCode = 2151743766;
pub const SCE_NET_ERROR_EIO: SceNetErrorCode = 2151743749;
pub const SCE_NET_ERROR_EISCONN: SceNetErrorCode = 2151743800;
pub const SCE_NET_ERROR_EISDIR: SceNetErrorCode = 2151743765;
pub const SCE_NET_ERROR_ELOOP: SceNetErrorCode = 2151743806;
pub const SCE_NET_ERROR_EMFILE: SceNetErrorCode = 2151743768;
pub const SCE_NET_ERROR_EMLINK: SceNetErrorCode = 2151743775;
pub const SCE_NET_ERROR_EMSGSIZE: SceNetErrorCode = 2151743784;
pub const SCE_NET_ERROR_ENAMETOOLONG: SceNetErrorCode = 2151743807;
pub const SCE_NET_ERROR_ENEEDAUTH: SceNetErrorCode = 2151743825;
pub const SCE_NET_ERROR_ENETDOWN: SceNetErrorCode = 2151743794;
pub const SCE_NET_ERROR_ENETRESET: SceNetErrorCode = 2151743796;
pub const SCE_NET_ERROR_ENETUNREACH: SceNetErrorCode = 2151743795;
pub const SCE_NET_ERROR_ENFILE: SceNetErrorCode = 2151743767;
pub const SCE_NET_ERROR_ENOBUFS: SceNetErrorCode = 2151743799;
pub const SCE_NET_ERROR_ENODATA: SceNetErrorCode = 2151743833;
pub const SCE_NET_ERROR_ENODEV: SceNetErrorCode = 2151743763;
pub const SCE_NET_ERROR_ENOENT: SceNetErrorCode = 2151743746;
pub const SCE_NET_ERROR_ENOEXEC: SceNetErrorCode = 2151743752;
pub const SCE_NET_ERROR_ENOLCK: SceNetErrorCode = 2151743821;
pub const SCE_NET_ERROR_ENOLIBMEM: SceNetErrorCode = 2151743945;
pub const SCE_NET_ERROR_ENOMEM: SceNetErrorCode = 2151743756;
pub const SCE_NET_ERROR_ENOMS: SceNetErrorCode = 2151743827;
pub const SCE_NET_ERROR_ENOPROTOOPT: SceNetErrorCode = 2151743786;
pub const SCE_NET_ERROR_ENOSPC: SceNetErrorCode = 2151743772;
pub const SCE_NET_ERROR_ENOSR: SceNetErrorCode = 2151743834;
pub const SCE_NET_ERROR_ENOSTR: SceNetErrorCode = 2151743835;
pub const SCE_NET_ERROR_ENOSYS: SceNetErrorCode = 2151743822;
pub const SCE_NET_ERROR_ENOTBLK: SceNetErrorCode = 2151743759;
pub const SCE_NET_ERROR_ENOTCONN: SceNetErrorCode = 2151743801;
pub const SCE_NET_ERROR_ENOTDIR: SceNetErrorCode = 2151743764;
pub const SCE_NET_ERROR_ENOTEMPTY: SceNetErrorCode = 2151743810;
pub const SCE_NET_ERROR_ENOTINIT: SceNetErrorCode = 2151743944;
pub const SCE_NET_ERROR_ENOTSOCK: SceNetErrorCode = 2151743782;
pub const SCE_NET_ERROR_ENOTSUP: SceNetErrorCode = 2151743830;
pub const SCE_NET_ERROR_ENOTTY: SceNetErrorCode = 2151743769;
pub const SCE_NET_ERROR_ENXIO: SceNetErrorCode = 2151743750;
pub const SCE_NET_ERROR_EOPNOTSUPP: SceNetErrorCode = 2151743789;
pub const SCE_NET_ERROR_EOVERFLOW: SceNetErrorCode = 2151743828;
pub const SCE_NET_ERROR_EPERM: SceNetErrorCode = 2151743745;
pub const SCE_NET_ERROR_EPFNOSUPPORT: SceNetErrorCode = 2151743790;
pub const SCE_NET_ERROR_EPIPE: SceNetErrorCode = 2151743776;
pub const SCE_NET_ERROR_EPROCLIM: SceNetErrorCode = 2151743811;
pub const SCE_NET_ERROR_EPROCUNAVAIL: SceNetErrorCode = 2151743820;
pub const SCE_NET_ERROR_EPROGMISMATCH: SceNetErrorCode = 2151743819;
pub const SCE_NET_ERROR_EPROGUNAVAIL: SceNetErrorCode = 2151743818;
pub const SCE_NET_ERROR_EPROTONOSUPPORT: SceNetErrorCode = 2151743787;
pub const SCE_NET_ERROR_EPROTOTYPE: SceNetErrorCode = 2151743785;
pub const SCE_NET_ERROR_ERANGE: SceNetErrorCode = 2151743778;
pub const SCE_NET_ERROR_EREMOTE: SceNetErrorCode = 2151743815;
pub const SCE_NET_ERROR_ERESERVED202: SceNetErrorCode = 2151743946;
pub const SCE_NET_ERROR_ERESUME: SceNetErrorCode = 2151743906;
pub const SCE_NET_ERROR_ERETURN: SceNetErrorCode = 2151743949;
pub const SCE_NET_ERROR_EROFS: SceNetErrorCode = 2151743774;
pub const SCE_NET_ERROR_ERPCMISMATCH: SceNetErrorCode = 2151743817;
pub const SCE_NET_ERROR_ESHUTDOWN: SceNetErrorCode = 2151743802;
pub const SCE_NET_ERROR_ESOCKTNOSUPPORT: SceNetErrorCode = 2151743788;
pub const SCE_NET_ERROR_ESPIPE: SceNetErrorCode = 2151743773;
pub const SCE_NET_ERROR_ESRCH: SceNetErrorCode = 2151743747;
pub const SCE_NET_ERROR_ESTALE: SceNetErrorCode = 2151743814;
pub const SCE_NET_ERROR_ETIME: SceNetErrorCode = 2151743836;
pub const SCE_NET_ERROR_ETIMEDOUT: SceNetErrorCode = 2151743804;
pub const SCE_NET_ERROR_ETOOMANYREFS: SceNetErrorCode = 2151743803;
pub const SCE_NET_ERROR_ETXTBSY: SceNetErrorCode = 2151743770;
pub const SCE_NET_ERROR_EUSERS: SceNetErrorCode = 2151743812;
pub const SCE_NET_ERROR_EWOULDBLOCK: SceNetErrorCode = 2151743779;
pub const SCE_NET_ERROR_EXDEV: SceNetErrorCode = 2151743762;
pub const SCE_NET_ERROR_RESOLVER_EALIGNMENT: SceNetErrorCode = 2151743978;
pub const SCE_NET_ERROR_RESOLVER_EBUSY: SceNetErrorCode = 2151743965;
pub const SCE_NET_ERROR_RESOLVER_EFORMAT: SceNetErrorCode = 2151743972;
pub const SCE_NET_ERROR_RESOLVER_EINTERNAL: SceNetErrorCode = 2151743964;
pub const SCE_NET_ERROR_RESOLVER_ENODNS: SceNetErrorCode = 2151743969;
pub const SCE_NET_ERROR_RESOLVER_ENOHOST: SceNetErrorCode = 2151743974;
pub const SCE_NET_ERROR_RESOLVER_ENORECORD: SceNetErrorCode = 2151743977;
pub const SCE_NET_ERROR_RESOLVER_ENOSPACE: SceNetErrorCode = 2151743966;
pub const SCE_NET_ERROR_RESOLVER_ENOSUPPORT: SceNetErrorCode = 2151743971;
pub const SCE_NET_ERROR_RESOLVER_ENOTIMPLEMENTED: SceNetErrorCode = 2151743975;
pub const SCE_NET_ERROR_RESOLVER_EPACKET: SceNetErrorCode = 2151743967;
pub const SCE_NET_ERROR_RESOLVER_ERESERVED22: SceNetErrorCode = 2151743968;
pub const SCE_NET_ERROR_RESOLVER_ESERVERFAILURE: SceNetErrorCode = 2151743973;
pub const SCE_NET_ERROR_RESOLVER_ESERVERREFUSED: SceNetErrorCode = 2151743976;
pub const SCE_NET_ERROR_RESOLVER_ETIMEDOUT: SceNetErrorCode = 2151743970;
pub const SCE_NET_ESHUTDOWN: SceNetKernelErrorCode = 58;
pub const SCE_NET_ESOCKTNOSUPPORT: SceNetKernelErrorCode = 44;
pub const SCE_NET_ESPIPE: SceNetKernelErrorCode = 29;
pub const SCE_NET_ESRCH: SceNetKernelErrorCode = 3;
pub const SCE_NET_ESTALE: SceNetKernelErrorCode = 70;
pub const SCE_NET_ETIME: SceNetKernelErrorCode = 92;
pub const SCE_NET_ETIMEDOUT: SceNetKernelErrorCode = 60;
pub const SCE_NET_ETLS: SceNetLibnetErrorCode = 202;
pub const SCE_NET_ETOOMANYREFS: SceNetKernelErrorCode = 59;
pub const SCE_NET_ETXTBSY: SceNetKernelErrorCode = 26;
pub const SCE_NET_EUSERS: SceNetKernelErrorCode = 68;
pub const SCE_NET_EWOULDBLOCK: SceNetKernelErrorCode = 35;
pub const SCE_NET_EXDEV: SceNetKernelErrorCode = 18;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_DST_HOST_UNKNOWN: SceNetIcmpCode = 7;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_DST_NET_UNKNOWN: SceNetIcmpCode = 6;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_FRAG_AND_DF: SceNetIcmpCode = 4;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_HOST_TOS: SceNetIcmpCode = 12;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_HOST_UNREACH: SceNetIcmpCode = 1;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_NET_ADMIN_PROHIBITED: SceNetIcmpCode = 9;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_NET_HOST_PROHIBITED: SceNetIcmpCode = 10;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_NET_TOS: SceNetIcmpCode = 11;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_NET_UNREACH: SceNetIcmpCode = 0;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_PORT_UNREACH: SceNetIcmpCode = 3;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_PROTO_UNREACH: SceNetIcmpCode = 2;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_SRC_HOST_FAILED: SceNetIcmpCode = 5;
pub const SCE_NET_ICMP_CODE_DEST_UNREACH_SRC_HOST_ISOLATED: SceNetIcmpCode = 8;
pub const SCE_NET_ICMP_CODE_TIME_EXCEEDED_FRT_EXCEEDED: SceNetIcmpCode = 1;
pub const SCE_NET_ICMP_CODE_TIME_EXCEEDED_TTL_EXCEEDED: SceNetIcmpCode = 0;
pub const SCE_NET_ICMP_TYPE_ADDRESS_MASK_REPLY: SceNetIcmpType = 18;
pub const SCE_NET_ICMP_TYPE_ADDRESS_MASK_REQUEST: SceNetIcmpType = 17;
pub const SCE_NET_ICMP_TYPE_DEST_UNREACH: SceNetIcmpType = 3;
pub const SCE_NET_ICMP_TYPE_ECHO_REPLY: SceNetIcmpType = 0;
pub const SCE_NET_ICMP_TYPE_ECHO_REQUEST: SceNetIcmpType = 8;
pub const SCE_NET_ICMP_TYPE_INFORMATION_REPLY: SceNetIcmpType = 16;
pub const SCE_NET_ICMP_TYPE_INFORMATION_REQUEST: SceNetIcmpType = 15;
pub const SCE_NET_ICMP_TYPE_PARAMETER_PROBLEM: SceNetIcmpType = 12;
pub const SCE_NET_ICMP_TYPE_REDIRECT: SceNetIcmpType = 5;
pub const SCE_NET_ICMP_TYPE_SOURCE_QUENCH: SceNetIcmpType = 4;
pub const SCE_NET_ICMP_TYPE_TIME_EXCEEDED: SceNetIcmpType = 11;
pub const SCE_NET_ICMP_TYPE_TIMESTAMP_REPLY: SceNetIcmpType = 14;
pub const SCE_NET_ICMP_TYPE_TIMESTAMP_REQUEST: SceNetIcmpType = 13;
pub const SCE_NET_ID_SOCKET_MAX: u32 = 1023;
pub const SCE_NET_ID_SOCKET_MIN: u32 = 0;
pub const SCE_NET_INADDR_ANY: u32 = 0;
pub const SCE_NET_INADDR_AUTOIP: u32 = 2851995648;
pub const SCE_NET_INADDR_BROADCAST: u32 = 4294967295;
pub const SCE_NET_INADDR_LOOPBACK: u32 = 2130706433;
pub const SCE_NET_INADDR_UNSPEC_GROUP: u32 = 3758096384;
pub const SCE_NET_IN_AUTOIP_NET: u32 = 4294901760;
pub const SCE_NET_IN_CLASSD_NET: u32 = 4026531840;
pub const SCE_NET_IP_ADD_MEMBERSHIP: SceNetSocketOption = 12;
pub const SCE_NET_IP_DEFAULT_MULTICAST_LOOP: u32 = 1;
pub const SCE_NET_IP_DEFAULT_MULTICAST_TTL: u32 = 1;
pub const SCE_NET_IP_DF: u32 = 16384;
pub const SCE_NET_IP_DROP_MEMBERSHIP: SceNetSocketOption = 13;
pub const SCE_NET_IP_HDRINCL: SceNetSocketOption = 2;
pub const SCE_NET_IP_MAXTTL: SceNetSocketOption = 24;
pub const SCE_NET_IP_MF: u32 = 8192;
pub const SCE_NET_IP_MULTICAST_IF: SceNetSocketOption = 9;
pub const SCE_NET_IP_MULTICAST_LOOP: SceNetSocketOption = 11;
pub const SCE_NET_IP_MULTICAST_TTL: SceNetSocketOption = 10;
pub const SCE_NET_IP_OFFMASK: u32 = 8191;
pub const SCE_NET_IPPROTO_ICMP: SceNetProtocol = 1;
pub const SCE_NET_IPPROTO_IGMP: SceNetProtocol = 2;
pub const SCE_NET_IPPROTO_IP: SceNetProtocol = 0;
pub const SCE_NET_IPPROTO_TCP: SceNetProtocol = 6;
pub const SCE_NET_IPPROTO_UDP: SceNetProtocol = 17;
pub const SCE_NET_IP_RF: u32 = 32768;
pub const SCE_NET_IP_TOS: SceNetSocketOption = 3;
pub const SCE_NET_IP_TTL: SceNetSocketOption = 4;
pub const SCE_NET_IP_TTLCHK: SceNetSocketOption = 23;
pub const SCE_NET_IPVERSION: u32 = 4;
pub const SCE_NET_MSG_DONTWAIT: SceNetMsgFlag = 128;
pub const SCE_NET_MSG_PEEK: SceNetMsgFlag = 2;
pub const SCE_NET_MSG_USECRYPTO: SceNetMsgFlag = 1024;
pub const SCE_NET_MSG_USESIGNATURE: SceNetMsgFlag = 2048;
pub const SCE_NET_MSG_WAITALL: SceNetMsgFlag = 64;
pub const SCE_NET_RESOLVER_ABORT_FLAG_ATON_PRESERVATION: SceNetResolverAbortFlag = 2;
pub const SCE_NET_RESOLVER_ABORT_FLAG_NTOA_PRESERVATION: SceNetResolverAbortFlag = 1;
pub const SCE_NET_RESOLVER_ASYNC: SceNetResolverFlag = 1;
pub const SCE_NET_RESOLVER_EALIGNMENT: SceNetResolverErrorCode = 234;
pub const SCE_NET_RESOLVER_EBUSY: SceNetResolverErrorCode = 221;
pub const SCE_NET_RESOLVER_EFORMAT: SceNetResolverErrorCode = 228;
pub const SCE_NET_RESOLVER_EINTERNAL: SceNetResolverErrorCode = 220;
pub const SCE_NET_RESOLVER_ENODNS: SceNetResolverErrorCode = 225;
pub const SCE_NET_RESOLVER_ENOHOST: SceNetResolverErrorCode = 230;
pub const SCE_NET_RESOLVER_ENORECORD: SceNetResolverErrorCode = 233;
pub const SCE_NET_RESOLVER_ENOSPACE: SceNetResolverErrorCode = 222;
pub const SCE_NET_RESOLVER_ENOSUPPORT: SceNetResolverErrorCode = 227;
pub const SCE_NET_RESOLVER_ENOTIMPLEMENTED: SceNetResolverErrorCode = 231;
pub const SCE_NET_RESOLVER_EPACKET: SceNetResolverErrorCode = 223;
pub const SCE_NET_RESOLVER_ERESERVED224: SceNetResolverErrorCode = 224;
pub const SCE_NET_RESOLVER_ESERVERFAILURE: SceNetResolverErrorCode = 229;
pub const SCE_NET_RESOLVER_ESERVERREFUSED: SceNetResolverErrorCode = 232;
pub const SCE_NET_RESOLVER_ETIMEDOUT: SceNetResolverErrorCode = 226;
pub const SCE_NET_RESOLVER_HOSTNAME_LEN_MAX: u32 = 255;
pub const SCE_NET_RESOLVER_PORT: u32 = 53;
pub const SCE_NET_RESOLVER_START_NTOA_DISABLE_IPADDRESS: SceNetResolverFlag = 65536;
pub const SCE_NET_SHUT_RD: SceNetShutdownFlag = 0;
pub const SCE_NET_SHUT_RDWR: SceNetShutdownFlag = 2;
pub const SCE_NET_SHUT_WR: SceNetShutdownFlag = 1;
pub const SCE_NET_SO_BROADCAST: SceNetSocketOption = 32;
pub const SCE_NET_SOCK_DGRAM: SceNetSocketType = 2;
pub const SCE_NET_SOCK_DGRAM_P2P: SceNetSocketType = 6;
pub const SCE_NET_SOCKET_ABORT_FLAG_RCV_PRESERVATION: SceNetSocketAbortFlag = 1;
pub const SCE_NET_SOCKET_ABORT_FLAG_SND_PRESERVATION: SceNetSocketAbortFlag = 2;
pub const SCE_NET_SOCKINFO_F_ALL: SceNetSockInfoFlag = 2031623;
pub const SCE_NET_SOCKINFO_F_KERNEL: SceNetSockInfoFlag = 2;
pub const SCE_NET_SOCKINFO_F_OTHERS: SceNetSockInfoFlag = 4;
pub const SCE_NET_SOCKINFO_F_RECV_EWAIT: SceNetSockInfoFlag = 262144;
pub const SCE_NET_SOCKINFO_F_RECV_WAIT: SceNetSockInfoFlag = 65536;
pub const SCE_NET_SOCKINFO_F_SELF: SceNetSockInfoFlag = 1;
pub const SCE_NET_SOCKINFO_F_SEND_EWAIT: SceNetSockInfoFlag = 524288;
pub const SCE_NET_SOCKINFO_F_SEND_WAIT: SceNetSockInfoFlag = 131072;
pub const SCE_NET_SOCKINFO_F_WAKEUP_SIGNAL: SceNetSockInfoFlag = 1048576;
pub const SCE_NET_SOCKINFO_STATE_CLOSED: SceNetSockInfoState = 1;
pub const SCE_NET_SOCKINFO_STATE_CLOSE_WAIT: SceNetSockInfoState = 9;
pub const SCE_NET_SOCKINFO_STATE_CLOSING: SceNetSockInfoState = 10;
pub const SCE_NET_SOCKINFO_STATE_ESTABLISHED: SceNetSockInfoState = 6;
pub const SCE_NET_SOCKINFO_STATE_FIN_WAIT_1: SceNetSockInfoState = 7;
pub const SCE_NET_SOCKINFO_STATE_FIN_WAIT_2: SceNetSockInfoState = 8;
pub const SCE_NET_SOCKINFO_STATE_LAST_ACK: SceNetSockInfoState = 11;
pub const SCE_NET_SOCKINFO_STATE_LISTEN: SceNetSockInfoState = 3;
pub const SCE_NET_SOCKINFO_STATE_OPENED: SceNetSockInfoState = 2;
pub const SCE_NET_SOCKINFO_STATE_SYN_RECEIVED: SceNetSockInfoState = 5;
pub const SCE_NET_SOCKINFO_STATE_SYN_SENT: SceNetSockInfoState = 4;
pub const SCE_NET_SOCKINFO_STATE_TIME_WAIT: SceNetSockInfoState = 12;
pub const SCE_NET_SOCKINFO_STATE_UNKNOWN: SceNetSockInfoState = 0;
pub const SCE_NET_SOCK_RAW: SceNetSocketType = 3;
pub const SCE_NET_SOCK_STREAM: SceNetSocketType = 1;
pub const SCE_NET_SOCK_STREAM_P2P: SceNetSocketType = 10;
pub const SCE_NET_SO_ERROR: SceNetSocketOption = 4103;
pub const SCE_NET_SO_KEEPALIVE: SceNetSocketOption = 8;
pub const SCE_NET_SO_LINGER: SceNetSocketOption = 128;
pub const SCE_NET_SOL_SOCKET: SceNetProtocol = 65535;
pub const SCE_NET_SO_NAME: SceNetSocketOption = 4354;
pub const SCE_NET_SO_NBIO: SceNetSocketOption = 4352;
pub const SCE_NET_SO_ONESBCAST: SceNetSocketOption = 2048;
pub const SCE_NET_SO_OOBINLINE: SceNetSocketOption = 256;
pub const SCE_NET_SO_RCVBUF: SceNetSocketOption = 4098;
pub const SCE_NET_SO_RCVLOWAT: SceNetSocketOption = 4100;
pub const SCE_NET_SO_RCVTIMEO: SceNetSocketOption = 4102;
pub const SCE_NET_SO_REUSEADDR: SceNetSocketOption = 4;
pub const SCE_NET_SO_REUSEPORT: SceNetSocketOption = 512;
pub const SCE_NET_SO_SNDBUF: SceNetSocketOption = 4097;
pub const SCE_NET_SO_SNDLOWAT: SceNetSocketOption = 4099;
pub const SCE_NET_SO_SNDTIMEO: SceNetSocketOption = 4101;
pub const SCE_NET_SO_TPPOLICY: SceNetSocketOption = 4353;
pub const SCE_NET_SO_TYPE: SceNetSocketOption = 4104;
pub const SCE_NET_SO_USECRYPTO: SceNetSocketOption = 4096;
pub const SCE_NET_SO_USESIGNATURE: SceNetSocketOption = 8192;
pub const SCE_NET_TCP_MAXSEG: SceNetSocketOption = 2;
pub const SCE_NET_TCP_MSS_TO_ADVERTISE: SceNetSocketOption = 3;
pub const SCE_NET_TCP_NODELAY: SceNetSocketOption = 1;
pub const SCE_NOTIFICATIONUTIL_ERROR_INTERNAL: SceNotificationUitlErrorCode = 2148557568;
pub const SCE_NOTIFICATIONUTIL_TEXT_MAX: u32 = 63;
pub const SCE_O_APPEND: SceIoMode = 256;
pub const SCE_O_CREAT: SceIoMode = 512;
pub const SCE_O_DIROPEN: SceIoMode = 8;
pub const SCE_O_EXCL: SceIoMode = 2048;
pub const SCE_O_FDEXCL: SceIoMode = 16777216;
pub const SCE_O_FGAMEDATA: SceIoMode = 1073741824;
pub const SCE_OK: u32 = 0;
pub const SCE_O_NBLOCK: SceIoMode = 4;
pub const SCE_O_NOBUF: SceIoMode = 16384;
pub const SCE_O_NOWAIT: SceIoMode = 32768;
pub const SCE_O_PWLOCK: SceIoMode = 33554432;
pub const SCE_O_RCOM: SceIoMode = 8192;
pub const SCE_O_RDLOCK: SceIoMode = 16;
pub const SCE_O_RDONLY: SceIoMode = 1;
pub const SCE_O_RDWR: SceIoMode = 3;
pub const SCE_O_SCAN: SceIoMode = 4096;
pub const SCE_O_TRUNC: SceIoMode = 1024;
pub const SCE_O_WRLOCK: SceIoMode = 32;
pub const SCE_O_WRONLY: SceIoMode = 2;
pub const SCE_PERF_ARM_PMON_BRANCH_MISPREDICT: _ScePerfArmPmonEventCode = 16;
pub const SCE_PERF_ARM_PMON_COHERENT_LF_HIT: _ScePerfArmPmonEventCode = 81;
pub const SCE_PERF_ARM_PMON_COHERENT_LF_MISS: _ScePerfArmPmonEventCode = 80;
pub const SCE_PERF_ARM_PMON_CYCLE_COUNT: _ScePerfArmPmonEventCode = 17;
pub const SCE_PERF_ARM_PMON_DATAENGINE_CLOCK: _ScePerfArmPmonEventCode = 139;
pub const SCE_PERF_ARM_PMON_DATA_EVICTION: _ScePerfArmPmonEventCode = 101;
pub const SCE_PERF_ARM_PMON_DATA_MAINTLB_STALL: _ScePerfArmPmonEventCode = 131;
pub const SCE_PERF_ARM_PMON_DATA_READ: _ScePerfArmPmonEventCode = 6;
pub const SCE_PERF_ARM_PMON_DATA_UTLB_STALL: _ScePerfArmPmonEventCode = 133;
pub const SCE_PERF_ARM_PMON_DATA_WRITE: _ScePerfArmPmonEventCode = 7;
pub const SCE_PERF_ARM_PMON_DCACHE_ACCESS: _ScePerfArmPmonEventCode = 4;
pub const SCE_PERF_ARM_PMON_DCACHE_MISS: _ScePerfArmPmonEventCode = 3;
pub const SCE_PERF_ARM_PMON_DCACHE_STALL: _ScePerfArmPmonEventCode = 97;
pub const SCE_PERF_ARM_PMON_DMB: _ScePerfArmPmonEventCode = 146;
pub const SCE_PERF_ARM_PMON_DMB_STALL: _ScePerfArmPmonEventCode = 134;
pub const SCE_PERF_ARM_PMON_DSB: _ScePerfArmPmonEventCode = 145;
pub const SCE_PERF_ARM_PMON_DTLB_MISS: _ScePerfArmPmonEventCode = 5;
pub const SCE_PERF_ARM_PMON_EXCEPTION_RETURN: _ScePerfArmPmonEventCode = 10;
pub const SCE_PERF_ARM_PMON_EXCEPTION_TAKEN: _ScePerfArmPmonEventCode = 9;
pub const SCE_PERF_ARM_PMON_EXT_INTERRUPT: _ScePerfArmPmonEventCode = 147;
pub const SCE_PERF_ARM_PMON_FPU_RENAME: _ScePerfArmPmonEventCode = 115;
pub const SCE_PERF_ARM_PMON_ICACHE_MISS: _ScePerfArmPmonEventCode = 1;
pub const SCE_PERF_ARM_PMON_ICACHE_STALL: _ScePerfArmPmonEventCode = 96;
pub const SCE_PERF_ARM_PMON_IMMEDIATE_BRANCH: _ScePerfArmPmonEventCode = 13;
pub const SCE_PERF_ARM_PMON_INST_MAINTLB_STALL: _ScePerfArmPmonEventCode = 130;
pub const SCE_PERF_ARM_PMON_INST_RENAME: _ScePerfArmPmonEventCode = 104;
pub const SCE_PERF_ARM_PMON_INST_UTLB_STALL: _ScePerfArmPmonEventCode = 132;
pub const SCE_PERF_ARM_PMON_INTEGER_CLOCK: _ScePerfArmPmonEventCode = 138;
pub const SCE_PERF_ARM_PMON_ISB: _ScePerfArmPmonEventCode = 144;
pub const SCE_PERF_ARM_PMON_ISSUE_EMPTY: _ScePerfArmPmonEventCode = 103;
pub const SCE_PERF_ARM_PMON_ISSUE_NO_DISPATCH: _ScePerfArmPmonEventCode = 102;
pub const SCE_PERF_ARM_PMON_ITLB_MISS: _ScePerfArmPmonEventCode = 2;
pub const SCE_PERF_ARM_PMON_LS_PIPE: _ScePerfArmPmonEventCode = 114;
pub const SCE_PERF_ARM_PMON_MAIN_PIPE: _ScePerfArmPmonEventCode = 112;
pub const SCE_PERF_ARM_PMON_MAINTLB_STALL: _ScePerfArmPmonEventCode = 98;
pub const SCE_PERF_ARM_PMON_NEON_RENAME: _ScePerfArmPmonEventCode = 116;
pub const SCE_PERF_ARM_PMON_PLD_STALL: _ScePerfArmPmonEventCode = 128;
pub const SCE_PERF_ARM_PMON_PLE_CHANNEL_SKIPPED: _ScePerfArmPmonEventCode = 161;
pub const SCE_PERF_ARM_PMON_PLE_FIFO_FLUSH: _ScePerfArmPmonEventCode = 162;
pub const SCE_PERF_ARM_PMON_PLE_FIFO_OVERFLOW: _ScePerfArmPmonEventCode = 164;
pub const SCE_PERF_ARM_PMON_PLE_LINE_REQ_COMPLETED: _ScePerfArmPmonEventCode = 160;
pub const SCE_PERF_ARM_PMON_PLE_REQ_COMPLETED: _ScePerfArmPmonEventCode = 163;
pub const SCE_PERF_ARM_PMON_PLE_REQ_PROGRAMMED: _ScePerfArmPmonEventCode = 165;
pub const SCE_PERF_ARM_PMON_PREDICT_BRANCH: _ScePerfArmPmonEventCode = 18;
pub const SCE_PERF_ARM_PMON_PREDICT_FUNC_RET: _ScePerfArmPmonEventCode = 110;
pub const SCE_PERF_ARM_PMON_SECOND_PIPE: _ScePerfArmPmonEventCode = 113;
pub const SCE_PERF_ARM_PMON_SOFT_CHANGEPC: _ScePerfArmPmonEventCode = 12;
pub const SCE_PERF_ARM_PMON_SOFT_INCREMENT: _ScePerfArmPmonEventCode = 0;
pub const SCE_PERF_ARM_PMON_STREX_FAILED: _ScePerfArmPmonEventCode = 100;
pub const SCE_PERF_ARM_PMON_STREX_PASSED: _ScePerfArmPmonEventCode = 99;
pub const SCE_PERF_ARM_PMON_THREAD_ID_SELF: u32 = 0;
pub const SCE_PERF_ARM_PMON_UNALIGNED: _ScePerfArmPmonEventCode = 15;
pub const SCE_PERF_ARM_PMON_WRITE_CONTEXTID: _ScePerfArmPmonEventCode = 11;
pub const SCE_PERF_ARM_PMON_WRITE_STALL: _ScePerfArmPmonEventCode = 129;
pub const SCE_PKG_TYPE_PSM: ScePromoterUtilityPackageType = 3;
pub const SCE_PKG_TYPE_PSP: ScePromoterUtilityPackageType = 1;
pub const SCE_PKG_TYPE_VITA: ScePromoterUtilityPackageType = 1;
pub const SCE_POWER_CB_AFTER_SYSTEM_RESUME: ScePowerCallbackType = 128;
pub const SCE_POWER_CB_APP_RESUME: ScePowerCallbackType = 2097152;
pub const SCE_POWER_CB_APP_RESUMING: ScePowerCallbackType = 8388608;
pub const SCE_POWER_CB_APP_SUSPEND: ScePowerCallbackType = 4194304;
pub const SCE_POWER_CB_BATTERY_ONLINE: ScePowerCallbackType = 256;
pub const SCE_POWER_CB_BUTTON_POWER_HOLD: ScePowerCallbackType = 1073741824;
pub const SCE_POWER_CB_BUTTON_POWER_PRESS: ScePowerCallbackType = 2147483648;
pub const SCE_POWER_CB_BUTTON_PS_HOLD: ScePowerCallbackType = 268435456;
pub const SCE_POWER_CB_BUTTON_PS_POWER_PRESS: ScePowerCallbackType = 134217728;
pub const SCE_POWER_CB_BUTTON_PS_PRESS: ScePowerCallbackType = 536870912;
pub const SCE_POWER_CB_BUTTON_PS_START_PRESS: ScePowerCallbackType = 67108864;
pub const SCE_POWER_CB_LOW_BATTERY: ScePowerCallbackType = 2048;
pub const SCE_POWER_CB_LOW_BATTERY_SUSPEND: ScePowerCallbackType = 1024;
pub const SCE_POWER_CB_POWER_ONLINE: ScePowerCallbackType = 4096;
pub const SCE_POWER_CB_SYSTEM_RESUME: ScePowerCallbackType = 262144;
pub const SCE_POWER_CB_SYSTEM_RESUMING: ScePowerCallbackType = 131072;
pub const SCE_POWER_CB_SYSTEM_SUSPEND: ScePowerCallbackType = 65536;
pub const SCE_POWER_CB_THERMAL_SUSPEND: ScePowerCallbackType = 512;
pub const SCE_POWER_CB_UNK_0x100000: ScePowerCallbackType = 1048576;
pub const SCE_POWER_CB_VALID_MASK_KERNEL: ScePowerCallbackType = 4244053888;
pub const SCE_POWER_CB_VALID_MASK_NON_SYSTEM: ScePowerCallbackType = 3543424;
pub const SCE_POWER_CB_VALID_MASK_SYSTEM: ScePowerCallbackType = 4244053888;
pub const __SCE_POWER_CONFIGURATION_MODE: ScePowerConfigurationMode = 4294967295;
pub const SCE_POWER_CONFIGURATION_MODE_A: ScePowerConfigurationMode = 128;
pub const SCE_POWER_CONFIGURATION_MODE_B: ScePowerConfigurationMode = 2048;
pub const SCE_POWER_CONFIGURATION_MODE_C: ScePowerConfigurationMode = 67712;
pub const SCE_POWER_ERROR_ALREADY_REGISTERED: ScePowerErrorCode = 2150301697;
pub const SCE_POWER_ERROR_CALLBACK_NOT_REGISTERED: ScePowerErrorCode = 2150301698;
pub const SCE_POWER_ERROR_CANT_SUSPEND: ScePowerErrorCode = 2150301699;
pub const SCE_POWER_ERROR_DETECTING: ScePowerErrorCode = 2150301953;
pub const SCE_POWER_ERROR_INVALID_VALUE: ScePowerErrorCode = 2150301696;
pub const SCE_POWER_ERROR_NO_BATTERY: ScePowerErrorCode = 2150301952;
pub const SCE_PRODUCT_CODE_CEX_AU3: SceProductCode = 265;
pub const SCE_PRODUCT_CODE_CEX_CEK: SceProductCode = 263;
pub const SCE_PRODUCT_CODE_CEX_CEL: SceProductCode = 261;
pub const SCE_PRODUCT_CODE_CEX_CN9: SceProductCode = 269;
pub const SCE_PRODUCT_CODE_CEX_E12: SceProductCode = 266;
pub const SCE_PRODUCT_CODE_CEX_HK5: SceProductCode = 270;
pub const SCE_PRODUCT_CODE_CEX_J1: SceProductCode = 259;
pub const SCE_PRODUCT_CODE_CEX_KR2: SceProductCode = 262;
pub const SCE_PRODUCT_CODE_CEX_MX2: SceProductCode = 264;
pub const SCE_PRODUCT_CODE_CEX_RSV1: SceProductCode = 271;
pub const SCE_PRODUCT_CODE_CEX_RSV2: SceProductCode = 272;
pub const SCE_PRODUCT_CODE_CEX_RSV3: SceProductCode = 273;
pub const SCE_PRODUCT_CODE_CEX_RU3: SceProductCode = 268;
pub const SCE_PRODUCT_CODE_CEX_TW1: SceProductCode = 267;
pub const SCE_PRODUCT_CODE_CEX_UC2: SceProductCode = 260;
pub const SCE_PRODUCT_CODE_DEX: SceProductCode = 258;
pub const SCE_PRODUCT_CODE_NONE: SceProductCode = 0;
pub const SCE_PRODUCT_CODE_TEST: SceProductCode = 256;
pub const SCE_PRODUCT_CODE_TOOL: SceProductCode = 257;
pub const SCE_PVF_COUNTRY_JAPAN: ScePvfFontVendorCountryCode = 1;
pub const SCE_PVF_COUNTRY_KOREA: ScePvfFontVendorCountryCode = 3;
pub const SCE_PVF_COUNTRY_USA: ScePvfFontVendorCountryCode = 2;
pub const SCE_PVF_DEFAULT_FAMILY_CODE: ScePvfFamilyCode = 0;
pub const SCE_PVF_DEFAULT_LANGUAGE_CODE: ScePvfLanguageCode = 0;
pub const SCE_PVF_DEFAULT_STYLE_CODE: ScePvfStyleCode = 0;
pub const SCE_PVF_ERROR_ARG: ScePvfErrorCode = 2152071171;
pub const SCE_PVF_ERROR_DATAINCONSISTENT: ScePvfErrorCode = 2152071179;
pub const SCE_PVF_ERROR_EXPIRED: ScePvfErrorCode = 2152071180;
pub const SCE_PVF_ERROR_FILECLOSE: ScePvfErrorCode = 2152071174;
pub const SCE_PVF_ERROR_FILEOPEN: ScePvfErrorCode = 2152071173;
pub const SCE_PVF_ERROR_FILEREAD: ScePvfErrorCode = 2152071175;
pub const SCE_PVF_ERROR_FILESEEK: ScePvfErrorCode = 2152071176;
pub const SCE_PVF_ERROR_ILLEGALVERSION: ScePvfErrorCode = 2152071178;
pub const SCE_PVF_ERROR_LIBID: ScePvfErrorCode = 2152071170;
pub const SCE_PVF_ERROR_NOFILE: ScePvfErrorCode = 2152071172;
pub const SCE_PVF_ERROR_NOGLYPH: ScePvfErrorCode = 2152071183;
pub const SCE_PVF_ERROR_NOMEMORY: ScePvfErrorCode = 2152071169;
pub const SCE_PVF_ERROR_NOSUPPORT: ScePvfErrorCode = 2152071182;
pub const SCE_PVF_ERROR_TOOMANYOPENED: ScePvfErrorCode = 2152071177;
pub const SCE_PVF_ERROR_UNKNOWN: ScePvfErrorCode = 2152136703;
pub const SCE_PVF_FALSE: ScePvfBoolValue = 0;
pub const SCE_PVF_FAMILY_ROUNDED: ScePvfFamilyCode = 3;
pub const SCE_PVF_FAMILY_SANSERIF: ScePvfFamilyCode = 1;
pub const SCE_PVF_FAMILY_SERIF: ScePvfFamilyCode = 2;
pub const SCE_PVF_FILEBASEDSTREAM: ScePvfDataAccessMode = 0;
pub const SCE_PVF_FONTFILENAME_LENGTH: u32 = 64;
pub const SCE_PVF_FONTNAME_LENGTH: u32 = 64;
pub const SCE_PVF_GENERIC_COUNTRY_CODE: ScePvfFontVendorCountryCode = 0;
pub const SCE_PVF_GENERIC_REGION_CODE: ScePvfRegionCode = 0;
pub const SCE_PVF_LANGUAGE_C: ScePvfLanguageCode = 4;
pub const SCE_PVF_LANGUAGE_CJK: ScePvfLanguageCode = 5;
pub const SCE_PVF_LANGUAGE_J: ScePvfLanguageCode = 1;
pub const SCE_PVF_LANGUAGE_K: ScePvfLanguageCode = 3;
pub const SCE_PVF_LANGUAGE_LATIN: ScePvfLanguageCode = 2;
pub const SCE_PVF_MAX_EMBOLDEN_RATE: f64 = 40.0;
pub const SCE_PVF_MAX_OPEN: u32 = 18;
pub const SCE_PVF_MAX_SKEW_VALUE: f64 = 30.0;
pub const SCE_PVF_MEMORYBASEDSTREAM: ScePvfDataAccessMode = 1;
pub const SCE_PVF_MIN_EMBOLDEN_RATE: f64 = -20.0;
pub const SCE_PVF_MIN_SKEW_VALUE: f64 = -30.0;
pub const SCE_PVF_REGION_001: ScePvfRegionCode = 1;
pub const SCE_PVF_REGION_002: ScePvfRegionCode = 2;
pub const SCE_PVF_REGION_003: ScePvfRegionCode = 3;
pub const SCE_PVF_REGION_004: ScePvfRegionCode = 4;
pub const SCE_PVF_REGION_005: ScePvfRegionCode = 5;
pub const SCE_PVF_REGION_006: ScePvfRegionCode = 6;
pub const SCE_PVF_REGION_007: ScePvfRegionCode = 7;
pub const SCE_PVF_STYLE_B: ScePvfStyleCode = 104;
pub const SCE_PVF_STYLE_BLACK: ScePvfStyleCode = 7;
pub const SCE_PVF_STYLE_BLACK_OBLIQUE: ScePvfStyleCode = 8;
pub const SCE_PVF_STYLE_BOLD: ScePvfStyleCode = 5;
pub const SCE_PVF_STYLE_BOLD_OBLIQUE: ScePvfStyleCode = 6;
pub const SCE_PVF_STYLE_DB: ScePvfStyleCode = 103;
pub const SCE_PVF_STYLE_EB: ScePvfStyleCode = 105;
pub const SCE_PVF_STYLE_L: ScePvfStyleCode = 101;
pub const SCE_PVF_STYLE_M: ScePvfStyleCode = 102;
pub const SCE_PVF_STYLENAME_LENGTH: u32 = 64;
pub const SCE_PVF_STYLE_NARROW: ScePvfStyleCode = 3;
pub const SCE_PVF_STYLE_NARROW_OBLIQUE: ScePvfStyleCode = 4;
pub const SCE_PVF_STYLE_OBLIQUE: ScePvfStyleCode = 2;
pub const SCE_PVF_STYLE_REGULAR: ScePvfStyleCode = 1;
pub const SCE_PVF_STYLE_UB: ScePvfStyleCode = 106;
pub const SCE_PVF_SUBSTYLE_PSEUDO_BOLD: ScePvfSubstyle = 2;
pub const SCE_PVF_SUBSTYLE_PSEUDO_SLANT: ScePvfSubstyle = 4;
pub const SCE_PVF_SUBSTYLE_VERTICALLAYOUT: ScePvfSubstyle = 1;
pub const SCE_PVF_TRUE: ScePvfBoolValue = 1;
pub const SCE_PVF_USERIMAGE_DIRECT4_L: ScePvfImageByfferPixelFormatType = 0;
pub const SCE_PVF_USERIMAGE_DIRECT8: ScePvfImageByfferPixelFormatType = 2;
pub const SCE_RAZOR_GPU_LIVE_METRICS_GROUP_OVERVIEW_1: SceRazorGpuLiveMetricsGroup = 1;
pub const SCE_RAZOR_GPU_LIVE_METRICS_GROUP_OVERVIEW_2: SceRazorGpuLiveMetricsGroup = 2;
pub const SCE_RAZOR_GPU_LIVE_METRICS_GROUP_OVERVIEW_3: SceRazorGpuLiveMetricsGroup = 3;
pub const SCE_RAZOR_GPU_LIVE_METRICS_GROUP_PBUFFER_USAGE: SceRazorGpuLiveMetricsGroup = 0;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_ENTRY_TYPE_FRAME: SceRazorLiveTraceMetricEntryType = 2;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_ENTRY_TYPE_JOB: SceRazorLiveTraceMetricEntryType = 0;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_ENTRY_TYPE_PARAMETER_BUFFER:
    SceRazorLiveTraceMetricEntryType = 1;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_JOB_TYPE_FIRMWARE: SceRazorLiveTraceMetricJobType = 0;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_JOB_TYPE_FRAGMENT0: SceRazorLiveTraceMetricJobType = 8;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_JOB_TYPE_FRAGMENT1: SceRazorLiveTraceMetricJobType = 2;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_JOB_TYPE_FRAGMENT2: SceRazorLiveTraceMetricJobType = 4;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_JOB_TYPE_FRAGMENT3: SceRazorLiveTraceMetricJobType = 6;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_JOB_TYPE_VERTEX0: SceRazorLiveTraceMetricJobType = 7;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_JOB_TYPE_VERTEX1: SceRazorLiveTraceMetricJobType = 1;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_JOB_TYPE_VERTEX2: SceRazorLiveTraceMetricJobType = 3;
pub const SCE_RAZOR_LIVE_TRACE_METRIC_JOB_TYPE_VERTEX3: SceRazorLiveTraceMetricJobType = 5;
pub const __SCE_RTC_DAYOFWEEK: SceRtcDayOfWeek = 4294967295;
pub const SCE_RTC_DAYOFWEEK_FRIDAY: SceRtcDayOfWeek = 5;
pub const SCE_RTC_DAYOFWEEK_MONDAY: SceRtcDayOfWeek = 1;
pub const SCE_RTC_DAYOFWEEK_SATURDAY: SceRtcDayOfWeek = 6;
pub const SCE_RTC_DAYOFWEEK_SUNDAY: SceRtcDayOfWeek = 0;
pub const SCE_RTC_DAYOFWEEK_THURSDAY: SceRtcDayOfWeek = 4;
pub const SCE_RTC_DAYOFWEEK_TUESDAY: SceRtcDayOfWeek = 2;
pub const SCE_RTC_DAYOFWEEK_WEDNESDAY: SceRtcDayOfWeek = 3;
pub const SCE_RTC_ERROR_ALREADY_REGISTERD: SceRtcErrorCode = 2149912579;
pub const SCE_RTC_ERROR_BAD_PARSE: SceRtcErrorCode = 2149912704;
pub const SCE_RTC_ERROR_INVALID_DAY: SceRtcErrorCode = 2149912707;
pub const SCE_RTC_ERROR_INVALID_HOUR: SceRtcErrorCode = 2149912708;
pub const SCE_RTC_ERROR_INVALID_MICROSECOND: SceRtcErrorCode = 2149912711;
pub const SCE_RTC_ERROR_INVALID_MINUTE: SceRtcErrorCode = 2149912709;
pub const SCE_RTC_ERROR_INVALID_MONTH: SceRtcErrorCode = 2149912706;
pub const SCE_RTC_ERROR_INVALID_POINTER: SceRtcErrorCode = 2149912577;
pub const SCE_RTC_ERROR_INVALID_SECOND: SceRtcErrorCode = 2149912710;
pub const SCE_RTC_ERROR_INVALID_VALUE: SceRtcErrorCode = 2149912576;
pub const SCE_RTC_ERROR_INVALID_YEAR: SceRtcErrorCode = 2149912705;
pub const SCE_RTC_ERROR_NOT_FOUND: SceRtcErrorCode = 2149912580;
pub const SCE_RTC_ERROR_NOT_INITIALIZED: SceRtcErrorCode = 2149912578;
pub const SCE_SCREENSHOT_ERROR_FILE_NOT_FOUND: SceScreenshotErrorCode = 2148544259;
pub const SCE_SCREENSHOT_ERROR_INTERNAL: SceScreenshotErrorCode = 2148544262;
pub const SCE_SCREENSHOT_ERROR_INVALID_ARGUMENT: SceScreenshotErrorCode = 2148544257;
pub const SCE_SCREENSHOT_ERROR_MEDIA_FULL: SceScreenshotErrorCode = 2148544261;
pub const SCE_SCREENSHOT_ERROR_NO_MEMORY: SceScreenshotErrorCode = 2148544258;
pub const SCE_SCREENSHOT_ERROR_NOT_SUPPORTED_FORMAT: SceScreenshotErrorCode = 2148544260;
pub const SCE_SCREENSHOT_MAX_FS_PATH: u32 = 1024;
pub const SCE_SCREENSHOT_MAX_GAME_COMMENT_LEN: u32 = 128;
pub const SCE_SCREENSHOT_MAX_GAME_COMMENT_SIZE: u32 = 512;
pub const SCE_SCREENSHOT_MAX_GAME_TITLE_LEN: u32 = 64;
pub const SCE_SCREENSHOT_MAX_GAME_TITLE_SIZE: u32 = 256;
pub const SCE_SCREENSHOT_MAX_PHOTO_TITLE_LEN: u32 = 64;
pub const SCE_SCREENSHOT_MAX_PHOTO_TITLE_SIZE: u32 = 256;
pub const SCE_SEEK_CUR: SceIoSeekMode = 1;
pub const SCE_SEEK_END: SceIoSeekMode = 2;
pub const SCE_SEEK_SET: SceIoSeekMode = 0;
pub const SCE_SHA1_BLOCK_SIZE: u32 = 64;
pub const SCE_SHA1_DIGEST_SIZE: u32 = 20;
pub const SCE_SHA224_BLOCK_SIZE: u32 = 64;
pub const SCE_SHA224_DIGEST_SIZE: u32 = 28;
pub const SCE_SHA256_BLOCK_SIZE: u32 = 64;
pub const SCE_SHA256_DIGEST_SIZE: u32 = 32;
pub const SCE_SHACCCG_DIAGNOSTIC_LEVEL_ERROR: SceShaccCgDiagnosticLevel = 2;
pub const SCE_SHACCCG_DIAGNOSTIC_LEVEL_INFO: SceShaccCgDiagnosticLevel = 0;
pub const SCE_SHACCCG_DIAGNOSTIC_LEVEL_WARNING: SceShaccCgDiagnosticLevel = 1;
pub const SCE_SHACCCG_ENGLISH: SceShaccCgLocale = 0;
pub const SCE_SHACCCG_JAPANESE: SceShaccCgLocale = 1;
pub const SCE_SHACCCG_PROFILE_FP: SceShaccCgTargetProfile = 1;
pub const SCE_SHACCCG_PROFILE_VP: SceShaccCgTargetProfile = 0;
pub const SCE_SHACCCG_SYSTEM_FILES: SceShaccCgCallbackDefaults = 0;
pub const SCE_SHACCCG_TRIVIAL: SceShaccCgCallbackDefaults = 1;
pub const SCE_SHELL_UTIL_LOCK_MODE_LOCK: SceShellUtilLockMode = 1;
pub const SCE_SHELL_UTIL_LOCK_MODE_UNLOCK: SceShellUtilLockMode = 2;
pub const SCE_SHELL_UTIL_LOCK_TYPE_MC_INSERTED: SceShellUtilLockType = 32;
pub const SCE_SHELL_UTIL_LOCK_TYPE_MC_REMOVED: SceShellUtilLockType = 64;
pub const SCE_SHELL_UTIL_LOCK_TYPE_MUSIC_PLAYER: SceShellUtilLockType = 1024;
pub const SCE_SHELL_UTIL_LOCK_TYPE_POWEROFF_MENU: SceShellUtilLockType = 4;
pub const SCE_SHELL_UTIL_LOCK_TYPE_PS_BTN: SceShellUtilLockType = 1;
pub const SCE_SHELL_UTIL_LOCK_TYPE_PS_BTN_2: SceShellUtilLockType = 2048;
pub const SCE_SHELL_UTIL_LOCK_TYPE_QUICK_MENU: SceShellUtilLockType = 2;
pub const SCE_SHELL_UTIL_LOCK_TYPE_UNK100: SceShellUtilLockType = 256;
pub const SCE_SHELL_UTIL_LOCK_TYPE_UNK200: SceShellUtilLockType = 512;
pub const SCE_SHELL_UTIL_LOCK_TYPE_UNK8: SceShellUtilLockType = 8;
pub const SCE_SHELL_UTIL_LOCK_TYPE_UNK80: SceShellUtilLockType = 128;
pub const SCE_SHELL_UTIL_LOCK_TYPE_USB_CONNECTION: SceShellUtilLockType = 16;
pub const SCE_SHUTTER_SOUND_ERROR_FATAL: SceShutterSoundErrorCode = 2148553219;
pub const SCE_SHUTTER_SOUND_ERROR_INTERNAL: SceShutterSoundErrorCode = 2148553218;
pub const SCE_SHUTTER_SOUND_ERROR_INVALID_ARGUMENT: SceShutterSoundErrorCode = 2148553217;
pub const SCE_SHUTTER_SOUND_TYPE_SAVE_IMAGE: SceShutterSoundType = 0;
pub const SCE_SHUTTER_SOUND_TYPE_SAVE_VIDEO_END: SceShutterSoundType = 2;
pub const SCE_SHUTTER_SOUND_TYPE_SAVE_VIDEO_START: SceShutterSoundType = 1;
pub const SCE_S_IFDIR: SceIoAccessMode = 4096;
pub const SCE_S_IFLNK: SceIoAccessMode = 16384;
pub const SCE_S_IFMT: SceIoAccessMode = 61440;
pub const SCE_S_IFREG: SceIoAccessMode = 8192;
pub const SCE_S_IRGRP: SceIoAccessMode = 0;
pub const SCE_S_IROTH: SceIoAccessMode = 4;
pub const SCE_S_IRSYS: SceIoAccessMode = 4;
pub const SCE_S_IRUSR: SceIoAccessMode = 256;
pub const SCE_S_IRWXG: SceIoAccessMode = 0;
pub const SCE_S_IRWXO: SceIoAccessMode = 7;
pub const SCE_S_IRWXS: SceIoAccessMode = 7;
pub const SCE_S_IRWXU: SceIoAccessMode = 448;
pub const SCE_S_ISGID: SceIoAccessMode = 0;
pub const SCE_S_ISUID: SceIoAccessMode = 0;
pub const SCE_S_ISVTX: SceIoAccessMode = 0;
pub const SCE_S_IWGRP: SceIoAccessMode = 0;
pub const SCE_S_IWOTH: SceIoAccessMode = 2;
pub const SCE_S_IWSYS: SceIoAccessMode = 2;
pub const SCE_S_IWUSR: SceIoAccessMode = 128;
pub const SCE_S_IXGRP: SceIoAccessMode = 0;
pub const SCE_S_IXOTH: SceIoAccessMode = 1;
pub const SCE_S_IXSYS: SceIoAccessMode = 1;
pub const SCE_S_IXUSR: SceIoAccessMode = 64;
pub const SCE_SO_IFDIR: SceIoFileMode = 16;
pub const SCE_SO_IFLNK: SceIoFileMode = 8;
pub const SCE_SO_IFMT: SceIoFileMode = 56;
pub const SCE_SO_IFREG: SceIoFileMode = 32;
pub const SCE_SO_IROTH: SceIoFileMode = 4;
pub const SCE_SO_IWOTH: SceIoFileMode = 2;
pub const SCE_SO_IXOTH: SceIoFileMode = 1;
pub const SCE_SSL_ERROR_ALREADY_INITED: SceSslErrorCode = 2151895072;
pub const SCE_SSL_ERROR_BEFORE_INIT: SceSslErrorCode = 2151895041;
pub const SCE_SSL_ERROR_INTERNAL: SceSslErrorCode = 2151895078;
pub const SCE_SSL_ERROR_INVALID_FORMAT: SceSslErrorCode = 2151895304;
pub const SCE_SSL_ERROR_INVALID_VALUE: SceSslErrorCode = 2151895550;
pub const SCE_SSL_ERROR_NOT_FOUND: SceSslErrorCode = 2151895077;
pub const SCE_SSL_ERROR_OUT_OF_MEMORY: SceSslErrorCode = 2151895074;
pub const SCE_SYSCON_CMD_RESET_DEVICE: SceSysconCmd = 12;
pub const SCE_SYSCON_CTRL_CIRCLE: SceSysconControl = 32;
pub const SCE_SYSCON_CTRL_CROSS: SceSysconControl = 64;
pub const SCE_SYSCON_CTRL_DOWN: SceSysconControl = 4;
pub const SCE_SYSCON_CTRL_HEADPHONE: SceSysconControl = 134217728;
pub const SCE_SYSCON_CTRL_LEFT: SceSysconControl = 8;
pub const SCE_SYSCON_CTRL_LTRIGGER: SceSysconControl = 512;
pub const SCE_SYSCON_CTRL_POWER: SceSysconControl = 16384;
pub const SCE_SYSCON_CTRL_PSBUTTON: SceSysconControl = 4096;
pub const SCE_SYSCON_CTRL_RIGHT: SceSysconControl = 2;
pub const SCE_SYSCON_CTRL_RTRIGGER: SceSysconControl = 1024;
pub const SCE_SYSCON_CTRL_SELECT: SceSysconControl = 256;
pub const SCE_SYSCON_CTRL_SQUARE: SceSysconControl = 128;
pub const SCE_SYSCON_CTRL_START: SceSysconControl = 2048;
pub const SCE_SYSCON_CTRL_TRIANGLE: SceSysconControl = 16;
pub const SCE_SYSCON_CTRL_UP: SceSysconControl = 1;
pub const SCE_SYSCON_CTRL_VOLDOWN: SceSysconControl = 131072;
pub const SCE_SYSCON_CTRL_VOLUP: SceSysconControl = 65536;
pub const SCE_SYSCON_PACKET_RX_LENGTH: u32 = 2;
pub const SCE_SYSCON_PACKET_RX_RESULT: u32 = 3;
pub const SCE_SYSCON_PACKET_RX_STATUS_HI: u32 = 1;
pub const SCE_SYSCON_PACKET_RX_STATUS_LO: u32 = 0;
pub const SCE_SYSCON_PACKET_TX_CMD_HI: u32 = 1;
pub const SCE_SYSCON_PACKET_TX_CMD_LO: u32 = 0;
pub const SCE_SYSCON_PACKET_TX_LENGTH: u32 = 2;
pub const SCE_SYSCON_RESET_TYPE_COLD_RESET: SceSysconResetType = 2;
pub const SCE_SYSCON_RESET_TYPE_POWEROFF: SceSysconResetType = 0;
pub const SCE_SYSCON_RESET_TYPE_SOFT_RESET: SceSysconResetType = 17;
pub const SCE_SYSCON_RESET_TYPE_SUSPEND: SceSysconResetType = 1;
pub const SCE_SYSMODULE_AACENC: SceSysmoduleModuleId = 46;
pub const SCE_SYSMODULE_APPUTIL: SceSysmoduleModuleId = 15;
pub const SCE_SYSMODULE_APPUTIL_EXT: SceSysmoduleModuleId = 82;
pub const SCE_SYSMODULE_ATRAC: SceSysmoduleModuleId = 48;
pub const SCE_SYSMODULE_AUDIOCODEC: SceSysmoduleModuleId = 55;
pub const SCE_SYSMODULE_AVCDEC: SceSysmoduleModuleId = 84;
pub const SCE_SYSMODULE_AVPLAYER: SceSysmoduleModuleId = 76;
pub const SCE_SYSMODULE_BEISOBMF: SceSysmoduleModuleId = 71;
pub const SCE_SYSMODULE_BEMP2SYS: SceSysmoduleModuleId = 72;
pub const SCE_SYSMODULE_BG_APP_UTIL: SceSysmoduleModuleId = 52;
pub const SCE_SYSMODULE_CLIPBOARD: SceSysmoduleModuleId = 40;
pub const SCE_SYSMODULE_CODECENGINE_PERF: SceSysmoduleModuleId = 34;
pub const SCE_SYSMODULE_DBG: SceSysmoduleModuleId = 8;
pub const SCE_SYSMODULE_DTCP_IP: SceSysmoduleModuleId = 68;
pub const SCE_SYSMODULE_ERROR_FATAL: SceSysmoduleErrorCode = 2153386239;
pub const SCE_SYSMODULE_ERROR_INVALID_VALUE: SceSysmoduleErrorCode = 2153385984;
pub const SCE_SYSMODULE_ERROR_UNLOADED: SceSysmoduleErrorCode = 2153385985;
pub const SCE_SYSMODULE_FACE: SceSysmoduleModuleId = 56;
pub const SCE_SYSMODULE_FIBER: SceSysmoduleModuleId = 6;
pub const SCE_SYSMODULE_FIOS2: SceSysmoduleModuleId = 16;
pub const SCE_SYSMODULE_GAME_UPDATE: SceSysmoduleModuleId = 77;
pub const SCE_SYSMODULE_HANDWRITING: SceSysmoduleModuleId = 47;
pub const SCE_SYSMODULE_HTTP: SceSysmoduleModuleId = 2;
pub const SCE_SYSMODULE_HTTPS: SceSysmoduleModuleId = 4;
pub const SCE_SYSMODULE_IME: SceSysmoduleModuleId = 17;
pub const SCE_SYSMODULE_INCOMING_DIALOG: SceSysmoduleModuleId = 53;
pub const SCE_SYSMODULE_INTERNAL_ACTIVITY_DB: SceSysmoduleInternalModuleId = 2147483659;
pub const SCE_SYSMODULE_INTERNAL_AUDIOCODEC: SceSysmoduleInternalModuleId = 2147483650;
pub const SCE_SYSMODULE_INTERNAL_BXCE: SceSysmoduleInternalModuleId = 2147483653;
pub const SCE_SYSMODULE_INTERNAL_CHECKOUT_DIALOG: SceSysmoduleInternalModuleId = 2147483665;
pub const SCE_SYSMODULE_INTERNAL_COMMON_DIALOG_MAIN: SceSysmoduleInternalModuleId = 2147483666;
pub const SCE_SYSMODULE_INTERNAL_COMMON_GUI_DIALOG: SceSysmoduleInternalModuleId = 2147483660;
pub const SCE_SYSMODULE_INTERNAL_DB_RECOVERY_UTILITY: SceSysmoduleInternalModuleId = 2147483683;
pub const SCE_SYSMODULE_INTERNAL_DBUTIL: SceSysmoduleInternalModuleId = 2147483658;
pub const SCE_SYSMODULE_INTERNAL_DRM_PSM_KDC: SceSysmoduleInternalModuleId = 2147483687;
pub const SCE_SYSMODULE_INTERNAL_FRIEND_LIST_DIALOG: SceSysmoduleInternalModuleId = 2147483672;
pub const SCE_SYSMODULE_INTERNAL_G729: SceSysmoduleInternalModuleId = 2147483652;
pub const SCE_SYSMODULE_INTERNAL_IME_DIALOG: SceSysmoduleInternalModuleId = 2147483662;
pub const SCE_SYSMODULE_INTERNAL_INI_FILE_PROCESSOR: SceSysmoduleInternalModuleId = 2147483654;
pub const SCE_SYSMODULE_INTERNAL_JPEG_ARM: SceSysmoduleInternalModuleId = 2147483651;
pub const SCE_SYSMODULE_INTERNAL_JPEG_ENC_ARM: SceSysmoduleInternalModuleId = 2147483649;
pub const SCE_SYSMODULE_INTERNAL_LOCATION_FACTORY: SceSysmoduleInternalModuleId = 2147483689;
pub const SCE_SYSMODULE_INTERNAL_LOCATION_INTERNAL: SceSysmoduleInternalModuleId = 2147483688;
pub const SCE_SYSMODULE_INTERNAL_MSG_DIALOG: SceSysmoduleInternalModuleId = 2147483667;
pub const SCE_SYSMODULE_INTERNAL_MUSIC_EXPORT: SceSysmoduleInternalModuleId = 2147483679;
pub const SCE_SYSMODULE_INTERNAL_NEAR_PROFILE: SceSysmoduleInternalModuleId = 2147483674;
pub const SCE_SYSMODULE_INTERNAL_NET_CHECK_DIALOG: SceSysmoduleInternalModuleId = 2147483668;
pub const SCE_SYSMODULE_INTERNAL_NP_ACTIVITY_NET: SceSysmoduleInternalModuleId = 2147483655;
pub const SCE_SYSMODULE_INTERNAL_NP_COMMERCE2: SceSysmoduleInternalModuleId = 2147483677;
pub const SCE_SYSMODULE_INTERNAL_NP_FRIEND_PRIVACY_LEVEL: SceSysmoduleInternalModuleId = 2147483675;
pub const SCE_SYSMODULE_INTERNAL_NP_KDC: SceSysmoduleInternalModuleId = 2147483678;
pub const SCE_SYSMODULE_INTERNAL_NP_MESSAGE_CONTACTS: SceSysmoduleInternalModuleId = 2147483682;
pub const SCE_SYSMODULE_INTERNAL_NP_MESSAGE_DIALOG: SceSysmoduleInternalModuleId = 2147483670;
pub const SCE_SYSMODULE_INTERNAL_NP_MESSAGE_DIALOG_IMPL: SceSysmoduleInternalModuleId = 2147483681;
pub const SCE_SYSMODULE_INTERNAL_PAF: SceSysmoduleInternalModuleId = 2147483656;
pub const SCE_SYSMODULE_INTERNAL_PARTY_MEMBER_LIST: SceSysmoduleInternalModuleId = 2147483686;
pub const SCE_SYSMODULE_INTERNAL_PHOTO_IMPORT_DIALOG: SceSysmoduleInternalModuleId = 2147483663;
pub const SCE_SYSMODULE_INTERNAL_PHOTO_REVIEW_DIALOG: SceSysmoduleInternalModuleId = 2147483664;
pub const SCE_SYSMODULE_INTERNAL_PROMOTER_UTIL: SceSysmoduleInternalModuleId = 2147483684;
pub const SCE_SYSMODULE_INTERNAL_SAVEDATA_DIALOG: SceSysmoduleInternalModuleId = 2147483669;
pub const SCE_SYSMODULE_INTERNAL_SQLITE_VSH: SceSysmoduleInternalModuleId = 2147483657;
pub const SCE_SYSMODULE_INTERNAL_STORE_CHECKOUT: SceSysmoduleInternalModuleId = 2147483661;
pub const SCE_SYSMODULE_INTERNAL_TROPHY_SETUP_DIALOG: SceSysmoduleInternalModuleId = 2147483671;
pub const SCE_SYSMODULE_INTERNAL_ULT: SceSysmoduleInternalModuleId = 2147483685;
pub const SCE_SYSMODULE_INTERNAL_VIDEO_EXPORT: SceSysmoduleInternalModuleId = 2147483680;
pub const SCE_SYSMODULE_INVALID: SceSysmoduleModuleId = 0;
pub const SCE_SYSMODULE_IPMI: SceSysmoduleModuleId = 54;
pub const SCE_SYSMODULE_JSON: SceSysmoduleModuleId = 85;
pub const SCE_SYSMODULE_LIVEAREA: SceSysmoduleModuleId = 35;
pub const SCE_SYSMODULE_LOADED: SceSysmoduleErrorCode = 0;
pub const SCE_SYSMODULE_LOCATION: SceSysmoduleModuleId = 20;
pub const SCE_SYSMODULE_LOCATION_EXTENSION: SceSysmoduleModuleId = 75;
pub const SCE_SYSMODULE_MAIL_API: SceSysmoduleModuleId = 78;
pub const SCE_SYSMODULE_MARLIN: SceSysmoduleModuleId = 58;
pub const SCE_SYSMODULE_MARLIN_APP_LIB: SceSysmoduleModuleId = 60;
pub const SCE_SYSMODULE_MARLIN_DOWNLOADER: SceSysmoduleModuleId = 59;
pub const SCE_SYSMODULE_MONO: SceSysmoduleModuleId = 64;
pub const SCE_SYSMODULE_MONO_BRIDGE: SceSysmoduleModuleId = 63;
pub const SCE_SYSMODULE_MP4: SceSysmoduleModuleId = 45;
pub const SCE_SYSMODULE_MP4_RECORDER: SceSysmoduleModuleId = 81;
pub const SCE_SYSMODULE_MUSIC_EXPORT: SceSysmoduleModuleId = 73;
pub const SCE_SYSMODULE_NEAR_DIALOG_UTIL: SceSysmoduleModuleId = 74;
pub const SCE_SYSMODULE_NEAR_UTIL: SceSysmoduleModuleId = 43;
pub const SCE_SYSMODULE_NET: SceSysmoduleModuleId = 1;
pub const SCE_SYSMODULE_NET_ADHOC_MATCHING: SceSysmoduleModuleId = 42;
pub const SCE_SYSMODULE_NGS: SceSysmoduleModuleId = 11;
pub const SCE_SYSMODULE_NOTIFICATION_UTIL: SceSysmoduleModuleId = 51;
pub const SCE_SYSMODULE_NP: SceSysmoduleModuleId = 21;
pub const SCE_SYSMODULE_NP_ACTIVITY: SceSysmoduleModuleId = 36;
pub const SCE_SYSMODULE_NP_BASIC: SceSysmoduleModuleId = 18;
pub const SCE_SYSMODULE_NP_COMMERCE2: SceSysmoduleModuleId = 24;
pub const SCE_SYSMODULE_NP_MATCHING2: SceSysmoduleModuleId = 28;
pub const SCE_SYSMODULE_NP_MESSAGE: SceSysmoduleModuleId = 38;
pub const SCE_SYSMODULE_NP_PARTY: SceSysmoduleModuleId = 41;
pub const SCE_SYSMODULE_NP_SCORE_RANKING: SceSysmoduleModuleId = 30;
pub const SCE_SYSMODULE_NP_SIGNALING: SceSysmoduleModuleId = 70;
pub const SCE_SYSMODULE_NP_SNS_FACEBOOK: SceSysmoduleModuleId = 49;
pub const SCE_SYSMODULE_NP_TROPHY: SceSysmoduleModuleId = 37;
pub const SCE_SYSMODULE_NP_TUS: SceSysmoduleModuleId = 44;
pub const SCE_SYSMODULE_NP_UTILITY: SceSysmoduleModuleId = 25;
pub const SCE_SYSMODULE_NP_WEBAPI: SceSysmoduleModuleId = 83;
pub const SCE_SYSMODULE_PERF: SceSysmoduleModuleId = 5;
pub const SCE_SYSMODULE_PGF: SceSysmoduleModuleId = 14;
pub const SCE_SYSMODULE_PHOTO_EXPORT: SceSysmoduleModuleId = 22;
pub const SCE_SYSMODULE_PSM: SceSysmoduleModuleId = 65;
pub const SCE_SYSMODULE_PSM_DEVAGENT: SceSysmoduleModuleId = 66;
pub const SCE_SYSMODULE_PSPNET_ADHOC: SceSysmoduleModuleId = 67;
pub const SCE_SYSMODULE_RAZOR_CAPTURE: SceSysmoduleModuleId = 9;
pub const SCE_SYSMODULE_RAZOR_HUD: SceSysmoduleModuleId = 10;
pub const SCE_SYSMODULE_RUDP: SceSysmoduleModuleId = 33;
pub const SCE_SYSMODULE_SAS: SceSysmoduleModuleId = 13;
pub const SCE_SYSMODULE_SCREEN_SHOT: SceSysmoduleModuleId = 29;
pub const SCE_SYSMODULE_SHACCCG: SceSysmoduleModuleId = 62;
pub const SCE_SYSMODULE_SHUTTER_SOUND: SceSysmoduleModuleId = 39;
pub const SCE_SYSMODULE_SMART: SceSysmoduleModuleId = 57;
pub const SCE_SYSMODULE_SQLITE: SceSysmoduleModuleId = 31;
pub const SCE_SYSMODULE_SSL: SceSysmoduleModuleId = 3;
pub const SCE_SYSMODULE_SULPHA: SceSysmoduleModuleId = 12;
pub const SCE_SYSMODULE_SYSTEM_GESTURE: SceSysmoduleModuleId = 19;
pub const SCE_SYSMODULE_TELEPHONY_UTIL: SceSysmoduleModuleId = 61;
pub const SCE_SYSMODULE_TELEPORT_CLIENT: SceSysmoduleModuleId = 79;
pub const SCE_SYSMODULE_TELEPORT_SERVER: SceSysmoduleModuleId = 80;
pub const SCE_SYSMODULE_TRIGGER_UTIL: SceSysmoduleModuleId = 32;
pub const SCE_SYSMODULE_ULT: SceSysmoduleModuleId = 7;
pub const SCE_SYSMODULE_VIDEO_EXPORT: SceSysmoduleModuleId = 50;
pub const SCE_SYSMODULE_VIDEO_SEARCH_EMPR: SceSysmoduleModuleId = 69;
pub const SCE_SYSMODULE_VOICE: SceSysmoduleModuleId = 26;
pub const SCE_SYSMODULE_VOICEQOS: SceSysmoduleModuleId = 27;
pub const SCE_SYSMODULE_XML: SceSysmoduleModuleId = 23;
pub const SCE_SYSROOT_INIT_CALLBACK_MAX_FUNC: u32 = 9;
pub const SCE_SYSROOT_INIT_CALLBACK_MAX_SLOT: u32 = 8;
pub const SCE_SYSTEM_PARAM_DATE_FORMAT_DDMMYYYY: SceSystemParamDateFormat = 1;
pub const SCE_SYSTEM_PARAM_DATE_FORMAT_MMDDYYYY: SceSystemParamDateFormat = 2;
pub const SCE_SYSTEM_PARAM_DATE_FORMAT_YYYYMMDD: SceSystemParamDateFormat = 0;
pub const SCE_SYSTEM_PARAM_ENTER_BUTTON_CIRCLE: SceSystemParamEnterButtonAssign = 0;
pub const SCE_SYSTEM_PARAM_ENTER_BUTTON_CROSS: SceSystemParamEnterButtonAssign = 1;
pub const SCE_SYSTEM_PARAM_ENTER_BUTTON_MAX_VALUE: SceSystemParamEnterButtonAssign = 4294967295;
pub const SCE_SYSTEM_PARAM_ID_DATE_FORMAT: SceSystemParamId = 4;
pub const SCE_SYSTEM_PARAM_ID_DAYLIGHT_SAVINGS: SceSystemParamId = 7;
pub const SCE_SYSTEM_PARAM_ID_ENTER_BUTTON: SceSystemParamId = 2;
pub const SCE_SYSTEM_PARAM_ID_LANG: SceSystemParamId = 1;
pub const SCE_SYSTEM_PARAM_ID_MAX_VALUE: SceSystemParamId = 4294967295;
pub const SCE_SYSTEM_PARAM_ID_TIME_FORMAT: SceSystemParamId = 5;
pub const SCE_SYSTEM_PARAM_ID_TIME_ZONE: SceSystemParamId = 6;
pub const SCE_SYSTEM_PARAM_ID_USERNAME: SceSystemParamId = 3;
pub const SCE_SYSTEM_PARAM_LANG_CHINESE_S: SceSystemParamLang = 11;
pub const SCE_SYSTEM_PARAM_LANG_CHINESE_T: SceSystemParamLang = 10;
pub const SCE_SYSTEM_PARAM_LANG_DANISH: SceSystemParamLang = 14;
pub const SCE_SYSTEM_PARAM_LANG_DUTCH: SceSystemParamLang = 6;
pub const SCE_SYSTEM_PARAM_LANG_ENGLISH_GB: SceSystemParamLang = 18;
pub const SCE_SYSTEM_PARAM_LANG_ENGLISH_US: SceSystemParamLang = 1;
pub const SCE_SYSTEM_PARAM_LANG_FINNISH: SceSystemParamLang = 12;
pub const SCE_SYSTEM_PARAM_LANG_FRENCH: SceSystemParamLang = 2;
pub const SCE_SYSTEM_PARAM_LANG_GERMAN: SceSystemParamLang = 4;
pub const SCE_SYSTEM_PARAM_LANG_ITALIAN: SceSystemParamLang = 5;
pub const SCE_SYSTEM_PARAM_LANG_JAPANESE: SceSystemParamLang = 0;
pub const SCE_SYSTEM_PARAM_LANG_KOREAN: SceSystemParamLang = 9;
pub const SCE_SYSTEM_PARAM_LANG_MAX_VALUE: SceSystemParamLang = 4294967295;
pub const SCE_SYSTEM_PARAM_LANG_NORWEGIAN: SceSystemParamLang = 15;
pub const SCE_SYSTEM_PARAM_LANG_POLISH: SceSystemParamLang = 16;
pub const SCE_SYSTEM_PARAM_LANG_PORTUGUESE_BR: SceSystemParamLang = 17;
pub const SCE_SYSTEM_PARAM_LANG_PORTUGUESE_PT: SceSystemParamLang = 7;
pub const SCE_SYSTEM_PARAM_LANG_RUSSIAN: SceSystemParamLang = 8;
pub const SCE_SYSTEM_PARAM_LANG_SPANISH: SceSystemParamLang = 3;
pub const SCE_SYSTEM_PARAM_LANG_SWEDISH: SceSystemParamLang = 13;
pub const SCE_SYSTEM_PARAM_LANG_TURKISH: SceSystemParamLang = 19;
pub const SCE_SYSTEM_PARAM_TIME_FORMAT_12HR: SceSystemParamTimeFormat = 0;
pub const SCE_SYSTEM_PARAM_TIME_FORMAT_24HR: SceSystemParamTimeFormat = 1;
pub const SCE_SYSTEM_PARAM_USERNAME_MAXSIZE: u32 = 17;
pub const SCE_SYSTIMER_CLOCK_SOURCE_48MHZ: SceSysTimerClockSource = 3;
pub const SCE_SYSTIMER_CLOCK_SOURCE_SYS: SceSysTimerClockSource = 0;
pub const SCE_SYSTIMER_TYPE_LONG: SceSysTimerType = 1;
pub const SCE_SYSTIMER_TYPE_WORD: SceSysTimerType = 2;
pub const SCE_THREAD_DEAD: SceThreadStatus = 64;
pub const SCE_THREAD_DELETED: SceThreadStatus = 32;
pub const SCE_THREAD_DORMANT: SceThreadStatus = 16;
pub const SCE_THREAD_KILLED: SceThreadStatus = 32;
pub const SCE_THREAD_READY: SceThreadStatus = 2;
pub const SCE_THREAD_RUNNING: SceThreadStatus = 1;
pub const SCE_THREAD_STAGNANT: SceThreadStatus = 128;
pub const SCE_THREAD_STANDBY: SceThreadStatus = 4;
pub const SCE_THREAD_STOPPED: SceThreadStatus = 16;
pub const SCE_THREAD_SUSPEND: SceThreadStatus = 8;
pub const SCE_THREAD_SUSPENDED: SceThreadStatus = 256;
pub const SCE_THREAD_WAITING: SceThreadStatus = 8;
pub const SCE_TOUCH_ERROR_FATAL: SceTouchErrorCode = 2150957311;
pub const SCE_TOUCH_ERROR_INVALID_ARG: SceTouchErrorCode = 2150957057;
pub const SCE_TOUCH_ERROR_PRIV_REQUIRED: SceTouchErrorCode = 2150957058;
pub const SCE_TOUCH_MAX_REPORT: u32 = 8;
pub const SCE_TOUCH_PORT_BACK: SceTouchPortType = 1;
pub const SCE_TOUCH_PORT_FRONT: SceTouchPortType = 0;
pub const SCE_TOUCH_PORT_MAX_NUM: SceTouchPortType = 2;
pub const SCE_TOUCH_REPORT_INFO_HIDE_UPPER_LAYER: SceTouchReportInfo = 1;
pub const SCE_TOUCH_SAMPLING_STATE_START: SceTouchSamplingState = 1;
pub const SCE_TOUCH_SAMPLING_STATE_STOP: SceTouchSamplingState = 0;
pub const SCE_TRIGGER_UTIL_ERROR_BUSY: SceTriggerUtilErrorCode = 2148546048;
pub const SCE_TRIGGER_UTIL_ERROR_EVENT_TYPE_MISMATCH: SceTriggerUtilErrorCode = 2148546084;
pub const SCE_TRIGGER_UTIL_ERROR_INVALID_ARG: SceTriggerUtilErrorCode = 2148546144;
pub const SCE_TRIGGER_UTIL_ERROR_NOT_FOUND_SYSTEM: SceTriggerUtilErrorCode = 2148546068;
pub const SCE_TRIGGER_UTIL_ERROR_NOT_FOUND_USER: SceTriggerUtilErrorCode = 2148546065;
pub const SCE_TRIGGER_UTIL_ERROR_NOT_REGISTERED: SceTriggerUtilErrorCode = 2148546081;
pub const SCE_TRIGGER_UTIL_FRIDAY: SceTriggerUtilDays = 32;
pub const SCE_TRIGGER_UTIL_MONDAY: SceTriggerUtilDays = 2;
pub const SCE_TRIGGER_UTIL_SATURDAY: SceTriggerUtilDays = 64;
pub const SCE_TRIGGER_UTIL_SUNDAY: SceTriggerUtilDays = 1;
pub const SCE_TRIGGER_UTIL_THURSDAY: SceTriggerUtilDays = 16;
pub const SCE_TRIGGER_UTIL_TUESDAY: SceTriggerUtilDays = 4;
pub const SCE_TRIGGER_UTIL_VERSION: u32 = 52428800;
pub const SCE_TRIGGER_UTIL_WEDNESDAY: SceTriggerUtilDays = 8;
pub const SCE_TRUE: _bindgen_ty_1 = 1;
pub const SCE_UDCD_DEVICE_REQUEST_ATTR_PHYCONT: SceUdcdDeviceRequestAttr = 1;
pub const SCE_UDCD_ERROR_ALREADY_DONE: SceUdcdErrorCode = 2149855233;
pub const SCE_UDCD_ERROR_ARGUMENT_EXCEEDED_LIMIT: SceUdcdErrorCode = 2149855235;
pub const SCE_UDCD_ERROR_BUS_DRIVER_NOT_STARTED: SceUdcdErrorCode = 2149855239;
pub const SCE_UDCD_ERROR_DRIVER_IN_PROGRESS: SceUdcdErrorCode = 2149855238;
pub const SCE_UDCD_ERROR_DRIVER_NOT_FOUND: SceUdcdErrorCode = 2149855237;
pub const SCE_UDCD_ERROR_ILLEGAL_CONTEXT: SceUdcdErrorCode = 2147483696;
pub const SCE_UDCD_ERROR_INVALID_ARGUMENT: SceUdcdErrorCode = 2149855234;
pub const SCE_UDCD_ERROR_INVALID_FLAG: SceUdcdErrorCode = 2147483909;
pub const SCE_UDCD_ERROR_INVALID_POINTER: SceUdcdErrorCode = 2147483907;
pub const SCE_UDCD_ERROR_INVALID_VALUE: SceUdcdErrorCode = 2147484158;
pub const SCE_UDCD_ERROR_MEMORY_EXHAUSTED: SceUdcdErrorCode = 2149855236;
pub const SCE_UDCD_ERROR_USBDRIVER_INVALID_DRIVER: SceUdcdErrorCode = 2149855744;
pub const SCE_UDCD_ERROR_USBDRIVER_INVALID_FUNCS: SceUdcdErrorCode = 2149855746;
pub const SCE_UDCD_ERROR_USBDRIVER_INVALID_NAME: SceUdcdErrorCode = 2149855745;
pub const SCE_UDCD_ERROR_WAIT_CANCEL: SceUdcdErrorCode = 2149855241;
pub const SCE_UDCD_ERROR_WAIT_TIMEOUT: SceUdcdErrorCode = 2149855240;
pub const SCE_UDCD_MAX_ALTERNATE: u32 = 2;
pub const SCE_UDCD_MAX_ENDPOINTS: u32 = 9;
pub const SCE_UDCD_MAX_INTERFACES: u32 = 8;
pub const SCE_UDCD_RETCODE_CANCEL: SceUdcdRetcode = -1;
pub const SCE_UDCD_RETCODE_CANCEL_ALL: SceUdcdRetcode = -2;
pub const SCE_UDCD_RETCODE_CANCEL_TRANSMISSION: SceUdcdRetcode = -3;
pub const SCE_UDCD_RETCODE_RECV: SceUdcdRetcode = 2;
pub const SCE_UDCD_RETCODE_SEND: SceUdcdRetcode = 1;
pub const SCE_UDCD_RETCODE_SUCCESS: SceUdcdRetcode = 0;
pub const SCE_UDCD_STATUS_ACTIVATED: SceUdcdStatus = 512;
pub const SCE_UDCD_STATUS_CABLE_CONNECTED: SceUdcdStatus = 32;
pub const SCE_UDCD_STATUS_CABLE_DISCONNECTED: SceUdcdStatus = 16;
pub const SCE_UDCD_STATUS_CONNECTION_ESTABLISHED: SceUdcdStatus = 2;
pub const SCE_UDCD_STATUS_CONNECTION_NEW: SceUdcdStatus = 1;
pub const SCE_UDCD_STATUS_CONNECTION_SUSPENDED: SceUdcdStatus = 4;
pub const SCE_UDCD_STATUS_DEACTIVATED: SceUdcdStatus = 256;
pub const SCE_UDCD_STATUS_DRIVER_REGISTERED: SceUdcdStatusDriver = 2;
pub const SCE_UDCD_STATUS_DRIVER_STARTED: SceUdcdStatusDriver = 1;
pub const SCE_UDCD_STATUS_IS_CHARGING: SceUdcdStatus = 1024;
pub const SCE_UDCD_STATUS_UNKNOWN_1000: SceUdcdStatus = 4096;
pub const SCE_UDCD_STATUS_UNKNOWN_2000: SceUdcdStatus = 8192;
pub const SCE_UDCD_STATUS_USE_USB_CHARGING: SceUdcdStatus = 2048;
pub const SCE_UID_NAMELEN: u32 = 31;
pub const SCE_UPDATE_MODE_SWU_CUI: u32 = 48;
pub const SCE_UPDATE_MODE_SWU_GUI: u32 = 16;
pub const SCE_USBAUDIO_IN_ERROR_CANNOT_GET_PORT_OWNERSHIP: SceUsbAudioInErrorCode = 2151546886;
pub const SCE_USBAUDIO_IN_ERROR_DEVICE_NOT_FOUND: SceUsbAudioInErrorCode = 2151546884;
pub const SCE_USBAUDIO_IN_ERROR_DEVICE_WAS_HALTED: SceUsbAudioInErrorCode = 2151546890;
pub const SCE_USBAUDIO_IN_ERROR_DUPLICATE_ID: SceUsbAudioInErrorCode = 2151546882;
pub const SCE_USBAUDIO_IN_ERROR_FAILED_TO_COPY_BUFFER: SceUsbAudioInErrorCode = 2151546892;
pub const SCE_USBAUDIO_IN_ERROR_FAILED_TO_READ_STREAM: SceUsbAudioInErrorCode = 2151546889;
pub const SCE_USBAUDIO_IN_ERROR_FAILED_TO_REQUEST_ISOCHRONOUS: SceUsbAudioInErrorCode = 2151546893;
pub const SCE_USBAUDIO_IN_ERROR_INVALID_ARGUMENT: SceUsbAudioInErrorCode = 2151546881;
pub const SCE_USBAUDIO_IN_ERROR_NO_DATA_TO_READ: SceUsbAudioInErrorCode = 2151546891;
pub const SCE_USBAUDIO_IN_ERROR_NO_MEMORY: SceUsbAudioInErrorCode = 2151546883;
pub const SCE_USBAUDIO_IN_ERROR_NOT_SUPPORTED: SceUsbAudioInErrorCode = 2151546885;
pub const SCE_USBAUDIO_IN_ERROR_PORT_IS_ALREADY_OPENED: SceUsbAudioInErrorCode = 2151546887;
pub const SCE_USBAUDIO_IN_ERROR_PROCESS_CANNOT_OPEN_MORE_DEVICE: SceUsbAudioInErrorCode =
    2151546895;
pub const SCE_USBAUDIO_IN_ERROR_PROCESS_HAS_NOT_A_DEVICE_OWNERSHIP: SceUsbAudioInErrorCode =
    2151546888;
pub const SCE_USBAUDIO_IN_ERROR_TIMEOUT: SceUsbAudioInErrorCode = 2151546894;
pub const SCE_USBD_ATTACH_FAILED: i32 = -1;
pub const SCE_USBD_ATTACH_SUCCEEDED: u32 = 0;
pub const SCE_USBD_CLASS_AUDIO: u32 = 1;
pub const SCE_USBD_CLASS_COMMUNICATIONS: u32 = 2;
pub const SCE_USBD_CLASS_DATA: u32 = 10;
pub const SCE_USBD_CLASS_HID: u32 = 3;
pub const SCE_USBD_CLASS_HUB: u32 = 9;
pub const SCE_USBD_CLASS_MONITOR: u32 = 4;
pub const SCE_USBD_CLASS_PER_INTERFACE: u32 = 0;
pub const SCE_USBD_CLASS_PHYSICAL: u32 = 5;
pub const SCE_USBD_CLASS_POWER: u32 = 6;
pub const SCE_USBD_CLASS_PRINTER: u32 = 7;
pub const SCE_USBD_CLASS_STORAGE: u32 = 8;
pub const SCE_USBD_CLASS_VENDOR_SPECIFIC: u32 = 255;
pub const SCE_USBD_CONFIGURATION_REMOTE_WAKEUP: u32 = 32;
pub const SCE_USBD_CONFIGURATION_RESERVED_ONE: u32 = 128;
pub const SCE_USBD_CONFIGURATION_RESERVED_ZERO: u32 = 31;
pub const SCE_USBD_CONFIGURATION_SELF_POWERED: u32 = 64;
pub const SCE_USBD_DESCRIPTOR_CONFIGURATION: SceUsbdDescriptorType = 2;
pub const SCE_USBD_DESCRIPTOR_DEVICE: SceUsbdDescriptorType = 1;
pub const SCE_USBD_DESCRIPTOR_DEVICE_QUALIFIER: SceUsbdDescriptorType = 6;
pub const SCE_USBD_DESCRIPTOR_ENDPOINT: SceUsbdDescriptorType = 5;
pub const SCE_USBD_DESCRIPTOR_HID: SceUsbdDescriptorType = 33;
pub const SCE_USBD_DESCRIPTOR_INTERFACE: SceUsbdDescriptorType = 4;
pub const SCE_USBD_DESCRIPTOR_INTERFACE_POWER: SceUsbdDescriptorType = 8;
pub const SCE_USBD_DESCRIPTOR_OTG: SceUsbdDescriptorType = 9;
pub const SCE_USBD_DESCRIPTOR_OTHER_SPEED: SceUsbdDescriptorType = 7;
pub const SCE_USBD_DESCRIPTOR_REPORT: SceUsbdDescriptorType = 34;
pub const SCE_USBD_DESCRIPTOR_STRING: SceUsbdDescriptorType = 3;
pub const SCE_USBD_DETACH_FAILED: i32 = -1;
pub const SCE_USBD_DETACH_SUCCEEDED: u32 = 0;
pub const SCE_USBD_DEVICE_SPEED_FS: u32 = 1;
pub const SCE_USBD_DEVICE_SPEED_HS: u32 = 2;
pub const SCE_USBD_DEVICE_SPEED_LS: u32 = 0;
pub const SCE_USBD_ENDPOINT_DIRECTION_BITS: u32 = 128;
pub const SCE_USBD_ENDPOINT_DIRECTION_IN: u32 = 128;
pub const SCE_USBD_ENDPOINT_DIRECTION_OUT: u32 = 0;
pub const SCE_USBD_ENDPOINT_DIRECTION_SHIFT: u32 = 7;
pub const SCE_USBD_ENDPOINT_NUMBER_BITS: u32 = 31;
pub const SCE_USBD_ENDPOINT_NUMBER_SHIFT: u32 = 0;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_BITS: u32 = 3;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_BULK: u32 = 2;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_CONTROL: u32 = 0;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_INTERRUPT: u32 = 3;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_ISOCHRONOUS: u32 = 1;
pub const SCE_USBD_ENDPOINT_TRANSFER_TYPE_SHIFT: u32 = 0;
pub const SCE_USBD_ERROR_80240007: SceUsbdErrorCode = 2149842951;
pub const SCE_USBD_ERROR_80240009: SceUsbdErrorCode = 2149842953;
pub const SCE_USBD_ERROR_8024000A: SceUsbdErrorCode = 2149842954;
pub const SCE_USBD_ERROR_ALREADY_INITIALIZED: SceUsbdErrorCode = 2149842946;
pub const SCE_USBD_ERROR_DEVICE_NOT_FOUND: SceUsbdErrorCode = 2149842950;
pub const SCE_USBD_ERROR_FATAL: SceUsbdErrorCode = 2149843199;
pub const SCE_USBD_ERROR_INVALID_PARAM: SceUsbdErrorCode = 2149842947;
pub const SCE_USBD_ERROR_NO_MEMORY: SceUsbdErrorCode = 2149842949;
pub const SCE_USBD_ERROR_NOT_INITIALIZED: SceUsbdErrorCode = 2149842945;
pub const SCE_USBD_ERROR_PIPE_NOT_FOUND: SceUsbdErrorCode = 2149842948;
pub const SCE_USBD_FEATURE_DEVICE_REMOTE_WAKEUP: u32 = 1;
pub const SCE_USBD_FEATURE_ENDPOINT_HALT: u32 = 0;
pub const SCE_USBD_MAX_BULK_PACKET_SIZE: u32 = 64;
pub const SCE_USBD_MAX_FS_CONTROL_PACKET_SIZE: u32 = 64;
pub const SCE_USBD_MAX_FS_INTERRUPT_PACKET_SIZE: u32 = 64;
pub const SCE_USBD_MAX_ISOCHRONOUS_PACKET_SIZE: u32 = 1023;
pub const SCE_USBD_MAX_LS_CONTROL_PACKET_SIZE: u32 = 8;
pub const SCE_USBD_MAX_LS_INTERRUPT_PACKET_SIZE: u32 = 8;
pub const SCE_USBD_PROBE_FAILED: i32 = -1;
pub const SCE_USBD_PROBE_SUCCEEDED: u32 = 0;
pub const SCE_USBD_REQTYPE_DIR_BITS: SceUsbdReqtype = 128;
pub const SCE_USBD_REQTYPE_DIR_TO_DEVICE: SceUsbdReqtype = 0;
pub const SCE_USBD_REQTYPE_DIR_TO_HOST: SceUsbdReqtype = 128;
pub const SCE_USBD_REQTYPE_RECIP_BITS: SceUsbdReqtype = 31;
pub const SCE_USBD_REQTYPE_RECIP_DEVICE: SceUsbdReqtype = 0;
pub const SCE_USBD_REQTYPE_RECIP_ENDPOINT: SceUsbdReqtype = 2;
pub const SCE_USBD_REQTYPE_RECIP_INTERFACE: SceUsbdReqtype = 1;
pub const SCE_USBD_REQTYPE_RECIP_OTHER: SceUsbdReqtype = 3;
pub const SCE_USBD_REQTYPE_TYPE_BITS: SceUsbdReqtype = 96;
pub const SCE_USBD_REQTYPE_TYPE_CLASS: SceUsbdReqtype = 32;
pub const SCE_USBD_REQTYPE_TYPE_RESERVED: SceUsbdReqtype = 96;
pub const SCE_USBD_REQTYPE_TYPE_STANDARD: SceUsbdReqtype = 0;
pub const SCE_USBD_REQTYPE_TYPE_VENDOR: SceUsbdReqtype = 64;
pub const SCE_USBD_REQUEST_CLEAR_FEATURE: SceUsbdRequest = 1;
pub const SCE_USBD_REQUEST_GET_CONFIGURATION: SceUsbdRequest = 8;
pub const SCE_USBD_REQUEST_GET_DESCRIPTOR: SceUsbdRequest = 6;
pub const SCE_USBD_REQUEST_GET_INTERFACE: SceUsbdRequest = 10;
pub const SCE_USBD_REQUEST_GET_STATUS: SceUsbdRequest = 0;
pub const SCE_USBD_REQUEST_SET_ADDRESS: SceUsbdRequest = 5;
pub const SCE_USBD_REQUEST_SET_CONFIGURATION: SceUsbdRequest = 9;
pub const SCE_USBD_REQUEST_SET_DESCRIPTOR: SceUsbdRequest = 7;
pub const SCE_USBD_REQUEST_SET_FEATURE: SceUsbdRequest = 3;
pub const SCE_USBD_REQUEST_SET_INTERFACE: SceUsbdRequest = 11;
pub const SCE_USBD_REQUEST_SYNCH_FRAME: SceUsbdRequest = 12;
pub const SCE_USBSERV_ERROR_FATAL: SceUsbservErrorCode = 2149875967;
pub const SCE_USBSERV_ERROR_INVALID_PARAM: SceUsbservErrorCode = 2149875715;
pub const SCE_USBSERV_ERROR_NOT_SUPPORTED: SceUsbservErrorCode = 2149875716;
pub const SCE_USBSERV_ERROR_UNAVAILABLE: SceUsbservErrorCode = 2149875714;
pub const SCE_USBSTOR_VSTOR_TYPE_CDROM: SceUsbstorVstorType = 5;
pub const SCE_USBSTOR_VSTOR_TYPE_FAT: SceUsbstorVstorType = 0;
pub const SCE_VIDEODEC_ERROR_ALREADY_USED: SceVideodecErrorCode = 2153908232;
pub const SCE_VIDEODEC_ERROR_ES_BUFFER_FULL: SceVideodecErrorCode = 2153908234;
pub const SCE_VIDEODEC_ERROR_INITIALIZE: SceVideodecErrorCode = 2153908235;
pub const SCE_VIDEODEC_ERROR_INVALID_ARGUMENT_SIZE: SceVideodecErrorCode = 2153908238;
pub const SCE_VIDEODEC_ERROR_INVALID_COLOR_FORMAT: SceVideodecErrorCode = 2153908230;
pub const SCE_VIDEODEC_ERROR_INVALID_PARAM: SceVideodecErrorCode = 2153908226;
pub const SCE_VIDEODEC_ERROR_INVALID_POINTER: SceVideodecErrorCode = 2153908233;
pub const SCE_VIDEODEC_ERROR_INVALID_STATE: SceVideodecErrorCode = 2153908228;
pub const SCE_VIDEODEC_ERROR_INVALID_STREAM: SceVideodecErrorCode = 2153908237;
pub const SCE_VIDEODEC_ERROR_INVALID_TYPE: SceVideodecErrorCode = 2153908225;
pub const SCE_VIDEODEC_ERROR_NOT_INITIALIZE: SceVideodecErrorCode = 2153908236;
pub const SCE_VIDEODEC_ERROR_NOT_PHY_CONTINUOUS_MEMORY: SceVideodecErrorCode = 2153908231;
pub const SCE_VIDEODEC_ERROR_OUT_OF_MEMORY: SceVideodecErrorCode = 2153908227;
pub const SCE_VIDEODEC_ERROR_UNSUPPORT_IMAGE_SIZE: SceVideodecErrorCode = 2153908229;
pub const SCE_VIDEODEC_TYPE_HW_AVCDEC: SceVideodecType = 4097;
pub const USB_CLASS_AUDIO: SceUdcdUsbClass = 1;
pub const USB_CLASS_COMM: SceUdcdUsbClass = 2;
pub const USB_CLASS_DATA: SceUdcdUsbClass = 10;
pub const USB_CLASS_HID: SceUdcdUsbClass = 3;
pub const USB_CLASS_HUB: SceUdcdUsbClass = 9;
pub const USB_CLASS_MASS_STORAGE: SceUdcdUsbClass = 8;
pub const USB_CLASS_PER_INTERFACE: SceUdcdUsbClass = 0;
pub const USB_CLASS_PRINTER: SceUdcdUsbClass = 7;
pub const USB_CLASS_PTP: SceUdcdUsbClass = 6;
pub const USB_CLASS_VENDOR_SPEC: SceUdcdUsbClass = 255;
pub const USB_CLASS_VIDEO: SceUdcdUsbClass = 14;
pub const USB_CTRLTYPE_DIR_DEVICE2HOST: u32 = 128;
pub const USB_CTRLTYPE_DIR_HOST2DEVICE: u32 = 0;
pub const USB_CTRLTYPE_DIR_MASK: u32 = 128;
pub const USB_CTRLTYPE_REC_DEVICE: u32 = 0;
pub const USB_CTRLTYPE_REC_ENDPOINT: u32 = 2;
pub const USB_CTRLTYPE_REC_INTERFACE: u32 = 1;
pub const USB_CTRLTYPE_REC_MASK: u32 = 31;
pub const USB_CTRLTYPE_REC_OTHER: u32 = 3;
pub const USB_CTRLTYPE_TYPE_CLASS: u32 = 32;
pub const USB_CTRLTYPE_TYPE_MASK: u32 = 96;
pub const USB_CTRLTYPE_TYPE_RESERVED: u32 = 96;
pub const USB_CTRLTYPE_TYPE_STANDARD: u32 = 0;
pub const USB_CTRLTYPE_TYPE_VENDOR: u32 = 64;
pub const USBD_CC_BABBLE: u32 = 4;
pub const USBD_CC_DATABUF: u32 = 8;
pub const USBD_CC_MISSED_MICRO_FRAME: u32 = 1;
pub const USBD_CC_NOERR: u32 = 0;
pub const USBD_CC_XACTERR: u32 = 2;
pub const USB_DESCRIPTOR_CONFIGURATION: u32 = 2;
pub const USB_DESCRIPTOR_DEVICE: u32 = 1;
pub const USB_DESCRIPTOR_DEVICE_QUALIFIER: u32 = 6;
pub const USB_DESCRIPTOR_ENDPOINT: u32 = 5;
pub const USB_DESCRIPTOR_INTERFACE: u32 = 4;
pub const USB_DESCRIPTOR_INTERFACE_POWER: u32 = 8;
pub const USB_DESCRIPTOR_OTG: u32 = 9;
pub const USB_DESCRIPTOR_OTHER_SPEED: u32 = 7;
pub const USB_DESCRIPTOR_STRING: u32 = 3;
pub const USB_DT_CONFIG: SceUdcdUsbDt = 2;
pub const USB_DT_CONFIG_SIZE: u32 = 9;
pub const USB_DT_DEVICE: SceUdcdUsbDt = 1;
pub const USB_DT_DEVICE_SIZE: u32 = 18;
pub const USB_DT_ENDPOINT: SceUdcdUsbDt = 5;
pub const USB_DT_ENDPOINT_AUDIO_SIZE: u32 = 9;
pub const USB_DT_ENDPOINT_SIZE: u32 = 7;
pub const USB_DT_HUB_NONVAR_SIZE: u32 = 7;
pub const USB_DT_INTERFACE: SceUdcdUsbDt = 4;
pub const USB_DT_INTERFACE_SIZE: u32 = 9;
pub const USB_DT_STRING: SceUdcdUsbDt = 3;
pub const USB_ENDPOINT_ADDRESS_MASK: u32 = 15;
pub const USB_ENDPOINT_DIR_MASK: u32 = 128;
pub const USB_ENDPOINT_IN: u32 = 128;
pub const USB_ENDPOINT_OUT: u32 = 0;
pub const USB_ENDPOINT_TYPE_BULK: u32 = 2;
pub const USB_ENDPOINT_TYPE_CONTROL: u32 = 0;
pub const USB_ENDPOINT_TYPE_INTERRUPT: u32 = 3;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS: u32 = 1;
pub const USB_ENDPOINT_TYPE_MASK: u32 = 3;
pub const USB_FEATURE_ENDPOINT_HALT: u32 = 0;
pub const USB_REQ_CLEAR_FEATURE: SceUdcdUsbReq = 1;
pub const USB_REQ_GET_CONFIG: SceUdcdUsbReq = 8;
pub const USB_REQ_GET_DESCRIPTOR: SceUdcdUsbReq = 6;
pub const USB_REQ_GET_INTERFACE: SceUdcdUsbReq = 10;
pub const USB_REQ_GET_STATUS: SceUdcdUsbReq = 0;
pub const USB_REQ_SET_ADDRESS: SceUdcdUsbReq = 5;
pub const USB_REQ_SET_CONFIG: SceUdcdUsbReq = 9;
pub const USB_REQ_SET_DESCRIPTOR: SceUdcdUsbReq = 7;
pub const USB_REQ_SET_FEATURE: SceUdcdUsbReq = 3;
pub const USB_REQ_SET_INTERFACE: SceUdcdUsbReq = 11;
pub const USB_REQ_SYNC_FRAME: SceUdcdUsbReq = 12;
#[link(name = "libScePiglet_stub", kind = "static")]
#[cfg(feature = "libScePiglet_stub")]
extern "C" {}
#[link(name = "SceAppMgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceAppMgrForDriver_stub")]
extern "C" {
    pub fn ksceAppMgrKillProcess(pid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceAppMgrLaunchAppByPath(
        path: *const crate::ctypes::c_char,
        args: *const crate::ctypes::c_char,
        arg_size: SceSize,
        type_: crate::ctypes::c_uint,
        param: *const SceAppMgrLaunchParam,
        unk: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceAppMgr_stub", kind = "static")]
#[cfg(feature = "SceAppMgr_stub")]
extern "C" {
    pub fn _sceAppMgrGetAppState(
        appState: *mut SceAppMgrAppState,
        len: SceSize,
        version: u32,
    ) -> crate::ctypes::c_int;
    pub fn _sceSharedFbOpen(index: crate::ctypes::c_int, sysver: crate::ctypes::c_int) -> SceUID;
    pub fn sceAppMgrAcquireBgmPort() -> crate::ctypes::c_int;
    pub fn sceAppMgrAppDataMount(
        id: crate::ctypes::c_int,
        mount_point: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrAppDataMountById(
        id: crate::ctypes::c_int,
        titleid: *const crate::ctypes::c_char,
        mount_point: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrAppParamGetString(
        pid: crate::ctypes::c_int,
        param: crate::ctypes::c_int,
        string: *mut crate::ctypes::c_char,
        length: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrConvertVs0UserDrivePath(
        path: *mut crate::ctypes::c_char,
        mount_point: *mut crate::ctypes::c_char,
        unk: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrDestroyAppByAppId(appId: SceInt32) -> crate::ctypes::c_int;
    pub fn sceAppMgrDestroyAppByName(name: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceAppMgrDestroyOtherApp() -> crate::ctypes::c_int;
    pub fn sceAppMgrDrmOpen(param: *const SceAppMgrDrmOpenParam) -> SceInt32;
    pub fn sceAppMgrGameDataMount(
        app_path: *const crate::ctypes::c_char,
        patch_path: *const crate::ctypes::c_char,
        rif_path: *const crate::ctypes::c_char,
        mount_point: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrGetAppInfo(
        unk: *const crate::ctypes::c_char,
        state: *mut SceAppMgrAppState,
    ) -> SceInt32;
    pub fn sceAppMgrGetAppParam(param: *mut crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceAppMgrGetBudgetInfo(info: *mut SceAppMgrBudgetInfo) -> crate::ctypes::c_int;
    pub fn sceAppMgrGetCoredumpStateForShell(
        state: *mut SceAppMgrCoredumpState,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrGetDevInfo(
        dev: *const crate::ctypes::c_char,
        max_size: *mut u64,
        free_size: *mut u64,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrGetIdByName(
        pid: *mut SceUID,
        name: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrGetNameById(
        pid: SceUID,
        name: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrGetProcessIdByAppIdForShell(appId: SceInt32) -> SceUID;
    pub fn sceAppMgrGetRawPath(
        path: *mut crate::ctypes::c_char,
        resolved_path: *mut crate::ctypes::c_char,
        resolved_path_size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrGetRawPathOfApp0ByAppIdForShell(
        appId: crate::ctypes::c_int,
        resolved_path: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrGetRunningAppIdListForShell(
        appIds: *mut SceInt32,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrLaunchAppByName(
        flags: crate::ctypes::c_int,
        name: *const crate::ctypes::c_char,
        param: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrLaunchAppByName2(
        name: *const crate::ctypes::c_char,
        param: *const crate::ctypes::c_char,
        optParam: *mut SceAppMgrLaunchAppOptParam,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrLaunchAppByName2ForShell(
        name: *const crate::ctypes::c_char,
        param: *const crate::ctypes::c_char,
        optParam: *mut SceAppMgrLaunchAppOptParam,
    ) -> SceUID;
    pub fn sceAppMgrLaunchAppByUri(
        flags: crate::ctypes::c_int,
        uri: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrLoadExec(
        appPath: *const crate::ctypes::c_char,
        argv: *const *mut crate::ctypes::c_char,
        optParam: *const SceAppMgrExecOptParam,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrMmsMount(
        id: crate::ctypes::c_int,
        mount_point: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrPspSaveDataRootMount(
        mount_point: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrReceiveSystemEvent(
        systemEvent: *mut SceAppMgrSystemEvent,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrReleaseBgmPort() -> crate::ctypes::c_int;
    pub fn sceAppMgrSaveDataDataRemove(
        data: *mut SceAppMgrSaveDataDataDelete,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrSaveDataDataSave(data: *mut SceAppMgrSaveDataData) -> crate::ctypes::c_int;
    pub fn sceAppMgrSaveDataSlotCreate(data: *mut SceAppMgrSaveDataSlot) -> crate::ctypes::c_int;
    pub fn sceAppMgrSaveDataSlotDelete(
        data: *mut SceAppMgrSaveDataSlotDelete,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrSaveDataSlotGetParam(data: *mut SceAppMgrSaveDataSlot) -> crate::ctypes::c_int;
    pub fn sceAppMgrSaveDataSlotSetParam(data: *mut SceAppMgrSaveDataSlot) -> crate::ctypes::c_int;
    pub fn sceAppMgrSetInfobarState(
        visibility: SceAppMgrInfoBarVisibility,
        color: SceAppMgrInfoBarColor,
        transparency: SceAppMgrInfoBarTransparency,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrUmount(mount_point: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceAppMgrWorkDirMount(
        id: crate::ctypes::c_int,
        mount_point: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppMgrWorkDirMountById(
        id: crate::ctypes::c_int,
        titleid: *const crate::ctypes::c_char,
        mount_point: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceSharedFbBegin(fb_id: SceUID, info: *mut SceSharedFbInfo) -> crate::ctypes::c_int;
    pub fn sceSharedFbClose(fb_id: SceUID) -> crate::ctypes::c_int;
    pub fn sceSharedFbEnd(fb_id: SceUID) -> crate::ctypes::c_int;
    pub fn sceSharedFbGetInfo(fb_id: SceUID, info: *mut SceSharedFbInfo) -> crate::ctypes::c_int;
}
#[link(name = "SceAppUtil_stub", kind = "static")]
#[cfg(feature = "SceAppUtil_stub")]
extern "C" {
    pub fn sceAppUtilAppEventParseLiveArea(
        eventParam: *const SceAppUtilAppEventParam,
        buffer: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilAppParamGetInt(
        paramId: SceAppUtilAppParamId,
        value: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilBgdlGetStatus(stat: *mut SceAppUtilBgdlStatus) -> crate::ctypes::c_int;
    pub fn sceAppUtilCacheMount() -> crate::ctypes::c_int;
    pub fn sceAppUtilCacheUmount() -> crate::ctypes::c_int;
    pub fn sceAppUtilInit(
        initParam: *mut SceAppUtilInitParam,
        bootParam: *mut SceAppUtilBootParam,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilLaunchWebBrowser(
        param: *mut SceAppUtilWebBrowserParam,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilLoadSafeMemory(
        buf: *mut crate::ctypes::c_void,
        bufSize: SceSize,
        offset: SceOff,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilMusicMount() -> crate::ctypes::c_int;
    pub fn sceAppUtilMusicUmount() -> crate::ctypes::c_int;
    pub fn sceAppUtilPhotoMount() -> crate::ctypes::c_int;
    pub fn sceAppUtilPhotoUmount() -> crate::ctypes::c_int;
    pub fn sceAppUtilReceiveAppEvent(
        eventParam: *mut SceAppUtilAppEventParam,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilSaveDataDataRemove(
        slot: *mut SceAppUtilSaveDataFileSlot,
        files: *mut SceAppUtilSaveDataRemoveItem,
        fileNum: crate::ctypes::c_uint,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilSaveDataDataSave(
        slot: *mut SceAppUtilSaveDataFileSlot,
        files: *mut SceAppUtilSaveDataFile,
        fileNum: crate::ctypes::c_uint,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
        requiredSizeKB: *mut SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilSaveDataSlotCreate(
        slotId: crate::ctypes::c_uint,
        param: *mut SceAppUtilSaveDataSlotParam,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilSaveDataSlotDelete(
        slotId: crate::ctypes::c_uint,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilSaveDataSlotGetParam(
        slotId: crate::ctypes::c_uint,
        param: *mut SceAppUtilSaveDataSlotParam,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilSaveDataSlotSetParam(
        slotId: crate::ctypes::c_uint,
        param: *mut SceAppUtilSaveDataSlotParam,
        mountPoint: *mut SceAppUtilSaveDataMountPoint,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilSaveSafeMemory(
        buf: *mut crate::ctypes::c_void,
        bufSize: SceSize,
        offset: SceOff,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilShutdown() -> crate::ctypes::c_int;
    pub fn sceAppUtilStoreBrowse(param: *mut SceAppUtilStoreBrowseParam) -> crate::ctypes::c_int;
    pub fn sceAppUtilSystemParamGetInt(
        paramId: crate::ctypes::c_uint,
        value: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAppUtilSystemParamGetString(
        paramId: crate::ctypes::c_uint,
        buf: *mut SceChar8,
        bufSize: SceSize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceAtrac_stub", kind = "static")]
#[cfg(feature = "SceAtrac_stub")]
extern "C" {
    pub fn sceAtracAddStreamData(
        atracHandle: crate::ctypes::c_int,
        addSize: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracCreateDecoderGroup(
        atracType: SceUInt32,
        pDecoderGroup: *const SceAtracDecoderGroup,
        pvWorkMem: *mut crate::ctypes::c_void,
        initAudiodecFlag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracDecode(
        atracHandle: crate::ctypes::c_int,
        pOutputBuffer: *mut crate::ctypes::c_void,
        pOutputSamples: *mut SceUInt32,
        pDecoderStatus: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracDeleteDecoderGroup(
        atracType: SceUInt32,
        termAudiodecFlag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetContentInfo(
        atracHandle: crate::ctypes::c_int,
        pContentInfo: *mut SceAtracContentInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetDecoderGroupInfo(
        atracType: SceUInt32,
        pCreatedDecoder: *mut SceAtracDecoderGroup,
        pAvailableDecoder: *mut SceAtracDecoderGroup,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetDecoderStatus(
        atracHandle: crate::ctypes::c_int,
        pDecoderStatus: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetInternalError(
        atracHandle: crate::ctypes::c_int,
        pInternalError: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetLoopInfo(
        atracHandle: crate::ctypes::c_int,
        pLoopNum: *mut crate::ctypes::c_int,
        pLoopStatus: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetNextOutputPosition(
        atracHandle: crate::ctypes::c_int,
        pNextOutputSample: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetOutputSamples(
        atracHandle: crate::ctypes::c_int,
        pOutputSamples: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetOutputableSamples(
        atracHandle: crate::ctypes::c_int,
        pOutputableSamples: *mut SceLong64,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetRemainSamples(
        atracHandle: crate::ctypes::c_int,
        pRemainSamples: *mut SceLong64,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetStreamInfo(
        atracHandle: crate::ctypes::c_int,
        pStreamInfo: *mut SceAtracStreamInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetSubBufferInfo(
        atracHandle: crate::ctypes::c_int,
        pReadPosition: *mut SceUInt32,
        pMinSubBufferSize: *mut SceUInt32,
        pDataSize: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracGetVacantSize(
        atracHandle: crate::ctypes::c_int,
        pVacantSize: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracIsSubBufferNeeded(atracHandle: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceAtracQueryDecoderGroupMemSize(
        atracType: SceUInt32,
        pDecoderGroup: *const SceAtracDecoderGroup,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracReleaseHandle(atracHandle: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceAtracResetNextOutputPosition(
        atracHandle: crate::ctypes::c_int,
        resetSample: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracSetDataAndAcquireHandle(
        pucBuffer: *mut SceUChar8,
        uiReadSize: SceUInt32,
        uiBufferSize: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracSetLoopNum(
        atracHandle: crate::ctypes::c_int,
        loopNum: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracSetOutputSamples(
        atracHandle: crate::ctypes::c_int,
        outputSamples: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceAtracSetSubBuffer(
        atracHandle: crate::ctypes::c_int,
        pSubBuffer: *mut SceUChar8,
        subBufferSize: SceUInt32,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceAudiodec_stub", kind = "static")]
#[cfg(feature = "SceAudiodec_stub")]
extern "C" {
    pub fn sceAudiodecClearContext(pCtrl: *mut SceAudiodecCtrl) -> SceInt32;
    pub fn sceAudiodecCreateDecoder(pCtrl: *mut SceAudiodecCtrl, codecType: SceUInt32) -> SceInt32;
    pub fn sceAudiodecCreateDecoderExternal(
        pCtrl: *mut SceAudiodecCtrl,
        codecType: SceUInt32,
        vaContext: SceUIntVAddr,
        contextSize: SceUInt32,
    ) -> SceInt32;
    pub fn sceAudiodecDecode(pCtrl: *mut SceAudiodecCtrl) -> SceInt32;
    pub fn sceAudiodecDecodeNFrames(pCtrl: *mut SceAudiodecCtrl, nFrames: SceUInt32) -> SceInt32;
    pub fn sceAudiodecDecodeNStreams(
        pCtrls: *mut *mut SceAudiodecCtrl,
        nStreams: SceUInt32,
    ) -> SceInt32;
    pub fn sceAudiodecDeleteDecoder(pCtrl: *mut SceAudiodecCtrl) -> SceInt32;
    pub fn sceAudiodecDeleteDecoderExternal(
        pCtrl: *mut SceAudiodecCtrl,
        pvaContext: *mut SceUIntVAddr,
    ) -> SceInt32;
    pub fn sceAudiodecGetContextSize(pCtrl: *mut SceAudiodecCtrl, codecType: SceUInt32)
        -> SceInt32;
    pub fn sceAudiodecGetInternalError(
        pCtrl: *mut SceAudiodecCtrl,
        pInternalError: *mut SceInt32,
    ) -> SceInt32;
    pub fn sceAudiodecInitLibrary(
        codecType: SceUInt32,
        pInitParam: *mut SceAudiodecInitParam,
    ) -> SceInt32;
    pub fn sceAudiodecTermLibrary(codecType: SceUInt32) -> SceInt32;
}
#[link(name = "SceAudioenc_stub", kind = "static")]
#[cfg(feature = "SceAudioenc_stub")]
extern "C" {
    pub fn sceAudioencClearContext(pCtrl: *mut SceAudioencCtrl) -> crate::ctypes::c_int;
    pub fn sceAudioencCreateEncoder(
        pCtrl: *mut SceAudioencCtrl,
        codecType: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAudioencDeleteEncoder(pCtrl: *mut SceAudioencCtrl) -> crate::ctypes::c_int;
    pub fn sceAudioencEncode(pCtrl: *mut SceAudioencCtrl) -> crate::ctypes::c_int;
    pub fn sceAudioencGetInternalError(
        pCtrl: *mut SceAudioencCtrl,
        pInternalError: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAudioencGetOptInfo(pCtrl: *mut SceAudioencCtrl) -> crate::ctypes::c_int;
    pub fn sceAudioencInitLibrary(
        codecType: crate::ctypes::c_int,
        pInitParam: *mut SceAudioencInitParam,
    ) -> crate::ctypes::c_int;
    pub fn sceAudioencTermLibrary(codecType: crate::ctypes::c_uint) -> crate::ctypes::c_int;
}
#[link(name = "SceAudioIn_stub", kind = "static")]
#[cfg(feature = "SceAudioIn_stub")]
extern "C" {
    pub fn sceAudioInGetAdopt(portType: SceAudioInPortType) -> crate::ctypes::c_int;
    pub fn sceAudioInGetStatus(select: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceAudioInInput(
        port: crate::ctypes::c_int,
        destPtr: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceAudioInOpenPort(
        portType: SceAudioInPortType,
        grain: crate::ctypes::c_int,
        freq: crate::ctypes::c_int,
        param: SceAudioInParam,
    ) -> crate::ctypes::c_int;
    pub fn sceAudioInReleasePort(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
#[link(name = "SceAudio_stub", kind = "static")]
#[cfg(feature = "SceAudio_stub")]
extern "C" {
    pub fn sceAudioOutGetAdopt(type_: SceAudioOutPortType) -> crate::ctypes::c_int;
    pub fn sceAudioOutGetConfig(
        port: crate::ctypes::c_int,
        type_: SceAudioOutConfigType,
    ) -> crate::ctypes::c_int;
    pub fn sceAudioOutGetRestSample(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceAudioOutOpenPort(
        type_: SceAudioOutPortType,
        len: crate::ctypes::c_int,
        freq: crate::ctypes::c_int,
        mode: SceAudioOutMode,
    ) -> crate::ctypes::c_int;
    pub fn sceAudioOutOutput(
        port: crate::ctypes::c_int,
        buf: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceAudioOutReleasePort(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceAudioOutSetAlcMode(mode: SceAudioOutAlcMode) -> crate::ctypes::c_int;
    pub fn sceAudioOutSetConfig(
        port: crate::ctypes::c_int,
        len: SceSize,
        freq: crate::ctypes::c_int,
        mode: SceAudioOutMode,
    ) -> crate::ctypes::c_int;
    pub fn sceAudioOutSetVolume(
        port: crate::ctypes::c_int,
        ch: SceAudioOutChannelFlag,
        vol: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceAvcodecForDriver_stub", kind = "static")]
#[cfg(feature = "SceAvcodecForDriver_stub")]
extern "C" {
    pub fn ksceJpegEncoderCsc(
        context: SceJpegEncoderContext,
        outBuffer: *mut crate::ctypes::c_void,
        inBuffer: *const crate::ctypes::c_void,
        inPitch: crate::ctypes::c_int,
        inPixelFormat: SceJpegEncoderPixelFormat,
    ) -> crate::ctypes::c_int;
    pub fn ksceJpegEncoderEncode(
        context: SceJpegEncoderContext,
        inBuffer: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceJpegEncoderEnd(context: SceJpegEncoderContext) -> crate::ctypes::c_int;
    pub fn ksceJpegEncoderGetContextSize() -> crate::ctypes::c_int;
    pub fn ksceJpegEncoderInit(
        context: SceJpegEncoderContext,
        inWidth: crate::ctypes::c_int,
        inHeight: crate::ctypes::c_int,
        pixelformat: SceJpegEncoderPixelFormat,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceJpegEncoderSetCompressionRatio(
        context: SceJpegEncoderContext,
        ratio: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceJpegEncoderSetHeaderMode(
        context: SceJpegEncoderContext,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceJpegEncoderSetOutputAddr(
        context: SceJpegEncoderContext,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceJpegEncoderSetValidRegion(
        context: SceJpegEncoderContext,
        inWidth: crate::ctypes::c_int,
        inHeight: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceAvcodec_stub", kind = "static")]
#[cfg(feature = "SceAvcodec_stub")]
extern "C" {}
#[link(name = "SceAvcodecUser_stub", kind = "static")]
#[cfg(feature = "SceAvcodecUser_stub")]
extern "C" {}
#[link(name = "SceAVConfig_stub", kind = "static")]
#[cfg(feature = "SceAVConfig_stub")]
extern "C" {
    pub fn sceAVConfigGetDisplayMaxBrightness(
        maxBrightness: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAVConfigGetShutterVol(volume: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceAVConfigGetSystemVol(volume: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceAVConfigMuteOn() -> crate::ctypes::c_int;
    pub fn sceAVConfigSetDisplayBrightness(
        brightness: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAVConfigSetDisplayColorSpaceMode(csm: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceAVConfigSetSystemVol(volume: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
#[link(name = "SceAvPlayer_stub", kind = "static")]
#[cfg(feature = "SceAvPlayer_stub")]
extern "C" {
    pub fn sceAvPlayerAddSource(
        handle: SceAvPlayerHandle,
        filename: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceAvPlayerClose(handle: SceAvPlayerHandle) -> crate::ctypes::c_int;
    pub fn sceAvPlayerCurrentTime(handle: SceAvPlayerHandle) -> u64;
    pub fn sceAvPlayerGetAudioData(
        handle: SceAvPlayerHandle,
        info: *mut SceAvPlayerFrameInfo,
    ) -> SceBool;
    pub fn sceAvPlayerGetStreamInfo(
        handle: SceAvPlayerHandle,
        id: u32,
        info: *mut SceAvPlayerStreamInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceAvPlayerGetVideoData(
        handle: SceAvPlayerHandle,
        info: *mut SceAvPlayerFrameInfo,
    ) -> SceBool;
    pub fn sceAvPlayerInit(data: *mut SceAvPlayerInitData) -> SceAvPlayerHandle;
    pub fn sceAvPlayerIsActive(handle: SceAvPlayerHandle) -> SceBool;
    pub fn sceAvPlayerJumpToTime(handle: SceAvPlayerHandle, offset: u64) -> crate::ctypes::c_int;
    pub fn sceAvPlayerPause(handle: SceAvPlayerHandle) -> crate::ctypes::c_int;
    pub fn sceAvPlayerResume(handle: SceAvPlayerHandle) -> crate::ctypes::c_int;
    pub fn sceAvPlayerSetLooping(
        handle: SceAvPlayerHandle,
        looping: SceBool,
    ) -> crate::ctypes::c_int;
    pub fn sceAvPlayerSetTrickSpeed(
        handle: SceAvPlayerHandle,
        speed: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceAvPlayerStart(handle: SceAvPlayerHandle) -> crate::ctypes::c_int;
    pub fn sceAvPlayerStop(handle: SceAvPlayerHandle) -> crate::ctypes::c_int;
}
#[link(name = "SceBacktraceForDriver_stub", kind = "static")]
#[cfg(feature = "SceBacktraceForDriver_stub")]
extern "C" {}
#[link(name = "SceBbmc_stub", kind = "static")]
#[cfg(feature = "SceBbmc_stub")]
extern "C" {}
#[link(name = "SceBgAppUtil_stub", kind = "static")]
#[cfg(feature = "SceBgAppUtil_stub")]
extern "C" {
    pub fn sceBgAppUtilStartBgApp(mode: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
#[link(name = "SceBtForDriver_stub", kind = "static")]
#[cfg(feature = "SceBtForDriver_stub")]
extern "C" {
    pub fn ksceBtAvrcpReadVolume(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtAvrcpSendButton(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtAvrcpSendVolume(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtAvrcpSetPlayStatus(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtAvrcpSetTitle(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtDeleteRegisteredInfo(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtFreqAudio(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtGetConfiguration() -> crate::ctypes::c_int;
    pub fn ksceBtGetConnectingInfo(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtGetDeviceName(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        name: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtGetInfoForTest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtGetLastError() -> crate::ctypes::c_int;
    pub fn ksceBtGetRegisteredInfo(
        device: crate::ctypes::c_int,
        unk: crate::ctypes::c_int,
        info: *mut SceBtRegisteredInfo,
        info_size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtGetStatusForTest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtGetVidPid(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        vid_pid: *mut crate::ctypes::c_ushort,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtHfpGetCurrentPhoneNumber(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtHfpRequest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtHidGetReportDescriptor(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        buffer: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtHidTransfer(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        request: *mut SceBtHidRequest,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtPushBip(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtPushOpp(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtReadEvent(
        events: *mut SceBtEvent,
        num_events: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtRecvAudio(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtRecvBip(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtRecvOpp(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtRecvSpp(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtRegisterCallback(
        cb: SceUID,
        unused: crate::ctypes::c_int,
        flags1: crate::ctypes::c_int,
        flags2: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtReplyPinCode(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        code: *mut crate::ctypes::c_uchar,
        length: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtReplyUserConfirmation(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
        unk: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtSendAudio(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtSendL2capEchoRequestForTest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtSendSpp(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtSetConfiguration(r0: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceBtSetContentProtection(r0: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceBtSetInquiryResultForTest(arg1: *mut crate::ctypes::c_uchar)
        -> crate::ctypes::c_int;
    pub fn ksceBtSetInquiryScan(r0: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceBtSetL2capEchoResponseBufferForTest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtSetStatusForTest(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtStartAudio(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtStartConnect(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtStartDisconnect(
        mac0: crate::ctypes::c_uint,
        mac1: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtStartInquiry() -> crate::ctypes::c_int;
    pub fn ksceBtStopAudio(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceBtStopInquiry() -> crate::ctypes::c_int;
    pub fn ksceBtUnregisterCallback(cb: SceUID) -> crate::ctypes::c_int;
}
#[link(name = "SceBt_stub", kind = "static")]
#[cfg(feature = "SceBt_stub")]
extern "C" {}
#[link(name = "SceCameraForDriver_stub", kind = "static")]
#[cfg(feature = "SceCameraForDriver_stub")]
extern "C" {}
#[link(name = "SceCamera_stub", kind = "static")]
#[cfg(feature = "SceCamera_stub")]
extern "C" {
    pub fn sceCameraClose(devnum: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCameraGetAntiFlicker(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetAutoControlHold(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetBacklight(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetBrightness(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetContrast(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetDeviceLocation(
        devnum: crate::ctypes::c_int,
        pLocation: *mut SceFVector3,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetEV(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetEffect(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetExposureCeiling(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetGain(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetISO(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetImageQuality(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetNightmode(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetNoiseReduction(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetReverse(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetSaturation(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetSharpness(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetSharpnessOff(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetWhiteBalance(
        devnum: crate::ctypes::c_int,
        pMode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraGetZoom(
        devnum: crate::ctypes::c_int,
        pLevel: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraIsActive(devnum: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCameraOpen(
        devnum: crate::ctypes::c_int,
        pInfo: *mut SceCameraInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraRead(
        devnum: crate::ctypes::c_int,
        pRead: *mut SceCameraRead,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetAntiFlicker(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetAutoControlHold(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetBacklight(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetBrightness(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetContrast(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetEV(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetEffect(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetExposureCeiling(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetGain(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetISO(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetImageQuality(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetNightmode(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetNoiseReduction(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetReverse(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetSaturation(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetSharpness(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetSharpnessOff(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetWhiteBalance(
        devnum: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraSetZoom(
        devnum: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCameraStart(devnum: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCameraStop(devnum: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
#[link(name = "SceClipboard_stub", kind = "static")]
#[cfg(feature = "SceClipboard_stub")]
extern "C" {}
#[link(name = "SceClockgenForDriver_stub", kind = "static")]
#[cfg(feature = "SceClockgenForDriver_stub")]
extern "C" {}
#[link(name = "SceCodecEnginePerf_stub", kind = "static")]
#[cfg(feature = "SceCodecEnginePerf_stub")]
extern "C" {}
#[link(name = "SceCodecEngine_stub", kind = "static")]
#[cfg(feature = "SceCodecEngine_stub")]
extern "C" {}
#[link(name = "SceCodecEngineWrapper_stub", kind = "static")]
#[cfg(feature = "SceCodecEngineWrapper_stub")]
extern "C" {}
#[link(name = "SceCommonDialog_stub", kind = "static")]
#[cfg(feature = "SceCommonDialog_stub")]
extern "C" {
    pub fn sceCommonDialogSetConfigParam(
        configParam: *const SceCommonDialogConfigParam,
    ) -> crate::ctypes::c_int;
    pub fn sceCommonDialogUpdate(
        updateParam: *const SceCommonDialogUpdateParam,
    ) -> crate::ctypes::c_int;
    pub fn sceImeDialogAbort() -> SceInt32;
    pub fn sceImeDialogGetResult(result: *mut SceImeDialogResult) -> SceInt32;
    pub fn sceImeDialogGetStatus() -> SceCommonDialogStatus;
    pub fn sceImeDialogInit(param: *const SceImeDialogParam) -> SceInt32;
    pub fn sceImeDialogTerm() -> SceInt32;
    pub fn sceMsgDialogAbort() -> crate::ctypes::c_int;
    pub fn sceMsgDialogClose() -> crate::ctypes::c_int;
    pub fn sceMsgDialogGetResult(result: *mut SceMsgDialogResult) -> crate::ctypes::c_int;
    pub fn sceMsgDialogGetStatus() -> SceCommonDialogStatus;
    pub fn sceMsgDialogInit(param: *const SceMsgDialogParam) -> crate::ctypes::c_int;
    pub fn sceMsgDialogProgressBarInc(
        target: SceMsgDialogProgressBarTarget,
        delta: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceMsgDialogProgressBarSetMsg(
        target: SceMsgDialogProgressBarTarget,
        barMsg: *const SceChar8,
    ) -> crate::ctypes::c_int;
    pub fn sceMsgDialogProgressBarSetValue(
        target: SceMsgDialogProgressBarTarget,
        rate: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceMsgDialogTerm() -> crate::ctypes::c_int;
    pub fn sceNetCheckDialogAbort() -> SceInt32;
    pub fn sceNetCheckDialogGetPS3ConnectInfo(
        info: *mut SceNetCheckDialogPS3ConnectInfo,
    ) -> SceInt32;
    pub fn sceNetCheckDialogGetResult(result: *mut SceNetCheckDialogResult) -> SceInt32;
    pub fn sceNetCheckDialogGetStatus() -> SceCommonDialogStatus;
    pub fn sceNetCheckDialogInit(param: *mut SceNetCheckDialogParam) -> SceInt32;
    pub fn sceNetCheckDialogTerm() -> SceInt32;
}
#[link(name = "SceCompat_stub", kind = "static")]
#[cfg(feature = "SceCompat_stub")]
extern "C" {
    pub fn sceCompatAllocCdramWithHole(cdram: *mut SceCompatCdram) -> crate::ctypes::c_int;
    pub fn sceCompatAvailableColorSpaceSetting() -> crate::ctypes::c_int;
    pub fn sceCompatCache(
        mode: crate::ctypes::c_int,
        addr: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceCompatCheckPocketStation() -> crate::ctypes::c_int;
    pub fn sceCompatFrameBufferInit(
        framebuffer: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceCompatGetCurrentSecureTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceCompatGetDevInf(info: *mut SceIoDevInfo) -> crate::ctypes::c_int;
    pub fn sceCompatGetPeripheralState(mode: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCompatGetPrimaryHead() -> crate::ctypes::c_int;
    pub fn sceCompatGetPspSystemSoftwareVersion() -> crate::ctypes::c_int;
    pub fn sceCompatGetStatus() -> crate::ctypes::c_int;
    pub fn sceCompatGetTitleList(
        buf: *mut crate::ctypes::c_void,
        length: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceCompatGetUpdateState() -> crate::ctypes::c_int;
    pub fn sceCompatInitEx(unk: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCompatInterrupt(intr_code: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCompatIsPocketStationTitle() -> crate::ctypes::c_int;
    pub fn sceCompatLCDCSync() -> crate::ctypes::c_int;
    pub fn sceCompatReadShared32(
        location: crate::ctypes::c_int,
        value: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCompatSetDisplayConfig(
        unk0: crate::ctypes::c_int,
        unk1: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCompatSetRif(rif: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceCompatSetSuspendSema(semaid1: SceUID, semaid2: SceUID) -> crate::ctypes::c_int;
    pub fn sceCompatSetUpdateState(state: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCompatStart() -> crate::ctypes::c_int;
    pub fn sceCompatStop() -> crate::ctypes::c_int;
    pub fn sceCompatSuspendResume(unk: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCompatUninit() -> crate::ctypes::c_int;
    pub fn sceCompatWaitAndGetRequest(
        mode: crate::ctypes::c_int,
        id: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCompatWaitSpecialRequest(unk: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCompatWriteShared32(
        location: crate::ctypes::c_int,
        value: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCompatWriteSharedCtrl(pad_data: *mut SceCtrlDataPsp) -> crate::ctypes::c_int;
}
#[link(name = "SceCoredumpForDriver_stub", kind = "static")]
#[cfg(feature = "SceCoredumpForDriver_stub")]
extern "C" {}
#[link(name = "SceCoredump_stub", kind = "static")]
#[cfg(feature = "SceCoredump_stub")]
extern "C" {}
#[link(name = "SceCpuForDriver_stub", kind = "static")]
#[cfg(feature = "SceCpuForDriver_stub")]
extern "C" {
    pub fn ksceKernelCpuDcacheAndL2InvalidateRange(ptr: *const crate::ctypes::c_void, len: SceSize);
    pub fn ksceKernelCpuDcacheAndL2WritebackInvalidateRange(
        ptr: *const crate::ctypes::c_void,
        len: SceSize,
    );
    pub fn ksceKernelCpuDcacheAndL2WritebackRange(ptr: *const crate::ctypes::c_void, len: SceSize);
    pub fn ksceKernelCpuDcacheInvalidateRange(ptr: *const crate::ctypes::c_void, len: SceSize);
    pub fn ksceKernelCpuDcacheWritebackRange(ptr: *const crate::ctypes::c_void, len: SceSize);
    pub fn ksceKernelCpuDisableInterrupts() -> crate::ctypes::c_int;
    pub fn ksceKernelCpuEnableInterrupts(flags: crate::ctypes::c_int);
    pub fn ksceKernelCpuGetCpuId() -> crate::ctypes::c_int;
    pub fn ksceKernelRWSpinlockLowReadLock(lock: *mut SceKernelRWSpinlock);
    pub fn ksceKernelRWSpinlockLowReadLockCpuSuspendIntr(
        lock: *mut SceKernelRWSpinlock,
    ) -> SceKernelIntrStatus;
    pub fn ksceKernelRWSpinlockLowReadUnlock(lock: *mut SceKernelRWSpinlock);
    pub fn ksceKernelRWSpinlockLowReadUnlockCpuResumeIntr(
        lock: *mut SceKernelRWSpinlock,
        intr_status: SceKernelIntrStatus,
    );
    pub fn ksceKernelRWSpinlockLowTryReadLock(
        lock: *mut SceKernelRWSpinlock,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelRWSpinlockLowTryReadLockCpuSuspendIntr(
        lock: *mut SceKernelRWSpinlock,
    ) -> SceKernelIntrStatus;
    pub fn ksceKernelRWSpinlockLowTryWriteLock(
        lock: *mut SceKernelRWSpinlock,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelRWSpinlockLowTryWriteLockCpuSuspendIntr(
        lock: *mut SceKernelRWSpinlock,
    ) -> SceKernelIntrStatus;
    pub fn ksceKernelRWSpinlockLowWriteLock(lock: *mut SceKernelRWSpinlock);
    pub fn ksceKernelRWSpinlockLowWriteLockCpuSuspendIntr(
        lock: *mut SceKernelRWSpinlock,
    ) -> SceKernelIntrStatus;
    pub fn ksceKernelRWSpinlockLowWriteUnlock(lock: *mut SceKernelRWSpinlock);
    pub fn ksceKernelRWSpinlockLowWriteUnlockCpuResumeIntr(
        lock: *mut SceKernelRWSpinlock,
        intr_status: SceKernelIntrStatus,
    );
    pub fn ksceKernelSpinlockLowLock(lock: *mut SceKernelSpinlock);
    pub fn ksceKernelSpinlockLowLockCpuSuspendIntr(
        lock: *mut SceKernelSpinlock,
    ) -> SceKernelIntrStatus;
    pub fn ksceKernelSpinlockLowTryLock(lock: *mut SceKernelSpinlock) -> crate::ctypes::c_int;
    pub fn ksceKernelSpinlockLowTryLockCpuSuspendIntr(
        lock: *mut SceKernelSpinlock,
    ) -> SceKernelIntrStatus;
    pub fn ksceKernelSpinlockLowUnlock(lock: *mut SceKernelSpinlock);
    pub fn ksceKernelSpinlockLowUnlockCpuResumeIntr(
        lock: *mut SceKernelSpinlock,
        intr_status: SceKernelIntrStatus,
    );
}
#[link(name = "SceCpuForKernel_363_stub", kind = "static")]
#[cfg(feature = "SceCpuForKernel_363_stub")]
extern "C" {}
#[cfg(any(feature = "SceCpuForKernel_363_stub", feature = "SceCpuForKernel_stub"))]
extern "C" {
    pub fn ksceKernelCorelockInitialize(ctx: *mut SceCorelockContext);
    pub fn ksceKernelCorelockLock(ctx: *mut SceCorelockContext, core: SceUInt32);
    pub fn ksceKernelCorelockUnlock(ctx: *mut SceCorelockContext);
    pub fn ksceKernelCpuDcacheInvalidateAll();
    pub fn ksceKernelCpuDcacheWritebackAll();
    pub fn ksceKernelCpuDcacheWritebackInvalidateAll();
    pub fn ksceKernelCpuDcacheWritebackInvalidateRange(
        ptr: *const crate::ctypes::c_void,
        len: SceSize,
    );
    pub fn ksceKernelCpuIcacheAndL2WritebackInvalidateRange(
        ptr: *const crate::ctypes::c_void,
        len: SceSize,
    );
    pub fn ksceKernelCpuIcacheInvalidateAll() -> crate::ctypes::c_int;
    pub fn ksceKernelCpuIcacheInvalidateRange(ptr: *const crate::ctypes::c_void, len: SceSize);
}
#[link(name = "SceCpuForKernel_stub", kind = "static")]
#[cfg(feature = "SceCpuForKernel_stub")]
extern "C" {}
#[link(name = "SceCtrlForDriver_stub", kind = "static")]
#[cfg(feature = "SceCtrlForDriver_stub")]
extern "C" {
    pub fn ksceCtrlClearRapidFire(
        port: crate::ctypes::c_int,
        idx: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceCtrlGetButtonIntercept(intercept: *mut crate::ctypes::c_int)
        -> crate::ctypes::c_int;
    pub fn ksceCtrlGetControllerPortInfo(info: *mut SceCtrlPortInfo) -> crate::ctypes::c_int;
    pub fn ksceCtrlGetMaskForAll(mask: *mut u32) -> crate::ctypes::c_int;
    pub fn ksceCtrlGetMaskForNonShell(mask: *mut u32) -> crate::ctypes::c_int;
    pub fn ksceCtrlGetSamplingMode(pMode: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceCtrlPeekBufferNegative(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceCtrlPeekBufferPositive(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceCtrlReadBufferNegative(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceCtrlReadBufferPositive(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceCtrlRegisterVirtualControllerDriver(
        driver: *mut SceCtrlVirtualControllerDriver,
    ) -> crate::ctypes::c_int;
    pub fn ksceCtrlSetAnalogEmulation(
        port: crate::ctypes::c_uint,
        slot: crate::ctypes::c_uchar,
        user_lX: crate::ctypes::c_uchar,
        user_lY: crate::ctypes::c_uchar,
        user_rX: crate::ctypes::c_uchar,
        user_rY: crate::ctypes::c_uchar,
        kernel_lX: crate::ctypes::c_uchar,
        kernel_lY: crate::ctypes::c_uchar,
        kernel_rX: crate::ctypes::c_uchar,
        kernel_rY: crate::ctypes::c_uchar,
        uiMake: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceCtrlSetButtonEmulation(
        port: crate::ctypes::c_uint,
        slot: crate::ctypes::c_uchar,
        userButtons: crate::ctypes::c_uint,
        kernelButtons: crate::ctypes::c_uint,
        uiMake: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceCtrlSetButtonIntercept(intercept: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceCtrlSetRapidFire(
        port: crate::ctypes::c_int,
        idx: crate::ctypes::c_int,
        pRule: *const SceCtrlRapidFireRule,
    ) -> crate::ctypes::c_int;
    pub fn ksceCtrlSetSamplingMode(mode: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceCtrlUpdateMaskForAll(
        clear_mask: crate::ctypes::c_int,
        set_mask: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceCtrlUpdateMaskForNonShell(
        clear_mask: crate::ctypes::c_int,
        set_mask: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceCtrl_stub", kind = "static")]
#[cfg(feature = "SceCtrl_stub")]
extern "C" {
    pub fn sceCtrlClearRapidFire(
        port: crate::ctypes::c_int,
        idx: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlGetBatteryInfo(
        port: crate::ctypes::c_int,
        batt: *mut SceUInt8,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlGetButtonIntercept(intercept: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCtrlGetControllerPortInfo(info: *mut SceCtrlPortInfo) -> crate::ctypes::c_int;
    pub fn sceCtrlGetSamplingMode(pMode: *mut SceCtrlPadInputMode) -> crate::ctypes::c_int;
    pub fn sceCtrlIsMultiControllerSupported() -> crate::ctypes::c_int;
    pub fn sceCtrlPeekBufferNegative(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlPeekBufferNegative2(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlPeekBufferPositive(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlPeekBufferPositive2(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlPeekBufferPositiveExt(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlPeekBufferPositiveExt2(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlReadBufferNegative(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlReadBufferNegative2(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlReadBufferPositive(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlReadBufferPositive2(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlReadBufferPositiveExt(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlReadBufferPositiveExt2(
        port: crate::ctypes::c_int,
        pad_data: *mut SceCtrlData,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlSetActuator(
        port: crate::ctypes::c_int,
        pState: *const SceCtrlActuator,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlSetButtonIntercept(intercept: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceCtrlSetLightBar(
        port: crate::ctypes::c_int,
        r: SceUInt8,
        g: SceUInt8,
        b: SceUInt8,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlSetRapidFire(
        port: crate::ctypes::c_int,
        idx: crate::ctypes::c_int,
        pRule: *const SceCtrlRapidFireRule,
    ) -> crate::ctypes::c_int;
    pub fn sceCtrlSetSamplingMode(mode: SceCtrlPadInputMode) -> crate::ctypes::c_int;
    pub fn sceCtrlSetSamplingModeExt(mode: SceCtrlPadInputMode) -> crate::ctypes::c_int;
}
#[link(name = "SceDebugForDriver_stub", kind = "static")]
#[cfg(feature = "SceDebugForDriver_stub")]
extern "C" {
    pub fn ksceEventLogGetInfo(
        buf: *mut crate::ctypes::c_void,
        buf_size: SceSize,
        read_blocks: *mut SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelAssert(
        condition: SceBool,
        dbginfo: *const SceKernelDebugInfo,
        lr: *const crate::ctypes::c_void,
    );
    pub fn ksceKernelAssertLevel(
        level: SceUInt32,
        condition: SceBool,
        dbginfo: *const SceKernelDebugInfo,
        lr: *const crate::ctypes::c_void,
    );
    pub fn ksceKernelGetAssertLevel() -> crate::ctypes::c_int;
    pub fn ksceKernelGetTtyInfo(
        buf: *mut crate::ctypes::c_char,
        buf_size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelPanic(
        dbginfo: *const SceKernelDebugInfo,
        lr: *const crate::ctypes::c_void,
    ) -> !;
    pub fn ksceKernelPrintf(fmt: *const crate::ctypes::c_char, ...) -> crate::ctypes::c_int;
    pub fn ksceKernelPrintfAssertLevel(
        level: SceUInt32,
        condition: SceBool,
        dbginfo: *const SceKernelDebugInfo,
        lr: *const crate::ctypes::c_void,
        fmt: *const crate::ctypes::c_char,
        ...
    );
    pub fn ksceKernelPrintfLevel(
        level: SceUInt32,
        fmt: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelPrintfLevelWithInfo(
        level: SceUInt32,
        flags: SceUInt32,
        dbginfo: *const SceKernelDebugInfo,
        fmt: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelPrintfPanic(
        dbginfo: *const SceKernelDebugInfo,
        lr: *const crate::ctypes::c_void,
        fmt: *const crate::ctypes::c_char,
        ...
    ) -> !;
    pub fn ksceKernelPrintfWithInfo(
        flags: SceUInt32,
        dbginfo: *const SceKernelDebugInfo,
        fmt: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelVprintf(
        fmt: *const crate::ctypes::c_char,
        arg: va_list,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelVprintfLevel(
        level: SceUInt32,
        fmt: *const crate::ctypes::c_char,
        arg: va_list,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceDebugForKernel_363_stub", kind = "static")]
#[cfg(feature = "SceDebugForKernel_363_stub")]
extern "C" {}
#[cfg(any(
    feature = "SceDebugForKernel_363_stub",
    feature = "SceDebugForKernel_stub"
))]
extern "C" {
    pub fn ksceDebugDisableInfoDump(flag: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceDebugGetPutcharHandler() -> *mut crate::ctypes::c_void;
    pub fn ksceDebugPutchar(character: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceDebugRegisterPutcharHandler(
        func: ::core::option::Option<
            unsafe extern "C" fn(
                args: *mut crate::ctypes::c_void,
                c: crate::ctypes::c_char,
            ) -> crate::ctypes::c_int,
        >,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceDebugSetHandlers(
        func: ::core::option::Option<
            unsafe extern "C" fn(
                unk: crate::ctypes::c_int,
                format: *const crate::ctypes::c_char,
                args: va_list,
            ) -> crate::ctypes::c_int,
        >,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceDebugForKernel_stub", kind = "static")]
#[cfg(feature = "SceDebugForKernel_stub")]
extern "C" {
    pub fn ksceKernelSetMinimumAssertionLevel(level: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
#[link(name = "SceDebugLedForDriver_stub", kind = "static")]
#[cfg(feature = "SceDebugLedForDriver_stub")]
extern "C" {
    pub fn ksceDebugLedInvokeHandle0(
        a1: crate::ctypes::c_int,
        a2: crate::ctypes::c_int,
        a3: crate::ctypes::c_int,
        a4: crate::ctypes::c_int,
    );
    pub fn ksceDebugLedInvokeHandle1(
        a1: crate::ctypes::c_int,
        a2: crate::ctypes::c_int,
        a3: crate::ctypes::c_int,
        a4: crate::ctypes::c_int,
    );
    pub fn ksceDebugLedRegisterHandle0(
        func: ::core::option::Option<
            unsafe extern "C" fn(
                a1: crate::ctypes::c_int,
                a2: crate::ctypes::c_int,
                a3: crate::ctypes::c_int,
                a4: crate::ctypes::c_int,
            ),
        >,
    );
    pub fn ksceDebugLedRegisterHandle1(
        func: ::core::option::Option<
            unsafe extern "C" fn(
                a1: crate::ctypes::c_int,
                a2: crate::ctypes::c_int,
                a3: crate::ctypes::c_int,
                a4: crate::ctypes::c_int,
            ),
        >,
    );
    pub fn ksceKernelGetGPI() -> SceUInt32;
    pub fn ksceKernelGetGPO() -> SceUInt32;
    pub fn ksceKernelSetGPI(gpi: SceUInt32);
    pub fn ksceKernelSetGPO(gpo: SceUInt32);
}
#[link(name = "SceDeci4pDbgpForDriver_stub", kind = "static")]
#[cfg(feature = "SceDeci4pDbgpForDriver_stub")]
extern "C" {}
#[link(name = "SceDeci4pUserp_stub", kind = "static")]
#[cfg(feature = "SceDeci4pUserp_stub")]
extern "C" {
    pub fn sceKernelDeci4pClose(socketid: SceUID) -> SceInt32;
    pub fn sceKernelDeci4pDisableWatchpoint() -> SceInt32;
    pub fn sceKernelDeci4pEnableWatchpoint() -> SceInt32;
    pub fn sceKernelDeci4pIsProcessAttached() -> SceInt32;
    pub fn sceKernelDeci4pOpen(
        protoname: *const crate::ctypes::c_char,
        protonum: SceUInt32,
        bufsize: SceSize,
    ) -> SceUID;
    pub fn sceKernelDeci4pRead(
        socketid: SceUID,
        buffer: *mut crate::ctypes::c_void,
        size: SceSize,
        reserved: SceUInt32,
    ) -> SceInt32;
    pub fn sceKernelDeci4pRegisterCallback(socketid: SceUID, cbid: SceUID) -> SceInt32;
    pub fn sceKernelDeci4pWrite(
        socketid: SceUID,
        buffer: *const crate::ctypes::c_void,
        size: SceSize,
        reserved: SceUInt32,
    ) -> SceInt32;
}
#[link(name = "SceDipswForDriver_stub", kind = "static")]
#[cfg(feature = "SceDipswForDriver_stub")]
extern "C" {
    pub fn ksceKernelCheckDipsw(bit: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn ksceKernelClearDipsw(bit: crate::ctypes::c_uint);
    pub fn ksceKernelGetDipswInfo(idx: crate::ctypes::c_uint) -> SceUInt32;
    pub fn ksceKernelSetDipsw(bit: crate::ctypes::c_uint);
}
#[link(name = "SceDisplayForDriver_stub", kind = "static")]
#[cfg(feature = "SceDisplayForDriver_stub")]
extern "C" {
    pub fn ksceDisplayGetFrameBuf(
        pParam: *mut SceDisplayFrameBuf,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplayGetMaximumFrameBufResolution(
        width: *mut crate::ctypes::c_int,
        height: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplayGetPrimaryHead() -> crate::ctypes::c_int;
    pub fn ksceDisplayGetProcFrameBufInternal(
        pid: SceUID,
        head: crate::ctypes::c_int,
        index: crate::ctypes::c_int,
        info: *mut SceDisplayFrameBufInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplayGetVcountInternal(display: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceDisplayRegisterFrameBufCallback(uid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceDisplayRegisterFrameBufCallbackInternal(
        display: crate::ctypes::c_int,
        uid: SceUID,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplayRegisterVblankStartCallback(uid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceDisplayRegisterVblankStartCallbackInternal(
        display: crate::ctypes::c_int,
        uid: SceUID,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplaySetFrameBuf(
        pParam: *const SceDisplayFrameBuf,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplaySetFrameBufInternal(
        head: crate::ctypes::c_int,
        index: crate::ctypes::c_int,
        pParam: *const SceDisplayFrameBuf,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplaySetInvertColors(
        display: crate::ctypes::c_int,
        enable: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplaySetOwner(
        head: crate::ctypes::c_int,
        index: crate::ctypes::c_int,
        pid: SceUID,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplayUnregisterVblankStartCallback(uid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceDisplayUnregisterVblankStartCallbackInternal(
        display: crate::ctypes::c_int,
        uid: SceUID,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitSetFrameBuf() -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitSetFrameBufCB() -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitSetFrameBufMulti(vcount: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitSetFrameBufMultiCB(vcount: crate::ctypes::c_uint)
        -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitVblankStart() -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitVblankStartCB() -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitVblankStartCBInternal(
        display: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitVblankStartInternal(
        display: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitVblankStartMulti(vcount: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitVblankStartMultiCB(vcount: crate::ctypes::c_uint)
        -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitVblankStartMultiCBInternal(
        display: crate::ctypes::c_int,
        vcount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceDisplayWaitVblankStartMultiInternal(
        display: crate::ctypes::c_int,
        vcount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceDisplay_stub", kind = "static")]
#[cfg(feature = "SceDisplay_stub")]
extern "C" {
    pub fn sceDisplayGetFrameBuf(
        pParam: *mut SceDisplayFrameBuf,
        sync: SceDisplaySetBufSync,
    ) -> crate::ctypes::c_int;
    pub fn sceDisplayGetMaximumFrameBufResolution(
        width: *mut crate::ctypes::c_int,
        height: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceDisplayGetPrimaryHead() -> crate::ctypes::c_int;
    pub fn sceDisplayGetRefreshRate(pFps: *mut f32) -> crate::ctypes::c_int;
    pub fn sceDisplayGetVcount() -> crate::ctypes::c_int;
    pub fn sceDisplayGetVcountInternal(display: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceDisplayRegisterVblankStartCallback(uid: SceUID) -> crate::ctypes::c_int;
    pub fn sceDisplaySetFrameBuf(
        pParam: *const SceDisplayFrameBuf,
        sync: SceDisplaySetBufSync,
    ) -> crate::ctypes::c_int;
    pub fn sceDisplayUnregisterVblankStartCallback(uid: SceUID) -> crate::ctypes::c_int;
    pub fn sceDisplayWaitSetFrameBuf() -> crate::ctypes::c_int;
    pub fn sceDisplayWaitSetFrameBufCB() -> crate::ctypes::c_int;
    pub fn sceDisplayWaitSetFrameBufMulti(vcount: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn sceDisplayWaitSetFrameBufMultiCB(vcount: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn sceDisplayWaitVblankStart() -> crate::ctypes::c_int;
    pub fn sceDisplayWaitVblankStartCB() -> crate::ctypes::c_int;
    pub fn sceDisplayWaitVblankStartMulti(vcount: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn sceDisplayWaitVblankStartMultiCB(vcount: crate::ctypes::c_uint) -> crate::ctypes::c_int;
}
#[link(name = "SceDmacmgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceDmacmgrForDriver_stub")]
extern "C" {
    pub fn ksceDmacMemcpy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceDmacMemset(
        dst: *mut crate::ctypes::c_void,
        c: crate::ctypes::c_int,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelDmaOpAlloc(name: *const crate::ctypes::c_char) -> SceKernelDmaOpId;
    pub fn ksceKernelDmaOpAssign(
        opid: SceKernelDmaOpId,
        dmac: SceKernelDmacId,
        channel: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelDmaOpConcatenate(
        opid: SceKernelDmaOpId,
        pTag: *mut SceKernelDmaOpTag,
        flag: SceKernelDmaOpFlag,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelDmaOpDeQueue(opid: SceKernelDmaOpId) -> crate::ctypes::c_int;
    pub fn ksceKernelDmaOpEnQueue(opid: SceKernelDmaOpId) -> crate::ctypes::c_int;
    pub fn ksceKernelDmaOpFree(opid: SceKernelDmaOpId) -> crate::ctypes::c_int;
    pub fn ksceKernelDmaOpQuit(opid: SceKernelDmaOpId) -> crate::ctypes::c_int;
    pub fn ksceKernelDmaOpSetCallback(
        opid: SceKernelDmaOpId,
        callback: SceKernelDmaOpCallback,
        pUserData: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelDmaOpSetupChain(
        opid: SceKernelDmaOpId,
        pTag: *mut SceKernelDmaOpTag,
        pParam: *mut SceKernelDmaOpChainParam,
        flag: SceKernelDmaOpFlag,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelDmaOpSetupDirect(
        opid: SceKernelDmaOpId,
        pParam: *mut SceKernelDmaOpDirectParam,
        flag: SceKernelDmaOpFlag,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelDmaOpSync(
        opid: SceKernelDmaOpId,
        syncMode: SceKernelDmaOpSyncMode,
        pTimeout: *mut SceUInt32,
        ppErrorTag: *mut *mut SceKernelDmaOpTag,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceDriverUser_stub", kind = "static")]
#[cfg(feature = "SceDriverUser_stub")]
extern "C" {}
#[link(name = "SceDsiForDriver_stub", kind = "static")]
#[cfg(feature = "SceDsiForDriver_stub")]
extern "C" {
    pub fn ksceDsiDcsRead(
        head: crate::ctypes::c_int,
        param: crate::ctypes::c_ushort,
        buff: *mut crate::ctypes::c_void,
        size: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceDsiDcsShortWrite(
        head: crate::ctypes::c_int,
        param0: crate::ctypes::c_ushort,
        param1: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDsiDisableHead(head: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceDsiEnableHead(head: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceDsiGenericReadRequest(
        head: crate::ctypes::c_int,
        param: crate::ctypes::c_int,
        buff: *mut crate::ctypes::c_void,
        size: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceDsiGenericShortWrite(
        head: crate::ctypes::c_int,
        param0: crate::ctypes::c_int,
        param1: crate::ctypes::c_int,
        param2: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDsiGetPixelClock(head: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceDsiGetVicResolution(
        vic: crate::ctypes::c_int,
        width: *mut crate::ctypes::c_int,
        height: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDsiSendBlankingPacket(head: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceDsiSetLanesAndPixelSize(
        head: crate::ctypes::c_int,
        lanes: crate::ctypes::c_int,
        pixelsize: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceDsiSetVic(
        head: crate::ctypes::c_int,
        vic: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceDTrace_stub", kind = "static")]
#[cfg(feature = "SceDTrace_stub")]
extern "C" {}
#[link(name = "SceError_stub", kind = "static")]
#[cfg(feature = "SceError_stub")]
extern "C" {}
#[link(name = "SceExcpmgrForKernel_363_stub", kind = "static")]
#[cfg(feature = "SceExcpmgrForKernel_363_stub")]
extern "C" {}
#[cfg(any(
    feature = "SceExcpmgrForKernel_363_stub",
    feature = "SceExcpmgrForKernel_stub"
))]
extern "C" {
    pub fn ksceExcpmgrGetData() -> *mut SceExcpmgrData;
    pub fn ksceExcpmgrRegisterHandler(
        kind: SceExcpKind,
        priority: crate::ctypes::c_int,
        handler: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceExcpmgrForKernel_stub", kind = "static")]
#[cfg(feature = "SceExcpmgrForKernel_stub")]
extern "C" {}
#[link(name = "SceFace_stub", kind = "static")]
#[cfg(feature = "SceFace_stub")]
extern "C" {}
#[link(name = "SceFiber_stub", kind = "static")]
#[cfg(feature = "SceFiber_stub")]
extern "C" {
    pub fn _sceFiberInitializeImpl(
        fiber: *mut SceFiber,
        name: *mut crate::ctypes::c_char,
        entry: SceFiberEntry,
        argOnInitialize: SceUInt32,
        addrContext: *mut crate::ctypes::c_void,
        sizeContext: SceSize,
        params: *mut SceFiberOptParam,
    ) -> SceInt32;
    pub fn sceFiberFinalize(fiber: *mut SceFiber) -> SceInt32;
    pub fn sceFiberGetInfo(fiber: *mut SceFiber, fiberInfo: *mut SceFiberInfo) -> SceInt32;
    pub fn sceFiberGetSelf(fiber: *mut SceFiber) -> SceInt32;
    pub fn sceFiberOptParamInitialize(optParam: *mut SceFiberOptParam) -> SceInt32;
    pub fn sceFiberReturnToThread(argOnReturn: SceUInt32, argOnRun: *mut SceUInt32) -> SceInt32;
    pub fn sceFiberRun(
        fiber: *mut SceFiber,
        argOnRunTo: SceUInt32,
        argOnRun: *mut SceUInt32,
    ) -> SceInt32;
    pub fn sceFiberSwitch(
        fiber: *mut SceFiber,
        argOnRunTo: SceUInt32,
        argOnRun: *mut SceUInt32,
    ) -> SceInt32;
}
#[link(name = "SceFios2KernelForDriver_stub", kind = "static")]
#[cfg(feature = "SceFios2KernelForDriver_stub")]
extern "C" {
    pub fn ksceFiosKernelOverlayAdd(
        overlay: *mut SceFiosOverlay,
        outID: *mut SceFiosOverlayID,
    ) -> crate::ctypes::c_int;
    pub fn ksceFiosKernelOverlayAddForProcess(
        pid: SceUID,
        overlay: *mut SceFiosOverlay,
        outID: *mut SceFiosOverlayID,
    ) -> crate::ctypes::c_int;
    pub fn ksceFiosKernelOverlayRemoveForProcess(
        pid: SceUID,
        id: SceFiosOverlayID,
    ) -> crate::ctypes::c_int;
    pub fn ksceFiosKernelOverlayResolveSync(
        pid: SceUID,
        resolveFlag: crate::ctypes::c_int,
        inPath: *const crate::ctypes::c_char,
        outPath: *mut crate::ctypes::c_char,
        maxPath: SceSize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceFios2Kernel_stub", kind = "static")]
#[cfg(feature = "SceFios2Kernel_stub")]
extern "C" {
    pub fn _sceFiosKernelOverlayAdd(
        overlay: *const SceFiosKernelOverlay,
        out_id: *mut SceFiosKernelOverlayID,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayAddForProcess(
        target_process: SceUID,
        overlay: *const SceFiosKernelOverlay,
        out_id: *mut SceFiosKernelOverlayID,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayDHChstatSync(
        dh: SceFiosKernelOverlayDH,
        new_stat: *const SceFiosNativeStat,
        cbit: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayDHCloseSync(dh: SceFiosKernelOverlayDH) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayDHOpenSync(
        out_dh: *mut SceFiosKernelOverlayDH,
        path: *const crate::ctypes::c_char,
        from_order: SceUInt8,
        args: *mut SceFiosDHOpenSyncSyscallArgs,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayDHReadSync(
        dh: SceFiosKernelOverlayDH,
        out_entry: *mut SceFiosNativeDirEntry,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayDHStatSync(
        dh: SceFiosKernelOverlayDH,
        out_stat: *mut SceFiosNativeStat,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayDHSyncSync(
        dh: SceFiosKernelOverlayDH,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayGetInfo(
        id: SceFiosKernelOverlayID,
        out_overlay: *mut SceFiosKernelOverlay,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayGetInfoForProcess(
        target_process: SceUID,
        id: SceFiosKernelOverlayID,
        out_overlay: *mut SceFiosKernelOverlay,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayGetList(
        pid: SceUID,
        min_order: SceUInt8,
        max_order: SceUInt8,
        args: *mut SceFiosGetListSyscallArgs,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayGetRecommendedScheduler(
        avail: crate::ctypes::c_int,
        partially_resolved_path: *const crate::ctypes::c_char,
        a3: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayModify(
        id: SceFiosKernelOverlayID,
        new_value: *const SceFiosKernelOverlay,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayModifyForProcess(
        target_process: SceUID,
        id: SceFiosKernelOverlayID,
        new_value: *const SceFiosKernelOverlay,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayRemove(id: SceFiosKernelOverlayID) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayRemoveForProcess(
        target_process: SceUID,
        id: SceFiosKernelOverlayID,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayResolveSync(
        pid: SceUID,
        resolve_flag: crate::ctypes::c_int,
        in_path: *const crate::ctypes::c_char,
        args: *mut SceFiosResolveSyncSyscallArgs,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayResolveWithRangeSync(
        pid: SceUID,
        resolve_flag: crate::ctypes::c_int,
        in_path: *const crate::ctypes::c_char,
        args: *mut SceFiosResolveWithRangeSyncSyscallArgs,
    ) -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayThreadIsDisabled() -> crate::ctypes::c_int;
    pub fn _sceFiosKernelOverlayThreadSetDisabled(disabled: SceInt32) -> crate::ctypes::c_int;
    pub fn sceFiosKernelOverlayAddForProcess02(
        pid: SceUID,
        overlay: *mut SceFiosOverlay,
        outID: *mut SceFiosOverlayID,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceFios2_stub", kind = "static")]
#[cfg(feature = "SceFios2_stub")]
extern "C" {}
#[link(name = "SceGameUpdate_stub", kind = "static")]
#[cfg(feature = "SceGameUpdate_stub")]
extern "C" {}
#[link(name = "SceGpioForDriver_stub", kind = "static")]
#[cfg(feature = "SceGpioForDriver_stub")]
extern "C" {
    pub fn ksceGpioAcquireIntr(
        bus: crate::ctypes::c_int,
        port: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpioGetIntrMode(
        bus: crate::ctypes::c_int,
        port: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpioGetPortMode(
        bus: crate::ctypes::c_int,
        port: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpioPortClear(
        bus: crate::ctypes::c_int,
        port: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpioPortRead(
        bus: crate::ctypes::c_int,
        port: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpioPortReset(
        bus: crate::ctypes::c_int,
        port: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpioPortSet(
        bus: crate::ctypes::c_int,
        port: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpioQueryIntr(
        bus: crate::ctypes::c_int,
        port: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpioSetIntrMode(
        bus: crate::ctypes::c_int,
        port: crate::ctypes::c_int,
        intr_mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpioSetPortMode(
        bus: crate::ctypes::c_int,
        port: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceGps_stub", kind = "static")]
#[cfg(feature = "SceGps_stub")]
extern "C" {
    pub fn _sceGpsClose() -> crate::ctypes::c_int;
    pub fn _sceGpsGetData(
        pos: *mut SceGpsPositionData,
        sat: *mut SceGpsSatelliteData,
    ) -> crate::ctypes::c_int;
    pub fn _sceGpsGetDeviceInfo(dev_info: *mut SceGpsDeviceInfo) -> crate::ctypes::c_int;
    pub fn _sceGpsGetState(state: *mut SceGpsStatus) -> crate::ctypes::c_int;
    pub fn _sceGpsIoctl(
        ioctl_command: SceUInt32,
        arg: *mut SceVoid,
        arg_size: SceSize,
        a4: *mut SceSize,
    ) -> crate::ctypes::c_int;
    pub fn _sceGpsIsDevice() -> crate::ctypes::c_int;
    pub fn _sceGpsOpen(cbid: SceUID) -> crate::ctypes::c_int;
    pub fn _sceGpsResumeCallback() -> crate::ctypes::c_int;
    pub fn _sceGpsSelectDevice(device_type: SceUInt32) -> crate::ctypes::c_int;
    pub fn _sceGpsStart(mode: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn _sceGpsStop() -> crate::ctypes::c_int;
}
#[link(name = "SceGpuEs4ForDriver_stub", kind = "static")]
#[cfg(feature = "SceGpuEs4ForDriver_stub")]
extern "C" {
    pub fn PVRSRVGetMiscInfoKM(info: *mut crate::ctypes::c_void) -> crate::ctypes::c_int;
    pub fn ksceGpuGetRegisterDump(
        dst: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpuMmuMapMemory(
        mmuContext: *mut crate::ctypes::c_void,
        vaddr: u32,
        base: *mut crate::ctypes::c_void,
        size: u32,
        flags: u32,
    ) -> crate::ctypes::c_int;
    pub fn ksceGpuMmuUnmapMemory(
        mmuContext: *mut crate::ctypes::c_void,
        vaddr: u32,
        size: u32,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceGpuEs4_stub", kind = "static")]
#[cfg(feature = "SceGpuEs4_stub")]
extern "C" {}
#[link(name = "SceGxm_stub", kind = "static")]
#[cfg(feature = "SceGxm_stub")]
extern "C" {
    pub fn sceGxmBeginCommandList(context: *mut SceGxmContext) -> crate::ctypes::c_int;
    pub fn sceGxmBeginScene(
        context: *mut SceGxmContext,
        flags: crate::ctypes::c_uint,
        renderTarget: *const SceGxmRenderTarget,
        validRegion: *const SceGxmValidRegion,
        vertexSyncObject: *mut SceGxmSyncObject,
        fragmentSyncObject: *mut SceGxmSyncObject,
        colorSurface: *const SceGxmColorSurface,
        depthStencil: *const SceGxmDepthStencilSurface,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmColorSurfaceGetClip(
        surface: *const SceGxmColorSurface,
        xMin: *mut crate::ctypes::c_uint,
        yMin: *mut crate::ctypes::c_uint,
        xMax: *mut crate::ctypes::c_uint,
        yMax: *mut crate::ctypes::c_uint,
    );
    pub fn sceGxmColorSurfaceGetData(
        surface: *const SceGxmColorSurface,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceGxmColorSurfaceGetDitherMode(
        surface: *const SceGxmColorSurface,
    ) -> SceGxmColorSurfaceDitherMode;
    pub fn sceGxmColorSurfaceGetFormat(surface: *const SceGxmColorSurface) -> SceGxmColorFormat;
    pub fn sceGxmColorSurfaceGetGammaMode(
        surface: *const SceGxmColorSurface,
    ) -> SceGxmColorSurfaceGammaMode;
    pub fn sceGxmColorSurfaceGetScaleMode(
        surface: *const SceGxmColorSurface,
    ) -> SceGxmColorSurfaceScaleMode;
    pub fn sceGxmColorSurfaceGetStrideInPixels(
        surface: *const SceGxmColorSurface,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmColorSurfaceGetType(surface: *const SceGxmColorSurface) -> SceGxmColorSurfaceType;
    pub fn sceGxmColorSurfaceInit(
        surface: *mut SceGxmColorSurface,
        colorFormat: SceGxmColorFormat,
        surfaceType: SceGxmColorSurfaceType,
        scaleMode: SceGxmColorSurfaceScaleMode,
        outputRegisterSize: SceGxmOutputRegisterSize,
        width: crate::ctypes::c_uint,
        height: crate::ctypes::c_uint,
        strideInPixels: crate::ctypes::c_uint,
        data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmColorSurfaceInitDisabled(surface: *mut SceGxmColorSurface)
        -> crate::ctypes::c_int;
    pub fn sceGxmColorSurfaceIsEnabled(surface: *const SceGxmColorSurface) -> SceBool;
    pub fn sceGxmColorSurfaceSetClip(
        surface: *mut SceGxmColorSurface,
        xMin: crate::ctypes::c_uint,
        yMin: crate::ctypes::c_uint,
        xMax: crate::ctypes::c_uint,
        yMax: crate::ctypes::c_uint,
    );
    pub fn sceGxmColorSurfaceSetData(
        surface: *mut SceGxmColorSurface,
        data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmColorSurfaceSetDitherMode(
        surface: *mut SceGxmColorSurface,
        ditherMode: SceGxmColorSurfaceDitherMode,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmColorSurfaceSetFormat(
        surface: *mut SceGxmColorSurface,
        format: SceGxmColorFormat,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmColorSurfaceSetGammaMode(
        surface: *mut SceGxmColorSurface,
        gammaMode: SceGxmColorSurfaceGammaMode,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmColorSurfaceSetScaleMode(
        surface: *mut SceGxmColorSurface,
        scaleMode: SceGxmColorSurfaceScaleMode,
    );
    pub fn sceGxmCreateContext(
        params: *const SceGxmContextParams,
        context: *mut *mut SceGxmContext,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmCreateDeferredContext(
        params: *const SceGxmDeferredContextParams,
        context: *mut *mut SceGxmContext,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmCreateRenderTarget(
        params: *const SceGxmRenderTargetParams,
        renderTarget: *mut *mut SceGxmRenderTarget,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmDepthStencilSurfaceGetBackgroundDepth(
        surface: *const SceGxmDepthStencilSurface,
    ) -> f32;
    pub fn sceGxmDepthStencilSurfaceGetBackgroundStencil(
        surface: *const SceGxmDepthStencilSurface,
    ) -> crate::ctypes::c_uchar;
    pub fn sceGxmDepthStencilSurfaceGetForceLoadMode(
        surface: *const SceGxmDepthStencilSurface,
    ) -> SceGxmDepthStencilForceLoadMode;
    pub fn sceGxmDepthStencilSurfaceGetForceStoreMode(
        surface: *const SceGxmDepthStencilSurface,
    ) -> SceGxmDepthStencilForceStoreMode;
    pub fn sceGxmDepthStencilSurfaceGetFormat(
        surface: *const SceGxmDepthStencilSurface,
    ) -> SceGxmDepthStencilFormat;
    pub fn sceGxmDepthStencilSurfaceGetStrideInSamples(
        surface: *const SceGxmDepthStencilSurface,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmDepthStencilSurfaceInit(
        surface: *mut SceGxmDepthStencilSurface,
        depthStencilFormat: SceGxmDepthStencilFormat,
        surfaceType: SceGxmDepthStencilSurfaceType,
        strideInSamples: crate::ctypes::c_uint,
        depthData: *mut crate::ctypes::c_void,
        stencilData: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmDepthStencilSurfaceInitDisabled(
        surface: *mut SceGxmDepthStencilSurface,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmDepthStencilSurfaceIsEnabled(surface: *const SceGxmDepthStencilSurface)
        -> SceBool;
    pub fn sceGxmDepthStencilSurfaceSetBackgroundDepth(
        surface: *mut SceGxmDepthStencilSurface,
        backgroundDepth: f32,
    );
    pub fn sceGxmDepthStencilSurfaceSetBackgroundStencil(
        surface: *mut SceGxmDepthStencilSurface,
        backgroundStencil: crate::ctypes::c_uchar,
    );
    pub fn sceGxmDepthStencilSurfaceSetForceLoadMode(
        surface: *mut SceGxmDepthStencilSurface,
        forceLoad: SceGxmDepthStencilForceLoadMode,
    );
    pub fn sceGxmDepthStencilSurfaceSetForceStoreMode(
        surface: *mut SceGxmDepthStencilSurface,
        forceStore: SceGxmDepthStencilForceStoreMode,
    );
    pub fn sceGxmDestroyContext(context: *mut SceGxmContext) -> crate::ctypes::c_int;
    pub fn sceGxmDestroyDeferredContext(context: *mut SceGxmContext) -> crate::ctypes::c_int;
    pub fn sceGxmDestroyRenderTarget(renderTarget: *mut SceGxmRenderTarget)
        -> crate::ctypes::c_int;
    pub fn sceGxmDisplayQueueAddEntry(
        oldBuffer: *mut SceGxmSyncObject,
        newBuffer: *mut SceGxmSyncObject,
        callbackData: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmDisplayQueueFinish() -> crate::ctypes::c_int;
    pub fn sceGxmDraw(
        context: *mut SceGxmContext,
        primType: SceGxmPrimitiveType,
        indexType: SceGxmIndexFormat,
        indexData: *const crate::ctypes::c_void,
        indexCount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmDrawInstanced(
        context: *mut SceGxmContext,
        primType: SceGxmPrimitiveType,
        indexType: SceGxmIndexFormat,
        indexData: *const crate::ctypes::c_void,
        indexCount: crate::ctypes::c_uint,
        indexWrap: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmDrawPrecomputed(
        context: *mut SceGxmContext,
        precomputedDraw: *const SceGxmPrecomputedDraw,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmEndCommandList(
        context: *mut SceGxmContext,
        list: *mut SceGxmCommandList,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmEndScene(
        context: *mut SceGxmContext,
        vertexNotification: *const SceGxmNotification,
        fragmentNotification: *const SceGxmNotification,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmExecuteCommandList(
        context: *mut SceGxmContext,
        list: *mut SceGxmCommandList,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmFinish(context: *mut SceGxmContext);
    pub fn sceGxmFragmentProgramGetProgram(
        fragmentProgram: *const SceGxmFragmentProgram,
    ) -> *const SceGxmProgram;
    pub fn sceGxmGetNotificationRegion() -> *mut crate::ctypes::c_uint;
    pub fn sceGxmGetPrecomputedDrawSize(
        vertexProgram: *const SceGxmVertexProgram,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmGetPrecomputedFragmentStateSize(
        fragmentProgram: *const SceGxmFragmentProgram,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmGetPrecomputedVertexStateSize(
        vertexProgram: *const SceGxmVertexProgram,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmGetRenderTargetMemSize(
        params: *const SceGxmRenderTargetParams,
        driverMemSize: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmInitialize(params: *const SceGxmInitializeParams) -> crate::ctypes::c_int;
    pub fn sceGxmMapFragmentUsseMemory(
        base: *mut crate::ctypes::c_void,
        size: SceSize,
        offset: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmMapMemory(
        base: *mut crate::ctypes::c_void,
        size: SceSize,
        attr: SceGxmMemoryAttribFlags,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmMapVertexUsseMemory(
        base: *mut crate::ctypes::c_void,
        size: SceSize,
        offset: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmMidSceneFlush(
        context: *mut SceGxmContext,
        flags: crate::ctypes::c_uint,
        vertexSyncObject: *mut SceGxmSyncObject,
        vertexNotification: *const SceGxmNotification,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmNotificationWait(notification: *const SceGxmNotification) -> crate::ctypes::c_int;
    pub fn sceGxmPadHeartbeat(
        displaySurface: *const SceGxmColorSurface,
        displaySyncObject: *mut SceGxmSyncObject,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPadTriggerGpuPaTrace() -> crate::ctypes::c_int;
    pub fn sceGxmPopUserMarker(context: *mut SceGxmContext) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedDrawInit(
        precomputedDraw: *mut SceGxmPrecomputedDraw,
        vertexProgram: *const SceGxmVertexProgram,
        memBlock: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedDrawSetAllVertexStreams(
        precomputedDraw: *mut SceGxmPrecomputedDraw,
        streamDataArray: *const *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedDrawSetParams(
        precomputedDraw: *mut SceGxmPrecomputedDraw,
        primType: SceGxmPrimitiveType,
        indexType: SceGxmIndexFormat,
        indexData: *const crate::ctypes::c_void,
        indexCount: crate::ctypes::c_uint,
    );
    pub fn sceGxmPrecomputedDrawSetParamsInstanced(
        precomputedDraw: *mut SceGxmPrecomputedDraw,
        primType: SceGxmPrimitiveType,
        indexType: SceGxmIndexFormat,
        indexData: *const crate::ctypes::c_void,
        indexCount: crate::ctypes::c_uint,
        indexWrap: crate::ctypes::c_uint,
    );
    pub fn sceGxmPrecomputedDrawSetVertexStream(
        precomputedDraw: *mut SceGxmPrecomputedDraw,
        streamIndex: crate::ctypes::c_uint,
        streamData: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedFragmentStateGetDefaultUniformBuffer(
        precomputedState: *const SceGxmPrecomputedFragmentState,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceGxmPrecomputedFragmentStateInit(
        precomputedState: *mut SceGxmPrecomputedFragmentState,
        fragmentProgram: *const SceGxmFragmentProgram,
        memBlock: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedFragmentStateSetAllAuxiliarySurfaces(
        precomputedState: *mut SceGxmPrecomputedFragmentState,
        auxSurfaceArray: *const SceGxmAuxiliarySurface,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedFragmentStateSetAllTextures(
        precomputedState: *mut SceGxmPrecomputedFragmentState,
        textureArray: *const SceGxmTexture,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedFragmentStateSetAllUniformBuffers(
        precomputedState: *mut SceGxmPrecomputedFragmentState,
        bufferDataArray: *const *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedFragmentStateSetDefaultUniformBuffer(
        precomputedState: *mut SceGxmPrecomputedFragmentState,
        defaultBuffer: *mut crate::ctypes::c_void,
    );
    pub fn sceGxmPrecomputedFragmentStateSetTexture(
        precomputedState: *mut SceGxmPrecomputedFragmentState,
        textureIndex: crate::ctypes::c_uint,
        texture: *const SceGxmTexture,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedFragmentStateSetUniformBuffer(
        precomputedState: *mut SceGxmPrecomputedFragmentState,
        bufferIndex: crate::ctypes::c_uint,
        bufferData: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedVertexStateGetDefaultUniformBuffer(
        precomputedState: *const SceGxmPrecomputedVertexState,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceGxmPrecomputedVertexStateInit(
        precomputedState: *mut SceGxmPrecomputedVertexState,
        vertexProgram: *const SceGxmVertexProgram,
        memBlock: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedVertexStateSetAllTextures(
        precomputedState: *mut SceGxmPrecomputedVertexState,
        textures: *const SceGxmTexture,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedVertexStateSetAllUniformBuffers(
        precomputedState: *mut SceGxmPrecomputedVertexState,
        bufferDataArray: *const *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedVertexStateSetDefaultUniformBuffer(
        precomputedState: *mut SceGxmPrecomputedVertexState,
        defaultBuffer: *mut crate::ctypes::c_void,
    );
    pub fn sceGxmPrecomputedVertexStateSetTexture(
        precomputedState: *mut SceGxmPrecomputedVertexState,
        textureIndex: crate::ctypes::c_uint,
        texture: *const SceGxmTexture,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmPrecomputedVertexStateSetUniformBuffer(
        precomputedState: *mut SceGxmPrecomputedVertexState,
        bufferIndex: crate::ctypes::c_uint,
        bufferData: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmProgramCheck(program: *const SceGxmProgram) -> crate::ctypes::c_int;
    pub fn sceGxmProgramFindParameterByName(
        program: *const SceGxmProgram,
        name: *const crate::ctypes::c_char,
    ) -> *const SceGxmProgramParameter;
    pub fn sceGxmProgramFindParameterBySemantic(
        program: *const SceGxmProgram,
        semantic: SceGxmParameterSemantic,
        index: crate::ctypes::c_uint,
    ) -> *const SceGxmProgramParameter;
    pub fn sceGxmProgramGetDefaultUniformBufferSize(
        program: *const SceGxmProgram,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmProgramGetParameter(
        program: *const SceGxmProgram,
        index: crate::ctypes::c_uint,
    ) -> *const SceGxmProgramParameter;
    pub fn sceGxmProgramGetParameterCount(program: *const SceGxmProgram) -> crate::ctypes::c_uint;
    pub fn sceGxmProgramGetSize(program: *const SceGxmProgram) -> crate::ctypes::c_uint;
    pub fn sceGxmProgramGetType(program: *const SceGxmProgram) -> SceGxmProgramType;
    pub fn sceGxmProgramIsDepthReplaceUsed(program: *const SceGxmProgram) -> SceBool;
    pub fn sceGxmProgramIsDiscardUsed(program: *const SceGxmProgram) -> SceBool;
    pub fn sceGxmProgramIsSpriteCoordUsed(program: *const SceGxmProgram) -> SceBool;
    pub fn sceGxmProgramParameterGetArraySize(
        parameter: *const SceGxmProgramParameter,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmProgramParameterGetCategory(
        parameter: *const SceGxmProgramParameter,
    ) -> SceGxmParameterCategory;
    pub fn sceGxmProgramParameterGetComponentCount(
        parameter: *const SceGxmProgramParameter,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmProgramParameterGetContainerIndex(
        parameter: *const SceGxmProgramParameter,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmProgramParameterGetIndex(
        program: *const SceGxmProgram,
        parameter: *const SceGxmProgramParameter,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmProgramParameterGetName(
        parameter: *const SceGxmProgramParameter,
    ) -> *const crate::ctypes::c_char;
    pub fn sceGxmProgramParameterGetResourceIndex(
        parameter: *const SceGxmProgramParameter,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmProgramParameterGetSemantic(
        parameter: *const SceGxmProgramParameter,
    ) -> SceGxmParameterSemantic;
    pub fn sceGxmProgramParameterGetSemanticIndex(
        parameter: *const SceGxmProgramParameter,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmProgramParameterGetType(
        parameter: *const SceGxmProgramParameter,
    ) -> SceGxmParameterType;
    pub fn sceGxmProgramParameterIsSamplerCube(parameter: *const SceGxmProgramParameter)
        -> SceBool;
    pub fn sceGxmPushUserMarker(
        context: *mut SceGxmContext,
        tag: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmRenderTargetGetDriverMemBlock(
        renderTarget: *const SceGxmRenderTarget,
        driverMemBlock: *mut SceUID,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmRenderTargetGetHostMem(
        renderTarget: *const SceGxmRenderTarget,
        hostMem: *mut *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmReserveFragmentDefaultUniformBuffer(
        context: *mut SceGxmContext,
        uniformBuffer: *mut *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmReserveVertexDefaultUniformBuffer(
        context: *mut SceGxmContext,
        uniformBuffer: *mut *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetAuxiliarySurface(
        context: *mut SceGxmContext,
        surfaceIndex: crate::ctypes::c_uint,
        surface: *const SceGxmAuxiliarySurface,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetBackDepthBias(
        context: *mut SceGxmContext,
        factor: crate::ctypes::c_int,
        units: crate::ctypes::c_int,
    );
    pub fn sceGxmSetBackDepthFunc(context: *mut SceGxmContext, depthFunc: SceGxmDepthFunc);
    pub fn sceGxmSetBackDepthWriteEnable(context: *mut SceGxmContext, enable: SceGxmDepthWriteMode);
    pub fn sceGxmSetBackFragmentProgramEnable(
        context: *mut SceGxmContext,
        enable: SceGxmFragmentProgramMode,
    );
    pub fn sceGxmSetBackLineFillLastPixelEnable(
        context: *mut SceGxmContext,
        enable: SceGxmLineFillLastPixelMode,
    );
    pub fn sceGxmSetBackPointLineWidth(context: *mut SceGxmContext, width: crate::ctypes::c_uint);
    pub fn sceGxmSetBackPolygonMode(context: *mut SceGxmContext, mode: SceGxmPolygonMode);
    pub fn sceGxmSetBackStencilFunc(
        context: *mut SceGxmContext,
        func: SceGxmStencilFunc,
        stencilFail: SceGxmStencilOp,
        depthFail: SceGxmStencilOp,
        depthPass: SceGxmStencilOp,
        compareMask: crate::ctypes::c_uchar,
        writeMask: crate::ctypes::c_uchar,
    );
    pub fn sceGxmSetBackStencilRef(context: *mut SceGxmContext, sref: crate::ctypes::c_uint);
    pub fn sceGxmSetBackVisibilityTestEnable(
        context: *mut SceGxmContext,
        enable: SceGxmVisibilityTestMode,
    );
    pub fn sceGxmSetBackVisibilityTestIndex(
        context: *mut SceGxmContext,
        index: crate::ctypes::c_uint,
    );
    pub fn sceGxmSetBackVisibilityTestOp(context: *mut SceGxmContext, op: SceGxmVisibilityTestOp);
    pub fn sceGxmSetCullMode(context: *mut SceGxmContext, mode: SceGxmCullMode);
    pub fn sceGxmSetDefaultRegionClipAndViewport(
        context: *mut SceGxmContext,
        xMax: crate::ctypes::c_uint,
        yMax: crate::ctypes::c_uint,
    );
    pub fn sceGxmSetFragmentDefaultUniformBuffer(
        context: *mut SceGxmContext,
        uniformBuffer: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetFragmentProgram(
        context: *mut SceGxmContext,
        fragmentProgram: *const SceGxmFragmentProgram,
    );
    pub fn sceGxmSetFragmentTexture(
        context: *mut SceGxmContext,
        textureIndex: crate::ctypes::c_uint,
        texture: *const SceGxmTexture,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetFragmentUniformBuffer(
        context: *mut SceGxmContext,
        bufferIndex: crate::ctypes::c_uint,
        bufferData: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetFrontDepthBias(
        context: *mut SceGxmContext,
        factor: crate::ctypes::c_int,
        units: crate::ctypes::c_int,
    );
    pub fn sceGxmSetFrontDepthFunc(context: *mut SceGxmContext, depthFunc: SceGxmDepthFunc);
    pub fn sceGxmSetFrontDepthWriteEnable(
        context: *mut SceGxmContext,
        enable: SceGxmDepthWriteMode,
    );
    pub fn sceGxmSetFrontFragmentProgramEnable(
        context: *mut SceGxmContext,
        enable: SceGxmFragmentProgramMode,
    );
    pub fn sceGxmSetFrontLineFillLastPixelEnable(
        context: *mut SceGxmContext,
        enable: SceGxmLineFillLastPixelMode,
    );
    pub fn sceGxmSetFrontPointLineWidth(context: *mut SceGxmContext, width: crate::ctypes::c_uint);
    pub fn sceGxmSetFrontPolygonMode(context: *mut SceGxmContext, mode: SceGxmPolygonMode);
    pub fn sceGxmSetFrontStencilFunc(
        context: *mut SceGxmContext,
        func: SceGxmStencilFunc,
        stencilFail: SceGxmStencilOp,
        depthFail: SceGxmStencilOp,
        depthPass: SceGxmStencilOp,
        compareMask: crate::ctypes::c_uchar,
        writeMask: crate::ctypes::c_uchar,
    );
    pub fn sceGxmSetFrontStencilRef(context: *mut SceGxmContext, sref: crate::ctypes::c_uint);
    pub fn sceGxmSetFrontVisibilityTestEnable(
        context: *mut SceGxmContext,
        enable: SceGxmVisibilityTestMode,
    );
    pub fn sceGxmSetFrontVisibilityTestIndex(
        context: *mut SceGxmContext,
        index: crate::ctypes::c_uint,
    );
    pub fn sceGxmSetFrontVisibilityTestOp(context: *mut SceGxmContext, op: SceGxmVisibilityTestOp);
    pub fn sceGxmSetPrecomputedFragmentState(
        context: *mut SceGxmContext,
        precomputedState: *const SceGxmPrecomputedFragmentState,
    );
    pub fn sceGxmSetPrecomputedVertexState(
        context: *mut SceGxmContext,
        precomputedState: *const SceGxmPrecomputedVertexState,
    );
    pub fn sceGxmSetRegionClip(
        context: *mut SceGxmContext,
        mode: SceGxmRegionClipMode,
        xMin: crate::ctypes::c_uint,
        yMin: crate::ctypes::c_uint,
        xMax: crate::ctypes::c_uint,
        yMax: crate::ctypes::c_uint,
    );
    pub fn sceGxmSetTwoSidedEnable(context: *mut SceGxmContext, enable: SceGxmTwoSidedMode);
    pub fn sceGxmSetUniformDataF(
        uniformBuffer: *mut crate::ctypes::c_void,
        parameter: *const SceGxmProgramParameter,
        componentOffset: crate::ctypes::c_uint,
        componentCount: crate::ctypes::c_uint,
        sourceData: *const f32,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetUserMarker(
        context: *mut SceGxmContext,
        tag: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetValidationEnable(context: *mut SceGxmContext, enable: SceBool);
    pub fn sceGxmSetVertexDefaultUniformBuffer(
        context: *mut SceGxmContext,
        uniformBuffer: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetVertexProgram(
        context: *mut SceGxmContext,
        vertexProgram: *const SceGxmVertexProgram,
    );
    pub fn sceGxmSetVertexStream(
        context: *mut SceGxmContext,
        streamIndex: crate::ctypes::c_uint,
        streamData: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetVertexTexture(
        context: *mut SceGxmContext,
        textureIndex: crate::ctypes::c_uint,
        texture: *const SceGxmTexture,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetVertexUniformBuffer(
        context: *mut SceGxmContext,
        bufferIndex: crate::ctypes::c_uint,
        bufferData: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetViewport(
        context: *mut SceGxmContext,
        xOffset: f32,
        xScale: f32,
        yOffset: f32,
        yScale: f32,
        zOffset: f32,
        zScale: f32,
    );
    pub fn sceGxmSetViewportEnable(context: *mut SceGxmContext, enable: SceGxmViewportMode);
    pub fn sceGxmSetVisibilityBuffer(
        context: *mut SceGxmContext,
        bufferBase: *mut crate::ctypes::c_void,
        stridePerCore: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSetWBufferEnable(context: *mut SceGxmContext, enable: SceGxmWBufferMode);
    pub fn sceGxmSetWClampEnable(context: *mut SceGxmContext, enable: SceGxmWClampMode);
    pub fn sceGxmSetWClampValue(context: *mut SceGxmContext, clampValue: f32);
    pub fn sceGxmSetYuvProfile(
        context: *mut SceGxmContext,
        index: crate::ctypes::c_uint,
        profile: SceGxmYuvProfile,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherAddRefFragmentProgram(
        shaderPatcher: *mut SceGxmShaderPatcher,
        fragmentProgram: *mut SceGxmFragmentProgram,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherAddRefVertexProgram(
        shaderPatcher: *mut SceGxmShaderPatcher,
        vertexProgram: *mut SceGxmVertexProgram,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherCreate(
        params: *const SceGxmShaderPatcherParams,
        shaderPatcher: *mut *mut SceGxmShaderPatcher,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherCreateFragmentProgram(
        shaderPatcher: *mut SceGxmShaderPatcher,
        programId: SceGxmShaderPatcherId,
        outputFormat: SceGxmOutputRegisterFormat,
        multisampleMode: SceGxmMultisampleMode,
        blendInfo: *const SceGxmBlendInfo,
        vertexProgram: *const SceGxmProgram,
        fragmentProgram: *mut *mut SceGxmFragmentProgram,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherCreateMaskUpdateFragmentProgram(
        shaderPatcher: *mut SceGxmShaderPatcher,
        fragmentProgram: *mut *mut SceGxmFragmentProgram,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherCreateVertexProgram(
        shaderPatcher: *mut SceGxmShaderPatcher,
        programId: SceGxmShaderPatcherId,
        attributes: *const SceGxmVertexAttribute,
        attributeCount: crate::ctypes::c_uint,
        streams: *const SceGxmVertexStream,
        streamCount: crate::ctypes::c_uint,
        vertexProgram: *mut *mut SceGxmVertexProgram,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherDestroy(
        shaderPatcher: *mut SceGxmShaderPatcher,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherForceUnregisterProgram(
        shaderPatcher: *mut SceGxmShaderPatcher,
        programId: SceGxmShaderPatcherId,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherGetBufferMemAllocated(
        shaderPatcher: *const SceGxmShaderPatcher,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmShaderPatcherGetFragmentProgramRefCount(
        shaderPatcher: *mut SceGxmShaderPatcher,
        fragmentProgram: *mut SceGxmFragmentProgram,
        count: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherGetFragmentUsseMemAllocated(
        shaderPatcher: *const SceGxmShaderPatcher,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmShaderPatcherGetHostMemAllocated(
        shaderPatcher: *const SceGxmShaderPatcher,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmShaderPatcherGetProgramFromId(
        programId: SceGxmShaderPatcherId,
    ) -> *const SceGxmProgram;
    pub fn sceGxmShaderPatcherGetUserData(
        shaderPatcher: *mut SceGxmShaderPatcher,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceGxmShaderPatcherGetVertexProgramRefCount(
        shaderPatcher: *mut SceGxmShaderPatcher,
        fragmentProgram: *mut SceGxmVertexProgram,
        count: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherGetVertexUsseMemAllocated(
        shaderPatcher: *const SceGxmShaderPatcher,
    ) -> crate::ctypes::c_uint;
    pub fn sceGxmShaderPatcherRegisterProgram(
        shaderPatcher: *mut SceGxmShaderPatcher,
        programHeader: *const SceGxmProgram,
        programId: *mut SceGxmShaderPatcherId,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherReleaseFragmentProgram(
        shaderPatcher: *mut SceGxmShaderPatcher,
        fragmentProgram: *mut SceGxmFragmentProgram,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherReleaseVertexProgram(
        shaderPatcher: *mut SceGxmShaderPatcher,
        vertexProgram: *mut SceGxmVertexProgram,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherSetAuxiliarySurface(
        shaderPatcher: *mut SceGxmShaderPatcher,
        auxSurfaceIndex: crate::ctypes::c_uint,
        auxSurface: *const SceGxmAuxiliarySurface,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherSetUserData(
        shaderPatcher: *mut SceGxmShaderPatcher,
        userData: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmShaderPatcherUnregisterProgram(
        shaderPatcher: *mut SceGxmShaderPatcher,
        programId: SceGxmShaderPatcherId,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmSyncObjectCreate(syncObject: *mut *mut SceGxmSyncObject) -> crate::ctypes::c_int;
    pub fn sceGxmSyncObjectDestroy(syncObject: *mut SceGxmSyncObject) -> crate::ctypes::c_int;
    pub fn sceGxmTerminate() -> crate::ctypes::c_int;
    pub fn sceGxmTextureGetData(texture: *const SceGxmTexture) -> *mut crate::ctypes::c_void;
    pub fn sceGxmTextureGetFormat(texture: *const SceGxmTexture) -> SceGxmTextureFormat;
    pub fn sceGxmTextureGetGammaMode(texture: *const SceGxmTexture) -> SceGxmTextureGammaMode;
    pub fn sceGxmTextureGetHeight(texture: *const SceGxmTexture) -> crate::ctypes::c_uint;
    pub fn sceGxmTextureGetLodBias(texture: *const SceGxmTexture) -> crate::ctypes::c_uint;
    pub fn sceGxmTextureGetMagFilter(texture: *const SceGxmTexture) -> SceGxmTextureFilter;
    pub fn sceGxmTextureGetMinFilter(texture: *const SceGxmTexture) -> SceGxmTextureFilter;
    pub fn sceGxmTextureGetMipFilter(texture: *const SceGxmTexture) -> SceGxmTextureMipFilter;
    pub fn sceGxmTextureGetMipmapCount(texture: *const SceGxmTexture) -> crate::ctypes::c_uint;
    pub fn sceGxmTextureGetPalette(texture: *const SceGxmTexture) -> *mut crate::ctypes::c_void;
    pub fn sceGxmTextureGetStride(texture: *const SceGxmTexture) -> crate::ctypes::c_uint;
    pub fn sceGxmTextureGetType(texture: *const SceGxmTexture) -> SceGxmTextureType;
    pub fn sceGxmTextureGetUAddrMode(texture: *const SceGxmTexture) -> SceGxmTextureAddrMode;
    pub fn sceGxmTextureGetVAddrMode(texture: *const SceGxmTexture) -> SceGxmTextureAddrMode;
    pub fn sceGxmTextureGetWidth(texture: *const SceGxmTexture) -> crate::ctypes::c_uint;
    pub fn sceGxmTextureInitCube(
        texture: *mut SceGxmTexture,
        data: *const crate::ctypes::c_void,
        texFormat: SceGxmTextureFormat,
        width: crate::ctypes::c_uint,
        height: crate::ctypes::c_uint,
        mipCount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureInitLinear(
        texture: *mut SceGxmTexture,
        data: *const crate::ctypes::c_void,
        texFormat: SceGxmTextureFormat,
        width: crate::ctypes::c_uint,
        height: crate::ctypes::c_uint,
        mipCount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureInitLinearStrided(
        texture: *mut SceGxmTexture,
        data: *const crate::ctypes::c_void,
        texFormat: SceGxmTextureFormat,
        width: crate::ctypes::c_uint,
        height: crate::ctypes::c_uint,
        byteStride: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureInitSwizzled(
        texture: *mut SceGxmTexture,
        data: *const crate::ctypes::c_void,
        texFormat: SceGxmTextureFormat,
        width: crate::ctypes::c_uint,
        height: crate::ctypes::c_uint,
        mipCount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureInitSwizzledArbitrary(
        texture: *mut SceGxmTexture,
        data: *const crate::ctypes::c_void,
        texFormat: SceGxmTextureFormat,
        width: crate::ctypes::c_uint,
        height: crate::ctypes::c_uint,
        mipCount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureInitTiled(
        texture: *mut SceGxmTexture,
        data: *const crate::ctypes::c_void,
        texFormat: SceGxmTextureFormat,
        width: crate::ctypes::c_uint,
        height: crate::ctypes::c_uint,
        mipCount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetData(
        texture: *mut SceGxmTexture,
        data: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetFormat(
        texture: *mut SceGxmTexture,
        texFormat: SceGxmTextureFormat,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetGammaMode(
        texture: *mut SceGxmTexture,
        gammaMode: SceGxmTextureGammaMode,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetHeight(
        texture: *mut SceGxmTexture,
        height: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetLodBias(
        texture: *mut SceGxmTexture,
        bias: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetMagFilter(
        texture: *mut SceGxmTexture,
        magFilter: SceGxmTextureFilter,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetMinFilter(
        texture: *mut SceGxmTexture,
        minFilter: SceGxmTextureFilter,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetMipFilter(
        texture: *mut SceGxmTexture,
        mipFilter: SceGxmTextureMipFilter,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetMipmapCount(
        texture: *mut SceGxmTexture,
        mipCount: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetPalette(
        texture: *mut SceGxmTexture,
        paletteData: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetStride(
        texture: *mut SceGxmTexture,
        byteStride: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetUAddrMode(
        texture: *mut SceGxmTexture,
        addrMode: SceGxmTextureAddrMode,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetVAddrMode(
        texture: *mut SceGxmTexture,
        addrMode: SceGxmTextureAddrMode,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureSetWidth(
        texture: *mut SceGxmTexture,
        width: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTextureValidate(texture: *const SceGxmTexture) -> crate::ctypes::c_int;
    pub fn sceGxmTransferCopy(
        width: u32,
        height: u32,
        colorKeyValue: u32,
        colorKeyMask: u32,
        colorKeyMode: SceGxmTransferColorKeyMode,
        srcFormat: SceGxmTransferFormat,
        srcType: SceGxmTransferType,
        srcAddress: *const crate::ctypes::c_void,
        srcX: u32,
        srcY: u32,
        srcStride: i32,
        destFormat: SceGxmTransferFormat,
        destType: SceGxmTransferType,
        destAddress: *mut crate::ctypes::c_void,
        destX: u32,
        destY: u32,
        destStride: i32,
        syncObject: *mut SceGxmSyncObject,
        syncFlags: u32,
        notification: *const SceGxmNotification,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTransferDownscale(
        srcFormat: SceGxmTransferFormat,
        srcAddress: *const crate::ctypes::c_void,
        srcX: crate::ctypes::c_uint,
        srcY: crate::ctypes::c_uint,
        srcWidth: crate::ctypes::c_uint,
        srcHeight: crate::ctypes::c_uint,
        srcStride: crate::ctypes::c_int,
        destFormat: SceGxmTransferFormat,
        destAddress: *mut crate::ctypes::c_void,
        destX: crate::ctypes::c_uint,
        destY: crate::ctypes::c_uint,
        destStride: crate::ctypes::c_int,
        syncObject: *mut SceGxmSyncObject,
        syncFlags: crate::ctypes::c_uint,
        notification: *const SceGxmNotification,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTransferFill(
        color: u32,
        destFormat: SceGxmTransferFormat,
        destAddress: *mut crate::ctypes::c_void,
        destX: u32,
        destY: u32,
        destWidth: u32,
        destHeight: u32,
        destStride: i32,
        syncObject: *mut SceGxmSyncObject,
        syncFlags: u32,
        notification: *const SceGxmNotification,
    ) -> crate::ctypes::c_int;
    pub fn sceGxmTransferFinish() -> crate::ctypes::c_int;
    pub fn sceGxmUnmapFragmentUsseMemory(base: *mut crate::ctypes::c_void) -> crate::ctypes::c_int;
    pub fn sceGxmUnmapMemory(base: *mut crate::ctypes::c_void) -> crate::ctypes::c_int;
    pub fn sceGxmUnmapVertexUsseMemory(base: *mut crate::ctypes::c_void) -> crate::ctypes::c_int;
    pub fn sceGxmVertexProgramGetProgram(
        vertexProgram: *const SceGxmVertexProgram,
    ) -> *const SceGxmProgram;
    pub fn sceGxmVshInitialize(params: *const SceGxmInitializeParams) -> crate::ctypes::c_int;
}
#[link(name = "SceHandwriting_stub", kind = "static")]
#[cfg(feature = "SceHandwriting_stub")]
extern "C" {}
#[link(name = "SceHidForDriver_stub", kind = "static")]
#[cfg(feature = "SceHidForDriver_stub")]
extern "C" {}
#[link(name = "SceHid_stub", kind = "static")]
#[cfg(feature = "SceHid_stub")]
extern "C" {
    pub fn sceHidKeyboardEnumerate(
        handle: *mut crate::ctypes::c_int,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHidKeyboardPeek(
        handle: SceUInt32,
        reports: *mut *mut SceHidKeyboardReport,
        nReports: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHidKeyboardRead(
        handle: SceUInt32,
        reports: *mut *mut SceHidKeyboardReport,
        nReports: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHidMouseEnumerate(
        handle: *mut crate::ctypes::c_int,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHidMouseRead(
        handle: SceUInt32,
        reports: *mut *mut SceHidMouseReport,
        nReports: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceHttp_stub", kind = "static")]
#[cfg(feature = "SceHttp_stub")]
extern "C" {
    pub fn sceHttpAbortRequest(reqId: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceHttpAddCookie(
        url: *const crate::ctypes::c_char,
        cookie: *const crate::ctypes::c_char,
        cookieLength: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpAddRequestHeader(
        id: crate::ctypes::c_int,
        name: *const crate::ctypes::c_char,
        value: *const crate::ctypes::c_char,
        mode: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpCreateConnection(
        tmplId: crate::ctypes::c_int,
        serverName: *const crate::ctypes::c_char,
        scheme: *const crate::ctypes::c_char,
        port: crate::ctypes::c_ushort,
        enableKeepalive: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpCreateConnectionWithURL(
        tmplId: crate::ctypes::c_int,
        url: *const crate::ctypes::c_char,
        enableKeepalive: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpCreateRequest(
        connId: crate::ctypes::c_int,
        method: crate::ctypes::c_int,
        path: *const crate::ctypes::c_char,
        contentLength: crate::ctypes::c_ulonglong,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpCreateRequestWithURL(
        connId: crate::ctypes::c_int,
        method: crate::ctypes::c_int,
        url: *const crate::ctypes::c_char,
        contentLength: crate::ctypes::c_ulonglong,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpCreateTemplate(
        userAgent: *const crate::ctypes::c_char,
        httpVer: crate::ctypes::c_int,
        autoProxyConf: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpDeleteConnection(connId: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceHttpDeleteRequest(reqId: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceHttpDeleteTemplate(tmplId: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceHttpGetAllResponseHeaders(
        reqId: crate::ctypes::c_int,
        header: *mut *mut crate::ctypes::c_char,
        headerSize: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpGetAuthEnabled(
        id: crate::ctypes::c_int,
        enable: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpGetAutoRedirect(
        id: crate::ctypes::c_int,
        enable: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpGetCookie(
        url: *const crate::ctypes::c_char,
        cookie: *mut crate::ctypes::c_char,
        cookieLength: *mut crate::ctypes::c_uint,
        prepare: crate::ctypes::c_uint,
        secure: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpGetCookieEnabled(
        id: crate::ctypes::c_int,
        enable: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpGetMemoryPoolStats(
        currentStat: *mut SceHttpMemoryPoolStats,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpGetResponseContentLength(
        reqId: crate::ctypes::c_int,
        contentLength: *mut crate::ctypes::c_ulonglong,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpGetStatusCode(
        reqId: crate::ctypes::c_int,
        statusCode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpInit(poolSize: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn sceHttpParseResponseHeader(
        header: *const crate::ctypes::c_char,
        headerLen: crate::ctypes::c_uint,
        fieldStr: *const crate::ctypes::c_char,
        fieldValue: *mut *const crate::ctypes::c_char,
        valueLen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpParseStatusLine(
        statusLine: *const crate::ctypes::c_char,
        lineLen: crate::ctypes::c_uint,
        httpMajorVer: *mut crate::ctypes::c_int,
        httpMinorVer: *mut crate::ctypes::c_int,
        responseCode: *mut crate::ctypes::c_int,
        reasonPhrase: *mut *const crate::ctypes::c_char,
        phraseLen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpReadData(
        reqId: crate::ctypes::c_int,
        data: *mut crate::ctypes::c_void,
        size: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpRemoveRequestHeader(
        id: crate::ctypes::c_int,
        name: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSendRequest(
        reqId: crate::ctypes::c_int,
        postData: *const crate::ctypes::c_void,
        size: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetAuthEnabled(
        id: crate::ctypes::c_int,
        enable: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetAuthInfoCallback(
        id: crate::ctypes::c_int,
        cbfunc: SceHttpAuthInfoCallback,
        userArg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetAutoRedirect(
        id: crate::ctypes::c_int,
        enable: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetConnectTimeOut(
        id: crate::ctypes::c_int,
        usec: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetCookieEnabled(
        id: crate::ctypes::c_int,
        enable: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetCookieRecvCallback(
        id: crate::ctypes::c_int,
        cbfunc: SceHttpCookieRecvCallback,
        userArg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetCookieSendCallback(
        id: crate::ctypes::c_int,
        cbfunc: SceHttpCookieSendCallback,
        userArg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetRecvTimeOut(
        id: crate::ctypes::c_int,
        usec: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetRedirectCallback(
        id: crate::ctypes::c_int,
        cbfunc: SceHttpRedirectCallback,
        userArg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetRequestContentLength(
        id: crate::ctypes::c_int,
        contentLength: crate::ctypes::c_ulonglong,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetResolveRetry(
        id: crate::ctypes::c_int,
        retry: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetResolveTimeOut(
        id: crate::ctypes::c_int,
        usec: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetResponseHeaderMaxSize(
        id: crate::ctypes::c_int,
        headerSize: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpSetSendTimeOut(
        id: crate::ctypes::c_int,
        usec: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpTerm() -> crate::ctypes::c_int;
    pub fn sceHttpUriBuild(
        out: *mut crate::ctypes::c_char,
        require: *mut crate::ctypes::c_uint,
        prepare: crate::ctypes::c_uint,
        srcElement: *const SceHttpUriElement,
        option: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpUriEscape(
        out: *mut crate::ctypes::c_char,
        require: *mut crate::ctypes::c_uint,
        prepare: crate::ctypes::c_uint,
        in_: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpUriMerge(
        mergedUrl: *mut crate::ctypes::c_char,
        url: *const crate::ctypes::c_char,
        relativeUrl: *const crate::ctypes::c_char,
        require: *mut crate::ctypes::c_uint,
        prepare: crate::ctypes::c_uint,
        option: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpUriParse(
        out: *mut SceHttpUriElement,
        srcUrl: *const crate::ctypes::c_char,
        pool: *mut crate::ctypes::c_void,
        require: *mut crate::ctypes::c_uint,
        prepare: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpUriSweepPath(
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        srcSize: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpUriUnescape(
        out: *mut crate::ctypes::c_char,
        require: *mut crate::ctypes::c_uint,
        prepare: crate::ctypes::c_uint,
        in_: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpsDisableOption(sslFlags: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn sceHttpsEnableOption(sslFlags: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn sceHttpsFreeCaList(caList: *mut SceHttpsCaList) -> crate::ctypes::c_int;
    pub fn sceHttpsGetCaList(caList: *mut SceHttpsCaList) -> crate::ctypes::c_int;
    pub fn sceHttpsGetSslError(
        id: crate::ctypes::c_int,
        errNum: *mut crate::ctypes::c_int,
        detail: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpsLoadCert(
        caCertNum: crate::ctypes::c_int,
        caList: *mut *const SceHttpsData,
        cert: *const SceHttpsData,
        privKey: *const SceHttpsData,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpsSetSslCallback(
        id: crate::ctypes::c_int,
        cbfunc: SceHttpsCallback,
        userArg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceHttpsUnloadCert() -> crate::ctypes::c_int;
}
#[link(name = "SceI2cForDriver_stub", kind = "static")]
#[cfg(feature = "SceI2cForDriver_stub")]
extern "C" {
    pub fn ksceI2cInit(bus: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceI2cReset(bus: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceI2cSetDebugHandlers(
        bus: crate::ctypes::c_int,
        debug_handlers: *mut SceI2cDebugHandlers,
    ) -> crate::ctypes::c_int;
    pub fn ksceI2cTransferRead(
        bus: crate::ctypes::c_int,
        addr: crate::ctypes::c_uint,
        buffer: *mut crate::ctypes::c_uchar,
        size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceI2cTransferWrite(
        bus: crate::ctypes::c_int,
        addr: crate::ctypes::c_uint,
        buffer: *const crate::ctypes::c_uchar,
        size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceI2cTransferWriteRead(
        bus: crate::ctypes::c_int,
        write_addr: crate::ctypes::c_uint,
        write_buffer: *mut crate::ctypes::c_uchar,
        write_size: crate::ctypes::c_int,
        read_addr: crate::ctypes::c_uint,
        read_buffer: *mut crate::ctypes::c_uchar,
        read_size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceIdStorageForDriver_stub", kind = "static")]
#[cfg(feature = "SceIdStorageForDriver_stub")]
extern "C" {
    pub fn ksceIdStorageReadLeaf(
        leafnum: SceSize,
        buf: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceIdStorageWriteLeaf(
        leafnum: SceSize,
        buf: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceIftuForDriver_stub", kind = "static")]
#[cfg(feature = "SceIftuForDriver_stub")]
extern "C" {
    pub fn ksceIftuCsc(
        dst: *mut SceIftuFrameBuf,
        src: *mut SceIftuPlaneState,
        params: *mut SceIftuConvParams,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceIme_stub", kind = "static")]
#[cfg(feature = "SceIme_stub")]
extern "C" {
    pub fn sceImeClose() -> SceInt32;
    pub fn sceImeOpen(param: *const SceImeParam) -> SceInt32;
    pub fn sceImeSetCaret(caret: *const SceImeCaret) -> SceInt32;
    pub fn sceImeSetPreeditGeometry(preedit: *const SceImePreeditGeometry) -> SceInt32;
    pub fn sceImeSetText(text: *const SceWChar16, length: SceUInt32) -> SceInt32;
    pub fn sceImeUpdate() -> SceInt32;
}
#[link(name = "SceIncomingDialog_stub", kind = "static")]
#[cfg(feature = "SceIncomingDialog_stub")]
extern "C" {
    pub fn sceIncomingDialogClose() -> SceInt32;
    pub fn sceIncomingDialogFinish() -> SceInt32;
    pub fn sceIncomingDialogGetStatus() -> SceInt32;
    pub fn sceIncomingDialogInitialize(init_type: crate::ctypes::c_int) -> SceInt32;
    pub fn sceIncomingDialogOpen(dialogParam: *mut SceIncomingDialogParam) -> SceInt32;
    pub fn sceIncomingDialogSwitchToDialog() -> SceInt32;
}
#[link(name = "SceIntrmgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceIntrmgrForDriver_stub")]
extern "C" {
    pub fn ksceKernelClearIntrPending(intr_code: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceKernelDisableIntr(intr_code: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceKernelDisableSubIntr(
        intr_code: crate::ctypes::c_int,
        subintr_code: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelEnableIntr(intr_code: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceKernelEnableSubIntr(
        intr_code: crate::ctypes::c_int,
        subintr_code: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetIntrPriority(
        intr_code: crate::ctypes::c_int,
        priority: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetIntrTarget(
        intr_code: crate::ctypes::c_int,
        cpu_target_list: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelIsIntrAllowedInCurrentContext(
        intr_code: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelIsIntrPending(intr_code: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceKernelRegisterIntrHandler(
        intr_code: crate::ctypes::c_int,
        name: *const crate::ctypes::c_char,
        interrupt_type: crate::ctypes::c_int,
        handler: SceKernelIntrHandler,
        user_ctx: *mut crate::ctypes::c_void,
        priority: crate::ctypes::c_int,
        target_cpu: crate::ctypes::c_int,
        opt: *mut SceKernelIntrOptParam,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelRegisterSubIntrHandler(
        intr_code: crate::ctypes::c_int,
        subintr_code: crate::ctypes::c_int,
        name: *const crate::ctypes::c_char,
        handler: SceKernelSubIntrHandler,
        register_arg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelReleaseIntrHandler(intr_code: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceKernelReleaseSubIntrHandler(
        intr_code: crate::ctypes::c_int,
        subintr_code: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelResumeIntr(
        intr_code: crate::ctypes::c_int,
        enabled: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSetIntrPriority(
        intr_code: crate::ctypes::c_int,
        priority: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSetIntrTarget(
        intr_code: crate::ctypes::c_int,
        cpu_target_list: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSuspendIntr(
        intr_code: crate::ctypes::c_int,
        enabled: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelTriggerSGI(
        intr_code: crate::ctypes::c_int,
        target_list_filter: crate::ctypes::c_uint,
        cpu_target_list: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelTriggerSubIntr(
        intr_code: crate::ctypes::c_int,
        subintr_code: crate::ctypes::c_int,
        subintr_arg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceIntrmgrForKernel_stub", kind = "static")]
#[cfg(feature = "SceIntrmgrForKernel_stub")]
extern "C" {}
#[link(name = "SceIofilemgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceIofilemgrForDriver_stub")]
extern "C" {
    pub fn ksceIoChstat(
        file: *const crate::ctypes::c_char,
        stat: *mut SceIoStat,
        bits: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoChstatByFd(
        fd: SceUID,
        stat: *mut SceIoStat,
        bits: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoClose(fd: SceUID) -> crate::ctypes::c_int;
    pub fn ksceIoDclose(fd: SceUID) -> crate::ctypes::c_int;
    pub fn ksceIoDevctl(
        dev: *const crate::ctypes::c_char,
        cmd: crate::ctypes::c_uint,
        indata: *mut crate::ctypes::c_void,
        inlen: crate::ctypes::c_int,
        outdata: *mut crate::ctypes::c_void,
        outlen: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoDopen(dirname: *const crate::ctypes::c_char) -> SceUID;
    pub fn ksceIoDread(fd: SceUID, dir: *mut SceIoDirent) -> crate::ctypes::c_int;
    pub fn ksceIoGetFileInfo(
        fd: SceUID,
        pid: SceUID,
        info: *mut SceIofileInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoGetGUIDFdListForDebugger(
        vis_level: crate::ctypes::c_int,
        dst: *mut SceIoFdInfo,
        max_size: SceSize,
        res_size: *mut SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoGetPUIDFdListForDebugger(
        vis_level: crate::ctypes::c_int,
        dst: *mut SceIoFdInfo,
        max_size: SceSize,
        res_size: *mut SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoGetRemoteKPLSData(
        pid: SceUID,
        dst: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoGetstat(
        file: *const crate::ctypes::c_char,
        stat: *mut SceIoStat,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoGetstatByFd(fd: SceUID, stat: *mut SceIoStat) -> crate::ctypes::c_int;
    pub fn ksceIoLseek(fd: SceUID, offset: SceOff, whence: crate::ctypes::c_int) -> SceOff;
    pub fn ksceIoMkdir(dir: *const crate::ctypes::c_char, mode: SceMode) -> crate::ctypes::c_int;
    pub fn ksceIoMount(
        id: crate::ctypes::c_int,
        path: *const crate::ctypes::c_char,
        permission: crate::ctypes::c_int,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
        a6: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoOpen(
        file: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
        mode: SceMode,
    ) -> SceUID;
    pub fn ksceIoPread(
        fd: SceUID,
        data: *mut crate::ctypes::c_void,
        size: SceSize,
        offset: SceOff,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoPwrite(
        fd: SceUID,
        data: *const crate::ctypes::c_void,
        size: SceSize,
        offset: SceOff,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoRead(
        fd: SceUID,
        data: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoRemove(file: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn ksceIoRename(
        oldname: *const crate::ctypes::c_char,
        newname: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoRmdir(path: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn ksceIoSync(
        device: *const crate::ctypes::c_char,
        unk: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoSyncByFd(fd: SceUID) -> crate::ctypes::c_int;
    pub fn ksceIoUmount(
        id: crate::ctypes::c_int,
        a2: crate::ctypes::c_int,
        a3: crate::ctypes::c_int,
        a4: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceIoWrite(
        fd: SceUID,
        data: *const crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceIofilemgr_stub", kind = "static")]
#[cfg(feature = "SceIofilemgr_stub")]
extern "C" {
    pub fn sceIoCancel(fd: SceUID) -> crate::ctypes::c_int;
    pub fn sceIoClose(fd: SceUID) -> crate::ctypes::c_int;
    pub fn sceIoDclose(fd: SceUID) -> crate::ctypes::c_int;
    pub fn sceIoGetPriority(fd: SceUID) -> crate::ctypes::c_int;
    pub fn sceIoGetProcessDefaultPriority() -> crate::ctypes::c_int;
    pub fn sceIoGetThreadDefaultPriority() -> crate::ctypes::c_int;
    pub fn sceIoLseek32(
        fd: SceUID,
        offset: crate::ctypes::c_long,
        whence: crate::ctypes::c_int,
    ) -> crate::ctypes::c_long;
    pub fn sceIoRead(fd: SceUID, buf: *mut crate::ctypes::c_void, nbyte: SceSize) -> SceSSize;
    pub fn sceIoSetPriority(fd: SceUID, priority: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceIoSetProcessDefaultPriority(priority: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceIoSetThreadDefaultPriority(priority: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceIoSyncByFd(fd: SceUID, flag: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceIoWrite(fd: SceUID, buf: *const crate::ctypes::c_void, nbyte: SceSize) -> SceSSize;
}
#[link(name = "SceJpegArm_stub", kind = "static")]
#[cfg(feature = "SceJpegArm_stub")]
extern "C" {
    pub fn sceJpegArmDecodeMJpeg(
        pJpeg: *const SceUInt8,
        isize_: SceSize,
        decodeMode: SceInt,
        pRGBA: *mut crate::ctypes::c_void,
        osize: SceSize,
        pCoefBuffer: *mut crate::ctypes::c_void,
        coefBufferSize: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegArmDecodeMJpegYCbCr(
        pJpeg: *const SceUInt8,
        isize_: SceSize,
        decodeMode: SceInt,
        pYCbCr: *mut SceUInt8,
        osize: SceSize,
        pCoefBuffer: *mut crate::ctypes::c_void,
        coefBufferSize: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegArmGetOutputInfo(
        pJpeg: *const SceUInt8,
        isize_: SceSize,
        decodeMode: SceInt,
        outputFormat: SceInt,
        pOutputInfo: *mut SceJpegOutputInfo,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceJpegEncArm_stub", kind = "static")]
#[cfg(feature = "SceJpegEncArm_stub")]
extern "C" {
    pub fn sceJpegArmEncoderEncode(
        context: SceJpegArmEncoderContext,
        inBuffer: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegArmEncoderEnd(context: SceJpegArmEncoderContext) -> crate::ctypes::c_int;
    pub fn sceJpegArmEncoderGetContextSize() -> SceSize;
    pub fn sceJpegArmEncoderInit(
        context: SceJpegArmEncoderContext,
        inWidth: SceUInt16,
        inHeight: SceUInt16,
        pixelformat: SceJpegArmEncoderPixelFormat,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegArmEncoderSetCompressionRatio(
        context: SceJpegArmEncoderContext,
        ratio: SceUInt8,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegArmEncoderSetHeaderMode(
        context: SceJpegArmEncoderContext,
        mode: SceJpegArmEncoderHeaderMode,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegArmEncoderSetOutputAddr(
        context: SceJpegArmEncoderContext,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegArmEncoderSetValidRegion(
        context: SceJpegArmEncoderContext,
        regionWidth: SceUInt16,
        regionHeight: SceUInt16,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceJpegEnc_stub", kind = "static")]
#[cfg(feature = "SceJpegEnc_stub")]
extern "C" {
    pub fn sceJpegEncoderCsc(
        context: SceJpegEncoderContext,
        outBuffer: *mut crate::ctypes::c_void,
        inBuffer: *const crate::ctypes::c_void,
        inPitch: crate::ctypes::c_int,
        inPixelFormat: SceJpegEncoderPixelFormat,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegEncoderEncode(
        context: SceJpegEncoderContext,
        inBuffer: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegEncoderEnd(context: SceJpegEncoderContext) -> crate::ctypes::c_int;
    pub fn sceJpegEncoderGetContextSize() -> crate::ctypes::c_int;
    pub fn sceJpegEncoderInit(
        context: SceJpegEncoderContext,
        inWidth: crate::ctypes::c_int,
        inHeight: crate::ctypes::c_int,
        pixelformat: SceJpegEncoderPixelFormat,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegEncoderInitWithParam(
        context: SceJpegEncoderContext,
        initParam: *const SceJpegEncoderInitParam,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegEncoderSetCompressionRatio(
        context: SceJpegEncoderContext,
        ratio: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegEncoderSetHeaderMode(
        context: SceJpegEncoderContext,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegEncoderSetOutputAddr(
        context: SceJpegEncoderContext,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegEncoderSetValidRegion(
        context: SceJpegEncoderContext,
        inWidth: crate::ctypes::c_int,
        inHeight: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceJpeg_stub", kind = "static")]
#[cfg(feature = "SceJpeg_stub")]
extern "C" {
    pub fn sceJpegDecodeMJpegYCbCr(
        jpegData: *const SceUInt8,
        jpegSize: SceSize,
        mode: SceInt32,
        output: *mut SceUInt8,
        outputSize: SceSize,
        buffer: *mut crate::ctypes::c_void,
        bufferSize: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegFinishMJpeg() -> crate::ctypes::c_int;
    pub fn sceJpegGetOutputInfo(
        jpegData: *const SceUInt8,
        jpegSize: SceSize,
        format: SceInt32,
        mode: SceInt32,
        output: *mut SceJpegOutputInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceJpegInitMJpeg(decoderCount: SceInt32) -> crate::ctypes::c_int;
    pub fn sceJpegInitMJpegWithParam(params: *const SceJpegMJpegInitParam) -> crate::ctypes::c_int;
    pub fn sceJpegMJpegCsc(
        rgba: *mut SceUInt8,
        yuv: *const SceUInt8,
        yuvSize: SceSize,
        imageWidth: SceInt32,
        format: SceInt32,
        sampling: SceInt32,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceKernelBootimage_stub", kind = "static")]
#[cfg(feature = "SceKernelBootimage_stub")]
extern "C" {}
#[link(name = "SceKernelDmacMgr_stub", kind = "static")]
#[cfg(feature = "SceKernelDmacMgr_stub")]
extern "C" {
    pub fn sceDmacMemcpy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceDmacMemset(
        dst: *mut crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceKernelModulemgr_stub", kind = "static")]
#[cfg(feature = "SceKernelModulemgr_stub")]
extern "C" {
    pub fn _sceKernelCloseModule(
        modid: SceUID,
        args: SceSize,
        argp: *const crate::ctypes::c_void,
        flags: SceUInt32,
    ) -> SceUID;
    pub fn _sceKernelLoadModule(
        module_filename: *const crate::ctypes::c_char,
        flags: SceUInt32,
        option: *const SceKernelLoadModuleOption,
    ) -> SceUID;
    pub fn _sceKernelLoadStartModule(
        module_filename: *const crate::ctypes::c_char,
        args: SceSize,
        argp: *const crate::ctypes::c_void,
        flags: SceUInt32,
    ) -> SceUID;
    pub fn _sceKernelOpenModule(
        module_filename: *const crate::ctypes::c_char,
        args: SceSize,
        argp: *const crate::ctypes::c_void,
        flags: SceUInt32,
    ) -> SceUID;
    pub fn _sceKernelStopModule(
        uid: SceUID,
        args: SceSize,
        argp: *const crate::ctypes::c_void,
        flags: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn _sceKernelStopUnloadModule(
        uid: SceUID,
        args: SceSize,
        argp: *const crate::ctypes::c_void,
        flags: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn _sceKernelUnloadModule(
        uid: SceUID,
        flags: SceUInt32,
        option: *const SceKernelUnloadModuleOption,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetAllowedSdkVersionOnSystem() -> SceUInt32;
    pub fn sceKernelGetLibraryInfoByNID(
        modid: SceUID,
        libnid: SceNID,
        info: *mut SceKernelLibraryInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetModuleIdByAddr(addr: *mut crate::ctypes::c_void) -> SceUID;
    pub fn sceKernelGetModuleInfo(
        uid: SceUID,
        info: *mut SceKernelModuleInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetModuleList(
        type_: SceUInt8,
        uids: *mut SceUID,
        num: *mut SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetSystemSwVersion(
        version: *mut SceKernelSystemSwVersion,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelIsCalledFromSysModule(lr: *mut crate::ctypes::c_void) -> crate::ctypes::c_int;
}
#[link(name = "SceKernelSuspendForDriver_stub", kind = "static")]
#[cfg(feature = "SceKernelSuspendForDriver_stub")]
extern "C" {
    pub fn ksceKernelPowerTick(type_: SceKernelPowerTickType) -> crate::ctypes::c_int;
    pub fn ksceKernelRegisterSysEventHandler(
        name: *const crate::ctypes::c_char,
        handler: SceSysEventHandler,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceKernelThreadMgr_stub", kind = "static")]
#[cfg(feature = "SceKernelThreadMgr_stub")]
extern "C" {
    pub fn sceKernelCancelCallback(cb: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelChangeThreadCpuAffinityMask(
        thid: SceUID,
        mask: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelChangeThreadPriority(
        thid: SceUID,
        priority: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelCheckCallback() -> crate::ctypes::c_int;
    pub fn sceKernelClearEventFlag(
        evid: SceUID,
        bits: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelCloseCond(condId: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelCloseMutex(mutexid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelCloseRWLock(rwlock_id: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelCloseSema(semaid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelCreateCallback(
        name: *const crate::ctypes::c_char,
        attr: crate::ctypes::c_uint,
        func: SceKernelCallbackFunction,
        userData: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelDelayThread(delay: SceUInt) -> crate::ctypes::c_int;
    pub fn sceKernelDelayThreadCB(delay: SceUInt) -> crate::ctypes::c_int;
    pub fn sceKernelDeleteCallback(cb: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelDeleteCond(condId: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelDeleteEventFlag(evid: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelDeleteMsgPipe(uid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelDeleteMutex(mutexid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelDeleteRWLock(rwlock_id: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelDeleteSema(semaid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelDeleteThread(thid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelExitDeleteThread(status: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelExitThread(status: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelGetCallbackCount(cb: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelGetProcessId() -> SceUID;
    pub fn sceKernelGetSystemTimeWide() -> SceInt64;
    pub fn sceKernelGetThreadCpuAffinityMask(thid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelGetThreadStackFreeSize(thid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelGetThreadTLSAddr(
        thid: SceUID,
        key: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceKernelGetThreadmgrUIDClass(uid: SceUID) -> SceKernelIdListType;
    pub fn sceKernelNotifyCallback(cb: SceUID, arg2: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelOpenCond(name: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceKernelOpenMutex(name: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceKernelOpenRWLock(name: *const crate::ctypes::c_char) -> SceUID;
    pub fn sceKernelOpenSema(name: *const crate::ctypes::c_char) -> SceUID;
    pub fn sceKernelPollSema(semaid: SceUID, signal: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelSendSignal(thid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelSetEventFlag(evid: SceUID, bits: crate::ctypes::c_uint)
        -> crate::ctypes::c_int;
    pub fn sceKernelSignalCond(condId: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelSignalCondAll(condId: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelSignalCondTo(condId: SceUID, threadId: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelSignalSema(
        semaid: SceUID,
        signal: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelTryLockMutex(
        mutexid: SceUID,
        lockCount: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelTryLockReadRWLock(rwlock_id: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelTryLockWriteRWLock(rwlock_id: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelUnlockMutex(
        mutexid: SceUID,
        unlockCount: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelUnlockReadRWLock(rwlock_id: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelUnlockWriteRWLock(rwlock_id: SceUID) -> crate::ctypes::c_int;
}
#[link(name = "SceKernelUtilsForDriver_stub", kind = "static")]
#[cfg(feature = "SceKernelUtilsForDriver_stub")]
extern "C" {
    pub fn ksceAesDecrypt1(
        ctx: *mut SceAesContext,
        src: *const crate::ctypes::c_void,
        dst: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceAesDecrypt2(
        ctx: *mut SceAesContext,
        src: *const crate::ctypes::c_void,
        dst: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceAesEncrypt1(
        ctx: *mut SceAesContext,
        src: *const crate::ctypes::c_void,
        dst: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceAesEncrypt2(
        ctx: *mut SceAesContext,
        src: *const crate::ctypes::c_void,
        dst: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceAesInit1(
        ctx: *mut SceAesContext,
        blocksize: SceSize,
        keysize: SceSize,
        key: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceAesInit2(
        ctx: *mut SceAesContext,
        blocksize: SceSize,
        keysize: SceSize,
        key: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceAesInit3(
        ctx: *mut SceAesContext,
        blocksize: SceSize,
        keysize: SceSize,
        key: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceDeflateDecompress(
        dst: *mut crate::ctypes::c_void,
        dst_size: SceSize,
        src: *const crate::ctypes::c_void,
        next: *mut *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceDeflateDecompressPartial(
        dst: *mut crate::ctypes::c_void,
        dst_size: SceSize,
        src: *const crate::ctypes::c_void,
        next: *mut *const crate::ctypes::c_void,
        cbInfo: *mut SceDeflatePartialInputParam,
    ) -> crate::ctypes::c_int;
    pub fn ksceGzipDecompress(
        dst: *mut crate::ctypes::c_void,
        dst_size: SceSize,
        src: *const crate::ctypes::c_void,
        crc32: *mut u32,
    ) -> crate::ctypes::c_int;
    pub fn ksceGzipGetComment(src: *const crate::ctypes::c_void) -> *const crate::ctypes::c_char;
    pub fn ksceGzipGetCompressedData(
        src: *const crate::ctypes::c_void,
    ) -> *const crate::ctypes::c_void;
    pub fn ksceGzipGetInfo(
        src: *const crate::ctypes::c_void,
        extra: *mut *const crate::ctypes::c_void,
        name: *mut *const crate::ctypes::c_char,
        comment: *mut *const crate::ctypes::c_char,
        crc: *mut crate::ctypes::c_ushort,
        data: *mut *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceGzipGetName(src: *const crate::ctypes::c_void) -> *const crate::ctypes::c_char;
    pub fn ksceGzipIsValid(src: *const crate::ctypes::c_void) -> crate::ctypes::c_int;
    pub fn ksceHmacSha1Digest(
        key: *const crate::ctypes::c_void,
        key_len: SceSize,
        plain: *const crate::ctypes::c_void,
        len: SceSize,
        digest: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceHmacSha224Digest(
        key: *const crate::ctypes::c_void,
        key_len: SceSize,
        plain: *const crate::ctypes::c_void,
        len: SceSize,
        digest: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceHmacSha256Digest(
        key: *const crate::ctypes::c_void,
        key_len: SceSize,
        plain: *const crate::ctypes::c_void,
        len: SceSize,
        digest: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceSha1BlockInit(ctx: *mut SceSha1Context) -> crate::ctypes::c_int;
    pub fn ksceSha1BlockResult(
        ctx: *mut SceSha1Context,
        result: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceSha1BlockUpdate(
        ctx: *mut SceSha1Context,
        plain: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceSha1Digest(
        plain: *const crate::ctypes::c_void,
        len: SceSize,
        digest: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceSha224BlockInit(ctx: *mut SceSha224Context) -> crate::ctypes::c_int;
    pub fn ksceSha224BlockResult(
        ctx: *mut SceSha224Context,
        result: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceSha224BlockUpdate(
        ctx: *mut SceSha224Context,
        plain: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceSha224Digest(
        plain: *const crate::ctypes::c_void,
        len: SceSize,
        digest: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceSha256BlockInit(ctx: *mut SceSha256Context) -> crate::ctypes::c_int;
    pub fn ksceSha256BlockResult(
        ctx: *mut SceSha256Context,
        result: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceSha256BlockUpdate(
        ctx: *mut SceSha256Context,
        plain: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceSha256Digest(
        plain: *const crate::ctypes::c_void,
        len: SceSize,
        digest: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceZlibDecompress(
        dst: *mut crate::ctypes::c_void,
        dst_size: SceSize,
        src: *const crate::ctypes::c_void,
        adler32: *mut u32,
    ) -> crate::ctypes::c_int;
    pub fn ksceZlibGetCompressedData(
        src: *const crate::ctypes::c_void,
    ) -> *const crate::ctypes::c_void;
    pub fn ksceZlibGetInfo(
        src: *const crate::ctypes::c_void,
        cmf: *mut crate::ctypes::c_uchar,
        flg: *mut crate::ctypes::c_uchar,
        dictid: *mut crate::ctypes::c_uint,
        data: *mut *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceLcdForDriver_stub", kind = "static")]
#[cfg(feature = "SceLcdForDriver_stub")]
extern "C" {}
#[link(name = "SceLedForDriver_stub", kind = "static")]
#[cfg(feature = "SceLedForDriver_stub")]
extern "C" {}
#[link(name = "SceLibc_stub", kind = "static")]
#[cfg(feature = "SceLibc_stub")]
extern "C" {}
#[cfg(any(feature = "SceLibc_stub", feature = "SceSysclibForDriver_stub"))]
extern "C" {
    pub fn memchr(
        src: *const crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        n: crate::ctypes::c_uint,
    ) -> *mut crate::ctypes::c_void;
    pub fn memcmp(
        s1: *const crate::ctypes::c_void,
        s2: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn memcpy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
    ) -> *mut crate::ctypes::c_void;
    pub fn memmove(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
    ) -> *mut crate::ctypes::c_void;
    pub fn memset(
        dst: *mut crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        len: crate::ctypes::c_uint,
    ) -> *mut crate::ctypes::c_void;
    pub fn snprintf(
        dst: *mut crate::ctypes::c_char,
        max: crate::ctypes::c_uint,
        fmt: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
    pub fn strchr(
        src: *const crate::ctypes::c_char,
        ch: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_char;
    pub fn strcmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn strlen(s: *const crate::ctypes::c_char) -> crate::ctypes::c_uint;
    pub fn strncmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
        n: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn strncpy(
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        n: crate::ctypes::c_uint,
    ) -> *mut crate::ctypes::c_char;
    pub fn strstr(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
    ) -> *mut crate::ctypes::c_char;
    pub fn strtol(
        str_: *const crate::ctypes::c_char,
        endptr: *mut *mut crate::ctypes::c_char,
        base: crate::ctypes::c_int,
    ) -> crate::ctypes::c_long;
    pub fn strtoll(
        str_: *const crate::ctypes::c_char,
        endptr: *mut *mut crate::ctypes::c_char,
        base: crate::ctypes::c_int,
    ) -> crate::ctypes::c_longlong;
    pub fn strtoul(
        str_: *const crate::ctypes::c_char,
        endptr: *mut *mut crate::ctypes::c_char,
        base: crate::ctypes::c_int,
    ) -> crate::ctypes::c_ulong;
    pub fn tolower(ch: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn toupper(ch: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn vsnprintf(
        dst: *mut crate::ctypes::c_char,
        max: crate::ctypes::c_uint,
        fmt: *const crate::ctypes::c_char,
        arg: u32,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceLibDbg_stub", kind = "static")]
#[cfg(feature = "SceLibDbg_stub")]
extern "C" {
    pub fn sceDbgAssertionHandler(
        file: *const crate::ctypes::c_char,
        line: crate::ctypes::c_int,
        unk: crate::ctypes::c_int,
        component: *const crate::ctypes::c_char,
        msg: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
    pub fn sceDbgLoggingHandler(
        file: *const crate::ctypes::c_char,
        line: crate::ctypes::c_int,
        logLevel: SceDbgLogLevel,
        component: *const crate::ctypes::c_char,
        msg: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
    pub fn sceDbgSetBreakOnErrorState(breakOnError: SceBool);
    pub fn sceDbgSetBreakOnWarningState(breakOnWarning: SceBool);
    pub fn sceDbgSetMinimumLogLevel(level: SceDbgLogLevel);
}
#[link(name = "SceLibG729_stub", kind = "static")]
#[cfg(feature = "SceLibG729_stub")]
extern "C" {}
#[link(name = "SceLibJson_stub", kind = "static")]
#[cfg(feature = "SceLibJson_stub")]
extern "C" {}
#[link(name = "SceLibKernel_stub", kind = "static")]
#[cfg(feature = "SceLibKernel_stub")]
extern "C" {
    pub fn sceClibAbort();
    pub fn sceClibLookCtypeTable(ch: crate::ctypes::c_char) -> crate::ctypes::c_char;
    pub fn sceClibMemchr(
        src: *const crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceClibMemcmp(
        s1: *const crate::ctypes::c_void,
        s2: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceClibMemcpy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceClibMemcpy_safe(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceClibMemmove(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceClibMemset(
        dst: *mut crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceClibMspaceCalloc(
        mspace: SceClibMspace,
        num: SceSize,
        size: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceClibMspaceCreate(
        memblock: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> SceClibMspace;
    pub fn sceClibMspaceDestroy(mspace: SceClibMspace);
    pub fn sceClibMspaceFree(mspace: SceClibMspace, ptr: *mut crate::ctypes::c_void);
    pub fn sceClibMspaceIsHeapEmpty(mspace: SceClibMspace) -> SceBool;
    pub fn sceClibMspaceMalloc(mspace: SceClibMspace, size: SceSize) -> *mut crate::ctypes::c_void;
    pub fn sceClibMspaceMallocStats(mspace: SceClibMspace, stats: *mut SceClibMspaceStats);
    pub fn sceClibMspaceMallocStatsFast(mspace: SceClibMspace, stats: *mut SceClibMspaceStats);
    pub fn sceClibMspaceMallocUsableSize(ptr: *mut crate::ctypes::c_void) -> SceSize;
    pub fn sceClibMspaceMemalign(
        mspace: SceClibMspace,
        alignment: SceSize,
        size: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceClibMspaceRealloc(
        mspace: SceClibMspace,
        ptr: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceClibMspaceReallocalign(
        mspace: SceClibMspace,
        ptr: *mut crate::ctypes::c_void,
        size: SceSize,
        alignment: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sceClibPrintf(fmt: *const crate::ctypes::c_char, ...) -> crate::ctypes::c_int;
    pub fn sceClibSnprintf(
        dst: *mut crate::ctypes::c_char,
        dst_max_size: SceSize,
        fmt: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
    pub fn sceClibStrchr(
        s: *const crate::ctypes::c_char,
        ch: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_char;
    pub fn sceClibStrcmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceClibStrncasecmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceClibStrncat(
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> *mut crate::ctypes::c_char;
    pub fn sceClibStrncmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceClibStrncpy(
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> *mut crate::ctypes::c_char;
    pub fn sceClibStrnlen(s1: *const crate::ctypes::c_char, max_len: SceSize) -> SceSize;
    pub fn sceClibStrrchr(
        src: *const crate::ctypes::c_char,
        ch: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_char;
    pub fn sceClibStrstr(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
    ) -> *mut crate::ctypes::c_char;
    pub fn sceClibTolower(ch: crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceClibToupper(ch: crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceClibVsnprintf(
        dst: *mut crate::ctypes::c_char,
        dst_max_size: SceSize,
        fmt: *const crate::ctypes::c_char,
        args: va_list,
    ) -> crate::ctypes::c_int;
    pub fn sceIoChstat(
        file: *const crate::ctypes::c_char,
        stat: *mut SceIoStat,
        bits: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceIoChstatByFd(
        fd: SceUID,
        buf: *const SceIoStat,
        cbit: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceIoDevctl(
        dev: *const crate::ctypes::c_char,
        cmd: crate::ctypes::c_uint,
        indata: *mut crate::ctypes::c_void,
        inlen: crate::ctypes::c_int,
        outdata: *mut crate::ctypes::c_void,
        outlen: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceIoDopen(dirname: *const crate::ctypes::c_char) -> SceUID;
    pub fn sceIoDread(fd: SceUID, dir: *mut SceIoDirent) -> crate::ctypes::c_int;
    pub fn sceIoGetstat(
        file: *const crate::ctypes::c_char,
        stat: *mut SceIoStat,
    ) -> crate::ctypes::c_int;
    pub fn sceIoGetstatByFd(fd: SceUID, stat: *mut SceIoStat) -> crate::ctypes::c_int;
    pub fn sceIoIoctl(
        fd: SceUID,
        cmd: crate::ctypes::c_uint,
        indata: *mut crate::ctypes::c_void,
        inlen: crate::ctypes::c_int,
        outdata: *mut crate::ctypes::c_void,
        outlen: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceIoIoctlAsync(
        fd: SceUID,
        cmd: crate::ctypes::c_uint,
        indata: *mut crate::ctypes::c_void,
        inlen: crate::ctypes::c_int,
        outdata: *mut crate::ctypes::c_void,
        outlen: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceIoLseek(fd: SceUID, offset: SceOff, whence: crate::ctypes::c_int) -> SceOff;
    pub fn sceIoMkdir(dir: *const crate::ctypes::c_char, mode: SceMode) -> crate::ctypes::c_int;
    pub fn sceIoOpen(
        file: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
        mode: SceMode,
    ) -> SceUID;
    pub fn sceIoPread(
        fd: SceUID,
        data: *mut crate::ctypes::c_void,
        size: SceSize,
        offset: SceOff,
    ) -> crate::ctypes::c_int;
    pub fn sceIoPwrite(
        fd: SceUID,
        data: *const crate::ctypes::c_void,
        size: SceSize,
        offset: SceOff,
    ) -> crate::ctypes::c_int;
    pub fn sceIoRemove(file: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceIoRename(
        oldname: *const crate::ctypes::c_char,
        newname: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceIoRmdir(path: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceIoSync(
        device: *const crate::ctypes::c_char,
        unk: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelAtomicAddAndGet16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicAddAndGet32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicAddAndGet64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicAddAndGet8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicAddUnless16(
        store: *mut SceInt16,
        value: SceInt16,
        cmpv: SceInt16,
    ) -> SceBool;
    pub fn sceKernelAtomicAddUnless32(
        store: *mut SceInt32,
        value: SceInt32,
        cmpv: SceInt32,
    ) -> SceBool;
    pub fn sceKernelAtomicAddUnless64(
        store: *mut SceInt64,
        value: SceInt64,
        cmpv: SceInt64,
    ) -> SceBool;
    pub fn sceKernelAtomicAddUnless8(store: *mut SceInt8, value: SceInt8, cmpv: SceInt8)
        -> SceBool;
    pub fn sceKernelAtomicAndAndGet16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicAndAndGet32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicAndAndGet64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicAndAndGet8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicClearAndGet16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicClearAndGet32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicClearAndGet64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicClearAndGet8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicClearMask16(store: *mut SceInt16, value: SceInt16);
    pub fn sceKernelAtomicClearMask32(store: *mut SceInt32, value: SceInt32);
    pub fn sceKernelAtomicClearMask64(store: *mut SceInt64, value: SceInt64);
    pub fn sceKernelAtomicClearMask8(store: *mut SceInt8, value: SceInt8);
    pub fn sceKernelAtomicCompareAndSet16(
        store: *mut SceInt16,
        value: SceInt16,
        new_value: SceInt16,
    ) -> SceInt16;
    pub fn sceKernelAtomicCompareAndSet32(
        store: *mut SceInt32,
        value: SceInt32,
        new_value: SceInt32,
    ) -> SceInt32;
    pub fn sceKernelAtomicCompareAndSet64(
        store: *mut SceInt64,
        value: SceInt64,
        new_value: SceInt64,
    ) -> SceInt64;
    pub fn sceKernelAtomicCompareAndSet8(
        store: *mut SceInt8,
        value: SceInt8,
        new_value: SceInt8,
    ) -> SceInt8;
    pub fn sceKernelAtomicDecIfPositive16(store: *mut SceInt16) -> SceInt16;
    pub fn sceKernelAtomicDecIfPositive32(store: *mut SceInt32) -> SceInt32;
    pub fn sceKernelAtomicDecIfPositive64(store: *mut SceInt64) -> SceInt64;
    pub fn sceKernelAtomicDecIfPositive8(store: *mut SceInt8) -> SceInt8;
    pub fn sceKernelAtomicGetAndAdd16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicGetAndAdd32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicGetAndAdd64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicGetAndAdd8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicGetAndAnd16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicGetAndAnd32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicGetAndAnd64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicGetAndAnd8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicGetAndClear16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicGetAndClear32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicGetAndClear64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicGetAndClear8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicGetAndOr16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicGetAndOr32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicGetAndOr64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicGetAndOr8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicGetAndSet16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicGetAndSet32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicGetAndSet64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicGetAndSet8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicGetAndSub16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicGetAndSub32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicGetAndSub64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicGetAndSub8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicGetAndXor16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicGetAndXor32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicGetAndXor64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicGetAndXor8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicOrAndGet16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicOrAndGet32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicOrAndGet64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicOrAndGet8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicSet16(store: *mut SceInt16, value: SceInt16);
    pub fn sceKernelAtomicSet32(store: *mut SceInt32, value: SceInt32);
    pub fn sceKernelAtomicSet64(store: *mut SceInt64, value: SceInt64);
    pub fn sceKernelAtomicSet8(store: *mut SceInt8, value: SceInt8);
    pub fn sceKernelAtomicSubAndGet16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicSubAndGet32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicSubAndGet64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicSubAndGet8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelAtomicXorAndGet16(store: *mut SceInt16, value: SceInt16) -> SceInt16;
    pub fn sceKernelAtomicXorAndGet32(store: *mut SceInt32, value: SceInt32) -> SceInt32;
    pub fn sceKernelAtomicXorAndGet64(store: *mut SceInt64, value: SceInt64) -> SceInt64;
    pub fn sceKernelAtomicXorAndGet8(store: *mut SceInt8, value: SceInt8) -> SceInt8;
    pub fn sceKernelCancelMsgPipe(
        uid: SceUID,
        psend: *mut crate::ctypes::c_int,
        precv: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelCancelMutex(
        mutexid: SceUID,
        newCount: crate::ctypes::c_int,
        numWaitThreads: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelCancelSema(
        semaid: SceUID,
        setCount: crate::ctypes::c_int,
        numWaitThreads: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelChangeCurrentThreadAttr(
        clearAttr: SceUInt,
        setAttr: SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelCheckThreadStack() -> crate::ctypes::c_int;
    pub fn sceKernelCreateCond(
        name: *const crate::ctypes::c_char,
        attr: SceUInt,
        mutexId: SceUID,
        option: *const SceKernelCondOptParam,
    ) -> SceUID;
    pub fn sceKernelCreateEventFlag(
        name: *const crate::ctypes::c_char,
        attr: crate::ctypes::c_int,
        bits: crate::ctypes::c_int,
        opt: *mut SceKernelEventFlagOptParam,
    ) -> SceUID;
    pub fn sceKernelCreateLwCond(
        pWork: *mut SceKernelLwCondWork,
        pName: *const crate::ctypes::c_char,
        attr: crate::ctypes::c_uint,
        pLwMutex: *mut SceKernelLwMutexWork,
        pOptParam: *const SceKernelLwCondOptParam,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelCreateLwMutex(
        pWork: *mut SceKernelLwMutexWork,
        pName: *const crate::ctypes::c_char,
        attr: crate::ctypes::c_uint,
        initCount: crate::ctypes::c_int,
        pOptParam: *const SceKernelLwMutexOptParam,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelCreateMsgPipe(
        name: *const crate::ctypes::c_char,
        type_: crate::ctypes::c_int,
        attr: crate::ctypes::c_int,
        bufSize: crate::ctypes::c_uint,
        opt: *mut crate::ctypes::c_void,
    ) -> SceUID;
    pub fn sceKernelCreateMutex(
        name: *const crate::ctypes::c_char,
        attr: SceUInt,
        initCount: crate::ctypes::c_int,
        option: *mut SceKernelMutexOptParam,
    ) -> SceUID;
    pub fn sceKernelCreateRWLock(
        name: *const crate::ctypes::c_char,
        attr: SceUInt32,
        opt_param: *const SceKernelRWLockOptParam,
    ) -> SceUID;
    pub fn sceKernelCreateSema(
        name: *const crate::ctypes::c_char,
        attr: SceUInt,
        initVal: crate::ctypes::c_int,
        maxVal: crate::ctypes::c_int,
        option: *mut SceKernelSemaOptParam,
    ) -> SceUID;
    pub fn sceKernelCreateThread(
        name: *const crate::ctypes::c_char,
        entry: SceKernelThreadEntry,
        initPriority: crate::ctypes::c_int,
        stackSize: SceSize,
        attr: SceUInt,
        cpuAffinityMask: crate::ctypes::c_int,
        option: *const SceKernelThreadOptParam,
    ) -> SceUID;
    pub fn sceKernelDeleteLwCond(pWork: *mut SceKernelLwCondWork) -> crate::ctypes::c_int;
    pub fn sceKernelDeleteLwMutex(pWork: *mut SceKernelLwMutexWork) -> crate::ctypes::c_int;
    pub fn sceKernelExitProcess(res: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelGetCallbackInfo(
        cb: SceUID,
        infop: *mut SceKernelCallbackInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetEventFlagInfo(
        event: SceUID,
        info: *mut SceKernelEventFlagInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetMsgPipeInfo(
        uid: SceUID,
        info: *mut SceKernelMppInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetMutexInfo(
        mutexid: SceUID,
        info: *mut SceKernelMutexInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetOpenPsId(id: *mut SceKernelOpenPsId) -> crate::ctypes::c_int;
    pub fn sceKernelGetProcessTime(pSysClock: *mut SceKernelSysClock) -> crate::ctypes::c_int;
    pub fn sceKernelGetProcessTimeLow() -> SceUInt32;
    pub fn sceKernelGetProcessTimeWide() -> SceUInt64;
    pub fn sceKernelGetRWLockInfo(
        rwlock_id: SceUID,
        info: *mut SceKernelRWLockInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetRandomNumber(
        output: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetSemaInfo(
        semaid: SceUID,
        info: *mut SceKernelSemaInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetSystemInfo(info: *mut SceKernelSystemInfo) -> crate::ctypes::c_int;
    pub fn sceKernelGetTLSAddr(key: crate::ctypes::c_int) -> *mut crate::ctypes::c_void;
    pub fn sceKernelGetThreadCurrentPriority() -> crate::ctypes::c_int;
    pub fn sceKernelGetThreadExitStatus(
        thid: SceUID,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetThreadId() -> crate::ctypes::c_int;
    pub fn sceKernelGetThreadInfo(
        thid: SceUID,
        info: *mut SceKernelThreadInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetThreadRunStatus(
        thid: SceUID,
        status: *mut SceKernelThreadRunStatus,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelLoadModule(
        path: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
    ) -> SceUID;
    pub fn sceKernelLoadStartModule(
        path: *const crate::ctypes::c_char,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
        status: *mut crate::ctypes::c_int,
    ) -> SceUID;
    pub fn sceKernelLockLwMutex(
        pWork: *mut SceKernelLwMutexWork,
        lockCount: crate::ctypes::c_int,
        pTimeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelLockMutex(
        mutexid: SceUID,
        lockCount: crate::ctypes::c_int,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelLockMutexCB(
        mutexid: SceUID,
        lockCount: crate::ctypes::c_int,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelLockReadRWLock(
        rwlock_id: SceUID,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelLockReadRWLockCB(
        rwlock_id: SceUID,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelLockWriteRWLock(
        rwlock_id: SceUID,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelLockWriteRWLockCB(
        rwlock_id: SceUID,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelPollEventFlag(
        evid: crate::ctypes::c_int,
        bits: crate::ctypes::c_uint,
        wait: crate::ctypes::c_uint,
        outBits: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelReceiveMsgPipe(
        uid: SceUID,
        message: *mut crate::ctypes::c_void,
        size: SceSize,
        unk1: crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_void,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelReceiveMsgPipeCB(
        uid: SceUID,
        message: *mut crate::ctypes::c_void,
        size: SceSize,
        unk1: crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_void,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelSendMsgPipe(
        uid: SceUID,
        message: *mut crate::ctypes::c_void,
        size: crate::ctypes::c_uint,
        unk1: crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_void,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelSendMsgPipeCB(
        uid: SceUID,
        message: *mut crate::ctypes::c_void,
        size: crate::ctypes::c_uint,
        unk1: crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_void,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelSignalLwCond(pWork: *mut SceKernelLwCondWork) -> crate::ctypes::c_int;
    pub fn sceKernelSignalLwCondAll(pWork: *mut SceKernelLwCondWork) -> crate::ctypes::c_int;
    pub fn sceKernelSignalLwCondTo(
        pWork: *mut SceKernelLwCondWork,
        threadId: SceUID,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelStartModule(
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut crate::ctypes::c_void,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelStartThread(
        thid: SceUID,
        arglen: SceSize,
        argp: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelStopModule(
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut crate::ctypes::c_void,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelStopUnloadModule(
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelTryLockLwMutex(
        pWork: *mut SceKernelLwMutexWork,
        lockCount: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelTryReceiveMsgPipe(
        uid: SceUID,
        message: *mut crate::ctypes::c_void,
        size: SceSize,
        unk1: crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelTrySendMsgPipe(
        uid: SceUID,
        message: *mut crate::ctypes::c_void,
        size: SceSize,
        unk1: crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelUnloadModule(
        modid: SceUID,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelUnlockLwMutex(
        pWork: *mut SceKernelLwMutexWork,
        unlockCount: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelWaitCond(
        condId: SceUID,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelWaitCondCB(
        condId: SceUID,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelWaitEventFlag(
        evid: crate::ctypes::c_int,
        bits: crate::ctypes::c_uint,
        wait: crate::ctypes::c_uint,
        outBits: *mut crate::ctypes::c_uint,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelWaitEventFlagCB(
        evid: crate::ctypes::c_int,
        bits: crate::ctypes::c_uint,
        wait: crate::ctypes::c_uint,
        outBits: *mut crate::ctypes::c_uint,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelWaitLwCond(
        pWork: *mut SceKernelLwCondWork,
        pTimeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelWaitSema(
        semaid: SceUID,
        signal: crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelWaitSemaCB(
        semaid: SceUID,
        signal: crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelWaitSignal(
        unk0: SceUInt32,
        delay: SceUInt32,
        timeout: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelWaitThreadEnd(
        thid: SceUID,
        stat: *mut crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelWaitThreadEndCB(
        thid: SceUID,
        stat: *mut crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn sceSblACMgrIsGameProgram(result: *mut SceBool) -> SceInt32;
}
#[cfg(any(feature = "SceLibKernel_stub", feature = "SceSysclibForDriver_stub"))]
extern "C" {
    pub fn __stack_chk_fail() -> !;
}
#[link(name = "SceLibMonoBridge_stub", kind = "static")]
#[cfg(feature = "SceLibMonoBridge_stub")]
extern "C" {
    pub fn pss_code_mem_alloc(arg1: *mut SceSize) -> *mut crate::ctypes::c_void;
    pub fn pss_code_mem_flush_icache(arg1: *const crate::ctypes::c_void, arg2: SceSize);
    pub fn pss_code_mem_lock();
    pub fn pss_code_mem_unlock();
    pub fn pss_crypto_close(handle: *mut ScePssCryptoHandle) -> crate::ctypes::c_int;
    pub fn pss_crypto_open(
        handle: *mut ScePssCryptoHandle,
        path: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn pss_crypto_read(
        handle: *mut ScePssCryptoHandle,
        mode: *mut crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_char;
}
#[link(name = "SceLibMono_stub", kind = "static")]
#[cfg(feature = "SceLibMono_stub")]
extern "C" {}
#[link(name = "SceLibMp4Recorder_stub", kind = "static")]
#[cfg(feature = "SceLibMp4Recorder_stub")]
extern "C" {}
#[link(name = "SceLibMtp_stub", kind = "static")]
#[cfg(feature = "SceLibMtp_stub")]
extern "C" {}
#[link(name = "SceLibXml_stub", kind = "static")]
#[cfg(feature = "SceLibXml_stub")]
extern "C" {}
#[link(name = "SceLiveArea_stub", kind = "static")]
#[cfg(feature = "SceLiveArea_stub")]
extern "C" {}
#[link(name = "SceLocationExtension_stub", kind = "static")]
#[cfg(feature = "SceLocationExtension_stub")]
extern "C" {}
#[link(name = "SceLocation_stub", kind = "static")]
#[cfg(feature = "SceLocation_stub")]
extern "C" {
    pub fn sceLocationCancelGetLocation(handle: SceLocationHandle) -> SceInt32;
    pub fn sceLocationClose(handle: SceLocationHandle) -> SceInt32;
    pub fn sceLocationConfirm(handle: SceLocationHandle) -> SceInt32;
    pub fn sceLocationConfirmAbort(handle: SceLocationHandle) -> SceInt32;
    pub fn sceLocationConfirmGetResult(
        handle: SceLocationHandle,
        result: *mut SceLocationDialogResult,
    ) -> SceInt32;
    pub fn sceLocationConfirmGetStatus(
        handle: SceLocationHandle,
        status: *mut SceLocationDialogStatus,
    ) -> SceInt32;
    pub fn sceLocationGetHeading(
        handle: SceLocationHandle,
        headingInfo: *mut SceLocationHeadingInfo,
    ) -> SceInt32;
    pub fn sceLocationGetLocation(
        handle: SceLocationHandle,
        locationInfo: *mut SceLocationLocationInfo,
    ) -> SceInt32;
    pub fn sceLocationGetMethod(
        handle: SceLocationHandle,
        locateMethod: *mut SceLocationLocationMethod,
        headingMethod: *mut SceLocationHeadingMethod,
    ) -> SceInt32;
    pub fn sceLocationGetPermission(
        handle: SceLocationHandle,
        info: *mut SceLocationPermissionInfo,
    ) -> SceInt32;
    pub fn sceLocationOpen(
        handle: *mut SceLocationHandle,
        locateMethod: SceLocationLocationMethod,
        headingMethod: SceLocationHeadingMethod,
    ) -> SceInt32;
    pub fn sceLocationReopen(
        handle: SceLocationHandle,
        locateMethod: SceLocationLocationMethod,
        headingMethod: SceLocationHeadingMethod,
    ) -> SceInt32;
    pub fn sceLocationSetGpsEmulationFile(filename: *mut crate::ctypes::c_uchar) -> SceInt32;
    pub fn sceLocationStartHeadingCallback(
        handle: SceLocationHandle,
        difference: SceUInt32,
        callback: SceLocationHeadingInfoCallback,
        userdata: *mut crate::ctypes::c_void,
    ) -> SceInt32;
    pub fn sceLocationStartLocationCallback(
        handle: SceLocationHandle,
        distance: SceUInt32,
        callback: SceLocationLocationInfoCallback,
        userdata: *mut crate::ctypes::c_void,
    ) -> SceInt32;
    pub fn sceLocationStopHeadingCallback(handle: SceLocationHandle) -> SceInt32;
    pub fn sceLocationStopLocationCallback(handle: SceLocationHandle) -> SceInt32;
}
#[link(name = "SceLsdb_stub", kind = "static")]
#[cfg(feature = "SceLsdb_stub")]
extern "C" {}
#[link(name = "SceModulemgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceModulemgrForDriver_stub")]
extern "C" {
    pub fn ksceKernelGetSystemSwVersion(data: *mut SceKernelFwInfo) -> crate::ctypes::c_int;
    pub fn ksceKernelLoadModule(
        path: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
    ) -> SceUID;
    pub fn ksceKernelLoadStartModule(
        path: *const crate::ctypes::c_char,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
        status: *mut crate::ctypes::c_int,
    ) -> SceUID;
    pub fn ksceKernelLoadStartModuleForPid(
        pid: SceUID,
        path: *const crate::ctypes::c_char,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
        status: *mut crate::ctypes::c_int,
    ) -> SceUID;
    pub fn ksceKernelLoadStartSharedModuleForPid(
        pid: SceUID,
        path: *const crate::ctypes::c_char,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
        status: *mut crate::ctypes::c_int,
    ) -> SceUID;
    pub fn ksceKernelSearchModuleByName(module_name: *const crate::ctypes::c_char) -> SceUID;
    pub fn ksceKernelStartModule(
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelStopModule(
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelStopUnloadModule(
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelStopUnloadModuleForPid(
        pid: SceUID,
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelStopUnloadSharedModuleForPid(
        pid: SceUID,
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelUnloadModule(
        modid: SceUID,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceModulemgrForKernel_363_stub", kind = "static")]
#[cfg(feature = "SceModulemgrForKernel_363_stub")]
extern "C" {}
#[cfg(any(
    feature = "SceModulemgrForKernel_363_stub",
    feature = "SceModulemgrForKernel_stub"
))]
extern "C" {
    pub fn ksceKernelGetModuleInfo(
        pid: SceUID,
        modid: SceUID,
        info: *mut SceKernelModuleInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetModuleList(
        pid: SceUID,
        flags1: crate::ctypes::c_int,
        flags2: crate::ctypes::c_int,
        modids: *mut SceUID,
        num: *mut SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelLoadModuleForPid(
        pid: SceUID,
        path: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
    ) -> SceUID;
    pub fn ksceKernelMountBootfs(
        bootImagePath: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelStartModuleForPid(
        pid: SceUID,
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelStopModuleForPid(
        pid: SceUID,
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelUmountBootfs() -> crate::ctypes::c_int;
    pub fn ksceKernelUnloadModuleForPid(
        pid: SceUID,
        modid: SceUID,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceModulemgrForKernel_stub", kind = "static")]
#[cfg(feature = "SceModulemgrForKernel_stub")]
extern "C" {
    pub fn ksceKernelGetLibraryInfoForDebugger(
        pid: SceUID,
        library_id: SceUID,
        info: *mut SceKernelModuleLibraryInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetModuleCB(
        modid: SceUID,
        info: *mut *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetModuleCBByAddr(
        pid: SceUID,
        module_addr: *const crate::ctypes::c_void,
        info: *mut *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetModuleIdByAddrForDebugger(
        pid: SceUID,
        module_addr: *const crate::ctypes::c_void,
    ) -> SceUID;
    pub fn ksceKernelGetModuleIdByPid(pid: SceUID) -> SceUID;
    pub fn ksceKernelGetModuleInfoForDebugger(
        pid: SceUID,
        infolists: *mut SceKernelModuleListInfo,
        num: *mut SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetModuleInfoMinByAddr(
        pid: SceUID,
        module_addr: *const crate::ctypes::c_void,
        module_nid: *mut u32,
        program_text_addr: *mut *const crate::ctypes::c_void,
        module_name: *mut SceKernelModuleName,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetModulePath(
        modid: SceUID,
        path: *mut crate::ctypes::c_char,
        pathlen: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelRegisterModulesAfterBoot();
    pub fn ksceKernelRegisterSyscall(syscall_id: SceSize, func: *const crate::ctypes::c_void);
}
#[link(name = "SceMotionDevForDriver_stub", kind = "static")]
#[cfg(feature = "SceMotionDevForDriver_stub")]
extern "C" {}
#[link(name = "SceMotionDev_stub", kind = "static")]
#[cfg(feature = "SceMotionDev_stub")]
extern "C" {
    pub fn sceMotionDevGetAccCalibData(data: *mut SceMotionDevAccCalibData)
        -> crate::ctypes::c_int;
    pub fn sceMotionDevGetAccCalibData2(
        port: crate::ctypes::c_int,
        data: *mut SceMotionDevAccCalibData,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetCalibrationData(
        block_id: SceUInt32,
        data: *mut SceMotionDevCalibrationData,
        data_num: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetCalibrationHeader(
        block_id: SceUInt32,
        calib_header: *mut SceMotionDevCalibrationHeader,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetCurrentMagnCalibData(
        data: *mut SceMotionDevMagnCalibData,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetCurrentMagnStabilityLevel(level: *mut SceUInt32) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetDeviceInfo(
        device_info: *mut SceMotionDevDeviceInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetDeviceLocation(
        location: *mut SceMotionDevDeviceLocation,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetFactoryMagnCalibData(
        data: *mut SceMotionDevMagnCalibData,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetGyroBias(bias: *mut SceMotionDevGyroBiasData) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetGyroBias2(
        port: crate::ctypes::c_int,
        bias: *mut SceMotionDevGyroBiasData,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetGyroCalibData(
        data: *mut SceMotionDevGyroCalibData,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetGyroCalibData2(
        port: crate::ctypes::c_int,
        data: *mut SceMotionDevGyroCalibData,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevGetMeasMode(mode_info: *mut SceMotionDevModeInfo) -> crate::ctypes::c_int;
    pub fn sceMotionDevIsReady() -> crate::ctypes::c_int;
    pub fn sceMotionDevMagnSamplingStart() -> crate::ctypes::c_int;
    pub fn sceMotionDevMagnSamplingStop() -> crate::ctypes::c_int;
    pub fn sceMotionDevRead(
        data: *mut SceMotionDevData,
        data_num: crate::ctypes::c_int,
        info: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevRead2(
        port: crate::ctypes::c_int,
        data: *mut SceMotionDevData,
        data_num: crate::ctypes::c_int,
        info: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevReadForMagnCalib(
        data: *mut SceMotionDevData,
        data_num: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevSamplingStart() -> crate::ctypes::c_int;
    pub fn sceMotionDevSamplingStart2(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceMotionDevSamplingStop() -> crate::ctypes::c_int;
    pub fn sceMotionDevSamplingStop2(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceMotionDevSetSamplingMode(mode: SceUInt32) -> crate::ctypes::c_int;
    pub fn sceMotionDevUpdateMagnCalibData(
        data: *const SceMotionDevMagnCalibData,
        tag: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionDevUpdateMagnStabilityLevel(level: SceUInt32) -> crate::ctypes::c_int;
}
#[link(name = "SceMotion_stub", kind = "static")]
#[cfg(feature = "SceMotion_stub")]
extern "C" {
    pub fn sceMotionGetAngleThreshold() -> f32;
    pub fn sceMotionGetBasicOrientation(basicOrientation: *mut SceFVector3)
        -> crate::ctypes::c_int;
    pub fn sceMotionGetDeadband() -> crate::ctypes::c_int;
    pub fn sceMotionGetDeviceLocation(
        deviceLocation: *mut SceMotionDeviceLocation,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionGetGyroBiasCorrection() -> crate::ctypes::c_int;
    pub fn sceMotionGetMagnetometerState() -> crate::ctypes::c_int;
    pub fn sceMotionGetSensorState(
        sensorState: *mut SceMotionSensorState,
        numRecords: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceMotionGetState(motionState: *mut SceMotionState) -> crate::ctypes::c_int;
    pub fn sceMotionGetTiltCorrection() -> crate::ctypes::c_int;
    pub fn sceMotionMagnetometerOff() -> crate::ctypes::c_int;
    pub fn sceMotionMagnetometerOn() -> crate::ctypes::c_int;
    pub fn sceMotionReset() -> crate::ctypes::c_int;
    pub fn sceMotionRotateYaw(radians: f32) -> crate::ctypes::c_int;
    pub fn sceMotionSetAngleThreshold(angle: f32) -> crate::ctypes::c_int;
    pub fn sceMotionSetDeadband(setValue: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceMotionSetGyroBiasCorrection(setValue: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceMotionSetTiltCorrection(setValue: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceMotionStartSampling() -> crate::ctypes::c_int;
    pub fn sceMotionStopSampling() -> crate::ctypes::c_int;
}
#[link(name = "SceMsifForDriver_stub", kind = "static")]
#[cfg(feature = "SceMsifForDriver_stub")]
extern "C" {
    pub fn ksceMsifGetMsInfo(info: *mut SceMsInfo) -> crate::ctypes::c_int;
}
#[link(name = "SceMtpIfDriver_stub", kind = "static")]
#[cfg(feature = "SceMtpIfDriver_stub")]
extern "C" {
    pub fn sceMtpIfStartDriver(flags: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceMtpIfStopDriver(flags: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
#[link(name = "SceMusicExport_stub", kind = "static")]
#[cfg(feature = "SceMusicExport_stub")]
extern "C" {
    pub fn sceMusicExportFromFile(
        path: *const crate::ctypes::c_char,
        param: *const MusicExportParam,
        workingMemory: *mut crate::ctypes::c_void,
        cancelCb: *mut crate::ctypes::c_void,
        progress: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut crate::ctypes::c_void, arg2: crate::ctypes::c_int),
        >,
        user: *mut crate::ctypes::c_void,
        outPath: *mut crate::ctypes::c_char,
        outPathSize: SceSize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceNearDialogUtil_stub", kind = "static")]
#[cfg(feature = "SceNearDialogUtil_stub")]
extern "C" {}
#[link(name = "SceNearUtil_stub", kind = "static")]
#[cfg(feature = "SceNearUtil_stub")]
extern "C" {}
#[link(name = "SceNetAdhocMatching_stub", kind = "static")]
#[cfg(feature = "SceNetAdhocMatching_stub")]
extern "C" {}
#[link(name = "SceNetCtl_stub", kind = "static")]
#[cfg(feature = "SceNetCtl_stub")]
extern "C" {
    pub fn sceNetCtlAdhocDisconnect() -> crate::ctypes::c_int;
    pub fn sceNetCtlAdhocGetInAddr(inaddr: *mut SceNetInAddr) -> crate::ctypes::c_int;
    pub fn sceNetCtlAdhocGetPeerList(
        buflen: *mut crate::ctypes::c_uint,
        buf: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetCtlAdhocGetResult(
        eventType: crate::ctypes::c_int,
        errorCode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetCtlAdhocGetState(state: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetCtlAdhocRegisterCallback(
        func: SceNetCtlCallback,
        arg: *mut crate::ctypes::c_void,
        cid: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetCtlAdhocUnregisterCallback(cid: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetCtlCheckCallback() -> crate::ctypes::c_int;
    pub fn sceNetCtlGetNatInfo(natinfo: *mut SceNetCtlNatInfo) -> crate::ctypes::c_int;
    pub fn sceNetCtlInetGetInfo(
        code: crate::ctypes::c_int,
        info: *mut SceNetCtlInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceNetCtlInetGetResult(
        eventType: crate::ctypes::c_int,
        errorCode: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetCtlInetGetState(state: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetCtlInetRegisterCallback(
        func: SceNetCtlCallback,
        arg: *mut crate::ctypes::c_void,
        cid: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetCtlInetUnregisterCallback(cid: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetCtlInit() -> crate::ctypes::c_int;
    pub fn sceNetCtlTerm();
}
#[link(name = "SceNetPsForDriver_stub", kind = "static")]
#[cfg(feature = "SceNetPsForDriver_stub")]
extern "C" {
    pub fn ksceNetAccept(
        s: crate::ctypes::c_int,
        addr: *mut SceNetSockaddr,
        addrlen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceNetBind(
        s: crate::ctypes::c_int,
        addr: *const SceNetSockaddr,
        addrlen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceNetClose(s: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceNetConnect(
        s: crate::ctypes::c_int,
        name: *const SceNetSockaddr,
        namelen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceNetListen(
        s: crate::ctypes::c_int,
        backlog: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceNetRecvfrom(
        s: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        flags: crate::ctypes::c_int,
        from: *mut SceNetSockaddr,
        fromlen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceNetSendto(
        s: crate::ctypes::c_int,
        msg: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        flags: crate::ctypes::c_int,
        to: *const SceNetSockaddr,
        tolen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceNetSetsockopt(
        s: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
        optname: crate::ctypes::c_int,
        optval: *const crate::ctypes::c_void,
        optlen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceNetSocket(
        name: *const crate::ctypes::c_char,
        domain: crate::ctypes::c_int,
        type_: crate::ctypes::c_int,
        protocol: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceNetPs_stub", kind = "static")]
#[cfg(feature = "SceNetPs_stub")]
extern "C" {
    pub fn sceNetSyscallAccept(
        s: crate::ctypes::c_int,
        addr: *mut crate::ctypes::c_void,
        addrlen: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallBind(
        s: crate::ctypes::c_int,
        addr: *const crate::ctypes::c_void,
        addrlen: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallClose(s: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetSyscallConnect(
        s: crate::ctypes::c_int,
        name: *const crate::ctypes::c_void,
        namelen: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallControl(
        if_index: crate::ctypes::c_int,
        code: crate::ctypes::c_int,
        ptr: *mut crate::ctypes::c_void,
        len: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallDescriptorClose(id: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetSyscallDescriptorCreate(
        name: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallDescriptorCtl(
        id: crate::ctypes::c_int,
        op: crate::ctypes::c_int,
        s: crate::ctypes::c_int,
        info: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallDumpAbort(
        id: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallDumpClose(id: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetSyscallDumpCreate(
        name: *const crate::ctypes::c_char,
        len: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallDumpRead(
        id: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
        len: crate::ctypes::c_int,
        pflags: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallEpollAbort(
        eid: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallEpollClose(eid: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetSyscallEpollCreate(
        name: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallEpollCtl(
        eid: crate::ctypes::c_int,
        op: crate::ctypes::c_int,
        id: crate::ctypes::c_int,
        event: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallEpollWait(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
    pub fn sceNetSyscallGetIfList(
        list: *mut crate::ctypes::c_void,
        n: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallGetSockinfo(
        s: crate::ctypes::c_int,
        ptr: *mut crate::ctypes::c_void,
        n: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallGetpeername(
        s: crate::ctypes::c_int,
        name: *mut crate::ctypes::c_void,
        namelen: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallGetsockname(
        s: crate::ctypes::c_int,
        name: *mut crate::ctypes::c_void,
        namelen: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallGetsockopt(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
    pub fn sceNetSyscallIcmConnect(
        s: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallIoctl(
        s: crate::ctypes::c_int,
        com: crate::ctypes::c_uint,
        data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallListen(
        s: crate::ctypes::c_int,
        backlog: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallRecvfrom(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
    pub fn sceNetSyscallRecvmsg(
        s: crate::ctypes::c_int,
        msg: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallSendmsg(
        s: crate::ctypes::c_int,
        msg: *const crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallSendto(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
    pub fn sceNetSyscallSetsockopt(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
    pub fn sceNetSyscallShutdown(
        s: crate::ctypes::c_int,
        how: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallSocket(
        name: *const crate::ctypes::c_char,
        domain: crate::ctypes::c_int,
        type_: crate::ctypes::c_int,
        protocol: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallSocketAbort(
        s: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSyscallSysctl(param: *mut SceNetSyscallParameter) -> crate::ctypes::c_int;
}
#[link(name = "SceNet_stub", kind = "static")]
#[cfg(feature = "SceNet_stub")]
extern "C" {
    pub fn sceNetAccept(
        s: crate::ctypes::c_int,
        addr: *mut SceNetSockaddr,
        addrlen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceNetBind(
        s: crate::ctypes::c_int,
        addr: *const SceNetSockaddr,
        addrlen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceNetClearDnsCache(flags: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetConnect(
        s: crate::ctypes::c_int,
        name: *const SceNetSockaddr,
        namelen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceNetDumpAbort(
        id: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetDumpCreate(
        name: *const crate::ctypes::c_char,
        len: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetDumpDestroy(id: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetDumpRead(
        id: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
        len: crate::ctypes::c_int,
        pflags: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetEmulationGet(
        param: *mut SceNetEmulationParam,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetEmulationSet(
        param: *mut SceNetEmulationParam,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetEpollAbort(
        eid: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetEpollControl(
        eid: crate::ctypes::c_int,
        op: crate::ctypes::c_int,
        id: crate::ctypes::c_int,
        event: *mut SceNetEpollEvent,
    ) -> crate::ctypes::c_int;
    pub fn sceNetEpollCreate(
        name: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetEpollDestroy(eid: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetEpollWait(
        eid: crate::ctypes::c_int,
        events: *mut SceNetEpollEvent,
        maxevents: crate::ctypes::c_int,
        timeout: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetEpollWaitCB(
        eid: crate::ctypes::c_int,
        events: *mut SceNetEpollEvent,
        maxevents: crate::ctypes::c_int,
        timeout: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetErrnoLoc() -> *mut crate::ctypes::c_int;
    pub fn sceNetEtherNtostr(
        n: *const SceNetEtherAddr,
        str_: *mut crate::ctypes::c_char,
        len: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceNetEtherStrton(
        str_: *const crate::ctypes::c_char,
        n: *mut SceNetEtherAddr,
    ) -> crate::ctypes::c_int;
    pub fn sceNetGetMacAddress(
        addr: *mut SceNetEtherAddr,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetGetSockIdInfo(
        fds: *mut SceNetFdSet,
        sockinfoflags: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetGetSockInfo(
        s: crate::ctypes::c_int,
        info: *mut SceNetSockInfo,
        n: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetGetStatisticsInfo(
        info: *mut SceNetStatisticsInfo,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetGetpeername(
        s: crate::ctypes::c_int,
        name: *mut SceNetSockaddr,
        namelen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceNetGetsockname(
        s: crate::ctypes::c_int,
        name: *mut SceNetSockaddr,
        namelen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceNetGetsockopt(
        s: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
        optname: crate::ctypes::c_int,
        optval: *mut crate::ctypes::c_void,
        optlen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceNetHtonl(host32: crate::ctypes::c_uint) -> crate::ctypes::c_uint;
    pub fn sceNetHtonll(host64: crate::ctypes::c_ulonglong) -> crate::ctypes::c_ulonglong;
    pub fn sceNetHtons(host16: crate::ctypes::c_ushort) -> crate::ctypes::c_ushort;
    pub fn sceNetInetNtop(
        af: crate::ctypes::c_int,
        src: *const crate::ctypes::c_void,
        dst: *mut crate::ctypes::c_char,
        size: crate::ctypes::c_uint,
    ) -> *const crate::ctypes::c_char;
    pub fn sceNetInetPton(
        af: crate::ctypes::c_int,
        src: *const crate::ctypes::c_char,
        dst: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetInit(param: *mut SceNetInitParam) -> crate::ctypes::c_int;
    pub fn sceNetListen(
        s: crate::ctypes::c_int,
        backlog: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetNtohl(net32: crate::ctypes::c_uint) -> crate::ctypes::c_uint;
    pub fn sceNetNtohll(net64: crate::ctypes::c_ulonglong) -> crate::ctypes::c_ulonglong;
    pub fn sceNetNtohs(net16: crate::ctypes::c_ushort) -> crate::ctypes::c_ushort;
    pub fn sceNetRecv(
        s: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetRecvfrom(
        s: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        flags: crate::ctypes::c_int,
        from: *mut SceNetSockaddr,
        fromlen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceNetRecvmsg(
        s: crate::ctypes::c_int,
        msg: *mut SceNetMsghdr,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetResolverAbort(
        rid: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetResolverCreate(
        name: *const crate::ctypes::c_char,
        param: *mut SceNetResolverParam,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetResolverDestroy(rid: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetResolverGetError(
        rid: crate::ctypes::c_int,
        result: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetResolverStartAton(
        rid: crate::ctypes::c_int,
        addr: *const SceNetInAddr,
        hostname: *mut crate::ctypes::c_char,
        len: crate::ctypes::c_int,
        timeout: crate::ctypes::c_int,
        retry: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetResolverStartNtoa(
        rid: crate::ctypes::c_int,
        hostname: *const crate::ctypes::c_char,
        addr: *mut SceNetInAddr,
        timeout: crate::ctypes::c_int,
        retry: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSend(
        s: crate::ctypes::c_int,
        msg: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSendmsg(
        s: crate::ctypes::c_int,
        msg: *const SceNetMsghdr,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSendto(
        s: crate::ctypes::c_int,
        msg: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        flags: crate::ctypes::c_int,
        to: *const SceNetSockaddr,
        tolen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSetDnsInfo(
        info: *mut SceNetDnsInfo,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSetsockopt(
        s: crate::ctypes::c_int,
        level: crate::ctypes::c_int,
        optname: crate::ctypes::c_int,
        optval: *const crate::ctypes::c_void,
        optlen: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceNetShowIfconfig(
        p: *mut crate::ctypes::c_void,
        b: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetShowNetstat() -> crate::ctypes::c_int;
    pub fn sceNetShowRoute() -> crate::ctypes::c_int;
    pub fn sceNetShutdown(
        s: crate::ctypes::c_int,
        how: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSocket(
        name: *const crate::ctypes::c_char,
        domain: crate::ctypes::c_int,
        type_: crate::ctypes::c_int,
        protocol: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSocketAbort(
        s: crate::ctypes::c_int,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetSocketClose(s: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceNetTerm() -> crate::ctypes::c_int;
}
#[link(name = "SceNgs_stub", kind = "static")]
#[cfg(feature = "SceNgs_stub")]
extern "C" {
    pub fn sceNgsModuleCheckParamsInRangeInternal(
        handle: SceNgsHVoice,
        module_id: SceNgsModuleID,
        descriptor: *const SceNgsParamsDescriptor,
        size: SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsModuleGetNumPresetsInternal(
        handle: SceNgsHSynSystem,
        module_id: SceNgsModuleID,
        num_presets: *mut SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsModuleGetPresetInternal(
        handle: SceNgsHSynSystem,
        module_id: SceNgsModuleID,
        preset_index: SceUInt32,
        info: *mut SceNgsBufferInfo,
    ) -> SceInt32;
    pub fn sceNgsPatchCreateRoutingInternal(
        info: *const SceNgsPatchSetupInfo,
        handle: *mut SceNgsHPatch,
    ) -> SceInt32;
    pub fn sceNgsPatchRemoveRoutingInternal(handle: SceNgsHPatch) -> SceInt32;
    pub fn sceNgsRackGetRequiredMemorySizeInternal(
        handle: SceNgsHSynSystem,
        rack_description: *const SceNgsRackDescription,
        user_size: *mut SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsRackGetVoiceHandleInternal(
        rack_handle: SceNgsHRack,
        index: SceUInt32,
        voice_handle: *mut SceNgsHVoice,
    ) -> SceInt32;
    pub fn sceNgsRackInitInternal(
        system_handle: SceNgsHSynSystem,
        rack_buffer: *mut SceNgsBufferInfo,
        rack_description: *const SceNgsRackDescription,
        rack_handle: *mut SceNgsHRack,
    ) -> SceInt32;
    pub fn sceNgsRackReleaseInternal(
        handle: SceNgsHRack,
        callback: SceNgsRackReleaseCallbackFunc,
    ) -> SceInt32;
    pub fn sceNgsRackSetParamErrorCallbackInternal(
        rack_handle: SceNgsHRack,
        callback: SceNgsParamsErrorCallbackFunc,
    ) -> SceInt32;
    pub fn sceNgsSulphaGetInfoInternal(
        obj_reg: *const SulphaNgsRegistration,
        info: *mut SceNgsBufferInfo,
    ) -> SceInt32;
    pub fn sceNgsSulphaGetModuleListInternal(
        module_ids: *mut SceUInt32,
        in_array_count: SceUInt32,
        count: *mut SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsSulphaGetSynthUpdateCallbackInternal(
        handle: SceNgsHSynSystem,
        update_callback: *mut SceNgsSulphaUpdateCallback,
        info: *mut SceNgsBufferInfo,
    ) -> SceInt32;
    pub fn sceNgsSulphaQueryModuleInternal(
        type_: SulphaNgsModuleQueryType,
        debug: *mut SulphaNgsModuleQuery,
    ) -> SceInt32;
    pub fn sceNgsSulphaSetSynthUpdateCallbackInternal(
        handle: SceNgsHSynSystem,
        update_callback: SceNgsSulphaUpdateCallback,
        info: *mut SceNgsBufferInfo,
    ) -> SceInt32;
    pub fn sceNgsSystemGetCallbackListInternal(
        handle: SceNgsHSynSystem,
        array: *mut *mut SceNgsCallbackListInfo,
        array_size: *mut SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsSystemGetRequiredMemorySizeInternal(
        params: *const SceNgsSystemInitParams,
        size: *mut SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsSystemInitInternal(
        buffer_info: *mut SceNgsBufferInfo,
        compiled_sdk_version: SceUInt32,
        params: *const SceNgsSystemInitParams,
        handle: *mut SceNgsHSynSystem,
    ) -> SceInt32;
    pub fn sceNgsSystemLockInternal(handle: SceNgsHSynSystem) -> SceInt32;
    pub fn sceNgsSystemPullDataInternal(
        handle: SceNgsHSynSystem,
        dirty_flags_a: SceUInt32,
        dirty_flags_b: SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsSystemPushDataInternal(handle: SceNgsHSynSystem) -> SceInt32;
    pub fn sceNgsSystemReleaseInternal(handle: SceNgsHSynSystem) -> SceInt32;
    pub fn sceNgsSystemSetFlagsInternal(
        handle: SceNgsHSynSystem,
        system_flags: SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsSystemSetParamErrorCallbackInternal(
        handle: SceNgsHSynSystem,
        callback_id: SceNgsParamsErrorCallbackFunc,
    ) -> SceInt32;
    pub fn sceNgsSystemUnlockInternal(handle: SceNgsHSynSystem) -> SceInt32;
    pub fn sceNgsSystemUpdateInternal(handle: SceNgsHSynSystem) -> SceInt32;
    pub fn sceNgsVoiceBypassModuleInternal(
        handle: SceNgsHVoice,
        module: SceUInt32,
        flag: SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsVoiceClearDirtyFlagInternal(
        handle: SceNgsHVoice,
        param_bit_flag: SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsVoiceDefGetAtrac9VoiceInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetCompressorBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetCompressorSideChainBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetDelayBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetDistortionBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetEnvelopeBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetEqBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetMasterBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetMixerBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetPauserBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetPitchshiftBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetReverbBussInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetSasEmuVoiceInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetScreamVoiceAT9Internal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetScreamVoiceInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetSimpleAtrac9VoiceInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetSimpleVoiceInternal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefGetTemplate1Internal() -> *const SceNgsVoiceDefinition;
    pub fn sceNgsVoiceDefinitionGetPresetInternal(
        definition: *const SceNgsVoiceDefinition,
        index: SceUInt32,
        presets: *mut *const SceNgsVoicePreset,
    ) -> SceInt32;
    pub fn sceNgsVoiceGetModuleBypassInternal(
        handle: SceNgsHVoice,
        module: SceUInt32,
        flag: *mut SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsVoiceGetOutputPatchInternal(
        handle: SceNgsHVoice,
        nOutputIndex: SceInt32,
        nSubIndex: SceInt32,
        pPatchHandle: *mut SceNgsHPatch,
    ) -> SceInt32;
    pub fn sceNgsVoiceGetParamsOutOfRangeBufferedInternal(
        handle: SceNgsHVoice,
        module: SceUInt32,
        message_buffer: *mut crate::ctypes::c_char,
    ) -> SceInt32;
    pub fn sceNgsVoiceInitInternal(
        handle: SceNgsHVoice,
        preset: *const SceNgsVoicePreset,
        flags: SceUInt32,
    ) -> SceInt32;
    pub fn sceNgsVoiceKeyOffInternal(handle: SceNgsHVoice) -> SceInt32;
    pub fn sceNgsVoiceKillInternal(handle: SceNgsHVoice) -> SceInt32;
    pub fn sceNgsVoicePauseInternal(handle: SceNgsHVoice) -> SceInt32;
    pub fn sceNgsVoicePlayInternal(handle: SceNgsHVoice) -> SceInt32;
    pub fn sceNgsVoiceResumeInternal(handle: SceNgsHVoice) -> SceInt32;
    pub fn sceNgsVoiceSetAllBypassesInternal(handle: SceNgsHVoice, bitflag: SceUInt32) -> SceInt32;
    pub fn sceNgsVoiceSetFinishedCallbackInternal(
        handle: SceNgsHVoice,
        callback: SceNgsCallbackFunc,
        userdata: *mut crate::ctypes::c_void,
    ) -> SceInt32;
    pub fn sceNgsVoiceSetModuleCallbackInternal(
        handle: SceNgsHVoice,
        module: SceUInt32,
        callback: SceNgsModuleCallbackFunc,
        callback_userdata: *mut crate::ctypes::c_void,
    ) -> SceInt32;
    pub fn sceNgsVoiceSetPresetInternal(
        handle: SceNgsHVoice,
        preset: *const SceNgsVoicePreset,
    ) -> SceInt32;
}
#[link(name = "SceNgsUser_stub", kind = "static")]
#[cfg(feature = "SceNgsUser_stub")]
extern "C" {}
#[link(name = "SceNotificationUtil_stub", kind = "static")]
#[cfg(feature = "SceNotificationUtil_stub")]
extern "C" {
    pub fn sceNotificationUtilBgAppInitialize() -> SceInt32;
    pub fn sceNotificationUtilCleanHistory() -> SceInt32;
    pub fn sceNotificationUtilProgressBegin(
        initParams: *mut SceNotificationUtilProgressInitParam,
    ) -> SceInt32;
    pub fn sceNotificationUtilProgressFinish(
        finishParams: *mut SceNotificationUtilProgressFinishParam,
    ) -> SceInt32;
    pub fn sceNotificationUtilProgressUpdate(
        updateParams: *mut SceNotificationUtilProgressUpdateParam,
    ) -> SceInt32;
    pub fn sceNotificationUtilSendNotification(text: *const SceWChar16) -> SceInt32;
}
#[link(name = "SceNpActivity_stub", kind = "static")]
#[cfg(feature = "SceNpActivity_stub")]
extern "C" {}
#[link(name = "SceNpBasic_stub", kind = "static")]
#[cfg(feature = "SceNpBasic_stub")]
extern "C" {}
#[link(name = "SceNpCommerce2_stub", kind = "static")]
#[cfg(feature = "SceNpCommerce2_stub")]
extern "C" {}
#[link(name = "SceNpCommon_stub", kind = "static")]
#[cfg(feature = "SceNpCommon_stub")]
extern "C" {}
#[link(name = "SceNpDrmForDriver_stub", kind = "static")]
#[cfg(feature = "SceNpDrmForDriver_stub")]
extern "C" {
    pub fn ksceNpDrmCheckActData(
        act_type: *mut crate::ctypes::c_int,
        version_flag: *mut crate::ctypes::c_int,
        account_id: *mut SceUInt64,
        act_start_time: *mut SceUInt64,
        act_end_time: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmEbootSigConvert(
        eboot_pbp_path: *const crate::ctypes::c_char,
        old_eboot_signature: *const crate::ctypes::c_void,
        new_eboot_signature: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmEbootSigGenMultiDisc(
        eboot_pbp_path: *const crate::ctypes::c_char,
        sce_discinfo: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
        sw_version: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmEbootSigGenPs1(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_sha256: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
        sw_version: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmEbootSigGenPsp(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_sha256: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
        sw_version: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmEbootSigVerify(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_signature: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmGetFixedRifName(
        name: *mut crate::ctypes::c_char,
        aid: SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmGetRifInfo(
        license: *const crate::ctypes::c_void,
        license_size: SceSize,
        check_sign: crate::ctypes::c_int,
        content_id: *mut crate::ctypes::c_char,
        account_id: *mut SceUInt64,
        license_version: *mut crate::ctypes::c_int,
        license_flags: *mut crate::ctypes::c_int,
        flags: *mut crate::ctypes::c_int,
        sku_flags: *mut crate::ctypes::c_int,
        lic_start_time: *mut SceInt64,
        lic_exp_time: *mut SceInt64,
        flags2: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmGetRifName(
        name: *mut crate::ctypes::c_char,
        aid: SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmGetRifPspKey(
        license: *const crate::ctypes::c_void,
        klicense: *mut crate::ctypes::c_void,
        flags: *mut crate::ctypes::c_int,
        lic_start_time: *mut SceUInt64,
        lic_exp_time: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmGetRifVitaKey(
        license: *const crate::ctypes::c_void,
        klicense: *mut crate::ctypes::c_void,
        flags: *mut crate::ctypes::c_int,
        sku_flags: *mut crate::ctypes::c_int,
        lic_start_time: *mut SceUInt64,
        lic_exp_time: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmPspEbootSigGen(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_sha256: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmPspEbootVerify(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_signature: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceNpDrmReadActData(act_data: *mut SceNpDrmActivationData) -> crate::ctypes::c_int;
}
#[link(name = "SceNpDrm_stub", kind = "static")]
#[cfg(feature = "SceNpDrm_stub")]
extern "C" {
    pub fn _sceNpDrmCheckActData(
        act_type: *mut crate::ctypes::c_int,
        version_flag: *mut crate::ctypes::c_int,
        account_id: *mut SceUInt64,
        act_exp_time: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn _sceNpDrmGetFixedRifName(
        rif_name: *mut crate::ctypes::c_char,
        aid: u64,
    ) -> crate::ctypes::c_int;
    pub fn _sceNpDrmGetRifName(
        rif_name: *mut crate::ctypes::c_char,
        aid: u64,
    ) -> crate::ctypes::c_int;
    pub fn _sceNpDrmGetRifNameForInstall(
        rif_name: *mut crate::ctypes::c_char,
        rif_data: *const crate::ctypes::c_void,
        unk: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn _sceNpDrmPackageCheck(
        buffer: *const crate::ctypes::c_void,
        size: SceSize,
        zero: crate::ctypes::c_int,
        identifier: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn _sceNpDrmPackageDecrypt(
        buffer: *mut crate::ctypes::c_void,
        size: SceSize,
        opt: *mut _sceNpDrmPackageDecrypt_opt,
    ) -> crate::ctypes::c_int;
    pub fn scePsmDrmGetRifKey(
        license_buf: *const ScePsmDrmLicense,
        keydata: *mut crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceNpManager_stub", kind = "static")]
#[cfg(feature = "SceNpManager_stub")]
extern "C" {}
#[link(name = "SceNpMatching2_stub", kind = "static")]
#[cfg(feature = "SceNpMatching2_stub")]
extern "C" {}
#[link(name = "SceNpMessage_stub", kind = "static")]
#[cfg(feature = "SceNpMessage_stub")]
extern "C" {}
#[link(name = "SceNpParty_stub", kind = "static")]
#[cfg(feature = "SceNpParty_stub")]
extern "C" {}
#[link(name = "SceNpScore_stub", kind = "static")]
#[cfg(feature = "SceNpScore_stub")]
extern "C" {}
#[link(name = "SceNpSignaling_stub", kind = "static")]
#[cfg(feature = "SceNpSignaling_stub")]
extern "C" {}
#[link(name = "SceNpSnsFacebook_stub", kind = "static")]
#[cfg(feature = "SceNpSnsFacebook_stub")]
extern "C" {}
#[link(name = "SceNpTrophy_stub", kind = "static")]
#[cfg(feature = "SceNpTrophy_stub")]
extern "C" {}
#[link(name = "SceNpTus_stub", kind = "static")]
#[cfg(feature = "SceNpTus_stub")]
extern "C" {}
#[link(name = "SceNpUtility_stub", kind = "static")]
#[cfg(feature = "SceNpUtility_stub")]
extern "C" {}
#[link(name = "SceNpWebApi_stub", kind = "static")]
#[cfg(feature = "SceNpWebApi_stub")]
extern "C" {}
#[link(name = "SceOledForDriver_stub", kind = "static")]
#[cfg(feature = "SceOledForDriver_stub")]
extern "C" {}
#[link(name = "ScePaf_stub", kind = "static")]
#[cfg(feature = "ScePaf_stub")]
extern "C" {
    pub fn scePafCreateHeap(
        context: *mut ScePafHeapContext,
        membase: *mut crate::ctypes::c_void,
        size: SceSize,
        name: *const crate::ctypes::c_char,
        opt: *mut ScePafHeapOpt,
    );
    pub fn scePafDeleteHeap(context: *mut ScePafHeapContext);
    pub fn scePafFreeWithContext(context: *mut ScePafHeapContext, ptr: *mut crate::ctypes::c_void);
    pub fn scePafGetCurrentClockLocalTime(data: *mut ScePafDateTime) -> crate::ctypes::c_int;
    pub static mut scePafGraphicsCurrentWave: SceUInt32;
    pub fn scePafGraphicsUpdateCurrentWave(
        index: SceUInt32,
        update_interval: SceFloat32,
    ) -> crate::ctypes::c_int;
    pub fn scePafMallocAlignWithContext(
        context: *mut ScePafHeapContext,
        align: SceUInt32,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn scePafMallocWithContext(
        context: *mut ScePafHeapContext,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn scePafReallocWithContext(
        context: *mut ScePafHeapContext,
        ptr: *mut crate::ctypes::c_void,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn scePafSha1Init(context: *mut ScePafSha1Context) -> crate::ctypes::c_int;
    pub fn scePafSha1Result(
        context: *mut ScePafSha1Context,
        dst: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn scePafSha1Update(
        context: *mut ScePafSha1Context,
        data: *const crate::ctypes::c_void,
        length: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sce_paf_bcmp(
        ptr1: *const crate::ctypes::c_void,
        ptr2: *const crate::ctypes::c_void,
        num: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sce_paf_bcopy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        n: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sce_paf_bzero(dst: *mut crate::ctypes::c_void, n: SceSize)
        -> *mut crate::ctypes::c_void;
    pub fn sce_paf_free(ptr: *mut crate::ctypes::c_void);
    pub fn sce_paf_malloc(size: SceSize) -> *mut crate::ctypes::c_void;
    pub fn sce_paf_memalign(align: SceSize, length: SceSize) -> *mut crate::ctypes::c_void;
    pub fn sce_paf_memchr(
        src: *const crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        length: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sce_paf_memcmp(
        s1: *const crate::ctypes::c_void,
        s2: *const crate::ctypes::c_void,
        n: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sce_paf_memcpy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sce_paf_memmove(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sce_paf_memset(
        dst: *mut crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        len: SceSize,
    ) -> *mut crate::ctypes::c_void;
    pub fn sce_paf_snprintf(
        dst: *mut crate::ctypes::c_char,
        max: crate::ctypes::c_uint,
        fmt: *const crate::ctypes::c_char,
        ...
    ) -> crate::ctypes::c_int;
    pub fn sce_paf_strcasecmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sce_paf_strchr(
        s: *const crate::ctypes::c_char,
        ch: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_char;
    pub fn sce_paf_strcmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sce_paf_strlen(s: *const crate::ctypes::c_char) -> usize;
    pub fn sce_paf_strncasecmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sce_paf_strncmp(
        s1: *const crate::ctypes::c_char,
        s2: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sce_paf_strncpy(
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> *mut crate::ctypes::c_char;
    pub fn sce_paf_strrchr(
        s: *const crate::ctypes::c_char,
        ch: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_char;
    pub fn sce_paf_strtod(
        nptr: *const crate::ctypes::c_char,
        endptr: *mut *mut crate::ctypes::c_char,
    ) -> f64;
    pub fn sce_paf_vsnprintf(
        dst: *mut crate::ctypes::c_char,
        max: crate::ctypes::c_uint,
        fmt: *const crate::ctypes::c_char,
        arg: va_list,
    ) -> crate::ctypes::c_int;
}
#[link(name = "ScePamgr_stub", kind = "static")]
#[cfg(feature = "ScePamgr_stub")]
extern "C" {
    pub fn _sceKernelPaAddArmTraceByKey(
        key: crate::ctypes::c_int,
        param: *const SceKernelPaArmTraceParam,
    ) -> crate::ctypes::c_int;
    pub fn _sceKernelPaAddCounterTraceByKey(
        key: crate::ctypes::c_int,
        param: *const SceKernelPaCounterTraceParam,
    ) -> crate::ctypes::c_int;
    pub fn _sceKernelPaAddGpuTraceByKey(
        key: crate::ctypes::c_int,
        param: *const SceKernelPaGpuTraceParam,
    ) -> crate::ctypes::c_int;
    pub fn _sceKernelPaGetGpuSampledData(
        data: *mut SceKernelPaGpuSampledData,
    ) -> crate::ctypes::c_int;
    pub fn _sceKernelPaSetupTraceBufferByKey(
        key: crate::ctypes::c_int,
        param: *const SceKernelPaTraceBufferParam,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelPaGetIoBaseAddress() -> SceUInt32;
    pub fn sceKernelPaGetTimebaseFrequency() -> SceUInt32;
    pub fn sceKernelPaGetTimebaseValue() -> SceUInt64;
    pub fn sceKernelPaGetTraceBufferSize(type_: SceUInt32) -> SceSize;
    pub fn sceKernelPaGetTraceBufferStatus() -> SceUInt32;
    pub fn sceKernelPaGetWritePointer() -> SceUInt32;
    pub fn sceKernelPaInsertBookmark(
        fifo: SceUInt32,
        channel: SceUInt32,
        data: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelPaRegister() -> crate::ctypes::c_int;
    pub fn sceKernelPaRemoveArmTraceByKey(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelPaRemoveCounterTraceByKey(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelPaRemoveGpuTraceByKey(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelPaSetBookmarkChannelEnableByKey(
        key: crate::ctypes::c_int,
        fifo: SceUInt32,
        mask: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelPaStartByKey(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelPaStopByKey(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelPaUnregister(key: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceKernelPerfArmPmonClose() -> crate::ctypes::c_int;
    pub fn sceKernelPerfArmPmonOpen() -> crate::ctypes::c_int;
    pub fn sceKernelPerfArmPmonReset(threadId: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelPerfArmPmonSelectEvent(
        threadId: SceUID,
        counter: SceUInt32,
        eventCode: SceUInt8,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelPerfArmPmonSetCounterValue(
        threadId: SceUID,
        counter: SceUInt32,
        value: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelPerfArmPmonStart(threadId: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelPerfArmPmonStop(threadId: SceUID) -> crate::ctypes::c_int;
}
#[link(name = "ScePerf_stub", kind = "static")]
#[cfg(feature = "ScePerf_stub")]
extern "C" {
    pub fn scePerfArmPmonGetCounterValue(
        thid: SceUID,
        counter: SceUInt32,
        value: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn scePerfArmPmonReset(thid: SceUID) -> crate::ctypes::c_int;
    pub fn scePerfArmPmonSelectEvent(
        thid: SceUID,
        counter: SceUInt32,
        event_code: SceUInt8,
    ) -> crate::ctypes::c_int;
    pub fn scePerfArmPmonSetCounterValue(
        thid: SceUID,
        counter: SceUInt32,
        value: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn scePerfArmPmonSoftwareIncrement(mask: SceUInt32) -> crate::ctypes::c_int;
    pub fn scePerfArmPmonStart(thid: SceUID) -> crate::ctypes::c_int;
    pub fn scePerfArmPmonStop(thid: SceUID) -> crate::ctypes::c_int;
    pub fn scePerfGetTimebaseFrequency() -> SceUInt32;
    pub fn scePerfGetTimebaseValue() -> SceUInt64;
}
#[link(name = "ScePervasiveForDriver_stub", kind = "static")]
#[cfg(feature = "ScePervasiveForDriver_stub")]
extern "C" {
    pub fn kscePervasiveDsiClockDisable(
        port: crate::ctypes::c_int,
        mask: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn kscePervasiveDsiClockEnable(
        port: crate::ctypes::c_int,
        mask: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn kscePervasiveDsiResetDisable(
        port: crate::ctypes::c_int,
        mask: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn kscePervasiveDsiResetEnable(
        port: crate::ctypes::c_int,
        mask: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn kscePervasiveDsiSetPixelClock(
        port: crate::ctypes::c_int,
        pixelclock: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn kscePervasiveGetSoCRevision() -> SceUInt32;
    pub fn kscePervasiveGpioClockDisable() -> crate::ctypes::c_int;
    pub fn kscePervasiveGpioClockEnable() -> crate::ctypes::c_int;
    pub fn kscePervasiveGpioResetDisable() -> crate::ctypes::c_int;
    pub fn kscePervasiveGpioResetEnable() -> crate::ctypes::c_int;
    pub fn kscePervasiveMsifClockDisable() -> crate::ctypes::c_int;
    pub fn kscePervasiveMsifClockEnable() -> crate::ctypes::c_int;
    pub fn kscePervasiveMsifResetDisable() -> crate::ctypes::c_int;
    pub fn kscePervasiveMsifResetEnable() -> crate::ctypes::c_int;
    pub fn kscePervasiveMsifSetClock(clock: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn kscePervasiveRemovableMemoryGetCardInsertState() -> crate::ctypes::c_int;
    pub fn kscePervasiveSpiClockDisable(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePervasiveSpiClockEnable(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePervasiveSpiResetDisable(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePervasiveSpiResetEnable(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePervasiveUartClockDisable(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePervasiveUartClockEnable(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePervasiveUartResetDisable(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePervasiveUartResetEnable(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePervasiveUartSetBaudrate(
        port: crate::ctypes::c_int,
        baudrate: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
#[link(name = "ScePfsMgrForKernel_stub", kind = "static")]
#[cfg(feature = "ScePfsMgrForKernel_stub")]
extern "C" {
    pub fn kscePfsApprove(
        rnd_drive_id: *const ScePfsRndDriveId,
        program_authority_id: SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn kscePfsDisapprove(
        rnd_drive_id: *const ScePfsRndDriveId,
        program_authority_id: SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn kscePfsMount(
        path: *const crate::ctypes::c_char,
        rnd_drive_id: *const ScePfsRndDriveId,
        program_authority_id: SceUInt64,
        klicensee: *const crate::ctypes::c_void,
        mode_index: u16,
    ) -> crate::ctypes::c_int;
    pub fn kscePfsMount2(
        path: *const crate::ctypes::c_char,
        rnd_drive_id: *const ScePfsRndDriveId,
        klicensee: *const crate::ctypes::c_void,
        mode_index: u16,
    ) -> crate::ctypes::c_int;
    pub fn kscePfsUnmount(rnd_drive_id: *const ScePfsRndDriveId) -> crate::ctypes::c_int;
}
#[link(name = "ScePgf_stub", kind = "static")]
#[cfg(feature = "ScePgf_stub")]
extern "C" {
    pub fn sceFontClose(fontHandle: SceFontHandle) -> crate::ctypes::c_int;
    pub fn sceFontDoneLib(libHandle: SceFontLibHandle) -> crate::ctypes::c_int;
    pub fn sceFontFindFont(
        libHandle: SceFontLibHandle,
        fontStyle: *mut SceFontStyle,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceFontFindOptimumFont(
        libHandle: SceFontLibHandle,
        fontStyle: *mut SceFontStyle,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceFontFlush(fontHandle: SceFontHandle) -> crate::ctypes::c_int;
    pub fn sceFontGetCharGlyphImage(
        fontHandle: SceFontHandle,
        charCode: crate::ctypes::c_uint,
        glyphImage: *mut SceFontGlyphImage,
    ) -> crate::ctypes::c_int;
    pub fn sceFontGetCharGlyphImage_Clip(
        fontHandle: SceFontHandle,
        charCode: crate::ctypes::c_uint,
        glyphImage: *mut SceFontGlyphImage,
        clipXPos: crate::ctypes::c_int,
        clipYPos: crate::ctypes::c_int,
        clipWidth: crate::ctypes::c_int,
        clipHeight: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceFontGetCharImageRect(
        fontHandle: SceFontHandle,
        charCode: crate::ctypes::c_uint,
        charRect: *mut SceFontImageRect,
    ) -> crate::ctypes::c_int;
    pub fn sceFontGetCharInfo(
        fontHandle: SceFontHandle,
        charCode: crate::ctypes::c_uint,
        charInfo: *mut SceFontCharInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceFontGetFontInfo(
        fontHandle: SceFontHandle,
        fontInfo: *mut SceFontInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceFontGetFontInfoByIndexNumber(
        libHandle: SceFontLibHandle,
        fontStyle: *mut SceFontStyle,
        unknown: crate::ctypes::c_int,
        fontIndex: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceFontGetFontList(
        libHandle: SceFontLibHandle,
        fontStyle: *mut SceFontStyle,
        numFonts: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceFontGetNumFontList(
        libHandle: SceFontLibHandle,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceFontNewLib(
        params: *mut SceFontNewLibParams,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> SceFontLibHandle;
    pub fn sceFontOpen(
        libHandle: SceFontLibHandle,
        index: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> SceFontHandle;
    pub fn sceFontOpenUserFile(
        libHandle: SceFontLibHandle,
        file: *mut crate::ctypes::c_char,
        mode: crate::ctypes::c_int,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> SceFontHandle;
    pub fn sceFontOpenUserMemory(
        libHandle: SceFontLibHandle,
        pMemoryFont: *mut crate::ctypes::c_void,
        pMemoryFontSize: SceSize,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> SceFontHandle;
    pub fn sceFontPixelToPointH(
        libHandle: SceFontLibHandle,
        fontPixelsH: f32,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> f32;
    pub fn sceFontPixelToPointV(
        libHandle: SceFontLibHandle,
        fontPixelsV: f32,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> f32;
    pub fn sceFontPointToPixelH(
        libHandle: SceFontLibHandle,
        fontPointsH: f32,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> f32;
    pub fn sceFontPointToPixelV(
        libHandle: SceFontLibHandle,
        fontPointsV: f32,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> f32;
    pub fn sceFontSetAltCharacterCode(
        libHandle: SceFontLibHandle,
        charCode: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceFontSetResolution(
        libHandle: SceFontLibHandle,
        hRes: f32,
        vRes: f32,
    ) -> crate::ctypes::c_int;
}
#[link(name = "ScePhotoExport_stub", kind = "static")]
#[cfg(feature = "ScePhotoExport_stub")]
extern "C" {
    pub fn scePhotoExportFromData(
        data: *const crate::ctypes::c_void,
        size: SceSize,
        param: *const PhotoExportParam,
        workingMemory: *mut crate::ctypes::c_void,
        cancelCb: *mut crate::ctypes::c_void,
        user: *mut crate::ctypes::c_void,
        outPath: *mut crate::ctypes::c_char,
        outPathSize: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn scePhotoExportFromFile(
        path: *const crate::ctypes::c_char,
        param: *const PhotoExportParam,
        workingMemory: *mut crate::ctypes::c_void,
        cancelCb: *mut crate::ctypes::c_void,
        user: *mut crate::ctypes::c_void,
        outPath: *mut crate::ctypes::c_char,
        outPathSize: SceSize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "ScePmMgrForDriver_stub", kind = "static")]
#[cfg(feature = "ScePmMgrForDriver_stub")]
extern "C" {
    pub fn kscePmMgrGetProductMode(result: *mut SceProductMode) -> crate::ctypes::c_int;
    pub fn kscePmMgrIsExternalBootMode() -> crate::ctypes::c_int;
}
#[link(name = "ScePowerForDriver_stub", kind = "static")]
#[cfg(feature = "ScePowerForDriver_stub")]
extern "C" {
    pub fn kscePowerGetArmClockFrequency() -> crate::ctypes::c_int;
    pub fn kscePowerGetBatteryCycleCount() -> crate::ctypes::c_int;
    pub fn kscePowerGetBatteryFullCapacity() -> crate::ctypes::c_int;
    pub fn kscePowerGetBatteryLifePercent() -> crate::ctypes::c_int;
    pub fn kscePowerGetBatteryLifeTime() -> crate::ctypes::c_int;
    pub fn kscePowerGetBatteryRemainCapacity() -> crate::ctypes::c_int;
    pub fn kscePowerGetBatterySOH() -> crate::ctypes::c_int;
    pub fn kscePowerGetBatteryTemp() -> crate::ctypes::c_int;
    pub fn kscePowerGetBatteryVolt() -> crate::ctypes::c_int;
    pub fn kscePowerGetBusClockFrequency() -> crate::ctypes::c_int;
    pub fn kscePowerGetGpuXbarClockFrequency() -> crate::ctypes::c_int;
    pub fn kscePowerGetSysClockFrequency() -> crate::ctypes::c_int;
    pub fn kscePowerIsBatteryCharging() -> SceBool;
    pub fn kscePowerIsLowBattery() -> SceBool;
    pub fn kscePowerIsPowerOnline() -> SceBool;
    pub fn kscePowerIsSuspendRequired() -> SceBool;
    pub fn kscePowerRegisterCallback(cbid: SceUID) -> crate::ctypes::c_int;
    pub fn kscePowerRequestColdReset() -> crate::ctypes::c_int;
    pub fn kscePowerRequestDisplayOff() -> crate::ctypes::c_int;
    pub fn kscePowerRequestSoftReset() -> crate::ctypes::c_int;
    pub fn kscePowerRequestStandby() -> crate::ctypes::c_int;
    pub fn kscePowerRequestSuspend() -> crate::ctypes::c_int;
    pub fn kscePowerSetArmClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePowerSetBusClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePowerSetDisplayBrightness(brightness: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePowerSetGpuClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePowerSetGpuXbarClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn kscePowerUnregisterCallback(cbid: SceUID) -> crate::ctypes::c_int;
}
#[link(name = "ScePower_stub", kind = "static")]
#[cfg(feature = "ScePower_stub")]
extern "C" {
    pub fn scePowerGetArmClockFrequency() -> crate::ctypes::c_int;
    pub fn scePowerGetBatteryCycleCount() -> crate::ctypes::c_int;
    pub fn scePowerGetBatteryFullCapacity() -> crate::ctypes::c_int;
    pub fn scePowerGetBatteryLifePercent() -> crate::ctypes::c_int;
    pub fn scePowerGetBatteryLifeTime() -> crate::ctypes::c_int;
    pub fn scePowerGetBatteryRemainCapacity() -> crate::ctypes::c_int;
    pub fn scePowerGetBatterySOH() -> crate::ctypes::c_int;
    pub fn scePowerGetBatteryTemp() -> crate::ctypes::c_int;
    pub fn scePowerGetBatteryVolt() -> crate::ctypes::c_int;
    pub fn scePowerGetBusClockFrequency() -> crate::ctypes::c_int;
    pub fn scePowerGetGpuClockFrequency() -> crate::ctypes::c_int;
    pub fn scePowerGetGpuXbarClockFrequency() -> crate::ctypes::c_int;
    pub fn scePowerGetUsingWireless() -> crate::ctypes::c_int;
    pub fn scePowerIsBatteryCharging() -> SceBool;
    pub fn scePowerIsLowBattery() -> SceBool;
    pub fn scePowerIsPowerOnline() -> SceBool;
    pub fn scePowerIsSuspendRequired() -> SceBool;
    pub fn scePowerRegisterCallback(cbid: SceUID) -> crate::ctypes::c_int;
    pub fn scePowerRequestColdReset() -> crate::ctypes::c_int;
    pub fn scePowerRequestDisplayOff() -> crate::ctypes::c_int;
    pub fn scePowerRequestDisplayOn() -> crate::ctypes::c_int;
    pub fn scePowerRequestStandby() -> crate::ctypes::c_int;
    pub fn scePowerRequestSuspend() -> crate::ctypes::c_int;
    pub fn scePowerSetArmClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn scePowerSetBusClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn scePowerSetConfigurationMode(conf: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn scePowerSetGpuClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn scePowerSetGpuXbarClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn scePowerSetUsingWireless(enabled: SceBool) -> crate::ctypes::c_int;
    pub fn scePowerUnregisterCallback(cbid: SceUID) -> crate::ctypes::c_int;
}
#[link(name = "SceProcessmgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceProcessmgrForDriver_stub")]
extern "C" {
    pub fn ksceKernelCreateProcessLocalStorage(
        name: *const crate::ctypes::c_char,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetProcessInfo(
        pid: SceUID,
        info: *mut SceKernelProcessInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetProcessLocalStorageAddr(
        key: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_void;
    pub fn ksceKernelGetProcessLocalStorageAddrForPid(
        pid: SceUID,
        key: crate::ctypes::c_int,
        out_addr: *mut *mut crate::ctypes::c_void,
        create_if_doesnt_exist: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetProcessStatus(
        pid: SceUID,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceProcessmgrForKernel_363_stub", kind = "static")]
#[cfg(feature = "SceProcessmgrForKernel_363_stub")]
extern "C" {}
#[cfg(any(
    feature = "SceProcessmgrForKernel_363_stub",
    feature = "SceProcessmgrForKernel_stub"
))]
extern "C" {
    pub fn ksceKernelGetProcessKernelBuf(pid: SceUID) -> *mut crate::ctypes::c_void;
}
#[link(name = "SceProcessmgrForKernel_stub", kind = "static")]
#[cfg(feature = "SceProcessmgrForKernel_stub")]
extern "C" {
    pub fn ksceKernelCreateProcess(
        titleid: *const crate::ctypes::c_char,
        type_: SceKernelProcessType,
        path: *const crate::ctypes::c_char,
        opt: *mut crate::ctypes::c_void,
    ) -> SceUID;
    pub fn ksceKernelGetProcessMainThread(pid: SceUID) -> SceUID;
    pub fn ksceKernelGetProcessModuleInfo(pid: SceUID) -> ScePVoid;
    pub fn ksceKernelGetProcessSelfAuthInfo(
        pid: SceUID,
        self_auth_info: *mut SceSelfAuthInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelResumeProcess(pid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelSuspendProcess(
        pid: SceUID,
        status: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceProcessmgr_stub", kind = "static")]
#[cfg(feature = "SceProcessmgr_stub")]
extern "C" {
    pub fn sceKernelGetCurrentProcess() -> SceUID;
    pub fn sceKernelGetProcessParam() -> *const crate::ctypes::c_void;
    pub fn sceKernelGetRemoteProcessTime(
        processId: SceUID,
        pClock: *mut SceKernelSysClock,
    ) -> SceInt32;
    pub fn sceKernelGetStderr() -> SceUID;
    pub fn sceKernelGetStdin() -> SceUID;
    pub fn sceKernelGetStdout() -> SceUID;
    pub fn sceKernelLibcClock() -> SceKernelClock;
    pub fn sceKernelLibcGettimeofday(
        tv: *mut SceKernelTimeval,
        tz: *mut SceKernelTimezone,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelLibcTime(tloc: *mut SceKernelTime) -> SceKernelTime;
    pub fn sceKernelPowerLock(type_: SceKernelPowerTickType) -> crate::ctypes::c_int;
    pub fn sceKernelPowerTick(type_: SceKernelPowerTickType) -> crate::ctypes::c_int;
    pub fn sceKernelPowerUnlock(type_: SceKernelPowerTickType) -> crate::ctypes::c_int;
}
#[link(name = "SceProcEventForDriver_stub", kind = "static")]
#[cfg(feature = "SceProcEventForDriver_stub")]
extern "C" {
    pub fn ksceKernelInvokeProcEventHandler(
        pid: SceUID,
        event_id: crate::ctypes::c_int,
        event_type: crate::ctypes::c_int,
        param: *mut crate::ctypes::c_void,
        a5: *mut crate::ctypes::c_void,
        a6: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelRegisterProcEventHandler(
        name: *const crate::ctypes::c_char,
        handler: *const SceProcEventHandler,
        a3: crate::ctypes::c_int,
    ) -> SceUID;
    pub fn ksceKernelUnregisterProcEventHandler(uid: SceUID) -> crate::ctypes::c_int;
}
#[link(name = "ScePromoterUtil_stub", kind = "static")]
#[cfg(feature = "ScePromoterUtil_stub")]
extern "C" {
    pub fn scePromoterUtilityCheckExist(
        titleid: *const crate::ctypes::c_char,
        res: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn scePromoterUtilityDeletePkg(
        titleid: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn scePromoterUtilityExit() -> crate::ctypes::c_int;
    pub fn scePromoterUtilityGetResult(res: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn scePromoterUtilityGetState(state: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn scePromoterUtilityInit() -> crate::ctypes::c_int;
    pub fn scePromoterUtilityPromoteImport(
        params: *mut ScePromoterUtilityImportParams,
    ) -> crate::ctypes::c_int;
    pub fn scePromoterUtilityPromotePkg(
        path: *const crate::ctypes::c_char,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn scePromoterUtilityPromotePkgWithRif(
        path: *const crate::ctypes::c_char,
        sync: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn scePromoterUtilityUpdateLiveArea(
        args: *mut ScePromoterUtilityLAUpdate,
    ) -> crate::ctypes::c_int;
}
#[link(name = "ScePsmDrmForDriver_stub", kind = "static")]
#[cfg(feature = "ScePsmDrmForDriver_stub")]
extern "C" {}
#[link(name = "ScePspnetAdhoc_stub", kind = "static")]
#[cfg(feature = "ScePspnetAdhoc_stub")]
extern "C" {
    pub fn sceNetAdhocGetPdpStat(
        buflen: *mut crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocGetPtpStat(
        buflen: *mut crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocGetSocketAlert(
        id: crate::ctypes::c_int,
        flag: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocInit() -> crate::ctypes::c_int;
    pub fn sceNetAdhocPdpCreate(
        saddr: *const SceNetEtherAddr,
        sport: SceUShort16,
        bufsize: crate::ctypes::c_uint,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPdpDelete(
        id: crate::ctypes::c_int,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPdpRecv(
        id: crate::ctypes::c_int,
        saddr: *mut SceNetEtherAddr,
        sport: *mut SceUShort16,
        buf: *mut crate::ctypes::c_void,
        len: *mut crate::ctypes::c_int,
        timeout: crate::ctypes::c_uint,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPdpSend(
        id: crate::ctypes::c_int,
        daddr: *const SceNetEtherAddr,
        dport: SceUShort16,
        data: *const crate::ctypes::c_void,
        len: crate::ctypes::c_int,
        timeout: crate::ctypes::c_uint,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPollSocket(
        sds: *mut SceNetAdhocPollSd,
        nsds: crate::ctypes::c_int,
        timeout: crate::ctypes::c_uint,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPtpAccept(
        id: crate::ctypes::c_int,
        addr: *mut SceNetEtherAddr,
        port: *mut SceUShort16,
        timeout: crate::ctypes::c_uint,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPtpClose(
        id: crate::ctypes::c_int,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPtpConnect(
        id: crate::ctypes::c_int,
        timeout: crate::ctypes::c_uint,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPtpFlush(
        id: crate::ctypes::c_int,
        timeout: crate::ctypes::c_uint,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPtpListen(
        saddr: *const SceNetEtherAddr,
        sport: SceUShort16,
        bufsize: crate::ctypes::c_uint,
        rexmt_int: crate::ctypes::c_uint,
        rexmt_cnt: crate::ctypes::c_int,
        backlog: crate::ctypes::c_int,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPtpOpen(
        saddr: *const SceNetEtherAddr,
        sport: SceUShort16,
        daddr: *const SceNetEtherAddr,
        dport: SceUShort16,
        bufsize: crate::ctypes::c_uint,
        rexmt_int: crate::ctypes::c_uint,
        rexmt_cnt: crate::ctypes::c_int,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPtpRecv(
        id: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
        len: *mut crate::ctypes::c_int,
        timeout: crate::ctypes::c_uint,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocPtpSend(
        id: crate::ctypes::c_int,
        data: *const crate::ctypes::c_void,
        len: *mut crate::ctypes::c_int,
        timeout: crate::ctypes::c_uint,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocSetSocketAlert(
        id: crate::ctypes::c_int,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocTerm() -> crate::ctypes::c_int;
    pub fn sceNetAdhocctlGetAddrByName(
        nickname: *const SceNetAdhocctlNickname,
        buflen: *mut crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocctlGetAdhocId(adhocId: *mut SceNetAdhocctlAdhocId) -> crate::ctypes::c_int;
    pub fn sceNetAdhocctlGetEtherAddr(addr: *mut SceNetEtherAddr) -> crate::ctypes::c_int;
    pub fn sceNetAdhocctlGetNameByAddr(
        addr: *const SceNetEtherAddr,
        nickname: *mut SceNetAdhocctlNickname,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocctlGetParameter(
        parameter: *mut SceNetAdhocctlParameter,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocctlGetPeerInfo(
        addr: *const SceNetEtherAddr,
        size: crate::ctypes::c_int,
        peerInfo: *mut SceNetAdhocctlPeerInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocctlGetPeerList(
        buflen: *mut crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceNetAdhocctlInit(adhocId: *const SceNetAdhocctlAdhocId) -> crate::ctypes::c_int;
    pub fn sceNetAdhocctlTerm() -> crate::ctypes::c_int;
}
#[link(name = "ScePvf_stub", kind = "static")]
#[cfg(feature = "ScePvf_stub")]
extern "C" {
    pub fn scePvfClose(fontID: ScePvfFontId) -> ScePvfError;
    pub fn scePvfDoneLib(libID: ScePvfLibId) -> ScePvfError;
    pub fn scePvfFindFont(
        libID: ScePvfLibId,
        fontStyleInfo: *mut ScePvfFontStyleInfo,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFontIndex;
    pub fn scePvfFindOptimumFont(
        libID: ScePvfLibId,
        fontStyleInfo: *mut ScePvfFontStyleInfo,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFontIndex;
    pub fn scePvfFlush(fontID: ScePvfFontId) -> ScePvfError;
    pub fn scePvfGetCharGlyphImage(
        fontID: ScePvfFontId,
        charCode: ScePvfCharCode,
        imageBuffer: *mut ScePvfUserImageBufferRec,
    ) -> ScePvfError;
    pub fn scePvfGetCharGlyphImage_Clip(
        fontID: ScePvfFontId,
        charCode: ScePvfCharCode,
        imageBuffer: *mut ScePvfUserImageBufferRec,
        clipX: ScePvfS32,
        clipY: ScePvfS32,
        clipWidth: ScePvfU32,
        clipHeight: ScePvfU32,
    ) -> ScePvfError;
    pub fn scePvfGetCharImageRect(
        fontID: ScePvfFontId,
        charCode: ScePvfCharCode,
        rect: *mut ScePvfIrect,
    ) -> ScePvfError;
    pub fn scePvfGetCharInfo(
        fontID: ScePvfFontId,
        charCode: ScePvfCharCode,
        charInfo: *mut ScePvfCharInfo,
    ) -> ScePvfError;
    pub fn scePvfGetFontInfo(fontID: ScePvfFontId, fontInfo: *mut ScePvfFontInfo) -> ScePvfError;
    pub fn scePvfGetFontInfoByIndexNumber(
        libID: ScePvfLibId,
        fontStyleInfo: *mut ScePvfFontStyleInfo,
        fontIndex: ScePvfFontIndex,
    ) -> ScePvfError;
    pub fn scePvfGetFontList(
        libID: ScePvfLibId,
        fontStyleInfo: *mut ScePvfFontStyleInfo,
        arraySize: ScePvfInt,
    ) -> ScePvfError;
    pub fn scePvfGetKerningInfo(
        fontID: ScePvfFontId,
        leftCharCode: ScePvfCharCode,
        rightCharCode: ScePvfCharCode,
        pKerningInfo: *mut ScePvfKerningInfo,
    ) -> ScePvfError;
    pub fn scePvfGetNumFontList(libID: ScePvfLibId, errorCode: *mut ScePvfError) -> ScePvfInt;
    pub fn scePvfIsElement(fontID: ScePvfFontId, charCode: ScePvfCharCode) -> ScePvfBool;
    pub fn scePvfNewLib(initParam: *mut ScePvfInitRec, errorCode: *mut ScePvfError) -> ScePvfLibId;
    pub fn scePvfOpen(
        libID: ScePvfLibId,
        fontIndex: ScePvfFontIndex,
        mode: ScePvfU32,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFontId;
    pub fn scePvfOpenDefaultJapaneseFontOnSharedMemory(
        libID: ScePvfLibId,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFontId;
    pub fn scePvfOpenDefaultLatinFontOnSharedMemory(
        libID: ScePvfLibId,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFontId;
    pub fn scePvfOpenUserFile(
        libID: ScePvfLibId,
        filename: ScePvfPointer,
        mode: ScePvfU32,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFontId;
    pub fn scePvfOpenUserFileWithSubfontIndex(
        libID: ScePvfLibId,
        filename: ScePvfPointer,
        mode: ScePvfU32,
        subFontIndex: ScePvfU32,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFontId;
    pub fn scePvfOpenUserMemory(
        libID: ScePvfLibId,
        addr: ScePvfPointer,
        size: ScePvfU32,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFontId;
    pub fn scePvfOpenUserMemoryWithSubfontIndex(
        libID: ScePvfLibId,
        addr: ScePvfPointer,
        size: ScePvfU32,
        subFontIndex: ScePvfU32,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFontId;
    pub fn scePvfPixelToPointH(
        libID: ScePvfLibId,
        pixel: ScePvfFloat32,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFloat32;
    pub fn scePvfPixelToPointV(
        libID: ScePvfLibId,
        pixel: ScePvfFloat32,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFloat32;
    pub fn scePvfPointToPixelH(
        libID: ScePvfLibId,
        point: ScePvfFloat32,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFloat32;
    pub fn scePvfPointToPixelV(
        libID: ScePvfLibId,
        point: ScePvfFloat32,
        errorCode: *mut ScePvfError,
    ) -> ScePvfFloat32;
    pub fn scePvfSetAltCharacterCode(libID: ScePvfLibId, charCode: ScePvfCharCode) -> ScePvfError;
    pub fn scePvfSetCharSize(
        fontID: ScePvfFontId,
        hSize: ScePvfFloat32,
        vSize: ScePvfFloat32,
    ) -> ScePvfError;
    pub fn scePvfSetEM(libID: ScePvfLibId, emValue: ScePvfFloat32) -> ScePvfError;
    pub fn scePvfSetEmboldenRate(fontID: ScePvfFontId, emboldenRate: ScePvfFloat32) -> ScePvfError;
    pub fn scePvfSetResolution(
        libID: ScePvfLibId,
        hResolution: ScePvfFloat32,
        vResolution: ScePvfFloat32,
    ) -> ScePvfError;
    pub fn scePvfSetSkewValue(
        fontID: ScePvfFontId,
        angleX: ScePvfFloat32,
        angleY: ScePvfFloat32,
    ) -> ScePvfError;
}
#[link(name = "SceQafMgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceQafMgrForDriver_stub")]
extern "C" {}
#[link(name = "SceRazorCapture_stub", kind = "static")]
#[cfg(feature = "SceRazorCapture_stub")]
extern "C" {
    pub fn sceRazorGpuCaptureEnableSalvage(filename: *const crate::ctypes::c_char);
    pub fn sceRazorGpuCaptureSetTriggerNextFrame(filename: *const crate::ctypes::c_char);
}
#[link(name = "SceRazorHud_stub", kind = "static")]
#[cfg(feature = "SceRazorHud_stub")]
extern "C" {
    pub fn sceRazorGpuLiveSetBuffer(
        buffer: *mut crate::ctypes::c_void,
        buf_size: SceSize,
        results: *mut SceRazorGpuLiveResultInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceRazorGpuLiveSetMetricsGroup(metrics: u32) -> crate::ctypes::c_int;
    pub fn sceRazorGpuLiveStart() -> crate::ctypes::c_int;
    pub fn sceRazorGpuLiveStop() -> crate::ctypes::c_int;
}
#[link(name = "SceRegistryMgr_stub", kind = "static")]
#[cfg(feature = "SceRegistryMgr_stub")]
extern "C" {
    pub fn sceRegMgrGetInitVals(
        category: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_char,
        elements_number: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrGetKeyBin(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_void,
        size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrGetKeyInt(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrGetKeyStr(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_char,
        size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrGetKeys(
        category: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_char,
        elements_number: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrGetRegVersion(
        version: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrSetKeyBin(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_void,
        size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrSetKeyInt(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrSetKeyStr(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_char,
        size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrSetKeys(
        category: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_char,
        elements_number: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrSystemParamGetInt(
        id: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRegMgrSystemParamGetStr(
        id: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_char,
        size: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceRegMgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceRegMgrForDriver_stub")]
extern "C" {
    pub fn ksceRegMgrGetKeyBin(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceRegMgrGetKeyInt(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceRegMgrGetKeyStr(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_char,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceRegMgrSetKeyBin(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceRegMgrSetKeyInt(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceRegMgrSetKeyStr(
        category: *const crate::ctypes::c_char,
        name: *const crate::ctypes::c_char,
        buf: *mut crate::ctypes::c_char,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceRegMgrServiceForDriver_stub", kind = "static")]
#[cfg(feature = "SceRegMgrServiceForDriver_stub")]
extern "C" {}
#[link(name = "SceRtabi_stub", kind = "static")]
#[cfg(feature = "SceRtabi_stub")]
extern "C" {}
#[link(name = "SceRtcForDriver_stub", kind = "static")]
#[cfg(feature = "SceRtcForDriver_stub")]
extern "C" {
    pub fn ksceRtcConvertDateTimeToUnixTime(
        src: *const SceDateTime,
        dst: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn ksceRtcConvertTickToDateTime(
        dst: *mut SceDateTime,
        src: *mut SceRtcTick,
    ) -> crate::ctypes::c_int;
    pub fn ksceRtcGetCurrentClock(
        time: *mut SceDateTime,
        time_zone: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceRtcGetCurrentClockLocalTime(time: *mut SceDateTime) -> crate::ctypes::c_int;
    pub fn ksceRtcGetCurrentDebugNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn ksceRtcGetCurrentNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn ksceRtcGetCurrentSecureTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn ksceRtcGetCurrentTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn ksceRtcSetCurrentDebugNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn ksceRtcSetCurrentNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn ksceRtcSetCurrentSecureTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn ksceRtcSetCurrentTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
}
#[link(name = "SceRtc_stub", kind = "static")]
#[cfg(feature = "SceRtc_stub")]
extern "C" {
    pub fn _sceRtcConvertLocalTimeToUtc(
        localtime: *const SceRtcTick,
        utc: *mut SceRtcTick,
    ) -> crate::ctypes::c_int;
    pub fn _sceRtcConvertUtcToLocalTime(
        utc: *const SceRtcTick,
        localtime: *mut SceRtcTick,
    ) -> crate::ctypes::c_int;
    pub fn _sceRtcFormatRFC2822(
        datetime: *mut crate::ctypes::c_char,
        utc: *const SceRtcTick,
        offset: crate::ctypes::c_int,
        a4: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn _sceRtcFormatRFC2822LocalTime(
        datetime: *mut crate::ctypes::c_char,
        utc: *const SceRtcTick,
        a3: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn _sceRtcFormatRFC3339(
        datetime: *mut crate::ctypes::c_char,
        utc: *const SceRtcTick,
        offset: crate::ctypes::c_int,
        a4: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn _sceRtcFormatRFC3339LocalTime(
        datetime: *mut crate::ctypes::c_char,
        utc: *const SceRtcTick,
        a3: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcCheckValid(time: *const SceDateTime) -> crate::ctypes::c_int;
    pub fn sceRtcCompareTick(
        pTick1: *const SceRtcTick,
        pTick2: *const SceRtcTick,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcConvertLocalTimeToUtc(
        local_time: *const SceRtcTick,
        utc: *mut SceRtcTick,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcConvertUtcToLocalTime(
        utc: *const SceRtcTick,
        local_time: *mut SceRtcTick,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcFormatRFC2822(
        pszDateTime: *mut crate::ctypes::c_char,
        utc: *const SceRtcTick,
        iTimeZoneMinutes: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcFormatRFC2822LocalTime(
        pszDateTime: *mut crate::ctypes::c_char,
        utc: *const SceRtcTick,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcFormatRFC3339(
        pszDateTime: *mut crate::ctypes::c_char,
        utc: *const SceRtcTick,
        iTimeZoneMinutes: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcFormatRFC3339LocalTime(
        pszDateTime: *mut crate::ctypes::c_char,
        utc: *const SceRtcTick,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcGetAccumulativeTime() -> SceULong64;
    pub fn sceRtcGetCurrentAdNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceRtcGetCurrentClock(
        time: *mut SceDateTime,
        time_zone: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcGetCurrentClockLocalTime(time: *mut SceDateTime) -> crate::ctypes::c_int;
    pub fn sceRtcGetCurrentDebugNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceRtcGetCurrentGpsTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceRtcGetCurrentNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceRtcGetCurrentRetainedNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceRtcGetCurrentTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceRtcGetDayOfWeek(
        year: crate::ctypes::c_int,
        month: crate::ctypes::c_int,
        day: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcGetDaysInMonth(
        year: crate::ctypes::c_int,
        month: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcGetDosTime(
        time: *const SceDateTime,
        puiDosTime: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcGetLastAdjustedTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceRtcGetLastReincarnatedTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceRtcGetTick(time: *const SceDateTime, tick: *mut SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceRtcGetTickResolution() -> crate::ctypes::c_uint;
    pub fn sceRtcGetTime64_t(
        time: *const SceDateTime,
        pullTime: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcGetTime_t(time: *const SceDateTime, piTime: *mut time_t) -> crate::ctypes::c_int;
    pub fn sceRtcGetWin32FileTime(
        time: *const SceDateTime,
        ulWin32Time: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcIsLeapYear(year: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceRtcParseDateTime(
        utc: *mut SceRtcTick,
        pszDateTime: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcParseRFC3339(
        utc: *mut SceRtcTick,
        pszDateTime: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcSetDosTime(
        time: *mut SceDateTime,
        uiDosTime: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcSetTick(time: *mut SceDateTime, tick: *const SceRtcTick) -> crate::ctypes::c_int;
    pub fn sceRtcSetTime64_t(time: *mut SceDateTime, ullTime: SceUInt64) -> crate::ctypes::c_int;
    pub fn sceRtcSetTime_t(time: *mut SceDateTime, iTime: time_t) -> crate::ctypes::c_int;
    pub fn sceRtcSetWin32FileTime(
        time: *mut SceDateTime,
        ulWin32Time: SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcTickAddDays(
        pTick0: *mut SceRtcTick,
        pTick1: *const SceRtcTick,
        lAdd: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcTickAddHours(
        pTick0: *mut SceRtcTick,
        pTick1: *const SceRtcTick,
        lAdd: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcTickAddMicroseconds(
        pTick0: *mut SceRtcTick,
        pTick1: *const SceRtcTick,
        lAdd: SceLong64,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcTickAddMinutes(
        pTick0: *mut SceRtcTick,
        pTick1: *const SceRtcTick,
        lAdd: SceLong64,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcTickAddMonths(
        pTick0: *mut SceRtcTick,
        pTick1: *const SceRtcTick,
        lAdd: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcTickAddSeconds(
        pTick0: *mut SceRtcTick,
        pTick1: *const SceRtcTick,
        lAdd: SceLong64,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcTickAddTicks(
        pTick0: *mut SceRtcTick,
        pTick1: *const SceRtcTick,
        lAdd: SceLong64,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcTickAddWeeks(
        pTick0: *mut SceRtcTick,
        pTick1: *const SceRtcTick,
        lAdd: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceRtcTickAddYears(
        pTick0: *mut SceRtcTick,
        pTick1: *const SceRtcTick,
        lAdd: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceRudp_stub", kind = "static")]
#[cfg(feature = "SceRudp_stub")]
extern "C" {}
#[link(name = "SceSas_stub", kind = "static")]
#[cfg(feature = "SceSas_stub")]
extern "C" {}
#[link(name = "SceSblACMgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceSblACMgrForDriver_stub")]
extern "C" {
    pub fn ksceSblACMgrGetMediaType(
        path: *const crate::ctypes::c_char,
        media_type: *mut SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn ksceSblACMgrGetProcessProgramAuthId(
        pid: SceUID,
        authid: *mut SceUInt64,
    ) -> crate::ctypes::c_int;
    pub fn ksceSblACMgrIsDevelopmentMode() -> crate::ctypes::c_int;
    pub fn ksceSblACMgrIsFself(pid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceSblACMgrIsGameProgram(pid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceSblACMgrIsNonGameProgram(pid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceSblACMgrIsPspEmu(pid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceSblACMgrIsRootProgram(pid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceSblACMgrIsSceShell(pid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceSblACMgrIsSystemProgram(pid: SceUID) -> crate::ctypes::c_int;
}
#[link(name = "SceSblACMgrForKernel_stub", kind = "static")]
#[cfg(feature = "SceSblACMgrForKernel_stub")]
extern "C" {}
#[link(name = "SceSblACMgr_stub", kind = "static")]
#[cfg(feature = "SceSblACMgr_stub")]
extern "C" {}
#[link(name = "SceSblAIMgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceSblAIMgrForDriver_stub")]
extern "C" {
    pub fn ksceSblAimgrGetProductCode() -> crate::ctypes::c_int;
    pub fn ksceSblAimgrGetProductSubCode() -> crate::ctypes::c_int;
    pub fn ksceSblAimgrGetSMI(info: *mut SceUInt32) -> crate::ctypes::c_int;
    pub fn ksceSblAimgrIsCEX() -> crate::ctypes::c_int;
    pub fn ksceSblAimgrIsDEX() -> crate::ctypes::c_int;
    pub fn ksceSblAimgrIsDolce() -> crate::ctypes::c_int;
    pub fn ksceSblAimgrIsGenuineDolce() -> crate::ctypes::c_int;
    pub fn ksceSblAimgrIsGenuineVITA() -> crate::ctypes::c_int;
    pub fn ksceSblAimgrIsTest() -> crate::ctypes::c_int;
    pub fn ksceSblAimgrIsTool() -> crate::ctypes::c_int;
    pub fn ksceSblAimgrIsVITA() -> crate::ctypes::c_int;
}
#[link(name = "SceSblAuthMgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceSblAuthMgrForDriver_stub")]
extern "C" {}
#[link(name = "SceSblAuthMgrForKernel_stub", kind = "static")]
#[cfg(feature = "SceSblAuthMgrForKernel_stub")]
extern "C" {
    pub fn ksceSblAuthMgrClearDmac5Key(
        slot_id: crate::ctypes::c_int,
        val: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceSblAuthMgrSetDmac5Key(
        key: *const crate::ctypes::c_void,
        keylen: SceSize,
        slot_id: crate::ctypes::c_int,
        key_id: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSblFwLoaderForDriver_stub", kind = "static")]
#[cfg(feature = "SceSblFwLoaderForDriver_stub")]
extern "C" {}
#[link(name = "SceSblGcAuthMgr_stub", kind = "static")]
#[cfg(feature = "SceSblGcAuthMgr_stub")]
extern "C" {}
#[link(name = "SceSblPostSsMgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceSblPostSsMgrForDriver_stub")]
extern "C" {
    pub fn ksceSblRSA2048CreateSignature(
        rsa_signature: *mut SceSblRsaDataParam,
        hash: *mut SceSblRsaDataParam,
        private_key: *mut SceSblRsaPrivateKeyParam,
        type_: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceSblRSA2048VerifySignature(
        rsa_signature: *mut SceSblRsaDataParam,
        hash: *mut SceSblRsaDataParam,
        public_key: *mut SceSblRsaPublicKeyParam,
        type_: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSblPostSsMgr_stub", kind = "static")]
#[cfg(feature = "SceSblPostSsMgr_stub")]
extern "C" {}
#[link(name = "SceSblSmCommForKernel_stub", kind = "static")]
#[cfg(feature = "SceSblSmCommForKernel_stub")]
extern "C" {
    pub fn ksceSblSmCommCallFunc(
        id: SceSblSmCommId,
        service_id: SceUInt32,
        service_result: *mut SceUInt32,
        data: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceSblSmCommStartSmFromFile(
        priority: SceUInt32,
        sm_path: *const crate::ctypes::c_char,
        cmd_id: SceUInt32,
        auth_info: *mut SceAuthInfo,
        id: *mut SceSblSmCommId,
    ) -> crate::ctypes::c_int;
    pub fn ksceSblSmCommStopSm(
        id: SceSblSmCommId,
        result: *mut SceSblSmCommPair,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSblSmSchedProxyForKernel_stub", kind = "static")]
#[cfg(feature = "SceSblSmSchedProxyForKernel_stub")]
extern "C" {}
#[link(name = "SceSblSsMgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceSblSsMgrForDriver_stub")]
extern "C" {
    pub fn ksceSblAimgrGetConsoleId(cid: *mut SceConsoleId) -> crate::ctypes::c_int;
    pub fn ksceSblAimgrGetOpenPsId(open_psid: *mut SceOpenPsId) -> crate::ctypes::c_int;
    pub fn ksceSblAimgrGetPscode(pscode: *mut ScePsCode) -> crate::ctypes::c_int;
    pub fn ksceSblDmac5AesCbcDec(
        src: *const crate::ctypes::c_void,
        dst: *mut crate::ctypes::c_void,
        size: crate::ctypes::c_int,
        key: *const crate::ctypes::c_void,
        key_size: crate::ctypes::c_int,
        iv: *mut crate::ctypes::c_void,
        mask_enable: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceSblDmac5AesCbcEnc(
        src: *const crate::ctypes::c_void,
        dst: *mut crate::ctypes::c_void,
        size: crate::ctypes::c_int,
        key: *const crate::ctypes::c_void,
        key_size: crate::ctypes::c_int,
        iv: *mut crate::ctypes::c_void,
        mask_enable: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceSblDmac5AesCtrDec(
        src: *const crate::ctypes::c_void,
        dst: *mut crate::ctypes::c_void,
        size: crate::ctypes::c_int,
        key: *const crate::ctypes::c_void,
        key_size: crate::ctypes::c_int,
        iv: *mut crate::ctypes::c_void,
        mask_enable: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceSblRngPseudoRandomNumber(
        result: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceSblSsDecryptWithPortability(
        key_type: SceUInt32,
        iv: *mut crate::ctypes::c_void,
        src: *mut ScePortabilityData,
        dst: *mut ScePortabilityData,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSblSsMgrForKernel_stub", kind = "static")]
#[cfg(feature = "SceSblSsMgrForKernel_stub")]
extern "C" {}
#[link(name = "SceSblSsMgr_stub", kind = "static")]
#[cfg(feature = "SceSblSsMgr_stub")]
extern "C" {
    pub fn sceSblDmac5EncDec(
        param: *mut SceSblDmac5EncDecParam,
        command: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceSblDmac5HashTransform(
        param: *mut SceSblDmac5HashTransformParam,
        command: SceUInt32,
        extra: SceUInt32,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSblUpdateMgr_stub", kind = "static")]
#[cfg(feature = "SceSblUpdateMgr_stub")]
extern "C" {
    pub fn sceSblUsGetUpdateMode(mode: *mut SceUpdateMode) -> crate::ctypes::c_int;
    pub fn sceSblUsSetUpdateMode(mode: SceUpdateMode) -> crate::ctypes::c_int;
    pub fn sceSblUsVerifyPup(path: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
}
#[link(name = "SceScreenShot_stub", kind = "static")]
#[cfg(feature = "SceScreenShot_stub")]
extern "C" {
    pub fn sceScreenShotDisable() -> crate::ctypes::c_int;
    pub fn sceScreenShotEnable() -> crate::ctypes::c_int;
    pub fn sceScreenShotSetOverlayImage(
        filepath: *const crate::ctypes::c_char,
        offsetX: crate::ctypes::c_int,
        offsetY: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceScreenShotSetParam(param: *const SceScreenShotParam) -> crate::ctypes::c_int;
}
#[link(name = "SceSdifForDriver_stub", kind = "static")]
#[cfg(feature = "SceSdifForDriver_stub")]
extern "C" {}
#[link(name = "SceShaccCg_stub", kind = "static")]
#[cfg(feature = "SceShaccCg_stub")]
extern "C" {
    pub fn sceShaccCgCompileProgram(
        options: *const SceShaccCgCompileOptions,
        callbacks: *const SceShaccCgCallbackList,
        unk: crate::ctypes::c_int,
    ) -> *const SceShaccCgCompileOutput;
    pub fn sceShaccCgDestroyCompileOutput(output: *const SceShaccCgCompileOutput);
    pub fn sceShaccCgGetVersionString() -> *const crate::ctypes::c_char;
    pub fn sceShaccCgInitializeCallbackList(
        callbacks: *mut SceShaccCgCallbackList,
        defaults: SceShaccCgCallbackDefaults,
    );
    pub fn sceShaccCgInitializeCompileOptions(
        options: *mut SceShaccCgCompileOptions,
    ) -> crate::ctypes::c_int;
    pub fn sceShaccCgReleaseCompiler();
    pub fn sceShaccCgSetDefaultAllocator(
        malloc_cb: ::core::option::Option<
            unsafe extern "C" fn(arg1: crate::ctypes::c_uint) -> *mut crate::ctypes::c_void,
        >,
        free_cb: ::core::option::Option<unsafe extern "C" fn(arg1: *mut crate::ctypes::c_void)>,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceShellSvc_stub", kind = "static")]
#[cfg(feature = "SceShellSvc_stub")]
extern "C" {
    pub fn sceShellUtilInitEvents(unk: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceShellUtilLaunchAppRequestLaunchApp(
        param: *mut SceShellUtilLaunchAppParam,
    ) -> crate::ctypes::c_int;
    pub fn sceShellUtilLock(type_: SceShellUtilLockType) -> crate::ctypes::c_int;
    pub fn sceShellUtilRegisterEventHandler(
        handler: SceShellUtilEventHandler,
        userData: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceShellUtilRequestLaunchApp(
        param: *mut SceShellUtilLaunchAppParam,
    ) -> crate::ctypes::c_int;
    pub fn sceShellUtilUnlock(type_: SceShellUtilLockType) -> crate::ctypes::c_int;
}
#[link(name = "SceShutterSound_stub", kind = "static")]
#[cfg(feature = "SceShutterSound_stub")]
extern "C" {
    pub fn sceShutterSoundPlay(type_: u32) -> crate::ctypes::c_int;
}
#[link(name = "SceSmart_stub", kind = "static")]
#[cfg(feature = "SceSmart_stub")]
extern "C" {}
#[link(name = "SceSqlite_stub", kind = "static")]
#[cfg(feature = "SceSqlite_stub")]
extern "C" {
    pub fn sceSqliteConfigMallocMethods(
        methods: *mut SceSqliteMallocMethods,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSsl_stub", kind = "static")]
#[cfg(feature = "SceSsl_stub")]
extern "C" {
    pub fn sceSslFreeSslCertName(certName: *mut SceSslCertName) -> crate::ctypes::c_int;
    pub fn sceSslGetIssuerName(sslCert: *mut SceSslCert) -> *mut SceSslCertName;
    pub fn sceSslGetMemoryPoolStats(
        currentStat: *mut SceSslMemoryPoolStats,
    ) -> crate::ctypes::c_int;
    pub fn sceSslGetNameEntryCount(certName: *mut SceSslCertName) -> crate::ctypes::c_int;
    pub fn sceSslGetNameEntryInfo(
        certName: *mut SceSslCertName,
        entryNum: crate::ctypes::c_int,
        oidname: *mut crate::ctypes::c_char,
        maxOidnameLen: crate::ctypes::c_uint,
        value: *mut crate::ctypes::c_char,
        maxValueLen: crate::ctypes::c_uint,
        valueLen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceSslGetNotAfter(
        sslCert: *mut SceSslCert,
        limit: *mut SceRtcTick,
    ) -> crate::ctypes::c_int;
    pub fn sceSslGetNotBefore(
        sslCert: *mut SceSslCert,
        begin: *mut SceRtcTick,
    ) -> crate::ctypes::c_int;
    pub fn sceSslGetSerialNumber(
        sslCert: *mut SceSslCert,
        sboData: *mut *const crate::ctypes::c_char,
        sboLen: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceSslGetSubjectName(sslCert: *mut SceSslCert) -> *mut SceSslCertName;
    pub fn sceSslInit(poolSize: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn sceSslTerm() -> crate::ctypes::c_int;
}
#[link(name = "SceStdio_0931_stub", kind = "static")]
#[cfg(feature = "SceStdio_0931_stub")]
extern "C" {}
#[link(name = "SceSulpha_stub", kind = "static")]
#[cfg(feature = "SceSulpha_stub")]
extern "C" {}
#[link(name = "SceSysclibForDriver_stub", kind = "static")]
#[cfg(feature = "SceSysclibForDriver_stub")]
extern "C" {
    pub fn __memcpy_chk(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        dst_len: crate::ctypes::c_uint,
    ) -> *mut crate::ctypes::c_void;
    pub fn __memmove_chk(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        dst_len: crate::ctypes::c_uint,
    ) -> *mut crate::ctypes::c_void;
    pub fn __memset_chk(
        dst: *mut crate::ctypes::c_void,
        ch: crate::ctypes::c_int,
        len: crate::ctypes::c_uint,
        dst_len: crate::ctypes::c_uint,
    ) -> *mut crate::ctypes::c_void;
    pub fn __prnt(
        callback: SceSysclibPrntCallback,
        argp: *mut crate::ctypes::c_void,
        fmt: *const crate::ctypes::c_char,
        list: va_list,
    );
    pub fn __strncpy_chk(
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        n: crate::ctypes::c_uint,
        dst_len: crate::ctypes::c_uint,
    ) -> *mut crate::ctypes::c_char;
    pub fn look_ctype_table(ch: crate::ctypes::c_char) -> crate::ctypes::c_char;
    pub fn strnlen(
        s: *const crate::ctypes::c_char,
        n: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_uint;
    pub fn timingsafe_memcmp(
        s1: *const crate::ctypes::c_void,
        s2: *const crate::ctypes::c_void,
        n: usize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSysconForDriver_stub", kind = "static")]
#[cfg(feature = "SceSysconForDriver_stub")]
extern "C" {
    pub fn ksceSysconBeginConfigstorageTransaction() -> crate::ctypes::c_int;
    pub fn ksceSysconClearTemperatureLog(arg1: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceSysconCmdExec(
        packet: *mut SceSysconPacket,
        flags: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconCmdExecAsync(
        packet: *mut SceSysconPacket,
        flags: crate::ctypes::c_uint,
        cb: SceSysconCmdExecAsyncCallback,
        argp: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconCmdSync(
        packet: *mut SceSysconPacket,
        noWait: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconCommitConfigstorageTransaction() -> crate::ctypes::c_int;
    pub fn ksceSysconCtrlHdmiCecPower(power: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceSysconCtrlLED(
        led: crate::ctypes::c_int,
        enable: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconCtrlManualChargeMode(arg1: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceSysconCtrlRMRPower(power: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceSysconCtrlSdPower(power: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceSysconEnableHibernateIO(arg1: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceSysconEndConfigstorageTransaction() -> crate::ctypes::c_int;
    pub fn ksceSysconGetBaryonTimestamp() -> crate::ctypes::c_ulonglong;
    pub fn ksceSysconGetBaryonVersion() -> crate::ctypes::c_int;
    pub fn ksceSysconGetBatteryCalibData(
        arg1: *mut crate::ctypes::c_int,
        arg2: *mut crate::ctypes::c_int,
        arg3: *mut crate::ctypes::c_int,
        arg4: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconGetControlsInfo(ctrl: *mut SceUInt32) -> crate::ctypes::c_int;
    pub fn ksceSysconGetHardwareInfo() -> crate::ctypes::c_int;
    pub fn ksceSysconGetHardwareInfo2(
        arg1: *mut crate::ctypes::c_int,
        arg2: *mut crate::ctypes::c_int,
        arg3: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconGetLogInfo(arg1: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceSysconGetManualChargeMode(arg1: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceSysconGetManufacturesStatus(arg1: *mut crate::ctypes::c_int)
        -> crate::ctypes::c_int;
    pub fn ksceSysconGetTemperatureLog(arg1: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceSysconGetUsbDetStatus(arg1: *mut crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceSysconIduModeClear() -> crate::ctypes::c_int;
    pub fn ksceSysconIduModeSet() -> crate::ctypes::c_int;
    pub fn ksceSysconIsDownLoaderMode() -> crate::ctypes::c_int;
    pub fn ksceSysconLoadConfigstorageScript(
        arg1: crate::ctypes::c_ushort,
        buff: *const crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconLogReadData(
        arg1: crate::ctypes::c_ushort,
        buff: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconLogStart() -> crate::ctypes::c_int;
    pub fn ksceSysconLogStartWaiting() -> crate::ctypes::c_int;
    pub fn ksceSysconReadCommand(
        cmd: crate::ctypes::c_ushort,
        buffer: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconSendCommand(
        cmd: crate::ctypes::c_ushort,
        buffer: *const crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconSetAlarmCallback(
        callback: SceSysconCallback,
        argp: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconSetDebugHandlers(
        handlers: *const SceSysconDebugHandlers,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconSetLowBatteryCallback(
        callback: SceSysconCallback,
        argp: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconSetPowerMode(
        type_: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconSetThermalAlertCallback(
        callback: SceSysconCallback,
        argp: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconShowModeClear() -> crate::ctypes::c_int;
    pub fn ksceSysconShowModeSet() -> crate::ctypes::c_int;
    pub fn ksceSysconVerifyConfigstorageScript(
        arg1: crate::ctypes::c_ushort,
        buff: *const crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysconWaitInitialized() -> crate::ctypes::c_int;
}
#[link(name = "SceSysmemForDriver_0990_stub", kind = "static")]
#[cfg(feature = "SceSysmemForDriver_0990_stub")]
extern "C" {}
#[link(name = "SceSysmemForDriver_stub", kind = "static")]
#[cfg(feature = "SceSysmemForDriver_stub")]
extern "C" {
    pub fn ksceGUIDClose(guid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceGUIDReferObject(
        guid: SceUID,
        object: *mut *mut SceObjectBase,
    ) -> crate::ctypes::c_int;
    pub fn ksceGUIDReferObjectWithClass(
        guid: SceUID,
        sce_class: *mut SceClass,
        object: *mut *mut SceObjectBase,
    ) -> crate::ctypes::c_int;
    pub fn ksceGUIDReferObjectWithClassLevel(
        guid: SceUID,
        pClass: *mut SceClass,
        level: SceUInt32,
        entry: *mut *mut SceObjectBase,
    ) -> crate::ctypes::c_int;
    pub fn ksceGUIDReleaseObject(guid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelAllocHeapMemory(uid: SceUID, size: SceSize) -> *mut crate::ctypes::c_void;
    pub fn ksceKernelAllocHeapMemoryWithOption(
        heapid: SceUID,
        len: SceSize,
        opt: *mut SceAllocOpt,
    ) -> *mut crate::ctypes::c_void;
    pub fn ksceKernelAllocMemBlock(
        name: *const crate::ctypes::c_char,
        type_: SceKernelMemBlockType,
        size: SceSize,
        opt: *mut SceKernelAllocMemBlockKernelOpt,
    ) -> SceUID;
    pub fn ksceKernelCreateClass(
        cls: *mut SceClass,
        name: *const crate::ctypes::c_char,
        uidclass: *mut crate::ctypes::c_void,
        itemsize: SceSize,
        create: SceClassCallback,
        destroy: SceClassCallback,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelCreateHeap(
        name: *const crate::ctypes::c_char,
        size: SceSize,
        opt: *mut SceKernelHeapCreateOpt,
    ) -> SceUID;
    pub fn ksceKernelDeleteHeap(uid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelFindMemBlockByAddr(
        addr: *const crate::ctypes::c_void,
        size: SceSize,
    ) -> SceUID;
    pub fn ksceKernelFindMemBlockByAddrForPid(
        pid: SceUID,
        addr: *const crate::ctypes::c_void,
        size: SceSize,
    ) -> SceUID;
    pub fn ksceKernelFreeHeapMemory(
        uid: SceUID,
        ptr: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelFreeMemBlock(uid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelGetMemBlockAllocMapSize(
        memid: SceUID,
        alloc_map_size: *mut SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetMemBlockBase(
        uid: SceUID,
        base: *mut *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetPidContext(
        pid: SceUID,
        ctx: *mut *mut SceKernelProcessContext,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetUidClass() -> *mut SceClass;
    pub fn ksceKernelMapBlockUserVisible(uid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelMapUserBlock(
        name: *const crate::ctypes::c_char,
        permission: crate::ctypes::c_int,
        type_: crate::ctypes::c_int,
        user_buf: *const crate::ctypes::c_void,
        size: SceSize,
        kernel_page: *mut *mut crate::ctypes::c_void,
        kernel_size: *mut SceSize,
        kernel_offset: *mut crate::ctypes::c_uint,
    ) -> SceUID;
    pub fn ksceKernelMemBlockRelease(uid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelMemRangeRelease(
        addr: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelMemRangeReleaseForPid(
        pid: SceUID,
        addr: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelMemRangeReleaseWithPerm(
        perm: SceKernelMemoryRefPerm,
        addr: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelMemRangeRetain(
        addr: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelMemRangeRetainForPid(
        pid: SceUID,
        addr: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelMemRangeRetainWithPerm(
        perm: SceKernelMemoryRefPerm,
        addr: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelMemcpyFromUser(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelMemcpyToUser(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelProcMemcpyFromUser(
        pid: SceUID,
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelProcMemcpyToUser(
        pid: SceUID,
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelProcStrncpyFromUser(
        pid: SceUID,
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> SceSSize;
    pub fn ksceKernelProcStrncpyToUser(
        pid: SceUID,
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> SceSSize;
    pub fn ksceKernelProcStrnlenUser(
        pid: SceUID,
        s: *const crate::ctypes::c_char,
        n: SceSize,
    ) -> SceSSize;
    pub fn ksceKernelProcUserMap(
        pid: SceUID,
        name: *const crate::ctypes::c_char,
        permission: crate::ctypes::c_int,
        user_buf: *const crate::ctypes::c_void,
        size: SceSize,
        kernel_page: *mut *mut crate::ctypes::c_void,
        kernel_size: *mut SceSize,
        kernel_offset: *mut SceUInt32,
    ) -> SceUID;
    pub fn ksceKernelProcUserMemcpy(
        pid: SceUID,
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelProcessGetContext(
        pid: SceUID,
        ctx: *mut *mut SceKernelProcessContext,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelProcessSwitchContext(
        new_context: *const SceKernelProcessContext,
        prev_context: *mut SceKernelProcessContext,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelRemapBlock(uid: SceUID, type_: SceKernelMemBlockType) -> crate::ctypes::c_int;
    pub fn ksceKernelStrncpyFromUser(
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> SceSSize;
    pub fn ksceKernelStrncpyToUser(
        dst: *mut crate::ctypes::c_char,
        src: *const crate::ctypes::c_char,
        len: SceSize,
    ) -> SceSSize;
    pub fn ksceKernelStrnlenUser(s: *const crate::ctypes::c_char, n: SceSize) -> SceSize;
    pub fn ksceKernelSwitchVmaForPid(pid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelUserMap(
        name: *const crate::ctypes::c_char,
        permission: crate::ctypes::c_int,
        user_buf: *const crate::ctypes::c_void,
        size: SceSize,
        kernel_page: *mut *mut crate::ctypes::c_void,
        kernel_size: *mut SceSize,
        kernel_offset: *mut SceUInt32,
    ) -> SceUID;
    pub fn ksceKernelUserMemcpy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelVARangeToPAVector(
        va_range: *const SceKernelVARange,
        pa_vector: *mut SceKernelPAVector,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelVAtoPA(
        va: *const crate::ctypes::c_void,
        pa: *mut usize,
    ) -> crate::ctypes::c_int;
    pub fn kscePUIDClose(pid: SceUID, puid: SceUID) -> crate::ctypes::c_int;
    pub fn kscePUIDOpenByGUID(pid: SceUID, guid: SceUID) -> SceUID;
    pub fn kscePUIDtoGUID(pid: SceUID, puid: SceUID) -> SceUID;
}
#[link(name = "SceSysmemForKernel_363_stub", kind = "static")]
#[cfg(feature = "SceSysmemForKernel_363_stub")]
extern "C" {}
#[cfg(any(
    feature = "SceSysmemForKernel_363_stub",
    feature = "SceSysmemForKernel_stub"
))]
extern "C" {
    pub fn ksceGUIDKernelCreateWithOpt(
        sce_class: *mut SceClass,
        name: *const crate::ctypes::c_char,
        opt: *mut SceGUIDKernelCreateOpt,
        obj: *mut *mut SceObjectBase,
    ) -> SceUID;
    pub fn ksceKernelFindClassByName(
        name: *const crate::ctypes::c_char,
        cls: *mut *mut SceClass,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetMemBlockType(
        uid: SceUID,
        type_: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelProcMemcpyToUserRx(
        pid: SceUID,
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSysmemForKernel_stub", kind = "static")]
#[cfg(feature = "SceSysmemForKernel_stub")]
extern "C" {
    pub fn ksceGUIDGetUIDVectorByClass(
        cls: *mut SceClass,
        vis_level: crate::ctypes::c_int,
        vector: *mut SceUID,
        num: SceSize,
        ret_num: *mut SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelAlloc(size: crate::ctypes::c_uint) -> *mut crate::ctypes::c_void;
    pub fn ksceKernelFree(ptr: *mut crate::ctypes::c_void) -> crate::ctypes::c_int;
    pub fn ksceKernelGetUidDLinkClass() -> *mut SceClass;
    pub fn ksceKernelGetUidHeapClass() -> *mut SceClass;
    pub fn ksceKernelGetUidMemBlockClass() -> *mut SceClass;
    pub fn ksceKernelMemcpyToUserRo(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelMemcpyToUserRx(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        len: SceSize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSysmem_stub", kind = "static")]
#[cfg(feature = "SceSysmem_stub")]
extern "C" {
    pub fn sceKernelAllocMemBlock(
        name: *const crate::ctypes::c_char,
        type_: SceKernelMemBlockType,
        size: SceSize,
        opt: *mut SceKernelAllocMemBlockOpt,
    ) -> SceUID;
    pub fn sceKernelAllocMemBlockForVM(name: *const crate::ctypes::c_char, size: SceSize)
        -> SceUID;
    pub fn sceKernelCloseMemBlock(uid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelCloseVMDomain() -> crate::ctypes::c_int;
    pub fn sceKernelFindMemBlockByAddr(addr: *const crate::ctypes::c_void, size: SceSize)
        -> SceUID;
    pub fn sceKernelFreeMemBlock(uid: SceUID) -> crate::ctypes::c_int;
    pub fn sceKernelGetCpuId() -> crate::ctypes::c_int;
    pub fn sceKernelGetFreeMemorySize(
        info: *mut SceKernelFreeMemorySizeInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetMemBlockBase(
        uid: SceUID,
        base: *mut *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetMemBlockInfoByAddr(
        base: *mut crate::ctypes::c_void,
        info: *mut SceKernelMemBlockInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetMemBlockInfoByRange(
        base: *mut crate::ctypes::c_void,
        size: SceSize,
        info: *mut SceKernelMemBlockInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelGetModel() -> crate::ctypes::c_int;
    pub fn sceKernelGetModelForCDialog() -> crate::ctypes::c_int;
    pub fn sceKernelIsPSVitaTV() -> crate::ctypes::c_int;
    pub fn sceKernelOpenMemBlock(
        name: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceKernelOpenVMDomain() -> crate::ctypes::c_int;
    pub fn sceKernelSyncVMDomain(
        uid: SceUID,
        data: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSysmodule_stub", kind = "static")]
#[cfg(feature = "SceSysmodule_stub")]
extern "C" {
    pub fn sceSysmoduleIsLoaded(id: SceSysmoduleModuleId) -> crate::ctypes::c_int;
    pub fn sceSysmoduleIsLoadedInternal(id: SceSysmoduleInternalModuleId) -> crate::ctypes::c_int;
    pub fn sceSysmoduleLoadModule(id: SceSysmoduleModuleId) -> crate::ctypes::c_int;
    pub fn sceSysmoduleLoadModuleInternal(id: SceSysmoduleInternalModuleId)
        -> crate::ctypes::c_int;
    pub fn sceSysmoduleLoadModuleInternalWithArg(
        id: SceSysmoduleInternalModuleId,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        option: *const SceSysmoduleOpt,
    ) -> crate::ctypes::c_int;
    pub fn sceSysmoduleUnloadModule(id: SceSysmoduleModuleId) -> crate::ctypes::c_int;
    pub fn sceSysmoduleUnloadModuleInternal(
        id: SceSysmoduleInternalModuleId,
    ) -> crate::ctypes::c_int;
    pub fn sceSysmoduleUnloadModuleInternalWithArg(
        id: SceSysmoduleInternalModuleId,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        option: *const SceSysmoduleOpt,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceSysrootForDriver_stub", kind = "static")]
#[cfg(feature = "SceSysrootForDriver_stub")]
extern "C" {
    pub fn ksceKernelSysrootGetShellPid() -> SceUID;
    pub fn ksceKernelSysrootGetSystemSwVersion() -> crate::ctypes::c_int;
    pub fn ksceKernelSysrootRegisterCoredumpTrigger(func: SceKernelCoredumpTriggerFunc);
    pub fn ksceKernelSysrootSetGetSystemSwVersionFunc(func: SceKernelGetSystemSwVersionFunc);
    pub fn ksceKernelSysrootSetProcessHandler(
        handlers: *const SceSysrootProcessHandler,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysrootGetHardwareFlags(flags: *mut crate::ctypes::c_void) -> crate::ctypes::c_int;
    pub fn ksceSysrootUseExternalStorage() -> crate::ctypes::c_int;
    pub fn ksceSysrootUseInternalStorage() -> crate::ctypes::c_int;
}
#[link(name = "SceSysrootForKernel_stub", kind = "static")]
#[cfg(feature = "SceSysrootForKernel_stub")]
extern "C" {
    pub fn ksceKernelSysrootGetKblParam() -> *mut crate::ctypes::c_void;
    pub fn ksceKernelSysrootGetProcessTitleId(
        pid: SceUID,
        titleid: *mut crate::ctypes::c_char,
        len: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSysrootRegisterDbgpHandler(
        handlers: *const SceSysrootDbgpHandler,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSysrootSetSysroot(sysroot_uid: SceUID);
    pub fn ksceKernelSysrootUnregisterDbgpHandler();
    pub fn ksceSysrootGetSelfInfo(
        index: SceKernelSysrootSelfIndex,
        info: *mut SceKernelSysrootSelfInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceSysrootGetSysroot() -> *mut SceSysroot;
    pub fn ksceSysrootIsBsodReboot() -> crate::ctypes::c_int;
    pub fn ksceSysrootIsExternalBootMode() -> crate::ctypes::c_int;
    pub fn ksceSysrootIsManufacturingMode() -> crate::ctypes::c_int;
    pub fn ksceSysrootIsNonRemovableCardMode() -> crate::ctypes::c_int;
    pub fn ksceSysrootIsSafeMode() -> crate::ctypes::c_int;
    pub fn ksceSysrootIsUpdateMode() -> crate::ctypes::c_int;
    pub fn ksceSysrootIsUsbEnumWakeup() -> crate::ctypes::c_int;
}
#[link(name = "SceSystemGesture_stub", kind = "static")]
#[cfg(feature = "SceSystemGesture_stub")]
extern "C" {}
#[link(name = "SceSystimerForDriver_stub", kind = "static")]
#[cfg(feature = "SceSystimerForDriver_stub")]
extern "C" {
    pub fn ksceKernelSysTimerAlloc(timerType: SceSysTimerType) -> SceSysTimerId;
    pub fn ksceKernelSysTimerFree(timerId: SceSysTimerId) -> crate::ctypes::c_int;
    pub fn ksceKernelSysTimerGetCount(
        timerId: SceSysTimerId,
        pCount: *mut SceKernelSysClock,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSysTimerResetCount(timerId: SceSysTimerId) -> crate::ctypes::c_int;
    pub fn ksceKernelSysTimerSetClockSource(
        timerId: SceSysTimerId,
        clockSource: SceSysTimerClockSource,
        prescaleFactor: SceUInt8,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSysTimerSetCount(
        timerId: SceSysTimerId,
        count: SceKernelSysClock,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSysTimerSetHandler(
        timerId: SceSysTimerId,
        callback: SceSysTimerCallback,
        targetCPU: SceUInt32,
        pUserData: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSysTimerSetInterval(
        timerId: SceSysTimerId,
        interval: SceKernelSysClock,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSysTimerStartCount(timerId: SceSysTimerId) -> crate::ctypes::c_int;
    pub fn ksceKernelSysTimerStopCount(timerId: SceSysTimerId) -> crate::ctypes::c_int;
}
#[link(name = "SceTeleportClient_stub", kind = "static")]
#[cfg(feature = "SceTeleportClient_stub")]
extern "C" {}
#[link(name = "SceTeleportServer_stub", kind = "static")]
#[cfg(feature = "SceTeleportServer_stub")]
extern "C" {}
#[link(name = "SceThreadmgrForDriver_stub", kind = "static")]
#[cfg(feature = "SceThreadmgrForDriver_stub")]
extern "C" {
    pub fn ksceKernelCancelCallback(cb: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelCancelMsgPipe(
        uid: SceUID,
        psend: *mut crate::ctypes::c_int,
        precv: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelCancelMutex(
        mutexid: SceUID,
        newCount: crate::ctypes::c_int,
        numWaitThreads: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelChangeThreadPriority(
        thid: SceUID,
        priority: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelChangeThreadSuspendStatus(
        thid: SceUID,
        status: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelCheckCallback() -> crate::ctypes::c_int;
    pub fn ksceKernelClearEventFlag(
        evfid: SceUID,
        bits: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelCreateCallback(
        name: *const crate::ctypes::c_char,
        attr: crate::ctypes::c_uint,
        func: SceKernelCallbackFunction,
        arg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelCreateCond(
        name: *const crate::ctypes::c_char,
        attr: SceUInt,
        mutexId: SceUID,
        option: *const SceKernelCondOptParam,
    ) -> SceUID;
    pub fn ksceKernelCreateEventFlag(
        name: *const crate::ctypes::c_char,
        attr: crate::ctypes::c_int,
        bits: crate::ctypes::c_int,
        opt: *mut SceKernelEventFlagOptParam,
    ) -> SceUID;
    pub fn ksceKernelCreateMsgPipe(
        name: *const crate::ctypes::c_char,
        type_: crate::ctypes::c_int,
        attr: crate::ctypes::c_int,
        bufSize: SceSize,
        opt: *mut crate::ctypes::c_void,
    ) -> SceUID;
    pub fn ksceKernelCreateMutex(
        name: *const crate::ctypes::c_char,
        attr: SceUInt,
        initCount: crate::ctypes::c_int,
        option: *mut SceKernelMutexOptParam,
    ) -> SceUID;
    pub fn ksceKernelCreateSema(
        name: *const crate::ctypes::c_char,
        attr: SceUInt,
        initVal: crate::ctypes::c_int,
        maxVal: crate::ctypes::c_int,
        option: *mut SceKernelSemaOptParam,
    ) -> SceUID;
    pub fn ksceKernelCreateThread(
        name: *const crate::ctypes::c_char,
        entry: SceKernelThreadEntry,
        initPriority: crate::ctypes::c_int,
        stackSize: SceSize,
        attr: SceUInt,
        cpuAffinityMask: crate::ctypes::c_int,
        option: *const SceKernelThreadOptParam,
    ) -> SceUID;
    pub fn ksceKernelDebugResumeThread(
        thid: SceUID,
        status: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelDebugSuspendThread(
        thid: SceUID,
        status: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelDelayThread(delay: SceUInt) -> crate::ctypes::c_int;
    pub fn ksceKernelDelayThreadCB(delay: SceUInt) -> crate::ctypes::c_int;
    pub fn ksceKernelDeleteCallback(cb: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelDeleteCond(condId: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelDeleteEventFlag(evfid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelDeleteMsgPipe(uid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelDeleteMutex(mutexid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelDeleteSema(semaid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelDeleteThread(thid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelEnqueueWorkQueue(
        uid: SceUID,
        name: *const crate::ctypes::c_char,
        work: SceKernelWorkQueueWorkFunction,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelExitDeleteThread(status: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceKernelExitThread(status: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceKernelFinalizeFastMutex(fast_mutex: *mut SceKernelFastMutex)
        -> crate::ctypes::c_int;
    pub fn ksceKernelGetCallbackCount(cb: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelGetEventFlagInfo(
        evfid: SceUID,
        info: *mut SceKernelEventFlagInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetFastMutexInfo(
        fast_mutex: *mut SceKernelFastMutex,
        info: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetLwCondInfo(
        lwcond_id: SceUID,
        info: *mut SceKernelLwCondInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetLwMutexInfo(
        lwmtxid: SceUID,
        info: *mut SceKernelLwMutexInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetMsgPipeInfo(
        msgpipe_id: SceUID,
        info: *mut SceKernelMsgPipeInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetMutexInfo(
        mutexid: SceUID,
        info: *mut SceKernelMutexInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetProcessId() -> SceUID;
    pub fn ksceKernelGetProcessIdFromTLS() -> SceUID;
    pub fn ksceKernelGetSemaInfo(
        semaid: SceUID,
        info: *mut SceKernelSemaInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetSystemTimeLow() -> SceUInt32;
    pub fn ksceKernelGetSystemTimeWide() -> SceInt64;
    pub fn ksceKernelGetTLSAddr(key: crate::ctypes::c_int) -> *mut crate::ctypes::c_void;
    pub fn ksceKernelGetThreadCpuRegisters(
        thid: SceUID,
        registers: *mut SceThreadCpuRegisters,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetThreadCurrentPriority() -> crate::ctypes::c_int;
    pub fn ksceKernelGetThreadId() -> crate::ctypes::c_int;
    pub fn ksceKernelGetThreadIdList(
        pid: SceUID,
        ids: *mut SceUID,
        n: crate::ctypes::c_int,
        copy_count: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetThreadInfo(
        thid: SceUID,
        info: *mut SceKernelThreadInfo,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetThreadInfoForDebugger(
        thid: SceUID,
        a2: crate::ctypes::c_int,
        pInfo: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelGetThreadStackFreeSize(thid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelGetThreadTLSAddr(
        thid: SceUID,
        key: crate::ctypes::c_int,
    ) -> *mut crate::ctypes::c_void;
    pub fn ksceKernelGetThreadmgrUIDClass(uid: SceUID) -> SceKernelIdListType;
    pub fn ksceKernelGetUserThreadId(thid: SceUID) -> SceUID;
    pub fn ksceKernelGetVfpRegisterForDebugger(
        thid: SceUID,
        pVfpRegister: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelInitializeFastMutex(
        fast_mutex: *mut SceKernelFastMutex,
        name: *const crate::ctypes::c_char,
        attr: crate::ctypes::c_int,
        opt: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelIsThreadDebugSuspended(thid: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelLockFastMutex(fast_mutex: *mut SceKernelFastMutex) -> crate::ctypes::c_int;
    pub fn ksceKernelLockMutex(
        mutexid: SceUID,
        lockCount: crate::ctypes::c_int,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelNotifyCallback(cb: SceUID, arg2: crate::ctypes::c_int)
        -> crate::ctypes::c_int;
    pub fn ksceKernelPollEventFlag(
        evfid: SceUID,
        bits: crate::ctypes::c_uint,
        wait: crate::ctypes::c_uint,
        outBits: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelPollSema(semaid: SceUID, signal: crate::ctypes::c_int)
        -> crate::ctypes::c_int;
    pub fn ksceKernelReceiveMsgPipeVector(
        uid: SceUID,
        v: *const MsgPipeRecvData,
        n: crate::ctypes::c_uint,
        unk1: crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_void,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelRunWithStack(
        stack_size: SceSize,
        to_call: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut crate::ctypes::c_void) -> crate::ctypes::c_int,
        >,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSendMsgPipeVector(
        uid: SceUID,
        v: *const MsgPipeSendData,
        n: crate::ctypes::c_uint,
        unk1: crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_void,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSetEventFlag(
        evfid: SceUID,
        bits: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelSetPermission(value: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceKernelSetProcessIdToTLS(pid: SceUID) -> SceUID;
    pub fn ksceKernelSignalCond(condId: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelSignalCondAll(condId: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelSignalCondTo(condId: SceUID, threadId: SceUID) -> crate::ctypes::c_int;
    pub fn ksceKernelSignalSema(
        semaid: SceUID,
        signal: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelStartThread(
        thid: SceUID,
        arglen: SceSize,
        argp: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelTryLockFastMutex(fast_mutex: *mut SceKernelFastMutex) -> crate::ctypes::c_int;
    pub fn ksceKernelTryLockMutex(
        mutexid: SceUID,
        lockCount: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelTryReceiveMsgPipeVector(
        uid: SceUID,
        v: *const MsgPipeRecvData,
        size: SceSize,
        unk1: crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelTrySendMsgPipeVector(
        uid: SceUID,
        v: *const MsgPipeSendData,
        size: SceSize,
        unk1: crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelUnlockFastMutex(fast_mutex: *mut SceKernelFastMutex) -> crate::ctypes::c_int;
    pub fn ksceKernelUnlockMutex(
        mutexid: SceUID,
        unlockCount: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelWaitCond(
        condId: SceUID,
        timeout: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelWaitEventFlag(
        evfid: SceUID,
        bits: crate::ctypes::c_uint,
        wait: crate::ctypes::c_uint,
        outBits: *mut crate::ctypes::c_uint,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelWaitEventFlagCB(
        evfid: SceUID,
        bits: crate::ctypes::c_uint,
        wait: crate::ctypes::c_uint,
        outBits: *mut crate::ctypes::c_uint,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelWaitSema(
        semaid: SceUID,
        signal: crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelWaitThreadEnd(
        thid: SceUID,
        stat: *mut crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn ksceKernelWaitThreadEndCB(
        thid: SceUID,
        stat: *mut crate::ctypes::c_int,
        timeout: *mut SceUInt,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceThreadmgrForKernel_363_stub", kind = "static")]
#[cfg(feature = "SceThreadmgrForKernel_363_stub")]
extern "C" {}
#[cfg(any(
    feature = "SceThreadmgrForKernel_363_stub",
    feature = "SceThreadmgrForKernel_stub"
))]
extern "C" {
    pub fn ksceKernelGetThreadContextInfo(
        pInfo: *mut SceKernelThreadContextInfo,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceThreadmgrForKernel_stub", kind = "static")]
#[cfg(feature = "SceThreadmgrForKernel_stub")]
extern "C" {}
#[link(name = "SceTouchForDriver_stub", kind = "static")]
#[cfg(feature = "SceTouchForDriver_stub")]
extern "C" {
    pub fn ksceTouchSetEnableFlag(port: SceUInt32, enable: SceBool) -> crate::ctypes::c_int;
}
#[link(name = "SceTouch_stub", kind = "static")]
#[cfg(feature = "SceTouch_stub")]
extern "C" {
    pub fn sceTouchDisableTouchForce(port: SceUInt32) -> crate::ctypes::c_int;
    pub fn sceTouchEnableTouchForce(port: SceUInt32) -> crate::ctypes::c_int;
    pub fn sceTouchGetPanelInfo(
        port: SceUInt32,
        pPanelInfo: *mut SceTouchPanelInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceTouchGetSamplingState(
        port: SceUInt32,
        pState: *mut SceTouchSamplingState,
    ) -> crate::ctypes::c_int;
    pub fn sceTouchPeek(
        port: SceUInt32,
        pData: *mut SceTouchData,
        nBufs: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceTouchRead(
        port: SceUInt32,
        pData: *mut SceTouchData,
        nBufs: SceUInt32,
    ) -> crate::ctypes::c_int;
    pub fn sceTouchSetSamplingState(
        port: SceUInt32,
        state: SceTouchSamplingState,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceTriggerUtil_stub", kind = "static")]
#[cfg(feature = "SceTriggerUtil_stub")]
extern "C" {
    pub fn sceTriggerUtilGetAutoStartStatus(
        status: *mut crate::ctypes::c_int,
        a2: crate::ctypes::c_int,
        a3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceTriggerUtilGetDailyEventInfo(
        eventId: crate::ctypes::c_int,
        param: *mut SceTriggerUtilEventParamDaily,
        a5: crate::ctypes::c_int,
        a6: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceTriggerUtilGetOneTimeEventInfo(
        eventId: crate::ctypes::c_int,
        triggerTime: *mut SceRtcTick,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceTriggerUtilGetRegisteredSystemTitleIdList(
        buffer: *mut crate::ctypes::c_char,
        numOfIds: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceTriggerUtilGetRegisteredUserTitleIdList(
        titleIdBuffer: *mut crate::ctypes::c_char,
        numOfIds: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceTriggerUtilGetSystemAppInfo(
        titleid: *const crate::ctypes::c_char,
        appInfo: *mut SceTriggerUtilSystemAppInfo,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceTriggerUtilGetUserAppInfo(
        titleid: *const crate::ctypes::c_char,
        appInfo: *mut SceTriggerUtilUserAppInfo,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceTriggerUtilRegisterDailyEvent(
        titleid: *const crate::ctypes::c_char,
        param: *const SceTriggerUtilEventParamDaily,
        eventId: crate::ctypes::c_int,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceTriggerUtilRegisterOneTimeEvent(
        titleid: *const crate::ctypes::c_char,
        param: *const SceTriggerUtilEventParamOneTime,
        eventId: crate::ctypes::c_int,
        a4: crate::ctypes::c_int,
        a5: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceTriggerUtilUnregisterDailyEvent(
        eventId: crate::ctypes::c_int,
        a2: crate::ctypes::c_int,
        a3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceTriggerUtilUnregisterOneTimeEvent(
        eventId: crate::ctypes::c_int,
        a2: crate::ctypes::c_int,
        a3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceUartForKernel_363_stub", kind = "static")]
#[cfg(feature = "SceUartForKernel_363_stub")]
extern "C" {}
#[cfg(any(
    feature = "SceUartForKernel_363_stub",
    feature = "SceUartForKernel_stub"
))]
extern "C" {
    pub fn ksceUartInit(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUartRead(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUartReadAvailable(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUartWrite(
        port: crate::ctypes::c_int,
        data: crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceUartForKernel_stub", kind = "static")]
#[cfg(feature = "SceUartForKernel_stub")]
extern "C" {}
#[link(name = "SceUdcdForDriver_stub", kind = "static")]
#[cfg(feature = "SceUdcdForDriver_stub")]
extern "C" {
    pub fn ksceUdcdActivate(productId: crate::ctypes::c_uint) -> crate::ctypes::c_int;
    pub fn ksceUdcdActivateInternal(
        productId: crate::ctypes::c_uint,
        bus_powered: crate::ctypes::c_uint,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdClearFIFO(endp: *mut SceUdcdEndpoint) -> crate::ctypes::c_int;
    pub fn ksceUdcdClearFIFOInternal(
        endp: *mut SceUdcdEndpoint,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdDeactivate() -> crate::ctypes::c_int;
    pub fn ksceUdcdDeactivateInternal(bus: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUdcdGetDeviceInfo(devInfo: *mut SceUdcdDeviceInfo) -> crate::ctypes::c_int;
    pub fn ksceUdcdGetDeviceInfoInternal(
        devInfo: *mut SceUdcdDeviceInfo,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdGetDeviceState() -> crate::ctypes::c_int;
    pub fn ksceUdcdGetDeviceStateInternal(bus: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUdcdGetDrvState(driverName: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn ksceUdcdGetDrvStateInternal(
        driverName: *const crate::ctypes::c_char,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdRegister(drv: *mut SceUdcdDriver) -> crate::ctypes::c_int;
    pub fn ksceUdcdRegisterInternal(
        drv: *mut SceUdcdDriver,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdReqCancelAll(endp: *mut SceUdcdEndpoint) -> crate::ctypes::c_int;
    pub fn ksceUdcdReqRecv(req: *mut SceUdcdDeviceRequest) -> crate::ctypes::c_int;
    pub fn ksceUdcdReqRecvInternal(
        req: *mut SceUdcdDeviceRequest,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdReqSend(req: *mut SceUdcdDeviceRequest) -> crate::ctypes::c_int;
    pub fn ksceUdcdReqSendInternal(
        req: *mut SceUdcdDeviceRequest,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdStall(endp: *mut SceUdcdEndpoint) -> crate::ctypes::c_int;
    pub fn ksceUdcdStallInternal(
        endp: *mut SceUdcdEndpoint,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdStart(
        driverName: *const crate::ctypes::c_char,
        size: crate::ctypes::c_int,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdStartCurrentInternal(
        unused: crate::ctypes::c_int,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdStartInternal(
        driverName: *const crate::ctypes::c_char,
        size: crate::ctypes::c_int,
        args: *mut crate::ctypes::c_void,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdStop(
        driverName: *const crate::ctypes::c_char,
        size: crate::ctypes::c_int,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdStopCurrentInternal(bus: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUdcdStopInternal(
        driverName: *const crate::ctypes::c_char,
        size: crate::ctypes::c_int,
        args: *mut crate::ctypes::c_void,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdUnregister(drv: *mut SceUdcdDriver) -> crate::ctypes::c_int;
    pub fn ksceUdcdUnregisterInternal(
        drv: *mut SceUdcdDriver,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdWaitBusInitialized(
        timeout: crate::ctypes::c_uint,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdWaitState(
        waitParam: *mut SceUdcdWaitParam,
        timeout: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn ksceUdcdWaitStateInternal(
        waitParam: *mut SceUdcdWaitParam,
        timeout: crate::ctypes::c_uint,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceUdcd_stub", kind = "static")]
#[cfg(feature = "SceUdcd_stub")]
extern "C" {
    pub fn sceUdcdGetDeviceInfo(devInfo: *mut SceUdcdDeviceInfo) -> crate::ctypes::c_int;
    pub fn sceUdcdGetDeviceState(state: *mut SceUdcdDeviceState) -> crate::ctypes::c_int;
    pub fn sceUdcdGetDrvState(driverName: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn sceUdcdRegisterCallback(
        cbid: SceUID,
        state: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceUdcdUnregisterCallback(cbid: SceUID) -> crate::ctypes::c_int;
    pub fn sceUdcdWaitState(
        waitParam: *mut SceUdcdWaitParam,
        timeout: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceUlobjMgr_stub", kind = "static")]
#[cfg(feature = "SceUlobjMgr_stub")]
extern "C" {}
#[link(name = "SceUlt_stub", kind = "static")]
#[cfg(feature = "SceUlt_stub")]
extern "C" {}
#[link(name = "SceUsbAudioForDriver_stub", kind = "static")]
#[cfg(feature = "SceUsbAudioForDriver_stub")]
extern "C" {}
#[link(name = "SceUsbAudioIn_stub", kind = "static")]
#[cfg(feature = "SceUsbAudioIn_stub")]
extern "C" {
    pub fn sceUsbAudioInCloseDevice(device_id: SceUInt32) -> SceInt32;
    pub fn sceUsbAudioInGetDeviceIdList(
        list: *mut SceUsbAudioInDeviceListItem,
        device_count: *mut SceUInt32,
        list_size: SceUInt32,
    ) -> SceInt32;
    pub fn sceUsbAudioInGetDeviceInformation(
        device_id: SceUInt32,
        info: *mut SceUsbAudioInDeviceInfo,
    ) -> SceInt32;
    pub fn sceUsbAudioInGetMaxValueOfVolume(
        device_id: SceUInt32,
        volume: *mut SceUInt32,
    ) -> SceInt32;
    pub fn sceUsbAudioInGetMinValueOfVolume(
        device_id: SceUInt32,
        volume: *mut SceUInt32,
    ) -> SceInt32;
    pub fn sceUsbAudioInInput(device_id: SceUInt32, buffer: *mut crate::ctypes::c_void)
        -> SceInt32;
    pub fn sceUsbAudioInOpenDevice(
        device_id: SceUInt32,
        bits: crate::ctypes::c_int,
        rate: crate::ctypes::c_int,
    ) -> SceInt32;
    pub fn sceUsbAudioInSetCurrentValueOfVolume(
        device_id: SceUInt32,
        volume: SceUInt32,
    ) -> SceInt32;
}
#[link(name = "SceUsbdForDriver_stub", kind = "static")]
#[cfg(feature = "SceUsbdForDriver_stub")]
extern "C" {
    pub fn ksceUsbdBulkTransfer(
        pipe_id: SceUID,
        buffer: *mut crate::ctypes::c_uchar,
        length: crate::ctypes::c_uint,
        cb: ksceUsbdDoneCallback,
        user_data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceUsbdBulkTransfer2(
        pipe_id: crate::ctypes::c_int,
        buffer: *mut crate::ctypes::c_uchar,
        length: crate::ctypes::c_uint,
        cb: ksceUsbdDoneCallback,
        user_data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceUsbdClosePipe(pipe_id: SceUID) -> crate::ctypes::c_int;
    pub fn ksceUsbdControlTransfer(
        pipe_id: SceUID,
        req: *const SceUsbdDeviceRequest,
        buffer: *mut crate::ctypes::c_uchar,
        cb: ksceUsbdDoneCallback,
        user_data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceUsbdGetDeviceLocation(device_id: SceUID, location: *mut u8) -> crate::ctypes::c_int;
    pub fn ksceUsbdGetDeviceSpeed(
        device_id: crate::ctypes::c_int,
        speed: *mut u8,
    ) -> crate::ctypes::c_int;
    pub fn ksceUsbdHostStart(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUsbdHostStop(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUsbdInterruptTransfer(
        pipe_id: SceUID,
        buffer: *mut crate::ctypes::c_uchar,
        length: SceSize,
        cb: ksceUsbdDoneCallback,
        user_data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceUsbdIsochronousTransfer(
        pipe_id: SceUID,
        transfer: *mut ksceUsbdIsochTransfer,
        cb: ksceUsbdIsochDoneCallback,
        user_data: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn ksceUsbdOpenPipe(
        device_id: crate::ctypes::c_int,
        endpoint: *mut SceUsbdEndpointDescriptor,
    ) -> SceUID;
    pub fn ksceUsbdRegisterCompositeLdd(
        driver: *const SceUsbdCompositeDriver,
    ) -> crate::ctypes::c_int;
    pub fn ksceUsbdRegisterDriver(driver: *const SceUsbdDriver) -> crate::ctypes::c_int;
    pub fn ksceUsbdResume(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUsbdScanStaticDescriptor(
        device_id: SceUID,
        start: *mut crate::ctypes::c_void,
        type_: SceUsbdDescriptorType,
    ) -> *mut crate::ctypes::c_void;
    pub fn ksceUsbdSuspend(port: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUsbdSuspendPhase2(
        port: crate::ctypes::c_int,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUsbdUnregisterCompositeLdd(
        driver: *const SceUsbdCompositeDriver,
    ) -> crate::ctypes::c_int;
    pub fn ksceUsbdUnregisterDriver(driver: *const SceUsbdDriver) -> crate::ctypes::c_int;
    pub fn ksceUsbd_05073925(
        device_id: SceUID,
        unk1: *mut crate::ctypes::c_int,
        unk2: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn ksceUsbd_7938DAC7(pipe_id: SceUID) -> crate::ctypes::c_int;
}
#[link(name = "SceUsbd_stub", kind = "static")]
#[cfg(feature = "SceUsbd_stub")]
extern "C" {
    pub fn sceUsbdAttach(
        uid: SceUID,
        driver_id: SceUID,
        bus: SceUInt,
        device: SceUInt,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdAttachCompositeLdd(
        uid: SceUID,
        param: *mut SceUsbdAttachCompositeParam,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdClosePipe(uid: SceUID, pipe_id: SceUID) -> crate::ctypes::c_int;
    pub fn sceUsbdEnd(uid: SceUID) -> crate::ctypes::c_int;
    pub fn sceUsbdGetDescriptor(
        uid: SceUID,
        device_id: SceUID,
        descriptor: *mut crate::ctypes::c_uchar,
        size: SceSize,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdGetDescriptorSize(
        uid: SceUID,
        device_id: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdGetDeviceAddress(
        uid: SceUID,
        device_id: SceUID,
        addr: *mut SceUsbdDeviceAddress,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdGetDeviceList(
        uid: SceUID,
        num: SceSize,
        info: *mut SceUsbdDeviceInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdGetDeviceSpeed(
        uid: SceUID,
        device_id: SceUID,
        speed: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdGetIsochTransferStatus(
        transfer_id: SceUID,
        status: *mut SceUsbdIsochTransferStatus,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdGetTransferStatus(
        transfer_id: SceUID,
        status: *mut SceUsbdTransferStatus,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdInit(uid: *mut SceUID) -> crate::ctypes::c_int;
    pub fn sceUsbdIsochTransferData(
        uid: SceUID,
        pipe_id: SceUID,
        transfer: *mut SceUsbdIsochTransfer,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdOpenDefaultPipe(uid: SceUID, device_id: SceUID) -> SceUID;
    pub fn sceUsbdOpenPipe(uid: SceUID, pipe: *mut SceUsbdDevicePipe) -> SceUID;
    pub fn sceUsbdReceiveEvent(
        uid: SceUID,
        event: *mut SceUsbdReceiveEvent,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdRegisterCallback(
        cbid: SceUID,
        flag: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbdRegisterCompositeLdd(uid: SceUID, name: *mut crate::ctypes::c_char) -> SceUID;
    pub fn sceUsbdRegisterLdd(uid: SceUID, name: *mut crate::ctypes::c_char) -> SceUID;
    pub fn sceUsbdResetDevice(uid: SceUID, device_id: SceUID) -> crate::ctypes::c_int;
    pub fn sceUsbdTransferData(uid: SceUID, data: *mut SceUsbdTransferData) -> SceUID;
    pub fn sceUsbdUnregisterCallback(cbid: SceUID) -> crate::ctypes::c_int;
    pub fn sceUsbdUnregisterLdd(
        uid: SceUID,
        name: *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceUsbPspcm_stub", kind = "static")]
#[cfg(feature = "SceUsbPspcm_stub")]
extern "C" {}
#[link(name = "SceUsbSerialForDriver_stub", kind = "static")]
#[cfg(feature = "SceUsbSerialForDriver_stub")]
extern "C" {
    pub fn ksceUsbSerialClose() -> crate::ctypes::c_int;
    pub fn ksceUsbSerialGetRecvBufferSize() -> crate::ctypes::c_uint;
    pub fn ksceUsbSerialRecv(
        buffer: *mut crate::ctypes::c_void,
        max_len: SceSize,
        unk1: crate::ctypes::c_int,
        unk2: crate::ctypes::c_int,
    ) -> SceSize;
    pub fn ksceUsbSerialSend(
        buffer: *const crate::ctypes::c_void,
        len: SceSize,
        unk1: crate::ctypes::c_int,
        unk2: crate::ctypes::c_int,
    ) -> SceSize;
    pub fn ksceUsbSerialSetup(unk: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn ksceUsbSerialStart() -> crate::ctypes::c_int;
    pub fn ksceUsbSerialStatus() -> crate::ctypes::c_int;
}
#[link(name = "SceUsbSerial_stub", kind = "static")]
#[cfg(feature = "SceUsbSerial_stub")]
extern "C" {
    pub fn sceUsbSerialClose() -> crate::ctypes::c_int;
    pub fn sceUsbSerialGetRecvBufferSize() -> crate::ctypes::c_uint;
    pub fn sceUsbSerialRecv(
        buffer: *mut crate::ctypes::c_void,
        max_len: crate::ctypes::c_uint,
        unk1: crate::ctypes::c_int,
        unk2: crate::ctypes::c_int,
    ) -> crate::ctypes::c_uint;
    pub fn sceUsbSerialSend(
        buffer: *const crate::ctypes::c_void,
        len: crate::ctypes::c_uint,
        unk1: crate::ctypes::c_int,
        unk2: crate::ctypes::c_int,
    ) -> crate::ctypes::c_uint;
    pub fn sceUsbSerialSetup(unk: crate::ctypes::c_int) -> crate::ctypes::c_int;
    pub fn sceUsbSerialStart() -> crate::ctypes::c_int;
    pub fn sceUsbSerialStatus() -> crate::ctypes::c_int;
}
#[link(name = "SceUsbServForDriver_stub", kind = "static")]
#[cfg(feature = "SceUsbServForDriver_stub")]
extern "C" {
    pub fn ksceUsbServAccessoryActivate() -> crate::ctypes::c_int;
    pub fn ksceUsbServAccessoryDeactivate() -> crate::ctypes::c_int;
    pub fn ksceUsbServDisableEtherSuspend() -> crate::ctypes::c_int;
    pub fn ksceUsbServEtherDisable() -> crate::ctypes::c_int;
    pub fn ksceUsbServEtherEnable() -> crate::ctypes::c_int;
    pub fn ksceUsbServMacGet(usbPort: SceUInt) -> SceBool;
    pub fn ksceUsbServMacSelect(usbPort: SceUInt, clientMode: SceBool) -> crate::ctypes::c_int;
}
#[link(name = "SceUsbServ_stub", kind = "static")]
#[cfg(feature = "SceUsbServ_stub")]
extern "C" {
    pub fn sceUsbServAccessoryActivate() -> crate::ctypes::c_int;
    pub fn sceUsbServAccessoryDeactivate() -> crate::ctypes::c_int;
}
#[link(name = "SceUsbstorVStorDriver_stub", kind = "static")]
#[cfg(feature = "SceUsbstorVStorDriver_stub")]
extern "C" {
    pub fn sceUsbstorVStorSetDeviceInfo(
        name: *const crate::ctypes::c_char,
        version: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbstorVStorSetImgFilePath(
        path: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn sceUsbstorVStorStart(type_: SceUsbstorVstorType) -> crate::ctypes::c_int;
    pub fn sceUsbstorVStorStop() -> crate::ctypes::c_int;
}
#[link(name = "SceVideodec_stub", kind = "static")]
#[cfg(feature = "SceVideodec_stub")]
extern "C" {
    pub fn sceAvcdecCreateDecoder(
        codec: SceVideodecType,
        decoder: *mut SceAvcdecCtrl,
        query: *const SceAvcdecQueryDecoderInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceAvcdecDecode(
        decoder: *const SceAvcdecCtrl,
        au: *const SceAvcdecAu,
        array_picture: *mut SceAvcdecArrayPicture,
    ) -> crate::ctypes::c_int;
    pub fn sceAvcdecDeleteDecoder(decoder: *mut SceAvcdecCtrl) -> crate::ctypes::c_int;
    pub fn sceAvcdecQueryDecoderMemSize(
        codec: SceVideodecType,
        query: *const SceAvcdecQueryDecoderInfo,
        decoderInfo: *mut SceAvcdecDecoderInfo,
    ) -> crate::ctypes::c_int;
    pub fn sceVideodecInitLibrary(
        codec: SceVideodecType,
        initInfo: *const SceVideodecQueryInitInfoHwAvcdec,
    ) -> crate::ctypes::c_int;
    pub fn sceVideodecTermLibrary(codec: SceVideodecType) -> crate::ctypes::c_int;
}
#[link(name = "SceVideoExport_stub", kind = "static")]
#[cfg(feature = "SceVideoExport_stub")]
extern "C" {
    pub fn sceVideoExportFromFile(
        in_param: *const VideoExportInputParam,
        unk: crate::ctypes::c_int,
        workingMemory: *mut crate::ctypes::c_void,
        cancelCb: *mut crate::ctypes::c_void,
        progress: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut crate::ctypes::c_void, arg2: crate::ctypes::c_int),
        >,
        user: *mut crate::ctypes::c_void,
        unk2: crate::ctypes::c_int,
        out_param: *mut VideoExportOutputParam,
    ) -> crate::ctypes::c_int;
}
#[link(name = "SceVoiceQoS_stub", kind = "static")]
#[cfg(feature = "SceVoiceQoS_stub")]
extern "C" {}
#[link(name = "SceVoice_stub", kind = "static")]
#[cfg(feature = "SceVoice_stub")]
extern "C" {}
#[link(name = "SceVshBridge_stub", kind = "static")]
#[cfg(feature = "SceVshBridge_stub")]
extern "C" {
    pub fn _vshIoMount(
        id: crate::ctypes::c_int,
        path: *const crate::ctypes::c_char,
        permission: crate::ctypes::c_int,
        buf: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn _vshKernelSearchModuleByName(
        module_name: *const crate::ctypes::c_char,
        buffer: *const crate::ctypes::c_void,
    ) -> SceUID;
    pub fn _vshNpDrmEbootSigConvert(
        eboot_pbp_path: *const crate::ctypes::c_char,
        old_eboot_signature: *const crate::ctypes::c_void,
        new_eboot_signature: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn _vshNpDrmEbootSigGenMultiDisc(
        eboot_pbp_path: *const crate::ctypes::c_char,
        sce_discinfo: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
        sw_version: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn _vshNpDrmEbootSigGenPs1(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_sha256: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
        sw_version: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn _vshNpDrmEbootSigGenPsp(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_sha256: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
        sw_version: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn _vshNpDrmEbootSigVerify(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_signature: *const crate::ctypes::c_char,
        eboot_signature_header: *mut *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn _vshNpDrmPspEbootSigGen(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_sha256: *const crate::ctypes::c_void,
        eboot_signature: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn _vshNpDrmPspEbootVerify(
        eboot_pbp_path: *const crate::ctypes::c_char,
        eboot_signature: *const crate::ctypes::c_char,
        eboot_signature_header: *mut *mut crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
    pub fn _vshSblAimgrGetConsoleId(CID: *mut crate::ctypes::c_char) -> crate::ctypes::c_int;
    pub fn _vshSblAimgrGetSMI(info: *mut SceUInt32) -> crate::ctypes::c_int;
    pub fn _vshSblGetSystemSwVersion(data: *mut SceKernelFwInfo) -> crate::ctypes::c_int;
    pub fn vshIdStorageIsDirty() -> crate::ctypes::c_int;
    pub fn vshIdStorageIsFormatted() -> crate::ctypes::c_int;
    pub fn vshIdStorageIsReadOnly() -> crate::ctypes::c_int;
    pub fn vshIdStorageReadLeaf(
        leafnum: SceSize,
        buf: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn vshIdStorageWriteLeaf(
        leafnum: SceSize,
        buf: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
    pub fn vshIoUmount(
        id: crate::ctypes::c_int,
        force: crate::ctypes::c_int,
        unk2: crate::ctypes::c_int,
        unk3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
    pub fn vshMemoryCardGetCardInsertState() -> crate::ctypes::c_int;
    pub fn vshMsifGetMsInfo(info: *mut SceMsInfo) -> crate::ctypes::c_int;
    pub fn vshRemovableMemoryGetCardInsertState() -> crate::ctypes::c_int;
    pub fn vshSblAimgrIsCEX() -> crate::ctypes::c_int;
    pub fn vshSblAimgrIsDEX() -> crate::ctypes::c_int;
    pub fn vshSblAimgrIsDolce() -> crate::ctypes::c_int;
    pub fn vshSblAimgrIsGenuineDolce() -> crate::ctypes::c_int;
    pub fn vshSblAimgrIsGenuineVITA() -> crate::ctypes::c_int;
    pub fn vshSblAimgrIsTest() -> crate::ctypes::c_int;
    pub fn vshSblAimgrIsTool() -> crate::ctypes::c_int;
    pub fn vshSblAimgrIsVITA() -> crate::ctypes::c_int;
    pub fn vshSblSsIsDevelopmentMode() -> crate::ctypes::c_int;
    pub fn vshSysconHasWWAN() -> crate::ctypes::c_int;
    pub fn vshSysconIduModeClear() -> crate::ctypes::c_int;
    pub fn vshSysconIduModeSet() -> crate::ctypes::c_int;
    pub fn vshSysconIsDownLoaderMode() -> crate::ctypes::c_int;
    pub fn vshSysconIsIduMode() -> crate::ctypes::c_int;
    pub fn vshSysconIsMCEmuCapable() -> crate::ctypes::c_int;
    pub fn vshSysconIsShowMode() -> crate::ctypes::c_int;
    pub fn vshSysconShowModeClear() -> crate::ctypes::c_int;
    pub fn vshSysconShowModeSet() -> crate::ctypes::c_int;
}
#[link(name = "SceWlanBtForDriver_stub", kind = "static")]
#[cfg(feature = "SceWlanBtForDriver_stub")]
extern "C" {}
#[link(name = "SceWlanBt_stub", kind = "static")]
#[cfg(feature = "SceWlanBt_stub")]
extern "C" {}
#[link(name = "vitasdk-utils", kind = "static")]
#[cfg(feature = "vitasdk-utils")]
extern "C" {
    pub fn vitasdk_delete_thread_reent(thid: SceUID) -> crate::ctypes::c_int;
    pub fn vitasdk_get_pthread_data(thid: SceUID) -> *mut crate::ctypes::c_void;
    pub fn vitasdk_get_tls_data(thid: SceUID) -> *mut crate::ctypes::c_void;
}
pub type ArmCpuRegisters = SceArmCpuRegisters;
pub type __gnuc_va_list = u32;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type int_fast8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type int_least8_t = i8;
pub type intmax_t = crate::ctypes::c_longlong;
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
pub type SceAppMgrApplicationMode = crate::ctypes::c_uint;
pub type SceAppMgrErrorCode = crate::ctypes::c_uint;
pub type SceAppMgrInfoBarColor = crate::ctypes::c_uint;
pub type SceAppMgrInfoBarTransparency = crate::ctypes::c_uint;
pub type SceAppMgrInfoBarVisibility = crate::ctypes::c_uint;
pub type SceAppMgrSystemEventType = crate::ctypes::c_uint;
pub type SceAppUtilAppEventType = crate::ctypes::c_uint;
pub type SceAppUtilAppParamId = crate::ctypes::c_uint;
pub type SceAppUtilBgdlStatusType = crate::ctypes::c_uint;
pub type SceAppUtilBootAttribute = crate::ctypes::c_uint;
pub type SceAppUtilErrorCode = crate::ctypes::c_uint;
pub type SceAppUtilSaveDataRemoveMode = crate::ctypes::c_uint;
pub type SceAppUtilSaveDataSaveMode = crate::ctypes::c_uint;
pub type SceAppUtilSaveDataSlotId = crate::ctypes::c_uint;
pub type SceAppUtilSaveDataSlotStatus = crate::ctypes::c_uint;
pub type SceAtracDecoderStatus = crate::ctypes::c_uint;
pub type SceAtracErrorCode = crate::ctypes::c_uint;
pub type SceAtracLoopStatus = crate::ctypes::c_uint;
pub type SceAudiodecCelpBitrate = crate::ctypes::c_uint;
pub type SceAudiodecErrorCode = crate::ctypes::c_uint;
pub type SceAudiodecMpegVersion = crate::ctypes::c_uint;
pub type SceAudiodecType = crate::ctypes::c_uint;
pub type SceAudioencCelpBitrate = crate::ctypes::c_uint;
pub type SceAudioencCelpErrorCode = crate::ctypes::c_uint;
pub type SceAudioencErrorCode = crate::ctypes::c_uint;
pub type SceAudioInErrorCode = crate::ctypes::c_uint;
pub type SceAudioInParam = crate::ctypes::c_uint;
pub type SceAudioInPortType = crate::ctypes::c_uint;
pub type SceAudioOutAlcMode = crate::ctypes::c_uint;
pub type SceAudioOutChannelFlag = crate::ctypes::c_uint;
pub type SceAudioOutConfigType = crate::ctypes::c_uint;
pub type SceAudioOutErrorCode = crate::ctypes::c_uint;
pub type SceAudioOutMode = crate::ctypes::c_uint;
pub type SceAudioOutParam = crate::ctypes::c_uint;
pub type SceAudioOutPortType = crate::ctypes::c_uint;
pub type SceAvcdecErrorCode = crate::ctypes::c_uint;
pub type SceAvcdecPixelFormat = crate::ctypes::c_uint;
pub type SceAVConfigColorSpaceMode = crate::ctypes::c_uint;
pub type SceAvPlayerAlloc = ::core::option::Option<
    unsafe extern "C" fn(
        arg: *mut crate::ctypes::c_void,
        alignment: u32,
        size: u32,
    ) -> *mut crate::ctypes::c_void,
>;
pub type SceAvPlayerAllocFrame = ::core::option::Option<
    unsafe extern "C" fn(
        arg: *mut crate::ctypes::c_void,
        alignment: u32,
        size: u32,
    ) -> *mut crate::ctypes::c_void,
>;
pub type SceAvPlayerCloseFile = ::core::option::Option<
    unsafe extern "C" fn(p: *mut crate::ctypes::c_void) -> crate::ctypes::c_int,
>;
pub type SceAvPlayerErrorCode = crate::ctypes::c_uint;
pub type SceAvPlayerEventCallback = ::core::option::Option<
    unsafe extern "C" fn(
        p: *mut crate::ctypes::c_void,
        argEventId: i32,
        argSourceId: i32,
        argEventData: *mut crate::ctypes::c_void,
    ),
>;
pub type SceAvPlayerFree = ::core::option::Option<
    unsafe extern "C" fn(arg: *mut crate::ctypes::c_void, ptr: *mut crate::ctypes::c_void),
>;
pub type SceAvPlayerFreeFrame = ::core::option::Option<
    unsafe extern "C" fn(arg: *mut crate::ctypes::c_void, ptr: *mut crate::ctypes::c_void),
>;
pub type SceAvPlayerHandle = crate::ctypes::c_int;
pub type SceAvPlayerOpenFile = ::core::option::Option<
    unsafe extern "C" fn(
        p: *mut crate::ctypes::c_void,
        filename: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int,
>;
pub type SceAvPlayerReadOffsetFile = ::core::option::Option<
    unsafe extern "C" fn(
        p: *mut crate::ctypes::c_void,
        buffer: *mut u8,
        position: u64,
        length: u32,
    ) -> crate::ctypes::c_int,
>;
pub type SceAvPlayerSizeFile =
    ::core::option::Option<unsafe extern "C" fn(p: *mut crate::ctypes::c_void) -> u64>;
pub type SceAvPlayerStreamType = crate::ctypes::c_uint;
pub type SceAvPlayerTrickSpeeds = crate::ctypes::c_int;
pub type SceBgAppUtilErrorCode = crate::ctypes::c_uint;
pub type SceBool = crate::ctypes::c_int;
pub type SceBtCallback = ::core::option::Option<
    unsafe extern "C" fn(
        r0: crate::ctypes::c_int,
        r1: crate::ctypes::c_int,
        r2: crate::ctypes::c_int,
        r3: crate::ctypes::c_int,
    ),
>;
pub type SceBtErrorCode = crate::ctypes::c_uint;
pub type SceBtHidRequest = _SceBtHidRequest;
pub type SceByte = crate::ctypes::c_uchar;
pub type SceByte8 = crate::ctypes::c_uchar;
pub type SceCameraAntiFlicker = crate::ctypes::c_uint;
pub type SceCameraBacklight = crate::ctypes::c_uint;
pub type SceCameraDevice = crate::ctypes::c_uint;
pub type SceCameraEffect = crate::ctypes::c_uint;
pub type SceCameraErrorCode = crate::ctypes::c_uint;
pub type SceCameraExposureCompensation = crate::ctypes::c_int;
pub type SceCameraFormat = crate::ctypes::c_uint;
pub type SceCameraFrameRate = crate::ctypes::c_uint;
pub type SceCameraGain = crate::ctypes::c_uint;
pub type SceCameraISO = crate::ctypes::c_uint;
pub type SceCameraNightmode = crate::ctypes::c_uint;
pub type SceCameraPriority = crate::ctypes::c_uint;
pub type SceCameraResolution = crate::ctypes::c_uint;
pub type SceCameraReverse = crate::ctypes::c_uint;
pub type SceCameraSaturation = crate::ctypes::c_uint;
pub type SceCameraSharpness = crate::ctypes::c_uint;
pub type SceCameraWhiteBalance = crate::ctypes::c_uint;
pub type SceChar8 = i8;
pub type SceClassCallback = ::core::option::Option<
    unsafe extern "C" fn(item: *mut crate::ctypes::c_void) -> crate::ctypes::c_int,
>;
pub type SceClibMspace = *mut crate::ctypes::c_void;
pub type SceCommonDialogBgColor = SceCommonDialogColor;
pub type SceCommonDialogErrorCode = crate::ctypes::c_uint;
pub type SceCommonDialogResult = crate::ctypes::c_uint;
pub type SceCommonDialogStatus = crate::ctypes::c_uint;
pub type SceCompatCacheMode = crate::ctypes::c_uint;
pub type SceCompatPeripheralMode = crate::ctypes::c_uint;
pub type SceCreateUidObjOpt = SceGUIDKernelCreateOpt;
pub type SceCtrlButtons = crate::ctypes::c_uint;
pub type SceCtrlErrorCode = crate::ctypes::c_uint;
pub type SceCtrlExternalInputMode = crate::ctypes::c_uint;
pub type SceCtrlPadInputMode = crate::ctypes::c_uint;
pub type SceCTypeFlag = crate::ctypes::c_uint;
pub type SceDbgLogLevel = crate::ctypes::c_uint;
pub type SceDisplayErrorCode = crate::ctypes::c_uint;
pub type SceDisplayPixelFormat = crate::ctypes::c_uint;
pub type SceDisplaySetBufSync = crate::ctypes::c_uint;
pub type SceDouble = f64;
pub type SceDouble64 = f64;
pub type SceDsiErrorCode = crate::ctypes::c_uint;
pub type SceDsiHead = crate::ctypes::c_uint;
pub type SceEventFlagAttributes = crate::ctypes::c_uint;
pub type SceEventFlagWaitTypes = crate::ctypes::c_uint;
pub type SceExcpHandlingCode = crate::ctypes::c_uint;
pub type SceExcpKind = crate::ctypes::c_uint;
pub type SceExcpmgrExceptionHandler = ::core::option::Option<
    unsafe extern "C" fn(context: *mut SceExcpmgrExceptionContext, code: SceExcpHandlingCode),
>;
pub type SceFiberEntry =
    ::core::option::Option<unsafe extern "C" fn(argOnInitialize: SceUInt32, argOnRun: SceUInt32)>;
pub type SceFiberErrorCode = crate::ctypes::c_uint;
pub type SceFiosKernelOverlayDH = i32;
pub type SceFiosKernelOverlayID = i32;
pub type SceFiosOverlayID = i32;
pub type SceFiosOverlayType = crate::ctypes::c_uint;
pub type SceFloat = f32;
pub type SceFloat32 = f32;
pub type SceFontErrorCode = crate::ctypes::c_uint;
pub type SceFontFamilyCode = crate::ctypes::c_uint;
pub type SceFontHandle = *mut crate::ctypes::c_void;
pub type SceFontLanguageCode = crate::ctypes::c_uint;
pub type SceFontLibHandle = *mut crate::ctypes::c_void;
pub type SceFontPixelFormatCode = crate::ctypes::c_uint;
pub type SceFontStyleCode = crate::ctypes::c_uint;
pub type SceGpioErrorCode = crate::ctypes::c_uint;
pub type SceGpioPortMasks = crate::ctypes::c_uint;
pub type SceGpioPortMode = crate::ctypes::c_uint;
pub type SceGxmAttributeFormat = crate::ctypes::c_uint;
pub type SceGxmBlendFactor = crate::ctypes::c_uint;
pub type SceGxmBlendFunc = crate::ctypes::c_uint;
pub type SceGxmColorBaseFormat = crate::ctypes::c_uint;
pub type SceGxmColorFormat = crate::ctypes::c_uint;
pub type SceGxmColorMask = crate::ctypes::c_uint;
pub type SceGxmColorSurfaceDitherMode = crate::ctypes::c_uint;
pub type SceGxmColorSurfaceGammaMode = crate::ctypes::c_uint;
pub type SceGxmColorSurfaceScaleMode = crate::ctypes::c_uint;
pub type SceGxmColorSurfaceType = crate::ctypes::c_uint;
pub type SceGxmColorSwizzle1Mode = crate::ctypes::c_uint;
pub type SceGxmColorSwizzle2Mode = crate::ctypes::c_uint;
pub type SceGxmColorSwizzle3Mode = crate::ctypes::c_uint;
pub type SceGxmColorSwizzle4Mode = crate::ctypes::c_uint;
pub type SceGxmCullMode = crate::ctypes::c_uint;
pub type SceGxmDepthFunc = crate::ctypes::c_uint;
pub type SceGxmDepthStencilForceLoadMode = crate::ctypes::c_uint;
pub type SceGxmDepthStencilForceStoreMode = crate::ctypes::c_uint;
pub type SceGxmDepthStencilFormat = crate::ctypes::c_uint;
pub type SceGxmDepthStencilSurfaceType = crate::ctypes::c_uint;
pub type SceGxmDepthWriteMode = crate::ctypes::c_uint;
pub type SceGxmDisplayQueueCallback =
    ::core::option::Option<unsafe extern "C" fn(callbackData: *const crate::ctypes::c_void)>;
pub type SceGxmEdgeEnableFlags = crate::ctypes::c_uint;
pub type SceGxmErrorCode = crate::ctypes::c_uint;
pub type SceGxmFragmentProgramMode = crate::ctypes::c_uint;
pub type SceGxmIndexFormat = crate::ctypes::c_uint;
pub type SceGxmIndexSource = crate::ctypes::c_uint;
pub type SceGxmLineFillLastPixelMode = crate::ctypes::c_uint;
pub type SceGxmMemoryAttribFlags = crate::ctypes::c_uint;
pub type SceGxmMidSceneFlags = crate::ctypes::c_uint;
pub type SceGxmMultisampleMode = crate::ctypes::c_uint;
pub type SceGxmOutputRegisterFormat = crate::ctypes::c_uint;
pub type SceGxmOutputRegisterSize = crate::ctypes::c_uint;
pub type SceGxmParameterCategory = crate::ctypes::c_uint;
pub type SceGxmParameterSemantic = crate::ctypes::c_uint;
pub type SceGxmParameterType = crate::ctypes::c_uint;
pub type SceGxmPassType = crate::ctypes::c_uint;
pub type SceGxmPolygonMode = crate::ctypes::c_uint;
pub type SceGxmPrecomputedWordCount = crate::ctypes::c_uint;
pub type SceGxmPrimitiveType = crate::ctypes::c_uint;
pub type SceGxmProgramType = crate::ctypes::c_uint;
pub type SceGxmRegionClipMode = crate::ctypes::c_uint;
pub type SceGxmRenderTargetFlags = crate::ctypes::c_uint;
pub type SceGxmSceneFlags = crate::ctypes::c_uint;
pub type SceGxmShaderPatcherBufferAllocCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userData: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> *mut crate::ctypes::c_void,
>;
pub type SceGxmShaderPatcherBufferFreeCallback = ::core::option::Option<
    unsafe extern "C" fn(userData: *mut crate::ctypes::c_void, mem: *mut crate::ctypes::c_void),
>;
pub type SceGxmShaderPatcherHostAllocCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userData: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> *mut crate::ctypes::c_void,
>;
pub type SceGxmShaderPatcherHostFreeCallback = ::core::option::Option<
    unsafe extern "C" fn(userData: *mut crate::ctypes::c_void, mem: *mut crate::ctypes::c_void),
>;
pub type SceGxmShaderPatcherId = *mut SceGxmRegisteredProgram;
pub type SceGxmShaderPatcherUsseAllocCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userData: *mut crate::ctypes::c_void,
        size: SceSize,
        usseOffset: *mut crate::ctypes::c_uint,
    ) -> *mut crate::ctypes::c_void,
>;
pub type SceGxmShaderPatcherUsseFreeCallback = ::core::option::Option<
    unsafe extern "C" fn(userData: *mut crate::ctypes::c_void, mem: *mut crate::ctypes::c_void),
>;
pub type SceGxmStencilFunc = crate::ctypes::c_uint;
pub type SceGxmStencilOp = crate::ctypes::c_uint;
pub type SceGxmTextureAddrMode = crate::ctypes::c_uint;
pub type SceGxmTextureBaseFormat = crate::ctypes::c_uint;
pub type SceGxmTextureFilter = crate::ctypes::c_uint;
pub type SceGxmTextureFormat = crate::ctypes::c_uint;
pub type SceGxmTextureGammaMode = crate::ctypes::c_uint;
pub type SceGxmTextureMipFilter = crate::ctypes::c_uint;
pub type SceGxmTextureNormalizeMode = crate::ctypes::c_uint;
pub type SceGxmTextureSwizzle1Mode = crate::ctypes::c_uint;
pub type SceGxmTextureSwizzle2Mode = crate::ctypes::c_uint;
pub type SceGxmTextureSwizzle2ModeAlt = crate::ctypes::c_uint;
pub type SceGxmTextureSwizzle3Mode = crate::ctypes::c_uint;
pub type SceGxmTextureSwizzle4Mode = crate::ctypes::c_uint;
pub type SceGxmTextureSwizzleYUV420Mode = crate::ctypes::c_uint;
pub type SceGxmTextureSwizzleYUV422Mode = crate::ctypes::c_uint;
pub type SceGxmTextureType = crate::ctypes::c_uint;
pub type SceGxmTransferColorKeyMode = crate::ctypes::c_uint;
pub type SceGxmTransferFlags = crate::ctypes::c_uint;
pub type SceGxmTransferFormat = crate::ctypes::c_uint;
pub type SceGxmTransferType = crate::ctypes::c_uint;
pub type SceGxmTwoSidedMode = crate::ctypes::c_uint;
pub type SceGxmViewportMode = crate::ctypes::c_uint;
pub type SceGxmVisibilityTestMode = crate::ctypes::c_uint;
pub type SceGxmVisibilityTestOp = crate::ctypes::c_uint;
pub type SceGxmWBufferMode = crate::ctypes::c_uint;
pub type SceGxmWClampMode = crate::ctypes::c_uint;
pub type SceGxmYuvProfile = crate::ctypes::c_uint;
pub type SceGxtErrorCode = crate::ctypes::c_uint;
pub type SceHttpAddHeaderMode = crate::ctypes::c_uint;
pub type SceHttpAuthInfoCallback = ::core::option::Option<
    unsafe extern "C" fn(
        request: crate::ctypes::c_int,
        authType: SceHttpAuthType,
        realm: *const crate::ctypes::c_char,
        username: *mut crate::ctypes::c_char,
        password: *mut crate::ctypes::c_char,
        needEntity: crate::ctypes::c_int,
        entityBody: *mut *mut crate::ctypes::c_uchar,
        entitySize: *mut crate::ctypes::c_uint,
        save: *mut crate::ctypes::c_int,
        userArg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceHttpAuthType = crate::ctypes::c_uint;
pub type SceHttpCookieRecvCallback = ::core::option::Option<
    unsafe extern "C" fn(
        request: crate::ctypes::c_int,
        url: *const crate::ctypes::c_char,
        cookieHeader: *const crate::ctypes::c_char,
        headerLen: crate::ctypes::c_uint,
        userArg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceHttpCookieSendCallback = ::core::option::Option<
    unsafe extern "C" fn(
        request: crate::ctypes::c_int,
        url: *const crate::ctypes::c_char,
        cookieHeader: *const crate::ctypes::c_char,
        userArg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceHttpErrorCode = crate::ctypes::c_uint;
pub type SceHttpMethods = crate::ctypes::c_uint;
pub type SceHttpProxyMode = crate::ctypes::c_uint;
pub type SceHttpRedirectCallback = ::core::option::Option<
    unsafe extern "C" fn(
        request: crate::ctypes::c_int,
        statusCode: crate::ctypes::c_int,
        method: *mut crate::ctypes::c_int,
        location: *const crate::ctypes::c_char,
        userArg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceHttpsCallback = ::core::option::Option<
    unsafe extern "C" fn(
        verifyEsrr: crate::ctypes::c_uint,
        sslCert: *const *mut crate::ctypes::c_void,
        certNum: crate::ctypes::c_int,
        userArg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceHttpsErrorCode = crate::ctypes::c_uint;
pub type SceHttpsFlag = crate::ctypes::c_uint;
pub type SceHttpSslVersion = crate::ctypes::c_uint;
pub type SceHttpsSslErrorCode = crate::ctypes::c_uint;
pub type SceHttpStatusCode = crate::ctypes::c_uint;
pub type SceHttpUriBuildType = crate::ctypes::c_uint;
pub type SceHttpVersion = crate::ctypes::c_uint;
pub type SceI2cErrorCode = crate::ctypes::c_uint;
pub type SceIftuErrorCode = crate::ctypes::c_uint;
pub type SceIftuPixelformat = crate::ctypes::c_uint;
pub type SceImeDialogButton = crate::ctypes::c_uint;
pub type SceImeDialogDialogMode = crate::ctypes::c_uint;
pub type SceImeDialogErrorCode = crate::ctypes::c_uint;
pub type SceImeDialogTextboxMode = crate::ctypes::c_uint;
pub type SceImeEnterLabel = crate::ctypes::c_uint;
pub type SceImeErrorCode = crate::ctypes::c_uint;
pub type SceImeEvent = crate::ctypes::c_uint;
pub type SceImeEventHandler = ::core::option::Option<
    unsafe extern "C" fn(arg: *mut crate::ctypes::c_void, e: *const SceImeEventData),
>;
pub type SceImeLanguage = crate::ctypes::c_uint;
pub type SceImeOption = crate::ctypes::c_uint;
pub type SceImeTextFilter = ::core::option::Option<
    unsafe extern "C" fn(
        outText: *mut SceWChar16,
        outTextLength: *mut SceUInt32,
        srcText: *const SceWChar16,
        srcTextLength: SceUInt32,
    ) -> SceInt32,
>;
pub type SceImeType = crate::ctypes::c_uint;
pub type SceIncomingDialogErrorCode = crate::ctypes::c_uint;
pub type SceIncomingDialogStatus = crate::ctypes::c_uint;
pub type SceInt = i32;
pub type SceInt16 = i16;
pub type SceInt32 = i32;
pub type SceInt64 = i64;
pub type SceInt8 = i8;
pub type SceIntPtr = crate::ctypes::c_int;
pub type SceIoAccessMode = crate::ctypes::c_uint;
pub type SceIoDevType = crate::ctypes::c_uint;
pub type SceIoFileMode = crate::ctypes::c_uint;
pub type SceIoMode = crate::ctypes::c_uint;
pub type SceIoSeekMode = crate::ctypes::c_uint;
pub type SceJpegArmEncoderContext = *mut crate::ctypes::c_void;
pub type SceJpegArmEncoderHeaderMode = crate::ctypes::c_uint;
pub type SceJpegArmEncoderPixelFormat = crate::ctypes::c_uint;
pub type SceJpegArmErrorCode = crate::ctypes::c_uint;
pub type SceJpegEncArmErrorCode = crate::ctypes::c_uint;
pub type SceJpegEncErrorCode = crate::ctypes::c_uint;
pub type SceJpegEncoderContext = *mut crate::ctypes::c_void;
pub type SceJpegEncoderHeaderMode = crate::ctypes::c_uint;
pub type SceJpegEncoderInitParamOption = crate::ctypes::c_uint;
pub type SceJpegEncoderPixelFormat = crate::ctypes::c_uint;
pub type SceKernelAllocMemBlockAttr = crate::ctypes::c_uint;
pub type SceKernelAssertLevel = crate::ctypes::c_uint;
pub type SceKernelCallbackFunction = ::core::option::Option<
    unsafe extern "C" fn(
        notifyId: crate::ctypes::c_int,
        notifyCount: crate::ctypes::c_int,
        notifyArg: crate::ctypes::c_int,
        userData: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceKernelClock = SceUInt64;
pub type SceKernelCoredumpStateFinishCallback = ::core::option::Option<
    unsafe extern "C" fn(
        task_id: crate::ctypes::c_int,
        pid: SceUID,
        result: crate::ctypes::c_int,
        path: *const crate::ctypes::c_char,
        path_len: SceSize,
        unk: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int,
>;
pub type SceKernelCoredumpStateUpdateCallback = ::core::option::Option<
    unsafe extern "C" fn(
        task_id: crate::ctypes::c_int,
        pid: SceUID,
        progress: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int,
>;
pub type SceKernelCoredumpTriggerFunc = ::core::option::Option<
    unsafe extern "C" fn(
        pid: SceUID,
        update_func: SceKernelCoredumpStateUpdateCallback,
        finish_func: SceKernelCoredumpStateFinishCallback,
        param: *mut SceCoredumpTriggerParam,
    ) -> crate::ctypes::c_int,
>;
pub type SceKernelDebugInfoFlags = crate::ctypes::c_uint;
pub type SceKernelDebugLevel = crate::ctypes::c_uint;
pub type SceKernelDebugMessageContext = SceKernelDebugInfo;
pub type SceKernelDmacId = crate::ctypes::c_uint;
pub type SceKernelDmaOpCallback = ::core::option::Option<
    unsafe extern "C" fn(
        opid: SceKernelDmaOpId,
        stat: SceUInt32,
        pUserData: *mut crate::ctypes::c_void,
        pTag: *mut SceKernelDmaOpTag,
    ),
>;
pub type SceKernelDmaOpFlag = crate::ctypes::c_uint;
pub type SceKernelDmaOpId = SceInt32;
pub type SceKernelDmaOpSyncMode = crate::ctypes::c_uint;
pub type SceKernelErrorCode = crate::ctypes::c_uint;
pub type SceKernelFwInfo = SceKernelSystemSwVersion;
pub type SceKernelGetSystemSwVersionFunc =
    ::core::option::Option<unsafe extern "C" fn() -> crate::ctypes::c_int>;
pub type SceKernelHeapAttr = crate::ctypes::c_uint;
pub type SceKernelIdListType = crate::ctypes::c_uint;
pub type SceKernelIntrHandler = ::core::option::Option<
    unsafe extern "C" fn(
        unk: crate::ctypes::c_int,
        userCtx: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceKernelIntrOptHandlersCb1 = ::core::option::Option<
    unsafe extern "C" fn(
        intr_code: crate::ctypes::c_int,
        subintr_code: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int,
>;
pub type SceKernelIntrOptHandlersCb2 = ::core::option::Option<
    unsafe extern "C" fn(
        intr_code: crate::ctypes::c_int,
        subintr_code: crate::ctypes::c_int,
        arg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceKernelIntrOptHandlersCb3 = ::core::option::Option<
    unsafe extern "C" fn(
        intr_code: crate::ctypes::c_int,
        subintr_code: crate::ctypes::c_int,
        handler: SceKernelSubIntrHandler,
        register_arg: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceKernelIntrStatus = crate::ctypes::c_int;
pub type SceKernelMemBlockType = SceUInt32;
pub type SceKernelMemoryAccessType = crate::ctypes::c_uint;
pub type SceKernelMemoryRefPerm = crate::ctypes::c_uint;
pub type SceKernelMemoryType = crate::ctypes::c_uint;
pub type SceKernelModel = crate::ctypes::c_uint;
pub type SceKernelModuleState = crate::ctypes::c_uint;
pub type SceKernelMutexAttribute = crate::ctypes::c_uint;
pub type SceKernelPaddrList = SceKernelPAVector;
pub type SceKernelPowerTickType = crate::ctypes::c_uint;
pub type SceKernelPreloadInhibit = crate::ctypes::c_uint;
pub type SceKernelProcessPrioritySystem = crate::ctypes::c_uint;
pub type SceKernelProcessPriorityUser = crate::ctypes::c_uint;
pub type SceKernelProcessType = SceUInt32;
pub type SceKernelRWSpinlock = crate::ctypes::c_int;
pub type SceKernelSpinlock = crate::ctypes::c_int;
pub type SceKernelSubIntrHandler = ::core::option::Option<
    unsafe extern "C" fn(
        subintr_arg: *mut crate::ctypes::c_void,
        register_arg: *mut crate::ctypes::c_void,
        intr_priority: crate::ctypes::c_uchar,
    ) -> crate::ctypes::c_int,
>;
pub type SceKernelSysClock = SceUInt64;
pub type SceKernelSysrootSelfIndex = crate::ctypes::c_uint;
pub type SceKernelThreadEntry = ::core::option::Option<
    unsafe extern "C" fn(args: SceSize, argp: *mut crate::ctypes::c_void) -> crate::ctypes::c_int,
>;
pub type SceKernelTime = SceUInt32;
pub type SceKernelWaitableAttribute = crate::ctypes::c_uint;
pub type SceKernelWorkQueueWorkFunction = ::core::option::Option<
    unsafe extern "C" fn(args: *mut crate::ctypes::c_void) -> crate::ctypes::c_int,
>;
pub type SceLocationDialogResult = crate::ctypes::c_uint;
pub type SceLocationDialogStatus = crate::ctypes::c_uint;
pub type SceLocationErrorCode = crate::ctypes::c_uint;
pub type SceLocationHandle = SceUInt32;
pub type SceLocationHeadingInfoCallback = ::core::option::Option<
    unsafe extern "C" fn(
        result: SceInt32,
        handle: SceLocationHandle,
        heading: *const SceLocationHeadingInfo,
        userdata: *mut crate::ctypes::c_void,
    ),
>;
pub type SceLocationHeadingMethod = crate::ctypes::c_uint;
pub type SceLocationLocationInfoCallback = ::core::option::Option<
    unsafe extern "C" fn(
        result: SceInt32,
        handle: SceLocationHandle,
        location: *const SceLocationLocationInfo,
        userdata: *mut crate::ctypes::c_void,
    ),
>;
pub type SceLocationLocationMethod = crate::ctypes::c_uint;
pub type SceLocationPermissionApplicationStatus = crate::ctypes::c_uint;
pub type SceLocationPermissionStatus = crate::ctypes::c_uint;
pub type SceLong64 = i64;
pub type SceMode = crate::ctypes::c_int;
pub type SceMotionErrorCode = crate::ctypes::c_uint;
pub type SceMotionMagFieldStability = crate::ctypes::c_uint;
pub type SceMsgDialogButtonId = crate::ctypes::c_uint;
pub type SceMsgDialogButtonType = crate::ctypes::c_uint;
pub type SceMsgDialogEnvFlag = crate::ctypes::c_uint;
pub type SceMsgDialogErrorCode = crate::ctypes::c_uint;
pub type SceMsgDialogFontSize = crate::ctypes::c_uint;
pub type SceMsgDialogMode = crate::ctypes::c_uint;
pub type SceMsgDialogProgressBarTarget = crate::ctypes::c_uint;
pub type SceMsgDialogProgressBarType = crate::ctypes::c_uint;
pub type SceMsgDialogSystemMessageType = crate::ctypes::c_uint;
pub type SceName = *mut crate::ctypes::c_char;
pub type SceNetCheckDialoErrorCode = crate::ctypes::c_uint;
pub type SceNetCheckDialogMode = crate::ctypes::c_uint;
pub type SceNetCheckDialogPS3ConnectAction = crate::ctypes::c_uint;
pub type SceNetCtlCallback = ::core::option::Option<
    unsafe extern "C" fn(
        event_type: crate::ctypes::c_int,
        arg: *mut crate::ctypes::c_void,
    ) -> *mut crate::ctypes::c_void,
>;
pub type SceNetCtlInfoType = crate::ctypes::c_uint;
pub type SceNetCtlState = crate::ctypes::c_uint;
pub type SceNetDumpType = crate::ctypes::c_uint;
pub type SceNetEmulationFlag = crate::ctypes::c_uint;
pub type SceNetEpollControlFlag = crate::ctypes::c_uint;
pub type SceNetEpollEventType = crate::ctypes::c_uint;
pub type SceNetErrorCode = crate::ctypes::c_uint;
pub type SceNetIcmpCode = crate::ctypes::c_uint;
pub type SceNetIcmpType = crate::ctypes::c_uint;
pub type SceNetKernelErrorCode = crate::ctypes::c_uint;
pub type SceNetLibnetErrorCode = crate::ctypes::c_uint;
pub type SceNetMsgFlag = crate::ctypes::c_uint;
pub type SceNetProtocol = crate::ctypes::c_uint;
pub type SceNetResolverAbortFlag = crate::ctypes::c_uint;
pub type SceNetResolverErrorCode = crate::ctypes::c_uint;
pub type SceNetResolverFlag = crate::ctypes::c_uint;
pub type SceNetResolverFunctionAllocate = ::core::option::Option<
    unsafe extern "C" fn(
        size: crate::ctypes::c_uint,
        rid: crate::ctypes::c_int,
        name: *const crate::ctypes::c_char,
        user: *mut crate::ctypes::c_void,
    ) -> *mut crate::ctypes::c_void,
>;
pub type SceNetResolverFunctionFree = ::core::option::Option<
    unsafe extern "C" fn(
        ptr: *mut crate::ctypes::c_void,
        rid: crate::ctypes::c_int,
        name: *const crate::ctypes::c_char,
        user: *mut crate::ctypes::c_void,
    ),
>;
pub type SceNetShutdownFlag = crate::ctypes::c_uint;
pub type SceNetSocketAbortFlag = crate::ctypes::c_uint;
pub type SceNetSocketOption = crate::ctypes::c_uint;
pub type SceNetSocketType = crate::ctypes::c_uint;
pub type SceNetSockInfoFlag = crate::ctypes::c_uint;
pub type SceNetSockInfoState = crate::ctypes::c_uint;
pub type SceNgsCallbackFunc =
    ::core::option::Option<unsafe extern "C" fn(callback_info: *const SceNgsCallbackInfo)>;
pub type SceNgsHPatch = SceUInt32;
pub type SceNgsHRack = SceUInt32;
pub type SceNgsHSynSystem = SceUInt32;
pub type SceNgsHVoice = SceUInt32;
pub type SceNgsModuleCallbackFunc = SceNgsCallbackFunc;
pub type SceNgsModuleID = SceUInt32;
pub type SceNgsParamsErrorCallbackFunc = SceNgsCallbackFunc;
pub type SceNgsRackReleaseCallbackFunc = SceNgsCallbackFunc;
pub type SceNgsSulphaUpdateCallback = *mut crate::ctypes::c_void;
pub type SceNID = crate::ctypes::c_uint;
pub type SceNotificationUitlErrorCode = crate::ctypes::c_uint;
pub type SceNotificationUtilProgressEventHandler =
    ::core::option::Option<unsafe extern "C" fn(eventId: crate::ctypes::c_int)>;
pub type _sceNpDrmPackageDecrypt_opt = _sceNpDrmPackageDecrypt;
pub type SceOff = SceInt64;
pub type _ScePerfArmPmonEventCode = crate::ctypes::c_uint;
pub type ScePID = crate::ctypes::c_int;
pub type ScePowerCallback = ::core::option::Option<
    unsafe extern "C" fn(
        notifyId: crate::ctypes::c_int,
        notifyCount: crate::ctypes::c_int,
        powerInfo: crate::ctypes::c_int,
        userData: *mut crate::ctypes::c_void,
    ),
>;
pub type ScePowerCallbackType = crate::ctypes::c_uint;
pub type ScePowerConfigurationMode = crate::ctypes::c_uint;
pub type ScePowerErrorCode = crate::ctypes::c_uint;
pub type SceProductCode = crate::ctypes::c_uint;
pub type SceProductMode = crate::ctypes::c_char;
pub type ScePromoterUtilityPackageType = crate::ctypes::c_uint;
pub type ScePspnetAdhocctlAdhocType = crate::ctypes::c_uint;
pub type ScePspnetAdhocctlErrorCode = crate::ctypes::c_uint;
pub type ScePspnetAdhocErrorCode = crate::ctypes::c_uint;
pub type ScePspnetAdhocEvent = crate::ctypes::c_uint;
pub type ScePspnetAdhocFlags = crate::ctypes::c_uint;
pub type ScePspnetAdhocPtpState = crate::ctypes::c_uint;
pub type ScePvfAllocFunc = ::core::option::Option<
    unsafe extern "C" fn(userData: ScePvfPointer, size: ScePvfU32) -> ScePvfPointer,
>;
pub type ScePvfBool = ScePvfU32;
pub type ScePvfBoolValue = crate::ctypes::c_uint;
pub type ScePvfCharCode = ScePvfU16;
pub type ScePvfDataAccessMode = crate::ctypes::c_uint;
pub type ScePvfError = ScePvfS32;
pub type ScePvfErrorCode = crate::ctypes::c_uint;
pub type ScePvfFamilyCode = crate::ctypes::c_uint;
pub type ScePvfFloat32 = f32;
pub type ScePvfFontCacheLockFunc =
    ::core::option::Option<unsafe extern "C" fn(cacheInstance: ScePvfPointer) -> ScePvfS32>;
pub type ScePvfFontChcheFindFunc = ::core::option::Option<
    unsafe extern "C" fn(
        chcheInstance: ScePvfPointer,
        hashValue: ScePvfU32,
        key: ScePvfPointer,
        result: *mut ScePvfBool,
    ) -> ScePvfPointer,
>;
pub type ScePvfFontChcheReadFromCacheFunc = ::core::option::Option<
    unsafe extern "C" fn(
        cacheInstance: ScePvfPointer,
        cacheSlot: ScePvfPointer,
        data0: ScePvfPointer,
    ) -> ScePvfS32,
>;
pub type ScePvfFontChcheUnlockFunc =
    ::core::option::Option<unsafe extern "C" fn(cacheInstance: ScePvfPointer) -> ScePvfS32>;
pub type ScePvfFontChcheWriteKeyValueToCacheFunc = ::core::option::Option<
    unsafe extern "C" fn(
        cacheInstance: ScePvfPointer,
        chcheSlot: ScePvfPointer,
        key: ScePvfPointer,
    ) -> ScePvfS32,
>;
pub type ScePvfFontChcheWriteToCacheFunc = ::core::option::Option<
    unsafe extern "C" fn(
        cacheInstance: ScePvfPointer,
        cacheSlot: ScePvfPointer,
        data0: ScePvfPointer,
        size: ScePvfInt,
    ) -> ScePvfS32,
>;
pub type ScePvfFontId = *mut crate::ctypes::c_void;
pub type ScePvfFontIndex = ScePvfS32;
pub type ScePvfFontVendorCountryCode = crate::ctypes::c_uint;
pub type ScePvfFreeFunc =
    ::core::option::Option<unsafe extern "C" fn(userData: ScePvfPointer, ptr: ScePvfPointer)>;
pub type ScePvfHandle = *mut crate::ctypes::c_void;
pub type ScePvfImageByfferPixelFormatType = crate::ctypes::c_uint;
pub type ScePvfInt = ScePvfS32;
pub type ScePvfLanguageCode = crate::ctypes::c_uint;
pub type ScePvfLibId = *mut crate::ctypes::c_void;
pub type ScePvfPointer = *mut crate::ctypes::c_void;
pub type ScePvfReallocFunc = ::core::option::Option<
    unsafe extern "C" fn(
        userData: ScePvfPointer,
        old_ptr: ScePvfPointer,
        size: ScePvfU32,
    ) -> ScePvfPointer,
>;
pub type ScePvfRegionCode = crate::ctypes::c_uint;
pub type ScePvfS32 = crate::ctypes::c_int;
pub type ScePvfStyleCode = crate::ctypes::c_uint;
pub type ScePvfSubstyle = crate::ctypes::c_uint;
pub type ScePvfU16 = crate::ctypes::c_ushort;
pub type ScePvfU32 = crate::ctypes::c_uint;
pub type ScePvfU8 = crate::ctypes::c_uchar;
pub type ScePVoid = *mut crate::ctypes::c_void;
pub type SceRazorGpuLiveMetricsGroup = crate::ctypes::c_uint;
pub type SceRazorLiveTraceMetricEntryType = crate::ctypes::c_uint;
pub type SceRazorLiveTraceMetricJobType = crate::ctypes::c_uint;
pub type SceRtcDayOfWeek = crate::ctypes::c_uint;
pub type SceRtcErrorCode = crate::ctypes::c_uint;
pub type SceSblSmCommId = crate::ctypes::c_int;
pub type SceSByte = crate::ctypes::c_schar;
pub type SceSByte8 = crate::ctypes::c_schar;
pub type SceScreenshotErrorCode = crate::ctypes::c_uint;
pub type SceShaccCgCallbackAbsolutePath = ::core::option::Option<
    unsafe extern "C" fn(
        fileName: *const crate::ctypes::c_char,
        includedFrom: *const SceShaccCgSourceLocation,
        compileOptions: *const SceShaccCgCompileOptions,
    ) -> *const crate::ctypes::c_char,
>;
pub type SceShaccCgCallbackDefaults = crate::ctypes::c_uint;
pub type SceShaccCgCallbackFileDate = ::core::option::Option<
    unsafe extern "C" fn(
        file: *const SceShaccCgSourceFile,
        includedFrom: *const SceShaccCgSourceLocation,
        compileOptions: *const SceShaccCgCompileOptions,
        timeLastStatusChange: *mut i64,
        timeLastModified: *mut i64,
    ) -> SceInt32,
>;
pub type SceShaccCgCallbackLocateFile = ::core::option::Option<
    unsafe extern "C" fn(
        fileName: *const crate::ctypes::c_char,
        includedFrom: *const SceShaccCgSourceLocation,
        searchPathCount: SceUInt32,
        searchPaths: *const *const crate::ctypes::c_char,
        compileOptions: *const SceShaccCgCompileOptions,
        errorString: *mut *const crate::ctypes::c_char,
    ) -> *const crate::ctypes::c_char,
>;
pub type SceShaccCgCallbackOpenFile = ::core::option::Option<
    unsafe extern "C" fn(
        fileName: *const crate::ctypes::c_char,
        includedFrom: *const SceShaccCgSourceLocation,
        compileOptions: *const SceShaccCgCompileOptions,
        errorString: *mut *const crate::ctypes::c_char,
    ) -> *mut SceShaccCgSourceFile,
>;
pub type SceShaccCgCallbackReleaseFile = ::core::option::Option<
    unsafe extern "C" fn(
        file: *const SceShaccCgSourceFile,
        compileOptions: *const SceShaccCgCompileOptions,
    ),
>;
pub type SceShaccCgCallbackReleaseFileName = ::core::option::Option<
    unsafe extern "C" fn(
        fileName: *const crate::ctypes::c_char,
        compileOptions: *const SceShaccCgCompileOptions,
    ),
>;
pub type SceShaccCgDiagnosticLevel = crate::ctypes::c_uint;
pub type SceShaccCgLocale = crate::ctypes::c_uint;
pub type SceShaccCgParameter = *const crate::ctypes::c_void;
pub type SceShaccCgTargetProfile = crate::ctypes::c_uint;
pub type SceShellUtilEventHandler = ::core::option::Option<
    unsafe extern "C" fn(
        result: crate::ctypes::c_int,
        mode: SceShellUtilLockMode,
        type_: SceShellUtilLockType,
        userData: *mut crate::ctypes::c_void,
    ),
>;
pub type SceShellUtilLockMode = crate::ctypes::c_uint;
pub type SceShellUtilLockType = crate::ctypes::c_uint;
pub type SceShort16 = i16;
pub type SceShutterSoundErrorCode = crate::ctypes::c_uint;
pub type SceShutterSoundType = crate::ctypes::c_uint;
pub type SceSize = crate::ctypes::c_uint;
pub type SceSSize = crate::ctypes::c_int;
pub type SceSslCert = crate::ctypes::c_void;
pub type SceSslCertName = crate::ctypes::c_void;
pub type SceSslErrorCode = crate::ctypes::c_uint;
pub type SceSysclibPrntCallback = ::core::option::Option<
    unsafe extern "C" fn(argp: *mut crate::ctypes::c_void, ch: crate::ctypes::c_int),
>;
pub type SceSysconCallback = ::core::option::Option<
    unsafe extern "C" fn(enable: crate::ctypes::c_int, argp: *mut crate::ctypes::c_void),
>;
pub type SceSysconCmd = crate::ctypes::c_uint;
pub type SceSysconCmdExecAsyncCallback = ::core::option::Option<
    unsafe extern "C" fn(
        packet: *mut SceSysconPacket,
        argp: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceSysconControl = crate::ctypes::c_uint;
pub type SceSysconResetType = crate::ctypes::c_uint;
pub type SceSysEventHandler = ::core::option::Option<
    unsafe extern "C" fn(
        resume: crate::ctypes::c_int,
        eventid: crate::ctypes::c_int,
        args: *mut crate::ctypes::c_void,
        opt: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int,
>;
pub type SceSysmoduleErrorCode = crate::ctypes::c_uint;
pub type SceSysmoduleInternalModuleId = crate::ctypes::c_uint;
pub type SceSysmoduleModuleId = crate::ctypes::c_uint;
pub type SceSysroot = SceUIDSysrootObject;
pub type SceSystemParamDateFormat = crate::ctypes::c_uint;
pub type SceSystemParamEnterButtonAssign = crate::ctypes::c_uint;
pub type SceSystemParamId = crate::ctypes::c_uint;
pub type SceSystemParamLang = crate::ctypes::c_uint;
pub type SceSystemParamTimeFormat = crate::ctypes::c_uint;
pub type SceSysTimerCallback = ::core::option::Option<
    unsafe extern "C" fn(timer: SceSysTimerId, pUserData: *mut crate::ctypes::c_void),
>;
pub type SceSysTimerClockSource = crate::ctypes::c_uint;
pub type SceSysTimerId = SceInt32;
pub type SceSysTimerType = crate::ctypes::c_uint;
pub type SceThreadStatus = crate::ctypes::c_uint;
pub type SceTouchErrorCode = crate::ctypes::c_uint;
pub type SceTouchPortType = crate::ctypes::c_uint;
pub type SceTouchReportInfo = crate::ctypes::c_uint;
pub type SceTouchSamplingState = crate::ctypes::c_uint;
pub type SceTriggerUtilDays = crate::ctypes::c_uint;
pub type SceTriggerUtilErrorCode = crate::ctypes::c_uint;
pub type SceUChar8 = u8;
pub type SceUdcdDeviceRequestAttr = crate::ctypes::c_uint;
pub type SceUdcdErrorCode = crate::ctypes::c_uint;
pub type SceUdcdHidDescriptor = crate::ctypes::c_uint;
pub type SceUdcdHidProtocol = crate::ctypes::c_uint;
pub type SceUdcdHidRequest = crate::ctypes::c_uint;
pub type SceUdcdProtocol = crate::ctypes::c_uint;
pub type SceUdcdRetcode = crate::ctypes::c_int;
pub type SceUdcdStatus = crate::ctypes::c_uint;
pub type SceUdcdStatusDriver = crate::ctypes::c_uint;
pub type SceUdcdUsbClass = crate::ctypes::c_uint;
pub type SceUdcdUsbDt = crate::ctypes::c_uint;
pub type SceUdcdUsbReq = crate::ctypes::c_uint;
pub type SceUID = crate::ctypes::c_int;
pub type SceUInt = u32;
pub type SceUInt16 = u16;
pub type SceUInt32 = u32;
pub type SceUInt64 = u64;
pub type SceUInt8 = u8;
pub type SceUIntPtr = crate::ctypes::c_uint;
pub type SceUIntVAddr = SceUIntPtr;
pub type SceULong64 = u64;
pub type SceUpdateMode = crate::ctypes::c_char;
pub type SceUsbAudioInErrorCode = crate::ctypes::c_uint;
pub type SceUsbdDescriptorType = crate::ctypes::c_uint;
pub type SceUsbdErrorCode = crate::ctypes::c_uint;
pub type SceUsbdReqtype = crate::ctypes::c_uint;
pub type SceUsbdRequest = crate::ctypes::c_uint;
pub type SceUsbservErrorCode = crate::ctypes::c_uint;
pub type SceUsbstorVstorType = crate::ctypes::c_uint;
pub type SceUShort16 = u16;
pub type SceVideodecErrorCode = crate::ctypes::c_uint;
pub type SceVideodecType = crate::ctypes::c_uint;
pub type SceVoid = crate::ctypes::c_void;
pub type SceWChar16 = u16;
pub type SceWChar32 = u32;
pub type SulphaNgsModuleQueryType = SceUInt32;
pub type ThreadCpuRegisters = SceThreadCpuRegisters;
pub type time_t = crate::ctypes::c_long;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type uint_fast8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type uint_least8_t = u8;
pub type uintmax_t = crate::ctypes::c_ulonglong;
pub type va_list = u32;
pub type wchar_t = crate::ctypes::c_uint;
