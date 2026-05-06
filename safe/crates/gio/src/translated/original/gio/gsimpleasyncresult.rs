extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GIOSchedulerJob;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new_valist(
        domain: GQuark,
        code: gint,
        format: *const gchar,
        args: ::core::ffi::VaList,
    ) -> *mut GError;
    fn g_error_free(error: *mut GError);
    fn g_error_copy(error: *const GError) -> *mut GError;
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_current_source() -> *mut GSource;
    fn g_main_context_push_thread_default(context: *mut GMainContext);
    fn g_main_context_pop_thread_default(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
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
    fn g_source_is_destroyed(source: *mut GSource) -> gboolean;
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_async_result_get_type() -> GType;
    fn g_async_result_get_source_object(res: *mut GAsyncResult) -> *mut GObject;
    fn g_cancellable_get_type() -> GType;
    fn g_cancellable_is_cancelled(cancellable: *mut GCancellable) -> gboolean;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_io_scheduler_push_job(
        job_func: GIOSchedulerJobFunc,
        user_data: gpointer,
        notify: GDestroyNotify,
        io_priority: gint,
        cancellable: *mut GCancellable,
    );
    fn g_io_error_quark() -> GQuark;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GIOSchedulerJob = _GIOSchedulerJob;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSimpleAsyncResult {
    pub parent_instance: GObject,
    pub source_object: *mut GObject,
    pub callback: GAsyncReadyCallback,
    pub user_data: gpointer,
    pub context: *mut GMainContext,
    pub error: *mut GError,
    pub failed: gboolean,
    pub handle_cancellation: gboolean,
    pub check_cancellable: *mut GCancellable,
    pub source_tag: gpointer,
    pub op_res: C2RustUnnamed_1,
    pub destroy_op_res: GDestroyNotify,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub v_pointer: gpointer,
    pub v_boolean: gboolean,
    pub v_ssize: gssize,
}
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GSimpleAsyncResult = _GSimpleAsyncResult;
pub type GIOSchedulerJobFunc =
    Option<unsafe extern "C" fn(*mut GIOSchedulerJob, *mut GCancellable, gpointer) -> gboolean>;
pub type GSimpleAsyncThreadFunc =
    Option<unsafe extern "C" fn(*mut GSimpleAsyncResult, *mut GObject, *mut GCancellable) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSimpleAsyncResultClass {
    pub parent_class: GObjectClass,
}
pub type GSimpleAsyncResultClass = _GSimpleAsyncResultClass;
pub type GAsyncResultIface = _GAsyncResultIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAsyncResultIface {
    pub g_iface: GTypeInterface,
    pub get_user_data: Option<unsafe extern "C" fn(*mut GAsyncResult) -> gpointer>,
    pub get_source_object: Option<unsafe extern "C" fn(*mut GAsyncResult) -> *mut GObject>,
    pub is_tagged: Option<unsafe extern "C" fn(*mut GAsyncResult, gpointer) -> gboolean>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RunInThreadData {
    pub simple: *mut GSimpleAsyncResult,
    pub cancellable: *mut GCancellable,
    pub func: GSimpleAsyncThreadFunc,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_simple_async_result_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSimpleAsyncResult\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSimpleAsyncResultClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_simple_async_result_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSimpleAsyncResult>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSimpleAsyncResult) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_simple_async_result_init
                    as unsafe extern "C" fn(*mut GSimpleAsyncResult) -> (),
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
            safe_c2rust_g_simple_async_result_async_result_iface_init
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
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_simple_async_result_get_type_once();
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
static mut safe_c2rust_GSimpleAsyncResult_private_offset: gint = 0;
static mut safe_c2rust_g_simple_async_result_parent_class: gpointer = NULL;
unsafe extern "C" fn safe_c2rust_g_simple_async_result_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_simple_async_result_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSimpleAsyncResult_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSimpleAsyncResult_private_offset,
        );
    }
    safe_c2rust_g_simple_async_result_class_init(klass as *mut GSimpleAsyncResultClass);
}
unsafe extern "C" fn safe_c2rust_clear_op_res(mut simple: *mut GSimpleAsyncResult) {
    if (*simple).destroy_op_res.is_some() {
        (*simple).destroy_op_res.expect("non-null function pointer")((*simple).op_res.v_pointer);
    }
    (*simple).destroy_op_res = None;
    (*simple).op_res.v_ssize = 0 as gssize;
}
unsafe extern "C" fn safe_c2rust_g_simple_async_result_finalize(mut object: *mut GObject) {
    let mut simple: *mut GSimpleAsyncResult = ::core::ptr::null_mut::<GSimpleAsyncResult>();
    simple = object as *mut ::core::ffi::c_void as *mut GSimpleAsyncResult;
    if !(*simple).source_object.is_null() {
        g_object_unref((*simple).source_object as gpointer);
    }
    if !(*simple).check_cancellable.is_null() {
        g_object_unref((*simple).check_cancellable as gpointer);
    }
    g_main_context_unref((*simple).context);
    safe_c2rust_clear_op_res(simple);
    if !(*simple).error.is_null() {
        g_error_free((*simple).error);
    }
    (*(safe_c2rust_g_simple_async_result_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_simple_async_result_class_init(
    mut klass: *mut GSimpleAsyncResultClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_simple_async_result_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_simple_async_result_init(mut simple: *mut GSimpleAsyncResult) {
    (*simple).handle_cancellation = TRUE as gboolean;
    (*simple).context = g_main_context_ref_thread_default();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_new(
    mut source_object: *mut GObject,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
    mut source_tag: gpointer,
) -> *mut GSimpleAsyncResult {
    let mut simple: *mut GSimpleAsyncResult = ::core::ptr::null_mut::<GSimpleAsyncResult>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if source_object.is_null()
            || g_type_check_instance_is_fundamentally_a(
                source_object as *mut GTypeInstance,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
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
            b"!source_object || G_IS_OBJECT (source_object)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSimpleAsyncResult>();
    }
    simple = g_object_new(
        safe_c2rust_g_simple_async_result_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GSimpleAsyncResult;
    (*simple).callback = callback;
    if !source_object.is_null() {
        (*simple).source_object =
            g_object_ref(source_object as gpointer) as *mut GObject as *mut GObject;
    } else {
        (*simple).source_object = ::core::ptr::null_mut::<GObject>();
    }
    (*simple).user_data = user_data;
    (*simple).source_tag = source_tag;
    return simple;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_new_from_error(
    mut source_object: *mut GObject,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
    mut error: *const GError,
) -> *mut GSimpleAsyncResult {
    let mut simple: *mut GSimpleAsyncResult = ::core::ptr::null_mut::<GSimpleAsyncResult>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if source_object.is_null()
            || g_type_check_instance_is_fundamentally_a(
                source_object as *mut GTypeInstance,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
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
            b"!source_object || G_IS_OBJECT (source_object)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSimpleAsyncResult>();
    }
    simple = safe_c2rust_g_simple_async_result_new(source_object, callback, user_data, NULL);
    safe_c2rust_g_simple_async_result_set_from_error(simple, error);
    return simple;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_new_take_error(
    mut source_object: *mut GObject,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
    mut error: *mut GError,
) -> *mut GSimpleAsyncResult {
    let mut simple: *mut GSimpleAsyncResult = ::core::ptr::null_mut::<GSimpleAsyncResult>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if source_object.is_null()
            || g_type_check_instance_is_fundamentally_a(
                source_object as *mut GTypeInstance,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
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
            b"!source_object || G_IS_OBJECT (source_object)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSimpleAsyncResult>();
    }
    simple = safe_c2rust_g_simple_async_result_new(source_object, callback, user_data, NULL);
    safe_c2rust_g_simple_async_result_take_error(simple, error);
    return simple;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_new_error(
    mut source_object: *mut GObject,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
    mut domain: GQuark,
    mut code: gint,
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) -> *mut GSimpleAsyncResult {
    let mut simple: *mut GSimpleAsyncResult = ::core::ptr::null_mut::<GSimpleAsyncResult>();
    let mut args_0: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if source_object.is_null()
            || g_type_check_instance_is_fundamentally_a(
                source_object as *mut GTypeInstance,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
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
            b"!source_object || G_IS_OBJECT (source_object)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSimpleAsyncResult>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if domain != 0 as GQuark {
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
            b"domain != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSimpleAsyncResult>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSimpleAsyncResult>();
    }
    simple = safe_c2rust_g_simple_async_result_new(source_object, callback, user_data, NULL);
    args_0 = args.clone();
    safe_c2rust_g_simple_async_result_set_error_va(
        simple,
        domain,
        code,
        format,
        args_0,
    );
    return simple;
}
unsafe extern "C" fn safe_c2rust_g_simple_async_result_get_user_data(
    mut res: *mut GAsyncResult,
) -> gpointer {
    return (*(res as *mut ::core::ffi::c_void as *mut GSimpleAsyncResult)).user_data;
}
unsafe extern "C" fn safe_c2rust_g_simple_async_result_get_source_object(
    mut res: *mut GAsyncResult,
) -> *mut GObject {
    if !(*(res as *mut ::core::ffi::c_void as *mut GSimpleAsyncResult))
        .source_object
        .is_null()
    {
        return g_object_ref(
            (*(res as *mut ::core::ffi::c_void as *mut GSimpleAsyncResult)).source_object
                as gpointer,
        ) as *mut GObject;
    }
    return ::core::ptr::null_mut::<GObject>();
}
unsafe extern "C" fn safe_c2rust_g_simple_async_result_is_tagged(
    mut res: *mut GAsyncResult,
    mut source_tag: gpointer,
) -> gboolean {
    return ((*(res as *mut ::core::ffi::c_void as *mut GSimpleAsyncResult)).source_tag
        == source_tag) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_simple_async_result_async_result_iface_init(
    mut iface: *mut GAsyncResultIface,
) {
    (*iface).get_user_data = Some(
        safe_c2rust_g_simple_async_result_get_user_data
            as unsafe extern "C" fn(*mut GAsyncResult) -> gpointer,
    ) as Option<unsafe extern "C" fn(*mut GAsyncResult) -> gpointer>;
    (*iface).get_source_object = Some(
        safe_c2rust_g_simple_async_result_get_source_object
            as unsafe extern "C" fn(*mut GAsyncResult) -> *mut GObject,
    )
        as Option<unsafe extern "C" fn(*mut GAsyncResult) -> *mut GObject>;
    (*iface).is_tagged = Some(
        safe_c2rust_g_simple_async_result_is_tagged
            as unsafe extern "C" fn(*mut GAsyncResult, gpointer) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GAsyncResult, gpointer) -> gboolean>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_set_handle_cancellation(
    mut simple: *mut GSimpleAsyncResult,
    mut handle_cancellation: gboolean,
) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*simple).handle_cancellation = handle_cancellation;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_get_source_tag(
    mut simple: *mut GSimpleAsyncResult,
) -> gpointer {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*simple).source_tag;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_propagate_error(
    mut simple: *mut GSimpleAsyncResult,
    mut dest: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_cancellable_set_error_if_cancelled((*simple).check_cancellable, dest) != 0 {
        return TRUE;
    }
    if (*simple).failed != 0 {
        g_propagate_error(dest, (*simple).error);
        (*simple).error = ::core::ptr::null_mut::<GError>();
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_set_op_res_gpointer(
    mut simple: *mut GSimpleAsyncResult,
    mut op_res: gpointer,
    mut destroy_op_res: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_clear_op_res(simple);
    (*simple).op_res.v_pointer = op_res;
    (*simple).destroy_op_res = destroy_op_res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_get_op_res_gpointer(
    mut simple: *mut GSimpleAsyncResult,
) -> gpointer {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*simple).op_res.v_pointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_set_op_res_gssize(
    mut simple: *mut GSimpleAsyncResult,
    mut op_res: gssize,
) {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_clear_op_res(simple);
    (*simple).op_res.v_ssize = op_res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_get_op_res_gssize(
    mut simple: *mut GSimpleAsyncResult,
) -> gssize {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gssize;
    }
    return (*simple).op_res.v_ssize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_set_op_res_gboolean(
    mut simple: *mut GSimpleAsyncResult,
    mut op_res: gboolean,
) {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_clear_op_res(simple);
    (*simple).op_res.v_boolean = (op_res != 0) as ::core::ffi::c_int as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_get_op_res_gboolean(
    mut simple: *mut GSimpleAsyncResult,
) -> gboolean {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*simple).op_res.v_boolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_set_from_error(
    mut simple: *mut GSimpleAsyncResult,
    mut error: *const GError,
) {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*simple).error.is_null() {
        g_error_free((*simple).error);
    }
    (*simple).error = g_error_copy(error);
    (*simple).failed = TRUE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_take_error(
    mut simple: *mut GSimpleAsyncResult,
    mut error: *mut GError,
) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*simple).error.is_null() {
        g_error_free((*simple).error);
    }
    (*simple).error = error;
    (*simple).failed = TRUE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_set_error_va(
    mut simple: *mut GSimpleAsyncResult,
    mut domain: GQuark,
    mut code: gint,
    mut format: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if domain != 0 as GQuark {
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
            b"domain != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*simple).error.is_null() {
        g_error_free((*simple).error);
    }
    (*simple).error = g_error_new_valist(domain, code, format as *const gchar, args);
    (*simple).failed = TRUE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_set_error(
    mut simple: *mut GSimpleAsyncResult,
    mut domain: GQuark,
    mut code: gint,
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if domain != 0 as GQuark {
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
            b"domain != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    args_0 = args.clone();
    safe_c2rust_g_simple_async_result_set_error_va(
        simple,
        domain,
        code,
        format,
        args_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_complete(
    mut simple: *mut GSimpleAsyncResult,
) {
    let mut current_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut current_context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    current_source = g_main_current_source();
    if !current_source.is_null() && g_source_is_destroyed(current_source) == 0 {
        current_context = g_source_get_context(current_source);
        if (*simple).context != current_context {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"g_simple_async_result_complete() called from wrong context!\0" as *const u8
                    as *const gchar,
            );
        }
    }
    if (*simple).callback.is_some() {
        g_main_context_push_thread_default((*simple).context);
        (*simple).callback.expect("non-null function pointer")(
            (*simple).source_object,
            simple as *mut ::core::ffi::c_void as *mut GAsyncResult,
            (*simple).user_data,
        );
        g_main_context_pop_thread_default((*simple).context);
    }
}
unsafe extern "C" fn safe_c2rust_complete_in_idle_cb(mut data: gpointer) -> gboolean {
    let mut simple: *mut GSimpleAsyncResult = data as *mut GSimpleAsyncResult;
    safe_c2rust_g_simple_async_result_complete(simple);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_complete_in_idle(
    mut simple: *mut GSimpleAsyncResult,
) {
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_object_ref(simple as gpointer);
    source = g_idle_source_new();
    g_source_set_priority(source, G_PRIORITY_DEFAULT);
    g_source_set_callback(
        source,
        Some(safe_c2rust_complete_in_idle_cb as unsafe extern "C" fn(gpointer) -> gboolean),
        simple as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_source_set_static_name(
        source,
        b"[gio] complete_in_idle_cb\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(source, (*simple).context);
    g_source_unref(source);
}
unsafe extern "C" fn safe_c2rust_complete_in_idle_cb_for_thread(mut _data: gpointer) -> gboolean {
    let mut data: *mut RunInThreadData = _data as *mut RunInThreadData;
    let mut simple: *mut GSimpleAsyncResult = ::core::ptr::null_mut::<GSimpleAsyncResult>();
    simple = (*data).simple;
    if (*simple).handle_cancellation != 0 && g_cancellable_is_cancelled((*data).cancellable) != 0 {
        safe_c2rust_g_simple_async_result_set_error(
            simple,
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            glib_gettext(b"Operation was cancelled\0" as *const u8 as *const gchar),
        );
    }
    safe_c2rust_g_simple_async_result_complete(simple);
    if !(*data).cancellable.is_null() {
        g_object_unref((*data).cancellable as gpointer);
    }
    g_object_unref((*data).simple as gpointer);
    g_free(data as gpointer);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_run_in_thread(
    mut job: *mut GIOSchedulerJob,
    mut c: *mut GCancellable,
    mut _data: gpointer,
) -> gboolean {
    let mut data: *mut RunInThreadData = _data as *mut RunInThreadData;
    let mut simple: *mut GSimpleAsyncResult = (*data).simple;
    let mut source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    if (*simple).handle_cancellation != 0 && g_cancellable_is_cancelled(c) != 0 {
        safe_c2rust_g_simple_async_result_set_error(
            simple,
            g_io_error_quark(),
            G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
            b"%s\0" as *const u8 as *const ::core::ffi::c_char,
            glib_gettext(b"Operation was cancelled\0" as *const u8 as *const gchar),
        );
    } else {
        (*data).func.expect("non-null function pointer")(simple, (*simple).source_object, c);
    }
    source = g_idle_source_new();
    g_source_set_priority(source, G_PRIORITY_DEFAULT);
    g_source_set_callback(
        source,
        Some(
            safe_c2rust_complete_in_idle_cb_for_thread
                as unsafe extern "C" fn(gpointer) -> gboolean,
        ),
        data as gpointer,
        None,
    );
    g_source_set_static_name(
        source,
        b"[gio] complete_in_idle_cb_for_thread\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(source, (*simple).context);
    g_source_unref(source);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_run_in_thread(
    mut simple: *mut GSimpleAsyncResult,
    mut func: GSimpleAsyncThreadFunc,
    mut io_priority: ::core::ffi::c_int,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut RunInThreadData = ::core::ptr::null_mut::<RunInThreadData>();
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<RunInThreadData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut RunInThreadData;
    (*data).func = func;
    (*data).simple =
        g_object_ref(simple as gpointer) as *mut GSimpleAsyncResult as *mut GSimpleAsyncResult;
    (*data).cancellable = cancellable;
    if !cancellable.is_null() {
        g_object_ref(cancellable as gpointer);
    }
    g_io_scheduler_push_job(
        Some(
            safe_c2rust_run_in_thread
                as unsafe extern "C" fn(
                    *mut GIOSchedulerJob,
                    *mut GCancellable,
                    gpointer,
                ) -> gboolean,
        ),
        data as gpointer,
        None,
        io_priority as gint,
        cancellable,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_is_valid(
    mut result: *mut GAsyncResult,
    mut source: *mut GObject,
    mut source_tag: gpointer,
) -> gboolean {
    let mut simple: *mut GSimpleAsyncResult = ::core::ptr::null_mut::<GSimpleAsyncResult>();
    let mut cmp_source: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut result_source_tag: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut __inst: *mut GTypeInstance = result as *mut GTypeInstance;
        let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
    simple = result as *mut GSimpleAsyncResult;
    cmp_source = g_async_result_get_source_object(result);
    if cmp_source != source {
        if !cmp_source.is_null() {
            g_object_unref(cmp_source as gpointer);
        }
        return FALSE;
    }
    if !cmp_source.is_null() {
        g_object_unref(cmp_source as gpointer);
    }
    result_source_tag = safe_c2rust_g_simple_async_result_get_source_tag(simple);
    return (source_tag.is_null() || result_source_tag.is_null() || source_tag == result_source_tag)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_report_error_in_idle(
    mut object: *mut GObject,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
    mut domain: GQuark,
    mut code: gint,
    mut format: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut simple: *mut GSimpleAsyncResult = ::core::ptr::null_mut::<GSimpleAsyncResult>();
    let mut args_0: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if object.is_null()
            || g_type_check_instance_is_fundamentally_a(
                object as *mut GTypeInstance,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
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
            b"!object || G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if domain != 0 as GQuark {
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
            b"domain != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !format.is_null() {
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
            b"format != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    simple = safe_c2rust_g_simple_async_result_new(object, callback, user_data, NULL);
    args_0 = args.clone();
    safe_c2rust_g_simple_async_result_set_error_va(
        simple,
        domain,
        code,
        format,
        args_0,
    );
    safe_c2rust_g_simple_async_result_complete_in_idle(simple);
    g_object_unref(simple as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_report_gerror_in_idle(
    mut object: *mut GObject,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
    mut error: *const GError,
) {
    let mut simple: *mut GSimpleAsyncResult = ::core::ptr::null_mut::<GSimpleAsyncResult>();
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if object.is_null()
            || g_type_check_instance_is_fundamentally_a(
                object as *mut GTypeInstance,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
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
            b"!object || G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !error.is_null() {
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
            b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    simple = safe_c2rust_g_simple_async_result_new_from_error(object, callback, user_data, error);
    safe_c2rust_g_simple_async_result_complete_in_idle(simple);
    g_object_unref(simple as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_report_take_gerror_in_idle(
    mut object: *mut GObject,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
    mut error: *mut GError,
) {
    let mut simple: *mut GSimpleAsyncResult = ::core::ptr::null_mut::<GSimpleAsyncResult>();
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if object.is_null()
            || g_type_check_instance_is_fundamentally_a(
                object as *mut GTypeInstance,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            ) != 0
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
            b"!object || G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
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
    simple = safe_c2rust_g_simple_async_result_new_take_error(object, callback, user_data, error);
    safe_c2rust_g_simple_async_result_complete_in_idle(simple);
    g_object_unref(simple as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_async_result_set_check_cancellable(
    mut simple: *mut GSimpleAsyncResult,
    mut check_cancellable: *mut GCancellable,
) {
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_async_result_get_type();
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
            b"G_IS_SIMPLE_ASYNC_RESULT (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if check_cancellable.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = check_cancellable as *mut GTypeInstance;
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
            b"check_cancellable == NULL || G_IS_CANCELLABLE (check_cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut _pp: *mut *mut GCancellable = &raw mut (*simple).check_cancellable;
    let mut _ptr: *mut GCancellable = *_pp;
    *_pp = ::core::ptr::null_mut::<GCancellable>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    if !check_cancellable.is_null() {
        (*simple).check_cancellable =
            g_object_ref(check_cancellable as gpointer) as *mut GCancellable as *mut GCancellable;
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
