use crate::ffi::*;

pub type GParamFlags = guint;

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

pub use crate::translated::original::gobject::gvalue::{GTypeCValue, GValueTransform};
