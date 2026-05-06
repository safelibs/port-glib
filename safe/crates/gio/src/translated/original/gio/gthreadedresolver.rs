use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GVariant;
    pub type _GWakeup;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInetAddressPrivate;
    pub type _GInetSocketAddressPrivate;
    pub type _GResolverPrivate;
    pub type _GTask;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_byte_array_new() -> *mut GByteArray;
    fn g_byte_array_free(array: *mut GByteArray, free_segment: gboolean) -> *mut guint8;
    fn g_byte_array_set_size(array: *mut GByteArray, length: guint) -> *mut GByteArray;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_clear_error(err: *mut *mut GError);
    fn g_propagate_prefixed_error(
        dest: *mut *mut GError,
        src: *mut GError,
        format: *const gchar,
        ...
    );
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_init(cond: *mut GCond);
    fn g_cond_clear(cond: *mut GCond);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_broadcast(cond: *mut GCond);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_locale_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_reverse(list: *mut GList) -> *mut GList;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_timeout_source_new(interval: guint) -> *mut GSource;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_new_strv(strv: *const *const gchar, length: gssize) -> *mut GVariant;
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
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
    fn g_thread_pool_new_full(
        func: GFunc,
        user_data: gpointer,
        item_free_func: GDestroyNotify,
        max_threads: gint,
        exclusive: gboolean,
        error: *mut *mut GError,
    ) -> *mut GThreadPool;
    fn g_thread_pool_free(pool: *mut GThreadPool, immediate: gboolean, wait_: gboolean);
    fn g_thread_pool_push(
        pool: *mut GThreadPool,
        data: gpointer,
        error: *mut *mut GError,
    ) -> gboolean;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn glib__private__() -> *const GLibPrivateVTable;
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
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_weak_ref_clear(weak_ref: *mut GWeakRef);
    fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> gpointer;
    fn g_weak_ref_set(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_cancellable_is_cancelled(cancellable: *mut GCancellable) -> gboolean;
    fn g_cancellable_source_new(cancellable: *mut GCancellable) -> *mut GSource;
    fn g_io_error_quark() -> GQuark;
    fn g_inet_address_to_string(address: *mut GInetAddress) -> *mut gchar;
    fn g_socket_address_new_from_native(native: gpointer, len: gsize) -> *mut GSocketAddress;
    fn g_socket_address_to_native(
        address: *mut GSocketAddress,
        dest: gpointer,
        destlen: gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_socket_address_get_native_size(address: *mut GSocketAddress) -> gssize;
    fn g_inet_socket_address_get_type() -> GType;
    fn g_inet_socket_address_new(address: *mut GInetAddress, port: guint16) -> *mut GSocketAddress;
    fn g_inet_socket_address_get_address(address: *mut GInetSocketAddress) -> *mut GInetAddress;
    fn g_resolver_get_type() -> GType;
    fn g_resolver_free_addresses(addresses: *mut GList);
    fn g_resolver_get_timeout(resolver: *mut GResolver) -> ::core::ffi::c_uint;
    fn g_resolver_error_quark() -> GQuark;
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
    fn g_task_set_name(task: *mut GTask, name: *const gchar);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_new_error_literal(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        message: *const ::core::ffi::c_char,
    );
    fn g_task_return_error_if_cancelled(task: *mut GTask) -> gboolean;
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn __h_errno_location() -> *mut ::core::ffi::c_int;
    fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut ::core::ffi::c_char,
        __hostlen: socklen_t,
        __serv: *mut ::core::ffi::c_char,
        __servlen: socklen_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn dn_expand(
        _: *const ::core::ffi::c_uchar,
        _: *const ::core::ffi::c_uchar,
        _: *const ::core::ffi::c_uchar,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __res_ninit(_: res_state) -> ::core::ffi::c_int;
    fn res_nquery(
        _: res_state,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __res_nclose(_: res_state);
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __socklen_t = ::core::ffi::c_uint;
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
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GByteArray {
    pub data: *mut guint8,
    pub len: guint,
}
pub type GByteArray = _GByteArray;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCond {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GCond = _GCond;
pub type socklen_t = __socklen_t;
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
pub struct _GSource {
    pub callback_data: gpointer,
    pub callback_funcs: *mut GSourceCallbackFuncs,
    pub source_funcs: *const GSourceFuncs,
    pub ref_count: guint,
    pub context: *mut GMainContext,
    pub priority: gint,
    pub flags: guint,
    pub source_id: guint,
    pub poll_fds: *mut GSList,
    pub prev: *mut GSource,
    pub next: *mut GSource,
    pub name: *mut ::core::ffi::c_char,
    pub priv_0: *mut GSourcePrivate,
}
pub type GSourcePrivate = _GSourcePrivate;
pub type GSource = _GSource;
pub type GSourceFuncs = _GSourceFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceFuncs {
    pub prepare: Option<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>,
    pub check: Option<unsafe extern "C" fn(*mut GSource) -> gboolean>,
    pub dispatch: Option<unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean>,
    pub finalize: Option<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub closure_callback: GSourceFunc,
    pub closure_marshal: GSourceDummyMarshal,
}
pub type GSourceDummyMarshal = Option<unsafe extern "C" fn() -> ()>;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallbackFuncs {
    pub ref_0: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub unref: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub get:
        Option<unsafe extern "C" fn(gpointer, *mut GSource, *mut GSourceFunc, *mut gpointer) -> ()>,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThreadPool {
    pub func: GFunc,
    pub user_data: gpointer,
    pub exclusive: gboolean,
}
pub type GThreadPool = _GThreadPool;
pub type GWakeup = _GWakeup;
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
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
pub struct GWeakRef {
    pub priv_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub p: gpointer,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_1 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_1 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_1 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_1 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_1 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_1 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_1 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_1 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_1 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_1 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_1 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_1 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_1 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_1 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_1 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_1 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_1 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_1 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_1 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_1 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_1 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_1 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_1 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_1 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_1 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_1 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_1 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_1 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_1 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_1 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_1 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_1 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_1 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_1 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_1 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_1 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_1 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_1 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_1 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_1 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_1 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_1 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_1 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_1 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_1 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_1 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_1 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_1 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_1 = 0;
pub type GResolverError = ::core::ffi::c_uint;
pub const G_RESOLVER_ERROR_INTERNAL: GResolverError = 2;
pub const G_RESOLVER_ERROR_TEMPORARY_FAILURE: GResolverError = 1;
pub const G_RESOLVER_ERROR_NOT_FOUND: GResolverError = 0;
pub type GResolverRecordType = ::core::ffi::c_uint;
pub const G_RESOLVER_RECORD_NS: GResolverRecordType = 5;
pub const G_RESOLVER_RECORD_SOA: GResolverRecordType = 4;
pub const G_RESOLVER_RECORD_TXT: GResolverRecordType = 3;
pub const G_RESOLVER_RECORD_MX: GResolverRecordType = 2;
pub const G_RESOLVER_RECORD_SRV: GResolverRecordType = 1;
pub type GAsyncResult = _GAsyncResult;
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
pub struct _GInetAddress {
    pub parent_instance: GObject,
    pub priv_0: *mut GInetAddressPrivate,
}
pub type GInetAddressPrivate = _GInetAddressPrivate;
pub type GInetAddress = _GInetAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInetSocketAddress {
    pub parent_instance: GSocketAddress,
    pub priv_0: *mut GInetSocketAddressPrivate,
}
pub type GInetSocketAddressPrivate = _GInetSocketAddressPrivate;
pub type GSocketAddress = _GSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketAddress {
    pub parent_instance: GObject,
}
pub type GInetSocketAddress = _GInetSocketAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResolver {
    pub parent_instance: GObject,
    pub priv_0: *mut GResolverPrivate,
}
pub type GResolverPrivate = _GResolverPrivate;
pub type GResolver = _GResolver;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResolverClass {
    pub parent_class: GObjectClass,
    pub reload: Option<unsafe extern "C" fn(*mut GResolver) -> ()>,
    pub lookup_by_name: Option<
        unsafe extern "C" fn(
            *mut GResolver,
            *const gchar,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GList,
    >,
    pub lookup_by_name_async: Option<
        unsafe extern "C" fn(
            *mut GResolver,
            *const gchar,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_by_name_finish: Option<
        unsafe extern "C" fn(*mut GResolver, *mut GAsyncResult, *mut *mut GError) -> *mut GList,
    >,
    pub lookup_by_address: Option<
        unsafe extern "C" fn(
            *mut GResolver,
            *mut GInetAddress,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut gchar,
    >,
    pub lookup_by_address_async: Option<
        unsafe extern "C" fn(
            *mut GResolver,
            *mut GInetAddress,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_by_address_finish: Option<
        unsafe extern "C" fn(*mut GResolver, *mut GAsyncResult, *mut *mut GError) -> *mut gchar,
    >,
    pub lookup_service: Option<
        unsafe extern "C" fn(
            *mut GResolver,
            *const gchar,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GList,
    >,
    pub lookup_service_async: Option<
        unsafe extern "C" fn(
            *mut GResolver,
            *const gchar,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_service_finish: Option<
        unsafe extern "C" fn(*mut GResolver, *mut GAsyncResult, *mut *mut GError) -> *mut GList,
    >,
    pub lookup_records: Option<
        unsafe extern "C" fn(
            *mut GResolver,
            *const gchar,
            GResolverRecordType,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GList,
    >,
    pub lookup_records_async: Option<
        unsafe extern "C" fn(
            *mut GResolver,
            *const gchar,
            GResolverRecordType,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_records_finish: Option<
        unsafe extern "C" fn(*mut GResolver, *mut GAsyncResult, *mut *mut GError) -> *mut GList,
    >,
    pub lookup_by_name_with_flags_async: Option<
        unsafe extern "C" fn(
            *mut GResolver,
            *const gchar,
            GResolverNameLookupFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub lookup_by_name_with_flags_finish: Option<
        unsafe extern "C" fn(*mut GResolver, *mut GAsyncResult, *mut *mut GError) -> *mut GList,
    >,
    pub lookup_by_name_with_flags: Option<
        unsafe extern "C" fn(
            *mut GResolver,
            *const gchar,
            GResolverNameLookupFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GList,
    >,
}
pub type GResolverNameLookupFlags = ::core::ffi::c_uint;
pub const G_RESOLVER_NAME_LOOKUP_FLAGS_IPV6_ONLY: GResolverNameLookupFlags = 2;
pub const G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY: GResolverNameLookupFlags = 1;
pub const G_RESOLVER_NAME_LOOKUP_FLAGS_DEFAULT: GResolverNameLookupFlags = 0;
pub type GResolverClass = _GResolverClass;
pub type GThreadedResolver = _GThreadedResolver;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThreadedResolver {
    pub parent_instance: GResolver,
    pub thread_pool: *mut GThreadPool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookupData {
    pub lookup_type: C2RustUnnamed_7,
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub cond: GCond,
    pub lock: GMutex,
    pub timeout_source: *mut GSource,
    pub cancellable_source: *mut GSource,
    pub will_return: C2RustUnnamed_2,
    pub has_returned: gboolean,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const CANCELLED: C2RustUnnamed_2 = 3;
pub const TIMED_OUT: C2RustUnnamed_2 = 2;
pub const COMPLETED: C2RustUnnamed_2 = 1;
pub const NOT_YET: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub lookup_by_name: C2RustUnnamed_6,
    pub lookup_by_address: C2RustUnnamed_5,
    pub lookup_records: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub rrname: *mut ::core::ffi::c_char,
    pub record_type: GResolverRecordType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub address: *mut GInetAddress,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub hostname: *mut ::core::ffi::c_char,
    pub address_family: ::core::ffi::c_int,
}
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const LOOKUP_RECORDS: C2RustUnnamed_7 = 2;
pub const LOOKUP_BY_ADDRESS: C2RustUnnamed_7 = 1;
pub const LOOKUP_BY_NAME: C2RustUnnamed_7 = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __res_state {
    pub retrans: ::core::ffi::c_int,
    pub retry: ::core::ffi::c_int,
    pub options: ::core::ffi::c_ulong,
    pub nscount: ::core::ffi::c_int,
    pub nsaddr_list: [sockaddr_in; 3],
    pub id: ::core::ffi::c_ushort,
    pub dnsrch: [*mut ::core::ffi::c_char; 7],
    pub defdname: [::core::ffi::c_char; 256],
    pub pfcode: ::core::ffi::c_ulong,
    #[bitfield(name = "ndots", ty = "::core::ffi::c_uint", bits = "0..=3")]
    #[bitfield(name = "nsort", ty = "::core::ffi::c_uint", bits = "4..=7")]
    #[bitfield(name = "ipv6_unavail", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "unused", ty = "::core::ffi::c_uint", bits = "9..=31")]
    pub ndots_nsort_ipv6_unavail_unused: [u8; 4],
    pub sort_list: [C2RustUnnamed_11; 10],
    pub __glibc_unused_qhook: *mut ::core::ffi::c_void,
    pub __glibc_unused_rhook: *mut ::core::ffi::c_void,
    pub res_h_errno: ::core::ffi::c_int,
    pub _vcsock: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_uint,
    pub _u: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub pad: [::core::ffi::c_char; 52],
    pub _ext: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub nscount: uint16_t,
    pub nsmap: [uint16_t; 3],
    pub nssocks: [::core::ffi::c_int; 3],
    pub nscount6: uint16_t,
    pub nsinit: uint16_t,
    pub nsaddrs: [*mut sockaddr_in6; 3],
    pub __glibc_reserved: [::core::ffi::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub addr: in_addr,
    pub mask: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
pub type res_state = *mut __res_state;
pub const ns_t_txt: __ns_type = 16;
pub const ns_t_ns: __ns_type = 2;
pub const ns_t_soa: __ns_type = 6;
pub const ns_t_mx: __ns_type = 15;
pub const ns_t_srv: __ns_type = 33;
pub const ns_c_in: __ns_class = 1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct HEADER {
    #[bitfield(name = "id", ty = "::core::ffi::c_uint", bits = "0..=15")]
    #[bitfield(name = "rd", ty = "::core::ffi::c_uint", bits = "16..=16")]
    #[bitfield(name = "tc", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(name = "aa", ty = "::core::ffi::c_uint", bits = "18..=18")]
    #[bitfield(name = "opcode", ty = "::core::ffi::c_uint", bits = "19..=22")]
    #[bitfield(name = "qr", ty = "::core::ffi::c_uint", bits = "23..=23")]
    #[bitfield(name = "rcode", ty = "::core::ffi::c_uint", bits = "24..=27")]
    #[bitfield(name = "cd", ty = "::core::ffi::c_uint", bits = "28..=28")]
    #[bitfield(name = "ad", ty = "::core::ffi::c_uint", bits = "29..=29")]
    #[bitfield(name = "unused", ty = "::core::ffi::c_uint", bits = "30..=30")]
    #[bitfield(name = "ra", ty = "::core::ffi::c_uint", bits = "31..=31")]
    #[bitfield(name = "qdcount", ty = "::core::ffi::c_uint", bits = "32..=47")]
    #[bitfield(name = "ancount", ty = "::core::ffi::c_uint", bits = "48..=63")]
    #[bitfield(name = "nscount", ty = "::core::ffi::c_uint", bits = "64..=79")]
    #[bitfield(name = "arcount", ty = "::core::ffi::c_uint", bits = "80..=95")]
    pub id_rd_tc_aa_opcode_qr_rcode_cd_ad_unused_ra_qdcount_ancount_nscount_arcount: [u8; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: ::core::ffi::c_int,
    pub ai_family: ::core::ffi::c_int,
    pub ai_socktype: ::core::ffi::c_int,
    pub ai_protocol: ::core::ffi::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut ::core::ffi::c_char,
    pub ai_next: *mut addrinfo,
}
pub const IPPROTO_TCP: C2RustUnnamed_12 = 6;
pub const SOCK_STREAM: __socket_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GThreadedResolverClass {
    pub parent_class: GResolverClass,
}
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_12 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_12 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_12 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_12 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_12 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_12 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_12 = 132;
pub const IPPROTO_L2TP: C2RustUnnamed_12 = 115;
pub const IPPROTO_COMP: C2RustUnnamed_12 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_12 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_12 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_12 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_12 = 92;
pub const IPPROTO_AH: C2RustUnnamed_12 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_12 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_12 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_12 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_12 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_12 = 33;
pub const IPPROTO_TP: C2RustUnnamed_12 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_12 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_12 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_12 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_12 = 8;
pub const IPPROTO_IPIP: C2RustUnnamed_12 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_12 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_12 = 1;
pub const IPPROTO_IP: C2RustUnnamed_12 = 0;
pub type __ns_type = ::core::ffi::c_uint;
pub const ns_t_max: __ns_type = 65536;
pub const ns_t_dlv: __ns_type = 32769;
pub const ns_t_ta: __ns_type = 32768;
pub const ns_t_avc: __ns_type = 258;
pub const ns_t_caa: __ns_type = 257;
pub const ns_t_uri: __ns_type = 256;
pub const ns_t_any: __ns_type = 255;
pub const ns_t_maila: __ns_type = 254;
pub const ns_t_mailb: __ns_type = 253;
pub const ns_t_axfr: __ns_type = 252;
pub const ns_t_ixfr: __ns_type = 251;
pub const ns_t_tsig: __ns_type = 250;
pub const ns_t_tkey: __ns_type = 249;
pub const ns_t_eui64: __ns_type = 109;
pub const ns_t_eui48: __ns_type = 108;
pub const ns_t_lp: __ns_type = 107;
pub const ns_t_l64: __ns_type = 106;
pub const ns_t_l32: __ns_type = 105;
pub const ns_t_nid: __ns_type = 104;
pub const ns_t_unspec: __ns_type = 103;
pub const ns_t_gid: __ns_type = 102;
pub const ns_t_uid: __ns_type = 101;
pub const ns_t_uinfo: __ns_type = 100;
pub const ns_t_spf: __ns_type = 99;
pub const ns_t_csync: __ns_type = 62;
pub const ns_t_openpgpkey: __ns_type = 61;
pub const ns_t_cdnskey: __ns_type = 60;
pub const ns_t_cds: __ns_type = 59;
pub const ns_t_talink: __ns_type = 58;
pub const ns_t_rkey: __ns_type = 57;
pub const ns_t_ninfo: __ns_type = 56;
pub const ns_t_hip: __ns_type = 55;
pub const ns_t_smimea: __ns_type = 53;
pub const ns_t_tlsa: __ns_type = 52;
pub const ns_t_nsec3param: __ns_type = 51;
pub const ns_t_nsec3: __ns_type = 50;
pub const ns_t_dhcid: __ns_type = 49;
pub const ns_t_dnskey: __ns_type = 48;
pub const ns_t_nsec: __ns_type = 47;
pub const ns_t_rrsig: __ns_type = 46;
pub const ns_t_ipseckey: __ns_type = 45;
pub const ns_t_sshfp: __ns_type = 44;
pub const ns_t_ds: __ns_type = 43;
pub const ns_t_apl: __ns_type = 42;
pub const ns_t_opt: __ns_type = 41;
pub const ns_t_sink: __ns_type = 40;
pub const ns_t_dname: __ns_type = 39;
pub const ns_t_a6: __ns_type = 38;
pub const ns_t_cert: __ns_type = 37;
pub const ns_t_kx: __ns_type = 36;
pub const ns_t_naptr: __ns_type = 35;
pub const ns_t_atma: __ns_type = 34;
pub const ns_t_nimloc: __ns_type = 32;
pub const ns_t_eid: __ns_type = 31;
pub const ns_t_nxt: __ns_type = 30;
pub const ns_t_loc: __ns_type = 29;
pub const ns_t_aaaa: __ns_type = 28;
pub const ns_t_gpos: __ns_type = 27;
pub const ns_t_px: __ns_type = 26;
pub const ns_t_key: __ns_type = 25;
pub const ns_t_sig: __ns_type = 24;
pub const ns_t_nsap_ptr: __ns_type = 23;
pub const ns_t_nsap: __ns_type = 22;
pub const ns_t_rt: __ns_type = 21;
pub const ns_t_isdn: __ns_type = 20;
pub const ns_t_x25: __ns_type = 19;
pub const ns_t_afsdb: __ns_type = 18;
pub const ns_t_rp: __ns_type = 17;
pub const ns_t_minfo: __ns_type = 14;
pub const ns_t_hinfo: __ns_type = 13;
pub const ns_t_ptr: __ns_type = 12;
pub const ns_t_wks: __ns_type = 11;
pub const ns_t_null: __ns_type = 10;
pub const ns_t_mr: __ns_type = 9;
pub const ns_t_mg: __ns_type = 8;
pub const ns_t_mb: __ns_type = 7;
pub const ns_t_cname: __ns_type = 5;
pub const ns_t_mf: __ns_type = 4;
pub const ns_t_md: __ns_type = 3;
pub const ns_t_a: __ns_type = 1;
pub const ns_t_invalid: __ns_type = 0;
pub type __ns_class = ::core::ffi::c_uint;
pub const ns_c_max: __ns_class = 65536;
pub const ns_c_any: __ns_class = 255;
pub const ns_c_none: __ns_class = 254;
pub const ns_c_hs: __ns_class = 4;
pub const ns_c_chaos: __ns_class = 3;
pub const ns_c_2: __ns_class = 2;
pub const ns_c_invalid: __ns_class = 0;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust___bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
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
#[inline]
unsafe extern "C" fn safe_c2rust_G_THREADED_RESOLVER(mut ptr: gpointer) -> *mut GThreadedResolver {
    return ptr as *mut GThreadedResolver;
}
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const HOST_NOT_FOUND: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TRY_AGAIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NO_DATA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const AI_ADDRCONFIG: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const EAI_NONAME: ::core::ffi::c_int = -2;
pub const EAI_AGAIN: ::core::ffi::c_int = -3;
pub const EAI_FAIL: ::core::ffi::c_int = -4;
pub const EAI_NODATA: ::core::ffi::c_int = -5;
pub const NI_NAMEREQD: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const NS_INT32SZ: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NS_INT16SZ: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut safe_c2rust_g_threaded_resolver_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_threaded_resolver_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_resolver_get_type(),
        g_intern_static_string(b"GThreadedResolver\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GThreadedResolverClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_threaded_resolver_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GThreadedResolver>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GThreadedResolver) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_threaded_resolver_init
                    as unsafe extern "C" fn(*mut GThreadedResolver) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_threaded_resolver_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_threaded_resolver_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GThreadedResolver_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GThreadedResolver_private_offset,
        );
    }
    safe_c2rust_g_threaded_resolver_class_init(klass as *mut GThreadedResolverClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_threaded_resolver_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_threaded_resolver_get_type_once();
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
static mut safe_c2rust_GThreadedResolver_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_threaded_resolver_init(mut self_0: *mut GThreadedResolver) {
    (*self_0).thread_pool = g_thread_pool_new_full(
        Some(
            safe_c2rust_threaded_resolver_worker_cb
                as unsafe extern "C" fn(gpointer, gpointer) -> (),
        ),
        self_0 as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        ),
        20 as gint,
        FALSE,
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
unsafe extern "C" fn safe_c2rust_g_threaded_resolver_finalize(mut object: *mut GObject) {
    let mut self_0: *mut GThreadedResolver = safe_c2rust_G_THREADED_RESOLVER(object as gpointer);
    g_thread_pool_free((*self_0).thread_pool, TRUE, FALSE);
    (*self_0).thread_pool = ::core::ptr::null_mut::<GThreadPool>();
    (*(safe_c2rust_g_threaded_resolver_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_resolver_error_from_addrinfo_error(
    mut err: gint,
) -> GResolverError {
    match err {
        EAI_FAIL | EAI_NODATA | EAI_NONAME => return G_RESOLVER_ERROR_NOT_FOUND,
        EAI_AGAIN => return G_RESOLVER_ERROR_TEMPORARY_FAILURE,
        _ => return G_RESOLVER_ERROR_INTERNAL,
    };
}
unsafe extern "C" fn safe_c2rust_lookup_data_new_by_name(
    mut hostname: *const ::core::ffi::c_char,
    mut address_family: ::core::ffi::c_int,
) -> *mut LookupData {
    let mut data: *mut LookupData = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<LookupData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut LookupData;
    (*data).lookup_type = LOOKUP_BY_NAME;
    g_cond_init(&raw mut (*data).cond);
    g_mutex_init(&raw mut (*data).lock);
    (*data).c2rust_unnamed.lookup_by_name.hostname = safe_c2rust_g_strdup_inline(hostname);
    (*data).c2rust_unnamed.lookup_by_name.address_family = address_family;
    return safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LookupData;
}
unsafe extern "C" fn safe_c2rust_lookup_data_new_by_address(
    mut address: *mut GInetAddress,
) -> *mut LookupData {
    let mut data: *mut LookupData = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<LookupData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut LookupData;
    (*data).lookup_type = LOOKUP_BY_ADDRESS;
    g_cond_init(&raw mut (*data).cond);
    g_mutex_init(&raw mut (*data).lock);
    (*data).c2rust_unnamed.lookup_by_address.address =
        g_object_ref(address as gpointer) as *mut GInetAddress as *mut GInetAddress;
    return safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LookupData;
}
unsafe extern "C" fn safe_c2rust_lookup_data_new_records(
    mut rrname: *const gchar,
    mut record_type: GResolverRecordType,
) -> *mut LookupData {
    let mut data: *mut LookupData = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<LookupData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut LookupData;
    (*data).lookup_type = LOOKUP_RECORDS;
    g_cond_init(&raw mut (*data).cond);
    g_mutex_init(&raw mut (*data).lock);
    (*data).c2rust_unnamed.lookup_records.rrname =
        safe_c2rust_g_strdup_inline(rrname as *const ::core::ffi::c_char);
    (*data).c2rust_unnamed.lookup_records.record_type = record_type;
    return safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LookupData;
}
unsafe extern "C" fn safe_c2rust_lookup_data_free(mut data: *mut LookupData) {
    match (*data).lookup_type as ::core::ffi::c_uint {
        0 => {
            g_free((*data).c2rust_unnamed.lookup_by_name.hostname as gpointer);
        }
        1 => {
            let mut _pp: *mut *mut GInetAddress =
                &raw mut (*data).c2rust_unnamed.lookup_by_address.address;
            let mut _ptr: *mut GInetAddress = *_pp;
            *_pp = ::core::ptr::null_mut::<GInetAddress>();
            if !_ptr.is_null() {
                g_object_unref(_ptr as gpointer);
            }
        }
        2 => {
            g_free((*data).c2rust_unnamed.lookup_records.rrname as gpointer);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gthreadedresolver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                237 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    if !(*data).timeout_source.is_null() {
        g_source_destroy((*data).timeout_source);
        let mut _pp_0: *mut *mut GSource = &raw mut (*data).timeout_source;
        let mut _ptr_0: *mut GSource = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<GSource>();
        if !_ptr_0.is_null() {
            g_source_unref(_ptr_0 as *mut GSource);
        }
    }
    if !(*data).cancellable_source.is_null() {
        g_source_destroy((*data).cancellable_source);
        let mut _pp_1: *mut *mut GSource = &raw mut (*data).cancellable_source;
        let mut _ptr_1: *mut GSource = *_pp_1;
        *_pp_1 = ::core::ptr::null_mut::<GSource>();
        if !_ptr_1.is_null() {
            g_source_unref(_ptr_1 as *mut GSource);
        }
    }
    g_mutex_clear(&raw mut (*data).lock);
    g_cond_clear(&raw mut (*data).cond);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_do_lookup_by_name(
    mut hostname: *const gchar,
    mut address_family: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut res: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    let mut addresses: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut retval: gint = 0;
    let mut addrinfo_hints: addrinfo = addrinfo {
        ai_flags: 0 as ::core::ffi::c_int,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: ::core::ptr::null_mut::<sockaddr>(),
        ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ai_next: ::core::ptr::null_mut::<addrinfo>(),
    };
    addrinfo_hints.ai_flags = AI_ADDRCONFIG;
    addrinfo_hints.ai_socktype = SOCK_STREAM as ::core::ffi::c_int;
    addrinfo_hints.ai_protocol = IPPROTO_TCP as ::core::ffi::c_int;
    addrinfo_hints.ai_family = address_family;
    retval = getaddrinfo(
        hostname as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut addrinfo_hints,
        &raw mut res,
    ) as gint;
    if retval == 0 as ::core::ffi::c_int {
        let mut ai: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        let mut sockaddr: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
        let mut addr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
        addresses = ::core::ptr::null_mut::<GList>();
        ai = res;
        while !ai.is_null() {
            sockaddr = g_socket_address_new_from_native(
                (*ai).ai_addr as gpointer,
                (*ai).ai_addrlen as gsize,
            );
            if !sockaddr.is_null() {
                if ({
                    let mut __inst: *mut GTypeInstance = sockaddr as *mut GTypeInstance;
                    let mut __t: GType = g_inet_socket_address_get_type();
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
                    let mut _pp: *mut *mut GSocketAddress = &raw mut sockaddr;
                    let mut _ptr: *mut GSocketAddress = *_pp;
                    *_pp = ::core::ptr::null_mut::<GSocketAddress>();
                    if !_ptr.is_null() {
                        g_object_unref(_ptr as gpointer);
                    }
                } else {
                    addr = g_object_ref(g_inet_socket_address_get_address(
                        sockaddr as *mut GInetSocketAddress,
                    ) as gpointer) as *mut GInetAddress
                        as *mut GInetAddress;
                    addresses = g_list_prepend(addresses, addr as gpointer);
                    g_object_unref(sockaddr as gpointer);
                }
            }
            ai = (*ai).ai_next;
        }
        let mut _pp_0: *mut *mut addrinfo = &raw mut res;
        let mut _ptr_0: *mut addrinfo = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<addrinfo>();
        if !_ptr_0.is_null() {
            freeaddrinfo(_ptr_0 as *mut addrinfo);
        }
        if !addresses.is_null() {
            addresses = g_list_reverse(addresses);
            return safe_c2rust_g_steal_pointer(&raw mut addresses as gpointer) as *mut GList;
        } else {
            g_set_error(
                error,
                g_resolver_error_quark(),
                G_RESOLVER_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Error resolving \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                        as *const gchar,
                ),
                hostname,
                glib_gettext(b"No valid addresses were found\0" as *const u8 as *const gchar),
            );
            return ::core::ptr::null_mut::<GList>();
        }
    } else {
        let mut error_message: *mut gchar = g_locale_to_utf8(
            gai_strerror(retval as ::core::ffi::c_int) as *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if error_message.is_null() {
            error_message = safe_c2rust_g_strdup_inline(
                b"[Invalid UTF-8]\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut gchar;
        }
        let mut _pp_1: *mut *mut addrinfo = &raw mut res;
        let mut _ptr_1: *mut addrinfo = *_pp_1;
        *_pp_1 = ::core::ptr::null_mut::<addrinfo>();
        if !_ptr_1.is_null() {
            freeaddrinfo(_ptr_1 as *mut addrinfo);
        }
        g_set_error(
            error,
            g_resolver_error_quark(),
            safe_c2rust_g_resolver_error_from_addrinfo_error(retval) as gint,
            glib_gettext(
                b"Error resolving \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8 as *const gchar,
            ),
            hostname,
            error_message,
        );
        g_free(error_message as gpointer);
        return ::core::ptr::null_mut::<GList>();
    };
}
unsafe extern "C" fn safe_c2rust_lookup_by_name(
    mut resolver: *mut GResolver,
    mut hostname: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut self_0: *mut GThreadedResolver = safe_c2rust_G_THREADED_RESOLVER(resolver as gpointer);
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut addresses: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut data: *mut LookupData = ::core::ptr::null_mut::<LookupData>();
    data = safe_c2rust_lookup_data_new_by_name(hostname as *const ::core::ffi::c_char, AF_UNSPEC);
    task = g_task_new(resolver as gpointer, cancellable, None, NULL_0);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GResolver,
                    *const gchar,
                    *mut GCancellable,
                    *mut *mut GError,
                ) -> *mut GList,
            >,
            gpointer,
        >(Some(
            safe_c2rust_lookup_by_name
                as unsafe extern "C" fn(
                    *mut GResolver,
                    *const gchar,
                    *mut GCancellable,
                    *mut *mut GError,
                ) -> *mut GList,
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(_task, b"lookup_by_name\0" as *const u8 as *const gchar);
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] resolver lookup\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] resolver lookup\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LookupData as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut LookupData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_lookup_data_free as unsafe extern "C" fn(*mut LookupData) -> ()),
        ),
    );
    safe_c2rust_run_task_in_thread_pool_sync(self_0, task);
    addresses = g_task_propagate_pointer(task, error) as *mut GList;
    g_object_unref(task as gpointer);
    return addresses;
}
unsafe extern "C" fn safe_c2rust_flags_to_family(
    mut flags: GResolverNameLookupFlags,
) -> ::core::ffi::c_int {
    let mut address_family: ::core::ffi::c_int = AF_UNSPEC;
    if flags as ::core::ffi::c_uint
        & G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        address_family = AF_INET;
    }
    if flags as ::core::ffi::c_uint
        & G_RESOLVER_NAME_LOOKUP_FLAGS_IPV6_ONLY as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        address_family = AF_INET6;
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if flags as ::core::ffi::c_uint
                & G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                == 0
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
                b"!(flags & G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return address_family;
        }
    }
    return address_family;
}
unsafe extern "C" fn safe_c2rust_lookup_by_name_with_flags(
    mut resolver: *mut GResolver,
    mut hostname: *const gchar,
    mut flags: GResolverNameLookupFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut self_0: *mut GThreadedResolver = safe_c2rust_G_THREADED_RESOLVER(resolver as gpointer);
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut addresses: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut data: *mut LookupData = ::core::ptr::null_mut::<LookupData>();
    data = safe_c2rust_lookup_data_new_by_name(
        hostname as *const ::core::ffi::c_char,
        safe_c2rust_flags_to_family(flags),
    );
    task = g_task_new(resolver as gpointer, cancellable, None, NULL_0);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GResolver,
                    *const gchar,
                    GResolverNameLookupFlags,
                    *mut GCancellable,
                    *mut *mut GError,
                ) -> *mut GList,
            >,
            gpointer,
        >(Some(
            safe_c2rust_lookup_by_name_with_flags
                as unsafe extern "C" fn(
                    *mut GResolver,
                    *const gchar,
                    GResolverNameLookupFlags,
                    *mut GCancellable,
                    *mut *mut GError,
                ) -> *mut GList,
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"lookup_by_name_with_flags\0" as *const u8 as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] resolver lookup\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] resolver lookup\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LookupData as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut LookupData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_lookup_data_free as unsafe extern "C" fn(*mut LookupData) -> ()),
        ),
    );
    safe_c2rust_run_task_in_thread_pool_sync(self_0, task);
    addresses = g_task_propagate_pointer(task, error) as *mut GList;
    g_object_unref(task as gpointer);
    return addresses;
}
unsafe extern "C" fn safe_c2rust_lookup_by_name_with_flags_async(
    mut resolver: *mut GResolver,
    mut hostname: *const gchar,
    mut flags: GResolverNameLookupFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut self_0: *mut GThreadedResolver = safe_c2rust_G_THREADED_RESOLVER(resolver as gpointer);
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut LookupData = ::core::ptr::null_mut::<LookupData>();
    data = safe_c2rust_lookup_data_new_by_name(
        hostname as *const ::core::ffi::c_char,
        safe_c2rust_flags_to_family(flags),
    );
    task = g_task_new(resolver as gpointer, cancellable, callback, user_data);
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_DEBUG,
        b"%s: starting new lookup for %s with GTask %p, LookupData %p\0" as *const u8
            as *const gchar,
        b"lookup_by_name_with_flags_async\0" as *const u8 as *const ::core::ffi::c_char,
        hostname,
        task,
        data,
    );
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GResolver,
                    *const gchar,
                    GResolverNameLookupFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_lookup_by_name_with_flags_async
                as unsafe extern "C" fn(
                    *mut GResolver,
                    *const gchar,
                    GResolverNameLookupFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"lookup_by_name_with_flags_async\0" as *const u8 as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] resolver lookup\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] resolver lookup\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LookupData as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut LookupData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_lookup_data_free as unsafe extern "C" fn(*mut LookupData) -> ()),
        ),
    );
    safe_c2rust_run_task_in_thread_pool_async(self_0, task);
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_lookup_by_name_async(
    mut resolver: *mut GResolver,
    mut hostname: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    safe_c2rust_lookup_by_name_with_flags_async(
        resolver,
        hostname,
        G_RESOLVER_NAME_LOOKUP_FLAGS_DEFAULT,
        cancellable,
        callback,
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_lookup_by_name_finish(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, resolver as gpointer) != 0 {
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
            b"g_task_is_valid (result, resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GList;
}
unsafe extern "C" fn safe_c2rust_lookup_by_name_with_flags_finish(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, resolver as gpointer) != 0 {
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
            b"g_task_is_valid (result, resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GList;
}
unsafe extern "C" fn safe_c2rust_do_lookup_by_address(
    mut address: *mut GInetAddress,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut sockaddr_address: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sockaddr_address_size: gsize = 0;
    let mut gsockaddr: *mut GSocketAddress = ::core::ptr::null_mut::<GSocketAddress>();
    let mut name: [gchar; 1025] = [0; 1025];
    let mut retval: gint = 0;
    gsockaddr = g_inet_socket_address_new(address, 0 as guint16);
    g_socket_address_to_native(
        gsockaddr,
        &raw mut sockaddr_address as *mut sockaddr as gpointer,
        ::core::mem::size_of::<sockaddr_storage>() as gsize,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    sockaddr_address_size = g_socket_address_get_native_size(gsockaddr) as gsize;
    g_object_unref(gsockaddr as gpointer);
    retval = getnameinfo(
        &raw mut sockaddr_address as *mut sockaddr,
        sockaddr_address_size as socklen_t,
        &raw mut name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[gchar; 1025]>() as socklen_t,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as socklen_t,
        NI_NAMEREQD,
    ) as gint;
    if retval == 0 as ::core::ffi::c_int {
        return safe_c2rust_g_strdup_inline(&raw mut name as *mut gchar) as *mut gchar;
    } else {
        let mut phys: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut error_message: *mut gchar = g_locale_to_utf8(
            gai_strerror(retval as ::core::ffi::c_int) as *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if error_message.is_null() {
            error_message = safe_c2rust_g_strdup_inline(
                b"[Invalid UTF-8]\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut gchar;
        }
        phys = g_inet_address_to_string(address);
        g_set_error(
            error,
            g_resolver_error_quark(),
            safe_c2rust_g_resolver_error_from_addrinfo_error(retval) as gint,
            glib_gettext(
                b"Error reverse-resolving \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                    as *const gchar,
            ),
            if !phys.is_null() {
                phys as *const gchar
            } else {
                b"(unknown)\0" as *const u8 as *const gchar
            },
            error_message,
        );
        g_free(phys as gpointer);
        g_free(error_message as gpointer);
        return ::core::ptr::null_mut::<gchar>();
    };
}
unsafe extern "C" fn safe_c2rust_lookup_by_address(
    mut resolver: *mut GResolver,
    mut address: *mut GInetAddress,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut self_0: *mut GThreadedResolver = safe_c2rust_G_THREADED_RESOLVER(resolver as gpointer);
    let mut data: *mut LookupData = ::core::ptr::null_mut::<LookupData>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    data = safe_c2rust_lookup_data_new_by_address(address);
    task = g_task_new(resolver as gpointer, cancellable, None, NULL_0);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GResolver,
                    *mut GInetAddress,
                    *mut GCancellable,
                    *mut *mut GError,
                ) -> *mut gchar,
            >,
            gpointer,
        >(Some(
            safe_c2rust_lookup_by_address
                as unsafe extern "C" fn(
                    *mut GResolver,
                    *mut GInetAddress,
                    *mut GCancellable,
                    *mut *mut GError,
                ) -> *mut gchar,
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(_task, b"lookup_by_address\0" as *const u8 as *const gchar);
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] resolver lookup\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] resolver lookup\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LookupData as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut LookupData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_lookup_data_free as unsafe extern "C" fn(*mut LookupData) -> ()),
        ),
    );
    safe_c2rust_run_task_in_thread_pool_sync(self_0, task);
    name = g_task_propagate_pointer(task, error) as *mut gchar;
    g_object_unref(task as gpointer);
    return name;
}
unsafe extern "C" fn safe_c2rust_lookup_by_address_async(
    mut resolver: *mut GResolver,
    mut address: *mut GInetAddress,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut self_0: *mut GThreadedResolver = safe_c2rust_G_THREADED_RESOLVER(resolver as gpointer);
    let mut data: *mut LookupData = ::core::ptr::null_mut::<LookupData>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    data = safe_c2rust_lookup_data_new_by_address(address);
    task = g_task_new(resolver as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GResolver,
                    *mut GInetAddress,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_lookup_by_address_async
                as unsafe extern "C" fn(
                    *mut GResolver,
                    *mut GInetAddress,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"lookup_by_address_async\0" as *const u8 as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] resolver lookup\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] resolver lookup\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LookupData as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut LookupData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_lookup_data_free as unsafe extern "C" fn(*mut LookupData) -> ()),
        ),
    );
    safe_c2rust_run_task_in_thread_pool_async(self_0, task);
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_lookup_by_address_finish(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, resolver as gpointer) != 0 {
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
            b"g_task_is_valid (result, resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_expand_name(
    mut rrname: *const gchar,
    mut answer: *const guint8,
    mut end: *const guint8,
    mut p: *mut *const guint8,
    mut namebuf: *mut gchar,
    mut namebuf_len: gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut expand_result: ::core::ffi::c_int = 0;
    expand_result = dn_expand(
        answer as *const ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        *p,
        namebuf as *mut ::core::ffi::c_char,
        namebuf_len as ::core::ffi::c_int,
    );
    if expand_result < 0 as ::core::ffi::c_int
        || (end.offset_from(*p) as ::core::ffi::c_long) < expand_result as ::core::ffi::c_long
    {
        g_set_error(
            error,
            g_resolver_error_quark(),
            G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error parsing DNS %s record: malformed DNS packet\0" as *const u8 as *const gchar,
            ),
            rrname,
        );
        return FALSE;
    }
    *p = (*p).offset(expand_result as isize);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_parse_res_srv(
    mut answer: *const guint8,
    mut end: *const guint8,
    mut p: *mut *const guint8,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut namebuf: [gchar; 1024] = [0; 1024];
    let mut priority: guint16 = 0;
    let mut weight: guint16 = 0;
    let mut port: guint16 = 0;
    if (end.offset_from(*p) as ::core::ffi::c_long) < 6 as ::core::ffi::c_long {
        g_set_error(
            error,
            g_resolver_error_quark(),
            G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error parsing DNS %s record: malformed DNS packet\0" as *const u8 as *const gchar,
            ),
            b"SRV\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    let mut t_cp: *const ::core::ffi::c_uchar = *p as *const ::core::ffi::c_uchar;
    priority = ((*t_cp.offset(0 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *t_cp.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
        as guint16;
    *p = (*p).offset(NS_INT16SZ as isize);
    let mut t_cp_0: *const ::core::ffi::c_uchar = *p as *const ::core::ffi::c_uchar;
    weight = ((*t_cp_0.offset(0 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *t_cp_0.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
        as guint16;
    *p = (*p).offset(NS_INT16SZ as isize);
    let mut t_cp_1: *const ::core::ffi::c_uchar = *p as *const ::core::ffi::c_uchar;
    port = ((*t_cp_1.offset(0 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *t_cp_1.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
        as guint16;
    *p = (*p).offset(NS_INT16SZ as isize);
    if safe_c2rust_expand_name(
        b"SRV\0" as *const u8 as *const gchar,
        answer,
        end,
        p,
        &raw mut namebuf as *mut gchar,
        ::core::mem::size_of::<[gchar; 1024]>() as gsize,
        error,
    ) == 0
    {
        return ::core::ptr::null_mut::<GVariant>();
    }
    return g_variant_new(
        b"(qqqs)\0" as *const u8 as *const gchar,
        priority as ::core::ffi::c_int,
        weight as ::core::ffi::c_int,
        port as ::core::ffi::c_int,
        &raw mut namebuf as *mut gchar,
    );
}
unsafe extern "C" fn safe_c2rust_parse_res_soa(
    mut answer: *const guint8,
    mut end: *const guint8,
    mut p: *mut *const guint8,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut mnamebuf: [gchar; 1024] = [0; 1024];
    let mut rnamebuf: [gchar; 1024] = [0; 1024];
    let mut serial: guint32 = 0;
    let mut refresh: guint32 = 0;
    let mut retry: guint32 = 0;
    let mut expire: guint32 = 0;
    let mut ttl: guint32 = 0;
    if safe_c2rust_expand_name(
        b"SOA\0" as *const u8 as *const gchar,
        answer,
        end,
        p,
        &raw mut mnamebuf as *mut gchar,
        ::core::mem::size_of::<[gchar; 1024]>() as gsize,
        error,
    ) == 0
    {
        return ::core::ptr::null_mut::<GVariant>();
    }
    if safe_c2rust_expand_name(
        b"SOA\0" as *const u8 as *const gchar,
        answer,
        end,
        p,
        &raw mut rnamebuf as *mut gchar,
        ::core::mem::size_of::<[gchar; 1024]>() as gsize,
        error,
    ) == 0
    {
        return ::core::ptr::null_mut::<GVariant>();
    }
    if (end.offset_from(*p) as ::core::ffi::c_long) < 20 as ::core::ffi::c_long {
        g_set_error(
            error,
            g_resolver_error_quark(),
            G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error parsing DNS %s record: malformed DNS packet\0" as *const u8 as *const gchar,
            ),
            b"SOA\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    let mut t_cp: *const ::core::ffi::c_uchar = *p as *const ::core::ffi::c_uchar;
    serial = ((*t_cp.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*t_cp.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*t_cp.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *t_cp.offset(3 as ::core::ffi::c_int as isize) as uint32_t) as guint32;
    *p = (*p).offset(NS_INT32SZ as isize);
    let mut t_cp_0: *const ::core::ffi::c_uchar = *p as *const ::core::ffi::c_uchar;
    refresh = ((*t_cp_0.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*t_cp_0.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*t_cp_0.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *t_cp_0.offset(3 as ::core::ffi::c_int as isize) as uint32_t) as guint32;
    *p = (*p).offset(NS_INT32SZ as isize);
    let mut t_cp_1: *const ::core::ffi::c_uchar = *p as *const ::core::ffi::c_uchar;
    retry = ((*t_cp_1.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*t_cp_1.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*t_cp_1.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *t_cp_1.offset(3 as ::core::ffi::c_int as isize) as uint32_t) as guint32;
    *p = (*p).offset(NS_INT32SZ as isize);
    let mut t_cp_2: *const ::core::ffi::c_uchar = *p as *const ::core::ffi::c_uchar;
    expire = ((*t_cp_2.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*t_cp_2.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*t_cp_2.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *t_cp_2.offset(3 as ::core::ffi::c_int as isize) as uint32_t) as guint32;
    *p = (*p).offset(NS_INT32SZ as isize);
    let mut t_cp_3: *const ::core::ffi::c_uchar = *p as *const ::core::ffi::c_uchar;
    ttl = ((*t_cp_3.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*t_cp_3.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*t_cp_3.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *t_cp_3.offset(3 as ::core::ffi::c_int as isize) as uint32_t) as guint32;
    *p = (*p).offset(NS_INT32SZ as isize);
    return g_variant_new(
        b"(ssuuuuu)\0" as *const u8 as *const gchar,
        &raw mut mnamebuf as *mut gchar,
        &raw mut rnamebuf as *mut gchar,
        serial,
        refresh,
        retry,
        expire,
        ttl,
    );
}
unsafe extern "C" fn safe_c2rust_parse_res_ns(
    mut answer: *const guint8,
    mut end: *const guint8,
    mut p: *mut *const guint8,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut namebuf: [gchar; 1024] = [0; 1024];
    if safe_c2rust_expand_name(
        b"NS\0" as *const u8 as *const gchar,
        answer,
        end,
        p,
        &raw mut namebuf as *mut gchar,
        ::core::mem::size_of::<[gchar; 1024]>() as gsize,
        error,
    ) == 0
    {
        return ::core::ptr::null_mut::<GVariant>();
    }
    return g_variant_new(
        b"(s)\0" as *const u8 as *const gchar,
        &raw mut namebuf as *mut gchar,
    );
}
unsafe extern "C" fn safe_c2rust_parse_res_mx(
    mut answer: *const guint8,
    mut end: *const guint8,
    mut p: *mut *const guint8,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut namebuf: [gchar; 1024] = [0; 1024];
    let mut preference: guint16 = 0;
    if (end.offset_from(*p) as ::core::ffi::c_long) < 2 as ::core::ffi::c_long {
        g_set_error(
            error,
            g_resolver_error_quark(),
            G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error parsing DNS %s record: malformed DNS packet\0" as *const u8 as *const gchar,
            ),
            b"MX\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    let mut t_cp: *const ::core::ffi::c_uchar = *p as *const ::core::ffi::c_uchar;
    preference = ((*t_cp.offset(0 as ::core::ffi::c_int as isize) as uint16_t
        as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *t_cp.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
        as guint16;
    *p = (*p).offset(NS_INT16SZ as isize);
    if safe_c2rust_expand_name(
        b"MX\0" as *const u8 as *const gchar,
        answer,
        end,
        p,
        &raw mut namebuf as *mut gchar,
        ::core::mem::size_of::<[gchar; 1024]>() as gsize,
        error,
    ) == 0
    {
        return ::core::ptr::null_mut::<GVariant>();
    }
    return g_variant_new(
        b"(qs)\0" as *const u8 as *const gchar,
        preference as ::core::ffi::c_int,
        &raw mut namebuf as *mut gchar,
    );
}
unsafe extern "C" fn safe_c2rust_parse_res_txt(
    mut answer: *const guint8,
    mut end: *const guint8,
    mut p: *mut *const guint8,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut record: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut at: *const guint8 = *p;
    let mut len: gsize = 0;
    if end.offset_from(*p) as ::core::ffi::c_long == 0 as ::core::ffi::c_long {
        g_set_error(
            error,
            g_resolver_error_quark(),
            G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error parsing DNS %s record: malformed DNS packet\0" as *const u8 as *const gchar,
            ),
            b"TXT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    array = g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    while at < end {
        let fresh5 = at;
        at = at.offset(1);
        len = *fresh5 as gsize;
        if len > end.offset_from(at) as ::core::ffi::c_long as gsize {
            g_set_error(
                error,
                g_resolver_error_quark(),
                G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Error parsing DNS %s record: malformed DNS packet\0" as *const u8
                        as *const gchar,
                ),
                b"TXT\0" as *const u8 as *const ::core::ffi::c_char,
            );
            g_ptr_array_free(array, TRUE);
            return ::core::ptr::null_mut::<GVariant>();
        }
        g_ptr_array_add(array, g_strndup(at as *mut gchar, len) as gpointer);
        at = at.offset(len as isize);
    }
    *p = at;
    record = g_variant_new(
        b"(@as)\0" as *const u8 as *const gchar,
        g_variant_new_strv((*array).pdata as *mut *const gchar, (*array).len as gssize),
    );
    g_ptr_array_free(array, TRUE);
    return record;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_record_type_to_rrtype(
    mut type_0: GResolverRecordType,
) -> gint {
    match type_0 as ::core::ffi::c_uint {
        1 => return ns_t_srv as ::core::ffi::c_int as gint,
        3 => return ns_t_txt as ::core::ffi::c_int as gint,
        4 => return ns_t_soa as ::core::ffi::c_int as gint,
        5 => return ns_t_ns as ::core::ffi::c_int as gint,
        2 => return ns_t_mx as ::core::ffi::c_int as gint,
        _ => {}
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_CRITICAL,
        b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gthreadedresolver.c\0"
            as *const u8 as *const ::core::ffi::c_char,
        934 as ::core::ffi::c_int,
        G_STRFUNC,
    );
    return -(1 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_records_from_res_query(
    mut rrname: *const gchar,
    mut rrtype: gint,
    mut answer: *const guint8,
    mut len: gssize,
    mut herr: gint,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut count: uint16_t = 0;
    let mut namebuf: [gchar; 1024] = [0; 1024];
    let mut end: *const guint8 = ::core::ptr::null::<guint8>();
    let mut p: *const guint8 = ::core::ptr::null::<guint8>();
    let mut type_0: guint16 = 0;
    let mut qclass: guint16 = 0;
    let mut rdlength: guint16 = 0;
    let mut header: *const HEADER = ::core::ptr::null::<HEADER>();
    let mut records: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut record: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut len_unsigned: gsize = 0;
    let mut parsing_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if len <= 0 as gssize {
        if len == 0 as gssize || herr == HOST_NOT_FOUND || herr == NO_DATA {
            g_set_error(
                error,
                g_resolver_error_quark(),
                G_RESOLVER_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"No DNS record of the requested type for \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                rrname,
            );
        } else if herr == TRY_AGAIN {
            g_set_error(
                error,
                g_resolver_error_quark(),
                G_RESOLVER_ERROR_TEMPORARY_FAILURE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Temporarily unable to resolve \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                        as *const gchar,
                ),
                rrname,
            );
        } else {
            g_set_error(
                error,
                g_resolver_error_quark(),
                G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Error resolving \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8 as *const gchar,
                ),
                rrname,
            );
        }
        return ::core::ptr::null_mut::<GList>();
    }
    len_unsigned = len as gsize;
    if (len_unsigned as usize) < ::core::mem::size_of::<HEADER>() as usize {
        g_set_error(
            error,
            g_resolver_error_quark(),
            G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error resolving \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8 as *const gchar,
            ),
            rrname,
            glib_gettext(b"Malformed DNS packet\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GList>();
    }
    records = ::core::ptr::null_mut::<GList>();
    header = answer as *mut HEADER;
    p = answer.offset(::core::mem::size_of::<HEADER>() as usize as isize);
    end = answer.offset(len_unsigned as isize);
    count = safe_c2rust___bswap_16((*header).qdcount() as __uint16_t) as uint16_t;
    loop {
        let fresh3 = count;
        count = count.wrapping_sub(1);
        if !(fresh3 as ::core::ffi::c_int != 0 && p < end) {
            break;
        }
        let mut expand_result: ::core::ffi::c_int = 0;
        expand_result = dn_expand(
            answer as *const ::core::ffi::c_uchar,
            end as *const ::core::ffi::c_uchar,
            p as *const ::core::ffi::c_uchar,
            &raw mut namebuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[gchar; 1024]>() as ::core::ffi::c_int,
        );
        if expand_result < 0 as ::core::ffi::c_int
            || (end.offset_from(p) as ::core::ffi::c_long)
                < (expand_result + 4 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            g_set_error(
                error,
                g_resolver_error_quark(),
                G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Error resolving \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                        as *const gchar,
                ),
                rrname,
                glib_gettext(b"Malformed DNS packet\0" as *const u8 as *const gchar),
            );
            return ::core::ptr::null_mut::<GList>();
        }
        p = p.offset(expand_result as isize);
        p = p.offset(4 as ::core::ffi::c_int as isize);
        namebuf[0 as ::core::ffi::c_int as usize] = namebuf[1 as ::core::ffi::c_int as usize];
    }
    count = safe_c2rust___bswap_16((*header).ancount() as __uint16_t) as uint16_t;
    loop {
        let fresh4 = count;
        count = count.wrapping_sub(1);
        if !(fresh4 as ::core::ffi::c_int != 0 && p < end) {
            break;
        }
        let mut expand_result_0: ::core::ffi::c_int = 0;
        expand_result_0 = dn_expand(
            answer as *const ::core::ffi::c_uchar,
            end as *const ::core::ffi::c_uchar,
            p as *const ::core::ffi::c_uchar,
            &raw mut namebuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[gchar; 1024]>() as ::core::ffi::c_int,
        );
        if expand_result_0 < 0 as ::core::ffi::c_int
            || (end.offset_from(p) as ::core::ffi::c_long)
                < (expand_result_0 + 10 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            g_set_error(
                &raw mut parsing_error,
                g_resolver_error_quark(),
                G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Error resolving \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                        as *const gchar,
                ),
                rrname,
                glib_gettext(b"Malformed DNS packet\0" as *const u8 as *const gchar),
            );
            break;
        } else {
            p = p.offset(expand_result_0 as isize);
            let mut t_cp: *const ::core::ffi::c_uchar = p as *const ::core::ffi::c_uchar;
            type_0 = ((*t_cp.offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *t_cp.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
                as guint16;
            p = p.offset(NS_INT16SZ as isize);
            let mut t_cp_0: *const ::core::ffi::c_uchar = p as *const ::core::ffi::c_uchar;
            qclass = ((*t_cp_0.offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *t_cp_0.offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as guint16;
            p = p.offset(NS_INT16SZ as isize);
            p = p.offset(4 as ::core::ffi::c_int as isize);
            let mut t_cp_1: *const ::core::ffi::c_uchar = p as *const ::core::ffi::c_uchar;
            rdlength = ((*t_cp_1.offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *t_cp_1.offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as guint16;
            p = p.offset(NS_INT16SZ as isize);
            if (end.offset_from(p) as ::core::ffi::c_long) < rdlength as ::core::ffi::c_long {
                g_set_error(
                    &raw mut parsing_error,
                    g_resolver_error_quark(),
                    G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Error resolving \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                            as *const gchar,
                    ),
                    rrname,
                    glib_gettext(b"Malformed DNS packet\0" as *const u8 as *const gchar),
                );
                break;
            } else if type_0 as ::core::ffi::c_int != rrtype
                || qclass as ::core::ffi::c_int != ns_c_in as ::core::ffi::c_int
            {
                p = p.offset(rdlength as ::core::ffi::c_int as isize);
            } else {
                match rrtype {
                    33 => {
                        record = safe_c2rust_parse_res_srv(
                            answer,
                            p.offset(rdlength as ::core::ffi::c_int as isize),
                            &raw mut p,
                            &raw mut parsing_error,
                        );
                    }
                    15 => {
                        record = safe_c2rust_parse_res_mx(
                            answer,
                            p.offset(rdlength as ::core::ffi::c_int as isize),
                            &raw mut p,
                            &raw mut parsing_error,
                        );
                    }
                    6 => {
                        record = safe_c2rust_parse_res_soa(
                            answer,
                            p.offset(rdlength as ::core::ffi::c_int as isize),
                            &raw mut p,
                            &raw mut parsing_error,
                        );
                    }
                    2 => {
                        record = safe_c2rust_parse_res_ns(
                            answer,
                            p.offset(rdlength as ::core::ffi::c_int as isize),
                            &raw mut p,
                            &raw mut parsing_error,
                        );
                    }
                    16 => {
                        record = safe_c2rust_parse_res_txt(
                            answer,
                            p.offset(rdlength as ::core::ffi::c_int as isize),
                            &raw mut p,
                            &raw mut parsing_error,
                        );
                    }
                    _ => {
                        g_log(
                            G_LOG_DOMAIN.as_ptr() as *const gchar,
                            G_LOG_LEVEL_DEBUG,
                            b"Unrecognised DNS record type %u\0" as *const u8 as *const gchar,
                            rrtype,
                        );
                        record = ::core::ptr::null_mut::<GVariant>();
                    }
                }
                if !record.is_null() {
                    records = g_list_prepend(records, record as gpointer);
                }
                if !parsing_error.is_null() {
                    break;
                }
            }
        }
    }
    if !parsing_error.is_null() {
        g_propagate_prefixed_error(
            error,
            parsing_error,
            glib_gettext(
                b"Failed to parse DNS response for \xE2\x80\x9C%s\xE2\x80\x9D: \0" as *const u8
                    as *const gchar,
            ),
            rrname,
        );
        g_list_free_full(
            records,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVariant) -> ()>,
                GDestroyNotify,
            >(Some(
                g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> (),
            )),
        );
        return ::core::ptr::null_mut::<GList>();
    } else if records.is_null() {
        g_set_error(
            error,
            g_resolver_error_quark(),
            G_RESOLVER_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"No DNS record of the requested type for \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            rrname,
        );
        return ::core::ptr::null_mut::<GList>();
    } else {
        return records;
    };
}
unsafe extern "C" fn safe_c2rust_free_records(mut records: *mut GList) {
    g_list_free_full(
        records,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GVariant) -> ()>, GDestroyNotify>(
            Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()),
        ),
    );
}
unsafe extern "C" fn safe_c2rust_do_lookup_records(
    mut rrname: *const gchar,
    mut record_type: GResolverRecordType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut records: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut len: gint = 512 as gint;
    let mut herr: gint = 0;
    let mut answer: *mut GByteArray = ::core::ptr::null_mut::<GByteArray>();
    let mut rrtype: gint = 0;
    let mut res: __res_state = {
        let mut init = __res_state {
            ndots_nsort_ipv6_unavail_unused: [0; 4],
            retrans: 0 as ::core::ffi::c_int,
            retry: 0,
            options: 0,
            nscount: 0,
            nsaddr_list: [sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            }; 3],
            id: 0,
            dnsrch: [::core::ptr::null_mut::<::core::ffi::c_char>(); 7],
            defdname: [0; 256],
            pfcode: 0,
            sort_list: [C2RustUnnamed_11 {
                addr: in_addr { s_addr: 0 },
                mask: 0,
            }; 10],
            __glibc_unused_qhook: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            __glibc_unused_rhook: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            res_h_errno: 0,
            _vcsock: 0,
            _flags: 0,
            _u: C2RustUnnamed_8 { pad: [0; 52] },
        };
        init.set_ndots(0);
        init.set_nsort(0);
        init.set_ipv6_unavail(0);
        init.set_unused(0);
        init
    };
    if __res_ninit(&raw mut res) != 0 as ::core::ffi::c_int {
        g_set_error(
            error,
            g_resolver_error_quark(),
            G_RESOLVER_ERROR_INTERNAL as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error resolving \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8 as *const gchar,
            ),
            rrname,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    rrtype = safe_c2rust_g_resolver_record_type_to_rrtype(record_type);
    answer = g_byte_array_new();
    loop {
        g_byte_array_set_size(
            answer,
            (len as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as guint,
        );
        len = res_nquery(
            &raw mut res,
            rrname as *const ::core::ffi::c_char,
            ns_c_in as ::core::ffi::c_int,
            rrtype as ::core::ffi::c_int,
            (*answer).data as *mut ::core::ffi::c_uchar,
            (*answer).len as ::core::ffi::c_int,
        ) as gint;
        if len < 0 as ::core::ffi::c_int || len < (*answer).len as gint {
            break;
        }
    }
    herr = *__h_errno_location() as gint;
    records = safe_c2rust_g_resolver_records_from_res_query(
        rrname,
        rrtype,
        (*answer).data,
        len as gssize,
        herr,
        error,
    );
    g_byte_array_free(answer, TRUE);
    __res_nclose(&raw mut res);
    return safe_c2rust_g_steal_pointer(&raw mut records as gpointer) as *mut GList;
}
unsafe extern "C" fn safe_c2rust_lookup_records(
    mut resolver: *mut GResolver,
    mut rrname: *const gchar,
    mut record_type: GResolverRecordType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut self_0: *mut GThreadedResolver = safe_c2rust_G_THREADED_RESOLVER(resolver as gpointer);
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut records: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut data: *mut LookupData = ::core::ptr::null_mut::<LookupData>();
    task = g_task_new(resolver as gpointer, cancellable, None, NULL_0);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GResolver,
                    *const gchar,
                    GResolverRecordType,
                    *mut GCancellable,
                    *mut *mut GError,
                ) -> *mut GList,
            >,
            gpointer,
        >(Some(
            safe_c2rust_lookup_records
                as unsafe extern "C" fn(
                    *mut GResolver,
                    *const gchar,
                    GResolverRecordType,
                    *mut GCancellable,
                    *mut *mut GError,
                ) -> *mut GList,
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(_task, b"lookup_records\0" as *const u8 as *const gchar);
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] resolver lookup records\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] resolver lookup records\0" as *const u8 as *const gchar,
        );
    }
    data = safe_c2rust_lookup_data_new_records(rrname, record_type);
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LookupData as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut LookupData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_lookup_data_free as unsafe extern "C" fn(*mut LookupData) -> ()),
        ),
    );
    safe_c2rust_run_task_in_thread_pool_sync(self_0, task);
    records = g_task_propagate_pointer(task, error) as *mut GList;
    g_object_unref(task as gpointer);
    return records;
}
unsafe extern "C" fn safe_c2rust_lookup_records_async(
    mut resolver: *mut GResolver,
    mut rrname: *const ::core::ffi::c_char,
    mut record_type: GResolverRecordType,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut self_0: *mut GThreadedResolver = safe_c2rust_G_THREADED_RESOLVER(resolver as gpointer);
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut LookupData = ::core::ptr::null_mut::<LookupData>();
    task = g_task_new(resolver as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GResolver,
                    *const ::core::ffi::c_char,
                    GResolverRecordType,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_lookup_records_async
                as unsafe extern "C" fn(
                    *mut GResolver,
                    *const ::core::ffi::c_char,
                    GResolverRecordType,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"lookup_records_async\0" as *const u8 as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] resolver lookup records\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] resolver lookup records\0" as *const u8 as *const gchar,
        );
    }
    data = safe_c2rust_lookup_data_new_records(rrname as *const gchar, record_type);
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LookupData as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut LookupData) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_lookup_data_free as unsafe extern "C" fn(*mut LookupData) -> ()),
        ),
    );
    safe_c2rust_run_task_in_thread_pool_async(self_0, task);
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_lookup_records_finish(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, resolver as gpointer) != 0 {
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
            b"g_task_is_valid (result, resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GList;
}
unsafe extern "C" fn safe_c2rust_timeout_cb(mut user_data: gpointer) -> gboolean {
    let mut weak_task: *mut GWeakRef = user_data as *mut GWeakRef;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut LookupData = ::core::ptr::null_mut::<LookupData>();
    let mut should_return: gboolean = 0;
    task = g_weak_ref_get(weak_task) as *mut GTask;
    if task.is_null() {
        return G_SOURCE_REMOVE;
    }
    data = g_task_get_task_data(task) as *mut LookupData;
    g_mutex_lock(&raw mut (*data).lock);
    should_return = ({
        let mut gaicae_oldval: gint = NOT_YET as ::core::ffi::c_int as gint;
        if 0 as ::core::ffi::c_int != 0 {
            (*data).will_return;
        } else {
        };
        let fresh7 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
            &raw mut (*data).will_return,
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut C2RustUnnamed_2),
            TIMED_OUT,
        );
        *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut C2RustUnnamed_2) = fresh7.0;
        if fresh7.1 as ::core::ffi::c_int != 0 {
            TRUE
        } else {
            FALSE
        }
    }) as gboolean;
    let mut _pp: *mut *mut GSource = &raw mut (*data).timeout_source;
    let mut _ptr: *mut GSource = *_pp;
    *_pp = ::core::ptr::null_mut::<GSource>();
    if !_ptr.is_null() {
        g_source_unref(_ptr as *mut GSource);
    }
    g_mutex_unlock(&raw mut (*data).lock);
    if should_return != 0 {
        g_task_return_new_error_literal(
            task,
            g_io_error_quark(),
            G_IO_ERROR_TIMED_OUT as ::core::ffi::c_int as gint,
            glib_gettext(b"Socket I/O timed out\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
    }
    g_mutex_lock(&raw mut (*data).lock);
    (*data).has_returned = TRUE as gboolean;
    g_cond_broadcast(&raw mut (*data).cond);
    g_mutex_unlock(&raw mut (*data).lock);
    g_object_unref(task as gpointer);
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_cancelled_cb(
    mut cancellable: *mut GCancellable,
    mut user_data: gpointer,
) -> gboolean {
    let mut weak_task: *mut GWeakRef = user_data as *mut GWeakRef;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut LookupData = ::core::ptr::null_mut::<LookupData>();
    let mut should_return: gboolean = 0;
    task = g_weak_ref_get(weak_task) as *mut GTask;
    if task.is_null() {
        return G_SOURCE_REMOVE;
    }
    data = g_task_get_task_data(task) as *mut LookupData;
    g_mutex_lock(&raw mut (*data).lock);
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if g_cancellable_is_cancelled(cancellable) != 0 {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gthreadedresolver.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1487 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_cancellable_is_cancelled (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    should_return = ({
        let mut gaicae_oldval: gint = NOT_YET as ::core::ffi::c_int as gint;
        if 0 as ::core::ffi::c_int != 0 {
            (*data).will_return;
        } else {
        };
        let fresh6 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
            &raw mut (*data).will_return,
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut C2RustUnnamed_2),
            CANCELLED,
        );
        *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut C2RustUnnamed_2) = fresh6.0;
        if fresh6.1 as ::core::ffi::c_int != 0 {
            TRUE
        } else {
            FALSE
        }
    }) as gboolean;
    let mut _pp: *mut *mut GSource = &raw mut (*data).cancellable_source;
    let mut _ptr: *mut GSource = *_pp;
    *_pp = ::core::ptr::null_mut::<GSource>();
    if !_ptr.is_null() {
        g_source_unref(_ptr as *mut GSource);
    }
    g_mutex_unlock(&raw mut (*data).lock);
    if should_return != 0 {
        g_task_return_error_if_cancelled(task);
    }
    g_mutex_lock(&raw mut (*data).lock);
    (*data).has_returned = TRUE as gboolean;
    g_cond_broadcast(&raw mut (*data).cond);
    g_mutex_unlock(&raw mut (*data).lock);
    g_object_unref(task as gpointer);
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_weak_ref_clear_and_free(mut weak_ref: *mut GWeakRef) {
    g_weak_ref_clear(weak_ref);
    g_free(weak_ref as gpointer);
}
unsafe extern "C" fn safe_c2rust_run_task_in_thread_pool_async(
    mut self_0: *mut GThreadedResolver,
    mut task: *mut GTask,
) {
    let mut data: *mut LookupData = g_task_get_task_data(task) as *mut LookupData;
    let mut timeout_ms: guint =
        g_resolver_get_timeout(self_0 as *mut ::core::ffi::c_void as *mut GResolver) as guint;
    let mut cancellable: *mut GCancellable = g_task_get_cancellable(task);
    g_mutex_lock(&raw mut (*data).lock);
    g_thread_pool_push(
        (*self_0).thread_pool,
        g_object_ref(task as gpointer) as *mut GTask as gpointer,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if timeout_ms != 0 as guint {
        let mut weak_task: *mut GWeakRef = ({
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
        g_weak_ref_set(weak_task, task as gpointer);
        (*data).timeout_source = g_timeout_source_new(timeout_ms);
        g_source_set_static_name(
            (*data).timeout_source,
            b"[gio] threaded resolver timeout\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_source_set_callback(
            (*data).timeout_source,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GSourceFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(gpointer) -> gboolean>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_timeout_cb as unsafe extern "C" fn(gpointer) -> gboolean,
                )),
            ),
            safe_c2rust_g_steal_pointer(&raw mut weak_task as gpointer) as *mut GWeakRef
                as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GWeakRef) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_weak_ref_clear_and_free as unsafe extern "C" fn(*mut GWeakRef) -> (),
            )),
        );
        g_source_attach(
            (*data).timeout_source,
            (*glib__private__())
                .g_get_worker_context
                .expect("non-null function pointer")(),
        );
    }
    if !cancellable.is_null() {
        let mut weak_task_0: *mut GWeakRef = ({
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
        g_weak_ref_set(weak_task_0, task as gpointer);
        (*data).cancellable_source = g_cancellable_source_new(cancellable);
        g_source_set_static_name(
            (*data).cancellable_source,
            b"[gio] threaded resolver cancellable\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_source_set_callback(
            (*data).cancellable_source,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GSourceFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> gboolean>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_cancelled_cb
                        as unsafe extern "C" fn(*mut GCancellable, gpointer) -> gboolean,
                )),
            ),
            safe_c2rust_g_steal_pointer(&raw mut weak_task_0 as gpointer) as *mut GWeakRef
                as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GWeakRef) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_weak_ref_clear_and_free as unsafe extern "C" fn(*mut GWeakRef) -> (),
            )),
        );
        g_source_attach(
            (*data).cancellable_source,
            (*glib__private__())
                .g_get_worker_context
                .expect("non-null function pointer")(),
        );
    }
    g_mutex_unlock(&raw mut (*data).lock);
}
unsafe extern "C" fn safe_c2rust_run_task_in_thread_pool_sync(
    mut self_0: *mut GThreadedResolver,
    mut task: *mut GTask,
) {
    let mut data: *mut LookupData = g_task_get_task_data(task) as *mut LookupData;
    safe_c2rust_run_task_in_thread_pool_async(self_0, task);
    g_mutex_lock(&raw mut (*data).lock);
    while (*data).has_returned == 0 {
        g_cond_wait(&raw mut (*data).cond, &raw mut (*data).lock);
    }
    g_mutex_unlock(&raw mut (*data).lock);
}
unsafe extern "C" fn safe_c2rust_threaded_resolver_worker_cb(
    mut task_data: gpointer,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask =
        safe_c2rust_g_steal_pointer(&raw mut task_data as gpointer) as *mut GTask;
    let mut data: *mut LookupData = g_task_get_task_data(task) as *mut LookupData;
    let mut cancellable: *mut GCancellable = g_task_get_cancellable(task);
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut should_return: gboolean = 0;
    match (*data).lookup_type as ::core::ffi::c_uint {
        0 => {
            let mut addresses: *mut GList = safe_c2rust_do_lookup_by_name(
                (*data).c2rust_unnamed.lookup_by_name.hostname,
                (*data).c2rust_unnamed.lookup_by_name.address_family,
                cancellable,
                &raw mut local_error,
            );
            g_mutex_lock(&raw mut (*data).lock);
            should_return = ({
                let mut gaicae_oldval: gint = NOT_YET as ::core::ffi::c_int as gint;
                if 0 as ::core::ffi::c_int != 0 {
                    (*data).will_return;
                } else {
                };
                let fresh0 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*data).will_return,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut C2RustUnnamed_2),
                    COMPLETED,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut C2RustUnnamed_2) =
                    fresh0.0;
                if fresh0.1 as ::core::ffi::c_int != 0 {
                    TRUE
                } else {
                    FALSE
                }
            }) as gboolean;
            g_mutex_unlock(&raw mut (*data).lock);
            if should_return != 0 {
                if !addresses.is_null() {
                    g_task_return_pointer(
                        task,
                        safe_c2rust_g_steal_pointer(&raw mut addresses as gpointer) as *mut GList
                            as gpointer,
                        ::core::mem::transmute::<
                            Option<unsafe extern "C" fn(*mut GList) -> ()>,
                            GDestroyNotify,
                        >(Some(
                            g_resolver_free_addresses as unsafe extern "C" fn(*mut GList) -> (),
                        )),
                    );
                } else {
                    g_task_return_error(
                        task,
                        safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer)
                            as *mut GError,
                    );
                }
            }
            let mut _pp: *mut *mut GList = &raw mut addresses;
            let mut _ptr: *mut GList = *_pp;
            *_pp = ::core::ptr::null_mut::<GList>();
            if !_ptr.is_null() {
                g_resolver_free_addresses(_ptr as *mut GList);
            }
            g_clear_error(&raw mut local_error);
        }
        1 => {
            let mut name: *mut gchar = safe_c2rust_do_lookup_by_address(
                (*data).c2rust_unnamed.lookup_by_address.address,
                cancellable,
                &raw mut local_error,
            );
            g_mutex_lock(&raw mut (*data).lock);
            should_return = ({
                let mut gaicae_oldval: gint = NOT_YET as ::core::ffi::c_int as gint;
                if 0 as ::core::ffi::c_int != 0 {
                    (*data).will_return;
                } else {
                };
                let fresh1 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*data).will_return,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut C2RustUnnamed_2),
                    COMPLETED,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut C2RustUnnamed_2) =
                    fresh1.0;
                if fresh1.1 as ::core::ffi::c_int != 0 {
                    TRUE
                } else {
                    FALSE
                }
            }) as gboolean;
            g_mutex_unlock(&raw mut (*data).lock);
            if should_return != 0 {
                if !name.is_null() {
                    g_task_return_pointer(
                        task,
                        safe_c2rust_g_steal_pointer(&raw mut name as gpointer) as *mut gchar
                            as gpointer,
                        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
                    );
                } else {
                    g_task_return_error(
                        task,
                        safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer)
                            as *mut GError,
                    );
                }
            }
            let mut _pp_0: *mut *mut gchar = &raw mut name;
            let mut _ptr_0: *mut gchar = *_pp_0;
            *_pp_0 = ::core::ptr::null_mut::<gchar>();
            if !_ptr_0.is_null() {
                g_free(_ptr_0 as gpointer);
            }
            g_clear_error(&raw mut local_error);
        }
        2 => {
            let mut records: *mut GList = safe_c2rust_do_lookup_records(
                (*data).c2rust_unnamed.lookup_records.rrname,
                (*data).c2rust_unnamed.lookup_records.record_type,
                cancellable,
                &raw mut local_error,
            );
            g_mutex_lock(&raw mut (*data).lock);
            should_return = ({
                let mut gaicae_oldval: gint = NOT_YET as ::core::ffi::c_int as gint;
                if 0 as ::core::ffi::c_int != 0 {
                    (*data).will_return;
                } else {
                };
                let fresh2 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*data).will_return,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut C2RustUnnamed_2),
                    COMPLETED,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut C2RustUnnamed_2) =
                    fresh2.0;
                if fresh2.1 as ::core::ffi::c_int != 0 {
                    TRUE
                } else {
                    FALSE
                }
            }) as gboolean;
            g_mutex_unlock(&raw mut (*data).lock);
            if should_return != 0 {
                if !records.is_null() {
                    g_task_return_pointer(
                        task,
                        safe_c2rust_g_steal_pointer(&raw mut records as gpointer) as *mut GList
                            as gpointer,
                        ::core::mem::transmute::<
                            Option<unsafe extern "C" fn(*mut GList) -> ()>,
                            GDestroyNotify,
                        >(Some(
                            safe_c2rust_free_records as unsafe extern "C" fn(*mut GList) -> (),
                        )),
                    );
                } else {
                    g_task_return_error(
                        task,
                        safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer)
                            as *mut GError,
                    );
                }
            }
            let mut _pp_1: *mut *mut GList = &raw mut records;
            let mut _ptr_1: *mut GList = *_pp_1;
            *_pp_1 = ::core::ptr::null_mut::<GList>();
            if !_ptr_1.is_null() {
                safe_c2rust_free_records(_ptr_1 as *mut GList);
            }
            g_clear_error(&raw mut local_error);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gthreadedresolver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1645 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    g_mutex_lock(&raw mut (*data).lock);
    (*data).has_returned = TRUE as gboolean;
    g_cond_broadcast(&raw mut (*data).cond);
    g_mutex_unlock(&raw mut (*data).lock);
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_threaded_resolver_class_init(
    mut threaded_class: *mut GThreadedResolverClass,
) {
    let mut object_class: *mut GObjectClass =
        threaded_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut resolver_class: *mut GResolverClass =
        threaded_class as *mut ::core::ffi::c_void as *mut GResolverClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_threaded_resolver_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*resolver_class).lookup_by_name = Some(
        safe_c2rust_lookup_by_name
            as unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GList,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GList,
        >;
    (*resolver_class).lookup_by_name_async = Some(
        safe_c2rust_lookup_by_name_async
            as unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*resolver_class).lookup_by_name_finish = Some(
        safe_c2rust_lookup_by_name_finish
            as unsafe extern "C" fn(
                *mut GResolver,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GList,
    )
        as Option<
            unsafe extern "C" fn(*mut GResolver, *mut GAsyncResult, *mut *mut GError) -> *mut GList,
        >;
    (*resolver_class).lookup_by_name_with_flags = Some(
        safe_c2rust_lookup_by_name_with_flags
            as unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                GResolverNameLookupFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GList,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                GResolverNameLookupFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GList,
        >;
    (*resolver_class).lookup_by_name_with_flags_async = Some(
        safe_c2rust_lookup_by_name_with_flags_async
            as unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                GResolverNameLookupFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                GResolverNameLookupFlags,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*resolver_class).lookup_by_name_with_flags_finish = Some(
        safe_c2rust_lookup_by_name_with_flags_finish
            as unsafe extern "C" fn(
                *mut GResolver,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GList,
    )
        as Option<
            unsafe extern "C" fn(*mut GResolver, *mut GAsyncResult, *mut *mut GError) -> *mut GList,
        >;
    (*resolver_class).lookup_by_address = Some(
        safe_c2rust_lookup_by_address
            as unsafe extern "C" fn(
                *mut GResolver,
                *mut GInetAddress,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GResolver,
                *mut GInetAddress,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut gchar,
        >;
    (*resolver_class).lookup_by_address_async = Some(
        safe_c2rust_lookup_by_address_async
            as unsafe extern "C" fn(
                *mut GResolver,
                *mut GInetAddress,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GResolver,
                *mut GInetAddress,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*resolver_class).lookup_by_address_finish = Some(
        safe_c2rust_lookup_by_address_finish
            as unsafe extern "C" fn(
                *mut GResolver,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut gchar,
    )
        as Option<
            unsafe extern "C" fn(*mut GResolver, *mut GAsyncResult, *mut *mut GError) -> *mut gchar,
        >;
    (*resolver_class).lookup_records = Some(
        safe_c2rust_lookup_records
            as unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                GResolverRecordType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GList,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                GResolverRecordType,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GList,
        >;
    (*resolver_class).lookup_records_async = Some(
        safe_c2rust_lookup_records_async
            as unsafe extern "C" fn(
                *mut GResolver,
                *const ::core::ffi::c_char,
                GResolverRecordType,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GResolver,
                *const gchar,
                GResolverRecordType,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*resolver_class).lookup_records_finish = Some(
        safe_c2rust_lookup_records_finish
            as unsafe extern "C" fn(
                *mut GResolver,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GList,
    )
        as Option<
            unsafe extern "C" fn(*mut GResolver, *mut GAsyncResult, *mut *mut GError) -> *mut GList,
        >;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
