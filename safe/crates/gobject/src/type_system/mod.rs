use crate::ffi::*;
use crate::value::GValue;

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
