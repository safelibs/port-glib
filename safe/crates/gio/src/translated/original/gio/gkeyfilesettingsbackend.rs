use ::c2rust_bitfields;
extern "C" {
    pub type _GChecksum;
    pub type _GData;
    pub type _GHashTable;
    pub type _GKeyFile;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GTree;
    pub type _GCancellable;
    pub type _GSettingsBackendPrivate;
    pub type _GPermission;
    pub type _GFileMonitorPrivate;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GIOExtension;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_clear_error(err: *mut *mut GError);
    fn g_get_user_config_dir() -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_checksum_new(checksum_type: GChecksumType) -> *mut GChecksum;
    fn g_checksum_free(checksum: *mut GChecksum);
    fn g_checksum_update(checksum: *mut GChecksum, data: *const guchar, length: gssize);
    fn g_checksum_get_digest(checksum: *mut GChecksum, buffer: *mut guint8, digest_len: *mut gsize);
    fn g_file_error_quark() -> GQuark;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_mkdir_with_parents(pathname: *const gchar, mode: gint) -> gint;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_timeout_add(interval: guint, function: GSourceFunc, data: gpointer) -> guint;
    fn g_source_remove(tag: guint) -> gboolean;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_key_file_new() -> *mut GKeyFile;
    fn g_key_file_unref(key_file: *mut GKeyFile);
    fn g_key_file_free(key_file: *mut GKeyFile);
    fn g_key_file_load_from_file(
        key_file: *mut GKeyFile,
        file: *const gchar,
        flags: GKeyFileFlags,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_key_file_load_from_data(
        key_file: *mut GKeyFile,
        data: *const gchar,
        length: gsize,
        flags: GKeyFileFlags,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_key_file_to_data(
        key_file: *mut GKeyFile,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_key_file_get_groups(key_file: *mut GKeyFile, length: *mut gsize) -> *mut *mut gchar;
    fn g_key_file_get_keys(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut *mut gchar;
    fn g_key_file_get_value(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_key_file_set_value(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        value: *const gchar,
    );
    fn g_key_file_remove_key(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        key: *const gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_key_file_remove_group(
        key_file: *mut GKeyFile,
        group_name: *const gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_variant_type_equal(type1: gconstpointer, type2: gconstpointer) -> gboolean;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_print(value: *mut GVariant, type_annotate: gboolean) -> *mut gchar;
    fn g_variant_parse(
        type_0: *const GVariantType,
        text: *const gchar,
        limit: *const gchar,
        endptr: *mut *const gchar,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn g_tree_new_full(
        key_compare_func: GCompareDataFunc,
        key_compare_data: gpointer,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GTree;
    fn g_tree_unref(tree: *mut GTree);
    fn g_tree_insert(tree: *mut GTree, key: gpointer, value: gpointer);
    fn g_tree_remove(tree: *mut GTree, key: gconstpointer) -> gboolean;
    fn g_tree_lookup(tree: *mut GTree, key: gconstpointer) -> gpointer;
    fn g_tree_foreach(tree: *mut GTree, func: GTraverseFunc, user_data: gpointer);
    fn g_tree_nnodes(tree: *mut GTree) -> gint;
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_file_new_for_path(path: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_peek_path(file: *mut GFile) -> *const ::core::ffi::c_char;
    fn g_file_get_parent(file: *mut GFile) -> *mut GFile;
    fn g_file_query_info(
        file: *mut GFile,
        attributes: *const ::core::ffi::c_char,
        flags: GFileQueryInfoFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_file_monitor(
        file: *mut GFile,
        flags: GFileMonitorFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileMonitor;
    fn g_file_load_contents(
        file: *mut GFile,
        cancellable: *mut GCancellable,
        contents: *mut *mut ::core::ffi::c_char,
        length: *mut gsize,
        etag_out: *mut *mut ::core::ffi::c_char,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_replace_contents(
        file: *mut GFile,
        contents: *const ::core::ffi::c_char,
        length: gsize,
        etag: *const ::core::ffi::c_char,
        make_backup: gboolean,
        flags: GFileCreateFlags,
        new_etag: *mut *mut ::core::ffi::c_char,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_info_get_attribute_boolean(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn g_file_monitor_cancel(monitor: *mut GFileMonitor) -> gboolean;
    fn g_simple_permission_new(allowed: gboolean) -> *mut GPermission;
    fn g_settings_backend_get_type() -> GType;
    fn g_settings_backend_changed(
        backend: *mut GSettingsBackend,
        key: *const gchar,
        origin_tag: gpointer,
    );
    fn g_settings_backend_path_writable_changed(backend: *mut GSettingsBackend, path: *const gchar);
    fn g_settings_backend_changed_tree(
        backend: *mut GSettingsBackend,
        tree: *mut GTree,
        origin_tag: gpointer,
    );
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
    fn glib_should_use_portal() -> gboolean;
    fn glib_has_dconf_access_in_sandbox() -> gboolean;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
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
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
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
pub type GChecksum = _GChecksum;
pub type GData = _GData;
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
pub type GHashTable = _GHashTable;
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
pub type GKeyFile = _GKeyFile;
pub type GKeyFileFlags = ::core::ffi::c_uint;
pub const G_KEY_FILE_KEEP_TRANSLATIONS: GKeyFileFlags = 2;
pub const G_KEY_FILE_KEEP_COMMENTS: GKeyFileFlags = 1;
pub const G_KEY_FILE_NONE: GKeyFileFlags = 0;
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
pub type GTree = _GTree;
pub type GTraverseFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>;
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
pub type GFileQueryInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS: GFileQueryInfoFlags = 1;
pub const G_FILE_QUERY_INFO_NONE: GFileQueryInfoFlags = 0;
pub type GFileCreateFlags = ::core::ffi::c_uint;
pub const G_FILE_CREATE_REPLACE_DESTINATION: GFileCreateFlags = 2;
pub const G_FILE_CREATE_PRIVATE: GFileCreateFlags = 1;
pub const G_FILE_CREATE_NONE: GFileCreateFlags = 0;
pub type GFileMonitorFlags = ::core::ffi::c_uint;
pub const G_FILE_MONITOR_WATCH_MOVES: GFileMonitorFlags = 8;
pub const G_FILE_MONITOR_WATCH_HARD_LINKS: GFileMonitorFlags = 4;
pub const G_FILE_MONITOR_SEND_MOVED: GFileMonitorFlags = 2;
pub const G_FILE_MONITOR_WATCH_MOUNTS: GFileMonitorFlags = 1;
pub const G_FILE_MONITOR_NONE: GFileMonitorFlags = 0;
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
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsBackend {
    pub parent_instance: GObject,
    pub priv_0: *mut GSettingsBackendPrivate,
}
pub type GSettingsBackendPrivate = _GSettingsBackendPrivate;
pub type GSettingsBackend = _GSettingsBackend;
pub type GPermission = _GPermission;
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
pub type GIOExtension = _GIOExtension;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsBackendClass {
    pub parent_class: GObjectClass,
    pub read: Option<
        unsafe extern "C" fn(
            *mut GSettingsBackend,
            *const gchar,
            *const GVariantType,
            gboolean,
        ) -> *mut GVariant,
    >,
    pub get_writable: Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> gboolean>,
    pub write: Option<
        unsafe extern "C" fn(
            *mut GSettingsBackend,
            *const gchar,
            *mut GVariant,
            gpointer,
        ) -> gboolean,
    >,
    pub write_tree:
        Option<unsafe extern "C" fn(*mut GSettingsBackend, *mut GTree, gpointer) -> gboolean>,
    pub reset: Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar, gpointer) -> ()>,
    pub subscribe: Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> ()>,
    pub unsubscribe: Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> ()>,
    pub sync: Option<unsafe extern "C" fn(*mut GSettingsBackend) -> ()>,
    pub get_permission:
        Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> *mut GPermission>,
    pub read_user_value: Option<
        unsafe extern "C" fn(
            *mut GSettingsBackend,
            *const gchar,
            *const GVariantType,
        ) -> *mut GVariant,
    >,
    pub padding: [gpointer; 23],
}
pub type GSettingsBackendClass = _GSettingsBackendClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GKeyfileSettingsBackend {
    pub parent_instance: GSettingsBackend,
    pub keyfile: *mut GKeyFile,
    pub permission: *mut GPermission,
    pub writable: gboolean,
    pub defaults_dir: *mut ::core::ffi::c_char,
    pub system_keyfile: *mut GKeyFile,
    pub system_locks: *mut GHashTable,
    pub prefix: *mut gchar,
    pub prefix_len: gsize,
    pub root_group: *mut gchar,
    pub root_group_len: gsize,
    pub file: *mut GFile,
    pub file_monitor: *mut GFileMonitor,
    pub digest: [guint8; 32],
    pub dir: *mut GFile,
    pub dir_monitor: *mut GFileMonitor,
    pub poll_source_id: guint,
}
pub type GKeyfileSettingsBackendClass = GSettingsBackendClass;
pub const PROP_DEFAULTS_DIR: GKeyfileSettingsBackendProperty = 4;
pub const PROP_ROOT_GROUP: GKeyfileSettingsBackendProperty = 3;
pub const PROP_ROOT_PATH: GKeyfileSettingsBackendProperty = 2;
pub const PROP_FILENAME: GKeyfileSettingsBackendProperty = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WriteManyData {
    pub kfsb: *mut GKeyfileSettingsBackend,
    pub failed: gboolean,
}
pub type GKeyfileSettingsBackendProperty = ::core::ffi::c_uint;
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
pub const G_VARIANT_TYPE_STRING: *const GVariantType =
    b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_FILE_ATTRIBUTE_ACCESS_CAN_WRITE: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"access::can-write\0")
};
pub const G_FILE_ATTRIBUTE_ACCESS_CAN_EXECUTE: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"access::can-execute\0")
};
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_keyfile_settings_backend_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GKeyfileSettingsBackend_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GKeyfileSettingsBackend_private_offset,
        );
    }
    safe_c2rust_g_keyfile_settings_backend_class_init(klass as *mut GKeyfileSettingsBackendClass);
}
static mut safe_c2rust_GKeyfileSettingsBackend_private_offset: gint = 0;
static mut safe_c2rust_g_keyfile_settings_backend_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_settings_backend_get_type(),
        g_intern_static_string(b"GKeyfileSettingsBackend\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GKeyfileSettingsBackendClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_keyfile_settings_backend_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GKeyfileSettingsBackend>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GKeyfileSettingsBackend) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_keyfile_settings_backend_init
                    as unsafe extern "C" fn(*mut GKeyfileSettingsBackend) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gsettings-backend\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"keyfile\0" as *const u8 as *const ::core::ffi::c_char,
        if glib_should_use_portal() != 0 && glib_has_dconf_access_in_sandbox() == 0 {
            110 as gint
        } else {
            10 as gint
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_keyfile_settings_backend_get_type_once();
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
unsafe extern "C" fn safe_c2rust_compute_checksum(
    mut digest: *mut guint8,
    mut contents: gconstpointer,
    mut length: gsize,
) {
    let mut checksum: *mut GChecksum = ::core::ptr::null_mut::<GChecksum>();
    let mut len: gsize = 32 as gsize;
    checksum = g_checksum_new(G_CHECKSUM_SHA256);
    g_checksum_update(checksum, contents as *const guchar, length as gssize);
    g_checksum_get_digest(checksum, digest, &raw mut len);
    g_checksum_free(checksum);
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if len == 32 as gsize {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gkeyfilesettingsbackend.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            107 as ::core::ffi::c_int,
            G_STRFUNC,
            b"len == 32\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_keyfile_write(
    mut kfsb: *mut GKeyfileSettingsBackend,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut length: gsize = 0;
    let mut success: gboolean = 0;
    contents = g_key_file_to_data(
        (*kfsb).keyfile,
        &raw mut length,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    success = g_file_replace_contents(
        (*kfsb).file,
        contents,
        length,
        ::core::ptr::null::<::core::ffi::c_char>(),
        FALSE,
        (G_FILE_CREATE_REPLACE_DESTINATION as ::core::ffi::c_int
            | G_FILE_CREATE_PRIVATE as ::core::ffi::c_int) as GFileCreateFlags,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<GCancellable>(),
        error,
    );
    safe_c2rust_compute_checksum(
        &raw mut (*kfsb).digest as *mut guint8,
        contents as gconstpointer,
        length,
    );
    g_free(contents as gpointer);
    return success;
}
unsafe extern "C" fn safe_c2rust_group_name_matches(
    mut group_name: *const gchar,
    mut prefix: *const gchar,
) -> gboolean {
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while *prefix.offset(i as isize) != 0 {
        if *prefix.offset(i as isize) as ::core::ffi::c_int
            != *group_name.offset(i as isize) as ::core::ffi::c_int
        {
            return FALSE;
        }
        i += 1;
    }
    return (*group_name.offset(i as isize) as ::core::ffi::c_int == '\0' as i32
        || *group_name.offset(i as isize) as ::core::ffi::c_int == '/' as i32)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_convert_path(
    mut kfsb: *mut GKeyfileSettingsBackend,
    mut key: *const gchar,
    mut group: *mut *mut gchar,
    mut basename: *mut *mut gchar,
) -> gboolean {
    let mut key_len: gsize = strlen(key as *const ::core::ffi::c_char) as gsize;
    let mut last_slash: *const gchar = ::core::ptr::null::<gchar>();
    if key_len < (*kfsb).prefix_len
        || memcmp(
            key as *const ::core::ffi::c_void,
            (*kfsb).prefix as *const ::core::ffi::c_void,
            (*kfsb).prefix_len as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    key_len = key_len.wrapping_sub((*kfsb).prefix_len);
    key = key.offset((*kfsb).prefix_len as isize);
    last_slash = strrchr(key as *const ::core::ffi::c_char, '/' as i32);
    if key_len == 0 as gsize
        || !last_slash.is_null()
            && (*last_slash.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32
                || last_slash == key)
    {
        return FALSE;
    }
    if !(*kfsb).root_group.is_null() {
        if !last_slash.is_null()
            && last_slash.offset_from(key) as ::core::ffi::c_long >= 0 as ::core::ffi::c_long
            && last_slash.offset_from(key) as ::core::ffi::c_long as gsize == (*kfsb).root_group_len
            && memcmp(
                key as *const ::core::ffi::c_void,
                (*kfsb).root_group as *const ::core::ffi::c_void,
                last_slash.offset_from(key) as ::core::ffi::c_long as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            return FALSE;
        }
    } else if last_slash.is_null() {
        return FALSE;
    }
    if !group.is_null() {
        if !last_slash.is_null() {
            *group = g_memdup2(
                key as gconstpointer,
                (last_slash.offset_from(key) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                    as gsize,
            ) as *mut gchar;
            *(*group).offset(last_slash.offset_from(key) as ::core::ffi::c_long as isize) =
                '\0' as i32 as gchar;
        } else {
            *group = safe_c2rust_g_strdup_inline((*kfsb).root_group) as *mut gchar;
        }
    }
    if !basename.is_null() {
        if !last_slash.is_null() {
            *basename = g_memdup2(
                last_slash.offset(1 as ::core::ffi::c_int as isize) as gconstpointer,
                key_len.wrapping_sub(last_slash.offset_from(key) as ::core::ffi::c_long as gsize),
            ) as *mut gchar;
        } else {
            *basename =
                safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as *mut gchar;
        }
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_path_is_valid(
    mut kfsb: *mut GKeyfileSettingsBackend,
    mut path: *const gchar,
) -> gboolean {
    return safe_c2rust_convert_path(
        kfsb,
        path,
        ::core::ptr::null_mut::<*mut gchar>(),
        ::core::ptr::null_mut::<*mut gchar>(),
    );
}
unsafe extern "C" fn safe_c2rust_get_from_keyfile(
    mut kfsb: *mut GKeyfileSettingsBackend,
    mut type_0: *const GVariantType,
    mut key: *const gchar,
) -> *mut GVariant {
    let mut return_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut group: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if safe_c2rust_convert_path(kfsb, key, &raw mut group, &raw mut name) != 0 {
        let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut sysstr: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if ({
            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
            if *name != 0 {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gkeyfilesettingsbackend.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                232 as ::core::ffi::c_int,
                G_STRFUNC,
                b"*name\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        sysstr = g_key_file_get_value(
            (*kfsb).system_keyfile,
            group,
            name,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        str = g_key_file_get_value(
            (*kfsb).keyfile,
            group,
            name,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !sysstr.is_null()
            && (g_hash_table_contains((*kfsb).system_locks, key as gconstpointer) != 0
                || str.is_null())
        {
            g_free(str as gpointer);
            str = safe_c2rust_g_steal_pointer(&raw mut sysstr as gpointer) as *mut gchar
                as *mut gchar;
        }
        if !str.is_null() {
            return_value = g_variant_parse(
                type_0,
                str,
                ::core::ptr::null::<gchar>(),
                ::core::ptr::null_mut::<*const gchar>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            if return_value.is_null()
                && g_variant_type_equal(
                    type_0 as gconstpointer,
                    G_VARIANT_TYPE_STRING as gconstpointer,
                ) != 0
                && *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '"' as i32
            {
                let mut s: *mut GString =
                    g_string_sized_new((strlen(str) as gsize).wrapping_add(2 as gsize));
                let mut p: *mut ::core::ffi::c_char = str as *mut ::core::ffi::c_char;
                safe_c2rust_g_string_append_c_inline(s, '"' as i32 as gchar);
                while *p != 0 {
                    if *p as ::core::ffi::c_int == '"' as i32 {
                        safe_c2rust_g_string_append_c_inline(s, '\\' as i32 as gchar);
                    }
                    safe_c2rust_g_string_append_c_inline(s, *p);
                    p = p.offset(1);
                }
                safe_c2rust_g_string_append_c_inline(s, '"' as i32 as gchar);
                return_value = g_variant_parse(
                    type_0,
                    (*s).str_0,
                    ::core::ptr::null::<gchar>(),
                    ::core::ptr::null_mut::<*const gchar>(),
                    ::core::ptr::null_mut::<*mut GError>(),
                );
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
            g_free(str as gpointer);
        }
        g_free(sysstr as gpointer);
        g_free(group as gpointer);
        g_free(name as gpointer);
    }
    return return_value;
}
unsafe extern "C" fn safe_c2rust_set_to_keyfile(
    mut kfsb: *mut GKeyfileSettingsBackend,
    mut key: *const gchar,
    mut value: *mut GVariant,
) -> gboolean {
    let mut group: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if g_hash_table_contains((*kfsb).system_locks, key as gconstpointer) != 0 {
        return FALSE;
    }
    if safe_c2rust_convert_path(kfsb, key, &raw mut group, &raw mut name) != 0 {
        if !value.is_null() {
            let mut str: *mut gchar = g_variant_print(value, FALSE);
            g_key_file_set_value((*kfsb).keyfile, group, name, str);
            g_variant_unref(g_variant_ref_sink(value));
            g_free(str as gpointer);
        } else if *name as ::core::ffi::c_int == '\0' as i32 {
            let mut groups: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
            let mut i: gint = 0;
            groups = g_key_file_get_groups((*kfsb).keyfile, ::core::ptr::null_mut::<gsize>());
            i = 0 as ::core::ffi::c_int as gint;
            while !(*groups.offset(i as isize)).is_null() {
                if safe_c2rust_group_name_matches(*groups.offset(i as isize), group) != 0 {
                    g_key_file_remove_group(
                        (*kfsb).keyfile,
                        *groups.offset(i as isize),
                        ::core::ptr::null_mut::<*mut GError>(),
                    );
                }
                i += 1;
            }
            g_strfreev(groups);
        } else {
            g_key_file_remove_key(
                (*kfsb).keyfile,
                group,
                name,
                ::core::ptr::null_mut::<*mut GError>(),
            );
        }
        g_free(group as gpointer);
        g_free(name as gpointer);
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_read(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut expected_type: *const GVariantType,
    mut default_value: gboolean,
) -> *mut GVariant {
    let mut kfsb: *mut GKeyfileSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GKeyfileSettingsBackend;
    if default_value != 0 {
        return ::core::ptr::null_mut::<GVariant>();
    }
    return safe_c2rust_get_from_keyfile(kfsb, expected_type, key);
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_write_one(
    mut key: gpointer,
    mut value: gpointer,
    mut user_data: gpointer,
) -> gboolean {
    let mut data: *mut WriteManyData = user_data as *mut WriteManyData;
    let mut success: gboolean = 0;
    success = safe_c2rust_set_to_keyfile((*data).kfsb, key as *const gchar, value as *mut GVariant);
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if success != 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gkeyfilesettingsbackend.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            358 as ::core::ffi::c_int,
            G_STRFUNC,
            b"success\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_check_one(
    mut key: gpointer,
    mut value: gpointer,
    mut user_data: gpointer,
) -> gboolean {
    let mut data: *mut WriteManyData = user_data as *mut WriteManyData;
    (*data).failed = (g_hash_table_contains((*(*data).kfsb).system_locks, key as gconstpointer)
        != 0
        || safe_c2rust_path_is_valid((*data).kfsb, key as *const gchar) == 0)
        as ::core::ffi::c_int as gboolean;
    return (*data).failed;
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_write_tree(
    mut backend: *mut GSettingsBackend,
    mut tree: *mut GTree,
    mut origin_tag: gpointer,
) -> gboolean {
    let mut data: WriteManyData = WriteManyData {
        kfsb: backend as *mut ::core::ffi::c_void as *mut GKeyfileSettingsBackend,
        failed: 0 as gboolean,
    };
    let mut success: gboolean = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if (*data.kfsb).writable == 0 {
        return FALSE;
    }
    g_tree_foreach(
        tree,
        Some(
            safe_c2rust_g_keyfile_settings_backend_check_one
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
        ),
        &raw mut data as gpointer,
    );
    if data.failed != 0 {
        return FALSE;
    }
    g_tree_foreach(
        tree,
        Some(
            safe_c2rust_g_keyfile_settings_backend_write_one
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
        ),
        &raw mut data as gpointer,
    );
    success = safe_c2rust_g_keyfile_settings_backend_keyfile_write(data.kfsb, &raw mut error);
    if !error.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Failed to write keyfile to %s: %s\0" as *const u8 as *const gchar,
            g_file_peek_path((*data.kfsb).file),
            (*error).message,
        );
        g_error_free(error);
    }
    g_settings_backend_changed_tree(backend, tree, origin_tag);
    return success;
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_write(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut value: *mut GVariant,
    mut origin_tag: gpointer,
) -> gboolean {
    let mut kfsb: *mut GKeyfileSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GKeyfileSettingsBackend;
    let mut success: gboolean = 0;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if (*kfsb).writable == 0 {
        return FALSE;
    }
    success = safe_c2rust_set_to_keyfile(kfsb, key, value);
    if success != 0 {
        g_settings_backend_changed(backend, key, origin_tag);
        success = safe_c2rust_g_keyfile_settings_backend_keyfile_write(kfsb, &raw mut error);
        if !error.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Failed to write keyfile to %s: %s\0" as *const u8 as *const gchar,
                g_file_peek_path((*kfsb).file),
                (*error).message,
            );
            g_error_free(error);
        }
    }
    return success;
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_reset(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut origin_tag: gpointer,
) {
    let mut kfsb: *mut GKeyfileSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GKeyfileSettingsBackend;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_set_to_keyfile(kfsb, key, ::core::ptr::null_mut::<GVariant>()) != 0 {
        safe_c2rust_g_keyfile_settings_backend_keyfile_write(kfsb, &raw mut error);
        if !error.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Failed to write keyfile to %s: %s\0" as *const u8 as *const gchar,
                g_file_peek_path((*kfsb).file),
                (*error).message,
            );
            g_error_free(error);
        }
    }
    g_settings_backend_changed(backend, key, origin_tag);
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_get_writable(
    mut backend: *mut GSettingsBackend,
    mut name: *const gchar,
) -> gboolean {
    let mut kfsb: *mut GKeyfileSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GKeyfileSettingsBackend;
    return ((*kfsb).writable != 0
        && g_hash_table_contains((*kfsb).system_locks, name as gconstpointer) == 0
        && safe_c2rust_path_is_valid(kfsb, name) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_get_permission(
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
) -> *mut GPermission {
    let mut kfsb: *mut GKeyfileSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GKeyfileSettingsBackend;
    return g_object_ref((*kfsb).permission as gpointer) as *mut GPermission;
}
unsafe extern "C" fn safe_c2rust_keyfile_to_tree(
    mut kfsb: *mut GKeyfileSettingsBackend,
    mut tree: *mut GTree,
    mut keyfile: *mut GKeyFile,
    mut dup_check: gboolean,
) {
    let mut groups: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gint = 0;
    groups = g_key_file_get_groups(keyfile, ::core::ptr::null_mut::<gsize>());
    i = 0 as ::core::ffi::c_int as gint;
    while !(*groups.offset(i as isize)).is_null() {
        let mut is_root_group: gboolean = 0;
        let mut keys: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        let mut j: gint = 0;
        is_root_group = (g_strcmp0((*kfsb).root_group, *groups.offset(i as isize))
            == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
        if !(is_root_group == 0
            && ((if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = *groups.offset(i as isize);
                    let __prefix: *const ::core::ffi::c_char =
                        b"/\0" as *const u8 as *const ::core::ffi::c_char;
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
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                    *groups.offset(i as isize),
                    b"/\0" as *const u8 as *const gchar,
                )
            }) != 0
                || (if 0 != 0 {
                    ({
                        let __str: *const ::core::ffi::c_char = *groups.offset(i as isize);
                        let __suffix: *const ::core::ffi::c_char =
                            b"/\0" as *const u8 as *const ::core::ffi::c_char;
                        let mut __result: gboolean = FALSE;
                        if ({
                            let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                            if __str.is_null() || __suffix.is_null() {
                                _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_14
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            __result =
                                g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
                        } else {
                            let __str_len: size_t = strlen(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize),
                            ) as size_t;
                            let __suffix_len: size_t = strlen(
                                __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize),
                            ) as size_t;
                            if __str_len >= __suffix_len {
                                __result = (memcmp(
                                    __str
                                        .offset(__str_len as isize)
                                        .offset(-(__suffix_len as isize))
                                        as *const ::core::ffi::c_void,
                                    __suffix
                                        .offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                                        as *const ::core::ffi::c_void,
                                    __suffix_len,
                                ) == 0 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int
                                    as gboolean;
                            }
                        }
                        __result
                    })
                } else {
                    g_str_has_suffix(
                        *groups.offset(i as isize),
                        b"/\0" as *const u8 as *const gchar,
                    )
                }) != 0
                || !strstr(
                    *groups.offset(i as isize),
                    b"//\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null()))
        {
            keys = g_key_file_get_keys(
                keyfile,
                *groups.offset(i as isize),
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            if ({
                let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                if !keys.is_null() {
                    _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_15
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gkeyfilesettingsbackend.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    499 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"keys != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            j = 0 as ::core::ffi::c_int as gint;
            while !(*keys.offset(j as isize)).is_null() {
                let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
                let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
                if strchr(*keys.offset(j as isize), '/' as i32).is_null() {
                    if is_root_group != 0 {
                        path = g_strdup_printf(
                            b"%s%s\0" as *const u8 as *const gchar,
                            (*kfsb).prefix,
                            *keys.offset(j as isize),
                        );
                    } else {
                        path = g_strdup_printf(
                            b"%s%s/%s\0" as *const u8 as *const gchar,
                            (*kfsb).prefix,
                            *groups.offset(i as isize),
                            *keys.offset(j as isize),
                        );
                    }
                    value = g_key_file_get_value(
                        keyfile,
                        *groups.offset(i as isize),
                        *keys.offset(j as isize),
                        ::core::ptr::null_mut::<*mut GError>(),
                    );
                    if dup_check != 0
                        && g_strcmp0(
                            g_tree_lookup(tree, path as gconstpointer)
                                as *const ::core::ffi::c_char,
                            value,
                        ) == 0 as ::core::ffi::c_int
                    {
                        g_tree_remove(tree, path as gconstpointer);
                        g_free(value as gpointer);
                        g_free(path as gpointer);
                    } else {
                        g_tree_insert(tree, path as gpointer, value as gpointer);
                    }
                }
                j += 1;
            }
            g_strfreev(keys);
        }
        i += 1;
    }
    g_strfreev(groups);
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_keyfile_reload(
    mut kfsb: *mut GKeyfileSettingsBackend,
) {
    let mut digest: [guint8; 32] = [0; 32];
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut length: gsize = 0;
    contents = ::core::ptr::null_mut::<gchar>();
    length = 0 as gsize;
    g_file_load_contents(
        (*kfsb).file,
        ::core::ptr::null_mut::<GCancellable>(),
        &raw mut contents,
        &raw mut length,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    safe_c2rust_compute_checksum(
        &raw mut digest as *mut guint8,
        contents as gconstpointer,
        length,
    );
    if memcmp(
        &raw mut (*kfsb).digest as *mut guint8 as *const ::core::ffi::c_void,
        &raw mut digest as *mut guint8 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[guint8; 32]>() as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        let mut keyfiles: [*mut GKeyFile; 2] = [::core::ptr::null_mut::<GKeyFile>(); 2];
        let mut tree: *mut GTree = ::core::ptr::null_mut::<GTree>();
        tree = g_tree_new_full(
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                GCompareDataFunc,
            >(Some(
                strcmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            )),
            NULL_0,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        keyfiles[0 as ::core::ffi::c_int as usize] = (*kfsb).keyfile;
        keyfiles[1 as ::core::ffi::c_int as usize] = g_key_file_new();
        if length > 0 as gsize {
            g_key_file_load_from_data(
                keyfiles[1 as ::core::ffi::c_int as usize],
                contents,
                length,
                (G_KEY_FILE_KEEP_COMMENTS as ::core::ffi::c_int
                    | G_KEY_FILE_KEEP_TRANSLATIONS as ::core::ffi::c_int)
                    as GKeyFileFlags,
                ::core::ptr::null_mut::<*mut GError>(),
            );
        }
        safe_c2rust_keyfile_to_tree(
            kfsb,
            tree,
            keyfiles[0 as ::core::ffi::c_int as usize],
            FALSE,
        );
        safe_c2rust_keyfile_to_tree(kfsb, tree, keyfiles[1 as ::core::ffi::c_int as usize], TRUE);
        g_key_file_free(keyfiles[0 as ::core::ffi::c_int as usize]);
        (*kfsb).keyfile = keyfiles[1 as ::core::ffi::c_int as usize];
        if g_tree_nnodes(tree) > 0 as ::core::ffi::c_int {
            g_settings_backend_changed_tree(&raw mut (*kfsb).parent_instance, tree, NULL_0);
        }
        g_tree_unref(tree);
        memcpy(
            &raw mut (*kfsb).digest as *mut guint8 as *mut ::core::ffi::c_void,
            &raw mut digest as *mut guint8 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[guint8; 32]>() as size_t,
        );
    }
    g_free(contents as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_keyfile_writable(
    mut kfsb: *mut GKeyfileSettingsBackend,
) {
    let mut fileinfo: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut writable: gboolean = 0;
    fileinfo = g_file_query_info(
        (*kfsb).dir,
        b"access::*\0" as *const u8 as *const ::core::ffi::c_char,
        G_FILE_QUERY_INFO_NONE,
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !fileinfo.is_null() {
        writable = (g_file_info_get_attribute_boolean(
            fileinfo,
            G_FILE_ATTRIBUTE_ACCESS_CAN_WRITE.as_ptr(),
        ) != 0
            && g_file_info_get_attribute_boolean(
                fileinfo,
                G_FILE_ATTRIBUTE_ACCESS_CAN_EXECUTE.as_ptr(),
            ) != 0) as ::core::ffi::c_int as gboolean;
        g_object_unref(fileinfo as gpointer);
    } else {
        writable = FALSE as gboolean;
    }
    if writable != (*kfsb).writable {
        (*kfsb).writable = writable;
        g_settings_backend_path_writable_changed(
            &raw mut (*kfsb).parent_instance,
            b"/\0" as *const u8 as *const gchar,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_finalize(mut object: *mut GObject) {
    let mut kfsb: *mut GKeyfileSettingsBackend =
        object as *mut ::core::ffi::c_void as *mut GKeyfileSettingsBackend;
    if (*kfsb).poll_source_id != 0 as guint {
        g_source_remove((*kfsb).poll_source_id);
        (*kfsb).poll_source_id = 0 as guint;
    }
    g_key_file_free((*kfsb).keyfile);
    g_object_unref((*kfsb).permission as gpointer);
    g_key_file_unref((*kfsb).system_keyfile);
    g_hash_table_unref((*kfsb).system_locks);
    g_free((*kfsb).defaults_dir as gpointer);
    if !(*kfsb).file_monitor.is_null() {
        g_file_monitor_cancel((*kfsb).file_monitor);
        g_object_unref((*kfsb).file_monitor as gpointer);
    }
    g_object_unref((*kfsb).file as gpointer);
    if !(*kfsb).dir_monitor.is_null() {
        g_file_monitor_cancel((*kfsb).dir_monitor);
        g_object_unref((*kfsb).dir_monitor as gpointer);
    }
    g_object_unref((*kfsb).dir as gpointer);
    g_free((*kfsb).root_group as gpointer);
    g_free((*kfsb).prefix as gpointer);
    (*(safe_c2rust_g_keyfile_settings_backend_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_init(
    mut kfsb: *mut GKeyfileSettingsBackend,
) {
}
unsafe extern "C" fn safe_c2rust_file_changed(
    mut monitor: *mut GFileMonitor,
    mut file: *mut GFile,
    mut other_file: *mut GFile,
    mut event_type: GFileMonitorEvent,
    mut user_data: gpointer,
) {
    let mut kfsb: *mut GKeyfileSettingsBackend = user_data as *mut GKeyfileSettingsBackend;
    if event_type as ::core::ffi::c_uint
        != G_FILE_MONITOR_EVENT_DELETED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        safe_c2rust_g_keyfile_settings_backend_keyfile_reload(kfsb);
    }
}
unsafe extern "C" fn safe_c2rust_dir_changed(
    mut monitor: *mut GFileMonitor,
    mut file: *mut GFile,
    mut other_file: *mut GFile,
    mut event_type: GFileMonitorEvent,
    mut user_data: gpointer,
) {
    let mut kfsb: *mut GKeyfileSettingsBackend = user_data as *mut GKeyfileSettingsBackend;
    safe_c2rust_g_keyfile_settings_backend_keyfile_writable(kfsb);
}
unsafe extern "C" fn safe_c2rust_keyfile_poll_changed(mut user_data: gpointer) -> gboolean {
    let mut kfsb: *mut GKeyfileSettingsBackend = user_data as *mut GKeyfileSettingsBackend;
    safe_c2rust_g_keyfile_settings_backend_keyfile_writable(kfsb);
    safe_c2rust_g_keyfile_settings_backend_keyfile_reload(kfsb);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_load_system_settings(mut kfsb: *mut GKeyfileSettingsBackend) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut dir: *const ::core::ffi::c_char =
        b"/etc/glib-2.0/settings\0" as *const u8 as *const ::core::ffi::c_char;
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut contents: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*kfsb).system_keyfile = g_key_file_new();
    (*kfsb).system_locks = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
    if !(*kfsb).defaults_dir.is_null() {
        dir = (*kfsb).defaults_dir;
    }
    path = g_build_filename(
        dir as *const gchar,
        b"defaults\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    if g_key_file_load_from_file(
        (*kfsb).system_keyfile,
        path,
        G_KEY_FILE_NONE,
        &raw mut error,
    ) == 0
    {
        if g_error_matches(
            error,
            g_file_error_quark(),
            G_FILE_ERROR_NOENT as ::core::ffi::c_int as gint,
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Failed to read %s: %s\0" as *const u8 as *const gchar,
                path,
                (*error).message,
            );
        }
        g_clear_error(&raw mut error);
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"Loading default settings from %s\0" as *const u8 as *const gchar,
            path,
        );
    }
    g_free(path as gpointer);
    path = g_build_filename(
        dir as *const gchar,
        b"locks\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    if g_file_get_contents(
        path,
        &raw mut contents,
        ::core::ptr::null_mut::<gsize>(),
        &raw mut error,
    ) == 0
    {
        if g_error_matches(
            error,
            g_file_error_quark(),
            G_FILE_ERROR_NOENT as ::core::ffi::c_int as gint,
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Failed to read %s: %s\0" as *const u8 as *const gchar,
                path,
                (*error).message,
            );
        }
        g_clear_error(&raw mut error);
    } else {
        let mut lines: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut i: gsize = 0;
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"Loading locks from %s\0" as *const u8 as *const gchar,
            path,
        );
        lines = g_strsplit(contents, b"\n\0" as *const u8 as *const gchar, 0 as gint)
            as *mut *mut ::core::ffi::c_char;
        i = 0 as gsize;
        while !(*lines.offset(i as isize)).is_null() {
            let mut line: *mut ::core::ffi::c_char = *lines.offset(i as isize);
            if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
                || *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
            {
                g_free(line as gpointer);
            } else {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_DEBUG,
                    b"Locking key %s\0" as *const u8 as *const gchar,
                    line,
                );
                g_hash_table_add(
                    (*kfsb).system_locks,
                    safe_c2rust_g_steal_pointer(&raw mut line as gpointer)
                        as *mut ::core::ffi::c_char as gpointer,
                );
            }
            i = i.wrapping_add(1);
        }
        g_free(lines as gpointer);
    }
    g_free(contents as gpointer);
    g_free(path as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_constructed(mut object: *mut GObject) {
    let mut kfsb: *mut GKeyfileSettingsBackend =
        object as *mut ::core::ffi::c_void as *mut GKeyfileSettingsBackend;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if (*kfsb).file.is_null() {
        let mut filename: *mut ::core::ffi::c_char = g_build_filename(
            g_get_user_config_dir(),
            b"glib-2.0\0" as *const u8 as *const ::core::ffi::c_char,
            b"settings\0" as *const u8 as *const ::core::ffi::c_char,
            b"keyfile\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
        (*kfsb).file = g_file_new_for_path(filename);
        g_free(filename as gpointer);
    }
    if (*kfsb).prefix.is_null() {
        (*kfsb).prefix =
            safe_c2rust_g_strdup_inline(b"/\0" as *const u8 as *const ::core::ffi::c_char)
                as *mut gchar;
        (*kfsb).prefix_len = 1 as gsize;
    }
    (*kfsb).keyfile = g_key_file_new();
    (*kfsb).permission = g_simple_permission_new(TRUE);
    (*kfsb).dir = g_file_get_parent((*kfsb).file);
    path = g_file_peek_path((*kfsb).dir);
    if g_mkdir_with_parents(path as *const gchar, 0o700 as gint) == -(1 as ::core::ffi::c_int) {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Failed to create %s: %s\0" as *const u8 as *const gchar,
            path,
            g_strerror(*__errno_location()),
        );
    }
    (*kfsb).file_monitor = g_file_monitor(
        (*kfsb).file,
        G_FILE_MONITOR_NONE,
        ::core::ptr::null_mut::<GCancellable>(),
        &raw mut error,
    );
    if (*kfsb).file_monitor.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Failed to create file monitor for %s: %s\0" as *const u8 as *const gchar,
            g_file_peek_path((*kfsb).file),
            (*error).message,
        );
        g_clear_error(&raw mut error);
    } else {
        g_signal_connect_data(
            (*kfsb).file_monitor as gpointer,
            b"changed\0" as *const u8 as *const gchar,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        gpointer,
                    ) -> (),
                >,
                GCallback,
            >(Some(
                safe_c2rust_file_changed
                    as unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        gpointer,
                    ) -> (),
            )),
            kfsb as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
    }
    (*kfsb).dir_monitor = g_file_monitor(
        (*kfsb).dir,
        G_FILE_MONITOR_NONE,
        ::core::ptr::null_mut::<GCancellable>(),
        &raw mut error,
    );
    if (*kfsb).dir_monitor.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Failed to create file monitor for %s: %s\0" as *const u8 as *const gchar,
            g_file_peek_path((*kfsb).file),
            (*error).message,
        );
        g_clear_error(&raw mut error);
    } else {
        g_signal_connect_data(
            (*kfsb).dir_monitor as gpointer,
            b"changed\0" as *const u8 as *const gchar,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        gpointer,
                    ) -> (),
                >,
                GCallback,
            >(Some(
                safe_c2rust_dir_changed
                    as unsafe extern "C" fn(
                        *mut GFileMonitor,
                        *mut GFile,
                        *mut GFile,
                        GFileMonitorEvent,
                        gpointer,
                    ) -> (),
            )),
            kfsb as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
    }
    safe_c2rust_compute_checksum(
        &raw mut (*kfsb).digest as *mut guint8,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as gsize,
    );
    safe_c2rust_g_keyfile_settings_backend_keyfile_writable(kfsb);
    safe_c2rust_g_keyfile_settings_backend_keyfile_reload(kfsb);
    (*kfsb).poll_source_id = g_timeout_add(
        250 as guint,
        Some(safe_c2rust_keyfile_poll_changed as unsafe extern "C" fn(gpointer) -> gboolean),
        kfsb as gpointer,
    );
    safe_c2rust_load_system_settings(kfsb);
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut kfsb: *mut GKeyfileSettingsBackend =
        object as *mut ::core::ffi::c_void as *mut GKeyfileSettingsBackend;
    match prop_id as GKeyfileSettingsBackendProperty as ::core::ffi::c_uint {
        1 => {
            if ({
                let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                if (*kfsb).file.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gkeyfilesettingsbackend.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    806 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"kfsb->file == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !g_value_get_string(value).is_null() {
                (*kfsb).file =
                    g_file_new_for_path(g_value_get_string(value) as *const ::core::ffi::c_char);
            }
        }
        2 => {
            if ({
                let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                if (*kfsb).prefix.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gkeyfilesettingsbackend.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    813 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"kfsb->prefix == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*kfsb).prefix = g_value_dup_string(value);
            if !(*kfsb).prefix.is_null() {
                (*kfsb).prefix_len = strlen((*kfsb).prefix) as gsize;
            }
        }
        3 => {
            if ({
                let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                if (*kfsb).root_group.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gkeyfilesettingsbackend.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    821 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"kfsb->root_group == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*kfsb).root_group = g_value_dup_string(value);
            if !(*kfsb).root_group.is_null() {
                (*kfsb).root_group_len = strlen((*kfsb).root_group) as gsize;
            }
        }
        4 => {
            if ({
                let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                if (*kfsb).defaults_dir.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gkeyfilesettingsbackend.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    829 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"kfsb->defaults_dir == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*kfsb).defaults_dir = g_value_dup_string(value) as *mut ::core::ffi::c_char;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gkeyfilesettingsbackend.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                834 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut kfsb: *mut GKeyfileSettingsBackend =
        object as *mut ::core::ffi::c_void as *mut GKeyfileSettingsBackend;
    match prop_id as GKeyfileSettingsBackendProperty as ::core::ffi::c_uint {
        1 => {
            g_value_set_string(value, g_file_peek_path((*kfsb).file) as *const gchar);
        }
        2 => {
            g_value_set_string(value, (*kfsb).prefix);
        }
        3 => {
            g_value_set_string(value, (*kfsb).root_group);
        }
        4 => {
            g_value_set_string(value, (*kfsb).defaults_dir);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gkeyfilesettingsbackend.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                866 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_class_init(
    mut class: *mut GKeyfileSettingsBackendClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize = Some(
        safe_c2rust_g_keyfile_settings_backend_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).constructed = Some(
        safe_c2rust_g_keyfile_settings_backend_constructed
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).get_property = Some(
        safe_c2rust_g_keyfile_settings_backend_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_keyfile_settings_backend_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*class).read = Some(
        safe_c2rust_g_keyfile_settings_backend_read
            as unsafe extern "C" fn(
                *mut GSettingsBackend,
                *const gchar,
                *const GVariantType,
                gboolean,
            ) -> *mut GVariant,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSettingsBackend,
                *const gchar,
                *const GVariantType,
                gboolean,
            ) -> *mut GVariant,
        >;
    (*class).write = Some(
        safe_c2rust_g_keyfile_settings_backend_write
            as unsafe extern "C" fn(
                *mut GSettingsBackend,
                *const gchar,
                *mut GVariant,
                gpointer,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSettingsBackend,
                *const gchar,
                *mut GVariant,
                gpointer,
            ) -> gboolean,
        >;
    (*class).write_tree = Some(
        safe_c2rust_g_keyfile_settings_backend_write_tree
            as unsafe extern "C" fn(*mut GSettingsBackend, *mut GTree, gpointer) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *mut GTree, gpointer) -> gboolean>;
    (*class).reset = Some(
        safe_c2rust_g_keyfile_settings_backend_reset
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar, gpointer) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar, gpointer) -> ()>;
    (*class).get_writable = Some(
        safe_c2rust_g_keyfile_settings_backend_get_writable
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> gboolean>;
    (*class).get_permission = Some(
        safe_c2rust_g_keyfile_settings_backend_get_permission
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> *mut GPermission,
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> *mut GPermission>;
    g_object_class_install_property(
        object_class,
        PROP_FILENAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"filename\0" as *const u8 as *const gchar,
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
    g_object_class_install_property(
        object_class,
        PROP_ROOT_PATH as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"root-path\0" as *const u8 as *const gchar,
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
    g_object_class_install_property(
        object_class,
        PROP_ROOT_GROUP as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"root-group\0" as *const u8 as *const gchar,
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
    g_object_class_install_property(
        object_class,
        PROP_DEFAULTS_DIR as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"defaults-dir\0" as *const u8 as *const gchar,
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_keyfile_settings_backend_new(
    mut filename: *const gchar,
    mut root_path: *const gchar,
    mut root_group: *const gchar,
) -> *mut GSettingsBackend {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !filename.is_null() {
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
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettingsBackend>();
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !root_path.is_null() {
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
            b"root_path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettingsBackend>();
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = root_path as *const ::core::ffi::c_char;
                let __prefix: *const ::core::ffi::c_char =
                    b"/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = 0 as gboolean;
                if ({
                    let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_22
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
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
            g_str_has_prefix(root_path, b"/\0" as *const u8 as *const gchar)
        } != 0
        {
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
            b"g_str_has_prefix (root_path, \"/\")\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettingsBackend>();
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = root_path as *const ::core::ffi::c_char;
                let __suffix: *const ::core::ffi::c_char =
                    b"/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = 0 as gboolean;
                if ({
                    let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                    if __str.is_null() || __suffix.is_null() {
                        _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_24
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __suffix_len: size_t =
                        strlen(__suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __suffix_len {
                        __result = (memcmp(
                            __str
                                .offset(__str_len as isize)
                                .offset(-(__suffix_len as isize))
                                as *const ::core::ffi::c_void,
                            __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __suffix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_suffix(root_path, b"/\0" as *const u8 as *const gchar)
        } != 0
        {
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
            b"g_str_has_suffix (root_path, \"/\")\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettingsBackend>();
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if strstr(
            root_path as *const ::core::ffi::c_char,
            b"//\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"strstr (root_path, \"//\") == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettingsBackend>();
    }
    return g_object_new(
        safe_c2rust_g_keyfile_settings_backend_get_type(),
        b"filename\0" as *const u8 as *const gchar,
        filename,
        b"root-path\0" as *const u8 as *const ::core::ffi::c_char,
        root_path,
        b"root-group\0" as *const u8 as *const ::core::ffi::c_char,
        root_group,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GSettingsBackend;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
