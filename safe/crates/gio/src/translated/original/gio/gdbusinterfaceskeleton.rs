use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GTask;
    pub type _GDBusConnection;
    pub type _GDBusMethodInvocation;
    pub type _GDBusInterfaceInfo;
    pub type _GDBusInterface;
    pub type _GDBusObject;
    pub type _GDBusObjectSkeletonPrivate;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_reverse(list: *mut GList) -> *mut GList;
    fn g_slist_append(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_main_context_invoke_full(
        context: *mut GMainContext,
        priority: gint,
        function: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_variant_take_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_is_object_path(string: *const gchar) -> gboolean;
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
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
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
    fn g_signal_set_va_marshaller(
        signal_id: guint,
        instance_type: GType,
        va_marshaller: GSignalCVaMarshaller,
    );
    fn g_signal_emit(instance: gpointer, signal_id: guint, detail: GQuark, ...);
    fn g_signal_emit_by_name(instance: gpointer, detailed_signal: *const gchar, ...);
    fn g_signal_has_handler_pending(
        instance: gpointer,
        signal_id: guint,
        detail: GQuark,
        may_be_blocked: gboolean,
    ) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_add_weak_pointer(object: *mut GObject, weak_pointer_location: *mut gpointer);
    fn g_object_remove_weak_pointer(object: *mut GObject, weak_pointer_location: *mut gpointer);
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_dbus_interface_get_type() -> GType;
    fn g_dbus_interface_skeleton_flags_get_type() -> GType;
    fn _g_signal_accumulator_false_handled(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
    ) -> gboolean;
    fn _g_dbus_object_skeleton_has_authorize_method_handlers(
        object: *mut GDBusObjectSkeleton,
    ) -> gboolean;
    fn g_dbus_method_invocation_get_type() -> GType;
    fn g_dbus_method_invocation_get_sender(invocation: *mut GDBusMethodInvocation) -> *const gchar;
    fn g_dbus_method_invocation_get_object_path(
        invocation: *mut GDBusMethodInvocation,
    ) -> *const gchar;
    fn g_dbus_method_invocation_get_interface_name(
        invocation: *mut GDBusMethodInvocation,
    ) -> *const gchar;
    fn g_dbus_method_invocation_get_method_name(
        invocation: *mut GDBusMethodInvocation,
    ) -> *const gchar;
    fn g_dbus_method_invocation_get_connection(
        invocation: *mut GDBusMethodInvocation,
    ) -> *mut GDBusConnection;
    fn g_dbus_method_invocation_get_parameters(
        invocation: *mut GDBusMethodInvocation,
    ) -> *mut GVariant;
    fn g_dbus_method_invocation_get_user_data(invocation: *mut GDBusMethodInvocation) -> gpointer;
    fn g_dbus_connection_get_type() -> GType;
    fn g_dbus_connection_register_object(
        connection: *mut GDBusConnection,
        object_path: *const gchar,
        interface_info: *mut GDBusInterfaceInfo,
        vtable: *const GDBusInterfaceVTable,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
        error: *mut *mut GError,
    ) -> guint;
    fn g_dbus_connection_unregister_object(
        connection: *mut GDBusConnection,
        registration_id: guint,
    ) -> gboolean;
    fn _g_cclosure_marshal_BOOLEAN__OBJECT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_BOOLEAN__OBJECTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
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
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_get_priority(task: *mut GTask) -> gint;
    fn g_task_get_context(task: *mut GTask) -> *mut GMainContext;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
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
pub type gconstpointer = *const ::core::ffi::c_void;
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
pub type GMainContext = _GMainContext;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
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
pub type GDBusInterfaceSkeletonFlags = ::core::ffi::c_uint;
pub const G_DBUS_INTERFACE_SKELETON_FLAGS_HANDLE_METHOD_INVOCATIONS_IN_THREAD:
    GDBusInterfaceSkeletonFlags = 1;
pub const G_DBUS_INTERFACE_SKELETON_FLAGS_NONE: GDBusInterfaceSkeletonFlags = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GDBusConnection = _GDBusConnection;
pub type GDBusMethodInvocation = _GDBusMethodInvocation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceVTable {
    pub method_call: GDBusInterfaceMethodCallFunc,
    pub get_property: GDBusInterfaceGetPropertyFunc,
    pub set_property: GDBusInterfaceSetPropertyFunc,
    pub padding: [gpointer; 8],
}
pub type GDBusInterfaceSetPropertyFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        *mut *mut GError,
        gpointer,
    ) -> gboolean,
>;
pub type GDBusInterfaceGetPropertyFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut *mut GError,
        gpointer,
    ) -> *mut GVariant,
>;
pub type GDBusInterfaceMethodCallFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        *mut GDBusMethodInvocation,
        gpointer,
    ) -> (),
>;
pub type GDBusInterfaceVTable = _GDBusInterfaceVTable;
pub type GDBusInterfaceInfo = _GDBusInterfaceInfo;
pub type GDBusInterface = _GDBusInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceSkeleton {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusInterfaceSkeletonPrivate,
}
pub type GDBusInterfaceSkeletonPrivate = _GDBusInterfaceSkeletonPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceSkeletonPrivate {
    pub lock: GMutex,
    pub object: *mut GDBusObject,
    pub flags: GDBusInterfaceSkeletonFlags,
    pub connections: *mut GSList,
    pub object_path: *mut gchar,
    pub hooked_vtable: *mut GDBusInterfaceVTable,
}
pub type GDBusObject = _GDBusObject;
pub type GDBusInterfaceSkeleton = _GDBusInterfaceSkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectSkeleton {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusObjectSkeletonPrivate,
}
pub type GDBusObjectSkeletonPrivate = _GDBusObjectSkeletonPrivate;
pub type GDBusObjectSkeleton = _GDBusObjectSkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceIface {
    pub parent_iface: GTypeInterface,
    pub get_info: Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusInterfaceInfo>,
    pub get_object: Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject>,
    pub set_object: Option<unsafe extern "C" fn(*mut GDBusInterface, *mut GDBusObject) -> ()>,
    pub dup_object: Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject>,
}
pub type GDBusInterfaceIface = _GDBusInterfaceIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceSkeletonClass {
    pub parent_class: GObjectClass,
    pub get_info:
        Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceInfo>,
    pub get_vtable:
        Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GDBusInterfaceVTable>,
    pub get_properties: Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> *mut GVariant>,
    pub flush: Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> ()>,
    pub vfunc_padding: [gpointer; 8],
    pub g_authorize_method: Option<
        unsafe extern "C" fn(*mut GDBusInterfaceSkeleton, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub signal_padding: [gpointer; 8],
}
pub type GDBusInterfaceSkeletonClass = _GDBusInterfaceSkeletonClass;
pub const G_AUTHORIZE_METHOD_SIGNAL: C2RustUnnamed_0 = 0;
pub const PROP_G_FLAGS: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConnectionData {
    pub connection: *mut GDBusConnection,
    pub registration_id: guint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DispatchData {
    pub ref_count: gint,
    pub method_call_func: GDBusInterfaceMethodCallFunc,
    pub invocation: *mut GDBusMethodInvocation,
}
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_1 = 0;
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
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_interface_skeleton_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusInterfaceSkeleton\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusInterfaceSkeletonClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_interface_skeleton_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusInterfaceSkeleton>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_interface_skeleton_init
                    as unsafe extern "C" fn(*mut GDBusInterfaceSkeleton) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GDBusInterfaceSkeleton_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDBusInterfaceSkeletonPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusInterfaceIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_dbus_interface_interface_init
                as unsafe extern "C" fn(*mut GDBusInterfaceIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_dbus_interface_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_dbus_interface_skeleton_parent_class: gpointer = NULL_0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_instance_private(
    mut self_0: *mut GDBusInterfaceSkeleton,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDBusInterfaceSkeleton_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GDBusInterfaceSkeleton_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_interface_skeleton_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusInterfaceSkeleton_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusInterfaceSkeleton_private_offset,
        );
    }
    safe_c2rust_g_dbus_interface_skeleton_class_init(klass as *mut GDBusInterfaceSkeletonClass);
}
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_finalize(mut object: *mut GObject) {
    let mut interface: *mut GDBusInterfaceSkeleton =
        object as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton;
    g_mutex_lock(&raw mut (*(*interface).priv_0).lock);
    while !(*(*interface).priv_0).connections.is_null() {
        let mut data: *mut ConnectionData =
            (*(*(*interface).priv_0).connections).data as *mut ConnectionData;
        safe_c2rust_remove_connection_locked(interface, (*data).connection);
    }
    safe_c2rust_set_object_path_locked(interface, ::core::ptr::null::<gchar>());
    g_mutex_unlock(&raw mut (*(*interface).priv_0).lock);
    g_free((*(*interface).priv_0).hooked_vtable as gpointer);
    if !(*(*interface).priv_0).object.is_null() {
        g_object_remove_weak_pointer(
            (*(*interface).priv_0).object as *mut ::core::ffi::c_void as *mut GObject,
            &raw mut (*(*interface).priv_0).object as *mut gpointer,
        );
    }
    g_mutex_clear(&raw mut (*(*interface).priv_0).lock);
    (*(safe_c2rust_g_dbus_interface_skeleton_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut interface: *mut GDBusInterfaceSkeleton =
        object as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton;
    match prop_id {
        1 => {
            g_value_set_flags(
                value,
                safe_c2rust_g_dbus_interface_skeleton_get_flags(interface) as guint,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusinterfaceskeleton.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                142 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut interface: *mut GDBusInterfaceSkeleton =
        object as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton;
    match prop_id {
        1 => {
            safe_c2rust_g_dbus_interface_skeleton_set_flags(
                interface,
                g_value_get_flags(value) as GDBusInterfaceSkeletonFlags,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusinterfaceskeleton.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                162 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_g_authorize_method_default(
    mut interface: *mut GDBusInterfaceSkeleton,
    mut invocation: *mut GDBusMethodInvocation,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_class_init(
    mut klass: *mut GDBusInterfaceSkeletonClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_dbus_interface_skeleton_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dbus_interface_skeleton_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dbus_interface_skeleton_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*klass).g_authorize_method = Some(
        safe_c2rust_g_dbus_interface_skeleton_g_authorize_method_default
            as unsafe extern "C" fn(
                *mut GDBusInterfaceSkeleton,
                *mut GDBusMethodInvocation,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDBusInterfaceSkeleton,
                *mut GDBusMethodInvocation,
            ) -> gboolean,
        >;
    g_object_class_install_property(
        gobject_class,
        PROP_G_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"g-flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_interface_skeleton_flags_get_type(),
            G_DBUS_INTERFACE_SKELETON_FLAGS_NONE as ::core::ffi::c_int as guint,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    safe_c2rust_signals[G_AUTHORIZE_METHOD_SIGNAL as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"g-authorize-method\0" as *const u8 as *const gchar),
        safe_c2rust_g_dbus_interface_skeleton_get_type(),
        G_SIGNAL_RUN_LAST,
        232 as ::core::ffi::c_ulong as glong as guint,
        Some(
            _g_signal_accumulator_false_handled
                as unsafe extern "C" fn(
                    *mut GSignalInvocationHint,
                    *mut GValue,
                    *const GValue,
                    gpointer,
                ) -> gboolean,
        ),
        NULL_0,
        Some(
            _g_cclosure_marshal_BOOLEAN__OBJECT
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
        1 as guint,
        g_dbus_method_invocation_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[G_AUTHORIZE_METHOD_SIGNAL as ::core::ffi::c_int as usize],
        (*(klass as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_BOOLEAN__OBJECTv
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
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_init(
    mut interface: *mut GDBusInterfaceSkeleton,
) {
    (*interface).priv_0 = safe_c2rust_g_dbus_interface_skeleton_get_instance_private(interface)
        as *mut GDBusInterfaceSkeletonPrivate;
    g_mutex_init(&raw mut (*(*interface).priv_0).lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_flags(
    mut interface_: *mut GDBusInterfaceSkeleton,
) -> GDBusInterfaceSkeletonFlags {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return G_DBUS_INTERFACE_SKELETON_FLAGS_NONE;
    }
    return (*(*interface_).priv_0).flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_set_flags(
    mut interface_: *mut GDBusInterfaceSkeleton,
    mut flags: GDBusInterfaceSkeletonFlags,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*interface_).priv_0).lock);
    if (*(*interface_).priv_0).flags as ::core::ffi::c_uint != flags as ::core::ffi::c_uint {
        (*(*interface_).priv_0).flags = flags;
        g_mutex_unlock(&raw mut (*(*interface_).priv_0).lock);
        g_object_notify(
            interface_ as *mut ::core::ffi::c_void as *mut GObject,
            b"g-flags\0" as *const u8 as *const gchar,
        );
    } else {
        g_mutex_unlock(&raw mut (*(*interface_).priv_0).lock);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_info(
    mut interface_: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceInfo {
    let mut ret: *mut GDBusInterfaceInfo = ::core::ptr::null_mut::<GDBusInterfaceInfo>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusInterfaceInfo>();
    }
    ret = (*((*(interface_ as *mut GTypeInstance)).g_class as *mut GDBusInterfaceSkeletonClass))
        .get_info
        .expect("non-null function pointer")(interface_);
    if !(({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !ret.is_null() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusinterfaceskeleton.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            332 as ::core::ffi::c_int,
            G_STRFUNC,
            b"ret != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_vtable(
    mut interface_: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusInterfaceVTable {
    let mut ret: *mut GDBusInterfaceVTable = ::core::ptr::null_mut::<GDBusInterfaceVTable>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusInterfaceVTable>();
    }
    ret = (*((*(interface_ as *mut GTypeInstance)).g_class as *mut GDBusInterfaceSkeletonClass))
        .get_vtable
        .expect("non-null function pointer")(interface_);
    if !(({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !ret.is_null() {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusinterfaceskeleton.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            354 as ::core::ffi::c_int,
            G_STRFUNC,
            b"ret != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_properties(
    mut interface_: *mut GDBusInterfaceSkeleton,
) -> *mut GVariant {
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    ret = (*((*(interface_ as *mut GTypeInstance)).g_class as *mut GDBusInterfaceSkeletonClass))
        .get_properties
        .expect("non-null function pointer")(interface_);
    return g_variant_take_ref(ret);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_flush(
    mut interface_: *mut GDBusInterfaceSkeleton,
) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*((*(interface_ as *mut GTypeInstance)).g_class as *mut GDBusInterfaceSkeletonClass))
        .flush
        .expect("non-null function pointer")(interface_);
}
unsafe extern "C" fn safe_c2rust__g_dbus_interface_skeleton_get_info(
    mut interface_: *mut GDBusInterface,
) -> *mut GDBusInterfaceInfo {
    let mut interface: *mut GDBusInterfaceSkeleton =
        interface_ as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton;
    return safe_c2rust_g_dbus_interface_skeleton_get_info(interface);
}
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_object(
    mut interface_: *mut GDBusInterface,
) -> *mut GDBusObject {
    let mut interface: *mut GDBusInterfaceSkeleton =
        interface_ as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton;
    let mut ret: *mut GDBusObject = ::core::ptr::null_mut::<GDBusObject>();
    g_mutex_lock(&raw mut (*(*interface).priv_0).lock);
    ret = (*(*interface).priv_0).object;
    g_mutex_unlock(&raw mut (*(*interface).priv_0).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_dup_object(
    mut interface_: *mut GDBusInterface,
) -> *mut GDBusObject {
    let mut interface: *mut GDBusInterfaceSkeleton =
        interface_ as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton;
    let mut ret: *mut GDBusObject = ::core::ptr::null_mut::<GDBusObject>();
    g_mutex_lock(&raw mut (*(*interface).priv_0).lock);
    ret = (*(*interface).priv_0).object;
    if !ret.is_null() {
        g_object_ref(ret as gpointer);
    }
    g_mutex_unlock(&raw mut (*(*interface).priv_0).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_set_object(
    mut interface_: *mut GDBusInterface,
    mut object: *mut GDBusObject,
) {
    let mut interface: *mut GDBusInterfaceSkeleton =
        interface_ as *mut ::core::ffi::c_void as *mut GDBusInterfaceSkeleton;
    g_mutex_lock(&raw mut (*(*interface).priv_0).lock);
    if !(*(*interface).priv_0).object.is_null() {
        g_object_remove_weak_pointer(
            (*(*interface).priv_0).object as *mut ::core::ffi::c_void as *mut GObject,
            &raw mut (*(*interface).priv_0).object as *mut gpointer,
        );
    }
    (*(*interface).priv_0).object = object;
    if !object.is_null() {
        g_object_add_weak_pointer(
            (*(*interface).priv_0).object as *mut ::core::ffi::c_void as *mut GObject,
            &raw mut (*(*interface).priv_0).object as *mut gpointer,
        );
    }
    g_mutex_unlock(&raw mut (*(*interface).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust_dbus_interface_interface_init(
    mut iface: *mut GDBusInterfaceIface,
) {
    (*iface).get_info = Some(
        safe_c2rust__g_dbus_interface_skeleton_get_info
            as unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusInterfaceInfo,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusInterfaceInfo>;
    (*iface).get_object = Some(
        safe_c2rust_g_dbus_interface_skeleton_get_object
            as unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject>;
    (*iface).dup_object = Some(
        safe_c2rust_g_dbus_interface_skeleton_dup_object
            as unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject>;
    (*iface).set_object = Some(
        safe_c2rust_g_dbus_interface_skeleton_set_object
            as unsafe extern "C" fn(*mut GDBusInterface, *mut GDBusObject) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterface, *mut GDBusObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_dispatch_data_unref(mut data: *mut DispatchData) {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*data).ref_count;
            (*data).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*data).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        let mut _pp: *mut *mut GDBusMethodInvocation = &raw mut (*data).invocation;
        let mut _ptr: *mut GDBusMethodInvocation = *_pp;
        *_pp = ::core::ptr::null_mut::<GDBusMethodInvocation>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
        g_slice_free1(
            ::core::mem::size_of::<DispatchData>() as gsize,
            data as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_dispatch_data_ref(
    mut data: *mut DispatchData,
) -> *mut DispatchData {
    if 0 as ::core::ffi::c_int != 0 {
        (*data).ref_count;
        (*data).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*data).ref_count, 1 as ::core::ffi::c_int);
    return data;
}
unsafe extern "C" fn safe_c2rust_dispatch_invoke_in_context_func(
    mut user_data: gpointer,
) -> gboolean {
    let mut data: *mut DispatchData = user_data as *mut DispatchData;
    (*data).method_call_func.expect("non-null function pointer")(
        g_dbus_method_invocation_get_connection((*data).invocation),
        g_dbus_method_invocation_get_sender((*data).invocation),
        g_dbus_method_invocation_get_object_path((*data).invocation),
        g_dbus_method_invocation_get_interface_name((*data).invocation),
        g_dbus_method_invocation_get_method_name((*data).invocation),
        g_dbus_method_invocation_get_parameters((*data).invocation),
        (*data).invocation,
        g_dbus_method_invocation_get_user_data((*data).invocation),
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_dispatch_in_thread_func(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut data: *mut DispatchData = task_data as *mut DispatchData;
    let mut interface: *mut GDBusInterfaceSkeleton =
        g_task_get_source_object(task) as *mut GDBusInterfaceSkeleton;
    let mut flags: GDBusInterfaceSkeletonFlags = G_DBUS_INTERFACE_SKELETON_FLAGS_NONE;
    let mut object: *mut GDBusObject = ::core::ptr::null_mut::<GDBusObject>();
    let mut authorized: gboolean = 0;
    g_mutex_lock(&raw mut (*(*interface).priv_0).lock);
    flags = (*(*interface).priv_0).flags;
    object = (*(*interface).priv_0).object;
    if !object.is_null() {
        g_object_ref(object as gpointer);
    }
    g_mutex_unlock(&raw mut (*(*interface).priv_0).lock);
    authorized = TRUE as gboolean;
    if !object.is_null() {
        g_signal_emit_by_name(
            object as gpointer,
            b"authorize-method\0" as *const u8 as *const gchar,
            interface,
            (*data).invocation,
            &raw mut authorized,
        );
    }
    if authorized != 0 {
        g_signal_emit(
            interface as gpointer,
            safe_c2rust_signals[G_AUTHORIZE_METHOD_SIGNAL as ::core::ffi::c_int as usize],
            0 as GQuark,
            (*data).invocation,
            &raw mut authorized,
        );
    }
    if authorized != 0 {
        let mut run_in_thread: gboolean = 0;
        run_in_thread = (flags as ::core::ffi::c_uint
            & G_DBUS_INTERFACE_SKELETON_FLAGS_HANDLE_METHOD_INVOCATIONS_IN_THREAD
                as ::core::ffi::c_int as ::core::ffi::c_uint) as gboolean;
        if run_in_thread != 0 {
            (*data).method_call_func.expect("non-null function pointer")(
                g_dbus_method_invocation_get_connection((*data).invocation),
                g_dbus_method_invocation_get_sender((*data).invocation),
                g_dbus_method_invocation_get_object_path((*data).invocation),
                g_dbus_method_invocation_get_interface_name((*data).invocation),
                g_dbus_method_invocation_get_method_name((*data).invocation),
                g_dbus_method_invocation_get_parameters((*data).invocation),
                (*data).invocation,
                g_dbus_method_invocation_get_user_data((*data).invocation),
            );
        } else {
            g_main_context_invoke_full(
                g_task_get_context(task),
                g_task_get_priority(task),
                Some(
                    safe_c2rust_dispatch_invoke_in_context_func
                        as unsafe extern "C" fn(gpointer) -> gboolean,
                ),
                safe_c2rust_dispatch_data_ref(data) as gpointer,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut DispatchData) -> ()>,
                    GDestroyNotify,
                >(Some(
                    safe_c2rust_dispatch_data_unref
                        as unsafe extern "C" fn(*mut DispatchData) -> (),
                )),
            );
        }
    }
    if !object.is_null() {
        g_object_unref(object as gpointer);
    }
    g_task_return_boolean(task, TRUE);
}
unsafe extern "C" fn safe_c2rust_g_dbus_interface_method_dispatch_helper(
    mut interface: *mut GDBusInterfaceSkeleton,
    mut method_call_func: GDBusInterfaceMethodCallFunc,
    mut invocation: *mut GDBusMethodInvocation,
) {
    let mut has_handlers: gboolean = 0;
    let mut has_default_class_handler: gboolean = 0;
    let mut emit_authorized_signal: gboolean = 0;
    let mut run_in_thread: gboolean = 0;
    let mut flags: GDBusInterfaceSkeletonFlags = G_DBUS_INTERFACE_SKELETON_FLAGS_NONE;
    let mut object: *mut GDBusObject = ::core::ptr::null_mut::<GDBusObject>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if method_call_func.is_some() {
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
            b"method_call_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = invocation as *mut GTypeInstance;
            let mut __t: GType = g_dbus_method_invocation_get_type();
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
            b"G_IS_DBUS_METHOD_INVOCATION (invocation)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*interface).priv_0).lock);
    flags = (*(*interface).priv_0).flags;
    object = (*(*interface).priv_0).object;
    if !object.is_null() {
        g_object_ref(object as gpointer);
    }
    g_mutex_unlock(&raw mut (*(*interface).priv_0).lock);
    has_handlers = g_signal_has_handler_pending(
        interface as gpointer,
        safe_c2rust_signals[G_AUTHORIZE_METHOD_SIGNAL as ::core::ffi::c_int as usize],
        0 as GQuark,
        TRUE,
    );
    has_default_class_handler = ((*((*(interface as *mut GTypeInstance)).g_class
        as *mut GDBusInterfaceSkeletonClass))
        .g_authorize_method
        == Some(
            safe_c2rust_g_dbus_interface_skeleton_g_authorize_method_default
                as unsafe extern "C" fn(
                    *mut GDBusInterfaceSkeleton,
                    *mut GDBusMethodInvocation,
                ) -> gboolean,
        )) as ::core::ffi::c_int as gboolean;
    emit_authorized_signal =
        (has_handlers != 0 || has_default_class_handler == 0) as ::core::ffi::c_int as gboolean;
    if emit_authorized_signal == 0 {
        if !object.is_null() {
            emit_authorized_signal = _g_dbus_object_skeleton_has_authorize_method_handlers(
                object as *mut ::core::ffi::c_void as *mut GDBusObjectSkeleton,
            );
        }
    }
    run_in_thread = (flags as ::core::ffi::c_uint
        & G_DBUS_INTERFACE_SKELETON_FLAGS_HANDLE_METHOD_INVOCATIONS_IN_THREAD as ::core::ffi::c_int
            as ::core::ffi::c_uint) as gboolean;
    if emit_authorized_signal == 0 && run_in_thread == 0 {
        method_call_func.expect("non-null function pointer")(
            g_dbus_method_invocation_get_connection(invocation),
            g_dbus_method_invocation_get_sender(invocation),
            g_dbus_method_invocation_get_object_path(invocation),
            g_dbus_method_invocation_get_interface_name(invocation),
            g_dbus_method_invocation_get_method_name(invocation),
            g_dbus_method_invocation_get_parameters(invocation),
            invocation,
            g_dbus_method_invocation_get_user_data(invocation),
        );
    } else {
        let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
        let mut data: *mut DispatchData = ::core::ptr::null_mut::<DispatchData>();
        data = ({
            let mut __s: gsize = ::core::mem::size_of::<DispatchData>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            __p = g_slice_alloc(__s);
            memset(
                __p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                __s as size_t,
            );
            __p
        }) as *mut DispatchData;
        (*data).method_call_func = method_call_func;
        (*data).invocation = g_object_ref(invocation as gpointer) as *mut GDBusMethodInvocation
            as *mut GDBusMethodInvocation;
        (*data).ref_count = 1 as ::core::ffi::c_int as gint;
        task = g_task_new(
            interface as gpointer,
            ::core::ptr::null_mut::<GCancellable>(),
            None,
            NULL_0,
        );
        let mut _task: *mut GTask = task;
        g_task_set_source_tag(
            _task,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GDBusInterfaceSkeleton,
                        GDBusInterfaceMethodCallFunc,
                        *mut GDBusMethodInvocation,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_dbus_interface_method_dispatch_helper
                    as unsafe extern "C" fn(
                        *mut GDBusInterfaceSkeleton,
                        GDBusInterfaceMethodCallFunc,
                        *mut GDBusMethodInvocation,
                    ) -> (),
            )),
        );
        if g_task_get_name(_task).is_null() {
            g_task_set_static_name(
                _task,
                b"g_dbus_interface_method_dispatch_helper\0" as *const u8 as *const gchar,
            );
        }
        let mut _task_0: *mut GTask = task;
        if 0 != 0 {
            g_task_set_static_name(
                _task_0,
                b"[gio] D-Bus interface method dispatch\0" as *const u8 as *const gchar,
            );
        } else {
            g_task_set_name(
                _task_0,
                b"[gio] D-Bus interface method dispatch\0" as *const u8 as *const gchar,
            );
        }
        g_task_set_task_data(
            task,
            data as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut DispatchData) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_dispatch_data_unref as unsafe extern "C" fn(*mut DispatchData) -> (),
            )),
        );
        g_task_run_in_thread(
            task,
            Some(
                safe_c2rust_dispatch_in_thread_func
                    as unsafe extern "C" fn(
                        *mut GTask,
                        gpointer,
                        gpointer,
                        *mut GCancellable,
                    ) -> (),
            ),
        );
        g_object_unref(task as gpointer);
    }
    if !object.is_null() {
        g_object_unref(object as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_skeleton_intercept_handle_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut interface: *mut GDBusInterfaceSkeleton = user_data as *mut GDBusInterfaceSkeleton;
    safe_c2rust_g_dbus_interface_method_dispatch_helper(
        interface,
        (*safe_c2rust_g_dbus_interface_skeleton_get_vtable(interface)).method_call,
        invocation,
    );
}
unsafe extern "C" fn safe_c2rust_new_connection(
    mut connection: *mut GDBusConnection,
    mut registration_id: guint,
) -> *mut ConnectionData {
    let mut data: *mut ConnectionData = ::core::ptr::null_mut::<ConnectionData>();
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<ConnectionData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut ConnectionData;
    (*data).connection =
        g_object_ref(connection as gpointer) as *mut GDBusConnection as *mut GDBusConnection;
    (*data).registration_id = registration_id;
    return data;
}
unsafe extern "C" fn safe_c2rust_free_connection(mut data: *mut ConnectionData) {
    if !data.is_null() {
        g_object_unref((*data).connection as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<ConnectionData>() as gsize,
            data as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_add_connection_locked(
    mut interface_: *mut GDBusInterfaceSkeleton,
    mut connection: *mut GDBusConnection,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut data: *mut ConnectionData = ::core::ptr::null_mut::<ConnectionData>();
    let mut registration_id: guint = 0;
    let mut ret: gboolean = FALSE;
    if (*(*interface_).priv_0).hooked_vtable.is_null() {
        (*(*interface_).priv_0).hooked_vtable = g_memdup2(
            safe_c2rust_g_dbus_interface_skeleton_get_vtable(interface_) as gconstpointer,
            ::core::mem::size_of::<GDBusInterfaceVTable>() as gsize,
        ) as *mut GDBusInterfaceVTable;
        (*(*(*interface_).priv_0).hooked_vtable).method_call = Some(
            safe_c2rust_skeleton_intercept_handle_method_call
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *const gchar,
                    *mut GVariant,
                    *mut GDBusMethodInvocation,
                    gpointer,
                ) -> (),
        )
            as GDBusInterfaceMethodCallFunc;
    }
    registration_id = g_dbus_connection_register_object(
        connection,
        (*(*interface_).priv_0).object_path,
        safe_c2rust_g_dbus_interface_skeleton_get_info(interface_),
        (*(*interface_).priv_0).hooked_vtable,
        interface_ as gpointer,
        None,
        error,
    );
    if registration_id > 0 as guint {
        data = safe_c2rust_new_connection(connection, registration_id);
        (*(*interface_).priv_0).connections =
            g_slist_append((*(*interface_).priv_0).connections, data as gpointer);
        ret = TRUE as gboolean;
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_remove_connection_locked(
    mut interface_: *mut GDBusInterfaceSkeleton,
    mut connection: *mut GDBusConnection,
) {
    let mut data: *mut ConnectionData = ::core::ptr::null_mut::<ConnectionData>();
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    l = (*(*interface_).priv_0).connections;
    while !l.is_null() {
        data = (*l).data as *mut ConnectionData;
        if (*data).connection == connection {
            if !(({
                let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                if g_dbus_connection_unregister_object((*data).connection, (*data).registration_id)
                    != 0
                {
                    _g_boolean_var_21 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_21 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_21
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusinterfaceskeleton.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    744 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_dbus_connection_unregister_object (data->connection, data->registration_id)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            safe_c2rust_free_connection(data);
            (*(*interface_).priv_0).connections =
                g_slist_delete_link((*(*interface_).priv_0).connections, l);
            break;
        } else {
            l = (*l).next;
        }
    }
}
unsafe extern "C" fn safe_c2rust_set_object_path_locked(
    mut interface_: *mut GDBusInterfaceSkeleton,
    mut object_path: *const gchar,
) {
    if g_strcmp0(
        (*(*interface_).priv_0).object_path,
        object_path as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        g_free((*(*interface_).priv_0).object_path as gpointer);
        (*(*interface_).priv_0).object_path =
            safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as *mut gchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_connection(
    mut interface_: *mut GDBusInterfaceSkeleton,
) -> *mut GDBusConnection {
    let mut data: *mut ConnectionData = ::core::ptr::null_mut::<ConnectionData>();
    let mut ret: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    g_mutex_lock(&raw mut (*(*interface_).priv_0).lock);
    ret = ::core::ptr::null_mut::<GDBusConnection>();
    if !(*(*interface_).priv_0).connections.is_null() {
        data = (*(*(*interface_).priv_0).connections).data as *mut ConnectionData;
        if !data.is_null() {
            ret = (*data).connection;
        }
    }
    g_mutex_unlock(&raw mut (*(*interface_).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_connections(
    mut interface_: *mut GDBusInterfaceSkeleton,
) -> *mut GList {
    let mut connections: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut data: *mut ConnectionData = ::core::ptr::null_mut::<ConnectionData>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    g_mutex_lock(&raw mut (*(*interface_).priv_0).lock);
    connections = ::core::ptr::null_mut::<GList>();
    l = (*(*interface_).priv_0).connections;
    while !l.is_null() {
        data = (*l).data as *mut ConnectionData;
        connections = g_list_prepend(
            connections,
            g_object_ref((*data).connection as gpointer) as *mut GDBusConnection as gpointer,
        );
        l = (*l).next;
    }
    g_mutex_unlock(&raw mut (*(*interface_).priv_0).lock);
    return g_list_reverse(connections);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_has_connection(
    mut interface_: *mut GDBusInterfaceSkeleton,
    mut connection: *mut GDBusConnection,
) -> gboolean {
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut ret: gboolean = FALSE;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut (*(*interface_).priv_0).lock);
    l = (*(*interface_).priv_0).connections;
    while !l.is_null() {
        let mut data: *mut ConnectionData = (*l).data as *mut ConnectionData;
        if (*data).connection == connection {
            ret = TRUE as gboolean;
            break;
        } else {
            l = (*l).next;
        }
    }
    g_mutex_unlock(&raw mut (*(*interface_).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_get_object_path(
    mut interface_: *mut GDBusInterfaceSkeleton,
) -> *const gchar {
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    g_mutex_lock(&raw mut (*(*interface_).priv_0).lock);
    ret = (*(*interface_).priv_0).object_path;
    g_mutex_unlock(&raw mut (*(*interface_).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_export(
    mut interface_: *mut GDBusInterfaceSkeleton,
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = FALSE;
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if (*(*interface_).priv_0).object_path.is_null()
            || g_strcmp0(
                (*(*interface_).priv_0).object_path,
                object_path as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
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
            b"interface_->priv->object_path == NULL || g_strcmp0 (interface_->priv->object_path, object_path) == 0\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut (*(*interface_).priv_0).lock);
    safe_c2rust_set_object_path_locked(interface_, object_path);
    ret = safe_c2rust_add_connection_locked(interface_, connection, error);
    g_mutex_unlock(&raw mut (*(*interface_).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_unexport(
    mut interface_: *mut GDBusInterfaceSkeleton,
) {
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !(*(*interface_).priv_0).connections.is_null() {
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
            b"interface_->priv->connections != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*interface_).priv_0).lock);
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !(*(*interface_).priv_0).object_path.is_null() {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusinterfaceskeleton.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            967 as ::core::ffi::c_int,
            G_STRFUNC,
            b"interface_->priv->object_path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !(*(*interface_).priv_0).hooked_vtable.is_null() {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusinterfaceskeleton.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            968 as ::core::ffi::c_int,
            G_STRFUNC,
            b"interface_->priv->hooked_vtable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    while !(*(*interface_).priv_0).connections.is_null() {
        let mut data: *mut ConnectionData =
            (*(*(*interface_).priv_0).connections).data as *mut ConnectionData;
        safe_c2rust_remove_connection_locked(interface_, (*data).connection);
    }
    safe_c2rust_set_object_path_locked(interface_, ::core::ptr::null::<gchar>());
    g_mutex_unlock(&raw mut (*(*interface_).priv_0).lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_interface_skeleton_unexport_from_connection(
    mut interface_: *mut GDBusInterfaceSkeleton,
    mut connection: *mut GDBusConnection,
) {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_interface_skeleton_get_type();
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
            b"G_IS_DBUS_INTERFACE_SKELETON (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !(*(*interface_).priv_0).connections.is_null() {
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
            b"interface_->priv->connections != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*interface_).priv_0).lock);
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !(*(*interface_).priv_0).object_path.is_null() {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusinterfaceskeleton.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1006 as ::core::ffi::c_int,
            G_STRFUNC,
            b"interface_->priv->object_path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !(*(*interface_).priv_0).hooked_vtable.is_null() {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusinterfaceskeleton.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1007 as ::core::ffi::c_int,
            G_STRFUNC,
            b"interface_->priv->hooked_vtable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_remove_connection_locked(interface_, connection);
    if (*(*interface_).priv_0).connections.is_null() {
        safe_c2rust_set_object_path_locked(interface_, ::core::ptr::null::<gchar>());
    }
    g_mutex_unlock(&raw mut (*(*interface_).priv_0).lock);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
