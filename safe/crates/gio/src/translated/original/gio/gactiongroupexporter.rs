use ::c2rust_bitfields;
extern "C" {
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GRemoteActionGroup;
    pub type _GActionGroup;
    pub type _GDBusConnection;
    pub type _GDBusMethodInvocation;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_remove_all(hash_table: *mut GHashTable);
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_variant_type_dup_string(type_0: *const GVariantType) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_iter_free(iter: *mut GVariantIter);
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_open(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_close(builder: *mut GVariantBuilder);
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
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
    fn g_remote_action_group_get_type() -> GType;
    fn g_remote_action_group_activate_action_full(
        remote: *mut GRemoteActionGroup,
        action_name: *const gchar,
        parameter: *mut GVariant,
        platform_data: *mut GVariant,
    );
    fn g_remote_action_group_change_action_state_full(
        remote: *mut GRemoteActionGroup,
        action_name: *const gchar,
        value: *mut GVariant,
        platform_data: *mut GVariant,
    );
    fn g_dbus_node_info_new_for_xml(
        xml_data: *const gchar,
        error: *mut *mut GError,
    ) -> *mut GDBusNodeInfo;
    fn g_dbus_node_info_lookup_interface(
        info: *mut GDBusNodeInfo,
        name: *const gchar,
    ) -> *mut GDBusInterfaceInfo;
    fn g_dbus_interface_info_ref(info: *mut GDBusInterfaceInfo) -> *mut GDBusInterfaceInfo;
    fn g_dbus_node_info_unref(info: *mut GDBusNodeInfo);
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
    fn g_action_group_has_action(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
    ) -> gboolean;
    fn g_action_group_list_actions(action_group: *mut GActionGroup) -> *mut *mut gchar;
    fn g_action_group_get_action_parameter_type(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
    ) -> *const GVariantType;
    fn g_action_group_get_action_enabled(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
    ) -> gboolean;
    fn g_action_group_get_action_state(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
    ) -> *mut GVariant;
    fn g_action_group_change_action_state(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
        value: *mut GVariant,
    );
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
    fn g_dbus_error_quark() -> GQuark;
}
pub type size_t = usize;
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
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
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
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSource {
    pub callback_data: gpointer,
    pub callback_funcs: *mut GSourceCallbackFuncs,
    pub source_funcs: *const GSourceFuncs,
    pub ref_count: guint,
    pub context: *mut GMainContext,
    pub priority: gint,
    pub flags: guint,
    pub source_id: guint,
    pub poll_fds: *mut GSList,
    pub prev: *mut GSource,
    pub next: *mut GSource,
    pub name: *mut ::core::ffi::c_char,
    pub priv_0: *mut GSourcePrivate,
}
pub type GSourcePrivate = _GSourcePrivate;
pub type GSource = _GSource;
pub type GSourceFuncs = _GSourceFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceFuncs {
    pub prepare: Option<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>,
    pub check: Option<unsafe extern "C" fn(*mut GSource) -> gboolean>,
    pub dispatch: Option<unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean>,
    pub finalize: Option<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub closure_callback: GSourceFunc,
    pub closure_marshal: GSourceDummyMarshal,
}
pub type GSourceDummyMarshal = Option<unsafe extern "C" fn() -> ()>;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallbackFuncs {
    pub ref_0: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub unref: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub get:
        Option<unsafe extern "C" fn(gpointer, *mut GSource, *mut GSourceFunc, *mut gpointer) -> ()>,
}
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantIter {
    pub x: [guintptr; 16],
}
pub type GVariantIter = _GVariantIter;
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const G_DBUS_ERROR_PROPERTY_READ_ONLY: C2RustUnnamed_2 = 44;
pub const G_DBUS_ERROR_UNKNOWN_PROPERTY: C2RustUnnamed_2 = 43;
pub const G_DBUS_ERROR_UNKNOWN_INTERFACE: C2RustUnnamed_2 = 42;
pub const G_DBUS_ERROR_UNKNOWN_OBJECT: C2RustUnnamed_2 = 41;
pub const G_DBUS_ERROR_OBJECT_PATH_IN_USE: C2RustUnnamed_2 = 40;
pub const G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN: C2RustUnnamed_2 = 39;
pub const G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN: C2RustUnnamed_2 = 38;
pub const G_DBUS_ERROR_INVALID_FILE_CONTENT: C2RustUnnamed_2 = 37;
pub const G_DBUS_ERROR_INVALID_SIGNATURE: C2RustUnnamed_2 = 36;
pub const G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN: C2RustUnnamed_2 = 35;
pub const G_DBUS_ERROR_SPAWN_NO_MEMORY: C2RustUnnamed_2 = 34;
pub const G_DBUS_ERROR_SPAWN_FILE_INVALID: C2RustUnnamed_2 = 33;
pub const G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID: C2RustUnnamed_2 = 32;
pub const G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND: C2RustUnnamed_2 = 31;
pub const G_DBUS_ERROR_SPAWN_SERVICE_INVALID: C2RustUnnamed_2 = 30;
pub const G_DBUS_ERROR_SPAWN_CONFIG_INVALID: C2RustUnnamed_2 = 29;
pub const G_DBUS_ERROR_SPAWN_SETUP_FAILED: C2RustUnnamed_2 = 28;
pub const G_DBUS_ERROR_SPAWN_FAILED: C2RustUnnamed_2 = 27;
pub const G_DBUS_ERROR_SPAWN_CHILD_SIGNALED: C2RustUnnamed_2 = 26;
pub const G_DBUS_ERROR_SPAWN_CHILD_EXITED: C2RustUnnamed_2 = 25;
pub const G_DBUS_ERROR_SPAWN_FORK_FAILED: C2RustUnnamed_2 = 24;
pub const G_DBUS_ERROR_SPAWN_EXEC_FAILED: C2RustUnnamed_2 = 23;
pub const G_DBUS_ERROR_MATCH_RULE_INVALID: C2RustUnnamed_2 = 22;
pub const G_DBUS_ERROR_MATCH_RULE_NOT_FOUND: C2RustUnnamed_2 = 21;
pub const G_DBUS_ERROR_TIMED_OUT: C2RustUnnamed_2 = 20;
pub const G_DBUS_ERROR_UNKNOWN_METHOD: C2RustUnnamed_2 = 19;
pub const G_DBUS_ERROR_FILE_EXISTS: C2RustUnnamed_2 = 18;
pub const G_DBUS_ERROR_FILE_NOT_FOUND: C2RustUnnamed_2 = 17;
pub const G_DBUS_ERROR_INVALID_ARGS: C2RustUnnamed_2 = 16;
pub const G_DBUS_ERROR_DISCONNECTED: C2RustUnnamed_2 = 15;
pub const G_DBUS_ERROR_ADDRESS_IN_USE: C2RustUnnamed_2 = 14;
pub const G_DBUS_ERROR_NO_NETWORK: C2RustUnnamed_2 = 13;
pub const G_DBUS_ERROR_TIMEOUT: C2RustUnnamed_2 = 12;
pub const G_DBUS_ERROR_NO_SERVER: C2RustUnnamed_2 = 11;
pub const G_DBUS_ERROR_AUTH_FAILED: C2RustUnnamed_2 = 10;
pub const G_DBUS_ERROR_ACCESS_DENIED: C2RustUnnamed_2 = 9;
pub const G_DBUS_ERROR_LIMITS_EXCEEDED: C2RustUnnamed_2 = 8;
pub const G_DBUS_ERROR_NOT_SUPPORTED: C2RustUnnamed_2 = 7;
pub const G_DBUS_ERROR_BAD_ADDRESS: C2RustUnnamed_2 = 6;
pub const G_DBUS_ERROR_IO_ERROR: C2RustUnnamed_2 = 5;
pub const G_DBUS_ERROR_NO_REPLY: C2RustUnnamed_2 = 4;
pub const G_DBUS_ERROR_NAME_HAS_NO_OWNER: C2RustUnnamed_2 = 3;
pub const G_DBUS_ERROR_SERVICE_UNKNOWN: C2RustUnnamed_2 = 2;
pub const G_DBUS_ERROR_NO_MEMORY: C2RustUnnamed_2 = 1;
pub const G_DBUS_ERROR_FAILED: C2RustUnnamed_2 = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GRemoteActionGroup = _GRemoteActionGroup;
pub type GActionGroup = _GActionGroup;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusNodeInfo {
    pub ref_count: gint,
    pub path: *mut gchar,
    pub interfaces: *mut *mut GDBusInterfaceInfo,
    pub nodes: *mut *mut GDBusNodeInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusNodeInfo = _GDBusNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GActionGroupExporter {
    pub action_group: *mut GActionGroup,
    pub connection: *mut GDBusConnection,
    pub context: *mut GMainContext,
    pub object_path: *mut gchar,
    pub pending_changes: *mut GHashTable,
    pub pending_source: *mut GSource,
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
unsafe extern "C" fn safe_c2rust_g_action_group_describe_action(
    mut action_group: *mut GActionGroup,
    mut name: *const gchar,
) -> *mut GVariant {
    let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut enabled: gboolean = 0;
    let mut state: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"(bgav)\0" as *const u8 as *const gchar),
    );
    enabled = g_action_group_get_action_enabled(action_group, name);
    g_variant_builder_add(
        &raw mut builder,
        b"b\0" as *const u8 as *const gchar,
        enabled,
    );
    type_0 = g_action_group_get_action_parameter_type(action_group, name);
    if !type_0.is_null() {
        let mut str: *mut gchar = g_variant_type_dup_string(type_0);
        g_variant_builder_add(&raw mut builder, b"g\0" as *const u8 as *const gchar, str);
        g_free(str as gpointer);
    } else {
        g_variant_builder_add(
            &raw mut builder,
            b"g\0" as *const u8 as *const gchar,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_variant_builder_open(
        &raw mut builder,
        g_variant_type_checked_(b"av\0" as *const u8 as *const gchar),
    );
    state = g_action_group_get_action_state(action_group, name);
    if !state.is_null() {
        g_variant_builder_add(&raw mut builder, b"v\0" as *const u8 as *const gchar, state);
        g_variant_unref(state);
    }
    g_variant_builder_close(&raw mut builder);
    return g_variant_builder_end(&raw mut builder);
}
#[no_mangle]
pub static mut safe_c2rust_org_gtk_Actions_xml: [::core::ffi::c_char; 1053] = unsafe {
    ::core::mem::transmute::<
        [u8; 1053],
        [::core::ffi::c_char; 1053],
    >(
        *b"<node>  <interface name='org.gtk.Actions'>    <method name='List'>      <arg type='as' name='list' direction='out'/>    </method>    <method name='Describe'>      <arg type='s' name='action_name' direction='in'/>      <arg type='(bgav)' name='description' direction='out'/>    </method>    <method name='DescribeAll'>      <arg type='a{s(bgav)}' name='descriptions' direction='out'/>    </method>    <method name='Activate'>      <arg type='s' name='action_name' direction='in'/>      <arg type='av' name='parameter' direction='in'/>      <arg type='a{sv}' name='platform_data' direction='in'/>    </method>    <method name='SetState'>      <arg type='s' name='action_name' direction='in'/>      <arg type='v' name='value' direction='in'/>      <arg type='a{sv}' name='platform_data' direction='in'/>    </method>    <signal name='Changed'>      <arg type='as' name='removals'/>      <arg type='a{sb}' name='enable_changes'/>      <arg type='a{sv}' name='state_changes'/>      <arg type='a{s(bgav)}' name='additions'/>    </signal>  </interface></node>\0",
    )
};
static mut safe_c2rust_org_gtk_Actions: *mut GDBusInterfaceInfo =
    ::core::ptr::null::<GDBusInterfaceInfo>() as *mut GDBusInterfaceInfo;
pub const ACTION_ADDED_EVENT: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 0 as ::core::ffi::c_int;
pub const ACTION_REMOVED_EVENT: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int;
pub const ACTION_STATE_CHANGED_EVENT: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int;
pub const ACTION_ENABLED_CHANGED_EVENT: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_action_group_exporter_dispatch_events(
    mut user_data: gpointer,
) -> gboolean {
    let mut exporter: *mut GActionGroupExporter = user_data as *mut GActionGroupExporter;
    let mut removes: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut enabled_changes: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut state_changes: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut adds: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    g_variant_builder_init(&raw mut removes, G_VARIANT_TYPE_STRING_ARRAY);
    g_variant_builder_init(
        &raw mut enabled_changes,
        g_variant_type_checked_(b"a{sb}\0" as *const u8 as *const gchar),
    );
    g_variant_builder_init(
        &raw mut state_changes,
        g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
    );
    g_variant_builder_init(
        &raw mut adds,
        g_variant_type_checked_(b"a{s(bgav)}\0" as *const u8 as *const gchar),
    );
    g_hash_table_iter_init(&raw mut iter, (*exporter).pending_changes);
    while g_hash_table_iter_next(&raw mut iter, &raw mut key, &raw mut value) != 0 {
        let mut events: guint = value as glong as gint as guint;
        let mut name: *const gchar = key as *const gchar;
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if (events as ::core::ffi::c_uint
                & ((1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int)
                == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
                != (events as ::core::ffi::c_uint
                    & ((1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_uint) << 0 as ::core::ffi::c_int)
                    == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gactiongroupexporter.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                164 as ::core::ffi::c_int,
                G_STRFUNC,
                b"((events & (ACTION_ENABLED_CHANGED_EVENT | ACTION_STATE_CHANGED_EVENT)) == 0) != ((events & (ACTION_REMOVED_EVENT | ACTION_ADDED_EVENT)) == 0)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if events as ::core::ffi::c_uint & ACTION_REMOVED_EVENT != 0 {
            g_variant_builder_add(&raw mut removes, b"s\0" as *const u8 as *const gchar, name);
        }
        if events as ::core::ffi::c_uint & ACTION_ENABLED_CHANGED_EVENT != 0 {
            let mut enabled: gboolean = 0;
            enabled = g_action_group_get_action_enabled((*exporter).action_group, name);
            g_variant_builder_add(
                &raw mut enabled_changes,
                b"{sb}\0" as *const u8 as *const gchar,
                name,
                enabled,
            );
        }
        if events as ::core::ffi::c_uint & ACTION_STATE_CHANGED_EVENT != 0 {
            let mut state: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            state = g_action_group_get_action_state((*exporter).action_group, name);
            g_variant_builder_add(
                &raw mut state_changes,
                b"{sv}\0" as *const u8 as *const gchar,
                name,
                state,
            );
            g_variant_unref(state);
        }
        if events as ::core::ffi::c_uint & ACTION_ADDED_EVENT != 0 {
            let mut description: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            description =
                safe_c2rust_g_action_group_describe_action((*exporter).action_group, name);
            g_variant_builder_add(
                &raw mut adds,
                b"{s@(bgav)}\0" as *const u8 as *const gchar,
                name,
                description,
            );
        }
    }
    g_hash_table_remove_all((*exporter).pending_changes);
    g_dbus_connection_emit_signal(
        (*exporter).connection,
        ::core::ptr::null::<gchar>(),
        (*exporter).object_path,
        b"org.gtk.Actions\0" as *const u8 as *const gchar,
        b"Changed\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(asa{sb}a{sv}a{s(bgav)})\0" as *const u8 as *const gchar,
            &raw mut removes,
            &raw mut enabled_changes,
            &raw mut state_changes,
            &raw mut adds,
        ),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    (*exporter).pending_source = ::core::ptr::null_mut::<GSource>();
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_action_group_exporter_flush_queue(
    mut exporter: *mut GActionGroupExporter,
) {
    if !(*exporter).pending_source.is_null() {
        g_source_destroy((*exporter).pending_source);
        safe_c2rust_g_action_group_exporter_dispatch_events(exporter as gpointer);
        if ({
            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
            if (*exporter).pending_source.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gactiongroupexporter.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                216 as ::core::ffi::c_int,
                G_STRFUNC,
                b"exporter->pending_source == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_action_group_exporter_get_events(
    mut exporter: *mut GActionGroupExporter,
    mut name: *const gchar,
) -> guint {
    return g_hash_table_lookup((*exporter).pending_changes, name as gconstpointer) as gsize
        as guint;
}
unsafe extern "C" fn safe_c2rust_g_action_group_exporter_set_events(
    mut exporter: *mut GActionGroupExporter,
    mut name: *const gchar,
    mut events: guint,
) {
    let mut have_events: gboolean = 0;
    let mut is_queued: gboolean = 0;
    if events != 0 as guint {
        g_hash_table_insert(
            (*exporter).pending_changes,
            safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as gpointer,
            events as glong as gpointer,
        );
    } else {
        g_hash_table_remove((*exporter).pending_changes, name as gconstpointer);
    }
    have_events = (g_hash_table_size((*exporter).pending_changes) > 0 as guint)
        as ::core::ffi::c_int as gboolean;
    is_queued =
        ((*exporter).pending_source != NULL_0 as *mut GSource) as ::core::ffi::c_int as gboolean;
    if have_events != 0 && is_queued == 0 {
        let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
        source = g_idle_source_new();
        (*exporter).pending_source = source;
        g_source_set_callback(
            source,
            Some(
                safe_c2rust_g_action_group_exporter_dispatch_events
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            exporter as gpointer,
            None,
        );
        g_source_set_static_name(
            source,
            b"[gio] g_action_group_exporter_dispatch_events\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        g_source_attach(source, (*exporter).context);
        g_source_unref(source);
    }
    if have_events == 0 && is_queued != 0 {
        g_source_destroy((*exporter).pending_source);
        (*exporter).pending_source = ::core::ptr::null_mut::<GSource>();
    }
}
unsafe extern "C" fn safe_c2rust_g_action_group_exporter_action_added(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut user_data: gpointer,
) {
    let mut exporter: *mut GActionGroupExporter = user_data as *mut GActionGroupExporter;
    let mut event_mask: guint = 0;
    event_mask = safe_c2rust_g_action_group_exporter_get_events(exporter, action_name);
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !(event_mask as ::core::ffi::c_uint)
            & ((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
            != 0
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gactiongroupexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            276 as ::core::ffi::c_int,
            G_STRFUNC,
            b"~event_mask & (ACTION_STATE_CHANGED_EVENT | ACTION_ENABLED_CHANGED_EVENT)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    event_mask |= ACTION_ADDED_EVENT;
    safe_c2rust_g_action_group_exporter_set_events(exporter, action_name, event_mask);
}
unsafe extern "C" fn safe_c2rust_g_action_group_exporter_action_removed(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut user_data: gpointer,
) {
    let mut exporter: *mut GActionGroupExporter = user_data as *mut GActionGroupExporter;
    let mut event_mask: guint = 0;
    event_mask = safe_c2rust_g_action_group_exporter_get_events(exporter, action_name);
    if event_mask as ::core::ffi::c_uint & ACTION_ADDED_EVENT != 0 {
        if ({
            let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
            if !(event_mask as ::core::ffi::c_uint)
                & !((1 as ::core::ffi::c_uint) << 2 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_uint) << 3 as ::core::ffi::c_int)
                != 0
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gactiongroupexporter.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int,
                G_STRFUNC,
                b"~event_mask & ~(ACTION_STATE_CHANGED_EVENT | ACTION_ENABLED_CHANGED_EVENT)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        event_mask &= !ACTION_ADDED_EVENT;
    } else {
        event_mask |= ACTION_REMOVED_EVENT;
        event_mask &= !(ACTION_STATE_CHANGED_EVENT | ACTION_ENABLED_CHANGED_EVENT);
    }
    safe_c2rust_g_action_group_exporter_set_events(exporter, action_name, event_mask);
}
unsafe extern "C" fn safe_c2rust_g_action_group_exporter_action_state_changed(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut value: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut exporter: *mut GActionGroupExporter = user_data as *mut GActionGroupExporter;
    let mut event_mask: guint = 0;
    event_mask = safe_c2rust_g_action_group_exporter_get_events(exporter, action_name);
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !(event_mask as ::core::ffi::c_uint)
            & (1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int
            != 0
            || event_mask as ::core::ffi::c_uint
                & (1 as ::core::ffi::c_uint) << 0 as ::core::ffi::c_int
                != 0
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gactiongroupexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            331 as ::core::ffi::c_int,
            G_STRFUNC,
            b"~event_mask & ACTION_REMOVED_EVENT || event_mask & ACTION_ADDED_EVENT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if !(event_mask as ::core::ffi::c_uint) & ACTION_ADDED_EVENT != 0 {
        event_mask |= ACTION_STATE_CHANGED_EVENT;
    }
    safe_c2rust_g_action_group_exporter_set_events(exporter, action_name, event_mask);
}
unsafe extern "C" fn safe_c2rust_g_action_group_exporter_action_enabled_changed(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut enabled: gboolean,
    mut user_data: gpointer,
) {
    let mut exporter: *mut GActionGroupExporter = user_data as *mut GActionGroupExporter;
    let mut event_mask: guint = 0;
    event_mask = safe_c2rust_g_action_group_exporter_get_events(exporter, action_name);
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !(event_mask as ::core::ffi::c_uint)
            & (1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int
            != 0
            || event_mask as ::core::ffi::c_uint
                & (1 as ::core::ffi::c_uint) << 0 as ::core::ffi::c_int
                != 0
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gactiongroupexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            356 as ::core::ffi::c_int,
            G_STRFUNC,
            b"~event_mask & ACTION_REMOVED_EVENT || event_mask & ACTION_ADDED_EVENT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if !(event_mask as ::core::ffi::c_uint) & ACTION_ADDED_EVENT != 0 {
        event_mask |= ACTION_ENABLED_CHANGED_EVENT;
    }
    safe_c2rust_g_action_group_exporter_set_events(exporter, action_name, event_mask);
}
unsafe extern "C" fn safe_c2rust_org_gtk_Actions_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut exporter: *mut GActionGroupExporter = user_data as *mut GActionGroupExporter;
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    safe_c2rust_g_action_group_exporter_flush_queue(exporter);
    if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"List\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut list: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        list = g_action_group_list_actions((*exporter).action_group);
        result = g_variant_new(b"(^as)\0" as *const u8 as *const gchar, list);
        g_strfreev(list);
    } else if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"Describe\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut name: *const gchar = ::core::ptr::null::<gchar>();
        let mut desc: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        g_variant_get(
            parameters,
            b"(&s)\0" as *const u8 as *const gchar,
            &raw mut name,
        );
        if g_action_group_has_action((*exporter).action_group, name) == 0 {
            g_dbus_method_invocation_return_error(
                invocation,
                g_dbus_error_quark(),
                G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
                b"The named action ('%s') does not exist.\0" as *const u8 as *const gchar,
                name,
            );
            return;
        }
        desc = safe_c2rust_g_action_group_describe_action((*exporter).action_group, name);
        result = g_variant_new(b"(@(bgav))\0" as *const u8 as *const gchar, desc);
    } else if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"DescribeAll\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut builder: GVariantBuilder = _GVariantBuilder {
            u: C2RustUnnamed {
                s: C2RustUnnamed_0 {
                    partial_magic: 0,
                    type_0: ::core::ptr::null::<GVariantType>(),
                    y: [0; 14],
                },
            },
        };
        let mut list_0: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        let mut i: gint = 0;
        list_0 = g_action_group_list_actions((*exporter).action_group);
        g_variant_builder_init(
            &raw mut builder,
            g_variant_type_checked_(b"a{s(bgav)}\0" as *const u8 as *const gchar),
        );
        i = 0 as ::core::ffi::c_int as gint;
        while !(*list_0.offset(i as isize)).is_null() {
            let mut name_0: *const gchar = *list_0.offset(i as isize);
            let mut description: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            description =
                safe_c2rust_g_action_group_describe_action((*exporter).action_group, name_0);
            g_variant_builder_add(
                &raw mut builder,
                b"{s@(bgav)}\0" as *const u8 as *const gchar,
                name_0,
                description,
            );
            i += 1;
        }
        result = g_variant_new(
            b"(a{s(bgav)})\0" as *const u8 as *const gchar,
            &raw mut builder,
        );
        g_strfreev(list_0);
    } else if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"Activate\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut parameter: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut platform_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
        let mut name_1: *const gchar = ::core::ptr::null::<gchar>();
        let mut parameter_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
        g_variant_get(
            parameters,
            b"(&sav@a{sv})\0" as *const u8 as *const gchar,
            &raw mut name_1,
            &raw mut iter,
            &raw mut platform_data,
        );
        g_variant_iter_next(
            iter,
            b"v\0" as *const u8 as *const gchar,
            &raw mut parameter,
        );
        g_variant_iter_free(iter);
        if g_action_group_query_action(
            (*exporter).action_group,
            name_1,
            ::core::ptr::null_mut::<gboolean>(),
            &raw mut parameter_type,
            ::core::ptr::null_mut::<*const GVariantType>(),
            ::core::ptr::null_mut::<*mut GVariant>(),
            ::core::ptr::null_mut::<*mut GVariant>(),
        ) == 0
        {
            g_dbus_method_invocation_return_error(
                invocation,
                g_dbus_error_quark(),
                G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
                b"Unknown action \xE2\x80\x98%s\xE2\x80\x99\0" as *const u8 as *const gchar,
                name_1,
            );
            let mut _pp: *mut *mut GVariant = &raw mut parameter;
            let mut _ptr: *mut GVariant = *_pp;
            *_pp = ::core::ptr::null_mut::<GVariant>();
            if !_ptr.is_null() {
                g_variant_unref(_ptr as *mut GVariant);
            }
            g_variant_unref(platform_data);
            return;
        }
        if !(parameter_type.is_null() && parameter.is_null()
            || !parameter_type.is_null()
                && !parameter.is_null()
                && g_variant_is_of_type(parameter, parameter_type) != 0)
        {
            g_dbus_method_invocation_return_error(
                invocation,
                g_dbus_error_quark(),
                G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
                b"Invalid parameter for action \xE2\x80\x98%s\xE2\x80\x99: expected type %s but got type %s\0"
                    as *const u8 as *const gchar,
                name_1,
                if !parameter_type.is_null() {
                    parameter_type as *const ::core::ffi::c_char
                } else {
                    b"()\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !parameter.is_null() {
                    g_variant_get_type_string(parameter) as *const ::core::ffi::c_char
                } else {
                    b"()\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            let mut _pp_0: *mut *mut GVariant = &raw mut parameter;
            let mut _ptr_0: *mut GVariant = *_pp_0;
            *_pp_0 = ::core::ptr::null_mut::<GVariant>();
            if !_ptr_0.is_null() {
                g_variant_unref(_ptr_0 as *mut GVariant);
            }
            g_variant_unref(platform_data);
            return;
        }
        if ({
            let mut __inst: *mut GTypeInstance = (*exporter).action_group as *mut GTypeInstance;
            let mut __t: GType = g_remote_action_group_get_type();
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
            g_remote_action_group_activate_action_full(
                (*exporter).action_group as *mut ::core::ffi::c_void as *mut GRemoteActionGroup,
                name_1,
                parameter,
                platform_data,
            );
        } else {
            g_action_group_activate_action((*exporter).action_group, name_1, parameter);
        }
        if !parameter.is_null() {
            g_variant_unref(parameter);
        }
        g_variant_unref(platform_data);
    } else if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"SetState\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut platform_data_0: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut name_2: *const gchar = ::core::ptr::null::<gchar>();
        let mut state: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut state_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
        g_variant_get(
            parameters,
            b"(&sv@a{sv})\0" as *const u8 as *const gchar,
            &raw mut name_2,
            &raw mut state,
            &raw mut platform_data_0,
        );
        if g_action_group_query_action(
            (*exporter).action_group,
            name_2,
            ::core::ptr::null_mut::<gboolean>(),
            ::core::ptr::null_mut::<*const GVariantType>(),
            &raw mut state_type,
            ::core::ptr::null_mut::<*mut GVariant>(),
            ::core::ptr::null_mut::<*mut GVariant>(),
        ) == 0
        {
            g_dbus_method_invocation_return_error(
                invocation,
                g_dbus_error_quark(),
                G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
                b"Unknown action \xE2\x80\x98%s\xE2\x80\x99\0" as *const u8 as *const gchar,
                name_2,
            );
            g_variant_unref(state);
            g_variant_unref(platform_data_0);
            return;
        }
        if state_type.is_null() {
            g_dbus_method_invocation_return_error(
                invocation,
                g_dbus_error_quark(),
                G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
                b"Cannot change state of action \xE2\x80\x98%s\xE2\x80\x99 as it is stateless\0"
                    as *const u8 as *const gchar,
                name_2,
            );
            g_variant_unref(state);
            g_variant_unref(platform_data_0);
            return;
        }
        if g_variant_is_of_type(state, state_type) == 0 {
            g_dbus_method_invocation_return_error(
                invocation,
                g_dbus_error_quark(),
                G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
                b"Invalid state for action \xE2\x80\x98%s\xE2\x80\x99: expected type %s but got type %s\0"
                    as *const u8 as *const gchar,
                name_2,
                state_type as *const gchar,
                g_variant_get_type_string(state),
            );
            g_variant_unref(state);
            g_variant_unref(platform_data_0);
            return;
        }
        if ({
            let mut __inst: *mut GTypeInstance = (*exporter).action_group as *mut GTypeInstance;
            let mut __t: GType = g_remote_action_group_get_type();
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
            g_remote_action_group_change_action_state_full(
                (*exporter).action_group as *mut ::core::ffi::c_void as *mut GRemoteActionGroup,
                name_2,
                state,
                platform_data_0,
            );
        } else {
            g_action_group_change_action_state((*exporter).action_group, name_2, state);
        }
        g_variant_unref(platform_data_0);
        g_variant_unref(state);
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gactiongroupexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            528 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    g_dbus_method_invocation_return_value(invocation, result);
}
unsafe extern "C" fn safe_c2rust_g_action_group_exporter_free(mut user_data: gpointer) {
    let mut exporter: *mut GActionGroupExporter = user_data as *mut GActionGroupExporter;
    g_signal_handlers_disconnect_matched(
        (*exporter).action_group as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, gpointer) -> ()>,
            gpointer,
        >(Some(
            safe_c2rust_g_action_group_exporter_action_added
                as unsafe extern "C" fn(*mut GActionGroup, *const gchar, gpointer) -> (),
        )),
        exporter as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*exporter).action_group as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, gboolean, gpointer) -> ()>,
            gpointer,
        >(Some(
            safe_c2rust_g_action_group_exporter_action_enabled_changed
                as unsafe extern "C" fn(*mut GActionGroup, *const gchar, gboolean, gpointer) -> (),
        )),
        exporter as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*exporter).action_group as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GActionGroup,
                    *const gchar,
                    *mut GVariant,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_action_group_exporter_action_state_changed
                as unsafe extern "C" fn(
                    *mut GActionGroup,
                    *const gchar,
                    *mut GVariant,
                    gpointer,
                ) -> (),
        )),
        exporter as gpointer,
    );
    g_signal_handlers_disconnect_matched(
        (*exporter).action_group as gpointer,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, gpointer) -> ()>,
            gpointer,
        >(Some(
            safe_c2rust_g_action_group_exporter_action_removed
                as unsafe extern "C" fn(*mut GActionGroup, *const gchar, gpointer) -> (),
        )),
        exporter as gpointer,
    );
    g_hash_table_unref((*exporter).pending_changes);
    if !(*exporter).pending_source.is_null() {
        g_source_destroy((*exporter).pending_source);
    }
    g_main_context_unref((*exporter).context);
    g_object_unref((*exporter).connection as gpointer);
    g_object_unref((*exporter).action_group as gpointer);
    g_free((*exporter).object_path as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GActionGroupExporter>() as gsize,
        exporter as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_export_action_group(
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
    mut action_group: *mut GActionGroup,
    mut error: *mut *mut GError,
) -> guint {
    let vtable: GDBusInterfaceVTable = _GDBusInterfaceVTable {
        method_call: Some(
            safe_c2rust_org_gtk_Actions_method_call
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
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ],
    };
    let mut exporter: *mut GActionGroupExporter = ::core::ptr::null_mut::<GActionGroupExporter>();
    let mut id: guint = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if safe_c2rust_org_gtk_Actions.is_null() {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
        let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut info: *mut GDBusNodeInfo = ::core::ptr::null_mut::<GDBusNodeInfo>();
        info = g_dbus_node_info_new_for_xml(
            &raw const safe_c2rust_org_gtk_Actions_xml as *const gchar,
            &raw mut my_error,
        );
        if ({
            let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
            if info.is_null() {
                _g_boolean_var_17 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_17 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_17
        }) as ::core::ffi::c_long
            != 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"%s\0" as *const u8 as *const gchar,
                (*my_error).message,
            );
            loop {}
        }
        safe_c2rust_org_gtk_Actions = g_dbus_node_info_lookup_interface(
            info,
            b"org.gtk.Actions\0" as *const u8 as *const gchar,
        );
        if ({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if !safe_c2rust_org_gtk_Actions.is_null() {
                _g_boolean_var_18 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_18 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_18
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gactiongroupexporter.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                613 as ::core::ffi::c_int,
                G_STRFUNC,
                b"org_gtk_Actions != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_dbus_interface_info_ref(safe_c2rust_org_gtk_Actions);
        g_dbus_node_info_unref(info);
    }
    exporter = g_slice_alloc(::core::mem::size_of::<GActionGroupExporter>() as gsize)
        as *mut GActionGroupExporter;
    id = g_dbus_connection_register_object(
        connection,
        object_path,
        safe_c2rust_org_gtk_Actions,
        &raw const vtable,
        exporter as gpointer,
        Some(safe_c2rust_g_action_group_exporter_free as unsafe extern "C" fn(gpointer) -> ()),
        error,
    );
    if id == 0 as guint {
        g_slice_free1(
            ::core::mem::size_of::<GActionGroupExporter>() as gsize,
            exporter as gpointer,
        );
        return 0 as guint;
    }
    (*exporter).context = g_main_context_ref_thread_default();
    (*exporter).pending_changes = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
    (*exporter).pending_source = ::core::ptr::null_mut::<GSource>();
    (*exporter).action_group =
        g_object_ref(action_group as gpointer) as *mut GActionGroup as *mut GActionGroup;
    (*exporter).connection =
        g_object_ref(connection as gpointer) as *mut GDBusConnection as *mut GDBusConnection;
    (*exporter).object_path =
        safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as *mut gchar;
    g_signal_connect_data(
        action_group as gpointer,
        b"action-added\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_g_action_group_exporter_action_added
                as unsafe extern "C" fn(*mut GActionGroup, *const gchar, gpointer) -> (),
        )),
        exporter as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        action_group as gpointer,
        b"action-removed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_g_action_group_exporter_action_removed
                as unsafe extern "C" fn(*mut GActionGroup, *const gchar, gpointer) -> (),
        )),
        exporter as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        action_group as gpointer,
        b"action-state-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GActionGroup,
                    *const gchar,
                    *mut GVariant,
                    gpointer,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_g_action_group_exporter_action_state_changed
                as unsafe extern "C" fn(
                    *mut GActionGroup,
                    *const gchar,
                    *mut GVariant,
                    gpointer,
                ) -> (),
        )),
        exporter as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        action_group as gpointer,
        b"action-enabled-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, gboolean, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_g_action_group_exporter_action_enabled_changed
                as unsafe extern "C" fn(*mut GActionGroup, *const gchar, gboolean, gpointer) -> (),
        )),
        exporter as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_unexport_action_group(
    mut connection: *mut GDBusConnection,
    mut export_id: guint,
) {
    g_dbus_connection_unregister_object(connection, export_id);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
