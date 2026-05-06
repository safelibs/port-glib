use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GCancellablePrivate;
    pub type _GInitable;
    pub type _GDBusMessage;
    pub type _GDBusConnection;
    pub type _GDBusMethodInvocation;
    pub type _GDBusServer;
    pub type _GDBusAuthObserver;
    pub type _GDBusInterfaceSkeletonPrivate;
    pub type __GFreedesktopDBusSkeletonPrivate;
    pub type __GFreedesktopDBus;
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_ptr_array_extend_and_steal(array_to_extend: *mut GPtrArray, array: *mut GPtrArray);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new_literal(domain: GQuark, code: gint, message: *const gchar) -> *mut GError;
    fn g_error_free(error: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_dir_make_tmp(tmpl: *const gchar, error: *mut *mut GError) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
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
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_get_values(hash_table: *mut GHashTable) -> *mut GList;
    fn g_hash_table_get_values_as_ptr_array(hash_table: *mut GHashTable) -> *mut GPtrArray;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_source_remove(tag: guint) -> gboolean;
    fn g_timeout_add(interval: guint, function: GSourceFunc, data: gpointer) -> guint;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_get_string(value: *mut GVariant, length: *mut gsize) -> *const gchar;
    fn g_variant_get_child_value(value: *mut GVariant, index_: gsize) -> *mut GVariant;
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_printerr(format: *const gchar, ...);
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
    fn g_rmdir(filename: *const gchar) -> ::core::ffi::c_int;
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
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_get_data(object: *mut GObject, key: *const gchar) -> gpointer;
    fn g_object_set_data(object: *mut GObject, key: *const gchar, data: gpointer);
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_initable_get_type() -> GType;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_dbus_connection_set_exit_on_close(
        connection: *mut GDBusConnection,
        exit_on_close: gboolean,
    );
    fn g_dbus_connection_send_message(
        connection: *mut GDBusConnection,
        message: *mut GDBusMessage,
        flags: GDBusSendMessageFlags,
        out_serial: *mut guint32,
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
    fn g_dbus_connection_add_filter(
        connection: *mut GDBusConnection,
        filter_function: GDBusMessageFilterFunction,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
    ) -> guint;
    fn g_dbus_error_quark() -> GQuark;
    fn g_dbus_error_encode_gerror(error: *const GError) -> *mut gchar;
    fn g_dbus_interface_skeleton_export(
        interface_: *mut GDBusInterfaceSkeleton,
        connection: *mut GDBusConnection,
        object_path: *const gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_dbus_interface_skeleton_unexport_from_connection(
        interface_: *mut GDBusInterfaceSkeleton,
        connection: *mut GDBusConnection,
    );
    fn g_dbus_message_new_signal(
        path: *const gchar,
        interface_: *const gchar,
        signal: *const gchar,
    ) -> *mut GDBusMessage;
    fn g_dbus_message_new_method_error_literal(
        method_call_message: *mut GDBusMessage,
        error_name: *const gchar,
        error_message: *const gchar,
    ) -> *mut GDBusMessage;
    fn g_dbus_message_get_locked(message: *mut GDBusMessage) -> gboolean;
    fn g_dbus_message_copy(
        message: *mut GDBusMessage,
        error: *mut *mut GError,
    ) -> *mut GDBusMessage;
    fn g_dbus_message_get_message_type(message: *mut GDBusMessage) -> GDBusMessageType;
    fn g_dbus_message_get_serial(message: *mut GDBusMessage) -> guint32;
    fn g_dbus_message_get_body(message: *mut GDBusMessage) -> *mut GVariant;
    fn g_dbus_message_set_body(message: *mut GDBusMessage, body: *mut GVariant);
    fn g_dbus_message_get_reply_serial(message: *mut GDBusMessage) -> guint32;
    fn g_dbus_message_get_interface(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_get_member(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_get_path(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_get_sender(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_set_sender(message: *mut GDBusMessage, value: *const gchar);
    fn g_dbus_message_get_destination(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_set_destination(message: *mut GDBusMessage, value: *const gchar);
    fn g_dbus_method_invocation_get_connection(
        invocation: *mut GDBusMethodInvocation,
    ) -> *mut GDBusConnection;
    fn g_dbus_method_invocation_return_error(
        invocation: *mut GDBusMethodInvocation,
        domain: GQuark,
        code: gint,
        format: *const gchar,
        ...
    );
    fn g_dbus_server_new_sync(
        address: *const gchar,
        flags: GDBusServerFlags,
        guid: *const gchar,
        observer: *mut GDBusAuthObserver,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusServer;
    fn g_dbus_server_get_client_address(server: *mut GDBusServer) -> *const gchar;
    fn g_dbus_server_start(server: *mut GDBusServer);
    fn g_dbus_generate_guid() -> *mut gchar;
    fn g_dbus_is_name(string: *const gchar) -> gboolean;
    fn _g_freedesktop_dbus_get_type() -> GType;
    fn _g_freedesktop_dbus_complete_hello(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        assigned_name: *const gchar,
    );
    fn _g_freedesktop_dbus_complete_request_name(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        value: guint,
    );
    fn _g_freedesktop_dbus_complete_release_name(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        value: guint,
    );
    fn _g_freedesktop_dbus_complete_start_service_by_name(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        value: guint,
    );
    fn _g_freedesktop_dbus_complete_name_has_owner(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        has_owner: gboolean,
    );
    fn _g_freedesktop_dbus_complete_list_names(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        names: *const *const gchar,
    );
    fn _g_freedesktop_dbus_complete_list_activatable_names(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        activatable_names: *const *const gchar,
    );
    fn _g_freedesktop_dbus_complete_add_match(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
    );
    fn _g_freedesktop_dbus_complete_remove_match(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
    );
    fn _g_freedesktop_dbus_complete_get_name_owner(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        unique_name: *const gchar,
    );
    fn _g_freedesktop_dbus_complete_list_queued_owners(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        queued_owners: *const *const gchar,
    );
    fn _g_freedesktop_dbus_complete_get_connection_selinux_security_context(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        security_context: *const gchar,
    );
    fn _g_freedesktop_dbus_complete_reload_config(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
    );
    fn _g_freedesktop_dbus_complete_get_id(
        object: *mut _GFreedesktopDBus,
        invocation: *mut GDBusMethodInvocation,
        unique_id: *const gchar,
    );
    fn _g_freedesktop_dbus_skeleton_get_type() -> GType;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
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
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type va_list = __builtin_va_list;
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
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed = 512;
pub const G_ASCII_SPACE: C2RustUnnamed = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed = 128;
pub const G_ASCII_PRINT: C2RustUnnamed = 64;
pub const G_ASCII_LOWER: C2RustUnnamed = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
    pub data: [C2RustUnnamed_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_DBUS_ERROR_PROPERTY_READ_ONLY: C2RustUnnamed_1 = 44;
pub const G_DBUS_ERROR_UNKNOWN_PROPERTY: C2RustUnnamed_1 = 43;
pub const G_DBUS_ERROR_UNKNOWN_INTERFACE: C2RustUnnamed_1 = 42;
pub const G_DBUS_ERROR_UNKNOWN_OBJECT: C2RustUnnamed_1 = 41;
pub const G_DBUS_ERROR_OBJECT_PATH_IN_USE: C2RustUnnamed_1 = 40;
pub const G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN: C2RustUnnamed_1 = 39;
pub const G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN: C2RustUnnamed_1 = 38;
pub const G_DBUS_ERROR_INVALID_FILE_CONTENT: C2RustUnnamed_1 = 37;
pub const G_DBUS_ERROR_INVALID_SIGNATURE: C2RustUnnamed_1 = 36;
pub const G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN: C2RustUnnamed_1 = 35;
pub const G_DBUS_ERROR_SPAWN_NO_MEMORY: C2RustUnnamed_1 = 34;
pub const G_DBUS_ERROR_SPAWN_FILE_INVALID: C2RustUnnamed_1 = 33;
pub const G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID: C2RustUnnamed_1 = 32;
pub const G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND: C2RustUnnamed_1 = 31;
pub const G_DBUS_ERROR_SPAWN_SERVICE_INVALID: C2RustUnnamed_1 = 30;
pub const G_DBUS_ERROR_SPAWN_CONFIG_INVALID: C2RustUnnamed_1 = 29;
pub const G_DBUS_ERROR_SPAWN_SETUP_FAILED: C2RustUnnamed_1 = 28;
pub const G_DBUS_ERROR_SPAWN_FAILED: C2RustUnnamed_1 = 27;
pub const G_DBUS_ERROR_SPAWN_CHILD_SIGNALED: C2RustUnnamed_1 = 26;
pub const G_DBUS_ERROR_SPAWN_CHILD_EXITED: C2RustUnnamed_1 = 25;
pub const G_DBUS_ERROR_SPAWN_FORK_FAILED: C2RustUnnamed_1 = 24;
pub const G_DBUS_ERROR_SPAWN_EXEC_FAILED: C2RustUnnamed_1 = 23;
pub const G_DBUS_ERROR_MATCH_RULE_INVALID: C2RustUnnamed_1 = 22;
pub const G_DBUS_ERROR_MATCH_RULE_NOT_FOUND: C2RustUnnamed_1 = 21;
pub const G_DBUS_ERROR_TIMED_OUT: C2RustUnnamed_1 = 20;
pub const G_DBUS_ERROR_UNKNOWN_METHOD: C2RustUnnamed_1 = 19;
pub const G_DBUS_ERROR_FILE_EXISTS: C2RustUnnamed_1 = 18;
pub const G_DBUS_ERROR_FILE_NOT_FOUND: C2RustUnnamed_1 = 17;
pub const G_DBUS_ERROR_INVALID_ARGS: C2RustUnnamed_1 = 16;
pub const G_DBUS_ERROR_DISCONNECTED: C2RustUnnamed_1 = 15;
pub const G_DBUS_ERROR_ADDRESS_IN_USE: C2RustUnnamed_1 = 14;
pub const G_DBUS_ERROR_NO_NETWORK: C2RustUnnamed_1 = 13;
pub const G_DBUS_ERROR_TIMEOUT: C2RustUnnamed_1 = 12;
pub const G_DBUS_ERROR_NO_SERVER: C2RustUnnamed_1 = 11;
pub const G_DBUS_ERROR_AUTH_FAILED: C2RustUnnamed_1 = 10;
pub const G_DBUS_ERROR_ACCESS_DENIED: C2RustUnnamed_1 = 9;
pub const G_DBUS_ERROR_LIMITS_EXCEEDED: C2RustUnnamed_1 = 8;
pub const G_DBUS_ERROR_NOT_SUPPORTED: C2RustUnnamed_1 = 7;
pub const G_DBUS_ERROR_BAD_ADDRESS: C2RustUnnamed_1 = 6;
pub const G_DBUS_ERROR_IO_ERROR: C2RustUnnamed_1 = 5;
pub const G_DBUS_ERROR_NO_REPLY: C2RustUnnamed_1 = 4;
pub const G_DBUS_ERROR_NAME_HAS_NO_OWNER: C2RustUnnamed_1 = 3;
pub const G_DBUS_ERROR_SERVICE_UNKNOWN: C2RustUnnamed_1 = 2;
pub const G_DBUS_ERROR_NO_MEMORY: C2RustUnnamed_1 = 1;
pub const G_DBUS_ERROR_FAILED: C2RustUnnamed_1 = 0;
pub type GDBusMessageType = ::core::ffi::c_uint;
pub const G_DBUS_MESSAGE_TYPE_SIGNAL: GDBusMessageType = 4;
pub const G_DBUS_MESSAGE_TYPE_ERROR: GDBusMessageType = 3;
pub const G_DBUS_MESSAGE_TYPE_METHOD_RETURN: GDBusMessageType = 2;
pub const G_DBUS_MESSAGE_TYPE_METHOD_CALL: GDBusMessageType = 1;
pub const G_DBUS_MESSAGE_TYPE_INVALID: GDBusMessageType = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GDBusServerFlags = ::core::ffi::c_uint;
pub const G_DBUS_SERVER_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER: GDBusServerFlags = 4;
pub const G_DBUS_SERVER_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS: GDBusServerFlags = 2;
pub const G_DBUS_SERVER_FLAGS_RUN_IN_THREAD: GDBusServerFlags = 1;
pub const G_DBUS_SERVER_FLAGS_NONE: GDBusServerFlags = 0;
pub type GDBusSendMessageFlags = ::core::ffi::c_uint;
pub const G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL: GDBusSendMessageFlags = 1;
pub const G_DBUS_SEND_MESSAGE_FLAGS_NONE: GDBusSendMessageFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GInitable = _GInitable;
pub type GDBusMessage = _GDBusMessage;
pub type GDBusConnection = _GDBusConnection;
pub type GDBusMethodInvocation = _GDBusMethodInvocation;
pub type GDBusServer = _GDBusServer;
pub type GDBusAuthObserver = _GDBusAuthObserver;
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
pub struct _GDBusInterfaceSkeleton {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusInterfaceSkeletonPrivate,
}
pub type GDBusInterfaceSkeletonPrivate = _GDBusInterfaceSkeletonPrivate;
pub type GDBusInterfaceSkeleton = _GDBusInterfaceSkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type GInitableIface = _GInitableIface;
pub type GDBusMessageFilterFunction = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *mut GDBusMessage,
        gboolean,
        gpointer,
    ) -> *mut GDBusMessage,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceSkeletonClass {
    pub parent_class: GObjectClass,
    pub get_info:
        Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo>,
    pub get_vtable:
        Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable>,
    pub get_properties: Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant>,
    pub flush: Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> ()>,
    pub vfunc_padding: [gpointer; 8],
    pub g_authorize_method: Option<
        unsafe extern "C" fn(*mut GDBusInterfaceSkeleton, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub signal_padding: [gpointer; 8],
}
pub type GDBusInterfaceSkeletonClass = _GDBusInterfaceSkeletonClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusDaemon {
    pub parent_instance: _GFreedesktopDBusSkeleton,
    pub address: *mut gchar,
    pub timeout: guint,
    pub tmpdir: *mut gchar,
    pub server: *mut GDBusServer,
    pub guid: *mut gchar,
    pub clients: *mut GHashTable,
    pub names: *mut GHashTable,
    pub next_major_id: guint32,
    pub next_minor_id: guint32,
}
pub type _GFreedesktopDBusSkeleton = __GFreedesktopDBusSkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __GFreedesktopDBusSkeleton {
    pub parent_instance: GDBusInterfaceSkeleton,
    pub priv_0: *mut _GFreedesktopDBusSkeletonPrivate,
}
pub type _GFreedesktopDBusSkeletonPrivate = __GFreedesktopDBusSkeletonPrivate;
pub type GDBusDaemon = _GDBusDaemon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusDaemonClass {
    pub parent_class: _GFreedesktopDBusSkeletonClass,
}
pub type _GFreedesktopDBusSkeletonClass = __GFreedesktopDBusSkeletonClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __GFreedesktopDBusSkeletonClass {
    pub parent_class: GDBusInterfaceSkeletonClass,
}
pub type GDBusDaemonClass = _GDBusDaemonClass;
pub const PROP_ADDRESS: C2RustUnnamed_2 = 1;
pub const SIGNAL_IDLE_TIMEOUT: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Client {
    pub daemon: *mut GDBusDaemon,
    pub id: *mut ::core::ffi::c_char,
    pub connection: *mut GDBusConnection,
    pub matches: *mut GList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Match {
    pub eavesdrop: gboolean,
    pub type_0: GDBusMessageType,
    pub n_elements: ::core::ffi::c_int,
    pub elements: *mut MatchElement,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchElement {
    pub type_0: guint16,
    pub arg: guint16,
    pub value: *mut ::core::ffi::c_char,
}
pub const CHECK_TYPE_NAMESPACE_PREFIX: C2RustUnnamed_5 = 4;
pub const CHECK_TYPE_PATH_RELATED: C2RustUnnamed_5 = 3;
pub const CHECK_TYPE_PATH_PREFIX: C2RustUnnamed_5 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameOwner {
    pub client: *mut Client,
    pub flags: guint32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Name {
    pub refcount: ::core::ffi::c_int,
    pub name: *mut ::core::ffi::c_char,
    pub daemon: *mut GDBusDaemon,
    pub owner: *mut NameOwner,
    pub queue: *mut GList,
}
pub const CHECK_TYPE_NAME: C2RustUnnamed_5 = 1;
pub const CHECK_TYPE_STRING: C2RustUnnamed_5 = 0;
pub const MATCH_ELEMENT_EAVESDROP: C2RustUnnamed_4 = 8;
pub const MATCH_ELEMENT_TYPE: C2RustUnnamed_4 = 0;
pub const MATCH_ELEMENT_ARGNPATH: C2RustUnnamed_4 = 10;
pub const MATCH_ELEMENT_ARGN: C2RustUnnamed_4 = 9;
pub const MATCH_ELEMENT_ARG0NAMESPACE: C2RustUnnamed_4 = 7;
pub const MATCH_ELEMENT_PATH_NAMESPACE: C2RustUnnamed_4 = 5;
pub const MATCH_ELEMENT_PATH: C2RustUnnamed_4 = 4;
pub const MATCH_ELEMENT_MEMBER: C2RustUnnamed_4 = 3;
pub const MATCH_ELEMENT_INTERFACE: C2RustUnnamed_4 = 2;
pub const MATCH_ELEMENT_DESTINATION: C2RustUnnamed_4 = 6;
pub const MATCH_ELEMENT_SENDER: C2RustUnnamed_4 = 1;
pub type _GFreedesktopDBusIface = __GFreedesktopDBusIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __GFreedesktopDBusIface {
    pub parent_iface: GTypeInterface,
    pub handle_add_match: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_get_connection_selinux_security_context: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_get_connection_unix_process_id: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_get_connection_unix_user: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_get_id: Option<
        unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub handle_get_name_owner: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_hello: Option<
        unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub handle_list_activatable_names: Option<
        unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub handle_list_names: Option<
        unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub handle_list_queued_owners: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_name_has_owner: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_release_name: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_reload_config: Option<
        unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub handle_remove_match: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
        ) -> gboolean,
    >,
    pub handle_request_name: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
            guint,
        ) -> gboolean,
    >,
    pub handle_start_service_by_name: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *const gchar,
            guint,
        ) -> gboolean,
    >,
    pub handle_update_activation_environment: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *mut GDBusMethodInvocation,
            *mut GVariant,
        ) -> gboolean,
    >,
    pub name_acquired: Option<unsafe extern "C" fn(*mut _GFreedesktopDBus, *const gchar) -> ()>,
    pub name_lost: Option<unsafe extern "C" fn(*mut _GFreedesktopDBus, *const gchar) -> ()>,
    pub name_owner_changed: Option<
        unsafe extern "C" fn(
            *mut _GFreedesktopDBus,
            *const gchar,
            *const gchar,
            *const gchar,
        ) -> (),
    >,
}
pub type _GFreedesktopDBus = __GFreedesktopDBus;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const NR_SIGNALS: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_atoi(
    mut __nptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_MAXUINT32: guint32 = 0xffffffff as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL_0 as gpointer;
    return ref_0;
}
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
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
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_c_inline(
    mut gstring: *mut GString,
    mut c: gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_3: ::core::ffi::c_int = 0;
        if !gstring.is_null() && (*gstring).len.wrapping_add(1 as gsize) < (*gstring).allocated_len
        {
            _g_boolean_var_3 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_3 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_3
    }) as ::core::ffi::c_long
        != 0
    {
        let fresh0 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh0 as isize) = c;
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
    } else {
        g_string_insert_c(gstring, -(1 as ::core::ffi::c_int) as gssize, c);
    }
    return gstring;
}
pub const G_VARIANT_TYPE_STRING: *const GVariantType =
    b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_OBJECT_PATH: *const GVariantType =
    b"o\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_TUPLE: *const GVariantType =
    b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const DBUS_SERVICE_NAME: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"org.freedesktop.DBus\0")
};
pub const DBUS_NAME_FLAG_ALLOW_REPLACEMENT: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const DBUS_NAME_FLAG_REPLACE_EXISTING: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const DBUS_NAME_FLAG_DO_NOT_QUEUE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const DBUS_REQUEST_NAME_REPLY_PRIMARY_OWNER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DBUS_REQUEST_NAME_REPLY_IN_QUEUE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DBUS_REQUEST_NAME_REPLY_EXISTS: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const DBUS_REQUEST_NAME_REPLY_ALREADY_OWNER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DBUS_RELEASE_NAME_REPLY_RELEASED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DBUS_RELEASE_NAME_REPLY_NON_EXISTENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DBUS_RELEASE_NAME_REPLY_NOT_OWNER: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const DBUS_START_REPLY_ALREADY_RUNNING: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IDLE_TIMEOUT_MSEC: ::core::ffi::c_int = 3000 as ::core::ffi::c_int;
static mut safe_c2rust_g_dbus_daemon_signals: [guint; 1] = [0; 1];
static mut safe_c2rust_GDBusDaemon_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_dbus_daemon_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_daemon_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusDaemon_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GDBusDaemon_private_offset);
    }
    safe_c2rust_g_dbus_daemon_class_init(klass as *mut GDBusDaemonClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_daemon_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        _g_freedesktop_dbus_skeleton_get_type(),
        g_intern_static_string(b"GDBusDaemon\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusDaemonClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_daemon_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusDaemon>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusDaemon) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_daemon_init as unsafe extern "C" fn(*mut GDBusDaemon) -> (),
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
            safe_c2rust_initable_iface_init as unsafe extern "C" fn(*mut GInitableIface) -> (),
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
            Option<unsafe extern "C" fn(*mut _GFreedesktopDBusIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_dbus_daemon_iface_init
                as unsafe extern "C" fn(*mut _GFreedesktopDBusIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        _g_freedesktop_dbus_get_type(),
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_daemon_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_daemon_get_type_once();
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
static mut safe_c2rust_g_dbus_daemon_parent_class: gpointer = NULL_1;
unsafe extern "C" fn safe_c2rust_name_owner_new(
    mut client: *mut Client,
    mut flags: guint32,
) -> *mut NameOwner {
    let mut owner: *mut NameOwner = ::core::ptr::null_mut::<NameOwner>();
    owner = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<NameOwner>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut NameOwner;
    (*owner).client = client;
    (*owner).flags = flags;
    return owner;
}
unsafe extern "C" fn safe_c2rust_name_owner_free(mut owner: *mut NameOwner) {
    g_free(owner as gpointer);
}
unsafe extern "C" fn safe_c2rust_name_new(
    mut daemon: *mut GDBusDaemon,
    mut str: *const ::core::ffi::c_char,
) -> *mut Name {
    let mut name: *mut Name = ::core::ptr::null_mut::<Name>();
    name = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<Name>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut Name;
    (*name).refcount = 1 as ::core::ffi::c_int;
    (*name).daemon = daemon;
    (*name).name = safe_c2rust_g_strdup_inline(str);
    g_hash_table_insert((*daemon).names, (*name).name as gpointer, name as gpointer);
    return name;
}
unsafe extern "C" fn safe_c2rust_name_ref(mut name: *mut Name) -> *mut Name {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*name).refcount > 0 as ::core::ffi::c_int {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            194 as ::core::ffi::c_int,
            G_STRFUNC,
            b"name->refcount > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*name).refcount += 1;
    return name;
}
unsafe extern "C" fn safe_c2rust_name_unref(mut name: *mut Name) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*name).refcount > 0 as ::core::ffi::c_int {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            202 as ::core::ffi::c_int,
            G_STRFUNC,
            b"name->refcount > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*name).refcount -= 1;
    if (*name).refcount == 0 as ::core::ffi::c_int {
        g_hash_table_remove((*(*name).daemon).names, (*name).name as gconstpointer);
        g_free((*name).name as gpointer);
        g_free(name as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_name_ensure(
    mut daemon: *mut GDBusDaemon,
    mut str: *const ::core::ffi::c_char,
) -> *mut Name {
    let mut name: *mut Name = ::core::ptr::null_mut::<Name>();
    name = g_hash_table_lookup((*daemon).names, str as gconstpointer) as *mut Name;
    if !name.is_null() {
        return safe_c2rust_name_ref(name);
    }
    return safe_c2rust_name_new(daemon, str);
}
unsafe extern "C" fn safe_c2rust_name_lookup(
    mut daemon: *mut GDBusDaemon,
    mut str: *const ::core::ffi::c_char,
) -> *mut Name {
    return g_hash_table_lookup((*daemon).names, str as gconstpointer) as *mut Name;
}
unsafe extern "C" fn safe_c2rust_is_key(
    mut key_start: *const ::core::ffi::c_char,
    mut key_end: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> gboolean {
    let mut len: gsize = strlen(value) as gsize;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if key_end >= key_start {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            234 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key_end >= key_start\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if len != key_end.offset_from(key_start) as ::core::ffi::c_long as gsize {
        return FALSE;
    }
    return (strncmp(key_start, value, len as size_t) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_parse_key(
    mut element: *mut MatchElement,
    mut key_start: *const ::core::ffi::c_char,
    mut key_end: *const ::core::ffi::c_char,
) -> gboolean {
    let mut res: gboolean = TRUE;
    if safe_c2rust_is_key(
        key_start,
        key_end,
        b"type\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        (*element).type_0 = MATCH_ELEMENT_TYPE as ::core::ffi::c_int as guint16;
    } else if safe_c2rust_is_key(
        key_start,
        key_end,
        b"sender\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        (*element).type_0 = MATCH_ELEMENT_SENDER as ::core::ffi::c_int as guint16;
    } else if safe_c2rust_is_key(
        key_start,
        key_end,
        b"interface\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        (*element).type_0 = MATCH_ELEMENT_INTERFACE as ::core::ffi::c_int as guint16;
    } else if safe_c2rust_is_key(
        key_start,
        key_end,
        b"member\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        (*element).type_0 = MATCH_ELEMENT_MEMBER as ::core::ffi::c_int as guint16;
    } else if safe_c2rust_is_key(
        key_start,
        key_end,
        b"path\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        (*element).type_0 = MATCH_ELEMENT_PATH as ::core::ffi::c_int as guint16;
    } else if safe_c2rust_is_key(
        key_start,
        key_end,
        b"path_namespace\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        (*element).type_0 = MATCH_ELEMENT_PATH_NAMESPACE as ::core::ffi::c_int as guint16;
    } else if safe_c2rust_is_key(
        key_start,
        key_end,
        b"destination\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        (*element).type_0 = MATCH_ELEMENT_DESTINATION as ::core::ffi::c_int as guint16;
    } else if safe_c2rust_is_key(
        key_start,
        key_end,
        b"arg0namespace\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        (*element).type_0 = MATCH_ELEMENT_ARG0NAMESPACE as ::core::ffi::c_int as guint16;
    } else if safe_c2rust_is_key(
        key_start,
        key_end,
        b"eavesdrop\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        (*element).type_0 = MATCH_ELEMENT_EAVESDROP as ::core::ffi::c_int as guint16;
    } else if key_end.offset_from(key_start) as ::core::ffi::c_long > 3 as ::core::ffi::c_long
        && safe_c2rust_is_key(
            key_start,
            key_start.offset(3 as ::core::ffi::c_int as isize),
            b"arg\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        let mut digits: *const ::core::ffi::c_char =
            key_start.offset(3 as ::core::ffi::c_int as isize);
        let mut end_digits: *const ::core::ffi::c_char = digits;
        while end_digits < key_end
            && *safe_c2rust_g_ascii_table.offset(*end_digits as guchar as isize)
                as ::core::ffi::c_int
                & G_ASCII_DIGIT as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            end_digits = end_digits.offset(1);
        }
        if end_digits == key_end {
            (*element).type_0 = MATCH_ELEMENT_ARGN as ::core::ffi::c_int as guint16;
            (*element).arg = safe_c2rust_atoi(digits) as guint16;
        } else if safe_c2rust_is_key(
            end_digits,
            key_end,
            b"path\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        {
            (*element).type_0 = MATCH_ELEMENT_ARGNPATH as ::core::ffi::c_int as guint16;
            (*element).arg = safe_c2rust_atoi(digits) as guint16;
        } else {
            res = FALSE as gboolean;
        }
    } else {
        res = FALSE as gboolean;
    }
    return res;
}
unsafe extern "C" fn safe_c2rust_parse_value(
    mut element: *mut MatchElement,
    mut s: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut quote_char: ::core::ffi::c_char = 0;
    let mut value: *mut GString = ::core::ptr::null_mut::<GString>();
    value = g_string_new(b"\0" as *const u8 as *const gchar);
    quote_char = 0 as ::core::ffi::c_char;
    while *s != 0 {
        if quote_char as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            match *s as ::core::ffi::c_int {
                39 => {
                    quote_char = '\'' as i32 as ::core::ffi::c_char;
                }
                44 => {
                    s = s.offset(1);
                    break;
                }
                92 => {
                    quote_char = '\\' as i32 as ::core::ffi::c_char;
                }
                _ => {
                    safe_c2rust_g_string_append_c_inline(value, *s);
                }
            }
        } else if quote_char as ::core::ffi::c_int == '\\' as i32 {
            if *s as ::core::ffi::c_int != '\'' as i32 {
                safe_c2rust_g_string_append_c_inline(value, '\\' as i32 as gchar);
            }
            safe_c2rust_g_string_append_c_inline(value, *s);
            quote_char = 0 as ::core::ffi::c_char;
        } else if *s as ::core::ffi::c_int == '\'' as i32 {
            quote_char = 0 as ::core::ffi::c_char;
        } else {
            safe_c2rust_g_string_append_c_inline(value, *s);
        }
        s = s.offset(1);
    }
    if quote_char as ::core::ffi::c_int == '\\' as i32 {
        safe_c2rust_g_string_append_c_inline(value, '\\' as i32 as gchar);
    } else if quote_char as ::core::ffi::c_int == '\'' as i32 {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(value, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(value);
            };
        } else {
            g_string_free(value, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    (*element).value = (if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(value, 0 as gboolean)
        } else {
            g_string_free_and_steal(value)
        }
    } else {
        g_string_free(value, 0 as gboolean)
    }) as *mut ::core::ffi::c_char;
    return s;
}
unsafe extern "C" fn safe_c2rust_match_new(mut str: *const ::core::ffi::c_char) -> *mut Match {
    let mut current_block: u64;
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut elements: *mut GArray = ::core::ptr::null_mut::<GArray>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut key_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut key_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut element: MatchElement = MatchElement {
        type_0: 0,
        arg: 0,
        value: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut eavesdrop: gboolean = 0;
    let mut type_0: GDBusMessageType = G_DBUS_MESSAGE_TYPE_INVALID;
    let mut i: gsize = 0;
    eavesdrop = FALSE as gboolean;
    type_0 = G_DBUS_MESSAGE_TYPE_INVALID;
    elements = g_array_new(TRUE, TRUE, ::core::mem::size_of::<MatchElement>() as guint);
    p = str;
    loop {
        if !(*p as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            current_block = 11048769245176032998;
            break;
        }
        memset(
            &raw mut element as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<MatchElement>() as size_t,
        );
        while *p as ::core::ffi::c_int != 0
            && *safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_SPACE as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        key_start = p;
        while *p as ::core::ffi::c_int != 0
            && *p as ::core::ffi::c_int != '=' as i32
            && !(*safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_SPACE as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
        {
            p = p.offset(1);
        }
        key_end = p;
        while *p as ::core::ffi::c_int != 0
            && *safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_SPACE as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if key_start == key_end {
            continue;
        }
        if *p as ::core::ffi::c_int != '=' as i32 {
            current_block = 10824818668892631709;
            break;
        }
        p = p.offset(1);
        if safe_c2rust_parse_key(&raw mut element, key_start, key_end) == 0 {
            current_block = 10824818668892631709;
            break;
        }
        p = safe_c2rust_parse_value(&raw mut element, p);
        if p.is_null() {
            current_block = 10824818668892631709;
            break;
        }
        if element.type_0 as ::core::ffi::c_int == MATCH_ELEMENT_EAVESDROP as ::core::ffi::c_int {
            if strcmp(
                element.value,
                b"true\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                eavesdrop = TRUE as gboolean;
            } else if strcmp(
                element.value,
                b"false\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                eavesdrop = FALSE as gboolean;
            } else {
                g_free(element.value as gpointer);
                current_block = 10824818668892631709;
                break;
            }
            g_free(element.value as gpointer);
        } else if element.type_0 as ::core::ffi::c_int == MATCH_ELEMENT_TYPE as ::core::ffi::c_int {
            if strcmp(
                element.value,
                b"signal\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                type_0 = G_DBUS_MESSAGE_TYPE_SIGNAL;
            } else if strcmp(
                element.value,
                b"method_call\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                type_0 = G_DBUS_MESSAGE_TYPE_METHOD_CALL;
            } else if strcmp(
                element.value,
                b"method_return\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                type_0 = G_DBUS_MESSAGE_TYPE_METHOD_RETURN;
            } else if strcmp(
                element.value,
                b"error\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                type_0 = G_DBUS_MESSAGE_TYPE_ERROR;
            } else {
                g_free(element.value as gpointer);
                current_block = 10824818668892631709;
                break;
            }
            g_free(element.value as gpointer);
        } else {
            g_array_append_vals(elements, &raw mut element as gconstpointer, 1 as guint);
        }
    }
    match current_block {
        10824818668892631709 => {
            i = 0 as gsize;
            while i < (*elements).len as gsize {
                g_free(
                    (*((*elements).data as *mut ::core::ffi::c_void as *mut MatchElement)
                        .offset(i as isize))
                    .value as gpointer,
                );
                i = i.wrapping_add(1);
            }
            g_array_free(elements, TRUE);
            return ::core::ptr::null_mut::<Match>();
        }
        _ => {
            match_0 = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<Match>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut Match;
            (*match_0).n_elements = (*elements).len as ::core::ffi::c_int;
            (*match_0).elements = g_array_free(elements, FALSE) as *mut MatchElement;
            (*match_0).eavesdrop = eavesdrop;
            (*match_0).type_0 = type_0;
            return match_0;
        }
    };
}
unsafe extern "C" fn safe_c2rust_match_free(mut match_0: *mut Match) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*match_0).n_elements {
        g_free((*(*match_0).elements.offset(i as isize)).value as gpointer);
        i += 1;
    }
    g_free((*match_0).elements as gpointer);
    g_free(match_0 as gpointer);
}
unsafe extern "C" fn safe_c2rust_match_equal(mut a: *mut Match, mut b: *mut Match) -> gboolean {
    let mut i: ::core::ffi::c_int = 0;
    if (*a).eavesdrop != (*b).eavesdrop {
        return FALSE;
    }
    if (*a).type_0 as ::core::ffi::c_uint != (*b).type_0 as ::core::ffi::c_uint {
        return FALSE;
    }
    if (*a).n_elements != (*b).n_elements {
        return FALSE;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*a).n_elements {
        if (*(*a).elements.offset(i as isize)).type_0 as ::core::ffi::c_int
            != (*(*b).elements.offset(i as isize)).type_0 as ::core::ffi::c_int
            || (*(*a).elements.offset(i as isize)).arg as ::core::ffi::c_int
                != (*(*b).elements.offset(i as isize)).arg as ::core::ffi::c_int
            || strcmp(
                (*(*a).elements.offset(i as isize)).value,
                (*(*b).elements.offset(i as isize)).value,
            ) != 0 as ::core::ffi::c_int
        {
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_message_get_argN(
    mut message: *mut GDBusMessage,
    mut n: ::core::ffi::c_int,
    mut allow_path: gboolean,
) -> *const gchar {
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    let mut body: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    ret = ::core::ptr::null::<gchar>();
    body = g_dbus_message_get_body(message);
    if !body.is_null() && g_variant_is_of_type(body, G_VARIANT_TYPE_TUPLE) != 0 {
        let mut item: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        item = g_variant_get_child_value(body, n as gsize);
        if g_variant_is_of_type(item, G_VARIANT_TYPE_STRING) != 0
            || allow_path != 0 && g_variant_is_of_type(item, G_VARIANT_TYPE_OBJECT_PATH) != 0
        {
            ret = g_variant_get_string(item, ::core::ptr::null_mut::<gsize>());
        }
        g_variant_unref(item);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_match_matches(
    mut daemon: *mut GDBusDaemon,
    mut match_0: *mut Match,
    mut message: *mut GDBusMessage,
    mut has_destination: gboolean,
) -> gboolean {
    let mut element: *mut MatchElement = ::core::ptr::null_mut::<MatchElement>();
    let mut name: *mut Name = ::core::ptr::null_mut::<Name>();
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut len2: ::core::ffi::c_int = 0;
    let mut value: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut check_type: ::core::ffi::c_int = 0;
    if has_destination != 0 && (*match_0).eavesdrop == 0 {
        return FALSE;
    }
    if (*match_0).type_0 as ::core::ffi::c_uint
        != G_DBUS_MESSAGE_TYPE_INVALID as ::core::ffi::c_int as ::core::ffi::c_uint
        && g_dbus_message_get_message_type(message) as ::core::ffi::c_uint
            != (*match_0).type_0 as ::core::ffi::c_uint
    {
        return FALSE;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*match_0).n_elements {
        element = (*match_0).elements.offset(i as isize) as *mut MatchElement;
        check_type = CHECK_TYPE_STRING as ::core::ffi::c_int;
        match (*element).type_0 as ::core::ffi::c_int {
            1 => {
                check_type = CHECK_TYPE_NAME as ::core::ffi::c_int;
                value = g_dbus_message_get_sender(message) as *const ::core::ffi::c_char;
                if value.is_null() {
                    value = DBUS_SERVICE_NAME.as_ptr();
                }
            }
            6 => {
                check_type = CHECK_TYPE_NAME as ::core::ffi::c_int;
                value = g_dbus_message_get_destination(message) as *const ::core::ffi::c_char;
            }
            2 => {
                value = g_dbus_message_get_interface(message) as *const ::core::ffi::c_char;
            }
            3 => {
                value = g_dbus_message_get_member(message) as *const ::core::ffi::c_char;
            }
            4 => {
                value = g_dbus_message_get_path(message) as *const ::core::ffi::c_char;
            }
            5 => {
                check_type = CHECK_TYPE_PATH_PREFIX as ::core::ffi::c_int;
                value = g_dbus_message_get_path(message) as *const ::core::ffi::c_char;
            }
            7 => {
                check_type = CHECK_TYPE_NAMESPACE_PREFIX as ::core::ffi::c_int;
                value = safe_c2rust_message_get_argN(message, 0 as ::core::ffi::c_int, FALSE)
                    as *const ::core::ffi::c_char;
            }
            9 => {
                value = safe_c2rust_message_get_argN(
                    message,
                    (*element).arg as ::core::ffi::c_int,
                    FALSE,
                ) as *const ::core::ffi::c_char;
            }
            10 => {
                check_type = CHECK_TYPE_PATH_RELATED as ::core::ffi::c_int;
                value = safe_c2rust_message_get_argN(
                    message,
                    (*element).arg as ::core::ffi::c_int,
                    TRUE,
                ) as *const ::core::ffi::c_char;
            }
            0 | 8 | _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    600 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
        if value.is_null() {
            return FALSE;
        }
        match check_type {
            0 => {
                if strcmp((*element).value, value) != 0 as ::core::ffi::c_int {
                    return FALSE;
                }
            }
            1 => {
                name = safe_c2rust_name_lookup(daemon, (*element).value);
                if !name.is_null() && !(*name).owner.is_null() {
                    if strcmp((*(*(*name).owner).client).id, value) != 0 as ::core::ffi::c_int {
                        return FALSE;
                    }
                } else if strcmp((*element).value, value) != 0 as ::core::ffi::c_int {
                    return FALSE;
                }
            }
            2 => {
                len = strlen((*element).value) as ::core::ffi::c_int;
                if !(len == 1 as ::core::ffi::c_int) {
                    if if 0 != 0 {
                        ({
                            let __str: *const ::core::ffi::c_char = value;
                            let __prefix: *const ::core::ffi::c_char = (*element).value;
                            let mut __result: gboolean = FALSE;
                            if ({
                                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                                if __str.is_null() || __prefix.is_null() {
                                    _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_13
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                __result = g_str_has_prefix(
                                    __str as *const gchar,
                                    __prefix as *const gchar,
                                );
                            } else {
                                let __str_len: size_t = strlen(
                                    __str.offset(__str.is_null() as ::core::ffi::c_int as isize),
                                ) as size_t;
                                let __prefix_len: size_t = strlen(
                                    __prefix
                                        .offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                                )
                                    as size_t;
                                if __str_len >= __prefix_len {
                                    __result = (memcmp(
                                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                            as *const ::core::ffi::c_void,
                                        __prefix.offset(
                                            __prefix.is_null() as ::core::ffi::c_int as isize
                                        )
                                            as *const ::core::ffi::c_void,
                                        __prefix_len,
                                    ) == 0 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                                        as gboolean;
                                }
                            }
                            __result
                        })
                    } else {
                        g_str_has_prefix(value as *const gchar, (*element).value)
                    } == 0
                    {
                        return FALSE;
                    }
                    if *value.offset(len as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                        && *value.offset(len as isize) as ::core::ffi::c_int != '/' as i32
                    {
                        return FALSE;
                    }
                }
            }
            3 => {
                len = strlen((*element).value) as ::core::ffi::c_int;
                len2 = strlen(value) as ::core::ffi::c_int;
                if !(strcmp(value, (*element).value) == 0 as ::core::ffi::c_int
                    || len2 > 0 as ::core::ffi::c_int
                        && *value.offset((len2 - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == '/' as i32
                        && (if 0 != 0 {
                            ({
                                let __str: *const ::core::ffi::c_char = (*element).value;
                                let __prefix: *const ::core::ffi::c_char = value;
                                let mut __result: gboolean = FALSE;
                                if ({
                                    let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                                    if __str.is_null() || __prefix.is_null() {
                                        _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_14
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    __result = g_str_has_prefix(
                                        __str as *const gchar,
                                        __prefix as *const gchar,
                                    );
                                } else {
                                    let __str_len: size_t = strlen(
                                        __str
                                            .offset(__str.is_null() as ::core::ffi::c_int as isize),
                                    )
                                        as size_t;
                                    let __prefix_len: size_t =
                                        strlen(__prefix.offset(__prefix.is_null()
                                            as ::core::ffi::c_int
                                            as isize))
                                            as size_t;
                                    if __str_len >= __prefix_len {
                                        __result = (memcmp(
                                            __str.offset(
                                                __str.is_null() as ::core::ffi::c_int as isize
                                            )
                                                as *const ::core::ffi::c_void,
                                            __prefix
                                                .offset(__prefix.is_null() as ::core::ffi::c_int
                                                    as isize)
                                                as *const ::core::ffi::c_void,
                                            __prefix_len,
                                        ) == 0 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                            as gboolean;
                                    }
                                }
                                __result
                            })
                        } else {
                            g_str_has_prefix((*element).value, value as *const gchar)
                        }) != 0
                    || len > 0 as ::core::ffi::c_int
                        && *(*element)
                            .value
                            .offset((len - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == '/' as i32
                        && (if 0 != 0 {
                            ({
                                let __str: *const ::core::ffi::c_char = value;
                                let __prefix: *const ::core::ffi::c_char = (*element).value;
                                let mut __result: gboolean = FALSE;
                                if ({
                                    let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                                    if __str.is_null() || __prefix.is_null() {
                                        _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_15
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    __result = g_str_has_prefix(
                                        __str as *const gchar,
                                        __prefix as *const gchar,
                                    );
                                } else {
                                    let __str_len: size_t = strlen(
                                        __str
                                            .offset(__str.is_null() as ::core::ffi::c_int as isize),
                                    )
                                        as size_t;
                                    let __prefix_len: size_t =
                                        strlen(__prefix.offset(__prefix.is_null()
                                            as ::core::ffi::c_int
                                            as isize))
                                            as size_t;
                                    if __str_len >= __prefix_len {
                                        __result = (memcmp(
                                            __str.offset(
                                                __str.is_null() as ::core::ffi::c_int as isize
                                            )
                                                as *const ::core::ffi::c_void,
                                            __prefix
                                                .offset(__prefix.is_null() as ::core::ffi::c_int
                                                    as isize)
                                                as *const ::core::ffi::c_void,
                                            __prefix_len,
                                        ) == 0 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                            as gboolean;
                                    }
                                }
                                __result
                            })
                        } else {
                            g_str_has_prefix(value as *const gchar, (*element).value)
                        }) != 0)
                {
                    return FALSE;
                }
            }
            4 => {
                len = strlen((*element).value) as ::core::ffi::c_int;
                if !((if 0 != 0 {
                    ({
                        let __str: *const ::core::ffi::c_char = value;
                        let __prefix: *const ::core::ffi::c_char = (*element).value;
                        let mut __result: gboolean = FALSE;
                        if ({
                            let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                            if __str.is_null() || __prefix.is_null() {
                                _g_boolean_var_16 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_16 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_16
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            __result =
                                g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                        } else {
                            let __str_len: size_t = strlen(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize),
                            ) as size_t;
                            let __prefix_len: size_t = strlen(
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                            ) as size_t;
                            if __str_len >= __prefix_len {
                                __result = (memcmp(
                                    __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                        as *const ::core::ffi::c_void,
                                    __prefix
                                        .offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                        as *const ::core::ffi::c_void,
                                    __prefix_len,
                                ) == 0 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int
                                    as gboolean;
                            }
                        }
                        __result
                    })
                } else {
                    g_str_has_prefix(value as *const gchar, (*element).value)
                }) != 0
                    && (*value.offset(len as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        || *value.offset(len as isize) as ::core::ffi::c_int == '.' as i32))
                {
                    return FALSE;
                }
            }
            _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    653 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
        i += 1;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_broadcast_message(
    mut daemon: *mut GDBusDaemon,
    mut message: *mut GDBusMessage,
    mut has_destination: gboolean,
    mut preserve_serial: gboolean,
    mut not_to: *mut Client,
) {
    let mut clients: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut ll: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut copy: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    clients = g_hash_table_get_values((*daemon).clients);
    l = clients;
    while !l.is_null() {
        let mut client: *mut Client = (*l).data as *mut Client;
        if !(client == not_to) {
            ll = (*client).matches;
            while !ll.is_null() {
                let mut match_0: *mut Match = (*ll).data as *mut Match;
                if safe_c2rust_match_matches(daemon, match_0, message, has_destination) != 0 {
                    break;
                }
                ll = (*ll).next;
            }
            if !ll.is_null() {
                copy = g_dbus_message_copy(message, ::core::ptr::null_mut::<*mut GError>());
                if !copy.is_null() {
                    g_dbus_connection_send_message(
                        (*client).connection,
                        copy,
                        (if preserve_serial != 0 {
                            G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as GDBusSendMessageFlags,
                        ::core::ptr::null_mut::<guint32>(),
                        ::core::ptr::null_mut::<*mut GError>(),
                    );
                    g_object_unref(copy as gpointer);
                }
            }
        }
        l = (*l).next;
    }
    g_list_free(clients);
}
unsafe extern "C" fn safe_c2rust_send_name_owner_changed(
    mut daemon: *mut GDBusDaemon,
    mut name: *const ::core::ffi::c_char,
    mut old_owner: *const ::core::ffi::c_char,
    mut new_owner: *const ::core::ffi::c_char,
) {
    let mut signal_message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    signal_message = g_dbus_message_new_signal(
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"NameOwnerChanged\0" as *const u8 as *const gchar,
    );
    g_dbus_message_set_body(
        signal_message,
        g_variant_new(
            b"(sss)\0" as *const u8 as *const gchar,
            name,
            if !old_owner.is_null() {
                old_owner
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            if !new_owner.is_null() {
                new_owner
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
        ),
    );
    safe_c2rust_broadcast_message(
        daemon,
        signal_message,
        FALSE,
        FALSE,
        ::core::ptr::null_mut::<Client>(),
    );
    g_object_unref(signal_message as gpointer);
}
unsafe extern "C" fn safe_c2rust_name_unqueue_owner(
    mut name: *mut Name,
    mut client: *mut Client,
) -> gboolean {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    l = (*name).queue;
    while !l.is_null() {
        let mut other: *mut NameOwner = (*l).data as *mut NameOwner;
        if (*other).client == client {
            (*name).queue = g_list_delete_link((*name).queue, l);
            safe_c2rust_name_unref(name);
            safe_c2rust_name_owner_free(other);
            return TRUE;
        }
        l = (*l).next;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_name_replace_owner(
    mut name: *mut Name,
    mut owner: *mut NameOwner,
) {
    let mut daemon: *mut GDBusDaemon = (*name).daemon;
    let mut old_owner: *mut NameOwner = ::core::ptr::null_mut::<NameOwner>();
    let mut old_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut new_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut new_client: *mut Client = ::core::ptr::null_mut::<Client>();
    if !owner.is_null() {
        new_client = (*owner).client;
    }
    safe_c2rust_name_ref(name);
    old_owner = (*name).owner;
    if !old_owner.is_null() {
        let mut old_client: *mut Client = (*old_owner).client;
        if ({
            let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
            if (*old_owner).client != new_client {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                762 as ::core::ffi::c_int,
                G_STRFUNC,
                b"old_owner->client != new_client\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_dbus_connection_emit_signal(
            (*old_client).connection,
            ::core::ptr::null::<gchar>(),
            b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"NameLost\0" as *const u8 as *const gchar,
            g_variant_new(b"(s)\0" as *const u8 as *const gchar, (*name).name),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        old_name = safe_c2rust_g_strdup_inline((*old_client).id);
        if (*old_owner).flags & DBUS_NAME_FLAG_DO_NOT_QUEUE as guint32 != 0 {
            safe_c2rust_name_unref(name);
            safe_c2rust_name_owner_free(old_owner);
        } else {
            (*name).queue = g_list_prepend((*name).queue, old_owner as gpointer);
        }
    }
    (*name).owner = owner;
    if !owner.is_null() {
        safe_c2rust_name_unqueue_owner(name, (*owner).client);
        safe_c2rust_name_ref(name);
        new_name = (*new_client).id;
        g_dbus_connection_emit_signal(
            (*new_client).connection,
            ::core::ptr::null::<gchar>(),
            b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"NameAcquired\0" as *const u8 as *const gchar,
            g_variant_new(b"(s)\0" as *const u8 as *const gchar, (*name).name),
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    safe_c2rust_send_name_owner_changed(daemon, (*name).name, old_name, new_name);
    g_free(old_name as gpointer);
    safe_c2rust_name_unref(name);
}
unsafe extern "C" fn safe_c2rust_name_release_owner(mut name: *mut Name) {
    let mut next_owner: *mut NameOwner = ::core::ptr::null_mut::<NameOwner>();
    safe_c2rust_name_ref(name);
    if !(*name).queue.is_null() {
        next_owner = (*(*name).queue).data as *mut NameOwner;
        safe_c2rust_name_unref(name);
        (*name).queue = g_list_delete_link((*name).queue, (*name).queue);
    }
    (*(*name).owner).flags |= DBUS_NAME_FLAG_DO_NOT_QUEUE as guint32;
    safe_c2rust_name_replace_owner(name, next_owner);
    safe_c2rust_name_unref(name);
}
unsafe extern "C" fn safe_c2rust_name_queue_owner(mut name: *mut Name, mut owner: *mut NameOwner) {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    l = (*name).queue;
    while !l.is_null() {
        let mut other: *mut NameOwner = (*l).data as *mut NameOwner;
        if (*other).client == (*owner).client {
            (*other).flags = (*owner).flags;
            safe_c2rust_name_owner_free(owner);
            return;
        }
        l = (*l).next;
    }
    (*name).queue = g_list_append((*name).queue, owner as gpointer);
    safe_c2rust_name_ref(name);
}
unsafe extern "C" fn safe_c2rust_client_new(
    mut daemon: *mut GDBusDaemon,
    mut connection: *mut GDBusConnection,
) -> *mut Client {
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    client = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<Client>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut Client;
    (*client).daemon = daemon;
    (*client).id = g_strdup_printf(
        b":%d.%d\0" as *const u8 as *const gchar,
        (*daemon).next_major_id,
        (*daemon).next_minor_id,
    ) as *mut ::core::ffi::c_char;
    (*client).connection =
        g_object_ref(connection as gpointer) as *mut GDBusConnection as *mut GDBusConnection;
    if (*daemon).next_minor_id == G_MAXUINT32 {
        (*daemon).next_minor_id = 0 as guint32;
        (*daemon).next_major_id = (*daemon).next_major_id.wrapping_add(1);
    } else {
        (*daemon).next_minor_id = (*daemon).next_minor_id.wrapping_add(1);
    }
    g_object_set_data(
        connection as *mut ::core::ffi::c_void as *mut GObject,
        b"client\0" as *const u8 as *const gchar,
        client as gpointer,
    );
    g_hash_table_insert(
        (*daemon).clients,
        (*client).id as gpointer,
        client as gpointer,
    );
    g_dbus_interface_skeleton_export(
        daemon as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        connection,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        &raw mut error,
    );
    if !error.is_null() {
        g_assertion_message_error(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            867 as ::core::ffi::c_int,
            G_STRFUNC,
            b"error\0" as *const u8 as *const ::core::ffi::c_char,
            error,
            0 as GQuark,
            0 as ::core::ffi::c_int,
        );
    }
    g_signal_connect_data(
        connection as gpointer,
        b"closed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDBusConnection,
                    gboolean,
                    *mut GError,
                    *mut Client,
                ) -> (),
            >,
            GCallback,
        >(Some(
            safe_c2rust_connection_closed
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    gboolean,
                    *mut GError,
                    *mut Client,
                ) -> (),
        )),
        client as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_dbus_connection_add_filter(
        connection,
        Some(
            safe_c2rust_filter_function
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *mut GDBusMessage,
                    gboolean,
                    gpointer,
                ) -> *mut GDBusMessage,
        ),
        client as gpointer,
        None,
    );
    safe_c2rust_send_name_owner_changed(
        daemon,
        (*client).id,
        ::core::ptr::null::<::core::ffi::c_char>(),
        (*client).id,
    );
    return client;
}
unsafe extern "C" fn safe_c2rust_client_free(mut client: *mut Client) {
    let mut daemon: *mut GDBusDaemon = (*client).daemon;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut names: *mut GList = ::core::ptr::null_mut::<GList>();
    g_dbus_interface_skeleton_unexport_from_connection(
        daemon as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton,
        (*client).connection,
    );
    g_hash_table_remove((*daemon).clients, (*client).id as gconstpointer);
    names = g_hash_table_get_values((*daemon).names);
    l = names;
    while !l.is_null() {
        let mut name: *mut Name = (*l).data as *mut Name;
        safe_c2rust_name_ref(name);
        if !(*name).owner.is_null() && (*(*name).owner).client == client {
            if ({
                let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                if (*name).refcount >= 2 as ::core::ffi::c_int {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    900 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"name->refcount >= 2\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            safe_c2rust_name_release_owner(name);
        }
        safe_c2rust_name_unqueue_owner(name, client);
        safe_c2rust_name_unref(name);
        l = (*l).next;
    }
    g_list_free(names);
    safe_c2rust_send_name_owner_changed(
        daemon,
        (*client).id,
        (*client).id,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    g_object_unref((*client).connection as gpointer);
    l = (*client).matches;
    while !l.is_null() {
        safe_c2rust_match_free((*l).data as *mut Match);
        l = (*l).next;
    }
    g_list_free((*client).matches);
    g_free((*client).id as gpointer);
    g_free(client as gpointer);
}
unsafe extern "C" fn safe_c2rust_idle_timeout_cb(mut user_data: gpointer) -> gboolean {
    let mut daemon: *mut GDBusDaemon = user_data as *mut GDBusDaemon;
    (*daemon).timeout = 0 as guint;
    g_signal_emit(
        daemon as gpointer,
        safe_c2rust_g_dbus_daemon_signals[SIGNAL_IDLE_TIMEOUT as ::core::ffi::c_int as usize],
        0 as GQuark,
    );
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_connection_closed(
    mut connection: *mut GDBusConnection,
    mut remote_peer_vanished: gboolean,
    mut error: *mut GError,
    mut client: *mut Client,
) {
    let mut daemon: *mut GDBusDaemon = (*client).daemon;
    safe_c2rust_client_free(client);
    if g_hash_table_size((*daemon).clients) == 0 as guint {
        (*daemon).timeout = g_timeout_add(
            IDLE_TIMEOUT_MSEC as guint,
            Some(safe_c2rust_idle_timeout_cb as unsafe extern "C" fn(gpointer) -> gboolean),
            daemon as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_handle_add_match(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_rule: *const gchar,
) -> gboolean {
    let mut client: *mut Client = g_object_get_data(
        g_dbus_method_invocation_get_connection(invocation) as *mut ::core::ffi::c_void
            as *mut GObject,
        b"client\0" as *const u8 as *const gchar,
    ) as *mut Client;
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    match_0 = safe_c2rust_match_new(arg_rule as *const ::core::ffi::c_char);
    if match_0.is_null() {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_MATCH_RULE_INVALID as ::core::ffi::c_int as gint,
            b"Invalid rule: %s\0" as *const u8 as *const gchar,
            arg_rule,
        );
    } else {
        (*client).matches = g_list_prepend((*client).matches, match_0 as gpointer);
        _g_freedesktop_dbus_complete_add_match(object, invocation);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_get_connection_selinux_security_context(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_name: *const gchar,
) -> gboolean {
    g_dbus_method_invocation_return_error(
        invocation,
        g_dbus_error_quark(),
        G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN as ::core::ffi::c_int as gint,
        b"selinux context not supported\0" as *const u8 as *const gchar,
    );
    _g_freedesktop_dbus_complete_get_connection_selinux_security_context(
        object,
        invocation,
        b"\0" as *const u8 as *const gchar,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_get_connection_unix_process_id(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_name: *const gchar,
) -> gboolean {
    g_dbus_method_invocation_return_error(
        invocation,
        g_dbus_error_quark(),
        G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN as ::core::ffi::c_int as gint,
        b"connection pid not supported\0" as *const u8 as *const gchar,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_get_connection_unix_user(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_name: *const gchar,
) -> gboolean {
    g_dbus_method_invocation_return_error(
        invocation,
        g_dbus_error_quark(),
        G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN as ::core::ffi::c_int as gint,
        b"connection user not supported\0" as *const u8 as *const gchar,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_get_id(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
) -> gboolean {
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    _g_freedesktop_dbus_complete_get_id(object, invocation, (*daemon).guid);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_get_name_owner(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_name: *const gchar,
) -> gboolean {
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    let mut name: *mut Name = ::core::ptr::null_mut::<Name>();
    if strcmp(
        arg_name as *const ::core::ffi::c_char,
        DBUS_SERVICE_NAME.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        _g_freedesktop_dbus_complete_get_name_owner(
            object,
            invocation,
            DBUS_SERVICE_NAME.as_ptr() as *const gchar,
        );
        return TRUE;
    }
    if *arg_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32 {
        if g_hash_table_lookup((*daemon).clients, arg_name as gconstpointer).is_null() {
            g_dbus_method_invocation_return_error(
                invocation,
                g_dbus_error_quark(),
                G_DBUS_ERROR_NAME_HAS_NO_OWNER as ::core::ffi::c_int as gint,
                b"Could not get owner of name '%s': no such name\0" as *const u8 as *const gchar,
                arg_name,
            );
        } else {
            _g_freedesktop_dbus_complete_get_name_owner(object, invocation, arg_name);
        }
        return TRUE;
    }
    name = safe_c2rust_name_lookup(daemon, arg_name as *const ::core::ffi::c_char);
    if name.is_null() || (*name).owner.is_null() {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_NAME_HAS_NO_OWNER as ::core::ffi::c_int as gint,
            b"Could not get owner of name '%s': no such name\0" as *const u8 as *const gchar,
            arg_name,
        );
        return TRUE;
    }
    _g_freedesktop_dbus_complete_get_name_owner(object, invocation, (*(*(*name).owner).client).id);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_hello(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
) -> gboolean {
    let mut client: *mut Client = g_object_get_data(
        g_dbus_method_invocation_get_connection(invocation) as *mut ::core::ffi::c_void
            as *mut GObject,
        b"client\0" as *const u8 as *const gchar,
    ) as *mut Client;
    _g_freedesktop_dbus_complete_hello(object, invocation, (*client).id);
    g_dbus_connection_emit_signal(
        (*client).connection,
        ::core::ptr::null::<gchar>(),
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"NameAcquired\0" as *const u8 as *const gchar,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, (*client).id),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_list_activatable_names(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
) -> gboolean {
    let mut names: [*const ::core::ffi::c_char; 1] = [::core::ptr::null::<::core::ffi::c_char>()];
    _g_freedesktop_dbus_complete_list_activatable_names(
        object,
        invocation,
        &raw mut names as *mut *const ::core::ffi::c_char as *const *const gchar,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_list_names(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
) -> gboolean {
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut clients: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut names: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    clients = g_hash_table_get_values_as_ptr_array((*daemon).clients);
    array = safe_c2rust_g_steal_pointer(&raw mut clients as gpointer) as *mut GPtrArray
        as *mut GPtrArray;
    names = g_hash_table_get_values_as_ptr_array((*daemon).names);
    g_ptr_array_extend_and_steal(
        array,
        safe_c2rust_g_steal_pointer(&raw mut names as gpointer) as *mut GPtrArray,
    );
    g_ptr_array_add(array, NULL_1);
    _g_freedesktop_dbus_complete_list_names(
        object,
        invocation,
        (*array).pdata as *const *const gchar,
    );
    g_ptr_array_free(array, TRUE);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_list_queued_owners(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_name: *const gchar,
) -> gboolean {
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut name: *mut Name = ::core::ptr::null_mut::<Name>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    array = g_ptr_array_new();
    name = safe_c2rust_name_lookup(daemon, arg_name as *const ::core::ffi::c_char);
    if !name.is_null() && !(*name).owner.is_null() {
        l = (*name).queue;
        while !l.is_null() {
            let mut client: *mut Client = (*l).data as *mut Client;
            g_ptr_array_add(array, (*client).id as gpointer);
            l = (*l).next;
        }
    }
    g_ptr_array_add(array, NULL_1);
    _g_freedesktop_dbus_complete_list_queued_owners(
        object,
        invocation,
        (*array).pdata as *const *const gchar,
    );
    g_ptr_array_free(array, TRUE);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_name_has_owner(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_name: *const gchar,
) -> gboolean {
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    let mut name: *mut Name = ::core::ptr::null_mut::<Name>();
    let mut client: *mut Client = ::core::ptr::null_mut::<Client>();
    name = safe_c2rust_name_lookup(daemon, arg_name as *const ::core::ffi::c_char);
    client = g_hash_table_lookup((*daemon).clients, arg_name as gconstpointer) as *mut Client;
    _g_freedesktop_dbus_complete_name_has_owner(
        object,
        invocation,
        (!name.is_null() || !client.is_null()) as ::core::ffi::c_int,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_release_name(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_name: *const gchar,
) -> gboolean {
    let mut client: *mut Client = g_object_get_data(
        g_dbus_method_invocation_get_connection(invocation) as *mut ::core::ffi::c_void
            as *mut GObject,
        b"client\0" as *const u8 as *const gchar,
    ) as *mut Client;
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    let mut name: *mut Name = ::core::ptr::null_mut::<Name>();
    let mut result: guint32 = 0;
    if g_dbus_is_name(arg_name) == 0 {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"Given bus name \"%s\" is not valid\0" as *const u8 as *const gchar,
            arg_name,
        );
        return TRUE;
    }
    if *arg_name as ::core::ffi::c_int == ':' as i32 {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"Cannot release a service starting with ':' such as \"%s\"\0" as *const u8
                as *const gchar,
            arg_name,
        );
        return TRUE;
    }
    if strcmp(
        arg_name as *const ::core::ffi::c_char,
        DBUS_SERVICE_NAME.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"Cannot release a service named org.freedesktop.DBus, because that is owned by the bus\0"
                as *const u8 as *const gchar,
        );
        return TRUE;
    }
    name = safe_c2rust_name_lookup(daemon, arg_name as *const ::core::ffi::c_char);
    if name.is_null() {
        result = DBUS_RELEASE_NAME_REPLY_NON_EXISTENT as guint32;
    } else if !(*name).owner.is_null() && (*(*name).owner).client == client {
        safe_c2rust_name_release_owner(name);
        result = DBUS_RELEASE_NAME_REPLY_RELEASED as guint32;
    } else if safe_c2rust_name_unqueue_owner(name, client) != 0 {
        result = DBUS_RELEASE_NAME_REPLY_RELEASED as guint32;
    } else {
        result = DBUS_RELEASE_NAME_REPLY_NOT_OWNER as guint32;
    }
    _g_freedesktop_dbus_complete_release_name(object, invocation, result as guint);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_reload_config(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
) -> gboolean {
    _g_freedesktop_dbus_complete_reload_config(object, invocation);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_update_activation_environment(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_environment: *mut GVariant,
) -> gboolean {
    g_dbus_method_invocation_return_error(
        invocation,
        g_dbus_error_quark(),
        G_DBUS_ERROR_FAILED as ::core::ffi::c_int as gint,
        b"UpdateActivationEnvironment not implemented\0" as *const u8 as *const gchar,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_remove_match(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_rule: *const gchar,
) -> gboolean {
    let mut client: *mut Client = g_object_get_data(
        g_dbus_method_invocation_get_connection(invocation) as *mut ::core::ffi::c_void
            as *mut GObject,
        b"client\0" as *const u8 as *const gchar,
    ) as *mut Client;
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut other_match: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    match_0 = safe_c2rust_match_new(arg_rule as *const ::core::ffi::c_char);
    if match_0.is_null() {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_MATCH_RULE_INVALID as ::core::ffi::c_int as gint,
            b"Invalid rule: %s\0" as *const u8 as *const gchar,
            arg_rule,
        );
    } else {
        l = (*client).matches;
        while !l.is_null() {
            other_match = (*l).data as *mut Match;
            if safe_c2rust_match_equal(match_0, other_match) != 0 {
                safe_c2rust_match_free(other_match);
                (*client).matches = g_list_delete_link((*client).matches, l);
                break;
            } else {
                l = (*l).next;
            }
        }
        if l.is_null() {
            g_dbus_method_invocation_return_error(
                invocation,
                g_dbus_error_quark(),
                G_DBUS_ERROR_MATCH_RULE_NOT_FOUND as ::core::ffi::c_int as gint,
                b"The given match rule wasn't found and can't be removed\0" as *const u8
                    as *const gchar,
            );
        } else {
            _g_freedesktop_dbus_complete_remove_match(object, invocation);
        }
    }
    if !match_0.is_null() {
        safe_c2rust_match_free(match_0);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_request_name(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_name: *const gchar,
    mut flags: guint,
) -> gboolean {
    let mut client: *mut Client = g_object_get_data(
        g_dbus_method_invocation_get_connection(invocation) as *mut ::core::ffi::c_void
            as *mut GObject,
        b"client\0" as *const u8 as *const gchar,
    ) as *mut Client;
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    let mut name: *mut Name = ::core::ptr::null_mut::<Name>();
    let mut owner: *mut NameOwner = ::core::ptr::null_mut::<NameOwner>();
    let mut result: guint32 = 0;
    if g_dbus_is_name(arg_name) == 0 {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"Requested bus name \"%s\" is not valid\0" as *const u8 as *const gchar,
            arg_name,
        );
        return TRUE;
    }
    if *arg_name as ::core::ffi::c_int == ':' as i32 {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"Cannot acquire a service starting with ':' such as \"%s\"\0" as *const u8
                as *const gchar,
            arg_name,
        );
        return TRUE;
    }
    if strcmp(
        arg_name as *const ::core::ffi::c_char,
        DBUS_SERVICE_NAME.as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
            b"Cannot acquire a service named org.freedesktop.DBus, because that is reserved\0"
                as *const u8 as *const gchar,
        );
        return TRUE;
    }
    name = safe_c2rust_name_ensure(daemon, arg_name as *const ::core::ffi::c_char);
    if (*name).owner.is_null() {
        owner = safe_c2rust_name_owner_new(client, flags as guint32);
        safe_c2rust_name_replace_owner(name, owner);
        result = DBUS_REQUEST_NAME_REPLY_PRIMARY_OWNER as guint32;
    } else if !(*name).owner.is_null() && (*(*name).owner).client == client {
        (*(*name).owner).flags = flags as guint32;
        result = DBUS_REQUEST_NAME_REPLY_ALREADY_OWNER as guint32;
    } else if flags & DBUS_NAME_FLAG_DO_NOT_QUEUE as guint != 0
        && (flags & DBUS_NAME_FLAG_REPLACE_EXISTING as guint == 0
            || (*(*name).owner).flags & DBUS_NAME_FLAG_ALLOW_REPLACEMENT as guint32 == 0)
    {
        safe_c2rust_name_unqueue_owner(name, client);
        result = DBUS_REQUEST_NAME_REPLY_EXISTS as guint32;
    } else if flags & DBUS_NAME_FLAG_DO_NOT_QUEUE as guint == 0
        && (flags & DBUS_NAME_FLAG_REPLACE_EXISTING as guint == 0
            || (*(*name).owner).flags & DBUS_NAME_FLAG_ALLOW_REPLACEMENT as guint32 == 0)
    {
        owner = safe_c2rust_name_owner_new(client, flags as guint32);
        safe_c2rust_name_queue_owner(name, owner);
        result = DBUS_REQUEST_NAME_REPLY_IN_QUEUE as guint32;
    } else {
        owner = safe_c2rust_name_owner_new(client, flags as guint32);
        safe_c2rust_name_replace_owner(name, owner);
        result = DBUS_REQUEST_NAME_REPLY_PRIMARY_OWNER as guint32;
    }
    safe_c2rust_name_unref(name);
    _g_freedesktop_dbus_complete_request_name(object, invocation, result as guint);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_handle_start_service_by_name(
    mut object: *mut _GFreedesktopDBus,
    mut invocation: *mut GDBusMethodInvocation,
    mut arg_name: *const gchar,
    mut arg_flags: guint,
) -> gboolean {
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    let mut name: *mut Name = ::core::ptr::null_mut::<Name>();
    name = safe_c2rust_name_lookup(daemon, arg_name as *const ::core::ffi::c_char);
    if !name.is_null() {
        _g_freedesktop_dbus_complete_start_service_by_name(
            object,
            invocation,
            DBUS_START_REPLY_ALREADY_RUNNING as guint,
        );
    } else {
        g_dbus_method_invocation_return_error(
            invocation,
            g_dbus_error_quark(),
            G_DBUS_ERROR_SERVICE_UNKNOWN as ::core::ffi::c_int as gint,
            b"No support for activation for name: %s\0" as *const u8 as *const gchar,
            arg_name,
        );
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_return_error(
    mut client: *mut Client,
    mut message: *mut GDBusMessage,
    mut domain: GQuark,
    mut code: gint,
    mut format: *const gchar,
    mut args: ...
) {
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut var_args: ::core::ffi::VaList;
    let mut error_message: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut dbus_error_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    var_args = args.clone();
    error_message = g_strdup_vprintf(format, var_args) as *mut ::core::ffi::c_char;
    error = g_error_new_literal(domain, code, b"\0" as *const u8 as *const gchar);
    dbus_error_name = g_dbus_error_encode_gerror(error);
    reply = g_dbus_message_new_method_error_literal(message, dbus_error_name, error_message);
    g_error_free(error);
    g_free(dbus_error_name as gpointer);
    g_free(error_message as gpointer);
    if g_dbus_connection_send_message(
        (*client).connection,
        reply,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        ::core::ptr::null_mut::<guint32>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Error sending reply\0" as *const u8 as *const gchar,
        );
    }
    g_object_unref(reply as gpointer);
}
unsafe extern "C" fn safe_c2rust_route_message(
    mut source_client: *mut Client,
    mut message: *mut GDBusMessage,
) -> *mut GDBusMessage {
    let mut dest: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dest_client: *mut Client = ::core::ptr::null_mut::<Client>();
    let mut daemon: *mut GDBusDaemon = ::core::ptr::null_mut::<GDBusDaemon>();
    daemon = (*source_client).daemon;
    dest_client = ::core::ptr::null_mut::<Client>();
    dest = g_dbus_message_get_destination(message) as *const ::core::ffi::c_char;
    if !dest.is_null() && strcmp(dest, DBUS_SERVICE_NAME.as_ptr()) != 0 as ::core::ffi::c_int {
        dest_client = g_hash_table_lookup((*daemon).clients, dest as gconstpointer) as *mut Client;
        if dest_client.is_null() {
            let mut name: *mut Name = ::core::ptr::null_mut::<Name>();
            name = safe_c2rust_name_lookup(daemon, dest);
            if !name.is_null() && !(*name).owner.is_null() {
                dest_client = (*(*name).owner).client;
            }
        }
        if dest_client.is_null() {
            if g_dbus_message_get_message_type(message) as ::core::ffi::c_uint
                == G_DBUS_MESSAGE_TYPE_METHOD_CALL as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                safe_c2rust_return_error(
                    source_client,
                    message,
                    g_dbus_error_quark(),
                    G_DBUS_ERROR_SERVICE_UNKNOWN as ::core::ffi::c_int as gint,
                    b"The name %s is unknown\0" as *const u8 as *const gchar,
                    dest,
                );
            }
        } else {
            let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
            if g_dbus_connection_send_message(
                (*dest_client).connection,
                message,
                G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL,
                ::core::ptr::null_mut::<guint32>(),
                &raw mut error,
            ) == 0
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Error forwarding message: %s\0" as *const u8 as *const gchar,
                    (*error).message,
                );
                g_error_free(error);
            }
        }
    }
    safe_c2rust_broadcast_message(
        daemon,
        message,
        (dest_client != NULL_1 as *mut Client) as ::core::ffi::c_int,
        TRUE,
        dest_client,
    );
    if dest.is_null() || strcmp(dest, DBUS_SERVICE_NAME.as_ptr()) != 0 as ::core::ffi::c_int {
        g_object_unref(message as gpointer);
        message = ::core::ptr::null_mut::<GDBusMessage>();
    }
    return message;
}
unsafe extern "C" fn safe_c2rust_copy_if_locked(
    mut message: *mut GDBusMessage,
) -> *mut GDBusMessage {
    if g_dbus_message_get_locked(message) != 0 {
        let mut copy: *mut GDBusMessage =
            g_dbus_message_copy(message, ::core::ptr::null_mut::<*mut GError>());
        g_object_unref(message as gpointer);
        message = copy;
    }
    return message;
}
unsafe extern "C" fn safe_c2rust_filter_function(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut incoming: gboolean,
    mut user_data: gpointer,
) -> *mut GDBusMessage {
    let mut client: *mut Client = user_data as *mut Client;
    if incoming != 0 {
        message = safe_c2rust_copy_if_locked(message);
        if message.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Failed to copy incoming message\0" as *const u8 as *const gchar,
            );
            return ::core::ptr::null_mut::<GDBusMessage>();
        }
        g_dbus_message_set_sender(message, (*client).id);
        return safe_c2rust_route_message(client, message);
    } else {
        if g_dbus_message_get_sender(message).is_null()
            || g_dbus_message_get_destination(message).is_null()
        {
            message = safe_c2rust_copy_if_locked(message);
            if message.is_null() {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Failed to copy outgoing message\0" as *const u8 as *const gchar,
                );
                return ::core::ptr::null_mut::<GDBusMessage>();
            }
        }
        if g_dbus_message_get_sender(message).is_null() {
            g_dbus_message_set_sender(message, DBUS_SERVICE_NAME.as_ptr() as *const gchar);
        }
        if g_dbus_message_get_destination(message).is_null() {
            g_dbus_message_set_destination(message, (*client).id);
        }
    }
    return message;
}
unsafe extern "C" fn safe_c2rust_on_new_connection(
    mut server: *mut GDBusServer,
    mut connection: *mut GDBusConnection,
    mut user_data: gpointer,
) -> gboolean {
    let mut daemon: *mut GDBusDaemon = user_data as *mut GDBusDaemon;
    g_dbus_connection_set_exit_on_close(connection, FALSE);
    if (*daemon).timeout != 0 {
        g_source_remove((*daemon).timeout);
        (*daemon).timeout = 0 as guint;
    }
    safe_c2rust_client_new(daemon, connection);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_dbus_daemon_finalize(mut object: *mut GObject) {
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    let mut clients: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    if (*daemon).timeout != 0 {
        g_source_remove((*daemon).timeout);
    }
    clients = g_hash_table_get_values((*daemon).clients);
    l = clients;
    while !l.is_null() {
        safe_c2rust_client_free((*l).data as *mut Client);
        l = (*l).next;
    }
    g_list_free(clients);
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if g_hash_table_size((*daemon).clients) == 0 as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1564 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_hash_table_size (daemon->clients) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if g_hash_table_size((*daemon).names) == 0 as guint {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1565 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_hash_table_size (daemon->names) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_destroy((*daemon).clients);
    g_hash_table_destroy((*daemon).names);
    g_object_unref((*daemon).server as gpointer);
    if !(*daemon).tmpdir.is_null() {
        g_rmdir((*daemon).tmpdir);
        g_free((*daemon).tmpdir as gpointer);
    }
    g_free((*daemon).guid as gpointer);
    g_free((*daemon).address as gpointer);
    (*(safe_c2rust_g_dbus_daemon_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_daemon_init(mut daemon: *mut GDBusDaemon) {
    (*daemon).next_major_id = 1 as guint32;
    (*daemon).clients = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        None,
    );
    (*daemon).names = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        None,
    );
    (*daemon).guid = g_dbus_generate_guid();
}
unsafe extern "C" fn safe_c2rust_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut daemon: *mut GDBusDaemon = initable as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    let mut flags: GDBusServerFlags = G_DBUS_SERVER_FLAGS_NONE;
    flags = G_DBUS_SERVER_FLAGS_NONE;
    if (*daemon).address.is_null() {
        (*daemon).tmpdir = g_dir_make_tmp(
            b"gdbus-daemon-XXXXXX\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        (*daemon).address = g_strdup_printf(
            b"unix:tmpdir=%s\0" as *const u8 as *const gchar,
            (*daemon).tmpdir,
        );
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GDBusServerFlags>(
            flags as ::core::ffi::c_uint
                | G_DBUS_SERVER_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
        );
    }
    (*daemon).server = g_dbus_server_new_sync(
        (*daemon).address,
        flags,
        (*daemon).guid,
        ::core::ptr::null_mut::<GDBusAuthObserver>(),
        cancellable,
        error,
    );
    if (*daemon).server.is_null() {
        return FALSE;
    }
    g_dbus_server_start((*daemon).server);
    g_signal_connect_data(
        (*daemon).server as gpointer,
        b"new-connection\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut GDBusServer, *mut GDBusConnection, gpointer) -> gboolean,
            >,
            GCallback,
        >(Some(
            safe_c2rust_on_new_connection
                as unsafe extern "C" fn(
                    *mut GDBusServer,
                    *mut GDBusConnection,
                    gpointer,
                ) -> gboolean,
        )),
        daemon as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_dbus_daemon_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    match prop_id {
        1 => {
            g_free((*daemon).address as gpointer);
            (*daemon).address = g_value_dup_string(value);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1651 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_daemon_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut daemon: *mut GDBusDaemon = object as *mut ::core::ffi::c_void as *mut GDBusDaemon;
    match prop_id {
        1 => {
            g_value_set_string(value, (*daemon).address);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusdaemon.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1670 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_daemon_class_init(mut klass: *mut GDBusDaemonClass) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_dbus_daemon_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dbus_daemon_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dbus_daemon_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    safe_c2rust_g_dbus_daemon_signals[SIGNAL_IDLE_TIMEOUT as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"idle-timeout\0" as *const u8 as *const gchar),
            safe_c2rust__g_dbus_daemon_get_type(),
            G_SIGNAL_RUN_LAST,
            0 as guint,
            None,
            NULL_1,
            None,
            G_TYPE_NONE,
            0 as guint,
        );
    g_object_class_install_property(
        gobject_class,
        PROP_ADDRESS as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"address\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_daemon_iface_init(mut iface: *mut _GFreedesktopDBusIface) {
    (*iface).handle_add_match = Some(
        safe_c2rust_handle_add_match
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
        >;
    (*iface).handle_get_connection_selinux_security_context = Some(
        safe_c2rust_handle_get_connection_selinux_security_context
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
        >;
    (*iface).handle_get_connection_unix_process_id = Some(
        safe_c2rust_handle_get_connection_unix_process_id
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
        >;
    (*iface).handle_get_connection_unix_user = Some(
        safe_c2rust_handle_get_connection_unix_user
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
        >;
    (*iface).handle_get_id = Some(
        safe_c2rust_handle_get_id
            as unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
        >;
    (*iface).handle_get_name_owner = Some(
        safe_c2rust_handle_get_name_owner
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
        >;
    (*iface).handle_hello = Some(
        safe_c2rust_handle_hello
            as unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
        >;
    (*iface).handle_list_activatable_names = Some(
        safe_c2rust_handle_list_activatable_names
            as unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
        >;
    (*iface).handle_list_names = Some(
        safe_c2rust_handle_list_names
            as unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
        >;
    (*iface).handle_list_queued_owners = Some(
        safe_c2rust_handle_list_queued_owners
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
        >;
    (*iface).handle_name_has_owner = Some(
        safe_c2rust_handle_name_has_owner
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
        >;
    (*iface).handle_release_name = Some(
        safe_c2rust_handle_release_name
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
        >;
    (*iface).handle_reload_config = Some(
        safe_c2rust_handle_reload_config
            as unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut _GFreedesktopDBus, *mut GDBusMethodInvocation) -> gboolean,
        >;
    (*iface).handle_update_activation_environment = Some(
        safe_c2rust_handle_update_activation_environment
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *mut GVariant,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *mut GVariant,
            ) -> gboolean,
        >;
    (*iface).handle_remove_match = Some(
        safe_c2rust_handle_remove_match
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
            ) -> gboolean,
        >;
    (*iface).handle_request_name = Some(
        safe_c2rust_handle_request_name
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
                guint,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
                guint,
            ) -> gboolean,
        >;
    (*iface).handle_start_service_by_name = Some(
        safe_c2rust_handle_start_service_by_name
            as unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
                guint,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut _GFreedesktopDBus,
                *mut GDBusMethodInvocation,
                *const gchar,
                guint,
            ) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_initable_iface_init(mut initable_iface: *mut GInitableIface) {
    (*initable_iface).init = Some(
        safe_c2rust_initable_init
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_daemon_new(
    mut address: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusDaemon {
    return g_initable_new(
        safe_c2rust__g_dbus_daemon_get_type(),
        cancellable,
        error,
        b"address\0" as *const u8 as *const gchar,
        address,
        NULL_1,
    ) as *mut GDBusDaemon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_daemon_get_address(
    mut daemon: *mut GDBusDaemon,
) -> *const ::core::ffi::c_char {
    return g_dbus_server_get_client_address((*daemon).server) as *const ::core::ffi::c_char;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
