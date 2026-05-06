use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInitable;
    pub type _GIOExtension;
    pub type _GDBusConnection;
    pub type _GDBusProxyPrivate;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_get_string(value: *mut GVariant, length: *mut gsize) -> *const gchar;
    fn g_variant_lookup(
        dictionary: *mut GVariant,
        key: *const gchar,
        format_string: *const gchar,
        ...
    ) -> gboolean;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_type_name(type_0: GType) -> *const gchar;
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
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_object_class_override_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        name: *const gchar,
    );
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_unref(object: gpointer);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_power_profile_monitor_get_type() -> GType;
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
    fn g_dbus_proxy_get_cached_property(
        proxy: *mut GDBusProxy,
        property_name: *const gchar,
    ) -> *mut GVariant;
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
pub type GStrv = *mut *mut gchar;
pub type GVariantType = _GVariantType;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPowerProfileMonitorInterface {
    pub g_iface: GTypeInterface,
}
pub type GPowerProfileMonitorInterface = _GPowerProfileMonitorInterface;
pub type GPowerProfileMonitorDBus = _GPowerProfileMonitorDBus;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPowerProfileMonitorDBus {
    pub parent_instance: GObject,
    pub watch_id: guint,
    pub cancellable: *mut GCancellable,
    pub proxy: *mut GDBusProxy,
    pub signal_id: gulong,
    pub power_saver_enabled: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GPowerProfileMonitorDBusClass {
    pub parent_class: GObjectClass,
}
pub const PROP_POWER_SAVER_ENABLED: GPowerProfileMonitorDBusProperty = 1;
pub type GPowerProfileMonitorDBusProperty = ::core::ffi::c_uint;
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
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
pub const G_VARIANT_TYPE_STRING: *const GVariantType =
    b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
#[inline]
unsafe extern "C" fn safe_c2rust_G_POWER_PROFILE_MONITOR_DBUS(
    mut ptr: gpointer,
) -> *mut GPowerProfileMonitorDBus {
    return ptr as *mut GPowerProfileMonitorDBus;
}
pub const POWERPROFILES_DBUS_NAME: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"net.hadess.PowerProfiles\0")
};
pub const POWERPROFILES_DBUS_IFACE: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"net.hadess.PowerProfiles\0")
};
pub const POWERPROFILES_DBUS_PATH: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(*b"/net/hadess/PowerProfiles\0")
};
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_power_profile_monitor_dbus_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GPowerProfileMonitorDBus\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GPowerProfileMonitorDBusClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_power_profile_monitor_dbus_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GPowerProfileMonitorDBus>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GPowerProfileMonitorDBus) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_power_profile_monitor_dbus_init
                    as unsafe extern "C" fn(*mut GPowerProfileMonitorDBus) -> (),
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
            safe_c2rust_g_power_profile_monitor_dbus_initable_iface_init
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
            Option<unsafe extern "C" fn(*mut GPowerProfileMonitorInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_power_profile_monitor_dbus_iface_init
                as unsafe extern "C" fn(*mut GPowerProfileMonitorInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_power_profile_monitor_get_type(),
        &raw const g_implement_interface_info_0,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gio-power-profile-monitor\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"dbus\0" as *const u8 as *const ::core::ffi::c_char,
        30 as gint,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_power_profile_monitor_dbus_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_power_profile_monitor_dbus_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust_g_power_profile_monitor_dbus_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GPowerProfileMonitorDBus_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GPowerProfileMonitorDBus_private_offset,
        );
    }
    safe_c2rust_g_power_profile_monitor_dbus_class_init(
        klass as *mut GPowerProfileMonitorDBusClass,
    );
}
static mut safe_c2rust_GPowerProfileMonitorDBus_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_power_profile_monitor_dbus_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_power_profile_monitor_dbus_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_power_profile_monitor_dbus_init(
    mut dbus: *mut GPowerProfileMonitorDBus,
) {
    (*dbus).power_saver_enabled = FALSE as gboolean;
}
unsafe extern "C" fn safe_c2rust_ppd_properties_changed_cb(
    mut proxy: *mut GDBusProxy,
    mut changed_properties: *mut GVariant,
    mut invalidated_properties: *mut GStrv,
    mut user_data: gpointer,
) {
    let mut dbus: *mut GPowerProfileMonitorDBus = user_data as *mut GPowerProfileMonitorDBus;
    let mut active_profile: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut enabled: gboolean = 0;
    if g_variant_lookup(
        changed_properties,
        b"ActiveProfile\0" as *const u8 as *const gchar,
        b"&s\0" as *const u8 as *const gchar,
        &raw mut active_profile,
    ) == 0
    {
        return;
    }
    enabled = (g_strcmp0(
        active_profile,
        b"power-saver\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    if enabled == (*dbus).power_saver_enabled {
        return;
    }
    (*dbus).power_saver_enabled = enabled;
    g_object_notify(
        dbus as *mut ::core::ffi::c_void as *mut GObject,
        b"power-saver-enabled\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_ppd_proxy_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut dbus: *mut GPowerProfileMonitorDBus = user_data as *mut GPowerProfileMonitorDBus;
    let mut active_profile_variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut proxy: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut active_profile: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut power_saver_enabled: gboolean = 0;
    proxy = g_dbus_proxy_new_finish(res, &raw mut error);
    if proxy.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"GPowerProfileMonitorDBus: Failed to create PowerProfiles D-Bus proxy: %s\0"
                as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    active_profile_variant =
        g_dbus_proxy_get_cached_property(proxy, b"ActiveProfile\0" as *const u8 as *const gchar);
    if !active_profile_variant.is_null()
        && g_variant_is_of_type(active_profile_variant, G_VARIANT_TYPE_STRING) != 0
    {
        active_profile =
            g_variant_get_string(active_profile_variant, ::core::ptr::null_mut::<gsize>())
                as *const ::core::ffi::c_char;
        power_saver_enabled = (g_strcmp0(
            active_profile,
            b"power-saver\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as gboolean;
        if power_saver_enabled != (*dbus).power_saver_enabled {
            (*dbus).power_saver_enabled = power_saver_enabled;
            g_object_notify(
                dbus as *mut ::core::ffi::c_void as *mut GObject,
                b"power-saver-enabled\0" as *const u8 as *const gchar,
            );
        }
    }
    let mut _pp: *mut *mut GVariant = &raw mut active_profile_variant;
    let mut _ptr: *mut GVariant = *_pp;
    *_pp = ::core::ptr::null_mut::<GVariant>();
    if !_ptr.is_null() {
        g_variant_unref(_ptr as *mut GVariant);
    }
    (*dbus).signal_id = g_signal_connect_data(
        proxy as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"g-properties-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *mut GStrv, gpointer) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_ppd_properties_changed_cb
                as unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *mut GStrv, gpointer) -> (),
        )),
        dbus as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    (*dbus).proxy = safe_c2rust_g_steal_pointer(&raw mut proxy as gpointer) as *mut GDBusProxy
        as *mut GDBusProxy;
}
unsafe extern "C" fn safe_c2rust_ppd_appeared_cb(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut name_owner: *const gchar,
    mut user_data: gpointer,
) {
    let mut dbus: *mut GPowerProfileMonitorDBus = user_data as *mut GPowerProfileMonitorDBus;
    g_dbus_proxy_new(
        connection,
        G_DBUS_PROXY_FLAGS_NONE,
        ::core::ptr::null_mut::<GDBusInterfaceInfo>(),
        POWERPROFILES_DBUS_NAME.as_ptr() as *const gchar,
        POWERPROFILES_DBUS_PATH.as_ptr() as *const gchar,
        POWERPROFILES_DBUS_IFACE.as_ptr() as *const gchar,
        (*dbus).cancellable,
        Some(
            safe_c2rust_ppd_proxy_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        dbus as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_ppd_vanished_cb(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut user_data: gpointer,
) {
    let mut dbus: *mut GPowerProfileMonitorDBus = user_data as *mut GPowerProfileMonitorDBus;
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
    (*dbus).power_saver_enabled = FALSE as gboolean;
    g_object_notify(
        dbus as *mut ::core::ffi::c_void as *mut GObject,
        b"power-saver-enabled\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_power_profile_monitor_dbus_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut dbus: *mut GPowerProfileMonitorDBus =
        safe_c2rust_G_POWER_PROFILE_MONITOR_DBUS(object as gpointer);
    match prop_id as GPowerProfileMonitorDBusProperty as ::core::ffi::c_uint {
        1 => {
            g_value_set_boolean(value, (*dbus).power_saver_enabled);
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gpowerprofilemonitordbus.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_power_profile_monitor_dbus_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut dbus: *mut GPowerProfileMonitorDBus =
        safe_c2rust_G_POWER_PROFILE_MONITOR_DBUS(initable as gpointer);
    (*dbus).cancellable = g_cancellable_new();
    (*dbus).watch_id = g_bus_watch_name(
        G_BUS_TYPE_SYSTEM,
        POWERPROFILES_DBUS_NAME.as_ptr() as *const gchar,
        G_BUS_NAME_WATCHER_FLAGS_AUTO_START,
        Some(
            safe_c2rust_ppd_appeared_cb
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    gpointer,
                ) -> (),
        ),
        Some(
            safe_c2rust_ppd_vanished_cb
                as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
        ),
        dbus as gpointer,
        None,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_power_profile_monitor_dbus_finalize(mut object: *mut GObject) {
    let mut dbus: *mut GPowerProfileMonitorDBus =
        safe_c2rust_G_POWER_PROFILE_MONITOR_DBUS(object as gpointer);
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
    (*(safe_c2rust_g_power_profile_monitor_dbus_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_power_profile_monitor_dbus_class_init(
    mut nl_class: *mut GPowerProfileMonitorDBusClass,
) {
    let mut gobject_class: *mut GObjectClass =
        nl_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_power_profile_monitor_dbus_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_power_profile_monitor_dbus_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_override_property(
        gobject_class,
        PROP_POWER_SAVER_ENABLED as ::core::ffi::c_int as guint,
        b"power-saver-enabled\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_power_profile_monitor_dbus_iface_init(
    mut monitor_iface: *mut GPowerProfileMonitorInterface,
) {
}
unsafe extern "C" fn safe_c2rust_g_power_profile_monitor_dbus_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    (*iface).init = Some(
        safe_c2rust_g_power_profile_monitor_dbus_initable_init
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
