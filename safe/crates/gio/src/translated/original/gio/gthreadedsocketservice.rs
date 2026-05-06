use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GIOStreamPrivate;
    pub type _GSocketPrivate;
    pub type _GSocketConnectionPrivate;
    pub type _GSocketListenerPrivate;
    pub type _GSocketServicePrivate;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_clear_error(err: *mut *mut GError);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_thread_pool_new(
        func: GFunc,
        user_data: gpointer,
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
    fn g_signal_accumulator_true_handled(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
    ) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_param_spec_int(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: gint,
        maximum: gint,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_get_int(value: *const GValue) -> gint;
    fn g_socket_connection_get_type() -> GType;
    fn g_socket_service_get_type() -> GType;
    fn g_socket_service_start(service: *mut GSocketService);
    fn g_socket_service_stop(service: *mut GSocketService);
    fn _g_cclosure_marshal_BOOLEAN__OBJECT_OBJECT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_BOOLEAN__OBJECT_OBJECTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
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
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
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
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub type GSocketListenerEvent = ::core::ffi::c_uint;
pub const G_SOCKET_LISTENER_LISTENED: GSocketListenerEvent = 3;
pub const G_SOCKET_LISTENER_LISTENING: GSocketListenerEvent = 2;
pub const G_SOCKET_LISTENER_BOUND: GSocketListenerEvent = 1;
pub const G_SOCKET_LISTENER_BINDING: GSocketListenerEvent = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocket {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketPrivate,
}
pub type GSocketPrivate = _GSocketPrivate;
pub type GSocket = _GSocket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketConnection {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GSocketConnectionPrivate,
}
pub type GSocketConnectionPrivate = _GSocketConnectionPrivate;
pub type GSocketConnection = _GSocketConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketListener {
    pub parent_instance: GObject,
    pub priv_0: *mut GSocketListenerPrivate,
}
pub type GSocketListenerPrivate = _GSocketListenerPrivate;
pub type GSocketListener = _GSocketListener;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketService {
    pub parent_instance: GSocketListener,
    pub priv_0: *mut GSocketServicePrivate,
}
pub type GSocketServicePrivate = _GSocketServicePrivate;
pub type GSocketService = _GSocketService;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThreadedSocketService {
    pub parent_instance: GSocketService,
    pub priv_0: *mut GThreadedSocketServicePrivate,
}
pub type GThreadedSocketServicePrivate = _GThreadedSocketServicePrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThreadedSocketServicePrivate {
    pub thread_pool: *mut GThreadPool,
    pub max_threads: ::core::ffi::c_int,
    pub job_count: gint,
}
pub type GThreadedSocketService = _GThreadedSocketService;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketListenerClass {
    pub parent_class: GObjectClass,
    pub changed: Option<unsafe extern "C" fn(*mut GSocketListener) -> ()>,
    pub event: Option<
        unsafe extern "C" fn(*mut GSocketListener, GSocketListenerEvent, *mut GSocket) -> (),
    >,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
}
pub type GSocketListenerClass = _GSocketListenerClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSocketServiceClass {
    pub parent_class: GSocketListenerClass,
    pub incoming: Option<
        unsafe extern "C" fn(*mut GSocketService, *mut GSocketConnection, *mut GObject) -> gboolean,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
}
pub type GSocketServiceClass = _GSocketServiceClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThreadedSocketServiceClass {
    pub parent_class: GSocketServiceClass,
    pub run: Option<
        unsafe extern "C" fn(
            *mut GThreadedSocketService,
            *mut GSocketConnection,
            *mut GObject,
        ) -> gboolean,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
}
pub type GThreadedSocketServiceClass = _GThreadedSocketServiceClass;
pub const PROP_MAX_THREADS: GThreadedSocketServiceProperty = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GThreadedSocketServiceData {
    pub service: *mut GThreadedSocketService,
    pub connection: *mut GSocketConnection,
    pub source_object: *mut GObject,
}
pub type GThreadedSocketServiceProperty = ::core::ffi::c_uint;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_OBJECT: GType = ((20 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_g_threaded_socket_service_run_signal: guint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_socket_service_get_type(),
        g_intern_static_string(b"GThreadedSocketService\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GThreadedSocketServiceClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_threaded_socket_service_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GThreadedSocketService>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GThreadedSocketService) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_threaded_socket_service_init
                    as unsafe extern "C" fn(*mut GThreadedSocketService) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GThreadedSocketService_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GThreadedSocketServicePrivate>() as gsize,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_threaded_socket_service_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_threaded_socket_service_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GThreadedSocketService_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GThreadedSocketService_private_offset,
        );
    }
    safe_c2rust_g_threaded_socket_service_class_init(klass as *mut GThreadedSocketServiceClass);
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_get_instance_private(
    mut self_0: *mut GThreadedSocketService,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GThreadedSocketService_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GThreadedSocketService_private_offset: gint = 0;
static mut safe_c2rust_g_threaded_socket_service_parent_class: gpointer = NULL;
static mut safe_c2rust_g__job_count_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_data_free(
    mut data: *mut GThreadedSocketServiceData,
) {
    let mut _pp: *mut *mut GThreadedSocketService = &raw mut (*data).service;
    let mut _ptr: *mut GThreadedSocketService = *_pp;
    *_pp = ::core::ptr::null_mut::<GThreadedSocketService>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GSocketConnection = &raw mut (*data).connection;
    let mut _ptr_0: *mut GSocketConnection = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GSocketConnection>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    let mut _pp_1: *mut *mut GObject = &raw mut (*data).source_object;
    let mut _ptr_1: *mut GObject = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GObject>();
    if !_ptr_1.is_null() {
        g_object_unref(_ptr_1 as gpointer);
    }
    g_slice_free1(
        ::core::mem::size_of::<GThreadedSocketServiceData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_func(
    mut job_data: gpointer,
    mut user_data: gpointer,
) {
    let mut data: *mut GThreadedSocketServiceData = job_data as *mut GThreadedSocketServiceData;
    let mut result: gboolean = 0;
    g_signal_emit(
        (*data).service as gpointer,
        safe_c2rust_g_threaded_socket_service_run_signal,
        0 as GQuark,
        (*data).connection,
        (*data).source_object,
        &raw mut result,
    );
    g_mutex_lock(&raw mut safe_c2rust_g__job_count_lock);
    let fresh0 = (*(*(*data).service).priv_0).job_count;
    (*(*(*data).service).priv_0).job_count = (*(*(*data).service).priv_0).job_count - 1;
    if fresh0 == (*(*(*data).service).priv_0).max_threads {
        g_socket_service_start((*data).service as *mut ::core::ffi::c_void as *mut GSocketService);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__job_count_lock);
    safe_c2rust_g_threaded_socket_service_data_free(data);
}
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_incoming(
    mut service: *mut GSocketService,
    mut connection: *mut GSocketConnection,
    mut source_object: *mut GObject,
) -> gboolean {
    let mut threaded: *mut GThreadedSocketService =
        ::core::ptr::null_mut::<GThreadedSocketService>();
    let mut data: *mut GThreadedSocketServiceData =
        ::core::ptr::null_mut::<GThreadedSocketServiceData>();
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    threaded = service as *mut ::core::ffi::c_void as *mut GThreadedSocketService;
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<GThreadedSocketServiceData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GThreadedSocketServiceData;
    (*data).service = g_object_ref(threaded as gpointer) as *mut GThreadedSocketService
        as *mut GThreadedSocketService;
    (*data).connection =
        g_object_ref(connection as gpointer) as *mut GSocketConnection as *mut GSocketConnection;
    (*data).source_object = (if !source_object.is_null() {
        g_object_ref(source_object as gpointer) as *mut GObject
    } else {
        ::core::ptr::null_mut::<GObject>()
    }) as *mut GObject;
    g_mutex_lock(&raw mut safe_c2rust_g__job_count_lock);
    (*(*threaded).priv_0).job_count += 1;
    if (*(*threaded).priv_0).job_count == (*(*threaded).priv_0).max_threads {
        g_socket_service_stop(service);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__job_count_lock);
    if g_thread_pool_push(
        (*(*threaded).priv_0).thread_pool,
        data as gpointer,
        &raw mut local_error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Error handling incoming socket: %s\0" as *const u8 as *const gchar,
            (*local_error).message,
        );
        safe_c2rust_g_threaded_socket_service_data_free(data);
    }
    g_clear_error(&raw mut local_error);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_init(
    mut service: *mut GThreadedSocketService,
) {
    (*service).priv_0 = safe_c2rust_g_threaded_socket_service_get_instance_private(service)
        as *mut GThreadedSocketServicePrivate;
    (*(*service).priv_0).max_threads = 10 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_constructed(mut object: *mut GObject) {
    let mut service: *mut GThreadedSocketService =
        object as *mut ::core::ffi::c_void as *mut GThreadedSocketService;
    (*(*service).priv_0).thread_pool = g_thread_pool_new(
        Some(
            safe_c2rust_g_threaded_socket_service_func
                as unsafe extern "C" fn(gpointer, gpointer) -> (),
        ),
        NULL,
        (*(*service).priv_0).max_threads as gint,
        FALSE,
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_finalize(mut object: *mut GObject) {
    let mut service: *mut GThreadedSocketService =
        object as *mut ::core::ffi::c_void as *mut GThreadedSocketService;
    g_thread_pool_free((*(*service).priv_0).thread_pool, FALSE, FALSE);
    (*(safe_c2rust_g_threaded_socket_service_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut service: *mut GThreadedSocketService =
        object as *mut ::core::ffi::c_void as *mut GThreadedSocketService;
    match prop_id as GThreadedSocketServiceProperty as ::core::ffi::c_uint {
        1 => {
            g_value_set_int(value, (*(*service).priv_0).max_threads as gint);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gthreadedsocketservice.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                189 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut service: *mut GThreadedSocketService =
        object as *mut ::core::ffi::c_void as *mut GThreadedSocketService;
    match prop_id as GThreadedSocketServiceProperty as ::core::ffi::c_uint {
        1 => {
            (*(*service).priv_0).max_threads = g_value_get_int(value) as ::core::ffi::c_int;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gthreadedsocketservice.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                208 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_class_init(
    mut class: *mut GThreadedSocketServiceClass,
) {
    let mut gobject_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut ss_class: *mut GSocketServiceClass = &raw mut (*class).parent_class;
    (*gobject_class).constructed = Some(
        safe_c2rust_g_threaded_socket_service_constructed
            as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_threaded_socket_service_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_threaded_socket_service_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_threaded_socket_service_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*ss_class).incoming = Some(
        safe_c2rust_g_threaded_socket_service_incoming
            as unsafe extern "C" fn(
                *mut GSocketService,
                *mut GSocketConnection,
                *mut GObject,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSocketService,
                *mut GSocketConnection,
                *mut GObject,
            ) -> gboolean,
        >;
    safe_c2rust_g_threaded_socket_service_run_signal = g_signal_new(
        g_intern_static_string(b"run\0" as *const u8 as *const gchar),
        (*(class as *mut GTypeClass)).g_type,
        G_SIGNAL_RUN_LAST,
        248 as ::core::ffi::c_ulong as glong as guint,
        Some(
            g_signal_accumulator_true_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL,
        Some(
            _g_cclosure_marshal_BOOLEAN__OBJECT_OBJECT
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        ),
        G_TYPE_BOOLEAN,
        2 as guint,
        g_socket_connection_get_type(),
        G_TYPE_OBJECT,
    );
    g_signal_set_va_marshaller(
        safe_c2rust_g_threaded_socket_service_run_signal,
        (*(class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_BOOLEAN__OBJECT_OBJECTv
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
    g_object_class_install_property(
        gobject_class,
        PROP_MAX_THREADS as ::core::ffi::c_int as guint,
        g_param_spec_int(
            b"max-threads\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            -(1 as gint),
            G_MAXINT,
            10 as gint,
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_threaded_socket_service_new(
    mut max_threads: ::core::ffi::c_int,
) -> *mut GSocketService {
    return g_object_new(
        safe_c2rust_g_threaded_socket_service_get_type(),
        b"max-threads\0" as *const u8 as *const gchar,
        max_threads,
        NULL,
    ) as *mut GSocketService;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
