use crate::ffi::*;
use crate::type_system::{GTypeClass, GTypeInstance};
use crate::value::{GParamFlags, GValue};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: gpointer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GParamSpec {
    pub g_type_instance: GTypeInstance,
    pub name: *const gchar,
    pub flags: GParamFlags,
    pub value_type: GType,
    pub owner_type: GType,
    pub _nick: *mut gchar,
    pub _blurb: *mut gchar,
    pub qdata: gpointer,
    pub ref_count: guint,
    pub param_id: guint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GObjectConstructParam {
    pub pspec: *mut GParamSpec,
    pub value: *mut GValue,
}

pub type GObjectConstructFunc =
    Option<unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject>;
pub type GObjectSetPropertyFunc =
    Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec)>;
pub type GObjectGetPropertyFunc =
    Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec)>;
pub type GObjectDisposeFunc = Option<unsafe extern "C" fn(*mut GObject)>;
pub type GObjectFinalizeFunc = Option<unsafe extern "C" fn(*mut GObject)>;
pub type GObjectDispatchPropertiesChangedFunc =
    Option<unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec)>;
pub type GObjectNotifyFunc = Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec)>;
pub type GObjectConstructedFunc = Option<unsafe extern "C" fn(*mut GObject)>;
pub type GParamSpecFinalizeFunc = Option<unsafe extern "C" fn(*mut GParamSpec)>;
pub type GParamSpecValueSetDefaultFunc =
    Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue)>;
pub type GParamSpecValueValidateFunc =
    Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean>;
pub type GParamSpecValuesCmpFunc =
    Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint>;
pub type GParamSpecValueIsValidFunc =
    Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GObjectClass {
    pub g_type_class: GTypeClass,
    pub construct_properties: gpointer,
    pub constructor: GObjectConstructFunc,
    pub set_property: GObjectSetPropertyFunc,
    pub get_property: GObjectGetPropertyFunc,
    pub dispose: GObjectDisposeFunc,
    pub finalize: GObjectFinalizeFunc,
    pub dispatch_properties_changed: GObjectDispatchPropertiesChangedFunc,
    pub notify: GObjectNotifyFunc,
    pub constructed: GObjectConstructedFunc,
    pub flags: gsize,
    pub n_construct_properties: gsize,
    pub pspecs: gpointer,
    pub n_pspecs: gsize,
    pub pdummy: [gpointer; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GParamSpecClass {
    pub g_type_class: GTypeClass,
    pub value_type: GType,
    pub finalize: GParamSpecFinalizeFunc,
    pub value_set_default: GParamSpecValueSetDefaultFunc,
    pub value_validate: GParamSpecValueValidateFunc,
    pub values_cmp: GParamSpecValuesCmpFunc,
    pub value_is_valid: GParamSpecValueIsValidFunc,
    pub dummy: [gpointer; 3],
}
