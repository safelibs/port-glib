extern "C" {
    pub type _GData;
    pub type _GBufferedInputStreamPrivate;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GDataInputStreamPrivate;
    pub type _GIOStreamPrivate;
    pub type _GSocketPrivate;
    pub type _GSocketConnectionPrivate;
    pub type _GCredentials;
    pub type _GDBusAuthObserver;
    pub type _GDataOutputStreamPrivate;
    pub type _GDBusAuthMechanismPrivate;
    pub type _GUnixConnectionPrivate;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
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
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_sort(list: *mut GList, compare_func: GCompareFunc) -> *mut GList;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_ascii_xdigit_value(c: gchar) -> gint;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strchug(string: *mut gchar) -> *mut gchar;
    fn g_strchomp(string: *mut gchar) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strjoinv(separator: *const gchar, str_array: *mut *mut gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_set_size(string: *mut GString, len: gsize) -> *mut GString;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_print(format: *const gchar, ...);
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn _g_dbus_auth_mechanism_get_priority(mechanism_type: GType) -> gint;
    fn _g_dbus_auth_mechanism_get_name(mechanism_type: GType) -> *const gchar;
    fn _g_dbus_auth_mechanism_is_supported(mechanism: *mut GDBusAuthMechanism) -> gboolean;
    fn _g_dbus_auth_mechanism_server_get_state(
        mechanism: *mut GDBusAuthMechanism,
    ) -> GDBusAuthMechanismState;
    fn _g_dbus_auth_mechanism_server_initiate(
        mechanism: *mut GDBusAuthMechanism,
        initial_response: *const gchar,
        initial_response_len: gsize,
    );
    fn _g_dbus_auth_mechanism_server_data_receive(
        mechanism: *mut GDBusAuthMechanism,
        data: *const gchar,
        data_len: gsize,
    );
    fn _g_dbus_auth_mechanism_server_data_send(
        mechanism: *mut GDBusAuthMechanism,
        out_data_len: *mut gsize,
    ) -> *mut gchar;
    fn _g_dbus_auth_mechanism_client_get_state(
        mechanism: *mut GDBusAuthMechanism,
    ) -> GDBusAuthMechanismState;
    fn _g_dbus_auth_mechanism_client_initiate(
        mechanism: *mut GDBusAuthMechanism,
        conn_flags: GDBusConnectionFlags,
        out_initial_response_len: *mut gsize,
    ) -> *mut gchar;
    fn _g_dbus_auth_mechanism_client_data_receive(
        mechanism: *mut GDBusAuthMechanism,
        data: *const gchar,
        data_len: gsize,
    );
    fn _g_dbus_auth_mechanism_client_data_send(
        mechanism: *mut GDBusAuthMechanism,
        out_data_len: *mut gsize,
    ) -> *mut gchar;
    fn _g_dbus_auth_mechanism_anon_get_type() -> GType;
    fn _g_dbus_auth_mechanism_external_get_type() -> GType;
    fn _g_dbus_auth_mechanism_sha1_get_type() -> GType;
    fn g_dbus_auth_observer_authorize_authenticated_peer(
        observer: *mut GDBusAuthObserver,
        stream: *mut GIOStream,
        credentials: *mut GCredentials,
    ) -> gboolean;
    fn g_dbus_auth_observer_allow_mechanism(
        observer: *mut GDBusAuthObserver,
        mechanism: *const gchar,
    ) -> gboolean;
    fn g_dbus_is_guid(string: *const gchar) -> gboolean;
    fn g_credentials_new() -> *mut GCredentials;
    fn g_credentials_to_string(credentials: *mut GCredentials) -> *mut gchar;
    fn g_credentials_is_same_user(
        credentials: *mut GCredentials,
        other_credentials: *mut GCredentials,
        error: *mut *mut GError,
    ) -> gboolean;
    fn _g_dbus_debug_authentication() -> gboolean;
    fn _g_dbus_debug_print_lock();
    fn _g_dbus_debug_print_unlock();
    fn _g_dbus_hexencode(str: *const gchar, str_len: gsize) -> *mut gchar;
    fn g_input_stream_read(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_io_error_quark() -> GQuark;
    fn g_io_stream_get_type() -> GType;
    fn g_io_stream_get_input_stream(stream: *mut GIOStream) -> *mut GInputStream;
    fn g_io_stream_get_output_stream(stream: *mut GIOStream) -> *mut GOutputStream;
    fn g_filter_input_stream_set_close_base_stream(
        stream: *mut GFilterInputStream,
        close_base: gboolean,
    );
    fn g_data_input_stream_new(base_stream: *mut GInputStream) -> *mut GDataInputStream;
    fn g_data_input_stream_set_newline_type(
        stream: *mut GDataInputStream,
        type_0: GDataStreamNewlineType,
    );
    fn g_data_input_stream_read_line(
        stream: *mut GDataInputStream,
        length: *mut gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut ::core::ffi::c_char;
    fn g_filter_output_stream_set_close_base_stream(
        stream: *mut GFilterOutputStream,
        close_base: gboolean,
    );
    fn g_data_output_stream_new(base_stream: *mut GOutputStream) -> *mut GDataOutputStream;
    fn g_data_output_stream_put_byte(
        stream: *mut GDataOutputStream,
        data: guchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_data_output_stream_put_string(
        stream: *mut GDataOutputStream,
        str: *const ::core::ffi::c_char,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_get_credentials(socket: *mut GSocket, error: *mut *mut GError)
        -> *mut GCredentials;
    fn g_socket_connection_get_type() -> GType;
    fn g_socket_connection_get_socket(connection: *mut GSocketConnection) -> *mut GSocket;
    fn g_unix_connection_get_type() -> GType;
    fn g_unix_connection_send_credentials(
        connection: *mut GUnixConnection,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_unix_connection_receive_credentials(
        connection: *mut GUnixConnection,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GCredentials;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
pub type guint8 = ::core::ffi::c_uchar;
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
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub type GDataStreamNewlineType = ::core::ffi::c_uint;
pub const G_DATA_STREAM_NEWLINE_TYPE_ANY: GDataStreamNewlineType = 3;
pub const G_DATA_STREAM_NEWLINE_TYPE_CR_LF: GDataStreamNewlineType = 2;
pub const G_DATA_STREAM_NEWLINE_TYPE_CR: GDataStreamNewlineType = 1;
pub const G_DATA_STREAM_NEWLINE_TYPE_LF: GDataStreamNewlineType = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_1 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_1 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_1 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_1 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_1 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_1 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_1 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_1 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_1 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_1 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_1 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_1 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_1 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_1 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_1 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_1 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_1 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_1 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_1 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_1 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_1 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_1 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_1 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_1 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_1 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_1 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_1 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_1 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_1 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_1 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_1 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_1 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_1 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_1 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_1 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_1 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_1 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_1 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_1 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_1 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_1 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_1 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_1 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_1 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_1 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_1 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_1 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_1 = 0;
pub type GDBusConnectionFlags = ::core::ffi::c_uint;
pub const G_DBUS_CONNECTION_FLAGS_CROSS_NAMESPACE: GDBusConnectionFlags = 64;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER: GDBusConnectionFlags = 32;
pub const G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING: GDBusConnectionFlags = 16;
pub const G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION: GDBusConnectionFlags = 8;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS: GDBusConnectionFlags = 4;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER: GDBusConnectionFlags = 2;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT: GDBusConnectionFlags = 1;
pub const G_DBUS_CONNECTION_FLAGS_NONE: GDBusConnectionFlags = 0;
pub type GDBusCapabilityFlags = ::core::ffi::c_uint;
pub const G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING: GDBusCapabilityFlags = 1;
pub const G_DBUS_CAPABILITY_FLAGS_NONE: GDBusCapabilityFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GBufferedInputStream {
    pub parent_instance: GFilterInputStream,
    pub priv_0: *mut GBufferedInputStreamPrivate,
}
pub type GBufferedInputStreamPrivate = _GBufferedInputStreamPrivate;
pub type GFilterInputStream = _GFilterInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFilterInputStream {
    pub parent_instance: GInputStream,
    pub base_stream: *mut GInputStream,
}
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GBufferedInputStream = _GBufferedInputStream;
pub type GFilterOutputStream = _GFilterOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFilterOutputStream {
    pub parent_instance: GOutputStream,
    pub base_stream: *mut GOutputStream,
}
pub type GOutputStream = _GOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GOutputStreamPrivate,
}
pub type GOutputStreamPrivate = _GOutputStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDataInputStream {
    pub parent_instance: GBufferedInputStream,
    pub priv_0: *mut GDataInputStreamPrivate,
}
pub type GDataInputStreamPrivate = _GDataInputStreamPrivate;
pub type GDataInputStream = _GDataInputStream;
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocket {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketPrivate,
}
pub type GSocketPrivate = _GSocketPrivate;
pub type GSocket = _GSocket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GSocketConnectionPrivate,
}
pub type GSocketConnectionPrivate = _GSocketConnectionPrivate;
pub type GSocketConnection = _GSocketConnection;
pub type GCredentials = _GCredentials;
pub type GDBusAuthObserver = _GDBusAuthObserver;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuth {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusAuthPrivate,
}
pub type GDBusAuthPrivate = _GDBusAuthPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthPrivate {
    pub stream: *mut GIOStream,
    pub available_mechanisms: *mut GList,
}
pub type GDBusAuth = _GDBusAuth;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthClass {
    pub parent_class: GObjectClass,
}
pub type GDBusAuthClass = _GDBusAuthClass;
pub const PROP_STREAM: C2RustUnnamed_2 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mechanism {
    pub name: *const gchar,
    pub priority: gint,
    pub gtype: GType,
}
pub type GDataOutputStream = _GDataOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDataOutputStream {
    pub parent_instance: GFilterOutputStream,
    pub priv_0: *mut GDataOutputStreamPrivate,
}
pub type GDataOutputStreamPrivate = _GDataOutputStreamPrivate;
pub type GDBusAuthMechanism = _GDBusAuthMechanism;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanism {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusAuthMechanismPrivate,
}
pub type GDBusAuthMechanismPrivate = _GDBusAuthMechanismPrivate;
pub const SERVER_STATE_WAITING_FOR_BEGIN: ServerState = 2;
pub const G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND: GDBusAuthMechanismState = 2;
pub type ServerState = ::core::ffi::c_uint;
pub const SERVER_STATE_WAITING_FOR_DATA: ServerState = 1;
pub const SERVER_STATE_WAITING_FOR_AUTH: ServerState = 0;
pub const G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA: GDBusAuthMechanismState = 1;
pub const G_DBUS_AUTH_MECHANISM_STATE_REJECTED: GDBusAuthMechanismState = 3;
pub const G_DBUS_AUTH_MECHANISM_STATE_ACCEPTED: GDBusAuthMechanismState = 4;
pub type GDBusAuthMechanismState = ::core::ffi::c_uint;
pub const G_DBUS_AUTH_MECHANISM_STATE_INVALID: GDBusAuthMechanismState = 0;
pub type GUnixConnection = _GUnixConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixConnection {
    pub parent_instance: GSocketConnection,
    pub priv_0: *mut GUnixConnectionPrivate,
}
pub type GUnixConnectionPrivate = _GUnixConnectionPrivate;
pub type ClientState = ::core::ffi::c_uint;
pub const CLIENT_STATE_WAITING_FOR_AGREE_UNIX_FD: ClientState = 3;
pub const CLIENT_STATE_WAITING_FOR_REJECT: ClientState = 2;
pub const CLIENT_STATE_WAITING_FOR_OK: ClientState = 1;
pub const CLIENT_STATE_WAITING_FOR_DATA: ClientState = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_2 = 0;
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
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_4: ::core::ffi::c_int = 0;
        if gstring.is_null() {
            _g_boolean_var_4 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_4 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_4
    }) as ::core::ffi::c_long
        != 0
    {
        return g_string_append_len(gstring, val as *const gchar, len);
    }
    if ({
        let mut _g_boolean_var_5: ::core::ffi::c_int = 0;
        if val.is_null() {
            _g_boolean_var_5 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_5 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_5
    }) as ::core::ffi::c_long
        != 0
    {
        return if len != 0 as gssize {
            g_string_append_len(gstring, val as *const gchar, len)
        } else {
            gstring
        };
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    if ({
        let mut _g_boolean_var_6: ::core::ffi::c_int = 0;
        if (*gstring).len.wrapping_add(len_unsigned) < (*gstring).allocated_len {
            _g_boolean_var_6 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_6 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_6
    }) as ::core::ffi::c_long
        != 0
    {
        let mut end: *mut ::core::ffi::c_char = (*gstring).str_0.offset((*gstring).len as isize);
        if ({
            let mut _g_boolean_var_7: ::core::ffi::c_int = 0;
            if val.offset(len_unsigned as isize) <= end as *const ::core::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const ::core::ffi::c_char
            {
                _g_boolean_var_7 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_7 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_7
        }) as ::core::ffi::c_long
            != 0
        {
            memcpy(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        } else {
            memmove(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        }
        (*gstring).len = (*gstring).len.wrapping_add(len_unsigned);
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
        return gstring;
    } else {
        return g_string_insert_len(
            gstring,
            -(1 as ::core::ffi::c_int) as gssize,
            val as *const gchar,
            len,
        );
    };
}
unsafe extern "C" fn safe_c2rust_debug_print(mut message: *const gchar, mut args: ...) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if _g_dbus_debug_authentication() != 0 {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
        let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut str: *mut GString = ::core::ptr::null_mut::<GString>();
        let mut var_args: ::core::ffi::VaList;
        let mut n: guint = 0;
        _g_dbus_debug_print_lock();
        var_args = args.clone();
        s = g_strdup_vprintf(message, var_args);
        str = g_string_new(::core::ptr::null::<gchar>());
        n = 0 as guint;
        while *s.offset(n as isize) as ::core::ffi::c_int != '\0' as i32 {
            if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if *s.offset(n as isize) as ::core::ffi::c_int == '\r' as i32 {
                    _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_11
            }) as ::core::ffi::c_long
                != 0
            {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"\\r\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            str,
                            __val,
                            if ({
                                let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_12
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        b"\\r\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            } else if ({
                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                if *s.offset(n as isize) as ::core::ffi::c_int == '\n' as i32 {
                    _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_13
            }) as ::core::ffi::c_long
                != 0
            {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"\\n\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            str,
                            __val,
                            if ({
                                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_14
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        b"\\n\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            } else {
                safe_c2rust_g_string_append_c_inline(str, *s.offset(n as isize));
            }
            n = n.wrapping_add(1);
        }
        g_print(
            b"GDBus-debug:Auth: %s\n\0" as *const u8 as *const gchar,
            (*str).str_0,
        );
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(str, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(str);
            };
        } else {
            g_string_free(str, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
        g_free(s as gpointer);
        _g_dbus_debug_print_unlock();
    }
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_class_intern_init(mut klass: gpointer) {
    safe_c2rust__g_dbus_auth_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusAuth_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GDBusAuth_private_offset);
    }
    safe_c2rust__g_dbus_auth_class_init(klass as *mut GDBusAuthClass);
}
static mut safe_c2rust__g_dbus_auth_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust__g_dbus_auth_get_type_once();
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
unsafe extern "C" fn safe_c2rust__g_dbus_auth_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusAuth\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusAuthClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_dbus_auth_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusAuth>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAuth) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_dbus_auth_init as unsafe extern "C" fn(*mut GDBusAuth) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDBusAuth_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDBusAuthPrivate>() as gsize,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GDBusAuth_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust__g_dbus_auth_get_instance_private(
    mut self_0: *mut GDBusAuth,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GDBusAuth_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_finalize(mut object: *mut GObject) {
    let mut auth: *mut GDBusAuth = object as *mut ::core::ffi::c_void as *mut GDBusAuth;
    if !(*(*auth).priv_0).stream.is_null() {
        g_object_unref((*(*auth).priv_0).stream as gpointer);
    }
    g_list_free_full(
        (*(*auth).priv_0).available_mechanisms,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut Mechanism) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_mechanism_free as unsafe extern "C" fn(*mut Mechanism) -> ()),
        ),
    );
    if (*(safe_c2rust__g_dbus_auth_parent_class as *mut GObjectClass))
        .finalize
        .is_some()
    {
        (*(safe_c2rust__g_dbus_auth_parent_class as *mut GObjectClass))
            .finalize
            .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut auth: *mut GDBusAuth = object as *mut ::core::ffi::c_void as *mut GDBusAuth;
    match prop_id {
        1 => {
            g_value_set_object(value, (*(*auth).priv_0).stream as gpointer);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauth.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut auth: *mut GDBusAuth = object as *mut ::core::ffi::c_void as *mut GDBusAuth;
    match prop_id {
        1 => {
            (*(*auth).priv_0).stream = g_value_dup_object(value) as *mut GIOStream;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauth.c\0"
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
unsafe extern "C" fn safe_c2rust__g_dbus_auth_class_init(mut klass: *mut GDBusAuthClass) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust__g_dbus_auth_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust__g_dbus_auth_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize =
        Some(safe_c2rust__g_dbus_auth_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_STREAM as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"stream\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_io_stream_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_mechanism_free(mut m: *mut Mechanism) {
    g_free(m as gpointer);
}
unsafe extern "C" fn safe_c2rust_add_mechanism(
    mut auth: *mut GDBusAuth,
    mut observer: *mut GDBusAuthObserver,
    mut mechanism_type: GType,
) {
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    name = _g_dbus_auth_mechanism_get_name(mechanism_type);
    if observer.is_null() || g_dbus_auth_observer_allow_mechanism(observer, name) != 0 {
        let mut m: *mut Mechanism = ::core::ptr::null_mut::<Mechanism>();
        m = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<Mechanism>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut Mechanism;
        (*m).name = name;
        (*m).priority = _g_dbus_auth_mechanism_get_priority(mechanism_type);
        (*m).gtype = mechanism_type;
        (*(*auth).priv_0).available_mechanisms =
            g_list_prepend((*(*auth).priv_0).available_mechanisms, m as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_mech_compare_func(
    mut a: *mut Mechanism,
    mut b: *mut Mechanism,
) -> gint {
    let mut ret: gint = 0;
    ret = (*b).priority - (*a).priority;
    if ret == 0 as ::core::ffi::c_int {
        ret = g_strcmp0(
            (*b).name as *const ::core::ffi::c_char,
            (*a).name as *const ::core::ffi::c_char,
        ) as gint;
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_init(mut auth: *mut GDBusAuth) {
    (*auth).priv_0 = safe_c2rust__g_dbus_auth_get_instance_private(auth) as *mut GDBusAuthPrivate;
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_add_mechs(
    mut auth: *mut GDBusAuth,
    mut observer: *mut GDBusAuthObserver,
) {
    safe_c2rust_add_mechanism(auth, observer, _g_dbus_auth_mechanism_anon_get_type());
    safe_c2rust_add_mechanism(auth, observer, _g_dbus_auth_mechanism_sha1_get_type());
    safe_c2rust_add_mechanism(auth, observer, _g_dbus_auth_mechanism_external_get_type());
    (*(*auth).priv_0).available_mechanisms = g_list_sort(
        (*(*auth).priv_0).available_mechanisms,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Mechanism, *mut Mechanism) -> gint>,
            GCompareFunc,
        >(Some(
            safe_c2rust_mech_compare_func
                as unsafe extern "C" fn(*mut Mechanism, *mut Mechanism) -> gint,
        )),
    );
}
unsafe extern "C" fn safe_c2rust_find_mech_by_name(
    mut auth: *mut GDBusAuth,
    mut name: *const gchar,
) -> GType {
    let mut ret: GType = 0;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    ret = 0 as ::core::ffi::c_int as GType;
    l = (*(*auth).priv_0).available_mechanisms;
    while !l.is_null() {
        let mut m: *mut Mechanism = (*l).data as *mut Mechanism;
        if g_strcmp0(
            name as *const ::core::ffi::c_char,
            (*m).name as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            ret = (*m).gtype;
            break;
        } else {
            l = (*l).next;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_new(
    mut stream: *mut GIOStream,
) -> *mut GDBusAuth {
    return g_object_new(
        safe_c2rust__g_dbus_auth_get_type(),
        b"stream\0" as *const u8 as *const gchar,
        stream,
        NULL_0,
    ) as *mut GDBusAuth;
}
unsafe extern "C" fn safe_c2rust__my_g_data_input_stream_read_line(
    mut dis: *mut GDataInputStream,
    mut out_line_length: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    ret = g_data_input_stream_read_line(dis, out_line_length, cancellable, error) as *mut gchar;
    if ret.is_null() && !error.is_null() && (*error).is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Unexpected lack of content trying to read a line\0" as *const u8 as *const gchar,
            ),
        );
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust__my_g_input_stream_read_line_safe(
    mut i: *mut GInputStream,
    mut out_line_length: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut current_block: u64;
    let mut str: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut c: gchar = 0;
    let mut num_read: gssize = 0;
    let mut last_was_cr: gboolean = 0;
    str = g_string_new(::core::ptr::null::<gchar>());
    last_was_cr = FALSE as gboolean;
    loop {
        if !(FALSE == 0) {
            current_block = 3923820712304628395;
            break;
        }
        num_read = g_input_stream_read(
            i,
            &raw mut c as *mut ::core::ffi::c_void,
            1 as gsize,
            cancellable,
            error,
        );
        if num_read == -(1 as ::core::ffi::c_int) as gssize {
            current_block = 7620308590842075819;
            break;
        }
        if num_read == 0 as gssize {
            if !error.is_null() && (*error).is_null() {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Unexpected lack of content trying to (safely) read a line\0" as *const u8
                            as *const gchar,
                    ),
                );
            }
            current_block = 7620308590842075819;
            break;
        } else {
            safe_c2rust_g_string_append_c_inline(str, c as gint as gchar);
            if last_was_cr != 0 {
                if c as ::core::ffi::c_int == 0xa as ::core::ffi::c_int {
                    if ({
                        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                        if (*str).len >= 2 as gsize {
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
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauth.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            346 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"str->len >= 2\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    g_string_set_size(str, (*str).len.wrapping_sub(2 as gsize));
                    current_block = 3923820712304628395;
                    break;
                }
            }
            last_was_cr = (c as ::core::ffi::c_int == 0xd as ::core::ffi::c_int)
                as ::core::ffi::c_int as gboolean;
        }
    }
    match current_block {
        7620308590842075819 => {
            if ({
                let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                if error.is_null() || !(*error).is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauth.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    360 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error == NULL || *error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(str, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                } else {
                    g_string_free_and_steal(str);
                };
            } else {
                g_string_free(str, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            };
            return ::core::ptr::null_mut::<gchar>();
        }
        _ => {
            if !out_line_length.is_null() {
                *out_line_length = (*str).len;
            }
            return if 0 != 0 {
                if 0 as ::core::ffi::c_int != 0 {
                    g_string_free(str, 0 as gboolean)
                } else {
                    g_string_free_and_steal(str)
                }
            } else {
                g_string_free(str, 0 as gboolean)
            };
        }
    };
}
unsafe extern "C" fn safe_c2rust_hexdecode(
    mut str: *const gchar,
    mut out_len: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut current_block: u64;
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut n: guint = 0;
    ret = ::core::ptr::null_mut::<gchar>();
    s = g_string_new(::core::ptr::null::<gchar>());
    n = 0 as guint;
    loop {
        if !(*str.offset(n as isize) as ::core::ffi::c_int != '\0' as i32) {
            current_block = 13109137661213826276;
            break;
        }
        let mut upper_nibble: gint = 0;
        let mut lower_nibble: gint = 0;
        let mut value: guint = 0;
        upper_nibble = g_ascii_xdigit_value(*str.offset(n as isize));
        lower_nibble = g_ascii_xdigit_value(*str.offset(n.wrapping_add(1 as guint) as isize));
        if upper_nibble == -(1 as ::core::ffi::c_int) || lower_nibble == -(1 as ::core::ffi::c_int)
        {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                b"Error hexdecoding string '%s' around position %d\0" as *const u8 as *const gchar,
                str,
                n,
            );
            current_block = 16392933850869155373;
            break;
        } else {
            value = (upper_nibble << 4 as ::core::ffi::c_int | lower_nibble) as guint;
            safe_c2rust_g_string_append_c_inline(s, value as gchar);
            n = n.wrapping_add(2 as guint);
        }
    }
    match current_block {
        13109137661213826276 => {
            *out_len = (*s).len;
            ret = if 0 != 0 {
                if 0 as ::core::ffi::c_int != 0 {
                    g_string_free(s, 0 as gboolean)
                } else {
                    g_string_free_and_steal(s)
                }
            } else {
                g_string_free(s, 0 as gboolean)
            };
            s = ::core::ptr::null_mut::<GString>();
        }
        _ => {}
    }
    if !s.is_null() {
        *out_len = 0 as gsize;
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(s);
            };
        } else {
            g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_client_choose_mech_and_send_initial_response(
    mut auth: *mut GDBusAuth,
    mut credentials_that_were_sent: *mut GCredentials,
    mut conn_flags: GDBusConnectionFlags,
    mut supported_auth_mechs: *const *const gchar,
    mut attempted_auth_mechs: *mut GPtrArray,
    mut dos: *mut GDataOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusAuthMechanism {
    let mut current_block: u64;
    let mut mech: *mut GDBusAuthMechanism = ::core::ptr::null_mut::<GDBusAuthMechanism>();
    let mut auth_mech_to_use_gtype: GType = 0;
    let mut n: guint = 0;
    let mut m: guint = 0;
    let mut initial_response: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut initial_response_len: gsize = 0;
    let mut encoded: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    loop {
        mech = ::core::ptr::null_mut::<GDBusAuthMechanism>();
        safe_c2rust_debug_print(
            b"CLIENT: Trying to choose mechanism\0" as *const u8 as *const gchar,
        );
        auth_mech_to_use_gtype = 0 as ::core::ffi::c_int as GType;
        n = 0 as guint;
        while !(*supported_auth_mechs.offset(n as isize)).is_null() {
            let mut attempted_already: gboolean = 0;
            attempted_already = FALSE as gboolean;
            m = 0 as guint;
            while m < (*attempted_auth_mechs).len {
                if g_strcmp0(
                    *supported_auth_mechs.offset(n as isize) as *const ::core::ffi::c_char,
                    *(*attempted_auth_mechs).pdata.offset(m as isize) as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    attempted_already = TRUE as gboolean;
                    break;
                } else {
                    m = m.wrapping_add(1);
                }
            }
            if attempted_already == 0 {
                auth_mech_to_use_gtype =
                    safe_c2rust_find_mech_by_name(auth, *supported_auth_mechs.offset(n as isize));
                if auth_mech_to_use_gtype != 0 as ::core::ffi::c_int as GType {
                    break;
                }
            }
            n = n.wrapping_add(1);
        }
        if auth_mech_to_use_gtype == 0 as ::core::ffi::c_int as GType {
            let mut available: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut tried_str: *mut GString = ::core::ptr::null_mut::<GString>();
            safe_c2rust_debug_print(
                b"CLIENT: Exhausted all available mechanisms\0" as *const u8 as *const gchar,
            );
            available = g_strjoinv(
                b", \0" as *const u8 as *const gchar,
                supported_auth_mechs as *mut *mut gchar,
            );
            tried_str = g_string_new(::core::ptr::null::<gchar>());
            n = 0 as guint;
            while n < (*attempted_auth_mechs).len {
                if n > 0 as guint {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                b", \0" as *const u8 as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                tried_str,
                                __val,
                                if ({
                                    let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_18
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    strlen(
                                        __val
                                            .offset(__val.is_null() as ::core::ffi::c_int as isize),
                                    ) as gssize
                                } else {
                                    -(1 as ::core::ffi::c_int) as gssize
                                },
                            );
                        });
                    } else {
                        safe_c2rust_g_string_append_len_inline(
                            tried_str,
                            b", \0" as *const u8 as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                }
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            *(*attempted_auth_mechs).pdata.offset(n as isize)
                                as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            tried_str,
                            __val,
                            if ({
                                let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_19 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_19 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_19
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        tried_str,
                        *(*attempted_auth_mechs).pdata.offset(n as isize)
                            as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                n = n.wrapping_add(1);
            }
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Exhausted all available authentication mechanisms (tried: %s) (available: %s)\0"
                        as *const u8 as *const gchar,
                ),
                (*tried_str).str_0,
                available,
            );
            if 0 != 0 {
                if 0 as ::core::ffi::c_int == 0 {
                    g_string_free(
                        tried_str,
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                    );
                } else {
                    g_string_free_and_steal(tried_str);
                };
            } else {
                g_string_free(
                    tried_str,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            };
            g_free(available as gpointer);
            current_block = 18442129007059861454;
            break;
        } else {
            mech = g_object_new(
                auth_mech_to_use_gtype,
                b"stream\0" as *const u8 as *const gchar,
                (*(*auth).priv_0).stream,
                b"credentials\0" as *const u8 as *const ::core::ffi::c_char,
                credentials_that_were_sent,
                NULL_0,
            ) as *mut GDBusAuthMechanism;
            safe_c2rust_debug_print(
                b"CLIENT: Trying mechanism '%s'\0" as *const u8 as *const gchar,
                _g_dbus_auth_mechanism_get_name(auth_mech_to_use_gtype),
            );
            g_ptr_array_add(
                attempted_auth_mechs,
                _g_dbus_auth_mechanism_get_name(auth_mech_to_use_gtype) as gpointer,
            );
            if _g_dbus_auth_mechanism_is_supported(mech) == 0 {
                safe_c2rust_debug_print(
                    b"CLIENT: Mechanism '%s' says it is not supported\0" as *const u8
                        as *const gchar,
                    _g_dbus_auth_mechanism_get_name(auth_mech_to_use_gtype),
                );
                g_object_unref(mech as gpointer);
                mech = ::core::ptr::null_mut::<GDBusAuthMechanism>();
            } else {
                initial_response_len = 0 as gsize;
                initial_response = _g_dbus_auth_mechanism_client_initiate(
                    mech,
                    conn_flags,
                    &raw mut initial_response_len,
                );
                if !initial_response.is_null() {
                    encoded = _g_dbus_hexencode(initial_response, initial_response_len);
                    s = g_strdup_printf(
                        b"AUTH %s %s\r\n\0" as *const u8 as *const gchar,
                        _g_dbus_auth_mechanism_get_name(auth_mech_to_use_gtype),
                        encoded,
                    );
                    g_free(initial_response as gpointer);
                    g_free(encoded as gpointer);
                } else {
                    s = g_strdup_printf(
                        b"AUTH %s\r\n\0" as *const u8 as *const gchar,
                        _g_dbus_auth_mechanism_get_name(auth_mech_to_use_gtype),
                    );
                }
                safe_c2rust_debug_print(b"CLIENT: writing '%s'\0" as *const u8 as *const gchar, s);
                if g_data_output_stream_put_string(dos, s, cancellable, error) == 0 {
                    current_block = 10891380440665537214;
                    break;
                } else {
                    current_block = 790185930182612747;
                    break;
                }
            }
        }
    }
    match current_block {
        10891380440665537214 => {
            g_object_unref(mech as gpointer);
            mech = ::core::ptr::null_mut::<GDBusAuthMechanism>();
            g_free(s as gpointer);
        }
        790185930182612747 => {
            g_free(s as gpointer);
        }
        _ => {}
    }
    return mech;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_run_client(
    mut auth: *mut GDBusAuth,
    mut observer: *mut GDBusAuthObserver,
    mut conn_flags: GDBusConnectionFlags,
    mut offered_capabilities: GDBusCapabilityFlags,
    mut out_negotiated_capabilities: *mut GDBusCapabilityFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut current_block: u64;
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut dis: *mut GDataInputStream = ::core::ptr::null_mut::<GDataInputStream>();
    let mut dos: *mut GDataOutputStream = ::core::ptr::null_mut::<GDataOutputStream>();
    let mut credentials: *mut GCredentials = ::core::ptr::null_mut::<GCredentials>();
    let mut ret_guid: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut line: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut line_length: gsize = 0;
    let mut supported_auth_mechs: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut attempted_auth_mechs: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut mech: *mut GDBusAuthMechanism = ::core::ptr::null_mut::<GDBusAuthMechanism>();
    let mut state: ClientState = CLIENT_STATE_WAITING_FOR_DATA;
    let mut negotiated_capabilities: GDBusCapabilityFlags = G_DBUS_CAPABILITY_FLAGS_NONE;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if conn_flags as ::core::ffi::c_uint
            & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT as ::core::ffi::c_int
                as ::core::ffi::c_uint
            != 0
        {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(conn_flags & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if conn_flags as ::core::ffi::c_uint
            & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER as ::core::ffi::c_int
                as ::core::ffi::c_uint
            == 0
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
            b"!(conn_flags & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    safe_c2rust_debug_print(b"CLIENT: initiating\0" as *const u8 as *const gchar);
    safe_c2rust__g_dbus_auth_add_mechs(auth, observer);
    ret_guid = ::core::ptr::null_mut::<gchar>();
    supported_auth_mechs = ::core::ptr::null_mut::<*mut gchar>();
    attempted_auth_mechs = g_ptr_array_new();
    mech = ::core::ptr::null_mut::<GDBusAuthMechanism>();
    negotiated_capabilities = G_DBUS_CAPABILITY_FLAGS_NONE;
    credentials = ::core::ptr::null_mut::<GCredentials>();
    dis = g_data_input_stream_new(g_io_stream_get_input_stream((*(*auth).priv_0).stream))
        as *mut ::core::ffi::c_void as *mut GDataInputStream;
    dos = g_data_output_stream_new(g_io_stream_get_output_stream((*(*auth).priv_0).stream))
        as *mut ::core::ffi::c_void as *mut GDataOutputStream;
    g_filter_input_stream_set_close_base_stream(
        dis as *mut ::core::ffi::c_void as *mut GFilterInputStream,
        FALSE,
    );
    g_filter_output_stream_set_close_base_stream(
        dos as *mut ::core::ffi::c_void as *mut GFilterOutputStream,
        FALSE,
    );
    g_data_input_stream_set_newline_type(dis, G_DATA_STREAM_NEWLINE_TYPE_CR_LF);
    if ({
        let mut __inst: *mut GTypeInstance = (*(*auth).priv_0).stream as *mut GTypeInstance;
        let mut __t: GType = g_unix_connection_get_type();
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
        credentials = g_credentials_new();
        if g_unix_connection_send_credentials(
            (*(*auth).priv_0).stream as *mut ::core::ffi::c_void as *mut GUnixConnection,
            cancellable,
            error,
        ) == 0
        {
            current_block = 4077107830511629165;
        } else {
            current_block = 3437258052017859086;
        }
    } else if g_data_output_stream_put_byte(dos, '\0' as i32 as guchar, cancellable, error) == 0 {
        current_block = 4077107830511629165;
    } else {
        current_block = 3437258052017859086;
    }
    match current_block {
        3437258052017859086 => {
            if !credentials.is_null() {
                if ({
                    let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                    if _g_dbus_debug_authentication() != 0 {
                        _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_22
                }) as ::core::ffi::c_long
                    != 0
                {
                    s = g_credentials_to_string(credentials);
                    safe_c2rust_debug_print(
                        b"CLIENT: sent credentials '%s'\0" as *const u8 as *const gchar,
                        s,
                    );
                    g_free(s as gpointer);
                }
            } else {
                safe_c2rust_debug_print(
                    b"CLIENT: didn't send any credentials\0" as *const u8 as *const gchar,
                );
            }
            s = b"AUTH\r\n\0" as *const u8 as *const ::core::ffi::c_char as *mut gchar;
            safe_c2rust_debug_print(b"CLIENT: writing '%s'\0" as *const u8 as *const gchar, s);
            if !(g_data_output_stream_put_string(dos, s, cancellable, error) == 0) {
                state = CLIENT_STATE_WAITING_FOR_REJECT;
                while FALSE == 0 {
                    match state as ::core::ffi::c_uint {
                        2 => {
                            safe_c2rust_debug_print(
                                b"CLIENT: WaitingForReject\0" as *const u8 as *const gchar,
                            );
                            line = safe_c2rust__my_g_data_input_stream_read_line(
                                dis,
                                &raw mut line_length,
                                cancellable,
                                error,
                            );
                            if line.is_null() {
                                break;
                            }
                            safe_c2rust_debug_print(
                                b"CLIENT: WaitingForReject, read '%s'\0" as *const u8
                                    as *const gchar,
                                line,
                            );
                        }
                        1 => {
                            safe_c2rust_debug_print(
                                b"CLIENT: WaitingForOK\0" as *const u8 as *const gchar,
                            );
                            line = safe_c2rust__my_g_data_input_stream_read_line(
                                dis,
                                &raw mut line_length,
                                cancellable,
                                error,
                            );
                            if line.is_null() {
                                break;
                            }
                            safe_c2rust_debug_print(
                                b"CLIENT: WaitingForOK, read '%s'\0" as *const u8 as *const gchar,
                                line,
                            );
                            if if 0 != 0 {
                                ({
                                    let __str: *const ::core::ffi::c_char = line;
                                    let __prefix: *const ::core::ffi::c_char =
                                        b"OK \0" as *const u8 as *const ::core::ffi::c_char;
                                    let mut __result: gboolean = FALSE;
                                    if ({
                                        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                                        if __str.is_null() || __prefix.is_null() {
                                            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                                        } else {
                                            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                                        }
                                        _g_boolean_var_24
                                    }) as ::core::ffi::c_long
                                        != 0
                                    {
                                        __result = g_str_has_prefix(
                                            __str as *const gchar,
                                            __prefix as *const gchar,
                                        );
                                    } else {
                                        let __str_len: size_t =
                                            strlen(__str.offset(__str.is_null()
                                                as ::core::ffi::c_int
                                                as isize))
                                                as size_t;
                                        let __prefix_len: size_t = strlen(__prefix.offset(
                                            __prefix.is_null() as ::core::ffi::c_int as isize,
                                        ))
                                            as size_t;
                                        if __str_len >= __prefix_len {
                                            __result = (memcmp(
                                                __str
                                                    .offset(__str.is_null() as ::core::ffi::c_int
                                                        as isize)
                                                    as *const ::core::ffi::c_void,
                                                __prefix.offset(__prefix.is_null()
                                                    as ::core::ffi::c_int
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
                                g_str_has_prefix(line, b"OK \0" as *const u8 as *const gchar)
                            } != 0
                            {
                                if g_dbus_is_guid(line.offset(3 as ::core::ffi::c_int as isize))
                                    == 0
                                {
                                    g_set_error(
                                        error,
                                        g_io_error_quark(),
                                        G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                        b"Invalid OK response '%s'\0" as *const u8 as *const gchar,
                                        line,
                                    );
                                    g_free(line as gpointer);
                                    break;
                                } else {
                                    ret_guid = safe_c2rust_g_strdup_inline(
                                        line.offset(3 as ::core::ffi::c_int as isize),
                                    ) as *mut gchar;
                                    g_free(line as gpointer);
                                    if offered_capabilities as ::core::ffi::c_uint
                                        & G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                        != 0
                                    {
                                        s = b"NEGOTIATE_UNIX_FD\r\n\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut gchar;
                                        safe_c2rust_debug_print(
                                            b"CLIENT: writing '%s'\0" as *const u8 as *const gchar,
                                            s,
                                        );
                                        if g_data_output_stream_put_string(
                                            dos,
                                            s,
                                            cancellable,
                                            error,
                                        ) == 0
                                        {
                                            break;
                                        }
                                        state = CLIENT_STATE_WAITING_FOR_AGREE_UNIX_FD;
                                        continue;
                                    } else {
                                        s = b"BEGIN\r\n\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut gchar;
                                        safe_c2rust_debug_print(
                                            b"CLIENT: writing '%s'\0" as *const u8 as *const gchar,
                                            s,
                                        );
                                        if g_data_output_stream_put_string(
                                            dos,
                                            s,
                                            cancellable,
                                            error,
                                        ) == 0
                                        {
                                            break;
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            } else if !(if 0 != 0 {
                                ({
                                    let __str: *const ::core::ffi::c_char = line;
                                    let __prefix: *const ::core::ffi::c_char =
                                        b"REJECTED \0" as *const u8 as *const ::core::ffi::c_char;
                                    let mut __result: gboolean = FALSE;
                                    if ({
                                        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                                        if __str.is_null() || __prefix.is_null() {
                                            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                                        } else {
                                            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                                        }
                                        _g_boolean_var_25
                                    }) as ::core::ffi::c_long
                                        != 0
                                    {
                                        __result = g_str_has_prefix(
                                            __str as *const gchar,
                                            __prefix as *const gchar,
                                        );
                                    } else {
                                        let __str_len: size_t =
                                            strlen(__str.offset(__str.is_null()
                                                as ::core::ffi::c_int
                                                as isize))
                                                as size_t;
                                        let __prefix_len: size_t = strlen(__prefix.offset(
                                            __prefix.is_null() as ::core::ffi::c_int as isize,
                                        ))
                                            as size_t;
                                        if __str_len >= __prefix_len {
                                            __result = (memcmp(
                                                __str
                                                    .offset(__str.is_null() as ::core::ffi::c_int
                                                        as isize)
                                                    as *const ::core::ffi::c_void,
                                                __prefix.offset(__prefix.is_null()
                                                    as ::core::ffi::c_int
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
                                g_str_has_prefix(line, b"REJECTED \0" as *const u8 as *const gchar)
                            } != 0)
                            {
                                g_set_error(
                                    error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                    b"In WaitingForOk: unexpected response '%s'\0" as *const u8
                                        as *const gchar,
                                    line,
                                );
                                g_free(line as gpointer);
                                break;
                            }
                        }
                        3 => {
                            safe_c2rust_debug_print(
                                b"CLIENT: WaitingForAgreeUnixFD\0" as *const u8 as *const gchar,
                            );
                            line = safe_c2rust__my_g_data_input_stream_read_line(
                                dis,
                                &raw mut line_length,
                                cancellable,
                                error,
                            );
                            if line.is_null() {
                                break;
                            }
                            safe_c2rust_debug_print(
                                b"CLIENT: WaitingForAgreeUnixFD, read='%s'\0" as *const u8
                                    as *const gchar,
                                line,
                            );
                            if g_strcmp0(
                                line,
                                b"AGREE_UNIX_FD\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                g_free(line as gpointer);
                                negotiated_capabilities = ::core::mem::transmute::<
                                    ::core::ffi::c_uint,
                                    GDBusCapabilityFlags,
                                >(
                                    negotiated_capabilities as ::core::ffi::c_uint
                                        | G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uint,
                                );
                                s = b"BEGIN\r\n\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut gchar;
                                safe_c2rust_debug_print(
                                    b"CLIENT: writing '%s'\0" as *const u8 as *const gchar,
                                    s,
                                );
                                if g_data_output_stream_put_string(dos, s, cancellable, error) == 0
                                {
                                    break;
                                } else {
                                    break;
                                }
                            } else if (if 0 != 0 {
                                ({
                                    let __str: *const ::core::ffi::c_char = line;
                                    let __prefix: *const ::core::ffi::c_char =
                                        b"ERROR\0" as *const u8 as *const ::core::ffi::c_char;
                                    let mut __result: gboolean = FALSE;
                                    if ({
                                        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
                                        if __str.is_null() || __prefix.is_null() {
                                            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
                                        } else {
                                            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
                                        }
                                        _g_boolean_var_26
                                    }) as ::core::ffi::c_long
                                        != 0
                                    {
                                        __result = g_str_has_prefix(
                                            __str as *const gchar,
                                            __prefix as *const gchar,
                                        );
                                    } else {
                                        let __str_len: size_t =
                                            strlen(__str.offset(__str.is_null()
                                                as ::core::ffi::c_int
                                                as isize))
                                                as size_t;
                                        let __prefix_len: size_t = strlen(__prefix.offset(
                                            __prefix.is_null() as ::core::ffi::c_int as isize,
                                        ))
                                            as size_t;
                                        if __str_len >= __prefix_len {
                                            __result = (memcmp(
                                                __str
                                                    .offset(__str.is_null() as ::core::ffi::c_int
                                                        as isize)
                                                    as *const ::core::ffi::c_void,
                                                __prefix.offset(__prefix.is_null()
                                                    as ::core::ffi::c_int
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
                                g_str_has_prefix(line, b"ERROR\0" as *const u8 as *const gchar)
                            }) != 0
                                && (*line.offset(5 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                    || *safe_c2rust_g_ascii_table
                                        .offset(*line.offset(5 as ::core::ffi::c_int as isize)
                                            as guchar
                                            as isize)
                                        as ::core::ffi::c_int
                                        & G_ASCII_SPACE as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int)
                            {
                                g_free(line as gpointer);
                                s = b"BEGIN\r\n\0" as *const u8 as *const ::core::ffi::c_char
                                    as *mut gchar;
                                safe_c2rust_debug_print(
                                    b"CLIENT: writing '%s'\0" as *const u8 as *const gchar,
                                    s,
                                );
                                if g_data_output_stream_put_string(dos, s, cancellable, error) == 0
                                {
                                    break;
                                } else {
                                    break;
                                }
                            } else {
                                g_set_error(
                                    error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                    b"In WaitingForAgreeUnixFd: unexpected response '%s'\0"
                                        as *const u8
                                        as *const gchar,
                                    line,
                                );
                                g_free(line as gpointer);
                                break;
                            }
                        }
                        0 => {
                            safe_c2rust_debug_print(
                                b"CLIENT: WaitingForData\0" as *const u8 as *const gchar,
                            );
                            line = safe_c2rust__my_g_data_input_stream_read_line(
                                dis,
                                &raw mut line_length,
                                cancellable,
                                error,
                            );
                            if line.is_null() {
                                break;
                            }
                            safe_c2rust_debug_print(
                                b"CLIENT: WaitingForData, read='%s'\0" as *const u8 as *const gchar,
                                line,
                            );
                            if strcmp(
                                line as *const ::core::ffi::c_char,
                                b"DATA\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                                || (if 0 != 0 {
                                    ({
                                        let __str: *const ::core::ffi::c_char = line;
                                        let __prefix: *const ::core::ffi::c_char =
                                            b"DATA \0" as *const u8 as *const ::core::ffi::c_char;
                                        let mut __result: gboolean = FALSE;
                                        if ({
                                            let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
                                            if __str.is_null() || __prefix.is_null() {
                                                _g_boolean_var_27 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_27 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_27
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                            __result = g_str_has_prefix(
                                                __str as *const gchar,
                                                __prefix as *const gchar,
                                            );
                                        } else {
                                            let __str_len: size_t = strlen(__str.offset(
                                                __str.is_null() as ::core::ffi::c_int as isize,
                                            ))
                                                as size_t;
                                            let __prefix_len: size_t =
                                                strlen(__prefix.offset(__prefix.is_null()
                                                    as ::core::ffi::c_int
                                                    as isize))
                                                    as size_t;
                                            if __str_len >= __prefix_len {
                                                __result = (memcmp(
                                                    __str.offset(__str.is_null()
                                                        as ::core::ffi::c_int
                                                        as isize)
                                                        as *const ::core::ffi::c_void,
                                                    __prefix.offset(__prefix.is_null()
                                                        as ::core::ffi::c_int
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
                                    g_str_has_prefix(line, b"DATA \0" as *const u8 as *const gchar)
                                }) != 0
                            {
                                let mut encoded: *mut gchar = ::core::ptr::null_mut::<gchar>();
                                let mut decoded_data: *mut gchar = ::core::ptr::null_mut::<gchar>();
                                let mut decoded_data_len: gsize = 0 as gsize;
                                encoded = safe_c2rust_g_strdup_inline(
                                    line.offset(4 as ::core::ffi::c_int as isize),
                                ) as *mut gchar;
                                g_free(line as gpointer);
                                g_strchomp(g_strchug(encoded));
                                decoded_data = safe_c2rust_hexdecode(
                                    encoded,
                                    &raw mut decoded_data_len,
                                    error,
                                );
                                g_free(encoded as gpointer);
                                if decoded_data.is_null() {
                                    g_prefix_error(
                                        error,
                                        b"DATA response is malformed: \0" as *const u8
                                            as *const gchar,
                                    );
                                    break;
                                } else {
                                    _g_dbus_auth_mechanism_client_data_receive(
                                        mech,
                                        decoded_data,
                                        decoded_data_len,
                                    );
                                    g_free(decoded_data as gpointer);
                                    if _g_dbus_auth_mechanism_client_get_state(mech)
                                        as ::core::ffi::c_uint
                                        == G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND
                                            as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        let mut data: *mut gchar = ::core::ptr::null_mut::<gchar>();
                                        let mut data_len: gsize = 0;
                                        data = _g_dbus_auth_mechanism_client_data_send(
                                            mech,
                                            &raw mut data_len,
                                        );
                                        if data_len == 0 as gsize {
                                            s = safe_c2rust_g_strdup_inline(
                                                b"DATA\r\n\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            )
                                                as *mut gchar;
                                        } else {
                                            let mut encoded_data: *mut gchar =
                                                _g_dbus_hexencode(data, data_len);
                                            s = g_strdup_printf(
                                                b"DATA %s\r\n\0" as *const u8 as *const gchar,
                                                encoded_data,
                                            );
                                            g_free(encoded_data as gpointer);
                                        }
                                        g_free(data as gpointer);
                                        safe_c2rust_debug_print(
                                            b"CLIENT: writing '%s'\0" as *const u8 as *const gchar,
                                            s,
                                        );
                                        if g_data_output_stream_put_string(
                                            dos,
                                            s,
                                            cancellable,
                                            error,
                                        ) == 0
                                        {
                                            g_free(s as gpointer);
                                            break;
                                        } else {
                                            g_free(s as gpointer);
                                        }
                                    }
                                    state = CLIENT_STATE_WAITING_FOR_OK;
                                    continue;
                                }
                            } else if !(if 0 != 0 {
                                ({
                                    let __str: *const ::core::ffi::c_char = line;
                                    let __prefix: *const ::core::ffi::c_char =
                                        b"REJECTED \0" as *const u8 as *const ::core::ffi::c_char;
                                    let mut __result: gboolean = FALSE;
                                    if ({
                                        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
                                        if __str.is_null() || __prefix.is_null() {
                                            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
                                        } else {
                                            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
                                        }
                                        _g_boolean_var_28
                                    }) as ::core::ffi::c_long
                                        != 0
                                    {
                                        __result = g_str_has_prefix(
                                            __str as *const gchar,
                                            __prefix as *const gchar,
                                        );
                                    } else {
                                        let __str_len: size_t =
                                            strlen(__str.offset(__str.is_null()
                                                as ::core::ffi::c_int
                                                as isize))
                                                as size_t;
                                        let __prefix_len: size_t = strlen(__prefix.offset(
                                            __prefix.is_null() as ::core::ffi::c_int as isize,
                                        ))
                                            as size_t;
                                        if __str_len >= __prefix_len {
                                            __result = (memcmp(
                                                __str
                                                    .offset(__str.is_null() as ::core::ffi::c_int
                                                        as isize)
                                                    as *const ::core::ffi::c_void,
                                                __prefix.offset(__prefix.is_null()
                                                    as ::core::ffi::c_int
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
                                g_str_has_prefix(line, b"REJECTED \0" as *const u8 as *const gchar)
                            } != 0)
                            {
                                g_set_error(
                                    error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                    b"In WaitingForData: unexpected response '%s'\0" as *const u8
                                        as *const gchar,
                                    line,
                                );
                                g_free(line as gpointer);
                                break;
                            }
                        }
                        _ => {
                            g_assertion_message_expr(
                                G_LOG_DOMAIN.as_ptr(),
                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauth.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                861 as ::core::ffi::c_int,
                                G_STRFUNC,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                            );
                        }
                    }
                    if if 0 != 0 {
                        ({
                            let __str: *const ::core::ffi::c_char = line;
                            let __prefix: *const ::core::ffi::c_char =
                                b"REJECTED \0" as *const u8 as *const ::core::ffi::c_char;
                            let mut __result: gboolean = FALSE;
                            if ({
                                let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                                if __str.is_null() || __prefix.is_null() {
                                    _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_23
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
                        g_str_has_prefix(line, b"REJECTED \0" as *const u8 as *const gchar)
                    } == 0
                    {
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                            b"In WaitingForReject: Expected 'REJECTED am1 am2 ... amN', got '%s'\0"
                                as *const u8 as *const gchar,
                            line,
                        );
                        g_free(line as gpointer);
                        break;
                    } else {
                        if supported_auth_mechs.is_null() {
                            supported_auth_mechs = g_strsplit(
                                line.offset(::core::mem::size_of::<[::core::ffi::c_char; 10]>()
                                    as usize as isize)
                                    .offset(-(1 as ::core::ffi::c_int as isize)),
                                b" \0" as *const u8 as *const gchar,
                                0 as gint,
                            );
                        }
                        g_free(line as gpointer);
                        mech = safe_c2rust_client_choose_mech_and_send_initial_response(
                            auth,
                            credentials,
                            conn_flags,
                            supported_auth_mechs as *const *const gchar,
                            attempted_auth_mechs,
                            dos,
                            cancellable,
                            error,
                        );
                        if mech.is_null() {
                            break;
                        }
                        if _g_dbus_auth_mechanism_client_get_state(mech) as ::core::ffi::c_uint
                            == G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                        {
                            state = CLIENT_STATE_WAITING_FOR_DATA;
                        } else {
                            state = CLIENT_STATE_WAITING_FOR_OK;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !mech.is_null() {
        g_object_unref(mech as gpointer);
    }
    g_ptr_array_unref(attempted_auth_mechs);
    g_strfreev(supported_auth_mechs);
    g_object_unref(dis as gpointer);
    g_object_unref(dos as gpointer);
    if !error.is_null() && !(*error).is_null() {
        g_free(ret_guid as gpointer);
        ret_guid = ::core::ptr::null_mut::<gchar>();
    }
    if !ret_guid.is_null() {
        if !out_negotiated_capabilities.is_null() {
            *out_negotiated_capabilities = negotiated_capabilities;
        }
    }
    if !credentials.is_null() {
        g_object_unref(credentials as gpointer);
    }
    safe_c2rust_debug_print(
        b"CLIENT: Done, authenticated=%d\0" as *const u8 as *const gchar,
        (ret_guid != NULL_0 as *mut gchar) as ::core::ffi::c_int,
    );
    return ret_guid;
}
unsafe extern "C" fn safe_c2rust_get_auth_mechanisms(
    mut auth: *mut GDBusAuth,
    mut allow_anonymous: gboolean,
    mut prefix: *const gchar,
    mut suffix: *const gchar,
    mut separator: *const gchar,
) -> *mut gchar {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut str: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut need_sep: gboolean = 0;
    str = g_string_new(prefix);
    need_sep = FALSE as gboolean;
    l = (*(*auth).priv_0).available_mechanisms;
    while !l.is_null() {
        let mut m: *mut Mechanism = (*l).data as *mut Mechanism;
        if !(allow_anonymous == 0
            && g_strcmp0(
                (*m).name as *const ::core::ffi::c_char,
                b"ANONYMOUS\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
        {
            if need_sep != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            separator as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            str,
                            __val,
                            if ({
                                let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_29 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_29 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_29
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        separator as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = (*m).name as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        str,
                        __val,
                        if ({
                            let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_30 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_30 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_30
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as ::core::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                safe_c2rust_g_string_append_len_inline(
                    str,
                    (*m).name as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            need_sep = TRUE as gboolean;
        }
        l = (*l).next;
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = suffix as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                str,
                __val,
                if ({
                    let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_31 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_31 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_31
                }) as ::core::ffi::c_long
                    != 0
                {
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            str,
            suffix as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(str, 0 as gboolean)
        } else {
            g_string_free_and_steal(str)
        }
    } else {
        g_string_free(str, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_run_server(
    mut auth: *mut GDBusAuth,
    mut observer: *mut GDBusAuthObserver,
    mut guid: *const gchar,
    mut allow_anonymous: gboolean,
    mut require_same_user: gboolean,
    mut offered_capabilities: GDBusCapabilityFlags,
    mut out_negotiated_capabilities: *mut GDBusCapabilityFlags,
    mut out_received_credentials: *mut *mut GCredentials,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut tokens: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut encoded: *const gchar = ::core::ptr::null::<gchar>();
    let mut mech_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut auth_mech_to_use_gtype: GType = 0;
    let mut initial_response: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut initial_response_len: gsize = 0;
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut state: ServerState = SERVER_STATE_WAITING_FOR_AUTH;
    let mut dos: *mut GDataOutputStream = ::core::ptr::null_mut::<GDataOutputStream>();
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut line: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut line_length: gsize = 0;
    let mut mech: *mut GDBusAuthMechanism = ::core::ptr::null_mut::<GDBusAuthMechanism>();
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut negotiated_capabilities: GDBusCapabilityFlags = G_DBUS_CAPABILITY_FLAGS_NONE;
    let mut credentials: *mut GCredentials = ::core::ptr::null_mut::<GCredentials>();
    let mut own_credentials: *mut GCredentials = ::core::ptr::null_mut::<GCredentials>();
    safe_c2rust_debug_print(b"SERVER: initiating\0" as *const u8 as *const gchar);
    safe_c2rust__g_dbus_auth_add_mechs(auth, observer);
    ret = FALSE as gboolean;
    dos = ::core::ptr::null_mut::<GDataOutputStream>();
    mech = ::core::ptr::null_mut::<GDBusAuthMechanism>();
    negotiated_capabilities = G_DBUS_CAPABILITY_FLAGS_NONE;
    credentials = ::core::ptr::null_mut::<GCredentials>();
    if g_dbus_is_guid(guid) == 0 {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            b"The given GUID '%s' is not valid\0" as *const u8 as *const gchar,
            guid,
        );
    } else {
        dos = g_data_output_stream_new(g_io_stream_get_output_stream((*(*auth).priv_0).stream))
            as *mut ::core::ffi::c_void as *mut GDataOutputStream;
        g_filter_output_stream_set_close_base_stream(
            dos as *mut ::core::ffi::c_void as *mut GFilterOutputStream,
            FALSE,
        );
        if ({
            let mut __inst: *mut GTypeInstance = (*(*auth).priv_0).stream as *mut GTypeInstance;
            let mut __t: GType = g_socket_connection_get_type();
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
            let mut sock: *mut GSocket = g_socket_connection_get_socket(
                (*(*auth).priv_0).stream as *mut ::core::ffi::c_void as *mut GSocketConnection,
            );
            local_error = ::core::ptr::null_mut::<GError>();
            credentials = g_socket_get_credentials(sock, &raw mut local_error);
            if credentials.is_null()
                && g_error_matches(
                    local_error,
                    g_io_error_quark(),
                    G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                ) == 0
            {
                g_propagate_error(error, local_error);
                current_block = 16086139763999343087;
            } else {
                g_clear_error(&raw mut local_error);
                current_block = 5689001924483802034;
            }
        } else {
            current_block = 5689001924483802034;
        }
        match current_block {
            16086139763999343087 => {}
            _ => {
                if credentials.is_null()
                    && ({
                        let mut __inst: *mut GTypeInstance =
                            (*(*auth).priv_0).stream as *mut GTypeInstance;
                        let mut __t: GType = g_unix_connection_get_type();
                        let mut __r: gboolean = 0;
                        if __inst.is_null() {
                            __r = FALSE as gboolean;
                        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t
                        {
                            __r = TRUE as gboolean;
                        } else {
                            __r = g_type_check_instance_is_a(__inst, __t);
                        }
                        __r
                    }) != 0
                {
                    local_error = ::core::ptr::null_mut::<GError>();
                    credentials = g_unix_connection_receive_credentials(
                        (*(*auth).priv_0).stream as *mut ::core::ffi::c_void
                            as *mut GUnixConnection,
                        cancellable,
                        &raw mut local_error,
                    );
                    if credentials.is_null()
                        && g_error_matches(
                            local_error,
                            g_io_error_quark(),
                            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                        ) == 0
                    {
                        g_propagate_error(error, local_error);
                        current_block = 16086139763999343087;
                    } else {
                        g_clear_error(&raw mut local_error);
                        current_block = 7205609094909031804;
                    }
                } else {
                    let mut c: gchar = 0;
                    let mut num_read: gssize = 0;
                    local_error = ::core::ptr::null_mut::<GError>();
                    num_read = g_input_stream_read(
                        g_io_stream_get_input_stream((*(*auth).priv_0).stream),
                        &raw mut c as *mut ::core::ffi::c_void,
                        1 as gsize,
                        cancellable,
                        &raw mut local_error,
                    );
                    if num_read != 1 as gssize || !local_error.is_null() {
                        if local_error.is_null() {
                            g_set_error_literal(
                                error,
                                g_io_error_quark(),
                                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"Unexpected lack of content trying to read a byte\0"
                                        as *const u8
                                        as *const gchar,
                                ),
                            );
                        } else {
                            g_propagate_error(error, local_error);
                        }
                        current_block = 16086139763999343087;
                    } else {
                        current_block = 7205609094909031804;
                    }
                }
                match current_block {
                    16086139763999343087 => {}
                    _ => {
                        if !credentials.is_null() {
                            if ({
                                let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
                                if _g_dbus_debug_authentication() != 0 {
                                    _g_boolean_var_32 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_32 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_32
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                s = g_credentials_to_string(credentials);
                                safe_c2rust_debug_print(
                                    b"SERVER: received credentials '%s'\0" as *const u8
                                        as *const gchar,
                                    s,
                                );
                                g_free(s as gpointer);
                            }
                        } else {
                            safe_c2rust_debug_print(
                                b"SERVER: didn't receive any credentials\0" as *const u8
                                    as *const gchar,
                            );
                        }
                        own_credentials = g_credentials_new();
                        state = SERVER_STATE_WAITING_FOR_AUTH;
                        's_193: loop {
                            if !(FALSE == 0) {
                                current_block = 11354253847736050364;
                                break;
                            }
                            match state as ::core::ffi::c_uint {
                                0 => {
                                    safe_c2rust_debug_print(
                                        b"SERVER: WaitingForAuth\0" as *const u8 as *const gchar,
                                    );
                                    line = safe_c2rust__my_g_input_stream_read_line_safe(
                                        g_io_stream_get_input_stream((*(*auth).priv_0).stream),
                                        &raw mut line_length,
                                        cancellable,
                                        error,
                                    );
                                    safe_c2rust_debug_print(
                                        b"SERVER: WaitingForAuth, read '%s'\0" as *const u8
                                            as *const gchar,
                                        line,
                                    );
                                    if line.is_null() {
                                        current_block = 16086139763999343087;
                                        break;
                                    }
                                    if g_strcmp0(
                                        line,
                                        b"AUTH\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        s = safe_c2rust_get_auth_mechanisms(
                                            auth,
                                            allow_anonymous,
                                            b"REJECTED \0" as *const u8 as *const gchar,
                                            b"\r\n\0" as *const u8 as *const gchar,
                                            b" \0" as *const u8 as *const gchar,
                                        );
                                        safe_c2rust_debug_print(
                                            b"SERVER: writing '%s'\0" as *const u8 as *const gchar,
                                            s,
                                        );
                                        if g_data_output_stream_put_string(
                                            dos,
                                            s,
                                            cancellable,
                                            error,
                                        ) == 0
                                        {
                                            g_free(s as gpointer);
                                            g_free(line as gpointer);
                                            current_block = 16086139763999343087;
                                            break;
                                        } else {
                                            g_free(s as gpointer);
                                            g_free(line as gpointer);
                                            continue;
                                        }
                                    } else if if 0 != 0 {
                                        ({
                                            let __str: *const ::core::ffi::c_char = line;
                                            let __prefix: *const ::core::ffi::c_char = b"AUTH \0"
                                                as *const u8
                                                as *const ::core::ffi::c_char;
                                            let mut __result: gboolean = FALSE;
                                            if ({
                                                let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
                                                if __str.is_null() || __prefix.is_null() {
                                                    _g_boolean_var_33 = 1 as ::core::ffi::c_int;
                                                } else {
                                                    _g_boolean_var_33 = 0 as ::core::ffi::c_int;
                                                }
                                                _g_boolean_var_33
                                            })
                                                as ::core::ffi::c_long
                                                != 0
                                            {
                                                __result = g_str_has_prefix(
                                                    __str as *const gchar,
                                                    __prefix as *const gchar,
                                                );
                                            } else {
                                                let __str_len: size_t =
                                                    strlen(__str.offset(__str.is_null()
                                                        as ::core::ffi::c_int
                                                        as isize))
                                                        as size_t;
                                                let __prefix_len: size_t =
                                                    strlen(__prefix.offset(__prefix.is_null()
                                                        as ::core::ffi::c_int
                                                        as isize))
                                                        as size_t;
                                                if __str_len >= __prefix_len {
                                                    __result = (memcmp(
                                                        __str.offset(__str.is_null()
                                                            as ::core::ffi::c_int
                                                            as isize)
                                                            as *const ::core::ffi::c_void,
                                                        __prefix.offset(__prefix.is_null()
                                                            as ::core::ffi::c_int
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
                                        g_str_has_prefix(
                                            line,
                                            b"AUTH \0" as *const u8 as *const gchar,
                                        )
                                    } != 0
                                    {
                                        tokens = ::core::ptr::null_mut::<*mut gchar>();
                                        encoded = ::core::ptr::null::<gchar>();
                                        mech_name = ::core::ptr::null::<gchar>();
                                        auth_mech_to_use_gtype = 0;
                                        tokens = g_strsplit(
                                            line,
                                            b" \0" as *const u8 as *const gchar,
                                            0 as gint,
                                        );
                                        match g_strv_length(tokens) {
                                            2 => {
                                                mech_name = *tokens
                                                    .offset(1 as ::core::ffi::c_int as isize);
                                                encoded = ::core::ptr::null::<gchar>();
                                            }
                                            3 => {
                                                mech_name = *tokens
                                                    .offset(1 as ::core::ffi::c_int as isize);
                                                encoded = *tokens
                                                    .offset(2 as ::core::ffi::c_int as isize);
                                            }
                                            _ => {
                                                g_set_error(
                                                    error,
                                                    g_io_error_quark(),
                                                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                                    b"Unexpected line '%s' while in WaitingForAuth state\0"
                                                        as *const u8 as *const gchar,
                                                    line,
                                                );
                                                g_strfreev(tokens);
                                                g_free(line as gpointer);
                                                current_block = 16086139763999343087;
                                                break;
                                            }
                                        }
                                        g_free(line as gpointer);
                                        auth_mech_to_use_gtype =
                                            safe_c2rust_find_mech_by_name(auth, mech_name);
                                        if auth_mech_to_use_gtype
                                            == 0 as ::core::ffi::c_int as GType
                                            || allow_anonymous == 0
                                                && g_strcmp0(
                                                    mech_name as *const ::core::ffi::c_char,
                                                    b"ANONYMOUS\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                ) == 0 as ::core::ffi::c_int
                                        {
                                            g_strfreev(tokens);
                                            s = safe_c2rust_get_auth_mechanisms(
                                                auth,
                                                allow_anonymous,
                                                b"REJECTED \0" as *const u8 as *const gchar,
                                                b"\r\n\0" as *const u8 as *const gchar,
                                                b" \0" as *const u8 as *const gchar,
                                            );
                                            safe_c2rust_debug_print(
                                                b"SERVER: writing '%s'\0" as *const u8
                                                    as *const gchar,
                                                s,
                                            );
                                            if g_data_output_stream_put_string(
                                                dos,
                                                s,
                                                cancellable,
                                                error,
                                            ) == 0
                                            {
                                                g_free(s as gpointer);
                                                current_block = 16086139763999343087;
                                                break;
                                            } else {
                                                g_free(s as gpointer);
                                                state = SERVER_STATE_WAITING_FOR_AUTH;
                                                continue;
                                            }
                                        } else {
                                            initial_response = ::core::ptr::null_mut::<gchar>();
                                            initial_response_len = 0;
                                            let mut _pp: *mut *mut GDBusAuthMechanism =
                                                &raw mut mech;
                                            let mut _ptr: *mut GDBusAuthMechanism = *_pp;
                                            *_pp = ::core::ptr::null_mut::<GDBusAuthMechanism>();
                                            if !_ptr.is_null() {
                                                g_object_unref(_ptr as gpointer);
                                            }
                                            mech = g_object_new(
                                                auth_mech_to_use_gtype,
                                                b"stream\0" as *const u8 as *const gchar,
                                                (*(*auth).priv_0).stream,
                                                b"credentials\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                credentials,
                                                NULL_0,
                                            )
                                                as *mut GDBusAuthMechanism;
                                            initial_response = ::core::ptr::null_mut::<gchar>();
                                            initial_response_len = 0 as gsize;
                                            if !encoded.is_null() {
                                                initial_response = safe_c2rust_hexdecode(
                                                    encoded,
                                                    &raw mut initial_response_len,
                                                    error,
                                                );
                                                if initial_response.is_null() {
                                                    g_prefix_error(
                                                        error,
                                                        b"Initial response is malformed: \0"
                                                            as *const u8
                                                            as *const gchar,
                                                    );
                                                    g_strfreev(tokens);
                                                    current_block = 16086139763999343087;
                                                    break;
                                                }
                                            }
                                            _g_dbus_auth_mechanism_server_initiate(
                                                mech,
                                                initial_response,
                                                initial_response_len,
                                            );
                                            g_free(initial_response as gpointer);
                                            g_strfreev(tokens);
                                        }
                                    } else {
                                        g_set_error(
                                            error,
                                            g_io_error_quark(),
                                            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                            b"Unexpected line '%s' while in WaitingForAuth state\0"
                                                as *const u8
                                                as *const gchar,
                                            line,
                                        );
                                        g_free(line as gpointer);
                                        current_block = 16086139763999343087;
                                        break;
                                    }
                                }
                                1 => {
                                    safe_c2rust_debug_print(
                                        b"SERVER: WaitingForData\0" as *const u8 as *const gchar,
                                    );
                                    line = safe_c2rust__my_g_input_stream_read_line_safe(
                                        g_io_stream_get_input_stream((*(*auth).priv_0).stream),
                                        &raw mut line_length,
                                        cancellable,
                                        error,
                                    );
                                    safe_c2rust_debug_print(
                                        b"SERVER: WaitingForData, read '%s'\0" as *const u8
                                            as *const gchar,
                                        line,
                                    );
                                    if line.is_null() {
                                        current_block = 16086139763999343087;
                                        break;
                                    }
                                    if strcmp(
                                        line as *const ::core::ffi::c_char,
                                        b"DATA\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                        || (if 0 != 0 {
                                            ({
                                                let __str: *const ::core::ffi::c_char = line;
                                                let __prefix: *const ::core::ffi::c_char =
                                                    b"DATA \0" as *const u8
                                                        as *const ::core::ffi::c_char;
                                                let mut __result: gboolean = FALSE;
                                                if ({
                                                    let mut _g_boolean_var_34: ::core::ffi::c_int =
                                                        0;
                                                    if __str.is_null() || __prefix.is_null() {
                                                        _g_boolean_var_34 = 1 as ::core::ffi::c_int;
                                                    } else {
                                                        _g_boolean_var_34 = 0 as ::core::ffi::c_int;
                                                    }
                                                    _g_boolean_var_34
                                                })
                                                    as ::core::ffi::c_long
                                                    != 0
                                                {
                                                    __result = g_str_has_prefix(
                                                        __str as *const gchar,
                                                        __prefix as *const gchar,
                                                    );
                                                } else {
                                                    let __str_len: size_t =
                                                        strlen(__str.offset(__str.is_null()
                                                            as ::core::ffi::c_int
                                                            as isize))
                                                            as size_t;
                                                    let __prefix_len: size_t =
                                                        strlen(__prefix.offset(__prefix.is_null()
                                                            as ::core::ffi::c_int
                                                            as isize))
                                                            as size_t;
                                                    if __str_len >= __prefix_len {
                                                        __result = (memcmp(
                                                            __str.offset(__str.is_null()
                                                                as ::core::ffi::c_int
                                                                as isize)
                                                                as *const ::core::ffi::c_void,
                                                            __prefix.offset(__prefix.is_null()
                                                                as ::core::ffi::c_int
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
                                            g_str_has_prefix(
                                                line,
                                                b"DATA \0" as *const u8 as *const gchar,
                                            )
                                        }) != 0
                                    {
                                        let mut encoded_0: *mut gchar =
                                            ::core::ptr::null_mut::<gchar>();
                                        let mut decoded_data: *mut gchar =
                                            ::core::ptr::null_mut::<gchar>();
                                        let mut decoded_data_len: gsize = 0 as gsize;
                                        encoded_0 = safe_c2rust_g_strdup_inline(
                                            line.offset(4 as ::core::ffi::c_int as isize),
                                        )
                                            as *mut gchar;
                                        g_free(line as gpointer);
                                        g_strchomp(g_strchug(encoded_0));
                                        decoded_data = safe_c2rust_hexdecode(
                                            encoded_0,
                                            &raw mut decoded_data_len,
                                            error,
                                        );
                                        g_free(encoded_0 as gpointer);
                                        if decoded_data.is_null() {
                                            g_prefix_error(
                                                error,
                                                b"DATA response is malformed: \0" as *const u8
                                                    as *const gchar,
                                            );
                                            current_block = 16086139763999343087;
                                            break;
                                        } else {
                                            _g_dbus_auth_mechanism_server_data_receive(
                                                mech,
                                                decoded_data,
                                                decoded_data_len,
                                            );
                                            g_free(decoded_data as gpointer);
                                        }
                                    } else {
                                        g_set_error(
                                            error,
                                            g_io_error_quark(),
                                            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                            b"Unexpected line '%s' while in WaitingForData state\0"
                                                as *const u8
                                                as *const gchar,
                                            line,
                                        );
                                        g_free(line as gpointer);
                                        current_block = 16086139763999343087;
                                        break;
                                    }
                                }
                                2 => {
                                    safe_c2rust_debug_print(
                                        b"SERVER: WaitingForBegin\0" as *const u8 as *const gchar,
                                    );
                                    line = safe_c2rust__my_g_input_stream_read_line_safe(
                                        g_io_stream_get_input_stream((*(*auth).priv_0).stream),
                                        &raw mut line_length,
                                        cancellable,
                                        error,
                                    );
                                    if line.is_null() {
                                        current_block = 16086139763999343087;
                                        break;
                                    }
                                    safe_c2rust_debug_print(
                                        b"SERVER: WaitingForBegin, read '%s'\0" as *const u8
                                            as *const gchar,
                                        line,
                                    );
                                    if g_strcmp0(
                                        line,
                                        b"BEGIN\0" as *const u8 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        ret = TRUE as gboolean;
                                        g_free(line as gpointer);
                                        current_block = 16086139763999343087;
                                        break;
                                    } else if g_strcmp0(
                                        line,
                                        b"NEGOTIATE_UNIX_FD\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        g_free(line as gpointer);
                                        if offered_capabilities as ::core::ffi::c_uint
                                            & G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING
                                                as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                            != 0
                                        {
                                            negotiated_capabilities = ::core::mem::transmute::<
                                                ::core::ffi::c_uint,
                                                GDBusCapabilityFlags,
                                            >(
                                                negotiated_capabilities as ::core::ffi::c_uint
                                                    | G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING
                                                        as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint,
                                            );
                                            s = b"AGREE_UNIX_FD\r\n\0" as *const u8
                                                as *const ::core::ffi::c_char
                                                as *mut gchar;
                                            safe_c2rust_debug_print(
                                                b"SERVER: writing '%s'\0" as *const u8
                                                    as *const gchar,
                                                s,
                                            );
                                            if g_data_output_stream_put_string(
                                                dos,
                                                s,
                                                cancellable,
                                                error,
                                            ) == 0
                                            {
                                                current_block = 16086139763999343087;
                                                break;
                                            } else {
                                                continue;
                                            }
                                        } else {
                                            s = b"ERROR \"fd passing not offered\"\r\n\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char
                                                as *mut gchar;
                                            safe_c2rust_debug_print(
                                                b"SERVER: writing '%s'\0" as *const u8
                                                    as *const gchar,
                                                s,
                                            );
                                            if g_data_output_stream_put_string(
                                                dos,
                                                s,
                                                cancellable,
                                                error,
                                            ) == 0
                                            {
                                                current_block = 16086139763999343087;
                                                break;
                                            } else {
                                                continue;
                                            }
                                        }
                                    } else {
                                        g_log(
                                            G_LOG_DOMAIN.as_ptr() as *const gchar,
                                            G_LOG_LEVEL_DEBUG,
                                            b"Unexpected line '%s' while in WaitingForBegin state\0"
                                                as *const u8
                                                as *const gchar,
                                            line,
                                        );
                                        g_free(line as gpointer);
                                        s = b"ERROR \"Unknown Command\"\r\n\0" as *const u8
                                            as *const ::core::ffi::c_char
                                            as *mut gchar;
                                        safe_c2rust_debug_print(
                                            b"SERVER: writing '%s'\0" as *const u8 as *const gchar,
                                            s,
                                        );
                                        if g_data_output_stream_put_string(
                                            dos,
                                            s,
                                            cancellable,
                                            error,
                                        ) == 0
                                        {
                                            current_block = 16086139763999343087;
                                            break;
                                        } else {
                                            continue;
                                        }
                                    }
                                }
                                _ => {
                                    g_assertion_message_expr(
                                        G_LOG_DOMAIN.as_ptr(),
                                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauth.c\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        1381 as ::core::ffi::c_int,
                                        G_STRFUNC,
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                    );
                                }
                            }
                            loop {
                                match _g_dbus_auth_mechanism_server_get_state(mech)
                                    as ::core::ffi::c_uint
                                {
                                    4 => {
                                        if require_same_user != 0
                                            && (credentials.is_null()
                                                || g_credentials_is_same_user(
                                                    credentials,
                                                    own_credentials,
                                                    ::core::ptr::null_mut::<*mut GError>(),
                                                ) == 0)
                                        {
                                            current_block = 3879520548144599102;
                                            break;
                                        } else {
                                            current_block = 18383263831861166299;
                                            break;
                                        }
                                    }
                                    3 => {
                                        s = safe_c2rust_get_auth_mechanisms(
                                            auth,
                                            allow_anonymous,
                                            b"REJECTED \0" as *const u8 as *const gchar,
                                            b"\r\n\0" as *const u8 as *const gchar,
                                            b" \0" as *const u8 as *const gchar,
                                        );
                                        safe_c2rust_debug_print(
                                            b"SERVER: writing '%s'\0" as *const u8 as *const gchar,
                                            s,
                                        );
                                        if g_data_output_stream_put_string(
                                            dos,
                                            s,
                                            cancellable,
                                            error,
                                        ) == 0
                                        {
                                            current_block = 18137396335907573669;
                                            break;
                                        } else {
                                            current_block = 18425699056680496821;
                                            break;
                                        }
                                    }
                                    1 => {
                                        state = SERVER_STATE_WAITING_FOR_DATA;
                                        continue 's_193;
                                    }
                                    2 => {
                                        let mut data: *mut gchar = ::core::ptr::null_mut::<gchar>();
                                        let mut data_len: gsize = 0;
                                        data = _g_dbus_auth_mechanism_server_data_send(
                                            mech,
                                            &raw mut data_len,
                                        );
                                        if data.is_null() {
                                            continue;
                                        }
                                        if data_len == 0 as gsize {
                                            s = safe_c2rust_g_strdup_inline(
                                                b"DATA\r\n\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            )
                                                as *mut gchar;
                                        } else {
                                            let mut encoded_data: *mut gchar =
                                                _g_dbus_hexencode(data, data_len);
                                            s = g_strdup_printf(
                                                b"DATA %s\r\n\0" as *const u8 as *const gchar,
                                                encoded_data,
                                            );
                                            g_free(encoded_data as gpointer);
                                        }
                                        g_free(data as gpointer);
                                        safe_c2rust_debug_print(
                                            b"SERVER: writing '%s'\0" as *const u8 as *const gchar,
                                            s,
                                        );
                                        if g_data_output_stream_put_string(
                                            dos,
                                            s,
                                            cancellable,
                                            error,
                                        ) == 0
                                        {
                                            g_free(s as gpointer);
                                            current_block = 16086139763999343087;
                                            break 's_193;
                                        } else {
                                            g_free(s as gpointer);
                                        }
                                    }
                                    _ => {
                                        g_assertion_message_expr(
                                            G_LOG_DOMAIN.as_ptr(),
                                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauth.c\0"
                                                as *const u8 as *const ::core::ffi::c_char,
                                            1275 as ::core::ffi::c_int,
                                            G_STRFUNC,
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                        );
                                    }
                                }
                            }
                            match current_block {
                                18137396335907573669 => {
                                    g_free(s as gpointer);
                                    current_block = 16086139763999343087;
                                    break;
                                }
                                18383263831861166299 => {
                                    if !observer.is_null()
                                        && g_dbus_auth_observer_authorize_authenticated_peer(
                                            observer,
                                            (*(*auth).priv_0).stream,
                                            credentials,
                                        ) == 0
                                    {
                                        g_set_error_literal(
                                            error,
                                            g_io_error_quark(),
                                            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                            glib_gettext(
                                                b"Cancelled via GDBusAuthObserver::authorize-authenticated-peer\0"
                                                    as *const u8 as *const gchar,
                                            ),
                                        );
                                        current_block = 16086139763999343087;
                                        break;
                                    } else {
                                        s = g_strdup_printf(
                                            b"OK %s\r\n\0" as *const u8 as *const gchar,
                                            guid,
                                        );
                                        safe_c2rust_debug_print(
                                            b"SERVER: writing '%s'\0" as *const u8 as *const gchar,
                                            s,
                                        );
                                        if g_data_output_stream_put_string(
                                            dos,
                                            s,
                                            cancellable,
                                            error,
                                        ) == 0
                                        {
                                            g_free(s as gpointer);
                                            current_block = 16086139763999343087;
                                            break;
                                        } else {
                                            g_free(s as gpointer);
                                            state = SERVER_STATE_WAITING_FOR_BEGIN;
                                        }
                                    }
                                }
                                3879520548144599102 => {
                                    g_set_error_literal(
                                        error,
                                        g_io_error_quark(),
                                        G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                        glib_gettext(
                                            b"User IDs must be the same for peer and server\0"
                                                as *const u8
                                                as *const gchar,
                                        ),
                                    );
                                    current_block = 16086139763999343087;
                                    break;
                                }
                                _ => {
                                    g_free(s as gpointer);
                                    state = SERVER_STATE_WAITING_FOR_AUTH;
                                }
                            }
                        }
                        match current_block {
                            16086139763999343087 => {}
                            _ => {
                                g_set_error_literal(
                                    error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                    b"Not implemented (server)\0" as *const u8 as *const gchar,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    let mut _pp_0: *mut *mut GDBusAuthMechanism = &raw mut mech;
    let mut _ptr_0: *mut GDBusAuthMechanism = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GDBusAuthMechanism>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut GDataOutputStream = &raw mut dos;
    let mut _ptr_1: *mut GDataOutputStream = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GDataOutputStream>();
    if !_ptr_1.is_null() {
        g_object_unref(_ptr_1 as gpointer);
    }
    let mut _pp_2: *mut *mut GCredentials = &raw mut own_credentials;
    let mut _ptr_2: *mut GCredentials = *_pp_2;
    *_pp_2 = ::core::ptr::null_mut::<GCredentials>();
    if !_ptr_2.is_null() {
        g_object_unref(_ptr_2 as gpointer);
    }
    if !error.is_null() && !(*error).is_null() {
        ret = FALSE as gboolean;
    }
    if ret != 0 {
        if !out_negotiated_capabilities.is_null() {
            *out_negotiated_capabilities = negotiated_capabilities;
        }
        if !out_received_credentials.is_null() {
            *out_received_credentials = (if !credentials.is_null() {
                g_object_ref(credentials as gpointer) as *mut GCredentials
            } else {
                ::core::ptr::null_mut::<GCredentials>()
            }) as *mut GCredentials;
        }
    }
    if !credentials.is_null() {
        g_object_unref(credentials as gpointer);
    }
    safe_c2rust_debug_print(
        b"SERVER: Done, authenticated=%d\0" as *const u8 as *const gchar,
        ret,
    );
    return ret;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
