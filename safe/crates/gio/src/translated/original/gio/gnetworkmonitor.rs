use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GNetworkMonitor;
    pub type _GSocketConnectable;
    pub type _GTask;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_object_interface_install_property(g_iface: gpointer, pspec: *mut GParamSpec);
    fn g_object_get(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_unref(object: gpointer);
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_enum(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        enum_type: GType,
        default_value: gint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_initable_get_type() -> GType;
    fn g_network_connectivity_get_type() -> GType;
    fn _g_io_module_get_default(
        extension_point: *const gchar,
        envvar: *const gchar,
        verify_func: GIOModuleVerifyFunc,
    ) -> gpointer;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
}
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
pub type GNetworkConnectivity = ::core::ffi::c_uint;
pub const G_NETWORK_CONNECTIVITY_FULL: GNetworkConnectivity = 4;
pub const G_NETWORK_CONNECTIVITY_PORTAL: GNetworkConnectivity = 3;
pub const G_NETWORK_CONNECTIVITY_LIMITED: GNetworkConnectivity = 2;
pub const G_NETWORK_CONNECTIVITY_LOCAL: GNetworkConnectivity = 1;
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
pub type GNetworkMonitor = _GNetworkMonitor;
pub type GSocketConnectable = _GSocketConnectable;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNetworkMonitorInterface {
    pub g_iface: GTypeInterface,
    pub network_changed: Option<unsafe extern "C" fn(*mut GNetworkMonitor, gboolean) -> ()>,
    pub can_reach: Option<
        unsafe extern "C" fn(
            *mut GNetworkMonitor,
            *mut GSocketConnectable,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub can_reach_async: Option<
        unsafe extern "C" fn(
            *mut GNetworkMonitor,
            *mut GSocketConnectable,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub can_reach_finish: Option<
        unsafe extern "C" fn(*mut GNetworkMonitor, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
}
pub type GNetworkMonitorInterface = _GNetworkMonitorInterface;
pub const NETWORK_CHANGED: C2RustUnnamed_0 = 0;
pub type GIOModuleVerifyFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_0 = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_INVALID: GType = ((0 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INTERFACE: GType =
    ((2 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_NETWORK_MONITOR_EXTENSION_POINT_NAME: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"gio-network-monitor\0")
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_get_type() -> GType {
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
            g_intern_static_string(b"GNetworkMonitor\0" as *const u8 as *const gchar),
            ::core::mem::size_of::<GNetworkMonitorInterface>() as guint,
            ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GNetworkMonitorInterface) -> ()>,
                    Option<unsafe extern "C" fn() -> ()>,
                >(Some(
                    safe_c2rust_g_network_monitor_default_init
                        as unsafe extern "C" fn(*mut GNetworkMonitorInterface) -> (),
                )),
            ),
            0 as guint,
            ::core::mem::transmute::<*mut ::core::ffi::c_void, GInstanceInitFunc>(NULL),
            G_TYPE_FLAG_NONE,
        );
        if ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType != G_TYPE_INVALID {
            g_type_interface_add_prerequisite(
                g_define_type_id,
                ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            );
        }
        g_type_interface_add_prerequisite(g_define_type_id, g_initable_get_type());
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
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
static mut safe_c2rust_network_monitor_default_singleton: *mut GNetworkMonitor =
    ::core::ptr::null::<GNetworkMonitor>() as *mut GNetworkMonitor;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_get_default() -> *mut GNetworkMonitor {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_network_monitor_default_singleton;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GNetworkMonitor =
                ::core::ptr::null_mut::<GNetworkMonitor>();
            let mut gapg_temp_atomic: *mut *mut GNetworkMonitor =
                &raw mut safe_c2rust_network_monitor_default_singleton;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_network_monitor_default_singleton as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut singleton: *mut GNetworkMonitor = ::core::ptr::null_mut::<GNetworkMonitor>();
        singleton = _g_io_module_get_default(
            G_NETWORK_MONITOR_EXTENSION_POINT_NAME.as_ptr() as *const gchar,
            b"GIO_USE_NETWORK_MONITOR\0" as *const u8 as *const gchar,
            None,
        ) as *mut GNetworkMonitor;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_network_monitor_default_singleton = singleton;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_network_monitor_default_singleton as *mut ::core::ffi::c_void,
            singleton as guintptr as gpointer,
        );
    }
    return safe_c2rust_network_monitor_default_singleton;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_get_network_available(
    mut monitor: *mut GNetworkMonitor,
) -> gboolean {
    let mut available: gboolean = FALSE;
    g_object_get(
        monitor as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"network-available\0" as *const u8 as *const gchar,
        &raw mut available,
        NULL,
    );
    return available;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_get_network_metered(
    mut monitor: *mut GNetworkMonitor,
) -> gboolean {
    let mut metered: gboolean = FALSE;
    g_object_get(
        monitor as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"network-metered\0" as *const u8 as *const gchar,
        &raw mut metered,
        NULL,
    );
    return metered;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_get_connectivity(
    mut monitor: *mut GNetworkMonitor,
) -> GNetworkConnectivity {
    let mut connectivity: GNetworkConnectivity = 0 as GNetworkConnectivity;
    g_object_get(
        monitor as *mut ::core::ffi::c_void as *mut GObject as gpointer,
        b"connectivity\0" as *const u8 as *const gchar,
        &raw mut connectivity,
        NULL,
    );
    return connectivity;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_can_reach(
    mut monitor: *mut GNetworkMonitor,
    mut connectable: *mut GSocketConnectable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GNetworkMonitorInterface =
        ::core::ptr::null_mut::<GNetworkMonitorInterface>();
    iface = g_type_interface_peek(
        (*(monitor as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_network_monitor_get_type(),
    ) as *mut GNetworkMonitorInterface;
    return (*iface).can_reach.expect("non-null function pointer")(
        monitor,
        connectable,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_real_can_reach_async(
    mut monitor: *mut GNetworkMonitor,
    mut connectable: *mut GSocketConnectable,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    task = g_task_new(monitor as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GNetworkMonitor,
                    *mut GSocketConnectable,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_network_monitor_real_can_reach_async
                as unsafe extern "C" fn(
                    *mut GNetworkMonitor,
                    *mut GSocketConnectable,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_network_monitor_real_can_reach_async\0" as *const u8 as *const gchar,
        );
    }
    if safe_c2rust_g_network_monitor_can_reach(monitor, connectable, cancellable, &raw mut error)
        != 0
    {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_can_reach_async(
    mut monitor: *mut GNetworkMonitor,
    mut connectable: *mut GSocketConnectable,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut iface: *mut GNetworkMonitorInterface =
        ::core::ptr::null_mut::<GNetworkMonitorInterface>();
    iface = g_type_interface_peek(
        (*(monitor as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_network_monitor_get_type(),
    ) as *mut GNetworkMonitorInterface;
    (*iface).can_reach_async.expect("non-null function pointer")(
        monitor,
        connectable,
        cancellable,
        callback,
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_real_can_reach_finish(
    mut monitor: *mut GNetworkMonitor,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if g_task_is_valid(result as gpointer, monitor as gpointer) != 0 {
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
            b"g_task_is_valid (result, monitor)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(result as *mut ::core::ffi::c_void as *mut GTask, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_network_monitor_can_reach_finish(
    mut monitor: *mut GNetworkMonitor,
    mut result: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut iface: *mut GNetworkMonitorInterface =
        ::core::ptr::null_mut::<GNetworkMonitorInterface>();
    iface = g_type_interface_peek(
        (*(monitor as *mut GTypeInstance)).g_class as gpointer,
        safe_c2rust_g_network_monitor_get_type(),
    ) as *mut GNetworkMonitorInterface;
    return (*iface)
        .can_reach_finish
        .expect("non-null function pointer")(monitor, result, error);
}
unsafe extern "C" fn safe_c2rust_g_network_monitor_default_init(
    mut iface: *mut GNetworkMonitorInterface,
) {
    (*iface).can_reach_async = Some(
        safe_c2rust_g_network_monitor_real_can_reach_async
            as unsafe extern "C" fn(
                *mut GNetworkMonitor,
                *mut GSocketConnectable,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GNetworkMonitor,
                *mut GSocketConnectable,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*iface).can_reach_finish = Some(
        safe_c2rust_g_network_monitor_real_can_reach_finish
            as unsafe extern "C" fn(
                *mut GNetworkMonitor,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GNetworkMonitor,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
        >;
    safe_c2rust_signals[NETWORK_CHANGED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"network-changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_network_monitor_get_type(),
        G_SIGNAL_RUN_LAST,
        16 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL,
        None,
        G_TYPE_NONE,
        1 as guint,
        G_TYPE_BOOLEAN,
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_boolean(
            b"network-available\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_boolean(
            b"network-metered\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_interface_install_property(
        iface as gpointer,
        g_param_spec_enum(
            b"connectivity\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_network_connectivity_get_type(),
            G_NETWORK_CONNECTIVITY_FULL as ::core::ffi::c_int as gint,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
