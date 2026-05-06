use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GFileInfo;
    pub type _GFileOutputStreamPrivate;
    pub type _GFileDescriptorBased;
    fn fchmod(__fd: ::core::ffi::c_int, __mode: __mode_t) -> ::core::ffi::c_int;
    fn statx(
        __dirfd: ::core::ffi::c_int,
        __path: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
        __mask: ::core::ffi::c_uint,
        __buf: *mut statx,
    ) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
        __whence: ::core::ffi::c_int,
    ) -> __off64_t;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn fchown(__fd: ::core::ffi::c_int, __owner: __uid_t, __group: __gid_t) -> ::core::ffi::c_int;
    fn link(
        __from: *const ::core::ffi::c_char,
        __to: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ftruncate(__fd: ::core::ffi::c_int, __length: __off64_t) -> ::core::ffi::c_int;
    fn g_filename_display_name(filename: *const gchar) -> *mut gchar;
    fn g_mkstemp_full(tmpl: *mut gchar, flags: gint, mode: gint) -> gint;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_path_get_dirname(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_unlink(filename: *const gchar) -> ::core::ffi::c_int;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_output_stream_get_type() -> GType;
    fn g_file_info_has_attribute(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn g_file_info_get_attribute_uint32(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> guint32;
    fn _g_local_file_info_get_from_fd(
        fd: ::core::ffi::c_int,
        attributes: *const ::core::ffi::c_char,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn _g_local_file_info_create_etag(statbuf: *mut statx) -> *mut ::core::ffi::c_char;
    fn g_file_descriptor_based_get_type() -> GType;
    fn writev(
        __fd: ::core::ffi::c_int,
        __iovec: *const iovec,
        __count: ::core::ffi::c_int,
    ) -> ssize_t;
}
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off64_t = ::core::ffi::c_long;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type ssize_t = isize;
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type goffset = gint64;
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
pub type GSeekType = ::core::ffi::c_uint;
pub const G_SEEK_END: GSeekType = 2;
pub const G_SEEK_SET: GSeekType = 1;
pub const G_SEEK_CUR: GSeekType = 0;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
pub type GFileCreateFlags = ::core::ffi::c_uint;
pub const G_FILE_CREATE_REPLACE_DESTINATION: GFileCreateFlags = 2;
pub const G_FILE_CREATE_PRIVATE: GFileCreateFlags = 1;
pub const G_FILE_CREATE_NONE: GFileCreateFlags = 0;
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
pub type GOutputStreamSpliceFlags = ::core::ffi::c_uint;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_TARGET: GOutputStreamSpliceFlags = 2;
pub const G_OUTPUT_STREAM_SPLICE_CLOSE_SOURCE: GOutputStreamSpliceFlags = 1;
pub const G_OUTPUT_STREAM_SPLICE_NONE: GOutputStreamSpliceFlags = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
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
pub type GFileInfo = _GFileInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GFileOutputStreamPrivate,
}
pub type GFileOutputStreamPrivate = _GFileOutputStreamPrivate;
pub type GFileOutputStream = _GFileOutputStream;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputVector {
    pub buffer: gconstpointer,
    pub size: gsize,
}
pub type GOutputVector = _GOutputVector;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStreamClass {
    pub parent_class: GObjectClass,
    pub write_fn: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const ::core::ffi::c_void,
            gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub splice: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *mut GInputStream,
            GOutputStreamSpliceFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub flush: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub write_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const ::core::ffi::c_void,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub write_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub splice_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *mut GInputStream,
            GOutputStreamSpliceFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub splice_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub flush_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub flush_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GOutputStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub writev_fn: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const GOutputVector,
            gsize,
            *mut gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub writev_async: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *const GOutputVector,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub writev_finish: Option<
        unsafe extern "C" fn(
            *mut GOutputStream,
            *mut GAsyncResult,
            *mut gsize,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved7: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved8: Option<unsafe extern "C" fn() -> ()>,
}
pub type GOutputStreamClass = _GOutputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileOutputStreamClass {
    pub parent_class: GOutputStreamClass,
    pub tell: Option<unsafe extern "C" fn(*mut GFileOutputStream) -> goffset>,
    pub can_seek: Option<unsafe extern "C" fn(*mut GFileOutputStream) -> gboolean>,
    pub seek: Option<
        unsafe extern "C" fn(
            *mut GFileOutputStream,
            goffset,
            GSeekType,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_truncate: Option<unsafe extern "C" fn(*mut GFileOutputStream) -> gboolean>,
    pub truncate_fn: Option<
        unsafe extern "C" fn(
            *mut GFileOutputStream,
            goffset,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub query_info: Option<
        unsafe extern "C" fn(
            *mut GFileOutputStream,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_info_async: Option<
        unsafe extern "C" fn(
            *mut GFileOutputStream,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_info_finish: Option<
        unsafe extern "C" fn(
            *mut GFileOutputStream,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub get_etag: Option<unsafe extern "C" fn(*mut GFileOutputStream) -> *mut ::core::ffi::c_char>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFileOutputStreamClass = _GFileOutputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileOutputStream {
    pub parent_instance: GFileOutputStream,
    pub priv_0: *mut GLocalFileOutputStreamPrivate,
}
pub type GLocalFileOutputStreamPrivate = _GLocalFileOutputStreamPrivate;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GLocalFileOutputStreamPrivate {
    pub tmp_filename: *mut ::core::ffi::c_char,
    pub original_filename: *mut ::core::ffi::c_char,
    pub backup_filename: *mut ::core::ffi::c_char,
    pub etag: *mut ::core::ffi::c_char,
    #[bitfield(name = "sync_on_close", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "do_close", ty = "guint", bits = "1..=1")]
    pub sync_on_close_do_close: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub fd: ::core::ffi::c_int,
}
pub type GLocalFileOutputStream = _GLocalFileOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileOutputStreamClass {
    pub parent_class: GFileOutputStreamClass,
}
pub type GLocalFileOutputStreamClass = _GLocalFileOutputStreamClass;
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
pub type GFileDescriptorBasedIface = _GFileDescriptorBasedIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileDescriptorBasedIface {
    pub g_iface: GTypeInterface,
    pub get_fd: Option<unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int>,
}
pub type GFileDescriptorBased = _GFileDescriptorBased;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const ELOOP: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const STATX_ALL: ::core::ffi::c_uint = 0xfff as ::core::ffi::c_uint;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const S_IFMT: ::core::ffi::c_int = __S_IFMT;
pub const AT_FDCWD: ::core::ffi::c_int = -(100 as ::core::ffi::c_int);
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const AT_NO_AUTOMOUNT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const AT_EMPTY_PATH: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const AT_STATX_SYNC_AS_STAT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ESPIPE: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const IOV_MAX: ::core::ffi::c_int = __IOV_MAX;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __IOV_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
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
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
unsafe extern "C" fn safe_c2rust_g_local_file_fstat(
    mut fd: ::core::ffi::c_int,
    mut mask: GLocalFileStatField,
    mut mask_required: GLocalFileStatField,
    mut stat_buf: *mut statx,
) -> ::core::ffi::c_int {
    return safe_c2rust_g_local_file_statx(
        fd,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        AT_EMPTY_PATH,
        mask,
        mask_required,
        stat_buf,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_local_file_lstat(
    mut path: *const ::core::ffi::c_char,
    mut mask: GLocalFileStatField,
    mut mask_required: GLocalFileStatField,
    mut stat_buf: *mut statx,
) -> ::core::ffi::c_int {
    return safe_c2rust_g_local_file_statx(
        AT_FDCWD,
        path,
        AT_NO_AUTOMOUNT | AT_SYMLINK_NOFOLLOW | AT_STATX_SYNC_AS_STAT,
        mask,
        mask_required,
        stat_buf,
    );
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
unsafe extern "C" fn safe_c2rust__g_stat_mode(mut buf: *const statx) -> guint16 {
    return (*buf).stx_mode as guint16;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_nlink(mut buf: *const statx) -> guint32 {
    return (*buf).stx_nlink as guint32;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_uid(mut buf: *const statx) -> guint32 {
    return (*buf).stx_uid as guint32;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_gid(mut buf: *const statx) -> guint32 {
    return (*buf).stx_gid as guint32;
}
pub const G_IOV_MAX: ::core::ffi::c_int = IOV_MAX;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_file_output_stream_get_type(),
        g_intern_static_string(b"GLocalFileOutputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GLocalFileOutputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_output_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GLocalFileOutputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GLocalFileOutputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_output_stream_init
                    as unsafe extern "C" fn(*mut GLocalFileOutputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GLocalFileOutputStream_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GLocalFileOutputStreamPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GFileDescriptorBasedIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_file_descriptor_based_iface_init
                as unsafe extern "C" fn(*mut GFileDescriptorBasedIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_file_descriptor_based_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_output_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_local_file_output_stream_get_type_once();
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
static mut safe_c2rust_g_local_file_output_stream_parent_class: gpointer = NULL_0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_get_instance_private(
    mut self_0: *mut GLocalFileOutputStream,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GLocalFileOutputStream_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GLocalFileOutputStream_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_local_file_output_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GLocalFileOutputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GLocalFileOutputStream_private_offset,
        );
    }
    safe_c2rust_g_local_file_output_stream_class_init(klass as *mut GLocalFileOutputStreamClass);
}
pub const BACKUP_EXTENSION: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"~\0") };
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_finalize(mut object: *mut GObject) {
    let mut file: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    file = object as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    g_free((*(*file).priv_0).tmp_filename as gpointer);
    g_free((*(*file).priv_0).original_filename as gpointer);
    g_free((*(*file).priv_0).backup_filename as gpointer);
    g_free((*(*file).priv_0).etag as gpointer);
    (*(safe_c2rust_g_local_file_output_stream_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_class_init(
    mut klass: *mut GLocalFileOutputStreamClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut stream_class: *mut GOutputStreamClass =
        klass as *mut ::core::ffi::c_void as *mut GOutputStreamClass;
    let mut file_stream_class: *mut GFileOutputStreamClass =
        klass as *mut ::core::ffi::c_void as *mut GFileOutputStreamClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_local_file_output_stream_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*stream_class).write_fn = Some(
        safe_c2rust_g_local_file_output_stream_write
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *const ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *const ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*stream_class).writev_fn = Some(
        safe_c2rust_g_local_file_output_stream_writev
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *const GOutputVector,
                gsize,
                *mut gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *const GOutputVector,
                gsize,
                *mut gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*stream_class).close_fn = Some(
        safe_c2rust_g_local_file_output_stream_close
            as unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GOutputStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*file_stream_class).query_info = Some(
        safe_c2rust_g_local_file_output_stream_query_info
            as unsafe extern "C" fn(
                *mut GFileOutputStream,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileOutputStream,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
    (*file_stream_class).get_etag = Some(
        safe_c2rust_g_local_file_output_stream_get_etag
            as unsafe extern "C" fn(*mut GFileOutputStream) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFileOutputStream) -> *mut ::core::ffi::c_char>;
    (*file_stream_class).tell = Some(
        safe_c2rust_g_local_file_output_stream_tell
            as unsafe extern "C" fn(*mut GFileOutputStream) -> goffset,
    )
        as Option<unsafe extern "C" fn(*mut GFileOutputStream) -> goffset>;
    (*file_stream_class).can_seek = Some(
        safe_c2rust_g_local_file_output_stream_can_seek
            as unsafe extern "C" fn(*mut GFileOutputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFileOutputStream) -> gboolean>;
    (*file_stream_class).seek = Some(
        safe_c2rust_g_local_file_output_stream_seek
            as unsafe extern "C" fn(
                *mut GFileOutputStream,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileOutputStream,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*file_stream_class).can_truncate = Some(
        safe_c2rust_g_local_file_output_stream_can_truncate
            as unsafe extern "C" fn(*mut GFileOutputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFileOutputStream) -> gboolean>;
    (*file_stream_class).truncate_fn = Some(
        safe_c2rust_g_local_file_output_stream_truncate
            as unsafe extern "C" fn(
                *mut GFileOutputStream,
                goffset,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileOutputStream,
                goffset,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_file_descriptor_based_iface_init(
    mut iface: *mut GFileDescriptorBasedIface,
) {
    (*iface).get_fd = Some(
        safe_c2rust_g_local_file_output_stream_get_fd
            as unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int,
    )
        as Option<unsafe extern "C" fn(*mut GFileDescriptorBased) -> ::core::ffi::c_int>;
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_init(
    mut stream: *mut GLocalFileOutputStream,
) {
    (*stream).priv_0 = safe_c2rust_g_local_file_output_stream_get_instance_private(stream)
        as *mut GLocalFileOutputStreamPrivate;
    (*(*stream).priv_0).set_do_close(TRUE as guint as guint);
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_write(
    mut stream: *mut GOutputStream,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut file: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    let mut res: gssize = 0;
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    loop {
        if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
            return -(1 as ::core::ffi::c_int) as gssize;
        }
        res = write((*(*file).priv_0).fd, buffer, count as size_t) as gssize;
        if !(res == -(1 as ::core::ffi::c_int) as gssize) {
            break;
        }
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        if errsv == EINTR {
            continue;
        }
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error writing to file: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
        break;
    }
    return res;
}
pub const G_OUTPUT_VECTOR_IS_IOVEC: ::core::ffi::c_int = (::core::mem::size_of::<iovec>() as usize
    == ::core::mem::size_of::<GOutputVector>() as usize
    && ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize
        == ::core::mem::size_of::<gconstpointer>() as usize
    && 0 as ::core::ffi::c_ulong as glong == 0 as ::core::ffi::c_ulong as glong
    && ::core::mem::size_of::<size_t>() as usize == ::core::mem::size_of::<gsize>() as usize
    && 8 as ::core::ffi::c_ulong as glong == 8 as ::core::ffi::c_ulong as glong)
    as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_writev(
    mut stream: *mut GOutputStream,
    mut vectors: *const GOutputVector,
    mut n_vectors: gsize,
    mut bytes_written: *mut gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut file: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    let mut res: gssize = 0;
    let mut iov: *mut iovec = ::core::ptr::null_mut::<iovec>();
    if !bytes_written.is_null() {
        *bytes_written = 0 as gsize;
    }
    if n_vectors > G_IOV_MAX as gsize {
        n_vectors = G_IOV_MAX as gsize;
    }
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    if G_OUTPUT_VECTOR_IS_IOVEC != 0 {
        iov = vectors as *mut iovec;
    } else {
        let mut i: gsize = 0;
        alloca_allocations.push(::std::vec::from_elem(
            0,
            (::core::mem::size_of::<iovec>() as usize).wrapping_mul(n_vectors as usize) as usize,
        ));
        iov = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut iovec;
        i = 0 as gsize;
        while i < n_vectors {
            let ref mut fresh0 = (*iov.offset(i as isize)).iov_base;
            *fresh0 = (*vectors.offset(i as isize)).buffer as *mut ::core::ffi::c_void;
            (*iov.offset(i as isize)).iov_len = (*vectors.offset(i as isize)).size as size_t;
            i = i.wrapping_add(1);
        }
    }
    loop {
        if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
            return FALSE;
        }
        res = writev((*(*file).priv_0).fd, iov, n_vectors as ::core::ffi::c_int) as gssize;
        if res == -(1 as ::core::ffi::c_int) as gssize {
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            if errsv == EINTR {
                continue;
            }
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                glib_gettext(b"Error writing to file: %s\0" as *const u8 as *const gchar),
                g_strerror(errsv as gint),
            );
            break;
        } else {
            if !bytes_written.is_null() {
                *bytes_written = res as gsize;
            }
            break;
        }
    }
    return (res != -(1 as ::core::ffi::c_int) as gssize) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_output_stream_set_do_close(
    mut out: *mut GLocalFileOutputStream,
    mut do_close: gboolean,
) {
    (*(*out).priv_0).set_do_close(do_close as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_output_stream_really_close(
    mut file: *mut GLocalFileOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut final_stat: statx = statx {
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
    if (*(*file).priv_0).sync_on_close() as ::core::ffi::c_int != 0
        && fsync((*(*file).priv_0).fd) != 0 as ::core::ffi::c_int
    {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error writing to file: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
    } else {
        if !(*(*file).priv_0).tmp_filename.is_null() {
            if !(*(*file).priv_0).backup_filename.is_null() {
                if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
                    current_block = 3756274531319868170;
                } else if g_unlink((*(*file).priv_0).backup_filename) != 0 as ::core::ffi::c_int
                    && *__errno_location() != ENOENT
                {
                    let mut errsv_0: ::core::ffi::c_int = *__errno_location();
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_CANT_CREATE_BACKUP as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Error removing old backup link: %s\0" as *const u8 as *const gchar,
                        ),
                        g_strerror(errsv_0 as gint),
                    );
                    current_block = 3756274531319868170;
                } else if link(
                    (*(*file).priv_0).original_filename,
                    (*(*file).priv_0).backup_filename,
                ) != 0 as ::core::ffi::c_int
                {
                    if rename(
                        (*(*file).priv_0).original_filename,
                        (*(*file).priv_0).backup_filename,
                    ) != 0 as ::core::ffi::c_int
                    {
                        let mut errsv_1: ::core::ffi::c_int = *__errno_location();
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_CANT_CREATE_BACKUP as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Error creating backup copy: %s\0" as *const u8 as *const gchar,
                            ),
                            g_strerror(errsv_1 as gint),
                        );
                        current_block = 3756274531319868170;
                    } else {
                        current_block = 12209867499936983673;
                    }
                } else {
                    current_block = 12209867499936983673;
                }
            } else {
                current_block = 12209867499936983673;
            }
            match current_block {
                3756274531319868170 => {}
                _ => {
                    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
                        current_block = 3756274531319868170;
                    } else if rename(
                        (*(*file).priv_0).tmp_filename,
                        (*(*file).priv_0).original_filename,
                    ) != 0 as ::core::ffi::c_int
                    {
                        let mut errsv_2: ::core::ffi::c_int = *__errno_location();
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            g_io_error_from_errno(errsv_2 as gint) as gint,
                            glib_gettext(
                                b"Error renaming temporary file: %s\0" as *const u8 as *const gchar,
                            ),
                            g_strerror(errsv_2 as gint),
                        );
                        current_block = 3756274531319868170;
                    } else {
                        let mut _pp: *mut *mut ::core::ffi::c_char =
                            &raw mut (*(*file).priv_0).tmp_filename;
                        let mut _ptr: *mut ::core::ffi::c_char = *_pp;
                        *_pp = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        if !_ptr.is_null() {
                            g_free(_ptr as gpointer);
                        }
                        current_block = 4495394744059808450;
                    }
                }
            }
        } else {
            current_block = 4495394744059808450;
        }
        match current_block {
            3756274531319868170 => {}
            _ => {
                if !(g_cancellable_set_error_if_cancelled(cancellable, error) != 0) {
                    if safe_c2rust_g_local_file_fstat(
                        (*(*file).priv_0).fd,
                        G_LOCAL_FILE_STAT_FIELD_MTIME,
                        4095 as GLocalFileStatField,
                        &raw mut final_stat,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*(*file).priv_0).etag =
                            _g_local_file_info_create_etag(&raw mut final_stat);
                    }
                    if g_close(
                        (*(*file).priv_0).fd as gint,
                        ::core::ptr::null_mut::<*mut GError>(),
                    ) == 0
                    {
                        let mut errsv_3: ::core::ffi::c_int = *__errno_location();
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            g_io_error_from_errno(errsv_3 as gint) as gint,
                            glib_gettext(b"Error closing file: %s\0" as *const u8 as *const gchar),
                            g_strerror(errsv_3 as gint),
                        );
                    } else {
                        return TRUE;
                    }
                }
            }
        }
    }
    g_close(
        (*(*file).priv_0).fd as gint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !(*(*file).priv_0).tmp_filename.is_null() {
        g_unlink((*(*file).priv_0).tmp_filename);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_close(
    mut stream: *mut GOutputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut file: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    if (*(*file).priv_0).do_close() != 0 {
        return safe_c2rust__g_local_file_output_stream_really_close(file, cancellable, error);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_get_etag(
    mut stream: *mut GFileOutputStream,
) -> *mut ::core::ffi::c_char {
    let mut file: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    return safe_c2rust_g_strdup_inline((*(*file).priv_0).etag);
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_tell(
    mut stream: *mut GFileOutputStream,
) -> goffset {
    let mut file: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    let mut pos: off_t = 0;
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    pos = lseek((*(*file).priv_0).fd, 0 as __off64_t, SEEK_CUR) as off_t;
    if pos == -(1 as ::core::ffi::c_int) as off_t {
        return 0 as goffset;
    }
    return pos as goffset;
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_can_seek(
    mut stream: *mut GFileOutputStream,
) -> gboolean {
    let mut file: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    let mut pos: off_t = 0;
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    pos = lseek((*(*file).priv_0).fd, 0 as __off64_t, SEEK_CUR) as off_t;
    if pos == -(1 as ::core::ffi::c_int) as off_t && *__errno_location() == ESPIPE {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_seek_type_to_lseek(mut type_0: GSeekType) -> ::core::ffi::c_int {
    match type_0 as ::core::ffi::c_uint {
        1 => return SEEK_SET,
        2 => return SEEK_END,
        0 | _ => return SEEK_CUR,
    };
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_seek(
    mut stream: *mut GFileOutputStream,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut file: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    let mut pos: off_t = 0;
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    pos = lseek(
        (*(*file).priv_0).fd,
        offset as __off64_t,
        safe_c2rust_seek_type_to_lseek(type_0),
    ) as off_t;
    if pos == -(1 as ::core::ffi::c_int) as off_t {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error seeking in file: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_can_truncate(
    mut stream: *mut GFileOutputStream,
) -> gboolean {
    return safe_c2rust_g_local_file_output_stream_can_seek(stream);
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_truncate(
    mut stream: *mut GFileOutputStream,
    mut size: goffset,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut file: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    let mut res: ::core::ffi::c_int = 0;
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    loop {
        res = ftruncate((*(*file).priv_0).fd, size as __off64_t);
        if res == -(1 as ::core::ffi::c_int) {
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            if errsv == EINTR {
                if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
                    return FALSE;
                }
            } else {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    g_io_error_from_errno(errsv as gint) as gint,
                    glib_gettext(b"Error truncating file: %s\0" as *const u8 as *const gchar),
                    g_strerror(errsv as gint),
                );
                return FALSE;
            }
        } else {
            return TRUE;
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_query_info(
    mut stream: *mut GFileOutputStream,
    mut attributes: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut file: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    file = stream as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    return _g_local_file_info_get_from_fd((*(*file).priv_0).fd, attributes, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_output_stream_new(
    mut fd: ::core::ffi::c_int,
) -> *mut GFileOutputStream {
    let mut stream: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    stream = g_object_new(
        safe_c2rust__g_local_file_output_stream_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GLocalFileOutputStream;
    (*(*stream).priv_0).fd = fd;
    return stream as *mut ::core::ffi::c_void as *mut GFileOutputStream;
}
unsafe extern "C" fn safe_c2rust_set_error_from_open_errno(
    mut filename: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) {
    let mut errsv: ::core::ffi::c_int = *__errno_location();
    if errsv == EINVAL {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_FILENAME as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid filename\0" as *const u8 as *const gchar),
        );
    } else {
        let mut display_name: *mut ::core::ffi::c_char =
            g_filename_display_name(filename as *const gchar) as *mut ::core::ffi::c_char;
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Error opening file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8 as *const gchar,
            ),
            display_name,
            g_strerror(errsv as gint),
        );
        g_free(display_name as gpointer);
    };
}
unsafe extern "C" fn safe_c2rust_output_stream_open(
    mut filename: *const ::core::ffi::c_char,
    mut open_flags: gint,
    mut mode: guint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut stream: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    let mut fd: gint = 0;
    fd = open(filename, open_flags as ::core::ffi::c_int, mode) as gint;
    if fd == -(1 as ::core::ffi::c_int) {
        safe_c2rust_set_error_from_open_errno(filename, error);
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    stream = g_object_new(
        safe_c2rust__g_local_file_output_stream_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GLocalFileOutputStream;
    (*(*stream).priv_0).fd = fd as ::core::ffi::c_int;
    return stream as *mut ::core::ffi::c_void as *mut GFileOutputStream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_output_stream_open(
    mut filename: *const ::core::ffi::c_char,
    mut readable: gboolean,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut open_flags: ::core::ffi::c_int = 0;
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    open_flags = O_BINARY | O_CLOEXEC;
    if readable != 0 {
        open_flags |= O_RDWR;
    } else {
        open_flags |= O_WRONLY;
    }
    return safe_c2rust_output_stream_open(
        filename,
        open_flags as gint,
        0o666 as guint,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_mode_from_flags_or_info(
    mut flags: GFileCreateFlags,
    mut reference_info: *mut GFileInfo,
) -> gint {
    if flags as ::core::ffi::c_uint
        & G_FILE_CREATE_PRIVATE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        return 0o600 as gint;
    } else if !reference_info.is_null()
        && g_file_info_has_attribute(
            reference_info,
            b"unix::mode\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        return (g_file_info_get_attribute_uint32(
            reference_info,
            b"unix::mode\0" as *const u8 as *const ::core::ffi::c_char,
        ) & !S_IFMT as guint32) as gint;
    } else {
        return 0o666 as gint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_output_stream_create(
    mut filename: *const ::core::ffi::c_char,
    mut readable: gboolean,
    mut flags: GFileCreateFlags,
    mut reference_info: *mut GFileInfo,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut mode: ::core::ffi::c_int = 0;
    let mut open_flags: ::core::ffi::c_int = 0;
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    mode = safe_c2rust_mode_from_flags_or_info(flags, reference_info) as ::core::ffi::c_int;
    open_flags = O_CREAT | O_EXCL | O_BINARY | O_CLOEXEC;
    if readable != 0 {
        open_flags |= O_RDWR;
    } else {
        open_flags |= O_WRONLY;
    }
    return safe_c2rust_output_stream_open(
        filename,
        open_flags as gint,
        mode as guint,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_output_stream_append(
    mut filename: *const ::core::ffi::c_char,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut mode: ::core::ffi::c_int = 0;
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    if flags as ::core::ffi::c_uint
        & G_FILE_CREATE_PRIVATE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        mode = 0o600 as ::core::ffi::c_int;
    } else {
        mode = 0o666 as ::core::ffi::c_int;
    }
    return safe_c2rust_output_stream_open(
        filename,
        O_CREAT | O_APPEND | O_WRONLY | O_BINARY | O_CLOEXEC,
        mode as guint,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_create_backup_filename(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    return g_strconcat(filename as *const gchar, BACKUP_EXTENSION.as_ptr(), NULL_0)
        as *mut ::core::ffi::c_char;
}
pub const BUFSIZE: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_copy_file_data(
    mut sfd: gint,
    mut dfd: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = TRUE;
    let mut buffer: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut write_buffer: *const gchar = ::core::ptr::null::<gchar>();
    let mut bytes_read: gssize = 0;
    let mut bytes_to_write: gssize = 0;
    let mut bytes_written: gssize = 0;
    buffer = g_malloc(BUFSIZE as gsize);
    loop {
        bytes_read = read(
            sfd as ::core::ffi::c_int,
            buffer as *mut ::core::ffi::c_void,
            BUFSIZE as size_t,
        ) as gssize;
        if bytes_read == -(1 as ::core::ffi::c_int) as gssize {
            let mut errsv: ::core::ffi::c_int = *__errno_location();
            if !(errsv == EINTR) {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    g_io_error_from_errno(errsv as gint) as gint,
                    glib_gettext(b"Error reading from file: %s\0" as *const u8 as *const gchar),
                    g_strerror(errsv as gint),
                );
                ret = FALSE as gboolean;
                break;
            }
        } else {
            bytes_to_write = bytes_read;
            write_buffer = buffer as *const gchar;
            loop {
                bytes_written = write(
                    dfd as ::core::ffi::c_int,
                    write_buffer as *const ::core::ffi::c_void,
                    bytes_to_write as size_t,
                ) as gssize;
                if bytes_written == -(1 as ::core::ffi::c_int) as gssize {
                    let mut errsv_0: ::core::ffi::c_int = *__errno_location();
                    if !(errsv_0 == EINTR) {
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            g_io_error_from_errno(errsv_0 as gint) as gint,
                            glib_gettext(
                                b"Error writing to file: %s\0" as *const u8 as *const gchar,
                            ),
                            g_strerror(errsv_0 as gint),
                        );
                        ret = FALSE as gboolean;
                        break;
                    }
                } else {
                    bytes_to_write -= bytes_written;
                    write_buffer = write_buffer.offset(bytes_written as isize);
                }
                if !(bytes_to_write > 0 as gssize) {
                    break;
                }
            }
        }
        if !(bytes_read != 0 as gssize && ret == TRUE) {
            break;
        }
    }
    g_free(buffer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_handle_overwrite_open(
    mut filename: *const ::core::ffi::c_char,
    mut readable: gboolean,
    mut etag: *const ::core::ffi::c_char,
    mut create_backup: gboolean,
    mut temp_filename: *mut *mut ::core::ffi::c_char,
    mut flags: GFileCreateFlags,
    mut reference_info: *mut GFileInfo,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut original_stat: statx = statx {
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
    let mut current_etag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut is_symlink: gboolean = 0;
    let mut open_flags: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut mode: ::core::ffi::c_int = 0;
    let mut errsv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut replace_destination_set: gboolean = (flags as ::core::ffi::c_uint
        & G_FILE_CREATE_REPLACE_DESTINATION as ::core::ffi::c_int as ::core::ffi::c_uint)
        as gboolean;
    mode = safe_c2rust_mode_from_flags_or_info(flags, reference_info) as ::core::ffi::c_int;
    if create_backup != 0 || readable != 0 {
        open_flags = O_RDWR | O_CREAT | O_BINARY | O_CLOEXEC;
    } else {
        open_flags = O_WRONLY | O_CREAT | O_BINARY | O_CLOEXEC;
    }
    is_symlink = FALSE as gboolean;
    fd = open(filename, open_flags | O_NOFOLLOW, mode);
    errsv = *__errno_location();
    if fd == -(1 as ::core::ffi::c_int) && errsv == ELOOP {
        is_symlink = TRUE as gboolean;
        if replace_destination_set == 0 {
            fd = open(filename, open_flags, mode);
        }
    }
    if fd == -(1 as ::core::ffi::c_int) && (is_symlink == 0 || replace_destination_set == 0) {
        let mut display_name: *mut ::core::ffi::c_char =
            g_filename_display_name(filename as *const gchar) as *mut ::core::ffi::c_char;
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Error opening file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8 as *const gchar,
            ),
            display_name,
            g_strerror(errsv as gint),
        );
        g_free(display_name as gpointer);
        return -(1 as ::core::ffi::c_int);
    }
    if is_symlink == 0 {
        res = safe_c2rust_g_local_file_fstat(
            fd,
            (G_LOCAL_FILE_STAT_FIELD_TYPE as ::core::ffi::c_int
                | G_LOCAL_FILE_STAT_FIELD_MODE as ::core::ffi::c_int
                | G_LOCAL_FILE_STAT_FIELD_UID as ::core::ffi::c_int
                | G_LOCAL_FILE_STAT_FIELD_GID as ::core::ffi::c_int
                | G_LOCAL_FILE_STAT_FIELD_MTIME as ::core::ffi::c_int
                | G_LOCAL_FILE_STAT_FIELD_NLINK as ::core::ffi::c_int)
                as GLocalFileStatField,
            4095 as GLocalFileStatField,
            &raw mut original_stat,
        );
        errsv = *__errno_location();
    } else {
        res = safe_c2rust_g_local_file_lstat(
            filename,
            (G_LOCAL_FILE_STAT_FIELD_TYPE as ::core::ffi::c_int
                | G_LOCAL_FILE_STAT_FIELD_MODE as ::core::ffi::c_int
                | G_LOCAL_FILE_STAT_FIELD_UID as ::core::ffi::c_int
                | G_LOCAL_FILE_STAT_FIELD_GID as ::core::ffi::c_int
                | G_LOCAL_FILE_STAT_FIELD_MTIME as ::core::ffi::c_int
                | G_LOCAL_FILE_STAT_FIELD_NLINK as ::core::ffi::c_int)
                as GLocalFileStatField,
            4095 as GLocalFileStatField,
            &raw mut original_stat,
        );
        errsv = *__errno_location();
    }
    if res != 0 as ::core::ffi::c_int {
        let mut display_name_0: *mut ::core::ffi::c_char =
            g_filename_display_name(filename as *const gchar) as *mut ::core::ffi::c_char;
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Error when getting information for file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                    as *const u8 as *const gchar,
            ),
            display_name_0,
            g_strerror(errsv as gint),
        );
        g_free(display_name_0 as gpointer);
    } else {
        if !(safe_c2rust__g_stat_mode(&raw mut original_stat) as ::core::ffi::c_int & __S_IFMT
            == 0o100000 as ::core::ffi::c_int)
        {
            if safe_c2rust__g_stat_mode(&raw mut original_stat) as ::core::ffi::c_int & __S_IFMT
                == 0o40000 as ::core::ffi::c_int
            {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_IS_DIRECTORY as ::core::ffi::c_int as gint,
                    glib_gettext(b"Target file is a directory\0" as *const u8 as *const gchar),
                );
                current_block = 17335395020742461848;
            } else if is_symlink == 0
                || !(safe_c2rust__g_stat_mode(&raw mut original_stat) as ::core::ffi::c_int
                    & __S_IFMT
                    == 0o120000 as ::core::ffi::c_int)
            {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_NOT_REGULAR_FILE as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Target file is not a regular file\0" as *const u8 as *const gchar,
                    ),
                );
                current_block = 17335395020742461848;
            } else {
                current_block = 4488286894823169796;
            }
        } else {
            current_block = 4488286894823169796;
        }
        match current_block {
            17335395020742461848 => {}
            _ => {
                if !etag.is_null() {
                    let mut etag_stat: statx = statx {
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
                    let mut etag_stat_pointer: *mut statx = ::core::ptr::null_mut::<statx>();
                    if is_symlink != 0 {
                        res = safe_c2rust_g_local_file_stat(
                            filename,
                            G_LOCAL_FILE_STAT_FIELD_MTIME,
                            4095 as GLocalFileStatField,
                            &raw mut etag_stat,
                        );
                        errsv = *__errno_location();
                        if res != 0 as ::core::ffi::c_int {
                            let mut display_name_1: *mut ::core::ffi::c_char =
                                g_filename_display_name(filename as *const gchar)
                                    as *mut ::core::ffi::c_char;
                            g_set_error(
                                error,
                                g_io_error_quark(),
                                g_io_error_from_errno(errsv as gint) as gint,
                                glib_gettext(
                                    b"Error when getting information for file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                                        as *const u8 as *const gchar,
                                ),
                                display_name_1,
                                g_strerror(errsv as gint),
                            );
                            g_free(display_name_1 as gpointer);
                            current_block = 17335395020742461848;
                        } else {
                            etag_stat_pointer = &raw mut etag_stat;
                            current_block = 4090602189656566074;
                        }
                    } else {
                        etag_stat_pointer = &raw mut original_stat;
                        current_block = 4090602189656566074;
                    }
                    match current_block {
                        17335395020742461848 => {}
                        _ => {
                            current_etag = _g_local_file_info_create_etag(etag_stat_pointer);
                            if strcmp(etag, current_etag) != 0 as ::core::ffi::c_int {
                                g_set_error_literal(
                                    error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_WRONG_ETAG as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"The file was externally modified\0" as *const u8
                                            as *const gchar,
                                    ),
                                );
                                g_free(current_etag as gpointer);
                                current_block = 17335395020742461848;
                            } else {
                                g_free(current_etag as gpointer);
                                current_block = 15512526488502093901;
                            }
                        }
                    }
                } else {
                    current_block = 15512526488502093901;
                }
                match current_block {
                    17335395020742461848 => {}
                    _ => {
                        if replace_destination_set != 0
                            || !(safe_c2rust__g_stat_nlink(&raw mut original_stat) > 1 as guint32)
                                && is_symlink == 0
                        {
                            let mut dirname: *mut ::core::ffi::c_char =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            let mut tmp_filename: *mut ::core::ffi::c_char =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            let mut tmpfd: ::core::ffi::c_int = 0;
                            dirname = g_path_get_dirname(filename as *const gchar)
                                as *mut ::core::ffi::c_char;
                            tmp_filename = g_build_filename(
                                dirname,
                                b".goutputstream-XXXXXX\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                NULL_0,
                            )
                                as *mut ::core::ffi::c_char;
                            g_free(dirname as gpointer);
                            tmpfd = g_mkstemp_full(
                                tmp_filename as *mut gchar,
                                (if readable != 0 { O_RDWR } else { O_WRONLY })
                                    | O_BINARY
                                    | O_CLOEXEC,
                                mode as gint,
                            ) as ::core::ffi::c_int;
                            if tmpfd == -(1 as ::core::ffi::c_int) {
                                g_free(tmp_filename as gpointer);
                            } else {
                                if replace_destination_set == 0
                                    && (fchown(
                                        tmpfd,
                                        safe_c2rust__g_stat_uid(&raw mut original_stat) as __uid_t,
                                        safe_c2rust__g_stat_gid(&raw mut original_stat) as __gid_t,
                                    ) == -(1 as ::core::ffi::c_int)
                                        || fchmod(
                                            tmpfd,
                                            (safe_c2rust__g_stat_mode(&raw mut original_stat)
                                                as ::core::ffi::c_int
                                                & !S_IFMT)
                                                as __mode_t,
                                        ) == -(1 as ::core::ffi::c_int)
                                        || 0 as ::core::ffi::c_int != 0)
                                {
                                    let mut tmp_statbuf: statx = statx {
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
                                    let mut tres: ::core::ffi::c_int = 0;
                                    tres = safe_c2rust_g_local_file_fstat(
                                        tmpfd,
                                        (G_LOCAL_FILE_STAT_FIELD_TYPE as ::core::ffi::c_int
                                            | G_LOCAL_FILE_STAT_FIELD_MODE as ::core::ffi::c_int
                                            | G_LOCAL_FILE_STAT_FIELD_UID as ::core::ffi::c_int
                                            | G_LOCAL_FILE_STAT_FIELD_GID as ::core::ffi::c_int)
                                            as GLocalFileStatField,
                                        4095 as GLocalFileStatField,
                                        &raw mut tmp_statbuf,
                                    );
                                    if tres != 0 as ::core::ffi::c_int
                                        || safe_c2rust__g_stat_uid(&raw mut original_stat)
                                            != safe_c2rust__g_stat_uid(&raw mut tmp_statbuf)
                                        || safe_c2rust__g_stat_gid(&raw mut original_stat)
                                            != safe_c2rust__g_stat_gid(&raw mut tmp_statbuf)
                                        || safe_c2rust__g_stat_mode(&raw mut original_stat)
                                            as ::core::ffi::c_int
                                            != safe_c2rust__g_stat_mode(&raw mut tmp_statbuf)
                                                as ::core::ffi::c_int
                                    {
                                        g_close(
                                            tmpfd as gint,
                                            ::core::ptr::null_mut::<*mut GError>(),
                                        );
                                        g_unlink(tmp_filename);
                                        g_free(tmp_filename as gpointer);
                                        current_block = 1046567527591038642;
                                    } else {
                                        current_block = 13763002826403452995;
                                    }
                                } else {
                                    current_block = 13763002826403452995;
                                }
                                match current_block {
                                    1046567527591038642 => {}
                                    _ => {
                                        if fd >= 0 as ::core::ffi::c_int {
                                            g_close(
                                                fd as gint,
                                                ::core::ptr::null_mut::<*mut GError>(),
                                            );
                                        }
                                        *temp_filename = tmp_filename;
                                        return tmpfd;
                                    }
                                }
                            }
                        }
                        if create_backup != 0 {
                            let mut tmp_statbuf_0: statx = statx {
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
                            let mut backup_filename: *mut ::core::ffi::c_char =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            let mut bfd: ::core::ffi::c_int = 0;
                            backup_filename = safe_c2rust_create_backup_filename(filename);
                            if g_unlink(backup_filename) == -(1 as ::core::ffi::c_int)
                                && *__errno_location() != ENOENT
                            {
                                g_set_error_literal(
                                    error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_CANT_CREATE_BACKUP as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"Backup file creation failed\0" as *const u8
                                            as *const gchar,
                                    ),
                                );
                                g_free(backup_filename as gpointer);
                                current_block = 17335395020742461848;
                            } else {
                                bfd = open(
                                    backup_filename,
                                    O_WRONLY | O_CREAT | O_EXCL | O_BINARY | O_CLOEXEC,
                                    safe_c2rust__g_stat_mode(&raw mut original_stat)
                                        as ::core::ffi::c_int
                                        & 0o777 as ::core::ffi::c_int,
                                );
                                if bfd == -(1 as ::core::ffi::c_int) {
                                    g_set_error_literal(
                                        error,
                                        g_io_error_quark(),
                                        G_IO_ERROR_CANT_CREATE_BACKUP as ::core::ffi::c_int as gint,
                                        glib_gettext(
                                            b"Backup file creation failed\0" as *const u8
                                                as *const gchar,
                                        ),
                                    );
                                    g_free(backup_filename as gpointer);
                                    current_block = 17335395020742461848;
                                } else if safe_c2rust_g_local_file_fstat(
                                    bfd,
                                    G_LOCAL_FILE_STAT_FIELD_GID,
                                    4095 as GLocalFileStatField,
                                    &raw mut tmp_statbuf_0,
                                ) != 0 as ::core::ffi::c_int
                                {
                                    g_set_error_literal(
                                        error,
                                        g_io_error_quark(),
                                        G_IO_ERROR_CANT_CREATE_BACKUP as ::core::ffi::c_int as gint,
                                        glib_gettext(
                                            b"Backup file creation failed\0" as *const u8
                                                as *const gchar,
                                        ),
                                    );
                                    g_unlink(backup_filename);
                                    g_close(bfd as gint, ::core::ptr::null_mut::<*mut GError>());
                                    g_free(backup_filename as gpointer);
                                    current_block = 17335395020742461848;
                                } else {
                                    if safe_c2rust__g_stat_gid(&raw mut original_stat)
                                        != safe_c2rust__g_stat_gid(&raw mut tmp_statbuf_0)
                                        && fchown(
                                            bfd,
                                            -(1 as ::core::ffi::c_int) as __uid_t,
                                            safe_c2rust__g_stat_gid(&raw mut original_stat)
                                                as __gid_t,
                                        ) != 0 as ::core::ffi::c_int
                                    {
                                        if fchmod(
                                            bfd,
                                            (safe_c2rust__g_stat_mode(&raw mut original_stat)
                                                as ::core::ffi::c_int
                                                & 0o707 as ::core::ffi::c_int
                                                | (safe_c2rust__g_stat_mode(&raw mut original_stat)
                                                    as ::core::ffi::c_int
                                                    & 0o7 as ::core::ffi::c_int)
                                                    << 3 as ::core::ffi::c_int)
                                                as __mode_t,
                                        ) != 0 as ::core::ffi::c_int
                                        {
                                            g_set_error_literal(
                                                error,
                                                g_io_error_quark(),
                                                G_IO_ERROR_CANT_CREATE_BACKUP as ::core::ffi::c_int
                                                    as gint,
                                                glib_gettext(
                                                    b"Backup file creation failed\0" as *const u8
                                                        as *const gchar,
                                                ),
                                            );
                                            g_unlink(backup_filename);
                                            g_close(
                                                bfd as gint,
                                                ::core::ptr::null_mut::<*mut GError>(),
                                            );
                                            g_free(backup_filename as gpointer);
                                            current_block = 17335395020742461848;
                                        } else {
                                            current_block = 8834769789432328951;
                                        }
                                    } else {
                                        current_block = 8834769789432328951;
                                    }
                                    match current_block {
                                        17335395020742461848 => {}
                                        _ => {
                                            if safe_c2rust_copy_file_data(
                                                fd as gint,
                                                bfd as gint,
                                                ::core::ptr::null_mut::<*mut GError>(),
                                            ) == 0
                                            {
                                                g_set_error_literal(
                                                    error,
                                                    g_io_error_quark(),
                                                    G_IO_ERROR_CANT_CREATE_BACKUP
                                                        as ::core::ffi::c_int
                                                        as gint,
                                                    glib_gettext(
                                                        b"Backup file creation failed\0"
                                                            as *const u8
                                                            as *const gchar,
                                                    ),
                                                );
                                                g_unlink(backup_filename);
                                                g_close(
                                                    bfd as gint,
                                                    ::core::ptr::null_mut::<*mut GError>(),
                                                );
                                                g_free(backup_filename as gpointer);
                                                current_block = 17335395020742461848;
                                            } else {
                                                g_close(
                                                    bfd as gint,
                                                    ::core::ptr::null_mut::<*mut GError>(),
                                                );
                                                g_free(backup_filename as gpointer);
                                                if lseek(fd, 0 as __off64_t, SEEK_SET)
                                                    == -(1 as ::core::ffi::c_int) as __off64_t
                                                {
                                                    errsv = *__errno_location();
                                                    g_set_error(
                                                        error,
                                                        g_io_error_quark(),
                                                        g_io_error_from_errno(errsv as gint)
                                                            as gint,
                                                        glib_gettext(
                                                            b"Error seeking in file: %s\0"
                                                                as *const u8
                                                                as *const gchar,
                                                        ),
                                                        g_strerror(errsv as gint),
                                                    );
                                                    current_block = 17335395020742461848;
                                                } else {
                                                    current_block = 18325745679564279244;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            current_block = 18325745679564279244;
                        }
                        match current_block {
                            17335395020742461848 => {}
                            _ => {
                                if replace_destination_set != 0 {
                                    g_close(fd as gint, ::core::ptr::null_mut::<*mut GError>());
                                    fd = -(1 as ::core::ffi::c_int);
                                    if g_unlink(filename as *const gchar) != 0 as ::core::ffi::c_int
                                    {
                                        errsv = *__errno_location();
                                        g_set_error(
                                            error,
                                            g_io_error_quark(),
                                            g_io_error_from_errno(errsv as gint) as gint,
                                            glib_gettext(
                                                b"Error removing old file: %s\0" as *const u8
                                                    as *const gchar,
                                            ),
                                            g_strerror(errsv as gint),
                                        );
                                        current_block = 17335395020742461848;
                                    } else {
                                        if readable != 0 {
                                            open_flags = O_RDWR | O_CREAT | O_BINARY | O_CLOEXEC;
                                        } else {
                                            open_flags = O_WRONLY | O_CREAT | O_BINARY | O_CLOEXEC;
                                        }
                                        fd = open(filename, open_flags, mode);
                                        if fd == -(1 as ::core::ffi::c_int) {
                                            let mut display_name_2: *mut ::core::ffi::c_char =
                                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                                            errsv = *__errno_location();
                                            display_name_2 =
                                                g_filename_display_name(filename as *const gchar)
                                                    as *mut ::core::ffi::c_char;
                                            g_set_error(
                                                error,
                                                g_io_error_quark(),
                                                g_io_error_from_errno(errsv as gint) as gint,
                                                glib_gettext(
                                                    b"Error opening file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                                                        as *const u8 as *const gchar,
                                                ),
                                                display_name_2,
                                                g_strerror(errsv as gint),
                                            );
                                            g_free(display_name_2 as gpointer);
                                            current_block = 17335395020742461848;
                                        } else {
                                            current_block = 1934991416718554651;
                                        }
                                    }
                                } else if ftruncate(fd, 0 as __off64_t)
                                    == -(1 as ::core::ffi::c_int)
                                {
                                    errsv = *__errno_location();
                                    g_set_error(
                                        error,
                                        g_io_error_quark(),
                                        g_io_error_from_errno(errsv as gint) as gint,
                                        glib_gettext(
                                            b"Error truncating file: %s\0" as *const u8
                                                as *const gchar,
                                        ),
                                        g_strerror(errsv as gint),
                                    );
                                    current_block = 17335395020742461848;
                                } else {
                                    current_block = 1934991416718554651;
                                }
                                match current_block {
                                    17335395020742461848 => {}
                                    _ => return fd,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if fd >= 0 as ::core::ffi::c_int {
        g_close(fd as gint, ::core::ptr::null_mut::<*mut GError>());
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_output_stream_replace(
    mut filename: *const ::core::ffi::c_char,
    mut readable: gboolean,
    mut etag: *const ::core::ffi::c_char,
    mut create_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut reference_info: *mut GFileInfo,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    let mut stream: *mut GLocalFileOutputStream = ::core::ptr::null_mut::<GLocalFileOutputStream>();
    let mut mode: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    let mut temp_file: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sync_on_close: gboolean = 0;
    let mut open_flags: ::core::ffi::c_int = 0;
    if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    temp_file = ::core::ptr::null_mut::<::core::ffi::c_char>();
    mode = safe_c2rust_mode_from_flags_or_info(flags, reference_info) as ::core::ffi::c_int;
    sync_on_close = FALSE as gboolean;
    open_flags = O_CREAT | O_EXCL | O_BINARY | O_CLOEXEC;
    if readable != 0 {
        open_flags |= O_RDWR;
    } else {
        open_flags |= O_WRONLY;
    }
    fd = open(filename, open_flags, mode);
    if fd == -(1 as ::core::ffi::c_int) && *__errno_location() == EEXIST {
        fd = safe_c2rust_handle_overwrite_open(
            filename,
            readable,
            etag,
            create_backup,
            &raw mut temp_file,
            flags,
            reference_info,
            cancellable,
            error,
        );
        if fd == -(1 as ::core::ffi::c_int) {
            return ::core::ptr::null_mut::<GFileOutputStream>();
        }
        sync_on_close = TRUE as gboolean;
    } else if fd == -(1 as ::core::ffi::c_int) {
        safe_c2rust_set_error_from_open_errno(filename, error);
        return ::core::ptr::null_mut::<GFileOutputStream>();
    }
    stream = g_object_new(
        safe_c2rust__g_local_file_output_stream_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GLocalFileOutputStream;
    (*(*stream).priv_0).fd = fd;
    (*(*stream).priv_0).set_sync_on_close(sync_on_close as guint as guint);
    (*(*stream).priv_0).tmp_filename = temp_file;
    if create_backup != 0 {
        (*(*stream).priv_0).backup_filename = safe_c2rust_create_backup_filename(filename);
    }
    (*(*stream).priv_0).original_filename = safe_c2rust_g_strdup_inline(filename);
    return stream as *mut ::core::ffi::c_void as *mut GFileOutputStream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_output_stream_get_fd(
    mut stream: *mut GLocalFileOutputStream,
) -> gint {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_local_file_output_stream_get_type();
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
            b"G_IS_LOCAL_FILE_OUTPUT_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    return (*(*stream).priv_0).fd as gint;
}
unsafe extern "C" fn safe_c2rust_g_local_file_output_stream_get_fd(
    mut fd_based: *mut GFileDescriptorBased,
) -> ::core::ffi::c_int {
    let mut stream: *mut GLocalFileOutputStream =
        fd_based as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream;
    return safe_c2rust__g_local_file_output_stream_get_fd(stream) as ::core::ffi::c_int;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
