use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GInitable;
    pub type _GIOExtension;
    pub type _GTask;
    pub type _GDBusConnection;
    pub type _GDBusMethodInvocation;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_remove_index_fast(array: *mut GPtrArray, index_: guint) -> gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new(domain: GQuark, code: gint, format: *const gchar, ...) -> *mut GError;
    fn g_thread_yield();
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_new_boolean(value: gboolean) -> *mut GVariant;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_log_set_debug_enabled(enabled: gboolean);
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
    fn g_signal_has_handler_pending(
        instance: gpointer,
        signal_id: guint,
        detail: GQuark,
        may_be_blocked: gboolean,
    ) -> gboolean;
    fn g_object_class_override_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        name: *const gchar,
    );
    fn g_object_class_install_properties(
        oclass: *mut GObjectClass,
        n_pspecs: guint,
        pspecs: *mut *mut GParamSpec,
    );
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_weak_ref_init(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_weak_ref_clear(weak_ref: *mut GWeakRef);
    fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> gpointer;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_initable_get_type() -> GType;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_cancellable_get_type() -> GType;
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_is_cancelled(cancellable: *mut GCancellable) -> gboolean;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_dbus_connection_get_type() -> GType;
    fn g_dbus_connection_emit_signal(
        connection: *mut GDBusConnection,
        destination_bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        signal_name: *const gchar,
        parameters: *mut GVariant,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn g_dbus_error_quark() -> GQuark;
    fn g_dbus_node_info_new_for_xml(
        xml_data: *const gchar,
        error: *mut *mut GError,
    ) -> *mut GDBusNodeInfo;
    fn g_dbus_node_info_lookup_interface(
        info: *mut GDBusNodeInfo,
        name: *const gchar,
    ) -> *mut GDBusInterfaceInfo;
    fn g_dbus_interface_info_ref(info: *mut GDBusInterfaceInfo) -> *mut GDBusInterfaceInfo;
    fn g_dbus_node_info_unref(info: *mut GDBusNodeInfo);
    fn g_dbus_method_invocation_get_type() -> GType;
    fn g_dbus_method_invocation_get_parameters(
        invocation: *mut GDBusMethodInvocation,
    ) -> *mut GVariant;
    fn g_dbus_method_invocation_return_value(
        invocation: *mut GDBusMethodInvocation,
        parameters: *mut GVariant,
    );
    fn g_dbus_method_invocation_take_error(
        invocation: *mut GDBusMethodInvocation,
        error: *mut GError,
    );
    fn g_debug_controller_get_type() -> GType;
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
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
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn _g_io_modules_ensure_extension_points_registered();
    fn dcgettext(
        __domainname: *const ::core::ffi::c_char,
        __msgid: *const ::core::ffi::c_char,
        __category: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn _g_signal_accumulator_false_handled(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
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
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
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
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: C2RustUnnamed_0,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
pub type GVariantBuilder = _GVariantBuilder;
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
    pub data: [C2RustUnnamed_1; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GWeakRef {
    pub priv_0: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub p: gpointer,
}
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const G_DBUS_ERROR_PROPERTY_READ_ONLY: C2RustUnnamed_3 = 44;
pub const G_DBUS_ERROR_UNKNOWN_PROPERTY: C2RustUnnamed_3 = 43;
pub const G_DBUS_ERROR_UNKNOWN_INTERFACE: C2RustUnnamed_3 = 42;
pub const G_DBUS_ERROR_UNKNOWN_OBJECT: C2RustUnnamed_3 = 41;
pub const G_DBUS_ERROR_OBJECT_PATH_IN_USE: C2RustUnnamed_3 = 40;
pub const G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN: C2RustUnnamed_3 = 39;
pub const G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN: C2RustUnnamed_3 = 38;
pub const G_DBUS_ERROR_INVALID_FILE_CONTENT: C2RustUnnamed_3 = 37;
pub const G_DBUS_ERROR_INVALID_SIGNATURE: C2RustUnnamed_3 = 36;
pub const G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN: C2RustUnnamed_3 = 35;
pub const G_DBUS_ERROR_SPAWN_NO_MEMORY: C2RustUnnamed_3 = 34;
pub const G_DBUS_ERROR_SPAWN_FILE_INVALID: C2RustUnnamed_3 = 33;
pub const G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID: C2RustUnnamed_3 = 32;
pub const G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND: C2RustUnnamed_3 = 31;
pub const G_DBUS_ERROR_SPAWN_SERVICE_INVALID: C2RustUnnamed_3 = 30;
pub const G_DBUS_ERROR_SPAWN_CONFIG_INVALID: C2RustUnnamed_3 = 29;
pub const G_DBUS_ERROR_SPAWN_SETUP_FAILED: C2RustUnnamed_3 = 28;
pub const G_DBUS_ERROR_SPAWN_FAILED: C2RustUnnamed_3 = 27;
pub const G_DBUS_ERROR_SPAWN_CHILD_SIGNALED: C2RustUnnamed_3 = 26;
pub const G_DBUS_ERROR_SPAWN_CHILD_EXITED: C2RustUnnamed_3 = 25;
pub const G_DBUS_ERROR_SPAWN_FORK_FAILED: C2RustUnnamed_3 = 24;
pub const G_DBUS_ERROR_SPAWN_EXEC_FAILED: C2RustUnnamed_3 = 23;
pub const G_DBUS_ERROR_MATCH_RULE_INVALID: C2RustUnnamed_3 = 22;
pub const G_DBUS_ERROR_MATCH_RULE_NOT_FOUND: C2RustUnnamed_3 = 21;
pub const G_DBUS_ERROR_TIMED_OUT: C2RustUnnamed_3 = 20;
pub const G_DBUS_ERROR_UNKNOWN_METHOD: C2RustUnnamed_3 = 19;
pub const G_DBUS_ERROR_FILE_EXISTS: C2RustUnnamed_3 = 18;
pub const G_DBUS_ERROR_FILE_NOT_FOUND: C2RustUnnamed_3 = 17;
pub const G_DBUS_ERROR_INVALID_ARGS: C2RustUnnamed_3 = 16;
pub const G_DBUS_ERROR_DISCONNECTED: C2RustUnnamed_3 = 15;
pub const G_DBUS_ERROR_ADDRESS_IN_USE: C2RustUnnamed_3 = 14;
pub const G_DBUS_ERROR_NO_NETWORK: C2RustUnnamed_3 = 13;
pub const G_DBUS_ERROR_TIMEOUT: C2RustUnnamed_3 = 12;
pub const G_DBUS_ERROR_NO_SERVER: C2RustUnnamed_3 = 11;
pub const G_DBUS_ERROR_AUTH_FAILED: C2RustUnnamed_3 = 10;
pub const G_DBUS_ERROR_ACCESS_DENIED: C2RustUnnamed_3 = 9;
pub const G_DBUS_ERROR_LIMITS_EXCEEDED: C2RustUnnamed_3 = 8;
pub const G_DBUS_ERROR_NOT_SUPPORTED: C2RustUnnamed_3 = 7;
pub const G_DBUS_ERROR_BAD_ADDRESS: C2RustUnnamed_3 = 6;
pub const G_DBUS_ERROR_IO_ERROR: C2RustUnnamed_3 = 5;
pub const G_DBUS_ERROR_NO_REPLY: C2RustUnnamed_3 = 4;
pub const G_DBUS_ERROR_NAME_HAS_NO_OWNER: C2RustUnnamed_3 = 3;
pub const G_DBUS_ERROR_SERVICE_UNKNOWN: C2RustUnnamed_3 = 2;
pub const G_DBUS_ERROR_NO_MEMORY: C2RustUnnamed_3 = 1;
pub const G_DBUS_ERROR_FAILED: C2RustUnnamed_3 = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GInitable = _GInitable;
pub type GIOExtension = _GIOExtension;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub methods: *mut *mut GDBusMethodInfo,
    pub signals: *mut *mut GDBusSignalInfo,
    pub properties: *mut *mut GDBusPropertyInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusAnnotationInfo = _GDBusAnnotationInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAnnotationInfo {
    pub ref_count: gint,
    pub key: *mut gchar,
    pub value: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusPropertyInfo = _GDBusPropertyInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusPropertyInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub flags: GDBusPropertyInfoFlags,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusSignalInfo = _GDBusSignalInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusSignalInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusArgInfo = _GDBusArgInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusArgInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusMethodInfo = _GDBusMethodInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMethodInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub in_args: *mut *mut GDBusArgInfo,
    pub out_args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusNodeInfo {
    pub ref_count: gint,
    pub path: *mut gchar,
    pub interfaces: *mut *mut GDBusInterfaceInfo,
    pub nodes: *mut *mut GDBusNodeInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusNodeInfo = _GDBusNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDebugControllerInterface {
    pub g_iface: GTypeInterface,
}
pub type GDebugControllerInterface = _GDebugControllerInterface;
pub type GDebugControllerDBus = _GDebugControllerDBus;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDebugControllerDBus {
    pub parent_instance: GObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GDebugControllerDBusPrivate {
    pub parent_instance: GObject,
    pub cancellable: *mut GCancellable,
    pub connection: *mut GDBusConnection,
    pub object_id: guint,
    pub pending_authorize_tasks: *mut GPtrArray,
    pub debug_enabled: gboolean,
}
pub type GDebugControllerDBusClass = _GDebugControllerDBusClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDebugControllerDBusClass {
    pub parent_class: GObjectClass,
    pub authorize: Option<
        unsafe extern "C" fn(*mut GDebugControllerDBus, *mut GDBusMethodInvocation) -> gboolean,
    >,
    pub padding: [gpointer; 12],
}
pub const SIGNAL_AUTHORIZE: C2RustUnnamed_4 = 0;
pub const PROP_DEBUG_ENABLED: GDebugControllerDBusProperty = 2;
pub const PROP_CONNECTION: GDebugControllerDBusProperty = 1;
pub type GDebugControllerDBusProperty = ::core::ffi::c_uint;
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
#[inline]
unsafe extern "C" fn safe_c2rust_G_DEBUG_CONTROLLER_DBUS_GET_CLASS(
    mut ptr: gpointer,
) -> *mut GDebugControllerDBusClass {
    return (*(ptr as *mut GTypeInstance)).g_class as *mut GDebugControllerDBusClass;
}
#[inline]
unsafe extern "C" fn safe_c2rust_G_DEBUG_CONTROLLER_DBUS(
    mut ptr: gpointer,
) -> *mut GDebugControllerDBus {
    return ptr as *mut GDebugControllerDBus;
}
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
static mut safe_c2rust_org_gtk_Debugging_xml: [gchar; 207] = unsafe {
    ::core::mem::transmute::<
        [u8; 207],
        [gchar; 207],
    >(
        *b"<node><interface name='org.gtk.Debugging'><property name='DebugEnabled' type='b' access='read'/><method name='SetDebugEnabled'><arg type='b' name='debug-enabled' direction='in'/></method></interface></node>\0",
    )
};
static mut safe_c2rust_org_gtk_Debugging: *mut GDBusInterfaceInfo =
    ::core::ptr::null::<GDBusInterfaceInfo>() as *mut GDBusInterfaceInfo;
static mut safe_c2rust_props: [*mut GParamSpec; 2] = [
    ::core::ptr::null::<GParamSpec>() as *mut GParamSpec,
    ::core::ptr::null::<GParamSpec>() as *mut GParamSpec,
];
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
static mut safe_c2rust_g_debug_controller_dbus_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDebugControllerDBus\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDebugControllerDBusClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_debug_controller_dbus_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDebugControllerDBus>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDebugControllerDBus) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_debug_controller_dbus_init
                    as unsafe extern "C" fn(*mut GDebugControllerDBus) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDebugControllerDBus_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDebugControllerDBusPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_debug_controller_dbus_initable_iface_init
                as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDebugControllerInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_debug_controller_dbus_iface_init
                as unsafe extern "C" fn(*mut GDebugControllerInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_debug_controller_get_type(),
        &raw const g_implement_interface_info_0,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gio-debug-controller\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"dbus\0" as *const u8 as *const ::core::ffi::c_char,
        30 as gint,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_debug_controller_dbus_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_debug_controller_dbus_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDebugControllerDBus_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDebugControllerDBus_private_offset,
        );
    }
    safe_c2rust_g_debug_controller_dbus_class_init(klass as *mut GDebugControllerDBusClass);
}
static mut safe_c2rust_GDebugControllerDBus_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_get_instance_private(
    mut self_0: *mut GDebugControllerDBus,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDebugControllerDBus_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_init(
    mut self_0: *mut GDebugControllerDBus,
) {
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
            as *mut GDebugControllerDBusPrivate;
    (*priv_0).cancellable = g_cancellable_new();
}
unsafe extern "C" fn safe_c2rust_set_debug_enabled(
    mut self_0: *mut GDebugControllerDBus,
    mut debug_enabled: gboolean,
) {
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
            as *mut GDebugControllerDBusPrivate;
    if g_cancellable_is_cancelled((*priv_0).cancellable) != 0 {
        return;
    }
    if debug_enabled != (*priv_0).debug_enabled {
        let mut builder: GVariantBuilder = _GVariantBuilder {
            u: C2RustUnnamed {
                s: C2RustUnnamed_0 {
                    partial_magic: 0,
                    type_0: ::core::ptr::null::<GVariantType>(),
                    y: [0; 14],
                },
            },
        };
        (*priv_0).debug_enabled = debug_enabled;
        g_log_set_debug_enabled(debug_enabled);
        g_object_notify(
            self_0 as *mut ::core::ffi::c_void as *mut GObject,
            b"debug-enabled\0" as *const u8 as *const gchar,
        );
        g_variant_builder_init(
            &raw mut builder,
            g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
        );
        g_variant_builder_add(
            &raw mut builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"DebugEnabled\0" as *const u8 as *const ::core::ffi::c_char,
            g_variant_new_boolean((*priv_0).debug_enabled),
        );
        g_dbus_connection_emit_signal(
            (*priv_0).connection,
            ::core::ptr::null::<gchar>(),
            b"/org/gtk/Debugging\0" as *const u8 as *const gchar,
            b"org.freedesktop.DBus.Properties\0" as *const u8 as *const gchar,
            b"PropertiesChanged\0" as *const u8 as *const gchar,
            g_variant_new(
                b"(sa{sv}as)\0" as *const u8 as *const gchar,
                b"org.gtk.Debugging\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut builder,
                NULL_0,
            ),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_DEBUG,
            b"Debug output %s\0" as *const u8 as *const gchar,
            if debug_enabled != 0 {
                b"enabled\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"disabled\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
}
unsafe extern "C" fn safe_c2rust_dbus_get_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> *mut GVariant {
    let mut self_0: *mut GDebugControllerDBus = user_data as *mut GDebugControllerDBus;
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
            as *mut GDebugControllerDBusPrivate;
    if strcmp(
        property_name as *const ::core::ffi::c_char,
        b"DebugEnabled\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return g_variant_new_boolean((*priv_0).debug_enabled);
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdebugcontrollerdbus.c\0"
            as *const u8 as *const ::core::ffi::c_char,
        273 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn safe_c2rust_weak_ref_new(mut obj: *mut GObject) -> *mut GWeakRef {
    let mut weak_ref: *mut GWeakRef = ({
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
    g_weak_ref_init(weak_ref, obj as gpointer);
    return safe_c2rust_g_steal_pointer(&raw mut weak_ref as gpointer) as *mut GWeakRef;
}
unsafe extern "C" fn safe_c2rust_weak_ref_free(mut weak_ref: *mut GWeakRef) {
    g_weak_ref_clear(weak_ref);
    g_free(weak_ref as gpointer);
}
unsafe extern "C" fn safe_c2rust_garbage_collect_weak_refs(mut self_0: *mut GDebugControllerDBus) {
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
            as *mut GDebugControllerDBusPrivate;
    let mut i: guint = 0;
    if (*priv_0).pending_authorize_tasks.is_null() {
        return;
    }
    i = (*(*priv_0).pending_authorize_tasks).len;
    while i > 0 as guint {
        let mut weak_ref: *mut GWeakRef = *(*(*priv_0).pending_authorize_tasks)
            .pdata
            .offset(i.wrapping_sub(1 as guint) as isize)
            as *mut GWeakRef;
        let mut obj: *mut GObject = g_weak_ref_get(weak_ref) as *mut GObject;
        if obj.is_null() {
            g_ptr_array_remove_index_fast(
                (*priv_0).pending_authorize_tasks,
                i.wrapping_sub(1 as guint),
            );
        } else {
            g_object_unref(obj as gpointer);
        }
        i = i.wrapping_sub(1);
    }
    if (*(*priv_0).pending_authorize_tasks).len == 0 as guint {
        let mut _pp: *mut *mut GPtrArray = &raw mut (*priv_0).pending_authorize_tasks;
        let mut _ptr: *mut GPtrArray = *_pp;
        *_pp = ::core::ptr::null_mut::<GPtrArray>();
        if !_ptr.is_null() {
            g_ptr_array_unref(_ptr as *mut GPtrArray);
        }
    }
}
unsafe extern "C" fn safe_c2rust_authorize_task_cb(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut self_0: *mut GDebugControllerDBus = safe_c2rust_G_DEBUG_CONTROLLER_DBUS(source_object);
    let mut invocation: *mut GDBusMethodInvocation = task_data as *mut GDBusMethodInvocation;
    let mut authorized: gboolean = TRUE;
    g_signal_emit(
        self_0 as gpointer,
        safe_c2rust_signals[SIGNAL_AUTHORIZE as ::core::ffi::c_int as usize],
        0 as GQuark,
        invocation,
        &raw mut authorized,
    );
    g_task_return_boolean(task, authorized);
}
unsafe extern "C" fn safe_c2rust_authorize_cb(
    mut object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut self_0: *mut GDebugControllerDBus =
        safe_c2rust_G_DEBUG_CONTROLLER_DBUS(object as gpointer);
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        ::core::ptr::null_mut::<GDebugControllerDBusPrivate>();
    let mut task: *mut GTask = result as *mut ::core::ffi::c_void as *mut GTask;
    let mut invocation: *mut GDBusMethodInvocation =
        g_task_get_task_data(task) as *mut GDBusMethodInvocation;
    let mut parameters: *mut GVariant = g_dbus_method_invocation_get_parameters(invocation);
    let mut enabled: gboolean = FALSE;
    let mut authorized: gboolean = 0;
    priv_0 = safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
        as *mut GDebugControllerDBusPrivate;
    authorized = g_task_propagate_boolean(task, ::core::ptr::null_mut::<*mut GError>());
    if authorized == 0 {
        let mut local_error: *mut GError = g_error_new(
            g_dbus_error_quark(),
            G_DBUS_ERROR_ACCESS_DENIED as ::core::ffi::c_int as gint,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Not authorized to change debug settings\0" as *const u8
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        g_dbus_method_invocation_take_error(
            invocation,
            safe_c2rust_g_steal_pointer(&raw mut local_error as gpointer) as *mut GError,
        );
    } else {
        g_variant_get(
            parameters,
            b"(b)\0" as *const u8 as *const gchar,
            &raw mut enabled,
        );
        safe_c2rust_set_debug_enabled(self_0, enabled);
        g_dbus_method_invocation_return_value(invocation, ::core::ptr::null_mut::<GVariant>());
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !(*priv_0).pending_authorize_tasks.is_null()
            && (*(*priv_0).pending_authorize_tasks).len > 0 as guint
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdebugcontrollerdbus.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            375 as ::core::ffi::c_int,
            G_STRFUNC,
            b"priv->pending_authorize_tasks != NULL && priv->pending_authorize_tasks->len > 0\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_dbus_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut self_0: *mut GDebugControllerDBus = user_data as *mut GDebugControllerDBus;
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
            as *mut GDebugControllerDBusPrivate;
    let mut klass: *mut GDebugControllerDBusClass =
        safe_c2rust_G_DEBUG_CONTROLLER_DBUS_GET_CLASS(self_0 as gpointer);
    if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"SetDebugEnabled\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
        task = g_task_new(
            self_0 as gpointer,
            (*priv_0).cancellable,
            Some(
                safe_c2rust_authorize_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            NULL_0,
        );
        let mut _task: *mut GTask = task;
        g_task_set_source_tag(
            _task,
            ::core::mem::transmute::<
                Option<
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
                >,
                gpointer,
            >(Some(
                safe_c2rust_dbus_method_call
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
            )),
        );
        if g_task_get_name(_task).is_null() {
            g_task_set_static_name(_task, b"dbus_method_call\0" as *const u8 as *const gchar);
        }
        g_task_set_task_data(
            task,
            g_object_ref(invocation as gpointer) as *mut GDBusMethodInvocation as gpointer,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
                Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
            ),
        );
        if (*priv_0).pending_authorize_tasks.is_null() {
            (*priv_0).pending_authorize_tasks =
                g_ptr_array_new_with_free_func(::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GWeakRef) -> ()>,
                    GDestroyNotify,
                >(Some(
                    safe_c2rust_weak_ref_free as unsafe extern "C" fn(*mut GWeakRef) -> (),
                )));
        }
        g_ptr_array_add(
            (*priv_0).pending_authorize_tasks,
            safe_c2rust_weak_ref_new(task as *mut ::core::ffi::c_void as *mut GObject) as gpointer,
        );
        safe_c2rust_garbage_collect_weak_refs(self_0);
        if g_signal_has_handler_pending(
            self_0 as gpointer,
            safe_c2rust_signals[SIGNAL_AUTHORIZE as ::core::ffi::c_int as usize],
            0 as GQuark,
            FALSE,
        ) != 0
            || (*klass).authorize
                != Some(
                    safe_c2rust_g_debug_controller_dbus_authorize_default
                        as unsafe extern "C" fn(
                            *mut GDebugControllerDBus,
                            *mut GDBusMethodInvocation,
                        ) -> gboolean,
                )
        {
            g_task_run_in_thread(
                task,
                Some(
                    safe_c2rust_authorize_task_cb
                        as unsafe extern "C" fn(
                            *mut GTask,
                            gpointer,
                            gpointer,
                            *mut GCancellable,
                        ) -> (),
                ),
            );
        } else {
            g_task_return_boolean(task, FALSE);
        }
        let mut _pp: *mut *mut GTask = &raw mut task;
        let mut _ptr: *mut GTask = *_pp;
        *_pp = ::core::ptr::null_mut::<GTask>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdebugcontrollerdbus.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            431 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    };
}
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut self_0: *mut GDebugControllerDBus =
        safe_c2rust_G_DEBUG_CONTROLLER_DBUS(initable as gpointer);
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
            as *mut GDebugControllerDBusPrivate;
    static mut safe_c2rust_vtable: GDBusInterfaceVTable = unsafe {
        _GDBusInterfaceVTable {
            method_call: Some(
                safe_c2rust_dbus_method_call
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
            ),
            get_property: Some(
                safe_c2rust_dbus_get_property
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut *mut GError,
                        gpointer,
                    ) -> *mut GVariant,
            ),
            set_property: None,
            padding: [
                ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
                ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
            ],
        }
    };
    if safe_c2rust_org_gtk_Debugging.is_null() {
        let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut info: *mut GDBusNodeInfo = ::core::ptr::null_mut::<GDBusNodeInfo>();
        info = g_dbus_node_info_new_for_xml(
            &raw const safe_c2rust_org_gtk_Debugging_xml as *const gchar,
            &raw mut local_error,
        );
        if ({
            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
            if info.is_null() {
                _g_boolean_var_11 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_11 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_11
        }) as ::core::ffi::c_long
            != 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"%s\0" as *const u8 as *const gchar,
                (*local_error).message,
            );
            loop {}
        }
        safe_c2rust_org_gtk_Debugging = g_dbus_node_info_lookup_interface(
            info,
            b"org.gtk.Debugging\0" as *const u8 as *const gchar,
        );
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if !safe_c2rust_org_gtk_Debugging.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdebugcontrollerdbus.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                457 as ::core::ffi::c_int,
                G_STRFUNC,
                b"org_gtk_Debugging != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_dbus_interface_info_ref(safe_c2rust_org_gtk_Debugging);
        g_dbus_node_info_unref(info);
    }
    (*priv_0).object_id = g_dbus_connection_register_object(
        (*priv_0).connection,
        b"/org/gtk/Debugging\0" as *const u8 as *const gchar,
        safe_c2rust_org_gtk_Debugging,
        &raw const safe_c2rust_vtable,
        self_0 as gpointer,
        None,
        error,
    );
    if (*priv_0).object_id == 0 as guint {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GDebugControllerDBus =
        safe_c2rust_G_DEBUG_CONTROLLER_DBUS(object as gpointer);
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
            as *mut GDebugControllerDBusPrivate;
    match prop_id as GDebugControllerDBusProperty as ::core::ffi::c_uint {
        1 => {
            g_value_set_object(value, (*priv_0).connection as gpointer);
        }
        2 => {
            g_value_set_boolean(value, (*priv_0).debug_enabled);
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdebugcontrollerdbus.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                490 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut self_0: *mut GDebugControllerDBus =
        safe_c2rust_G_DEBUG_CONTROLLER_DBUS(object as gpointer);
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
            as *mut GDebugControllerDBusPrivate;
    match prop_id as GDebugControllerDBusProperty as ::core::ffi::c_uint {
        1 => {
            if ({
                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                if (*priv_0).connection.is_null() {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdebugcontrollerdbus.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    508 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"priv->connection == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*priv_0).connection = g_value_dup_object(value) as *mut GDBusConnection;
        }
        2 => {
            safe_c2rust_set_debug_enabled(self_0, g_value_get_boolean(value));
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdebugcontrollerdbus.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                515 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_dispose(mut object: *mut GObject) {
    let mut self_0: *mut GDebugControllerDBus =
        safe_c2rust_G_DEBUG_CONTROLLER_DBUS(object as gpointer);
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
            as *mut GDebugControllerDBusPrivate;
    safe_c2rust_g_debug_controller_dbus_stop(self_0);
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*priv_0).pending_authorize_tasks.is_null() {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdebugcontrollerdbus.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            527 as ::core::ffi::c_int,
            G_STRFUNC,
            b"priv->pending_authorize_tasks == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _pp: *mut *mut GDBusConnection = &raw mut (*priv_0).connection;
    let mut _ptr: *mut GDBusConnection = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusConnection>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    let mut _pp_0: *mut *mut GCancellable = &raw mut (*priv_0).cancellable;
    let mut _ptr_0: *mut GCancellable = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GCancellable>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
    (*(safe_c2rust_g_debug_controller_dbus_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_authorize_default(
    mut self_0: *mut GDebugControllerDBus,
    mut invocation: *mut GDBusMethodInvocation,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_class_init(
    mut klass: *mut GDebugControllerDBusClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_debug_controller_dbus_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_debug_controller_dbus_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).dispose = Some(
        safe_c2rust_g_debug_controller_dbus_dispose as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*klass).authorize = Some(
        safe_c2rust_g_debug_controller_dbus_authorize_default
            as unsafe extern "C" fn(
                *mut GDebugControllerDBus,
                *mut GDBusMethodInvocation,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GDebugControllerDBus, *mut GDBusMethodInvocation) -> gboolean,
        >;
    safe_c2rust_props[PROP_CONNECTION as ::core::ffi::c_int as usize] = g_param_spec_object(
        b"connection\0" as *const u8 as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
        g_dbus_connection_get_type(),
        (G_PARAM_READWRITE as ::core::ffi::c_int
            | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
            | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
    );
    g_object_class_install_properties(
        gobject_class,
        (::core::mem::size_of::<[*mut GParamSpec; 2]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut GParamSpec>() as usize) as guint,
        &raw mut safe_c2rust_props as *mut *mut GParamSpec,
    );
    g_object_class_override_property(
        gobject_class,
        PROP_DEBUG_ENABLED as ::core::ffi::c_int as guint,
        b"debug-enabled\0" as *const u8 as *const gchar,
    );
    safe_c2rust_signals[SIGNAL_AUTHORIZE as ::core::ffi::c_int as usize] = g_signal_new(
        b"authorize\0" as *const u8 as *const gchar,
        safe_c2rust_g_debug_controller_dbus_get_type(),
        G_SIGNAL_RUN_LAST,
        136 as ::core::ffi::c_ulong as glong as guint,
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
        safe_c2rust_signals[SIGNAL_AUTHORIZE as ::core::ffi::c_int as usize],
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
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_iface_init(
    mut iface: *mut GDebugControllerInterface,
) {
}
unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_initable_iface_init(
    mut iface: *mut GInitableIface,
) {
    (*iface).init = Some(
        safe_c2rust_g_debug_controller_dbus_initable_init
            as unsafe extern "C" fn(
                *mut GInitable,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_new(
    mut connection: *mut GDBusConnection,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDebugControllerDBus {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDebugControllerDBus>();
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
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
            b"cancellable == NULL || G_IS_CANCELLABLE (cancellable)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDebugControllerDBus>();
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDebugControllerDBus>();
    }
    return g_initable_new(
        safe_c2rust_g_debug_controller_dbus_get_type(),
        cancellable,
        error,
        b"connection\0" as *const u8 as *const gchar,
        connection,
        NULL_0,
    ) as *mut GDebugControllerDBus;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_debug_controller_dbus_stop(
    mut self_0: *mut GDebugControllerDBus,
) {
    let mut priv_0: *mut GDebugControllerDBusPrivate =
        safe_c2rust_g_debug_controller_dbus_get_instance_private(self_0)
            as *mut GDebugControllerDBusPrivate;
    g_cancellable_cancel((*priv_0).cancellable);
    if (*priv_0).object_id != 0 as guint {
        g_dbus_connection_unregister_object((*priv_0).connection, (*priv_0).object_id);
        (*priv_0).object_id = 0 as guint;
    }
    while !(*priv_0).pending_authorize_tasks.is_null() {
        safe_c2rust_garbage_collect_weak_refs(self_0);
        g_thread_yield();
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
