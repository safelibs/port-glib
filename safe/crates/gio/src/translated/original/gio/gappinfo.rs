use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GVariant;
    pub type _GAppInfo;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GFile;
    pub type _GIcon;
    pub type _GTask;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_clear_error(err: *mut *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
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
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_uri_parse_scheme(uri: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_cclosure_marshal_VOID__VOID(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
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
    fn g_signal_set_va_marshaller(
        signal_id: guint,
        instance_type: GType,
        va_marshaller: GSignalCVaMarshaller,
    );
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_app_info_get_default_for_type(
        content_type: *const ::core::ffi::c_char,
        must_support_uris: gboolean,
    ) -> *mut GAppInfo;
    fn g_app_info_get_default_for_uri_scheme(
        uri_scheme: *const ::core::ffi::c_char,
    ) -> *mut GAppInfo;
    fn g_context_specific_group_get(
        group: *mut GContextSpecificGroup,
        type_0: GType,
        context_offset: goffset,
        start_func: GCallback,
    ) -> gpointer;
    fn g_context_specific_group_remove(
        group: *mut GContextSpecificGroup,
        context: *mut GMainContext,
        instance: gpointer,
        stop_func: GCallback,
    );
    fn g_context_specific_group_emit(group: *mut GContextSpecificGroup, signal_id: guint);
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
    fn g_task_set_check_cancellable(task: *mut GTask, check_cancellable: gboolean);
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_source_tag(task: *mut GTask) -> gpointer;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_new_error(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        format: *const ::core::ffi::c_char,
        ...
    );
    fn g_task_return_new_error_literal(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        message: *const ::core::ffi::c_char,
    );
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_cancellable_get_type() -> GType;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn _g_cclosure_marshal_VOID__OBJECT_VARIANT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__OBJECT_VARIANTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_io_error_quark() -> GQuark;
    fn g_file_new_for_uri(uri: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_query_default_handler(
        file: *mut GFile,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GAppInfo;
    fn g_file_query_default_handler_async(
        file: *mut GFile,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_file_query_default_handler_finish(
        file: *mut GFile,
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GAppInfo;
    fn glib_should_use_portal() -> gboolean;
    fn g_openuri_portal_open_uri(
        uri: *const ::core::ffi::c_char,
        parent_window: *const ::core::ffi::c_char,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_openuri_portal_open_uri_async(
        uri: *const ::core::ffi::c_char,
        parent_window: *const ::core::ffi::c_char,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_openuri_portal_open_uri_finish(
        result: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCond {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GCond = _GCond;
pub type GData = _GData;
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
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
pub type GVariant = _GVariant;
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
pub type GVaClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        gpointer,
        ::core::ffi::VaList,
        gpointer,
        ::core::ffi::c_int,
        *mut GType,
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
pub type GSignalCVaMarshaller = GVaClosureMarshal;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAppLaunchContext {
    pub parent_instance: GObject,
    pub priv_0: *mut GAppLaunchContextPrivate,
}
pub type GAppLaunchContextPrivate = _GAppLaunchContextPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAppLaunchContextPrivate {
    pub envp: *mut *mut ::core::ffi::c_char,
}
pub type GAppLaunchContext = _GAppLaunchContext;
pub type GAppInfo = _GAppInfo;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GFile = _GFile;
pub type GIcon = _GIcon;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAppLaunchContextClass {
    pub parent_class: GObjectClass,
    pub get_display: Option<
        unsafe extern "C" fn(
            *mut GAppLaunchContext,
            *mut GAppInfo,
            *mut GList,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub get_startup_notify_id: Option<
        unsafe extern "C" fn(
            *mut GAppLaunchContext,
            *mut GAppInfo,
            *mut GList,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub launch_failed:
        Option<unsafe extern "C" fn(*mut GAppLaunchContext, *const ::core::ffi::c_char) -> ()>,
    pub launched:
        Option<unsafe extern "C" fn(*mut GAppLaunchContext, *mut GAppInfo, *mut GVariant) -> ()>,
    pub launch_started:
        Option<unsafe extern "C" fn(*mut GAppLaunchContext, *mut GAppInfo, *mut GVariant) -> ()>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
}
pub type GAppLaunchContextClass = _GAppLaunchContextClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAppInfoIface {
    pub g_iface: GTypeInterface,
    pub dup: Option<unsafe extern "C" fn(*mut GAppInfo) -> *mut GAppInfo>,
    pub equal: Option<unsafe extern "C" fn(*mut GAppInfo, *mut GAppInfo) -> gboolean>,
    pub get_id: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub get_name: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub get_description: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub get_executable: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub get_icon: Option<unsafe extern "C" fn(*mut GAppInfo) -> *mut GIcon>,
    pub launch: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *mut GList,
            *mut GAppLaunchContext,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub supports_uris: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub supports_files: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub launch_uris: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *mut GList,
            *mut GAppLaunchContext,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub should_show: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub set_as_default_for_type: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub set_as_default_for_extension: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub add_supports_type: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_remove_supports_type: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub remove_supports_type: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_delete: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub do_delete: Option<unsafe extern "C" fn(*mut GAppInfo) -> gboolean>,
    pub get_commandline: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub get_display_name: Option<unsafe extern "C" fn(*mut GAppInfo) -> *const ::core::ffi::c_char>,
    pub set_as_last_used_for_type: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub get_supported_types:
        Option<unsafe extern "C" fn(*mut GAppInfo) -> *mut *const ::core::ffi::c_char>,
    pub launch_uris_async: Option<
        unsafe extern "C" fn(
            *mut GAppInfo,
            *mut GList,
            *mut GAppLaunchContext,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub launch_uris_finish: Option<
        unsafe extern "C" fn(*mut GAppInfo, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
}
pub type GAppInfoIface = _GAppInfoIface;
pub type GAppInfoInterface = GAppInfoIface;
pub const LAUNCHED: C2RustUnnamed_1 = 2;
pub const LAUNCH_STARTED: C2RustUnnamed_1 = 1;
pub const LAUNCH_FAILED: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DefaultForTypeData {
    pub content_type: *mut ::core::ffi::c_char,
    pub must_support_uris: gboolean,
}
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LaunchDefaultForUriData {
    pub uri: *mut gchar,
    pub context: *mut GAppLaunchContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAppInfoMonitor {
    pub parent_instance: GObject,
    pub context: *mut GMainContext,
}
pub type GAppInfoMonitor = _GAppInfoMonitor;
pub type GAppInfoMonitorClass = _GAppInfoMonitorClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAppInfoMonitorClass {
    pub parent_class: GObjectClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GContextSpecificGroup {
    pub table: *mut GHashTable,
    pub lock: GMutex,
    pub cond: GCond,
    pub requested_state: gboolean,
    pub requested_func: GCallback,
    pub effective_state: gboolean,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_1 = 3;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_VARIANT: GType = ((21 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_type() -> GType {
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
            g_intern_static_string(b"GAppInfo\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GAppInfoInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GAppInfoInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_app_info_default_init
                        as unsafe extern "C" fn(*mut GAppInfoInterface) -> (),
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
unsafe extern "C" fn safe_c2rust_g_app_info_default_init(mut iface: *mut GAppInfoInterface) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_dup(mut appinfo: *mut GAppInfo) -> *mut GAppInfo {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).dup.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_equal(
    mut appinfo1: *mut GAppInfo,
    mut appinfo2: *mut GAppInfo,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo1 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo1)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo2 as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo2)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*(appinfo1 as *mut GTypeInstance)).g_class).g_type
        != (*(*(appinfo2 as *mut GTypeInstance)).g_class).g_type
    {
        return FALSE;
    }
    iface = g_type_interface_peek(
        (*(appinfo1 as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).equal.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo1, appinfo2);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_id(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).get_id.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_name(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).get_name.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_display_name(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).get_display_name.is_none() {
        return Some((*iface).get_name.expect("non-null function pointer"))
            .expect("non-null function pointer")(appinfo);
    }
    return Some(
        (*iface)
            .get_display_name
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(appinfo);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_description(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).get_description.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_executable(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).get_executable.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_commandline(
    mut appinfo: *mut GAppInfo,
) -> *const ::core::ffi::c_char {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).get_commandline.is_some() {
        return Some((*iface).get_commandline.expect("non-null function pointer"))
            .expect("non-null function pointer")(appinfo);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_set_as_default_for_type(
    mut appinfo: *mut GAppInfo,
    mut content_type: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !content_type.is_null() {
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
            b"content_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).set_as_default_for_type.is_some() {
        return Some(
            (*iface)
                .set_as_default_for_type
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(appinfo, content_type, error);
    }
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        glib_gettext(
            b"Setting default applications not supported yet\0" as *const u8 as *const gchar,
        ),
    );
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_set_as_last_used_for_type(
    mut appinfo: *mut GAppInfo,
    mut content_type: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !content_type.is_null() {
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
            b"content_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).set_as_last_used_for_type.is_some() {
        return Some(
            (*iface)
                .set_as_last_used_for_type
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(appinfo, content_type, error);
    }
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        glib_gettext(
            b"Setting application as last used for type not supported yet\0" as *const u8
                as *const gchar,
        ),
    );
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_set_as_default_for_extension(
    mut appinfo: *mut GAppInfo,
    mut extension: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !extension.is_null() {
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
            b"extension != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).set_as_default_for_extension.is_some() {
        return Some(
            (*iface)
                .set_as_default_for_extension
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(appinfo, extension, error);
    }
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        b"g_app_info_set_as_default_for_extension not supported yet\0" as *const u8 as *const gchar,
    );
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_add_supports_type(
    mut appinfo: *mut GAppInfo,
    mut content_type: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !content_type.is_null() {
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
            b"content_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).add_supports_type.is_some() {
        return Some(
            (*iface)
                .add_supports_type
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(appinfo, content_type, error);
    }
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        b"g_app_info_add_supports_type not supported yet\0" as *const u8 as *const gchar,
    );
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_can_remove_supports_type(
    mut appinfo: *mut GAppInfo,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).can_remove_supports_type.is_some() {
        return Some(
            (*iface)
                .can_remove_supports_type
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(appinfo);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_remove_supports_type(
    mut appinfo: *mut GAppInfo,
    mut content_type: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !content_type.is_null() {
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
            b"content_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).remove_supports_type.is_some() {
        return Some(
            (*iface)
                .remove_supports_type
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(appinfo, content_type, error);
    }
    g_set_error_literal(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        b"g_app_info_remove_supports_type not supported yet\0" as *const u8 as *const gchar,
    );
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_supported_types(
    mut appinfo: *mut GAppInfo,
) -> *mut *const ::core::ffi::c_char {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).get_supported_types.is_some() {
        return (*iface)
            .get_supported_types
            .expect("non-null function pointer")(appinfo);
    } else {
        return ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_icon(mut appinfo: *mut GAppInfo) -> *mut GIcon {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).get_icon.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_launch(
    mut appinfo: *mut GAppInfo,
    mut files: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).launch.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo, files, launch_context, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_supports_uris(
    mut appinfo: *mut GAppInfo,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).supports_uris.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_supports_files(
    mut appinfo: *mut GAppInfo,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).supports_files.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_launch_uris(
    mut appinfo: *mut GAppInfo,
    mut uris: *mut GList,
    mut launch_context: *mut GAppLaunchContext,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).launch_uris.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo, uris, launch_context, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_launch_uris_async(
    mut appinfo: *mut GAppInfo,
    mut uris: *mut GList,
    mut context: *mut GAppLaunchContext,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if context.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = context as *mut GTypeInstance;
                let mut __t: GType = safe_c2rust_g_app_launch_context_get_type();
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
            b"context == NULL || G_IS_APP_LAUNCH_CONTEXT (context)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).launch_uris_async.is_none() {
        let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
        task = g_task_new(appinfo as gpointer, cancellable, callback, user_data);
        let mut _task: *mut GTask = task;
        g_task_set_source_tag(
            _task,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GAppInfo,
                        *mut GList,
                        *mut GAppLaunchContext,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_app_info_launch_uris_async
                    as unsafe extern "C" fn(
                        *mut GAppInfo,
                        *mut GList,
                        *mut GAppLaunchContext,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        );
        if g_task_get_name(_task).is_null() {
            g_task_set_static_name(
                _task,
                b"g_app_info_launch_uris_async\0" as *const u8 as *const gchar,
            );
        }
        g_task_return_new_error_literal(
            task,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            b"Operation not supported for the current backend.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        g_object_unref(task as gpointer);
        return;
    }
    Some(
        (*iface)
            .launch_uris_async
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        appinfo, uris, context, cancellable, callback, user_data
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_launch_uris_finish(
    mut appinfo: *mut GAppInfo,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).launch_uris_finish.is_none() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            b"Operation not supported for the current backend.\0" as *const u8 as *const gchar,
        );
        return FALSE;
    }
    return Some(
        (*iface)
            .launch_uris_finish
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(appinfo, result, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_should_show(
    mut appinfo: *mut GAppInfo,
) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    return Some((*iface).should_show.expect("non-null function pointer"))
        .expect("non-null function pointer")(appinfo);
}
unsafe extern "C" fn safe_c2rust_default_for_type_data_free(mut data: *mut DefaultForTypeData) {
    g_free((*data).content_type as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_get_default_for_type_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut DefaultForTypeData = task_data as *mut DefaultForTypeData;
    let mut info: *mut GAppInfo = ::core::ptr::null_mut::<GAppInfo>();
    info = g_app_info_get_default_for_type((*data).content_type, (*data).must_support_uris);
    if info.is_null() {
        g_task_return_new_error(
            task,
            g_io_error_quark(),
            G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Failed to find default application for content type \xE2\x80\x98%s\xE2\x80\x99\0"
                    as *const u8 as *const gchar,
            ) as *const ::core::ffi::c_char,
            (*data).content_type,
        );
        return;
    }
    g_task_return_pointer(
        task,
        safe_c2rust_g_steal_pointer(&raw mut info as gpointer) as *mut GAppInfo as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_default_for_type_async(
    mut content_type: *const ::core::ffi::c_char,
    mut must_support_uris: gboolean,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut DefaultForTypeData = ::core::ptr::null_mut::<DefaultForTypeData>();
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !content_type.is_null() && *content_type as ::core::ffi::c_int != '\0' as i32 {
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
            b"content_type != NULL && *content_type != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<DefaultForTypeData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut DefaultForTypeData;
    (*data).content_type = safe_c2rust_g_strdup_inline(content_type);
    (*data).must_support_uris = must_support_uris;
    task = g_task_new(NULL_0, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    gboolean,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_app_info_get_default_for_type_async
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    gboolean,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_app_info_get_default_for_type_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut DefaultForTypeData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_default_for_type_data_free
                as unsafe extern "C" fn(*mut DefaultForTypeData) -> (),
        )),
    );
    g_task_set_check_cancellable(task, TRUE);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_get_default_for_type_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_get_default_for_scheme_thread(
    mut task: *mut GTask,
    mut object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut uri_scheme: *const ::core::ffi::c_char = task_data as *const ::core::ffi::c_char;
    let mut info: *mut GAppInfo = ::core::ptr::null_mut::<GAppInfo>();
    info = g_app_info_get_default_for_uri_scheme(uri_scheme);
    if info.is_null() {
        g_task_return_new_error(
            task,
            g_io_error_quark(),
            G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Failed to find default application for URI Scheme \xE2\x80\x98%s\xE2\x80\x99\0"
                    as *const u8 as *const gchar,
            ) as *const ::core::ffi::c_char,
            uri_scheme,
        );
        return;
    }
    g_task_return_pointer(
        task,
        safe_c2rust_g_steal_pointer(&raw mut info as gpointer) as *mut GAppInfo as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_default_for_uri_scheme_async(
    mut uri_scheme: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !uri_scheme.is_null() && *uri_scheme as ::core::ffi::c_int != '\0' as i32 {
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
            b"uri_scheme != NULL && *uri_scheme != '\\0'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    task = g_task_new(NULL_0, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_app_info_get_default_for_uri_scheme_async
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_app_info_get_default_for_uri_scheme_async\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        safe_c2rust_g_strdup_inline(uri_scheme) as gpointer,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_task_set_check_cancellable(task, TRUE);
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_get_default_for_scheme_thread
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_default_for_uri_scheme_finish(
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GAppInfo {
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if g_task_is_valid(
            result as gpointer,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) != 0
        {
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
            b"g_task_is_valid (result, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if g_task_get_source_tag(result as *mut ::core::ffi::c_void as *mut GTask)
            == ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_app_info_get_default_for_uri_scheme_async
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            ))
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
            b"g_task_get_source_tag (G_TASK (result)) == g_app_info_get_default_for_uri_scheme_async\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GAppInfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_get_default_for_type_finish(
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GAppInfo {
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if g_task_is_valid(
            result as gpointer,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) != 0
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
            b"g_task_is_valid (result, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if g_task_get_source_tag(result as *mut ::core::ffi::c_void as *mut GTask)
            == ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        gboolean,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_app_info_get_default_for_type_async
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        gboolean,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            ))
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
            b"g_task_get_source_tag (G_TASK (result)) == g_app_info_get_default_for_type_async\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAppInfo>();
    }
    return g_task_propagate_pointer(result as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GAppInfo;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_launch_default_for_uri(
    mut uri: *const ::core::ffi::c_char,
    mut launch_context: *mut GAppLaunchContext,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut uri_scheme: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut app_info: *mut GAppInfo = ::core::ptr::null_mut::<GAppInfo>();
    let mut res: gboolean = FALSE;
    uri_scheme = g_uri_parse_scheme(uri);
    if !uri_scheme.is_null()
        && *uri_scheme.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        app_info = g_app_info_get_default_for_uri_scheme(uri_scheme);
    }
    g_free(uri_scheme as gpointer);
    if app_info.is_null() {
        let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
        file = g_file_new_for_uri(uri);
        app_info =
            g_file_query_default_handler(file, ::core::ptr::null_mut::<GCancellable>(), error);
        g_object_unref(file as gpointer);
    }
    if !app_info.is_null() {
        let mut l: GList = _GList {
            data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            next: ::core::ptr::null_mut::<GList>(),
            prev: ::core::ptr::null_mut::<GList>(),
        };
        l.data = uri as *mut ::core::ffi::c_char as gpointer;
        l.prev = ::core::ptr::null_mut::<GList>();
        l.next = l.prev;
        res = safe_c2rust_g_app_info_launch_uris(app_info, &raw mut l, launch_context, error);
        g_object_unref(app_info as gpointer);
    }
    if res == 0 && glib_should_use_portal() != 0 {
        let mut parent_window: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        g_clear_error(error);
        if !launch_context.is_null() && !(*(*launch_context).priv_0).envp.is_null() {
            parent_window = g_environ_getenv(
                (*(*launch_context).priv_0).envp as *mut *mut gchar,
                b"PARENT_WINDOW_ID\0" as *const u8 as *const gchar,
            ) as *const ::core::ffi::c_char;
        }
        return g_openuri_portal_open_uri(uri, parent_window, error);
    }
    return res;
}
unsafe extern "C" fn safe_c2rust_launch_default_for_uri_data_free(
    mut data: *mut LaunchDefaultForUriData,
) {
    g_free((*data).uri as gpointer);
    let mut _pp: *mut *mut GAppLaunchContext = &raw mut (*data).context;
    let mut _ptr: *mut GAppLaunchContext = *_pp;
    *_pp = ::core::ptr::null_mut::<GAppLaunchContext>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_launch_default_for_uri_portal_open_uri_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if g_openuri_portal_open_uri_finish(result, &raw mut error) != 0 {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(
            task,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_launch_default_for_uri_portal_open_uri(
    mut task: *mut GTask,
    mut error: *mut GError,
) {
    let mut data: *mut LaunchDefaultForUriData =
        g_task_get_task_data(task) as *mut LaunchDefaultForUriData;
    let mut cancellable: *mut GCancellable = g_task_get_cancellable(task);
    if glib_should_use_portal() != 0 {
        let mut parent_window: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        g_error_free(error);
        if !(*data).context.is_null() && !(*(*(*data).context).priv_0).envp.is_null() {
            parent_window = g_environ_getenv(
                (*(*(*data).context).priv_0).envp as *mut *mut gchar,
                b"PARENT_WINDOW_ID\0" as *const u8 as *const gchar,
            ) as *const ::core::ffi::c_char;
        }
        g_openuri_portal_open_uri_async(
            (*data).uri,
            parent_window,
            cancellable,
            Some(
                safe_c2rust_launch_default_for_uri_portal_open_uri_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
        );
        return;
    }
    g_task_return_error(
        task,
        safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_launch_default_for_uri_launch_uris_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut app_info: *mut GAppInfo = object as *mut ::core::ffi::c_void as *mut GAppInfo;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_app_info_launch_uris_finish(app_info, result, &raw mut error) != 0 {
        g_task_return_boolean(task, TRUE);
        g_object_unref(task as gpointer);
    } else {
        safe_c2rust_launch_default_for_uri_portal_open_uri(
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    };
}
unsafe extern "C" fn safe_c2rust_launch_default_for_uri_launch_uris(
    mut task: *mut GTask,
    mut app_info: *mut GAppInfo,
) {
    let mut cancellable: *mut GCancellable = g_task_get_cancellable(task);
    let mut l: GList = _GList {
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        next: ::core::ptr::null_mut::<GList>(),
        prev: ::core::ptr::null_mut::<GList>(),
    };
    let mut data: *mut LaunchDefaultForUriData =
        g_task_get_task_data(task) as *mut LaunchDefaultForUriData;
    l.data = (*data).uri as *mut ::core::ffi::c_char as gpointer;
    l.prev = ::core::ptr::null_mut::<GList>();
    l.next = l.prev;
    safe_c2rust_g_app_info_launch_uris_async(
        app_info,
        &raw mut l,
        (*data).context,
        cancellable,
        Some(
            safe_c2rust_launch_default_for_uri_launch_uris_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
    );
    g_object_unref(app_info as gpointer);
}
unsafe extern "C" fn safe_c2rust_launch_default_for_uri_default_handler_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut file: *mut GFile = object as *mut ::core::ffi::c_void as *mut GFile;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut app_info: *mut GAppInfo = ::core::ptr::null_mut::<GAppInfo>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    app_info = g_file_query_default_handler_finish(file, result, &raw mut error);
    if !app_info.is_null() {
        safe_c2rust_launch_default_for_uri_launch_uris(
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask,
            safe_c2rust_g_steal_pointer(&raw mut app_info as gpointer) as *mut GAppInfo,
        );
    } else {
        safe_c2rust_launch_default_for_uri_portal_open_uri(
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask,
            safe_c2rust_g_steal_pointer(&raw mut error as gpointer) as *mut GError,
        );
    };
}
unsafe extern "C" fn safe_c2rust_launch_default_app_for_default_handler(mut task: *mut GTask) {
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut cancellable: *mut GCancellable = ::core::ptr::null_mut::<GCancellable>();
    let mut data: *mut LaunchDefaultForUriData = ::core::ptr::null_mut::<LaunchDefaultForUriData>();
    data = g_task_get_task_data(task) as *mut LaunchDefaultForUriData;
    cancellable = g_task_get_cancellable(task);
    file = g_file_new_for_uri((*data).uri);
    g_file_query_default_handler_async(
        file,
        G_PRIORITY_DEFAULT,
        cancellable,
        Some(
            safe_c2rust_launch_default_for_uri_default_handler_cb
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
    );
    g_object_unref(file as gpointer);
}
unsafe extern "C" fn safe_c2rust_launch_default_app_for_uri_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut app_info: *mut GAppInfo = ::core::ptr::null_mut::<GAppInfo>();
    app_info = safe_c2rust_g_app_info_get_default_for_uri_scheme_finish(
        result,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if app_info.is_null() {
        safe_c2rust_launch_default_app_for_default_handler(safe_c2rust_g_steal_pointer(
            &raw mut task as gpointer,
        ) as *mut GTask);
    } else {
        safe_c2rust_launch_default_for_uri_launch_uris(
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask,
            safe_c2rust_g_steal_pointer(&raw mut app_info as gpointer) as *mut GAppInfo,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_launch_default_for_uri_async(
    mut uri: *const ::core::ffi::c_char,
    mut context: *mut GAppLaunchContext,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut uri_scheme: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut LaunchDefaultForUriData = ::core::ptr::null_mut::<LaunchDefaultForUriData>();
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !uri.is_null() {
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
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    task = g_task_new(NULL_0, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut GAppLaunchContext,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_app_info_launch_default_for_uri_async
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut GAppLaunchContext,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_app_info_launch_default_for_uri_async\0" as *const u8 as *const gchar,
        );
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<LaunchDefaultForUriData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut LaunchDefaultForUriData;
    (*data).uri = safe_c2rust_g_strdup_inline(uri) as *mut gchar;
    (*data).context = (if !context.is_null() {
        g_object_ref(context as gpointer) as *mut GAppLaunchContext
    } else {
        ::core::ptr::null_mut::<GAppLaunchContext>()
    }) as *mut GAppLaunchContext;
    g_task_set_task_data(
        task,
        safe_c2rust_g_steal_pointer(&raw mut data as gpointer) as *mut LaunchDefaultForUriData
            as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut LaunchDefaultForUriData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_launch_default_for_uri_data_free
                as unsafe extern "C" fn(*mut LaunchDefaultForUriData) -> (),
        )),
    );
    uri_scheme = g_uri_parse_scheme(uri);
    if !uri_scheme.is_null()
        && *uri_scheme.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
    {
        safe_c2rust_g_app_info_get_default_for_uri_scheme_async(
            uri_scheme,
            cancellable,
            Some(
                safe_c2rust_launch_default_app_for_uri_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
        );
    } else {
        safe_c2rust_launch_default_app_for_default_handler(safe_c2rust_g_steal_pointer(
            &raw mut task as gpointer,
        ) as *mut GTask);
    }
    g_free(uri_scheme as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_launch_default_for_uri_finish(
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if g_task_is_valid(
            result as gpointer,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) != 0
        {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (result, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_can_delete(mut appinfo: *mut GAppInfo) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).can_delete.is_some() {
        return Some((*iface).can_delete.expect("non-null function pointer"))
            .expect("non-null function pointer")(appinfo);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_delete(mut appinfo: *mut GAppInfo) -> gboolean {
    let mut iface: *mut GAppInfoIface = ::core::ptr::null_mut::<GAppInfoIface>();
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = appinfo as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            b"G_IS_APP_INFO (appinfo)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    iface = g_type_interface_peek(
        (*(appinfo as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_app_info_get_type(),
    ) as *mut GAppInfoIface;
    if (*iface).do_delete.is_some() {
        return Some((*iface).do_delete.expect("non-null function pointer"))
            .expect("non-null function pointer")(appinfo);
    }
    return FALSE;
}
static mut safe_c2rust_signals: [guint; 3] = [0 as ::core::ffi::c_int as guint, 0, 0];
static mut safe_c2rust_g_app_launch_context_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_app_launch_context_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GAppLaunchContext\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GAppLaunchContextClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_app_launch_context_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GAppLaunchContext>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GAppLaunchContext) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_app_launch_context_init
                    as unsafe extern "C" fn(*mut GAppLaunchContext) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GAppLaunchContext_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GAppLaunchContextPrivate>() as gsize,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_app_launch_context_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_app_launch_context_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GAppLaunchContext_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GAppLaunchContext_private_offset,
        );
    }
    safe_c2rust_g_app_launch_context_class_init(klass as *mut GAppLaunchContextClass);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_app_launch_context_get_instance_private(
    mut self_0: *mut GAppLaunchContext,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GAppLaunchContext_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GAppLaunchContext_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_launch_context_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_app_launch_context_get_type_once();
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_launch_context_new() -> *mut GAppLaunchContext {
    return g_object_new(
        safe_c2rust_g_app_launch_context_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GAppLaunchContext;
}
unsafe extern "C" fn safe_c2rust_g_app_launch_context_finalize(mut object: *mut GObject) {
    let mut context: *mut GAppLaunchContext =
        object as *mut ::core::ffi::c_void as *mut GAppLaunchContext;
    g_strfreev((*(*context).priv_0).envp as *mut *mut gchar);
    (*(safe_c2rust_g_app_launch_context_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_app_launch_context_class_init(
    mut klass: *mut GAppLaunchContextClass,
) {
    let mut object_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_app_launch_context_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    safe_c2rust_signals[LAUNCH_FAILED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"launch-failed\0" as *const u8 as *const gchar),
        (*(object_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        152 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        None,
        G_TYPE_NONE,
        1 as guint,
        G_TYPE_STRING,
    );
    safe_c2rust_signals[LAUNCH_STARTED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"launch-started\0" as *const u8 as *const gchar),
        (*(object_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        168 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__OBJECT_VARIANT
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_NONE,
        2 as guint,
        safe_c2rust_g_app_info_get_type(),
        G_TYPE_VARIANT,
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[LAUNCH_STARTED as ::core::ffi::c_int as usize],
        (*(klass as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__OBJECT_VARIANTv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
    safe_c2rust_signals[LAUNCHED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"launched\0" as *const u8 as *const gchar),
        (*(object_class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        160 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__OBJECT_VARIANT
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_NONE,
        2 as guint,
        safe_c2rust_g_app_info_get_type(),
        G_TYPE_VARIANT,
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[LAUNCHED as ::core::ffi::c_int as usize],
        (*(klass as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__OBJECT_VARIANTv
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    gpointer,
                    ::core::ffi::VaList,
                    gpointer,
                    ::core::ffi::c_int,
                    *mut GType,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_app_launch_context_init(mut context: *mut GAppLaunchContext) {
    (*context).priv_0 = safe_c2rust_g_app_launch_context_get_instance_private(context)
        as *mut GAppLaunchContextPrivate;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_launch_context_setenv(
    mut context: *mut GAppLaunchContext,
    mut variable: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = context as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_launch_context_get_type();
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
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APP_LAUNCH_CONTEXT (context)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !variable.is_null() {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"variable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*context).priv_0).envp.is_null() {
        (*(*context).priv_0).envp = g_get_environ() as *mut *mut ::core::ffi::c_char;
    }
    (*(*context).priv_0).envp = g_environ_setenv(
        (*(*context).priv_0).envp as *mut *mut gchar,
        variable as *const gchar,
        value as *const gchar,
        TRUE,
    ) as *mut *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_launch_context_unsetenv(
    mut context: *mut GAppLaunchContext,
    mut variable: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = context as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_launch_context_get_type();
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
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APP_LAUNCH_CONTEXT (context)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !variable.is_null() {
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"variable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*context).priv_0).envp.is_null() {
        (*(*context).priv_0).envp = g_get_environ() as *mut *mut ::core::ffi::c_char;
    }
    (*(*context).priv_0).envp = g_environ_unsetenv(
        (*(*context).priv_0).envp as *mut *mut gchar,
        variable as *const gchar,
    ) as *mut *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_launch_context_get_environment(
    mut context: *mut GAppLaunchContext,
) -> *mut *mut ::core::ffi::c_char {
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = context as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_launch_context_get_type();
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
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APP_LAUNCH_CONTEXT (context)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    if (*(*context).priv_0).envp.is_null() {
        (*(*context).priv_0).envp = g_get_environ() as *mut *mut ::core::ffi::c_char;
    }
    return g_strdupv((*(*context).priv_0).envp as *mut *mut gchar)
        as *mut *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_launch_context_get_display(
    mut context: *mut GAppLaunchContext,
    mut info: *mut GAppInfo,
    mut files: *mut GList,
) -> *mut ::core::ffi::c_char {
    let mut class: *mut GAppLaunchContextClass = ::core::ptr::null_mut::<GAppLaunchContextClass>();
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = context as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_launch_context_get_type();
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
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APP_LAUNCH_CONTEXT (context)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    class = (*(context as *mut GTypeInstance)).g_class as *mut GAppLaunchContextClass;
    if (*class).get_display.is_none() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return (*class).get_display.expect("non-null function pointer")(context, info, files);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_launch_context_get_startup_notify_id(
    mut context: *mut GAppLaunchContext,
    mut info: *mut GAppInfo,
    mut files: *mut GList,
) -> *mut ::core::ffi::c_char {
    let mut class: *mut GAppLaunchContextClass = ::core::ptr::null_mut::<GAppLaunchContextClass>();
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = context as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_launch_context_get_type();
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
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APP_LAUNCH_CONTEXT (context)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = info as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_info_get_type();
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
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APP_INFO (info)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    class = (*(context as *mut GTypeInstance)).g_class as *mut GAppLaunchContextClass;
    if (*class).get_startup_notify_id.is_none() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return (*class)
        .get_startup_notify_id
        .expect("non-null function pointer")(context, info, files);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_launch_context_launch_failed(
    mut context: *mut GAppLaunchContext,
    mut startup_notify_id: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = context as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_app_launch_context_get_type();
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
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APP_LAUNCH_CONTEXT (context)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if !startup_notify_id.is_null() {
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"startup_notify_id != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_signal_emit(
        context as gpointer,
        safe_c2rust_signals[LAUNCH_FAILED as ::core::ffi::c_int as usize],
        0 as GQuark,
        startup_notify_id,
    );
}
static mut safe_c2rust_g_app_info_monitor_group: GContextSpecificGroup = GContextSpecificGroup {
    table: ::core::ptr::null::<GHashTable>() as *mut GHashTable,
    lock: _GMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    cond: _GCond {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        i: [0; 2],
    },
    requested_state: 0,
    requested_func: None,
    effective_state: 0,
};
static mut safe_c2rust_g_app_info_monitor_changed_signal: guint = 0;
static mut safe_c2rust_GAppInfoMonitor_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_monitor_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_app_info_monitor_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_app_info_monitor_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GAppInfoMonitor\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GAppInfoMonitorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_app_info_monitor_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GAppInfoMonitor>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GAppInfoMonitor) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_app_info_monitor_init
                    as unsafe extern "C" fn(*mut GAppInfoMonitor) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_app_info_monitor_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_app_info_monitor_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GAppInfoMonitor_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GAppInfoMonitor_private_offset,
        );
    }
    safe_c2rust_g_app_info_monitor_class_init(klass as *mut GAppInfoMonitorClass);
}
static mut safe_c2rust_g_app_info_monitor_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_app_info_monitor_finalize(mut object: *mut GObject) {
    let mut monitor: *mut GAppInfoMonitor =
        object as *mut ::core::ffi::c_void as *mut GAppInfoMonitor;
    g_context_specific_group_remove(
        &raw mut safe_c2rust_g_app_info_monitor_group,
        (*monitor).context,
        monitor as gpointer,
        None,
    );
    (*(safe_c2rust_g_app_info_monitor_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_app_info_monitor_init(mut monitor: *mut GAppInfoMonitor) {}
unsafe extern "C" fn safe_c2rust_g_app_info_monitor_class_init(
    mut class: *mut GAppInfoMonitorClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    safe_c2rust_g_app_info_monitor_changed_signal = g_signal_new(
        g_intern_static_string(b"changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_app_info_monitor_get_type(),
        G_SIGNAL_RUN_FIRST,
        0 as guint,
        None,
        NULL_0,
        Some(
            g_cclosure_marshal_VOID__VOID
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_NONE,
        0 as guint,
    );
    (*object_class).finalize =
        Some(safe_c2rust_g_app_info_monitor_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_monitor_get() -> *mut GAppInfoMonitor {
    return g_context_specific_group_get(
        &raw mut safe_c2rust_g_app_info_monitor_group,
        safe_c2rust_g_app_info_monitor_get_type(),
        24 as ::core::ffi::c_ulong as goffset,
        None,
    ) as *mut GAppInfoMonitor;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_app_info_monitor_fire() {
    g_context_specific_group_emit(
        &raw mut safe_c2rust_g_app_info_monitor_group,
        safe_c2rust_g_app_info_monitor_changed_signal,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
