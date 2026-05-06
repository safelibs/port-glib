extern "C" {
    pub type _GHashTable;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
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
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_error_new(domain: GQuark, code: gint, format: *const gchar, ...) -> *mut GError;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_int_hash(v: gconstpointer) -> guint;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
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
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_io_error_quark() -> GQuark;
    fn _g_dbus_initialize();
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
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GHashTable = _GHashTable;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_0 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_0 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_0 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_0 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_0 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_0 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_0 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_0 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_0 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_0 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_0 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_0 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_0 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_0 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_0 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_0 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_0 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_0 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_0 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_0 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_0 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_0 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_0 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_0 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_0 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_0 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_0 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_0 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_0 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_0 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_0 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_0 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_0 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_0 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_0 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_0 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_0 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_0 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_0 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_0 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_0 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_0 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_0 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_0 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_0 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_0 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_0 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_0 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_0 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_0 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusErrorEntry {
    pub error_code: gint,
    pub dbus_error_name: *const gchar,
}
pub type GDBusErrorEntry = _GDBusErrorEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RegisteredError {
    pub pair: QuarkCodePair,
    pub dbus_error_name: *mut gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QuarkCodePair {
    pub error_domain: GQuark,
    pub error_code: gint,
}
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
static mut safe_c2rust_g_dbus_error_entries: [GDBusErrorEntry; 45] = [
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_FAILED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Failed\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_NO_MEMORY as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.NoMemory\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SERVICE_UNKNOWN as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.ServiceUnknown\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_NAME_HAS_NO_OWNER as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.NameHasNoOwner\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_NO_REPLY as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.NoReply\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_IO_ERROR as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.IOError\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_BAD_ADDRESS as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.BadAddress\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.NotSupported\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_LIMITS_EXCEEDED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.LimitsExceeded\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_ACCESS_DENIED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.AccessDenied\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_AUTH_FAILED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.AuthFailed\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_NO_SERVER as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.NoServer\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_TIMEOUT as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Timeout\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_NO_NETWORK as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.NoNetwork\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_ADDRESS_IN_USE as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.AddressInUse\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_DISCONNECTED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Disconnected\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_INVALID_ARGS as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.InvalidArgs\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_FILE_NOT_FOUND as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.FileNotFound\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_FILE_EXISTS as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.FileExists\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_UNKNOWN_METHOD as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.UnknownMethod\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_TIMED_OUT as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.TimedOut\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_MATCH_RULE_NOT_FOUND as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.MatchRuleNotFound\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_MATCH_RULE_INVALID as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.MatchRuleInvalid\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_EXEC_FAILED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.ExecFailed\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_FORK_FAILED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.ForkFailed\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_CHILD_EXITED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.ChildExited\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_CHILD_SIGNALED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.ChildSignaled\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_FAILED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.Failed\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_SETUP_FAILED as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.FailedToSetup\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_CONFIG_INVALID as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.ConfigInvalid\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_SERVICE_INVALID as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.ServiceNotValid\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.ServiceNotFound\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.PermissionsInvalid\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_FILE_INVALID as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.FileInvalid\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SPAWN_NO_MEMORY as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.Spawn.NoMemory\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.UnixProcessIdUnknown\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_INVALID_SIGNATURE as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.InvalidSignature\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_INVALID_FILE_CONTENT as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.InvalidFileContent\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.SELinuxSecurityContextUnknown\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.AdtAuditDataUnknown\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_OBJECT_PATH_IN_USE as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.ObjectPathInUse\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_UNKNOWN_OBJECT as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.UnknownObject\0" as *const u8 as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_UNKNOWN_INTERFACE as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.UnknownInterface\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_UNKNOWN_PROPERTY as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.UnknownProperty\0" as *const u8
            as *const gchar,
    },
    _GDBusErrorEntry {
        error_code: G_DBUS_ERROR_PROPERTY_READ_ONLY as ::core::ffi::c_int as gint,
        dbus_error_name: b"org.freedesktop.DBus.Error.PropertyReadOnly\0" as *const u8
            as *const gchar,
    },
];
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_quark() -> GQuark {
    static mut safe_c2rust_quark: gsize = 0 as gsize;
    safe_c2rust_g_dbus_error_register_error_domain(
        b"g-dbus-error-quark\0" as *const u8 as *const gchar,
        &raw mut safe_c2rust_quark as *mut gsize,
        &raw const safe_c2rust_g_dbus_error_entries as *const GDBusErrorEntry,
        (::core::mem::size_of::<[GDBusErrorEntry; 45]>() as usize)
            .wrapping_div(::core::mem::size_of::<GDBusErrorEntry>() as usize) as guint,
    );
    return safe_c2rust_quark as GQuark;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_register_error_domain(
    mut error_domain_quark_name: *const gchar,
    mut quark_volatile: *mut gsize,
    mut entries: *const GDBusErrorEntry,
    mut num_entries: guint,
) {
    let mut quark: *mut gsize = ::core::ptr::null_mut::<gsize>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !error_domain_quark_name.is_null() {
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
            b"error_domain_quark_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !quark_volatile.is_null() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"quark_volatile != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !entries.is_null() {
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
            b"entries != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if num_entries > 0 as guint {
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
            b"num_entries > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    quark = quark_volatile as *mut gsize;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            *quark;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = quark as *mut gsize;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(quark as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        let mut n: guint = 0;
        let mut new_quark: GQuark = 0;
        new_quark = g_quark_from_static_string(error_domain_quark_name);
        n = 0 as guint;
        while n < num_entries {
            if !(({
                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                if safe_c2rust_g_dbus_error_register_error(
                    new_quark,
                    (*entries.offset(n as isize)).error_code,
                    (*entries.offset(n as isize)).dbus_error_name,
                ) != 0
                {
                    _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_14
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    139 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_dbus_error_register_error (new_quark, entries[n].error_code, entries[n].dbus_error_name)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            n = n.wrapping_add(1);
        }
        if 0 as ::core::ffi::c_int != 0 {
            *quark = new_quark as gsize;
        } else {
        };
        g_once_init_leave(quark as *mut ::core::ffi::c_void, new_quark as gsize);
    }
}
unsafe extern "C" fn safe_c2rust__g_dbus_error_decode_gerror(
    mut dbus_name: *const gchar,
    mut out_error_domain: *mut GQuark,
    mut out_error_code: *mut gint,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut n: guint = 0;
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut domain_quark_string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    ret = FALSE as gboolean;
    s = ::core::ptr::null_mut::<GString>();
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = dbus_name as *const ::core::ffi::c_char;
            let __prefix: *const ::core::ffi::c_char = b"org.gtk.GDBus.UnmappedGError.Quark._\0"
                as *const u8
                as *const ::core::ffi::c_char;
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
            dbus_name,
            b"org.gtk.GDBus.UnmappedGError.Quark._\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        s = g_string_new(::core::ptr::null::<gchar>());
        n = (::core::mem::size_of::<[::core::ffi::c_char; 37]>() as usize).wrapping_sub(1 as usize)
            as guint;
        loop {
            if !(*dbus_name.offset(n as isize) as ::core::ffi::c_int != '.' as i32
                && *dbus_name.offset(n as isize) as ::core::ffi::c_int != '\0' as i32)
            {
                current_block = 15089075282327824602;
                break;
            }
            if *safe_c2rust_g_ascii_table.offset(*dbus_name.offset(n as isize) as guchar as isize)
                as ::core::ffi::c_int
                & G_ASCII_ALNUM as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                safe_c2rust_g_string_append_c_inline(s, *dbus_name.offset(n as isize));
            } else {
                if !(*dbus_name.offset(n as isize) as ::core::ffi::c_int == '_' as i32) {
                    current_block = 16658539534146690776;
                    break;
                }
                let mut nibble_top: guint = 0;
                let mut nibble_bottom: guint = 0;
                n = n.wrapping_add(1);
                nibble_top = *dbus_name.offset(n as isize) as guint;
                if nibble_top >= '0' as i32 as guint && nibble_top <= '9' as i32 as guint {
                    nibble_top = nibble_top.wrapping_sub('0' as i32 as guint);
                } else {
                    if !(nibble_top >= 'a' as i32 as guint && nibble_top <= 'f' as i32 as guint) {
                        current_block = 16658539534146690776;
                        break;
                    }
                    nibble_top =
                        nibble_top.wrapping_sub(('a' as i32 - 10 as ::core::ffi::c_int) as guint);
                }
                n = n.wrapping_add(1);
                nibble_bottom = *dbus_name.offset(n as isize) as guint;
                if nibble_bottom >= '0' as i32 as guint && nibble_bottom <= '9' as i32 as guint {
                    nibble_bottom = nibble_bottom.wrapping_sub('0' as i32 as guint);
                } else {
                    if !(nibble_bottom >= 'a' as i32 as guint
                        && nibble_bottom <= 'f' as i32 as guint)
                    {
                        current_block = 16658539534146690776;
                        break;
                    }
                    nibble_bottom = nibble_bottom
                        .wrapping_sub(('a' as i32 - 10 as ::core::ffi::c_int) as guint);
                }
                safe_c2rust_g_string_append_c_inline(
                    s,
                    (nibble_top << 4 as ::core::ffi::c_int | nibble_bottom) as gchar,
                );
            }
            n = n.wrapping_add(1);
        }
        match current_block {
            16658539534146690776 => {}
            _ => {
                if !(if 0 != 0 {
                    ({
                        let __str: *const ::core::ffi::c_char = dbus_name.offset(n as isize);
                        let __prefix: *const ::core::ffi::c_char =
                            b".Code\0" as *const u8 as *const ::core::ffi::c_char;
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
                    g_str_has_prefix(
                        dbus_name.offset(n as isize),
                        b".Code\0" as *const u8 as *const gchar,
                    )
                } == 0)
                {
                    domain_quark_string = if 0 != 0 {
                        if 0 as ::core::ffi::c_int != 0 {
                            g_string_free(s, 0 as gboolean)
                        } else {
                            g_string_free_and_steal(s)
                        }
                    } else {
                        g_string_free(s, 0 as gboolean)
                    };
                    s = ::core::ptr::null_mut::<GString>();
                    if !out_error_domain.is_null() {
                        *out_error_domain = g_quark_from_string(domain_quark_string);
                    }
                    g_free(domain_quark_string as gpointer);
                    if !out_error_code.is_null() {
                        *out_error_code = safe_c2rust_atoi(
                            dbus_name
                                .offset(n as isize)
                                .offset(::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize
                                    as isize)
                                .offset(-(1 as ::core::ffi::c_int as isize)),
                        ) as gint;
                    }
                    ret = TRUE as gboolean;
                }
            }
        }
    }
    if !s.is_null() {
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
unsafe extern "C" fn safe_c2rust_quark_code_pair_hash_func(
    mut pair: *const QuarkCodePair,
) -> guint {
    let mut val: gint = 0;
    val = (*pair)
        .error_domain
        .wrapping_add((*pair).error_code as GQuark) as gint;
    return g_int_hash(&raw mut val as gconstpointer);
}
unsafe extern "C" fn safe_c2rust_quark_code_pair_equal_func(
    mut a: *const QuarkCodePair,
    mut b: *const QuarkCodePair,
) -> gboolean {
    return ((*a).error_domain == (*b).error_domain && (*a).error_code == (*b).error_code)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_registered_error_free(mut re: *mut RegisteredError) {
    g_free((*re).dbus_error_name as gpointer);
    g_free(re as gpointer);
}
static mut safe_c2rust_g__error_lock_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_quark_code_pair_to_re: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_dbus_error_name_to_re: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_register_error(
    mut error_domain: GQuark,
    mut error_code: gint,
    mut dbus_error_name: *const gchar,
) -> gboolean {
    let mut ret: gboolean = 0;
    let mut pair: QuarkCodePair = QuarkCodePair {
        error_domain: 0,
        error_code: 0,
    };
    let mut re: *mut RegisteredError = ::core::ptr::null_mut::<RegisteredError>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !dbus_error_name.is_null() {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"dbus_error_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    g_mutex_lock(&raw mut safe_c2rust_g__error_lock_lock);
    if safe_c2rust_quark_code_pair_to_re.is_null() {
        if ({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if safe_c2rust_dbus_error_name_to_re.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                305 as ::core::ffi::c_int,
                G_STRFUNC,
                b"dbus_error_name_to_re == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        safe_c2rust_quark_code_pair_to_re = g_hash_table_new(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*const QuarkCodePair) -> guint>,
                GHashFunc,
            >(Some(
                safe_c2rust_quark_code_pair_hash_func
                    as unsafe extern "C" fn(*const QuarkCodePair) -> guint,
            )),
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*const QuarkCodePair, *const QuarkCodePair) -> gboolean,
                >,
                GEqualFunc,
            >(Some(
                safe_c2rust_quark_code_pair_equal_func
                    as unsafe extern "C" fn(*const QuarkCodePair, *const QuarkCodePair) -> gboolean,
            )),
        );
        safe_c2rust_dbus_error_name_to_re = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            None,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut RegisteredError) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_registered_error_free
                    as unsafe extern "C" fn(*mut RegisteredError) -> (),
            )),
        );
    }
    if g_hash_table_lookup(
        safe_c2rust_dbus_error_name_to_re,
        dbus_error_name as gconstpointer,
    )
    .is_null()
    {
        pair.error_domain = error_domain;
        pair.error_code = error_code;
        if g_hash_table_lookup(
            safe_c2rust_quark_code_pair_to_re,
            &raw mut pair as gconstpointer,
        )
        .is_null()
        {
            re = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<RegisteredError>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut RegisteredError;
            (*re).pair = pair;
            (*re).dbus_error_name =
                safe_c2rust_g_strdup_inline(dbus_error_name as *const ::core::ffi::c_char)
                    as *mut gchar;
            g_hash_table_insert(
                safe_c2rust_quark_code_pair_to_re,
                &raw mut (*re).pair as gpointer,
                re as gpointer,
            );
            g_hash_table_insert(
                safe_c2rust_dbus_error_name_to_re,
                (*re).dbus_error_name as gpointer,
                re as gpointer,
            );
            ret = TRUE as gboolean;
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__error_lock_lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_unregister_error(
    mut error_domain: GQuark,
    mut error_code: gint,
    mut dbus_error_name: *const gchar,
) -> gboolean {
    let mut ret: gboolean = 0;
    let mut re: *mut RegisteredError = ::core::ptr::null_mut::<RegisteredError>();
    let mut hash_size: guint = 0;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !dbus_error_name.is_null() {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"dbus_error_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    g_mutex_lock(&raw mut safe_c2rust_g__error_lock_lock);
    if safe_c2rust_dbus_error_name_to_re.is_null() {
        if ({
            let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
            if safe_c2rust_quark_code_pair_to_re.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                366 as ::core::ffi::c_int,
                G_STRFUNC,
                b"quark_code_pair_to_re == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        re = g_hash_table_lookup(
            safe_c2rust_dbus_error_name_to_re,
            dbus_error_name as gconstpointer,
        ) as *mut RegisteredError;
        if re.is_null() {
            let mut pair: QuarkCodePair = QuarkCodePair {
                error_domain: 0,
                error_code: 0,
            };
            pair.error_domain = error_domain;
            pair.error_code = error_code;
            if !(({
                let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                if g_hash_table_lookup(
                    safe_c2rust_quark_code_pair_to_re,
                    &raw mut pair as gconstpointer,
                )
                .is_null()
                {
                    _g_boolean_var_21 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_21 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_21
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    376 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_hash_table_lookup (quark_code_pair_to_re, &pair) == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        } else {
            ret = TRUE as gboolean;
            if !(({
                let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                if g_hash_table_lookup(
                    safe_c2rust_quark_code_pair_to_re,
                    &raw mut (*re).pair as gconstpointer,
                ) == re as gpointer
                {
                    _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_22
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    382 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_hash_table_lookup (quark_code_pair_to_re, &(re->pair)) == re\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if !(({
                let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                if g_hash_table_remove(
                    safe_c2rust_quark_code_pair_to_re,
                    &raw mut (*re).pair as gconstpointer,
                ) != 0
                {
                    _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_23
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    384 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_hash_table_remove (quark_code_pair_to_re, &(re->pair))\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if !(({
                let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                if g_hash_table_remove(
                    safe_c2rust_dbus_error_name_to_re,
                    (*re).dbus_error_name as gconstpointer,
                ) != 0
                {
                    _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_24
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    385 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_hash_table_remove (dbus_error_name_to_re, re->dbus_error_name)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            hash_size = g_hash_table_size(safe_c2rust_dbus_error_name_to_re);
            if hash_size == 0 as guint {
                if !(({
                    let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                    if g_hash_table_size(safe_c2rust_quark_code_pair_to_re) == 0 as guint {
                        _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_25
                }) as ::core::ffi::c_long
                    != 0)
                {
                    g_warn_message(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        391 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"g_hash_table_size (quark_code_pair_to_re) == 0\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                g_hash_table_unref(safe_c2rust_dbus_error_name_to_re);
                safe_c2rust_dbus_error_name_to_re = ::core::ptr::null_mut::<GHashTable>();
                g_hash_table_unref(safe_c2rust_quark_code_pair_to_re);
                safe_c2rust_quark_code_pair_to_re = ::core::ptr::null_mut::<GHashTable>();
            } else if !(({
                let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
                if g_hash_table_size(safe_c2rust_quark_code_pair_to_re) == hash_size {
                    _g_boolean_var_26 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_26 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_26
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    400 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_hash_table_size (quark_code_pair_to_re) == hash_size\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__error_lock_lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_is_remote_error(
    mut error: *const GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = (*error).message;
            let __prefix: *const ::core::ffi::c_char =
                b"GDBus.Error:\0" as *const u8 as *const ::core::ffi::c_char;
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
            (*error).message,
            b"GDBus.Error:\0" as *const u8 as *const gchar,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_get_remote_error(
    mut error: *const GError,
) -> *mut gchar {
    let mut re: *mut RegisteredError = ::core::ptr::null_mut::<RegisteredError>();
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    _g_dbus_initialize();
    ret = ::core::ptr::null_mut::<gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__error_lock_lock);
    re = ::core::ptr::null_mut::<RegisteredError>();
    if !safe_c2rust_quark_code_pair_to_re.is_null() {
        let mut pair: QuarkCodePair = QuarkCodePair {
            error_domain: 0,
            error_code: 0,
        };
        pair.error_domain = (*error).domain;
        pair.error_code = (*error).code;
        if ({
            let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
            if !safe_c2rust_dbus_error_name_to_re.is_null() {
                _g_boolean_var_30 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_30 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_30
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                467 as ::core::ffi::c_int,
                G_STRFUNC,
                b"dbus_error_name_to_re != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        re = g_hash_table_lookup(
            safe_c2rust_quark_code_pair_to_re,
            &raw mut pair as gconstpointer,
        ) as *mut RegisteredError;
    }
    if !re.is_null() {
        ret = safe_c2rust_g_strdup_inline((*re).dbus_error_name) as *mut gchar;
    } else if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = (*error).message;
            let __prefix: *const ::core::ffi::c_char =
                b"GDBus.Error:\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_31 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_31 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_31
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
            (*error).message,
            b"GDBus.Error:\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        let mut begin: *const gchar = ::core::ptr::null::<gchar>();
        let mut end: *const gchar = ::core::ptr::null::<gchar>();
        begin = (*error)
            .message
            .offset(::core::mem::size_of::<[::core::ffi::c_char; 13]>() as usize as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        end = strstr(
            begin as *const ::core::ffi::c_char,
            b":\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !end.is_null()
            && *end.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
        {
            ret = g_strndup(
                begin,
                end.offset_from(begin) as ::core::ffi::c_long as gsize,
            );
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__error_lock_lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_new_for_dbus_error(
    mut dbus_error_name: *const gchar,
    mut dbus_error_message: *const gchar,
) -> *mut GError {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut re: *mut RegisteredError = ::core::ptr::null_mut::<RegisteredError>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !dbus_error_name.is_null() {
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
            b"dbus_error_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GError>();
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !dbus_error_message.is_null() {
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
            b"dbus_error_message != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GError>();
    }
    _g_dbus_initialize();
    g_mutex_lock(&raw mut safe_c2rust_g__error_lock_lock);
    re = ::core::ptr::null_mut::<RegisteredError>();
    if !safe_c2rust_dbus_error_name_to_re.is_null() {
        if ({
            let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
            if !safe_c2rust_quark_code_pair_to_re.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                551 as ::core::ffi::c_int,
                G_STRFUNC,
                b"quark_code_pair_to_re != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        re = g_hash_table_lookup(
            safe_c2rust_dbus_error_name_to_re,
            dbus_error_name as gconstpointer,
        ) as *mut RegisteredError;
    }
    if !re.is_null() {
        error = g_error_new(
            (*re).pair.error_domain,
            (*re).pair.error_code,
            b"GDBus.Error:%s: %s\0" as *const u8 as *const gchar,
            dbus_error_name,
            dbus_error_message,
        );
    } else {
        let mut error_domain: GQuark = 0 as GQuark;
        let mut error_code: gint = 0 as gint;
        if safe_c2rust__g_dbus_error_decode_gerror(
            dbus_error_name,
            &raw mut error_domain,
            &raw mut error_code,
        ) != 0
        {
            error = g_error_new(
                error_domain,
                error_code,
                b"GDBus.Error:%s: %s\0" as *const u8 as *const gchar,
                dbus_error_name,
                dbus_error_message,
            );
        } else {
            error = g_error_new(
                g_io_error_quark(),
                G_IO_ERROR_DBUS_ERROR as ::core::ffi::c_int as gint,
                b"GDBus.Error:%s: %s\0" as *const u8 as *const gchar,
                dbus_error_name,
                dbus_error_message,
            );
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__error_lock_lock);
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_set_dbus_error(
    mut error: *mut *mut GError,
    mut dbus_error_name: *const gchar,
    mut dbus_error_message: *const gchar,
    mut format: *const gchar,
    mut args: ...
) {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !dbus_error_name.is_null() {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"dbus_error_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !dbus_error_message.is_null() {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"dbus_error_message != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if error.is_null() {
        return;
    }
    if format.is_null() {
        *error = safe_c2rust_g_dbus_error_new_for_dbus_error(dbus_error_name, dbus_error_message);
    } else {
        let mut var_args: ::core::ffi::VaList;
        var_args = args.clone();
        safe_c2rust_g_dbus_error_set_dbus_error_valist(
            error,
            dbus_error_name,
            dbus_error_message,
            format,
            var_args,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_set_dbus_error_valist(
    mut error: *mut *mut GError,
    mut dbus_error_name: *const gchar,
    mut dbus_error_message: *const gchar,
    mut format: *const gchar,
    mut var_args: ::core::ffi::VaList,
) {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !dbus_error_name.is_null() {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"dbus_error_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !dbus_error_message.is_null() {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"dbus_error_message != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if error.is_null() {
        return;
    }
    if !format.is_null() {
        let mut message: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
        message = g_strdup_vprintf(format, var_args);
        s = g_strdup_printf(
            b"%s: %s\0" as *const u8 as *const gchar,
            message,
            dbus_error_message,
        );
        *error = safe_c2rust_g_dbus_error_new_for_dbus_error(dbus_error_name, s);
        g_free(s as gpointer);
        g_free(message as gpointer);
    } else {
        *error = safe_c2rust_g_dbus_error_new_for_dbus_error(dbus_error_name, dbus_error_message);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_strip_remote_error(
    mut error: *mut GError,
) -> gboolean {
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !error.is_null() {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = (*error).message;
            let __prefix: *const ::core::ffi::c_char =
                b"GDBus.Error:\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_42 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_42 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_42
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
            (*error).message,
            b"GDBus.Error:\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        let mut begin: *const gchar = ::core::ptr::null::<gchar>();
        let mut end: *const gchar = ::core::ptr::null::<gchar>();
        let mut new_message: *mut gchar = ::core::ptr::null_mut::<gchar>();
        begin = (*error)
            .message
            .offset(::core::mem::size_of::<[::core::ffi::c_char; 13]>() as usize as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        end = strstr(
            begin as *const ::core::ffi::c_char,
            b":\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !end.is_null()
            && *end.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
        {
            new_message = safe_c2rust_g_strdup_inline(end.offset(2 as ::core::ffi::c_int as isize))
                as *mut gchar;
            g_free((*error).message as gpointer);
            (*error).message = new_message;
            ret = TRUE as gboolean;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_error_encode_gerror(
    mut error: *const GError,
) -> *mut gchar {
    let mut re: *mut RegisteredError = ::core::ptr::null_mut::<RegisteredError>();
    let mut error_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !error.is_null() {
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    _g_dbus_initialize();
    error_name = ::core::ptr::null_mut::<gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__error_lock_lock);
    re = ::core::ptr::null_mut::<RegisteredError>();
    if !safe_c2rust_quark_code_pair_to_re.is_null() {
        let mut pair: QuarkCodePair = QuarkCodePair {
            error_domain: 0,
            error_code: 0,
        };
        pair.error_domain = (*error).domain;
        pair.error_code = (*error).code;
        if ({
            let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
            if !safe_c2rust_dbus_error_name_to_re.is_null() {
                _g_boolean_var_44 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_44 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_44
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbuserror.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                764 as ::core::ffi::c_int,
                G_STRFUNC,
                b"dbus_error_name_to_re != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        re = g_hash_table_lookup(
            safe_c2rust_quark_code_pair_to_re,
            &raw mut pair as gconstpointer,
        ) as *mut RegisteredError;
    }
    if !re.is_null() {
        error_name = safe_c2rust_g_strdup_inline((*re).dbus_error_name) as *mut gchar;
        g_mutex_unlock(&raw mut safe_c2rust_g__error_lock_lock);
    } else {
        let mut domain_as_string: *const gchar = ::core::ptr::null::<gchar>();
        let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
        let mut n: guint = 0;
        g_mutex_unlock(&raw mut safe_c2rust_g__error_lock_lock);
        domain_as_string = g_quark_to_string((*error).domain);
        if ({
            let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
            if !domain_as_string.is_null() {
                _g_boolean_var_45 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_45 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_45
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"domain_as_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<gchar>();
        }
        s = g_string_new(b"org.gtk.GDBus.UnmappedGError.Quark._\0" as *const u8 as *const gchar);
        n = 0 as guint;
        while *domain_as_string.offset(n as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            let mut c: gint = *domain_as_string.offset(n as isize) as gint;
            if *safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_ALNUM as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                safe_c2rust_g_string_append_c_inline(s, c as gchar);
            } else {
                let mut nibble_top: guint = 0;
                let mut nibble_bottom: guint = 0;
                safe_c2rust_g_string_append_c_inline(s, '_' as i32 as gchar);
                nibble_top = (*domain_as_string.offset(n as isize) as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int) as guint;
                nibble_bottom = (*domain_as_string.offset(n as isize) as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int) as guint;
                if nibble_top < 10 as guint {
                    nibble_top = nibble_top.wrapping_add('0' as i32 as guint);
                } else {
                    nibble_top =
                        nibble_top.wrapping_add(('a' as i32 - 10 as ::core::ffi::c_int) as guint);
                }
                if nibble_bottom < 10 as guint {
                    nibble_bottom = nibble_bottom.wrapping_add('0' as i32 as guint);
                } else {
                    nibble_bottom = nibble_bottom
                        .wrapping_add(('a' as i32 - 10 as ::core::ffi::c_int) as guint);
                }
                safe_c2rust_g_string_append_c_inline(s, nibble_top as gchar);
                safe_c2rust_g_string_append_c_inline(s, nibble_bottom as gchar);
            }
            n = n.wrapping_add(1);
        }
        g_string_append_printf(s, b".Code%d\0" as *const u8 as *const gchar, (*error).code);
        error_name = if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(s, 0 as gboolean)
            } else {
                g_string_free_and_steal(s)
            }
        } else {
            g_string_free(s, 0 as gboolean)
        };
    }
    return error_name;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
