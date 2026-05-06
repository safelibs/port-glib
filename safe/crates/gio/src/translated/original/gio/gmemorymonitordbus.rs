use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInitable;
    pub type _GIOExtension;
    pub type _GDBusConnection;
    pub type _GDBusProxyPrivate;
    pub type _GMemoryMonitor;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
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
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_object_unref(object: gpointer);
    fn g_memory_monitor_get_type() -> GType;
    fn g_initable_get_type() -> GType;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_dbus_proxy_new(
        connection: *mut GDBusConnection,
        flags: GDBusProxyFlags,
        info: *mut GDBusInterfaceInfo,
        name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_proxy_new_finish(res: *mut GAsyncResult, error: *mut *mut GError) -> *mut GDBusProxy;
    fn g_bus_watch_name(
        bus_type: GBusType,
        name: *const gchar,
        flags: GBusNameWatcherFlags,
        name_appeared_handler: GBusNameAppearedCallback,
        name_vanished_handler: GBusNameVanishedCallback,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
    ) -> guint;
    fn g_bus_unwatch_name(watcher_id: guint);
}
pub type guint8 = ::core::ffi::c_uchar;
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
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GVariant = _GVariant;
pub type GLogLevelFlags = ::core::ffi::c_int;
pub const G_LOG_LEVEL_MASK: GLogLevelFlags = -4;
pub const G_LOG_LEVEL_DEBUG: GLogLevelFlags = 128;
pub const G_LOG_LEVEL_INFO: GLogLevelFlags = 64;
pub const G_LOG_LEVEL_MESSAGE: GLogLevelFlags = 32;
pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 16;
pub const G_LOG_LEVEL_CRITICAL: GLogLevelFlags = 8;
pub const G_LOG_LEVEL_ERROR: GLogLevelFlags = 4;
pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 2;
pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1;
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
pub struct _GTypeInterface {
    pub g_type: GType,
    pub g_instance_type: GType,
}
pub type GTypeInterface = _GTypeInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInterfaceInfo {
    pub interface_init: GInterfaceInitFunc,
    pub interface_finalize: GInterfaceFinalizeFunc,
    pub interface_data: gpointer,
}
pub type GInterfaceFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GInterfaceInfo = _GInterfaceInfo;
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
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GBusNameWatcherFlags = ::core::ffi::c_uint;
pub const G_BUS_NAME_WATCHER_FLAGS_AUTO_START: GBusNameWatcherFlags = 1;
pub const G_BUS_NAME_WATCHER_FLAGS_NONE: GBusNameWatcherFlags = 0;
pub type GDBusProxyFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROXY_FLAGS_NO_MATCH_RULE: GDBusProxyFlags = 32;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START_AT_CONSTRUCTION: GDBusProxyFlags = 16;
pub const G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES: GDBusProxyFlags = 8;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START: GDBusProxyFlags = 4;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS: GDBusProxyFlags = 2;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES: GDBusProxyFlags = 1;
pub const G_DBUS_PROXY_FLAGS_NONE: GDBusProxyFlags = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GMemoryMonitorWarningLevel = ::core::ffi::c_uint;
pub const G_MEMORY_MONITOR_WARNING_LEVEL_CRITICAL: GMemoryMonitorWarningLevel = 255;
pub const G_MEMORY_MONITOR_WARNING_LEVEL_MEDIUM: GMemoryMonitorWarningLevel = 100;
pub const G_MEMORY_MONITOR_WARNING_LEVEL_LOW: GMemoryMonitorWarningLevel = 50;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GInitable = _GInitable;
pub type GIOExtension = _GIOExtension;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusProxy {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusProxyPrivate,
}
pub type GDBusProxyPrivate = _GDBusProxyPrivate;
pub type GDBusProxy = _GDBusProxy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAnnotationInfo {
    pub ref_count: gint,
    pub key: *mut gchar,
    pub value: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusAnnotationInfo = _GDBusAnnotationInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusArgInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusArgInfo = _GDBusArgInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMethodInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub in_args: *mut *mut GDBusArgInfo,
    pub out_args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusMethodInfo = _GDBusMethodInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusSignalInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusSignalInfo = _GDBusSignalInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusPropertyInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub flags: GDBusPropertyInfoFlags,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusPropertyInfo = _GDBusPropertyInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub methods: *mut *mut GDBusMethodInfo,
    pub signals: *mut *mut GDBusSignalInfo,
    pub properties: *mut *mut GDBusPropertyInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusInterfaceInfo = _GDBusInterfaceInfo;
pub type GMemoryMonitor = _GMemoryMonitor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMemoryMonitorInterface {
    pub g_iface: GTypeInterface,
    pub low_memory_warning:
        Option<unsafe extern "C" fn(*mut GMemoryMonitor, GMemoryMonitorWarningLevel) -> ()>,
}
pub type GMemoryMonitorInterface = _GMemoryMonitorInterface;
pub type GMemoryMonitorDBus = _GMemoryMonitorDBus;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMemoryMonitorDBus {
    pub parent_instance: GObject,
    pub watch_id: guint,
    pub cancellable: *mut GCancellable,
    pub proxy: *mut GDBusProxy,
    pub signal_id: gulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GMemoryMonitorDBusClass {
    pub parent_class: GObjectClass,
}
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type GBusNameVanishedCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> ()>;
pub type GBusNameAppearedCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, *const gchar, gpointer) -> ()>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_G_MEMORY_MONITOR_DBUS(
    mut ptr: gpointer,
) -> *mut GMemoryMonitorDBus {
    return ptr as *mut GMemoryMonitorDBus;
}
static mut safe_c2rust_g_memory_monitor_dbus_parent_class: gpointer = NULL;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_memory_monitor_dbus_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GMemoryMonitorDBus\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMemoryMonitorDBusClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_memory_monitor_dbus_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMemoryMonitorDBus>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMemoryMonitorDBus) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_memory_monitor_dbus_init
                    as unsafe extern "C" fn(*mut GMemoryMonitorDBus) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_memory_monitor_dbus_initable_iface_init
                as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GMemoryMonitorInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_memory_monitor_dbus_iface_init
                as unsafe extern "C" fn(*mut GMemoryMonitorInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_memory_monitor_get_type(),
        &raw const g_implement_interface_info_0,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gio-memory-monitor\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"dbus\0" as *const u8 as *const ::core::ffi::c_char,
        30 as gint,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GMemoryMonitorDBus_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_monitor_dbus_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_memory_monitor_dbus_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_memory_monitor_dbus_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_memory_monitor_dbus_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMemoryMonitorDBus_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GMemoryMonitorDBus_private_offset,
        );
    }
    safe_c2rust_g_memory_monitor_dbus_class_init(klass as *mut GMemoryMonitorDBusClass);
}
unsafe extern "C" fn safe_c2rust_g_memory_monitor_dbus_init(mut dbus: *mut GMemoryMonitorDBus) {}
unsafe extern "C" fn safe_c2rust_proxy_signal_cb(
    mut proxy: *mut GDBusProxy,
    mut sender_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut dbus: *mut GMemoryMonitorDBus,
) {
    let mut level: guint8 = 0;
    if g_strcmp0(
        signal_name as *const ::core::ffi::c_char,
        b"LowMemoryWarning\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        return;
    }
    if parameters.is_null() {
        return;
    }
    g_variant_get(
        parameters,
        b"(y)\0" as *const u8 as *const gchar,
        &raw mut level,
    );
    g_signal_emit_by_name(
        dbus as gpointer,
        b"low-memory-warning\0" as *const u8 as *const gchar,
        level as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn safe_c2rust_lmm_proxy_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut dbus: *mut GMemoryMonitorDBus = user_data as *mut GMemoryMonitorDBus;
    let mut proxy: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    proxy = g_dbus_proxy_new_finish(res, &raw mut error);
    if proxy.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"Failed to create LowMemoryMonitor D-Bus proxy: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    (*dbus).signal_id = g_signal_connect_data(
        proxy as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"g-signal\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDBusProxy,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut GMemoryMonitorDBus,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_proxy_signal_cb
                as unsafe extern "C" fn(
                    *mut GDBusProxy,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut GMemoryMonitorDBus,
                ) -> (),
        )),
        dbus as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*dbus).proxy = proxy;
}
unsafe extern "C" fn safe_c2rust_lmm_appeared_cb(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut name_owner: *const gchar,
    mut user_data: gpointer,
) {
    let mut dbus: *mut GMemoryMonitorDBus = user_data as *mut GMemoryMonitorDBus;
    g_dbus_proxy_new(
        connection,
        G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START,
        ::core::ptr::null_mut::<GDBusInterfaceInfo>(),
        b"org.freedesktop.LowMemoryMonitor\0" as *const u8 as *const gchar,
        b"/org/freedesktop/LowMemoryMonitor\0" as *const u8 as *const gchar,
        b"org.freedesktop.LowMemoryMonitor\0" as *const u8 as *const gchar,
        (*dbus).cancellable,
        Some(
            safe_c2rust_lmm_proxy_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        dbus as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_lmm_vanished_cb(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut user_data: gpointer,
) {
    let mut dbus: *mut GMemoryMonitorDBus = user_data as *mut GMemoryMonitorDBus;
    let _instance: gpointer = (*dbus).proxy as gpointer;
    let _handler_id_ptr: *mut gulong = &raw mut (*dbus).signal_id;
    let _handler_id: gulong = *_handler_id_ptr;
    if _handler_id > 0 as gulong {
        *_handler_id_ptr = 0 as gulong;
        g_signal_handler_disconnect(_instance, _handler_id);
    }
    let mut _pp: *mut *mut GDBusProxy = &raw mut (*dbus).proxy;
    let mut _ptr: *mut GDBusProxy = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusProxy>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_memory_monitor_dbus_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut dbus: *mut GMemoryMonitorDBus = safe_c2rust_G_MEMORY_MONITOR_DBUS(initable as gpointer);
    (*dbus).cancellable = g_cancellable_new();
    (*dbus).watch_id = g_bus_watch_name(
        G_BUS_TYPE_SYSTEM,
        b"org.freedesktop.LowMemoryMonitor\0" as *const u8 as *const gchar,
        G_BUS_NAME_WATCHER_FLAGS_AUTO_START,
        Some(
            safe_c2rust_lmm_appeared_cb
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    gpointer,
                ) -> (),
        ),
        Some(
            safe_c2rust_lmm_vanished_cb
                as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
        ),
        dbus as gpointer,
        None,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_monitor_dbus_finalize(mut object: *mut GObject) {
    let mut dbus: *mut GMemoryMonitorDBus = safe_c2rust_G_MEMORY_MONITOR_DBUS(object as gpointer);
    g_cancellable_cancel((*dbus).cancellable);
    let mut _pp: *mut *mut GCancellable = &raw mut (*dbus).cancellable;
    let mut _ptr: *mut GCancellable = *_pp;
    *_pp = ::core::ptr::null_mut::<GCancellable>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let _instance: gpointer = (*dbus).proxy as gpointer;
    let _handler_id_ptr: *mut gulong = &raw mut (*dbus).signal_id;
    let _handler_id: gulong = *_handler_id_ptr;
    if _handler_id > 0 as gulong {
        *_handler_id_ptr = 0 as gulong;
        g_signal_handler_disconnect(_instance, _handler_id);
    }
    let mut _pp_0: *mut *mut GDBusProxy = &raw mut (*dbus).proxy;
    let mut _ptr_0: *mut GDBusProxy = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GDBusProxy>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    let mut _tag_ptr: *mut guint = &raw mut (*dbus).watch_id;
    let mut _handle_id: guint = 0;
    _handle_id = *_tag_ptr;
    if _handle_id > 0 as guint {
        *_tag_ptr = 0 as guint;
        g_bus_unwatch_name(_handle_id);
    }
    (*(safe_c2rust_g_memory_monitor_dbus_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_memory_monitor_dbus_class_init(
    mut nl_class: *mut GMemoryMonitorDBusClass,
) {
    let mut gobject_class: *mut GObjectClass =
        nl_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_memory_monitor_dbus_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_memory_monitor_dbus_iface_init(
    mut monitor_iface: *mut GMemoryMonitorInterface,
) {
}
unsafe extern "C" fn safe_c2rust_g_memory_monitor_dbus_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    (*iface).init = Some(
        safe_c2rust_g_memory_monitor_dbus_initable_init
            as unsafe extern "C" fn(
                *mut GInitable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
