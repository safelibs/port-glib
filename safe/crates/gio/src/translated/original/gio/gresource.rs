extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GMappedFile;
    pub type _GVariant;
    pub type _GCancellable;
    pub type _GConverter;
    pub type _GInputStreamPrivate;
    pub type _GZlibDecompressor;
    pub type _GFile;
    pub type _GFileInputStream;
    pub type _GvdbTable;
    pub type _GWakeup;
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
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
    fn g_rw_lock_writer_lock(rw_lock: *mut GRWLock);
    fn g_rw_lock_writer_unlock(rw_lock: *mut GRWLock);
    fn g_rw_lock_reader_lock(rw_lock: *mut GRWLock);
    fn g_rw_lock_reader_unlock(rw_lock: *mut GRWLock);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_bytes_new(data: gconstpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_new_take(data: gpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_new_static(data: gconstpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_new_with_free_func(
        data: gconstpointer,
        size: gsize,
        free_func: GDestroyNotify,
        user_data: gpointer,
    ) -> *mut GBytes;
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_bytes_get_size(bytes: *mut GBytes) -> gsize;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_dir_open(path: *const gchar, flags: guint, error: *mut *mut GError) -> *mut GDir;
    fn g_dir_read_name(dir: *mut GDir) -> *const gchar;
    fn g_dir_close(dir: *mut GDir);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_file_error_quark() -> GQuark;
    fn g_file_test(filename: *const gchar, test: GFileTest) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_dgettext(domain: *const gchar, msgid: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_remove(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_list_find(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_steal_all(hash_table: *mut GHashTable);
    fn g_hash_table_get_keys_as_array(
        hash_table: *mut GHashTable,
        length: *mut guint,
    ) -> *mut gpointer;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_mapped_file_new(
        filename: *const gchar,
        writable: gboolean,
        error: *mut *mut GError,
    ) -> *mut GMappedFile;
    fn g_mapped_file_get_bytes(file: *mut GMappedFile) -> *mut GBytes;
    fn g_mapped_file_unref(file: *mut GMappedFile);
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_get_size(value: *mut GVariant) -> gsize;
    fn g_variant_get_data(value: *mut GVariant) -> gconstpointer;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_boxed_type_register_static(
        name: *const gchar,
        boxed_copy: GBoxedCopyFunc,
        boxed_free: GBoxedFreeFunc,
    ) -> GType;
    fn g_object_unref(object: gpointer);
    fn g_object_set_data_full(
        object: *mut GObject,
        key: *const gchar,
        data: gpointer,
        destroy: GDestroyNotify,
    );
    fn gvdb_table_new(
        filename: *const gchar,
        trusted: gboolean,
        error: *mut *mut GError,
    ) -> *mut GvdbTable;
    fn gvdb_table_new_from_bytes(
        bytes: *mut GBytes,
        trusted: gboolean,
        error: *mut *mut GError,
    ) -> *mut GvdbTable;
    fn gvdb_table_free(table: *mut GvdbTable);
    fn gvdb_table_get_raw_value(table: *mut GvdbTable, key: *const gchar) -> *mut GVariant;
    fn gvdb_table_list(table: *mut GvdbTable, key: *const gchar) -> *mut *mut gchar;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn g_file_new_for_path(path: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_read(
        file: *mut GFile,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileInputStream;
    fn g_io_error_quark() -> GQuark;
    fn g_memory_input_stream_new_from_data(
        data: *const ::core::ffi::c_void,
        len: gssize,
        destroy: GDestroyNotify,
    ) -> *mut GInputStream;
    fn g_converter_convert(
        converter: *mut GConverter,
        inbuf: *const ::core::ffi::c_void,
        inbuf_size: gsize,
        outbuf: *mut ::core::ffi::c_void,
        outbuf_size: gsize,
        flags: GConverterFlags,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> GConverterResult;
    fn g_zlib_decompressor_new(format: GZlibCompressorFormat) -> *mut GZlibDecompressor;
    fn g_converter_input_stream_new(
        base_stream: *mut GInputStream,
        converter: *mut GConverter,
    ) -> *mut GInputStream;
    fn glib__private__() -> *const GLibPrivateVTable;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GBytes = _GBytes;
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
pub struct _GRWLock {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GRWLock = _GRWLock;
pub type GData = _GData;
pub type GDir = _GDir;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_FILE_ERROR_FAILED: C2RustUnnamed = 24;
pub const G_FILE_ERROR_NOSYS: C2RustUnnamed = 23;
pub const G_FILE_ERROR_PERM: C2RustUnnamed = 22;
pub const G_FILE_ERROR_IO: C2RustUnnamed = 21;
pub const G_FILE_ERROR_INTR: C2RustUnnamed = 20;
pub const G_FILE_ERROR_AGAIN: C2RustUnnamed = 19;
pub const G_FILE_ERROR_PIPE: C2RustUnnamed = 18;
pub const G_FILE_ERROR_INVAL: C2RustUnnamed = 17;
pub const G_FILE_ERROR_BADF: C2RustUnnamed = 16;
pub const G_FILE_ERROR_NFILE: C2RustUnnamed = 15;
pub const G_FILE_ERROR_MFILE: C2RustUnnamed = 14;
pub const G_FILE_ERROR_NOMEM: C2RustUnnamed = 13;
pub const G_FILE_ERROR_NOSPC: C2RustUnnamed = 12;
pub const G_FILE_ERROR_LOOP: C2RustUnnamed = 11;
pub const G_FILE_ERROR_FAULT: C2RustUnnamed = 10;
pub const G_FILE_ERROR_TXTBSY: C2RustUnnamed = 9;
pub const G_FILE_ERROR_ROFS: C2RustUnnamed = 8;
pub const G_FILE_ERROR_NODEV: C2RustUnnamed = 7;
pub const G_FILE_ERROR_NXIO: C2RustUnnamed = 6;
pub const G_FILE_ERROR_NOTDIR: C2RustUnnamed = 5;
pub const G_FILE_ERROR_NOENT: C2RustUnnamed = 4;
pub const G_FILE_ERROR_NAMETOOLONG: C2RustUnnamed = 3;
pub const G_FILE_ERROR_ACCES: C2RustUnnamed = 2;
pub const G_FILE_ERROR_ISDIR: C2RustUnnamed = 1;
pub const G_FILE_ERROR_EXIST: C2RustUnnamed = 0;
pub type GFileTest = ::core::ffi::c_uint;
pub const G_FILE_TEST_EXISTS: GFileTest = 16;
pub const G_FILE_TEST_IS_EXECUTABLE: GFileTest = 8;
pub const G_FILE_TEST_IS_DIR: GFileTest = 4;
pub const G_FILE_TEST_IS_SYMLINK: GFileTest = 2;
pub const G_FILE_TEST_IS_REGULAR: GFileTest = 1;
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
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
pub type GMainContext = _GMainContext;
pub type GMappedFile = _GMappedFile;
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
pub type GBoxedCopyFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GBoxedFreeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GConverterFlags = ::core::ffi::c_uint;
pub const G_CONVERTER_FLUSH: GConverterFlags = 2;
pub const G_CONVERTER_INPUT_AT_END: GConverterFlags = 1;
pub const G_CONVERTER_NO_FLAGS: GConverterFlags = 0;
pub type GConverterResult = ::core::ffi::c_uint;
pub const G_CONVERTER_FLUSHED: GConverterResult = 3;
pub const G_CONVERTER_FINISHED: GConverterResult = 2;
pub const G_CONVERTER_CONVERTED: GConverterResult = 1;
pub const G_CONVERTER_ERROR: GConverterResult = 0;
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
pub const G_RESOURCE_ERROR_INTERNAL: C2RustUnnamed_1 = 1;
pub const G_RESOURCE_ERROR_NOT_FOUND: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const G_RESOURCE_FLAGS_COMPRESSED: C2RustUnnamed_2 = 1;
pub const G_RESOURCE_FLAGS_NONE: C2RustUnnamed_2 = 0;
pub type GResourceLookupFlags = ::core::ffi::c_uint;
pub const G_RESOURCE_LOOKUP_FLAGS_NONE: GResourceLookupFlags = 0;
pub type GZlibCompressorFormat = ::core::ffi::c_uint;
pub const G_ZLIB_COMPRESSOR_FORMAT_RAW: GZlibCompressorFormat = 2;
pub const G_ZLIB_COMPRESSOR_FORMAT_GZIP: GZlibCompressorFormat = 1;
pub const G_ZLIB_COMPRESSOR_FORMAT_ZLIB: GZlibCompressorFormat = 0;
pub type GCancellable = _GCancellable;
pub type GConverter = _GConverter;
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GZlibDecompressor = _GZlibDecompressor;
pub type GFile = _GFile;
pub type GFileInputStream = _GFileInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResource {
    pub ref_count: ::core::ffi::c_int,
    pub table: *mut GvdbTable,
}
pub type GvdbTable = _GvdbTable;
pub type GResource = _GResource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GStaticResource {
    pub data: *const guint8,
    pub data_len: gsize,
    pub resource: *mut GResource,
    pub next: *mut GStaticResource,
    pub padding: gpointer,
}
pub type GStaticResource = _GStaticResource;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GResource) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GResource) -> *mut GResource>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GResource) -> *mut GResource>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
pub type CheckCandidate = Option<unsafe extern "C" fn(*const gchar, gpointer) -> gboolean>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfoData {
    pub size: gsize,
    pub flags: guint32,
}
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
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_SEARCHPATH_SEPARATOR_S: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b":\0") };
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_resource_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_resource_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_4, C2RustUnnamed_3) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_4, C2RustUnnamed_3) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GResource\0" as *const u8 as *const gchar),
        C2RustUnnamed_4 {
            do_copy_type: Some(
                safe_c2rust_g_resource_ref
                    as unsafe extern "C" fn(*mut GResource) -> *mut GResource,
            ),
        },
        C2RustUnnamed_3 {
            do_free_type: Some(
                safe_c2rust_g_resource_unref as unsafe extern "C" fn(*mut GResource) -> (),
            ),
        },
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_open_overlay_stream(
    mut candidate: *const gchar,
    mut user_data: gpointer,
) -> gboolean {
    let mut res: *mut *mut GInputStream = user_data as *mut *mut GInputStream;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    file = g_file_new_for_path(candidate as *const ::core::ffi::c_char);
    *res = g_file_read(
        file,
        ::core::ptr::null_mut::<GCancellable>(),
        &raw mut error,
    ) as *mut GInputStream;
    if !(*res).is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_MESSAGE,
            b"Opened file '%s' as a resource overlay\0" as *const u8 as *const gchar,
            candidate,
        );
    } else {
        if g_error_matches(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Can't open overlay file '%s': %s\0" as *const u8 as *const gchar,
                candidate,
                (*error).message,
            );
        }
        g_error_free(error);
    }
    g_object_unref(file as gpointer);
    return (*res != NULL_0 as *mut GInputStream) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_get_overlay_bytes(
    mut candidate: *const gchar,
    mut user_data: gpointer,
) -> gboolean {
    let mut res: *mut *mut GBytes = user_data as *mut *mut GBytes;
    let mut mapped_file: *mut GMappedFile = ::core::ptr::null_mut::<GMappedFile>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    mapped_file = g_mapped_file_new(candidate, FALSE, &raw mut error);
    if !mapped_file.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_MESSAGE,
            b"Mapped file '%s' as a resource overlay\0" as *const u8 as *const gchar,
            candidate,
        );
        *res = g_mapped_file_get_bytes(mapped_file);
        g_mapped_file_unref(mapped_file);
    } else {
        if g_error_matches(
            error,
            g_file_error_quark(),
            G_FILE_ERROR_NOENT as ::core::ffi::c_int as gint,
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Can't mmap overlay file '%s': %s\0" as *const u8 as *const gchar,
                candidate,
                (*error).message,
            );
        }
        g_error_free(error);
    }
    return (*res != NULL_0 as *mut GBytes) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_enumerate_overlay_dir(
    mut candidate: *const gchar,
    mut user_data: gpointer,
) -> gboolean {
    let mut hash: *mut *mut GHashTable = user_data as *mut *mut GHashTable;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut dir: *mut GDir = ::core::ptr::null_mut::<GDir>();
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    dir = g_dir_open(candidate, 0 as guint, &raw mut error);
    if !dir.is_null() {
        if (*hash).is_null() {
            *hash = g_hash_table_new_full(
                Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                None,
            );
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_MESSAGE,
            b"Enumerating directory '%s' as resource overlay\0" as *const u8 as *const gchar,
            candidate,
        );
        loop {
            name = g_dir_read_name(dir);
            if name.is_null() {
                break;
            }
            let mut fullname: *mut gchar = g_build_filename(candidate, name, NULL_0);
            if g_file_test(fullname, G_FILE_TEST_IS_DIR) != 0 {
                g_hash_table_add(
                    *hash,
                    g_strconcat(
                        name,
                        b"/\0" as *const u8 as *const ::core::ffi::c_char,
                        NULL_0,
                    ) as gpointer,
                );
            } else {
                g_hash_table_add(
                    *hash,
                    safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as gpointer,
                );
            }
            g_free(fullname as gpointer);
        }
        g_dir_close(dir);
    } else {
        if g_error_matches(
            error,
            g_file_error_quark(),
            G_FILE_ERROR_NOENT as ::core::ffi::c_int as gint,
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Can't enumerate overlay directory '%s': %s\0" as *const u8 as *const gchar,
                candidate,
                (*error).message,
            );
        }
        g_error_free(error);
        return FALSE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_get_overlay_info(
    mut candidate: *const gchar,
    mut user_data: gpointer,
) -> gboolean {
    let mut info: *mut InfoData = user_data as *mut InfoData;
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
    if stat(candidate as *const ::core::ffi::c_char, &raw mut buf) < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    (*info).size = buf.st_size as gsize;
    (*info).flags = G_RESOURCE_FLAGS_NONE as ::core::ffi::c_int as guint32;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_resource_find_overlay(
    mut path: *const gchar,
    mut check: CheckCandidate,
    mut user_data: gpointer,
) -> gboolean {
    static mut safe_c2rust_overlay_dirs: *const *const gchar = ::core::ptr::null::<*const gchar>();
    let mut res: gboolean = FALSE;
    let mut path_len: gint = -(1 as gint);
    let mut i: gint = 0;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_overlay_dirs;
        } else {
        };
        (({
            let mut gapg_temp_newval: *const *const gchar = ::core::ptr::null::<*const gchar>();
            let mut gapg_temp_atomic: *mut *const *const gchar = &raw mut safe_c2rust_overlay_dirs;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_overlay_dirs as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut is_setuid: gboolean = (*glib__private__())
            .g_check_setuid
            .expect("non-null function pointer")();
        let mut result: *const *const gchar = ::core::ptr::null::<*const gchar>();
        let mut envvar: *const gchar = ::core::ptr::null::<gchar>();
        envvar = if is_setuid == 0 {
            g_getenv(b"G_RESOURCE_OVERLAYS\0" as *const u8 as *const gchar)
        } else {
            ::core::ptr::null::<gchar>()
        };
        if !envvar.is_null() {
            let mut parts: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
            let mut j: gint = 0;
            parts = g_strsplit(
                envvar,
                G_SEARCHPATH_SEPARATOR_S.as_ptr() as *const gchar,
                0 as gint,
            );
            j = 0 as ::core::ffi::c_int as gint;
            i = j;
            while !(*parts.offset(i as isize)).is_null() {
                let mut part: *mut gchar = *parts.offset(i as isize);
                let mut eq: *mut gchar = ::core::ptr::null_mut::<gchar>();
                eq = strchr(part, '=' as i32) as *mut gchar;
                if eq.is_null() {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"G_RESOURCE_OVERLAYS segment '%s' lacks '='.  Ignoring.\0" as *const u8
                            as *const gchar,
                        part,
                    );
                    g_free(part as gpointer);
                } else if eq == part {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"G_RESOURCE_OVERLAYS segment '%s' lacks path before '='.  Ignoring.\0"
                            as *const u8 as *const gchar,
                        part,
                    );
                    g_free(part as gpointer);
                } else if *eq.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"G_RESOURCE_OVERLAYS segment '%s' lacks path after '='.  Ignoring\0"
                            as *const u8 as *const gchar,
                        part,
                    );
                    g_free(part as gpointer);
                } else if *part.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '/' as i32
                {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"G_RESOURCE_OVERLAYS segment '%s' lacks leading '/'.  Ignoring.\0"
                            as *const u8 as *const gchar,
                        part,
                    );
                    g_free(part as gpointer);
                } else if *eq.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == '/' as i32
                {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"G_RESOURCE_OVERLAYS segment '%s' has trailing '/' before '='.  Ignoring\0"
                            as *const u8 as *const gchar,
                        part,
                    );
                    g_free(part as gpointer);
                } else if g_path_is_absolute(eq.offset(1 as ::core::ffi::c_int as isize)) == 0 {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"G_RESOURCE_OVERLAYS segment '%s' does not have an absolute path after '='.  Ignoring\0"
                            as *const u8 as *const gchar,
                        part,
                    );
                    g_free(part as gpointer);
                } else {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_MESSAGE,
                        b"Adding GResources overlay '%s'\0" as *const u8 as *const gchar,
                        part,
                    );
                    let fresh1 = j;
                    j = j + 1;
                    let ref mut fresh2 = *parts.offset(fresh1 as isize);
                    *fresh2 = part;
                }
                i += 1;
            }
            let ref mut fresh3 = *parts.offset(j as isize);
            *fresh3 = ::core::ptr::null_mut::<gchar>();
            result = parts as *mut *const gchar;
        } else {
            static mut safe_c2rust_empty_strv: [*const gchar; 1] = [::core::ptr::null::<gchar>()];
            result = &raw const safe_c2rust_empty_strv as *const *const gchar;
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_overlay_dirs = result;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_overlay_dirs as *mut ::core::ffi::c_void,
            result as guintptr as gpointer,
        );
    }
    i = 0 as ::core::ffi::c_int as gint;
    while !(*safe_c2rust_overlay_dirs.offset(i as isize)).is_null() {
        let mut src: *const gchar = ::core::ptr::null::<gchar>();
        let mut src_len: gint = 0;
        let mut dst: *const gchar = ::core::ptr::null::<gchar>();
        let mut dst_len: gint = 0;
        let mut candidate: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut eq_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
        src = *safe_c2rust_overlay_dirs.offset(i as isize);
        eq_0 = strchr(src as *const ::core::ffi::c_char, '=' as i32) as *mut gchar;
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if !eq_0.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gresource.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                463 as ::core::ffi::c_int,
                G_STRFUNC,
                b"eq\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        src_len = eq_0.offset_from(src) as ::core::ffi::c_long as gint;
        dst = eq_0.offset(1 as ::core::ffi::c_int as isize);
        if path_len == -(1 as ::core::ffi::c_int) {
            path_len = strlen(path as *const ::core::ffi::c_char) as gint;
        }
        if !(path_len < src_len) {
            if !(memcmp(
                path as *const ::core::ffi::c_void,
                src as *const ::core::ffi::c_void,
                src_len as size_t,
            ) != 0 as ::core::ffi::c_int)
            {
                if !(*path.offset(src_len as isize) as ::core::ffi::c_int != 0
                    && *path.offset(src_len as isize) as ::core::ffi::c_int != '/' as i32)
                {
                    dst_len = strlen(dst as *const ::core::ffi::c_char) as gint;
                    candidate = g_malloc(
                        (dst_len as ::core::ffi::c_int
                            + (path_len as ::core::ffi::c_int - src_len as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_int) as gsize,
                    ) as *mut gchar;
                    memcpy(
                        candidate as *mut ::core::ffi::c_void,
                        dst as *const ::core::ffi::c_void,
                        dst_len as size_t,
                    );
                    memcpy(
                        candidate.offset(dst_len as isize) as *mut ::core::ffi::c_void,
                        path.offset(src_len as isize) as *const ::core::ffi::c_void,
                        (path_len - src_len) as size_t,
                    );
                    *candidate.offset((dst_len + (path_len - src_len)) as isize) =
                        '\0' as i32 as gchar;
                    res = Some(check.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        candidate, user_data
                    );
                    g_free(candidate as gpointer);
                    if res != 0 {
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-resource-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_ref(
    mut resource: *mut GResource,
) -> *mut GResource {
    if 0 as ::core::ffi::c_int != 0 {
        (*resource).ref_count;
        (*resource).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*resource).ref_count, 1 as ::core::ffi::c_int);
    return resource;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_unref(mut resource: *mut GResource) {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*resource).ref_count;
            (*resource).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*resource).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        gvdb_table_free((*resource).table);
        g_free(resource as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_resource_new_from_table(
    mut table: *mut GvdbTable,
) -> *mut GResource {
    let mut resource: *mut GResource = ::core::ptr::null_mut::<GResource>();
    resource = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GResource>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GResource;
    (*resource).ref_count = 1 as ::core::ffi::c_int;
    (*resource).table = table;
    return resource;
}
unsafe extern "C" fn safe_c2rust_g_resource_error_from_gvdb_table_error(
    mut g_resource_error: *mut *mut GError,
    mut gvdb_table_error: *mut GError,
) {
    if g_error_matches(
        gvdb_table_error,
        g_file_error_quark(),
        G_FILE_ERROR_INVAL as ::core::ffi::c_int as gint,
    ) != 0
    {
        g_set_error_literal(
            g_resource_error,
            safe_c2rust_g_resource_error_quark(),
            G_RESOURCE_ERROR_INTERNAL as ::core::ffi::c_int as gint,
            (*gvdb_table_error).message,
        );
    } else {
        g_propagate_error(
            g_resource_error,
            safe_c2rust_g_steal_pointer(&raw mut gvdb_table_error as gpointer) as *mut GError,
        );
    }
    g_clear_error(&raw mut gvdb_table_error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_new_from_data(
    mut data: *mut GBytes,
    mut error: *mut *mut GError,
) -> *mut GResource {
    let mut table: *mut GvdbTable = ::core::ptr::null_mut::<GvdbTable>();
    let mut unref_data: gboolean = FALSE;
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if (g_bytes_get_data(data, ::core::ptr::null_mut::<gsize>()) as usize)
        .wrapping_rem(::core::mem::size_of::<gpointer>() as usize)
        != 0 as usize
    {
        data = g_bytes_new(
            g_bytes_get_data(data, ::core::ptr::null_mut::<gsize>()),
            g_bytes_get_size(data),
        );
        unref_data = TRUE as gboolean;
    }
    table = gvdb_table_new_from_bytes(data, TRUE, &raw mut local_error);
    if unref_data != 0 {
        g_bytes_unref(data);
    }
    if table.is_null() {
        safe_c2rust_g_resource_error_from_gvdb_table_error(
            error,
            safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer) as *mut GError,
        );
        return ::core::ptr::null_mut::<GResource>();
    }
    return safe_c2rust_g_resource_new_from_table(table);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_load(
    mut filename: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GResource {
    let mut table: *mut GvdbTable = ::core::ptr::null_mut::<GvdbTable>();
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    table = gvdb_table_new(filename, FALSE, &raw mut local_error);
    if table.is_null() {
        safe_c2rust_g_resource_error_from_gvdb_table_error(
            error,
            safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer) as *mut GError,
        );
        return ::core::ptr::null_mut::<GResource>();
    }
    return safe_c2rust_g_resource_new_from_table(table);
}
unsafe extern "C" fn safe_c2rust_do_lookup(
    mut resource: *mut GResource,
    mut path: *const gchar,
    mut lookup_flags: GResourceLookupFlags,
    mut size: *mut gsize,
    mut flags: *mut guint32,
    mut data: *mut *const ::core::ffi::c_void,
    mut data_size: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut free_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut path_len: gsize = 0;
    let mut res: gboolean = FALSE;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    path_len = strlen(path as *const ::core::ffi::c_char) as gsize;
    if path_len >= 1 as gsize
        && *path.offset(path_len.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        free_path = safe_c2rust_g_strdup_inline(path as *const ::core::ffi::c_char);
        path = free_path;
        *free_path.offset(path_len.wrapping_sub(1 as gsize) as isize) = 0 as ::core::ffi::c_char;
    }
    value = gvdb_table_get_raw_value((*resource).table, path);
    if value.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_resource_error_quark(),
            G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            g_dgettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0" as *const u8
                    as *const gchar,
            ) as *mut ::core::ffi::c_char,
            path,
        );
    } else {
        let mut _size: guint32 = 0;
        let mut _flags: guint32 = 0;
        let mut array: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        g_variant_get(
            value,
            b"(uu@ay)\0" as *const u8 as *const gchar,
            &raw mut _size,
            &raw mut _flags,
            &raw mut array,
        );
        _size = _size;
        _flags = _flags;
        if !size.is_null() {
            *size = _size as gsize;
        }
        if !flags.is_null() {
            *flags = _flags;
        }
        if !data.is_null() {
            *data = g_variant_get_data(array) as *const ::core::ffi::c_void;
        }
        if !data_size.is_null() {
            if _flags & G_RESOURCE_FLAGS_COMPRESSED as ::core::ffi::c_int as guint32 != 0 {
                *data_size = g_variant_get_size(array);
            } else {
                *data_size = g_variant_get_size(array).wrapping_sub(1 as gsize);
            }
        }
        g_variant_unref(array);
        g_variant_unref(value);
        res = TRUE as gboolean;
    }
    g_free(free_path as gpointer);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_open_stream(
    mut resource: *mut GResource,
    mut path: *const gchar,
    mut lookup_flags: GResourceLookupFlags,
    mut error: *mut *mut GError,
) -> *mut GInputStream {
    let mut data: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    let mut data_size: gsize = 0;
    let mut flags: guint32 = 0;
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut stream2: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    if safe_c2rust_do_lookup(
        resource,
        path,
        lookup_flags,
        ::core::ptr::null_mut::<gsize>(),
        &raw mut flags,
        &raw mut data,
        &raw mut data_size,
        error,
    ) == 0
    {
        return ::core::ptr::null_mut::<GInputStream>();
    }
    stream = g_memory_input_stream_new_from_data(data, data_size as gssize, None);
    g_object_set_data_full(
        stream as *mut ::core::ffi::c_void as *mut GObject,
        b"g-resource\0" as *const u8 as *const gchar,
        safe_c2rust_g_resource_ref(resource) as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GResource) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_g_resource_unref as unsafe extern "C" fn(*mut GResource) -> ()),
        ),
    );
    if flags & G_RESOURCE_FLAGS_COMPRESSED as ::core::ffi::c_int as guint32 != 0 {
        let mut decompressor: *mut GZlibDecompressor =
            g_zlib_decompressor_new(G_ZLIB_COMPRESSOR_FORMAT_ZLIB);
        stream2 = g_converter_input_stream_new(
            stream,
            decompressor as *mut ::core::ffi::c_void as *mut GConverter,
        );
        g_object_unref(decompressor as gpointer);
        g_object_unref(stream as gpointer);
        stream = stream2;
    }
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_lookup_data(
    mut resource: *mut GResource,
    mut path: *const gchar,
    mut lookup_flags: GResourceLookupFlags,
    mut error: *mut *mut GError,
) -> *mut GBytes {
    let mut data: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    let mut flags: guint32 = 0;
    let mut data_size: gsize = 0;
    let mut size: gsize = 0;
    if safe_c2rust_do_lookup(
        resource,
        path,
        lookup_flags,
        &raw mut size,
        &raw mut flags,
        &raw mut data,
        &raw mut data_size,
        error,
    ) == 0
    {
        return ::core::ptr::null_mut::<GBytes>();
    }
    if size == 0 as gsize {
        return g_bytes_new_with_free_func(
            b"\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
            0 as gsize,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GResource) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_resource_unref as unsafe extern "C" fn(*mut GResource) -> (),
            )),
            safe_c2rust_g_resource_ref(resource) as gpointer,
        );
    } else if flags & G_RESOURCE_FLAGS_COMPRESSED as ::core::ffi::c_int as guint32 != 0 {
        let mut uncompressed: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut d: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut res: GConverterResult = G_CONVERTER_ERROR;
        let mut d_size: gsize = 0;
        let mut s_size: gsize = 0;
        let mut bytes_read: gsize = 0;
        let mut bytes_written: gsize = 0;
        let mut decompressor: *mut GZlibDecompressor =
            g_zlib_decompressor_new(G_ZLIB_COMPRESSOR_FORMAT_ZLIB);
        uncompressed = g_malloc(size.wrapping_add(1 as gsize)) as *mut ::core::ffi::c_char;
        s = data as *const ::core::ffi::c_char;
        s_size = data_size;
        d = uncompressed;
        d_size = size;
        loop {
            res = g_converter_convert(
                decompressor as *mut ::core::ffi::c_void as *mut GConverter,
                s as *const ::core::ffi::c_void,
                s_size,
                d as *mut ::core::ffi::c_void,
                d_size,
                G_CONVERTER_INPUT_AT_END,
                &raw mut bytes_read,
                &raw mut bytes_written,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            if res as ::core::ffi::c_uint
                == G_CONVERTER_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                g_free(uncompressed as gpointer);
                g_object_unref(decompressor as gpointer);
                g_set_error(
                    error,
                    safe_c2rust_g_resource_error_quark(),
                    G_RESOURCE_ERROR_INTERNAL as ::core::ffi::c_int as gint,
                    g_dgettext(
                        GETTEXT_PACKAGE.as_ptr() as *const gchar,
                        b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D failed to decompress\0"
                            as *const u8 as *const gchar,
                    ) as *mut ::core::ffi::c_char,
                    path,
                );
                return ::core::ptr::null_mut::<GBytes>();
            }
            s = s.offset(bytes_read as isize);
            s_size = s_size.wrapping_sub(bytes_read);
            d = d.offset(bytes_written as isize);
            d_size = d_size.wrapping_sub(bytes_written);
            if !(res as ::core::ffi::c_uint
                != G_CONVERTER_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                break;
            }
        }
        *uncompressed.offset(size as isize) = 0 as ::core::ffi::c_char;
        g_object_unref(decompressor as gpointer);
        return g_bytes_new_take(uncompressed as gpointer, size);
    } else {
        return g_bytes_new_with_free_func(
            data as gconstpointer,
            data_size,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GResource) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_resource_unref as unsafe extern "C" fn(*mut GResource) -> (),
            )),
            safe_c2rust_g_resource_ref(resource) as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_get_info(
    mut resource: *mut GResource,
    mut path: *const gchar,
    mut lookup_flags: GResourceLookupFlags,
    mut size: *mut gsize,
    mut flags: *mut guint32,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_do_lookup(
        resource,
        path,
        lookup_flags,
        size,
        flags,
        ::core::ptr::null_mut::<*const ::core::ffi::c_void>(),
        ::core::ptr::null_mut::<gsize>(),
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_enumerate_children(
    mut resource: *mut GResource,
    mut path: *const gchar,
    mut lookup_flags: GResourceLookupFlags,
    mut error: *mut *mut GError,
) -> *mut *mut ::core::ffi::c_char {
    let mut local_str: [gchar; 256] = [0; 256];
    let mut path_with_slash: *const gchar = ::core::ptr::null::<gchar>();
    let mut children: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut free_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut path_len: gsize = 0;
    if *path as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if !error.is_null() {
            g_set_error(
                error,
                safe_c2rust_g_resource_error_quark(),
                G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                g_dgettext(
                    GETTEXT_PACKAGE.as_ptr() as *const gchar,
                    b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0" as *const u8
                        as *const gchar,
                ) as *mut ::core::ffi::c_char,
                path,
            );
        }
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    path_len = strlen(path as *const ::core::ffi::c_char) as gsize;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if *path.offset(path_len.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
            != '/' as i32
        {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
        if (path_len as usize)
            < (::core::mem::size_of::<[gchar; 256]>() as usize).wrapping_sub(2 as usize)
        {
            memcpy(
                &raw mut local_str as *mut gchar as *mut ::core::ffi::c_void,
                path as *const ::core::ffi::c_void,
                path_len as size_t,
            );
            local_str[path_len as usize] = '/' as i32 as gchar;
            local_str[path_len.wrapping_add(1 as gsize) as usize] = 0 as gchar;
            path_with_slash = &raw mut local_str as *mut gchar;
        } else {
            free_path = g_strconcat(
                path,
                b"/\0" as *const u8 as *const ::core::ffi::c_char,
                NULL_0,
            );
            path_with_slash = free_path;
        }
    } else {
        path_with_slash = path;
    }
    children = gvdb_table_list((*resource).table, path_with_slash);
    g_free(free_path as gpointer);
    if children.is_null() {
        if !error.is_null() {
            g_set_error(
                error,
                safe_c2rust_g_resource_error_quark(),
                G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                g_dgettext(
                    GETTEXT_PACKAGE.as_ptr() as *const gchar,
                    b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0" as *const u8
                        as *const gchar,
                ) as *mut ::core::ffi::c_char,
                path,
            );
        }
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    return children as *mut *mut ::core::ffi::c_char;
}
static mut safe_c2rust_resources_lock: GRWLock = _GRWLock {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    i: [0; 2],
};
static mut safe_c2rust_registered_resources: *mut GList =
    ::core::ptr::null::<GList>() as *mut GList;
static mut safe_c2rust_lazy_register_resources: *mut GStaticResource =
    ::core::ptr::null::<GStaticResource>() as *mut GStaticResource;
unsafe extern "C" fn safe_c2rust_g_resources_register_unlocked(mut resource: *mut GResource) {
    safe_c2rust_registered_resources = g_list_prepend(
        safe_c2rust_registered_resources,
        safe_c2rust_g_resource_ref(resource) as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_resources_unregister_unlocked(mut resource: *mut GResource) {
    if g_list_find(safe_c2rust_registered_resources, resource as gconstpointer).is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Tried to remove not registered resource\0" as *const u8 as *const gchar,
        );
    } else {
        safe_c2rust_registered_resources =
            g_list_remove(safe_c2rust_registered_resources, resource as gconstpointer);
        safe_c2rust_g_resource_unref(resource);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resources_register(mut resource: *mut GResource) {
    g_rw_lock_writer_lock(&raw mut safe_c2rust_resources_lock);
    safe_c2rust_g_resources_register_unlocked(resource);
    g_rw_lock_writer_unlock(&raw mut safe_c2rust_resources_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resources_unregister(mut resource: *mut GResource) {
    g_rw_lock_writer_lock(&raw mut safe_c2rust_resources_lock);
    safe_c2rust_g_resources_unregister_unlocked(resource);
    g_rw_lock_writer_unlock(&raw mut safe_c2rust_resources_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resources_open_stream(
    mut path: *const gchar,
    mut lookup_flags: GResourceLookupFlags,
    mut error: *mut *mut GError,
) -> *mut GInputStream {
    let mut res: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    if safe_c2rust_g_resource_find_overlay(
        path,
        Some(
            safe_c2rust_open_overlay_stream
                as unsafe extern "C" fn(*const gchar, gpointer) -> gboolean,
        ),
        &raw mut res as gpointer,
    ) != 0
    {
        return res;
    }
    safe_c2rust_register_lazy_static_resources();
    g_rw_lock_reader_lock(&raw mut safe_c2rust_resources_lock);
    l = safe_c2rust_registered_resources;
    while !l.is_null() {
        let mut r: *mut GResource = (*l).data as *mut GResource;
        let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
        stream = safe_c2rust_g_resource_open_stream(r, path, lookup_flags, &raw mut my_error);
        if stream.is_null()
            && g_error_matches(
                my_error,
                safe_c2rust_g_resource_error_quark(),
                G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            ) != 0
        {
            g_clear_error(&raw mut my_error);
            l = (*l).next;
        } else {
            if stream.is_null() {
                g_propagate_error(error, my_error);
            }
            res = stream;
            break;
        }
    }
    if l.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_resource_error_quark(),
            G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            g_dgettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0" as *const u8
                    as *const gchar,
            ) as *mut ::core::ffi::c_char,
            path,
        );
    }
    g_rw_lock_reader_unlock(&raw mut safe_c2rust_resources_lock);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resources_lookup_data(
    mut path: *const gchar,
    mut lookup_flags: GResourceLookupFlags,
    mut error: *mut *mut GError,
) -> *mut GBytes {
    let mut res: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut data: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    if safe_c2rust_g_resource_find_overlay(
        path,
        Some(
            safe_c2rust_get_overlay_bytes
                as unsafe extern "C" fn(*const gchar, gpointer) -> gboolean,
        ),
        &raw mut res as gpointer,
    ) != 0
    {
        return res;
    }
    safe_c2rust_register_lazy_static_resources();
    g_rw_lock_reader_lock(&raw mut safe_c2rust_resources_lock);
    l = safe_c2rust_registered_resources;
    while !l.is_null() {
        let mut r: *mut GResource = (*l).data as *mut GResource;
        let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
        data = safe_c2rust_g_resource_lookup_data(r, path, lookup_flags, &raw mut my_error);
        if data.is_null()
            && g_error_matches(
                my_error,
                safe_c2rust_g_resource_error_quark(),
                G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            ) != 0
        {
            g_clear_error(&raw mut my_error);
            l = (*l).next;
        } else {
            if data.is_null() {
                g_propagate_error(error, my_error);
            }
            res = data;
            break;
        }
    }
    if l.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_resource_error_quark(),
            G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            g_dgettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0" as *const u8
                    as *const gchar,
            ) as *mut ::core::ffi::c_char,
            path,
        );
    }
    g_rw_lock_reader_unlock(&raw mut safe_c2rust_resources_lock);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resources_enumerate_children(
    mut path: *const gchar,
    mut lookup_flags: GResourceLookupFlags,
    mut error: *mut *mut GError,
) -> *mut *mut ::core::ffi::c_char {
    let mut hash: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut children: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    safe_c2rust_g_resource_find_overlay(
        path,
        Some(
            safe_c2rust_enumerate_overlay_dir
                as unsafe extern "C" fn(*const gchar, gpointer) -> gboolean,
        ),
        &raw mut hash as gpointer,
    );
    safe_c2rust_register_lazy_static_resources();
    g_rw_lock_reader_lock(&raw mut safe_c2rust_resources_lock);
    l = safe_c2rust_registered_resources;
    while !l.is_null() {
        let mut r: *mut GResource = (*l).data as *mut GResource;
        children = safe_c2rust_g_resource_enumerate_children(
            r,
            path,
            G_RESOURCE_LOOKUP_FLAGS_NONE,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !children.is_null() {
            if hash.is_null() {
                hash = g_hash_table_new_full(
                    Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                    Some(
                        g_str_equal
                            as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
                    ),
                    Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                    None,
                );
            }
            i = 0 as ::core::ffi::c_int;
            while !(*children.offset(i as isize)).is_null() {
                g_hash_table_add(hash, *children.offset(i as isize) as gpointer);
                i += 1;
            }
            g_free(children as gpointer);
        }
        l = (*l).next;
    }
    g_rw_lock_reader_unlock(&raw mut safe_c2rust_resources_lock);
    if hash.is_null() {
        if !error.is_null() {
            g_set_error(
                error,
                safe_c2rust_g_resource_error_quark(),
                G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                g_dgettext(
                    GETTEXT_PACKAGE.as_ptr() as *const gchar,
                    b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0" as *const u8
                        as *const gchar,
                ) as *mut ::core::ffi::c_char,
                path,
            );
        }
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    } else {
        children = g_hash_table_get_keys_as_array(hash, ::core::ptr::null_mut::<guint>())
            as *mut *mut gchar as *mut *mut ::core::ffi::c_char;
        g_hash_table_steal_all(hash);
        g_hash_table_destroy(hash);
        return children;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resources_get_info(
    mut path: *const gchar,
    mut lookup_flags: GResourceLookupFlags,
    mut size: *mut gsize,
    mut flags: *mut guint32,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut res: gboolean = FALSE;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut r_res: gboolean = 0;
    let mut info: InfoData = InfoData { size: 0, flags: 0 };
    if safe_c2rust_g_resource_find_overlay(
        path,
        Some(
            safe_c2rust_get_overlay_info
                as unsafe extern "C" fn(*const gchar, gpointer) -> gboolean,
        ),
        &raw mut info as gpointer,
    ) != 0
    {
        if !size.is_null() {
            *size = info.size;
        }
        if !flags.is_null() {
            *flags = info.flags;
        }
        return TRUE;
    }
    safe_c2rust_register_lazy_static_resources();
    g_rw_lock_reader_lock(&raw mut safe_c2rust_resources_lock);
    l = safe_c2rust_registered_resources;
    while !l.is_null() {
        let mut r: *mut GResource = (*l).data as *mut GResource;
        let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
        r_res =
            safe_c2rust_g_resource_get_info(r, path, lookup_flags, size, flags, &raw mut my_error);
        if r_res == 0
            && g_error_matches(
                my_error,
                safe_c2rust_g_resource_error_quark(),
                G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            ) != 0
        {
            g_clear_error(&raw mut my_error);
            l = (*l).next;
        } else {
            if r_res == 0 {
                g_propagate_error(error, my_error);
            }
            res = r_res;
            break;
        }
    }
    if l.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_resource_error_quark(),
            G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            g_dgettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0" as *const u8
                    as *const gchar,
            ) as *mut ::core::ffi::c_char,
            path,
        );
    }
    g_rw_lock_reader_unlock(&raw mut safe_c2rust_resources_lock);
    return res;
}
unsafe extern "C" fn safe_c2rust_register_lazy_static_resources_unlocked() {
    let mut list: *mut GStaticResource = ({
        let mut gapg_temp_newval: *mut GStaticResource = ::core::ptr::null_mut::<GStaticResource>();
        let mut gapg_temp_atomic: *mut *mut GStaticResource =
            &raw mut safe_c2rust_lazy_register_resources;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    });
    while ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_lazy_register_resources;
        } else {
        };
        if 0 as ::core::ffi::c_int != 0 {
            list;
        } else {
        };
        list = list;
        let fresh0 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
            &raw mut safe_c2rust_lazy_register_resources,
            *&raw mut list,
            ::core::ptr::null_mut::<GStaticResource>(),
        );
        *&raw mut list = fresh0.0;
        if fresh0.1 as ::core::ffi::c_int != 0 {
            TRUE
        } else {
            FALSE
        }
    }) == 0
    {}
    while !list.is_null() {
        let mut bytes: *mut GBytes =
            g_bytes_new_static((*list).data as gconstpointer, (*list).data_len);
        let mut resource: *mut GResource =
            safe_c2rust_g_resource_new_from_data(bytes, ::core::ptr::null_mut::<*mut GError>());
        if !resource.is_null() {
            safe_c2rust_g_resources_register_unlocked(resource);
            let mut gaps_temp_atomic: *mut *mut GResource = &raw mut (*list).resource;
            let mut gaps_temp_newval: *mut GResource = resource as *mut GResource;
            if 0 as ::core::ffi::c_int != 0 {
                (*list).resource;
            } else {
            };
            crate::translated::compat::atomic_store_seqcst(gaps_temp_atomic, *&raw mut gaps_temp_newval);
        }
        g_bytes_unref(bytes);
        list = (*list).next;
    }
}
unsafe extern "C" fn safe_c2rust_register_lazy_static_resources() {
    if ({
        let mut gapg_temp_newval: *mut GStaticResource = ::core::ptr::null_mut::<GStaticResource>();
        let mut gapg_temp_atomic: *mut *mut GStaticResource =
            &raw mut safe_c2rust_lazy_register_resources;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    })
    .is_null()
    {
        return;
    }
    g_rw_lock_writer_lock(&raw mut safe_c2rust_resources_lock);
    safe_c2rust_register_lazy_static_resources_unlocked();
    g_rw_lock_writer_unlock(&raw mut safe_c2rust_resources_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_resource_init(
    mut static_resource: *mut GStaticResource,
) {
    let mut next: *mut GStaticResource = ::core::ptr::null_mut::<GStaticResource>();
    loop {
        next = ({
            let mut gapg_temp_newval: *mut GStaticResource =
                ::core::ptr::null_mut::<GStaticResource>();
            let mut gapg_temp_atomic: *mut *mut GStaticResource =
                &raw mut safe_c2rust_lazy_register_resources;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) as *mut GStaticResource;
        (*static_resource).next = next;
        if !(({
            let mut gapcae_oldval: gpointer = next as gpointer;
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_lazy_register_resources;
            } else {
            };
            let fresh4 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut safe_c2rust_lazy_register_resources,
                *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut *mut GStaticResource),
                static_resource,
            );
            *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut *mut GStaticResource) =
                fresh4.0;
            if fresh4.1 as ::core::ffi::c_int != 0 {
                TRUE
            } else {
                FALSE
            }
        }) == 0)
        {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_resource_fini(
    mut static_resource: *mut GStaticResource,
) {
    let mut resource: *mut GResource = ::core::ptr::null_mut::<GResource>();
    g_rw_lock_writer_lock(&raw mut safe_c2rust_resources_lock);
    safe_c2rust_register_lazy_static_resources_unlocked();
    resource = ({
        if 0 as ::core::ffi::c_int != 0 {
            (*static_resource).resource;
        } else {
        };
        crate::translated::compat::atomic_xchg_seqcst(
            &raw mut (*static_resource).resource,
            ::core::ptr::null_mut::<GResource>(),
        ) as gpointer
    }) as *mut GResource;
    if !resource.is_null() {
        if ({
            let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
            if ({
                let mut gaig_temp: gint = 0;
                if 0 as ::core::ffi::c_int != 0 {
                    (*resource).ref_count;
                    (*resource).ref_count;
                } else {
                };
                *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                    &raw mut (*resource).ref_count as *mut gint,
                );
                gaig_temp
            }) >= 2 as ::core::ffi::c_int
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gresource.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1473 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_atomic_int_get (&resource->ref_count) >= 2\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        safe_c2rust_g_resources_unregister_unlocked(resource);
        safe_c2rust_g_resource_unref(resource);
    }
    g_rw_lock_writer_unlock(&raw mut safe_c2rust_resources_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_static_resource_get_resource(
    mut static_resource: *mut GStaticResource,
) -> *mut GResource {
    safe_c2rust_register_lazy_static_resources();
    return ({
        let mut gapg_temp_newval: *mut GResource = ::core::ptr::null_mut::<GResource>();
        let mut gapg_temp_atomic: *mut *mut GResource = &raw mut (*static_resource).resource;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    });
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const GETTEXT_PACKAGE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"glib20\0") };
