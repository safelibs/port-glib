extern "C" {
    pub type _GBytes;
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GCancellable;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GFileAttributeMatcher;
    pub type _GFileAttributeInfoList;
    pub type _GFileIcon;
    pub type _GIcon;
    pub type _GEmblem;
    pub type _GEmblemedIconPrivate;
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
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
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
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
    fn g_string_append_uri_escaped(
        string: *mut GString,
        unescaped: *const gchar,
        reserved_chars_allowed: *const gchar,
        allow_utf8: gboolean,
    ) -> *mut GString;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_take_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_get_string(value: *mut GVariant, length: *mut gsize) -> *const gchar;
    fn g_variant_get_strv(value: *mut GVariant, length: *mut gsize) -> *mut *const gchar;
    fn g_variant_lookup(
        dictionary: *mut GVariant,
        key: *const gchar,
        format_string: *const gchar,
        ...
    ) -> gboolean;
    fn g_variant_get_data_as_bytes(value: *mut GVariant) -> *mut GBytes;
    fn g_variant_iter_free(iter: *mut GVariantIter);
    fn g_variant_iter_next_value(iter: *mut GVariantIter) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_uri_parse_scheme(uri: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_from_name(name: *const gchar) -> GType;
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_interface_add_prerequisite(interface_type: GType, prerequisite_type: GType);
    fn g_type_ensure(type_0: GType);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_type_name_from_instance(instance: *mut GTypeInstance) -> *const gchar;
    fn g_object_get(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_unref(object: gpointer);
    fn g_enum_get_value_by_nick(enum_class: *mut GEnumClass, nick: *const gchar)
        -> *mut GEnumValue;
    fn g_themed_icon_get_type() -> GType;
    fn g_themed_icon_new(iconname: *const ::core::ffi::c_char) -> *mut GIcon;
    fn g_themed_icon_new_from_names(
        iconnames: *mut *mut ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> *mut GIcon;
    fn g_file_icon_get_type() -> GType;
    fn g_file_icon_new(file: *mut GFile) -> *mut GIcon;
    fn g_file_icon_get_file(icon: *mut GFileIcon) -> *mut GFile;
    fn g_emblem_get_type() -> GType;
    fn g_emblem_new(icon: *mut GIcon) -> *mut GEmblem;
    fn g_emblem_new_with_origin(icon: *mut GIcon, origin: GEmblemOrigin) -> *mut GEmblem;
    fn g_emblemed_icon_get_type() -> GType;
    fn g_emblemed_icon_new(icon: *mut GIcon, emblem: *mut GEmblem) -> *mut GIcon;
    fn g_emblemed_icon_add_emblem(emblemed: *mut GEmblemedIcon, emblem: *mut GEmblem);
    fn g_bytes_icon_new(bytes: *mut GBytes) -> *mut GIcon;
    fn g_file_new_for_commandline_arg(arg: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_get_path(file: *mut GFile) -> *mut ::core::ffi::c_char;
    fn g_file_get_uri(file: *mut GFile) -> *mut ::core::ffi::c_char;
    fn g_file_is_native(file: *mut GFile) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn g_emblem_origin_get_type() -> GType;
    fn g_vfs_get_default() -> *mut GVfs;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
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
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GBytes = _GBytes;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantIter {
    pub x: [guintptr; 16],
}
pub type GVariantIter = _GVariantIter;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumClass {
    pub g_type_class: GTypeClass,
    pub minimum: gint,
    pub maximum: gint,
    pub n_values: guint,
    pub values: *mut GEnumValue,
}
pub type GEnumValue = _GEnumValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumValue {
    pub value: gint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GEnumClass = _GEnumClass;
pub type GFileQueryInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS: GFileQueryInfoFlags = 1;
pub const G_FILE_QUERY_INFO_NONE: GFileQueryInfoFlags = 0;
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
pub type GEmblemOrigin = ::core::ffi::c_uint;
pub const G_EMBLEM_ORIGIN_TAG: GEmblemOrigin = 3;
pub const G_EMBLEM_ORIGIN_LIVEMETADATA: GEmblemOrigin = 2;
pub const G_EMBLEM_ORIGIN_DEVICE: GEmblemOrigin = 1;
pub const G_EMBLEM_ORIGIN_UNKNOWN: GEmblemOrigin = 0;
pub type GCancellable = _GCancellable;
pub type GFile = _GFile;
pub type GFileInfo = _GFileInfo;
pub type GFileAttributeMatcher = _GFileAttributeMatcher;
pub type GFileAttributeInfoList = _GFileAttributeInfoList;
pub type GFileIcon = _GFileIcon;
pub type GIcon = _GIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVfs {
    pub parent_instance: GObject,
}
pub type GVfs = _GVfs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIconIface {
    pub g_iface: GTypeInterface,
    pub hash: Option<unsafe extern "C" fn(*mut GIcon) -> guint>,
    pub equal: Option<unsafe extern "C" fn(*mut GIcon, *mut GIcon) -> gboolean>,
    pub to_tokens: Option<unsafe extern "C" fn(*mut GIcon, *mut GPtrArray, *mut gint) -> gboolean>,
    pub from_tokens:
        Option<unsafe extern "C" fn(*mut *mut gchar, gint, gint, *mut *mut GError) -> *mut GIcon>,
    pub serialize: Option<unsafe extern "C" fn(*mut GIcon) -> *mut GVariant>,
}
pub type GIconIface = _GIconIface;
pub type GIconInterface = GIconIface;
pub type GVfsClass = _GVfsClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVfsClass {
    pub parent_class: GObjectClass,
    pub is_active: Option<unsafe extern "C" fn(*mut GVfs) -> gboolean>,
    pub get_file_for_path:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>,
    pub get_file_for_uri:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>,
    pub get_supported_uri_schemes: Option<unsafe extern "C" fn(*mut GVfs) -> *const *const gchar>,
    pub parse_name:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>,
    pub local_file_add_info: Option<
        unsafe extern "C" fn(
            *mut GVfs,
            *const ::core::ffi::c_char,
            guint64,
            *mut GFileAttributeMatcher,
            *mut GFileInfo,
            *mut GCancellable,
            *mut gpointer,
            *mut GDestroyNotify,
        ) -> (),
    >,
    pub add_writable_namespaces:
        Option<unsafe extern "C" fn(*mut GVfs, *mut GFileAttributeInfoList) -> ()>,
    pub local_file_set_attributes: Option<
        unsafe extern "C" fn(
            *mut GVfs,
            *const ::core::ffi::c_char,
            *mut GFileInfo,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub local_file_removed:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> ()>,
    pub local_file_moved: Option<
        unsafe extern "C" fn(
            *mut GVfs,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub deserialize_icon: Option<unsafe extern "C" fn(*mut GVfs, *mut GVariant) -> *mut GIcon>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
}
pub type GEmblem = _GEmblem;
pub type GEmblemedIcon = _GEmblemedIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEmblemedIcon {
    pub parent_instance: GObject,
    pub priv_0: *mut GEmblemedIconPrivate,
}
pub type GEmblemedIconPrivate = _GEmblemedIconPrivate;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const G_DIR_SEPARATOR: ::core::ffi::c_int = '/' as i32;
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
pub const G_VARIANT_TYPE_STRING: *const GVariantType =
    b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_STRING_ARRAY: *const GVariantType =
    b"as\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_BYTESTRING: *const GVariantType =
    b"ay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_ICON_SERIALIZATION_MAGIC0: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b". \0") };
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_icon_get_type() -> GType {
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
        let mut g_define_type_id: GType = g_type_register_static_simple(
            G_TYPE_INTERFACE,
            g_intern_static_string(b"GIcon\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GIconInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GIconInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_icon_default_init
                        as unsafe extern "C" fn(*mut GIconInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL_0),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
        }
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
unsafe extern "C" fn safe_c2rust_g_icon_default_init(mut iface: *mut GIconInterface) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_icon_hash(mut icon: gconstpointer) -> guint {
    let mut iface: *mut GIconIface = ::core::ptr::null_mut::<GIconIface>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_icon_get_type();
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
            b"G_IS_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    iface = g_type_interface_peek(
        (*(icon as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_icon_get_type(),
    ) as *mut GIconIface;
    return Some((*iface).hash.expect("non-null function pointer"))
        .expect("non-null function pointer")(icon as *mut GIcon);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_icon_equal(
    mut icon1: *mut GIcon,
    mut icon2: *mut GIcon,
) -> gboolean {
    let mut iface: *mut GIconIface = ::core::ptr::null_mut::<GIconIface>();
    if icon1.is_null() && icon2.is_null() {
        return TRUE;
    }
    if icon1.is_null() || icon2.is_null() {
        return FALSE;
    }
    if (*(*(icon1 as *mut GTypeInstance)).g_class).g_type
        != (*(*(icon2 as *mut GTypeInstance)).g_class).g_type
    {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(icon1 as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_icon_get_type(),
    ) as *mut GIconIface;
    return Some((*iface).equal.expect("non-null function pointer"))
        .expect("non-null function pointer")(icon1, icon2);
}
unsafe extern "C" fn safe_c2rust_g_icon_to_string_tokenized(
    mut icon: *mut GIcon,
    mut s: *mut GString,
) -> gboolean {
    let mut tokens: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut version: gint = 0;
    let mut icon_iface: *mut GIconIface = ::core::ptr::null_mut::<GIconIface>();
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !icon.is_null() {
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
            b"icon != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_icon_get_type();
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
            b"G_IS_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    icon_iface = g_type_interface_peek(
        (*(icon as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_icon_get_type(),
    ) as *mut GIconIface;
    if (*icon_iface).to_tokens.is_none() {
        return FALSE;
    }
    tokens = g_ptr_array_new();
    if (*icon_iface).to_tokens.expect("non-null function pointer")(icon, tokens, &raw mut version)
        == 0
    {
        g_ptr_array_free(tokens, TRUE);
        return FALSE;
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                g_type_name_from_instance(icon as *mut GTypeInstance) as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                s,
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
            s,
            g_type_name_from_instance(icon as *mut GTypeInstance) as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if version != 0 as ::core::ffi::c_int {
        g_string_append_printf(s, b".%d\0" as *const u8 as *const gchar, version);
    }
    i = 0 as guint;
    while i < (*tokens).len {
        let mut token: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        token = *(*tokens).pdata.offset(i as isize) as *mut ::core::ffi::c_char;
        safe_c2rust_g_string_append_c_inline(s, ' ' as i32 as gchar);
        g_string_append_uri_escaped(
            s,
            token,
            b"!$&'()*+,;=:@/\0" as *const u8 as *const gchar,
            TRUE,
        );
        g_free(token as gpointer);
        i = i.wrapping_add(1);
    }
    g_ptr_array_free(tokens, TRUE);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_icon_to_string(mut icon: *mut GIcon) -> *mut gchar {
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !icon.is_null() {
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
            b"icon != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_icon_get_type();
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
            b"G_IS_ICON (icon)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    ret = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
        let mut __t: GType = g_file_icon_get_type();
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
        let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
        file = g_file_icon_get_file(icon as *mut ::core::ffi::c_void as *mut GFileIcon);
        if g_file_is_native(file) != 0 {
            ret = g_file_get_path(file) as *mut gchar;
            if g_utf8_validate(
                ret,
                -(1 as ::core::ffi::c_int) as gssize,
                ::core::ptr::null_mut::<*const gchar>(),
            ) == 0
            {
                g_free(ret as gpointer);
                ret = ::core::ptr::null_mut::<gchar>();
            }
        } else {
            ret = g_file_get_uri(file) as *mut gchar;
        }
    } else if ({
        let mut __inst: *mut GTypeInstance = icon as *mut GTypeInstance;
        let mut __t: GType = g_themed_icon_get_type();
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
        let mut names: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut use_default_fallbacks: gboolean = FALSE;
        g_object_get(
            icon as *mut ::core::ffi::c_void as *mut GObject as gpointer,
            b"names\0" as *const u8 as *const gchar,
            &raw mut names,
            b"use-default-fallbacks\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut use_default_fallbacks,
            NULL_0,
        );
        if !names.is_null()
            && !(*names.offset(0 as ::core::ffi::c_int as isize)).is_null()
            && *(*names.offset(0 as ::core::ffi::c_int as isize))
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '.' as i32
            && g_utf8_validate(
                *names.offset(0 as ::core::ffi::c_int as isize),
                -(1 as ::core::ffi::c_int) as gssize,
                ::core::ptr::null_mut::<*const gchar>(),
            ) != 0
            && (*names.offset(1 as ::core::ffi::c_int as isize)).is_null()
            && use_default_fallbacks == 0
        {
            ret = safe_c2rust_g_strdup_inline(*names.offset(0 as ::core::ffi::c_int as isize))
                as *mut gchar;
        }
        g_strfreev(names as *mut *mut gchar);
    }
    if ret.is_null() {
        let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
        s = g_string_new(G_ICON_SERIALIZATION_MAGIC0.as_ptr() as *const gchar);
        if safe_c2rust_g_icon_to_string_tokenized(icon, s) != 0 {
            ret = if 0 != 0 {
                if 0 as ::core::ffi::c_int != 0 {
                    g_string_free(s, 0 as gboolean)
                } else {
                    g_string_free_and_steal(s)
                }
            } else {
                g_string_free(s, 0 as gboolean)
            };
        } else {
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
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_icon_new_from_tokens(
    mut tokens: *mut *mut ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GIcon {
    let mut current_block: u64;
    let mut icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    let mut typename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut version_str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut type_0: GType = 0;
    let mut klass: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut icon_iface: *mut GIconIface = ::core::ptr::null_mut::<GIconIface>();
    let mut version: gint = 0;
    let mut endp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut num_tokens: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    icon = ::core::ptr::null_mut::<GIcon>();
    klass = NULL_0 as gpointer;
    num_tokens = g_strv_length(tokens as *mut *mut gchar) as ::core::ffi::c_int;
    if num_tokens < 1 as ::core::ffi::c_int {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Wrong number of tokens (%d)\0" as *const u8 as *const gchar),
            num_tokens,
        );
    } else {
        typename = *tokens.offset(0 as ::core::ffi::c_int as isize);
        version_str = strchr(typename, '.' as i32);
        if !version_str.is_null() {
            *version_str = 0 as ::core::ffi::c_char;
            version_str = version_str.offset(1 as ::core::ffi::c_int as isize);
        }
        type_0 = g_type_from_name(*tokens.offset(0 as ::core::ffi::c_int as isize));
        if type_0 == 0 as GType {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(b"No type for class name %s\0" as *const u8 as *const gchar),
                *tokens.offset(0 as ::core::ffi::c_int as isize),
            );
        } else if !(type_0 == safe_c2rust_g_icon_get_type()
            || g_type_is_a(type_0, safe_c2rust_g_icon_get_type()) != 0)
        {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Type %s does not implement the GIcon interface\0" as *const u8
                        as *const gchar,
                ),
                *tokens.offset(0 as ::core::ffi::c_int as isize),
            );
        } else {
            klass = g_type_class_ref(type_0);
            if klass.is_null() {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(b"Type %s is not classed\0" as *const u8 as *const gchar),
                    *tokens.offset(0 as ::core::ffi::c_int as isize),
                );
            } else {
                version = 0 as ::core::ffi::c_int as gint;
                if !version_str.is_null() {
                    version = strtol(version_str, &raw mut endp, 10 as ::core::ffi::c_int) as gint;
                    if endp.is_null() || *endp as ::core::ffi::c_int != '\0' as i32 {
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Malformed version number: %s\0" as *const u8 as *const gchar,
                            ),
                            version_str,
                        );
                        current_block = 3207391197105929111;
                    } else {
                        current_block = 5634871135123216486;
                    }
                } else {
                    current_block = 5634871135123216486;
                }
                match current_block {
                    3207391197105929111 => {}
                    _ => {
                        icon_iface = g_type_interface_peek(klass, safe_c2rust_g_icon_get_type())
                            as *mut GIconIface;
                        if ({
                            let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                            if !icon_iface.is_null() {
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
                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gicon.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                360 as ::core::ffi::c_int,
                                G_STRFUNC,
                                b"icon_iface != NULL\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        if (*icon_iface).from_tokens.is_none() {
                            g_set_error(
                                error,
                                g_io_error_quark(),
                                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"Type %s does not implement from_tokens() on the GIcon interface\0"
                                        as *const u8 as *const gchar,
                                ),
                                *tokens.offset(0 as ::core::ffi::c_int as isize),
                            );
                        } else {
                            i = 1 as ::core::ffi::c_int;
                            while i < num_tokens {
                                let mut escaped: *mut ::core::ffi::c_char =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                escaped = *tokens.offset(i as isize);
                                let ref mut fresh1 = *tokens.offset(i as isize);
                                *fresh1 = g_uri_unescape_string(
                                    escaped,
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                );
                                g_free(escaped as gpointer);
                                i += 1;
                            }
                            icon = (*icon_iface)
                                .from_tokens
                                .expect("non-null function pointer")(
                                tokens.offset(1 as ::core::ffi::c_int as isize),
                                num_tokens as gint - 1 as gint,
                                version,
                                error,
                            );
                        }
                    }
                }
            }
        }
    }
    if !klass.is_null() {
        g_type_class_unref(klass);
    }
    return icon;
}
unsafe extern "C" fn safe_c2rust_ensure_builtin_icon_types() {
    g_type_ensure(g_themed_icon_get_type());
    g_type_ensure(g_file_icon_get_type());
    g_type_ensure(g_emblemed_icon_get_type());
    g_type_ensure(g_emblem_get_type());
}
unsafe extern "C" fn safe_c2rust_g_icon_new_for_string_simple(mut str: *const gchar) -> *mut GIcon {
    let mut scheme: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    if *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32 {
        return ::core::ptr::null_mut::<GIcon>();
    }
    scheme = g_uri_parse_scheme(str as *const ::core::ffi::c_char) as *mut gchar;
    if !scheme.is_null()
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == G_DIR_SEPARATOR
    {
        let mut location: *mut GFile = ::core::ptr::null_mut::<GFile>();
        location = g_file_new_for_commandline_arg(str as *const ::core::ffi::c_char);
        icon = g_file_icon_new(location);
        g_object_unref(location as gpointer);
    } else {
        icon = g_themed_icon_new(str as *const ::core::ffi::c_char);
    }
    g_free(scheme as gpointer);
    return icon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_icon_new_for_string(
    mut str: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GIcon {
    let mut icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    icon = safe_c2rust_g_icon_new_for_string_simple(str);
    if !icon.is_null() {
        return icon;
    }
    safe_c2rust_ensure_builtin_icon_types();
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = str as *const ::core::ffi::c_char;
            let __prefix: *const ::core::ffi::c_char =
                b". \0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_18
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
        g_str_has_prefix(str, b". \0" as *const u8 as *const gchar)
    } != 0
    {
        let mut tokens: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        tokens = g_strsplit(
            str.offset(::core::mem::size_of::<[::core::ffi::c_char; 3]>() as usize as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
            b" \0" as *const u8 as *const gchar,
            0 as gint,
        );
        icon = safe_c2rust_g_icon_new_from_tokens(tokens as *mut *mut ::core::ffi::c_char, error);
        g_strfreev(tokens);
    } else {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Can\xE2\x80\x99t handle the supplied version of the icon encoding\0" as *const u8
                    as *const gchar,
            ),
        );
    }
    return icon;
}
unsafe extern "C" fn safe_c2rust_g_icon_deserialize_emblem(
    mut value: *mut GVariant,
) -> *mut GEmblem {
    let mut emblem_metadata: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut emblem_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut origin_nick: *const gchar = ::core::ptr::null::<gchar>();
    let mut emblem_icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    let mut emblem: *mut GEmblem = ::core::ptr::null_mut::<GEmblem>();
    g_variant_get(
        value,
        b"(v@a{sv})\0" as *const u8 as *const gchar,
        &raw mut emblem_data,
        &raw mut emblem_metadata,
    );
    emblem = ::core::ptr::null_mut::<GEmblem>();
    emblem_icon = safe_c2rust_g_icon_deserialize(emblem_data);
    if !emblem_icon.is_null() {
        if g_variant_lookup(
            emblem_metadata,
            b"origin\0" as *const u8 as *const gchar,
            b"&s\0" as *const u8 as *const gchar,
            &raw mut origin_nick,
        ) != 0
        {
            let mut origin_class: *mut GEnumClass = ::core::ptr::null_mut::<GEnumClass>();
            let mut origin_value: *mut GEnumValue = ::core::ptr::null_mut::<GEnumValue>();
            origin_class = g_type_class_ref(g_emblem_origin_get_type()) as *mut GEnumClass;
            origin_value = g_enum_get_value_by_nick(origin_class, origin_nick);
            if !origin_value.is_null() {
                emblem =
                    g_emblem_new_with_origin(emblem_icon, (*origin_value).value as GEmblemOrigin);
            }
            g_type_class_unref(origin_class as gpointer);
        }
        if emblem.is_null() {
            emblem = g_emblem_new(emblem_icon);
        }
        g_object_unref(emblem_icon as gpointer);
    }
    g_variant_unref(emblem_metadata);
    g_variant_unref(emblem_data);
    return emblem;
}
unsafe extern "C" fn safe_c2rust_g_icon_deserialize_emblemed(
    mut value: *mut GVariant,
) -> *mut GIcon {
    let mut emblems: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    let mut icon_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut main_icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    let mut icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    g_variant_get(
        value,
        b"(va(va{sv}))\0" as *const u8 as *const gchar,
        &raw mut icon_data,
        &raw mut emblems,
    );
    main_icon = safe_c2rust_g_icon_deserialize(icon_data);
    if !main_icon.is_null() {
        let mut emblem_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        icon = g_emblemed_icon_new(main_icon, ::core::ptr::null_mut::<GEmblem>());
        loop {
            emblem_data = g_variant_iter_next_value(emblems);
            if emblem_data.is_null() {
                break;
            }
            let mut emblem: *mut GEmblem = ::core::ptr::null_mut::<GEmblem>();
            emblem = safe_c2rust_g_icon_deserialize_emblem(emblem_data);
            if !emblem.is_null() {
                g_emblemed_icon_add_emblem(
                    icon as *mut ::core::ffi::c_void as *mut GEmblemedIcon,
                    emblem,
                );
                g_object_unref(emblem as gpointer);
            }
            g_variant_unref(emblem_data);
        }
        g_object_unref(main_icon as gpointer);
    } else {
        icon = ::core::ptr::null_mut::<GIcon>();
    }
    g_variant_iter_free(emblems);
    g_variant_unref(icon_data);
    return icon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_icon_deserialize(mut value: *mut GVariant) -> *mut GIcon {
    let mut tag: *const gchar = ::core::ptr::null::<gchar>();
    let mut val: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if g_variant_is_of_type(
            value,
            b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) != 0
            || g_variant_is_of_type(
                value,
                g_variant_type_checked_(b"(sv)\0" as *const u8 as *const gchar),
            ) != 0
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
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_STRING) || g_variant_is_of_type (value, G_VARIANT_TYPE (\"(sv)\"))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    if g_variant_is_of_type(value, G_VARIANT_TYPE_STRING) != 0 {
        return safe_c2rust_g_icon_new_for_string_simple(g_variant_get_string(
            value,
            ::core::ptr::null_mut::<gsize>(),
        ));
    }
    g_variant_get(
        value,
        b"(&sv)\0" as *const u8 as *const gchar,
        &raw mut tag,
        &raw mut val,
    );
    icon = ::core::ptr::null_mut::<GIcon>();
    if strcmp(
        tag as *const ::core::ffi::c_char,
        b"file\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_variant_is_of_type(val, G_VARIANT_TYPE_STRING) != 0
    {
        let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
        file = g_file_new_for_commandline_arg(g_variant_get_string(
            val,
            ::core::ptr::null_mut::<gsize>(),
        ) as *const ::core::ffi::c_char);
        icon = g_file_icon_new(file);
        g_object_unref(file as gpointer);
    } else if strcmp(
        tag as *const ::core::ffi::c_char,
        b"themed\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_variant_is_of_type(val, G_VARIANT_TYPE_STRING_ARRAY) != 0
    {
        let mut names: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
        let mut size: gsize = 0;
        names = g_variant_get_strv(val, &raw mut size);
        icon = g_themed_icon_new_from_names(
            names as *mut *mut ::core::ffi::c_char,
            size as ::core::ffi::c_int,
        );
        g_free(names as gpointer);
    } else if strcmp(
        tag as *const ::core::ffi::c_char,
        b"bytes\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_variant_is_of_type(val, G_VARIANT_TYPE_BYTESTRING) != 0
    {
        let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
        bytes = g_variant_get_data_as_bytes(val);
        icon = g_bytes_icon_new(bytes);
        g_bytes_unref(bytes);
    } else if strcmp(
        tag as *const ::core::ffi::c_char,
        b"emblem\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_variant_is_of_type(
            val,
            g_variant_type_checked_(b"(va{sv})\0" as *const u8 as *const gchar),
        ) != 0
    {
        let mut emblem: *mut GEmblem = ::core::ptr::null_mut::<GEmblem>();
        emblem = safe_c2rust_g_icon_deserialize_emblem(val);
        if !emblem.is_null() {
            icon = emblem as *mut ::core::ffi::c_void as *mut GIcon;
        }
    } else if strcmp(
        tag as *const ::core::ffi::c_char,
        b"emblemed\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_variant_is_of_type(
            val,
            g_variant_type_checked_(b"(va(va{sv}))\0" as *const u8 as *const gchar),
        ) != 0
    {
        icon = safe_c2rust_g_icon_deserialize_emblemed(val);
    } else if strcmp(
        tag as *const ::core::ffi::c_char,
        b"gvfs\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
        let mut vfs: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
        vfs = g_vfs_get_default();
        class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
        if (*class).deserialize_icon.is_some() {
            icon = Some(
                (*class)
                    .deserialize_icon
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(vfs, val);
        }
    }
    g_variant_unref(val);
    return icon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_icon_serialize(mut icon: *mut GIcon) -> *mut GVariant {
    let mut iface: *mut GIconInterface = ::core::ptr::null_mut::<GIconInterface>();
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    iface = g_type_interface_peek(
        (*(icon as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_icon_get_type(),
    ) as *mut GIconIface as *mut GIconInterface;
    if (*iface).serialize.is_none() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_icon_serialize() on icon type '%s' is not implemented\0" as *const u8
                as *const gchar,
            g_type_name((*(*(icon as *mut GTypeInstance)).g_class).g_type),
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    result = Some((*iface).serialize.expect("non-null function pointer"))
        .expect("non-null function pointer")(icon);
    if !result.is_null() {
        g_variant_take_ref(result);
        if g_variant_is_of_type(
            result,
            g_variant_type_checked_(b"(sv)\0" as *const u8 as *const gchar),
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_icon_serialize() on icon type '%s' returned GVariant of type '%s' but it must return one with type '(sv)'\0"
                    as *const u8 as *const gchar,
                g_type_name((*(*(icon as *mut GTypeInstance)).g_class).g_type),
                g_variant_get_type_string(result),
            );
            g_variant_unref(result);
            result = ::core::ptr::null_mut::<GVariant>();
        }
    }
    return result;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
