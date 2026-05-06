use crate::ffi::*;
use crate::value::GValue;

pub type GClosureMarshal =
    Option<unsafe extern "C" fn(*mut GClosure, *mut GValue, guint, *const GValue, gpointer, gpointer)>;
pub type GSignalFlags = guint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GSignalInvocationHint {
    pub signal_id: guint,
    pub detail: GQuark,
    pub run_type: GSignalFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GSignalQuery {
    pub signal_id: guint,
    pub signal_name: *const gchar,
    pub itype: GType,
    pub signal_flags: GSignalFlags,
    pub return_type: GType,
    pub n_params: guint,
    pub param_types: *const GType,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GClosureNotifyData {
    pub data: gpointer,
    pub notify: GDestroyNotify,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GClosure {
    pub flags: guint,
    pub reserved: guint,
    pub marshal: GClosureMarshal,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GCClosure {
    pub closure: GClosure,
    pub callback: gpointer,
}
