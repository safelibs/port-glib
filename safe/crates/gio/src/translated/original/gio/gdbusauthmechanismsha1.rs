extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GMainContext;
    pub type _GDBusAuthMechanismPrivate;
    pub type _GWakeup;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_clear_error(err: *mut *mut GError);
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_propagate_prefixed_error(
        dest: *mut *mut GError,
        src: *mut GError,
        format: *const gchar,
        ...
    );
    fn g_get_home_dir() -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getuid() -> __uid_t;
    fn g_compute_checksum_for_string(
        checksum_type: GChecksumType,
        str: *const gchar,
        length: gssize,
    ) -> *mut gchar;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_file_error_quark() -> GQuark;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_set_contents_full(
        filename: *const gchar,
        contents: *const gchar,
        length: gssize,
        flags: GFileSetContentsFlags,
        mode: ::core::ffi::c_int,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_mkdir_with_parents(pathname: *const gchar, mode: gint) -> gint;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_get_real_time() -> gint64;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_ascii_strtoll(nptr: *const gchar, endptr: *mut *mut gchar, base: guint) -> gint64;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_printerr(format: *const gchar, ...);
    fn g_random_int_range(begin: gint32, end: gint32) -> gint32;
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
    fn g_usleep(microseconds: gulong);
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn statx(
        __dirfd: ::core::ffi::c_int,
        __path: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
        __mask: ::core::ffi::c_uint,
        __buf: *mut statx,
    ) -> ::core::ffi::c_int;
    fn g_unlink(filename: *const gchar) -> ::core::ffi::c_int;
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
    fn _g_dbus_auth_mechanism_get_type() -> GType;
    fn glib__private__() -> *const GLibPrivateVTable;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn _g_dbus_hexencode(str: *const gchar, str_len: gsize) -> *mut gchar;
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
pub type guint8 = ::core::ffi::c_uchar;
pub type gint32 = ::core::ffi::c_int;
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
pub type gushort = ::core::ffi::c_ushort;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type GChecksumType = ::core::ffi::c_uint;
pub const G_CHECKSUM_SHA384: GChecksumType = 4;
pub const G_CHECKSUM_SHA512: GChecksumType = 3;
pub const G_CHECKSUM_SHA256: GChecksumType = 2;
pub const G_CHECKSUM_SHA1: GChecksumType = 1;
pub const G_CHECKSUM_MD5: GChecksumType = 0;
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
pub type GFileSetContentsFlags = ::core::ffi::c_uint;
pub const G_FILE_SET_CONTENTS_ONLY_EXISTING: GFileSetContentsFlags = 4;
pub const G_FILE_SET_CONTENTS_DURABLE: GFileSetContentsFlags = 2;
pub const G_FILE_SET_CONTENTS_CONSISTENT: GFileSetContentsFlags = 1;
pub const G_FILE_SET_CONTENTS_NONE: GFileSetContentsFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
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
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type __u16 = ::core::ffi::c_ushort;
pub type __s32 = ::core::ffi::c_int;
pub type __u32 = ::core::ffi::c_uint;
pub type __s64 = ::core::ffi::c_longlong;
pub type __u64 = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statx {
    pub stx_mask: __u32,
    pub stx_blksize: __u32,
    pub stx_attributes: __u64,
    pub stx_nlink: __u32,
    pub stx_uid: __u32,
    pub stx_gid: __u32,
    pub stx_mode: __u16,
    pub __spare0: [__u16; 1],
    pub stx_ino: __u64,
    pub stx_size: __u64,
    pub stx_blocks: __u64,
    pub stx_attributes_mask: __u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __u32,
    pub stx_rdev_minor: __u32,
    pub stx_dev_major: __u32,
    pub stx_dev_minor: __u32,
    pub stx_mnt_id: __u64,
    pub stx_dio_mem_align: __u32,
    pub stx_dio_offset_align: __u32,
    pub __spare3: [__u64; 12],
}
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
pub type GIOErrorEnum = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: GIOErrorEnum = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: GIOErrorEnum = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: GIOErrorEnum = 46;
pub const G_IO_ERROR_NOT_CONNECTED: GIOErrorEnum = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: GIOErrorEnum = 44;
pub const G_IO_ERROR_BROKEN_PIPE: GIOErrorEnum = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: GIOErrorEnum = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: GIOErrorEnum = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: GIOErrorEnum = 41;
pub const G_IO_ERROR_PROXY_FAILED: GIOErrorEnum = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: GIOErrorEnum = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: GIOErrorEnum = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: GIOErrorEnum = 37;
pub const G_IO_ERROR_DBUS_ERROR: GIOErrorEnum = 36;
pub const G_IO_ERROR_INVALID_DATA: GIOErrorEnum = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: GIOErrorEnum = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: GIOErrorEnum = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: GIOErrorEnum = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: GIOErrorEnum = 31;
pub const G_IO_ERROR_FAILED_HANDLED: GIOErrorEnum = 30;
pub const G_IO_ERROR_WOULD_MERGE: GIOErrorEnum = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: GIOErrorEnum = 28;
pub const G_IO_ERROR_WOULD_BLOCK: GIOErrorEnum = 27;
pub const G_IO_ERROR_BUSY: GIOErrorEnum = 26;
pub const G_IO_ERROR_WOULD_RECURSE: GIOErrorEnum = 25;
pub const G_IO_ERROR_TIMED_OUT: GIOErrorEnum = 24;
pub const G_IO_ERROR_WRONG_ETAG: GIOErrorEnum = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: GIOErrorEnum = 22;
pub const G_IO_ERROR_READ_ONLY: GIOErrorEnum = 21;
pub const G_IO_ERROR_PENDING: GIOErrorEnum = 20;
pub const G_IO_ERROR_CANCELLED: GIOErrorEnum = 19;
pub const G_IO_ERROR_CLOSED: GIOErrorEnum = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: GIOErrorEnum = 17;
pub const G_IO_ERROR_NOT_MOUNTED: GIOErrorEnum = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: GIOErrorEnum = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: GIOErrorEnum = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: GIOErrorEnum = 13;
pub const G_IO_ERROR_NO_SPACE: GIOErrorEnum = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: GIOErrorEnum = 11;
pub const G_IO_ERROR_INVALID_FILENAME: GIOErrorEnum = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: GIOErrorEnum = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: GIOErrorEnum = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: GIOErrorEnum = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: GIOErrorEnum = 6;
pub const G_IO_ERROR_NOT_EMPTY: GIOErrorEnum = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: GIOErrorEnum = 4;
pub const G_IO_ERROR_IS_DIRECTORY: GIOErrorEnum = 3;
pub const G_IO_ERROR_EXISTS: GIOErrorEnum = 2;
pub const G_IO_ERROR_NOT_FOUND: GIOErrorEnum = 1;
pub const G_IO_ERROR_FAILED: GIOErrorEnum = 0;
pub type GDBusConnectionFlags = ::core::ffi::c_uint;
pub const G_DBUS_CONNECTION_FLAGS_CROSS_NAMESPACE: GDBusConnectionFlags = 64;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER: GDBusConnectionFlags = 32;
pub const G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING: GDBusConnectionFlags = 16;
pub const G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION: GDBusConnectionFlags = 8;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS: GDBusConnectionFlags = 4;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER: GDBusConnectionFlags = 2;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT: GDBusConnectionFlags = 1;
pub const G_DBUS_CONNECTION_FLAGS_NONE: GDBusConnectionFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanism {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusAuthMechanismPrivate,
}
pub type GDBusAuthMechanismPrivate = _GDBusAuthMechanismPrivate;
pub type GDBusAuthMechanism = _GDBusAuthMechanism;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanismClass {
    pub parent_class: GObjectClass,
    pub get_priority: Option<unsafe extern "C" fn() -> gint>,
    pub get_name: Option<unsafe extern "C" fn() -> *const gchar>,
    pub is_supported: Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> gboolean>,
    pub encode_data: Option<
        unsafe extern "C" fn(
            *mut GDBusAuthMechanism,
            *const gchar,
            gsize,
            *mut gsize,
        ) -> *mut gchar,
    >,
    pub decode_data: Option<
        unsafe extern "C" fn(
            *mut GDBusAuthMechanism,
            *const gchar,
            gsize,
            *mut gsize,
        ) -> *mut gchar,
    >,
    pub server_get_state:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState>,
    pub server_initiate:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>,
    pub server_data_receive:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>,
    pub server_data_send:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar>,
    pub server_get_reject_reason:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> *mut gchar>,
    pub server_shutdown: Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> ()>,
    pub client_get_state:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState>,
    pub client_initiate: Option<
        unsafe extern "C" fn(
            *mut GDBusAuthMechanism,
            GDBusConnectionFlags,
            *mut gsize,
        ) -> *mut gchar,
    >,
    pub client_data_receive:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>,
    pub client_data_send:
        Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar>,
    pub client_shutdown: Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> ()>,
}
pub type GDBusAuthMechanismState = ::core::ffi::c_uint;
pub const G_DBUS_AUTH_MECHANISM_STATE_ACCEPTED: GDBusAuthMechanismState = 4;
pub const G_DBUS_AUTH_MECHANISM_STATE_REJECTED: GDBusAuthMechanismState = 3;
pub const G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND: GDBusAuthMechanismState = 2;
pub const G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA: GDBusAuthMechanismState = 1;
pub const G_DBUS_AUTH_MECHANISM_STATE_INVALID: GDBusAuthMechanismState = 0;
pub type GDBusAuthMechanismClass = _GDBusAuthMechanismClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanismSha1 {
    pub parent_instance: GDBusAuthMechanism,
    pub priv_0: *mut GDBusAuthMechanismSha1Private,
}
pub type GDBusAuthMechanismSha1Private = _GDBusAuthMechanismSha1Private;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanismSha1Private {
    pub is_client: gboolean,
    pub is_server: gboolean,
    pub state: GDBusAuthMechanismState,
    pub reject_reason: *mut gchar,
    pub to_send: *mut gchar,
    pub cookie: *mut gchar,
    pub server_challenge: *mut gchar,
}
pub type GDBusAuthMechanismSha1 = _GDBusAuthMechanismSha1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuthMechanismSha1Class {
    pub parent_class: GDBusAuthMechanismClass,
}
pub type GDBusAuthMechanismSha1Class = _GDBusAuthMechanismSha1Class;
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
pub type GLocalFileStatField = ::core::ffi::c_uint;
pub const G_LOCAL_FILE_STAT_FIELD_BTIME: GLocalFileStatField = 2048;
pub const G_LOCAL_FILE_STAT_FIELD_BLOCKS: GLocalFileStatField = 1024;
pub const G_LOCAL_FILE_STAT_FIELD_SIZE: GLocalFileStatField = 512;
pub const G_LOCAL_FILE_STAT_FIELD_INO: GLocalFileStatField = 256;
pub const G_LOCAL_FILE_STAT_FIELD_CTIME: GLocalFileStatField = 128;
pub const G_LOCAL_FILE_STAT_FIELD_MTIME: GLocalFileStatField = 64;
pub const G_LOCAL_FILE_STAT_FIELD_ATIME: GLocalFileStatField = 32;
pub const G_LOCAL_FILE_STAT_FIELD_GID: GLocalFileStatField = 16;
pub const G_LOCAL_FILE_STAT_FIELD_UID: GLocalFileStatField = 8;
pub const G_LOCAL_FILE_STAT_FIELD_NLINK: GLocalFileStatField = 4;
pub const G_LOCAL_FILE_STAT_FIELD_MODE: GLocalFileStatField = 2;
pub const G_LOCAL_FILE_STAT_FIELD_TYPE: GLocalFileStatField = 1;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const AT_FDCWD: ::core::ffi::c_int = -(100 as ::core::ffi::c_int);
pub const AT_NO_AUTOMOUNT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const AT_STATX_SYNC_AS_STAT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
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
pub const G_USEC_PER_SEC: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
pub const STATX_ALL: ::core::ffi::c_uint = 0xfff as ::core::ffi::c_uint;
pub const G_LOCAL_FILE_STAT_FIELD_ALL: ::core::ffi::c_uint = STATX_ALL;
#[inline]
unsafe extern "C" fn safe_c2rust_g_local_file_statx(
    mut dirfd: ::core::ffi::c_int,
    mut pathname: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mask: GLocalFileStatField,
    mut mask_required: GLocalFileStatField,
    mut stat_buf: *mut statx,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 0;
    mask_required = ::core::mem::transmute::<::core::ffi::c_uint, GLocalFileStatField>(
        mask_required as ::core::ffi::c_uint & mask as ::core::ffi::c_uint,
    );
    retval = statx(
        dirfd,
        pathname,
        flags,
        mask as ::core::ffi::c_uint,
        stat_buf,
    );
    if retval == 0 as ::core::ffi::c_int
        && (*stat_buf).stx_mask as ::core::ffi::c_uint & mask_required as ::core::ffi::c_uint
            != mask_required as ::core::ffi::c_uint
    {
        *__errno_location() = ERANGE;
        return -(1 as ::core::ffi::c_int);
    }
    return retval;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_local_file_stat(
    mut path: *const ::core::ffi::c_char,
    mut mask: GLocalFileStatField,
    mut mask_required: GLocalFileStatField,
    mut stat_buf: *mut statx,
) -> ::core::ffi::c_int {
    return safe_c2rust_g_local_file_statx(
        AT_FDCWD,
        path,
        AT_NO_AUTOMOUNT | AT_STATX_SYNC_AS_STAT,
        mask,
        mask_required,
        stat_buf,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_mtime(mut buf: *const statx) -> gint64 {
    return (*buf).stx_mtime.tv_sec as gint64;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_mtim_nsec(mut buf: *const statx) -> guint32 {
    return (*buf).stx_mtime.tv_nsec as guint32;
}
pub const NEW_KEY_TIMEOUT_SECONDS: ::core::ffi::c_int =
    60 as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
pub const EXPIRE_KEYS_TIMEOUT_SECONDS: ::core::ffi::c_int =
    NEW_KEY_TIMEOUT_SECONDS + 60 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
pub const MAX_TIME_TRAVEL_SECONDS: ::core::ffi::c_int =
    60 as ::core::ffi::c_int * 5 as ::core::ffi::c_int;
static mut safe_c2rust__g_dbus_auth_mechanism_sha1_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_sha1_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        _g_dbus_auth_mechanism_get_type(),
        g_intern_static_string(b"GDBusAuthMechanismSha1\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusAuthMechanismSha1Class>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_dbus_auth_mechanism_sha1_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusAuthMechanismSha1>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusAuthMechanismSha1) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust__g_dbus_auth_mechanism_sha1_init
                    as unsafe extern "C" fn(*mut GDBusAuthMechanismSha1) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDBusAuthMechanismSha1_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDBusAuthMechanismSha1Private>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_sha1_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type_once();
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
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_sha1_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust__g_dbus_auth_mechanism_sha1_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusAuthMechanismSha1_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusAuthMechanismSha1_private_offset,
        );
    }
    safe_c2rust__g_dbus_auth_mechanism_sha1_class_init(klass as *mut GDBusAuthMechanismSha1Class);
}
static mut safe_c2rust_GDBusAuthMechanismSha1_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_sha1_get_instance_private(
    mut self_0: *mut GDBusAuthMechanismSha1,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDBusAuthMechanismSha1_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_sha1_finalize(mut object: *mut GObject) {
    let mut mechanism: *mut GDBusAuthMechanismSha1 =
        object as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    g_free((*(*mechanism).priv_0).reject_reason as gpointer);
    g_free((*(*mechanism).priv_0).to_send as gpointer);
    g_free((*(*mechanism).priv_0).cookie as gpointer);
    g_free((*(*mechanism).priv_0).server_challenge as gpointer);
    if (*(safe_c2rust__g_dbus_auth_mechanism_sha1_parent_class as *mut GObjectClass))
        .finalize
        .is_some()
    {
        (*(safe_c2rust__g_dbus_auth_mechanism_sha1_parent_class as *mut GObjectClass))
            .finalize
            .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_sha1_class_init(
    mut klass: *mut GDBusAuthMechanismSha1Class,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut mechanism_class: *mut GDBusAuthMechanismClass =
        ::core::ptr::null_mut::<GDBusAuthMechanismClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust__g_dbus_auth_mechanism_sha1_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    mechanism_class = klass as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismClass;
    (*mechanism_class).get_priority =
        Some(safe_c2rust_mechanism_get_priority as unsafe extern "C" fn() -> gint)
            as Option<unsafe extern "C" fn() -> gint>;
    (*mechanism_class).get_name =
        Some(safe_c2rust_mechanism_get_name as unsafe extern "C" fn() -> *const gchar)
            as Option<unsafe extern "C" fn() -> *const gchar>;
    (*mechanism_class).is_supported = Some(
        safe_c2rust_mechanism_is_supported
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> gboolean>;
    (*mechanism_class).encode_data = Some(
        safe_c2rust_mechanism_encode_data
            as unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                *const gchar,
                gsize,
                *mut gsize,
            ) -> *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                *const gchar,
                gsize,
                *mut gsize,
            ) -> *mut gchar,
        >;
    (*mechanism_class).decode_data = Some(
        safe_c2rust_mechanism_decode_data
            as unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                *const gchar,
                gsize,
                *mut gsize,
            ) -> *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                *const gchar,
                gsize,
                *mut gsize,
            ) -> *mut gchar,
        >;
    (*mechanism_class).server_get_state = Some(
        safe_c2rust_mechanism_server_get_state
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState>;
    (*mechanism_class).server_initiate = Some(
        safe_c2rust_mechanism_server_initiate
            as unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>;
    (*mechanism_class).server_data_receive = Some(
        safe_c2rust_mechanism_server_data_receive
            as unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>;
    (*mechanism_class).server_data_send = Some(
        safe_c2rust_mechanism_server_data_send
            as unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar>;
    (*mechanism_class).server_get_reject_reason = Some(
        safe_c2rust_mechanism_server_get_reject_reason
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> *mut gchar>;
    (*mechanism_class).server_shutdown = Some(
        safe_c2rust_mechanism_server_shutdown
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> ()>;
    (*mechanism_class).client_get_state = Some(
        safe_c2rust_mechanism_client_get_state
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> GDBusAuthMechanismState>;
    (*mechanism_class).client_initiate = Some(
        safe_c2rust_mechanism_client_initiate
            as unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                GDBusConnectionFlags,
                *mut gsize,
            ) -> *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDBusAuthMechanism,
                GDBusConnectionFlags,
                *mut gsize,
            ) -> *mut gchar,
        >;
    (*mechanism_class).client_data_receive = Some(
        safe_c2rust_mechanism_client_data_receive
            as unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *const gchar, gsize) -> ()>;
    (*mechanism_class).client_data_send = Some(
        safe_c2rust_mechanism_client_data_send
            as unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism, *mut gsize) -> *mut gchar>;
    (*mechanism_class).client_shutdown = Some(
        safe_c2rust_mechanism_client_shutdown
            as unsafe extern "C" fn(*mut GDBusAuthMechanism) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusAuthMechanism) -> ()>;
}
unsafe extern "C" fn safe_c2rust__g_dbus_auth_mechanism_sha1_init(
    mut mechanism: *mut GDBusAuthMechanismSha1,
) {
    (*mechanism).priv_0 = safe_c2rust__g_dbus_auth_mechanism_sha1_get_instance_private(mechanism)
        as *mut GDBusAuthMechanismSha1Private;
}
unsafe extern "C" fn safe_c2rust_mechanism_get_priority() -> gint {
    return 0 as gint;
}
unsafe extern "C" fn safe_c2rust_mechanism_get_name() -> *const gchar {
    return b"DBUS_COOKIE_SHA1\0" as *const u8 as *const gchar;
}
unsafe extern "C" fn safe_c2rust_mechanism_is_supported(
    mut mechanism: *mut GDBusAuthMechanism,
) -> gboolean {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_mechanism_encode_data(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn safe_c2rust_mechanism_decode_data(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn safe_c2rust_random_ascii() -> gint {
    let mut ret: gint = 0;
    ret = g_random_int_range(0 as gint32, 60 as gint32) as gint;
    if ret < 25 as ::core::ffi::c_int {
        ret += 'A' as i32;
    } else if ret < 50 as ::core::ffi::c_int {
        ret += 'a' as i32 - 25 as ::core::ffi::c_int;
    } else {
        ret += '0' as i32 - 50 as ::core::ffi::c_int;
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_random_ascii_string(mut len: guint) -> *mut gchar {
    let mut challenge: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut n: guint = 0;
    challenge = g_string_new(::core::ptr::null::<gchar>());
    n = 0 as guint;
    while n < len {
        safe_c2rust_g_string_append_c_inline(challenge, safe_c2rust_random_ascii() as gchar);
        n = n.wrapping_add(1);
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(challenge, 0 as gboolean)
        } else {
            g_string_free_and_steal(challenge)
        }
    } else {
        g_string_free(challenge, 0 as gboolean)
    };
}
unsafe extern "C" fn safe_c2rust_random_blob(mut len: guint) -> *mut gchar {
    let mut challenge: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut n: guint = 0;
    challenge = g_string_new(::core::ptr::null::<gchar>());
    n = 0 as guint;
    while n < len {
        safe_c2rust_g_string_append_c_inline(
            challenge,
            g_random_int_range(0 as gint32, 256 as gint32) as gchar,
        );
        n = n.wrapping_add(1);
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(challenge, 0 as gboolean)
        } else {
            g_string_free_and_steal(challenge)
        }
    } else {
        g_string_free(challenge, 0 as gboolean)
    };
}
unsafe extern "C" fn safe_c2rust_ensure_keyring_directory(
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut e: *const gchar = ::core::ptr::null::<gchar>();
    let mut is_setuid: gboolean = 0;
    let mut statbuf: stat = stat {
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
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    e = g_getenv(b"G_DBUS_COOKIE_SHA1_KEYRING_DIR\0" as *const u8 as *const gchar);
    if !e.is_null() {
        path = safe_c2rust_g_strdup_inline(e as *const ::core::ffi::c_char) as *mut gchar;
    } else {
        path = g_build_filename(
            g_get_home_dir(),
            b".dbus-keyrings\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        );
    }
    if stat(path, &raw mut statbuf) != 0 as ::core::ffi::c_int {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        if errsv != ENOENT {
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                glib_gettext(
                    b"Error when getting information for directory \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                        as *const u8 as *const gchar,
                ),
                path,
                g_strerror(errsv as gint),
            );
            let mut _pp: *mut *mut gchar = &raw mut path;
            let mut _ptr: *mut gchar = *_pp;
            *_pp = ::core::ptr::null_mut::<gchar>();
            if !_ptr.is_null() {
                g_free(_ptr as gpointer);
            }
            return ::core::ptr::null_mut::<gchar>();
        }
    } else if statbuf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
        if g_getenv(
            b"G_DBUS_COOKIE_SHA1_KEYRING_DIR_IGNORE_PERMISSION\0" as *const u8 as *const gchar,
        )
        .is_null()
            && statbuf.st_mode & 0o777 as __mode_t != 0o700 as __mode_t
        {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Permissions on directory \xE2\x80\x9C%s\xE2\x80\x9D are malformed. Expected mode 0700, got 0%o\0"
                        as *const u8 as *const gchar,
                ),
                path,
                statbuf.st_mode & 0o777 as __mode_t,
            );
            let mut _pp_0: *mut *mut gchar = &raw mut path;
            let mut _ptr_0: *mut gchar = *_pp_0;
            *_pp_0 = ::core::ptr::null_mut::<gchar>();
            if !_ptr_0.is_null() {
                g_free(_ptr_0 as gpointer);
            }
            return ::core::ptr::null_mut::<gchar>();
        }
        return safe_c2rust_g_steal_pointer(&raw mut path as gpointer) as *mut gchar;
    }
    is_setuid = (*glib__private__())
        .g_check_setuid
        .expect("non-null function pointer")();
    if is_setuid == 0 && g_mkdir_with_parents(path, 0o700 as gint) != 0 as ::core::ffi::c_int {
        let mut errsv_0: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv_0 as gint) as gint,
            glib_gettext(
                b"Error creating directory \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                    as *const gchar,
            ),
            path,
            g_strerror(errsv_0 as gint),
        );
        let mut _pp_1: *mut *mut gchar = &raw mut path;
        let mut _ptr_1: *mut gchar = *_pp_1;
        *_pp_1 = ::core::ptr::null_mut::<gchar>();
        if !_ptr_1.is_null() {
            g_free(_ptr_1 as gpointer);
        }
        return ::core::ptr::null_mut::<gchar>();
    } else if is_setuid != 0 {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_PERMISSION_DENIED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error creating directory \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                    as *const gchar,
            ),
            path,
            glib_gettext(b"Operation not supported\0" as *const u8 as *const gchar),
        );
        let mut _pp_2: *mut *mut gchar = &raw mut path;
        let mut _ptr_2: *mut gchar = *_pp_2;
        *_pp_2 = ::core::ptr::null_mut::<gchar>();
        if !_ptr_2.is_null() {
            g_free(_ptr_2 as gpointer);
        }
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_steal_pointer(&raw mut path as gpointer) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_keyring_lookup_entry(
    mut cookie_context: *const gchar,
    mut cookie_id: gint,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut current_block: u64;
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut keyring_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut n: guint = 0;
    let mut lines: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !cookie_context.is_null() {
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
            b"cookie_context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    ret = ::core::ptr::null_mut::<gchar>();
    path = ::core::ptr::null_mut::<gchar>();
    contents = ::core::ptr::null_mut::<gchar>();
    lines = ::core::ptr::null_mut::<*mut gchar>();
    keyring_dir = safe_c2rust_ensure_keyring_directory(error);
    if !keyring_dir.is_null() {
        path = g_build_filename(keyring_dir, cookie_context, NULL_0);
        if g_file_get_contents(
            path,
            &raw mut contents,
            ::core::ptr::null_mut::<gsize>(),
            error,
        ) == 0
        {
            g_prefix_error(
                error,
                glib_gettext(
                    b"Error opening keyring \xE2\x80\x9C%s\xE2\x80\x9D for reading: \0" as *const u8
                        as *const gchar,
                ),
                path,
            );
        } else {
            if ({
                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                if !contents.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanismsha1.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    415 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"contents != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            lines = g_strsplit(contents, b"\n\0" as *const u8 as *const gchar, 0 as gint);
            n = 0 as guint;
            loop {
                if (*lines.offset(n as isize)).is_null() {
                    current_block = 12997042908615822766;
                    break;
                }
                let mut line: *const gchar = *lines.offset(n as isize);
                let mut tokens: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
                let mut endp: *mut gchar = ::core::ptr::null_mut::<gchar>();
                let mut line_id: gint = 0;
                if !(*line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32)
                {
                    tokens = g_strsplit(line, b" \0" as *const u8 as *const gchar, 0 as gint);
                    if g_strv_length(tokens) != 3 as guint {
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Line %d of the keyring at \xE2\x80\x9C%s\xE2\x80\x9D with content \xE2\x80\x9C%s\xE2\x80\x9D is malformed\0"
                                    as *const u8 as *const gchar,
                            ),
                            n.wrapping_add(1 as guint),
                            path,
                            line,
                        );
                        g_strfreev(tokens);
                        current_block = 17631475890441575399;
                        break;
                    } else {
                        line_id = g_ascii_strtoll(
                            *tokens.offset(0 as ::core::ffi::c_int as isize),
                            &raw mut endp,
                            10 as guint,
                        ) as gint;
                        if *endp as ::core::ffi::c_int != '\0' as i32 {
                            g_set_error(
                                error,
                                g_io_error_quark(),
                                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"First token of line %d of the keyring at \xE2\x80\x9C%s\xE2\x80\x9D with content \xE2\x80\x9C%s\xE2\x80\x9D is malformed\0"
                                        as *const u8 as *const gchar,
                                ),
                                n.wrapping_add(1 as guint),
                                path,
                                line,
                            );
                            g_strfreev(tokens);
                            current_block = 17631475890441575399;
                            break;
                        } else {
                            g_ascii_strtoll(
                                *tokens.offset(1 as ::core::ffi::c_int as isize),
                                &raw mut endp,
                                10 as guint,
                            );
                            if *endp as ::core::ffi::c_int != '\0' as i32 {
                                g_set_error(
                                    error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"Second token of line %d of the keyring at \xE2\x80\x9C%s\xE2\x80\x9D with content \xE2\x80\x9C%s\xE2\x80\x9D is malformed\0"
                                            as *const u8 as *const gchar,
                                    ),
                                    n.wrapping_add(1 as guint),
                                    path,
                                    line,
                                );
                                g_strfreev(tokens);
                                current_block = 17631475890441575399;
                                break;
                            } else if line_id == cookie_id {
                                ret = *tokens.offset(2 as ::core::ffi::c_int as isize);
                                let ref mut fresh1 =
                                    *tokens.offset(2 as ::core::ffi::c_int as isize);
                                *fresh1 = ::core::ptr::null_mut::<gchar>();
                                g_strfreev(tokens);
                                current_block = 17631475890441575399;
                                break;
                            } else {
                                g_strfreev(tokens);
                            }
                        }
                    }
                }
                n = n.wrapping_add(1);
            }
            match current_block {
                17631475890441575399 => {}
                _ => {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Didn\xE2\x80\x99t find cookie with id %d in the keyring at \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                as *const u8 as *const gchar,
                        ),
                        cookie_id,
                        path,
                    );
                }
            }
        }
    }
    g_free(keyring_dir as gpointer);
    g_free(path as gpointer);
    g_free(contents as gpointer);
    g_strfreev(lines);
    return ret;
}
unsafe extern "C" fn safe_c2rust__log(mut message: *const gchar, mut args: ...) {
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut var_args: ::core::ffi::VaList;
    var_args = args.clone();
    s = g_strdup_vprintf(message, var_args);
    g_printerr(
        b"GDBus-DBUS_COOKIE_SHA1: %s\n\0" as *const u8 as *const gchar,
        s,
    );
    g_free(s as gpointer);
}
unsafe extern "C" fn safe_c2rust_create_lock_exclusive(
    mut lock_path: *const gchar,
    mut mtime_nsec: *mut gint64,
    mut error: *mut *mut GError,
) -> gint {
    let mut errsv: ::core::ffi::c_int = 0;
    let mut ret: gint = 0;
    ret = open(
        lock_path as *const ::core::ffi::c_char,
        O_CREAT | O_EXCL | O_CLOEXEC,
        0o600 as ::core::ffi::c_int,
    ) as gint;
    errsv = *__errno_location();
    if ret < 0 as ::core::ffi::c_int {
        let mut stat_buf: statx = statx {
            stx_mask: 0,
            stx_blksize: 0,
            stx_attributes: 0,
            stx_nlink: 0,
            stx_uid: 0,
            stx_gid: 0,
            stx_mode: 0,
            __spare0: [0; 1],
            stx_ino: 0,
            stx_size: 0,
            stx_blocks: 0,
            stx_attributes_mask: 0,
            stx_atime: statx_timestamp {
                tv_sec: 0,
                tv_nsec: 0,
                __reserved: 0,
            },
            stx_btime: statx_timestamp {
                tv_sec: 0,
                tv_nsec: 0,
                __reserved: 0,
            },
            stx_ctime: statx_timestamp {
                tv_sec: 0,
                tv_nsec: 0,
                __reserved: 0,
            },
            stx_mtime: statx_timestamp {
                tv_sec: 0,
                tv_nsec: 0,
                __reserved: 0,
            },
            stx_rdev_major: 0,
            stx_rdev_minor: 0,
            stx_dev_major: 0,
            stx_dev_minor: 0,
            stx_mnt_id: 0,
            stx_dio_mem_align: 0,
            stx_dio_offset_align: 0,
            __spare3: [0; 12],
        };
        if !mtime_nsec.is_null()
            && safe_c2rust_g_local_file_stat(
                lock_path as *const ::core::ffi::c_char,
                G_LOCAL_FILE_STAT_FIELD_MTIME,
                4095 as GLocalFileStatField,
                &raw mut stat_buf,
            ) == 0 as ::core::ffi::c_int
        {
            *mtime_nsec = safe_c2rust__g_stat_mtime(&raw mut stat_buf)
                * G_USEC_PER_SEC as gint64
                * 1000 as gint64
                + safe_c2rust__g_stat_mtim_nsec(&raw mut stat_buf) as gint64;
        } else if !mtime_nsec.is_null() {
            *mtime_nsec = 0 as gint64;
        }
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Error creating lock file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                    as *const gchar,
            ),
            lock_path,
            g_strerror(errsv as gint),
        );
        return -(1 as gint);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_keyring_acquire_lock(
    mut path: *const gchar,
    mut error: *mut *mut GError,
) -> gint {
    let mut lock: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ret: gint = 0;
    let mut num_tries: guint = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    let mut lock_mtime_nsec: gint64 = 0 as gint64;
    let mut lock_mtime_nsec_prev: gint64 = 0 as gint64;
    let max_tries: guint = 50 as guint;
    let timeout_usec: guint = (1000 as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as guint;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !path.is_null() {
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
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    ret = -(1 as ::core::ffi::c_int) as gint;
    lock = g_strconcat(
        path,
        b".lock\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    num_tries = 0 as guint;
    while num_tries < max_tries {
        lock_mtime_nsec_prev = lock_mtime_nsec;
        ret = safe_c2rust_create_lock_exclusive(
            lock,
            &raw mut lock_mtime_nsec,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if ret >= 0 as ::core::ffi::c_int {
            break;
        }
        g_usleep(timeout_usec as gulong);
        if num_tries > 0 as guint && lock_mtime_nsec != lock_mtime_nsec_prev {
            num_tries = num_tries.wrapping_sub(1);
        }
        num_tries = num_tries.wrapping_add(1);
    }
    if num_tries == max_tries {
        if g_unlink(lock) != 0 as ::core::ffi::c_int {
            errsv = *__errno_location();
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                glib_gettext(
                    b"Error deleting stale lock file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                        as *const gchar,
                ),
                lock,
                g_strerror(errsv as gint),
            );
        } else {
            safe_c2rust__log(
                b"Deleted stale lock file '%s'\0" as *const u8 as *const gchar,
                lock,
            );
            ret = safe_c2rust_create_lock_exclusive(lock, ::core::ptr::null_mut::<gint64>(), error);
            ret < 0 as ::core::ffi::c_int;
        }
    }
    g_free(lock as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_keyring_release_lock(
    mut path: *const gchar,
    mut lock_fd: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut lock: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !path.is_null() {
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
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if lock_fd != -(1 as ::core::ffi::c_int) {
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
            b"lock_fd != -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
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
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    lock = g_strdup_printf(b"%s.lock\0" as *const u8 as *const gchar, path);
    if close(lock_fd as ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Error closing (unlinked) lock file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                    as *const gchar,
            ),
            lock,
            g_strerror(errsv as gint),
        );
    } else if g_unlink(lock) != 0 as ::core::ffi::c_int {
        let mut errsv_0: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv_0 as gint) as gint,
            glib_gettext(
                b"Error unlinking lock file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                    as *const gchar,
            ),
            lock,
            g_strerror(errsv_0 as gint),
        );
    } else {
        ret = TRUE as gboolean;
    }
    g_free(lock as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_keyring_generate_entry(
    mut cookie_context: *const gchar,
    mut out_id: *mut gint,
    mut out_cookie: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut keyring_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut lines: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut max_line_id: gint = 0;
    let mut new_contents: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut now: gint64 = 0;
    let mut have_id: gboolean = 0;
    let mut use_id: gint = 0;
    let mut use_cookie: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut changed_file: gboolean = 0;
    let mut lock_fd: gint = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !cookie_context.is_null() {
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
            b"cookie_context != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !out_id.is_null() {
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
            b"out_id != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !out_cookie.is_null() {
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
            b"out_cookie != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    path = ::core::ptr::null_mut::<gchar>();
    contents = ::core::ptr::null_mut::<gchar>();
    lines = ::core::ptr::null_mut::<*mut gchar>();
    new_contents = ::core::ptr::null_mut::<GString>();
    have_id = FALSE as gboolean;
    use_id = 0 as ::core::ffi::c_int as gint;
    use_cookie = ::core::ptr::null_mut::<gchar>();
    lock_fd = -(1 as ::core::ffi::c_int) as gint;
    keyring_dir = safe_c2rust_ensure_keyring_directory(error);
    if !keyring_dir.is_null() {
        path = g_build_filename(keyring_dir, cookie_context, NULL_0);
        lock_fd = safe_c2rust_keyring_acquire_lock(path, error);
        if !(lock_fd == -(1 as ::core::ffi::c_int)) {
            contents = ::core::ptr::null_mut::<gchar>();
            if g_file_get_contents(
                path,
                &raw mut contents,
                ::core::ptr::null_mut::<gsize>(),
                &raw mut local_error,
            ) == 0
            {
                if (*local_error).domain == g_file_error_quark()
                    && (*local_error).code == G_FILE_ERROR_NOENT as ::core::ffi::c_int
                {
                    g_clear_error(&raw mut local_error);
                    current_block = 15512526488502093901;
                } else {
                    g_propagate_prefixed_error(
                        error,
                        safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer)
                            as *mut GError,
                        glib_gettext(
                            b"Error opening keyring \xE2\x80\x9C%s\xE2\x80\x9D for writing: \0"
                                as *const u8 as *const gchar,
                        ),
                        path,
                    );
                    current_block = 16996945439345483477;
                }
            } else {
                current_block = 15512526488502093901;
            }
            match current_block {
                16996945439345483477 => {}
                _ => {
                    new_contents = g_string_new(::core::ptr::null::<gchar>());
                    now = g_get_real_time() / G_USEC_PER_SEC as gint64;
                    changed_file = FALSE as gboolean;
                    max_line_id = 0 as ::core::ffi::c_int as gint;
                    if !contents.is_null() {
                        let mut n: guint = 0;
                        lines =
                            g_strsplit(contents, b"\n\0" as *const u8 as *const gchar, 0 as gint);
                        n = 0 as guint;
                        loop {
                            if (*lines.offset(n as isize)).is_null() {
                                current_block = 3879520548144599102;
                                break;
                            }
                            let mut line: *const gchar = *lines.offset(n as isize);
                            let mut tokens: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
                            let mut endp: *mut gchar = ::core::ptr::null_mut::<gchar>();
                            let mut line_id: gint = 0;
                            let mut line_when: gint64 = 0;
                            let mut keep_entry: gboolean = 0;
                            if !(*line.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\0' as i32)
                            {
                                tokens = g_strsplit(
                                    line,
                                    b" \0" as *const u8 as *const gchar,
                                    0 as gint,
                                );
                                if g_strv_length(tokens) != 3 as guint {
                                    g_set_error(
                                        error,
                                        g_io_error_quark(),
                                        G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                        glib_gettext(
                                            b"Line %d of the keyring at \xE2\x80\x9C%s\xE2\x80\x9D with content \xE2\x80\x9C%s\xE2\x80\x9D is malformed\0"
                                                as *const u8 as *const gchar,
                                        ),
                                        n.wrapping_add(1 as guint),
                                        path,
                                        line,
                                    );
                                    g_strfreev(tokens);
                                    current_block = 16996945439345483477;
                                    break;
                                } else {
                                    line_id = g_ascii_strtoll(
                                        *tokens.offset(0 as ::core::ffi::c_int as isize),
                                        &raw mut endp,
                                        10 as guint,
                                    ) as gint;
                                    if *endp as ::core::ffi::c_int != '\0' as i32 {
                                        g_set_error(
                                            error,
                                            g_io_error_quark(),
                                            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                            glib_gettext(
                                                b"First token of line %d of the keyring at \xE2\x80\x9C%s\xE2\x80\x9D with content \xE2\x80\x9C%s\xE2\x80\x9D is malformed\0"
                                                    as *const u8 as *const gchar,
                                            ),
                                            n.wrapping_add(1 as guint),
                                            path,
                                            line,
                                        );
                                        g_strfreev(tokens);
                                        current_block = 16996945439345483477;
                                        break;
                                    } else {
                                        line_when = g_ascii_strtoll(
                                            *tokens.offset(1 as ::core::ffi::c_int as isize),
                                            &raw mut endp,
                                            10 as guint,
                                        );
                                        if *endp as ::core::ffi::c_int != '\0' as i32 {
                                            g_set_error(
                                                error,
                                                g_io_error_quark(),
                                                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                                glib_gettext(
                                                    b"Second token of line %d of the keyring at \xE2\x80\x9C%s\xE2\x80\x9D with content \xE2\x80\x9C%s\xE2\x80\x9D is malformed\0"
                                                        as *const u8 as *const gchar,
                                                ),
                                                n.wrapping_add(1 as guint),
                                                path,
                                                line,
                                            );
                                            g_strfreev(tokens);
                                            current_block = 16996945439345483477;
                                            break;
                                        } else {
                                            keep_entry = TRUE as gboolean;
                                            if line_when > now {
                                                if line_when - now
                                                    > MAX_TIME_TRAVEL_SECONDS as gint64
                                                {
                                                    keep_entry = FALSE as gboolean;
                                                    safe_c2rust__log(
                                                        b"Deleted SHA1 cookie from %lu seconds in the future\0"
                                                            as *const u8 as *const gchar,
                                                        line_when - now,
                                                    );
                                                }
                                            } else if now - line_when
                                                > EXPIRE_KEYS_TIMEOUT_SECONDS as gint64
                                            {
                                                keep_entry = FALSE as gboolean;
                                            }
                                            if keep_entry == 0 {
                                                changed_file = FALSE as gboolean;
                                            } else {
                                                g_string_append_printf(
                                                    new_contents,
                                                    b"%d %lu %s\n\0" as *const u8 as *const gchar,
                                                    line_id,
                                                    line_when,
                                                    *tokens
                                                        .offset(2 as ::core::ffi::c_int as isize),
                                                );
                                                max_line_id = if line_id > max_line_id {
                                                    line_id
                                                } else {
                                                    max_line_id
                                                };
                                                if now - line_when
                                                    < NEW_KEY_TIMEOUT_SECONDS as gint64
                                                {
                                                    if have_id == 0 {
                                                        use_id = line_id;
                                                        use_cookie = *tokens.offset(
                                                            2 as ::core::ffi::c_int as isize,
                                                        );
                                                        let ref mut fresh2 = *tokens.offset(
                                                            2 as ::core::ffi::c_int as isize,
                                                        );
                                                        *fresh2 = ::core::ptr::null_mut::<gchar>();
                                                        have_id = TRUE as gboolean;
                                                    }
                                                }
                                            }
                                            g_strfreev(tokens);
                                        }
                                    }
                                }
                            }
                            n = n.wrapping_add(1);
                        }
                    } else {
                        current_block = 3879520548144599102;
                    }
                    match current_block {
                        16996945439345483477 => {}
                        _ => {
                            ret = TRUE as gboolean;
                            if have_id != 0 {
                                *out_id = use_id;
                                *out_cookie = use_cookie;
                                use_cookie = ::core::ptr::null_mut::<gchar>();
                            } else {
                                let mut raw_cookie: *mut gchar = ::core::ptr::null_mut::<gchar>();
                                *out_id = (max_line_id as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int)
                                    as gint;
                                raw_cookie = safe_c2rust_random_blob(32 as guint);
                                *out_cookie = _g_dbus_hexencode(raw_cookie, 32 as gsize);
                                g_free(raw_cookie as gpointer);
                                g_string_append_printf(
                                    new_contents,
                                    b"%d %li %s\n\0" as *const u8 as *const gchar,
                                    *out_id,
                                    g_get_real_time() / G_USEC_PER_SEC as gint64,
                                    *out_cookie,
                                );
                                changed_file = TRUE as gboolean;
                            }
                            if changed_file != 0 {
                                if g_file_set_contents_full(
                                    path,
                                    (*new_contents).str_0,
                                    -(1 as ::core::ffi::c_int) as gssize,
                                    G_FILE_SET_CONTENTS_CONSISTENT,
                                    0o600 as ::core::ffi::c_int,
                                    error,
                                ) == 0
                                {
                                    *out_id = 0 as ::core::ffi::c_int as gint;
                                    g_free(*out_cookie as gpointer);
                                    *out_cookie = ::core::ptr::null_mut::<gchar>();
                                    ret = FALSE as gboolean;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if local_error.is_null() {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanismsha1.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            921 as ::core::ffi::c_int,
            G_STRFUNC,
            b"local_error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if lock_fd != -(1 as ::core::ffi::c_int) {
        if safe_c2rust_keyring_release_lock(path, lock_fd, &raw mut local_error) == 0 {
            if !error.is_null() {
                if (*error).is_null() {
                    *error = local_error;
                } else {
                    g_prefix_error(
                        error,
                        glib_gettext(
                            b"(Additionally, releasing the lock for \xE2\x80\x9C%s\xE2\x80\x9D also failed: %s) \0"
                                as *const u8 as *const gchar,
                        ),
                        path,
                        (*local_error).message,
                    );
                    g_error_free(local_error);
                }
            } else {
                g_error_free(local_error);
            }
        }
    }
    g_free(keyring_dir as gpointer);
    g_free(path as gpointer);
    g_strfreev(lines);
    g_free(contents as gpointer);
    if !new_contents.is_null() {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(
                    new_contents,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            } else {
                g_string_free_and_steal(new_contents);
            };
        } else {
            g_string_free(
                new_contents,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        };
    }
    g_free(use_cookie as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_generate_sha1(
    mut server_challenge: *const gchar,
    mut client_challenge: *const gchar,
    mut cookie: *const gchar,
) -> *mut gchar {
    let mut str: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut sha1: *mut gchar = ::core::ptr::null_mut::<gchar>();
    str = g_string_new(server_challenge);
    safe_c2rust_g_string_append_c_inline(str, ':' as i32 as gchar);
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = client_challenge as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                str,
                __val,
                if ({
                    let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_25
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
            client_challenge as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    safe_c2rust_g_string_append_c_inline(str, ':' as i32 as gchar);
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = cookie as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                str,
                __val,
                if ({
                    let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_26 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_26 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_26
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
            cookie as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    sha1 = g_compute_checksum_for_string(
        G_CHECKSUM_SHA1,
        (*str).str_0,
        -(1 as ::core::ffi::c_int) as gssize,
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
    return sha1;
}
unsafe extern "C" fn safe_c2rust_mechanism_server_get_state(
    mut mechanism: *mut GDBusAuthMechanism,
) -> GDBusAuthMechanismState {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_DBUS_AUTH_MECHANISM_STATE_INVALID;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server != 0 && (*(*m).priv_0).is_client == 0 {
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
            b"m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_DBUS_AUTH_MECHANISM_STATE_INVALID;
    }
    return (*(*m).priv_0).state;
}
unsafe extern "C" fn safe_c2rust_mechanism_server_initiate(
    mut mechanism: *mut GDBusAuthMechanism,
    mut initial_response: *const gchar,
    mut initial_response_len: gsize,
) {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server == 0 && (*(*m).priv_0).is_client == 0 {
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
            b"!m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*m).priv_0).is_server = TRUE as gboolean;
    (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_REJECTED;
    if !initial_response.is_null() && initial_response_len > 0 as gsize {
        let mut uid: gint64 = 0;
        let mut endp: *mut gchar = ::core::ptr::null_mut::<gchar>();
        uid = g_ascii_strtoll(initial_response, &raw mut endp, 10 as guint);
        if *endp as ::core::ffi::c_int == '\0' as i32 {
            if uid == getuid() as gint64 {
                (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND;
            }
        }
    }
}
unsafe extern "C" fn safe_c2rust_mechanism_server_data_receive(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
) {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    let mut tokens: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut client_challenge: *const gchar = ::core::ptr::null::<gchar>();
    let mut alleged_sha1: *const gchar = ::core::ptr::null::<gchar>();
    let mut sha1: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server != 0 && (*(*m).priv_0).is_client == 0 {
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
            b"m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).state as ::core::ffi::c_uint
            == G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA as ::core::ffi::c_int
                as ::core::ffi::c_uint
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
            b"m->priv->state == G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    tokens = ::core::ptr::null_mut::<*mut gchar>();
    sha1 = ::core::ptr::null_mut::<gchar>();
    tokens = g_strsplit(data, b" \0" as *const u8 as *const gchar, 0 as gint);
    if g_strv_length(tokens) != 2 as guint {
        g_free((*(*m).priv_0).reject_reason as gpointer);
        (*(*m).priv_0).reject_reason =
            g_strdup_printf(b"Malformed data '%s'\0" as *const u8 as *const gchar, data);
        (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_REJECTED;
    } else {
        client_challenge = *tokens.offset(0 as ::core::ffi::c_int as isize);
        alleged_sha1 = *tokens.offset(1 as ::core::ffi::c_int as isize);
        sha1 = safe_c2rust_generate_sha1(
            (*(*m).priv_0).server_challenge,
            client_challenge,
            (*(*m).priv_0).cookie,
        );
        if g_strcmp0(sha1, alleged_sha1 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
            (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_ACCEPTED;
        } else {
            g_free((*(*m).priv_0).reject_reason as gpointer);
            (*(*m).priv_0).reject_reason =
                g_strdup_printf(b"SHA-1 mismatch\0" as *const u8 as *const gchar);
            (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_REJECTED;
        }
    }
    g_strfreev(tokens);
    g_free(sha1 as gpointer);
}
unsafe extern "C" fn safe_c2rust_mechanism_server_data_send(
    mut mechanism: *mut GDBusAuthMechanism,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut cookie_id: gint = 0;
    let mut cookie_context: *const gchar = ::core::ptr::null::<gchar>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server != 0 && (*(*m).priv_0).is_client == 0 {
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
            b"m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).state as ::core::ffi::c_uint
            == G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND as ::core::ffi::c_int
                as ::core::ffi::c_uint
        {
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
            b"m->priv->state == G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    s = ::core::ptr::null_mut::<gchar>();
    *out_data_len = 0 as gsize;
    cookie_context =
        b"org_gtk_gdbus_general\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    cookie_id = -(1 as ::core::ffi::c_int) as gint;
    error = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_keyring_generate_entry(
        cookie_context,
        &raw mut cookie_id,
        &raw mut (*(*m).priv_0).cookie,
        &raw mut error,
    ) == 0
    {
        g_free((*(*m).priv_0).reject_reason as gpointer);
        (*(*m).priv_0).reject_reason = g_strdup_printf(
            b"Error adding entry to keyring: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
        (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_REJECTED;
    } else {
        (*(*m).priv_0).server_challenge = safe_c2rust_random_ascii_string(16 as guint);
        s = g_strdup_printf(
            b"%s %d %s\0" as *const u8 as *const gchar,
            cookie_context,
            cookie_id,
            (*(*m).priv_0).server_challenge,
        );
        *out_data_len = strlen(s) as gsize;
        (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA;
    }
    return s;
}
unsafe extern "C" fn safe_c2rust_mechanism_server_get_reject_reason(
    mut mechanism: *mut GDBusAuthMechanism,
) -> *mut gchar {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server != 0 && (*(*m).priv_0).is_client == 0 {
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
            b"m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).state as ::core::ffi::c_uint
            == G_DBUS_AUTH_MECHANISM_STATE_REJECTED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"m->priv->state == G_DBUS_AUTH_MECHANISM_STATE_REJECTED\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_strdup_inline((*(*m).priv_0).reject_reason) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_mechanism_server_shutdown(mut mechanism: *mut GDBusAuthMechanism) {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server != 0 && (*(*m).priv_0).is_client == 0 {
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
            b"m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*m).priv_0).is_server = FALSE as gboolean;
}
unsafe extern "C" fn safe_c2rust_mechanism_client_get_state(
    mut mechanism: *mut GDBusAuthMechanism,
) -> GDBusAuthMechanismState {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_DBUS_AUTH_MECHANISM_STATE_INVALID;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_client != 0 && (*(*m).priv_0).is_server == 0 {
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
            b"m->priv->is_client && !m->priv->is_server\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_DBUS_AUTH_MECHANISM_STATE_INVALID;
    }
    return (*(*m).priv_0).state;
}
unsafe extern "C" fn safe_c2rust_mechanism_client_initiate(
    mut mechanism: *mut GDBusAuthMechanism,
    mut conn_flags: GDBusConnectionFlags,
    mut out_initial_response_len: *mut gsize,
) -> *mut gchar {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    let mut initial_response: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_server == 0 && (*(*m).priv_0).is_client == 0 {
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
            b"!m->priv->is_server && !m->priv->is_client\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    (*(*m).priv_0).is_client = TRUE as gboolean;
    *out_initial_response_len = 0 as gsize;
    initial_response = g_strdup_printf(b"%li\0" as *const u8 as *const gchar, getuid() as gint64);
    if !initial_response.is_null() {
        (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA;
        *out_initial_response_len = strlen(initial_response) as gsize;
    } else {
        (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_REJECTED;
    }
    return initial_response;
}
unsafe extern "C" fn safe_c2rust_mechanism_client_data_receive(
    mut mechanism: *mut GDBusAuthMechanism,
    mut data: *const gchar,
    mut data_len: gsize,
) {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    let mut tokens: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut cookie_context: *const gchar = ::core::ptr::null::<gchar>();
    let mut cookie_id: guint = 0;
    let mut server_challenge: *const gchar = ::core::ptr::null::<gchar>();
    let mut client_challenge: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut endp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut cookie: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut sha1: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            _g_boolean_var_46 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_46 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_46
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_client != 0 && (*(*m).priv_0).is_server == 0 {
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"m->priv->is_client && !m->priv->is_server\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).state as ::core::ffi::c_uint
            == G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA as ::core::ffi::c_int
                as ::core::ffi::c_uint
        {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"m->priv->state == G_DBUS_AUTH_MECHANISM_STATE_WAITING_FOR_DATA\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    tokens = ::core::ptr::null_mut::<*mut gchar>();
    cookie = ::core::ptr::null_mut::<gchar>();
    client_challenge = ::core::ptr::null_mut::<gchar>();
    tokens = g_strsplit(data, b" \0" as *const u8 as *const gchar, 0 as gint);
    if g_strv_length(tokens) != 3 as guint {
        g_free((*(*m).priv_0).reject_reason as gpointer);
        (*(*m).priv_0).reject_reason =
            g_strdup_printf(b"Malformed data '%s'\0" as *const u8 as *const gchar, data);
        (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_REJECTED;
    } else {
        cookie_context = *tokens.offset(0 as ::core::ffi::c_int as isize);
        cookie_id = g_ascii_strtoll(
            *tokens.offset(1 as ::core::ffi::c_int as isize),
            &raw mut endp,
            10 as guint,
        ) as guint;
        if *endp as ::core::ffi::c_int != '\0' as i32 {
            g_free((*(*m).priv_0).reject_reason as gpointer);
            (*(*m).priv_0).reject_reason = g_strdup_printf(
                b"Malformed cookie_id '%s'\0" as *const u8 as *const gchar,
                *tokens.offset(1 as ::core::ffi::c_int as isize),
            );
            (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_REJECTED;
        } else {
            server_challenge = *tokens.offset(2 as ::core::ffi::c_int as isize);
            error = ::core::ptr::null_mut::<GError>();
            cookie =
                safe_c2rust_keyring_lookup_entry(cookie_context, cookie_id as gint, &raw mut error);
            if cookie.is_null() {
                g_free((*(*m).priv_0).reject_reason as gpointer);
                (*(*m).priv_0).reject_reason = g_strdup_printf(
                    b"Problems looking up entry in keyring: %s\0" as *const u8 as *const gchar,
                    (*error).message,
                );
                g_error_free(error);
                (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_REJECTED;
            } else {
                client_challenge = safe_c2rust_random_ascii_string(16 as guint);
                sha1 = safe_c2rust_generate_sha1(server_challenge, client_challenge, cookie);
                (*(*m).priv_0).to_send = g_strdup_printf(
                    b"%s %s\0" as *const u8 as *const gchar,
                    client_challenge,
                    sha1,
                );
                g_free(sha1 as gpointer);
                (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND;
            }
        }
    }
    g_strfreev(tokens);
    g_free(cookie as gpointer);
    g_free(client_challenge as gpointer);
}
unsafe extern "C" fn safe_c2rust_mechanism_client_data_send(
    mut mechanism: *mut GDBusAuthMechanism,
    mut out_data_len: *mut gsize,
) -> *mut gchar {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_client != 0 && (*(*m).priv_0).is_server == 0 {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"m->priv->is_client && !m->priv->is_server\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).state as ::core::ffi::c_uint
            == G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND as ::core::ffi::c_int
                as ::core::ffi::c_uint
        {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"m->priv->state == G_DBUS_AUTH_MECHANISM_STATE_HAVE_DATA_TO_SEND\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !(*(*m).priv_0).to_send.is_null() {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusauthmechanismsha1.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1278 as ::core::ffi::c_int,
            G_STRFUNC,
            b"m->priv->to_send != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(*m).priv_0).state = G_DBUS_AUTH_MECHANISM_STATE_ACCEPTED;
    *out_data_len = strlen((*(*m).priv_0).to_send) as gsize;
    return safe_c2rust_g_strdup_inline((*(*m).priv_0).to_send) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_mechanism_client_shutdown(mut mechanism: *mut GDBusAuthMechanism) {
    let mut m: *mut GDBusAuthMechanismSha1 =
        mechanism as *mut ::core::ffi::c_void as *mut GDBusAuthMechanismSha1;
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = mechanism as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_dbus_auth_mechanism_sha1_get_type();
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
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_AUTH_MECHANISM_SHA1 (mechanism)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if (*(*m).priv_0).is_client != 0 && (*(*m).priv_0).is_server == 0 {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"m->priv->is_client && !m->priv->is_server\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*m).priv_0).is_client = FALSE as gboolean;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
