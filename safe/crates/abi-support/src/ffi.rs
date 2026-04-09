#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{
    c_char,
    c_double,
    c_float,
    c_int,
    c_long,
    c_short,
    c_uchar,
    c_uint,
    c_ulong,
    c_ushort,
    c_void,
};

pub type gboolean = c_int;
pub type gint = c_int;
pub type guint = c_uint;
pub type gint8 = i8;
pub type guint8 = u8;
pub type gint16 = i16;
pub type guint16 = u16;
pub type gint32 = i32;
pub type guint32 = u32;
pub type gint64 = i64;
pub type guint64 = u64;
pub type glong = c_long;
pub type gulong = c_ulong;
pub type gshort = c_short;
pub type gushort = c_ushort;
pub type gchar = c_char;
pub type guchar = c_uchar;
pub type gfloat = c_float;
pub type gdouble = c_double;
pub type gpointer = *mut c_void;
pub type gconstpointer = *const c_void;
pub type gsize = usize;
pub type gssize = isize;
pub type guintptr = usize;
pub type GQuark = guint32;
pub type GType = gsize;
pub type GenericFn = Option<unsafe extern "C" fn()>;
pub type GCallback = GenericFn;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer)>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glongdouble {
    pub bytes: [u8; 16],
}
