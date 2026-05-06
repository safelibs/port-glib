extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GFileEnumeratorPrivate;
    pub type _GFileMonitorPrivate;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GFileInputStreamPrivate;
    pub type _GFileOutputStreamPrivate;
    pub type _GFileIOStreamPrivate;
    pub type _GIOStreamPrivate;
    pub type _GMount;
    pub type _GMountOperationPrivate;
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
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_path_get_dirname(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_str_hash(v: gconstpointer) -> guint;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_ascii_tolower(c: gchar) -> gchar;
    fn g_ascii_xdigit_value(c: gchar) -> gint;
    fn g_ascii_strcasecmp(s1: *const gchar, s2: *const gchar) -> gint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_file_get_type() -> GType;
    fn g_file_get_child(file: *mut GFile, name: *const ::core::ffi::c_char) -> *mut GFile;
}
pub type size_t = usize;
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
pub type guchar = ::core::ffi::c_uchar;
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
pub type GFileAttributeType = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_TYPE_STRINGV: GFileAttributeType = 9;
pub const G_FILE_ATTRIBUTE_TYPE_OBJECT: GFileAttributeType = 8;
pub const G_FILE_ATTRIBUTE_TYPE_INT64: GFileAttributeType = 7;
pub const G_FILE_ATTRIBUTE_TYPE_UINT64: GFileAttributeType = 6;
pub const G_FILE_ATTRIBUTE_TYPE_INT32: GFileAttributeType = 5;
pub const G_FILE_ATTRIBUTE_TYPE_UINT32: GFileAttributeType = 4;
pub const G_FILE_ATTRIBUTE_TYPE_BOOLEAN: GFileAttributeType = 3;
pub const G_FILE_ATTRIBUTE_TYPE_BYTE_STRING: GFileAttributeType = 2;
pub const G_FILE_ATTRIBUTE_TYPE_STRING: GFileAttributeType = 1;
pub const G_FILE_ATTRIBUTE_TYPE_INVALID: GFileAttributeType = 0;
pub type GFileAttributeInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED: GFileAttributeInfoFlags = 2;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE: GFileAttributeInfoFlags = 1;
pub const G_FILE_ATTRIBUTE_INFO_NONE: GFileAttributeInfoFlags = 0;
pub type GFileQueryInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS: GFileQueryInfoFlags = 1;
pub const G_FILE_QUERY_INFO_NONE: GFileQueryInfoFlags = 0;
pub type GFileCreateFlags = ::core::ffi::c_uint;
pub const G_FILE_CREATE_REPLACE_DESTINATION: GFileCreateFlags = 2;
pub const G_FILE_CREATE_PRIVATE: GFileCreateFlags = 1;
pub const G_FILE_CREATE_NONE: GFileCreateFlags = 0;
pub type GFileMeasureFlags = ::core::ffi::c_uint;
pub const G_FILE_MEASURE_NO_XDEV: GFileMeasureFlags = 8;
pub const G_FILE_MEASURE_APPARENT_SIZE: GFileMeasureFlags = 4;
pub const G_FILE_MEASURE_REPORT_ANY_ERROR: GFileMeasureFlags = 2;
pub const G_FILE_MEASURE_NONE: GFileMeasureFlags = 0;
pub type GMountMountFlags = ::core::ffi::c_uint;
pub const G_MOUNT_MOUNT_NONE: GMountMountFlags = 0;
pub type GMountUnmountFlags = ::core::ffi::c_uint;
pub const G_MOUNT_UNMOUNT_FORCE: GMountUnmountFlags = 1;
pub const G_MOUNT_UNMOUNT_NONE: GMountUnmountFlags = 0;
pub type GDriveStartFlags = ::core::ffi::c_uint;
pub const G_DRIVE_START_NONE: GDriveStartFlags = 0;
pub type GFileCopyFlags = ::core::ffi::c_uint;
pub const G_FILE_COPY_TARGET_DEFAULT_MODIFIED_TIME: GFileCopyFlags = 64;
pub const G_FILE_COPY_TARGET_DEFAULT_PERMS: GFileCopyFlags = 32;
pub const G_FILE_COPY_NO_FALLBACK_FOR_MOVE: GFileCopyFlags = 16;
pub const G_FILE_COPY_ALL_METADATA: GFileCopyFlags = 8;
pub const G_FILE_COPY_NOFOLLOW_SYMLINKS: GFileCopyFlags = 4;
pub const G_FILE_COPY_BACKUP: GFileCopyFlags = 2;
pub const G_FILE_COPY_OVERWRITE: GFileCopyFlags = 1;
pub const G_FILE_COPY_NONE: GFileCopyFlags = 0;
pub type GFileMonitorFlags = ::core::ffi::c_uint;
pub const G_FILE_MONITOR_WATCH_MOVES: GFileMonitorFlags = 8;
pub const G_FILE_MONITOR_WATCH_HARD_LINKS: GFileMonitorFlags = 4;
pub const G_FILE_MONITOR_SEND_MOVED: GFileMonitorFlags = 2;
pub const G_FILE_MONITOR_WATCH_MOUNTS: GFileMonitorFlags = 1;
pub const G_FILE_MONITOR_NONE: GFileMonitorFlags = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileEnumerator {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileEnumeratorPrivate,
}
pub type GFileEnumeratorPrivate = _GFileEnumeratorPrivate;
pub type GFileEnumerator = _GFileEnumerator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileMonitor {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileMonitorPrivate,
}
pub type GFileMonitorPrivate = _GFileMonitorPrivate;
pub type GFileMonitor = _GFileMonitor;
pub type GFile = _GFile;
pub type GFileInfo = _GFileInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeInfo {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: GFileAttributeType,
    pub flags: GFileAttributeInfoFlags,
}
pub type GFileAttributeInfo = _GFileAttributeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeInfoList {
    pub infos: *mut GFileAttributeInfo,
    pub n_infos: ::core::ffi::c_int,
}
pub type GFileAttributeInfoList = _GFileAttributeInfoList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileInputStream {
    pub parent_instance: GInputStream,
    pub priv_0: *mut GFileInputStreamPrivate,
}
pub type GFileInputStreamPrivate = _GFileInputStreamPrivate;
pub type GFileInputStream = _GFileInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GFileOutputStreamPrivate,
}
pub type GFileOutputStreamPrivate = _GFileOutputStreamPrivate;
pub type GFileOutputStream = _GFileOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileIOStream {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GFileIOStreamPrivate,
}
pub type GFileIOStreamPrivate = _GFileIOStreamPrivate;
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GFileIOStream = _GFileIOStream;
pub type GMount = _GMount;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMountOperation {
    pub parent_instance: GObject,
    pub priv_0: *mut GMountOperationPrivate,
}
pub type GMountOperationPrivate = _GMountOperationPrivate;
pub type GMountOperation = _GMountOperation;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GFileProgressCallback = Option<unsafe extern "C" fn(goffset, goffset, gpointer) -> ()>;
pub type GFileMeasureProgressCallback =
    Option<unsafe extern "C" fn(gboolean, guint64, guint64, guint64, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileIface {
    pub g_iface: GTypeInterface,
    pub dup: Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>,
    pub hash: Option<unsafe extern "C" fn(*mut GFile) -> guint>,
    pub equal: Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>,
    pub is_native: Option<unsafe extern "C" fn(*mut GFile) -> gboolean>,
    pub has_uri_scheme:
        Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> gboolean>,
    pub get_uri_scheme: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_basename: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_path: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_uri: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_parse_name: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_parent: Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>,
    pub prefix_matches: Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>,
    pub get_relative_path:
        Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> *mut ::core::ffi::c_char>,
    pub resolve_relative_path:
        Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> *mut GFile>,
    pub get_child_for_display_name: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> *mut GFile,
    >,
    pub enumerate_children: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileEnumerator,
    >,
    pub enumerate_children_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub enumerate_children_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileEnumerator,
    >,
    pub query_info: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_info_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_info_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileInfo,
    >,
    pub query_filesystem_info: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_filesystem_info_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_filesystem_info_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileInfo,
    >,
    pub find_enclosing_mount: Option<
        unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> *mut GMount,
    >,
    pub find_enclosing_mount_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub find_enclosing_mount_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GMount,
    >,
    pub set_display_name: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFile,
    >,
    pub set_display_name_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub set_display_name_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFile>,
    pub query_settable_attributes: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileAttributeInfoList,
    >,
    pub _query_settable_attributes_async: Option<unsafe extern "C" fn() -> ()>,
    pub _query_settable_attributes_finish: Option<unsafe extern "C" fn() -> ()>,
    pub query_writable_namespaces: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileAttributeInfoList,
    >,
    pub _query_writable_namespaces_async: Option<unsafe extern "C" fn() -> ()>,
    pub _query_writable_namespaces_finish: Option<unsafe extern "C" fn() -> ()>,
    pub set_attribute: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileAttributeType,
            gpointer,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub set_attributes_from_info: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFileInfo,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub set_attributes_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFileInfo,
            GFileQueryInfoFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub set_attributes_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GFileInfo,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub read_fn: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInputStream,
    >,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub read_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileInputStream,
    >,
    pub append_to: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub append_to_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub append_to_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub create: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub create_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub create_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub replace: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub replace_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub replace_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub delete_file:
        Option<unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean>,
    pub delete_file_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub delete_file_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub trash:
        Option<unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean>,
    pub trash_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub trash_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub make_directory:
        Option<unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean>,
    pub make_directory_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub make_directory_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub make_symbolic_link: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub make_symbolic_link_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub make_symbolic_link_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub copy_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub copy_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub move_0: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub move_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub move_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub mount_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountMountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub mount_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFile>,
    pub unmount_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub unmount_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub eject_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub mount_enclosing_volume: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountMountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub mount_enclosing_volume_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub monitor_dir: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMonitorFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileMonitor,
    >,
    pub monitor_file: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMonitorFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileMonitor,
    >,
    pub open_readwrite: Option<
        unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub open_readwrite_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub open_readwrite_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub create_readwrite: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileIOStream,
    >,
    pub create_readwrite_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub create_readwrite_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub replace_readwrite: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileIOStream,
    >,
    pub replace_readwrite_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub replace_readwrite_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub start_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GDriveStartFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub start_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub stop_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub stop_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub supports_thread_contexts: gboolean,
    pub unmount_mountable_with_operation: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub unmount_mountable_with_operation_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub eject_mountable_with_operation: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_mountable_with_operation_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub poll_mountable: Option<
        unsafe extern "C" fn(*mut GFile, *mut GCancellable, GAsyncReadyCallback, gpointer) -> (),
    >,
    pub poll_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub measure_disk_usage: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMeasureFlags,
            *mut GCancellable,
            GFileMeasureProgressCallback,
            gpointer,
            *mut guint64,
            *mut guint64,
            *mut guint64,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub measure_disk_usage_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMeasureFlags,
            gint,
            *mut GCancellable,
            GFileMeasureProgressCallback,
            gpointer,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub measure_disk_usage_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut guint64,
            *mut guint64,
            *mut guint64,
            *mut *mut GError,
        ) -> gboolean,
    >,
}
pub type GFileIface = _GFileIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyFile {
    pub parent_instance: GObject,
    pub decoded_uri: *mut GDecodedUri,
    pub text_uri: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GDecodedUri {
    pub scheme: *mut ::core::ffi::c_char,
    pub userinfo: *mut ::core::ffi::c_char,
    pub host: *mut ::core::ffi::c_char,
    pub port: ::core::ffi::c_int,
    pub path: *mut ::core::ffi::c_char,
    pub query: *mut ::core::ffi::c_char,
    pub fragment: *mut ::core::ffi::c_char,
}
pub type GDummyFile = _GDummyFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDummyFileClass {
    pub parent_class: GObjectClass,
}
pub type GDummyFileClass = _GDummyFileClass;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
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
static mut safe_c2rust_GDummyFile_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dummy_file_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dummy_file_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dummy_file_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDummyFile\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDummyFileClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_file_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDummyFile>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDummyFile) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dummy_file_init as unsafe extern "C" fn(*mut GDummyFile) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GFileIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_dummy_file_file_iface_init as unsafe extern "C" fn(*mut GFileIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_file_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dummy_file_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDummyFile_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GDummyFile_private_offset);
    }
    safe_c2rust_g_dummy_file_class_init(klass as *mut GDummyFileClass);
}
static mut safe_c2rust_g_dummy_file_parent_class: gpointer = NULL_1;
unsafe extern "C" fn safe_c2rust_g_dummy_file_finalize(mut object: *mut GObject) {
    let mut dummy: *mut GDummyFile = ::core::ptr::null_mut::<GDummyFile>();
    dummy = object as *mut ::core::ffi::c_void as *mut GDummyFile;
    if !(*dummy).decoded_uri.is_null() {
        safe_c2rust__g_decoded_uri_free((*dummy).decoded_uri);
    }
    g_free((*dummy).text_uri as gpointer);
    (*(safe_c2rust_g_dummy_file_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_class_init(mut klass: *mut GDummyFileClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_dummy_file_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_init(mut dummy: *mut GDummyFile) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dummy_file_new(
    mut uri: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut dummy: *mut GDummyFile = ::core::ptr::null_mut::<GDummyFile>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !uri.is_null() {
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
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    dummy = g_object_new(
        safe_c2rust__g_dummy_file_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GDummyFile;
    (*dummy).text_uri = safe_c2rust_g_strdup_inline(uri);
    (*dummy).decoded_uri = safe_c2rust__g_decode_uri(uri);
    return dummy as *mut ::core::ffi::c_void as *mut GFile;
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_is_native(mut file: *mut GFile) -> gboolean {
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_get_basename(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut dummy: *mut GDummyFile = file as *mut ::core::ffi::c_void as *mut GDummyFile;
    if !(*dummy).decoded_uri.is_null() {
        return g_path_get_basename((*(*dummy).decoded_uri).path) as *mut ::core::ffi::c_char;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_get_path(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_get_uri(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    return safe_c2rust_g_strdup_inline(
        (*(file as *mut ::core::ffi::c_void as *mut GDummyFile)).text_uri,
    );
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_get_parse_name(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    return safe_c2rust_g_strdup_inline(
        (*(file as *mut ::core::ffi::c_void as *mut GDummyFile)).text_uri,
    );
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_get_parent(mut file: *mut GFile) -> *mut GFile {
    let mut dummy: *mut GDummyFile = file as *mut ::core::ffi::c_void as *mut GDummyFile;
    let mut parent: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut dirname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut uri: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut new_decoded_uri: GDecodedUri = GDecodedUri {
        scheme: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        userinfo: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        host: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        port: 0,
        path: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        query: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        fragment: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if (*dummy).decoded_uri.is_null()
        || g_strcmp0(
            (*(*dummy).decoded_uri).path,
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<GFile>();
    }
    dirname = g_path_get_dirname((*(*dummy).decoded_uri).path) as *mut ::core::ffi::c_char;
    if strcmp(dirname, b".\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        g_free(dirname as gpointer);
        return ::core::ptr::null_mut::<GFile>();
    }
    new_decoded_uri = *(*dummy).decoded_uri;
    new_decoded_uri.path = dirname;
    uri = safe_c2rust__g_encode_uri(&raw mut new_decoded_uri);
    g_free(dirname as gpointer);
    parent = safe_c2rust__g_dummy_file_new(uri);
    g_free(uri as gpointer);
    return parent;
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_dup(mut file: *mut GFile) -> *mut GFile {
    let mut dummy: *mut GDummyFile = file as *mut ::core::ffi::c_void as *mut GDummyFile;
    return safe_c2rust__g_dummy_file_new((*dummy).text_uri);
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_hash(mut file: *mut GFile) -> guint {
    let mut dummy: *mut GDummyFile = file as *mut ::core::ffi::c_void as *mut GDummyFile;
    return g_str_hash((*dummy).text_uri as gconstpointer);
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_equal(
    mut file1: *mut GFile,
    mut file2: *mut GFile,
) -> gboolean {
    let mut dummy1: *mut GDummyFile = file1 as *mut ::core::ffi::c_void as *mut GDummyFile;
    let mut dummy2: *mut GDummyFile = file2 as *mut ::core::ffi::c_void as *mut GDummyFile;
    return (strcmp(
        (*dummy1).text_uri as *const ::core::ffi::c_char,
        (*dummy2).text_uri as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_safe_strcmp(
    mut a: *const ::core::ffi::c_char,
    mut b: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if a.is_null() {
        a = b"\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if b.is_null() {
        b = b"\0" as *const u8 as *const ::core::ffi::c_char;
    }
    return strcmp(a, b);
}
unsafe extern "C" fn safe_c2rust_uri_same_except_path(
    mut a: *mut GDecodedUri,
    mut b: *mut GDecodedUri,
) -> gboolean {
    if safe_c2rust_safe_strcmp((*a).scheme, (*b).scheme) != 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if safe_c2rust_safe_strcmp((*a).userinfo, (*b).userinfo) != 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if safe_c2rust_safe_strcmp((*a).host, (*b).host) != 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if (*a).port != (*b).port {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_match_prefix(
    mut path: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut prefix_len: ::core::ffi::c_int = 0;
    prefix_len = strlen(prefix) as ::core::ffi::c_int;
    if strncmp(path, prefix, prefix_len as size_t) != 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return path.offset(prefix_len as isize);
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_prefix_matches(
    mut parent: *mut GFile,
    mut descendant: *mut GFile,
) -> gboolean {
    let mut parent_dummy: *mut GDummyFile = parent as *mut ::core::ffi::c_void as *mut GDummyFile;
    let mut descendant_dummy: *mut GDummyFile =
        descendant as *mut ::core::ffi::c_void as *mut GDummyFile;
    let mut remainder: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !(*parent_dummy).decoded_uri.is_null() && !(*descendant_dummy).decoded_uri.is_null() {
        if safe_c2rust_uri_same_except_path(
            (*parent_dummy).decoded_uri,
            (*descendant_dummy).decoded_uri,
        ) != 0
        {
            remainder = safe_c2rust_match_prefix(
                (*(*descendant_dummy).decoded_uri).path,
                (*(*parent_dummy).decoded_uri).path,
            );
            if !remainder.is_null() && *remainder as ::core::ffi::c_int == '/' as i32 {
                while *remainder as ::core::ffi::c_int == '/' as i32 {
                    remainder = remainder.offset(1);
                }
                if *remainder as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    return TRUE;
                }
            }
        }
    } else {
        remainder =
            safe_c2rust_match_prefix((*descendant_dummy).text_uri, (*parent_dummy).text_uri);
        if !remainder.is_null() && *remainder as ::core::ffi::c_int == '/' as i32 {
            while *remainder as ::core::ffi::c_int == '/' as i32 {
                remainder = remainder.offset(1);
            }
            if *remainder as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                return TRUE;
            }
        }
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_get_relative_path(
    mut parent: *mut GFile,
    mut descendant: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut parent_dummy: *mut GDummyFile = parent as *mut ::core::ffi::c_void as *mut GDummyFile;
    let mut descendant_dummy: *mut GDummyFile =
        descendant as *mut ::core::ffi::c_void as *mut GDummyFile;
    let mut remainder: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !(*parent_dummy).decoded_uri.is_null() && !(*descendant_dummy).decoded_uri.is_null() {
        if safe_c2rust_uri_same_except_path(
            (*parent_dummy).decoded_uri,
            (*descendant_dummy).decoded_uri,
        ) != 0
        {
            remainder = safe_c2rust_match_prefix(
                (*(*descendant_dummy).decoded_uri).path,
                (*(*parent_dummy).decoded_uri).path,
            );
            if !remainder.is_null() && *remainder as ::core::ffi::c_int == '/' as i32 {
                while *remainder as ::core::ffi::c_int == '/' as i32 {
                    remainder = remainder.offset(1);
                }
                if *remainder as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    return safe_c2rust_g_strdup_inline(remainder);
                }
            }
        }
    } else {
        remainder =
            safe_c2rust_match_prefix((*descendant_dummy).text_uri, (*parent_dummy).text_uri);
        if !remainder.is_null() && *remainder as ::core::ffi::c_int == '/' as i32 {
            while *remainder as ::core::ffi::c_int == '/' as i32 {
                remainder = remainder.offset(1);
            }
            if *remainder as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                return safe_c2rust_unescape_string(
                    remainder as *const gchar,
                    ::core::ptr::null::<gchar>(),
                    b"/\0" as *const u8 as *const gchar,
                );
            }
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_resolve_relative_path(
    mut file: *mut GFile,
    mut relative_path: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut dummy: *mut GDummyFile = file as *mut ::core::ffi::c_void as *mut GDummyFile;
    let mut child: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut uri: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut new_decoded_uri: GDecodedUri = GDecodedUri {
        scheme: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        userinfo: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        host: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        port: 0,
        path: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        query: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        fragment: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut str: *mut GString = ::core::ptr::null_mut::<GString>();
    if (*dummy).decoded_uri.is_null() {
        str = g_string_new((*dummy).text_uri);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"/\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_11
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
                b"/\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        safe_c2rust_g_string_append_encoded(
            str,
            relative_path,
            b"!$&'()*+,;=:@/\0" as *const u8 as *const ::core::ffi::c_char,
        );
        child = safe_c2rust__g_dummy_file_new((*str).str_0);
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(str, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(str);
            };
        } else {
            g_string_free(str, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
    } else {
        new_decoded_uri = *(*dummy).decoded_uri;
        if g_path_is_absolute(relative_path as *const gchar) != 0 {
            new_decoded_uri.path = safe_c2rust_g_strdup_inline(relative_path);
        } else {
            new_decoded_uri.path = g_build_filename(new_decoded_uri.path, relative_path, NULL_1)
                as *mut ::core::ffi::c_char;
        }
        uri = safe_c2rust__g_encode_uri(&raw mut new_decoded_uri);
        g_free(new_decoded_uri.path as gpointer);
        child = safe_c2rust__g_dummy_file_new(uri);
        g_free(uri as gpointer);
    }
    return child;
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_get_child_for_display_name(
    mut file: *mut GFile,
    mut display_name: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GFile {
    return g_file_get_child(file, display_name);
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_has_uri_scheme(
    mut file: *mut GFile,
    mut uri_scheme: *const ::core::ffi::c_char,
) -> gboolean {
    let mut dummy: *mut GDummyFile = file as *mut ::core::ffi::c_void as *mut GDummyFile;
    if !(*dummy).decoded_uri.is_null() {
        return (g_ascii_strcasecmp(uri_scheme as *const gchar, (*(*dummy).decoded_uri).scheme)
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_get_uri_scheme(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut dummy: *mut GDummyFile = file as *mut ::core::ffi::c_void as *mut GDummyFile;
    if !(*dummy).decoded_uri.is_null() {
        return safe_c2rust_g_strdup_inline((*(*dummy).decoded_uri).scheme);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_g_dummy_file_file_iface_init(mut iface: *mut GFileIface) {
    (*iface).dup =
        Some(safe_c2rust_g_dummy_file_dup as unsafe extern "C" fn(*mut GFile) -> *mut GFile)
            as Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>;
    (*iface).hash = Some(safe_c2rust_g_dummy_file_hash as unsafe extern "C" fn(*mut GFile) -> guint)
        as Option<unsafe extern "C" fn(*mut GFile) -> guint>;
    (*iface).equal = Some(
        safe_c2rust_g_dummy_file_equal as unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>;
    (*iface).is_native =
        Some(safe_c2rust_g_dummy_file_is_native as unsafe extern "C" fn(*mut GFile) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GFile) -> gboolean>;
    (*iface).has_uri_scheme = Some(
        safe_c2rust_g_dummy_file_has_uri_scheme
            as unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> gboolean>;
    (*iface).get_uri_scheme = Some(
        safe_c2rust_g_dummy_file_get_uri_scheme
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_basename = Some(
        safe_c2rust_g_dummy_file_get_basename
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_path = Some(
        safe_c2rust_g_dummy_file_get_path
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_uri = Some(
        safe_c2rust_g_dummy_file_get_uri
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_parse_name = Some(
        safe_c2rust_g_dummy_file_get_parse_name
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_parent =
        Some(safe_c2rust_g_dummy_file_get_parent as unsafe extern "C" fn(*mut GFile) -> *mut GFile)
            as Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>;
    (*iface).prefix_matches = Some(
        safe_c2rust_g_dummy_file_prefix_matches
            as unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>;
    (*iface).get_relative_path = Some(
        safe_c2rust_g_dummy_file_get_relative_path
            as unsafe extern "C" fn(*mut GFile, *mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).resolve_relative_path = Some(
        safe_c2rust_g_dummy_file_resolve_relative_path
            as unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> *mut GFile,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> *mut GFile>;
    (*iface).get_child_for_display_name = Some(
        safe_c2rust_g_dummy_file_get_child_for_display_name
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> *mut GFile,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> *mut GFile,
        >;
    (*iface).supports_thread_contexts = TRUE as gboolean;
}
unsafe extern "C" fn safe_c2rust_unescape_character(
    mut scanner: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut first_digit: ::core::ffi::c_int = 0;
    let mut second_digit: ::core::ffi::c_int = 0;
    let fresh7 = scanner;
    scanner = scanner.offset(1);
    first_digit = g_ascii_xdigit_value(*fresh7) as ::core::ffi::c_int;
    if first_digit < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    let fresh8 = scanner;
    scanner = scanner.offset(1);
    second_digit = g_ascii_xdigit_value(*fresh8) as ::core::ffi::c_int;
    if second_digit < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return first_digit << 4 as ::core::ffi::c_int | second_digit;
}
unsafe extern "C" fn safe_c2rust_unescape_string(
    mut escaped_string: *const gchar,
    mut escaped_string_end: *const gchar,
    mut illegal_characters: *const gchar,
) -> *mut ::core::ffi::c_char {
    let mut in_0: *const gchar = ::core::ptr::null::<gchar>();
    let mut out: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut character: gint = 0;
    if escaped_string.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if escaped_string_end.is_null() {
        escaped_string_end =
            escaped_string.offset(strlen(escaped_string as *const ::core::ffi::c_char) as isize);
    }
    result = g_malloc(
        (escaped_string_end.offset_from(escaped_string) as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as gsize,
    ) as *mut gchar;
    out = result;
    in_0 = escaped_string;
    while in_0 < escaped_string_end {
        character = *in_0 as gint;
        if *in_0 as ::core::ffi::c_int == '%' as i32 {
            in_0 = in_0.offset(1);
            if (escaped_string_end.offset_from(in_0) as ::core::ffi::c_long)
                < 2 as ::core::ffi::c_long
            {
                g_free(result as gpointer);
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            character = safe_c2rust_unescape_character(in_0 as *const ::core::ffi::c_char) as gint;
            if character <= 0 as ::core::ffi::c_int
                || !illegal_characters.is_null()
                    && !strchr(
                        illegal_characters as *const ::core::ffi::c_char,
                        character as ::core::ffi::c_char as ::core::ffi::c_int,
                    )
                    .is_null()
            {
                g_free(result as gpointer);
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            in_0 = in_0.offset(1);
        }
        let fresh6 = out;
        out = out.offset(1);
        *fresh6 = character as ::core::ffi::c_char as gchar;
        in_0 = in_0.offset(1);
    }
    *out = '\0' as i32 as gchar;
    if !(({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if out.offset_from(result) as ::core::ffi::c_long as size_t
            <= strlen(escaped_string as *const ::core::ffi::c_char)
        {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdummyfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            487 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(gsize) (out - result) <= strlen (escaped_string)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return result as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust__g_decoded_uri_free(mut decoded: *mut GDecodedUri) {
    if decoded.is_null() {
        return;
    }
    g_free((*decoded).scheme as gpointer);
    g_free((*decoded).query as gpointer);
    g_free((*decoded).fragment as gpointer);
    g_free((*decoded).userinfo as gpointer);
    g_free((*decoded).host as gpointer);
    g_free((*decoded).path as gpointer);
    g_free(decoded as gpointer);
}
unsafe extern "C" fn safe_c2rust__g_decoded_uri_new() -> *mut GDecodedUri {
    let mut uri: *mut GDecodedUri = ::core::ptr::null_mut::<GDecodedUri>();
    uri = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GDecodedUri>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GDecodedUri;
    (*uri).port = -(1 as ::core::ffi::c_int);
    return uri;
}
unsafe extern "C" fn safe_c2rust__g_decode_uri(
    mut uri: *const ::core::ffi::c_char,
) -> *mut GDecodedUri {
    let mut decoded: *mut GDecodedUri = ::core::ptr::null_mut::<GDecodedUri>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut in_0: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut hier_part_start: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut hier_part_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut query_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut fragment_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut out: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_char = 0;
    p = uri;
    if !(*safe_c2rust_g_ascii_table.offset(*p as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_ALPHA as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int)
    {
        return ::core::ptr::null_mut::<GDecodedUri>();
    }
    loop {
        let fresh1 = p;
        p = p.offset(1);
        c = *fresh1;
        if c as ::core::ffi::c_int == ':' as i32 {
            break;
        }
        if !(*safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_ALNUM as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '+' as i32
            || c as ::core::ffi::c_int == '-' as i32
            || c as ::core::ffi::c_int == '.' as i32)
        {
            return ::core::ptr::null_mut::<GDecodedUri>();
        }
    }
    decoded = safe_c2rust__g_decoded_uri_new();
    (*decoded).scheme =
        g_malloc(p.offset_from(uri) as ::core::ffi::c_long as gsize) as *mut ::core::ffi::c_char;
    out = (*decoded).scheme;
    in_0 = uri;
    while in_0 < p.offset(-(1 as ::core::ffi::c_int as isize)) {
        let fresh2 = out;
        out = out.offset(1);
        *fresh2 = g_ascii_tolower(*in_0) as ::core::ffi::c_char;
        in_0 = in_0.offset(1);
    }
    *out = 0 as ::core::ffi::c_char;
    hier_part_start = p;
    query_start = strchr(p, '?' as i32);
    if !query_start.is_null() {
        let fresh3 = query_start;
        query_start = query_start.offset(1);
        hier_part_end = fresh3;
        fragment_start = strchr(query_start, '#' as i32);
        if !fragment_start.is_null() {
            (*decoded).query = g_strndup(
                query_start as *const gchar,
                fragment_start.offset_from(query_start) as ::core::ffi::c_long as gsize,
            ) as *mut ::core::ffi::c_char;
            (*decoded).fragment = safe_c2rust_g_strdup_inline(
                fragment_start.offset(1 as ::core::ffi::c_int as isize),
            );
        } else {
            (*decoded).query = safe_c2rust_g_strdup_inline(query_start);
            (*decoded).fragment = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    } else {
        (*decoded).query = ::core::ptr::null_mut::<::core::ffi::c_char>();
        fragment_start = strchr(p, '#' as i32);
        if !fragment_start.is_null() {
            let fresh4 = fragment_start;
            fragment_start = fragment_start.offset(1);
            hier_part_end = fresh4;
            (*decoded).fragment = safe_c2rust_g_strdup_inline(fragment_start);
        } else {
            hier_part_end = p.offset(strlen(p) as isize);
            (*decoded).fragment = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    if *hier_part_start.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32
        && *hier_part_start.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        let mut authority_start: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        let mut authority_end: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        let mut userinfo_start: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        let mut userinfo_end: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        let mut host_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut host_end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut port_start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        authority_start = hier_part_start.offset(2 as ::core::ffi::c_int as isize);
        authority_end = memchr(
            authority_start as *const ::core::ffi::c_void,
            '/' as i32,
            hier_part_end.offset_from(authority_start) as ::core::ffi::c_long as size_t,
        ) as *const ::core::ffi::c_char;
        if authority_end.is_null() {
            authority_end = hier_part_end;
        }
        userinfo_end = memchr(
            authority_start as *const ::core::ffi::c_void,
            '@' as i32,
            authority_end.offset_from(authority_start) as ::core::ffi::c_long as size_t,
        ) as *const ::core::ffi::c_char;
        if !userinfo_end.is_null() {
            userinfo_start = authority_start;
            (*decoded).userinfo = safe_c2rust_unescape_string(
                userinfo_start as *const gchar,
                userinfo_end as *const gchar,
                ::core::ptr::null::<gchar>(),
            );
            if (*decoded).userinfo.is_null() {
                safe_c2rust__g_decoded_uri_free(decoded);
                return ::core::ptr::null_mut::<GDecodedUri>();
            }
            host_start = userinfo_end.offset(1 as ::core::ffi::c_int as isize);
        } else {
            host_start = authority_start;
        }
        port_start = memchr(
            host_start as *const ::core::ffi::c_void,
            ':' as i32,
            authority_end.offset_from(host_start) as ::core::ffi::c_long as size_t,
        ) as *const ::core::ffi::c_char;
        if !port_start.is_null() {
            let fresh5 = port_start;
            port_start = port_start.offset(1);
            host_end = fresh5;
            (*decoded).port = safe_c2rust_atoi(port_start);
        } else {
            host_end = authority_end;
            (*decoded).port = -(1 as ::core::ffi::c_int);
        }
        (*decoded).host = g_strndup(
            host_start as *const gchar,
            host_end.offset_from(host_start) as ::core::ffi::c_long as gsize,
        ) as *mut ::core::ffi::c_char;
        hier_part_start = authority_end;
    }
    (*decoded).path = safe_c2rust_unescape_string(
        hier_part_start as *const gchar,
        hier_part_end as *const gchar,
        b"/\0" as *const u8 as *const gchar,
    );
    if (*decoded).path.is_null() {
        safe_c2rust__g_decoded_uri_free(decoded);
        return ::core::ptr::null_mut::<GDecodedUri>();
    }
    return decoded;
}
unsafe extern "C" fn safe_c2rust_is_valid(
    mut c: ::core::ffi::c_char,
    mut reserved_chars_allowed: *const ::core::ffi::c_char,
) -> gboolean {
    if *safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_ALNUM as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        || c as ::core::ffi::c_int == '-' as i32
        || c as ::core::ffi::c_int == '.' as i32
        || c as ::core::ffi::c_int == '_' as i32
        || c as ::core::ffi::c_int == '~' as i32
    {
        return TRUE;
    }
    if !reserved_chars_allowed.is_null()
        && !strchr(reserved_chars_allowed, c as ::core::ffi::c_int).is_null()
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_string_append_encoded(
    mut string: *mut GString,
    mut encoded: *const ::core::ffi::c_char,
    mut reserved_chars_allowed: *const ::core::ffi::c_char,
) {
    let mut c: ::core::ffi::c_uchar = 0;
    static mut safe_c2rust_hex: [gchar; 17] =
        unsafe { ::core::mem::transmute::<[u8; 17], [gchar; 17]>(*b"0123456789ABCDEF\0") };
    loop {
        c = *encoded as ::core::ffi::c_uchar;
        if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
        if safe_c2rust_is_valid(c as ::core::ffi::c_char, reserved_chars_allowed) != 0 {
            safe_c2rust_g_string_append_c_inline(string, c as gchar);
            encoded = encoded.offset(1);
        } else {
            safe_c2rust_g_string_append_c_inline(string, '%' as i32 as gchar);
            safe_c2rust_g_string_append_c_inline(
                string,
                safe_c2rust_hex[(c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as usize],
            );
            safe_c2rust_g_string_append_c_inline(
                string,
                safe_c2rust_hex[(c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize],
            );
            encoded = encoded.offset(1);
        }
    }
}
unsafe extern "C" fn safe_c2rust__g_encode_uri(
    mut decoded: *mut GDecodedUri,
) -> *mut ::core::ffi::c_char {
    let mut uri: *mut GString = ::core::ptr::null_mut::<GString>();
    uri = g_string_new(::core::ptr::null::<gchar>());
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = (*decoded).scheme;
            safe_c2rust_g_string_append_len_inline(
                uri,
                __val,
                if ({
                    let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_13
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
            uri,
            (*decoded).scheme,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"://\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                uri,
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
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            uri,
            b"://\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if !(*decoded).host.is_null() {
        if !(*decoded).userinfo.is_null() {
            safe_c2rust_g_string_append_encoded(
                uri,
                (*decoded).userinfo,
                b"!$&'()*+,;=:\0" as *const u8 as *const ::core::ffi::c_char,
            );
            safe_c2rust_g_string_append_c_inline(uri, '@' as i32 as gchar);
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = (*decoded).host;
                safe_c2rust_g_string_append_len_inline(
                    uri,
                    __val,
                    if ({
                        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_15
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
                uri,
                (*decoded).host,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        if (*decoded).port != -(1 as ::core::ffi::c_int) {
            safe_c2rust_g_string_append_c_inline(uri, ':' as i32 as gchar);
            g_string_append_printf(uri, b"%d\0" as *const u8 as *const gchar, (*decoded).port);
        }
    }
    safe_c2rust_g_string_append_encoded(
        uri,
        (*decoded).path,
        b"!$&'()*+,;=:@/\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !(*decoded).query.is_null() {
        safe_c2rust_g_string_append_c_inline(uri, '?' as i32 as gchar);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = (*decoded).query;
                safe_c2rust_g_string_append_len_inline(
                    uri,
                    __val,
                    if ({
                        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_16
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
                uri,
                (*decoded).query,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if !(*decoded).fragment.is_null() {
        safe_c2rust_g_string_append_c_inline(uri, '#' as i32 as gchar);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = (*decoded).fragment;
                safe_c2rust_g_string_append_len_inline(
                    uri,
                    __val,
                    if ({
                        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_17
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
                uri,
                (*decoded).fragment,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(uri, 0 as gboolean) as *mut ::core::ffi::c_char
        } else {
            g_string_free_and_steal(uri) as *mut ::core::ffi::c_char
        }
    } else {
        g_string_free(uri, 0 as gboolean) as *mut ::core::ffi::c_char
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
