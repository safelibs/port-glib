extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GCancellablePrivate;
    pub type _GModule;
    pub type _GWakeup;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
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
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_clear_error(err: *mut *mut GError);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_rec_mutex_lock(rec_mutex: *mut GRecMutex);
    fn g_rec_mutex_unlock(rec_mutex: *mut GRecMutex);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_dir_open(path: *const gchar, flags: guint, error: *mut *mut GError) -> *mut GDir;
    fn g_dir_read_name(dir: *mut GDir) -> *const gchar;
    fn g_dir_close(dir: *mut GDir);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_insert_sorted(list: *mut GList, data: gpointer, func: GCompareFunc) -> *mut GList;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup_extended(
        hash_table: *mut GHashTable,
        lookup_key: gconstpointer,
        orig_key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
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
    fn g_print(format: *const gchar, ...);
    fn g_printerr(format: *const gchar, ...);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
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
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
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
    fn g_type_ensure(type_0: GType);
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_weak_ref_init(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_weak_ref_clear(weak_ref: *mut GWeakRef);
    fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> gpointer;
    fn g_weak_ref_set(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_type_module_get_type() -> GType;
    fn g_type_module_use(module: *mut GTypeModule) -> gboolean;
    fn g_type_module_unuse(module: *mut GTypeModule);
    fn g_module_supported() -> gboolean;
    fn g_module_open_full(
        file_name: *const gchar,
        flags: GModuleFlags,
        error: *mut *mut GError,
    ) -> *mut GModule;
    fn g_module_close(module: *mut GModule) -> gboolean;
    fn g_module_error() -> *const gchar;
    fn g_module_symbol(
        module: *mut GModule,
        symbol_name: *const gchar,
        symbol: *mut gpointer,
    ) -> gboolean;
    fn _g_io_module_extract_name(filename: *const ::core::ffi::c_char) -> *mut gchar;
    fn __lsan_ignore_object(p: *const ::core::ffi::c_void);
    fn glib__private__() -> *const GLibPrivateVTable;
    fn g_initable_get_type() -> GType;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_debug_controller_get_type() -> GType;
    fn g_debug_controller_dbus_get_type() -> GType;
    fn g_memory_monitor_get_type() -> GType;
    fn g_volume_monitor_get_type() -> GType;
    fn g_native_volume_monitor_get_type() -> GType;
    fn g_network_monitor_get_type() -> GType;
    fn g_power_profile_monitor_get_type() -> GType;
    fn g_proxy_get_type() -> GType;
    fn g_proxy_resolver_get_type() -> GType;
    fn g_tls_backend_get_type() -> GType;
    fn g_vfs_get_type() -> GType;
    fn g_local_file_monitor_get_type() -> GType;
    fn g_null_settings_backend_get_type() -> GType;
    fn g_memory_settings_backend_get_type() -> GType;
    fn g_keyfile_settings_backend_get_type() -> GType;
    fn _g_http_proxy_get_type() -> GType;
    fn _g_https_proxy_get_type() -> GType;
    fn _g_socks4_proxy_get_type() -> GType;
    fn _g_socks4a_proxy_get_type() -> GType;
    fn _g_socks5_proxy_get_type() -> GType;
    fn g_notification_backend_get_type() -> GType;
    fn g_memory_monitor_portal_get_type() -> GType;
    fn g_memory_monitor_dbus_get_type() -> GType;
    fn g_power_profile_monitor_dbus_get_type() -> GType;
    fn g_power_profile_monitor_portal_get_type() -> GType;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn g_desktop_app_info_lookup_get_type() -> GType;
    fn g_inotify_file_monitor_get_type() -> GType;
    fn _g_unix_volume_monitor_get_type() -> GType;
    fn _g_local_vfs_get_type() -> GType;
    fn _g_dummy_proxy_resolver_get_type() -> GType;
    fn _g_dummy_tls_backend_get_type() -> GType;
    fn g_network_monitor_base_get_type() -> GType;
    fn _g_network_monitor_netlink_get_type() -> GType;
    fn _g_network_monitor_nm_get_type() -> GType;
    fn g_fdo_notification_backend_get_type() -> GType;
    fn g_gtk_notification_backend_get_type() -> GType;
    fn g_portal_notification_backend_get_type() -> GType;
    fn g_proxy_resolver_portal_get_type() -> GType;
    fn g_network_monitor_portal_get_type() -> GType;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gushort = ::core::ffi::c_ushort;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRecMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GRecMutex = _GRecMutex;
pub type GData = _GData;
pub type GDir = _GDir;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GWeakRef {
    pub priv_0: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub p: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeModule {
    pub parent_instance: GObject,
    pub use_count: guint,
    pub type_infos: *mut GSList,
    pub interface_infos: *mut GSList,
    pub name: *mut gchar,
}
pub type GTypeModule = _GTypeModule;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeModuleClass {
    pub parent_class: GObjectClass,
    pub load: Option<unsafe extern "C" fn(*mut GTypeModule) -> gboolean>,
    pub unload: Option<unsafe extern "C" fn(*mut GTypeModule) -> ()>,
    pub reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub reserved4: Option<unsafe extern "C" fn() -> ()>,
}
pub type GTypeModuleClass = _GTypeModuleClass;
pub type GIOModuleScopeFlags = ::core::ffi::c_uint;
pub const G_IO_MODULE_SCOPE_BLOCK_DUPLICATES: GIOModuleScopeFlags = 1;
pub const G_IO_MODULE_SCOPE_NONE: GIOModuleScopeFlags = 0;
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
pub struct _GIOModule {
    pub parent_instance: GTypeModule,
    pub filename: *mut gchar,
    pub library: *mut GModule,
    pub initialized: gboolean,
    pub load: Option<unsafe extern "C" fn(*mut GIOModule) -> ()>,
    pub unload: Option<unsafe extern "C" fn(*mut GIOModule) -> ()>,
}
pub type GIOModule = _GIOModule;
pub type GModule = _GModule;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOExtensionPoint {
    pub required_type: GType,
    pub name: *mut ::core::ffi::c_char,
    pub extensions: *mut GList,
    pub lazy_load_modules: *mut GList,
}
pub type GIOExtensionPoint = _GIOExtensionPoint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOExtension {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: GType,
    pub priority: gint,
}
pub type GIOExtension = _GIOExtension;
pub type GModuleFlags = ::core::ffi::c_uint;
pub const G_MODULE_BIND_MASK: GModuleFlags = 3;
pub const G_MODULE_BIND_LOCAL: GModuleFlags = 2;
pub const G_MODULE_BIND_LAZY: GModuleFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOModuleScope {
    pub flags: GIOModuleScopeFlags,
    pub basenames: *mut GHashTable,
}
pub type GIOModuleScope = _GIOModuleScope;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOModuleClass {
    pub parent_class: GTypeModuleClass,
}
pub type GIOModuleClass = _GIOModuleClass;
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
pub type GIOModuleVerifyFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type verify_func = Option<unsafe extern "C" fn() -> gboolean>;
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
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_OBJECT: GType = ((20 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[inline]
unsafe extern "C" fn safe_c2rust_g_ignore_leak(mut p: gconstpointer) {
    if !p.is_null()
        && Some(__lsan_ignore_object as unsafe extern "C" fn(*const ::core::ffi::c_void) -> ())
            .is_some()
    {
        __lsan_ignore_object(p as *const ::core::ffi::c_void);
    }
}
pub const G_DEBUG_CONTROLLER_EXTENSION_POINT_NAME: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"gio-debug-controller\0")
};
pub const G_MEMORY_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"gio-memory-monitor\0")
};
pub const G_VOLUME_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"gio-volume-monitor\0")
};
pub const G_NATIVE_VOLUME_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(*b"gio-native-volume-monitor\0")
};
pub const G_NETWORK_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"gio-network-monitor\0")
};
pub const G_POWER_PROFILE_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(*b"gio-power-profile-monitor\0")
};
pub const G_PROXY_EXTENSION_POINT_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"gio-proxy\0") };
pub const G_PROXY_RESOLVER_EXTENSION_POINT_NAME: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"gio-proxy-resolver\0")
};
pub const G_TLS_BACKEND_EXTENSION_POINT_NAME: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"gio-tls-backend\0") };
pub const G_VFS_EXTENSION_POINT_NAME: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"gio-vfs\0") };
pub const G_LOCAL_FILE_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"gio-local-file-monitor\0")
};
pub const G_NFS_FILE_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"gio-nfs-file-monitor\0")
};
pub const G_NOTIFICATION_BACKEND_EXTENSION_POINT_NAME: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"gnotification-backend\0")
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_module_scope_new(
    mut flags: GIOModuleScopeFlags,
) -> *mut GIOModuleScope {
    let mut scope: *mut GIOModuleScope = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GIOModuleScope>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GIOModuleScope;
    (*scope).flags = flags;
    (*scope).basenames = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
    return scope;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_module_scope_free(mut scope: *mut GIOModuleScope) {
    if scope.is_null() {
        return;
    }
    g_hash_table_destroy((*scope).basenames);
    g_free(scope as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_module_scope_block(
    mut scope: *mut GIOModuleScope,
    mut basename: *const gchar,
) {
    let mut key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !scope.is_null() {
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
            b"scope != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !basename.is_null() {
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
            b"basename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    key = safe_c2rust_g_strdup_inline(basename as *const ::core::ffi::c_char) as *mut gchar;
    g_hash_table_add((*scope).basenames, key as gpointer);
}
unsafe extern "C" fn safe_c2rust__g_io_module_scope_contains(
    mut scope: *mut GIOModuleScope,
    mut basename: *const gchar,
) -> gboolean {
    return g_hash_table_contains((*scope).basenames, basename as gconstpointer);
}
static mut safe_c2rust_extension_points: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_g__extension_points_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_module_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_io_module_get_type_once();
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
static mut safe_c2rust_GIOModule_private_offset: gint = 0;
static mut safe_c2rust_g_io_module_parent_class: gpointer = NULL_1;
unsafe extern "C" fn safe_c2rust_g_io_module_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_io_module_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GIOModule_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GIOModule_private_offset);
    }
    safe_c2rust_g_io_module_class_init(klass as *mut GIOModuleClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_io_module_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_type_module_get_type(),
        g_intern_static_string(b"GIOModule\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GIOModuleClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_io_module_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GIOModule>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GIOModule) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_io_module_init as unsafe extern "C" fn(*mut GIOModule) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_io_module_class_init(mut class: *mut GIOModuleClass) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut type_module_class: *mut GTypeModuleClass =
        class as *mut ::core::ffi::c_void as *mut GTypeModuleClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_io_module_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*type_module_class).load = Some(
        safe_c2rust_g_io_module_load_module as unsafe extern "C" fn(*mut GTypeModule) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GTypeModule) -> gboolean>;
    (*type_module_class).unload =
        Some(safe_c2rust_g_io_module_unload_module as unsafe extern "C" fn(*mut GTypeModule) -> ())
            as Option<unsafe extern "C" fn(*mut GTypeModule) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_io_module_init(mut module: *mut GIOModule) {}
unsafe extern "C" fn safe_c2rust_g_io_module_finalize(mut object: *mut GObject) {
    let mut module: *mut GIOModule = object as *mut ::core::ffi::c_void as *mut GIOModule;
    g_free((*module).filename as gpointer);
    (*(safe_c2rust_g_io_module_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_load_symbols(mut module: *mut GIOModule) -> gboolean {
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut load_symname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut unload_symname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ret: gboolean = 0;
    name = _g_io_module_extract_name((*module).filename);
    load_symname = g_strconcat(
        b"g_io_\0" as *const u8 as *const gchar,
        name,
        b"_load\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_1,
    );
    unload_symname = g_strconcat(
        b"g_io_\0" as *const u8 as *const gchar,
        name,
        b"_unload\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_1,
    );
    ret = (g_module_symbol(
        (*module).library,
        load_symname,
        &raw mut (*module).load as gpointer as *mut gpointer,
    ) != 0
        && g_module_symbol(
            (*module).library,
            unload_symname,
            &raw mut (*module).unload as gpointer as *mut gpointer,
        ) != 0) as ::core::ffi::c_int as gboolean;
    if ret == 0 {
        ret = (g_module_symbol(
            (*module).library,
            b"g_io_module_load\0" as *const u8 as *const gchar,
            &raw mut (*module).load as gpointer as *mut gpointer,
        ) != 0
            && g_module_symbol(
                (*module).library,
                b"g_io_module_unload\0" as *const u8 as *const gchar,
                &raw mut (*module).unload as gpointer as *mut gpointer,
            ) != 0) as ::core::ffi::c_int as gboolean;
    }
    g_free(name as gpointer);
    g_free(load_symname as gpointer);
    g_free(unload_symname as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_io_module_load_module(
    mut gmodule: *mut GTypeModule,
) -> gboolean {
    let mut module: *mut GIOModule = gmodule as *mut ::core::ffi::c_void as *mut GIOModule;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if (*module).filename.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"GIOModule path not set\0" as *const u8 as *const gchar,
        );
        return FALSE;
    }
    (*module).library = g_module_open_full(
        (*module).filename,
        (G_MODULE_BIND_LAZY as ::core::ffi::c_int | G_MODULE_BIND_LOCAL as ::core::ffi::c_int)
            as GModuleFlags,
        &raw mut error,
    );
    if (*module).library.is_null() {
        g_printerr(b"%s\n\0" as *const u8 as *const gchar, (*error).message);
        g_clear_error(&raw mut error);
        return FALSE;
    }
    if safe_c2rust_load_symbols(module) == 0 {
        g_printerr(b"%s\n\0" as *const u8 as *const gchar, g_module_error());
        g_module_close((*module).library);
        return FALSE;
    }
    (*module).load.expect("non-null function pointer")(module);
    (*module).initialized = TRUE as gboolean;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_io_module_unload_module(mut gmodule: *mut GTypeModule) {
    let mut module: *mut GIOModule = gmodule as *mut ::core::ffi::c_void as *mut GIOModule;
    (*module).unload.expect("non-null function pointer")(module);
    g_module_close((*module).library);
    (*module).library = ::core::ptr::null_mut::<GModule>();
    (*module).load = None;
    (*module).unload = None;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_module_new(mut filename: *const gchar) -> *mut GIOModule {
    let mut module: *mut GIOModule = ::core::ptr::null_mut::<GIOModule>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !filename.is_null() {
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
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOModule>();
    }
    module = g_object_new(
        safe_c2rust_g_io_module_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GIOModule;
    (*module).filename =
        safe_c2rust_g_strdup_inline(filename as *const ::core::ffi::c_char) as *mut gchar;
    return module;
}
unsafe extern "C" fn safe_c2rust_is_valid_module_name(
    mut basename: *const gchar,
    mut scope: *mut GIOModuleScope,
) -> gboolean {
    let mut result: gboolean = 0;
    if (if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = basename as *const ::core::ffi::c_char;
            let __prefix: *const ::core::ffi::c_char =
                b"lib\0" as *const u8 as *const ::core::ffi::c_char;
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
        g_str_has_prefix(basename, b"lib\0" as *const u8 as *const gchar)
    }) == 0
        || (if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = basename as *const ::core::ffi::c_char;
                let __suffix: *const ::core::ffi::c_char =
                    b".so\0" as *const u8 as *const ::core::ffi::c_char;
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
            g_str_has_suffix(basename, b".so\0" as *const u8 as *const gchar)
        }) == 0
    {
        return FALSE;
    }
    result = TRUE as gboolean;
    if !scope.is_null() {
        result = (if safe_c2rust__g_io_module_scope_contains(scope, basename) != 0 {
            FALSE
        } else {
            TRUE
        }) as gboolean;
        if result != 0
            && (*scope).flags as ::core::ffi::c_uint
                & G_IO_MODULE_SCOPE_BLOCK_DUPLICATES as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
        {
            safe_c2rust_g_io_module_scope_block(scope, basename);
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_modules_scan_all_in_directory_with_scope(
    mut dirname: *const ::core::ffi::c_char,
    mut scope: *mut GIOModuleScope,
) {
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dir: *mut GDir = ::core::ptr::null_mut::<GDir>();
    let mut statbuf: GStatBuf = stat {
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
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cache_time: time_t = 0;
    let mut cache: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    if g_module_supported() == 0 {
        return;
    }
    dir = g_dir_open(
        dirname as *const gchar,
        0 as guint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if dir.is_null() {
        return;
    }
    filename = g_build_filename(
        dirname as *const gchar,
        b"giomodule.cache\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_1,
    ) as *mut ::core::ffi::c_char;
    cache = ::core::ptr::null_mut::<GHashTable>();
    cache_time = 0 as time_t;
    if stat(filename, &raw mut statbuf) == 0 as ::core::ffi::c_int
        && g_file_get_contents(
            filename,
            &raw mut data,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) != 0
    {
        let mut lines: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut i: ::core::ffi::c_int = 0;
        cache_time = (if statbuf.st_mtim.tv_sec > statbuf.st_ctim.tv_sec {
            statbuf.st_mtim.tv_sec
        } else {
            statbuf.st_ctim.tv_sec
        }) as time_t;
        lines = g_strsplit(data, b"\n\0" as *const u8 as *const gchar, -(1 as gint))
            as *mut *mut ::core::ffi::c_char;
        g_free(data as gpointer);
        i = 0 as ::core::ffi::c_int;
        while !(*lines.offset(i as isize)).is_null() {
            let mut line: *mut ::core::ffi::c_char = *lines.offset(i as isize);
            let mut file: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut colon: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut strv_extension_points: *mut *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
            if !(*line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32)
            {
                colon = strchr(line, ':' as i32);
                if !(colon.is_null() || line == colon) {
                    *colon = 0 as ::core::ffi::c_char;
                    file = safe_c2rust_g_strdup_inline(line);
                    colon = colon.offset(1);
                    while *safe_c2rust_g_ascii_table.offset(*colon as guchar as isize)
                        as ::core::ffi::c_int
                        & G_ASCII_SPACE as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        colon = colon.offset(1);
                    }
                    if ({
                        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                        if cache.is_null() {
                            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_15
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        cache = g_hash_table_new_full(
                            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                            Some(
                                g_str_equal
                                    as unsafe extern "C" fn(
                                        gconstpointer,
                                        gconstpointer,
                                    )
                                        -> gboolean,
                            ),
                            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                            ::core::mem::transmute::<
                                Option<unsafe extern "C" fn(*mut *mut gchar) -> ()>,
                                GDestroyNotify,
                            >(Some(
                                g_strfreev as unsafe extern "C" fn(*mut *mut gchar) -> (),
                            )),
                        );
                    }
                    strv_extension_points =
                        g_strsplit(colon, b",\0" as *const u8 as *const gchar, -(1 as gint))
                            as *mut *mut ::core::ffi::c_char;
                    g_hash_table_insert(cache, file as gpointer, strv_extension_points as gpointer);
                }
            }
            i += 1;
        }
        g_strfreev(lines as *mut *mut gchar);
    }
    loop {
        name = g_dir_read_name(dir);
        if name.is_null() {
            break;
        }
        if safe_c2rust_is_valid_module_name(name, scope) != 0 {
            let mut extension_point: *mut GIOExtensionPoint =
                ::core::ptr::null_mut::<GIOExtensionPoint>();
            let mut module: *mut GIOModule = ::core::ptr::null_mut::<GIOModule>();
            let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut strv_extension_points_0: *mut *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
            let mut i_0: ::core::ffi::c_int = 0;
            path = g_build_filename(dirname as *const gchar, name, NULL_1);
            module = safe_c2rust_g_io_module_new(path);
            if !cache.is_null() {
                strv_extension_points_0 = g_hash_table_lookup(cache, name as gconstpointer)
                    as *mut *mut ::core::ffi::c_char;
            }
            if !strv_extension_points_0.is_null()
                && stat(path, &raw mut statbuf) == 0 as ::core::ffi::c_int
                && statbuf.st_ctim.tv_sec <= cache_time
            {
                i_0 = 0 as ::core::ffi::c_int;
                while !(*strv_extension_points_0.offset(i_0 as isize)).is_null() {
                    extension_point = safe_c2rust_g_io_extension_point_register(
                        *strv_extension_points_0.offset(i_0 as isize),
                    );
                    (*extension_point).lazy_load_modules =
                        g_list_prepend((*extension_point).lazy_load_modules, module as gpointer);
                    i_0 += 1;
                }
            } else if g_type_module_use(module as *mut ::core::ffi::c_void as *mut GTypeModule) != 0
            {
                g_type_module_unuse(module as *mut ::core::ffi::c_void as *mut GTypeModule);
                safe_c2rust_g_ignore_leak(module as gconstpointer);
            } else {
                g_printerr(
                    b"Failed to load module: %s\n\0" as *const u8 as *const gchar,
                    path,
                );
                g_object_unref(module as gpointer);
            }
            g_free(path as gpointer);
        }
    }
    g_dir_close(dir);
    if !cache.is_null() {
        g_hash_table_destroy(cache);
    }
    g_free(filename as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_modules_scan_all_in_directory(
    mut dirname: *const ::core::ffi::c_char,
) {
    safe_c2rust_g_io_modules_scan_all_in_directory_with_scope(
        dirname,
        ::core::ptr::null_mut::<GIOModuleScope>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_modules_load_all_in_directory_with_scope(
    mut dirname: *const ::core::ffi::c_char,
    mut scope: *mut GIOModuleScope,
) -> *mut GList {
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut dir: *mut GDir = ::core::ptr::null_mut::<GDir>();
    let mut modules: *mut GList = ::core::ptr::null_mut::<GList>();
    if g_module_supported() == 0 {
        return ::core::ptr::null_mut::<GList>();
    }
    dir = g_dir_open(
        dirname as *const gchar,
        0 as guint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if dir.is_null() {
        return ::core::ptr::null_mut::<GList>();
    }
    modules = ::core::ptr::null_mut::<GList>();
    loop {
        name = g_dir_read_name(dir);
        if name.is_null() {
            break;
        }
        if !(safe_c2rust_is_valid_module_name(name, scope) != 0) {
            continue;
        }
        let mut module: *mut GIOModule = ::core::ptr::null_mut::<GIOModule>();
        let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
        path = g_build_filename(dirname as *const gchar, name, NULL_1);
        module = safe_c2rust_g_io_module_new(path);
        if g_type_module_use(module as *mut ::core::ffi::c_void as *mut GTypeModule) == 0 {
            g_printerr(
                b"Failed to load module: %s\n\0" as *const u8 as *const gchar,
                path,
            );
            g_object_unref(module as gpointer);
            g_free(path as gpointer);
        } else {
            g_free(path as gpointer);
            modules = g_list_prepend(modules, module as gpointer);
        }
    }
    g_dir_close(dir);
    return modules;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_modules_load_all_in_directory(
    mut dirname: *const ::core::ffi::c_char,
) -> *mut GList {
    return safe_c2rust_g_io_modules_load_all_in_directory_with_scope(
        dirname,
        ::core::ptr::null_mut::<GIOModuleScope>(),
    );
}
unsafe extern "C" fn safe_c2rust_try_class(
    mut extension: *mut GIOExtension,
    mut is_supported_offset: guint,
) -> gpointer {
    let mut type_0: GType = safe_c2rust_g_io_extension_get_type(extension);
    let mut class: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    class = g_type_class_ref(type_0);
    if is_supported_offset == 0
        || Some(
            (*((class as *mut guint8).offset(is_supported_offset as glong as isize) as gpointer
                as *mut verify_func))
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")()
            != 0
    {
        return class;
    }
    g_type_class_unref(class);
    return NULL_1;
}
unsafe extern "C" fn safe_c2rust_print_help(
    mut envvar: *const ::core::ffi::c_char,
    mut ep: *mut GIOExtensionPoint,
) {
    g_print(
        b"Supported arguments for %s environment variable:\n\0" as *const u8 as *const gchar,
        envvar,
    );
    if safe_c2rust_g_io_extension_point_get_extensions(ep).is_null() {
        g_print(b" (none)\n\0" as *const u8 as *const gchar);
    } else {
        let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut extension: *mut GIOExtension = ::core::ptr::null_mut::<GIOExtension>();
        let mut width: gsize = 0 as gsize;
        l = safe_c2rust_g_io_extension_point_get_extensions(ep);
        while !l.is_null() {
            extension = (*l).data as *mut GIOExtension;
            width = (if width as size_t > strlen(safe_c2rust_g_io_extension_get_name(extension)) {
                width as size_t
            } else {
                strlen(safe_c2rust_g_io_extension_get_name(extension))
            }) as gsize;
            l = (*l).next;
        }
        l = safe_c2rust_g_io_extension_point_get_extensions(ep);
        while !l.is_null() {
            extension = (*l).data as *mut GIOExtension;
            g_print(
                b" %*s - %d\n\0" as *const u8 as *const gchar,
                (if width < 2147483647 as gsize {
                    width
                } else {
                    2147483647 as gsize
                }) as ::core::ffi::c_int,
                safe_c2rust_g_io_extension_get_name(extension),
                safe_c2rust_g_io_extension_get_priority(extension),
            );
            l = (*l).next;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_io_module_get_default_type(
    mut extension_point: *const gchar,
    mut envvar: *const gchar,
    mut is_supported_offset: guint,
) -> GType {
    let mut current_block: u64;
    static mut safe_c2rust_default_modules_lock: GRecMutex = _GRecMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        i: [0; 2],
    };
    static mut safe_c2rust_default_modules: *mut GHashTable =
        ::core::ptr::null::<GHashTable>() as *mut GHashTable;
    let mut use_this: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut ep: *mut GIOExtensionPoint = ::core::ptr::null_mut::<GIOExtensionPoint>();
    let mut extension: *mut GIOExtension = ::core::ptr::null_mut::<GIOExtension>();
    let mut preferred: *mut GIOExtension = ::core::ptr::null_mut::<GIOExtension>();
    let mut impl_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    g_rec_mutex_lock(&raw mut safe_c2rust_default_modules_lock);
    if !safe_c2rust_default_modules.is_null() {
        let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if g_hash_table_lookup_extended(
            safe_c2rust_default_modules,
            extension_point as gconstpointer,
            &raw mut key,
            &raw mut impl_0,
        ) != 0
        {
            g_rec_mutex_unlock(&raw mut safe_c2rust_default_modules_lock);
            return if !impl_0.is_null() {
                (*(impl_0 as *mut GTypeClass)).g_type
            } else {
                G_TYPE_INVALID
            };
        }
    } else {
        safe_c2rust_default_modules = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
    }
    safe_c2rust__g_io_modules_ensure_loaded();
    ep = safe_c2rust_g_io_extension_point_lookup(extension_point as *const ::core::ffi::c_char);
    if ep.is_null() {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/giomodule.c\0" as *const u8
                as *const ::core::ffi::c_char,
            823 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        g_rec_mutex_unlock(&raw mut safe_c2rust_default_modules_lock);
        return G_TYPE_INVALID;
    }
    use_this = (if !envvar.is_null() {
        g_getenv(envvar)
    } else {
        ::core::ptr::null::<gchar>()
    }) as *const ::core::ffi::c_char;
    if g_strcmp0(
        use_this,
        b"help\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        safe_c2rust_print_help(envvar as *const ::core::ffi::c_char, ep);
        use_this = ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !use_this.is_null() {
        preferred = safe_c2rust_g_io_extension_point_get_extension_by_name(ep, use_this);
        if !preferred.is_null() {
            impl_0 = safe_c2rust_try_class(preferred, is_supported_offset);
            if !impl_0.is_null() {
                current_block = 6406481448373700448;
            } else {
                current_block = 4068382217303356765;
            }
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Can't find module '%s' specified in %s\0" as *const u8 as *const gchar,
                use_this,
                envvar,
            );
            current_block = 4068382217303356765;
        }
    } else {
        preferred = ::core::ptr::null_mut::<GIOExtension>();
        current_block = 4068382217303356765;
    }
    match current_block {
        4068382217303356765 => {
            l = safe_c2rust_g_io_extension_point_get_extensions(ep);
            loop {
                if l.is_null() {
                    current_block = 2891135413264362348;
                    break;
                }
                extension = (*l).data as *mut GIOExtension;
                if !(extension == preferred) {
                    impl_0 = safe_c2rust_try_class(extension, is_supported_offset);
                    if !impl_0.is_null() {
                        current_block = 6406481448373700448;
                        break;
                    }
                }
                l = (*l).next;
            }
            match current_block {
                6406481448373700448 => {}
                _ => {
                    impl_0 = NULL_1 as gpointer;
                }
            }
        }
        _ => {}
    }
    g_hash_table_insert(
        safe_c2rust_default_modules,
        safe_c2rust_g_strdup_inline(extension_point as *const ::core::ffi::c_char) as gpointer,
        impl_0,
    );
    g_rec_mutex_unlock(&raw mut safe_c2rust_default_modules_lock);
    return if !impl_0.is_null() {
        (*(impl_0 as *mut GTypeClass)).g_type
    } else {
        G_TYPE_INVALID
    };
}
unsafe extern "C" fn safe_c2rust_try_implementation(
    mut extension_point: *const ::core::ffi::c_char,
    mut extension: *mut GIOExtension,
    mut verify_func: GIOModuleVerifyFunc,
) -> gpointer {
    let mut type_0: GType = safe_c2rust_g_io_extension_get_type(extension);
    let mut impl_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if type_0 == g_initable_get_type() || g_type_is_a(type_0, g_initable_get_type()) != 0 {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        impl_0 = g_initable_new(
            type_0,
            ::core::ptr::null_mut::<GCancellable>(),
            &raw mut error,
            ::core::ptr::null::<gchar>(),
        );
        if !impl_0.is_null() {
            return impl_0;
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"Failed to initialize %s (%s) for %s: %s\0" as *const u8 as *const gchar,
            safe_c2rust_g_io_extension_get_name(extension),
            g_type_name(type_0),
            extension_point,
            if !error.is_null() {
                (*error).message as *const gchar
            } else {
                b"\0" as *const u8 as *const gchar
            },
        );
        g_clear_error(&raw mut error);
        return NULL_1;
    } else {
        impl_0 = g_object_new(type_0, ::core::ptr::null::<gchar>());
        if verify_func.is_none() || verify_func.expect("non-null function pointer")(impl_0) != 0 {
            return impl_0;
        }
        g_object_unref(impl_0);
        return NULL_1;
    };
}
unsafe extern "C" fn safe_c2rust_weak_ref_free(mut weak_ref: *mut GWeakRef) {
    g_weak_ref_clear(weak_ref);
    g_free(weak_ref as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_io_module_get_default(
    mut extension_point: *const gchar,
    mut envvar: *const gchar,
    mut verify_func: GIOModuleVerifyFunc,
) -> gpointer {
    let mut current_block: u64;
    static mut safe_c2rust_default_modules_lock: GRecMutex = _GRecMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        i: [0; 2],
    };
    static mut safe_c2rust_default_modules: *mut GHashTable =
        ::core::ptr::null::<GHashTable>() as *mut GHashTable;
    let mut use_this: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut ep: *mut GIOExtensionPoint = ::core::ptr::null_mut::<GIOExtensionPoint>();
    let mut extension: *mut GIOExtension = ::core::ptr::null_mut::<GIOExtension>();
    let mut preferred: *mut GIOExtension = ::core::ptr::null_mut::<GIOExtension>();
    let mut impl_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut impl_weak_ref: *mut GWeakRef = ::core::ptr::null_mut::<GWeakRef>();
    g_rec_mutex_lock(&raw mut safe_c2rust_default_modules_lock);
    if !safe_c2rust_default_modules.is_null() {
        if g_hash_table_lookup_extended(
            safe_c2rust_default_modules,
            extension_point as gconstpointer,
            ::core::ptr::null_mut::<gpointer>(),
            &raw mut value,
        ) != 0
        {
            impl_weak_ref = value as *mut GWeakRef;
            impl_0 = g_weak_ref_get(impl_weak_ref);
            if !impl_0.is_null() {
                g_rec_mutex_unlock(&raw mut safe_c2rust_default_modules_lock);
                return safe_c2rust_g_steal_pointer(&raw mut impl_0 as gpointer);
            }
        }
    } else {
        safe_c2rust_default_modules = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GWeakRef) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_weak_ref_free as unsafe extern "C" fn(*mut GWeakRef) -> (),
            )),
        );
    }
    safe_c2rust__g_io_modules_ensure_loaded();
    ep = safe_c2rust_g_io_extension_point_lookup(extension_point as *const ::core::ffi::c_char);
    if ep.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"%s: Failed to find extension point \xE2\x80\x98%s\xE2\x80\x99\0" as *const u8
                as *const gchar,
            b"_g_io_module_get_default\0" as *const u8 as *const ::core::ffi::c_char,
            extension_point,
        );
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/giomodule.c\0" as *const u8
                as *const ::core::ffi::c_char,
            991 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        g_rec_mutex_unlock(&raw mut safe_c2rust_default_modules_lock);
        return NULL_1;
    }
    use_this = (if !envvar.is_null() {
        g_getenv(envvar)
    } else {
        ::core::ptr::null::<gchar>()
    }) as *const ::core::ffi::c_char;
    if g_strcmp0(
        use_this,
        b"help\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        safe_c2rust_print_help(envvar as *const ::core::ffi::c_char, ep);
        use_this = ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !use_this.is_null() {
        preferred = safe_c2rust_g_io_extension_point_get_extension_by_name(ep, use_this);
        if !preferred.is_null() {
            impl_0 = safe_c2rust_try_implementation(
                extension_point as *const ::core::ffi::c_char,
                preferred,
                verify_func,
            );
            extension = preferred;
            if !impl_0.is_null() {
                current_block = 2450911916770385668;
            } else {
                current_block = 8693738493027456495;
            }
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Can't find module '%s' specified in %s\0" as *const u8 as *const gchar,
                use_this,
                envvar,
            );
            current_block = 8693738493027456495;
        }
    } else {
        preferred = ::core::ptr::null_mut::<GIOExtension>();
        current_block = 8693738493027456495;
    }
    match current_block {
        8693738493027456495 => {
            l = safe_c2rust_g_io_extension_point_get_extensions(ep);
            loop {
                if l.is_null() {
                    current_block = 10692455896603418738;
                    break;
                }
                extension = (*l).data as *mut GIOExtension;
                if !(extension == preferred) {
                    impl_0 = safe_c2rust_try_implementation(
                        extension_point as *const ::core::ffi::c_char,
                        extension,
                        verify_func,
                    );
                    if !impl_0.is_null() {
                        current_block = 2450911916770385668;
                        break;
                    }
                }
                l = (*l).next;
            }
            match current_block {
                2450911916770385668 => {}
                _ => {
                    impl_0 = NULL_1 as gpointer;
                }
            }
        }
        _ => {}
    }
    if impl_weak_ref.is_null() {
        impl_weak_ref = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<GWeakRef>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut GWeakRef;
        g_weak_ref_init(impl_weak_ref, impl_0);
        g_hash_table_insert(
            safe_c2rust_default_modules,
            safe_c2rust_g_strdup_inline(extension_point as *const ::core::ffi::c_char) as gpointer,
            safe_c2rust_g_steal_pointer(&raw mut impl_weak_ref as gpointer) as *mut GWeakRef
                as gpointer,
        );
    } else {
        g_weak_ref_set(impl_weak_ref, impl_0);
    }
    g_rec_mutex_unlock(&raw mut safe_c2rust_default_modules_lock);
    if !impl_0.is_null() {
        if ({
            let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
            if !extension.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/giomodule.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1052 as ::core::ffi::c_int,
                G_STRFUNC,
                b"extension != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"%s: Found default implementation %s (%s) for \xE2\x80\x98%s\xE2\x80\x99\0"
                as *const u8 as *const gchar,
            b"_g_io_module_get_default\0" as *const u8 as *const ::core::ffi::c_char,
            safe_c2rust_g_io_extension_get_name(extension),
            g_type_name((*(*(impl_0 as *mut GTypeInstance)).g_class).g_type),
            extension_point,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"%s: Failed to find default implementation for \xE2\x80\x98%s\xE2\x80\x99\0"
                as *const u8 as *const gchar,
            b"_g_io_module_get_default\0" as *const u8 as *const ::core::ffi::c_char,
            extension_point,
        );
    }
    return safe_c2rust_g_steal_pointer(&raw mut impl_0 as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_io_modules_ensure_extension_points_registered() {
    static mut safe_c2rust_registered_extensions: gsize = FALSE as gsize;
    let mut ep: *mut GIOExtensionPoint = ::core::ptr::null_mut::<GIOExtensionPoint>();
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_registered_extensions;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_registered_extensions;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(
                &raw mut safe_c2rust_registered_extensions as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        ep = safe_c2rust_g_io_extension_point_register(
            b"gio-desktop-app-info-lookup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        safe_c2rust_g_io_extension_point_set_required_type(
            ep,
            g_desktop_app_info_lookup_get_type(),
        );
        ep = safe_c2rust_g_io_extension_point_register(
            G_LOCAL_FILE_MONITOR_EXTENSION_POINT_NAME.as_ptr(),
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_local_file_monitor_get_type());
        ep = safe_c2rust_g_io_extension_point_register(
            G_NFS_FILE_MONITOR_EXTENSION_POINT_NAME.as_ptr(),
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_local_file_monitor_get_type());
        ep = safe_c2rust_g_io_extension_point_register(
            G_VOLUME_MONITOR_EXTENSION_POINT_NAME.as_ptr(),
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_volume_monitor_get_type());
        ep = safe_c2rust_g_io_extension_point_register(
            G_NATIVE_VOLUME_MONITOR_EXTENSION_POINT_NAME.as_ptr(),
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_native_volume_monitor_get_type());
        ep = safe_c2rust_g_io_extension_point_register(G_VFS_EXTENSION_POINT_NAME.as_ptr());
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_vfs_get_type());
        ep = safe_c2rust_g_io_extension_point_register(
            b"gsettings-backend\0" as *const u8 as *const ::core::ffi::c_char,
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, G_TYPE_OBJECT);
        ep = safe_c2rust_g_io_extension_point_register(
            G_PROXY_RESOLVER_EXTENSION_POINT_NAME.as_ptr(),
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_proxy_resolver_get_type());
        ep = safe_c2rust_g_io_extension_point_register(G_PROXY_EXTENSION_POINT_NAME.as_ptr());
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_proxy_get_type());
        ep = safe_c2rust_g_io_extension_point_register(G_TLS_BACKEND_EXTENSION_POINT_NAME.as_ptr());
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_tls_backend_get_type());
        ep = safe_c2rust_g_io_extension_point_register(
            G_NETWORK_MONITOR_EXTENSION_POINT_NAME.as_ptr(),
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_network_monitor_get_type());
        ep = safe_c2rust_g_io_extension_point_register(
            G_NOTIFICATION_BACKEND_EXTENSION_POINT_NAME.as_ptr(),
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_notification_backend_get_type());
        ep = safe_c2rust_g_io_extension_point_register(
            G_DEBUG_CONTROLLER_EXTENSION_POINT_NAME.as_ptr(),
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_debug_controller_get_type());
        ep = safe_c2rust_g_io_extension_point_register(
            G_MEMORY_MONITOR_EXTENSION_POINT_NAME.as_ptr(),
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_memory_monitor_get_type());
        ep = safe_c2rust_g_io_extension_point_register(
            G_POWER_PROFILE_MONITOR_EXTENSION_POINT_NAME.as_ptr(),
        );
        safe_c2rust_g_io_extension_point_set_required_type(ep, g_power_profile_monitor_get_type());
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_registered_extensions =
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_registered_extensions as *mut ::core::ffi::c_void,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
        );
    }
}
unsafe extern "C" fn safe_c2rust_get_gio_module_dir() -> *mut gchar {
    let mut module_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut is_setuid: gboolean = (*glib__private__())
        .g_check_setuid
        .expect("non-null function pointer")();
    module_dir = (if is_setuid == 0 {
        safe_c2rust_g_strdup_inline(g_getenv(b"GIO_MODULE_DIR\0" as *const u8 as *const gchar)
            as *const ::core::ffi::c_char)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    }) as *mut gchar;
    if module_dir.is_null() {
        module_dir = safe_c2rust_g_strdup_inline(
            b"/usr/local/lib/x86_64-linux-gnu/gio/modules\0" as *const u8
                as *const ::core::ffi::c_char,
        ) as *mut gchar;
    }
    return module_dir;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_io_modules_ensure_loaded() {
    static mut safe_c2rust_loaded_dirs: gsize = FALSE as gsize;
    let mut module_path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut scope: *mut GIOModuleScope = ::core::ptr::null_mut::<GIOModuleScope>();
    safe_c2rust__g_io_modules_ensure_extension_points_registered();
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_loaded_dirs;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_loaded_dirs;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_loaded_dirs as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        let mut is_setuid: gboolean = (*glib__private__())
            .g_check_setuid
            .expect("non-null function pointer")();
        let mut module_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
        scope = safe_c2rust_g_io_module_scope_new(G_IO_MODULE_SCOPE_BLOCK_DUPLICATES);
        module_path = (if is_setuid == 0 {
            g_getenv(b"GIO_EXTRA_MODULES\0" as *const u8 as *const gchar)
        } else {
            ::core::ptr::null::<gchar>()
        }) as *const ::core::ffi::c_char;
        if !module_path.is_null() {
            let mut paths: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
            let mut i: ::core::ffi::c_int = 0;
            paths = g_strsplit(
                module_path as *const gchar,
                G_SEARCHPATH_SEPARATOR_S.as_ptr() as *const gchar,
                0 as gint,
            );
            i = 0 as ::core::ffi::c_int;
            while !(*paths.offset(i as isize)).is_null() {
                safe_c2rust_g_io_modules_scan_all_in_directory_with_scope(
                    *paths.offset(i as isize),
                    scope,
                );
                i += 1;
            }
            g_strfreev(paths);
        }
        module_dir = safe_c2rust_get_gio_module_dir();
        safe_c2rust_g_io_modules_scan_all_in_directory_with_scope(module_dir, scope);
        g_free(module_dir as gpointer);
        safe_c2rust_g_io_module_scope_free(scope);
        g_type_ensure(g_null_settings_backend_get_type());
        g_type_ensure(g_memory_settings_backend_get_type());
        g_type_ensure(g_keyfile_settings_backend_get_type());
        g_type_ensure(g_power_profile_monitor_dbus_get_type());
        g_type_ensure(g_inotify_file_monitor_get_type());
        g_type_ensure(_g_unix_volume_monitor_get_type());
        g_type_ensure(g_debug_controller_dbus_get_type());
        g_type_ensure(g_fdo_notification_backend_get_type());
        g_type_ensure(g_gtk_notification_backend_get_type());
        g_type_ensure(g_portal_notification_backend_get_type());
        g_type_ensure(g_memory_monitor_dbus_get_type());
        g_type_ensure(g_memory_monitor_portal_get_type());
        g_type_ensure(g_network_monitor_portal_get_type());
        g_type_ensure(g_power_profile_monitor_portal_get_type());
        g_type_ensure(g_proxy_resolver_portal_get_type());
        g_type_ensure(_g_local_vfs_get_type());
        g_type_ensure(_g_dummy_proxy_resolver_get_type());
        g_type_ensure(_g_http_proxy_get_type());
        g_type_ensure(_g_https_proxy_get_type());
        g_type_ensure(_g_socks4a_proxy_get_type());
        g_type_ensure(_g_socks4_proxy_get_type());
        g_type_ensure(_g_socks5_proxy_get_type());
        g_type_ensure(_g_dummy_tls_backend_get_type());
        g_type_ensure(g_network_monitor_base_get_type());
        g_type_ensure(_g_network_monitor_netlink_get_type());
        g_type_ensure(_g_network_monitor_nm_get_type());
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_loaded_dirs = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_loaded_dirs as *mut ::core::ffi::c_void,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_io_extension_point_free(mut ep: *mut GIOExtensionPoint) {
    g_free((*ep).name as gpointer);
    g_free(ep as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_point_register(
    mut name: *const ::core::ffi::c_char,
) -> *mut GIOExtensionPoint {
    let mut ep: *mut GIOExtensionPoint = ::core::ptr::null_mut::<GIOExtensionPoint>();
    g_mutex_lock(&raw mut safe_c2rust_g__extension_points_lock);
    if safe_c2rust_extension_points.is_null() {
        safe_c2rust_extension_points = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            None,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GIOExtensionPoint) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_g_io_extension_point_free
                    as unsafe extern "C" fn(*mut GIOExtensionPoint) -> (),
            )),
        );
    }
    ep = g_hash_table_lookup(safe_c2rust_extension_points, name as gconstpointer)
        as *mut GIOExtensionPoint;
    if !ep.is_null() {
        g_mutex_unlock(&raw mut safe_c2rust_g__extension_points_lock);
        return ep;
    }
    ep = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GIOExtensionPoint>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GIOExtensionPoint;
    (*ep).name = safe_c2rust_g_strdup_inline(name);
    g_hash_table_insert(
        safe_c2rust_extension_points,
        (*ep).name as gpointer,
        ep as gpointer,
    );
    g_mutex_unlock(&raw mut safe_c2rust_g__extension_points_lock);
    return ep;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_point_lookup(
    mut name: *const ::core::ffi::c_char,
) -> *mut GIOExtensionPoint {
    let mut ep: *mut GIOExtensionPoint = ::core::ptr::null_mut::<GIOExtensionPoint>();
    g_mutex_lock(&raw mut safe_c2rust_g__extension_points_lock);
    ep = ::core::ptr::null_mut::<GIOExtensionPoint>();
    if !safe_c2rust_extension_points.is_null() {
        ep = g_hash_table_lookup(safe_c2rust_extension_points, name as gconstpointer)
            as *mut GIOExtensionPoint;
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__extension_points_lock);
    return ep;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_point_set_required_type(
    mut extension_point: *mut GIOExtensionPoint,
    mut type_0: GType,
) {
    (*extension_point).required_type = type_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_point_get_required_type(
    mut extension_point: *mut GIOExtensionPoint,
) -> GType {
    return (*extension_point).required_type;
}
unsafe extern "C" fn safe_c2rust_lazy_load_modules(mut extension_point: *mut GIOExtensionPoint) {
    let mut module: *mut GIOModule = ::core::ptr::null_mut::<GIOModule>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    l = (*extension_point).lazy_load_modules;
    while !l.is_null() {
        module = (*l).data as *mut GIOModule;
        if (*module).initialized == 0 {
            if g_type_module_use(module as *mut ::core::ffi::c_void as *mut GTypeModule) != 0 {
                g_type_module_unuse(module as *mut ::core::ffi::c_void as *mut GTypeModule);
            } else {
                g_printerr(
                    b"Failed to load module: %s\n\0" as *const u8 as *const gchar,
                    (*module).filename,
                );
            }
        }
        l = (*l).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_point_get_extensions(
    mut extension_point: *mut GIOExtensionPoint,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !extension_point.is_null() {
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
            b"extension_point != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    safe_c2rust_lazy_load_modules(extension_point);
    return (*extension_point).extensions;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_point_get_extension_by_name(
    mut extension_point: *mut GIOExtensionPoint,
    mut name: *const ::core::ffi::c_char,
) -> *mut GIOExtension {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !name.is_null() {
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
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOExtension>();
    }
    safe_c2rust_lazy_load_modules(extension_point);
    l = (*extension_point).extensions;
    while !l.is_null() {
        let mut e: *mut GIOExtension = (*l).data as *mut GIOExtension;
        if !(*e).name.is_null() && strcmp((*e).name, name) == 0 as ::core::ffi::c_int {
            return e;
        }
        l = (*l).next;
    }
    return ::core::ptr::null_mut::<GIOExtension>();
}
unsafe extern "C" fn safe_c2rust_extension_prio_compare(
    mut a: gconstpointer,
    mut b: gconstpointer,
) -> gint {
    let mut extension_a: *const GIOExtension = a as *const GIOExtension;
    let mut extension_b: *const GIOExtension = b as *const GIOExtension;
    if (*extension_a).priority > (*extension_b).priority {
        return -(1 as gint);
    }
    if (*extension_b).priority > (*extension_a).priority {
        return 1 as gint;
    }
    return 0 as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_point_implement(
    mut extension_point_name: *const ::core::ffi::c_char,
    mut type_0: GType,
    mut extension_name: *const ::core::ffi::c_char,
    mut priority: gint,
) -> *mut GIOExtension {
    let mut extension_point: *mut GIOExtensionPoint = ::core::ptr::null_mut::<GIOExtensionPoint>();
    let mut extension: *mut GIOExtension = ::core::ptr::null_mut::<GIOExtension>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !extension_point_name.is_null() {
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
            b"extension_point_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOExtension>();
    }
    extension_point = safe_c2rust_g_io_extension_point_lookup(extension_point_name);
    if extension_point.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Tried to implement non-registered extension point %s\0" as *const u8 as *const gchar,
            extension_point_name,
        );
        return ::core::ptr::null_mut::<GIOExtension>();
    }
    if (*extension_point).required_type != 0 as GType
        && !(type_0 == (*extension_point).required_type
            || g_type_is_a(type_0, (*extension_point).required_type) != 0)
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Tried to register an extension of the type %s to extension point %s. Expected type is %s.\0"
                as *const u8 as *const gchar,
            g_type_name(type_0),
            extension_point_name,
            g_type_name((*extension_point).required_type),
        );
        return ::core::ptr::null_mut::<GIOExtension>();
    }
    l = (*extension_point).extensions;
    while !l.is_null() {
        extension = (*l).data as *mut GIOExtension;
        if (*extension).type_0 == type_0 {
            return extension;
        }
        l = (*l).next;
    }
    extension = ({
        let mut __s: gsize = ::core::mem::size_of::<GIOExtension>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GIOExtension;
    (*extension).type_0 = type_0;
    (*extension).name = safe_c2rust_g_strdup_inline(extension_name);
    (*extension).priority = priority;
    (*extension_point).extensions = g_list_insert_sorted(
        (*extension_point).extensions,
        extension as gpointer,
        Some(
            safe_c2rust_extension_prio_compare
                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
        ),
    );
    return extension;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_ref_class(
    mut extension: *mut GIOExtension,
) -> *mut GTypeClass {
    return g_type_class_ref((*extension).type_0) as *mut GTypeClass;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_get_type(
    mut extension: *mut GIOExtension,
) -> GType {
    return (*extension).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_get_name(
    mut extension: *mut GIOExtension,
) -> *const ::core::ffi::c_char {
    return (*extension).name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_io_extension_get_priority(
    mut extension: *mut GIOExtension,
) -> gint {
    return (*extension).priority;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
