extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GFileEnumeratorPrivate;
    pub type _GFileMonitorPrivate;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GFileAttributeMatcher;
    pub type _GFileInputStreamPrivate;
    pub type _GInputStreamPrivate;
    pub type _GFileOutputStream;
    pub type _GFileIOStream;
    pub type _GMount;
    pub type _GMountOperation;
    pub type _GSeekable;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_build_path(separator: *const gchar, first_element: *const gchar, ...) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_ascii_strcasecmp(s1: *const gchar, s2: *const gchar) -> gint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_uri_unescape_string(
        escaped_string: *const ::core::ffi::c_char,
        illegal_characters: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn g_uri_escape_string(
        unescaped: *const ::core::ffi::c_char,
        reserved_chars_allowed: *const ::core::ffi::c_char,
        allow_utf8: gboolean,
    ) -> *mut ::core::ffi::c_char;
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
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_resource_error_quark() -> GQuark;
    fn g_resources_open_stream(
        path: *const ::core::ffi::c_char,
        lookup_flags: GResourceLookupFlags,
        error: *mut *mut GError,
    ) -> *mut GInputStream;
    fn g_resources_lookup_data(
        path: *const ::core::ffi::c_char,
        lookup_flags: GResourceLookupFlags,
        error: *mut *mut GError,
    ) -> *mut GBytes;
    fn g_resources_enumerate_children(
        path: *const ::core::ffi::c_char,
        lookup_flags: GResourceLookupFlags,
        error: *mut *mut GError,
    ) -> *mut *mut ::core::ffi::c_char;
    fn g_resources_get_info(
        path: *const ::core::ffi::c_char,
        lookup_flags: GResourceLookupFlags,
        size: *mut gsize,
        flags: *mut guint32,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_attribute_info_list_new() -> *mut GFileAttributeInfoList;
    fn g_file_attribute_info_list_ref(
        list: *mut GFileAttributeInfoList,
    ) -> *mut GFileAttributeInfoList;
    fn g_file_info_new() -> *mut GFileInfo;
    fn g_file_info_set_attribute_string(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
        attr_value: *const ::core::ffi::c_char,
    );
    fn g_file_info_set_attribute_boolean(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
        attr_value: gboolean,
    );
    fn g_file_info_set_file_type(info: *mut GFileInfo, type_0: GFileType);
    fn g_file_info_set_name(info: *mut GFileInfo, name: *const ::core::ffi::c_char);
    fn g_file_info_set_display_name(info: *mut GFileInfo, display_name: *const ::core::ffi::c_char);
    fn g_file_info_set_size(info: *mut GFileInfo, size: goffset);
    fn g_file_attribute_matcher_new(
        attributes: *const ::core::ffi::c_char,
    ) -> *mut GFileAttributeMatcher;
    fn g_file_attribute_matcher_unref(matcher: *mut GFileAttributeMatcher);
    fn g_file_attribute_matcher_matches(
        matcher: *mut GFileAttributeMatcher,
        attribute: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn _g_file_attribute_matcher_matches_id(
        matcher: *mut GFileAttributeMatcher,
        id: guint32,
    ) -> gboolean;
    fn _g_file_info_set_attribute_string_by_id(
        info: *mut GFileInfo,
        attribute: guint32,
        attr_value: *const ::core::ffi::c_char,
    );
    fn _g_file_info_set_attribute_boolean_by_id(
        info: *mut GFileInfo,
        attribute: guint32,
        attr_value: gboolean,
    );
    fn g_file_get_type() -> GType;
    fn g_file_get_child(file: *mut GFile, name: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_query_info(
        file: *mut GFile,
        attributes: *const ::core::ffi::c_char,
        flags: GFileQueryInfoFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_file_monitor_get_type() -> GType;
    fn g_seekable_get_type() -> GType;
    fn g_seekable_tell(seekable: *mut GSeekable) -> goffset;
    fn g_seekable_can_seek(seekable: *mut GSeekable) -> gboolean;
    fn g_seekable_seek(
        seekable: *mut GSeekable,
        offset: goffset,
        type_0: GSeekType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_input_stream_read(
        stream: *mut GInputStream,
        buffer: *mut ::core::ffi::c_void,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_input_stream_skip(
        stream: *mut GInputStream,
        count: gsize,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gssize;
    fn g_input_stream_close(
        stream: *mut GInputStream,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_input_stream_get_type() -> GType;
    fn g_file_enumerator_get_type() -> GType;
    fn g_content_type_guess(
        filename: *const gchar,
        data: *const guchar,
        data_size: gsize,
        result_uncertain: *mut gboolean,
    ) -> *mut gchar;
    fn g_io_error_quark() -> GQuark;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
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
pub type GFileType = ::core::ffi::c_uint;
pub const G_FILE_TYPE_MOUNTABLE: GFileType = 6;
pub const G_FILE_TYPE_SHORTCUT: GFileType = 5;
pub const G_FILE_TYPE_SPECIAL: GFileType = 4;
pub const G_FILE_TYPE_SYMBOLIC_LINK: GFileType = 3;
pub const G_FILE_TYPE_DIRECTORY: GFileType = 2;
pub const G_FILE_TYPE_REGULAR: GFileType = 1;
pub const G_FILE_TYPE_UNKNOWN: GFileType = 0;
pub type GFileMonitorEvent = ::core::ffi::c_uint;
pub const G_FILE_MONITOR_EVENT_MOVED_OUT: GFileMonitorEvent = 10;
pub const G_FILE_MONITOR_EVENT_MOVED_IN: GFileMonitorEvent = 9;
pub const G_FILE_MONITOR_EVENT_RENAMED: GFileMonitorEvent = 8;
pub const G_FILE_MONITOR_EVENT_MOVED: GFileMonitorEvent = 7;
pub const G_FILE_MONITOR_EVENT_UNMOUNTED: GFileMonitorEvent = 6;
pub const G_FILE_MONITOR_EVENT_PRE_UNMOUNT: GFileMonitorEvent = 5;
pub const G_FILE_MONITOR_EVENT_ATTRIBUTE_CHANGED: GFileMonitorEvent = 4;
pub const G_FILE_MONITOR_EVENT_CREATED: GFileMonitorEvent = 3;
pub const G_FILE_MONITOR_EVENT_DELETED: GFileMonitorEvent = 2;
pub const G_FILE_MONITOR_EVENT_CHANGES_DONE_HINT: GFileMonitorEvent = 1;
pub const G_FILE_MONITOR_EVENT_CHANGED: GFileMonitorEvent = 0;
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
pub type GAsyncResult = _GAsyncResult;
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
pub type GFileAttributeMatcher = _GFileAttributeMatcher;
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
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GFileInputStream = _GFileInputStream;
pub type GFileOutputStream = _GFileOutputStream;
pub type GFileIOStream = _GFileIOStream;
pub type GMount = _GMount;
pub type GMountOperation = _GMountOperation;
pub type GSeekable = _GSeekable;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GFileProgressCallback = Option<unsafe extern "C" fn(goffset, goffset, gpointer) -> ()>;
pub type GFileMeasureProgressCallback =
    Option<unsafe extern "C" fn(gboolean, guint64, guint64, guint64, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResourceFile {
    pub parent_instance: GObject,
    pub path: *mut ::core::ffi::c_char,
}
pub type GResourceFile = _GResourceFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResourceFileClass {
    pub parent_class: GObjectClass,
}
pub type GResourceFileClass = _GResourceFileClass;
pub type GFileIface = _GFileIface;
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
pub type GResourceFileMonitor = GFileMonitor;
pub type GResourceFileMonitorClass = GFileMonitorClass;
pub type GFileMonitorClass = _GFileMonitorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileMonitorClass {
    pub parent_class: GObjectClass,
    pub changed: Option<
        unsafe extern "C" fn(*mut GFileMonitor, *mut GFile, *mut GFile, GFileMonitorEvent) -> (),
    >,
    pub cancel: Option<unsafe extern "C" fn(*mut GFileMonitor) -> gboolean>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GResourceFileInputStream = _GResourceFileInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResourceFileInputStream {
    pub parent_instance: GFileInputStream,
    pub stream: *mut GInputStream,
    pub file: *mut GFile,
}
pub type GResourceFileInputStreamClass = _GResourceFileInputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResourceFileInputStreamClass {
    pub parent_class: GFileInputStreamClass,
}
pub type GFileInputStreamClass = _GFileInputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileInputStreamClass {
    pub parent_class: GInputStreamClass,
    pub tell: Option<unsafe extern "C" fn(*mut GFileInputStream) -> goffset>,
    pub can_seek: Option<unsafe extern "C" fn(*mut GFileInputStream) -> gboolean>,
    pub seek: Option<
        unsafe extern "C" fn(
            *mut GFileInputStream,
            goffset,
            GSeekType,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub query_info: Option<
        unsafe extern "C" fn(
            *mut GFileInputStream,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_info_async: Option<
        unsafe extern "C" fn(
            *mut GFileInputStream,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_info_finish: Option<
        unsafe extern "C" fn(
            *mut GFileInputStream,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GInputStreamClass = _GInputStreamClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStreamClass {
    pub parent_class: GObjectClass,
    pub read_fn: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            *mut ::core::ffi::c_void,
            gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub skip: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            gsize,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gssize,
    >,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            *mut ::core::ffi::c_void,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub read_finish: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub skip_async: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            gsize,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub skip_finish: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gssize,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GInputStream,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GInputStream, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GResourceFileEnumerator = _GResourceFileEnumerator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResourceFileEnumerator {
    pub parent: GFileEnumerator,
    pub matcher: *mut GFileAttributeMatcher,
    pub path: *mut ::core::ffi::c_char,
    pub attributes: *mut ::core::ffi::c_char,
    pub flags: GFileQueryInfoFlags,
    pub index: ::core::ffi::c_int,
    pub children: *mut *mut ::core::ffi::c_char,
}
pub type GResourceFileEnumeratorClass = _GResourceFileEnumeratorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResourceFileEnumeratorClass {
    pub parent_class: GFileEnumeratorClass,
}
pub type GFileEnumeratorClass = _GFileEnumeratorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileEnumeratorClass {
    pub parent_class: GObjectClass,
    pub next_file: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GFileEnumerator, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub next_files_async: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub next_files_finish: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GList,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GFileEnumerator, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved7: Option<unsafe extern "C" fn() -> ()>,
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
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
pub const G_FILE_ATTRIBUTE_FILESYSTEM_TYPE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"filesystem::type\0")
};
pub const G_FILE_ATTRIBUTE_FILESYSTEM_READONLY: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"filesystem::readonly\0")
};
pub const G_FILE_ATTRIBUTE_ID_STANDARD_CONTENT_TYPE: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_FAST_CONTENT_TYPE: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 13 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_READ: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_WRITE: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_EXECUTE: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_DELETE: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 4 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_TRASH: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 5 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_RENAME: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 6 as ::core::ffi::c_int;
static mut safe_c2rust_resource_writable_attributes: *mut GFileAttributeInfoList =
    ::core::ptr::null::<GFileAttributeInfoList>() as *mut GFileAttributeInfoList;
static mut safe_c2rust_resource_writable_namespaces: *mut GFileAttributeInfoList =
    ::core::ptr::null::<GFileAttributeInfoList>() as *mut GFileAttributeInfoList;
static mut safe_c2rust_g_resource_file_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GResourceFile_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_resource_file_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_resource_file_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_resource_file_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GResourceFile\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GResourceFileClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_resource_file_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GResourceFile>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GResourceFile) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_resource_file_init as unsafe extern "C" fn(*mut GResourceFile) -> (),
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
            safe_c2rust_g_resource_file_file_iface_init
                as unsafe extern "C" fn(*mut GFileIface) -> (),
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
unsafe extern "C" fn safe_c2rust_g_resource_file_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_resource_file_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GResourceFile_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GResourceFile_private_offset,
        );
    }
    safe_c2rust_g_resource_file_class_init(klass as *mut GResourceFileClass);
}
static mut safe_c2rust_GResourceFileEnumerator_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_resource_file_enumerator_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_resource_file_enumerator_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GResourceFileEnumerator_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GResourceFileEnumerator_private_offset,
        );
    }
    safe_c2rust_g_resource_file_enumerator_class_init(klass as *mut GResourceFileEnumeratorClass);
}
static mut safe_c2rust_g_resource_file_enumerator_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust__g_resource_file_enumerator_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_resource_file_enumerator_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_resource_file_enumerator_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_file_enumerator_get_type(),
        g_intern_static_string(b"GResourceFileEnumerator\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GResourceFileEnumeratorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_resource_file_enumerator_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GResourceFileEnumerator>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GResourceFileEnumerator) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_resource_file_enumerator_init
                    as unsafe extern "C" fn(*mut GResourceFileEnumerator) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_finalize(mut object: *mut GObject) {
    let mut resource: *mut GResourceFile = ::core::ptr::null_mut::<GResourceFile>();
    resource = object as *mut ::core::ffi::c_void as *mut GResourceFile;
    g_free((*resource).path as gpointer);
    (*(safe_c2rust_g_resource_file_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_class_init(mut klass: *mut GResourceFileClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_resource_file_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    safe_c2rust_resource_writable_attributes = g_file_attribute_info_list_new();
    safe_c2rust_resource_writable_namespaces = g_file_attribute_info_list_new();
}
unsafe extern "C" fn safe_c2rust_g_resource_file_init(mut resource: *mut GResourceFile) {}
#[inline]
unsafe extern "C" fn safe_c2rust_scan_backwards(
    mut begin: *const gchar,
    mut end: *const gchar,
    mut c: gchar,
) -> *mut gchar {
    while end >= begin {
        if *end as ::core::ffi::c_int == c as ::core::ffi::c_int {
            return end as *mut gchar;
        }
        end = end.offset(-1);
    }
    return ::core::ptr::null_mut::<gchar>();
}
#[inline]
unsafe extern "C" fn safe_c2rust_pop_to_previous_part(
    mut begin: *const gchar,
    mut out: *mut *mut gchar,
) {
    if *out > begin as *mut gchar {
        *out = safe_c2rust_scan_backwards(
            begin,
            (*out).offset(-(1 as ::core::ffi::c_int as isize)),
            '/' as i32 as gchar,
        );
    }
}
unsafe extern "C" fn safe_c2rust_canonicalize_filename(
    mut in_0: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut bptr: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut out: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    out = g_malloc((strlen(in_0) as gsize).wrapping_add(2 as gsize)) as *mut ::core::ffi::c_char;
    bptr = out as *mut gchar;
    *out = '/' as i32 as ::core::ffi::c_char;
    while *in_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if *out as ::core::ffi::c_int == '/' as i32 {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gresourcefile.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_int,
                G_STRFUNC,
                b"*out == '/'\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        while *in_0 as ::core::ffi::c_int == '/' as i32 {
            in_0 = in_0.offset(1);
        }
        if *in_0 as ::core::ffi::c_int == '.' as i32 {
            if *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
                && (*in_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '/' as i32
                    || *in_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int)
            {
                safe_c2rust_pop_to_previous_part(bptr, &raw mut out);
                in_0 = in_0.offset(2 as ::core::ffi::c_int as isize);
                continue;
            } else if *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32
                || *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                in_0 = in_0.offset(1 as ::core::ffi::c_int as isize);
                continue;
            }
        }
        while *in_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            && *in_0 as ::core::ffi::c_int != '/' as i32
        {
            let fresh1 = in_0;
            in_0 = in_0.offset(1);
            out = out.offset(1);
            *out = *fresh1;
        }
        if *in_0 as ::core::ffi::c_int == '/' as i32 {
            let fresh2 = in_0;
            in_0 = in_0.offset(1);
            out = out.offset(1);
            *out = *fresh2;
        }
    }
    if out > bptr && *out as ::core::ffi::c_int == '/' as i32 {
        *out = 0 as ::core::ffi::c_char;
    } else {
        out = out.offset(1);
        *out = 0 as ::core::ffi::c_char;
    }
    return bptr as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_new_for_path(
    mut path: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut resource: *mut GResourceFile = g_object_new(
        safe_c2rust__g_resource_file_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GResourceFile;
    (*resource).path = safe_c2rust_canonicalize_filename(path);
    return resource as *mut ::core::ffi::c_void as *mut GFile;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_resource_file_new(
    mut uri: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut resource: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    path = g_uri_unescape_string(
        uri.offset(strlen(b"resource:\0" as *const u8 as *const ::core::ffi::c_char) as isize),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if path.is_null() {
        return ::core::ptr::null_mut::<GFile>();
    }
    resource = safe_c2rust_g_resource_file_new_for_path(path);
    g_free(path as gpointer);
    return resource as *mut ::core::ffi::c_void as *mut GFile;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_is_native(mut file: *mut GFile) -> gboolean {
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_has_uri_scheme(
    mut file: *mut GFile,
    mut uri_scheme: *const ::core::ffi::c_char,
) -> gboolean {
    return (g_ascii_strcasecmp(
        uri_scheme as *const gchar,
        b"resource\0" as *const u8 as *const gchar,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_get_uri_scheme(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    return safe_c2rust_g_strdup_inline(b"resource\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_get_basename(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut base: *mut gchar = ::core::ptr::null_mut::<gchar>();
    base = strrchr(
        (*(file as *mut ::core::ffi::c_void as *mut GResourceFile)).path,
        '/' as i32,
    ) as *mut gchar;
    return safe_c2rust_g_strdup_inline(base.offset(1 as ::core::ffi::c_int as isize));
}
unsafe extern "C" fn safe_c2rust_g_resource_file_get_path(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_g_resource_file_get_uri(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut escaped: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    escaped = g_uri_escape_string(
        (*(file as *mut ::core::ffi::c_void as *mut GResourceFile)).path,
        b"!$&'()*+,;=:@/\0" as *const u8 as *const ::core::ffi::c_char,
        FALSE,
    );
    res = g_strconcat(
        b"resource://\0" as *const u8 as *const gchar,
        escaped,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    g_free(escaped as gpointer);
    return res;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_get_parse_name(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    return safe_c2rust_g_resource_file_get_uri(file);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_get_parent(mut file: *mut GFile) -> *mut GFile {
    let mut resource: *mut GResourceFile = file as *mut ::core::ffi::c_void as *mut GResourceFile;
    let mut parent: *mut GResourceFile = ::core::ptr::null_mut::<GResourceFile>();
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    end = strrchr((*resource).path, '/' as i32) as *mut gchar;
    if end == (*(file as *mut ::core::ffi::c_void as *mut GResourceFile)).path {
        return ::core::ptr::null_mut::<GFile>();
    }
    parent = g_object_new(
        safe_c2rust__g_resource_file_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GResourceFile;
    (*parent).path = g_strndup(
        (*resource).path,
        end.offset_from((*resource).path) as ::core::ffi::c_long as gsize,
    ) as *mut ::core::ffi::c_char;
    return parent as *mut ::core::ffi::c_void as *mut GFile;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_dup(mut file: *mut GFile) -> *mut GFile {
    let mut resource: *mut GResourceFile = file as *mut ::core::ffi::c_void as *mut GResourceFile;
    return safe_c2rust_g_resource_file_new_for_path((*resource).path);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_hash(mut file: *mut GFile) -> guint {
    let mut resource: *mut GResourceFile = file as *mut ::core::ffi::c_void as *mut GResourceFile;
    return g_str_hash((*resource).path as gconstpointer);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_equal(
    mut file1: *mut GFile,
    mut file2: *mut GFile,
) -> gboolean {
    let mut resource1: *mut GResourceFile = file1 as *mut ::core::ffi::c_void as *mut GResourceFile;
    let mut resource2: *mut GResourceFile = file2 as *mut ::core::ffi::c_void as *mut GResourceFile;
    return (strcmp(
        (*resource1).path as *const ::core::ffi::c_char,
        (*resource2).path as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
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
    if prefix_len > 0 as ::core::ffi::c_int
        && *prefix.offset((prefix_len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        prefix_len -= 1;
    }
    return path.offset(prefix_len as isize);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_prefix_matches(
    mut parent: *mut GFile,
    mut descendant: *mut GFile,
) -> gboolean {
    let mut parent_resource: *mut GResourceFile =
        parent as *mut ::core::ffi::c_void as *mut GResourceFile;
    let mut descendant_resource: *mut GResourceFile =
        descendant as *mut ::core::ffi::c_void as *mut GResourceFile;
    let mut remainder: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    remainder = safe_c2rust_match_prefix((*descendant_resource).path, (*parent_resource).path);
    if !remainder.is_null() && *remainder as ::core::ffi::c_int == '/' as i32 {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_get_relative_path(
    mut parent: *mut GFile,
    mut descendant: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut parent_resource: *mut GResourceFile =
        parent as *mut ::core::ffi::c_void as *mut GResourceFile;
    let mut descendant_resource: *mut GResourceFile =
        descendant as *mut ::core::ffi::c_void as *mut GResourceFile;
    let mut remainder: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    remainder = safe_c2rust_match_prefix((*descendant_resource).path, (*parent_resource).path);
    if !remainder.is_null() && *remainder as ::core::ffi::c_int == '/' as i32 {
        return safe_c2rust_g_strdup_inline(remainder.offset(1 as ::core::ffi::c_int as isize));
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_g_resource_file_resolve_relative_path(
    mut file: *mut GFile,
    mut relative_path: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut resource: *mut GResourceFile = file as *mut ::core::ffi::c_void as *mut GResourceFile;
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut child: *mut GFile = ::core::ptr::null_mut::<GFile>();
    if *relative_path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
        return safe_c2rust_g_resource_file_new_for_path(relative_path);
    }
    filename = g_build_path(
        b"/\0" as *const u8 as *const gchar,
        (*resource).path,
        relative_path,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    child = safe_c2rust_g_resource_file_new_for_path(filename);
    g_free(filename as gpointer);
    return child;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_enumerate_children(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileEnumerator {
    let mut resource: *mut GResourceFile = file as *mut ::core::ffi::c_void as *mut GResourceFile;
    return safe_c2rust__g_resource_file_enumerator_new(
        resource,
        attributes,
        flags,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_resource_file_get_child_for_display_name(
    mut file: *mut GFile,
    mut display_name: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GFile {
    let mut new_file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    new_file = g_file_get_child(file, display_name);
    return new_file;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_query_info(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut resource: *mut GResourceFile = file as *mut ::core::ffi::c_void as *mut GResourceFile;
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut matcher: *mut GFileAttributeMatcher = ::core::ptr::null_mut::<GFileAttributeMatcher>();
    let mut res: gboolean = 0;
    let mut size: gsize = 0 as gsize;
    let mut resource_flags: guint32 = 0 as guint32;
    let mut children: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut is_dir: gboolean = 0;
    let mut base: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    is_dir = FALSE as gboolean;
    children = g_resources_enumerate_children(
        (*resource).path,
        G_RESOURCE_LOOKUP_FLAGS_NONE,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !children.is_null() {
        g_strfreev(children as *mut *mut gchar);
        is_dir = TRUE as gboolean;
    }
    if strcmp(
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
        (*resource).path,
    ) == 0 as ::core::ffi::c_int
    {
        is_dir = TRUE as gboolean;
    }
    if is_dir == 0 {
        res = g_resources_get_info(
            (*resource).path,
            G_RESOURCE_LOOKUP_FLAGS_NONE,
            &raw mut size,
            &raw mut resource_flags,
            &raw mut my_error,
        );
        if res == 0 {
            if g_error_matches(
                my_error,
                g_resource_error_quark(),
                G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            ) != 0
            {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0" as *const u8
                            as *const gchar,
                    ),
                    (*resource).path,
                );
            } else {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                    (*my_error).message,
                );
            }
            g_clear_error(&raw mut my_error);
            return ::core::ptr::null_mut::<GFileInfo>();
        }
    }
    matcher = g_file_attribute_matcher_new(attributes);
    info = g_file_info_new();
    base = safe_c2rust_g_resource_file_get_basename(file);
    g_file_info_set_name(info, base);
    g_file_info_set_display_name(info, base);
    _g_file_info_set_attribute_boolean_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_ACCESS_CAN_READ as guint32,
        TRUE,
    );
    _g_file_info_set_attribute_boolean_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_ACCESS_CAN_WRITE as guint32,
        FALSE,
    );
    _g_file_info_set_attribute_boolean_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_ACCESS_CAN_EXECUTE as guint32,
        FALSE,
    );
    _g_file_info_set_attribute_boolean_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_ACCESS_CAN_RENAME as guint32,
        FALSE,
    );
    _g_file_info_set_attribute_boolean_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_ACCESS_CAN_DELETE as guint32,
        FALSE,
    );
    _g_file_info_set_attribute_boolean_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_ACCESS_CAN_TRASH as guint32,
        FALSE,
    );
    if is_dir != 0 {
        g_file_info_set_file_type(info, G_FILE_TYPE_DIRECTORY);
    } else {
        let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
        let mut content_type: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        g_file_info_set_file_type(info, G_FILE_TYPE_REGULAR);
        g_file_info_set_size(info, size as goffset);
        if (_g_file_attribute_matcher_matches_id(
            matcher,
            G_FILE_ATTRIBUTE_ID_STANDARD_CONTENT_TYPE as guint32,
        ) != 0
            || !resource_flags & G_RESOURCE_FLAGS_COMPRESSED as ::core::ffi::c_int as guint32 != 0
                && _g_file_attribute_matcher_matches_id(
                    matcher,
                    G_FILE_ATTRIBUTE_ID_STANDARD_FAST_CONTENT_TYPE as guint32,
                ) != 0)
            && {
                bytes = g_resources_lookup_data(
                    (*resource).path,
                    G_RESOURCE_LOOKUP_FLAGS_NONE,
                    ::core::ptr::null_mut::<*mut GError>(),
                );
                !bytes.is_null()
            }
        {
            let mut data: *const guchar = ::core::ptr::null::<guchar>();
            let mut data_size: gsize = 0;
            data = g_bytes_get_data(bytes, &raw mut data_size) as *const guchar;
            content_type =
                g_content_type_guess(base, data, data_size, ::core::ptr::null_mut::<gboolean>())
                    as *mut ::core::ffi::c_char;
            g_bytes_unref(bytes);
        } else {
            content_type = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !content_type.is_null() {
            _g_file_info_set_attribute_string_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_STANDARD_CONTENT_TYPE as guint32,
                content_type,
            );
            _g_file_info_set_attribute_string_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_STANDARD_FAST_CONTENT_TYPE as guint32,
                content_type,
            );
            g_free(content_type as gpointer);
        }
    }
    g_free(base as gpointer);
    g_file_attribute_matcher_unref(matcher);
    return info;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_query_filesystem_info(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut matcher: *mut GFileAttributeMatcher = ::core::ptr::null_mut::<GFileAttributeMatcher>();
    info = g_file_info_new();
    matcher = g_file_attribute_matcher_new(attributes);
    if g_file_attribute_matcher_matches(matcher, G_FILE_ATTRIBUTE_FILESYSTEM_TYPE.as_ptr()) != 0 {
        g_file_info_set_attribute_string(
            info,
            G_FILE_ATTRIBUTE_FILESYSTEM_TYPE.as_ptr(),
            b"resource\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if g_file_attribute_matcher_matches(matcher, G_FILE_ATTRIBUTE_FILESYSTEM_READONLY.as_ptr()) != 0
    {
        g_file_info_set_attribute_boolean(
            info,
            G_FILE_ATTRIBUTE_FILESYSTEM_READONLY.as_ptr(),
            TRUE,
        );
    }
    g_file_attribute_matcher_unref(matcher);
    return info;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_query_settable_attributes(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileAttributeInfoList {
    return g_file_attribute_info_list_ref(safe_c2rust_resource_writable_attributes);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_query_writable_namespaces(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileAttributeInfoList {
    return g_file_attribute_info_list_ref(safe_c2rust_resource_writable_namespaces);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_read(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInputStream {
    let mut resource: *mut GResourceFile = file as *mut ::core::ffi::c_void as *mut GResourceFile;
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut stream: *mut GInputStream = ::core::ptr::null_mut::<GInputStream>();
    let mut res: *mut GFileInputStream = ::core::ptr::null_mut::<GFileInputStream>();
    stream = g_resources_open_stream(
        (*resource).path,
        G_RESOURCE_LOOKUP_FLAGS_NONE,
        &raw mut my_error,
    );
    if stream.is_null() {
        if g_error_matches(
            my_error,
            g_resource_error_quark(),
            G_RESOURCE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
        ) != 0
        {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0" as *const u8
                        as *const gchar,
                ),
                (*resource).path,
            );
        } else {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                (*my_error).message,
            );
        }
        g_clear_error(&raw mut my_error);
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    res = safe_c2rust__g_resource_file_input_stream_new(stream, file);
    g_object_unref(stream as gpointer);
    return res;
}
static mut safe_c2rust_g_resource_file_monitor_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GResourceFileMonitor_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resource_file_monitor_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_resource_file_monitor_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_resource_file_monitor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_file_monitor_get_type(),
        g_intern_static_string(b"GResourceFileMonitor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GResourceFileMonitorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_resource_file_monitor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GResourceFileMonitor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GResourceFileMonitor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_resource_file_monitor_init
                    as unsafe extern "C" fn(*mut GResourceFileMonitor) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_monitor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_resource_file_monitor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GResourceFileMonitor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GResourceFileMonitor_private_offset,
        );
    }
    safe_c2rust_g_resource_file_monitor_class_init(klass as *mut GResourceFileMonitorClass);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_monitor_cancel(
    mut monitor: *mut GFileMonitor,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_monitor_init(
    mut monitor: *mut GResourceFileMonitor,
) {
}
unsafe extern "C" fn safe_c2rust_g_resource_file_monitor_class_init(
    mut class: *mut GResourceFileMonitorClass,
) {
    (*class).cancel = Some(
        safe_c2rust_g_resource_file_monitor_cancel
            as unsafe extern "C" fn(*mut GFileMonitor) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GFileMonitor) -> gboolean>;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_monitor_file(
    mut file: *mut GFile,
    mut flags: GFileMonitorFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileMonitor {
    return g_object_new(
        safe_c2rust_g_resource_file_monitor_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GFileMonitor;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_set_display_name(
    mut file: *mut GFile,
    mut display_name: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFile {
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        glib_gettext(b"Resource files cannot be renamed\0" as *const u8 as *const gchar),
    );
    return ::core::ptr::null_mut::<GFile>();
}
unsafe extern "C" fn safe_c2rust_g_resource_file_file_iface_init(mut iface: *mut GFileIface) {
    (*iface).dup =
        Some(safe_c2rust_g_resource_file_dup as unsafe extern "C" fn(*mut GFile) -> *mut GFile)
            as Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>;
    (*iface).hash =
        Some(safe_c2rust_g_resource_file_hash as unsafe extern "C" fn(*mut GFile) -> guint)
            as Option<unsafe extern "C" fn(*mut GFile) -> guint>;
    (*iface).equal = Some(
        safe_c2rust_g_resource_file_equal
            as unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>;
    (*iface).is_native =
        Some(safe_c2rust_g_resource_file_is_native as unsafe extern "C" fn(*mut GFile) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GFile) -> gboolean>;
    (*iface).has_uri_scheme = Some(
        safe_c2rust_g_resource_file_has_uri_scheme
            as unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> gboolean>;
    (*iface).get_uri_scheme = Some(
        safe_c2rust_g_resource_file_get_uri_scheme
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_basename = Some(
        safe_c2rust_g_resource_file_get_basename
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_path = Some(
        safe_c2rust_g_resource_file_get_path
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_uri = Some(
        safe_c2rust_g_resource_file_get_uri
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_parse_name = Some(
        safe_c2rust_g_resource_file_get_parse_name
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_parent = Some(
        safe_c2rust_g_resource_file_get_parent as unsafe extern "C" fn(*mut GFile) -> *mut GFile,
    ) as Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>;
    (*iface).prefix_matches = Some(
        safe_c2rust_g_resource_file_prefix_matches
            as unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>;
    (*iface).get_relative_path = Some(
        safe_c2rust_g_resource_file_get_relative_path
            as unsafe extern "C" fn(*mut GFile, *mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).resolve_relative_path = Some(
        safe_c2rust_g_resource_file_resolve_relative_path
            as unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> *mut GFile,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> *mut GFile>;
    (*iface).get_child_for_display_name = Some(
        safe_c2rust_g_resource_file_get_child_for_display_name
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
    (*iface).set_display_name = Some(
        safe_c2rust_g_resource_file_set_display_name
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFile,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFile,
        >;
    (*iface).enumerate_children = Some(
        safe_c2rust_g_resource_file_enumerate_children
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileEnumerator,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileEnumerator,
        >;
    (*iface).query_info = Some(
        safe_c2rust_g_resource_file_query_info
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
    (*iface).query_filesystem_info = Some(
        safe_c2rust_g_resource_file_query_filesystem_info
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
    (*iface).query_settable_attributes = Some(
        safe_c2rust_g_resource_file_query_settable_attributes
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileAttributeInfoList,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileAttributeInfoList,
        >;
    (*iface).query_writable_namespaces = Some(
        safe_c2rust_g_resource_file_query_writable_namespaces
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileAttributeInfoList,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileAttributeInfoList,
        >;
    (*iface).read_fn = Some(
        safe_c2rust_g_resource_file_read
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInputStream,
        >;
    (*iface).monitor_file = Some(
        safe_c2rust_g_resource_file_monitor_file
            as unsafe extern "C" fn(
                *mut GFile,
                GFileMonitorFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileMonitor,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileMonitorFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileMonitor,
        >;
    (*iface).supports_thread_contexts = TRUE as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_enumerator_finalize(mut object: *mut GObject) {
    let mut resource: *mut GResourceFileEnumerator =
        ::core::ptr::null_mut::<GResourceFileEnumerator>();
    resource = object as *mut ::core::ffi::c_void as *mut GResourceFileEnumerator;
    g_strfreev((*resource).children as *mut *mut gchar);
    g_free((*resource).path as gpointer);
    g_free((*resource).attributes as gpointer);
    (*(safe_c2rust_g_resource_file_enumerator_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_enumerator_class_init(
    mut klass: *mut GResourceFileEnumeratorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut enumerator_class: *mut GFileEnumeratorClass =
        klass as *mut ::core::ffi::c_void as *mut GFileEnumeratorClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_resource_file_enumerator_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*enumerator_class).next_file = Some(
        safe_c2rust_g_resource_file_enumerator_next_file
            as unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
    (*enumerator_class).close_fn = Some(
        safe_c2rust_g_resource_file_enumerator_close
            as unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_enumerator_init(
    mut resource: *mut GResourceFileEnumerator,
) {
}
unsafe extern "C" fn safe_c2rust__g_resource_file_enumerator_new(
    mut file: *mut GResourceFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileEnumerator {
    let mut resource: *mut GResourceFileEnumerator =
        ::core::ptr::null_mut::<GResourceFileEnumerator>();
    let mut children: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut res: gboolean = 0;
    children = g_resources_enumerate_children(
        (*file).path,
        G_RESOURCE_LOOKUP_FLAGS_NONE,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if children.is_null()
        && strcmp(
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
            (*file).path,
        ) != 0 as ::core::ffi::c_int
    {
        res = g_resources_get_info(
            (*file).path,
            G_RESOURCE_LOOKUP_FLAGS_NONE,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<guint32>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if res != 0 {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_DIRECTORY as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D is not a directory\0" as *const u8
                        as *const gchar,
                ),
                (*file).path,
            );
        } else {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"The resource at \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0" as *const u8
                        as *const gchar,
                ),
                (*file).path,
            );
        }
        return ::core::ptr::null_mut::<GFileEnumerator>();
    }
    resource = g_object_new(
        safe_c2rust__g_resource_file_enumerator_get_type(),
        b"container\0" as *const u8 as *const gchar,
        file,
        NULL_0,
    ) as *mut GResourceFileEnumerator;
    (*resource).children = children;
    (*resource).path = safe_c2rust_g_strdup_inline((*file).path);
    (*resource).attributes = safe_c2rust_g_strdup_inline(attributes);
    (*resource).flags = flags;
    return resource as *mut ::core::ffi::c_void as *mut GFileEnumerator;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_enumerator_next_file(
    mut enumerator: *mut GFileEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut resource: *mut GResourceFileEnumerator =
        enumerator as *mut ::core::ffi::c_void as *mut GResourceFileEnumerator;
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    if (*resource).children.is_null()
        || (*(*resource).children.offset((*resource).index as isize)).is_null()
    {
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    let fresh0 = (*resource).index;
    (*resource).index = (*resource).index + 1;
    path = g_build_path(
        b"/\0" as *const u8 as *const gchar,
        (*resource).path,
        *(*resource).children.offset(fresh0 as isize),
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    file = safe_c2rust_g_resource_file_new_for_path(path);
    g_free(path as gpointer);
    info = g_file_query_info(
        file,
        (*resource).attributes,
        (*resource).flags,
        cancellable,
        error,
    );
    g_object_unref(file as gpointer);
    return info;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_enumerator_close(
    mut enumerator: *mut GFileEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    return TRUE;
}
static mut safe_c2rust_GResourceFileInputStream_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust_g_resource_file_input_stream_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GResourceFileInputStream_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GResourceFileInputStream_private_offset,
        );
    }
    safe_c2rust_g_resource_file_input_stream_class_init(
        klass as *mut GResourceFileInputStreamClass,
    );
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_file_input_stream_get_type(),
        g_intern_static_string(b"GResourceFileInputStream\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GResourceFileInputStreamClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_resource_file_input_stream_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GResourceFileInputStream>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GResourceFileInputStream) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_resource_file_input_stream_init
                    as unsafe extern "C" fn(*mut GResourceFileInputStream) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust__g_resource_file_input_stream_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_resource_file_input_stream_get_type_once();
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
static mut safe_c2rust_g_resource_file_input_stream_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_finalize(mut object: *mut GObject) {
    let mut file: *mut GResourceFileInputStream =
        object as *mut ::core::ffi::c_void as *mut GResourceFileInputStream;
    g_object_unref((*file).stream as gpointer);
    g_object_unref((*file).file as gpointer);
    (*(safe_c2rust_g_resource_file_input_stream_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_class_init(
    mut klass: *mut GResourceFileInputStreamClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut stream_class: *mut GInputStreamClass =
        klass as *mut ::core::ffi::c_void as *mut GInputStreamClass;
    let mut file_stream_class: *mut GFileInputStreamClass =
        klass as *mut ::core::ffi::c_void as *mut GFileInputStreamClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_resource_file_input_stream_finalize
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*stream_class).read_fn = Some(
        safe_c2rust_g_resource_file_input_stream_read
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                *mut ::core::ffi::c_void,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*stream_class).skip = Some(
        safe_c2rust_g_resource_file_input_stream_skip
            as unsafe extern "C" fn(
                *mut GInputStream,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                gsize,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gssize,
        >;
    (*stream_class).close_fn = Some(
        safe_c2rust_g_resource_file_input_stream_close
            as unsafe extern "C" fn(
                *mut GInputStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GInputStream,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*file_stream_class).tell = Some(
        safe_c2rust_g_resource_file_input_stream_tell
            as unsafe extern "C" fn(*mut GFileInputStream) -> goffset,
    )
        as Option<unsafe extern "C" fn(*mut GFileInputStream) -> goffset>;
    (*file_stream_class).can_seek = Some(
        safe_c2rust_g_resource_file_input_stream_can_seek
            as unsafe extern "C" fn(*mut GFileInputStream) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFileInputStream) -> gboolean>;
    (*file_stream_class).seek = Some(
        safe_c2rust_g_resource_file_input_stream_seek
            as unsafe extern "C" fn(
                *mut GFileInputStream,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileInputStream,
                goffset,
                GSeekType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*file_stream_class).query_info = Some(
        safe_c2rust_g_resource_file_input_stream_query_info
            as unsafe extern "C" fn(
                *mut GFileInputStream,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileInputStream,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_init(
    mut info: *mut GResourceFileInputStream,
) {
}
unsafe extern "C" fn safe_c2rust__g_resource_file_input_stream_new(
    mut in_stream: *mut GInputStream,
    mut file: *mut GFile,
) -> *mut GFileInputStream {
    let mut stream: *mut GResourceFileInputStream =
        ::core::ptr::null_mut::<GResourceFileInputStream>();
    stream = g_object_new(
        safe_c2rust__g_resource_file_input_stream_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GResourceFileInputStream;
    (*stream).stream =
        g_object_ref(in_stream as gpointer) as *mut GInputStream as *mut GInputStream;
    (*stream).file = g_object_ref(file as gpointer) as *mut GFile as *mut GFile;
    return stream as *mut ::core::ffi::c_void as *mut GFileInputStream;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_read(
    mut stream: *mut GInputStream,
    mut buffer: *mut ::core::ffi::c_void,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut file: *mut GResourceFileInputStream =
        stream as *mut ::core::ffi::c_void as *mut GResourceFileInputStream;
    return g_input_stream_read((*file).stream, buffer, count, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_skip(
    mut stream: *mut GInputStream,
    mut count: gsize,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gssize {
    let mut file: *mut GResourceFileInputStream =
        stream as *mut ::core::ffi::c_void as *mut GResourceFileInputStream;
    return g_input_stream_skip((*file).stream, count, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_close(
    mut stream: *mut GInputStream,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut file: *mut GResourceFileInputStream =
        stream as *mut ::core::ffi::c_void as *mut GResourceFileInputStream;
    return g_input_stream_close((*file).stream, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_tell(
    mut stream: *mut GFileInputStream,
) -> goffset {
    let mut file: *mut GResourceFileInputStream =
        stream as *mut ::core::ffi::c_void as *mut GResourceFileInputStream;
    if ({
        let mut __inst: *mut GTypeInstance = (*file).stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        return 0 as goffset;
    }
    return g_seekable_tell((*file).stream as *mut ::core::ffi::c_void as *mut GSeekable);
}
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_can_seek(
    mut stream: *mut GFileInputStream,
) -> gboolean {
    let mut file: *mut GResourceFileInputStream =
        stream as *mut ::core::ffi::c_void as *mut GResourceFileInputStream;
    return (({
        let mut __inst: *mut GTypeInstance = (*file).stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
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
        && g_seekable_can_seek((*file).stream as *mut ::core::ffi::c_void as *mut GSeekable) != 0)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_seek(
    mut stream: *mut GFileInputStream,
    mut offset: goffset,
    mut type_0: GSeekType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut file: *mut GResourceFileInputStream =
        stream as *mut ::core::ffi::c_void as *mut GResourceFileInputStream;
    if ({
        let mut __inst: *mut GTypeInstance = (*file).stream as *mut GTypeInstance;
        let mut __t: GType = g_seekable_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Input stream doesn\xE2\x80\x99t implement seek\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    return g_seekable_seek(
        (*file).stream as *mut ::core::ffi::c_void as *mut GSeekable,
        offset,
        type_0,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_resource_file_input_stream_query_info(
    mut stream: *mut GFileInputStream,
    mut attributes: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut file: *mut GResourceFileInputStream =
        stream as *mut ::core::ffi::c_void as *mut GResourceFileInputStream;
    return g_file_query_info(
        (*file).file,
        attributes,
        G_FILE_QUERY_INFO_NONE,
        cancellable,
        error,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
