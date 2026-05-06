extern "C" {
    pub type _GData;
    pub type _GCancellable;
    pub type _GDrive;
    pub type _GMount;
    pub type _GVolume;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_volume_monitor_get_type() -> GType;
}
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub type GValue = _GValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
pub type GParamFlags = ::core::ffi::c_int;
pub const G_PARAM_DEPRECATED: GParamFlags = -2147483648;
pub const G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;
pub const G_PARAM_STATIC_BLURB: GParamFlags = 128;
pub const G_PARAM_STATIC_NICK: GParamFlags = 64;
pub const G_PARAM_PRIVATE: GParamFlags = 32;
pub const G_PARAM_STATIC_NAME: GParamFlags = 32;
pub const G_PARAM_LAX_VALIDATION: GParamFlags = 16;
pub const G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;
pub const G_PARAM_CONSTRUCT: GParamFlags = 4;
pub const G_PARAM_READWRITE: GParamFlags = 3;
pub const G_PARAM_WRITABLE: GParamFlags = 2;
pub const G_PARAM_READABLE: GParamFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpec {
    pub g_type_instance: GTypeInstance,
    pub name: *const gchar,
    pub flags: GParamFlags,
    pub value_type: GType,
    pub owner_type: GType,
    pub _nick: *mut gchar,
    pub _blurb: *mut gchar,
    pub qdata: *mut GData,
    pub ref_count: guint,
    pub param_id: guint,
}
pub type GParamSpec = _GParamSpec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectClass {
    pub g_type_class: GTypeClass,
    pub construct_properties: *mut GSList,
    pub constructor:
        Option<unsafe extern "C" fn(GType, guint, *mut GObjectConstructParam) -> *mut GObject>,
    pub set_property:
        Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>,
    pub get_property:
        Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>,
    pub dispose: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub finalize: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub dispatch_properties_changed:
        Option<unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> ()>,
    pub notify: Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>,
    pub constructed: Option<unsafe extern "C" fn(*mut GObject) -> ()>,
    pub flags: gsize,
    pub n_construct_properties: gsize,
    pub pspecs: gpointer,
    pub n_pspecs: gsize,
    pub pdummy: [gpointer; 3],
}
pub type GObjectConstructParam = _GObjectConstructParam;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObjectConstructParam {
    pub pspec: *mut GParamSpec,
    pub value: *mut GValue,
}
pub type GObjectClass = _GObjectClass;
pub type GCancellable = _GCancellable;
pub type GDrive = _GDrive;
pub type GMount = _GMount;
pub type GVolume = _GVolume;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVolumeMonitor {
    pub parent_instance: GObject,
    pub priv_0: gpointer,
}
pub type GVolumeMonitor = _GVolumeMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVolumeMonitorClass {
    pub parent_class: GObjectClass,
    pub volume_added: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GVolume) -> ()>,
    pub volume_removed: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GVolume) -> ()>,
    pub volume_changed: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GVolume) -> ()>,
    pub mount_added: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount) -> ()>,
    pub mount_removed: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount) -> ()>,
    pub mount_pre_unmount: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount) -> ()>,
    pub mount_changed: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GMount) -> ()>,
    pub drive_connected: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GDrive) -> ()>,
    pub drive_disconnected: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GDrive) -> ()>,
    pub drive_changed: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GDrive) -> ()>,
    pub is_supported: Option<unsafe extern "C" fn() -> gboolean>,
    pub get_connected_drives: Option<unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList>,
    pub get_volumes: Option<unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList>,
    pub get_mounts: Option<unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList>,
    pub get_volume_for_uuid: Option<
        unsafe extern "C" fn(*mut GVolumeMonitor, *const ::core::ffi::c_char) -> *mut GVolume,
    >,
    pub get_mount_for_uuid: Option<
        unsafe extern "C" fn(*mut GVolumeMonitor, *const ::core::ffi::c_char) -> *mut GMount,
    >,
    pub adopt_orphan_mount:
        Option<unsafe extern "C" fn(*mut GMount, *mut GVolumeMonitor) -> *mut GVolume>,
    pub drive_eject_button: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GDrive) -> ()>,
    pub drive_stop_button: Option<unsafe extern "C" fn(*mut GVolumeMonitor, *mut GDrive) -> ()>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
}
pub type GVolumeMonitorClass = _GVolumeMonitorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNativeVolumeMonitor {
    pub parent_instance: GVolumeMonitor,
}
pub type GNativeVolumeMonitor = _GNativeVolumeMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNativeVolumeMonitorClass {
    pub parent_class: GVolumeMonitorClass,
    pub get_mount_for_mount_path:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut GCancellable) -> *mut GMount>,
}
pub type GNativeVolumeMonitorClass = _GNativeVolumeMonitorClass;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_native_volume_monitor_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_native_volume_monitor_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_native_volume_monitor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_volume_monitor_get_type(),
        g_intern_static_string(b"GNativeVolumeMonitor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GNativeVolumeMonitorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_native_volume_monitor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GNativeVolumeMonitor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GNativeVolumeMonitor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_native_volume_monitor_init
                    as unsafe extern "C" fn(*mut GNativeVolumeMonitor) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_native_volume_monitor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_native_volume_monitor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GNativeVolumeMonitor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GNativeVolumeMonitor_private_offset,
        );
    }
    safe_c2rust_g_native_volume_monitor_class_init(klass as *mut GNativeVolumeMonitorClass);
}
static mut safe_c2rust_g_native_volume_monitor_parent_class: gpointer = NULL;
static mut safe_c2rust_GNativeVolumeMonitor_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_native_volume_monitor_finalize(mut object: *mut GObject) {
    (*(safe_c2rust_g_native_volume_monitor_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_native_volume_monitor_class_init(
    mut klass: *mut GNativeVolumeMonitorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_native_volume_monitor_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_native_volume_monitor_init(
    mut native_monitor: *mut GNativeVolumeMonitor,
) {
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
