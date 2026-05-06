extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GActionGroup;
    pub type _GApplicationPrivate;
    pub type _GNotification;
    pub type _GFile;
    pub type _GFileIcon;
    pub type _GIcon;
    pub type _GIOExtension;
    pub type _GThemedIcon;
    pub type _GDBusConnection;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_get_application_name() -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_remove(list: *mut GSList, data: gconstpointer) -> *mut GSList;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_is_floating(value: *mut GVariant) -> gboolean;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new_byte(value: guint8) -> *mut GVariant;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_take_string(string: *mut gchar) -> *mut GVariant;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_add_value(builder: *mut GVariantBuilder, value: *mut GVariant);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_notification_backend_get_type() -> GType;
    fn g_application_get_application_id(application: *mut GApplication) -> *const gchar;
    fn g_application_activate(application: *mut GApplication);
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn g_notification_get_title(notification: *mut GNotification) -> *const gchar;
    fn g_notification_get_body(notification: *mut GNotification) -> *const gchar;
    fn g_notification_get_category(notification: *mut GNotification) -> *const gchar;
    fn g_notification_get_icon(notification: *mut GNotification) -> *mut GIcon;
    fn g_notification_get_priority(notification: *mut GNotification) -> GNotificationPriority;
    fn g_notification_get_n_buttons(notification: *mut GNotification) -> guint;
    fn g_notification_get_button(
        notification: *mut GNotification,
        index: gint,
        label: *mut *mut gchar,
        action: *mut *mut gchar,
        target: *mut *mut GVariant,
    );
    fn g_notification_get_default_action(
        notification: *mut GNotification,
        action: *mut *mut gchar,
        target: *mut *mut GVariant,
    ) -> gboolean;
    fn g_dbus_connection_call(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_connection_call_finish(
        connection: *mut GDBusConnection,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_connection_signal_subscribe(
        connection: *mut GDBusConnection,
        sender: *const gchar,
        interface_name: *const gchar,
        member: *const gchar,
        object_path: *const gchar,
        arg0: *const gchar,
        flags: GDBusSignalFlags,
        callback: GDBusSignalCallback,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
    ) -> guint;
    fn g_dbus_connection_signal_unsubscribe(
        connection: *mut GDBusConnection,
        subscription_id: guint,
    );
    fn g_bus_watch_name_on_connection(
        connection: *mut GDBusConnection,
        name: *const gchar,
        flags: GBusNameWatcherFlags,
        name_appeared_handler: GBusNameAppearedCallback,
        name_vanished_handler: GBusNameVanishedCallback,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
    ) -> guint;
    fn g_bus_unwatch_name(watcher_id: guint);
    fn g_action_group_activate_action(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
        parameter: *mut GVariant,
    );
    fn g_action_group_query_action(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
        enabled: *mut gboolean,
        parameter_type: *mut *const GVariantType,
        state_type: *mut *const GVariantType,
        state_hint: *mut *mut GVariant,
        state: *mut *mut GVariant,
    ) -> gboolean;
    fn g_action_parse_detailed_name(
        detailed_name: *const gchar,
        action_name: *mut *mut gchar,
        target_value: *mut *mut GVariant,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_action_print_detailed_name(
        action_name: *const gchar,
        target_value: *mut GVariant,
    ) -> *mut gchar;
    fn g_themed_icon_get_type() -> GType;
    fn g_themed_icon_get_names(icon: *mut GThemedIcon) -> *const *const gchar;
    fn g_file_icon_get_type() -> GType;
    fn g_file_icon_get_file(icon: *mut GFileIcon) -> *mut GFile;
    fn g_file_get_path(file: *mut GFile) -> *mut ::core::ffi::c_char;
    fn g_dbus_generate_guid() -> *mut gchar;
}
pub type size_t = usize;
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
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
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
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: C2RustUnnamed_0,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
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
    pub data: [C2RustUnnamed_1; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub type GBusNameWatcherFlags = ::core::ffi::c_uint;
pub const G_BUS_NAME_WATCHER_FLAGS_AUTO_START: GBusNameWatcherFlags = 1;
pub const G_BUS_NAME_WATCHER_FLAGS_NONE: GBusNameWatcherFlags = 0;
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GDBusSignalFlags = ::core::ffi::c_uint;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH: GDBusSignalFlags = 4;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE: GDBusSignalFlags = 2;
pub const G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE: GDBusSignalFlags = 1;
pub const G_DBUS_SIGNAL_FLAGS_NONE: GDBusSignalFlags = 0;
pub type GNotificationPriority = ::core::ffi::c_uint;
pub const G_NOTIFICATION_PRIORITY_URGENT: GNotificationPriority = 3;
pub const G_NOTIFICATION_PRIORITY_HIGH: GNotificationPriority = 2;
pub const G_NOTIFICATION_PRIORITY_LOW: GNotificationPriority = 1;
pub const G_NOTIFICATION_PRIORITY_NORMAL: GNotificationPriority = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
pub type GActionGroup = _GActionGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplication {
    pub parent_instance: GObject,
    pub priv_0: *mut GApplicationPrivate,
}
pub type GApplicationPrivate = _GApplicationPrivate;
pub type GApplication = _GApplication;
pub type GNotification = _GNotification;
pub type GFile = _GFile;
pub type GFileIcon = _GFileIcon;
pub type GIcon = _GIcon;
pub type GIOExtension = _GIOExtension;
pub type GThemedIcon = _GThemedIcon;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNotificationBackend {
    pub parent_instance: GObject,
    pub application: *mut GApplication,
    pub dbus_connection: *mut GDBusConnection,
}
pub type GNotificationBackend = _GNotificationBackend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNotificationBackendClass {
    pub parent_class: GObjectClass,
    pub is_supported: Option<unsafe extern "C" fn() -> gboolean>,
    pub send_notification: Option<
        unsafe extern "C" fn(*mut GNotificationBackend, *const gchar, *mut GNotification) -> (),
    >,
    pub withdraw_notification:
        Option<unsafe extern "C" fn(*mut GNotificationBackend, *const gchar) -> ()>,
}
pub type GNotificationBackendClass = _GNotificationBackendClass;
pub type GDBusSignalCallback = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        gpointer,
    ) -> (),
>;
pub type GBusNameAppearedCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, *const gchar, gpointer) -> ()>;
pub type GBusNameVanishedCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFdoNotificationBackend {
    pub parent: GNotificationBackend,
    pub bus_name_id: guint,
    pub notify_subscription: guint,
    pub notifications: *mut GSList,
}
pub type GFdoNotificationBackend = _GFdoNotificationBackend;
pub type GFdoNotificationBackendClass = GNotificationBackendClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FreedesktopNotification {
    pub backend: *mut GFdoNotificationBackend,
    pub id: *mut gchar,
    pub notify_id: guint32,
    pub default_action: *mut gchar,
    pub default_action_target: *mut GVariant,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_strdup_inline(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if 0 != 0 && str.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if 0 != 0 && !str.is_null() && 0 != 0 {
        let len: size_t = (strlen(str) as size_t).wrapping_add(1 as size_t);
        let mut dup_str: *mut ::core::ffi::c_char =
            g_malloc(len as gsize) as *mut ::core::ffi::c_char;
        return memcpy(
            dup_str as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            len,
        ) as *mut ::core::ffi::c_char;
    }
    return g_strdup(str as *const gchar) as *mut ::core::ffi::c_char;
}
pub const G_VARIANT_TYPE_STRING_ARRAY: *const GVariantType =
    b"as\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_fdo_notification_backend_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GFdoNotificationBackend_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GFdoNotificationBackend_private_offset,
        );
    }
    safe_c2rust_g_fdo_notification_backend_class_init(klass as *mut GFdoNotificationBackendClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_notification_backend_get_type(),
        g_intern_static_string(b"GFdoNotificationBackend\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GFdoNotificationBackendClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_fdo_notification_backend_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GFdoNotificationBackend>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GFdoNotificationBackend) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_fdo_notification_backend_init
                    as unsafe extern "C" fn(*mut GFdoNotificationBackend) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gnotification-backend\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"freedesktop\0" as *const u8 as *const ::core::ffi::c_char,
        0 as gint,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_fdo_notification_backend_get_type_once();
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
static mut safe_c2rust_g_fdo_notification_backend_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GFdoNotificationBackend_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_freedesktop_notification_free(mut data: gpointer) {
    let mut n: *mut FreedesktopNotification = data as *mut FreedesktopNotification;
    g_object_unref((*n).backend as gpointer);
    g_free((*n).id as gpointer);
    g_free((*n).default_action as gpointer);
    if !(*n).default_action_target.is_null() {
        g_variant_unref((*n).default_action_target);
    }
    g_slice_free1(
        ::core::mem::size_of::<FreedesktopNotification>() as gsize,
        n as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_freedesktop_notification_new(
    mut backend: *mut GFdoNotificationBackend,
    mut id: *const gchar,
    mut notification: *mut GNotification,
) -> *mut FreedesktopNotification {
    let mut n: *mut FreedesktopNotification = ::core::ptr::null_mut::<FreedesktopNotification>();
    n = ({
        let mut __s: gsize = ::core::mem::size_of::<FreedesktopNotification>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut FreedesktopNotification;
    (*n).backend = g_object_ref(backend as gpointer) as *mut GFdoNotificationBackend
        as *mut GFdoNotificationBackend;
    (*n).id = safe_c2rust_g_strdup_inline(id as *const ::core::ffi::c_char) as *mut gchar;
    (*n).notify_id = 0 as guint32;
    g_notification_get_default_action(
        notification,
        &raw mut (*n).default_action,
        &raw mut (*n).default_action_target,
    );
    return n;
}
unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_find_notification(
    mut backend: *mut GFdoNotificationBackend,
    mut id: *const gchar,
) -> *mut FreedesktopNotification {
    let mut it: *mut GSList = ::core::ptr::null_mut::<GSList>();
    it = (*backend).notifications;
    while !it.is_null() {
        let mut n: *mut FreedesktopNotification = (*it).data as *mut FreedesktopNotification;
        if strcmp(
            (*n).id as *const ::core::ffi::c_char,
            id as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return n;
        }
        it = (*it).next;
    }
    return ::core::ptr::null_mut::<FreedesktopNotification>();
}
unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_find_notification_by_notify_id(
    mut backend: *mut GFdoNotificationBackend,
    mut id: guint32,
) -> *mut FreedesktopNotification {
    let mut it: *mut GSList = ::core::ptr::null_mut::<GSList>();
    it = (*backend).notifications;
    while !it.is_null() {
        let mut n: *mut FreedesktopNotification = (*it).data as *mut FreedesktopNotification;
        if (*n).notify_id == id {
            return n;
        }
        it = (*it).next;
    }
    return ::core::ptr::null_mut::<FreedesktopNotification>();
}
unsafe extern "C" fn safe_c2rust_activate_action(
    mut backend: *mut GFdoNotificationBackend,
    mut name: *const gchar,
    mut parameter: *mut GVariant,
) -> gboolean {
    let mut g_backend: *mut GNotificationBackend =
        backend as *mut ::core::ffi::c_void as *mut GNotificationBackend;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if parameter.is_null() || g_variant_is_floating(parameter) == 0 {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gfdonotificationbackend.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            142 as ::core::ffi::c_int,
            G_STRFUNC,
            b"parameter == NULL || !g_variant_is_floating (parameter)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if !name.is_null()
        && (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = name as *const ::core::ffi::c_char;
                let __prefix: *const ::core::ffi::c_char =
                    b"app.\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_11
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __prefix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(name, b"app.\0" as *const u8 as *const gchar)
        }) != 0
    {
        let mut parameter_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
        let mut action_name: *const gchar =
            name.offset(strlen(b"app.\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        if g_action_group_query_action(
            (*g_backend).application as *mut ::core::ffi::c_void as *mut GActionGroup,
            action_name,
            ::core::ptr::null_mut::<gboolean>(),
            &raw mut parameter_type,
            ::core::ptr::null_mut::<*const GVariantType>(),
            ::core::ptr::null_mut::<*mut GVariant>(),
            ::core::ptr::null_mut::<*mut GVariant>(),
        ) != 0
            && (parameter_type.is_null() && parameter.is_null()
                || !parameter_type.is_null()
                    && !parameter.is_null()
                    && g_variant_is_of_type(parameter, parameter_type) != 0)
        {
            g_action_group_activate_action(
                (*g_backend).application as *mut ::core::ffi::c_void as *mut GActionGroup,
                action_name,
                parameter,
            );
            return TRUE;
        }
    } else if name.is_null() {
        g_application_activate((*g_backend).application);
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_notify_signal(
    mut connection: *mut GDBusConnection,
    mut sender_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut backend: *mut GFdoNotificationBackend = user_data as *mut GFdoNotificationBackend;
    let mut id: guint32 = 0 as guint32;
    let mut action: *const gchar = ::core::ptr::null::<gchar>();
    let mut n: *mut FreedesktopNotification = ::core::ptr::null_mut::<FreedesktopNotification>();
    let mut notification_closed: gboolean = TRUE;
    if strcmp(
        signal_name as *const ::core::ffi::c_char,
        b"NotificationClosed\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_variant_is_of_type(
            parameters,
            g_variant_type_checked_(b"(uu)\0" as *const u8 as *const gchar),
        ) != 0
    {
        g_variant_get(
            parameters,
            b"(uu)\0" as *const u8 as *const gchar,
            &raw mut id,
            NULL_0,
        );
    } else if strcmp(
        signal_name as *const ::core::ffi::c_char,
        b"ActionInvoked\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_variant_is_of_type(
            parameters,
            g_variant_type_checked_(b"(us)\0" as *const u8 as *const gchar),
        ) != 0
    {
        g_variant_get(
            parameters,
            b"(u&s)\0" as *const u8 as *const gchar,
            &raw mut id,
            &raw mut action,
        );
    } else {
        return;
    }
    n = safe_c2rust_g_fdo_notification_backend_find_notification_by_notify_id(backend, id);
    if n.is_null() {
        return;
    }
    if !action.is_null() {
        if strcmp(
            action as *const ::core::ffi::c_char,
            b"default\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if safe_c2rust_activate_action(backend, (*n).default_action, (*n).default_action_target)
                == 0
            {
                notification_closed = FALSE as gboolean;
            }
        } else {
            let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut target: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            if g_action_parse_detailed_name(
                action,
                &raw mut name,
                &raw mut target,
                ::core::ptr::null_mut::<*mut GError>(),
            ) == 0
                || safe_c2rust_activate_action(backend, name, target) == 0
            {
                notification_closed = FALSE as gboolean;
            }
            g_free(name as gpointer);
            let mut _pp: *mut *mut GVariant = &raw mut target;
            let mut _ptr: *mut GVariant = *_pp;
            *_pp = ::core::ptr::null_mut::<GVariant>();
            if !_ptr.is_null() {
                g_variant_unref(_ptr as *mut GVariant);
            }
        }
    }
    if notification_closed != 0 {
        n = safe_c2rust_g_fdo_notification_backend_find_notification_by_notify_id(backend, id);
        if !n.is_null() {
            (*backend).notifications = g_slist_remove((*backend).notifications, n as gconstpointer);
            safe_c2rust_freedesktop_notification_free(n as gpointer);
        }
    }
}
unsafe extern "C" fn safe_c2rust_name_vanished_handler_cb(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut user_data: gpointer,
) {
    let mut backend: *mut GFdoNotificationBackend = user_data as *mut GFdoNotificationBackend;
    if !(*backend).notifications.is_null() {
        g_slist_free_full(
            (*backend).notifications,
            Some(safe_c2rust_freedesktop_notification_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        (*backend).notifications = ::core::ptr::null_mut::<GSList>();
    }
}
unsafe extern "C" fn safe_c2rust_urgency_from_priority(
    mut priority: GNotificationPriority,
) -> guchar {
    match priority as ::core::ffi::c_uint {
        1 => return 0 as guchar,
        3 => return 2 as guchar,
        0 | 2 | _ => return 1 as guchar,
    };
}
unsafe extern "C" fn safe_c2rust_call_notify(
    mut con: *mut GDBusConnection,
    mut app: *mut GApplication,
    mut replace_id: guint32,
    mut notification: *mut GNotification,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut action_builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut n_buttons: guint = 0;
    let mut i: guint = 0;
    let mut hints_builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    let mut parameters: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut app_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut body: *const gchar = ::core::ptr::null::<gchar>();
    let mut urgency: guchar = 0;
    g_variant_builder_init(&raw mut action_builder, G_VARIANT_TYPE_STRING_ARRAY);
    if g_notification_get_default_action(
        notification,
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut GVariant>(),
    ) != 0
    {
        g_variant_builder_add(
            &raw mut action_builder,
            b"s\0" as *const u8 as *const gchar,
            b"default\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_variant_builder_add(
            &raw mut action_builder,
            b"s\0" as *const u8 as *const gchar,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    n_buttons = g_notification_get_n_buttons(notification);
    i = 0 as guint;
    while i < n_buttons {
        let mut label: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut action: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut target: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut detailed_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
        g_notification_get_button(
            notification,
            i as gint,
            &raw mut label,
            &raw mut action,
            &raw mut target,
        );
        detailed_name = g_action_print_detailed_name(action, target);
        if strcmp(
            detailed_name as *const ::core::ffi::c_char,
            b"default\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            g_free(detailed_name as gpointer);
            detailed_name = g_dbus_generate_guid();
        }
        g_variant_builder_add_value(
            &raw mut action_builder,
            g_variant_new_take_string(detailed_name),
        );
        g_variant_builder_add_value(&raw mut action_builder, g_variant_new_take_string(label));
        g_free(action as gpointer);
        if !target.is_null() {
            g_variant_unref(target);
        }
        i = i.wrapping_add(1);
    }
    g_variant_builder_init(
        &raw mut hints_builder,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    g_variant_builder_add(
        &raw mut hints_builder,
        b"{sv}\0" as *const u8 as *const gchar,
        b"desktop-entry\0" as *const u8 as *const ::core::ffi::c_char,
        g_variant_new_string(g_application_get_application_id(app)),
    );
    urgency = safe_c2rust_urgency_from_priority(g_notification_get_priority(notification));
    g_variant_builder_add(
        &raw mut hints_builder,
        b"{sv}\0" as *const u8 as *const gchar,
        b"urgency\0" as *const u8 as *const ::core::ffi::c_char,
        g_variant_new_byte(urgency as guint8),
    );
    if !g_notification_get_category(notification).is_null() {
        g_variant_builder_add(
            &raw mut hints_builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"category\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_new_string(g_notification_get_category(notification)),
        );
    }
    icon = g_notification_get_icon(notification);
    if !icon.is_null() {
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = g_file_icon_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = FALSE as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
            file = g_file_icon_get_file(icon as *mut ::core::ffi::c_void as *mut GFileIcon);
            g_variant_builder_add(
                &raw mut hints_builder,
                b"{sv}\0" as *const u8 as *const gchar,
                b"image-path\0" as *const u8 as *const ::core::ffi::c_char,
                g_variant_new_take_string(g_file_get_path(file) as *mut gchar),
            );
        } else if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = g_themed_icon_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = FALSE as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {
            let mut icon_names: *const *const gchar =
                g_themed_icon_get_names(icon as *mut ::core::ffi::c_void as *mut GThemedIcon);
            g_variant_builder_add(
                &raw mut hints_builder,
                b"{sv}\0" as *const u8 as *const gchar,
                b"image-path\0" as *const u8 as *const ::core::ffi::c_char,
                g_variant_new_string(*icon_names.offset(0 as ::core::ffi::c_int as isize)),
            );
        }
    }
    app_name = g_get_application_name();
    body = g_notification_get_body(notification);
    parameters = g_variant_new(
        b"(susssasa{sv}i)\0" as *const u8 as *const gchar,
        if !app_name.is_null() {
            app_name as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        replace_id,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        g_notification_get_title(notification),
        if !body.is_null() {
            body as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        &raw mut action_builder,
        &raw mut hints_builder,
        -(1 as ::core::ffi::c_int),
    );
    g_dbus_connection_call(
        con,
        b"org.freedesktop.Notifications\0" as *const u8 as *const gchar,
        b"/org/freedesktop/Notifications\0" as *const u8 as *const gchar,
        b"org.freedesktop.Notifications\0" as *const u8 as *const gchar,
        b"Notify\0" as *const u8 as *const gchar,
        parameters,
        g_variant_type_checked_(b"(u)\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        callback,
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_notification_sent(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut n: *mut FreedesktopNotification = user_data as *mut FreedesktopNotification;
    let mut val: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    static mut safe_c2rust_warning_printed: gboolean = FALSE;
    val = g_dbus_connection_call_finish(
        source_object as *mut ::core::ffi::c_void as *mut GDBusConnection,
        result,
        &raw mut error,
    );
    if !val.is_null() {
        let mut backend: *mut GFdoNotificationBackend = (*n).backend;
        let mut match_0: *mut FreedesktopNotification =
            ::core::ptr::null_mut::<FreedesktopNotification>();
        g_variant_get(
            val,
            b"(u)\0" as *const u8 as *const gchar,
            &raw mut (*n).notify_id,
        );
        g_variant_unref(val);
        match_0 = safe_c2rust_g_fdo_notification_backend_find_notification_by_notify_id(
            backend,
            (*n).notify_id,
        );
        if !match_0.is_null() {
            (*backend).notifications =
                g_slist_remove((*backend).notifications, match_0 as gconstpointer);
            safe_c2rust_freedesktop_notification_free(match_0 as gpointer);
        }
        (*backend).notifications = g_slist_prepend((*backend).notifications, n as gpointer);
    } else {
        if safe_c2rust_warning_printed == 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"unable to send notifications through org.freedesktop.Notifications: %s\0"
                    as *const u8 as *const gchar,
                (*error).message,
            );
            safe_c2rust_warning_printed = TRUE as gboolean;
        }
        safe_c2rust_freedesktop_notification_free(n as gpointer);
        g_error_free(error);
    };
}
unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_dispose(mut object: *mut GObject) {
    let mut backend: *mut GFdoNotificationBackend =
        object as *mut ::core::ffi::c_void as *mut GFdoNotificationBackend;
    if (*backend).bus_name_id != 0 {
        g_bus_unwatch_name((*backend).bus_name_id);
        (*backend).bus_name_id = 0 as guint;
    }
    if (*backend).notify_subscription != 0 {
        let mut session_bus: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
        session_bus =
            (*(backend as *mut ::core::ffi::c_void as *mut GNotificationBackend)).dbus_connection;
        g_dbus_connection_signal_unsubscribe(session_bus, (*backend).notify_subscription);
        (*backend).notify_subscription = 0 as guint;
    }
    if !(*backend).notifications.is_null() {
        g_slist_free_full(
            (*backend).notifications,
            Some(safe_c2rust_freedesktop_notification_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        (*backend).notifications = ::core::ptr::null_mut::<GSList>();
    }
    (*(safe_c2rust_g_fdo_notification_backend_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_is_supported() -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_send_notification(
    mut backend: *mut GNotificationBackend,
    mut id: *const gchar,
    mut notification: *mut GNotification,
) {
    let mut self_0: *mut GFdoNotificationBackend =
        backend as *mut ::core::ffi::c_void as *mut GFdoNotificationBackend;
    let mut n: *mut FreedesktopNotification = ::core::ptr::null_mut::<FreedesktopNotification>();
    let mut tmp: *mut FreedesktopNotification = ::core::ptr::null_mut::<FreedesktopNotification>();
    if (*self_0).bus_name_id == 0 as guint {
        (*self_0).bus_name_id = g_bus_watch_name_on_connection(
            (*backend).dbus_connection,
            b"org.freedesktop.Notifications\0" as *const u8 as *const gchar,
            G_BUS_NAME_WATCHER_FLAGS_NONE,
            None,
            Some(
                safe_c2rust_name_vanished_handler_cb
                    as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
            ),
            backend as gpointer,
            None,
        );
    }
    if (*self_0).notify_subscription == 0 as guint {
        (*self_0).notify_subscription = g_dbus_connection_signal_subscribe(
            (*backend).dbus_connection,
            b"org.freedesktop.Notifications\0" as *const u8 as *const gchar,
            b"org.freedesktop.Notifications\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            b"/org/freedesktop/Notifications\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            G_DBUS_SIGNAL_FLAGS_NONE,
            Some(
                safe_c2rust_notify_signal
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut GVariant,
                        gpointer,
                    ) -> (),
            ),
            backend as gpointer,
            None,
        );
    }
    n = safe_c2rust_freedesktop_notification_new(self_0, id, notification);
    tmp = safe_c2rust_g_fdo_notification_backend_find_notification(self_0, id);
    if !tmp.is_null() {
        (*n).notify_id = (*tmp).notify_id;
    }
    safe_c2rust_call_notify(
        (*backend).dbus_connection,
        (*backend).application,
        (*n).notify_id,
        notification,
        Some(
            safe_c2rust_notification_sent
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        n as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_withdraw_notification(
    mut backend: *mut GNotificationBackend,
    mut id: *const gchar,
) {
    let mut self_0: *mut GFdoNotificationBackend =
        backend as *mut ::core::ffi::c_void as *mut GFdoNotificationBackend;
    let mut n: *mut FreedesktopNotification = ::core::ptr::null_mut::<FreedesktopNotification>();
    n = safe_c2rust_g_fdo_notification_backend_find_notification(self_0, id);
    if !n.is_null() {
        if (*n).notify_id > 0 as guint32 {
            g_dbus_connection_call(
                (*backend).dbus_connection,
                b"org.freedesktop.Notifications\0" as *const u8 as *const gchar,
                b"/org/freedesktop/Notifications\0" as *const u8 as *const gchar,
                b"org.freedesktop.Notifications\0" as *const u8 as *const gchar,
                b"CloseNotification\0" as *const u8 as *const gchar,
                g_variant_new(b"(u)\0" as *const u8 as *const gchar, (*n).notify_id),
                ::core::ptr::null::<GVariantType>(),
                G_DBUS_CALL_FLAGS_NONE,
                -(1 as gint),
                ::core::ptr::null_mut::<GCancellable>(),
                None,
                NULL_0,
            );
        }
        (*self_0).notifications = g_slist_remove((*self_0).notifications, n as gconstpointer);
        safe_c2rust_freedesktop_notification_free(n as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_init(
    mut backend: *mut GFdoNotificationBackend,
) {
}
unsafe extern "C" fn safe_c2rust_g_fdo_notification_backend_class_init(
    mut class: *mut GFdoNotificationBackendClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut backend_class: *mut GNotificationBackendClass =
        class as *mut ::core::ffi::c_void as *mut GNotificationBackendClass;
    (*object_class).dispose = Some(
        safe_c2rust_g_fdo_notification_backend_dispose as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*backend_class).is_supported = Some(
        safe_c2rust_g_fdo_notification_backend_is_supported as unsafe extern "C" fn() -> gboolean,
    ) as Option<unsafe extern "C" fn() -> gboolean>;
    (*backend_class).send_notification = Some(
        safe_c2rust_g_fdo_notification_backend_send_notification
            as unsafe extern "C" fn(
                *mut GNotificationBackend,
                *const gchar,
                *mut GNotification,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut GNotificationBackend, *const gchar, *mut GNotification) -> (),
        >;
    (*backend_class).withdraw_notification = Some(
        safe_c2rust_g_fdo_notification_backend_withdraw_notification
            as unsafe extern "C" fn(*mut GNotificationBackend, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GNotificationBackend, *const gchar) -> ()>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
