extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GMainLoop;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GRemoteActionGroup;
    pub type _GDBusActionGroup;
    pub type _GActionGroup;
    pub type _GApplicationPrivate;
    pub type _GApplicationCommandLinePrivate;
    pub type _GFile;
    pub type _GUnixFDListPrivate;
    pub type _GDBusMessage;
    pub type _GDBusConnection;
    pub type _GDBusMethodInvocation;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_main_context_new() -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_push_thread_default(context: *mut GMainContext);
    fn g_main_context_pop_thread_default(context: *mut GMainContext);
    fn g_main_loop_new(context: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
    fn g_main_loop_run(loop_0: *mut GMainLoop);
    fn g_main_loop_quit(loop_0: *mut GMainLoop);
    fn g_main_loop_unref(loop_0: *mut GMainLoop);
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new_boolean(value: gboolean) -> *mut GVariant;
    fn g_variant_n_children(value: *mut GVariant) -> gsize;
    fn g_variant_get_child(value: *mut GVariant, index_: gsize, format_string: *const gchar, ...);
    fn g_variant_get_child_value(value: *mut GVariant, index_: gsize) -> *mut GVariant;
    fn g_variant_iter_free(iter: *mut GVariantIter);
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_open(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_close(builder: *mut GVariantBuilder);
    fn g_variant_builder_add_value(builder: *mut GVariantBuilder, value: *mut GVariant);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_print(format: *const gchar, ...);
    fn g_printerr(format: *const gchar, ...);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_assertion_message_error(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        error: *const GError,
        error_domain: GQuark,
        error_code: ::core::ffi::c_int,
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
    fn g_signal_emit_by_name(instance: gpointer, detailed_signal: *const gchar, ...);
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
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
    fn g_dbus_connection_export_action_group(
        connection: *mut GDBusConnection,
        object_path: *const gchar,
        action_group: *mut GActionGroup,
        error: *mut *mut GError,
    ) -> guint;
    fn g_dbus_connection_unexport_action_group(connection: *mut GDBusConnection, export_id: guint);
    fn g_dbus_action_group_get(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
    ) -> *mut GDBusActionGroup;
    fn g_dbus_action_group_sync(
        group: *mut GDBusActionGroup,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_application_get_flags(application: *mut GApplication) -> GApplicationFlags;
    fn g_file_new_for_uri(uri: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_get_uri(file: *mut GFile) -> *mut ::core::ffi::c_char;
    fn g_bus_get_sync(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusConnection;
    fn g_dbus_connection_flush_sync(
        connection: *mut GDBusConnection,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_dbus_connection_emit_signal(
        connection: *mut GDBusConnection,
        destination_bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        signal_name: *const gchar,
        parameters: *mut GVariant,
        error: *mut *mut GError,
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
    fn g_dbus_connection_call_sync(
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
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_connection_call_with_unix_fd_list(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        fd_list: *mut GUnixFDList,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_connection_call_with_unix_fd_list_finish(
        connection: *mut GDBusConnection,
        out_fd_list: *mut *mut GUnixFDList,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GVariant;
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
    fn g_dbus_error_quark() -> GQuark;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn g_application_command_line_get_type() -> GType;
    fn g_application_command_line_get_exit_status(
        cmdline: *mut GApplicationCommandLine,
    ) -> ::core::ffi::c_int;
    fn g_application_command_line_set_exit_status(
        cmdline: *mut GApplicationCommandLine,
        exit_status: ::core::ffi::c_int,
    );
    fn g_dbus_method_invocation_get_sender(invocation: *mut GDBusMethodInvocation) -> *const gchar;
    fn g_dbus_method_invocation_get_connection(
        invocation: *mut GDBusMethodInvocation,
    ) -> *mut GDBusConnection;
    fn g_dbus_method_invocation_get_message(
        invocation: *mut GDBusMethodInvocation,
    ) -> *mut GDBusMessage;
    fn g_dbus_method_invocation_get_parameters(
        invocation: *mut GDBusMethodInvocation,
    ) -> *mut GVariant;
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
    fn g_dbus_message_get_unix_fd_list(message: *mut GDBusMessage) -> *mut GUnixFDList;
    fn g_unix_fd_list_new() -> *mut GUnixFDList;
    fn g_unix_fd_list_append(list: *mut GUnixFDList, fd: gint, error: *mut *mut GError) -> gint;
    fn g_unix_fd_list_get_length(list: *mut GUnixFDList) -> gint;
    fn g_unix_fd_list_steal_fds(list: *mut GUnixFDList, length: *mut gint) -> *mut gint;
    fn g_unix_input_stream_new(fd: gint, close_fd: gboolean) -> *mut GInputStream;
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
pub type GMainContext = _GMainContext;
pub type GMainLoop = _GMainLoop;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantDict {
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: C2RustUnnamed_2,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub asv: *mut GVariant,
    pub partial_magic: gsize,
    pub y: [guintptr; 14],
}
pub type GVariantDict = _GVariantDict;
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
    pub data: [C2RustUnnamed_3; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GBusNameOwnerFlags = ::core::ffi::c_uint;
pub const G_BUS_NAME_OWNER_FLAGS_DO_NOT_QUEUE: GBusNameOwnerFlags = 4;
pub const G_BUS_NAME_OWNER_FLAGS_REPLACE: GBusNameOwnerFlags = 2;
pub const G_BUS_NAME_OWNER_FLAGS_ALLOW_REPLACEMENT: GBusNameOwnerFlags = 1;
pub const G_BUS_NAME_OWNER_FLAGS_NONE: GBusNameOwnerFlags = 0;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const G_DBUS_ERROR_PROPERTY_READ_ONLY: C2RustUnnamed_4 = 44;
pub const G_DBUS_ERROR_UNKNOWN_PROPERTY: C2RustUnnamed_4 = 43;
pub const G_DBUS_ERROR_UNKNOWN_INTERFACE: C2RustUnnamed_4 = 42;
pub const G_DBUS_ERROR_UNKNOWN_OBJECT: C2RustUnnamed_4 = 41;
pub const G_DBUS_ERROR_OBJECT_PATH_IN_USE: C2RustUnnamed_4 = 40;
pub const G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN: C2RustUnnamed_4 = 39;
pub const G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN: C2RustUnnamed_4 = 38;
pub const G_DBUS_ERROR_INVALID_FILE_CONTENT: C2RustUnnamed_4 = 37;
pub const G_DBUS_ERROR_INVALID_SIGNATURE: C2RustUnnamed_4 = 36;
pub const G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN: C2RustUnnamed_4 = 35;
pub const G_DBUS_ERROR_SPAWN_NO_MEMORY: C2RustUnnamed_4 = 34;
pub const G_DBUS_ERROR_SPAWN_FILE_INVALID: C2RustUnnamed_4 = 33;
pub const G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID: C2RustUnnamed_4 = 32;
pub const G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND: C2RustUnnamed_4 = 31;
pub const G_DBUS_ERROR_SPAWN_SERVICE_INVALID: C2RustUnnamed_4 = 30;
pub const G_DBUS_ERROR_SPAWN_CONFIG_INVALID: C2RustUnnamed_4 = 29;
pub const G_DBUS_ERROR_SPAWN_SETUP_FAILED: C2RustUnnamed_4 = 28;
pub const G_DBUS_ERROR_SPAWN_FAILED: C2RustUnnamed_4 = 27;
pub const G_DBUS_ERROR_SPAWN_CHILD_SIGNALED: C2RustUnnamed_4 = 26;
pub const G_DBUS_ERROR_SPAWN_CHILD_EXITED: C2RustUnnamed_4 = 25;
pub const G_DBUS_ERROR_SPAWN_FORK_FAILED: C2RustUnnamed_4 = 24;
pub const G_DBUS_ERROR_SPAWN_EXEC_FAILED: C2RustUnnamed_4 = 23;
pub const G_DBUS_ERROR_MATCH_RULE_INVALID: C2RustUnnamed_4 = 22;
pub const G_DBUS_ERROR_MATCH_RULE_NOT_FOUND: C2RustUnnamed_4 = 21;
pub const G_DBUS_ERROR_TIMED_OUT: C2RustUnnamed_4 = 20;
pub const G_DBUS_ERROR_UNKNOWN_METHOD: C2RustUnnamed_4 = 19;
pub const G_DBUS_ERROR_FILE_EXISTS: C2RustUnnamed_4 = 18;
pub const G_DBUS_ERROR_FILE_NOT_FOUND: C2RustUnnamed_4 = 17;
pub const G_DBUS_ERROR_INVALID_ARGS: C2RustUnnamed_4 = 16;
pub const G_DBUS_ERROR_DISCONNECTED: C2RustUnnamed_4 = 15;
pub const G_DBUS_ERROR_ADDRESS_IN_USE: C2RustUnnamed_4 = 14;
pub const G_DBUS_ERROR_NO_NETWORK: C2RustUnnamed_4 = 13;
pub const G_DBUS_ERROR_TIMEOUT: C2RustUnnamed_4 = 12;
pub const G_DBUS_ERROR_NO_SERVER: C2RustUnnamed_4 = 11;
pub const G_DBUS_ERROR_AUTH_FAILED: C2RustUnnamed_4 = 10;
pub const G_DBUS_ERROR_ACCESS_DENIED: C2RustUnnamed_4 = 9;
pub const G_DBUS_ERROR_LIMITS_EXCEEDED: C2RustUnnamed_4 = 8;
pub const G_DBUS_ERROR_NOT_SUPPORTED: C2RustUnnamed_4 = 7;
pub const G_DBUS_ERROR_BAD_ADDRESS: C2RustUnnamed_4 = 6;
pub const G_DBUS_ERROR_IO_ERROR: C2RustUnnamed_4 = 5;
pub const G_DBUS_ERROR_NO_REPLY: C2RustUnnamed_4 = 4;
pub const G_DBUS_ERROR_NAME_HAS_NO_OWNER: C2RustUnnamed_4 = 3;
pub const G_DBUS_ERROR_SERVICE_UNKNOWN: C2RustUnnamed_4 = 2;
pub const G_DBUS_ERROR_NO_MEMORY: C2RustUnnamed_4 = 1;
pub const G_DBUS_ERROR_FAILED: C2RustUnnamed_4 = 0;
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GDBusSignalFlags = ::core::ffi::c_uint;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH: GDBusSignalFlags = 4;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE: GDBusSignalFlags = 2;
pub const G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE: GDBusSignalFlags = 1;
pub const G_DBUS_SIGNAL_FLAGS_NONE: GDBusSignalFlags = 0;
pub type GApplicationFlags = ::core::ffi::c_uint;
pub const G_APPLICATION_REPLACE: GApplicationFlags = 256;
pub const G_APPLICATION_ALLOW_REPLACEMENT: GApplicationFlags = 128;
pub const G_APPLICATION_CAN_OVERRIDE_APP_ID: GApplicationFlags = 64;
pub const G_APPLICATION_NON_UNIQUE: GApplicationFlags = 32;
pub const G_APPLICATION_SEND_ENVIRONMENT: GApplicationFlags = 16;
pub const G_APPLICATION_HANDLES_COMMAND_LINE: GApplicationFlags = 8;
pub const G_APPLICATION_HANDLES_OPEN: GApplicationFlags = 4;
pub const G_APPLICATION_IS_LAUNCHER: GApplicationFlags = 2;
pub const G_APPLICATION_IS_SERVICE: GApplicationFlags = 1;
pub const G_APPLICATION_DEFAULT_FLAGS: GApplicationFlags = 0;
pub const G_APPLICATION_FLAGS_NONE: GApplicationFlags = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GRemoteActionGroup = _GRemoteActionGroup;
pub type GDBusActionGroup = _GDBusActionGroup;
pub type GActionGroup = _GActionGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplication {
    pub parent_instance: GObject,
    pub priv_0: *mut GApplicationPrivate,
}
pub type GApplicationPrivate = _GApplicationPrivate;
pub type GApplication = _GApplication;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplicationCommandLine {
    pub parent_instance: GObject,
    pub priv_0: *mut GApplicationCommandLinePrivate,
}
pub type GApplicationCommandLinePrivate = _GApplicationCommandLinePrivate;
pub type GApplicationCommandLine = _GApplicationCommandLine;
pub type GFile = _GFile;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
pub type GDBusMessage = _GDBusMessage;
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
pub struct _GApplicationImpl {
    pub session_bus: *mut GDBusConnection,
    pub exported_actions: *mut GActionGroup,
    pub bus_name: *const gchar,
    pub name_lost_signal: guint,
    pub object_path: *mut gchar,
    pub object_id: guint,
    pub fdo_object_id: guint,
    pub actions_id: guint,
    pub properties_live: gboolean,
    pub primary: gboolean,
    pub busy: gboolean,
    pub registered: gboolean,
    pub app: *mut GApplication,
}
pub type GApplicationImpl = _GApplicationImpl;
pub type GApplicationClass = _GApplicationClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplicationClass {
    pub parent_class: GObjectClass,
    pub startup: Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
    pub activate: Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
    pub open:
        Option<unsafe extern "C" fn(*mut GApplication, *mut *mut GFile, gint, *const gchar) -> ()>,
    pub command_line: Option<
        unsafe extern "C" fn(*mut GApplication, *mut GApplicationCommandLine) -> ::core::ffi::c_int,
    >,
    pub local_command_line: Option<
        unsafe extern "C" fn(
            *mut GApplication,
            *mut *mut *mut gchar,
            *mut ::core::ffi::c_int,
        ) -> gboolean,
    >,
    pub before_emit: Option<unsafe extern "C" fn(*mut GApplication, *mut GVariant) -> ()>,
    pub after_emit: Option<unsafe extern "C" fn(*mut GApplication, *mut GVariant) -> ()>,
    pub add_platform_data:
        Option<unsafe extern "C" fn(*mut GApplication, *mut GVariantBuilder) -> ()>,
    pub quit_mainloop: Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
    pub run_mainloop: Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
    pub shutdown: Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
    pub dbus_register: Option<
        unsafe extern "C" fn(
            *mut GApplication,
            *mut GDBusConnection,
            *const gchar,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub dbus_unregister:
        Option<unsafe extern "C" fn(*mut GApplication, *mut GDBusConnection, *const gchar) -> ()>,
    pub handle_local_options:
        Option<unsafe extern "C" fn(*mut GApplication, *mut GVariantDict) -> gint>,
    pub name_lost: Option<unsafe extern "C" fn(*mut GApplication) -> gboolean>,
    pub padding: [gpointer; 7],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GDBusCommandLine {
    pub parent_instance: GApplicationCommandLine,
    pub invocation: *mut GDBusMethodInvocation,
    pub connection: *mut GDBusConnection,
    pub bus_name: *const gchar,
    pub object_path: *const gchar,
}
pub type GDBusCommandLineClass = GApplicationCommandLineClass;
pub type GApplicationCommandLineClass = _GApplicationCommandLineClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplicationCommandLineClass {
    pub parent_class: GObjectClass,
    pub print_literal:
        Option<unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> ()>,
    pub printerr_literal:
        Option<unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> ()>,
    pub get_stdin: Option<unsafe extern "C" fn(*mut GApplicationCommandLine) -> *mut GInputStream>,
    pub done: Option<unsafe extern "C" fn(*mut GApplicationCommandLine) -> ()>,
    pub padding: [gpointer; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandLineData {
    pub loop_0: *mut GMainLoop,
    pub status: ::core::ffi::c_int,
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
pub const G_VARIANT_TYPE_ARRAY: *const GVariantType =
    b"a*\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_STRING_ARRAY: *const GVariantType =
    b"as\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
static mut safe_c2rust_org_gtk_Application_xml: [gchar; 599] = unsafe {
    ::core::mem::transmute::<
        [u8; 599],
        [gchar; 599],
    >(
        *b"<node><interface name='org.gtk.Application'><method name='Activate'><arg type='a{sv}' name='platform-data' direction='in'/></method><method name='Open'><arg type='as' name='uris' direction='in'/><arg type='s' name='hint' direction='in'/><arg type='a{sv}' name='platform-data' direction='in'/></method><method name='CommandLine'><arg type='o' name='path' direction='in'/><arg type='aay' name='arguments' direction='in'/><arg type='a{sv}' name='platform-data' direction='in'/><arg type='i' name='exit-status' direction='out'/></method><property name='Busy' type='b' access='read'/></interface></node>\0",
    )
};
static mut safe_c2rust_org_gtk_Application: *mut GDBusInterfaceInfo =
    ::core::ptr::null::<GDBusInterfaceInfo>() as *mut GDBusInterfaceInfo;
static mut safe_c2rust_org_freedesktop_Application_xml: [gchar; 478] = unsafe {
    ::core::mem::transmute::<
        [u8; 478],
        [gchar; 478],
    >(
        *b"<node><interface name='org.freedesktop.Application'><method name='Activate'><arg type='a{sv}' name='platform-data' direction='in'/></method><method name='Open'><arg type='as' name='uris' direction='in'/><arg type='a{sv}' name='platform-data' direction='in'/></method><method name='ActivateAction'><arg type='s' name='action-name' direction='in'/><arg type='av' name='parameter' direction='in'/><arg type='a{sv}' name='platform-data' direction='in'/></method></interface></node>\0",
    )
};
static mut safe_c2rust_org_freedesktop_Application: *mut GDBusInterfaceInfo =
    ::core::ptr::null::<GDBusInterfaceInfo>() as *mut GDBusInterfaceInfo;
static mut safe_c2rust_org_gtk_private_CommandLine_xml: [gchar; 227] = unsafe {
    ::core::mem::transmute::<
        [u8; 227],
        [gchar; 227],
    >(
        *b"<node><interface name='org.gtk.private.CommandLine'><method name='Print'><arg type='s' name='message' direction='in'/></method><method name='PrintError'><arg type='s' name='message' direction='in'/></method></interface></node>\0",
    )
};
static mut safe_c2rust_org_gtk_private_CommandLine: *mut GDBusInterfaceInfo =
    ::core::ptr::null::<GDBusInterfaceInfo>() as *mut GDBusInterfaceInfo;
unsafe extern "C" fn safe_c2rust_g_application_impl_get_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> *mut GVariant {
    let mut impl_0: *mut GApplicationImpl = user_data as *mut GApplicationImpl;
    if strcmp(
        property_name as *const ::core::ffi::c_char,
        b"Busy\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return g_variant_new_boolean((*impl_0).busy);
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationimpl-dbus.c\0"
            as *const u8 as *const ::core::ffi::c_char,
        148 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn safe_c2rust_send_property_change(mut impl_0: *mut GApplicationImpl) {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    g_variant_builder_init(&raw mut builder, G_VARIANT_TYPE_ARRAY);
    g_variant_builder_add(
        &raw mut builder,
        b"{sv}\0" as *const u8 as *const gchar,
        b"Busy\0" as *const u8 as *const ::core::ffi::c_char,
        g_variant_new_boolean((*impl_0).busy),
    );
    g_dbus_connection_emit_signal(
        (*impl_0).session_bus,
        ::core::ptr::null::<gchar>(),
        (*impl_0).object_path,
        b"org.freedesktop.DBus.Properties\0" as *const u8 as *const gchar,
        b"PropertiesChanged\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(sa{sv}as)\0" as *const u8 as *const gchar,
            b"org.gtk.Application\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut builder,
            NULL_0,
        ),
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
unsafe extern "C" fn safe_c2rust_g_application_impl_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut impl_0: *mut GApplicationImpl = user_data as *mut GApplicationImpl;
    let mut class: *mut GApplicationClass = ::core::ptr::null_mut::<GApplicationClass>();
    class = (*((*impl_0).app as *mut GTypeInstance)).g_class as *mut GApplicationClass;
    if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"Activate\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut platform_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        g_variant_get(
            parameters,
            b"(@a{sv})\0" as *const u8 as *const gchar,
            &raw mut platform_data,
        );
        (*class).before_emit.expect("non-null function pointer")((*impl_0).app, platform_data);
        g_signal_emit_by_name(
            (*impl_0).app as gpointer,
            b"activate\0" as *const u8 as *const gchar,
        );
        (*class).after_emit.expect("non-null function pointer")((*impl_0).app, platform_data);
        g_variant_unref(platform_data);
        g_dbus_method_invocation_return_value(invocation, ::core::ptr::null_mut::<GVariant>());
    } else if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"Open\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut flags: GApplicationFlags = G_APPLICATION_FLAGS_NONE;
        let mut platform_data_0: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut hint: *const gchar = ::core::ptr::null::<gchar>();
        let mut array: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut files: *mut *mut GFile = ::core::ptr::null_mut::<*mut GFile>();
        let mut n: gint = 0;
        let mut i: gint = 0;
        flags = g_application_get_flags((*impl_0).app);
        if flags as ::core::ffi::c_uint
            & G_APPLICATION_HANDLES_OPEN as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            g_dbus_method_invocation_return_error(
                invocation,
                g_dbus_error_quark(),
                G_DBUS_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                b"Application does not open files\0" as *const u8 as *const gchar,
            );
            return;
        }
        if strcmp(
            interface_name as *const ::core::ffi::c_char,
            b"org.freedesktop.Application\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            g_variant_get(
                parameters,
                b"(@as@a{sv})\0" as *const u8 as *const gchar,
                &raw mut array,
                &raw mut platform_data_0,
            );
            hint = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        } else {
            g_variant_get(
                parameters,
                b"(@as&s@a{sv})\0" as *const u8 as *const gchar,
                &raw mut array,
                &raw mut hint,
                &raw mut platform_data_0,
            );
        }
        n = g_variant_n_children(array) as gint;
        files = ({
            let mut __n: gsize = (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut GFile>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut GFile;
        i = 0 as ::core::ffi::c_int as gint;
        while i < n {
            let mut uri: *const gchar = ::core::ptr::null::<gchar>();
            g_variant_get_child(
                array,
                i as gsize,
                b"&s\0" as *const u8 as *const gchar,
                &raw mut uri,
            );
            let ref mut fresh0 = *files.offset(i as isize);
            *fresh0 = g_file_new_for_uri(uri as *const ::core::ffi::c_char);
            i += 1;
        }
        g_variant_unref(array);
        let ref mut fresh1 = *files.offset(n as isize);
        *fresh1 = ::core::ptr::null_mut::<GFile>();
        (*class).before_emit.expect("non-null function pointer")((*impl_0).app, platform_data_0);
        g_signal_emit_by_name(
            (*impl_0).app as gpointer,
            b"open\0" as *const u8 as *const gchar,
            files,
            n,
            hint,
        );
        (*class).after_emit.expect("non-null function pointer")((*impl_0).app, platform_data_0);
        g_variant_unref(platform_data_0);
        i = 0 as ::core::ffi::c_int as gint;
        while i < n {
            g_object_unref(*files.offset(i as isize) as gpointer);
            i += 1;
        }
        g_free(files as gpointer);
        g_dbus_method_invocation_return_value(invocation, ::core::ptr::null_mut::<GVariant>());
    } else if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"CommandLine\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut flags_0: GApplicationFlags = G_APPLICATION_FLAGS_NONE;
        let mut cmdline: *mut GApplicationCommandLine =
            ::core::ptr::null_mut::<GApplicationCommandLine>();
        let mut platform_data_1: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut status: ::core::ffi::c_int = 0;
        flags_0 = g_application_get_flags((*impl_0).app);
        if flags_0 as ::core::ffi::c_uint
            & G_APPLICATION_HANDLES_COMMAND_LINE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            g_dbus_method_invocation_return_error(
                invocation,
                g_dbus_error_quark(),
                G_DBUS_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                b"Application does not handle command line arguments\0" as *const u8
                    as *const gchar,
            );
            return;
        }
        cmdline = safe_c2rust_g_dbus_command_line_new(invocation);
        platform_data_1 = g_variant_get_child_value(parameters, 2 as gsize);
        (*class).before_emit.expect("non-null function pointer")((*impl_0).app, platform_data_1);
        g_signal_emit_by_name(
            (*impl_0).app as gpointer,
            b"command-line\0" as *const u8 as *const gchar,
            cmdline,
            &raw mut status,
        );
        g_application_command_line_set_exit_status(cmdline, status);
        (*class).after_emit.expect("non-null function pointer")((*impl_0).app, platform_data_1);
        g_variant_unref(platform_data_1);
        g_object_unref(cmdline as gpointer);
    } else if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"ActivateAction\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut parameter: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut platform_data_2: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
        let mut name: *const gchar = ::core::ptr::null::<gchar>();
        let mut parameter_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
        g_variant_get(
            parameters,
            b"(&sav@a{sv})\0" as *const u8 as *const gchar,
            &raw mut name,
            &raw mut iter,
            &raw mut platform_data_2,
        );
        g_variant_iter_next(
            iter,
            b"v\0" as *const u8 as *const gchar,
            &raw mut parameter,
        );
        g_variant_iter_free(iter);
        if g_action_group_query_action(
            (*impl_0).exported_actions,
            name,
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
                name,
            );
            let mut _pp: *mut *mut GVariant = &raw mut parameter;
            let mut _ptr: *mut GVariant = *_pp;
            *_pp = ::core::ptr::null_mut::<GVariant>();
            if !_ptr.is_null() {
                g_variant_unref(_ptr as *mut GVariant);
            }
            g_variant_unref(platform_data_2);
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
                name,
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
            g_variant_unref(platform_data_2);
            return;
        }
        (*class).before_emit.expect("non-null function pointer")((*impl_0).app, platform_data_2);
        g_action_group_activate_action((*impl_0).exported_actions, name, parameter);
        (*class).after_emit.expect("non-null function pointer")((*impl_0).app, platform_data_2);
        if !parameter.is_null() {
            g_variant_unref(parameter);
        }
        g_variant_unref(platform_data_2);
        g_dbus_method_invocation_return_value(invocation, ::core::ptr::null_mut::<GVariant>());
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationimpl-dbus.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            334 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    };
}
unsafe extern "C" fn safe_c2rust_application_path_from_appid(
    mut appid: *const gchar,
) -> *mut gchar {
    let mut appid_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut iter: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if appid.is_null() {
        return safe_c2rust_g_strdup_inline(
            b"/org/gtk/Application/anonymous\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut gchar;
    }
    appid_path = g_strconcat(b"/\0" as *const u8 as *const gchar, appid, NULL_0);
    iter = appid_path;
    while *iter != 0 {
        if *iter as ::core::ffi::c_int == '.' as i32 {
            *iter = '/' as i32 as gchar;
        }
        if *iter as ::core::ffi::c_int == '-' as i32 {
            *iter = '_' as i32 as gchar;
        }
        iter = iter.offset(1);
    }
    return appid_path;
}
unsafe extern "C" fn safe_c2rust_name_lost(
    mut bus: *mut GDBusConnection,
    mut sender_name: *const ::core::ffi::c_char,
    mut object_path: *const ::core::ffi::c_char,
    mut interface_name: *const ::core::ffi::c_char,
    mut signal_name: *const ::core::ffi::c_char,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut impl_0: *mut GApplicationImpl = user_data as *mut GApplicationImpl;
    let mut handled: gboolean = 0;
    (*impl_0).primary = FALSE as gboolean;
    safe_c2rust_g_application_impl_stop_primary(impl_0);
    g_signal_emit_by_name(
        (*impl_0).app as gpointer,
        b"name-lost\0" as *const u8 as *const gchar,
        &raw mut handled,
    );
}
unsafe extern "C" fn safe_c2rust_g_application_impl_attempt_primary(
    mut impl_0: *mut GApplicationImpl,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    static mut safe_c2rust_vtable: GDBusInterfaceVTable = unsafe {
        _GDBusInterfaceVTable {
            method_call: Some(
                safe_c2rust_g_application_impl_method_call
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
            get_property: Some(
                safe_c2rust_g_application_impl_get_property
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut *mut GError,
                        gpointer,
                    ) -> *mut GVariant,
            ),
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
    let mut app_class: *mut GApplicationClass =
        (*((*impl_0).app as *mut GTypeInstance)).g_class as *mut GApplicationClass;
    let mut name_owner_flags: GBusNameOwnerFlags = G_BUS_NAME_OWNER_FLAGS_NONE;
    let mut app_flags: GApplicationFlags = G_APPLICATION_FLAGS_NONE;
    let mut reply: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut rval: guint32 = 0;
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_org_gtk_Application.is_null() {
        let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut info: *mut GDBusNodeInfo = ::core::ptr::null_mut::<GDBusNodeInfo>();
        info = g_dbus_node_info_new_for_xml(
            &raw const safe_c2rust_org_gtk_Application_xml as *const gchar,
            &raw mut my_error,
        );
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if info.is_null() {
                _g_boolean_var_10 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_10 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_10
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
        safe_c2rust_org_gtk_Application = g_dbus_node_info_lookup_interface(
            info,
            b"org.gtk.Application\0" as *const u8 as *const gchar,
        );
        if ({
            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
            if !safe_c2rust_org_gtk_Application.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationimpl-dbus.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                414 as ::core::ffi::c_int,
                G_STRFUNC,
                b"org_gtk_Application != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        g_dbus_interface_info_ref(safe_c2rust_org_gtk_Application);
        g_dbus_node_info_unref(info);
        info = g_dbus_node_info_new_for_xml(
            &raw const safe_c2rust_org_freedesktop_Application_xml as *const gchar,
            &raw mut my_error,
        );
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if info.is_null() {
                _g_boolean_var_12 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_12 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_12
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
        safe_c2rust_org_freedesktop_Application = g_dbus_node_info_lookup_interface(
            info,
            b"org.freedesktop.Application\0" as *const u8 as *const gchar,
        );
        if ({
            let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
            if !safe_c2rust_org_freedesktop_Application.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationimpl-dbus.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                422 as ::core::ffi::c_int,
                G_STRFUNC,
                b"org_freedesktop_Application != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        g_dbus_interface_info_ref(safe_c2rust_org_freedesktop_Application);
        g_dbus_node_info_unref(info);
    }
    (*impl_0).object_id = g_dbus_connection_register_object(
        (*impl_0).session_bus,
        (*impl_0).object_path,
        safe_c2rust_org_gtk_Application,
        &raw const safe_c2rust_vtable,
        impl_0 as gpointer,
        None,
        error,
    );
    if (*impl_0).object_id == 0 as guint {
        return FALSE;
    }
    (*impl_0).fdo_object_id = g_dbus_connection_register_object(
        (*impl_0).session_bus,
        (*impl_0).object_path,
        safe_c2rust_org_freedesktop_Application,
        &raw const safe_c2rust_vtable,
        impl_0 as gpointer,
        None,
        error,
    );
    if (*impl_0).fdo_object_id == 0 as guint {
        return FALSE;
    }
    (*impl_0).actions_id = g_dbus_connection_export_action_group(
        (*impl_0).session_bus,
        (*impl_0).object_path,
        (*impl_0).exported_actions,
        error,
    );
    if (*impl_0).actions_id == 0 as guint {
        return FALSE;
    }
    (*impl_0).registered = TRUE as gboolean;
    if (*app_class)
        .dbus_register
        .expect("non-null function pointer")(
        (*impl_0).app,
        (*impl_0).session_bus,
        (*impl_0).object_path,
        &raw mut local_error,
    ) == 0
    {
        if ({
            let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
            if !local_error.is_null() {
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
                b"local_error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
        g_propagate_error(
            error,
            safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer) as *mut GError,
        );
        return FALSE;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if local_error.is_null() {
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
            b"local_error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*impl_0).bus_name.is_null() {
        (*impl_0).primary = TRUE as gboolean;
        return TRUE;
    }
    name_owner_flags = G_BUS_NAME_OWNER_FLAGS_DO_NOT_QUEUE;
    app_flags = g_application_get_flags((*impl_0).app);
    if app_flags as ::core::ffi::c_uint
        & G_APPLICATION_ALLOW_REPLACEMENT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        (*impl_0).name_lost_signal = g_dbus_connection_signal_subscribe(
            (*impl_0).session_bus,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"NameLost\0" as *const u8 as *const gchar,
            b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
            (*impl_0).bus_name,
            G_DBUS_SIGNAL_FLAGS_NONE,
            Some(
                safe_c2rust_name_lost
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut GVariant,
                        gpointer,
                    ) -> (),
            ),
            impl_0 as gpointer,
            None,
        );
        name_owner_flags = ::core::mem::transmute::<::core::ffi::c_uint, GBusNameOwnerFlags>(
            name_owner_flags as ::core::ffi::c_uint
                | G_BUS_NAME_OWNER_FLAGS_ALLOW_REPLACEMENT as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
        );
    }
    if app_flags as ::core::ffi::c_uint
        & G_APPLICATION_REPLACE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        name_owner_flags = ::core::mem::transmute::<::core::ffi::c_uint, GBusNameOwnerFlags>(
            name_owner_flags as ::core::ffi::c_uint
                | G_BUS_NAME_OWNER_FLAGS_REPLACE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    reply = g_dbus_connection_call_sync(
        (*impl_0).session_bus,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"RequestName\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(su)\0" as *const u8 as *const gchar,
            (*impl_0).bus_name,
            name_owner_flags as ::core::ffi::c_uint,
        ),
        g_variant_type_checked_(b"(u)\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if reply.is_null() {
        return FALSE;
    }
    g_variant_get(reply, b"(u)\0" as *const u8 as *const gchar, &raw mut rval);
    g_variant_unref(reply);
    (*impl_0).primary = (rval != 3 as guint32) as ::core::ffi::c_int as gboolean;
    if (*impl_0).primary == 0 && (*impl_0).name_lost_signal != 0 {
        g_dbus_connection_signal_unsubscribe((*impl_0).session_bus, (*impl_0).name_lost_signal);
        (*impl_0).name_lost_signal = 0 as guint;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_application_impl_stop_primary(
    mut impl_0: *mut GApplicationImpl,
) {
    let mut app_class: *mut GApplicationClass =
        (*((*impl_0).app as *mut GTypeInstance)).g_class as *mut GApplicationClass;
    if (*impl_0).registered != 0 {
        (*app_class)
            .dbus_unregister
            .expect("non-null function pointer")(
            (*impl_0).app,
            (*impl_0).session_bus,
            (*impl_0).object_path,
        );
        (*impl_0).registered = FALSE as gboolean;
    }
    if (*impl_0).object_id != 0 {
        g_dbus_connection_unregister_object((*impl_0).session_bus, (*impl_0).object_id);
        (*impl_0).object_id = 0 as guint;
    }
    if (*impl_0).fdo_object_id != 0 {
        g_dbus_connection_unregister_object((*impl_0).session_bus, (*impl_0).fdo_object_id);
        (*impl_0).fdo_object_id = 0 as guint;
    }
    if (*impl_0).actions_id != 0 {
        g_dbus_connection_unexport_action_group((*impl_0).session_bus, (*impl_0).actions_id);
        (*impl_0).actions_id = 0 as guint;
    }
    if (*impl_0).name_lost_signal != 0 {
        g_dbus_connection_signal_unsubscribe((*impl_0).session_bus, (*impl_0).name_lost_signal);
        (*impl_0).name_lost_signal = 0 as guint;
    }
    if (*impl_0).primary != 0 && !(*impl_0).bus_name.is_null() {
        g_dbus_connection_call(
            (*impl_0).session_bus,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"ReleaseName\0" as *const u8 as *const gchar,
            g_variant_new(b"(s)\0" as *const u8 as *const gchar, (*impl_0).bus_name),
            ::core::ptr::null::<GVariantType>(),
            G_DBUS_CALL_FLAGS_NONE,
            -(1 as gint),
            ::core::ptr::null_mut::<GCancellable>(),
            None,
            NULL_0,
        );
        (*impl_0).primary = FALSE as gboolean;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_impl_set_busy_state(
    mut impl_0: *mut GApplicationImpl,
    mut busy: gboolean,
) {
    if (*impl_0).busy != busy {
        (*impl_0).busy = busy;
        safe_c2rust_send_property_change(impl_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_impl_destroy(mut impl_0: *mut GApplicationImpl) {
    safe_c2rust_g_application_impl_stop_primary(impl_0);
    if !(*impl_0).session_bus.is_null() {
        g_object_unref((*impl_0).session_bus as gpointer);
    }
    g_free((*impl_0).object_path as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GApplicationImpl>() as gsize,
        impl_0 as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_impl_register(
    mut application: *mut GApplication,
    mut appid: *const gchar,
    mut flags: GApplicationFlags,
    mut exported_actions: *mut GActionGroup,
    mut remote_actions: *mut *mut GRemoteActionGroup,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GApplicationImpl {
    let mut actions: *mut GDBusActionGroup = ::core::ptr::null_mut::<GDBusActionGroup>();
    let mut impl_0: *mut GApplicationImpl = ::core::ptr::null_mut::<GApplicationImpl>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & G_APPLICATION_NON_UNIQUE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            || !appid.is_null()
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationimpl-dbus.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            626 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(flags & G_APPLICATION_NON_UNIQUE) || appid != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    impl_0 = ({
        let mut __s: gsize = ::core::mem::size_of::<GApplicationImpl>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GApplicationImpl;
    (*impl_0).app = application;
    (*impl_0).exported_actions = exported_actions;
    if !(flags as ::core::ffi::c_uint)
        & G_APPLICATION_NON_UNIQUE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        (*impl_0).bus_name = appid;
    }
    (*impl_0).session_bus = g_bus_get_sync(
        G_BUS_TYPE_SESSION,
        cancellable,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if (*impl_0).session_bus.is_null() {
        *remote_actions = ::core::ptr::null_mut::<GRemoteActionGroup>();
        return impl_0;
    }
    (*impl_0).object_path = safe_c2rust_application_path_from_appid(appid);
    if !(flags as ::core::ffi::c_uint)
        & G_APPLICATION_IS_LAUNCHER as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        if safe_c2rust_g_application_impl_attempt_primary(impl_0, cancellable, error) == 0 {
            safe_c2rust_g_application_impl_destroy(impl_0);
            return ::core::ptr::null_mut::<GApplicationImpl>();
        }
        if (*impl_0).primary != 0 {
            return impl_0;
        }
        safe_c2rust_g_application_impl_stop_primary(impl_0);
        if flags as ::core::ffi::c_uint
            & G_APPLICATION_IS_SERVICE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            g_set_error(
                error,
                g_dbus_error_quark(),
                G_DBUS_ERROR_FAILED as ::core::ffi::c_int as gint,
                b"Unable to acquire bus name '%s'\0" as *const u8 as *const gchar,
                appid,
            );
            safe_c2rust_g_application_impl_destroy(impl_0);
            return ::core::ptr::null_mut::<GApplicationImpl>();
        }
    }
    actions = g_dbus_action_group_get(
        (*impl_0).session_bus,
        (*impl_0).bus_name,
        (*impl_0).object_path,
    );
    if g_dbus_action_group_sync(actions, cancellable, error) == 0 {
        safe_c2rust_g_application_impl_destroy(impl_0);
        g_object_unref(actions as gpointer);
        return ::core::ptr::null_mut::<GApplicationImpl>();
    }
    *remote_actions = actions as *mut ::core::ffi::c_void as *mut GRemoteActionGroup;
    return impl_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_impl_activate(
    mut impl_0: *mut GApplicationImpl,
    mut platform_data: *mut GVariant,
) {
    g_dbus_connection_call(
        (*impl_0).session_bus,
        (*impl_0).bus_name,
        (*impl_0).object_path,
        b"org.gtk.Application\0" as *const u8 as *const gchar,
        b"Activate\0" as *const u8 as *const gchar,
        g_variant_new(b"(@a{sv})\0" as *const u8 as *const gchar, platform_data),
        ::core::ptr::null::<GVariantType>(),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        None,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_impl_open(
    mut impl_0: *mut GApplicationImpl,
    mut files: *mut *mut GFile,
    mut n_files: gint,
    mut hint: *const gchar,
    mut platform_data: *mut GVariant,
) {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut i: gint = 0;
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"(assa{sv})\0" as *const u8 as *const gchar),
    );
    g_variant_builder_open(&raw mut builder, G_VARIANT_TYPE_STRING_ARRAY);
    i = 0 as ::core::ffi::c_int as gint;
    while i < n_files {
        let mut uri: *mut gchar = g_file_get_uri(*files.offset(i as isize)) as *mut gchar;
        g_variant_builder_add(&raw mut builder, b"s\0" as *const u8 as *const gchar, uri);
        g_free(uri as gpointer);
        i += 1;
    }
    g_variant_builder_close(&raw mut builder);
    g_variant_builder_add(&raw mut builder, b"s\0" as *const u8 as *const gchar, hint);
    g_variant_builder_add_value(&raw mut builder, platform_data);
    g_dbus_connection_call(
        (*impl_0).session_bus,
        (*impl_0).bus_name,
        (*impl_0).object_path,
        b"org.gtk.Application\0" as *const u8 as *const gchar,
        b"Open\0" as *const u8 as *const gchar,
        g_variant_builder_end(&raw mut builder),
        ::core::ptr::null::<GVariantType>(),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        None,
        NULL_0,
    );
}
unsafe extern "C" fn safe_c2rust_g_application_impl_cmdline_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut message: *const gchar = ::core::ptr::null::<gchar>();
    g_variant_get_child(
        parameters,
        0 as gsize,
        b"&s\0" as *const u8 as *const gchar,
        &raw mut message,
    );
    if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"Print\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        g_print(b"%s\0" as *const u8 as *const gchar, message);
    } else if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"PrintError\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        g_printerr(b"%s\0" as *const u8 as *const gchar, message);
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationimpl-dbus.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            759 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    g_dbus_method_invocation_return_value(invocation, ::core::ptr::null_mut::<GVariant>());
}
unsafe extern "C" fn safe_c2rust_g_application_impl_cmdline_done(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut CommandLineData = user_data as *mut CommandLineData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut reply: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    reply = g_dbus_connection_call_with_unix_fd_list_finish(
        source as *mut ::core::ffi::c_void as *mut GDBusConnection,
        ::core::ptr::null_mut::<*mut GUnixFDList>(),
        result,
        &raw mut error,
    );
    if !reply.is_null() {
        g_variant_get(
            reply,
            b"(i)\0" as *const u8 as *const gchar,
            &raw mut (*data).status,
        );
        g_variant_unref(reply);
    } else {
        g_printerr(b"%s\n\0" as *const u8 as *const gchar, (*error).message);
        g_error_free(error);
        (*data).status = 1 as ::core::ffi::c_int;
    }
    g_main_loop_quit((*data).loop_0);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_impl_command_line(
    mut impl_0: *mut GApplicationImpl,
    mut arguments: *const *const gchar,
    mut platform_data: *mut GVariant,
) -> ::core::ffi::c_int {
    static mut safe_c2rust_vtable: GDBusInterfaceVTable = unsafe {
        _GDBusInterfaceVTable {
            method_call: Some(
                safe_c2rust_g_application_impl_cmdline_method_call
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
    let mut object_path: *const gchar =
        b"/org/gtk/Application/CommandLine\0" as *const u8 as *const gchar;
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    let mut data: CommandLineData = CommandLineData {
        loop_0: ::core::ptr::null_mut::<GMainLoop>(),
        status: 0,
    };
    let mut object_id: guint = 0;
    context = g_main_context_new();
    data.loop_0 = g_main_loop_new(context, FALSE);
    g_main_context_push_thread_default(context);
    if safe_c2rust_org_gtk_private_CommandLine.is_null() {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut info: *mut GDBusNodeInfo = ::core::ptr::null_mut::<GDBusNodeInfo>();
        info = g_dbus_node_info_new_for_xml(
            &raw const safe_c2rust_org_gtk_private_CommandLine_xml as *const gchar,
            &raw mut error,
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
                (*error).message,
            );
            loop {}
        }
        safe_c2rust_org_gtk_private_CommandLine = g_dbus_node_info_lookup_interface(
            info,
            b"org.gtk.private.CommandLine\0" as *const u8 as *const gchar,
        );
        if ({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if !safe_c2rust_org_gtk_private_CommandLine.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationimpl-dbus.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                828 as ::core::ffi::c_int,
                G_STRFUNC,
                b"org_gtk_private_CommandLine != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        g_dbus_interface_info_ref(safe_c2rust_org_gtk_private_CommandLine);
        g_dbus_node_info_unref(info);
    }
    object_id = g_dbus_connection_register_object(
        (*impl_0).session_bus,
        object_path,
        safe_c2rust_org_gtk_private_CommandLine,
        &raw const safe_c2rust_vtable,
        &raw mut data as gpointer,
        None,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if object_id != 0 as guint {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationimpl-dbus.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            837 as ::core::ffi::c_int,
            G_STRFUNC,
            b"object_id != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut error_0: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut fd_list: *mut GUnixFDList = ::core::ptr::null_mut::<GUnixFDList>();
    fd_list = g_unix_fd_list_new();
    g_unix_fd_list_append(fd_list, 0 as gint, &raw mut error_0);
    if !error_0.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplicationimpl-dbus.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            849 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error_0,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    g_dbus_connection_call_with_unix_fd_list(
        (*impl_0).session_bus,
        (*impl_0).bus_name,
        (*impl_0).object_path,
        b"org.gtk.Application\0" as *const u8 as *const gchar,
        b"CommandLine\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(o^aay@a{sv})\0" as *const u8 as *const gchar,
            object_path,
            arguments,
            platform_data,
        ),
        g_variant_type_checked_(b"(i)\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        G_MAXINT,
        fd_list,
        ::core::ptr::null_mut::<GCancellable>(),
        Some(
            safe_c2rust_g_application_impl_cmdline_done
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        &raw mut data as gpointer,
    );
    g_object_unref(fd_list as gpointer);
    g_main_loop_run(data.loop_0);
    g_main_context_pop_thread_default(context);
    g_main_context_unref(context);
    g_main_loop_unref(data.loop_0);
    return data.status;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_impl_flush(mut impl_0: *mut GApplicationImpl) {
    if !(*impl_0).session_bus.is_null() {
        g_dbus_connection_flush_sync(
            (*impl_0).session_bus,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_impl_get_dbus_connection(
    mut impl_0: *mut GApplicationImpl,
) -> *mut GDBusConnection {
    return (*impl_0).session_bus;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_impl_get_dbus_object_path(
    mut impl_0: *mut GApplicationImpl,
) -> *const gchar {
    return (*impl_0).object_path;
}
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_command_line_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusCommandLine_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusCommandLine_private_offset,
        );
    }
    safe_c2rust_g_dbus_command_line_class_init(klass as *mut GApplicationCommandLineClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_application_command_line_get_type(),
        g_intern_static_string(b"GDBusCommandLine\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusCommandLineClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_command_line_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusCommandLine>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusCommandLine) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_command_line_init
                    as unsafe extern "C" fn(*mut GDBusCommandLine) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_command_line_get_type_once();
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
static mut safe_c2rust_GDBusCommandLine_private_offset: gint = 0;
static mut safe_c2rust_g_dbus_command_line_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_print_literal(
    mut cmdline: *mut GApplicationCommandLine,
    mut message: *const gchar,
) {
    let mut gdbcl: *mut GDBusCommandLine = cmdline as *mut GDBusCommandLine;
    g_dbus_connection_call(
        (*gdbcl).connection,
        (*gdbcl).bus_name,
        (*gdbcl).object_path,
        b"org.gtk.private.CommandLine\0" as *const u8 as *const gchar,
        b"Print\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, message),
        ::core::ptr::null::<GVariantType>(),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        None,
        NULL_0,
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_printerr_literal(
    mut cmdline: *mut GApplicationCommandLine,
    mut message: *const gchar,
) {
    let mut gdbcl: *mut GDBusCommandLine = cmdline as *mut GDBusCommandLine;
    g_dbus_connection_call(
        (*gdbcl).connection,
        (*gdbcl).bus_name,
        (*gdbcl).object_path,
        b"org.gtk.private.CommandLine\0" as *const u8 as *const gchar,
        b"PrintError\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, message),
        ::core::ptr::null::<GVariantType>(),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        None,
        NULL_0,
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_get_stdin(
    mut cmdline: *mut GApplicationCommandLine,
) -> *mut GInputStream {
    let mut gdbcl: *mut GDBusCommandLine = cmdline as *mut GDBusCommandLine;
    let mut result: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut fd_list: *mut GUnixFDList = ::core::ptr::null_mut::<GUnixFDList>();
    message = g_dbus_method_invocation_get_message((*gdbcl).invocation);
    fd_list = g_dbus_message_get_unix_fd_list(message);
    if !fd_list.is_null() && g_unix_fd_list_get_length(fd_list) != 0 {
        let mut fds: *mut gint = ::core::ptr::null_mut::<gint>();
        let mut n_fds: gint = 0;
        let mut i: gint = 0;
        fds = g_unix_fd_list_steal_fds(fd_list, &raw mut n_fds);
        result = g_unix_input_stream_new(*fds.offset(0 as ::core::ffi::c_int as isize), TRUE);
        i = 1 as ::core::ffi::c_int as gint;
        while i < n_fds {
            g_close(
                *fds.offset(i as isize),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            i += 1;
        }
        g_free(fds as gpointer);
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_finalize(mut object: *mut GObject) {
    let mut gdbcl: *mut GDBusCommandLine = object as *mut GDBusCommandLine;
    g_object_unref((*gdbcl).invocation as gpointer);
    (*(safe_c2rust_g_dbus_command_line_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_done(
    mut cmdline: *mut GApplicationCommandLine,
) {
    let mut gdbcl: *mut GDBusCommandLine = cmdline as *mut GDBusCommandLine;
    let mut status: gint = 0;
    status = g_application_command_line_get_exit_status(cmdline) as gint;
    g_dbus_method_invocation_return_value(
        (*gdbcl).invocation,
        g_variant_new(b"(i)\0" as *const u8 as *const gchar, status),
    );
    (*(safe_c2rust_g_dbus_command_line_parent_class as *mut GApplicationCommandLineClass))
        .done
        .expect("non-null function pointer")(cmdline);
}
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_init(mut gdbcl: *mut GDBusCommandLine) {}
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_class_init(
    mut class: *mut GApplicationCommandLineClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_dbus_command_line_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*class).printerr_literal = Some(
        safe_c2rust_g_dbus_command_line_printerr_literal
            as unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> ()>;
    (*class).print_literal = Some(
        safe_c2rust_g_dbus_command_line_print_literal
            as unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GApplicationCommandLine, *const gchar) -> ()>;
    (*class).get_stdin = Some(
        safe_c2rust_g_dbus_command_line_get_stdin
            as unsafe extern "C" fn(*mut GApplicationCommandLine) -> *mut GInputStream,
    )
        as Option<unsafe extern "C" fn(*mut GApplicationCommandLine) -> *mut GInputStream>;
    (*class).done = Some(
        safe_c2rust_g_dbus_command_line_done
            as unsafe extern "C" fn(*mut GApplicationCommandLine) -> (),
    ) as Option<unsafe extern "C" fn(*mut GApplicationCommandLine) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_dbus_command_line_new(
    mut invocation: *mut GDBusMethodInvocation,
) -> *mut GApplicationCommandLine {
    let mut gdbcl: *mut GDBusCommandLine = ::core::ptr::null_mut::<GDBusCommandLine>();
    let mut args: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut arguments: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut platform_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    args = g_dbus_method_invocation_get_parameters(invocation);
    arguments = g_variant_get_child_value(args, 1 as gsize);
    platform_data = g_variant_get_child_value(args, 2 as gsize);
    gdbcl = g_object_new(
        safe_c2rust_g_dbus_command_line_get_type(),
        b"arguments\0" as *const u8 as *const gchar,
        arguments,
        b"platform-data\0" as *const u8 as *const ::core::ffi::c_char,
        platform_data,
        NULL_0,
    ) as *mut GDBusCommandLine;
    g_variant_unref(arguments);
    g_variant_unref(platform_data);
    (*gdbcl).connection = g_dbus_method_invocation_get_connection(invocation);
    (*gdbcl).bus_name = g_dbus_method_invocation_get_sender(invocation);
    g_variant_get_child(
        args,
        0 as gsize,
        b"&o\0" as *const u8 as *const gchar,
        &raw mut (*gdbcl).object_path,
    );
    (*gdbcl).invocation = g_object_ref(invocation as gpointer) as *mut GDBusMethodInvocation
        as *mut GDBusMethodInvocation;
    return gdbcl as *mut ::core::ffi::c_void as *mut GApplicationCommandLine;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
