extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GAsyncResult;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GIOStreamPrivate;
    pub type _GSocketClientPrivate;
    pub type _GSocketConnectionPrivate;
    pub type _GSocketConnectable;
    pub type _GTask;
    pub type _GWakeup;
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_error_new_literal(domain: GQuark, code: gint, message: *const gchar) -> *mut GError;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_get_user_runtime_dir() -> *const gchar;
    fn geteuid() -> __uid_t;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_setenv(variable: *const gchar, value: *const gchar, overwrite: gboolean) -> gboolean;
    fn g_unsetenv(variable: *const gchar);
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_get_keys_as_ptr_array(hash_table: *mut GHashTable) -> *mut GPtrArray;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert(string: *mut GString, pos: gssize, val: *const gchar) -> *mut GString;
    fn g_string_append_uri_escaped(
        string: *mut GString,
        unescaped: *const gchar,
        reserved_chars_allowed: *const gchar,
        allow_utf8: gboolean,
    ) -> *mut GString;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_print(format: *const gchar, ...);
    fn g_spawn_command_line_sync(
        command_line: *const gchar,
        standard_output: *mut *mut gchar,
        standard_error: *mut *mut gchar,
        wait_status: *mut gint,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_spawn_check_wait_status(wait_status: gint, error: *mut *mut GError) -> gboolean;
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
    fn g_uri_unescape_segment(
        escaped_string: *const ::core::ffi::c_char,
        escaped_string_end: *const ::core::ffi::c_char,
        illegal_characters: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn g_object_unref(object: gpointer);
    fn g_io_error_quark() -> GQuark;
    fn g_bus_type_get_type() -> GType;
    fn glib__private__() -> *const GLibPrivateVTable;
    fn g_network_address_new(hostname: *const gchar, port: guint16) -> *mut GSocketConnectable;
    fn g_socket_client_new() -> *mut GSocketClient;
    fn g_socket_client_set_enable_proxy(client: *mut GSocketClient, enable: gboolean);
    fn g_socket_client_connect(
        client: *mut GSocketClient,
        connectable: *mut GSocketConnectable,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GSocketConnection;
    fn g_output_stream_write_all(
        stream: *mut GOutputStream,
        buffer: *const ::core::ffi::c_void,
        count: gsize,
        bytes_written: *mut gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_io_stream_get_output_stream(stream: *mut GIOStream) -> *mut GOutputStream;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_task_data(
        task: *mut GTask,
        task_data: gpointer,
        task_data_destroy: GDestroyNotify,
    );
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn _g_dbus_debug_address() -> gboolean;
    fn _g_dbus_debug_print_lock();
    fn _g_dbus_debug_print_unlock();
    fn _g_dbus_hexdump(data: *const gchar, len: gsize, indent: guint) -> *mut gchar;
    fn _g_dbus_get_machine_id(error: *mut *mut GError) -> *mut gchar;
    fn _g_dbus_enum_to_string(enum_type: GType, value: gint) -> *mut gchar;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn g_unix_socket_address_new(path: *const gchar) -> *mut GSocketAddress;
    fn g_unix_socket_address_new_with_type(
        path: *const gchar,
        path_len: gint,
        type_0: GUnixSocketAddressType,
    ) -> *mut GSocketAddress;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type guint = ::core::ffi::c_uint;
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
pub type GData = _GData;
pub type GDir = _GDir;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
pub type GMainContext = _GMainContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GType = gsize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed = 0;
pub type GUnixSocketAddressType = ::core::ffi::c_uint;
pub const G_UNIX_SOCKET_ADDRESS_ABSTRACT_PADDED: GUnixSocketAddressType = 4;
pub const G_UNIX_SOCKET_ADDRESS_ABSTRACT: GUnixSocketAddressType = 3;
pub const G_UNIX_SOCKET_ADDRESS_PATH: GUnixSocketAddressType = 2;
pub const G_UNIX_SOCKET_ADDRESS_ANONYMOUS: GUnixSocketAddressType = 1;
pub const G_UNIX_SOCKET_ADDRESS_INVALID: GUnixSocketAddressType = 0;
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GAsyncResult = _GAsyncResult;
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
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketClient {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketClientPrivate,
}
pub type GSocketClientPrivate = _GSocketClientPrivate;
pub type GSocketClient = _GSocketClient;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GSocketConnectionPrivate,
}
pub type GSocketConnectionPrivate = _GSocketConnectionPrivate;
pub type GSocketConnection = _GSocketConnection;
pub type GSocketConnectable = _GSocketConnectable;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetStreamData {
    pub address: *mut gchar,
    pub guid: *mut gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLibPrivateVTable {
    pub g_wakeup_new: Option<unsafe extern "C" fn() -> *mut GWakeup>,
    pub g_wakeup_free: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_get_pollfd: Option<unsafe extern "C" fn(*mut GWakeup, *mut GPollFD) -> ()>,
    pub g_wakeup_signal: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_acknowledge: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_get_worker_context: Option<unsafe extern "C" fn() -> *mut GMainContext>,
    pub g_check_setuid: Option<unsafe extern "C" fn() -> gboolean>,
    pub g_main_context_new_with_next_id: Option<unsafe extern "C" fn(guint) -> *mut GMainContext>,
    pub g_dir_open_with_errno: Option<unsafe extern "C" fn(*const gchar, guint) -> *mut GDir>,
    pub g_dir_new_from_dirp: Option<unsafe extern "C" fn(gpointer) -> *mut GDir>,
    pub glib_init: Option<unsafe extern "C" fn() -> ()>,
    pub g_win32_push_empty_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_win32_pop_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_find_program_for_path: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub g_uri_get_default_scheme_port:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub g_set_prgname_once: Option<unsafe extern "C" fn(*const gchar) -> gboolean>,
    pub g_datalist_id_update_atomic: Option<
        unsafe extern "C" fn(
            *mut *mut GData,
            GQuark,
            GDataListUpdateAtomicFunc,
            gpointer,
        ) -> gpointer,
    >,
}
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWakeup = _GWakeup;
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
pub type GStatBuf = stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IFSOCK: ::core::ffi::c_int = 0o140000 as ::core::ffi::c_int;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
pub const S_IFMT: ::core::ffi::c_int = __S_IFMT;
pub const S_IFSOCK: ::core::ffi::c_int = __S_IFSOCK;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_is_address(mut string: *const gchar) -> gboolean {
    let mut current_block: u64;
    let mut n: guint = 0;
    let mut a: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut ret: gboolean = 0;
    ret = FALSE as gboolean;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    a = g_strsplit(string, b";\0" as *const u8 as *const gchar, 0 as gint);
    if !(*a.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        n = 0 as guint;
        loop {
            if (*a.offset(n as isize)).is_null() {
                current_block = 1841672684692190573;
                break;
            }
            if safe_c2rust__g_dbus_address_parse_entry(
                *a.offset(n as isize),
                ::core::ptr::null_mut::<*mut gchar>(),
                ::core::ptr::null_mut::<*mut GHashTable>(),
                ::core::ptr::null_mut::<*mut GError>(),
            ) == 0
            {
                current_block = 12455471583797641292;
                break;
            }
            n = n.wrapping_add(1);
        }
        match current_block {
            12455471583797641292 => {}
            _ => {
                ret = TRUE as gboolean;
            }
        }
    }
    g_strfreev(a);
    return ret;
}
unsafe extern "C" fn safe_c2rust_is_valid_unix(
    mut address_entry: *const gchar,
    mut key_value_pairs: *mut GHashTable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut keys: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut path: *const gchar = ::core::ptr::null::<gchar>();
    let mut dir: *const gchar = ::core::ptr::null::<gchar>();
    let mut tmpdir: *const gchar = ::core::ptr::null::<gchar>();
    let mut abstract_0: *const gchar = ::core::ptr::null::<gchar>();
    ret = FALSE as gboolean;
    path = ::core::ptr::null::<gchar>();
    dir = ::core::ptr::null::<gchar>();
    tmpdir = ::core::ptr::null::<gchar>();
    abstract_0 = ::core::ptr::null::<gchar>();
    keys = g_hash_table_get_keys_as_ptr_array(key_value_pairs);
    let mut i: guint = 0 as guint;
    loop {
        if !(i < (*keys).len) {
            current_block = 15976848397966268834;
            break;
        }
        let mut key: *const gchar = *(*keys).pdata.offset(i as isize) as *const gchar;
        if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"path\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            path = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"dir\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            dir = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"tmpdir\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            tmpdir = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"abstract\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            abstract_0 = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"guid\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unsupported key \xE2\x80\x9C%s\xE2\x80\x9D in address entry \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                key,
                address_entry,
            );
            current_block = 12101703805452200484;
            break;
        }
        i = i.wrapping_add(1);
    }
    match current_block {
        15976848397966268834 => {
            if (path != NULL_0 as *const gchar) as ::core::ffi::c_int
                + (dir != NULL_0 as *const gchar) as ::core::ffi::c_int
                + (tmpdir != NULL_0 as *const gchar) as ::core::ffi::c_int
                + (abstract_0 != NULL_0 as *const gchar) as ::core::ffi::c_int
                > 1 as ::core::ffi::c_int
            {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Meaningless key/value pair combination in address entry \xE2\x80\x9C%s\xE2\x80\x9D\0"
                            as *const u8 as *const gchar,
                    ),
                    address_entry,
                );
            } else if path.is_null() && dir.is_null() && tmpdir.is_null() && abstract_0.is_null() {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Address \xE2\x80\x9C%s\xE2\x80\x9D is invalid (need exactly one of path, dir, tmpdir, or abstract keys)\0"
                            as *const u8 as *const gchar,
                    ),
                    address_entry,
                );
            } else {
                ret = TRUE as gboolean;
            }
        }
        _ => {}
    }
    g_ptr_array_unref(keys);
    return ret;
}
unsafe extern "C" fn safe_c2rust_is_valid_nonce_tcp(
    mut address_entry: *const gchar,
    mut key_value_pairs: *mut GHashTable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut keys: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut host: *const gchar = ::core::ptr::null::<gchar>();
    let mut port: *const gchar = ::core::ptr::null::<gchar>();
    let mut family: *const gchar = ::core::ptr::null::<gchar>();
    let mut nonce_file: *const gchar = ::core::ptr::null::<gchar>();
    let mut port_num: gint = 0;
    let mut endp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    ret = FALSE as gboolean;
    host = ::core::ptr::null::<gchar>();
    port = ::core::ptr::null::<gchar>();
    family = ::core::ptr::null::<gchar>();
    nonce_file = ::core::ptr::null::<gchar>();
    keys = g_hash_table_get_keys_as_ptr_array(key_value_pairs);
    let mut i: guint = 0 as guint;
    loop {
        if !(i < (*keys).len) {
            current_block = 8457315219000651999;
            break;
        }
        let mut key: *const gchar = *(*keys).pdata.offset(i as isize) as *const gchar;
        if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"host\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            host = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"port\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            port = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"family\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            family = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"noncefile\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            nonce_file = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"guid\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unsupported key \xE2\x80\x9C%s\xE2\x80\x9D in address entry \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                key,
                address_entry,
            );
            current_block = 17465482436384987634;
            break;
        }
        i = i.wrapping_add(1);
    }
    match current_block {
        8457315219000651999 => {
            if !port.is_null() {
                port_num = strtol(
                    port as *const ::core::ffi::c_char,
                    &raw mut endp,
                    10 as ::core::ffi::c_int,
                ) as gint;
                if *port as ::core::ffi::c_int == '\0' as i32
                    || *endp as ::core::ffi::c_int != '\0' as i32
                    || port_num < 0 as ::core::ffi::c_int
                    || port_num >= 65536 as ::core::ffi::c_int
                {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Error in address \xE2\x80\x9C%s\xE2\x80\x9D \xE2\x80\x94 the \xE2\x80\x9C%s\xE2\x80\x9D attribute is malformed\0"
                                as *const u8 as *const gchar,
                        ),
                        address_entry,
                        b"port\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    current_block = 17465482436384987634;
                } else {
                    current_block = 5783071609795492627;
                }
            } else {
                current_block = 5783071609795492627;
            }
            match current_block {
                17465482436384987634 => {}
                _ => {
                    if !family.is_null()
                        && !(g_strcmp0(
                            family as *const ::core::ffi::c_char,
                            b"ipv4\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                            || g_strcmp0(
                                family as *const ::core::ffi::c_char,
                                b"ipv6\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int)
                    {
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Error in address \xE2\x80\x9C%s\xE2\x80\x9D \xE2\x80\x94 the \xE2\x80\x9C%s\xE2\x80\x9D attribute is malformed\0"
                                    as *const u8 as *const gchar,
                            ),
                            address_entry,
                            b"family\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        !host.is_null();
                        if !nonce_file.is_null() && *nonce_file as ::core::ffi::c_int == '\0' as i32
                        {
                            g_set_error(
                                error,
                                g_io_error_quark(),
                                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"Error in address \xE2\x80\x9C%s\xE2\x80\x9D \xE2\x80\x94 the \xE2\x80\x9C%s\xE2\x80\x9D attribute is malformed\0"
                                        as *const u8 as *const gchar,
                                ),
                                address_entry,
                                b"noncefile\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                        } else {
                            ret = TRUE as gboolean;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    g_ptr_array_unref(keys);
    return ret;
}
unsafe extern "C" fn safe_c2rust_is_valid_tcp(
    mut address_entry: *const gchar,
    mut key_value_pairs: *mut GHashTable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut keys: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut host: *const gchar = ::core::ptr::null::<gchar>();
    let mut port: *const gchar = ::core::ptr::null::<gchar>();
    let mut family: *const gchar = ::core::ptr::null::<gchar>();
    let mut port_num: gint = 0;
    let mut endp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    ret = FALSE as gboolean;
    host = ::core::ptr::null::<gchar>();
    port = ::core::ptr::null::<gchar>();
    family = ::core::ptr::null::<gchar>();
    keys = g_hash_table_get_keys_as_ptr_array(key_value_pairs);
    let mut i: guint = 0 as guint;
    loop {
        if !(i < (*keys).len) {
            current_block = 7976072742316086414;
            break;
        }
        let mut key: *const gchar = *(*keys).pdata.offset(i as isize) as *const gchar;
        if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"host\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            host = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"port\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            port = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"family\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            family = g_hash_table_lookup(key_value_pairs, key as gconstpointer) as *const gchar;
        } else if g_strcmp0(
            key as *const ::core::ffi::c_char,
            b"guid\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unsupported key \xE2\x80\x9C%s\xE2\x80\x9D in address entry \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                key,
                address_entry,
            );
            current_block = 9259402581197019563;
            break;
        }
        i = i.wrapping_add(1);
    }
    match current_block {
        7976072742316086414 => {
            if !port.is_null() {
                port_num = strtol(
                    port as *const ::core::ffi::c_char,
                    &raw mut endp,
                    10 as ::core::ffi::c_int,
                ) as gint;
                if *port as ::core::ffi::c_int == '\0' as i32
                    || *endp as ::core::ffi::c_int != '\0' as i32
                    || port_num < 0 as ::core::ffi::c_int
                    || port_num >= 65536 as ::core::ffi::c_int
                {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Error in address \xE2\x80\x9C%s\xE2\x80\x9D \xE2\x80\x94 the \xE2\x80\x9C%s\xE2\x80\x9D attribute is malformed\0"
                                as *const u8 as *const gchar,
                        ),
                        address_entry,
                        b"port\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    current_block = 9259402581197019563;
                } else {
                    current_block = 13242334135786603907;
                }
            } else {
                current_block = 13242334135786603907;
            }
            match current_block {
                9259402581197019563 => {}
                _ => {
                    if !family.is_null()
                        && !(g_strcmp0(
                            family as *const ::core::ffi::c_char,
                            b"ipv4\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                            || g_strcmp0(
                                family as *const ::core::ffi::c_char,
                                b"ipv6\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int)
                    {
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Error in address \xE2\x80\x9C%s\xE2\x80\x9D \xE2\x80\x94 the \xE2\x80\x9C%s\xE2\x80\x9D attribute is malformed\0"
                                    as *const u8 as *const gchar,
                            ),
                            address_entry,
                            b"family\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        !host.is_null();
                        ret = TRUE as gboolean;
                    }
                }
            }
        }
        _ => {}
    }
    g_ptr_array_unref(keys);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_is_supported_address(
    mut string: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut n: guint = 0;
    let mut a: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut ret: gboolean = 0;
    ret = FALSE as gboolean;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    a = g_strsplit(string, b";\0" as *const u8 as *const gchar, 0 as gint);
    n = 0 as guint;
    loop {
        if (*a.offset(n as isize)).is_null() {
            current_block = 14359455889292382949;
            break;
        }
        let mut transport_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut key_value_pairs: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        let mut supported: gboolean = 0;
        if safe_c2rust__g_dbus_address_parse_entry(
            *a.offset(n as isize),
            &raw mut transport_name,
            &raw mut key_value_pairs,
            error,
        ) == 0
        {
            current_block = 7805635556290909496;
            break;
        }
        supported = FALSE as gboolean;
        if g_strcmp0(
            transport_name,
            b"unix\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            supported = safe_c2rust_is_valid_unix(*a.offset(n as isize), key_value_pairs, error);
        } else if g_strcmp0(
            transport_name,
            b"tcp\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            supported = safe_c2rust_is_valid_tcp(*a.offset(n as isize), key_value_pairs, error);
        } else if g_strcmp0(
            transport_name,
            b"nonce-tcp\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            supported =
                safe_c2rust_is_valid_nonce_tcp(*a.offset(n as isize), key_value_pairs, error);
        } else if g_strcmp0(
            *a.offset(n as isize),
            b"autolaunch:\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            supported = TRUE as gboolean;
        } else {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unknown or unsupported transport \xE2\x80\x9C%s\xE2\x80\x9D for address \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                transport_name,
                *a.offset(n as isize),
            );
        }
        g_free(transport_name as gpointer);
        g_hash_table_unref(key_value_pairs);
        if supported == 0 {
            current_block = 7805635556290909496;
            break;
        }
        n = n.wrapping_add(1);
    }
    match current_block {
        14359455889292382949 => {
            ret = TRUE as gboolean;
        }
        _ => {}
    }
    g_strfreev(a);
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ret != 0 || ret == 0 && (error.is_null() || !(*error).is_null()) {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusaddress.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            435 as ::core::ffi::c_int,
            G_STRFUNC,
            b"ret || (!ret && (error == NULL || *error != NULL))\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_address_parse_entry(
    mut address_entry: *const gchar,
    mut out_transport_name: *mut *mut gchar,
    mut out_key_value_pairs: *mut *mut GHashTable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut key_value_pairs: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut transport_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut kv_pairs: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut s: *const gchar = ::core::ptr::null::<gchar>();
    let mut n: guint = 0;
    ret = FALSE as gboolean;
    kv_pairs = ::core::ptr::null_mut::<*mut gchar>();
    transport_name = ::core::ptr::null_mut::<gchar>();
    key_value_pairs = ::core::ptr::null_mut::<GHashTable>();
    s = strchr(address_entry as *const ::core::ffi::c_char, ':' as i32);
    if s.is_null() {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Address element \xE2\x80\x9C%s\xE2\x80\x9D does not contain a colon (:)\0"
                    as *const u8 as *const gchar,
            ),
            address_entry,
        );
    } else if s == address_entry {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Transport name in address element \xE2\x80\x9C%s\xE2\x80\x9D must not be empty\0"
                    as *const u8 as *const gchar,
            ),
            address_entry,
        );
    } else {
        transport_name = g_strndup(
            address_entry,
            s.offset_from(address_entry) as ::core::ffi::c_long as gsize,
        );
        key_value_pairs = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        kv_pairs = g_strsplit(
            s.offset(1 as ::core::ffi::c_int as isize),
            b",\0" as *const u8 as *const gchar,
            0 as gint,
        );
        n = 0 as guint;
        loop {
            if (*kv_pairs.offset(n as isize)).is_null() {
                current_block = 2719512138335094285;
                break;
            }
            let mut kv_pair: *const gchar = *kv_pairs.offset(n as isize);
            let mut key: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
            s = strchr(kv_pair as *const ::core::ffi::c_char, '=' as i32);
            if s.is_null() {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Key/Value pair %d, \xE2\x80\x9C%s\xE2\x80\x9D, in address element \xE2\x80\x9C%s\xE2\x80\x9D does not contain an equal sign\0"
                            as *const u8 as *const gchar,
                    ),
                    n,
                    kv_pair,
                    address_entry,
                );
                current_block = 17167124154147772610;
                break;
            } else if s == kv_pair {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Key/Value pair %d, \xE2\x80\x9C%s\xE2\x80\x9D, in address element \xE2\x80\x9C%s\xE2\x80\x9D must not have an empty key\0"
                            as *const u8 as *const gchar,
                    ),
                    n,
                    kv_pair,
                    address_entry,
                );
                current_block = 17167124154147772610;
                break;
            } else {
                key = g_uri_unescape_segment(
                    kv_pair as *const ::core::ffi::c_char,
                    s as *const ::core::ffi::c_char,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ) as *mut gchar;
                value = g_uri_unescape_segment(
                    s.offset(1 as ::core::ffi::c_int as isize),
                    kv_pair.offset(strlen(kv_pair as *const ::core::ffi::c_char) as isize),
                    ::core::ptr::null::<::core::ffi::c_char>(),
                ) as *mut gchar;
                if key.is_null() || value.is_null() {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Error unescaping key or value in Key/Value pair %d, \xE2\x80\x9C%s\xE2\x80\x9D, in address element \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                as *const u8 as *const gchar,
                        ),
                        n,
                        kv_pair,
                        address_entry,
                    );
                    g_free(key as gpointer);
                    g_free(value as gpointer);
                    current_block = 17167124154147772610;
                    break;
                } else {
                    g_hash_table_insert(key_value_pairs, key as gpointer, value as gpointer);
                    n = n.wrapping_add(1);
                }
            }
        }
        match current_block {
            17167124154147772610 => {}
            _ => {
                ret = TRUE as gboolean;
            }
        }
    }
    if ret != 0 {
        if !out_transport_name.is_null() {
            *out_transport_name = safe_c2rust_g_steal_pointer(&raw mut transport_name as gpointer)
                as *mut gchar as *mut gchar;
        }
        if !out_key_value_pairs.is_null() {
            *out_key_value_pairs = safe_c2rust_g_steal_pointer(&raw mut key_value_pairs as gpointer)
                as *mut GHashTable as *mut GHashTable;
        }
    }
    let mut _pp: *mut *mut GHashTable = &raw mut key_value_pairs;
    let mut _ptr: *mut GHashTable = *_pp;
    *_pp = ::core::ptr::null_mut::<GHashTable>();
    if !_ptr.is_null() {
        g_hash_table_unref(_ptr as *mut GHashTable);
    }
    g_free(transport_name as gpointer);
    g_strfreev(kv_pairs);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_address_connect(
    mut address_entry: *const gchar,
    mut transport_name: *const gchar,
    mut key_value_pairs: *mut GHashTable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GIOStream {
    let mut current_block: u64;
    let mut ret: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut connectable: *mut GSocketConnectable = ::core::ptr::null_mut::<GSocketConnectable>();
    let mut nonce_file: *const gchar = ::core::ptr::null::<gchar>();
    connectable = ::core::ptr::null_mut::<GSocketConnectable>();
    ret = ::core::ptr::null_mut::<GIOStream>();
    nonce_file = ::core::ptr::null::<gchar>();
    if g_strcmp0(
        transport_name as *const ::core::ffi::c_char,
        b"unix\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut path: *const gchar = ::core::ptr::null::<gchar>();
        let mut abstract_0: *const gchar = ::core::ptr::null::<gchar>();
        path = g_hash_table_lookup(
            key_value_pairs,
            b"path\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
        ) as *const gchar;
        abstract_0 = g_hash_table_lookup(
            key_value_pairs,
            b"abstract\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
        ) as *const gchar;
        if path.is_null() && abstract_0.is_null() || !path.is_null() && !abstract_0.is_null() {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Error in address \xE2\x80\x9C%s\xE2\x80\x9D \xE2\x80\x94 the unix transport requires exactly one of the keys \xE2\x80\x9Cpath\xE2\x80\x9D or \xE2\x80\x9Cabstract\xE2\x80\x9D to be set\0"
                        as *const u8 as *const gchar,
                ),
                address_entry,
            );
        } else if !path.is_null() {
            connectable = g_unix_socket_address_new(path) as *mut ::core::ffi::c_void
                as *mut GSocketConnectable;
        } else if !abstract_0.is_null() {
            connectable = g_unix_socket_address_new_with_type(
                abstract_0,
                -(1 as gint),
                G_UNIX_SOCKET_ADDRESS_ABSTRACT,
            ) as *mut ::core::ffi::c_void as *mut GSocketConnectable;
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                603 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        current_block = 17784502470059252271;
    } else if g_strcmp0(
        transport_name as *const ::core::ffi::c_char,
        b"tcp\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || g_strcmp0(
            transport_name as *const ::core::ffi::c_char,
            b"nonce-tcp\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        let mut s: *const gchar = ::core::ptr::null::<gchar>();
        let mut host: *const gchar = ::core::ptr::null::<gchar>();
        let mut port: glong = 0;
        let mut endp: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut is_nonce: gboolean = 0;
        is_nonce = (g_strcmp0(
            transport_name as *const ::core::ffi::c_char,
            b"nonce-tcp\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
        host = g_hash_table_lookup(
            key_value_pairs,
            b"host\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
        ) as *const gchar;
        if host.is_null() {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Error in address \xE2\x80\x9C%s\xE2\x80\x9D \xE2\x80\x94 the host attribute is missing or malformed\0"
                        as *const u8 as *const gchar,
                ),
                address_entry,
            );
            current_block = 14941196108663249936;
        } else {
            s = g_hash_table_lookup(
                key_value_pairs,
                b"port\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
            ) as *const gchar;
            if s.is_null() {
                s = b"0\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
            }
            port = strtol(
                s as *const ::core::ffi::c_char,
                &raw mut endp,
                10 as ::core::ffi::c_int,
            ) as glong;
            if *s as ::core::ffi::c_int == '\0' as i32
                || *endp as ::core::ffi::c_int != '\0' as i32
                || port < 0 as glong
                || port >= 65536 as glong
            {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Error in address \xE2\x80\x9C%s\xE2\x80\x9D \xE2\x80\x94 the port attribute is missing or malformed\0"
                            as *const u8 as *const gchar,
                    ),
                    address_entry,
                );
                current_block = 14941196108663249936;
            } else {
                if is_nonce != 0 {
                    nonce_file = g_hash_table_lookup(
                        key_value_pairs,
                        b"noncefile\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
                    ) as *const gchar;
                    if nonce_file.is_null() {
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Error in address \xE2\x80\x9C%s\xE2\x80\x9D \xE2\x80\x94 the noncefile attribute is missing or malformed\0"
                                    as *const u8 as *const gchar,
                            ),
                            address_entry,
                        );
                        current_block = 14941196108663249936;
                    } else {
                        current_block = 17281240262373992796;
                    }
                } else {
                    current_block = 17281240262373992796;
                }
                match current_block {
                    14941196108663249936 => {}
                    _ => {
                        connectable = g_network_address_new(host, port as guint16);
                        current_block = 17784502470059252271;
                    }
                }
            }
        }
    } else if g_strcmp0(
        address_entry as *const ::core::ffi::c_char,
        b"autolaunch:\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut autolaunch_address: *mut gchar = ::core::ptr::null_mut::<gchar>();
        autolaunch_address = safe_c2rust_get_session_address_dbus_launch(error);
        if !autolaunch_address.is_null() {
            ret = safe_c2rust_g_dbus_address_try_connect_one(
                autolaunch_address,
                ::core::ptr::null_mut::<*mut gchar>(),
                cancellable,
                error,
            );
            g_free(autolaunch_address as gpointer);
            current_block = 14941196108663249936;
        } else {
            g_prefix_error(
                error,
                glib_gettext(b"Error auto-launching: \0" as *const u8 as *const gchar),
            );
            current_block = 17784502470059252271;
        }
    } else {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Unknown or unsupported transport \xE2\x80\x9C%s\xE2\x80\x9D for address \xE2\x80\x9C%s\xE2\x80\x9D\0"
                    as *const u8 as *const gchar,
            ),
            transport_name,
            address_entry,
        );
        current_block = 17784502470059252271;
    }
    match current_block {
        17784502470059252271 => {
            if !connectable.is_null() {
                let mut client: *mut GSocketClient = ::core::ptr::null_mut::<GSocketClient>();
                let mut connection: *mut GSocketConnection =
                    ::core::ptr::null_mut::<GSocketConnection>();
                if ({
                    let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                    if ret.is_null() {
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
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusaddress.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        689 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"ret == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                client = g_socket_client_new();
                g_socket_client_set_enable_proxy(client, FALSE);
                connection = g_socket_client_connect(client, connectable, cancellable, error);
                g_object_unref(connectable as gpointer);
                g_object_unref(client as gpointer);
                if !connection.is_null() {
                    ret = connection as *mut ::core::ffi::c_void as *mut GIOStream;
                    if !nonce_file.is_null() {
                        let mut nonce_contents: [gchar; 17] = [0; 17];
                        let mut num_bytes_read: size_t = 0;
                        let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
                        let mut errsv: ::core::ffi::c_int = 0;
                        f = fopen(
                            nonce_file as *const ::core::ffi::c_char,
                            b"rbe\0" as *const u8 as *const ::core::ffi::c_char,
                        ) as *mut FILE;
                        errsv = *__errno_location();
                        if f.is_null() {
                            g_set_error(
                                error,
                                g_io_error_quark(),
                                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"Error opening nonce file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                                        as *const u8
                                        as *const gchar,
                                ),
                                nonce_file,
                                g_strerror(errsv as gint),
                            );
                            g_object_unref(ret as gpointer);
                            ret = ::core::ptr::null_mut::<GIOStream>();
                        } else {
                            num_bytes_read = fread(
                                &raw mut nonce_contents as *mut gchar as *mut ::core::ffi::c_void,
                                ::core::mem::size_of::<gchar>() as size_t,
                                (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
                                f,
                            ) as size_t;
                            errsv = *__errno_location();
                            if num_bytes_read != 16 as size_t {
                                if num_bytes_read == 0 as size_t {
                                    g_set_error(
                                        error,
                                        g_io_error_quark(),
                                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                        glib_gettext(
                                            b"Error reading from nonce file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                                                as *const u8 as *const gchar,
                                        ),
                                        nonce_file,
                                        g_strerror(errsv as gint),
                                    );
                                } else {
                                    g_set_error(
                                        error,
                                        g_io_error_quark(),
                                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                        glib_gettext(
                                            b"Error reading from nonce file \xE2\x80\x9C%s\xE2\x80\x9D, expected 16 bytes, got %d\0"
                                                as *const u8 as *const gchar,
                                        ),
                                        nonce_file,
                                        num_bytes_read as gint,
                                    );
                                }
                                g_object_unref(ret as gpointer);
                                ret = ::core::ptr::null_mut::<GIOStream>();
                                fclose(f);
                            } else {
                                fclose(f);
                                if g_output_stream_write_all(
                                    g_io_stream_get_output_stream(ret),
                                    &raw mut nonce_contents as *mut gchar
                                        as *const ::core::ffi::c_void,
                                    16 as gsize,
                                    ::core::ptr::null_mut::<gsize>(),
                                    cancellable,
                                    error,
                                ) == 0
                                {
                                    g_prefix_error(
                                        error,
                                        glib_gettext(
                                            b"Error writing contents of nonce file \xE2\x80\x9C%s\xE2\x80\x9D to stream:\0"
                                                as *const u8 as *const gchar,
                                        ),
                                        nonce_file,
                                    );
                                    g_object_unref(ret as gpointer);
                                    ret = ::core::ptr::null_mut::<GIOStream>();
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_address_try_connect_one(
    mut address_entry: *const gchar,
    mut out_guid: *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GIOStream {
    let mut ret: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut key_value_pairs: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut transport_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut guid: *const gchar = ::core::ptr::null::<gchar>();
    ret = ::core::ptr::null_mut::<GIOStream>();
    transport_name = ::core::ptr::null_mut::<gchar>();
    key_value_pairs = ::core::ptr::null_mut::<GHashTable>();
    if !(safe_c2rust__g_dbus_address_parse_entry(
        address_entry,
        &raw mut transport_name,
        &raw mut key_value_pairs,
        error,
    ) == 0)
    {
        ret = safe_c2rust_g_dbus_address_connect(
            address_entry,
            transport_name,
            key_value_pairs,
            cancellable,
            error,
        );
        if !ret.is_null() {
            guid = g_hash_table_lookup(
                key_value_pairs,
                b"guid\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
            ) as *const gchar;
            if !guid.is_null() && !out_guid.is_null() {
                *out_guid =
                    safe_c2rust_g_strdup_inline(guid as *const ::core::ffi::c_char) as *mut gchar;
            }
        }
    }
    g_free(transport_name as gpointer);
    if !key_value_pairs.is_null() {
        g_hash_table_unref(key_value_pairs);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_get_stream_data_free(mut data: *mut GetStreamData) {
    g_free((*data).address as gpointer);
    g_free((*data).guid as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_get_stream_thread_func(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut GetStreamData = task_data as *mut GetStreamData;
    let mut stream: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    stream = safe_c2rust_g_dbus_address_get_stream_sync(
        (*data).address,
        &raw mut (*data).guid,
        cancellable,
        &raw mut error,
    );
    if !stream.is_null() {
        g_task_return_pointer(
            task,
            stream as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    } else {
        g_task_return_error(task, error);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_address_get_stream(
    mut address: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut GetStreamData = ::core::ptr::null_mut::<GetStreamData>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !address.is_null() {
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
            b"address != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GetStreamData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GetStreamData;
    (*data).address =
        safe_c2rust_g_strdup_inline(address as *const ::core::ffi::c_char) as *mut gchar;
    task = g_task_new(NULL_0, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const gchar,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_dbus_address_get_stream
                as unsafe extern "C" fn(
                    *const gchar,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_dbus_address_get_stream\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GetStreamData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_get_stream_data_free as unsafe extern "C" fn(*mut GetStreamData) -> (),
        )),
    );
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_get_stream_thread_func
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_address_get_stream_finish(
    mut res: *mut GAsyncResult,
    mut out_guid: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> *mut GIOStream {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut GetStreamData = ::core::ptr::null_mut::<GetStreamData>();
    let mut ret: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if g_task_is_valid(
            res as gpointer,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) != 0
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
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOStream>();
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOStream>();
    }
    task = res as *mut ::core::ffi::c_void as *mut GTask;
    ret = g_task_propagate_pointer(task, error) as *mut GIOStream;
    if !ret.is_null() && !out_guid.is_null() {
        data = g_task_get_task_data(task) as *mut GetStreamData;
        *out_guid = (*data).guid;
        (*data).guid = ::core::ptr::null_mut::<gchar>();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_address_get_stream_sync(
    mut address: *const gchar,
    mut out_guid: *mut *mut gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GIOStream {
    let mut ret: *mut GIOStream = ::core::ptr::null_mut::<GIOStream>();
    let mut addr_array: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut n: guint = 0;
    let mut last_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !address.is_null() {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"address != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOStream>();
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOStream>();
    }
    ret = ::core::ptr::null_mut::<GIOStream>();
    last_error = ::core::ptr::null_mut::<GError>();
    addr_array = g_strsplit(address, b";\0" as *const u8 as *const gchar, 0 as gint);
    if (*addr_array.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        last_error = g_error_new_literal(
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"The given address is empty\0" as *const u8 as *const gchar),
        );
    } else {
        n = 0 as guint;
        while !(*addr_array.offset(n as isize)).is_null() {
            let mut addr: *const gchar = *addr_array.offset(n as isize);
            let mut this_error: *mut GError = ::core::ptr::null_mut::<GError>();
            this_error = ::core::ptr::null_mut::<GError>();
            ret = safe_c2rust_g_dbus_address_try_connect_one(
                addr,
                out_guid,
                cancellable,
                &raw mut this_error,
            );
            if !ret.is_null() {
                break;
            }
            if ({
                let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
                if !this_error.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusaddress.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1005 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"this_error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !last_error.is_null() {
                g_error_free(last_error);
            }
            last_error = this_error;
            n = n.wrapping_add(1);
        }
    }
    if !ret.is_null() {
        if !last_error.is_null() {
            g_error_free(last_error);
        }
    } else {
        if ({
            let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
            if !last_error.is_null() {
                _g_boolean_var_21 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_21 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_21
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusaddress.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1020 as ::core::ffi::c_int,
                G_STRFUNC,
                b"last_error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_propagate_error(error, last_error);
    }
    g_strfreev(addr_array);
    return ret;
}
unsafe extern "C" fn safe_c2rust_get_session_address_xdg() -> *mut gchar {
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut bus: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut buf: GStatBuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    bus = g_build_filename(
        g_get_user_runtime_dir(),
        b"bus\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    if !(stat(bus, &raw mut buf) < 0 as ::core::ffi::c_int) {
        if !(buf.st_uid != geteuid()) {
            if !(buf.st_mode & S_IFMT as __mode_t != S_IFSOCK as __mode_t) {
                tmp = safe_c2rust_g_dbus_address_escape_value(bus);
                ret = g_strconcat(b"unix:path=\0" as *const u8 as *const gchar, tmp, NULL_0);
                g_free(tmp as gpointer);
            }
        }
    }
    g_free(bus as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_get_session_address_dbus_launch(
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut machine_id: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut command_line: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut launch_stdout: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut launch_stderr: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut wait_status: gint = 0;
    let mut old_dbus_verbose: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut restore_dbus_verbose: gboolean = 0;
    ret = ::core::ptr::null_mut::<gchar>();
    machine_id = ::core::ptr::null_mut::<gchar>();
    command_line = ::core::ptr::null_mut::<gchar>();
    launch_stdout = ::core::ptr::null_mut::<gchar>();
    launch_stderr = ::core::ptr::null_mut::<gchar>();
    restore_dbus_verbose = FALSE as gboolean;
    old_dbus_verbose = ::core::ptr::null_mut::<gchar>();
    if (*glib__private__())
        .g_check_setuid
        .expect("non-null function pointer")()
        != 0
    {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Cannot spawn a message bus when AT_SECURE is set\0" as *const u8 as *const gchar,
            ),
        );
    } else {
        machine_id = _g_dbus_get_machine_id(error);
        if machine_id.is_null() {
            g_prefix_error(
                error,
                glib_gettext(
                    b"Cannot spawn a message bus without a machine-id: \0" as *const u8
                        as *const gchar,
                ),
            );
        } else if g_getenv(b"DISPLAY\0" as *const u8 as *const gchar).is_null() {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Cannot autolaunch D-Bus without X11 $DISPLAY\0" as *const u8 as *const gchar,
                ),
            );
        } else {
            command_line = g_strdup_printf(
                b"dbus-launch --autolaunch=%s --binary-syntax --close-stderr\0" as *const u8
                    as *const gchar,
                machine_id,
            );
            if ({
                let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                if _g_dbus_debug_address() != 0 {
                    _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_22
            }) as ::core::ffi::c_long
                != 0
            {
                _g_dbus_debug_print_lock();
                g_print(
                    b"GDBus-debug:Address: Running '%s' to get bus address (possibly autolaunching)\n\0"
                        as *const u8 as *const gchar,
                    command_line,
                );
                old_dbus_verbose = safe_c2rust_g_strdup_inline(g_getenv(
                    b"DBUS_VERBOSE\0" as *const u8 as *const gchar,
                )
                    as *const ::core::ffi::c_char) as *mut gchar;
                restore_dbus_verbose = TRUE as gboolean;
                g_setenv(
                    b"DBUS_VERBOSE\0" as *const u8 as *const gchar,
                    b"1\0" as *const u8 as *const gchar,
                    TRUE,
                );
                _g_dbus_debug_print_unlock();
            }
            if !(g_spawn_command_line_sync(
                command_line,
                &raw mut launch_stdout,
                &raw mut launch_stderr,
                &raw mut wait_status,
                error,
            ) == 0)
            {
                if g_spawn_check_wait_status(wait_status, error) == 0 {
                    g_prefix_error(
                        error,
                        glib_gettext(
                            b"Error spawning command line \xE2\x80\x9C%s\xE2\x80\x9D: \0"
                                as *const u8 as *const gchar,
                        ),
                        command_line,
                    );
                } else {
                    ret = safe_c2rust_g_strdup_inline(launch_stdout) as *mut gchar;
                }
            }
        }
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if _g_dbus_debug_address() != 0 {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
        let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
        _g_dbus_debug_print_lock();
        g_print(b"GDBus-debug:Address: dbus-launch output:\0" as *const u8 as *const gchar);
        if !launch_stdout.is_null() {
            s = _g_dbus_hexdump(
                launch_stdout,
                (strlen(launch_stdout) as gsize)
                    .wrapping_add(1 as gsize)
                    .wrapping_add(::core::mem::size_of::<pid_t>() as gsize)
                    .wrapping_add(::core::mem::size_of::<::core::ffi::c_long>() as gsize),
                2 as guint,
            );
            g_print(b"\n%s\0" as *const u8 as *const gchar, s);
            g_free(s as gpointer);
        } else {
            g_print(b" (none)\n\0" as *const u8 as *const gchar);
        }
        g_print(b"GDBus-debug:Address: dbus-launch stderr output:\0" as *const u8 as *const gchar);
        if !launch_stderr.is_null() {
            g_print(b"\n%s\0" as *const u8 as *const gchar, launch_stderr);
        } else {
            g_print(b" (none)\n\0" as *const u8 as *const gchar);
        }
        _g_dbus_debug_print_unlock();
    }
    g_free(machine_id as gpointer);
    g_free(command_line as gpointer);
    g_free(launch_stdout as gpointer);
    g_free(launch_stderr as gpointer);
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if restore_dbus_verbose != 0 {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
        if !old_dbus_verbose.is_null() {
            g_setenv(
                b"DBUS_VERBOSE\0" as *const u8 as *const gchar,
                old_dbus_verbose,
                TRUE,
            );
        } else {
            g_unsetenv(b"DBUS_VERBOSE\0" as *const u8 as *const gchar);
        }
    }
    g_free(old_dbus_verbose as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_get_session_address_platform_specific(
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    ret = safe_c2rust_get_session_address_xdg();
    if !ret.is_null() {
        return ret;
    }
    return safe_c2rust_get_session_address_dbus_launch(error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_address_get_for_bus_sync(
    mut bus_type: GBusType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut has_elevated_privileges: gboolean = (*glib__private__())
        .g_check_setuid
        .expect("non-null function pointer")();
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut starter_bus: *const gchar = ::core::ptr::null::<gchar>();
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    ret = ::core::ptr::null_mut::<gchar>();
    local_error = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if _g_dbus_debug_address() != 0 {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
        let mut n: guint = 0;
        _g_dbus_debug_print_lock();
        s = _g_dbus_enum_to_string(g_bus_type_get_type(), bus_type as gint);
        g_print(
            b"GDBus-debug:Address: In g_dbus_address_get_for_bus_sync() for bus type '%s'\n\0"
                as *const u8 as *const gchar,
            s,
        );
        g_free(s as gpointer);
        n = 0 as guint;
        while n < 3 as guint {
            let mut k: *const gchar = ::core::ptr::null::<gchar>();
            let mut v: *const gchar = ::core::ptr::null::<gchar>();
            match n {
                0 => {
                    k = b"DBUS_SESSION_BUS_ADDRESS\0" as *const u8 as *const ::core::ffi::c_char
                        as *const gchar;
                }
                1 => {
                    k = b"DBUS_SYSTEM_BUS_ADDRESS\0" as *const u8 as *const ::core::ffi::c_char
                        as *const gchar;
                }
                2 => {
                    k = b"DBUS_STARTER_BUS_TYPE\0" as *const u8 as *const ::core::ffi::c_char
                        as *const gchar;
                }
                _ => {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusaddress.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        1313 as ::core::ffi::c_int,
                        G_STRFUNC,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
            }
            v = g_getenv(k);
            g_print(
                b"GDBus-debug:Address: env var %s\0" as *const u8 as *const gchar,
                k,
            );
            if !v.is_null() {
                g_print(b"='%s'\n\0" as *const u8 as *const gchar, v);
            } else {
                g_print(b" is not set\n\0" as *const u8 as *const gchar);
            }
            n = n.wrapping_add(1);
        }
        _g_dbus_debug_print_unlock();
    }
    match bus_type as ::core::ffi::c_int {
        1 => {
            if has_elevated_privileges != 0 {
                ret = ::core::ptr::null_mut::<gchar>();
            } else {
                ret = safe_c2rust_g_strdup_inline(g_getenv(
                    b"DBUS_SYSTEM_BUS_ADDRESS\0" as *const u8 as *const gchar,
                ) as *const ::core::ffi::c_char) as *mut gchar;
            }
            if ret.is_null() {
                ret = safe_c2rust_g_strdup_inline(
                    b"unix:path=/run/dbus/system_bus_socket\0" as *const u8
                        as *const ::core::ffi::c_char,
                ) as *mut gchar;
            }
        }
        2 => {
            if has_elevated_privileges != 0 {
                ret = ::core::ptr::null_mut::<gchar>();
            } else {
                ret = safe_c2rust_g_strdup_inline(g_getenv(
                    b"DBUS_SESSION_BUS_ADDRESS\0" as *const u8 as *const gchar,
                ) as *const ::core::ffi::c_char) as *mut gchar;
            }
            if ret.is_null() {
                ret = safe_c2rust_get_session_address_platform_specific(&raw mut local_error);
            }
        }
        -1 => {
            starter_bus = g_getenv(b"DBUS_STARTER_BUS_TYPE\0" as *const u8 as *const gchar);
            if g_strcmp0(
                starter_bus as *const ::core::ffi::c_char,
                b"session\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                ret = safe_c2rust_g_dbus_address_get_for_bus_sync(
                    G_BUS_TYPE_SESSION,
                    cancellable,
                    &raw mut local_error,
                );
            } else if g_strcmp0(
                starter_bus as *const ::core::ffi::c_char,
                b"system\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                ret = safe_c2rust_g_dbus_address_get_for_bus_sync(
                    G_BUS_TYPE_SYSTEM,
                    cancellable,
                    &raw mut local_error,
                );
            } else if !starter_bus.is_null() {
                g_set_error(
                    &raw mut local_error,
                    g_io_error_quark(),
                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Cannot determine bus address from DBUS_STARTER_BUS_TYPE environment variable \xE2\x80\x94 unknown value \xE2\x80\x9C%s\xE2\x80\x9D\0"
                            as *const u8 as *const gchar,
                    ),
                    starter_bus,
                );
            } else {
                g_set_error_literal(
                    &raw mut local_error,
                    g_io_error_quark(),
                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Cannot determine bus address because the DBUS_STARTER_BUS_TYPE environment variable is not set\0"
                            as *const u8 as *const gchar,
                    ),
                );
            }
        }
        _ => {
            g_set_error(
                &raw mut local_error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(b"Unknown bus type %d\0" as *const u8 as *const gchar),
                bus_type as ::core::ffi::c_int,
            );
        }
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if _g_dbus_debug_address() != 0 {
            _g_boolean_var_27 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_27 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_27
    }) as ::core::ffi::c_long
        != 0
    {
        _g_dbus_debug_print_lock();
        s = _g_dbus_enum_to_string(g_bus_type_get_type(), bus_type as gint);
        if !ret.is_null() {
            g_print(
                b"GDBus-debug:Address: Returning address '%s' for bus type '%s'\n\0" as *const u8
                    as *const gchar,
                ret,
                s,
            );
        } else {
            g_print(
                b"GDBus-debug:Address: Cannot look-up address bus type '%s': %s\n\0" as *const u8
                    as *const gchar,
                s,
                if !local_error.is_null() {
                    (*local_error).message as *const gchar
                } else {
                    b"\0" as *const u8 as *const gchar
                },
            );
        }
        g_free(s as gpointer);
        _g_dbus_debug_print_unlock();
    }
    if !local_error.is_null() {
        g_propagate_error(error, local_error);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_address_escape_value(
    mut string: *const gchar,
) -> *mut gchar {
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    s = g_string_sized_new(strlen(string as *const ::core::ffi::c_char) as gsize);
    g_string_append_uri_escaped(s, string, b"\\/\0" as *const u8 as *const gchar, FALSE);
    i = 0 as gsize;
    while i < (*s).len {
        if ({
            let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
            if *(*s).str_0.offset(i as isize) as ::core::ffi::c_int == '~' as i32 {
                _g_boolean_var_29 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_29 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_29
        }) as ::core::ffi::c_long
            != 0
        {
            *(*s).str_0.offset(i as isize) = '%' as i32 as gchar;
            g_string_insert(
                s,
                i.wrapping_add(1 as gsize) as gssize,
                b"7E\0" as *const u8 as *const gchar,
            );
            i = i.wrapping_add(2 as gsize);
        }
        i = i.wrapping_add(1);
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(s, 0 as gboolean)
        } else {
            g_string_free_and_steal(s)
        }
    } else {
        g_string_free(s, 0 as gboolean)
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
