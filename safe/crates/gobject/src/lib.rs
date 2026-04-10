#![allow(non_camel_case_types)]

#[path = "../../abi-support/src/ffi.rs"]
pub mod ffi;

mod runtime;
pub mod exports;

unsafe fn initialize_exports(handle: *mut core::ffi::c_void) {
    exports::initialize(handle);
    runtime::initialize_live_exports(handle);
}

extern "C" fn initialize_library() {
    unsafe {
        runtime::initialize(initialize_exports);
    }
}

#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
static INIT_ARRAY: extern "C" fn() = initialize_library;

pub mod abi {
    use super::ffi::*;

    pub type GBaseInitFunc = Option<unsafe extern "C" fn(gpointer)>;
    pub type GBaseFinalizeFunc = Option<unsafe extern "C" fn(gpointer)>;
    pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gconstpointer)>;
    pub type GClassFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gconstpointer)>;
    pub type GInstanceInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer)>;
    pub type GTypeValueInitFunc = Option<unsafe extern "C" fn(*mut GValue)>;
    pub type GTypeValueFreeFunc = Option<unsafe extern "C" fn(*mut GValue)>;
    pub type GTypeValueCopyFunc = Option<unsafe extern "C" fn(*const GValue, *mut GValue)>;
    pub type GTypeValuePeekPointerFunc = Option<unsafe extern "C" fn(*const GValue) -> gpointer>;
    pub type GTypeValueCollectFunc =
        Option<unsafe extern "C" fn(*mut GValue, guint, *mut gpointer, guint) -> *mut gchar>;
    pub type GTypeValueLCopyFunc =
        Option<unsafe extern "C" fn(*const GValue, guint, *mut gpointer, guint) -> *mut gchar>;
    pub type GInterfaceInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer)>;
    pub type GInterfaceFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gpointer)>;
    pub type GClosureMarshal =
        Option<unsafe extern "C" fn(*mut GClosure, *mut GValue, guint, *const GValue, gpointer, gpointer)>;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GTypeClass {
        pub g_type: GType,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GTypeInstance {
        pub g_class: *mut GTypeClass,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GTypeInterface {
        pub g_type: GType,
        pub g_instance_type: GType,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GTypeQuery {
        pub type_: GType,
        pub type_name: *const gchar,
        pub class_size: guint,
        pub instance_size: guint,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GTypeInfo {
        pub class_size: guint16,
        pub base_init: GBaseInitFunc,
        pub base_finalize: GBaseFinalizeFunc,
        pub class_init: GClassInitFunc,
        pub class_finalize: GClassFinalizeFunc,
        pub class_data: gconstpointer,
        pub instance_size: guint16,
        pub n_preallocs: guint16,
        pub instance_init: GInstanceInitFunc,
        pub value_table: *const GTypeValueTable,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GInterfaceInfo {
        pub interface_init: GInterfaceInitFunc,
        pub interface_finalize: GInterfaceFinalizeFunc,
        pub interface_data: gpointer,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GTypeValueTable {
        pub value_init: GTypeValueInitFunc,
        pub value_free: GTypeValueFreeFunc,
        pub value_copy: GTypeValueCopyFunc,
        pub value_peek_pointer: GTypeValuePeekPointerFunc,
        pub collect_format: *const gchar,
        pub collect_value: GTypeValueCollectFunc,
        pub lcopy_format: *const gchar,
        pub lcopy_value: GTypeValueLCopyFunc,
    }

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

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GObjectConstructParam {
        pub pspec: gpointer,
        pub value: *mut GValue,
    }
}

pub const CRATE_ID: &str = "safe-gobject";

pub fn bootstrap_marker() -> &'static str {
    "impl-safe-bootstrap"
}
