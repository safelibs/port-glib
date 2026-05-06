extern "C" {
    pub type _GData;
    pub type _GCancellable;
    pub type _GInitable;
    pub type _GSubprocess;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_unref(array: *mut GArray);
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_get_environ() -> *mut *mut gchar;
    fn g_environ_getenv(envp: *mut *mut gchar, variable: *const gchar) -> *const gchar;
    fn g_environ_setenv(
        envp: *mut *mut gchar,
        variable: *const gchar,
        value: *const gchar,
        overwrite: gboolean,
    ) -> *mut *mut gchar;
    fn g_environ_unsetenv(envp: *mut *mut gchar, variable: *const gchar) -> *mut *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
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
    fn g_type_class_peek(type_0: GType) -> gpointer;
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
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_subprocess_set_launcher(subprocess: *mut GSubprocess, launcher: *mut GSubprocessLauncher);
    fn g_subprocess_flags_get_type() -> GType;
    fn g_subprocess_get_type() -> GType;
    fn g_initable_init(
        initable: *mut GInitable,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
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
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
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
pub type GSpawnChildSetupFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub struct _GFlagsClass {
    pub g_type_class: GTypeClass,
    pub mask: guint,
    pub n_values: guint,
    pub values: *mut GFlagsValue,
}
pub type GFlagsValue = _GFlagsValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFlagsValue {
    pub value: guint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GFlagsClass = _GFlagsClass;
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
pub type GSubprocessFlags = ::core::ffi::c_uint;
pub const G_SUBPROCESS_FLAGS_SEARCH_PATH_FROM_ENVP: GSubprocessFlags = 256;
pub const G_SUBPROCESS_FLAGS_INHERIT_FDS: GSubprocessFlags = 128;
pub const G_SUBPROCESS_FLAGS_STDERR_MERGE: GSubprocessFlags = 64;
pub const G_SUBPROCESS_FLAGS_STDERR_SILENCE: GSubprocessFlags = 32;
pub const G_SUBPROCESS_FLAGS_STDERR_PIPE: GSubprocessFlags = 16;
pub const G_SUBPROCESS_FLAGS_STDOUT_SILENCE: GSubprocessFlags = 8;
pub const G_SUBPROCESS_FLAGS_STDOUT_PIPE: GSubprocessFlags = 4;
pub const G_SUBPROCESS_FLAGS_STDIN_INHERIT: GSubprocessFlags = 2;
pub const G_SUBPROCESS_FLAGS_STDIN_PIPE: GSubprocessFlags = 1;
pub const G_SUBPROCESS_FLAGS_NONE: GSubprocessFlags = 0;
pub type GCancellable = _GCancellable;
pub type GInitable = _GInitable;
pub type GSubprocess = _GSubprocess;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSubprocessLauncher {
    pub parent: GObject,
    pub flags: GSubprocessFlags,
    pub envp: *mut *mut ::core::ffi::c_char,
    pub cwd: *mut ::core::ffi::c_char,
    pub stdin_fd: gint,
    pub stdin_path: *mut gchar,
    pub stdout_fd: gint,
    pub stdout_path: *mut gchar,
    pub stderr_fd: gint,
    pub stderr_path: *mut gchar,
    pub source_fds: *mut GArray,
    pub target_fds: *mut GArray,
    pub closed_fd: gboolean,
    pub child_setup_func: GSpawnChildSetupFunc,
    pub child_setup_user_data: gpointer,
    pub child_setup_destroy_notify: GDestroyNotify,
}
pub type GSubprocessLauncher = _GSubprocessLauncher;
pub type GSubprocessLauncherClass = GObjectClass;
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
pub const F_GETFD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_subprocess_launcher_get_type_once();
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
static mut safe_c2rust_g_subprocess_launcher_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GSubprocessLauncher_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_subprocess_launcher_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSubprocessLauncher_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSubprocessLauncher_private_offset,
        );
    }
    safe_c2rust_g_subprocess_launcher_class_init(klass as *mut GSubprocessLauncherClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSubprocessLauncher\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSubprocessLauncherClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_subprocess_launcher_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSubprocessLauncher>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSubprocessLauncher) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_subprocess_launcher_init
                    as unsafe extern "C" fn(*mut GSubprocessLauncher) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_verify_disposition(
    mut stream_name: *const gchar,
    mut filtered_flags: GSubprocessFlags,
    mut fd: gint,
    mut filename: *const gchar,
) -> gboolean {
    let mut n_bits: guint = 0;
    if filtered_flags as u64 == 0 {
        n_bits = 0 as guint;
    } else if (filtered_flags as ::core::ffi::c_uint).wrapping_sub(1 as ::core::ffi::c_uint)
        & filtered_flags as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
        n_bits = 1 as guint;
    } else {
        n_bits = 2 as guint;
    }
    if n_bits
        .wrapping_add((fd >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as guint)
        .wrapping_add((filename != NULL_0 as *const gchar) as ::core::ffi::c_int as guint)
        > 1 as guint
    {
        let mut err: *mut GString = ::core::ptr::null_mut::<GString>();
        err = g_string_new(::core::ptr::null::<gchar>());
        if n_bits != 0 {
            let mut class: *mut GFlagsClass = ::core::ptr::null_mut::<GFlagsClass>();
            let mut i: guint = 0;
            class = g_type_class_peek(g_subprocess_flags_get_type()) as *mut GFlagsClass;
            i = 0 as guint;
            while i < (*class).n_values {
                let mut value: *const GFlagsValue =
                    (*class).values.offset(i as isize) as *mut GFlagsValue;
                if filtered_flags as guint & (*value).value != 0 {
                    g_string_append_printf(
                        err,
                        b" %s\0" as *const u8 as *const gchar,
                        (*value).value_name,
                    );
                }
                i = i.wrapping_add(1);
            }
            g_type_class_unref(class as gpointer);
        }
        if fd >= 0 as ::core::ffi::c_int {
            g_string_append_printf(
                err,
                b" g_subprocess_launcher_take_%s_fd()\0" as *const u8 as *const gchar,
                stream_name,
            );
        }
        if !filename.is_null() {
            g_string_append_printf(
                err,
                b" g_subprocess_launcher_set_%s_file_path()\0" as *const u8 as *const gchar,
                stream_name,
            );
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"You may specify at most one disposition for the %s stream, but you specified:%s.\0"
                as *const u8 as *const gchar,
            stream_name,
            (*err).str_0,
        );
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(err, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(err);
            };
        } else {
            g_string_free(err, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_verify_flags(mut flags: GSubprocessFlags) -> gboolean {
    return (safe_c2rust_verify_disposition(
        b"stdin\0" as *const u8 as *const gchar,
        (flags as ::core::ffi::c_uint
            & (G_SUBPROCESS_FLAGS_STDIN_PIPE as ::core::ffi::c_int
                | G_SUBPROCESS_FLAGS_STDIN_INHERIT as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as GSubprocessFlags,
        -(1 as gint),
        ::core::ptr::null::<gchar>(),
    ) != 0
        && safe_c2rust_verify_disposition(
            b"stdout\0" as *const u8 as *const gchar,
            (flags as ::core::ffi::c_uint
                & (G_SUBPROCESS_FLAGS_STDOUT_PIPE as ::core::ffi::c_int
                    | G_SUBPROCESS_FLAGS_STDOUT_SILENCE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as GSubprocessFlags,
            -(1 as gint),
            ::core::ptr::null::<gchar>(),
        ) != 0
        && safe_c2rust_verify_disposition(
            b"stderr\0" as *const u8 as *const gchar,
            (flags as ::core::ffi::c_uint
                & (G_SUBPROCESS_FLAGS_STDERR_PIPE as ::core::ffi::c_int
                    | G_SUBPROCESS_FLAGS_STDERR_SILENCE as ::core::ffi::c_int
                    | G_SUBPROCESS_FLAGS_STDERR_MERGE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as GSubprocessFlags,
            -(1 as gint),
            ::core::ptr::null::<gchar>(),
        ) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut launcher: *mut GSubprocessLauncher =
        object as *mut ::core::ffi::c_void as *mut GSubprocessLauncher;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if prop_id == 1 as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocesslauncher.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            127 as ::core::ffi::c_int,
            G_STRFUNC,
            b"prop_id == 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if safe_c2rust_verify_flags(g_value_get_flags(value) as GSubprocessFlags) != 0 {
        (*launcher).flags = g_value_get_flags(value) as GSubprocessFlags;
    }
}
unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_dispose(mut object: *mut GObject) {
    let mut self_0: *mut GSubprocessLauncher =
        object as *mut ::core::ffi::c_void as *mut GSubprocessLauncher;
    let mut _pp: *mut *mut gchar = &raw mut (*self_0).stdin_path;
    let mut _ptr: *mut gchar = *_pp;
    *_pp = ::core::ptr::null_mut::<gchar>();
    if !_ptr.is_null() {
        g_free(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut gchar = &raw mut (*self_0).stdout_path;
    let mut _ptr_0: *mut gchar = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<gchar>();
    if !_ptr_0.is_null() {
        g_free(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut gchar = &raw mut (*self_0).stderr_path;
    let mut _ptr_1: *mut gchar = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<gchar>();
    if !_ptr_1.is_null() {
        g_free(_ptr_1 as gpointer);
    }
    safe_c2rust_g_subprocess_launcher_close(self_0);
    if (*self_0).child_setup_destroy_notify.is_some() {
        Some(
            (*self_0)
                .child_setup_destroy_notify
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")((*self_0).child_setup_user_data);
    }
    (*self_0).child_setup_destroy_notify = None;
    (*self_0).child_setup_user_data = NULL_0 as gpointer;
    let mut _pp_2: *mut *mut *mut ::core::ffi::c_char = &raw mut (*self_0).envp;
    let mut _ptr_2: *mut *mut ::core::ffi::c_char = *_pp_2;
    *_pp_2 = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if !_ptr_2.is_null() {
        g_strfreev(_ptr_2 as *mut *mut gchar);
    }
    let mut _pp_3: *mut *mut ::core::ffi::c_char = &raw mut (*self_0).cwd;
    let mut _ptr_3: *mut ::core::ffi::c_char = *_pp_3;
    *_pp_3 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !_ptr_3.is_null() {
        g_free(_ptr_3 as gpointer);
    }
    (*(safe_c2rust_g_subprocess_launcher_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_init(mut self_0: *mut GSubprocessLauncher) {
    (*self_0).envp = g_get_environ() as *mut *mut ::core::ffi::c_char;
    (*self_0).stdin_fd = -(1 as ::core::ffi::c_int) as gint;
    (*self_0).stdout_fd = -(1 as ::core::ffi::c_int) as gint;
    (*self_0).stderr_fd = -(1 as ::core::ffi::c_int) as gint;
    (*self_0).source_fds = g_array_new(
        FALSE,
        0 as gboolean,
        ::core::mem::size_of::<::core::ffi::c_int>() as guint,
    );
    (*self_0).target_fds = g_array_new(
        FALSE,
        0 as gboolean,
        ::core::mem::size_of::<::core::ffi::c_int>() as guint,
    );
}
unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_class_init(
    mut class: *mut GSubprocessLauncherClass,
) {
    let mut gobject_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_subprocess_launcher_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).dispose =
        Some(safe_c2rust_g_subprocess_launcher_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        gobject_class,
        1 as guint,
        g_param_spec_flags(
            b"flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_subprocess_flags_get_type(),
            0 as guint,
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_new(
    mut flags: GSubprocessFlags,
) -> *mut GSubprocessLauncher {
    if safe_c2rust_verify_flags(flags) == 0 {
        return ::core::ptr::null_mut::<GSubprocessLauncher>();
    }
    return g_object_new(
        safe_c2rust_g_subprocess_launcher_get_type(),
        b"flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        NULL_0,
    ) as *mut GSubprocessLauncher;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_set_environ(
    mut self_0: *mut GSubprocessLauncher,
    mut env: *mut *mut gchar,
) {
    g_strfreev((*self_0).envp as *mut *mut gchar);
    (*self_0).envp = g_strdupv(env) as *mut *mut ::core::ffi::c_char;
    if (*self_0).envp.is_null() {
        (*self_0).envp = g_get_environ() as *mut *mut ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_setenv(
    mut self_0: *mut GSubprocessLauncher,
    mut variable: *const gchar,
    mut value: *const gchar,
    mut overwrite: gboolean,
) {
    (*self_0).envp = g_environ_setenv(
        (*self_0).envp as *mut *mut gchar,
        variable,
        value,
        overwrite,
    ) as *mut *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_unsetenv(
    mut self_0: *mut GSubprocessLauncher,
    mut variable: *const gchar,
) {
    (*self_0).envp = g_environ_unsetenv((*self_0).envp as *mut *mut gchar, variable)
        as *mut *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_getenv(
    mut self_0: *mut GSubprocessLauncher,
    mut variable: *const gchar,
) -> *const gchar {
    return g_environ_getenv((*self_0).envp as *mut *mut gchar, variable);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_set_cwd(
    mut self_0: *mut GSubprocessLauncher,
    mut cwd: *const gchar,
) {
    g_free((*self_0).cwd as gpointer);
    (*self_0).cwd = safe_c2rust_g_strdup_inline(cwd as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_set_flags(
    mut self_0: *mut GSubprocessLauncher,
    mut flags: GSubprocessFlags,
) {
    let mut stdin_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut stdout_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut stderr_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut stdin_fd: gint = -(1 as gint);
    let mut stdout_fd: gint = -(1 as gint);
    let mut stderr_fd: gint = -(1 as gint);
    stdin_fd = (*self_0).stdin_fd;
    stdout_fd = (*self_0).stdout_fd;
    stderr_fd = (*self_0).stderr_fd;
    stdin_path = (*self_0).stdin_path;
    stdout_path = (*self_0).stdout_path;
    stderr_path = (*self_0).stderr_path;
    if safe_c2rust_verify_disposition(
        b"stdin\0" as *const u8 as *const gchar,
        (flags as ::core::ffi::c_uint
            & (G_SUBPROCESS_FLAGS_STDIN_PIPE as ::core::ffi::c_int
                | G_SUBPROCESS_FLAGS_STDIN_INHERIT as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as GSubprocessFlags,
        stdin_fd,
        stdin_path,
    ) != 0
        && safe_c2rust_verify_disposition(
            b"stdout\0" as *const u8 as *const gchar,
            (flags as ::core::ffi::c_uint
                & (G_SUBPROCESS_FLAGS_STDOUT_PIPE as ::core::ffi::c_int
                    | G_SUBPROCESS_FLAGS_STDOUT_SILENCE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as GSubprocessFlags,
            stdout_fd,
            stdout_path,
        ) != 0
        && safe_c2rust_verify_disposition(
            b"stderr\0" as *const u8 as *const gchar,
            (flags as ::core::ffi::c_uint
                & (G_SUBPROCESS_FLAGS_STDERR_PIPE as ::core::ffi::c_int
                    | G_SUBPROCESS_FLAGS_STDERR_SILENCE as ::core::ffi::c_int
                    | G_SUBPROCESS_FLAGS_STDERR_MERGE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint) as GSubprocessFlags,
            stderr_fd,
            stderr_path,
        ) != 0
    {
        (*self_0).flags = flags;
    }
}
unsafe extern "C" fn safe_c2rust_assign_fd(mut fd_ptr: *mut gint, mut fd: gint) {
    let mut flags: gint = 0;
    if *fd_ptr != -(1 as ::core::ffi::c_int) {
        close(*fd_ptr);
    }
    *fd_ptr = fd;
    if fd != -(1 as ::core::ffi::c_int) {
        flags = fcntl(fd as ::core::ffi::c_int, F_GETFD) as gint;
        if !(flags as ::core::ffi::c_int) & FD_CLOEXEC != 0 {
            fcntl(
                fd as ::core::ffi::c_int,
                F_SETFD,
                flags as ::core::ffi::c_int | FD_CLOEXEC,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_set_stdin_file_path(
    mut self_0: *mut GSubprocessLauncher,
    mut path: *const gchar,
) {
    if safe_c2rust_verify_disposition(
        b"stdin\0" as *const u8 as *const gchar,
        ((*self_0).flags as ::core::ffi::c_uint
            & (G_SUBPROCESS_FLAGS_STDIN_PIPE as ::core::ffi::c_int
                | G_SUBPROCESS_FLAGS_STDIN_INHERIT as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as GSubprocessFlags,
        (*self_0).stdin_fd,
        path,
    ) != 0
    {
        g_free((*self_0).stdin_path as gpointer);
        (*self_0).stdin_path =
            safe_c2rust_g_strdup_inline(path as *const ::core::ffi::c_char) as *mut gchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_take_stdin_fd(
    mut self_0: *mut GSubprocessLauncher,
    mut fd: gint,
) {
    if safe_c2rust_verify_disposition(
        b"stdin\0" as *const u8 as *const gchar,
        ((*self_0).flags as ::core::ffi::c_uint
            & (G_SUBPROCESS_FLAGS_STDIN_PIPE as ::core::ffi::c_int
                | G_SUBPROCESS_FLAGS_STDIN_INHERIT as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as GSubprocessFlags,
        fd,
        (*self_0).stdin_path,
    ) != 0
    {
        safe_c2rust_assign_fd(&raw mut (*self_0).stdin_fd, fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_set_stdout_file_path(
    mut self_0: *mut GSubprocessLauncher,
    mut path: *const gchar,
) {
    if safe_c2rust_verify_disposition(
        b"stdout\0" as *const u8 as *const gchar,
        ((*self_0).flags as ::core::ffi::c_uint
            & (G_SUBPROCESS_FLAGS_STDOUT_PIPE as ::core::ffi::c_int
                | G_SUBPROCESS_FLAGS_STDOUT_SILENCE as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as GSubprocessFlags,
        (*self_0).stdout_fd,
        path,
    ) != 0
    {
        g_free((*self_0).stdout_path as gpointer);
        (*self_0).stdout_path =
            safe_c2rust_g_strdup_inline(path as *const ::core::ffi::c_char) as *mut gchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_take_stdout_fd(
    mut self_0: *mut GSubprocessLauncher,
    mut fd: gint,
) {
    if safe_c2rust_verify_disposition(
        b"stdout\0" as *const u8 as *const gchar,
        ((*self_0).flags as ::core::ffi::c_uint
            & (G_SUBPROCESS_FLAGS_STDOUT_PIPE as ::core::ffi::c_int
                | G_SUBPROCESS_FLAGS_STDOUT_SILENCE as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as GSubprocessFlags,
        fd,
        (*self_0).stdout_path,
    ) != 0
    {
        safe_c2rust_assign_fd(&raw mut (*self_0).stdout_fd, fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_set_stderr_file_path(
    mut self_0: *mut GSubprocessLauncher,
    mut path: *const gchar,
) {
    if safe_c2rust_verify_disposition(
        b"stderr\0" as *const u8 as *const gchar,
        ((*self_0).flags as ::core::ffi::c_uint
            & (G_SUBPROCESS_FLAGS_STDERR_PIPE as ::core::ffi::c_int
                | G_SUBPROCESS_FLAGS_STDERR_SILENCE as ::core::ffi::c_int
                | G_SUBPROCESS_FLAGS_STDERR_MERGE as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as GSubprocessFlags,
        (*self_0).stderr_fd,
        path,
    ) != 0
    {
        g_free((*self_0).stderr_path as gpointer);
        (*self_0).stderr_path =
            safe_c2rust_g_strdup_inline(path as *const ::core::ffi::c_char) as *mut gchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_take_stderr_fd(
    mut self_0: *mut GSubprocessLauncher,
    mut fd: gint,
) {
    if safe_c2rust_verify_disposition(
        b"stderr\0" as *const u8 as *const gchar,
        ((*self_0).flags as ::core::ffi::c_uint
            & (G_SUBPROCESS_FLAGS_STDERR_PIPE as ::core::ffi::c_int
                | G_SUBPROCESS_FLAGS_STDERR_SILENCE as ::core::ffi::c_int
                | G_SUBPROCESS_FLAGS_STDERR_MERGE as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as GSubprocessFlags,
        fd,
        (*self_0).stderr_path,
    ) != 0
    {
        safe_c2rust_assign_fd(&raw mut (*self_0).stderr_fd, fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_take_fd(
    mut self_0: *mut GSubprocessLauncher,
    mut source_fd: gint,
    mut target_fd: gint,
) {
    if !(*self_0).source_fds.is_null() && !(*self_0).target_fds.is_null() {
        g_array_append_vals(
            (*self_0).source_fds,
            &raw mut source_fd as gconstpointer,
            1 as guint,
        );
        g_array_append_vals(
            (*self_0).target_fds,
            &raw mut target_fd as gconstpointer,
            1 as guint,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_close(
    mut self_0: *mut GSubprocessLauncher,
) {
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = self_0 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_subprocess_launcher_get_type();
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
            b"G_IS_SUBPROCESS_LAUNCHER (self)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*self_0).stdin_fd != -(1 as ::core::ffi::c_int) {
        close((*self_0).stdin_fd as ::core::ffi::c_int);
    }
    (*self_0).stdin_fd = -(1 as ::core::ffi::c_int) as gint;
    if (*self_0).stdout_fd != -(1 as ::core::ffi::c_int) {
        close((*self_0).stdout_fd as ::core::ffi::c_int);
    }
    (*self_0).stdout_fd = -(1 as ::core::ffi::c_int) as gint;
    if (*self_0).stderr_fd != -(1 as ::core::ffi::c_int) {
        close((*self_0).stderr_fd as ::core::ffi::c_int);
    }
    (*self_0).stderr_fd = -(1 as ::core::ffi::c_int) as gint;
    if !(*self_0).source_fds.is_null() {
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if !(*self_0).target_fds.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocesslauncher.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                667 as ::core::ffi::c_int,
                G_STRFUNC,
                b"self->target_fds != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
            if (*(*self_0).source_fds).len == (*(*self_0).target_fds).len {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsubprocesslauncher.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                668 as ::core::ffi::c_int,
                G_STRFUNC,
                b"self->source_fds->len == self->target_fds->len\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        i = 0 as guint;
        while i < (*(*self_0).source_fds).len {
            close(
                *((*(*self_0).source_fds).data as *mut ::core::ffi::c_void
                    as *mut ::core::ffi::c_int)
                    .offset(i as isize),
            );
            i = i.wrapping_add(1);
        }
        let mut _pp: *mut *mut GArray = &raw mut (*self_0).source_fds;
        let mut _ptr: *mut GArray = *_pp;
        *_pp = ::core::ptr::null_mut::<GArray>();
        if !_ptr.is_null() {
            g_array_unref(_ptr as *mut GArray);
        }
        let mut _pp_0: *mut *mut GArray = &raw mut (*self_0).target_fds;
        let mut _ptr_0: *mut GArray = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<GArray>();
        if !_ptr_0.is_null() {
            g_array_unref(_ptr_0 as *mut GArray);
        }
    }
    (*self_0).closed_fd = TRUE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_set_child_setup(
    mut self_0: *mut GSubprocessLauncher,
    mut child_setup: GSpawnChildSetupFunc,
    mut user_data: gpointer,
    mut destroy_notify: GDestroyNotify,
) {
    if (*self_0).child_setup_destroy_notify.is_some() {
        Some(
            (*self_0)
                .child_setup_destroy_notify
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")((*self_0).child_setup_user_data);
    }
    (*self_0).child_setup_func = child_setup;
    (*self_0).child_setup_user_data = user_data;
    (*self_0).child_setup_destroy_notify = destroy_notify;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_spawn(
    mut launcher: *mut GSubprocessLauncher,
    mut error: *mut *mut GError,
    mut argv0: *const gchar,
    mut args: ...
) -> *mut GSubprocess {
    let mut result: *mut GSubprocess = ::core::ptr::null_mut::<GSubprocess>();
    let mut args_0: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut arg: *const gchar = ::core::ptr::null::<gchar>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !argv0.is_null()
            && *argv0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
        {
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
            b"argv0 != NULL && argv0[0] != '\\0'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSubprocess>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSubprocess>();
    }
    args_0 = g_ptr_array_new();
    ap = args.clone();
    g_ptr_array_add(args_0, argv0 as *mut gchar as gpointer);
    loop {
        arg = ap.arg::<*const gchar>();
        if arg.is_null() {
            break;
        }
        g_ptr_array_add(args_0, arg as *mut gchar as gpointer);
    }
    g_ptr_array_add(args_0, NULL_0);
    result = safe_c2rust_g_subprocess_launcher_spawnv(
        launcher,
        (*args_0).pdata as *const *const gchar,
        error,
    );
    g_ptr_array_free(args_0, TRUE);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_subprocess_launcher_spawnv(
    mut launcher: *mut GSubprocessLauncher,
    mut argv: *const *const gchar,
    mut error: *mut *mut GError,
) -> *mut GSubprocess {
    let mut subprocess: *mut GSubprocess = ::core::ptr::null_mut::<GSubprocess>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !argv.is_null()
            && !(*argv.offset(0 as ::core::ffi::c_int as isize)).is_null()
            && *(*argv.offset(0 as ::core::ffi::c_int as isize))
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '\0' as i32
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
            b"argv != NULL && argv[0] != NULL && argv[0][0] != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSubprocess>();
    }
    if (*launcher).closed_fd != 0 {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            b"Can't spawn a new child because a passed file descriptor has been closed.\0"
                as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<GSubprocess>();
    }
    subprocess = g_object_new(
        g_subprocess_get_type(),
        b"argv\0" as *const u8 as *const gchar,
        argv,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        (*launcher).flags as ::core::ffi::c_uint,
        NULL_0,
    ) as *mut GSubprocess;
    g_subprocess_set_launcher(subprocess, launcher);
    if g_initable_init(
        subprocess as *mut ::core::ffi::c_void as *mut GInitable,
        ::core::ptr::null_mut::<GCancellable>(),
        error,
    ) == 0
    {
        g_object_unref(subprocess as gpointer);
        return ::core::ptr::null_mut::<GSubprocess>();
    }
    return subprocess;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
