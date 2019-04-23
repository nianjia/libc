pub use ffi::c_void;
use core::unimplemented;

pub type c_char = i8;
pub type c_uchar = u8;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = i64;
pub type pid_t = i32;
pub type int8_t = i8;
pub type uint8_t = u8;
pub type int16_t = i16;
pub type uint16_t = u16;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type int64_t = i64;
pub type uint64_t = u64;
pub type clock_t = c_longlong;
pub type time_t = c_longlong;
pub type c_double = f64;
pub type c_float = f32;

extern {
	pub fn strlen(cs: *const c_char) -> size_t;
}
