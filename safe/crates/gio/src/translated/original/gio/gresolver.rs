use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInetAddressPrivate;
    pub type _GSrvTarget;
    pub type _GTask;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
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
    fn g_list_free(list: *mut GList);
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn g_hostname_is_non_ascii(hostname: *const gchar) -> gboolean;
    fn g_hostname_to_ascii(hostname: *const gchar) -> *mut gchar;
    fn g_ascii_strncasecmp(s1: *const gchar, s2: *const gchar, n: gsize) -> gint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
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
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_signal_new(
        signal_name: *const gchar,
        itype: GType,
        signal_flags: GSignalFlags,
        class_offset: guint,
        accumulator: GSignalAccumulator,
        accu_data: gpointer,
        c_marshaller: GSignalCMarshaller,
        return_type: GType,
        n_params: guint,
        ...
    ) -> guint;
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_object_class_install_properties(
        oclass: *mut GObjectClass,
        n_pspecs: guint,
        pspecs: *mut *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify_by_pspec(object: *mut GObject, pspec: *mut GParamSpec);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn gai_strerror(__ecode: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn inet_aton(__cp: *const ::core::ffi::c_char, __inp: *mut in_addr) -> ::core::ffi::c_int;
    fn g_networking_init();
    fn g_async_result_get_type() -> GType;
    fn g_async_result_legacy_propagate_error(
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_async_result_is_tagged(res: *mut GAsyncResult, source_tag: gpointer) -> gboolean;
    fn g_inet_address_get_type() -> GType;
    fn g_inet_address_new_from_string(string: *const gchar) -> *mut GInetAddress;
    fn g_inet_address_new_loopback(family: GSocketFamily) -> *mut GInetAddress;
    fn g_inet_address_equal(
        address: *mut GInetAddress,
        other_address: *mut GInetAddress,
    ) -> gboolean;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_report_new_error(
        source_object: gpointer,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
        source_tag: gpointer,
        domain: GQuark,
        code: gint,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_name(task: *mut GTask, name: *const gchar);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_srv_target_new(
        hostname: *const gchar,
        port: guint16,
        priority: guint16,
        weight: guint16,
    ) -> *mut GSrvTarget;
    fn g_srv_target_free(target: *mut GSrvTarget);
    fn g_srv_target_list_sort(targets: *mut GList) -> *mut GList;
    fn g_cancellable_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
    fn g_threaded_resolver_get_type() -> GType;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
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
pub type __uint32_t = u32;
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
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type GClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        guint,
        *const GValue,
        gpointer,
        gpointer,
    ) -> (),
>;
pub type GSignalFlags = ::core::ffi::c_uint;
pub const G_SIGNAL_ACCUMULATOR_FIRST_RUN: GSignalFlags = 131072;
pub const G_SIGNAL_DEPRECATED: GSignalFlags = 256;
pub const G_SIGNAL_MUST_COLLECT: GSignalFlags = 128;
pub const G_SIGNAL_NO_HOOKS: GSignalFlags = 64;
pub const G_SIGNAL_ACTION: GSignalFlags = 32;
pub const G_SIGNAL_DETAILED: GSignalFlags = 16;
pub const G_SIGNAL_NO_RECURSE: GSignalFlags = 8;
pub const G_SIGNAL_RUN_CLEANUP: GSignalFlags = 4;
pub const G_SIGNAL_RUN_LAST: GSignalFlags = 2;
pub const G_SIGNAL_RUN_FIRST: GSignalFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSignalInvocationHint {
    pub signal_id: guint,
    pub detail: GQuark,
    pub run_type: GSignalFlags,
}
pub type GSignalInvocationHint = _GSignalInvocationHint;
pub type GSignalCMarshaller = GClosureMarshal;
pub type GSignalAccumulator = Option<
    unsafe extern "C" fn(
        *mut GSignalInvocationHint,
        *mut GValue,
        *const GValue,
        gpointer,
    ) -> gboolean,
>;
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
pub const G_RESOLVER_ERROR_INTERNAL: C2RustUnnamed_1 = 2;
pub const G_RESOLVER_ERROR_TEMPORARY_FAILURE: C2RustUnnamed_1 = 1;
pub const G_RESOLVER_ERROR_NOT_FOUND: C2RustUnnamed_1 = 0;
pub type GResolverRecordType = ::core::ffi::c_uint;
pub const G_RESOLVER_RECORD_NS: GResolverRecordType = 5;
pub const G_RESOLVER_RECORD_SOA: GResolverRecordType = 4;
pub const G_RESOLVER_RECORD_TXT: GResolverRecordType = 3;
pub const G_RESOLVER_RECORD_MX: GResolverRecordType = 2;
pub const G_RESOLVER_RECORD_SRV: GResolverRecordType = 1;
pub type GSocketFamily = ::core::ffi::c_uint;
pub const G_SOCKET_FAMILY_IPV6: GSocketFamily = 10;
pub const G_SOCKET_FAMILY_IPV4: GSocketFamily = 2;
pub const G_SOCKET_FAMILY_UNIX: GSocketFamily = 1;
pub const G_SOCKET_FAMILY_INVALID: GSocketFamily = 0;
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
pub struct _GResolver {
    pub parent_instance: GObject,
    pub priv_0: *mut GResolverPrivate,
}
pub type GResolverPrivate = _GResolverPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GResolverPrivate {
    pub timeout_ms: ::core::ffi::c_uint,
    pub mutex: GMutex,
    pub resolv_conf_timestamp: time_t,
}
pub type GResolver = _GResolver;
pub type GSrvTarget = _GSrvTarget;
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
pub const RELOAD: C2RustUnnamed_2 = 0;
pub const PROP_TIMEOUT: GResolverProperty = 1;
pub type GResolverProperty = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_2 = 1;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
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
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const EAI_NONAME: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const _PATH_RESCONF: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"/etc/resolv.conf\0")
};
static mut safe_c2rust_props: [*mut GParamSpec; 2] = [
    ::core::ptr::null::<GParamSpec>() as *mut GParamSpec,
    ::core::ptr::null::<GParamSpec>() as *mut GParamSpec,
];
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
static mut safe_c2rust_g_resolver_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_resolver_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GResolver\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GResolverClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_resolver_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GResolver>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GResolver) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_resolver_init as unsafe extern "C" fn(*mut GResolver) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GResolver_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GResolverPrivate>() as gsize,
    );
    g_networking_init();
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_resolver_get_instance_private(
    mut self_0: *mut GResolver,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GResolver_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_resolver_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_resolver_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GResolver_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GResolver_private_offset);
    }
    safe_c2rust_g_resolver_class_init(klass as *mut GResolverClass);
}
static mut safe_c2rust_GResolver_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_resolver_get_type_once();
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
unsafe extern "C" fn safe_c2rust_srv_records_to_targets(mut records: *mut GList) -> *mut GList {
    let mut hostname: *const gchar = ::core::ptr::null::<gchar>();
    let mut port: guint16 = 0;
    let mut priority: guint16 = 0;
    let mut weight: guint16 = 0;
    let mut target: *mut GSrvTarget = ::core::ptr::null_mut::<GSrvTarget>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    l = records;
    while !l.is_null() {
        g_variant_get(
            (*l).data as *mut GVariant,
            b"(qqq&s)\0" as *const u8 as *const gchar,
            &raw mut priority,
            &raw mut weight,
            &raw mut port,
            &raw mut hostname,
        );
        target = g_srv_target_new(hostname, port, priority, weight);
        g_variant_unref((*l).data as *mut GVariant);
        (*l).data = target as gpointer;
        l = if !l.is_null() {
            (*l).next
        } else {
            ::core::ptr::null_mut::<GList>()
        };
    }
    return g_srv_target_list_sort(records);
}
unsafe extern "C" fn safe_c2rust_g_resolver_real_lookup_service(
    mut resolver: *mut GResolver,
    mut rrname: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut records: *mut GList = ::core::ptr::null_mut::<GList>();
    records = (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_records
        .expect("non-null function pointer")(
        resolver,
        rrname,
        G_RESOLVER_RECORD_SRV,
        cancellable,
        error,
    );
    return safe_c2rust_srv_records_to_targets(records);
}
unsafe extern "C" fn safe_c2rust_g_resolver_real_lookup_service_async(
    mut resolver: *mut GResolver,
    mut rrname: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_records_async
        .expect("non-null function pointer")(
        resolver,
        rrname,
        G_RESOLVER_RECORD_SRV,
        cancellable,
        callback,
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_g_resolver_real_lookup_service_finish(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut records: *mut GList = ::core::ptr::null_mut::<GList>();
    records = (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_records_finish
        .expect("non-null function pointer")(resolver, result, error);
    return safe_c2rust_srv_records_to_targets(records);
}
unsafe extern "C" fn safe_c2rust_g_resolver_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GResolver = object as *mut ::core::ffi::c_void as *mut GResolver;
    match prop_id as GResolverProperty as ::core::ffi::c_uint {
        1 => {
            g_value_set_uint(value, safe_c2rust_g_resolver_get_timeout(self_0) as guint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gresolver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_resolver_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GResolver = object as *mut ::core::ffi::c_void as *mut GResolver;
    match prop_id as GResolverProperty as ::core::ffi::c_uint {
        1 => {
            safe_c2rust_g_resolver_set_timeout(
                self_0,
                g_value_get_uint(value) as ::core::ffi::c_uint,
            );
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gresolver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                192 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_resolver_finalize(mut object: *mut GObject) {
    let mut resolver: *mut GResolver = object as *mut ::core::ffi::c_void as *mut GResolver;
    g_mutex_clear(&raw mut (*(*resolver).priv_0).mutex);
    (*(safe_c2rust_g_resolver_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_resolver_class_init(mut resolver_class: *mut GResolverClass) {
    let mut object_class: *mut GObjectClass =
        resolver_class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).get_property = Some(
        safe_c2rust_g_resolver_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_resolver_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).finalize =
        Some(safe_c2rust_g_resolver_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*resolver_class).lookup_service = Some(
        safe_c2rust_g_resolver_real_lookup_service
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
    (*resolver_class).lookup_service_async = Some(
        safe_c2rust_g_resolver_real_lookup_service_async
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
    (*resolver_class).lookup_service_finish = Some(
        safe_c2rust_g_resolver_real_lookup_service_finish
            as unsafe extern "C" fn(
                *mut GResolver,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> *mut GList,
    )
        as Option<
            unsafe extern "C" fn(*mut GResolver, *mut GAsyncResult, *mut *mut GError) -> *mut GList,
        >;
    safe_c2rust_props[PROP_TIMEOUT as ::core::ffi::c_int as usize] = g_param_spec_uint(
        b"timeout\0" as *const u8 as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
        0 as guint,
        G_MAXUINT,
        0 as guint,
        (G_PARAM_READWRITE as ::core::ffi::c_int
            | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)
            | G_PARAM_EXPLICIT_NOTIFY as ::core::ffi::c_int) as GParamFlags,
    );
    g_object_class_install_properties(
        object_class,
        (::core::mem::size_of::<[*mut GParamSpec; 2]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut GParamSpec>() as usize) as guint,
        &raw mut safe_c2rust_props as *mut *mut GParamSpec,
    );
    safe_c2rust_signals[RELOAD as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"reload\0" as *const u8 as *const gchar),
        safe_c2rust_g_resolver_get_type(),
        G_SIGNAL_RUN_LAST,
        136 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
}
unsafe extern "C" fn safe_c2rust_g_resolver_init(mut resolver: *mut GResolver) {
    let mut st: stat = stat {
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
    (*resolver).priv_0 =
        safe_c2rust_g_resolver_get_instance_private(resolver) as *mut GResolverPrivate;
    if stat(_PATH_RESCONF.as_ptr(), &raw mut st) == 0 as ::core::ffi::c_int {
        (*(*resolver).priv_0).resolv_conf_timestamp = st.st_mtim.tv_sec as time_t;
    }
    g_mutex_init(&raw mut (*(*resolver).priv_0).mutex);
}
static mut safe_c2rust_g__default_resolver_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_default_resolver: *mut GResolver =
    ::core::ptr::null::<GResolver>() as *mut GResolver;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_get_default() -> *mut GResolver {
    let mut ret: *mut GResolver = ::core::ptr::null_mut::<GResolver>();
    g_mutex_lock(&raw mut safe_c2rust_g__default_resolver_lock);
    if safe_c2rust_default_resolver.is_null() {
        safe_c2rust_default_resolver = g_object_new(
            g_threaded_resolver_get_type(),
            b"timeout\0" as *const u8 as *const gchar,
            30000 as ::core::ffi::c_int,
            NULL_0,
        ) as *mut GResolver;
    }
    ret =
        g_object_ref(safe_c2rust_default_resolver as gpointer) as *mut GResolver as *mut GResolver;
    g_mutex_unlock(&raw mut safe_c2rust_g__default_resolver_lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_set_default(mut resolver: *mut GResolver) {
    g_mutex_lock(&raw mut safe_c2rust_g__default_resolver_lock);
    if !safe_c2rust_default_resolver.is_null() {
        g_object_unref(safe_c2rust_default_resolver as gpointer);
    }
    safe_c2rust_default_resolver =
        g_object_ref(resolver as gpointer) as *mut GResolver as *mut GResolver;
    g_mutex_unlock(&raw mut safe_c2rust_g__default_resolver_lock);
}
unsafe extern "C" fn safe_c2rust_maybe_emit_reload(mut resolver: *mut GResolver) {
    let mut st: stat = stat {
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
    if stat(_PATH_RESCONF.as_ptr(), &raw mut st) == 0 as ::core::ffi::c_int {
        g_mutex_lock(&raw mut (*(*resolver).priv_0).mutex);
        if st.st_mtim.tv_sec != (*(*resolver).priv_0).resolv_conf_timestamp {
            (*(*resolver).priv_0).resolv_conf_timestamp = st.st_mtim.tv_sec as time_t;
            g_mutex_unlock(&raw mut (*(*resolver).priv_0).mutex);
            g_signal_emit(
                resolver as gpointer,
                safe_c2rust_signals[RELOAD as ::core::ffi::c_int as usize],
                0 as GQuark,
            );
        } else {
            g_mutex_unlock(&raw mut (*(*resolver).priv_0).mutex);
        }
    }
}
unsafe extern "C" fn safe_c2rust_remove_duplicates(mut addrs: *mut GList) {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut ll: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut lll: *mut GList = ::core::ptr::null_mut::<GList>();
    l = addrs;
    while !l.is_null() {
        let mut address: *mut GInetAddress = (*l).data as *mut GInetAddress;
        ll = (*l).next;
        while !ll.is_null() {
            let mut other_address: *mut GInetAddress = (*ll).data as *mut GInetAddress;
            lll = (*ll).next;
            if g_inet_address_equal(address, other_address) != 0 {
                g_object_unref(other_address as gpointer);
                if !(({
                    let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                    if g_list_delete_link(addrs, ll) == addrs {
                        _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_10
                }) as ::core::ffi::c_long
                    != 0)
                {
                    g_warn_message(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gresolver.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        379 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"g_list_delete_link (addrs, ll) == addrs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            }
            ll = lll;
        }
        l = (*l).next;
    }
}
unsafe extern "C" fn safe_c2rust_hostname_is_localhost(
    mut hostname: *const ::core::ffi::c_char,
) -> gboolean {
    let mut len: size_t = strlen(hostname);
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if len < strlen(b"localhost\0" as *const u8 as *const ::core::ffi::c_char) {
        return FALSE;
    }
    if *hostname.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int == '.' as i32
    {
        len = len.wrapping_sub(1);
    }
    p = hostname
        .offset(len as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    while p >= hostname {
        if *p as ::core::ffi::c_int == '.' as i32 {
            p = p.offset(1);
            break;
        } else {
            if p == hostname {
                break;
            }
            p = p.offset(-1);
        }
    }
    len = len.wrapping_sub(p.offset_from(hostname) as ::core::ffi::c_long as size_t);
    return (g_ascii_strncasecmp(
        p as *const gchar,
        b"localhost\0" as *const u8 as *const gchar,
        (if len > strlen(b"localhost\0" as *const u8 as *const ::core::ffi::c_char) {
            len as gsize
        } else {
            strlen(b"localhost\0" as *const u8 as *const ::core::ffi::c_char) as gsize
        }),
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_handle_ip_address_or_localhost(
    mut hostname: *const ::core::ffi::c_char,
    mut addrs: *mut *mut GList,
    mut flags: GResolverNameLookupFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut addr: *mut GInetAddress = ::core::ptr::null_mut::<GInetAddress>();
    let mut ip4addr: in_addr = in_addr { s_addr: 0 };
    addr = g_inet_address_new_from_string(hostname as *const gchar);
    if !addr.is_null() {
        *addrs = g_list_append(::core::ptr::null_mut::<GList>(), addr as gpointer);
        return TRUE;
    }
    *addrs = ::core::ptr::null_mut::<GList>();
    if inet_aton(hostname, &raw mut ip4addr) != 0 {
        let mut error_message: *mut gchar = g_locale_to_utf8(
            gai_strerror(EAI_NONAME) as *const gchar,
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
        g_set_error(
            error,
            safe_c2rust_g_resolver_error_quark(),
            G_RESOLVER_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error resolving \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8 as *const gchar,
            ),
            hostname,
            error_message,
        );
        g_free(error_message as gpointer);
        return TRUE;
    }
    if safe_c2rust_hostname_is_localhost(hostname) != 0 {
        if flags as ::core::ffi::c_uint
            & G_RESOLVER_NAME_LOOKUP_FLAGS_IPV6_ONLY as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            *addrs = g_list_append(
                *addrs,
                g_inet_address_new_loopback(G_SOCKET_FAMILY_IPV6) as gpointer,
            );
        }
        if flags as ::core::ffi::c_uint
            & G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            *addrs = g_list_append(
                *addrs,
                g_inet_address_new_loopback(G_SOCKET_FAMILY_IPV4) as gpointer,
            );
        }
        if (*addrs).is_null() {
            *addrs = g_list_append(
                *addrs,
                g_inet_address_new_loopback(G_SOCKET_FAMILY_IPV6) as gpointer,
            );
            *addrs = g_list_append(
                *addrs,
                g_inet_address_new_loopback(G_SOCKET_FAMILY_IPV4) as gpointer,
            );
        }
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_lookup_by_name_real(
    mut resolver: *mut GResolver,
    mut hostname: *const gchar,
    mut flags: GResolverNameLookupFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut addrs: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut ascii_hostname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !hostname.is_null() {
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
            b"hostname != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = cancellable as *mut GTypeInstance;
                let mut __t: GType = g_cancellable_get_type();
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if safe_c2rust_handle_ip_address_or_localhost(
        hostname as *const ::core::ffi::c_char,
        &raw mut addrs,
        flags,
        error,
    ) != 0
    {
        return addrs;
    }
    if g_hostname_is_non_ascii(hostname) != 0 {
        ascii_hostname = g_hostname_to_ascii(hostname);
        hostname = ascii_hostname;
    }
    if hostname.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid hostname\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GList>();
    }
    safe_c2rust_maybe_emit_reload(resolver);
    if flags as ::core::ffi::c_uint
        != G_RESOLVER_NAME_LOOKUP_FLAGS_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
            .lookup_by_name_with_flags
            .is_none()
        {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(b"%s not implemented\0" as *const u8 as *const gchar),
                b"lookup_by_name_with_flags\0" as *const u8 as *const ::core::ffi::c_char,
            );
            g_free(ascii_hostname as gpointer);
            return ::core::ptr::null_mut::<GList>();
        }
        addrs = (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
            .lookup_by_name_with_flags
            .expect("non-null function pointer")(
            resolver, hostname, flags, cancellable, error
        );
    } else {
        addrs = (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
            .lookup_by_name
            .expect("non-null function pointer")(
            resolver, hostname, cancellable, error
        );
    }
    safe_c2rust_remove_duplicates(addrs);
    g_free(ascii_hostname as gpointer);
    return addrs;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_by_name(
    mut resolver: *mut GResolver,
    mut hostname: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    return safe_c2rust_lookup_by_name_real(
        resolver,
        hostname,
        G_RESOLVER_NAME_LOOKUP_FLAGS_DEFAULT,
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_by_name_with_flags(
    mut resolver: *mut GResolver,
    mut hostname: *const gchar,
    mut flags: GResolverNameLookupFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    return safe_c2rust_lookup_by_name_real(resolver, hostname, flags, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_lookup_by_name_async_real(
    mut resolver: *mut GResolver,
    mut hostname: *const gchar,
    mut flags: GResolverNameLookupFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut ascii_hostname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut addrs: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !hostname.is_null() {
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
            b"hostname != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !(flags as ::core::ffi::c_uint
            & G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && flags as ::core::ffi::c_uint
                & G_RESOLVER_NAME_LOOKUP_FLAGS_IPV6_ONLY as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0)
        {
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
            b"!(flags & G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY && flags & G_RESOLVER_NAME_LOOKUP_FLAGS_IPV6_ONLY)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_handle_ip_address_or_localhost(
        hostname as *const ::core::ffi::c_char,
        &raw mut addrs,
        flags,
        &raw mut error,
    ) != 0
    {
        let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
        task = g_task_new(resolver as gpointer, cancellable, callback, user_data);
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
                safe_c2rust_lookup_by_name_async_real
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
                b"lookup_by_name_async_real\0" as *const u8 as *const gchar,
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
        if !addrs.is_null() {
            g_task_return_pointer(
                task,
                addrs as gpointer,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GList) -> ()>,
                    GDestroyNotify,
                >(Some(
                    safe_c2rust_g_resolver_free_addresses as unsafe extern "C" fn(*mut GList) -> (),
                )),
            );
        } else {
            g_task_return_error(task, error);
        }
        g_object_unref(task as gpointer);
        return;
    }
    if g_hostname_is_non_ascii(hostname) != 0 {
        ascii_hostname = g_hostname_to_ascii(hostname);
        hostname = ascii_hostname;
    }
    if hostname.is_null() {
        let mut task_0: *mut GTask = ::core::ptr::null_mut::<GTask>();
        g_set_error_literal(
            &raw mut error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid hostname\0" as *const u8 as *const gchar),
        );
        task_0 = g_task_new(resolver as gpointer, cancellable, callback, user_data);
        let mut _task_1: *mut GTask = task_0;
        g_task_set_source_tag(
            _task_1,
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
                safe_c2rust_lookup_by_name_async_real
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
        if g_task_get_name(_task_1).is_null() {
            g_task_set_static_name(
                _task_1,
                b"lookup_by_name_async_real\0" as *const u8 as *const gchar,
            );
        }
        let mut _task_2: *mut GTask = task_0;
        if 0 != 0 {
            g_task_set_static_name(
                _task_2,
                b"[gio] resolver lookup\0" as *const u8 as *const gchar,
            );
        } else {
            g_task_set_name(
                _task_2,
                b"[gio] resolver lookup\0" as *const u8 as *const gchar,
            );
        }
        g_task_return_error(task_0, error);
        g_object_unref(task_0 as gpointer);
        return;
    }
    safe_c2rust_maybe_emit_reload(resolver);
    if flags as ::core::ffi::c_uint
        != G_RESOLVER_NAME_LOOKUP_FLAGS_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
            .lookup_by_name_with_flags_async
            .is_none()
        {
            let mut task_1: *mut GTask = ::core::ptr::null_mut::<GTask>();
            g_set_error(
                &raw mut error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(b"%s not implemented\0" as *const u8 as *const gchar),
                b"lookup_by_name_with_flags_async\0" as *const u8 as *const ::core::ffi::c_char,
            );
            task_1 = g_task_new(resolver as gpointer, cancellable, callback, user_data);
            let mut _task_3: *mut GTask = task_1;
            g_task_set_source_tag(
                _task_3,
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
                    safe_c2rust_lookup_by_name_async_real
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
            if g_task_get_name(_task_3).is_null() {
                g_task_set_static_name(
                    _task_3,
                    b"lookup_by_name_async_real\0" as *const u8 as *const gchar,
                );
            }
            let mut _task_4: *mut GTask = task_1;
            if 0 != 0 {
                g_task_set_static_name(
                    _task_4,
                    b"[gio] resolver lookup\0" as *const u8 as *const gchar,
                );
            } else {
                g_task_set_name(
                    _task_4,
                    b"[gio] resolver lookup\0" as *const u8 as *const gchar,
                );
            }
            g_task_return_error(task_1, error);
            g_object_unref(task_1 as gpointer);
        } else {
            (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
                .lookup_by_name_with_flags_async
                .expect("non-null function pointer")(
                resolver,
                hostname,
                flags,
                cancellable,
                callback,
                user_data,
            );
        }
    } else {
        (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
            .lookup_by_name_async
            .expect("non-null function pointer")(
            resolver,
            hostname,
            cancellable,
            callback,
            user_data,
        );
    }
    g_free(ascii_hostname as gpointer);
}
unsafe extern "C" fn safe_c2rust_lookup_by_name_finish_real(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
    mut with_flags: gboolean,
) -> *mut GList {
    let mut addrs: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
            let mut __t: GType = g_async_result_get_type();
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
            b"G_IS_ASYNC_RESULT (result)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return ::core::ptr::null_mut::<GList>();
    } else if g_async_result_is_tagged(
        result,
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
            safe_c2rust_lookup_by_name_async_real
                as unsafe extern "C" fn(
                    *mut GResolver,
                    *const gchar,
                    GResolverNameLookupFlags,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    ) != 0
    {
        return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
            as *mut GList;
    }
    if with_flags != 0 {
        if ({
            let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
            if (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
                .lookup_by_name_with_flags_finish
                .is_some()
            {
                _g_boolean_var_21 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_21 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_21
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gresolver.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                741 as ::core::ffi::c_int,
                G_STRFUNC,
                b"G_RESOLVER_GET_CLASS (resolver)->lookup_by_name_with_flags_finish != NULL\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        addrs = (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
            .lookup_by_name_with_flags_finish
            .expect("non-null function pointer")(resolver, result, error);
    } else {
        addrs = (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
            .lookup_by_name_finish
            .expect("non-null function pointer")(resolver, result, error);
    }
    safe_c2rust_remove_duplicates(addrs);
    return addrs;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_by_name_with_flags_async(
    mut resolver: *mut GResolver,
    mut hostname: *const gchar,
    mut flags: GResolverNameLookupFlags,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    safe_c2rust_lookup_by_name_async_real(
        resolver,
        hostname,
        flags,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_by_name_async(
    mut resolver: *mut GResolver,
    mut hostname: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    safe_c2rust_lookup_by_name_async_real(
        resolver,
        hostname,
        G_RESOLVER_NAME_LOOKUP_FLAGS_DEFAULT,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_by_name_finish(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    return safe_c2rust_lookup_by_name_finish_real(resolver, result, error, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_by_name_with_flags_finish(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    return safe_c2rust_lookup_by_name_finish_real(resolver, result, error, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_free_addresses(mut addresses: *mut GList) {
    let mut a: *mut GList = ::core::ptr::null_mut::<GList>();
    a = addresses;
    while !a.is_null() {
        g_object_unref((*a).data);
        a = (*a).next;
    }
    g_list_free(addresses);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_by_address(
    mut resolver: *mut GResolver,
    mut address: *mut GInetAddress,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    safe_c2rust_maybe_emit_reload(resolver);
    return (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_by_address
        .expect("non-null function pointer")(resolver, address, cancellable, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_by_address_async(
    mut resolver: *mut GResolver,
    mut address: *mut GInetAddress,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = address as *mut GTypeInstance;
            let mut __t: GType = g_inet_address_get_type();
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
            b"G_IS_INET_ADDRESS (address)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_maybe_emit_reload(resolver);
    (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_by_address_async
        .expect("non-null function pointer")(
        resolver, address, cancellable, callback, user_data
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_by_address_finish(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return ::core::ptr::null_mut::<gchar>();
    }
    return (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_by_address_finish
        .expect("non-null function pointer")(resolver, result, error);
}
unsafe extern "C" fn safe_c2rust_g_resolver_get_service_rrname(
    mut service: *const ::core::ffi::c_char,
    mut protocol: *const ::core::ffi::c_char,
    mut domain: *const ::core::ffi::c_char,
) -> *mut gchar {
    let mut rrname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ascii_domain: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if g_hostname_is_non_ascii(domain as *const gchar) != 0 {
        ascii_domain = g_hostname_to_ascii(domain as *const gchar);
        domain = ascii_domain;
    }
    if domain.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    rrname = g_strdup_printf(
        b"_%s._%s.%s\0" as *const u8 as *const gchar,
        service,
        protocol,
        domain,
    );
    g_free(ascii_domain as gpointer);
    return rrname;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_service(
    mut resolver: *mut GResolver,
    mut service: *const gchar,
    mut protocol: *const gchar,
    mut domain: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut targets: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut rrname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !service.is_null() {
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
            b"service != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !protocol.is_null() {
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
            b"protocol != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !domain.is_null() {
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
            b"domain != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    rrname = safe_c2rust_g_resolver_get_service_rrname(
        service as *const ::core::ffi::c_char,
        protocol as *const ::core::ffi::c_char,
        domain as *const ::core::ffi::c_char,
    );
    if rrname.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid domain\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GList>();
    }
    safe_c2rust_maybe_emit_reload(resolver);
    targets = (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_service
        .expect("non-null function pointer")(resolver, rrname, cancellable, error);
    g_free(rrname as gpointer);
    return targets;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_service_async(
    mut resolver: *mut GResolver,
    mut service: *const gchar,
    mut protocol: *const gchar,
    mut domain: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut rrname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !service.is_null() {
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
            b"service != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !protocol.is_null() {
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
            b"protocol != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !domain.is_null() {
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
            b"domain != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    rrname = safe_c2rust_g_resolver_get_service_rrname(
        service as *const ::core::ffi::c_char,
        protocol as *const ::core::ffi::c_char,
        domain as *const ::core::ffi::c_char,
    );
    if rrname.is_null() {
        g_task_report_new_error(
            resolver as gpointer,
            callback,
            user_data,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GResolver,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_resolver_lookup_service_async
                    as unsafe extern "C" fn(
                        *mut GResolver,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid domain\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_maybe_emit_reload(resolver);
    (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_service_async
        .expect("non-null function pointer")(resolver, rrname, cancellable, callback, user_data);
    g_free(rrname as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_service_finish(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if g_async_result_legacy_propagate_error(result, error) != 0 {
        return ::core::ptr::null_mut::<GList>();
    }
    return (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_service_finish
        .expect("non-null function pointer")(resolver, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_free_targets(mut targets: *mut GList) {
    let mut t: *mut GList = ::core::ptr::null_mut::<GList>();
    t = targets;
    while !t.is_null() {
        g_srv_target_free((*t).data as *mut GSrvTarget);
        t = (*t).next;
    }
    g_list_free(targets);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_records(
    mut resolver: *mut GResolver,
    mut rrname: *const gchar,
    mut record_type: GResolverRecordType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GList {
    let mut records: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !rrname.is_null() {
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
            b"rrname != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    safe_c2rust_maybe_emit_reload(resolver);
    records = (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_records
        .expect("non-null function pointer")(
        resolver, rrname, record_type, cancellable, error
    );
    return records;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_records_async(
    mut resolver: *mut GResolver,
    mut rrname: *const gchar,
    mut record_type: GResolverRecordType,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !rrname.is_null() {
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
            b"rrname != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_maybe_emit_reload(resolver);
    (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_records_async
        .expect("non-null function pointer")(
        resolver,
        rrname,
        record_type,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_lookup_records_finish(
    mut resolver: *mut GResolver,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return (*((*(resolver as *mut GTypeInstance)).g_class as *mut GResolverClass))
        .lookup_records_finish
        .expect("non-null function pointer")(resolver, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_get_serial(
    mut resolver: *mut GResolver,
) -> guint64 {
    let mut result: guint64 = 0;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint64;
    }
    safe_c2rust_maybe_emit_reload(resolver);
    g_mutex_lock(&raw mut (*(*resolver).priv_0).mutex);
    result = (*(*resolver).priv_0).resolv_conf_timestamp as guint64;
    g_mutex_unlock(&raw mut (*(*resolver).priv_0).mutex);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_get_timeout(
    mut resolver: *mut GResolver,
) -> ::core::ffi::c_uint {
    let mut priv_0: *mut GResolverPrivate =
        safe_c2rust_g_resolver_get_instance_private(resolver) as *mut GResolverPrivate;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_uint;
    }
    return (*priv_0).timeout_ms;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_set_timeout(
    mut resolver: *mut GResolver,
    mut timeout_ms: ::core::ffi::c_uint,
) {
    let mut priv_0: *mut GResolverPrivate =
        safe_c2rust_g_resolver_get_instance_private(resolver) as *mut GResolverPrivate;
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = resolver as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_resolver_get_type();
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
            b"G_IS_RESOLVER (resolver)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*priv_0).timeout_ms == timeout_ms {
        return;
    }
    (*priv_0).timeout_ms = timeout_ms;
    g_object_notify_by_pspec(
        resolver as *mut ::core::ffi::c_void as *mut GObject,
        safe_c2rust_props[PROP_TIMEOUT as ::core::ffi::c_int as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_resolver_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-resolver-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
