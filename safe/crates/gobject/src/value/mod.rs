use crate::ffi::*;

pub type GParamFlags = gint;

#[repr(C)]
#[derive(Copy, Clone)]
pub union GValueData {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GValue {
    pub g_type: GType,
    pub data: [GValueData; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union GTypeCValue {
    pub v_int: gint,
    pub v_long: glong,
    pub v_int64: gint64,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}

pub type GValueTransform = Option<unsafe extern "C" fn(*const GValue, *mut GValue)>;
