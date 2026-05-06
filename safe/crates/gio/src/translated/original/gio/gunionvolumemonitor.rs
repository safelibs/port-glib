use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GCancellable;
    pub type _GDrive;
    pub type _GIOExtensionPoint;
    pub type _GIOExtension;
    pub type _GMount;
    pub type _GVolume;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_rec_mutex_lock(rec_mutex: *mut GRecMutex);
    fn g_rec_mutex_unlock(rec_mutex: *mut GRecMutex);
    fn g_once_impl(once: *mut GOnce, func: GThreadFunc, arg: gpointer) -> gpointer;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_concat(list1: *mut GList, list2: *mut GList) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn g_list_find(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
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
    fn g_object_run_dispose(object: *mut GObject);
    fn g_volume_monitor_get_type() -> GType;
    fn g_volume_monitor_get_connected_drives(volume_monitor: *mut GVolumeMonitor) -> *mut GList;
    fn g_volume_monitor_get_volumes(volume_monitor: *mut GVolumeMonitor) -> *mut GList;
    fn g_volume_monitor_get_mounts(volume_monitor: *mut GVolumeMonitor) -> *mut GList;
    fn g_volume_monitor_get_volume_for_uuid(
        volume_monitor: *mut GVolumeMonitor,
        uuid: *const ::core::ffi::c_char,
    ) -> *mut GVolume;
    fn g_volume_monitor_get_mount_for_uuid(
        volume_monitor: *mut GVolumeMonitor,
        uuid: *const ::core::ffi::c_char,
    ) -> *mut GMount;
    fn g_io_extension_point_lookup(name: *const ::core::ffi::c_char) -> *mut GIOExtensionPoint;
    fn g_io_extension_point_get_extensions(extension_point: *mut GIOExtensionPoint) -> *mut GList;
    fn g_io_extension_get_type(extension: *mut GIOExtension) -> GType;
    fn g_io_extension_ref_class(extension: *mut GIOExtension) -> *mut GTypeClass;
    fn _g_io_module_get_default_type(
        extension_point: *const gchar,
        envvar: *const gchar,
        is_supported_offset: guint,
    ) -> GType;
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
pub type GQuark = guint32;
pub type GThreadFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRecMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GRecMutex = _GRecMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOnce {
    pub status: GOnceStatus,
    pub retval: gpointer,
}
pub type GOnceStatus = ::core::ffi::c_uint;
pub const G_ONCE_STATUS_READY: GOnceStatus = 2;
pub const G_ONCE_STATUS_PROGRESS: GOnceStatus = 1;
pub const G_ONCE_STATUS_NOTCALLED: GOnceStatus = 0;
pub type GOnce = _GOnce;
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
pub type GCancellable = _GCancellable;
pub type GDrive = _GDrive;
pub type GIOExtensionPoint = _GIOExtensionPoint;
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
pub type GUnionVolumeMonitor = _GUnionVolumeMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnionVolumeMonitor {
    pub parent: GVolumeMonitor,
    pub monitors: *mut GList,
}
pub type GNativeVolumeMonitorClass = _GNativeVolumeMonitorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNativeVolumeMonitorClass {
    pub parent_class: GVolumeMonitorClass,
    pub get_mount_for_mount_path:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut GCancellable) -> *mut GMount>,
}
pub type GUnionVolumeMonitorClass = _GUnionVolumeMonitorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnionVolumeMonitorClass {
    pub parent_class: GVolumeMonitorClass,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_VOLUME_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"gio-volume-monitor\0")
};
pub const G_NATIVE_VOLUME_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(*b"gio-native-volume-monitor\0")
};
static mut safe_c2rust_GUnionVolumeMonitor_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_union_volume_monitor_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_union_volume_monitor_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_union_volume_monitor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_volume_monitor_get_type(),
        g_intern_static_string(b"GUnionVolumeMonitor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GUnionVolumeMonitorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_union_volume_monitor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GUnionVolumeMonitor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GUnionVolumeMonitor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_union_volume_monitor_init
                    as unsafe extern "C" fn(*mut GUnionVolumeMonitor) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_union_volume_monitor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_union_volume_monitor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GUnionVolumeMonitor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GUnionVolumeMonitor_private_offset,
        );
    }
    safe_c2rust_g_union_volume_monitor_class_init(klass as *mut GUnionVolumeMonitorClass);
}
static mut safe_c2rust_g_union_volume_monitor_parent_class: gpointer = NULL;
static mut safe_c2rust_the_volume_monitor_mutex: GRecMutex = _GRecMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    i: [0; 2],
};
static mut safe_c2rust_the_volume_monitor: *mut GUnionVolumeMonitor =
    ::core::ptr::null::<GUnionVolumeMonitor>() as *mut GUnionVolumeMonitor;
unsafe extern "C" fn safe_c2rust_g_union_volume_monitor_finalize(mut object: *mut GObject) {
    let mut monitor: *mut GUnionVolumeMonitor = ::core::ptr::null_mut::<GUnionVolumeMonitor>();
    let mut child_monitor: *mut GVolumeMonitor = ::core::ptr::null_mut::<GVolumeMonitor>();
    monitor = object as *mut ::core::ffi::c_void as *mut GUnionVolumeMonitor;
    while !(*monitor).monitors.is_null() {
        child_monitor = (*(*monitor).monitors).data as *mut GVolumeMonitor;
        safe_c2rust_g_union_volume_monitor_remove_monitor(monitor, child_monitor);
        g_object_unref(child_monitor as gpointer);
    }
    (*(safe_c2rust_g_union_volume_monitor_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_union_volume_monitor_dispose(mut object: *mut GObject) {
    let mut monitor: *mut GUnionVolumeMonitor = ::core::ptr::null_mut::<GUnionVolumeMonitor>();
    let mut child_monitor: *mut GVolumeMonitor = ::core::ptr::null_mut::<GVolumeMonitor>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    monitor = object as *mut ::core::ffi::c_void as *mut GUnionVolumeMonitor;
    g_rec_mutex_lock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    safe_c2rust_the_volume_monitor = ::core::ptr::null_mut::<GUnionVolumeMonitor>();
    l = (*monitor).monitors;
    while !l.is_null() {
        child_monitor = (*l).data as *mut GVolumeMonitor;
        g_object_run_dispose(child_monitor as *mut ::core::ffi::c_void as *mut GObject);
        l = (*l).next;
    }
    g_rec_mutex_unlock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    (*(safe_c2rust_g_union_volume_monitor_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_get_mounts(mut volume_monitor: *mut GVolumeMonitor) -> *mut GList {
    let mut monitor: *mut GUnionVolumeMonitor = ::core::ptr::null_mut::<GUnionVolumeMonitor>();
    let mut child_monitor: *mut GVolumeMonitor = ::core::ptr::null_mut::<GVolumeMonitor>();
    let mut res: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    monitor = volume_monitor as *mut ::core::ffi::c_void as *mut GUnionVolumeMonitor;
    res = ::core::ptr::null_mut::<GList>();
    g_rec_mutex_lock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    l = (*monitor).monitors;
    while !l.is_null() {
        child_monitor = (*l).data as *mut GVolumeMonitor;
        res = g_list_concat(res, g_volume_monitor_get_mounts(child_monitor));
        l = (*l).next;
    }
    g_rec_mutex_unlock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    return res;
}
unsafe extern "C" fn safe_c2rust_get_volumes(
    mut volume_monitor: *mut GVolumeMonitor,
) -> *mut GList {
    let mut monitor: *mut GUnionVolumeMonitor = ::core::ptr::null_mut::<GUnionVolumeMonitor>();
    let mut child_monitor: *mut GVolumeMonitor = ::core::ptr::null_mut::<GVolumeMonitor>();
    let mut res: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    monitor = volume_monitor as *mut ::core::ffi::c_void as *mut GUnionVolumeMonitor;
    res = ::core::ptr::null_mut::<GList>();
    g_rec_mutex_lock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    l = (*monitor).monitors;
    while !l.is_null() {
        child_monitor = (*l).data as *mut GVolumeMonitor;
        res = g_list_concat(res, g_volume_monitor_get_volumes(child_monitor));
        l = (*l).next;
    }
    g_rec_mutex_unlock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    return res;
}
unsafe extern "C" fn safe_c2rust_get_connected_drives(
    mut volume_monitor: *mut GVolumeMonitor,
) -> *mut GList {
    let mut monitor: *mut GUnionVolumeMonitor = ::core::ptr::null_mut::<GUnionVolumeMonitor>();
    let mut child_monitor: *mut GVolumeMonitor = ::core::ptr::null_mut::<GVolumeMonitor>();
    let mut res: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    monitor = volume_monitor as *mut ::core::ffi::c_void as *mut GUnionVolumeMonitor;
    res = ::core::ptr::null_mut::<GList>();
    g_rec_mutex_lock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    l = (*monitor).monitors;
    while !l.is_null() {
        child_monitor = (*l).data as *mut GVolumeMonitor;
        res = g_list_concat(res, g_volume_monitor_get_connected_drives(child_monitor));
        l = (*l).next;
    }
    g_rec_mutex_unlock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    return res;
}
unsafe extern "C" fn safe_c2rust_get_volume_for_uuid(
    mut volume_monitor: *mut GVolumeMonitor,
    mut uuid: *const ::core::ffi::c_char,
) -> *mut GVolume {
    let mut monitor: *mut GUnionVolumeMonitor = ::core::ptr::null_mut::<GUnionVolumeMonitor>();
    let mut child_monitor: *mut GVolumeMonitor = ::core::ptr::null_mut::<GVolumeMonitor>();
    let mut volume: *mut GVolume = ::core::ptr::null_mut::<GVolume>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    monitor = volume_monitor as *mut ::core::ffi::c_void as *mut GUnionVolumeMonitor;
    volume = ::core::ptr::null_mut::<GVolume>();
    g_rec_mutex_lock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    l = (*monitor).monitors;
    while !l.is_null() {
        child_monitor = (*l).data as *mut GVolumeMonitor;
        volume = g_volume_monitor_get_volume_for_uuid(child_monitor, uuid);
        if !volume.is_null() {
            break;
        }
        l = (*l).next;
    }
    g_rec_mutex_unlock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    return volume;
}
unsafe extern "C" fn safe_c2rust_get_mount_for_uuid(
    mut volume_monitor: *mut GVolumeMonitor,
    mut uuid: *const ::core::ffi::c_char,
) -> *mut GMount {
    let mut monitor: *mut GUnionVolumeMonitor = ::core::ptr::null_mut::<GUnionVolumeMonitor>();
    let mut child_monitor: *mut GVolumeMonitor = ::core::ptr::null_mut::<GVolumeMonitor>();
    let mut mount: *mut GMount = ::core::ptr::null_mut::<GMount>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    monitor = volume_monitor as *mut ::core::ffi::c_void as *mut GUnionVolumeMonitor;
    mount = ::core::ptr::null_mut::<GMount>();
    g_rec_mutex_lock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    l = (*monitor).monitors;
    while !l.is_null() {
        child_monitor = (*l).data as *mut GVolumeMonitor;
        mount = g_volume_monitor_get_mount_for_uuid(child_monitor, uuid);
        if !mount.is_null() {
            break;
        }
        l = (*l).next;
    }
    g_rec_mutex_unlock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    return mount;
}
unsafe extern "C" fn safe_c2rust_g_union_volume_monitor_class_init(
    mut klass: *mut GUnionVolumeMonitorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut monitor_class: *mut GVolumeMonitorClass =
        klass as *mut ::core::ffi::c_void as *mut GVolumeMonitorClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_union_volume_monitor_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).dispose = Some(
        safe_c2rust_g_union_volume_monitor_dispose as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*monitor_class).get_connected_drives = Some(
        safe_c2rust_get_connected_drives as unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList,
    )
        as Option<unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList>;
    (*monitor_class).get_volumes =
        Some(safe_c2rust_get_volumes as unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList)
            as Option<unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList>;
    (*monitor_class).get_mounts =
        Some(safe_c2rust_get_mounts as unsafe extern "C" fn(*mut GVolumeMonitor) -> *mut GList)
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
}
unsafe extern "C" fn safe_c2rust_child_volume_added(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_volume: *mut GVolume,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"volume-added\0" as *const u8 as *const gchar,
        child_volume,
    );
}
unsafe extern "C" fn safe_c2rust_child_volume_removed(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_volume: *mut GVolume,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"volume-removed\0" as *const u8 as *const gchar,
        child_volume,
    );
}
unsafe extern "C" fn safe_c2rust_child_volume_changed(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_volume: *mut GVolume,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"volume-changed\0" as *const u8 as *const gchar,
        child_volume,
    );
}
unsafe extern "C" fn safe_c2rust_child_mount_added(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_mount: *mut GMount,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"mount-added\0" as *const u8 as *const gchar,
        child_mount,
    );
}
unsafe extern "C" fn safe_c2rust_child_mount_removed(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_mount: *mut GMount,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"mount-removed\0" as *const u8 as *const gchar,
        child_mount,
    );
}
unsafe extern "C" fn safe_c2rust_child_mount_pre_unmount(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_mount: *mut GMount,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"mount-pre-unmount\0" as *const u8 as *const gchar,
        child_mount,
    );
}
unsafe extern "C" fn safe_c2rust_child_mount_changed(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_mount: *mut GMount,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"mount-changed\0" as *const u8 as *const gchar,
        child_mount,
    );
}
unsafe extern "C" fn safe_c2rust_child_drive_connected(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_drive: *mut GDrive,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"drive-connected\0" as *const u8 as *const gchar,
        child_drive,
    );
}
unsafe extern "C" fn safe_c2rust_child_drive_disconnected(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_drive: *mut GDrive,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"drive-disconnected\0" as *const u8 as *const gchar,
        child_drive,
    );
}
unsafe extern "C" fn safe_c2rust_child_drive_changed(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_drive: *mut GDrive,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"drive-changed\0" as *const u8 as *const gchar,
        child_drive,
    );
}
unsafe extern "C" fn safe_c2rust_child_drive_eject_button(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_drive: *mut GDrive,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"drive-eject-button\0" as *const u8 as *const gchar,
        child_drive,
    );
}
unsafe extern "C" fn safe_c2rust_child_drive_stop_button(
    mut child_monitor: *mut GVolumeMonitor,
    mut child_drive: *mut GDrive,
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    g_signal_emit_by_name(
        union_monitor as gpointer,
        b"drive-stop-button\0" as *const u8 as *const gchar,
        child_drive,
    );
}
unsafe extern "C" fn safe_c2rust_g_union_volume_monitor_add_monitor(
    mut union_monitor: *mut GUnionVolumeMonitor,
    mut volume_monitor: *mut GVolumeMonitor,
) {
    if !g_list_find((*union_monitor).monitors, volume_monitor as gconstpointer).is_null() {
        return;
    }
    (*union_monitor).monitors = g_list_prepend(
        (*union_monitor).monitors,
        g_object_ref(volume_monitor as gpointer) as *mut GVolumeMonitor as gpointer,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"volume-added\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_volume_added
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"volume-removed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_volume_removed
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"volume-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_volume_changed
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"mount-added\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_mount_added
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"mount-removed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_mount_removed
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"mount-pre-unmount\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_mount_pre_unmount
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"mount-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_mount_changed
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"drive-connected\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_drive_connected
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"drive-disconnected\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_drive_disconnected
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"drive-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_drive_changed
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"drive-eject-button\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_drive_eject_button
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        volume_monitor as gpointer,
        b"drive-stop-button\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_child_drive_stop_button
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
}
unsafe extern "C" fn safe_c2rust_g_union_volume_monitor_remove_monitor(
    mut union_monitor: *mut GUnionVolumeMonitor,
    mut child_monitor: *mut GVolumeMonitor,
) {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    l = g_list_find((*union_monitor).monitors, child_monitor as gconstpointer);
    if l.is_null() {
        return;
    }
    (*union_monitor).monitors = g_list_delete_link((*union_monitor).monitors, l);
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_volume_added
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_volume_removed
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_volume_changed
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GVolume,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_mount_added
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_mount_removed
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_mount_pre_unmount
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_mount_changed
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GMount,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_drive_connected
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_drive_disconnected
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_drive_changed
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_drive_eject_button
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        child_monitor as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_child_drive_stop_button
                as unsafe extern "C" fn(
                    *mut GVolumeMonitor,
                    *mut GDrive,
                    *mut GUnionVolumeMonitor,
                ) -> (),
        )),
        union_monitor as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_get_default_native_class(mut data: gpointer) -> GType {
    return _g_io_module_get_default_type(
        G_NATIVE_VOLUME_MONITOR_EXTENSION_POINT_NAME.as_ptr() as *const gchar,
        b"GIO_USE_VOLUME_MONITOR\0" as *const u8 as *const gchar,
        216 as ::core::ffi::c_ulong as glong as guint,
    );
}
unsafe extern "C" fn safe_c2rust_get_native_class() -> *mut GNativeVolumeMonitorClass {
    static mut safe_c2rust_once_init: GOnce = _GOnce {
        status: G_ONCE_STATUS_NOTCALLED,
        retval: NULL,
    };
    let mut type_class: *mut GTypeClass = ::core::ptr::null_mut::<GTypeClass>();
    type_class = ::core::ptr::null_mut::<GTypeClass>();
    if crate::translated::compat::atomic_load_acquire(&raw mut safe_c2rust_once_init.status)
        as ::core::ffi::c_uint
        == G_ONCE_STATUS_READY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        &raw mut safe_c2rust_once_init;
    } else {
        g_once_impl(
            &raw mut safe_c2rust_once_init,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> GType>, GThreadFunc>(
                Some(
                    safe_c2rust_get_default_native_class as unsafe extern "C" fn(gpointer) -> GType,
                ),
            ),
            &raw mut type_class as gpointer,
        ) as *mut ::core::ffi::c_void;
    };
    if type_class.is_null() && !safe_c2rust_once_init.retval.is_null() {
        type_class = g_type_class_ref(safe_c2rust_once_init.retval as GType) as *mut GTypeClass;
    }
    return type_class as *mut GNativeVolumeMonitorClass;
}
unsafe extern "C" fn safe_c2rust_g_union_volume_monitor_init(
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
}
unsafe extern "C" fn safe_c2rust_populate_union_monitor(
    mut union_monitor: *mut GUnionVolumeMonitor,
) {
    let mut monitor: *mut GVolumeMonitor = ::core::ptr::null_mut::<GVolumeMonitor>();
    let mut native_class: *mut GNativeVolumeMonitorClass =
        ::core::ptr::null_mut::<GNativeVolumeMonitorClass>();
    let mut klass: *mut GVolumeMonitorClass = ::core::ptr::null_mut::<GVolumeMonitorClass>();
    let mut ep: *mut GIOExtensionPoint = ::core::ptr::null_mut::<GIOExtensionPoint>();
    let mut extension: *mut GIOExtension = ::core::ptr::null_mut::<GIOExtension>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    native_class = safe_c2rust_get_native_class();
    if !native_class.is_null() {
        monitor = g_object_new(
            (*(native_class as *mut GTypeClass)).g_type,
            ::core::ptr::null::<gchar>(),
        ) as *mut GVolumeMonitor;
        safe_c2rust_g_union_volume_monitor_add_monitor(union_monitor, monitor);
        g_object_unref(monitor as gpointer);
        g_type_class_unref(native_class as gpointer);
    }
    ep = g_io_extension_point_lookup(G_VOLUME_MONITOR_EXTENSION_POINT_NAME.as_ptr());
    l = g_io_extension_point_get_extensions(ep);
    while !l.is_null() {
        extension = (*l).data as *mut GIOExtension;
        klass = g_io_extension_ref_class(extension) as *mut ::core::ffi::c_void
            as *mut GVolumeMonitorClass;
        if (*klass).is_supported.is_none()
            || (*klass).is_supported.expect("non-null function pointer")() != 0
        {
            monitor = g_object_new(
                g_io_extension_get_type(extension),
                ::core::ptr::null::<gchar>(),
            ) as *mut GVolumeMonitor;
            safe_c2rust_g_union_volume_monitor_add_monitor(union_monitor, monitor);
            g_object_unref(monitor as gpointer);
        }
        g_type_class_unref(klass as gpointer);
        l = (*l).next;
    }
}
unsafe extern "C" fn safe_c2rust_g_union_volume_monitor_new() -> *mut GUnionVolumeMonitor {
    let mut monitor: *mut GUnionVolumeMonitor = ::core::ptr::null_mut::<GUnionVolumeMonitor>();
    monitor = g_object_new(
        safe_c2rust__g_union_volume_monitor_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GUnionVolumeMonitor;
    return monitor;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_volume_monitor_get() -> *mut GVolumeMonitor {
    let mut vm: *mut GVolumeMonitor = ::core::ptr::null_mut::<GVolumeMonitor>();
    g_rec_mutex_lock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    if !safe_c2rust_the_volume_monitor.is_null() {
        vm = g_object_ref(safe_c2rust_the_volume_monitor as gpointer) as *mut GUnionVolumeMonitor
            as *mut ::core::ffi::c_void as *mut GVolumeMonitor;
    } else {
        safe_c2rust_the_volume_monitor = safe_c2rust_g_union_volume_monitor_new();
        safe_c2rust_populate_union_monitor(safe_c2rust_the_volume_monitor);
        vm = safe_c2rust_the_volume_monitor as *mut ::core::ffi::c_void as *mut GVolumeMonitor;
    }
    g_rec_mutex_unlock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    return vm;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_mount_get_for_mount_path(
    mut mount_path: *const gchar,
    mut cancellable: *mut GCancellable,
) -> *mut GMount {
    let mut klass: *mut GNativeVolumeMonitorClass =
        ::core::ptr::null_mut::<GNativeVolumeMonitorClass>();
    let mut mount: *mut GMount = ::core::ptr::null_mut::<GMount>();
    klass = safe_c2rust_get_native_class();
    if klass.is_null() {
        return ::core::ptr::null_mut::<GMount>();
    }
    mount = ::core::ptr::null_mut::<GMount>();
    if (*klass).get_mount_for_mount_path.is_some() {
        mount = (*klass)
            .get_mount_for_mount_path
            .expect("non-null function pointer")(
            mount_path as *const ::core::ffi::c_char,
            cancellable,
        );
    }
    g_type_class_unref(klass as gpointer);
    return mount;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_volume_monitor_adopt_orphan_mount(
    mut mount: *mut GMount,
) -> *mut GVolume {
    let mut child_monitor: *mut GVolumeMonitor = ::core::ptr::null_mut::<GVolumeMonitor>();
    let mut child_monitor_class: *mut GVolumeMonitorClass =
        ::core::ptr::null_mut::<GVolumeMonitorClass>();
    let mut volume: *mut GVolume = ::core::ptr::null_mut::<GVolume>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !mount.is_null() {
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
            b"mount != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVolume>();
    }
    if safe_c2rust_the_volume_monitor.is_null() {
        return ::core::ptr::null_mut::<GVolume>();
    }
    volume = ::core::ptr::null_mut::<GVolume>();
    g_rec_mutex_lock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    l = (*safe_c2rust_the_volume_monitor).monitors;
    while !l.is_null() {
        child_monitor = (*l).data as *mut GVolumeMonitor;
        child_monitor_class =
            (*(child_monitor as *mut GTypeInstance)).g_class as *mut GVolumeMonitorClass;
        if (*child_monitor_class).adopt_orphan_mount.is_some() {
            volume = (*child_monitor_class)
                .adopt_orphan_mount
                .expect("non-null function pointer")(mount, child_monitor);
            if !volume.is_null() {
                break;
            }
        }
        l = (*l).next;
    }
    g_rec_mutex_unlock(&raw mut safe_c2rust_the_volume_monitor_mutex);
    return volume;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_ACQUIRE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
