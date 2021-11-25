/* Note that we use u8 and u16 explicitly, as the fake65c02
 * codebase uses uint8_t and uint16_t, not char and short.
 */
use std::ffi::c_void;
use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct fake65c02_t {
    m: *mut c_void,

    pub read: unsafe extern "C" fn(context: *mut fake65c02_t, address: u16) -> u8,
    pub write: unsafe extern "C" fn(context: *mut fake65c02_t, address: u16, value: u8),
    pub hook: unsafe extern "C" fn(context: *mut fake65c02_t),

    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,

    pub instructions: u32,
    pub clockticks: u32,

    ea: u16,

    status: u8,
    opcode: u8,
    oldstatus: u8,

    penaltyop: u8,
    penaltyaddr: u8,

    reladdr: u16,
    result: u16,
    value: u16,
    oldpc: u16,

    clockgoal: u32,
}

#[link(name="fake65c02")]
extern "C" {
    pub fn new_fake65c02(m: *mut ::std::ffi::c_void) -> *mut fake65c02_t;
    pub fn free_fake65c02(context: *mut fake65c02_t);
    pub fn reset65c02(context: *mut fake65c02_t) -> c_int;
    pub fn step65c02(context: *mut fake65c02_t) -> c_int;
    pub fn irq65c02(context: *mut fake65c02_t) -> c_int;
    pub fn nmi65c02(context: *mut fake65c02_t) -> c_int;
    pub fn exec65c02(context: *mut fake65c02_t, tickcount: u32) -> c_int;
}
