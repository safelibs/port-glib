use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GWakeup;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new_literal(domain: GQuark, code: gint, message: *const gchar) -> *mut GError;
    fn g_error_new_valist(
        domain: GQuark,
        code: gint,
        format: *const gchar,
        args: ::core::ffi::VaList,
    ) -> *mut GError;
    fn g_error_free(error: *mut GError);
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_prefix_error_literal(err: *mut *mut GError, prefix: *const gchar);
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_cond_init(cond: *mut GCond);
    fn g_cond_clear(cond: *mut GCond);
    fn g_cond_wait(cond: *mut GCond, mutex: *mut GMutex);
    fn g_cond_signal(cond: *mut GCond);
    fn g_private_get(key: *mut GPrivate) -> gpointer;
    fn g_private_set(key: *mut GPrivate, value: gpointer);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_current_source() -> *mut GSource;
    fn g_main_context_push_thread_default(context: *mut GMainContext);
    fn g_main_context_pop_thread_default(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_source_new(source_funcs: *mut GSourceFuncs, struct_size: guint) -> *mut GSource;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_get_context(source: *mut GSource) -> *mut GMainContext;
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_get_name(source: *mut GSource) -> *const ::core::ffi::c_char;
    fn g_source_set_ready_time(source: *mut GSource, ready_time: gint64);
    fn g_source_get_time(source: *mut GSource) -> gint64;
    fn g_idle_source_new() -> *mut GSource;
    fn g_get_monotonic_time() -> gint64;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
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
    fn g_thread_pool_new(
        func: GFunc,
        user_data: gpointer,
        max_threads: gint,
        exclusive: gboolean,
        error: *mut *mut GError,
    ) -> *mut GThreadPool;
    fn g_thread_pool_push(
        pool: *mut GThreadPool,
        data: gpointer,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_thread_pool_unprocessed(pool: *mut GThreadPool) -> guint;
    fn g_thread_pool_set_sort_function(
        pool: *mut GThreadPool,
        func: GCompareDataFunc,
        user_data: gpointer,
    );
    fn g_thread_pool_move_to_front(pool: *mut GThreadPool, data: gpointer) -> gboolean;
    fn g_thread_pool_set_max_threads(
        pool: *mut GThreadPool,
        max_threads: gint,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_copy(src_value: *const GValue, dest_value: *mut GValue);
    fn g_value_unset(value: *mut GValue);
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handlers_disconnect_matched(
        instance: gpointer,
        mask: GSignalMatchType,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        func: gpointer,
        data: gpointer,
    ) -> guint;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_set_pointer(value: *mut GValue, v_pointer: gpointer);
    fn g_async_result_get_type() -> GType;
    fn g_cancellable_is_cancelled(cancellable: *mut GCancellable) -> gboolean;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn glib__private__() -> *const GLibPrivateVTable;
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
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
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
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPrivate {
    pub p: gpointer,
    pub notify: GDestroyNotify,
    pub future: [gpointer; 2],
}
pub type GPrivate = _GPrivate;
pub type GData = _GData;
pub type GDir = _GDir;
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
pub type GSignalMatchType = ::core::ffi::c_uint;
pub const G_SIGNAL_MATCH_UNBLOCKED: GSignalMatchType = 32;
pub const G_SIGNAL_MATCH_DATA: GSignalMatchType = 16;
pub const G_SIGNAL_MATCH_FUNC: GSignalMatchType = 8;
pub const G_SIGNAL_MATCH_CLOSURE: GSignalMatchType = 4;
pub const G_SIGNAL_MATCH_DETAIL: GSignalMatchType = 2;
pub const G_SIGNAL_MATCH_ID: GSignalMatchType = 1;
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
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GTask {
    pub parent_instance: GObject,
    pub source_object: gpointer,
    pub source_tag: gpointer,
    pub name: *mut gchar,
    pub task_data: gpointer,
    pub task_data_destroy: GDestroyNotify,
    pub context: *mut GMainContext,
    pub creation_time: gint64,
    pub priority: gint,
    pub cancellable: *mut GCancellable,
    pub callback: GAsyncReadyCallback,
    pub callback_data: gpointer,
    pub task_func: GTaskThreadFunc,
    pub lock: GMutex,
    pub cond: GCond,
    pub thread_cancelled: gboolean,
    #[bitfield(name = "thread_complete", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "return_on_cancel", ty = "guint", bits = "1..=1")]
    pub thread_complete_return_on_cancel: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    #[bitfield(name = "completed", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "had_error", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "result_set", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "ever_returned", ty = "guint", bits = "3..=3")]
    pub completed_had_error_result_set_ever_returned: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 3],
    #[bitfield(name = "check_cancellable", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "synchronous", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "blocking_other_task", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "name_is_static", ty = "guint", bits = "3..=3")]
    pub check_cancellable_synchronous_blocking_other_task_name_is_static: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
    pub error: *mut GError,
    pub result: C2RustUnnamed_0,
    pub result_destroy: GDestroyNotify,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub pointer: gpointer,
    pub size: gssize,
    pub boolean: gboolean,
}
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTaskClass {
    pub parent_class: GObjectClass,
}
pub type GTaskClass = _GTaskClass;
pub const PROP_COMPLETED: GTaskProperty = 1;
pub type GTaskProperty = ::core::ffi::c_uint;
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
pub type GTaskReturnType = ::core::ffi::c_uint;
pub const G_TASK_RETURN_FROM_THREAD: GTaskReturnType = 2;
pub const G_TASK_RETURN_ERROR: GTaskReturnType = 1;
pub const G_TASK_RETURN_SUCCESS: GTaskReturnType = 0;
pub type GAsyncResultIface = _GAsyncResultIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAsyncResultIface {
    pub g_iface: GTypeInterface,
    pub get_user_data: Option<unsafe extern "C" fn(*mut GAsyncResult) -> gpointer>,
    pub get_source_object: Option<unsafe extern "C" fn(*mut GAsyncResult) -> *mut GObject>,
    pub is_tagged: Option<unsafe extern "C" fn(*mut GAsyncResult, gpointer) -> gboolean>,
}
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
pub const G_TYPE_POINTER: GType = ((17 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_task_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_task_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GTask\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GTaskClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_task_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GTask>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GTask) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_task_init as unsafe extern "C" fn(*mut GTask) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GAsyncResultIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_task_async_result_iface_init
                as unsafe extern "C" fn(*mut GAsyncResultIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_async_result_get_type(),
        &raw const g_implement_interface_info,
    );
    safe_c2rust_g_task_thread_pool_init();
    return g_define_type_id;
}
static mut safe_c2rust_GTask_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_task_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_task_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GTask_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GTask_private_offset);
    }
    safe_c2rust_g_task_class_init(klass as *mut GTaskClass);
}
static mut safe_c2rust_g_task_parent_class: gpointer = NULL_0;
static mut safe_c2rust_task_pool: *mut GThreadPool =
    ::core::ptr::null::<GThreadPool>() as *mut GThreadPool;
static mut safe_c2rust_task_pool_mutex: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_task_private: GPrivate = _GPrivate {
    p: NULL_0,
    notify: None,
    future: [NULL_0, NULL_0],
};
static mut safe_c2rust_task_pool_manager: *mut GSource =
    ::core::ptr::null::<GSource>() as *mut GSource;
static mut safe_c2rust_task_wait_time: guint64 = 0;
static mut safe_c2rust_tasks_running: gint = 0;
static mut safe_c2rust_task_pool_max_counter: guint = 0;
static mut safe_c2rust_tasks_running_counter: guint = 0;
pub const G_TASK_POOL_SIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const G_TASK_WAIT_TIME_BASE: ::core::ffi::c_int = 100000 as ::core::ffi::c_int;
pub const G_TASK_WAIT_TIME_MULTIPLIER: ::core::ffi::c_double = 1.03f64;
pub const G_TASK_WAIT_TIME_MAX_POOL_SIZE: ::core::ffi::c_int = 330 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_task_init(mut task: *mut GTask) {
    (*task).set_check_cancellable(TRUE as guint as guint);
}
unsafe extern "C" fn safe_c2rust_g_task_finalize(mut object: *mut GObject) {
    let mut task: *mut GTask = object as *mut ::core::ffi::c_void as *mut GTask;
    if (*task).ever_returned() == 0 {
        let mut owned_task_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut task_name: *const gchar = safe_c2rust_g_task_get_name(task);
        if task_name.is_null() {
            owned_task_name = g_strdup_printf(b"%p\0" as *const u8 as *const gchar, task);
            task_name = owned_task_name;
        }
        if (*task).callback.is_some() && (*task).task_func.is_none() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"GTask %s (source object: %p, source tag: %p) finalized without ever returning (using g_task_return_*()). This potentially indicates a bug in the program.\0"
                    as *const u8 as *const gchar,
                task_name,
                (*task).source_object,
                (*task).source_tag,
            );
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_DEBUG,
                b"GTask %s (source object: %p, source tag: %p) finalized without ever returning (using g_task_return_*()). This potentially indicates a bug in the program.\0"
                    as *const u8 as *const gchar,
                task_name,
                (*task).source_object,
                (*task).source_tag,
            );
        }
        g_free(owned_task_name as gpointer);
    }
    let mut _pp: *mut gpointer = &raw mut (*task).source_object;
    let mut _ptr: gpointer = *_pp;
    *_pp = NULL_0 as gpointer;
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GCancellable = &raw mut (*task).cancellable;
    let mut _ptr_0: *mut GCancellable = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GCancellable>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    if (*task).name_is_static() == 0 {
        g_free((*task).name as gpointer);
    }
    if !(*task).context.is_null() {
        g_main_context_unref((*task).context);
    }
    if (*task).task_data_destroy.is_some() {
        (*task)
            .task_data_destroy
            .expect("non-null function pointer")((*task).task_data);
    }
    if (*task).result_destroy.is_some() && !(*task).result.pointer.is_null() {
        (*task).result_destroy.expect("non-null function pointer")((*task).result.pointer);
    }
    if !(*task).error.is_null() {
        g_error_free((*task).error);
    }
    if (*task).task_func.is_some() {
        g_mutex_clear(&raw mut (*task).lock);
        g_cond_clear(&raw mut (*task).cond);
    }
    (*(safe_c2rust_g_task_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_new(
    mut source_object: gpointer,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut callback_data: gpointer,
) -> *mut GTask {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    task = g_object_new(safe_c2rust_g_task_get_type(), ::core::ptr::null::<gchar>()) as *mut GTask;
    (*task).source_object = (if !source_object.is_null() {
        g_object_ref(source_object)
    } else {
        NULL_0
    }) as gpointer;
    (*task).cancellable = (if !cancellable.is_null() {
        g_object_ref(cancellable as gpointer) as *mut GCancellable
    } else {
        ::core::ptr::null_mut::<GCancellable>()
    }) as *mut GCancellable;
    (*task).callback = callback;
    (*task).callback_data = callback_data;
    (*task).context = g_main_context_ref_thread_default();
    source = g_main_current_source();
    if !source.is_null() {
        (*task).creation_time = g_source_get_time(source);
    }
    return task;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_report_error(
    mut source_object: gpointer,
    mut callback: GAsyncReadyCallback,
    mut callback_data: gpointer,
    mut source_tag: gpointer,
    mut error: *mut GError,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = safe_c2rust_g_task_new(
        source_object,
        ::core::ptr::null_mut::<GCancellable>(),
        callback,
        callback_data,
    );
    let mut _task: *mut GTask = task;
    safe_c2rust_g_task_set_source_tag(_task, source_tag);
    if safe_c2rust_g_task_get_name(_task).is_null() {
        safe_c2rust_g_task_set_static_name(
            _task,
            b"source_tag\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_g_task_set_static_name(task, G_STRFUNC);
    safe_c2rust_g_task_return_error(task, error);
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_report_new_error(
    mut source_object: gpointer,
    mut callback: GAsyncReadyCallback,
    mut callback_data: gpointer,
    mut source_tag: gpointer,
    mut domain: GQuark,
    mut code: gint,
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut ap: ::core::ffi::VaList;
    ap = args.clone();
    error = g_error_new_valist(domain, code, format as *const gchar, ap);
    safe_c2rust_g_task_report_error(source_object, callback, callback_data, source_tag, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_set_task_data(
    mut task: *mut GTask,
    mut task_data: gpointer,
    mut task_data_destroy: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*task).task_data_destroy.is_some() {
        (*task)
            .task_data_destroy
            .expect("non-null function pointer")((*task).task_data);
    }
    (*task).task_data = task_data;
    (*task).task_data_destroy = task_data_destroy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_set_priority(mut task: *mut GTask, mut priority: gint) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*task).priority = priority;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_set_check_cancellable(
    mut task: *mut GTask,
    mut check_cancellable: gboolean,
) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if check_cancellable != 0 || (*task).return_on_cancel() == 0 {
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
            b"check_cancellable || !task->return_on_cancel\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*task).set_check_cancellable(check_cancellable as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_set_return_on_cancel(
    mut task: *mut GTask,
    mut return_on_cancel: gboolean,
) -> gboolean {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*task).check_cancellable() as ::core::ffi::c_int != 0 || return_on_cancel == 0 {
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
            b"task->check_cancellable || !return_on_cancel\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*task).task_func.is_none() {
        (*task).set_return_on_cancel(return_on_cancel as guint as guint);
        return TRUE;
    }
    g_mutex_lock(&raw mut (*task).lock);
    if (*task).thread_cancelled != 0 {
        if return_on_cancel != 0 && (*task).return_on_cancel() == 0 {
            g_mutex_unlock(&raw mut (*task).lock);
            safe_c2rust_g_task_thread_complete(task);
        } else {
            g_mutex_unlock(&raw mut (*task).lock);
        }
        return FALSE;
    }
    (*task).set_return_on_cancel(return_on_cancel as guint as guint);
    g_mutex_unlock(&raw mut (*task).lock);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_set_source_tag(
    mut task: *mut GTask,
    mut source_tag: gpointer,
) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*task).source_tag = source_tag;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_set_name(
    mut task: *mut GTask,
    mut name: *const ::core::ffi::c_char,
) {
    let mut new_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    new_name = safe_c2rust_g_strdup_inline(name);
    if (*task).name_is_static() == 0 {
        g_free((*task).name as gpointer);
    }
    (*task).name = safe_c2rust_g_steal_pointer(&raw mut new_name as gpointer)
        as *mut ::core::ffi::c_char as *mut gchar;
    (*task).set_name_is_static(FALSE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_set_static_name(
    mut task: *mut GTask,
    mut name: *const ::core::ffi::c_char,
) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*task).name_is_static() == 0 {
        g_free((*task).name as gpointer);
    }
    (*task).name = name as *mut ::core::ffi::c_char as *mut gchar;
    (*task).set_name_is_static(TRUE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_source_object(mut task: *mut GTask) -> gpointer {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*task).source_object;
}
unsafe extern "C" fn safe_c2rust_g_task_ref_source_object(
    mut res: *mut GAsyncResult,
) -> *mut GObject {
    let mut task: *mut GTask = res as *mut ::core::ffi::c_void as *mut GTask;
    if !(*task).source_object.is_null() {
        return g_object_ref((*task).source_object) as *mut GObject;
    } else {
        return ::core::ptr::null_mut::<GObject>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_task_data(mut task: *mut GTask) -> gpointer {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*task).task_data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_priority(mut task: *mut GTask) -> gint {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*task).priority;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_context(mut task: *mut GTask) -> *mut GMainContext {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMainContext>();
    }
    return (*task).context;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_cancellable(
    mut task: *mut GTask,
) -> *mut GCancellable {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCancellable>();
    }
    return (*task).cancellable;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_check_cancellable(
    mut task: *mut GTask,
) -> gboolean {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return if (*task).check_cancellable() as ::core::ffi::c_int != 0 {
        TRUE
    } else {
        FALSE
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_return_on_cancel(mut task: *mut GTask) -> gboolean {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return if (*task).return_on_cancel() as ::core::ffi::c_int != 0 {
        TRUE
    } else {
        FALSE
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_source_tag(mut task: *mut GTask) -> gpointer {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*task).source_tag;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_name(mut task: *mut GTask) -> *const gchar {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*task).name;
}
unsafe extern "C" fn safe_c2rust_g_task_return_now(mut task: *mut GTask) {
    g_main_context_push_thread_default((*task).context);
    if (*task).callback.is_some() {
        (*task).callback.expect("non-null function pointer")(
            (*task).source_object as *mut GObject,
            task as *mut ::core::ffi::c_void as *mut GAsyncResult,
            (*task).callback_data,
        );
    }
    (*task).set_completed(TRUE as guint as guint);
    g_object_notify(
        task as *mut ::core::ffi::c_void as *mut GObject,
        b"completed\0" as *const u8 as *const gchar,
    );
    g_main_context_pop_thread_default((*task).context);
}
unsafe extern "C" fn safe_c2rust_complete_in_idle_cb(mut task: gpointer) -> gboolean {
    safe_c2rust_g_task_return_now(task as *mut GTask);
    g_object_unref(task);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_task_return(mut task: *mut GTask, mut type_0: GTaskReturnType) {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if type_0 as ::core::ffi::c_uint
        != G_TASK_RETURN_FROM_THREAD as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*task).set_ever_returned(TRUE as guint as guint);
    }
    if type_0 as ::core::ffi::c_uint
        == G_TASK_RETURN_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*task).set_result_set(TRUE as guint as guint);
    }
    if (*task).synchronous() != 0 {
        return;
    }
    if (*task).task_func.is_some()
        && type_0 as ::core::ffi::c_uint
            != G_TASK_RETURN_FROM_THREAD as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    g_object_ref(task as gpointer);
    source = g_main_current_source();
    if !source.is_null() && g_source_get_context(source) == (*task).context {
        if g_source_get_time(source) > (*task).creation_time {
            if g_cancellable_is_cancelled((*task).cancellable) == 0 {
                safe_c2rust_g_task_return_now(task);
                g_object_unref(task as gpointer);
                return;
            }
        }
    }
    source = g_idle_source_new();
    if (*task).name.is_null() {
        g_source_set_static_name(
            source,
            b"[gio] (unnamed) complete_in_idle_cb\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        let mut source_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
        source_name = g_strconcat(
            b"[gio] \0" as *const u8 as *const gchar,
            (*task).name,
            b" complete_in_idle_cb\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        );
        g_source_set_name(source, source_name);
        g_free(source_name as gpointer);
    }
    safe_c2rust_g_task_attach_source(
        task,
        source,
        Some(safe_c2rust_complete_in_idle_cb as unsafe extern "C" fn(gpointer) -> gboolean),
    );
    g_source_unref(source);
}
unsafe extern "C" fn safe_c2rust_g_task_thread_complete(mut task: *mut GTask) {
    g_mutex_lock(&raw mut (*task).lock);
    if (*task).thread_complete() != 0 {
        g_mutex_unlock(&raw mut (*task).lock);
        return;
    }
    (*task).set_thread_complete(TRUE as guint as guint);
    g_mutex_unlock(&raw mut (*task).lock);
    if !(*task).cancellable.is_null() {
        g_signal_handlers_disconnect_matched(
            (*task).cancellable as gpointer,
            (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
                as GSignalMatchType,
            0 as guint,
            0 as GQuark,
            ::core::ptr::null_mut::<GClosure>(),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> ()>,
                gpointer,
            >(Some(
                safe_c2rust_task_thread_cancelled
                    as unsafe extern "C" fn(*mut GCancellable, gpointer) -> (),
            )),
            task as gpointer,
        );
    }
    if (*task).synchronous() != 0 {
        g_cond_signal(&raw mut (*task).cond);
    } else {
        safe_c2rust_g_task_return(task, G_TASK_RETURN_FROM_THREAD);
    };
}
unsafe extern "C" fn safe_c2rust_task_pool_manager_timeout(mut user_data: gpointer) -> gboolean {
    g_mutex_lock(&raw mut safe_c2rust_task_pool_mutex);
    g_thread_pool_set_max_threads(
        safe_c2rust_task_pool,
        safe_c2rust_tasks_running + 1 as gint,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_source_set_ready_time(
        safe_c2rust_task_pool_manager,
        -(1 as ::core::ffi::c_int) as gint64,
    );
    g_mutex_unlock(&raw mut safe_c2rust_task_pool_mutex);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_task_thread_setup() {
    g_private_set(
        &raw mut safe_c2rust_task_private,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gulong as gpointer,
    );
    g_mutex_lock(&raw mut safe_c2rust_task_pool_mutex);
    safe_c2rust_tasks_running += 1;
    if safe_c2rust_tasks_running == G_TASK_POOL_SIZE {
        safe_c2rust_task_wait_time = G_TASK_WAIT_TIME_BASE as guint64;
    } else if safe_c2rust_tasks_running > G_TASK_POOL_SIZE
        && safe_c2rust_tasks_running < G_TASK_WAIT_TIME_MAX_POOL_SIZE
    {
        safe_c2rust_task_wait_time = (safe_c2rust_task_wait_time as ::core::ffi::c_double
            * G_TASK_WAIT_TIME_MULTIPLIER) as guint64;
    }
    if safe_c2rust_tasks_running >= G_TASK_POOL_SIZE {
        g_source_set_ready_time(
            safe_c2rust_task_pool_manager,
            (g_get_monotonic_time() as guint64).wrapping_add(safe_c2rust_task_wait_time) as gint64,
        );
    }
    g_mutex_unlock(&raw mut safe_c2rust_task_pool_mutex);
}
unsafe extern "C" fn safe_c2rust_g_task_thread_cleanup() {
    let mut tasks_pending: gint = 0;
    g_mutex_lock(&raw mut safe_c2rust_task_pool_mutex);
    tasks_pending = g_thread_pool_unprocessed(safe_c2rust_task_pool) as gint;
    if safe_c2rust_tasks_running > G_TASK_POOL_SIZE {
        g_thread_pool_set_max_threads(
            safe_c2rust_task_pool,
            safe_c2rust_tasks_running - 1 as gint,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    } else if safe_c2rust_tasks_running + tasks_pending < G_TASK_POOL_SIZE {
        g_source_set_ready_time(
            safe_c2rust_task_pool_manager,
            -(1 as ::core::ffi::c_int) as gint64,
        );
    }
    if safe_c2rust_tasks_running > G_TASK_POOL_SIZE
        && safe_c2rust_tasks_running < G_TASK_WAIT_TIME_MAX_POOL_SIZE
    {
        safe_c2rust_task_wait_time = (safe_c2rust_task_wait_time as ::core::ffi::c_double
            / G_TASK_WAIT_TIME_MULTIPLIER) as guint64;
    }
    safe_c2rust_tasks_running -= 1;
    g_mutex_unlock(&raw mut safe_c2rust_task_pool_mutex);
    g_private_set(
        &raw mut safe_c2rust_task_private,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
unsafe extern "C" fn safe_c2rust_g_task_thread_pool_thread(
    mut thread_data: gpointer,
    mut pool_data: gpointer,
) {
    let mut task: *mut GTask = thread_data as *mut GTask;
    safe_c2rust_g_task_thread_setup();
    (*task).task_func.expect("non-null function pointer")(
        task,
        (*task).source_object,
        (*task).task_data,
        (*task).cancellable,
    );
    safe_c2rust_g_task_thread_complete(task);
    g_object_unref(task as gpointer);
    safe_c2rust_g_task_thread_cleanup();
}
unsafe extern "C" fn safe_c2rust_task_thread_cancelled(
    mut cancellable: *mut GCancellable,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    g_thread_pool_move_to_front(safe_c2rust_task_pool, task as gpointer);
    g_mutex_lock(&raw mut (*task).lock);
    (*task).thread_cancelled = TRUE as gboolean;
    if (*task).return_on_cancel() == 0 {
        g_mutex_unlock(&raw mut (*task).lock);
        return;
    }
    g_mutex_unlock(&raw mut (*task).lock);
    safe_c2rust_g_task_thread_complete(task);
}
unsafe extern "C" fn safe_c2rust_task_thread_cancelled_disconnect_notify(
    mut task: gpointer,
    mut closure: *mut GClosure,
) {
    g_object_unref(task);
}
unsafe extern "C" fn safe_c2rust_g_task_start_task_thread(
    mut task: *mut GTask,
    mut task_func: GTaskThreadFunc,
) {
    g_mutex_init(&raw mut (*task).lock);
    g_cond_init(&raw mut (*task).cond);
    g_mutex_lock(&raw mut (*task).lock);
    (*task).task_func = task_func;
    if !(*task).cancellable.is_null() {
        if (*task).return_on_cancel() as ::core::ffi::c_int != 0
            && g_cancellable_set_error_if_cancelled((*task).cancellable, &raw mut (*task).error)
                != 0
        {
            (*task).set_thread_complete(TRUE as guint as guint);
            (*task).thread_cancelled = (*task).thread_complete() as gboolean;
            g_thread_pool_push(
                safe_c2rust_task_pool,
                g_object_ref(task as gpointer) as *mut GTask as gpointer,
                ::core::ptr::null_mut::<*mut GError>(),
            );
            return;
        }
        g_signal_connect_data(
            (*task).cancellable as gpointer,
            b"cancelled\0" as *const u8 as *const gchar,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> ()>,
                GCallback,
            >(Some(
                safe_c2rust_task_thread_cancelled
                    as unsafe extern "C" fn(*mut GCancellable, gpointer) -> (),
            )),
            g_object_ref(task as gpointer) as *mut GTask as gpointer,
            Some(
                safe_c2rust_task_thread_cancelled_disconnect_notify
                    as unsafe extern "C" fn(gpointer, *mut GClosure) -> (),
            ),
            G_CONNECT_DEFAULT,
        );
    }
    if !g_private_get(&raw mut safe_c2rust_task_private).is_null() {
        (*task).set_blocking_other_task(TRUE as guint as guint);
    }
    g_thread_pool_push(
        safe_c2rust_task_pool,
        g_object_ref(task as gpointer) as *mut GTask as gpointer,
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_run_in_thread(
    mut task: *mut GTask,
    mut task_func: GTaskThreadFunc,
) {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_ref(task as gpointer);
    safe_c2rust_g_task_start_task_thread(task, task_func);
    if (*task).thread_complete() != 0 {
        g_mutex_unlock(&raw mut (*task).lock);
        safe_c2rust_g_task_return(task, G_TASK_RETURN_FROM_THREAD);
    } else {
        g_mutex_unlock(&raw mut (*task).lock);
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_run_in_thread_sync(
    mut task: *mut GTask,
    mut task_func: GTaskThreadFunc,
) {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_ref(task as gpointer);
    (*task).set_synchronous(TRUE as guint as guint);
    safe_c2rust_g_task_start_task_thread(task, task_func);
    while (*task).thread_complete() == 0 {
        g_cond_wait(&raw mut (*task).cond, &raw mut (*task).lock);
    }
    g_mutex_unlock(&raw mut (*task).lock);
    (*task).set_completed(TRUE as guint as guint);
    g_object_notify(
        task as *mut ::core::ffi::c_void as *mut GObject,
        b"completed\0" as *const u8 as *const gchar,
    );
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_attach_source(
    mut task: *mut GTask,
    mut source: *mut GSource,
    mut callback: GSourceFunc,
) {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_source_set_callback(
        source,
        callback,
        g_object_ref(task as gpointer) as *mut GTask as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_source_set_priority(source, (*task).priority);
    if !(*task).name.is_null() && g_source_get_name(source).is_null() {
        g_source_set_name(source, (*task).name);
    }
    g_source_attach(source, (*task).context);
}
unsafe extern "C" fn safe_c2rust_g_task_propagate_error(
    mut task: *mut GTask,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut error_set: gboolean = 0;
    if (*task).check_cancellable() as ::core::ffi::c_int != 0
        && g_cancellable_set_error_if_cancelled((*task).cancellable, error) != 0
    {
        error_set = TRUE as gboolean;
    } else if !(*task).error.is_null() {
        g_propagate_error(error, (*task).error);
        (*task).error = ::core::ptr::null_mut::<GError>();
        (*task).set_had_error(TRUE as guint as guint);
        error_set = TRUE as gboolean;
    } else {
        error_set = FALSE as gboolean;
    }
    return error_set;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_return_pointer(
    mut task: *mut GTask,
    mut result: gpointer,
    mut result_destroy: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if (*task).ever_returned() == 0 {
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
            b"!task->ever_returned\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*task).result.pointer = result;
    (*task).result_destroy = result_destroy;
    safe_c2rust_g_task_return(task, G_TASK_RETURN_SUCCESS);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_propagate_pointer(
    mut task: *mut GTask,
    mut error: *mut *mut GError,
) -> gpointer {
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if safe_c2rust_g_task_propagate_error(task, error) != 0 {
        return NULL_0;
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if (*task).result_set() != 0 {
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
            b"task->result_set\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*task).result_destroy = None;
    (*task).set_result_set(FALSE as guint as guint);
    return (*task).result.pointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_return_int(mut task: *mut GTask, mut result: gssize) {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if (*task).ever_returned() == 0 {
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
            b"!task->ever_returned\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*task).result.size = result;
    safe_c2rust_g_task_return(task, G_TASK_RETURN_SUCCESS);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_propagate_int(
    mut task: *mut GTask,
    mut error: *mut *mut GError,
) -> gssize {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if safe_c2rust_g_task_propagate_error(task, error) != 0 {
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if (*task).result_set() != 0 {
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
            b"task->result_set\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    (*task).set_result_set(FALSE as guint as guint);
    return (*task).result.size;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_return_boolean(
    mut task: *mut GTask,
    mut result: gboolean,
) {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if (*task).ever_returned() == 0 {
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
            b"!task->ever_returned\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*task).result.boolean = result;
    safe_c2rust_g_task_return(task, G_TASK_RETURN_SUCCESS);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_propagate_boolean(
    mut task: *mut GTask,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_task_propagate_error(task, error) != 0 {
        return FALSE;
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if (*task).result_set() != 0 {
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
            b"task->result_set\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    (*task).set_result_set(FALSE as guint as guint);
    return (*task).result.boolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_return_error(
    mut task: *mut GTask,
    mut error: *mut GError,
) {
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if (*task).ever_returned() == 0 {
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
            b"!task->ever_returned\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*task).error = error;
    safe_c2rust_g_task_return(task, G_TASK_RETURN_ERROR);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_return_prefixed_error(
    mut task: *mut GTask,
    mut error: *mut GError,
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut prefix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if (*task).ever_returned() == 0 {
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
            b"!task->ever_returned\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*task).error = error;
    ap = args.clone();
    prefix = g_strdup_vprintf(format as *const gchar, ap) as *mut ::core::ffi::c_char;
    g_prefix_error_literal(&raw mut (*task).error, prefix);
    g_free(prefix as gpointer);
    safe_c2rust_g_task_return(task, G_TASK_RETURN_ERROR);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_return_new_error(
    mut task: *mut GTask,
    mut domain: GQuark,
    mut code: gint,
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    error = g_error_new_valist(domain, code, format as *const gchar, args_0);
    safe_c2rust_g_task_return_error(task, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_return_new_error_literal(
    mut task: *mut GTask,
    mut domain: GQuark,
    mut code: gint,
    mut message: *const ::core::ffi::c_char,
) {
    safe_c2rust_g_task_return_error(
        task,
        g_error_new_literal(domain, code, message as *const gchar),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_return_error_if_cancelled(
    mut task: *mut GTask,
) -> gboolean {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if (*task).ever_returned() == 0 {
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
            b"!task->ever_returned\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled((*task).cancellable, &raw mut error) != 0 {
        g_clear_error(&raw mut (*task).error);
        (*task).error = error;
        safe_c2rust_g_task_return(task, G_TASK_RETURN_ERROR);
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_had_error(mut task: *mut GTask) -> gboolean {
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !(*task).error.is_null() || (*task).had_error() as ::core::ffi::c_int != 0 {
        return TRUE;
    }
    if (*task).check_cancellable() as ::core::ffi::c_int != 0
        && g_cancellable_is_cancelled((*task).cancellable) != 0
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_value_free(mut value: gpointer) {
    g_value_unset(value as *mut GValue);
    g_free(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_return_value(
    mut task: *mut GTask,
    mut result: *mut GValue,
) {
    let mut value: *mut GValue = ::core::ptr::null_mut::<GValue>();
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if (*task).ever_returned() == 0 {
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
            b"!task->ever_returned\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GValue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GValue;
    if result.is_null() {
        g_value_init(value, G_TYPE_POINTER);
        g_value_set_pointer(value, NULL_0);
    } else {
        g_value_init(value, (*result).g_type);
        g_value_copy(result, value);
    }
    safe_c2rust_g_task_return_pointer(
        task,
        value as gpointer,
        Some(safe_c2rust_value_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_propagate_value(
    mut task: *mut GTask,
    mut value: *mut GValue,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_task_propagate_error(task, error) != 0 {
        return FALSE;
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if (*task).result_set() != 0 {
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
            b"task->result_set\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if (*task).result_destroy
            == Some(safe_c2rust_value_free as unsafe extern "C" fn(gpointer) -> ())
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
            b"task->result_destroy == value_free\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    memcpy(
        value as *mut ::core::ffi::c_void,
        (*task).result.pointer as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GValue>() as size_t,
    );
    g_free((*task).result.pointer);
    (*task).result_destroy = None;
    (*task).set_result_set(FALSE as guint as guint);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_get_completed(mut task: *mut GTask) -> gboolean {
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = task as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_task_get_type();
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
            b"G_IS_TASK (task)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return if (*task).completed() as ::core::ffi::c_int != 0 {
        TRUE
    } else {
        FALSE
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_task_is_valid(
    mut result: gpointer,
    mut source_object: gpointer,
) -> gboolean {
    if ({
        let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
        let mut __t: GType = safe_c2rust_g_task_get_type();
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
        return FALSE;
    }
    return ((*(result as *mut GTask)).source_object == source_object) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_task_compare_priority(
    mut a: gconstpointer,
    mut b: gconstpointer,
    mut user_data: gpointer,
) -> gint {
    let mut ta: *const GTask = a as *const GTask;
    let mut tb: *const GTask = b as *const GTask;
    let mut a_cancelled: gboolean = 0;
    let mut b_cancelled: gboolean = 0;
    if (*ta).blocking_other_task() as ::core::ffi::c_int != 0 && (*tb).blocking_other_task() == 0 {
        return -(1 as gint);
    } else if (*tb).blocking_other_task() as ::core::ffi::c_int != 0
        && (*ta).blocking_other_task() == 0
    {
        return 1 as gint;
    }
    a_cancelled = ((*ta).check_cancellable() as ::core::ffi::c_int != 0
        && g_cancellable_is_cancelled((*ta).cancellable) != 0)
        as ::core::ffi::c_int as gboolean;
    b_cancelled = ((*tb).check_cancellable() as ::core::ffi::c_int != 0
        && g_cancellable_is_cancelled((*tb).cancellable) != 0)
        as ::core::ffi::c_int as gboolean;
    if a_cancelled != 0 && b_cancelled == 0 {
        return -(1 as gint);
    } else if b_cancelled != 0 && a_cancelled == 0 {
        return 1 as gint;
    }
    return (*ta).priority - (*tb).priority;
}
unsafe extern "C" fn safe_c2rust_trivial_source_dispatch(
    mut source: *mut GSource,
    mut callback: GSourceFunc,
    mut user_data: gpointer,
) -> gboolean {
    return callback.expect("non-null function pointer")(user_data);
}
#[no_mangle]
pub static mut safe_c2rust_trivial_source_funcs: GSourceFuncs = unsafe {
    _GSourceFuncs {
        prepare: None,
        check: None,
        dispatch: Some(
            safe_c2rust_trivial_source_dispatch
                as unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
        ),
        finalize: None,
        closure_callback: None,
        closure_marshal: None,
    }
};
unsafe extern "C" fn safe_c2rust_g_task_thread_pool_init() {
    safe_c2rust_task_pool = g_thread_pool_new(
        Some(
            safe_c2rust_g_task_thread_pool_thread as unsafe extern "C" fn(gpointer, gpointer) -> (),
        ),
        NULL_0,
        G_TASK_POOL_SIZE,
        FALSE,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if !safe_c2rust_task_pool.is_null() {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gtask.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2413 as ::core::ffi::c_int,
            G_STRFUNC,
            b"task_pool != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_thread_pool_set_sort_function(
        safe_c2rust_task_pool,
        Some(
            safe_c2rust_g_task_compare_priority
                as unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
        ),
        NULL_0,
    );
    safe_c2rust_task_pool_manager = g_source_new(
        &raw mut safe_c2rust_trivial_source_funcs,
        ::core::mem::size_of::<GSource>() as guint,
    );
    g_source_set_static_name(
        safe_c2rust_task_pool_manager,
        b"GTask thread pool manager\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_set_callback(
        safe_c2rust_task_pool_manager,
        Some(safe_c2rust_task_pool_manager_timeout as unsafe extern "C" fn(gpointer) -> gboolean),
        NULL_0,
        None,
    );
    g_source_set_ready_time(
        safe_c2rust_task_pool_manager,
        -(1 as ::core::ffi::c_int) as gint64,
    );
    g_source_attach(
        safe_c2rust_task_pool_manager,
        (*glib__private__())
            .g_get_worker_context
            .expect("non-null function pointer")(),
    );
    g_source_unref(safe_c2rust_task_pool_manager);
}
unsafe extern "C" fn safe_c2rust_g_task_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut task: *mut GTask = object as *mut ::core::ffi::c_void as *mut GTask;
    match prop_id as GTaskProperty as ::core::ffi::c_uint {
        1 => {
            g_value_set_boolean(value, safe_c2rust_g_task_get_completed(task));
        }
        _ => {}
    };
}
unsafe extern "C" fn safe_c2rust_g_task_class_init(mut klass: *mut GTaskClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_task_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_task_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_COMPLETED as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"completed\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if safe_c2rust_task_pool_max_counter == 0 as guint {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_task_pool_max_counter = -(1 as ::core::ffi::c_int) as guint;
        safe_c2rust_tasks_running_counter = -(1 as ::core::ffi::c_int) as guint;
    }
}
unsafe extern "C" fn safe_c2rust_g_task_get_user_data(mut res: *mut GAsyncResult) -> gpointer {
    return (*(res as *mut ::core::ffi::c_void as *mut GTask)).callback_data;
}
unsafe extern "C" fn safe_c2rust_g_task_is_tagged(
    mut res: *mut GAsyncResult,
    mut source_tag: gpointer,
) -> gboolean {
    return ((*(res as *mut ::core::ffi::c_void as *mut GTask)).source_tag == source_tag)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_task_async_result_iface_init(mut iface: *mut GAsyncResultIface) {
    (*iface).get_user_data = Some(
        safe_c2rust_g_task_get_user_data as unsafe extern "C" fn(*mut GAsyncResult) -> gpointer,
    ) as Option<unsafe extern "C" fn(*mut GAsyncResult) -> gpointer>;
    (*iface).get_source_object = Some(
        safe_c2rust_g_task_ref_source_object
            as unsafe extern "C" fn(*mut GAsyncResult) -> *mut GObject,
    )
        as Option<unsafe extern "C" fn(*mut GAsyncResult) -> *mut GObject>;
    (*iface).is_tagged = Some(
        safe_c2rust_g_task_is_tagged
            as unsafe extern "C" fn(*mut GAsyncResult, gpointer) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GAsyncResult, gpointer) -> gboolean>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
