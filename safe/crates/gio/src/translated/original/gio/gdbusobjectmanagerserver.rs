use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GDBusConnection;
    pub type _GDBusMethodInvocation;
    pub type _GDBusInterface;
    pub type _GDBusInterfaceSkeletonPrivate;
    pub type _GDBusObject;
    pub type _GDBusObjectSkeletonPrivate;
    pub type _GDBusObjectManager;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_is_object_path(string: *const gchar) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_warn_message(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        warnexpr: *const ::core::ffi::c_char,
    );
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
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
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_get_object(value: *const GValue) -> gpointer;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_dbus_object_manager_get_type() -> GType;
    fn g_dbus_object_manager_get_object_path(manager: *mut GDBusObjectManager) -> *const gchar;
    fn g_dbus_object_manager_get_object(
        manager: *mut GDBusObjectManager,
        object_path: *const gchar,
    ) -> *mut GDBusObject;
    fn g_dbus_object_get_type() -> GType;
    fn g_dbus_object_get_object_path(object: *mut GDBusObject) -> *const gchar;
    fn g_dbus_object_get_interfaces(object: *mut GDBusObject) -> *mut GList;
    fn g_dbus_object_get_interface(
        object: *mut GDBusObject,
        interface_name: *const gchar,
    ) -> *mut GDBusInterface;
    fn g_dbus_object_skeleton_set_object_path(
        object: *mut GDBusObjectSkeleton,
        object_path: *const gchar,
    );
    fn g_dbus_interface_skeleton_get_info(
        interface_: *mut GDBusInterfaceSkeleton,
    ) -> *mut GDBusInterfaceInfo;
    fn g_dbus_interface_skeleton_get_properties(
        interface_: *mut GDBusInterfaceSkeleton,
    ) -> *mut GVariant;
    fn g_dbus_interface_skeleton_export(
        interface_: *mut GDBusInterfaceSkeleton,
        connection: *mut GDBusConnection,
        object_path: *const gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_dbus_interface_skeleton_unexport(interface_: *mut GDBusInterfaceSkeleton);
    fn g_dbus_interface_skeleton_get_connection(
        interface_: *mut GDBusInterfaceSkeleton,
    ) -> *mut GDBusConnection;
    fn g_dbus_connection_get_type() -> GType;
    fn g_dbus_connection_emit_signal(
        connection: *mut GDBusConnection,
        destination_bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        signal_name: *const gchar,
        parameters: *mut GVariant,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_dbus_connection_register_object(
        connection: *mut GDBusConnection,
        object_path: *const gchar,
        interface_info: *mut GDBusInterfaceInfo,
        vtable: *const GDBusInterfaceVTable,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
        error: *mut *mut GError,
    ) -> guint;
    fn g_dbus_connection_unregister_object(
        connection: *mut GDBusConnection,
        registration_id: guint,
    ) -> gboolean;
    fn g_dbus_method_invocation_return_value(
        invocation: *mut GDBusMethodInvocation,
        parameters: *mut GVariant,
    );
    fn g_dbus_method_invocation_return_error(
        invocation: *mut GDBusMethodInvocation,
        domain: GQuark,
        code: gint,
        format: *const gchar,
        ...
    );
    fn g_dbus_error_quark() -> GQuark;
    fn g_io_error_quark() -> GQuark;
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
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHashTableIter {
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: gpointer,
    pub dummy4: ::core::ffi::c_int,
    pub dummy5: gboolean,
    pub dummy6: gpointer,
}
pub type GHashTableIter = _GHashTableIter;
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_2 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_2 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_2 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_2 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_2 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_2 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_2 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_2 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_2 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_2 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_2 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_2 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_2 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_2 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_2 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_2 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_2 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_2 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_2 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_2 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_2 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_2 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_2 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_2 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_2 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_2 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_2 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_2 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_2 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_2 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_2 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_2 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_2 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_2 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_2 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_2 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_2 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_2 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_2 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_2 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_2 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_2 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_2 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_2 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_2 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_2 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_2 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_2 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_2 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const G_DBUS_ERROR_PROPERTY_READ_ONLY: C2RustUnnamed_3 = 44;
pub const G_DBUS_ERROR_UNKNOWN_PROPERTY: C2RustUnnamed_3 = 43;
pub const G_DBUS_ERROR_UNKNOWN_INTERFACE: C2RustUnnamed_3 = 42;
pub const G_DBUS_ERROR_UNKNOWN_OBJECT: C2RustUnnamed_3 = 41;
pub const G_DBUS_ERROR_OBJECT_PATH_IN_USE: C2RustUnnamed_3 = 40;
pub const G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN: C2RustUnnamed_3 = 39;
pub const G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN: C2RustUnnamed_3 = 38;
pub const G_DBUS_ERROR_INVALID_FILE_CONTENT: C2RustUnnamed_3 = 37;
pub const G_DBUS_ERROR_INVALID_SIGNATURE: C2RustUnnamed_3 = 36;
pub const G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN: C2RustUnnamed_3 = 35;
pub const G_DBUS_ERROR_SPAWN_NO_MEMORY: C2RustUnnamed_3 = 34;
pub const G_DBUS_ERROR_SPAWN_FILE_INVALID: C2RustUnnamed_3 = 33;
pub const G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID: C2RustUnnamed_3 = 32;
pub const G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND: C2RustUnnamed_3 = 31;
pub const G_DBUS_ERROR_SPAWN_SERVICE_INVALID: C2RustUnnamed_3 = 30;
pub const G_DBUS_ERROR_SPAWN_CONFIG_INVALID: C2RustUnnamed_3 = 29;
pub const G_DBUS_ERROR_SPAWN_SETUP_FAILED: C2RustUnnamed_3 = 28;
pub const G_DBUS_ERROR_SPAWN_FAILED: C2RustUnnamed_3 = 27;
pub const G_DBUS_ERROR_SPAWN_CHILD_SIGNALED: C2RustUnnamed_3 = 26;
pub const G_DBUS_ERROR_SPAWN_CHILD_EXITED: C2RustUnnamed_3 = 25;
pub const G_DBUS_ERROR_SPAWN_FORK_FAILED: C2RustUnnamed_3 = 24;
pub const G_DBUS_ERROR_SPAWN_EXEC_FAILED: C2RustUnnamed_3 = 23;
pub const G_DBUS_ERROR_MATCH_RULE_INVALID: C2RustUnnamed_3 = 22;
pub const G_DBUS_ERROR_MATCH_RULE_NOT_FOUND: C2RustUnnamed_3 = 21;
pub const G_DBUS_ERROR_TIMED_OUT: C2RustUnnamed_3 = 20;
pub const G_DBUS_ERROR_UNKNOWN_METHOD: C2RustUnnamed_3 = 19;
pub const G_DBUS_ERROR_FILE_EXISTS: C2RustUnnamed_3 = 18;
pub const G_DBUS_ERROR_FILE_NOT_FOUND: C2RustUnnamed_3 = 17;
pub const G_DBUS_ERROR_INVALID_ARGS: C2RustUnnamed_3 = 16;
pub const G_DBUS_ERROR_DISCONNECTED: C2RustUnnamed_3 = 15;
pub const G_DBUS_ERROR_ADDRESS_IN_USE: C2RustUnnamed_3 = 14;
pub const G_DBUS_ERROR_NO_NETWORK: C2RustUnnamed_3 = 13;
pub const G_DBUS_ERROR_TIMEOUT: C2RustUnnamed_3 = 12;
pub const G_DBUS_ERROR_NO_SERVER: C2RustUnnamed_3 = 11;
pub const G_DBUS_ERROR_AUTH_FAILED: C2RustUnnamed_3 = 10;
pub const G_DBUS_ERROR_ACCESS_DENIED: C2RustUnnamed_3 = 9;
pub const G_DBUS_ERROR_LIMITS_EXCEEDED: C2RustUnnamed_3 = 8;
pub const G_DBUS_ERROR_NOT_SUPPORTED: C2RustUnnamed_3 = 7;
pub const G_DBUS_ERROR_BAD_ADDRESS: C2RustUnnamed_3 = 6;
pub const G_DBUS_ERROR_IO_ERROR: C2RustUnnamed_3 = 5;
pub const G_DBUS_ERROR_NO_REPLY: C2RustUnnamed_3 = 4;
pub const G_DBUS_ERROR_NAME_HAS_NO_OWNER: C2RustUnnamed_3 = 3;
pub const G_DBUS_ERROR_SERVICE_UNKNOWN: C2RustUnnamed_3 = 2;
pub const G_DBUS_ERROR_NO_MEMORY: C2RustUnnamed_3 = 1;
pub const G_DBUS_ERROR_FAILED: C2RustUnnamed_3 = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GDBusConnection = _GDBusConnection;
pub type GDBusMethodInvocation = _GDBusMethodInvocation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceVTable {
    pub method_call: GDBusInterfaceMethodCallFunc,
    pub get_property: GDBusInterfaceGetPropertyFunc,
    pub set_property: GDBusInterfaceSetPropertyFunc,
    pub padding: [gpointer; 8],
}
pub type GDBusInterfaceSetPropertyFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        *mut *mut GError,
        gpointer,
    ) -> gboolean,
>;
pub type GDBusInterfaceGetPropertyFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut *mut GError,
        gpointer,
    ) -> *mut GVariant,
>;
pub type GDBusInterfaceMethodCallFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        *mut GDBusMethodInvocation,
        gpointer,
    ) -> (),
>;
pub type GDBusInterfaceVTable = _GDBusInterfaceVTable;
pub type GDBusInterfaceInfo = _GDBusInterfaceInfo;
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
pub type GDBusAnnotationInfo = _GDBusAnnotationInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAnnotationInfo {
    pub ref_count: gint,
    pub key: *mut gchar,
    pub value: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusPropertyInfo = _GDBusPropertyInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusPropertyInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub flags: GDBusPropertyInfoFlags,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusSignalInfo = _GDBusSignalInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusSignalInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusArgInfo = _GDBusArgInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusArgInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusMethodInfo = _GDBusMethodInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMethodInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub in_args: *mut *mut GDBusArgInfo,
    pub out_args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusInterface = _GDBusInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceSkeleton {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusInterfaceSkeletonPrivate,
}
pub type GDBusInterfaceSkeletonPrivate = _GDBusInterfaceSkeletonPrivate;
pub type GDBusInterfaceSkeleton = _GDBusInterfaceSkeleton;
pub type GDBusObject = _GDBusObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectSkeleton {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusObjectSkeletonPrivate,
}
pub type GDBusObjectSkeletonPrivate = _GDBusObjectSkeletonPrivate;
pub type GDBusObjectSkeleton = _GDBusObjectSkeleton;
pub type GDBusObjectManager = _GDBusObjectManager;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectManagerServer {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusObjectManagerServerPrivate,
}
pub type GDBusObjectManagerServerPrivate = _GDBusObjectManagerServerPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectManagerServerPrivate {
    pub lock: GMutex,
    pub connection: *mut GDBusConnection,
    pub object_path: *mut gchar,
    pub object_path_ending_in_slash: *mut gchar,
    pub map_object_path_to_data: *mut GHashTable,
    pub manager_reg_id: guint,
}
pub type GDBusObjectManagerServer = _GDBusObjectManagerServer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectManagerIface {
    pub parent_iface: GTypeInterface,
    pub get_object_path: Option<unsafe extern "C" fn(*mut GDBusObjectManager) -> *const gchar>,
    pub get_objects: Option<unsafe extern "C" fn(*mut GDBusObjectManager) -> *mut GList>,
    pub get_object:
        Option<unsafe extern "C" fn(*mut GDBusObjectManager, *const gchar) -> *mut GDBusObject>,
    pub get_interface: Option<
        unsafe extern "C" fn(
            *mut GDBusObjectManager,
            *const gchar,
            *const gchar,
        ) -> *mut GDBusInterface,
    >,
    pub object_added: Option<unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject) -> ()>,
    pub object_removed:
        Option<unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject) -> ()>,
    pub interface_added: Option<
        unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject, *mut GDBusInterface) -> (),
    >,
    pub interface_removed: Option<
        unsafe extern "C" fn(*mut GDBusObjectManager, *mut GDBusObject, *mut GDBusInterface) -> (),
    >,
}
pub type GDBusObjectManagerIface = _GDBusObjectManagerIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectManagerServerClass {
    pub parent_class: GObjectClass,
    pub padding: [gpointer; 8],
}
pub type GDBusObjectManagerServerClass = _GDBusObjectManagerServerClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RegistrationData {
    pub object: *mut GDBusObjectSkeleton,
    pub manager: *mut GDBusObjectManagerServer,
    pub map_iface_name_to_iface: *mut GHashTable,
    pub exported: gboolean,
}
pub const PROP_OBJECT_PATH: C2RustUnnamed_4 = 2;
pub const PROP_CONNECTION: C2RustUnnamed_4 = 1;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_4 = 0;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_object_manager_server_get_type_once();
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
static mut safe_c2rust_GDBusObjectManagerServer_private_offset: gint = 0;
static mut safe_c2rust_g_dbus_object_manager_server_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusObjectManagerServer\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusObjectManagerServerClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_object_manager_server_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusObjectManagerServer>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusObjectManagerServer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_object_manager_server_init
                    as unsafe extern "C" fn(*mut GDBusObjectManagerServer) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDBusObjectManagerServer_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDBusObjectManagerServerPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusObjectManagerIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_dbus_object_manager_interface_init
                as unsafe extern "C" fn(*mut GDBusObjectManagerIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_dbus_object_manager_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_get_instance_private(
    mut self_0: *mut GDBusObjectManagerServer,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDBusObjectManagerServer_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust_g_dbus_object_manager_server_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusObjectManagerServer_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusObjectManagerServer_private_offset,
        );
    }
    safe_c2rust_g_dbus_object_manager_server_class_init(
        klass as *mut GDBusObjectManagerServerClass,
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_finalize(mut object: *mut GObject) {
    let mut manager: *mut GDBusObjectManagerServer =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectManagerServer;
    if !(*(*manager).priv_0).connection.is_null() {
        safe_c2rust_unexport_all(manager, TRUE);
        g_object_unref((*(*manager).priv_0).connection as gpointer);
    }
    g_hash_table_unref((*(*manager).priv_0).map_object_path_to_data);
    g_free((*(*manager).priv_0).object_path as gpointer);
    g_free((*(*manager).priv_0).object_path_ending_in_slash as gpointer);
    g_mutex_clear(&raw mut (*(*manager).priv_0).lock);
    if (*(safe_c2rust_g_dbus_object_manager_server_parent_class as *mut GObjectClass))
        .finalize
        .is_some()
    {
        (*(safe_c2rust_g_dbus_object_manager_server_parent_class as *mut GObjectClass))
            .finalize
            .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut manager: *mut GDBusObjectManagerServer =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectManagerServer;
    match prop_id {
        1 => {
            g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
            g_value_set_object(value, (*(*manager).priv_0).connection as gpointer);
            g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
        }
        2 => {
            g_value_set_string(
                value,
                g_dbus_object_manager_get_object_path(
                    manager as *mut ::core::ffi::c_void as *mut GDBusObjectManager,
                ),
            );
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                158 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut manager: *mut GDBusObjectManagerServer =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectManagerServer;
    match prop_id {
        1 => {
            safe_c2rust_g_dbus_object_manager_server_set_connection(
                manager,
                g_value_get_object(value) as *mut GDBusConnection,
            );
        }
        2 => {
            if ({
                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                if (*(*manager).priv_0).object_path.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    178 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"manager->priv->object_path == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if g_variant_is_object_path(g_value_get_string(value)) != 0 {
                    _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_11
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    179 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_variant_is_object_path (g_value_get_string (value))\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*(*manager).priv_0).object_path = g_value_dup_string(value);
            if strcmp(
                (*(*manager).priv_0).object_path as *const ::core::ffi::c_char,
                b"/\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*(*manager).priv_0).object_path_ending_in_slash =
                    safe_c2rust_g_strdup_inline((*(*manager).priv_0).object_path) as *mut gchar;
            } else {
                (*(*manager).priv_0).object_path_ending_in_slash = g_strdup_printf(
                    b"%s/\0" as *const u8 as *const gchar,
                    (*(*manager).priv_0).object_path,
                );
            }
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_class_init(
    mut klass: *mut GDBusObjectManagerServerClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_dbus_object_manager_server_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).constructed = Some(
        safe_c2rust_g_dbus_object_manager_server_constructed
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dbus_object_manager_server_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dbus_object_manager_server_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_CONNECTION as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"connection\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_connection_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_OBJECT_PATH as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"object-path\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_init(
    mut manager: *mut GDBusObjectManagerServer,
) {
    (*manager).priv_0 = safe_c2rust_g_dbus_object_manager_server_get_instance_private(manager)
        as *mut GDBusObjectManagerServerPrivate;
    g_mutex_init(&raw mut (*(*manager).priv_0).lock);
    (*(*manager).priv_0).map_object_path_to_data = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut RegistrationData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_registration_data_free as unsafe extern "C" fn(*mut RegistrationData) -> (),
        )),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_new(
    mut object_path: *const gchar,
) -> *mut GDBusObjectManagerServer {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectManagerServer>();
    }
    return g_object_new(
        safe_c2rust_g_dbus_object_manager_server_get_type(),
        b"object-path\0" as *const u8 as *const gchar,
        object_path,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GDBusObjectManagerServer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_set_connection(
    mut manager: *mut GDBusObjectManagerServer,
    mut connection: *mut GDBusConnection,
) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_server_get_type();
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
            b"G_IS_DBUS_OBJECT_MANAGER_SERVER (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if connection.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
                let mut __t: GType = g_dbus_connection_get_type();
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
            b"connection == NULL || G_IS_DBUS_CONNECTION (connection)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    if (*(*manager).priv_0).connection == connection {
        g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    } else {
        if !(*(*manager).priv_0).connection.is_null() {
            safe_c2rust_unexport_all(manager, FALSE);
            g_object_unref((*(*manager).priv_0).connection as gpointer);
            (*(*manager).priv_0).connection = ::core::ptr::null_mut::<GDBusConnection>();
        }
        (*(*manager).priv_0).connection = (if !connection.is_null() {
            g_object_ref(connection as gpointer) as *mut GDBusConnection
        } else {
            ::core::ptr::null_mut::<GDBusConnection>()
        }) as *mut GDBusConnection;
        if !(*(*manager).priv_0).connection.is_null() {
            safe_c2rust_export_all(manager);
        }
        g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
        g_object_notify(
            manager as *mut ::core::ffi::c_void as *mut GObject,
            b"connection\0" as *const u8 as *const gchar,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_get_connection(
    mut manager: *mut GDBusObjectManagerServer,
) -> *mut GDBusConnection {
    let mut ret: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_server_get_type();
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
            b"G_IS_DBUS_OBJECT_MANAGER_SERVER (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    ret = (if !(*(*manager).priv_0).connection.is_null() {
        g_object_ref((*(*manager).priv_0).connection as gpointer) as *mut GDBusConnection
    } else {
        ::core::ptr::null_mut::<GDBusConnection>()
    }) as *mut GDBusConnection;
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_registration_data_export_interface(
    mut data: *mut RegistrationData,
    mut interface_skeleton: *mut GDBusInterfaceSkeleton,
    mut object_path: *const gchar,
) {
    let mut info: *mut GDBusInterfaceInfo = ::core::ptr::null_mut::<GDBusInterfaceInfo>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    info = g_dbus_interface_skeleton_get_info(interface_skeleton);
    error = ::core::ptr::null_mut::<GError>();
    if !(*(*(*data).manager).priv_0).connection.is_null() {
        if g_dbus_interface_skeleton_export(
            interface_skeleton,
            (*(*(*data).manager).priv_0).connection,
            object_path,
            &raw mut error,
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s: Error registering object at %s with interface %s: %s\0"
                    as *const u8 as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c:355\0"
                    as *const u8 as *const ::core::ffi::c_char,
                object_path,
                (*info).name,
                (*error).message,
            );
            g_error_free(error);
        }
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if g_hash_table_lookup(
            (*data).map_iface_name_to_iface,
            (*info).name as gconstpointer,
        )
        .is_null()
        {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            363 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_hash_table_lookup (data->map_iface_name_to_iface, info->name) == NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_insert(
        (*data).map_iface_name_to_iface,
        (*info).name as gpointer,
        g_object_ref(interface_skeleton as gpointer) as *mut GDBusInterfaceSkeleton as gpointer,
    );
    if (*data).exported != 0 {
        let mut interfaces: [*const gchar; 2] = [::core::ptr::null::<gchar>(); 2];
        interfaces[0 as ::core::ffi::c_int as usize] = (*info).name;
        interfaces[1 as ::core::ffi::c_int as usize] = ::core::ptr::null::<gchar>();
        safe_c2rust_g_dbus_object_manager_server_emit_interfaces_added(
            (*data).manager,
            data,
            &raw mut interfaces as *mut *const gchar,
            object_path,
        );
    }
}
unsafe extern "C" fn safe_c2rust_registration_data_unexport_interface(
    mut data: *mut RegistrationData,
    mut interface_skeleton: *mut GDBusInterfaceSkeleton,
) {
    let mut info: *mut GDBusInterfaceInfo = ::core::ptr::null_mut::<GDBusInterfaceInfo>();
    let mut iface: *mut GDBusInterfaceSkeleton = ::core::ptr::null_mut::<GDBusInterfaceSkeleton>();
    info = g_dbus_interface_skeleton_get_info(interface_skeleton);
    iface = g_hash_table_lookup(
        (*data).map_iface_name_to_iface,
        (*info).name as gconstpointer,
    ) as *mut GDBusInterfaceSkeleton;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !iface.is_null() {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            388 as ::core::ffi::c_int,
            G_STRFUNC,
            b"iface != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*(*(*data).manager).priv_0).connection.is_null() {
        g_dbus_interface_skeleton_unexport(iface);
    }
    if !(({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if g_hash_table_remove(
            (*data).map_iface_name_to_iface,
            (*info).name as gconstpointer,
        ) != 0
        {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            393 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_hash_table_remove (data->map_iface_name_to_iface, info->name)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*data).exported != 0 {
        let mut interfaces: [*const gchar; 2] = [::core::ptr::null::<gchar>(); 2];
        interfaces[0 as ::core::ffi::c_int as usize] = (*info).name;
        interfaces[1 as ::core::ffi::c_int as usize] = ::core::ptr::null::<gchar>();
        safe_c2rust_g_dbus_object_manager_server_emit_interfaces_removed(
            (*data).manager,
            data,
            &raw mut interfaces as *mut *const gchar,
        );
    }
}
unsafe extern "C" fn safe_c2rust_on_interface_added(
    mut object: *mut GDBusObject,
    mut interface: *mut GDBusInterface,
    mut user_data: gpointer,
) {
    let mut data: *mut RegistrationData = user_data as *mut RegistrationData;
    let mut object_path: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut (*(*(*data).manager).priv_0).lock);
    object_path = g_dbus_object_get_object_path(
        (*data).object as *mut ::core::ffi::c_void as *mut GDBusObject,
    );
    safe_c2rust_registration_data_export_interface(
        data,
        interface as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        object_path,
    );
    g_mutex_unlock(&raw mut (*(*(*data).manager).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust_on_interface_removed(
    mut object: *mut GDBusObject,
    mut interface: *mut GDBusInterface,
    mut user_data: gpointer,
) {
    let mut data: *mut RegistrationData = user_data as *mut RegistrationData;
    g_mutex_lock(&raw mut (*(*(*data).manager).priv_0).lock);
    safe_c2rust_registration_data_unexport_interface(
        data,
        interface as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
    );
    g_mutex_unlock(&raw mut (*(*(*data).manager).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust_registration_data_free(mut data: *mut RegistrationData) {
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut iface: *mut GDBusInterfaceSkeleton = ::core::ptr::null_mut::<GDBusInterfaceSkeleton>();
    (*data).exported = FALSE as gboolean;
    g_hash_table_iter_init(&raw mut iter, (*data).map_iface_name_to_iface);
    while g_hash_table_iter_next(
        &raw mut iter,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut iface as gpointer as *mut gpointer,
    ) != 0
    {
        if !(*(*(*data).manager).priv_0).connection.is_null() {
            g_dbus_interface_skeleton_unexport(iface);
        }
    }
    g_signal_handlers_disconnect_matched(
        (*data).object as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<GCallback, gpointer>(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusObject, *mut GDBusInterface, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_on_interface_added
                as unsafe extern "C" fn(*mut GDBusObject, *mut GDBusInterface, gpointer) -> (),
        ))),
        data as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*data).object as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<GCallback, gpointer>(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusObject, *mut GDBusInterface, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_on_interface_removed
                as unsafe extern "C" fn(*mut GDBusObject, *mut GDBusInterface, gpointer) -> (),
        ))),
        data as gpointer,
    );
    g_object_unref((*data).object as gpointer);
    g_hash_table_destroy((*data).map_iface_name_to_iface);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_is_valid_child_object_path(
    mut manager: *mut GDBusObjectManagerServer,
    mut child_object_path: *const gchar,
) -> gboolean {
    if !(({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !(strcmp(
            child_object_path as *const ::core::ffi::c_char,
            (*(*manager).priv_0).object_path_ending_in_slash as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            480 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!g_str_equal (child_object_path, manager->priv->object_path_ending_in_slash)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = child_object_path as *const ::core::ffi::c_char;
            let __prefix: *const ::core::ffi::c_char =
                (*(*manager).priv_0).object_path_ending_in_slash;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_20 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_20 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_20
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
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
        g_str_has_prefix(
            child_object_path,
            (*(*manager).priv_0).object_path_ending_in_slash,
        )
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_export_unlocked(
    mut manager: *mut GDBusObjectManagerServer,
    mut object: *mut GDBusObjectSkeleton,
    mut object_path: *const gchar,
) {
    let mut data: *mut RegistrationData = ::core::ptr::null_mut::<RegistrationData>();
    let mut existing_interfaces: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut interface_names: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_server_get_type();
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
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_OBJECT_MANAGER_SERVER (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = g_dbus_object_get_type();
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
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if safe_c2rust_is_valid_child_object_path(manager, object_path) != 0 {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_child_object_path (manager, object_path)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    interface_names = g_ptr_array_new();
    data = g_hash_table_lookup(
        (*(*manager).priv_0).map_object_path_to_data,
        object_path as gconstpointer,
    ) as *mut RegistrationData;
    if !data.is_null() {
        safe_c2rust_g_dbus_object_manager_server_unexport_unlocked(manager, object_path);
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<RegistrationData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut RegistrationData;
    (*data).object =
        g_object_ref(object as gpointer) as *mut GDBusObjectSkeleton as *mut GDBusObjectSkeleton;
    (*data).manager = manager;
    (*data).map_iface_name_to_iface = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        ),
    );
    g_signal_connect_data(
        object as gpointer,
        b"interface-added\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusObject, *mut GDBusInterface, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_on_interface_added
                as unsafe extern "C" fn(*mut GDBusObject, *mut GDBusInterface, gpointer) -> (),
        )),
        data as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        object as gpointer,
        b"interface-removed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusObject, *mut GDBusInterface, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_on_interface_removed
                as unsafe extern "C" fn(*mut GDBusObject, *mut GDBusInterface, gpointer) -> (),
        )),
        data as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    existing_interfaces =
        g_dbus_object_get_interfaces(object as *mut ::core::ffi::c_void as *mut GDBusObject);
    l = existing_interfaces;
    while !l.is_null() {
        let mut interface_skeleton: *mut GDBusInterfaceSkeleton =
            (*l).data as *mut GDBusInterfaceSkeleton;
        safe_c2rust_registration_data_export_interface(data, interface_skeleton, object_path);
        g_ptr_array_add(
            interface_names,
            (*g_dbus_interface_skeleton_get_info(interface_skeleton)).name as gpointer,
        );
        l = (*l).next;
    }
    g_list_free_full(
        existing_interfaces,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_ptr_array_add(interface_names, NULL_0);
    (*data).exported = TRUE as gboolean;
    safe_c2rust_g_dbus_object_manager_server_emit_interfaces_added(
        manager,
        data,
        (*interface_names).pdata as *const *const gchar,
        object_path,
    );
    g_ptr_array_unref(interface_names);
    g_hash_table_insert(
        (*(*manager).priv_0).map_object_path_to_data,
        safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as gpointer,
        data as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_export(
    mut manager: *mut GDBusObjectManagerServer,
    mut object: *mut GDBusObjectSkeleton,
) {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_server_get_type();
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
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_OBJECT_MANAGER_SERVER (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    safe_c2rust_g_dbus_object_manager_server_export_unlocked(
        manager,
        object,
        g_dbus_object_get_object_path(object as *mut ::core::ffi::c_void as *mut GDBusObject),
    );
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_export_uniquely(
    mut manager: *mut GDBusObjectManagerServer,
    mut object: *mut GDBusObjectSkeleton,
) {
    let mut orig_object_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut object_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut count: guint = 0;
    let mut modified: gboolean = 0;
    orig_object_path =
        g_dbus_object_get_object_path(object as *mut ::core::ffi::c_void as *mut GDBusObject);
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_server_get_type();
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
            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_25
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_OBJECT_MANAGER_SERVER (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = g_dbus_object_get_type();
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
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if safe_c2rust_is_valid_child_object_path(manager, orig_object_path) != 0 {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_child_object_path (manager, orig_object_path)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    object_path =
        safe_c2rust_g_strdup_inline(orig_object_path as *const ::core::ffi::c_char) as *mut gchar;
    count = 1 as guint;
    modified = FALSE as gboolean;
    while FALSE == 0 {
        let mut data: *mut RegistrationData = ::core::ptr::null_mut::<RegistrationData>();
        data = g_hash_table_lookup(
            (*(*manager).priv_0).map_object_path_to_data,
            object_path as gconstpointer,
        ) as *mut RegistrationData;
        if data.is_null() {
            break;
        }
        g_free(object_path as gpointer);
        let fresh0 = count;
        count = count.wrapping_add(1);
        object_path = g_strdup_printf(
            b"%s_%d\0" as *const u8 as *const gchar,
            orig_object_path,
            fresh0,
        );
        modified = TRUE as gboolean;
    }
    safe_c2rust_g_dbus_object_manager_server_export_unlocked(manager, object, object_path);
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    if modified != 0 {
        g_dbus_object_skeleton_set_object_path(
            object as *mut ::core::ffi::c_void as *mut GDBusObjectSkeleton,
            object_path,
        );
    }
    g_free(object_path as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_is_exported(
    mut manager: *mut GDBusObjectManagerServer,
    mut object: *mut GDBusObjectSkeleton,
) -> gboolean {
    let mut data: *mut RegistrationData = ::core::ptr::null_mut::<RegistrationData>();
    let mut object_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut object_is_exported: gboolean = 0;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_server_get_type();
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
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_OBJECT_MANAGER_SERVER (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = g_dbus_object_get_type();
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
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    object_path =
        g_dbus_object_get_object_path(object as *mut ::core::ffi::c_void as *mut GDBusObject);
    if !object_path.is_null() {
        data = g_hash_table_lookup(
            (*(*manager).priv_0).map_object_path_to_data,
            object_path as gconstpointer,
        ) as *mut RegistrationData;
    }
    object_is_exported =
        (data != NULL_0 as *mut RegistrationData) as ::core::ffi::c_int as gboolean;
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return object_is_exported;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_unexport_unlocked(
    mut manager: *mut GDBusObjectManagerServer,
    mut object_path: *const gchar,
) -> gboolean {
    let mut data: *mut RegistrationData = ::core::ptr::null_mut::<RegistrationData>();
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_server_get_type();
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
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_OBJECT_MANAGER_SERVER (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if safe_c2rust_is_valid_child_object_path(manager, object_path) != 0 {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_child_object_path (manager, object_path)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    data = g_hash_table_lookup(
        (*(*manager).priv_0).map_object_path_to_data,
        object_path as gconstpointer,
    ) as *mut RegistrationData;
    if !data.is_null() {
        let mut interface_names: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
        let mut iter: GHashTableIter = _GHashTableIter {
            dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy4: 0,
            dummy5: 0,
            dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        let mut iface_name: *const gchar = ::core::ptr::null::<gchar>();
        interface_names = g_ptr_array_new();
        g_hash_table_iter_init(&raw mut iter, (*data).map_iface_name_to_iface);
        while g_hash_table_iter_next(
            &raw mut iter,
            &raw mut iface_name as gpointer as *mut gpointer,
            ::core::ptr::null_mut::<gpointer>(),
        ) != 0
        {
            g_ptr_array_add(interface_names, iface_name as gpointer);
        }
        g_ptr_array_add(interface_names, NULL_0);
        safe_c2rust_g_dbus_object_manager_server_emit_interfaces_removed(
            manager,
            data,
            (*interface_names).pdata as *const *const gchar,
        );
        g_ptr_array_unref(interface_names);
        g_hash_table_remove(
            (*(*manager).priv_0).map_object_path_to_data,
            object_path as gconstpointer,
        );
        ret = TRUE as gboolean;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_unexport(
    mut manager: *mut GDBusObjectManagerServer,
    mut object_path: *const gchar,
) -> gboolean {
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = manager as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_manager_server_get_type();
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
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_OBJECT_MANAGER_SERVER (manager)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    ret = safe_c2rust_g_dbus_object_manager_server_unexport_unlocked(manager, object_path);
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return ret;
}
static mut safe_c2rust_manager_interfaces_added_signal_info_arg0: GDBusArgInfo = _GDBusArgInfo {
    ref_count: -(1 as gint),
    name: b"object_path\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
    signature: b"o\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
    annotations: NULL_0 as *mut *mut GDBusAnnotationInfo,
};
static mut safe_c2rust_manager_interfaces_added_signal_info_arg1: GDBusArgInfo = _GDBusArgInfo {
    ref_count: -(1 as gint),
    name: b"interfaces_and_properties\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
    signature: b"a{sa{sv}}\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
    annotations: NULL_0 as *mut *mut GDBusAnnotationInfo,
};
static mut safe_c2rust_manager_interfaces_added_signal_info_arg_pointers: [*const GDBusArgInfo; 3] = unsafe {
    [
        &raw const safe_c2rust_manager_interfaces_added_signal_info_arg0,
        &raw const safe_c2rust_manager_interfaces_added_signal_info_arg1,
        ::core::ptr::null::<GDBusArgInfo>(),
    ]
};
static mut safe_c2rust_manager_interfaces_added_signal_info: GDBusSignalInfo = unsafe {
    _GDBusSignalInfo {
        ref_count: -(1 as gint),
        name: b"InterfacesAdded\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        args: &raw const safe_c2rust_manager_interfaces_added_signal_info_arg_pointers
            as *mut *mut GDBusArgInfo,
        annotations: NULL_0 as *mut *mut GDBusAnnotationInfo,
    }
};
static mut safe_c2rust_manager_interfaces_removed_signal_info_arg0: GDBusArgInfo = _GDBusArgInfo {
    ref_count: -(1 as gint),
    name: b"object_path\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
    signature: b"o\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
    annotations: NULL_0 as *mut *mut GDBusAnnotationInfo,
};
static mut safe_c2rust_manager_interfaces_removed_signal_info_arg1: GDBusArgInfo = _GDBusArgInfo {
    ref_count: -(1 as gint),
    name: b"interfaces\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
    signature: b"as\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
    annotations: NULL_0 as *mut *mut GDBusAnnotationInfo,
};
static mut safe_c2rust_manager_interfaces_removed_signal_info_arg_pointers: [*const GDBusArgInfo;
    3] = unsafe {
    [
        &raw const safe_c2rust_manager_interfaces_removed_signal_info_arg0,
        &raw const safe_c2rust_manager_interfaces_removed_signal_info_arg1,
        ::core::ptr::null::<GDBusArgInfo>(),
    ]
};
static mut safe_c2rust_manager_interfaces_removed_signal_info: GDBusSignalInfo = unsafe {
    _GDBusSignalInfo {
        ref_count: -(1 as gint),
        name: b"InterfacesRemoved\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        args: &raw const safe_c2rust_manager_interfaces_removed_signal_info_arg_pointers
            as *mut *mut GDBusArgInfo,
        annotations: NULL_0 as *mut *mut GDBusAnnotationInfo,
    }
};
static mut safe_c2rust_manager_signal_info_pointers: [*const GDBusSignalInfo; 3] = unsafe {
    [
        &raw const safe_c2rust_manager_interfaces_added_signal_info,
        &raw const safe_c2rust_manager_interfaces_removed_signal_info,
        ::core::ptr::null::<GDBusSignalInfo>(),
    ]
};
static mut safe_c2rust_manager_get_all_method_info_out_arg0: GDBusArgInfo = _GDBusArgInfo {
    ref_count: -(1 as gint),
    name: b"object_paths_interfaces_and_properties\0" as *const u8 as *const ::core::ffi::c_char
        as *mut gchar,
    signature: b"a{oa{sa{sv}}}\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
    annotations: NULL_0 as *mut *mut GDBusAnnotationInfo,
};
static mut safe_c2rust_manager_get_all_method_info_out_arg_pointers: [*const GDBusArgInfo; 2] = unsafe {
    [
        &raw const safe_c2rust_manager_get_all_method_info_out_arg0,
        ::core::ptr::null::<GDBusArgInfo>(),
    ]
};
static mut safe_c2rust_manager_get_all_method_info: GDBusMethodInfo = unsafe {
    _GDBusMethodInfo {
        ref_count: -(1 as gint),
        name: b"GetManagedObjects\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar,
        in_args: NULL_0 as *mut *mut GDBusArgInfo,
        out_args: &raw const safe_c2rust_manager_get_all_method_info_out_arg_pointers
            as *mut *mut GDBusArgInfo,
        annotations: NULL_0 as *mut *mut GDBusAnnotationInfo,
    }
};
static mut safe_c2rust_manager_method_info_pointers: [*const GDBusMethodInfo; 2] = unsafe {
    [
        &raw const safe_c2rust_manager_get_all_method_info,
        ::core::ptr::null::<GDBusMethodInfo>(),
    ]
};
static mut safe_c2rust_manager_interface_info: GDBusInterfaceInfo = unsafe {
    _GDBusInterfaceInfo {
        ref_count: -(1 as gint),
        name: b"org.freedesktop.DBus.ObjectManager\0" as *const u8 as *const ::core::ffi::c_char
            as *mut gchar,
        methods: &raw const safe_c2rust_manager_method_info_pointers
            as *const *const GDBusMethodInfo as *mut *mut GDBusMethodInfo,
        signals: &raw const safe_c2rust_manager_signal_info_pointers
            as *const *const GDBusSignalInfo as *mut *mut GDBusSignalInfo,
        properties: NULL_0 as *mut *mut GDBusPropertyInfo,
        annotations: NULL_0 as *mut *mut GDBusAnnotationInfo,
    }
};
unsafe extern "C" fn safe_c2rust_manager_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut manager: *mut GDBusObjectManagerServer = user_data as *mut GDBusObjectManagerServer;
    let mut array_builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut object_iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut data: *mut RegistrationData = ::core::ptr::null_mut::<RegistrationData>();
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    if g_strcmp0(
        method_name as *const ::core::ffi::c_char,
        b"GetManagedObjects\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        g_variant_builder_init(
            &raw mut array_builder,
            g_variant_type_checked_(b"a{oa{sa{sv}}}\0" as *const u8 as *const gchar),
        );
        g_hash_table_iter_init(
            &raw mut object_iter,
            (*(*manager).priv_0).map_object_path_to_data,
        );
        while g_hash_table_iter_next(
            &raw mut object_iter,
            ::core::ptr::null_mut::<gpointer>(),
            &raw mut data as gpointer as *mut gpointer,
        ) != 0
        {
            let mut interfaces_builder: GVariantBuilder = _GVariantBuilder {
                u: C2RustUnnamed {
                    s: C2RustUnnamed_0 {
                        partial_magic: 0,
                        type_0: ::core::ptr::null::<GVariantType>(),
                        y: [0; 14],
                    },
                },
            };
            let mut interface_iter: GHashTableIter = _GHashTableIter {
                dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                dummy4: 0,
                dummy5: 0,
                dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            };
            let mut iface: *mut GDBusInterfaceSkeleton =
                ::core::ptr::null_mut::<GDBusInterfaceSkeleton>();
            let mut iter_object_path: *const gchar = ::core::ptr::null::<gchar>();
            g_variant_builder_init(
                &raw mut interfaces_builder,
                g_variant_type_checked_(b"a{sa{sv}}\0" as *const u8 as *const gchar),
            );
            g_hash_table_iter_init(&raw mut interface_iter, (*data).map_iface_name_to_iface);
            while g_hash_table_iter_next(
                &raw mut interface_iter,
                ::core::ptr::null_mut::<gpointer>(),
                &raw mut iface as gpointer as *mut gpointer,
            ) != 0
            {
                let mut properties: *mut GVariant = g_dbus_interface_skeleton_get_properties(iface);
                g_variant_builder_add(
                    &raw mut interfaces_builder,
                    b"{s@a{sv}}\0" as *const u8 as *const gchar,
                    (*g_dbus_interface_skeleton_get_info(iface)).name,
                    properties,
                );
                g_variant_unref(properties);
            }
            iter_object_path = g_dbus_object_get_object_path(
                (*data).object as *mut ::core::ffi::c_void as *mut GDBusObject,
            );
            g_variant_builder_add(
                &raw mut array_builder,
                b"{oa{sa{sv}}}\0" as *const u8 as *const gchar,
                iter_object_path,
                &raw mut interfaces_builder,
            );
        }
        g_dbus_method_invocation_return_value(
            invocation,
            g_variant_new(
                b"(a{oa{sa{sv}}})\0" as *const u8 as *const gchar,
                &raw mut array_builder,
            ),
        );
    } else {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
            b"Unknown method %s - only GetManagedObjects() is supported\0" as *const u8
                as *const gchar,
            method_name,
        );
    }
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
}
static mut safe_c2rust_manager_interface_vtable: GDBusInterfaceVTable = unsafe {
    _GDBusInterfaceVTable {
        method_call: Some(
            safe_c2rust_manager_method_call
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut GDBusMethodInvocation,
                    gpointer,
                ) -> (),
        ),
        get_property: None,
        set_property: None,
        padding: [
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        ],
    }
};
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_constructed(
    mut object: *mut GObject,
) {
    let mut manager: *mut GDBusObjectManagerServer =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectManagerServer;
    if !(*(*manager).priv_0).connection.is_null() {
        safe_c2rust_export_all(manager);
    }
    if (*(safe_c2rust_g_dbus_object_manager_server_parent_class as *mut GObjectClass))
        .constructed
        .is_some()
    {
        (*(safe_c2rust_g_dbus_object_manager_server_parent_class as *mut GObjectClass))
            .constructed
            .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_emit_interfaces_added(
    mut manager: *mut GDBusObjectManagerServer,
    mut data: *mut RegistrationData,
    mut interfaces: *const *const gchar,
    mut object_path: *const gchar,
) {
    let mut array_builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut n: guint = 0;
    if !(*(*(*data).manager).priv_0).connection.is_null() {
        g_variant_builder_init(
            &raw mut array_builder,
            g_variant_type_checked_(b"a{sa{sv}}\0" as *const u8 as *const gchar),
        );
        n = 0 as guint;
        while !(*interfaces.offset(n as isize)).is_null() {
            let mut iface: *mut GDBusInterfaceSkeleton =
                ::core::ptr::null_mut::<GDBusInterfaceSkeleton>();
            let mut properties: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            iface = g_hash_table_lookup(
                (*data).map_iface_name_to_iface,
                *interfaces.offset(n as isize) as gconstpointer,
            ) as *mut GDBusInterfaceSkeleton;
            if ({
                let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
                if !iface.is_null() {
                    _g_boolean_var_34 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_34 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_34
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    953 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"iface != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            properties = g_dbus_interface_skeleton_get_properties(iface);
            g_variant_builder_add(
                &raw mut array_builder,
                b"{s@a{sv}}\0" as *const u8 as *const gchar,
                *interfaces.offset(n as isize),
                properties,
            );
            g_variant_unref(properties);
            n = n.wrapping_add(1);
        }
        error = ::core::ptr::null_mut::<GError>();
        g_dbus_connection_emit_signal(
            (*(*(*data).manager).priv_0).connection,
            ::core::ptr::null::<gchar>(),
            (*(*manager).priv_0).object_path,
            safe_c2rust_manager_interface_info.name,
            b"InterfacesAdded\0" as *const u8 as *const gchar,
            g_variant_new(
                b"(oa{sa{sv}})\0" as *const u8 as *const gchar,
                object_path,
                &raw mut array_builder,
            ),
            &raw mut error,
        );
        if !error.is_null() {
            if g_error_matches(
                error,
                g_io_error_quark(),
                G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            ) == 0
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Couldn't emit InterfacesAdded signal: %s\0" as *const u8 as *const gchar,
                    (*error).message,
                );
            }
            g_error_free(error);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_emit_interfaces_removed(
    mut manager: *mut GDBusObjectManagerServer,
    mut data: *mut RegistrationData,
    mut interfaces: *const *const gchar,
) {
    let mut array_builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut n: guint = 0;
    let mut object_path: *const gchar = ::core::ptr::null::<gchar>();
    if !(*(*(*data).manager).priv_0).connection.is_null() {
        g_variant_builder_init(
            &raw mut array_builder,
            g_variant_type_checked_(b"as\0" as *const u8 as *const gchar),
        );
        n = 0 as guint;
        while !(*interfaces.offset(n as isize)).is_null() {
            g_variant_builder_add(
                &raw mut array_builder,
                b"s\0" as *const u8 as *const gchar,
                *interfaces.offset(n as isize),
            );
            n = n.wrapping_add(1);
        }
        error = ::core::ptr::null_mut::<GError>();
        object_path = g_dbus_object_get_object_path(
            (*data).object as *mut ::core::ffi::c_void as *mut GDBusObject,
        );
        g_dbus_connection_emit_signal(
            (*(*(*data).manager).priv_0).connection,
            ::core::ptr::null::<gchar>(),
            (*(*manager).priv_0).object_path,
            safe_c2rust_manager_interface_info.name,
            b"InterfacesRemoved\0" as *const u8 as *const gchar,
            g_variant_new(
                b"(oas)\0" as *const u8 as *const gchar,
                object_path,
                &raw mut array_builder,
            ),
            &raw mut error,
        );
        if !error.is_null() {
            if g_error_matches(
                error,
                g_io_error_quark(),
                G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            ) == 0
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Couldn't emit InterfacesRemoved signal: %s\0" as *const u8 as *const gchar,
                    (*error).message,
                );
            }
            g_error_free(error);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_get_objects(
    mut _manager: *mut GDBusObjectManager,
) -> *mut GList {
    let mut manager: *mut GDBusObjectManagerServer =
        _manager as *mut ::core::ffi::c_void as *mut GDBusObjectManagerServer;
    let mut ret: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut data: *mut RegistrationData = ::core::ptr::null_mut::<RegistrationData>();
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    ret = ::core::ptr::null_mut::<GList>();
    g_hash_table_iter_init(&raw mut iter, (*(*manager).priv_0).map_object_path_to_data);
    while g_hash_table_iter_next(
        &raw mut iter,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut data as gpointer as *mut gpointer,
    ) != 0
    {
        ret = g_list_prepend(
            ret,
            g_object_ref((*data).object as gpointer) as *mut GDBusObjectSkeleton as gpointer,
        );
    }
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_get_object_path(
    mut _manager: *mut GDBusObjectManager,
) -> *const gchar {
    let mut manager: *mut GDBusObjectManagerServer =
        _manager as *mut ::core::ffi::c_void as *mut GDBusObjectManagerServer;
    return (*(*manager).priv_0).object_path;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_get_object(
    mut _manager: *mut GDBusObjectManager,
    mut object_path: *const gchar,
) -> *mut GDBusObject {
    let mut manager: *mut GDBusObjectManagerServer =
        _manager as *mut ::core::ffi::c_void as *mut GDBusObjectManagerServer;
    let mut ret: *mut GDBusObject = ::core::ptr::null_mut::<GDBusObject>();
    let mut data: *mut RegistrationData = ::core::ptr::null_mut::<RegistrationData>();
    ret = ::core::ptr::null_mut::<GDBusObject>();
    g_mutex_lock(&raw mut (*(*manager).priv_0).lock);
    data = g_hash_table_lookup(
        (*(*manager).priv_0).map_object_path_to_data,
        object_path as gconstpointer,
    ) as *mut RegistrationData;
    if !data.is_null() {
        ret = g_object_ref(
            (*data).object as *mut ::core::ffi::c_void as *mut GDBusObject as gpointer,
        ) as *mut GDBusObject as *mut GDBusObject;
    }
    g_mutex_unlock(&raw mut (*(*manager).priv_0).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_manager_server_get_interface(
    mut _manager: *mut GDBusObjectManager,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
) -> *mut GDBusInterface {
    let mut ret: *mut GDBusInterface = ::core::ptr::null_mut::<GDBusInterface>();
    let mut object: *mut GDBusObject = ::core::ptr::null_mut::<GDBusObject>();
    ret = ::core::ptr::null_mut::<GDBusInterface>();
    object = g_dbus_object_manager_get_object(_manager, object_path);
    if !object.is_null() {
        ret = g_dbus_object_get_interface(object, interface_name);
        g_object_unref(object as gpointer);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_dbus_object_manager_interface_init(
    mut iface: *mut GDBusObjectManagerIface,
) {
    (*iface).get_object_path = Some(
        safe_c2rust_g_dbus_object_manager_server_get_object_path
            as unsafe extern "C" fn(*mut GDBusObjectManager) -> *const gchar,
    )
        as Option<unsafe extern "C" fn(*mut GDBusObjectManager) -> *const gchar>;
    (*iface).get_objects = Some(
        safe_c2rust_g_dbus_object_manager_server_get_objects
            as unsafe extern "C" fn(*mut GDBusObjectManager) -> *mut GList,
    )
        as Option<unsafe extern "C" fn(*mut GDBusObjectManager) -> *mut GList>;
    (*iface).get_object = Some(
        safe_c2rust_g_dbus_object_manager_server_get_object
            as unsafe extern "C" fn(*mut GDBusObjectManager, *const gchar) -> *mut GDBusObject,
    )
        as Option<unsafe extern "C" fn(*mut GDBusObjectManager, *const gchar) -> *mut GDBusObject>;
    (*iface).get_interface = Some(
        safe_c2rust_g_dbus_object_manager_server_get_interface
            as unsafe extern "C" fn(
                *mut GDBusObjectManager,
                *const gchar,
                *const gchar,
            ) -> *mut GDBusInterface,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDBusObjectManager,
                *const gchar,
                *const gchar,
            ) -> *mut GDBusInterface,
        >;
}
unsafe extern "C" fn safe_c2rust_export_all(mut manager: *mut GDBusObjectManagerServer) {
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut object_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut data: *mut RegistrationData = ::core::ptr::null_mut::<RegistrationData>();
    let mut iface_iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut iface: *mut GDBusInterfaceSkeleton = ::core::ptr::null_mut::<GDBusInterfaceSkeleton>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !(*(*manager).priv_0).connection.is_null() {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"manager->priv->connection != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    error = ::core::ptr::null_mut::<GError>();
    if !(({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if (*(*manager).priv_0).manager_reg_id == 0 as guint {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1112 as ::core::ffi::c_int,
            G_STRFUNC,
            b"manager->priv->manager_reg_id == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*(*manager).priv_0).manager_reg_id = g_dbus_connection_register_object(
        (*(*manager).priv_0).connection,
        (*(*manager).priv_0).object_path,
        &raw const safe_c2rust_manager_interface_info as *mut GDBusInterfaceInfo,
        &raw const safe_c2rust_manager_interface_vtable,
        manager as gpointer,
        None,
        &raw mut error,
    );
    if (*(*manager).priv_0).manager_reg_id == 0 as guint {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: Error registering manager at %s: %s\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c:1123\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*(*manager).priv_0).object_path,
            (*error).message,
        );
        g_error_free(error);
    }
    g_hash_table_iter_init(&raw mut iter, (*(*manager).priv_0).map_object_path_to_data);
    while g_hash_table_iter_next(
        &raw mut iter,
        &raw mut object_path as gpointer as *mut gpointer,
        &raw mut data as gpointer as *mut gpointer,
    ) != 0
    {
        g_hash_table_iter_init(&raw mut iface_iter, (*data).map_iface_name_to_iface);
        while g_hash_table_iter_next(
            &raw mut iface_iter,
            ::core::ptr::null_mut::<gpointer>(),
            &raw mut iface as gpointer as *mut gpointer,
        ) != 0
        {
            if !(({
                let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
                if g_dbus_interface_skeleton_get_connection(iface).is_null() {
                    _g_boolean_var_37 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_37 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_37
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1135 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_dbus_interface_skeleton_get_connection (iface) == NULL\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            error = ::core::ptr::null_mut::<GError>();
            if g_dbus_interface_skeleton_export(
                iface,
                (*(*manager).priv_0).connection,
                object_path,
                &raw mut error,
            ) == 0
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"%s: Error registering object at %s with interface %s: %s\0"
                        as *const u8 as *const gchar,
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c:1143\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    object_path,
                    (*g_dbus_interface_skeleton_get_info(iface)).name,
                    (*error).message,
                );
                g_error_free(error);
            }
        }
    }
}
unsafe extern "C" fn safe_c2rust_unexport_all(
    mut manager: *mut GDBusObjectManagerServer,
    mut only_manager: gboolean,
) {
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut data: *mut RegistrationData = ::core::ptr::null_mut::<RegistrationData>();
    let mut iface_iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut iface: *mut GDBusInterfaceSkeleton = ::core::ptr::null_mut::<GDBusInterfaceSkeleton>();
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !(*(*manager).priv_0).connection.is_null() {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"manager->priv->connection != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if (*(*manager).priv_0).manager_reg_id > 0 as guint {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1163 as ::core::ffi::c_int,
            G_STRFUNC,
            b"manager->priv->manager_reg_id > 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*(*manager).priv_0).manager_reg_id > 0 as guint {
        if !(({
            let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
            if g_dbus_connection_unregister_object(
                (*(*manager).priv_0).connection,
                (*(*manager).priv_0).manager_reg_id,
            ) != 0
            {
                _g_boolean_var_40 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_40 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_40
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1167 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_dbus_connection_unregister_object (manager->priv->connection, manager->priv->manager_reg_id)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*(*manager).priv_0).manager_reg_id = 0 as guint;
    }
    if !(only_manager != 0) {
        g_hash_table_iter_init(&raw mut iter, (*(*manager).priv_0).map_object_path_to_data);
        while g_hash_table_iter_next(
            &raw mut iter,
            ::core::ptr::null_mut::<gpointer>(),
            &raw mut data as gpointer as *mut gpointer,
        ) != 0
        {
            g_hash_table_iter_init(&raw mut iface_iter, (*data).map_iface_name_to_iface);
            while g_hash_table_iter_next(
                &raw mut iface_iter,
                ::core::ptr::null_mut::<gpointer>(),
                &raw mut iface as gpointer as *mut gpointer,
            ) != 0
            {
                if !(({
                    let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
                    if !g_dbus_interface_skeleton_get_connection(iface).is_null() {
                        _g_boolean_var_41 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_41 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_41
                }) as ::core::ffi::c_long
                    != 0)
                {
                    g_warn_message(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectmanagerserver.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1179 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"g_dbus_interface_skeleton_get_connection (iface) != NULL\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                g_dbus_interface_skeleton_unexport(iface);
            }
        }
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
