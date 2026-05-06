use crate::ffi::*;
use crate::value::GValue;

pub type GClosureMarshal =
    Option<unsafe extern "C" fn(*mut GClosure, *mut GValue, guint, *const GValue, gpointer, gpointer)>;
pub type GClosureNotify = Option<unsafe extern "C" fn(gpointer, *mut GClosure)>;
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
    pub notify: GClosureNotify,
}

#[repr(C)]
#[derive(Copy, Clone, BitfieldStruct)]
pub struct GClosure {
    #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
    #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
    #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
    #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
    #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
    pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
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
