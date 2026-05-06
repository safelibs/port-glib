use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GCancellablePrivate;
    pub type _GDrive;
    pub type _GIOExtension;
    pub type _GMount;
    pub type _GVolume;
    pub type _GUnixMountMonitor;
    pub type _GUnixMount;
    pub type _GUnixVolume;
    pub type _GUnixMountEntry;
    pub type _GUnixMountPoint;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_list_free(list: *mut GList);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_remove(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_list_copy_deep(list: *mut GList, func: GCopyFunc, user_data: gpointer) -> *mut GList;
    fn g_list_sort(list: *mut GList, compare_func: GCompareFunc) -> *mut GList;
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
    fn g_signal_emit_by_name(instance: gpointer, detailed_signal: *const gchar, ...);
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handlers_disconnect_matched(
        instance: gpointer,
        mask: GSignalMatchType,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        func: gpointer,
        data: gpointer,
    ) -> guint;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_native_volume_monitor_get_type() -> GType;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn g_unix_mount_free(mount_entry: *mut GUnixMountEntry);
    fn g_unix_mount_point_free(mount_point: *mut GUnixMountPoint);
    fn g_unix_mount_compare(mount1: *mut GUnixMountEntry, mount2: *mut GUnixMountEntry) -> gint;
    fn g_unix_mount_get_mount_path(mount_entry: *mut GUnixMountEntry)
        -> *const ::core::ffi::c_char;
    fn g_unix_mount_point_compare(
        mount1: *mut GUnixMountPoint,
        mount2: *mut GUnixMountPoint,
    ) -> gint;
    fn g_unix_mount_point_get_mount_path(
        mount_point: *mut GUnixMountPoint,
    ) -> *const ::core::ffi::c_char;
    fn g_unix_mount_points_get(time_read: *mut guint64) -> *mut GList;
    fn g_unix_mounts_get(time_read: *mut guint64) -> *mut GList;
    fn g_unix_mount_at(
        mount_path: *const ::core::ffi::c_char,
        time_read: *mut guint64,
    ) -> *mut GUnixMountEntry;
    fn g_unix_mount_monitor_get() -> *mut GUnixMountMonitor;
    fn _g_unix_mount_new(
        volume_monitor: *mut GVolumeMonitor,
        mount_entry: *mut GUnixMountEntry,
        volume: *mut GUnixVolume,
    ) -> *mut GUnixMount;
    fn _g_unix_mount_has_mount_path(
        mount: *mut GUnixMount,
        mount_path: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn _g_unix_mount_unmounted(mount: *mut GUnixMount);
    fn _g_unix_volume_new(
        volume_monitor: *mut GVolumeMonitor,
        mountpoint: *mut GUnixMountPoint,
    ) -> *mut GUnixVolume;
    fn _g_unix_volume_has_mount_path(
        volume: *mut GUnixVolume,
        mount_path: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn _g_unix_volume_disconnected(volume: *mut GUnixVolume);
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
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GCopyFunc = Option<unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer>;
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
pub type GCallback = Option<unsafe extern "C" fn() -> ()>;
pub type GConnectFlags = ::core::ffi::c_uint;
pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
pub const G_CONNECT_AFTER: GConnectFlags = 1;
pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
pub type GSignalMatchType = ::core::ffi::c_uint;
pub const G_SIGNAL_MATCH_UNBLOCKED: GSignalMatchType = 32;
pub const G_SIGNAL_MATCH_DATA: GSignalMatchType = 16;
pub const G_SIGNAL_MATCH_FUNC: GSignalMatchType = 8;
pub const G_SIGNAL_MATCH_CLOSURE: GSignalMatchType = 4;
pub const G_SIGNAL_MATCH_DETAIL: GSignalMatchType = 2;
pub const G_SIGNAL_MATCH_ID: GSignalMatchType = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GDrive = _GDrive;
pub type GIOExtension = _GIOExtension;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixVolumeMonitor {
    pub parent: GNativeVolumeMonitor,
    pub mount_monitor: *mut GUnixMountMonitor,
    pub last_mountpoints: *mut GList,
    pub last_mounts: *mut GList,
    pub volumes: *mut GList,
    pub mounts: *mut GList,
}
pub type GUnixMountMonitor = _GUnixMountMonitor;
pub type GUnixVolumeMonitor = _GUnixVolumeMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixVolumeMonitorClass {
    pub parent_class: GNativeVolumeMonitorClass,
}
pub type GUnixVolumeMonitorClass = _GUnixVolumeMonitorClass;
pub type GUnixMount = _GUnixMount;
pub type GUnixVolume = _GUnixVolume;
pub type GUnixMountEntry = _GUnixMountEntry;
pub type GUnixMountPoint = _GUnixMountPoint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_volume_monitor_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_unix_volume_monitor_get_type_once();
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
static mut safe_c2rust_g_unix_volume_monitor_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_unix_volume_monitor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_unix_volume_monitor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GUnixVolumeMonitor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GUnixVolumeMonitor_private_offset,
        );
    }
    safe_c2rust_g_unix_volume_monitor_class_init(klass as *mut GUnixVolumeMonitorClass);
}
static mut safe_c2rust_GUnixVolumeMonitor_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_unix_volume_monitor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_native_volume_monitor_get_type(),
        g_intern_static_string(b"GUnixVolumeMonitor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GUnixVolumeMonitorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_volume_monitor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GUnixVolumeMonitor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnixVolumeMonitor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_unix_volume_monitor_init
                    as unsafe extern "C" fn(*mut GUnixVolumeMonitor) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    g_io_extension_point_implement(
        b"gio-native-volume-monitor\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"unix\0" as *const u8 as *const ::core::ffi::c_char,
        0 as gint,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_monitor_finalize(mut object: *mut GObject) {
    let mut monitor: *mut GUnixVolumeMonitor = ::core::ptr::null_mut::<GUnixVolumeMonitor>();
    monitor = object as *mut ::core::ffi::c_void as *mut GUnixVolumeMonitor;
    g_signal_handlers_disconnect_matched(
        (*monitor).mount_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountMonitor, gpointer) -> ()>,
            gpointer,
        >(Some(
            safe_c2rust_mountpoints_changed
                as unsafe extern "C" fn(*mut GUnixMountMonitor, gpointer) -> (),
        )),
        monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*monitor).mount_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountMonitor, gpointer) -> ()>,
            gpointer,
        >(Some(
            safe_c2rust_mounts_changed
                as unsafe extern "C" fn(*mut GUnixMountMonitor, gpointer) -> (),
        )),
        monitor as gpointer,
    );
    g_object_unref((*monitor).mount_monitor as gpointer);
    g_list_free_full(
        (*monitor).last_mountpoints,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountPoint) -> ()>,
            GDestroyNotify,
        >(Some(
            g_unix_mount_point_free as unsafe extern "C" fn(*mut GUnixMountPoint) -> (),
        )),
    );
    g_list_free_full(
        (*monitor).last_mounts,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountEntry) -> ()>,
            GDestroyNotify,
        >(Some(
            g_unix_mount_free as unsafe extern "C" fn(*mut GUnixMountEntry) -> (),
        )),
    );
    g_list_free_full(
        (*monitor).volumes,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_list_free_full(
        (*monitor).mounts,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*(safe_c2rust_g_unix_volume_monitor_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_monitor_dispose(mut object: *mut GObject) {
    let mut monitor: *mut GUnixVolumeMonitor = ::core::ptr::null_mut::<GUnixVolumeMonitor>();
    monitor = object as *mut ::core::ffi::c_void as *mut GUnixVolumeMonitor;
    g_list_free_full(
        (*monitor).volumes,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*monitor).volumes = ::core::ptr::null_mut::<GList>();
    g_list_free_full(
        (*monitor).mounts,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*monitor).mounts = ::core::ptr::null_mut::<GList>();
    (*(safe_c2rust_g_unix_volume_monitor_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_get_mounts(mut volume_monitor: *mut GVolumeMonitor) -> *mut GList {
    let mut monitor: *mut GUnixVolumeMonitor = ::core::ptr::null_mut::<GUnixVolumeMonitor>();
    monitor = volume_monitor as *mut ::core::ffi::c_void as *mut GUnixVolumeMonitor;
    return g_list_copy_deep(
        (*monitor).mounts,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gpointer>, GCopyFunc>(
            Some(g_object_ref as unsafe extern "C" fn(gpointer) -> gpointer),
        ),
        NULL,
    );
}
unsafe extern "C" fn safe_c2rust_get_volumes(
    mut volume_monitor: *mut GVolumeMonitor,
) -> *mut GList {
    let mut monitor: *mut GUnixVolumeMonitor = ::core::ptr::null_mut::<GUnixVolumeMonitor>();
    monitor = volume_monitor as *mut ::core::ffi::c_void as *mut GUnixVolumeMonitor;
    return g_list_copy_deep(
        (*monitor).volumes,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gpointer>, GCopyFunc>(
            Some(g_object_ref as unsafe extern "C" fn(gpointer) -> gpointer),
        ),
        NULL,
    );
}
unsafe extern "C" fn safe_c2rust_get_connected_drives(
    mut volume_monitor: *mut GVolumeMonitor,
) -> *mut GList {
    return ::core::ptr::null_mut::<GList>();
}
unsafe extern "C" fn safe_c2rust_get_volume_for_uuid(
    mut volume_monitor: *mut GVolumeMonitor,
    mut uuid: *const ::core::ffi::c_char,
) -> *mut GVolume {
    return ::core::ptr::null_mut::<GVolume>();
}
unsafe extern "C" fn safe_c2rust_get_mount_for_uuid(
    mut volume_monitor: *mut GVolumeMonitor,
    mut uuid: *const ::core::ffi::c_char,
) -> *mut GMount {
    return ::core::ptr::null_mut::<GMount>();
}
unsafe extern "C" fn safe_c2rust_is_supported() -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_get_mount_for_mount_path(
    mut mount_path: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
) -> *mut GMount {
    let mut mount_entry: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
    let mut mount: *mut GUnixMount = ::core::ptr::null_mut::<GUnixMount>();
    mount_entry = g_unix_mount_at(mount_path, ::core::ptr::null_mut::<guint64>());
    if mount_entry.is_null() {
        return ::core::ptr::null_mut::<GMount>();
    }
    mount = _g_unix_mount_new(
        ::core::ptr::null_mut::<GVolumeMonitor>(),
        mount_entry,
        ::core::ptr::null_mut::<GUnixVolume>(),
    );
    g_unix_mount_free(mount_entry);
    return mount as *mut ::core::ffi::c_void as *mut GMount;
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_monitor_class_init(
    mut klass: *mut GUnixVolumeMonitorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut monitor_class: *mut GVolumeMonitorClass =
        klass as *mut ::core::ffi::c_void as *mut GVolumeMonitorClass;
    let mut native_class: *mut GNativeVolumeMonitorClass =
        klass as *mut ::core::ffi::c_void as *mut GNativeVolumeMonitorClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_unix_volume_monitor_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).dispose =
        Some(safe_c2rust_g_unix_volume_monitor_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*monitor_class).get_mounts =
        Some(safe_c2rust_get_mounts as unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList)
            as Option<unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList>;
    (*monitor_class).get_volumes =
        Some(safe_c2rust_get_volumes as unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList)
            as Option<unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList>;
    (*monitor_class).get_connected_drives = Some(
        safe_c2rust_get_connected_drives as unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList,
    )
        as Option<unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList>;
    (*monitor_class).get_volume_for_uuid = Some(
        safe_c2rust_get_volume_for_uuid
            as unsafe extern "C" fn(
                *mut GVolumeMonitor,
                *const ::core::ffi::c_char,
            ) -> *mut GVolume,
    )
        as Option<
            unsafe extern "C" fn(*mut GVolumeMonitor, *const ::core::ffi::c_char) -> *mut GVolume,
        >;
    (*monitor_class).get_mount_for_uuid = Some(
        safe_c2rust_get_mount_for_uuid
            as unsafe extern "C" fn(*mut GVolumeMonitor, *const ::core::ffi::c_char) -> *mut GMount,
    )
        as Option<
            unsafe extern "C" fn(*mut GVolumeMonitor, *const ::core::ffi::c_char) -> *mut GMount,
        >;
    (*monitor_class).is_supported =
        Some(safe_c2rust_is_supported as unsafe extern "C" fn() -> gboolean)
            as Option<unsafe extern "C" fn() -> gboolean>;
    (*native_class).get_mount_for_mount_path = Some(
        safe_c2rust_get_mount_for_mount_path
            as unsafe extern "C" fn(*const ::core::ffi::c_char, *mut GCancellable) -> *mut GMount,
    )
        as Option<
            unsafe extern "C" fn(*const ::core::ffi::c_char, *mut GCancellable) -> *mut GMount,
        >;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_volume_monitor_update(
    mut unix_monitor: *mut GUnixVolumeMonitor,
) {
    safe_c2rust_update_volumes(unix_monitor);
    safe_c2rust_update_mounts(unix_monitor);
}
unsafe extern "C" fn safe_c2rust_mountpoints_changed(
    mut mount_monitor: *mut GUnixMountMonitor,
    mut user_data: gpointer,
) {
    let mut unix_monitor: *mut GUnixVolumeMonitor = user_data as *mut GUnixVolumeMonitor;
    safe_c2rust__g_unix_volume_monitor_update(unix_monitor);
}
unsafe extern "C" fn safe_c2rust_mounts_changed(
    mut mount_monitor: *mut GUnixMountMonitor,
    mut user_data: gpointer,
) {
    let mut unix_monitor: *mut GUnixVolumeMonitor = user_data as *mut GUnixVolumeMonitor;
    safe_c2rust__g_unix_volume_monitor_update(unix_monitor);
}
unsafe extern "C" fn safe_c2rust_g_unix_volume_monitor_init(
    mut unix_monitor: *mut GUnixVolumeMonitor,
) {
    (*unix_monitor).mount_monitor = g_unix_mount_monitor_get();
    g_signal_connect_data(
        (*unix_monitor).mount_monitor as gpointer,
        b"mounts-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountMonitor, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_mounts_changed
                as unsafe extern "C" fn(*mut GUnixMountMonitor, gpointer) -> (),
        )),
        unix_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        (*unix_monitor).mount_monitor as gpointer,
        b"mountpoints-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountMonitor, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_mountpoints_changed
                as unsafe extern "C" fn(*mut GUnixMountMonitor, gpointer) -> (),
        )),
        unix_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    safe_c2rust__g_unix_volume_monitor_update(unix_monitor);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_volume_monitor_new() -> *mut GVolumeMonitor {
    let mut monitor: *mut GUnixVolumeMonitor = ::core::ptr::null_mut::<GUnixVolumeMonitor>();
    monitor = g_object_new(
        safe_c2rust__g_unix_volume_monitor_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GUnixVolumeMonitor;
    return monitor as *mut ::core::ffi::c_void as *mut GVolumeMonitor;
}
unsafe extern "C" fn safe_c2rust_diff_sorted_lists(
    mut list1: *mut GList,
    mut list2: *mut GList,
    mut compare: GCompareFunc,
    mut added: *mut *mut GList,
    mut removed: *mut *mut GList,
) {
    let mut order: ::core::ffi::c_int = 0;
    *removed = ::core::ptr::null_mut::<GList>();
    *added = *removed;
    while !list1.is_null() && !list2.is_null() {
        order = Some(compare.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*list1).data as gconstpointer,
            (*list2).data as gconstpointer,
        ) as ::core::ffi::c_int;
        if order < 0 as ::core::ffi::c_int {
            *removed = g_list_prepend(*removed, (*list1).data);
            list1 = (*list1).next;
        } else if order > 0 as ::core::ffi::c_int {
            *added = g_list_prepend(*added, (*list2).data);
            list2 = (*list2).next;
        } else {
            list1 = (*list1).next;
            list2 = (*list2).next;
        }
    }
    while !list1.is_null() {
        *removed = g_list_prepend(*removed, (*list1).data);
        list1 = (*list1).next;
    }
    while !list2.is_null() {
        *added = g_list_prepend(*added, (*list2).data);
        list2 = (*list2).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_volume_monitor_lookup_volume_for_mount_path(
    mut monitor: *mut GUnixVolumeMonitor,
    mut mount_path: *const ::core::ffi::c_char,
) -> *mut GUnixVolume {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    l = (*monitor).volumes;
    while !l.is_null() {
        let mut volume: *mut GUnixVolume = (*l).data as *mut GUnixVolume;
        if _g_unix_volume_has_mount_path(volume, mount_path) != 0 {
            return volume;
        }
        l = (*l).next;
    }
    return ::core::ptr::null_mut::<GUnixVolume>();
}
unsafe extern "C" fn safe_c2rust_find_mount_by_mountpath(
    mut monitor: *mut GUnixVolumeMonitor,
    mut mount_path: *const ::core::ffi::c_char,
) -> *mut GUnixMount {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    l = (*monitor).mounts;
    while !l.is_null() {
        let mut mount: *mut GUnixMount = (*l).data as *mut GUnixMount;
        if _g_unix_mount_has_mount_path(mount, mount_path) != 0 {
            return mount;
        }
        l = (*l).next;
    }
    return ::core::ptr::null_mut::<GUnixMount>();
}
unsafe extern "C" fn safe_c2rust_update_volumes(mut monitor: *mut GUnixVolumeMonitor) {
    let mut new_mountpoints: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut removed: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut added: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut volume: *mut GUnixVolume = ::core::ptr::null_mut::<GUnixVolume>();
    new_mountpoints = g_unix_mount_points_get(::core::ptr::null_mut::<guint64>());
    new_mountpoints = g_list_sort(
        new_mountpoints,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountPoint, *mut GUnixMountPoint) -> gint>,
            GCompareFunc,
        >(Some(
            g_unix_mount_point_compare
                as unsafe extern "C" fn(*mut GUnixMountPoint, *mut GUnixMountPoint) -> gint,
        )),
    );
    safe_c2rust_diff_sorted_lists(
        (*monitor).last_mountpoints,
        new_mountpoints,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountPoint, *mut GUnixMountPoint) -> gint>,
            GCompareFunc,
        >(Some(
            g_unix_mount_point_compare
                as unsafe extern "C" fn(*mut GUnixMountPoint, *mut GUnixMountPoint) -> gint,
        )),
        &raw mut added,
        &raw mut removed,
    );
    l = removed;
    while !l.is_null() {
        let mut mountpoint: *mut GUnixMountPoint = (*l).data as *mut GUnixMountPoint;
        volume = safe_c2rust__g_unix_volume_monitor_lookup_volume_for_mount_path(
            monitor,
            g_unix_mount_point_get_mount_path(mountpoint),
        );
        if !volume.is_null() {
            _g_unix_volume_disconnected(volume);
            (*monitor).volumes = g_list_remove((*monitor).volumes, volume as gconstpointer);
            g_signal_emit_by_name(
                monitor as gpointer,
                b"volume-removed\0" as *const u8 as *const gchar,
                volume,
            );
            g_signal_emit_by_name(
                volume as gpointer,
                b"removed\0" as *const u8 as *const gchar,
            );
            g_object_unref(volume as gpointer);
        }
        l = (*l).next;
    }
    l = added;
    while !l.is_null() {
        let mut mountpoint_0: *mut GUnixMountPoint = (*l).data as *mut GUnixMountPoint;
        volume = _g_unix_volume_new(
            monitor as *mut ::core::ffi::c_void as *mut GVolumeMonitor,
            mountpoint_0,
        );
        if !volume.is_null() {
            (*monitor).volumes = g_list_prepend((*monitor).volumes, volume as gpointer);
            g_signal_emit_by_name(
                monitor as gpointer,
                b"volume-added\0" as *const u8 as *const gchar,
                volume,
            );
        }
        l = (*l).next;
    }
    g_list_free(added);
    g_list_free(removed);
    g_list_free_full(
        (*monitor).last_mountpoints,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountPoint) -> ()>,
            GDestroyNotify,
        >(Some(
            g_unix_mount_point_free as unsafe extern "C" fn(*mut GUnixMountPoint) -> (),
        )),
    );
    (*monitor).last_mountpoints = new_mountpoints;
}
unsafe extern "C" fn safe_c2rust_update_mounts(mut monitor: *mut GUnixVolumeMonitor) {
    let mut new_mounts: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut removed: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut added: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut mount: *mut GUnixMount = ::core::ptr::null_mut::<GUnixMount>();
    let mut volume: *mut GUnixVolume = ::core::ptr::null_mut::<GUnixVolume>();
    let mut mount_path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    new_mounts = g_unix_mounts_get(::core::ptr::null_mut::<guint64>());
    new_mounts = g_list_sort(
        new_mounts,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountEntry, *mut GUnixMountEntry) -> gint>,
            GCompareFunc,
        >(Some(
            g_unix_mount_compare
                as unsafe extern "C" fn(*mut GUnixMountEntry, *mut GUnixMountEntry) -> gint,
        )),
    );
    safe_c2rust_diff_sorted_lists(
        (*monitor).last_mounts,
        new_mounts,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountEntry, *mut GUnixMountEntry) -> gint>,
            GCompareFunc,
        >(Some(
            g_unix_mount_compare
                as unsafe extern "C" fn(*mut GUnixMountEntry, *mut GUnixMountEntry) -> gint,
        )),
        &raw mut added,
        &raw mut removed,
    );
    l = removed;
    while !l.is_null() {
        let mut mount_entry: *mut GUnixMountEntry = (*l).data as *mut GUnixMountEntry;
        mount =
            safe_c2rust_find_mount_by_mountpath(monitor, g_unix_mount_get_mount_path(mount_entry));
        if !mount.is_null() {
            _g_unix_mount_unmounted(mount);
            (*monitor).mounts = g_list_remove((*monitor).mounts, mount as gconstpointer);
            g_signal_emit_by_name(
                monitor as gpointer,
                b"mount-removed\0" as *const u8 as *const gchar,
                mount,
            );
            g_signal_emit_by_name(
                mount as gpointer,
                b"unmounted\0" as *const u8 as *const gchar,
            );
            g_object_unref(mount as gpointer);
        }
        l = (*l).next;
    }
    l = added;
    while !l.is_null() {
        let mut mount_entry_0: *mut GUnixMountEntry = (*l).data as *mut GUnixMountEntry;
        mount_path = g_unix_mount_get_mount_path(mount_entry_0);
        volume =
            safe_c2rust__g_unix_volume_monitor_lookup_volume_for_mount_path(monitor, mount_path);
        mount = _g_unix_mount_new(
            monitor as *mut ::core::ffi::c_void as *mut GVolumeMonitor,
            mount_entry_0,
            volume,
        );
        if !mount.is_null() {
            (*monitor).mounts = g_list_prepend((*monitor).mounts, mount as gpointer);
            g_signal_emit_by_name(
                monitor as gpointer,
                b"mount-added\0" as *const u8 as *const gchar,
                mount,
            );
        }
        l = (*l).next;
    }
    g_list_free(added);
    g_list_free(removed);
    g_list_free_full(
        (*monitor).last_mounts,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GUnixMountEntry) -> ()>,
            GDestroyNotify,
        >(Some(
            g_unix_mount_free as unsafe extern "C" fn(*mut GUnixMountEntry) -> (),
        )),
    );
    (*monitor).last_mounts = new_mounts;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
