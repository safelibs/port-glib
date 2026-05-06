use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GDrive;
    pub type _GMount;
    pub type _GVolume;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_signal_new(
        signal_name: *const gchar,
        itype: GType,
        signal_flags: GSignalFlags,
        class_offset: guint,
        accumulator: GSignalAccumulator,
        accu_data: gpointer,
        c_marshaller: GSignalCMarshaller,
        return_type: GType,
        n_params: guint,
        ...
    ) -> guint;
    fn g_volume_get_type() -> GType;
    fn g_mount_get_type() -> GType;
    fn g_drive_get_type() -> GType;
}
pub type guint32 = ::core::ffi::c_uint;
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
pub type GQuark = guint32;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GClosure {
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
    pub marshal: Option<
        unsafe extern "C" fn(
            *mut GClosure,
            *mut GValue,
            guint,
            *const GValue,
            gpointer,
            gpointer,
        ) -> (),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}
pub type GClosureNotifyData = _GClosureNotifyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}
pub type GClosureNotify = Option<unsafe extern "C" fn(gpointer, *mut GClosure) -> ()>;
pub type GClosure = _GClosure;
pub type GClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        guint,
        *const GValue,
        gpointer,
        gpointer,
    ) -> (),
>;
pub type GSignalFlags = ::core::ffi::c_uint;
pub const G_SIGNAL_ACCUMULATOR_FIRST_RUN: GSignalFlags = 131072;
pub const G_SIGNAL_DEPRECATED: GSignalFlags = 256;
pub const G_SIGNAL_MUST_COLLECT: GSignalFlags = 128;
pub const G_SIGNAL_NO_HOOKS: GSignalFlags = 64;
pub const G_SIGNAL_ACTION: GSignalFlags = 32;
pub const G_SIGNAL_DETAILED: GSignalFlags = 16;
pub const G_SIGNAL_NO_RECURSE: GSignalFlags = 8;
pub const G_SIGNAL_RUN_CLEANUP: GSignalFlags = 4;
pub const G_SIGNAL_RUN_LAST: GSignalFlags = 2;
pub const G_SIGNAL_RUN_FIRST: GSignalFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalInvocationHint {
    pub signal_id: guint,
    pub detail: GQuark,
    pub run_type: GSignalFlags,
}
pub type GSignalInvocationHint = _GSignalInvocationHint;
pub type GSignalCMarshaller = GClosureMarshal;
pub type GSignalAccumulator = Option<
    unsafe extern "C" fn(
        *mut GSignalInvocationHint,
        *mut GValue,
        *const GValue,
        gpointer,
    ) -> gboolean,
>;
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
pub const DRIVE_STOP_BUTTON: C2RustUnnamed_0 = 11;
pub const DRIVE_EJECT_BUTTON: C2RustUnnamed_0 = 10;
pub const DRIVE_CHANGED: C2RustUnnamed_0 = 9;
pub const DRIVE_DISCONNECTED: C2RustUnnamed_0 = 8;
pub const DRIVE_CONNECTED: C2RustUnnamed_0 = 7;
pub const MOUNT_CHANGED: C2RustUnnamed_0 = 6;
pub const MOUNT_PRE_UNMOUNT: C2RustUnnamed_0 = 5;
pub const MOUNT_REMOVED: C2RustUnnamed_0 = 4;
pub const MOUNT_ADDED: C2RustUnnamed_0 = 3;
pub const VOLUME_CHANGED: C2RustUnnamed_0 = 2;
pub const VOLUME_REMOVED: C2RustUnnamed_0 = 1;
pub const VOLUME_ADDED: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_0 = 12;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_volume_monitor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GVolumeMonitor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GVolumeMonitorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_volume_monitor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GVolumeMonitor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVolumeMonitor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_volume_monitor_init
                    as unsafe extern "C" fn(*mut GVolumeMonitor) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_volume_monitor_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_volume_monitor_get_type_once();
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
static mut safe_c2rust_g_volume_monitor_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_volume_monitor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_volume_monitor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GVolumeMonitor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GVolumeMonitor_private_offset,
        );
    }
    safe_c2rust_g_volume_monitor_class_init(klass as *mut GVolumeMonitorClass);
}
static mut safe_c2rust_GVolumeMonitor_private_offset: gint = 0;
static mut safe_c2rust_signals: [guint; 12] = [
    0 as ::core::ffi::c_int as guint,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn safe_c2rust_g_volume_monitor_finalize(mut object: *mut GObject) {
    (*(safe_c2rust_g_volume_monitor_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_volume_monitor_class_init(mut klass: *mut GVolumeMonitorClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_volume_monitor_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    safe_c2rust_signals[VOLUME_ADDED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"volume-added\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        136 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_volume_get_type(),
    );
    safe_c2rust_signals[VOLUME_REMOVED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"volume-removed\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        144 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_volume_get_type(),
    );
    safe_c2rust_signals[VOLUME_CHANGED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"volume-changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        152 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_volume_get_type(),
    );
    safe_c2rust_signals[MOUNT_ADDED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"mount-added\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        160 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_mount_get_type(),
    );
    safe_c2rust_signals[MOUNT_REMOVED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"mount-removed\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        168 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_mount_get_type(),
    );
    safe_c2rust_signals[MOUNT_PRE_UNMOUNT as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"mount-pre-unmount\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        176 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_mount_get_type(),
    );
    safe_c2rust_signals[MOUNT_CHANGED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"mount-changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        184 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_mount_get_type(),
    );
    safe_c2rust_signals[DRIVE_CONNECTED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"drive-connected\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        192 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_drive_get_type(),
    );
    safe_c2rust_signals[DRIVE_DISCONNECTED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"drive-disconnected\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        200 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_drive_get_type(),
    );
    safe_c2rust_signals[DRIVE_CHANGED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"drive-changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        208 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_drive_get_type(),
    );
    safe_c2rust_signals[DRIVE_EJECT_BUTTON as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"drive-eject-button\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        272 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_drive_get_type(),
    );
    safe_c2rust_signals[DRIVE_STOP_BUTTON as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"drive-stop-button\0" as *const u8 as *const gchar),
        safe_c2rust_g_volume_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        280 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        g_drive_get_type(),
    );
}
unsafe extern "C" fn safe_c2rust_g_volume_monitor_init(mut monitor: *mut GVolumeMonitor) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_volume_monitor_get_connected_drives(
    mut volume_monitor: *mut GVolumeMonitor,
) -> *mut GList {
    let mut class: *mut GVolumeMonitorClass = ::core::ptr::null_mut::<GVolumeMonitorClass>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = volume_monitor as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_volume_monitor_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_VOLUME_MONITOR (volume_monitor)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    class = (*(volume_monitor as *mut GTypeInstance)).g_class as *mut GVolumeMonitorClass;
    return (*class)
        .get_connected_drives
        .expect("non-null function pointer")(volume_monitor);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_volume_monitor_get_volumes(
    mut volume_monitor: *mut GVolumeMonitor,
) -> *mut GList {
    let mut class: *mut GVolumeMonitorClass = ::core::ptr::null_mut::<GVolumeMonitorClass>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = volume_monitor as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_volume_monitor_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_VOLUME_MONITOR (volume_monitor)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    class = (*(volume_monitor as *mut GTypeInstance)).g_class as *mut GVolumeMonitorClass;
    return (*class).get_volumes.expect("non-null function pointer")(volume_monitor);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_volume_monitor_get_mounts(
    mut volume_monitor: *mut GVolumeMonitor,
) -> *mut GList {
    let mut class: *mut GVolumeMonitorClass = ::core::ptr::null_mut::<GVolumeMonitorClass>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = volume_monitor as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_volume_monitor_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_VOLUME_MONITOR (volume_monitor)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    class = (*(volume_monitor as *mut GTypeInstance)).g_class as *mut GVolumeMonitorClass;
    return (*class).get_mounts.expect("non-null function pointer")(volume_monitor);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_volume_monitor_get_volume_for_uuid(
    mut volume_monitor: *mut GVolumeMonitor,
    mut uuid: *const ::core::ffi::c_char,
) -> *mut GVolume {
    let mut class: *mut GVolumeMonitorClass = ::core::ptr::null_mut::<GVolumeMonitorClass>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = volume_monitor as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_volume_monitor_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_VOLUME_MONITOR (volume_monitor)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVolume>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !uuid.is_null() {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uuid != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVolume>();
    }
    class = (*(volume_monitor as *mut GTypeInstance)).g_class as *mut GVolumeMonitorClass;
    return (*class)
        .get_volume_for_uuid
        .expect("non-null function pointer")(volume_monitor, uuid);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_volume_monitor_get_mount_for_uuid(
    mut volume_monitor: *mut GVolumeMonitor,
    mut uuid: *const ::core::ffi::c_char,
) -> *mut GMount {
    let mut class: *mut GVolumeMonitorClass = ::core::ptr::null_mut::<GVolumeMonitorClass>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = volume_monitor as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_volume_monitor_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as ::core::ffi::c_int as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_VOLUME_MONITOR (volume_monitor)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMount>();
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !uuid.is_null() {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"uuid != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMount>();
    }
    class = (*(volume_monitor as *mut GTypeInstance)).g_class as *mut GVolumeMonitorClass;
    return (*class)
        .get_mount_for_uuid
        .expect("non-null function pointer")(volume_monitor, uuid);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
