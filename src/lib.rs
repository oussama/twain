#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;

extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};
use std::{
    ffi::{c_void, CString},
    fmt::Debug,
    io::Write,
    marker::PhantomData,
    os::raw::{c_char, c_int},
    ptr::{null, null_mut},
    sync::{
        mpsc::{channel, Receiver, RecvError, Sender},
        Mutex,
    },
    usize,
};

mod data;
mod test;

use data::*;

// https://github.com/twain/twain-dsm/blob/9e7bb25d522a8cb2715bbf3b0904ff07a86ea2ee/TWAIN_DSM/src/twain.h

type Handle = *const c_void;

/*

#ifndef TWAIN
#define TWAIN

/****************************************************************************
 * TWAIN Version                                                            *
 ****************************************************************************/
    PROTOCOLMINOR   4        /* Changed for Version 2.4            */
    PROTOCOLMAJOR   2

/****************************************************************************
 * Platform Dependent Definitions and Typedefs                              *
 ****************************************************************************/

/* Microsoft C/C++ Compiler */
#if defined(WIN32) || defined(WIN64) || defined (_WINDOWS)
    #define TWH_CMP_MSC
    #if  defined(_WIN64) || defined(WIN64)
      #define TWH_64BIT
    #elif defined(WIN32) || defined(_WIN32)
      #define TWH_32BIT
    #endif

/* GNU C/C++ Compiler */
#elif defined(__GNUC__)
    #define TWH_CMP_GNU
    #if defined(__alpha__)\
        ||defined(__ia64__)\
        ||defined(__ppc64__)\
        ||defined(__s390x__)\
        ||defined(__x86_64__)
      #define TWH_64BIT
    #else
      #define TWH_32BIT
    #endif


/* Borland C/C++ Compiler */
#elif defined(__BORLAND__)
    #define TWH_CMP_BORLAND
    #define TWH_32BIT
/* Unrecognized */
#else
    #error Unrecognized compiler
#endif

/* Apple Compiler (which is GNU now) */
#if defined(__APPLE__)
  #define TWH_CMP_XCODE
  #ifdef __MWERKS__
    #include <Carbon.h>
  #else
    #include <Carbon/Carbon.h>
  #endif
#endif

/* Win32 and Win64 systems */
#if defined(TWH_CMP_MSC) | defined(TWH_CMP_BORLAND)
    typedef HANDLE  TW_HANDLE;
    typedef LPVOID  TW_MEMREF;
    typedef UINT_PTR TW_UINTPTR;

/* MacOS/X... */
#elif defined(TWH_CMP_XCODE)
    #define PASCAL   pascal
    #define FAR
    typedef Handle   TW_HANDLE;
    typedef char    *TW_MEMREF;
    typedef unsigned char BYTE;

    #ifdef TWH_32BIT
      //32 bit GNU
      typedef unsigned long      TW_UINTPTR;
    #else
      //64 bit GNU
      typedef unsigned long long TW_UINTPTR;
    #endif

/* Everything else... */
#else
    #define PASCAL
    #define FAR
    typedef void* TW_HANDLE;
    typedef void* TW_MEMREF;
    typedef unsigned char BYTE;

    #ifdef TWH_32BIT
      //32 bit GNU
      typedef unsigned long      TW_UINTPTR;
    #else
      //64 bit GNU
      typedef unsigned long long TW_UINTPTR;
    #endif
#endif


/* Set the packing: this occurs before any structures are defined */
#ifdef TWH_CMP_MSC
    #pragma pack (push, before_twain)
    #pragma pack (2)
#elif defined(TWH_CMP_GNU)
    #if defined(__APPLE__) /* cf: Mac version of TWAIN.h */
        #pragma options align = power
    #else
        #pragma pack (push, before_twain)
        #pragma pack (2)
    #endif
#elif defined(TWH_CMP_BORLAND)
    #pragma option -a2
#endif


/****************************************************************************
 * Type Definitions                                                         *
 ****************************************************************************/

/* String types. These include room for the strings and a NULL char,     *
 * or, on the Mac, a length byte followed by the string.                 *
 * TW_STR255 must hold less than 256 chars so length fits in first byte. */
#if defined(__APPLE__)/* cf: Mac version of TWAIN.h */
    typedef unsigned char TW_STR32[34],     FAR *pTW_STR32;
    typedef unsigned char TW_STR64[66],     FAR *pTW_STR64;
    typedef unsigned char TW_STR128[130],   FAR *pTW_STR128;
    typedef unsigned char TW_STR255[256],   FAR *pTW_STR255;
#else
    typedef char          TW_STR32[34],     FAR *pTW_STR32;
    typedef char          TW_STR64[66],     FAR *pTW_STR64;
    typedef char          TW_STR128[130],   FAR *pTW_STR128;
    typedef char          TW_STR255[256],   FAR *pTW_STR255;
#endif

/* Numeric types. */
typedef char              TW_INT8,          FAR *pTW_INT8;
typedef short             i16,         FAR *pi16;
#if defined(_WIN32)
    typedef long          i32,         FAR *pi32;
#else
    typedef int           i32,         FAR *pi32;
#endif
typedef unsigned char     u8,         FAR *pu8;
typedef unsigned short    u16,        FAR *pu16;
#if defined(_WIN32)
    typedef unsigned long u32,        FAR *pu32;
#else
    typedef unsigned int  u32,        FAR *pu32;
#endif
typedef unsigned short    TW_BOOL,          FAR *pTW_BOOL;

*/
/****************************************************************************
 * Structure Definitions                                                    *
 ****************************************************************************/

/* Fixed point structure type. */
#[derive(Debug, Default)]
#[repr(C, packed(2))]
pub struct FixedPoint32 {
    whole: i16,
    frac: u16,
}

/* Defines a frame rectangle in ICAP_UNITS coordinates. */
pub struct Frame {
    left: FixedPoint32,
    top: FixedPoint32,
    right: FixedPoint32,
    bottom: FixedPoint32,
}

/* Defines the parameters used for channel-specific transformation. */
pub struct DecodeFunction {
    start_in: FixedPoint32,
    break_in: FixedPoint32,
    end_in: FixedPoint32,
    start_out: FixedPoint32,
    break_out: FixedPoint32,
    end_out: FixedPoint32,
    gamma: FixedPoint32,
    sample_count: FixedPoint32,
}

/* Stores a Fixed point number in two parts, a whole and a fractional part. */
pub struct TransformStage {
    decode: [DecodeFunction; 3],
    mix: [[FixedPoint32; 3]; 3],
}

/* Container for array of values */
pub struct Array {
    item_type: u16,
    num_items: u32,
    item_list: [u8; 1],
}

/* Information about audio data */
pub struct AudioInfo {
    name: Str255,
    reserved: u32,
}

/* Used to register callbacks. */
pub struct CallBack {
    proc: *mut c_void,
    ref_con: u32,
    message: i16,
    /*
    TW_MEMREF      CallBackProc;
    #if defined(__APPLE__) /* cf: Mac version of TWAIN.h */
        TW_MEMREF  RefCon;
    #else
        u32  RefCon;
    #endif*/
}

impl CallBack {
    pub fn new() -> CallBack {
        CallBack {
            proc: null_mut(),
            ref_con: 0,
            message: 0,
        }
    }
}

/* Used to register callbacks. */
pub struct CallBack2 {
    proc: *mut c_void,
    ref_con: *mut usize,
    message: i16,
}
/* Used by application to get/set capability from/in a data source. */
#[derive(Debug)]
pub struct Capability {
    cap: u16,
    con_type: TWON,
    h_container: *mut c_void,
}

impl Capability {
    fn default() -> Capability {
        Capability {
            cap: 0,
            con_type: TWON::ONEVALUE,
            h_container: null_mut(),
        }
    }
}

/* Defines a CIE XYZ space tri-stimulus value. */
pub struct CiePoint {
    x: FixedPoint32,
    y: FixedPoint32,
    z: FixedPoint32,
}

/* Defines the mapping from an RGB color space device into CIE 1931 (XYZ) color space. */
pub struct CieColor {
    color_space: u16,
    low_endian: i16,
    device_dependent: i16,
    version_number: i32,
    stage_abc: TransformStage,
    stage_lmn: TransformStage,
    white_point: CiePoint,
    black_point: CiePoint,
    white_paper: CiePoint,
    black_ink: CiePoint,
    samples: [FixedPoint32; 1],
}

/* Allows for a data source and application to pass custom data to each other. */
pub struct CustomDSData {
    info_length: u32,
    h_data: *mut c_void,
}

/* Provides information about the Event that was raised by the Source */
pub struct DeviceEvent {
    event: u32,
    device_name: Str255,
    battery_minutes: u32,
    battery_percentage: i16,
    power_supply: i32,
    x_resolution: FixedPoint32,
    y_resolution: FixedPoint32,
    flash_used2: u32,
    automatic_capture: u32,
    time_before_first_capture: u32,
    time_between_captures: u32,
}

/* This structure holds the tri-stimulus color palette information for TW_PALETTE8 structures.*/
pub struct Element8 {
    index: u8,
    channel1: u8,
    channel2: u8,
    channel3: u8,
}

/* Stores a group of individual values describing a capability. */
pub struct Enumeration {
    item_type: u16,
    num_items: u32,
    current_index: u32,
    default_index: u32,
    item_list: [u8; 1],
}

/* Used to pass application events/messages from the application to the Source. */
pub struct Event {
    p_event: *const c_void,
    message: u16,
}

/* This structure is used to pass specific information between the data source and the application. */
pub struct Info {
    id: u16,
    item_type: u16,
    num_items: u16,
    return_code: u16,
    /** Deprecated, do not use */
    cond_code: u16,
    item: *mut usize,
    /*
    union {
        u16   ReturnCode;
        u16   CondCode;
    };*/
}

pub struct ExtImageInfo {
    num_infos: u32,
    info: [Info; 1],
}

/* Provides information about the currently selected device */
pub struct FileSystem {
    input_name: Str255,
    output_name: Str255,
    context: *mut c_void,
    recursive: i32,
    sub_directories: u16,
    file_type: i32,
    file_system_type: u32,
    size: u32,
    create_time_date: Str32,
    modified_time_date: Str32,
    free_space: u32,
    new_image_size: i32,
    number_of_files: u32,
    number_of_snippets: u32,
    device_group_mask: u32,
    reserved: [i8; 508],
}

/* This structure is used by the application to specify a set of mapping values to be applied to grayscale data. */
pub struct GrayResponse {
    response: [Element8; 1],
}

/* A general way to describe the version of software that is running. */
#[repr(C, packed(2))]
#[derive(Debug, Clone)]
pub struct Version {
    major_num: u16,
    minor_num: u16,
    language: u16, // TWLG,
    country: u16,  // TWCY,
    info: Str32,
}

impl Default for Version {
    fn default() -> Self {
        Version {
            major_num: 1,
            minor_num: 0,
            language: TWLG::USA_OR_ENGLISH_USA as _,
            country: TWCY::USA as _,
            info: Str32::default(),
        }
    }
}

/* Provides identification information about a TWAIN entity.*/
#[repr(C, packed(2))]
#[derive(Debug, Default, Clone)]
pub struct Identity {
    id: u32,
    version: Version,
    protocol_major: u16,
    protocol_minor: u16,
    supported_groups: u32,
    manufacturer: Str32,
    product_family: Str32,
    product_name: Str32,
}

/* Describes the "real" image data, that is, the complete image being transferred between the Source and application. */
#[derive(Debug, Default)]
#[repr(C, packed(2))]
pub struct ImageInfo {
    x_resolution: FixedPoint32,
    y_resolution: FixedPoint32,
    image_width: i32,
    image_length: i32,
    samples_per_pixel: i16,
    bits_per_sample: [i16; 8],
    bits_per_pixed: i16,
    planar: u16,
    pixel_type: i16,
    compression: u16,
}

/* Involves information about the original size of the acquired image. */
pub struct ImageLayout {
    frame: Frame,
    document_number: u32,
    page_number: u32,
    frame_number: u32,
}

/* Provides information for managing memory buffers. */
pub struct Memory {
    flags: u32,
    length: u32,
    the_mem: *const c_void,
}

/* Describes the form of the acquired data being passed from the Source to the application.*/
pub struct ImageMemxfer {
    compression: i16,
    bytes_per_row: u32,
    columns: u32,
    rows: u32,
    xoffset: u32,
    yoffset: u32,
    bytes_written: u32,
    memory: Memory,
}

/* Describes the information necessary to transfer a JPEG-compressed image. */
pub struct JpegCompression {
    color_space: u16,
    sub_sampling: u32,
    num_components: u16,
    restart_frequency: u16,
    quant_map: [u16; 4],
    quant_table: [Memory; 4],
    huffman_map: [u16; 4],
    huffman_dc: [Memory; 2],
    huffman_ac: [Memory; 2],
}

/* Collects scanning metrics after returning to state 4 */
pub struct Metrics {
    size_of: u32,
    image_count: u32,
    sheet_count: u32,
}

/* Stores a single value (item) which describes a capability. */
pub struct OneValue {
    item_type: u16,
    item: u32,
}

/* This structure holds the color palette information. */
pub struct Palette8 {
    num_colors: u16,
    palette_type: u16,
    colors: [Element8; 256],
}

/* Used to bypass the TWAIN protocol when communicating with a device */
pub struct PassThru {
    p_command: *const c_void,
    command_bytes: u32,
    direction: i32,
    p_data: *const c_void,
    data_bytes: u32,
    data_bytes_xfered: u32,
}

/* This structure tells the application how many more complete transfers the Source currently has available. */
pub struct PendingTransfers {
    count: u16,
    eoj: u32,
    reserved: u32,
    /*
    union {
        u32 EOJ;
        u32 Reserved;
        #if defined(__APPLE__) /* cf: Mac version of TWAIN.h */
            union {
                u32 EOJ;
                u32 Reserved;
            } TW_JOBCONTROL;
        #endif
    };*/
}

/* Stores a range of individual values describing a capability. */
pub struct Range {
    item_type: u16,
    min_value: u32,
    max_value: u32,
    step_size: u32,
    default_value: u32,
    current_value: u32,
}

/* This structure is used by the application to specify a set of mapping values to be applied to RGB color data. */
pub struct RgbResponse {
    response: [Element8; 1],
}

/* Describes the file format and file specification information for a transfer through a disk file. */
pub struct SetupFileTransfer {
    filename: Str255,
    format: u16,
    vref_num: i16,
}

/* Provides the application information about the Source's requirements and preferences regarding allocation of transfer buffer(s). */
pub struct SetupMemoryTransfer {
    min_buf_size: u32,
    max_buf_size: u32,
    preferred: u32,
}

/* Describes the status of a source. */
pub struct Status {
    condition_code: u16,
    data: u16,
    reserved: u16,
    /*
    union {
      u16  Data;
      u16  Reserved;
    };*/
}

/* Translates the contents of Status into a localized UTF8string. */
pub struct StatusUtf8 {
    status: Status,
    size: u32,
    utf8_string: *const c_void,
}

pub struct TwainDirect {
    size_of: u32,
    communication_manager: u16,
    send: *const c_void,
    send_size: u32,
    receive: *const c_void,
    receive_size: u32,
}

/* This structure is used to handle the user interface coordination between an application and a Source. */
pub struct UserInterface {
    show_ui: u16,  // bool
    modal_ui: u16, // bool
    /// Windows - The application should place a handle to the Window that is acting as the Source’s parent.
    /// Macintosh/Linux - The application sets this field to NULL.
    h_parent: Handle,
}

impl UserInterface {
    pub fn new(show_ui: bool, model_ui: bool) -> UserInterface {
        UserInterface {
            show_ui: show_ui as _,
            modal_ui: model_ui as _,
            /// for windows handle to parent window
            h_parent: null(),
        }
    }
}

/* DAT_SETUPFILEXFER2. Sets up DS to application data transfer via a file. Added 1.9 */
pub struct SetupFileTransfer2 {
    file_name: *const c_void,
    file_name_type: u16,
    format: u16,
    v_ref_num: i16,
    part_id: u32,
}

/* DAT_TWUNKIDENTITY. Provides DS identity and 'other' information necessary */
/*                    across thunk link. */
pub struct TwunkIdentity {
    identity: Identity,
    ds_path: Str255,
}

/* Provides DS_Entry parameters over thunk link. */
pub struct TwunkDsEntryParams {
    dest_flag: i8,
    dest: Identity,
    data_group: i32,
    data_arg_type: i16,
    message: i16,
    p_data_size: i32,
    //  TW_MEMREF   pData;
}

/* Provides DS_Entry results over thunk link. */
pub struct TwunkDsEntryReturn {
    return_code: u16,
    condition_code: u16,
    p_data_size: i32,
    //  TW_MEMREF   pData;
}

pub struct CapExt {
    cap: u16,
    properties: u16,
}

/* DAT_SETUPAUDIOFILEXFER, information required to setup an audio file transfer */
pub struct SetupAudioFileTransfer {
    file_name: Str255,
    format: TWAF, // u16,
    v_ref_num: i16,
}

/* DAT_ENTRYPOINT. returns essential entry points. */
#[derive(Debug)]
#[repr(C, packed(2))]
pub struct EntryPoint {
    size: u32,
    // pub ds_entry:
    //    fn(p_origin: *const Identity, dg: DG, dat: DAT, msg: MSG, p_data: *const c_void) -> u16,
    pub ds_entry: *const fn(
        p_origin: *const Identity,
        p_dest: *const Identity,
        dg: DG,
        dat: DAT,
        msg: MSG,
        p_data: *const c_void,
    ) -> u16,
    pub mem_allocate: *mut fn(u32) -> Handle,
    pub mem_free: *const fn(Handle) -> (),
    pub mem_lock: *const fn(Handle) -> *const c_void,
    pub mem_unlock: *const fn(Handle) -> (),
}

impl Default for EntryPoint {
    fn default() -> Self {
        EntryPoint {
            size: 44,
            /*ds_entry: |p_origin: *const Identity,
                      dg: DG,
                      dat: DAT,
                      msg: MSG,
                      p_data: *const c_void|
            -> u16 { 0 },*/
            ds_entry: null(),
            mem_allocate: null_mut(),
            mem_free: null(),
            mem_lock: null(),
            mem_unlock: null(),
        }
    }
}

impl EntryPoint {
    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn allocate(&self, size: u32) -> Handle {
        unsafe { (*self.mem_allocate)(size) }
    }
}

/* DAT_FILTER*/
#[repr(C)]
pub struct FilterDescriptor {
    size: u32,
    hue_start: u32,
    hue_end: u32,
    saturation_start: u32,
    saturation_end: u32,
    value_start: u32,
    value_end: u32,
    replacement: u32,
}

/* DAT_FILTER */
#[repr(C)]
pub struct Filter {
    size: u32,
    descriptor_count: u32,
    max_descriptor_count: u32,
    condition: u32,
    h_descriptors: Handle,
}

#[derive(WrapperApi)]
struct Api {
    #[dlopen_name = "DSM_Entry"]
    dsm_entry: unsafe extern "C" fn(
        p_origin: *const Identity,
        p_dest: *const Identity,
        dg: DG,
        dat: DAT,
        msg: MSG,
        entry_point: *const usize,
    ) -> TWRC,
}

pub enum State {
    None = 1,     // Nothing loaded or open
    LOADED = 2,   // DSM loaded
    OPEN = 3,     // DSM open
    DSOPEN = 4,   // Data Source open, programmatic mode (no GUI)
    WAITING = 5,  // GUI up or waiting to transfer first image
    READY = 6,    // ready to start transferring image
    TRANSFER = 7, // transferring image or transfer done
}

#[derive(Clone)]
#[repr(C, packed(2))]
pub struct Str32([u8; 34]);

impl Default for Str32 {
    fn default() -> Self {
        Str32([0; 34])
    }
}

impl std::fmt::Debug for Str32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Str32 {
    fn new(val: &str) -> Str32 {
        let mut a = Self::default();
        let t = CString::new(val).unwrap().to_bytes();

        let chars = val.chars().into_iter().collect::<Vec<char>>();
        let mut i = 0;
        for char in chars {
            a.0[i] = char as _;
            i += 1;
        }
        a
    }

    fn to_string(&self) -> String {
        let s: CString = unsafe { CString::from_vec_unchecked(self.0.to_vec()) };

        "".to_owned()
    }
}

pub struct Str255([char; 256]);

impl std::fmt::Debug for Str255 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Default for Str255 {
    fn default() -> Self {
        Str255([' '; 256])
    }
}

impl Str255 {
    fn new(val: String) -> Str255 {
        let mut a = Self::default();
        a.0.copy_from_slice(&val.chars().into_iter().collect::<Vec<char>>());
        a
    }

    fn to_string(&self) -> String {
        let chars: Vec<char> = self.0.to_vec();
        chars.into_iter().collect()
    }
}

impl TWRC {
    fn ok(self) -> Result<(), TWRC> {
        match self {
            TWRC::SUCCESS => Ok(()),
            code => Err(code),
        }
    }
}
/*
#[no_mangle]
pub extern "C" fn DS_Entry(
    origin: *const Identity,
    dg: u32,
    dat: u16,
    msg: u16,
    data: *const c_void,
) -> u16 {
    println!("got a message");
    TWRC::SUCCESS as _
}
*/

#[derive(Debug)]
#[repr(C)]
pub struct BitmapInfoHeader {
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bit_count: u16,
    pub vompression: u32,
    pub size_image: u32,
    pub x_pels_per_meter: i32,
    pub y_pels_per_meter: i32,
    pub clr_used: u32,
    pub clr_important: u32,
}

#[derive(Debug, Default)]
#[repr(C, packed(2))]
pub struct BitmapFileHeader {
    /** Specifies the file type, must be BM. */
    typ: u16,
    /** Specifies the size, in bytes, of the bitmap file. */
    size: u32,
    /** Reserved; must be zero. */
    reserved1: u16,
    /** Reserved; must be zero. */
    reserved2: u16,
    /** Specifies the offset, in bytes, from the beginning of                                          the BITMAPFILEHEADER structure to the bitmap bits. */
    off_bits: u32,
}

pub struct Client {
    api: Container<Api>,
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

use dlopen::Error as DlOpenError;

impl Client {
    pub fn new() -> Result<Client, DlOpenError> {
        let lib_path = "./lib/windows/TWAINDSM.dll";
        let cont: Container<Api> = unsafe { Container::load(lib_path) }?;

        let p = std::env::current_exe().unwrap();
        let filename = p.file_name().unwrap().to_str().unwrap();
        let path = p.to_str().unwrap().replace(filename, "").to_string() + "platforms";
        println!("{:?}", path);

        let bytes = include_bytes!("../lib/windows/qwindows.dll");

        // std::fs::create_dir(path.clone()).unwrap();
        // std::fs::write(path + "/qwindows.dll", bytes).unwrap();
        Ok(Client { api: cont })
    }
    pub fn open_dsm(self, identity: Identity) -> Result<DSM, TWRC> {
        unsafe {
            self.api.dsm_entry(
                &identity,
                null(),
                DG::CONTROL,
                DAT::PARENT,
                MSG::OPENDSM,
                null_mut(),
            )
        }
        .ok()
        .map(|_| DSM {
            api: self.api,
            identity,
        })
    }
}

pub struct DSM {
    api: Container<Api>,
    pub identity: Identity,
}

impl DSM {
    pub fn open_ds(self, identity: Identity) -> Result<OpenDS, TWRC> {
        let res = unsafe {
            self.api.dsm_entry(
                &self.identity,
                null(),
                DG::CONTROL,
                DAT::IDENTITY,
                MSG::OPENDS,
                &identity as *const Identity as *const usize,
            )
        };
        let receiver = self.register_callback(&identity).unwrap();

        res.ok().map(|_| OpenDS {
            api: self.api,
            identity: self.identity,
            selected: identity,
        })
    }

    fn register_callback(&self, selected: &Identity) -> Result<(), TWRC> {
        let mut callback = CallBack::new();
        callback.proc = DS_Entry as *mut c_void;
        let res = unsafe {
            self.api.dsm_entry(
                &self.identity,
                selected,
                DG::CONTROL,
                DAT::CALLBACK,
                MSG::REGISTER_CALLBACK,
                &callback as *const CallBack as *const usize,
            )
        };

        res.ok()
    }

    fn get_ds_identity(&self, msg: MSG) -> Result<Identity, TWRC> {
        let mut out_identity = Identity::default();
        out_identity.supported_groups = DF::APP2 as _;

        let res = unsafe {
            self.api.dsm_entry(
                &self.identity,
                null(),
                DG::CONTROL,
                DAT::IDENTITY,
                msg,
                &out_identity as *const Identity as *const usize,
            )
        };
        println!("RES {:?}", res);
        res.ok().map(|_| out_identity)
    }

    pub fn get_default_ds_identity(&self) -> Result<Identity, TWRC> {
        self.get_ds_identity(MSG::GETDEFAULT)
    }

    pub fn get_first_ds_identity(&self) -> Result<Identity, TWRC> {
        self.get_ds_identity(MSG::GETFIRST)
    }

    pub fn get_next_ds_identity(&self) -> Result<Identity, TWRC> {
        self.get_ds_identity(MSG::GETNEXT)
    }

    pub fn open_select_ds(&self) -> Result<Identity, TWRC> {
        self.get_ds_identity(MSG::USERSELECT)
    }

    pub fn get_entrypoint(&self) -> EntryPoint {
        let mut entry_point = EntryPoint::default();
        entry_point.size = 44;
        let res = unsafe {
            self.api.dsm_entry(
                &self.identity,
                null(),
                DG::CONTROL,
                DAT::ENTRYPOINT,
                MSG::GET,
                &mut entry_point as *const EntryPoint as *const usize,
            )
        };
        entry_point
    }
}

pub struct OpenDS {
    api: Container<Api>,
    pub identity: Identity,
    pub selected: Identity,
}

impl OpenDS {
    pub fn recv(&self) -> Result<DSEvent, RecvError> {
        ds_events_channel.lock().unwrap().1.recv()
    }

    pub fn capability(&self, msg: MSG) -> Option<Capability> {
        let mut out = Capability::default();
        let res = unsafe {
            self.api.dsm_entry(
                &self.identity,
                &self.selected,
                DG::CONTROL,
                DAT::CAPABILITY,
                msg,
                &mut out as *const Capability as *const usize,
            )
        };
        Some(out)
    }

    pub fn user_interface_enable_ds(self, config: &UserInterface) -> Result<EnabledDS, TWRC> {
        let res = unsafe {
            self.api.dsm_entry(
                &self.identity,
                &self.selected,
                DG::CONTROL,
                DAT::USERINTERFACE,
                MSG::ENABLEDS,
                config as *const UserInterface as *const usize,
            )
        };
        res.ok().map(|_| EnabledDS {
            api: self.api,
            identity: self.identity,
            selected: self.selected,
        })
    }
}

pub struct EnabledDS {
    api: Container<Api>,
    pub identity: Identity,
    pub selected: Identity,
}

impl EnabledDS {
    pub fn ready(self, event: ReadyDSEvent) -> ReadyDS {
        ReadyDS {
            api: self.api,
            identity: self.identity,
            selected: self.selected,
        }
    }

    pub fn recv(&self) -> Result<DSEvent, RecvError> {
        ds_events_channel.lock().unwrap().1.recv()
    }
}

#[derive(Debug, Clone)]
pub struct ReadyDSEvent {
    phantom: PhantomData<()>,
}

pub struct ReadyDS {
    api: Container<Api>,
    pub identity: Identity,
    pub selected: Identity,
}

impl ReadyDS {
    pub fn image_info(&self) -> Result<ImageInfo, TWRC> {
        let mut info = ImageInfo::default();
        let res = unsafe {
            self.api.dsm_entry(
                &self.identity,
                &self.selected,
                DG::IMAGE,
                DAT::IMAGEINFO,
                MSG::GET,
                &info as *const ImageInfo as *const _,
            )
        };
        println!("RES {:?}", res);
        res.ok().map(|_| info)
    }

    pub fn image_native_transfer(&self) -> Result<(), TWRC> {
        let handle = 0;
        let res = unsafe {
            self.api.dsm_entry(
                &self.identity,
                &self.selected,
                DG::IMAGE,
                DAT::IMAGENATIVEXFER,
                MSG::GET,
                &handle,
            )
        };
        let header = unsafe { &*(handle as *const BitmapInfoHeader) };
        println!("RES {:?} {:?}", res, header);

        let dwPaletteSize = match header.bit_count {
            1 => 2,
            8 => 256,
            24 => 0,
            _ => panic!("invalid bit count"),
        };

        println!("size of info {}", size_of::<BitmapInfoHeader>());
        let rgbquad_size = 4;

        use std::mem::size_of;
        let nImageSize = header.size_image as usize
            + rgbquad_size * dwPaletteSize
            + size_of::<BitmapInfoHeader>();
        println!("nImageSize {}", nImageSize);

        println!("size of info {}", size_of::<BitmapInfoHeader>());
        let mut bmpFIH = BitmapFileHeader::default();

        bmpFIH.typ = 0x4D42; // ((('M' as i32) << 8 as i32) as u16 as i32 | 'B' as i32) as u16;
        bmpFIH.size = (nImageSize + size_of::<BitmapFileHeader>()) as u32;
        bmpFIH.off_bits = (size_of::<BitmapFileHeader>()
            + size_of::<BitmapInfoHeader>()
            + rgbquad_size * dwPaletteSize) as u32;

        use std::fs;

        let image_data = std::ptr::slice_from_raw_parts(handle as *const u8, nImageSize as usize);
        let mut file = fs::File::create("./img1.bmp").unwrap();
        file.write_all(unsafe { any_as_u8_slice(&bmpFIH) }).unwrap();
        file.write_all(unsafe { &*image_data }).unwrap();

        res.ok()
    }

    /**  The triplet the application sends to transfer data from the Source into a file is: */
    pub fn transfer_data_from_src_to_file(&self) {
        let res = unsafe {
            self.api.dsm_entry(
                &self.identity,
                null(),
                DG::CONTROL,
                DAT::IMAGEFILEXFER,
                MSG::GET,
                null(), //&out_identity as *const Identity as *const usize,
            )
        };
    }
}

#[no_mangle]
extern "C" fn DS_Entry(
    p_origin: *const Identity,
    p_dest: *const Identity,
    dg: DG,
    dat: DAT,
    msg: MSG,
    entry_point: *const usize,
) -> TWRC {
    println!("{:?} {:?} {:?}", dg, dat, msg);

    ds_events_channel
        .lock()
        .unwrap()
        .0
        .send(DSEvent::Ready(ReadyDSEvent {
            phantom: PhantomData,
        }))
        .unwrap();
    TWRC::SUCCESS
}

// https://github.com/mrlitong/TWAIN-SDK

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

lazy_static! {
    static ref ds_events_channel: Mutex<(Sender<DSEvent>, Receiver<DSEvent>)> =
        Mutex::new(channel());
}

#[derive(Debug)]
pub enum DSEvent {
    Ready(ReadyDSEvent),
}
