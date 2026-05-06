use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GMainLoop;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GAsyncInitable;
    pub type _GCancellablePrivate;
    pub type _GInitable;
    pub type _GTask;
    pub type _GUnixFDListPrivate;
    pub type _GDBusConnection;
    pub type _GDBusObject;
    pub type _GDBusInterface;
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_ptr_array_sort_values(array: *mut GPtrArray, compare_func: GCompareFunc);
    fn g_quark_try_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_new(domain: GQuark, code: gint, format: *const gchar, ...) -> *mut GError;
    fn g_error_free(error: *mut GError);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_remove_all(hash_table: *mut GHashTable);
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_main_context_new() -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_push_thread_default(context: *mut GMainContext);
    fn g_main_context_pop_thread_default(context: *mut GMainContext);
    fn g_main_loop_new(context: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
    fn g_main_loop_run(loop_0: *mut GMainLoop);
    fn g_main_loop_quit(loop_0: *mut GMainLoop);
    fn g_main_loop_unref(loop_0: *mut GMainLoop);
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_type_free(type_0: *mut GVariantType);
    fn g_variant_type_dup_string(type_0: *const GVariantType) -> *mut gchar;
    fn g_variant_type_equal(type1: gconstpointer, type2: gconstpointer) -> gboolean;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type(value: *mut GVariant) -> *const GVariantType;
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_is_object_path(string: *const gchar) -> gboolean;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_free(iter: *mut GVariantIter);
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
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
    fn g_strv_get_type() -> GType;
    fn g_value_set_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_get_boxed(value: *const GValue) -> gpointer;
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
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_weak_ref_init(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_weak_ref_clear(weak_ref: *mut GWeakRef);
    fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> gpointer;
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_int(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: gint,
        maximum: gint,
        default_value: gint,
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
    fn g_param_spec_flags(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags_type: GType,
        default_value: guint,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_boxed(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        boxed_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_get_int(value: *const GValue) -> gint;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_value_take_string(value: *mut GValue, v_string: *mut gchar);
    fn g_dbus_is_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_unique_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_member_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_interface_name(string: *const gchar) -> gboolean;
    fn g_dbus_interface_info_lookup_method(
        info: *mut GDBusInterfaceInfo,
        name: *const gchar,
    ) -> *mut GDBusMethodInfo;
    fn g_dbus_interface_info_lookup_signal(
        info: *mut GDBusInterfaceInfo,
        name: *const gchar,
    ) -> *mut GDBusSignalInfo;
    fn g_dbus_interface_info_lookup_property(
        info: *mut GDBusInterfaceInfo,
        name: *const gchar,
    ) -> *mut GDBusPropertyInfo;
    fn g_dbus_interface_info_cache_build(info: *mut GDBusInterfaceInfo);
    fn g_dbus_interface_info_cache_release(info: *mut GDBusInterfaceInfo);
    fn g_dbus_interface_info_ref(info: *mut GDBusInterfaceInfo) -> *mut GDBusInterfaceInfo;
    fn g_dbus_interface_info_unref(info: *mut GDBusInterfaceInfo);
    fn g_dbus_interface_info_get_type() -> GType;
    fn g_bus_type_get_type() -> GType;
    fn g_dbus_proxy_flags_get_type() -> GType;
    fn g_dbus_connection_get_type() -> GType;
    fn g_bus_get(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_bus_get_finish(res: *mut GAsyncResult, error: *mut *mut GError) -> *mut GDBusConnection;
    fn g_bus_get_sync(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GDBusConnection;
    fn g_dbus_connection_get_unique_name(connection: *mut GDBusConnection) -> *const gchar;
    fn g_dbus_connection_get_flags(connection: *mut GDBusConnection) -> GDBusConnectionFlags;
    fn g_dbus_connection_call(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_connection_call_finish(
        connection: *mut GDBusConnection,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_connection_call_with_unix_fd_list(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        fd_list: *mut GUnixFDList,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_dbus_connection_call_with_unix_fd_list_finish(
        connection: *mut GDBusConnection,
        out_fd_list: *mut *mut GUnixFDList,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_connection_call_with_unix_fd_list_sync(
        connection: *mut GDBusConnection,
        bus_name: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        parameters: *mut GVariant,
        reply_type: *const GVariantType,
        flags: GDBusCallFlags,
        timeout_msec: gint,
        fd_list: *mut GUnixFDList,
        out_fd_list: *mut *mut GUnixFDList,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GVariant;
    fn g_dbus_connection_signal_subscribe(
        connection: *mut GDBusConnection,
        sender: *const gchar,
        interface_name: *const gchar,
        member: *const gchar,
        object_path: *const gchar,
        arg0: *const gchar,
        flags: GDBusSignalFlags,
        callback: GDBusSignalCallback,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
    ) -> guint;
    fn g_dbus_connection_signal_unsubscribe(
        connection: *mut GDBusConnection,
        subscription_id: guint,
    );
    fn g_dbus_error_quark() -> GQuark;
    fn g_dbus_error_get_remote_error(error: *const GError) -> *mut gchar;
    fn g_dbus_error_strip_remote_error(error: *mut GError) -> gboolean;
    fn _g_dbus_initialize();
    fn _g_dbus_debug_proxy() -> gboolean;
    fn _g_dbus_compute_complete_signature(args: *mut *mut GDBusArgInfo) -> *mut GVariantType;
    fn g_initable_get_type() -> GType;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_async_initable_get_type() -> GType;
    fn g_async_initable_new_async(
        object_type: GType,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
        first_property_name: *const gchar,
        ...
    );
    fn g_async_initable_new_finish(
        initable: *mut GAsyncInitable,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> *mut GObject;
    fn g_io_error_quark() -> GQuark;
    fn g_task_new(
        source_object: gpointer,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        callback_data: gpointer,
    ) -> *mut GTask;
    fn g_task_set_priority(task: *mut GTask, priority: gint);
    fn g_task_set_source_tag(task: *mut GTask, source_tag: gpointer);
    fn g_task_set_name(task: *mut GTask, name: *const gchar);
    fn g_task_set_static_name(task: *mut GTask, name: *const gchar);
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_get_priority(task: *mut GTask) -> gint;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
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
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn g_task_had_error(task: *mut GTask) -> gboolean;
    fn g_cancellable_new() -> *mut GCancellable;
    fn g_cancellable_cancel(cancellable: *mut GCancellable);
    fn g_dbus_interface_get_type() -> GType;
    fn g_async_result_get_source_object(res: *mut GAsyncResult) -> *mut GObject;
    fn g_unix_fd_list_get_type() -> GType;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn _g_cclosure_marshal_VOID__STRING_STRING_VARIANT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__STRING_STRING_VARIANTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_cclosure_marshal_VOID__VARIANT_BOXED(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__VARIANT_BOXEDv(
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
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
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
pub type GData = _GData;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHashTableIter {
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: gpointer,
    pub dummy4: ::core::ffi::c_int,
    pub dummy5: gboolean,
    pub dummy6: gpointer,
}
pub type GHashTableIter = _GHashTableIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
pub type GMainLoop = _GMainLoop;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantIter {
    pub x: [guintptr; 16],
}
pub type GVariantIter = _GVariantIter;
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
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_3 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_3 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_3 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_3 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_3 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_3 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_3 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_3 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_3 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_3 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_3 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_3 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_3 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_3 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_3 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_3 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_3 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_3 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_3 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_3 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_3 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_3 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_3 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_3 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_3 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_3 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_3 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_3 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_3 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_3 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_3 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_3 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_3 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_3 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_3 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_3 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_3 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_3 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_3 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_3 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_3 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_3 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_3 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_3 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_3 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_3 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_3 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_3 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_3 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_3 = 0;
pub type GBusType = ::core::ffi::c_int;
pub const G_BUS_TYPE_SESSION: GBusType = 2;
pub const G_BUS_TYPE_SYSTEM: GBusType = 1;
pub const G_BUS_TYPE_NONE: GBusType = 0;
pub const G_BUS_TYPE_STARTER: GBusType = -1;
pub type GDBusProxyFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROXY_FLAGS_NO_MATCH_RULE: GDBusProxyFlags = 32;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START_AT_CONSTRUCTION: GDBusProxyFlags = 16;
pub const G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES: GDBusProxyFlags = 8;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START: GDBusProxyFlags = 4;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS: GDBusProxyFlags = 2;
pub const G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES: GDBusProxyFlags = 1;
pub const G_DBUS_PROXY_FLAGS_NONE: GDBusProxyFlags = 0;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const G_DBUS_ERROR_PROPERTY_READ_ONLY: C2RustUnnamed_4 = 44;
pub const G_DBUS_ERROR_UNKNOWN_PROPERTY: C2RustUnnamed_4 = 43;
pub const G_DBUS_ERROR_UNKNOWN_INTERFACE: C2RustUnnamed_4 = 42;
pub const G_DBUS_ERROR_UNKNOWN_OBJECT: C2RustUnnamed_4 = 41;
pub const G_DBUS_ERROR_OBJECT_PATH_IN_USE: C2RustUnnamed_4 = 40;
pub const G_DBUS_ERROR_ADT_AUDIT_DATA_UNKNOWN: C2RustUnnamed_4 = 39;
pub const G_DBUS_ERROR_SELINUX_SECURITY_CONTEXT_UNKNOWN: C2RustUnnamed_4 = 38;
pub const G_DBUS_ERROR_INVALID_FILE_CONTENT: C2RustUnnamed_4 = 37;
pub const G_DBUS_ERROR_INVALID_SIGNATURE: C2RustUnnamed_4 = 36;
pub const G_DBUS_ERROR_UNIX_PROCESS_ID_UNKNOWN: C2RustUnnamed_4 = 35;
pub const G_DBUS_ERROR_SPAWN_NO_MEMORY: C2RustUnnamed_4 = 34;
pub const G_DBUS_ERROR_SPAWN_FILE_INVALID: C2RustUnnamed_4 = 33;
pub const G_DBUS_ERROR_SPAWN_PERMISSIONS_INVALID: C2RustUnnamed_4 = 32;
pub const G_DBUS_ERROR_SPAWN_SERVICE_NOT_FOUND: C2RustUnnamed_4 = 31;
pub const G_DBUS_ERROR_SPAWN_SERVICE_INVALID: C2RustUnnamed_4 = 30;
pub const G_DBUS_ERROR_SPAWN_CONFIG_INVALID: C2RustUnnamed_4 = 29;
pub const G_DBUS_ERROR_SPAWN_SETUP_FAILED: C2RustUnnamed_4 = 28;
pub const G_DBUS_ERROR_SPAWN_FAILED: C2RustUnnamed_4 = 27;
pub const G_DBUS_ERROR_SPAWN_CHILD_SIGNALED: C2RustUnnamed_4 = 26;
pub const G_DBUS_ERROR_SPAWN_CHILD_EXITED: C2RustUnnamed_4 = 25;
pub const G_DBUS_ERROR_SPAWN_FORK_FAILED: C2RustUnnamed_4 = 24;
pub const G_DBUS_ERROR_SPAWN_EXEC_FAILED: C2RustUnnamed_4 = 23;
pub const G_DBUS_ERROR_MATCH_RULE_INVALID: C2RustUnnamed_4 = 22;
pub const G_DBUS_ERROR_MATCH_RULE_NOT_FOUND: C2RustUnnamed_4 = 21;
pub const G_DBUS_ERROR_TIMED_OUT: C2RustUnnamed_4 = 20;
pub const G_DBUS_ERROR_UNKNOWN_METHOD: C2RustUnnamed_4 = 19;
pub const G_DBUS_ERROR_FILE_EXISTS: C2RustUnnamed_4 = 18;
pub const G_DBUS_ERROR_FILE_NOT_FOUND: C2RustUnnamed_4 = 17;
pub const G_DBUS_ERROR_INVALID_ARGS: C2RustUnnamed_4 = 16;
pub const G_DBUS_ERROR_DISCONNECTED: C2RustUnnamed_4 = 15;
pub const G_DBUS_ERROR_ADDRESS_IN_USE: C2RustUnnamed_4 = 14;
pub const G_DBUS_ERROR_NO_NETWORK: C2RustUnnamed_4 = 13;
pub const G_DBUS_ERROR_TIMEOUT: C2RustUnnamed_4 = 12;
pub const G_DBUS_ERROR_NO_SERVER: C2RustUnnamed_4 = 11;
pub const G_DBUS_ERROR_AUTH_FAILED: C2RustUnnamed_4 = 10;
pub const G_DBUS_ERROR_ACCESS_DENIED: C2RustUnnamed_4 = 9;
pub const G_DBUS_ERROR_LIMITS_EXCEEDED: C2RustUnnamed_4 = 8;
pub const G_DBUS_ERROR_NOT_SUPPORTED: C2RustUnnamed_4 = 7;
pub const G_DBUS_ERROR_BAD_ADDRESS: C2RustUnnamed_4 = 6;
pub const G_DBUS_ERROR_IO_ERROR: C2RustUnnamed_4 = 5;
pub const G_DBUS_ERROR_NO_REPLY: C2RustUnnamed_4 = 4;
pub const G_DBUS_ERROR_NAME_HAS_NO_OWNER: C2RustUnnamed_4 = 3;
pub const G_DBUS_ERROR_SERVICE_UNKNOWN: C2RustUnnamed_4 = 2;
pub const G_DBUS_ERROR_NO_MEMORY: C2RustUnnamed_4 = 1;
pub const G_DBUS_ERROR_FAILED: C2RustUnnamed_4 = 0;
pub type GDBusConnectionFlags = ::core::ffi::c_uint;
pub const G_DBUS_CONNECTION_FLAGS_CROSS_NAMESPACE: GDBusConnectionFlags = 64;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER: GDBusConnectionFlags = 32;
pub const G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING: GDBusConnectionFlags = 16;
pub const G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION: GDBusConnectionFlags = 8;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS: GDBusConnectionFlags = 4;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER: GDBusConnectionFlags = 2;
pub const G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT: GDBusConnectionFlags = 1;
pub const G_DBUS_CONNECTION_FLAGS_NONE: GDBusConnectionFlags = 0;
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GDBusSignalFlags = ::core::ffi::c_uint;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH: GDBusSignalFlags = 4;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE: GDBusSignalFlags = 2;
pub const G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE: GDBusSignalFlags = 1;
pub const G_DBUS_SIGNAL_FLAGS_NONE: GDBusSignalFlags = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GAsyncInitable = _GAsyncInitable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
pub type GInitable = _GInitable;
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusProxy {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusProxyPrivate,
}
pub type GDBusProxyPrivate = _GDBusProxyPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusProxyPrivate {
    pub bus_type: GBusType,
    pub flags: GDBusProxyFlags,
    pub connection: *mut GDBusConnection,
    pub name: *mut gchar,
    pub name_owner: *mut gchar,
    pub object_path: *mut gchar,
    pub interface_name: *mut gchar,
    pub timeout_msec: gint,
    pub name_owner_changed_subscription_id: guint,
    pub get_all_cancellable: *mut GCancellable,
    pub properties: *mut GHashTable,
    pub expected_interface: *mut GDBusInterfaceInfo,
    pub properties_changed_subscription_id: guint,
    pub signals_subscription_id: guint,
    pub initialized: gboolean,
    pub object: *mut GDBusObject,
}
pub type GDBusObject = _GDBusObject;
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
pub type GDBusProxy = _GDBusProxy;
pub type GDBusInterface = _GDBusInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusProxyClass {
    pub parent_class: GObjectClass,
    pub g_properties_changed:
        Option<unsafe extern "C" fn(*mut GDBusProxy, *mut GVariant, *const *const gchar) -> ()>,
    pub g_signal: Option<
        unsafe extern "C" fn(*mut GDBusProxy, *const gchar, *const gchar, *mut GVariant) -> (),
    >,
    pub padding: [gpointer; 32],
}
pub type GDBusProxyClass = _GDBusProxyClass;
pub const SIGNAL_SIGNAL: C2RustUnnamed_6 = 1;
pub const PROPERTIES_CHANGED_SIGNAL: C2RustUnnamed_6 = 0;
pub const PROP_G_DEFAULT_TIMEOUT: C2RustUnnamed_5 = 8;
pub const PROP_G_INTERFACE_NAME: C2RustUnnamed_5 = 7;
pub const PROP_G_OBJECT_PATH: C2RustUnnamed_5 = 6;
pub const PROP_G_NAME_OWNER: C2RustUnnamed_5 = 4;
pub const PROP_G_NAME: C2RustUnnamed_5 = 3;
pub const PROP_G_FLAGS: C2RustUnnamed_5 = 5;
pub const PROP_G_BUS_TYPE: C2RustUnnamed_5 = 2;
pub const PROP_G_CONNECTION: C2RustUnnamed_5 = 1;
pub const PROP_G_INTERFACE_INFO: C2RustUnnamed_5 = 9;
pub type GAsyncInitableIface = _GAsyncInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAsyncInitableIface {
    pub g_iface: GTypeInterface,
    pub init_async: Option<
        unsafe extern "C" fn(
            *mut GAsyncInitable,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub init_finish: Option<
        unsafe extern "C" fn(*mut GAsyncInitable, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadPropertiesOnNameOwnerChangedData {
    pub proxy: *mut GDBusProxy,
    pub cancellable: *mut GCancellable,
    pub name_owner: *mut gchar,
}
pub type GDBusSignalCallback = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut GVariant,
        gpointer,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InvalidatedPropGetData {
    pub proxy: *mut GDBusProxy,
    pub prop_name: *mut gchar,
}
pub type GInitableIface = _GInitableIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInitableIface {
    pub g_iface: GTypeInterface,
    pub init: Option<
        unsafe extern "C" fn(*mut GInitable, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InitableAsyncInitableData {
    pub context: *mut GMainContext,
    pub loop_0: *mut GMainLoop,
    pub res: *mut GAsyncResult,
}
pub type GDBusInterfaceIface = _GDBusInterfaceIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceIface {
    pub parent_iface: GTypeInterface,
    pub get_info: Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusInterfaceInfo>,
    pub get_object: Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject>,
    pub set_object: Option<unsafe extern "C" fn(*mut GDBusInterface, *mut GDBusObject) -> ()>,
    pub dup_object: Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReplyData {
    pub value: *mut GVariant,
    pub fd_list: *mut GUnixFDList,
}
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_6 = 2;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_VARIANT: GType = ((21 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_FLAG_RESERVED_ID_BIT: GType =
    ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType;
pub const G_SIGNAL_TYPE_STATIC_SCOPE: GType =
    ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType;
static mut safe_c2rust_g__properties_lock_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
unsafe extern "C" fn safe_c2rust_weak_ref_new(mut object: *mut GObject) -> *mut GWeakRef {
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
    g_weak_ref_init(weak_ref, object as gpointer);
    return safe_c2rust_g_steal_pointer(&raw mut weak_ref as gpointer) as *mut GWeakRef;
}
unsafe extern "C" fn safe_c2rust_weak_ref_free(mut weak_ref: *mut GWeakRef) {
    g_weak_ref_clear(weak_ref);
    g_free(weak_ref as gpointer);
}
static mut safe_c2rust_signals: [guint; 2] = [0 as ::core::ffi::c_int as guint, 0];
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_proxy_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusProxy_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GDBusProxy_private_offset);
    }
    safe_c2rust_g_dbus_proxy_class_init(klass as *mut GDBusProxyClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_proxy_get_type_once();
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
static mut safe_c2rust_g_dbus_proxy_parent_class: gpointer = NULL_0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_instance_private(
    mut self_0: *mut GDBusProxy,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GDBusProxy_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GDBusProxy_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusProxy\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusProxyClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_proxy_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusProxy>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusProxy) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_proxy_init as unsafe extern "C" fn(*mut GDBusProxy) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDBusProxy_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDBusProxyPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusInterfaceIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_dbus_interface_iface_init
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
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_initable_iface_init as unsafe extern "C" fn(*mut GInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_initable_get_type(),
        &raw const g_implement_interface_info_0,
    );
    let g_implement_interface_info_1: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GAsyncInitableIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_async_initable_iface_init
                as unsafe extern "C" fn(*mut GAsyncInitableIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_async_initable_get_type(),
        &raw const g_implement_interface_info_1,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_finalize(mut object: *mut GObject) {
    let mut proxy: *mut GDBusProxy = object as *mut ::core::ffi::c_void as *mut GDBusProxy;
    if !(({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*(*proxy).priv_0).get_all_cancellable.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusproxy.c\0" as *const u8
                as *const ::core::ffi::c_char,
            197 as ::core::ffi::c_int,
            G_STRFUNC,
            b"proxy->priv->get_all_cancellable == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*(*proxy).priv_0).name_owner_changed_subscription_id > 0 as guint {
        g_dbus_connection_signal_unsubscribe(
            (*(*proxy).priv_0).connection,
            (*(*proxy).priv_0).name_owner_changed_subscription_id,
        );
    }
    if (*(*proxy).priv_0).properties_changed_subscription_id > 0 as guint {
        g_dbus_connection_signal_unsubscribe(
            (*(*proxy).priv_0).connection,
            (*(*proxy).priv_0).properties_changed_subscription_id,
        );
    }
    if (*(*proxy).priv_0).signals_subscription_id > 0 as guint {
        g_dbus_connection_signal_unsubscribe(
            (*(*proxy).priv_0).connection,
            (*(*proxy).priv_0).signals_subscription_id,
        );
    }
    if !(*(*proxy).priv_0).connection.is_null() {
        g_object_unref((*(*proxy).priv_0).connection as gpointer);
    }
    g_free((*(*proxy).priv_0).name as gpointer);
    g_free((*(*proxy).priv_0).name_owner as gpointer);
    g_free((*(*proxy).priv_0).object_path as gpointer);
    g_free((*(*proxy).priv_0).interface_name as gpointer);
    if !(*(*proxy).priv_0).properties.is_null() {
        g_hash_table_unref((*(*proxy).priv_0).properties);
    }
    if !(*(*proxy).priv_0).expected_interface.is_null() {
        g_dbus_interface_info_cache_release((*(*proxy).priv_0).expected_interface);
        g_dbus_interface_info_unref((*(*proxy).priv_0).expected_interface);
    }
    if !(*(*proxy).priv_0).object.is_null() {
        g_object_remove_weak_pointer(
            (*(*proxy).priv_0).object as *mut ::core::ffi::c_void as *mut GObject,
            &raw mut (*(*proxy).priv_0).object as *mut gpointer,
        );
    }
    (*(safe_c2rust_g_dbus_proxy_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut proxy: *mut GDBusProxy = object as *mut ::core::ffi::c_void as *mut GDBusProxy;
    match prop_id {
        1 => {
            g_value_set_object(value, (*(*proxy).priv_0).connection as gpointer);
        }
        5 => {
            g_value_set_flags(value, (*(*proxy).priv_0).flags as guint);
        }
        3 => {
            g_value_set_string(value, (*(*proxy).priv_0).name);
        }
        4 => {
            g_value_take_string(value, safe_c2rust_g_dbus_proxy_get_name_owner(proxy));
        }
        6 => {
            g_value_set_string(value, (*(*proxy).priv_0).object_path);
        }
        7 => {
            g_value_set_string(value, (*(*proxy).priv_0).interface_name);
        }
        8 => {
            g_value_set_int(value, safe_c2rust_g_dbus_proxy_get_default_timeout(proxy));
        }
        9 => {
            g_value_set_boxed(
                value,
                safe_c2rust_g_dbus_proxy_get_interface_info(proxy) as gconstpointer,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusproxy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                275 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut proxy: *mut GDBusProxy = object as *mut ::core::ffi::c_void as *mut GDBusProxy;
    match prop_id {
        1 => {
            (*(*proxy).priv_0).connection = g_value_dup_object(value) as *mut GDBusConnection;
        }
        5 => {
            (*(*proxy).priv_0).flags = g_value_get_flags(value) as GDBusProxyFlags;
        }
        3 => {
            (*(*proxy).priv_0).name = g_value_dup_string(value);
        }
        6 => {
            (*(*proxy).priv_0).object_path = g_value_dup_string(value);
        }
        7 => {
            (*(*proxy).priv_0).interface_name = g_value_dup_string(value);
        }
        8 => {
            safe_c2rust_g_dbus_proxy_set_default_timeout(proxy, g_value_get_int(value));
        }
        9 => {
            safe_c2rust_g_dbus_proxy_set_interface_info(
                proxy,
                g_value_get_boxed(value) as *mut GDBusInterfaceInfo,
            );
        }
        2 => {
            (*(*proxy).priv_0).bus_type = g_value_get_enum(value) as GBusType;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusproxy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                323 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_class_init(mut klass: *mut GDBusProxyClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_dbus_proxy_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dbus_proxy_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dbus_proxy_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_G_INTERFACE_INFO as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"g-interface-info\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_interface_info_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_G_CONNECTION as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"g-connection\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_connection_get_type(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_G_BUS_TYPE as ::core::ffi::c_int as guint,
        g_param_spec_enum(
            b"g-bus-type\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_bus_type_get_type(),
            G_BUS_TYPE_NONE as ::core::ffi::c_int as gint,
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_G_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"g-flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_proxy_flags_get_type(),
            G_DBUS_PROXY_FLAGS_NONE as ::core::ffi::c_int as guint,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_G_NAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"g-name\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_G_NAME_OWNER as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"g-name-owner\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_G_OBJECT_PATH as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"g-object-path\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_G_INTERFACE_NAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"g-interface-name\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_G_DEFAULT_TIMEOUT as ::core::ffi::c_int as guint,
        g_param_spec_int(
            b"g-default-timeout\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            -(1 as gint),
            G_MAXINT,
            -(1 as gint),
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    safe_c2rust_signals[PROPERTIES_CHANGED_SIGNAL as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"g-properties-changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_dbus_proxy_get_type(),
        (G_SIGNAL_RUN_LAST as ::core::ffi::c_int | G_SIGNAL_MUST_COLLECT as ::core::ffi::c_int)
            as GSignalFlags,
        136 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__VARIANT_BOXED
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
        G_TYPE_VARIANT,
        g_strv_get_type() | G_SIGNAL_TYPE_STATIC_SCOPE,
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[PROPERTIES_CHANGED_SIGNAL as ::core::ffi::c_int as usize],
        (*(klass as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__VARIANT_BOXEDv
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
    safe_c2rust_signals[SIGNAL_SIGNAL as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"g-signal\0" as *const u8 as *const gchar),
        safe_c2rust_g_dbus_proxy_get_type(),
        (G_SIGNAL_RUN_LAST as ::core::ffi::c_int
            | G_SIGNAL_DETAILED as ::core::ffi::c_int
            | G_SIGNAL_MUST_COLLECT as ::core::ffi::c_int) as GSignalFlags,
        144 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__STRING_STRING_VARIANT
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
        3 as guint,
        G_TYPE_STRING,
        G_TYPE_STRING,
        G_TYPE_VARIANT,
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[SIGNAL_SIGNAL as ::core::ffi::c_int as usize],
        (*(klass as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__STRING_STRING_VARIANTv
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
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_init(mut proxy: *mut GDBusProxy) {
    (*proxy).priv_0 =
        safe_c2rust_g_dbus_proxy_get_instance_private(proxy) as *mut GDBusProxyPrivate;
    (*(*proxy).priv_0).properties = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GVariant) -> ()>, GDestroyNotify>(
            Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_cached_property_names(
    mut proxy: *mut GDBusProxy,
) -> *mut *mut gchar {
    let mut names: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut p: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut key: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    names = ::core::ptr::null_mut::<*mut gchar>();
    if !(g_hash_table_size((*(*proxy).priv_0).properties) == 0 as guint) {
        p = g_ptr_array_new();
        g_hash_table_iter_init(&raw mut iter, (*(*proxy).priv_0).properties);
        while g_hash_table_iter_next(
            &raw mut iter,
            &raw mut key as gpointer as *mut gpointer,
            ::core::ptr::null_mut::<gpointer>(),
        ) != 0
        {
            g_ptr_array_add(
                p,
                safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
            );
        }
        g_ptr_array_sort_values(
            p,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                GCompareFunc,
            >(Some(
                g_strcmp0
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            )),
        );
        g_ptr_array_add(p, NULL_0);
        names = g_ptr_array_free(p, FALSE) as *mut *mut gchar;
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
    return names;
}
unsafe extern "C" fn safe_c2rust_lookup_property_info(
    mut proxy: *mut GDBusProxy,
    mut property_name: *const gchar,
) -> *const GDBusPropertyInfo {
    let mut info: *const GDBusPropertyInfo = ::core::ptr::null::<GDBusPropertyInfo>();
    if !(*(*proxy).priv_0).expected_interface.is_null() {
        info = g_dbus_interface_info_lookup_property(
            (*(*proxy).priv_0).expected_interface,
            property_name,
        );
    }
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_cached_property(
    mut proxy: *mut GDBusProxy,
    mut property_name: *const gchar,
) -> *mut GVariant {
    let mut current_block: u64;
    let mut info: *const GDBusPropertyInfo = ::core::ptr::null::<GDBusPropertyInfo>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !property_name.is_null() {
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
            b"property_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    value = g_hash_table_lookup(
        (*(*proxy).priv_0).properties,
        property_name as gconstpointer,
    ) as *mut GVariant;
    if !value.is_null() {
        info = safe_c2rust_lookup_property_info(proxy, property_name);
        if !info.is_null() {
            let mut type_string: *const gchar = g_variant_get_type_string(value);
            if g_strcmp0(type_string as *const ::core::ffi::c_char, (*info).signature)
                != 0 as ::core::ffi::c_int
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Trying to get property %s with type %s but according to the expected interface the type is %s\0"
                        as *const u8 as *const gchar,
                    property_name,
                    type_string,
                    (*info).signature,
                );
                value = ::core::ptr::null_mut::<GVariant>();
                current_block = 11762248566346914425;
            } else {
                current_block = 2370887241019905314;
            }
        } else {
            current_block = 2370887241019905314;
        }
        match current_block {
            11762248566346914425 => {}
            _ => {
                g_variant_ref(value);
            }
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_set_cached_property(
    mut proxy: *mut GDBusProxy,
    mut property_name: *const gchar,
    mut value: *mut GVariant,
) {
    let mut current_block: u64;
    let mut info: *const GDBusPropertyInfo = ::core::ptr::null::<GDBusPropertyInfo>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !property_name.is_null() {
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
            b"property_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    if !value.is_null() {
        info = safe_c2rust_lookup_property_info(proxy, property_name);
        if !info.is_null() {
            if g_strcmp0(
                (*info).signature,
                g_variant_get_type_string(value) as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Trying to set property %s of type %s but according to the expected interface the type is %s\0"
                        as *const u8 as *const gchar,
                    property_name,
                    g_variant_get_type_string(value),
                    (*info).signature,
                );
                current_block = 12074267507492971375;
            } else {
                current_block = 5143058163439228106;
            }
        } else {
            current_block = 5143058163439228106;
        }
        match current_block {
            12074267507492971375 => {}
            _ => {
                g_hash_table_insert(
                    (*(*proxy).priv_0).properties,
                    safe_c2rust_g_strdup_inline(property_name as *const ::core::ffi::c_char)
                        as gpointer,
                    g_variant_ref_sink(value) as gpointer,
                );
            }
        }
    } else {
        g_hash_table_remove(
            (*(*proxy).priv_0).properties,
            property_name as gconstpointer,
        );
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
}
unsafe extern "C" fn safe_c2rust_on_signal_received(
    mut connection: *mut GDBusConnection,
    mut sender_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut current_block: u64;
    let mut proxy_weak: *mut GWeakRef = user_data as *mut GWeakRef;
    let mut proxy: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    proxy = g_weak_ref_get(proxy_weak) as *mut GDBusProxy;
    if proxy.is_null() {
        return;
    }
    if !((*(*proxy).priv_0).initialized == 0) {
        g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
        if !(*(*proxy).priv_0).name_owner.is_null()
            && g_strcmp0(
                sender_name as *const ::core::ffi::c_char,
                (*(*proxy).priv_0).name_owner,
            ) != 0 as ::core::ffi::c_int
        {
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
        } else {
            if !(*(*proxy).priv_0).expected_interface.is_null() {
                let mut info: *const GDBusSignalInfo = ::core::ptr::null::<GDBusSignalInfo>();
                info = g_dbus_interface_info_lookup_signal(
                    (*(*proxy).priv_0).expected_interface,
                    signal_name,
                );
                if !info.is_null() {
                    let mut expected_type: *mut GVariantType =
                        ::core::ptr::null_mut::<GVariantType>();
                    expected_type = _g_dbus_compute_complete_signature((*info).args);
                    if g_variant_type_equal(
                        expected_type as gconstpointer,
                        g_variant_get_type(parameters) as gconstpointer,
                    ) == 0
                    {
                        let mut expected_type_string: *mut gchar =
                            g_variant_type_dup_string(expected_type);
                        g_log(
                            G_LOG_DOMAIN.as_ptr() as *const gchar,
                            G_LOG_LEVEL_WARNING,
                            b"Dropping signal %s of type %s since the type from the expected interface is %s\0"
                                as *const u8 as *const gchar,
                            (*info).name,
                            g_variant_get_type_string(parameters),
                            expected_type_string,
                        );
                        g_free(expected_type_string as gpointer);
                        g_variant_type_free(expected_type);
                        g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
                        current_block = 12893935009979329006;
                    } else {
                        g_variant_type_free(expected_type);
                        current_block = 13056961889198038528;
                    }
                } else {
                    current_block = 13056961889198038528;
                }
            } else {
                current_block = 13056961889198038528;
            }
            match current_block {
                12893935009979329006 => {}
                _ => {
                    g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
                    g_signal_emit(
                        proxy as gpointer,
                        safe_c2rust_signals[SIGNAL_SIGNAL as ::core::ffi::c_int as usize],
                        g_quark_try_string(signal_name),
                        sender_name,
                        signal_name,
                        parameters,
                    );
                }
            }
        }
    }
    let mut _pp: *mut *mut GDBusProxy = &raw mut proxy;
    let mut _ptr: *mut GDBusProxy = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusProxy>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_insert_property_checked(
    mut proxy: *mut GDBusProxy,
    mut property_name: *mut gchar,
    mut value: *mut GVariant,
) {
    if !(*(*proxy).priv_0).expected_interface.is_null() {
        let mut info: *const GDBusPropertyInfo = ::core::ptr::null::<GDBusPropertyInfo>();
        info = g_dbus_interface_info_lookup_property(
            (*(*proxy).priv_0).expected_interface,
            property_name,
        );
        if !info.is_null() {
            if g_strcmp0(
                (*info).signature,
                g_variant_get_type_string(value) as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Received property %s with type %s does not match expected type %s in the expected interface\0"
                        as *const u8 as *const gchar,
                    property_name,
                    g_variant_get_type_string(value),
                    (*info).signature,
                );
                g_variant_unref(value);
                g_free(property_name as gpointer);
                return;
            }
        }
    }
    g_hash_table_insert(
        (*(*proxy).priv_0).properties,
        property_name as gpointer,
        value as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_invalidated_property_get_cb(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut InvalidatedPropGetData = user_data as *mut InvalidatedPropGetData;
    let mut invalidated_properties: [*const gchar; 1] = [::core::ptr::null::<gchar>()];
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut unpacked_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    value = g_dbus_connection_call_finish(connection, res, ::core::ptr::null_mut::<*mut GError>());
    if !value.is_null() {
        if g_variant_is_of_type(
            value,
            g_variant_type_checked_(b"(v)\0" as *const u8 as *const gchar),
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Expected type '(v)' for Get() reply, got '%s'\0" as *const u8 as *const gchar,
                g_variant_get_type_string(value),
            );
        } else {
            g_variant_get(
                value,
                b"(v)\0" as *const u8 as *const gchar,
                &raw mut unpacked_value,
            );
            g_variant_builder_init(
                &raw mut builder,
                g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
            );
            g_variant_builder_add(
                &raw mut builder,
                b"{sv}\0" as *const u8 as *const gchar,
                (*data).prop_name,
                unpacked_value,
            );
            g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
            safe_c2rust_insert_property_checked((*data).proxy, (*data).prop_name, unpacked_value);
            (*data).prop_name = ::core::ptr::null_mut::<gchar>();
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
            g_signal_emit(
                (*data).proxy as gpointer,
                safe_c2rust_signals[PROPERTIES_CHANGED_SIGNAL as ::core::ffi::c_int as usize],
                0 as GQuark,
                g_variant_builder_end(&raw mut builder),
                &raw mut invalidated_properties as *mut *const gchar,
            );
        }
    }
    if !value.is_null() {
        g_variant_unref(value);
    }
    g_object_unref((*data).proxy as gpointer);
    g_free((*data).prop_name as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<InvalidatedPropGetData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_on_properties_changed(
    mut connection: *mut GDBusConnection,
    mut sender_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut proxy_weak: *mut GWeakRef = user_data as *mut GWeakRef;
    let mut emit_g_signal: gboolean = FALSE;
    let mut proxy: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    let mut interface_name_for_signal: *const gchar = ::core::ptr::null::<gchar>();
    let mut changed_properties: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut invalidated_properties: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut n: guint = 0;
    changed_properties = ::core::ptr::null_mut::<GVariant>();
    invalidated_properties = ::core::ptr::null_mut::<*mut gchar>();
    proxy = g_weak_ref_get(proxy_weak) as *mut GDBusProxy;
    if proxy.is_null() {
        return;
    }
    if !((*(*proxy).priv_0).initialized == 0) {
        g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
        if !(*(*proxy).priv_0).name_owner.is_null()
            && g_strcmp0(
                sender_name as *const ::core::ffi::c_char,
                (*(*proxy).priv_0).name_owner,
            ) != 0 as ::core::ffi::c_int
        {
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
        } else if g_variant_is_of_type(
            parameters,
            g_variant_type_checked_(b"(sa{sv}as)\0" as *const u8 as *const gchar),
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Value for PropertiesChanged signal with type '%s' does not match '(sa{sv}as)'\0"
                    as *const u8 as *const gchar,
                g_variant_get_type_string(parameters),
            );
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
        } else {
            g_variant_get(
                parameters,
                b"(&s@a{sv}^a&s)\0" as *const u8 as *const gchar,
                &raw mut interface_name_for_signal,
                &raw mut changed_properties,
                &raw mut invalidated_properties,
            );
            if g_strcmp0(
                interface_name_for_signal as *const ::core::ffi::c_char,
                (*(*proxy).priv_0).interface_name,
            ) != 0 as ::core::ffi::c_int
            {
                g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
            } else {
                g_variant_iter_init(&raw mut iter, changed_properties);
                while g_variant_iter_next(
                    &raw mut iter,
                    b"{sv}\0" as *const u8 as *const gchar,
                    &raw mut key,
                    &raw mut value,
                ) != 0
                {
                    safe_c2rust_insert_property_checked(proxy, key, value);
                    emit_g_signal = TRUE as gboolean;
                }
                if (*(*proxy).priv_0).flags as ::core::ffi::c_uint
                    & G_DBUS_PROXY_FLAGS_GET_INVALIDATED_PROPERTIES as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                {
                    if !(*(*proxy).priv_0).name_owner.is_null() {
                        n = 0 as guint;
                        while !(*invalidated_properties.offset(n as isize)).is_null() {
                            let mut data: *mut InvalidatedPropGetData =
                                ::core::ptr::null_mut::<InvalidatedPropGetData>();
                            data = ({
                                let mut __s: gsize =
                                    ::core::mem::size_of::<InvalidatedPropGetData>() as gsize;
                                let mut __p: gpointer =
                                    ::core::ptr::null_mut::<::core::ffi::c_void>();
                                __p = g_slice_alloc(__s);
                                memset(
                                    __p as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    __s as size_t,
                                );
                                __p
                            }) as *mut InvalidatedPropGetData;
                            (*data).proxy = g_object_ref(proxy as gpointer) as *mut GDBusProxy
                                as *mut GDBusProxy;
                            (*data).prop_name = safe_c2rust_g_strdup_inline(
                                *invalidated_properties.offset(n as isize),
                            ) as *mut gchar;
                            g_dbus_connection_call(
                                (*(*proxy).priv_0).connection,
                                (*(*proxy).priv_0).name_owner,
                                (*(*proxy).priv_0).object_path,
                                b"org.freedesktop.DBus.Properties\0" as *const u8 as *const gchar,
                                b"Get\0" as *const u8 as *const gchar,
                                g_variant_new(
                                    b"(ss)\0" as *const u8 as *const gchar,
                                    (*(*proxy).priv_0).interface_name,
                                    (*data).prop_name,
                                ),
                                g_variant_type_checked_(b"(v)\0" as *const u8 as *const gchar),
                                G_DBUS_CALL_FLAGS_NONE,
                                -(1 as gint),
                                ::core::ptr::null_mut::<GCancellable>(),
                                ::core::mem::transmute::<
                                    Option<
                                        unsafe extern "C" fn(
                                            *mut GDBusConnection,
                                            *mut GAsyncResult,
                                            gpointer,
                                        )
                                            -> (),
                                    >,
                                    GAsyncReadyCallback,
                                >(Some(
                                    safe_c2rust_invalidated_property_get_cb
                                        as unsafe extern "C" fn(
                                            *mut GDBusConnection,
                                            *mut GAsyncResult,
                                            gpointer,
                                        )
                                            -> (),
                                )),
                                data as gpointer,
                            );
                            n = n.wrapping_add(1);
                        }
                    }
                } else {
                    emit_g_signal = TRUE as gboolean;
                    n = 0 as guint;
                    while !(*invalidated_properties.offset(n as isize)).is_null() {
                        g_hash_table_remove(
                            (*(*proxy).priv_0).properties,
                            *invalidated_properties.offset(n as isize) as gconstpointer,
                        );
                        n = n.wrapping_add(1);
                    }
                }
                g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
                if emit_g_signal != 0 {
                    g_signal_emit(
                        proxy as gpointer,
                        safe_c2rust_signals
                            [PROPERTIES_CHANGED_SIGNAL as ::core::ffi::c_int as usize],
                        0 as GQuark,
                        changed_properties,
                        invalidated_properties,
                    );
                }
            }
        }
    }
    let mut _pp: *mut *mut GVariant = &raw mut changed_properties;
    let mut _ptr: *mut GVariant = *_pp;
    *_pp = ::core::ptr::null_mut::<GVariant>();
    if !_ptr.is_null() {
        g_variant_unref(_ptr as *mut GVariant);
    }
    g_free(invalidated_properties as gpointer);
    let mut _pp_0: *mut *mut GDBusProxy = &raw mut proxy;
    let mut _ptr_0: *mut GDBusProxy = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GDBusProxy>();
    if !_ptr_0.is_null() {
        g_object_unref(_ptr_0 as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_process_get_all_reply(
    mut proxy: *mut GDBusProxy,
    mut result: *mut GVariant,
) {
    let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    let mut key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut num_properties: guint = 0;
    if g_variant_is_of_type(
        result,
        g_variant_type_checked_(b"(a{sv})\0" as *const u8 as *const gchar),
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Value for GetAll reply with type '%s' does not match '(a{sv})'\0" as *const u8
                as *const gchar,
            g_variant_get_type_string(result),
        );
    } else {
        g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
        g_variant_get(
            result,
            b"(a{sv})\0" as *const u8 as *const gchar,
            &raw mut iter,
        );
        while g_variant_iter_next(
            iter,
            b"{sv}\0" as *const u8 as *const gchar,
            &raw mut key,
            &raw mut value,
        ) != 0
        {
            safe_c2rust_insert_property_checked(proxy, key, value);
        }
        g_variant_iter_free(iter);
        num_properties = g_hash_table_size((*(*proxy).priv_0).properties);
        g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
        if num_properties > 0 as guint {
            let mut changed_properties: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            let mut invalidated_properties: [*const gchar; 1] = [::core::ptr::null::<gchar>()];
            g_variant_get(
                result,
                b"(@a{sv})\0" as *const u8 as *const gchar,
                &raw mut changed_properties,
            );
            g_signal_emit(
                proxy as gpointer,
                safe_c2rust_signals[PROPERTIES_CHANGED_SIGNAL as ::core::ffi::c_int as usize],
                0 as GQuark,
                changed_properties,
                &raw mut invalidated_properties as *mut *const gchar,
            );
            g_variant_unref(changed_properties);
        }
    };
}
unsafe extern "C" fn safe_c2rust_on_name_owner_changed_get_all_cb(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut LoadPropertiesOnNameOwnerChangedData =
        user_data as *mut LoadPropertiesOnNameOwnerChangedData;
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut cancelled: gboolean = 0;
    cancelled = FALSE as gboolean;
    error = ::core::ptr::null_mut::<GError>();
    result = g_dbus_connection_call_finish(connection, res, &raw mut error);
    if result.is_null() {
        if (*error).domain == g_io_error_quark()
            && (*error).code == G_IO_ERROR_CANCELLED as ::core::ffi::c_int
        {
            cancelled = TRUE as gboolean;
        }
        if ({
            let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
            if _g_dbus_debug_proxy() != 0 {
                _g_boolean_var_16 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_16 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_16
        }) as ::core::ffi::c_long
            != 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_DEBUG,
                b"error: %d %d %s\0" as *const u8 as *const gchar,
                (*error).domain,
                (*error).code,
                (*error).message,
            );
        }
        g_error_free(error);
    }
    if cancelled == 0 {
        g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
        g_free((*(*(*data).proxy).priv_0).name_owner as gpointer);
        (*(*(*data).proxy).priv_0).name_owner =
            safe_c2rust_g_steal_pointer(&raw mut (*data).name_owner as gpointer) as *mut gchar
                as *mut gchar;
        g_hash_table_remove_all((*(*(*data).proxy).priv_0).properties);
        g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
        if !result.is_null() {
            safe_c2rust_process_get_all_reply((*data).proxy, result);
            g_variant_unref(result);
        }
        g_object_notify(
            (*data).proxy as *mut ::core::ffi::c_void as *mut GObject,
            b"g-name-owner\0" as *const u8 as *const gchar,
        );
    }
    if (*data).cancellable == (*(*(*data).proxy).priv_0).get_all_cancellable {
        (*(*(*data).proxy).priv_0).get_all_cancellable = ::core::ptr::null_mut::<GCancellable>();
    }
    g_object_unref((*data).proxy as gpointer);
    g_object_unref((*data).cancellable as gpointer);
    g_free((*data).name_owner as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_on_name_owner_changed(
    mut connection: *mut GDBusConnection,
    mut sender_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut proxy_weak: *mut GWeakRef = user_data as *mut GWeakRef;
    let mut proxy: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    let mut old_owner: *const gchar = ::core::ptr::null::<gchar>();
    let mut new_owner: *const gchar = ::core::ptr::null::<gchar>();
    proxy = g_weak_ref_get(proxy_weak) as *mut GDBusProxy;
    if proxy.is_null() {
        return;
    }
    if !(*(*proxy).priv_0).get_all_cancellable.is_null() {
        g_cancellable_cancel((*(*proxy).priv_0).get_all_cancellable);
        (*(*proxy).priv_0).get_all_cancellable = ::core::ptr::null_mut::<GCancellable>();
    }
    g_variant_get(
        parameters,
        b"(&s&s&s)\0" as *const u8 as *const gchar,
        NULL_0,
        &raw mut old_owner,
        &raw mut new_owner,
    );
    if strlen(new_owner as *const ::core::ffi::c_char) == 0 as size_t {
        g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
        g_free((*(*proxy).priv_0).name_owner as gpointer);
        (*(*proxy).priv_0).name_owner = ::core::ptr::null_mut::<gchar>();
        if (*(*proxy).priv_0).flags as ::core::ffi::c_uint
            & G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
            && g_hash_table_size((*(*proxy).priv_0).properties) > 0 as guint
        {
            let mut builder: GVariantBuilder = _GVariantBuilder {
                u: C2RustUnnamed {
                    s: C2RustUnnamed_0 {
                        partial_magic: 0,
                        type_0: ::core::ptr::null::<GVariantType>(),
                        y: [0; 14],
                    },
                },
            };
            let mut invalidated_properties: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
            let mut iter: GHashTableIter = _GHashTableIter {
                dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                dummy4: 0,
                dummy5: 0,
                dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            };
            let mut key: *const gchar = ::core::ptr::null::<gchar>();
            g_variant_builder_init(
                &raw mut builder,
                g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
            );
            invalidated_properties = g_ptr_array_new_with_free_func(Some(
                g_free as unsafe extern "C" fn(gpointer) -> (),
            ));
            g_hash_table_iter_init(&raw mut iter, (*(*proxy).priv_0).properties);
            while g_hash_table_iter_next(
                &raw mut iter,
                &raw mut key as gpointer as *mut gpointer,
                ::core::ptr::null_mut::<gpointer>(),
            ) != 0
            {
                g_ptr_array_add(
                    invalidated_properties,
                    safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
                );
            }
            g_ptr_array_add(invalidated_properties, NULL_0);
            g_hash_table_remove_all((*(*proxy).priv_0).properties);
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
            g_signal_emit(
                proxy as gpointer,
                safe_c2rust_signals[PROPERTIES_CHANGED_SIGNAL as ::core::ffi::c_int as usize],
                0 as GQuark,
                g_variant_builder_end(&raw mut builder),
                (*invalidated_properties).pdata as *const *const gchar,
            );
            g_ptr_array_unref(invalidated_properties);
        } else {
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
        }
        g_object_notify(
            proxy as *mut ::core::ffi::c_void as *mut GObject,
            b"g-name-owner\0" as *const u8 as *const gchar,
        );
    } else {
        g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
        if g_strcmp0(
            new_owner as *const ::core::ffi::c_char,
            (*(*proxy).priv_0).name_owner,
        ) == 0 as ::core::ffi::c_int
        {
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
        } else if (*(*proxy).priv_0).flags as ::core::ffi::c_uint
            & G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            g_free((*(*proxy).priv_0).name_owner as gpointer);
            (*(*proxy).priv_0).name_owner =
                safe_c2rust_g_strdup_inline(new_owner as *const ::core::ffi::c_char) as *mut gchar;
            g_hash_table_remove_all((*(*proxy).priv_0).properties);
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
            g_object_notify(
                proxy as *mut ::core::ffi::c_void as *mut GObject,
                b"g-name-owner\0" as *const u8 as *const gchar,
            );
        } else {
            let mut data: *mut LoadPropertiesOnNameOwnerChangedData =
                ::core::ptr::null_mut::<LoadPropertiesOnNameOwnerChangedData>();
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
            if ({
                let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                if (*(*proxy).priv_0).get_all_cancellable.is_null() {
                    _g_boolean_var_17 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_17 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_17
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusproxy.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1324 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"proxy->priv->get_all_cancellable == NULL\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            (*(*proxy).priv_0).get_all_cancellable = g_cancellable_new();
            data = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize =
                    ::core::mem::size_of::<LoadPropertiesOnNameOwnerChangedData>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut LoadPropertiesOnNameOwnerChangedData;
            (*data).proxy = g_object_ref(proxy as gpointer) as *mut GDBusProxy as *mut GDBusProxy;
            (*data).cancellable = (*(*proxy).priv_0).get_all_cancellable;
            (*data).name_owner =
                safe_c2rust_g_strdup_inline(new_owner as *const ::core::ffi::c_char) as *mut gchar;
            g_dbus_connection_call(
                (*(*proxy).priv_0).connection,
                (*data).name_owner,
                (*(*proxy).priv_0).object_path,
                b"org.freedesktop.DBus.Properties\0" as *const u8 as *const gchar,
                b"GetAll\0" as *const u8 as *const gchar,
                g_variant_new(
                    b"(s)\0" as *const u8 as *const gchar,
                    (*(*proxy).priv_0).interface_name,
                ),
                g_variant_type_checked_(b"(a{sv})\0" as *const u8 as *const gchar),
                G_DBUS_CALL_FLAGS_NONE,
                -(1 as gint),
                (*(*proxy).priv_0).get_all_cancellable,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *mut GDBusConnection,
                            *mut GAsyncResult,
                            gpointer,
                        ) -> (),
                    >,
                    GAsyncReadyCallback,
                >(Some(
                    safe_c2rust_on_name_owner_changed_get_all_cb
                        as unsafe extern "C" fn(
                            *mut GDBusConnection,
                            *mut GAsyncResult,
                            gpointer,
                        ) -> (),
                )),
                data as gpointer,
            );
        }
    }
    let mut _pp: *mut *mut GDBusProxy = &raw mut proxy;
    let mut _ptr: *mut GDBusProxy = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusProxy>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_async_init_get_all_cb(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    error = ::core::ptr::null_mut::<GError>();
    result = g_dbus_connection_call_finish(connection, res, &raw mut error);
    if result.is_null() {
        if ({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if _g_dbus_debug_proxy() != 0 {
                _g_boolean_var_18 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_18 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_18
        }) as ::core::ffi::c_long
            != 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_DEBUG,
                b"error: %d %d %s\0" as *const u8 as *const gchar,
                (*error).domain,
                (*error).code,
                (*error).message,
            );
        }
        g_error_free(error);
    }
    g_task_return_pointer(
        task,
        result as gpointer,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GVariant) -> ()>, GDestroyNotify>(
            Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()),
        ),
    );
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_async_init_data_set_name_owner(
    mut task: *mut GTask,
    mut name_owner: *const gchar,
) {
    let mut proxy: *mut GDBusProxy = g_task_get_source_object(task) as *mut GDBusProxy;
    let mut get_all: gboolean = 0;
    if !name_owner.is_null() {
        g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
        g_free((*(*proxy).priv_0).name_owner as gpointer);
        (*(*proxy).priv_0).name_owner =
            safe_c2rust_g_strdup_inline(name_owner as *const ::core::ffi::c_char) as *mut gchar;
        g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
    }
    get_all = TRUE as gboolean;
    if (*(*proxy).priv_0).flags as ::core::ffi::c_uint
        & G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        get_all = FALSE as gboolean;
    } else if name_owner.is_null() && !(*(*proxy).priv_0).name.is_null() {
        get_all = FALSE as gboolean;
    }
    if get_all != 0 {
        g_dbus_connection_call(
            (*(*proxy).priv_0).connection,
            name_owner,
            (*(*proxy).priv_0).object_path,
            b"org.freedesktop.DBus.Properties\0" as *const u8 as *const gchar,
            b"GetAll\0" as *const u8 as *const gchar,
            g_variant_new(
                b"(s)\0" as *const u8 as *const gchar,
                (*(*proxy).priv_0).interface_name,
            ),
            g_variant_type_checked_(b"(a{sv})\0" as *const u8 as *const gchar),
            G_DBUS_CALL_FLAGS_NONE,
            -(1 as gint),
            g_task_get_cancellable(task),
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut GDBusConnection, *mut GAsyncResult, gpointer) -> (),
                >,
                GAsyncReadyCallback,
            >(Some(
                safe_c2rust_async_init_get_all_cb
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *mut GAsyncResult,
                        gpointer,
                    ) -> (),
            )),
            task as gpointer,
        );
    } else {
        g_task_return_pointer(task, NULL_0, None);
        g_object_unref(task as gpointer);
    };
}
unsafe extern "C" fn safe_c2rust_async_init_get_name_owner_cb(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    error = ::core::ptr::null_mut::<GError>();
    result = g_dbus_connection_call_finish(connection, res, &raw mut error);
    if result.is_null() {
        if (*error).domain == g_dbus_error_quark()
            && (*error).code == G_DBUS_ERROR_NAME_HAS_NO_OWNER as ::core::ffi::c_int
        {
            g_error_free(error);
            safe_c2rust_async_init_data_set_name_owner(task, ::core::ptr::null::<gchar>());
        } else {
            g_task_return_error(task, error);
            g_object_unref(task as gpointer);
        }
    } else {
        let mut name_owner: *const gchar = ::core::ptr::null::<gchar>();
        g_variant_get(
            result,
            b"(&s)\0" as *const u8 as *const gchar,
            &raw mut name_owner,
        );
        safe_c2rust_async_init_data_set_name_owner(task, name_owner);
        g_variant_unref(result);
    };
}
unsafe extern "C" fn safe_c2rust_async_init_call_get_name_owner(mut task: *mut GTask) {
    let mut proxy: *mut GDBusProxy = g_task_get_source_object(task) as *mut GDBusProxy;
    g_dbus_connection_call(
        (*(*proxy).priv_0).connection,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"GetNameOwner\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(s)\0" as *const u8 as *const gchar,
            (*(*proxy).priv_0).name,
        ),
        g_variant_type_checked_(b"(s)\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        g_task_get_cancellable(task),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusConnection, *mut GAsyncResult, gpointer) -> ()>,
            GAsyncReadyCallback,
        >(Some(
            safe_c2rust_async_init_get_name_owner_cb
                as unsafe extern "C" fn(*mut GDBusConnection, *mut GAsyncResult, gpointer) -> (),
        )),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_async_init_start_service_by_name_cb(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut current_block: u64;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut proxy: *mut GDBusProxy = g_task_get_source_object(task) as *mut GDBusProxy;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    error = ::core::ptr::null_mut::<GError>();
    result = g_dbus_connection_call_finish(connection, res, &raw mut error);
    if result.is_null() {
        if (*error).domain == g_dbus_error_quark()
            && (*error).code == G_DBUS_ERROR_SERVICE_UNKNOWN as ::core::ffi::c_int
        {
            g_error_free(error);
            current_block = 5689001924483802034;
        } else {
            let mut remote_error: *mut gchar = g_dbus_error_get_remote_error(error);
            if g_strcmp0(
                remote_error,
                b"org.freedesktop.systemd1.Masked\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                g_error_free(error);
                g_free(remote_error as gpointer);
                current_block = 5689001924483802034;
            } else {
                g_dbus_error_strip_remote_error(error);
                g_prefix_error(
                    &raw mut error,
                    glib_gettext(
                        b"Error calling StartServiceByName for %s: \0" as *const u8 as *const gchar,
                    ),
                    (*(*proxy).priv_0).name,
                );
                g_free(remote_error as gpointer);
                current_block = 9660365119270090090;
            }
        }
    } else {
        let mut start_service_result: guint32 = 0;
        g_variant_get(
            result,
            b"(u)\0" as *const u8 as *const gchar,
            &raw mut start_service_result,
        );
        g_variant_unref(result);
        if start_service_result == 1 as guint32 || start_service_result == 2 as guint32 {
            current_block = 5689001924483802034;
        } else {
            error = g_error_new(
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unexpected reply %d from StartServiceByName(\"%s\") method\0" as *const u8
                        as *const gchar,
                ),
                start_service_result,
                (*(*proxy).priv_0).name,
            );
            current_block = 9660365119270090090;
        }
    }
    match current_block {
        9660365119270090090 => {
            if !(({
                let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                if !error.is_null() {
                    _g_boolean_var_19 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_19 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_19
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusproxy.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1586 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            g_task_return_error(task, error);
            g_object_unref(task as gpointer);
            return;
        }
        _ => {
            safe_c2rust_async_init_call_get_name_owner(task);
            return;
        }
    };
}
unsafe extern "C" fn safe_c2rust_async_init_call_start_service_by_name(mut task: *mut GTask) {
    let mut proxy: *mut GDBusProxy = g_task_get_source_object(task) as *mut GDBusProxy;
    g_dbus_connection_call(
        (*(*proxy).priv_0).connection,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"StartServiceByName\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(su)\0" as *const u8 as *const gchar,
            (*(*proxy).priv_0).name,
            0 as ::core::ffi::c_int,
        ),
        g_variant_type_checked_(b"(u)\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        g_task_get_cancellable(task),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusConnection, *mut GAsyncResult, gpointer) -> ()>,
            GAsyncReadyCallback,
        >(Some(
            safe_c2rust_async_init_start_service_by_name_cb
                as unsafe extern "C" fn(*mut GDBusConnection, *mut GAsyncResult, gpointer) -> (),
        )),
        task as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_async_initable_init_second_async(
    mut initable: *mut GAsyncInitable,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut proxy: *mut GDBusProxy = initable as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(proxy as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GAsyncInitable,
                    gint,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_async_initable_init_second_async
                as unsafe extern "C" fn(
                    *mut GAsyncInitable,
                    gint,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"async_initable_init_second_async\0" as *const u8 as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] D-Bus proxy init\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] D-Bus proxy init\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority);
    if (*(*proxy).priv_0).name.is_null() {
        safe_c2rust_async_init_data_set_name_owner(task, ::core::ptr::null::<gchar>());
    } else if g_dbus_is_unique_name((*(*proxy).priv_0).name) != 0 {
        safe_c2rust_async_init_data_set_name_owner(task, (*(*proxy).priv_0).name);
    } else if (*(*proxy).priv_0).flags as ::core::ffi::c_uint
        & G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        || (*(*proxy).priv_0).flags as ::core::ffi::c_uint
            & G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START_AT_CONSTRUCTION as ::core::ffi::c_int
                as ::core::ffi::c_uint
            != 0
    {
        safe_c2rust_async_init_call_get_name_owner(task);
    } else {
        safe_c2rust_async_init_call_start_service_by_name(task);
    };
}
unsafe extern "C" fn safe_c2rust_async_initable_init_second_finish(
    mut initable: *mut GAsyncInitable,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut proxy: *mut GDBusProxy = initable as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut task: *mut GTask = res as *mut ::core::ffi::c_void as *mut GTask;
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ret: gboolean = 0;
    ret = (g_task_had_error(task) == 0) as ::core::ffi::c_int as gboolean;
    result = g_task_propagate_pointer(task, error) as *mut GVariant;
    if !result.is_null() {
        safe_c2rust_process_get_all_reply(proxy, result);
        g_variant_unref(result);
    }
    (*(*proxy).priv_0).initialized = TRUE as gboolean;
    return ret;
}
unsafe extern "C" fn safe_c2rust_async_initable_init_first(mut initable: *mut GAsyncInitable) {
    let mut proxy: *mut GDBusProxy = initable as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut signal_flags: GDBusSignalFlags = G_DBUS_SIGNAL_FLAGS_NONE;
    if (*(*proxy).priv_0).flags as ::core::ffi::c_uint
        & G_DBUS_PROXY_FLAGS_NO_MATCH_RULE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        signal_flags = ::core::mem::transmute::<::core::ffi::c_uint, GDBusSignalFlags>(
            signal_flags as ::core::ffi::c_uint
                | G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if (*(*proxy).priv_0).flags as ::core::ffi::c_uint
        & G_DBUS_PROXY_FLAGS_DO_NOT_LOAD_PROPERTIES as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
    {
        (*(*proxy).priv_0).properties_changed_subscription_id = g_dbus_connection_signal_subscribe(
            (*(*proxy).priv_0).connection,
            (*(*proxy).priv_0).name,
            b"org.freedesktop.DBus.Properties\0" as *const u8 as *const gchar,
            b"PropertiesChanged\0" as *const u8 as *const gchar,
            (*(*proxy).priv_0).object_path,
            (*(*proxy).priv_0).interface_name,
            signal_flags,
            Some(
                safe_c2rust_on_properties_changed
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut GVariant,
                        gpointer,
                    ) -> (),
            ),
            safe_c2rust_weak_ref_new(proxy as *mut ::core::ffi::c_void as *mut GObject) as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GWeakRef) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_weak_ref_free as unsafe extern "C" fn(*mut GWeakRef) -> (),
            )),
        );
    }
    if (*(*proxy).priv_0).flags as ::core::ffi::c_uint
        & G_DBUS_PROXY_FLAGS_DO_NOT_CONNECT_SIGNALS as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
    {
        (*(*proxy).priv_0).signals_subscription_id = g_dbus_connection_signal_subscribe(
            (*(*proxy).priv_0).connection,
            (*(*proxy).priv_0).name,
            (*(*proxy).priv_0).interface_name,
            ::core::ptr::null::<gchar>(),
            (*(*proxy).priv_0).object_path,
            ::core::ptr::null::<gchar>(),
            signal_flags,
            Some(
                safe_c2rust_on_signal_received
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut GVariant,
                        gpointer,
                    ) -> (),
            ),
            safe_c2rust_weak_ref_new(proxy as *mut ::core::ffi::c_void as *mut GObject) as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GWeakRef) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_weak_ref_free as unsafe extern "C" fn(*mut GWeakRef) -> (),
            )),
        );
    }
    if !(*(*proxy).priv_0).name.is_null()
        && g_dbus_connection_get_flags((*(*proxy).priv_0).connection) as ::core::ffi::c_uint
            & G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
                as ::core::ffi::c_uint
            != 0
    {
        (*(*proxy).priv_0).name_owner_changed_subscription_id = g_dbus_connection_signal_subscribe(
            (*(*proxy).priv_0).connection,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
            b"NameOwnerChanged\0" as *const u8 as *const gchar,
            b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
            (*(*proxy).priv_0).name,
            signal_flags,
            Some(
                safe_c2rust_on_name_owner_changed
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut GVariant,
                        gpointer,
                    ) -> (),
            ),
            safe_c2rust_weak_ref_new(proxy as *mut ::core::ffi::c_void as *mut GObject) as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GWeakRef) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_weak_ref_free as unsafe extern "C" fn(*mut GWeakRef) -> (),
            )),
        );
    }
}
unsafe extern "C" fn safe_c2rust_init_second_async_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_async_initable_init_second_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        &raw mut error,
    ) != 0
    {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_get_connection_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut proxy: *mut GDBusProxy = g_task_get_source_object(task) as *mut GDBusProxy;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    error = ::core::ptr::null_mut::<GError>();
    (*(*proxy).priv_0).connection = g_bus_get_finish(res, &raw mut error);
    if (*(*proxy).priv_0).connection.is_null() {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
    } else {
        safe_c2rust_async_initable_init_first(
            proxy as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        );
        safe_c2rust_async_initable_init_second_async(
            proxy as *mut ::core::ffi::c_void as *mut GAsyncInitable,
            g_task_get_priority(task),
            g_task_get_cancellable(task),
            Some(
                safe_c2rust_init_second_async_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_async_initable_init_async(
    mut initable: *mut GAsyncInitable,
    mut io_priority: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut proxy: *mut GDBusProxy = initable as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    task = g_task_new(proxy as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GAsyncInitable,
                    gint,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_async_initable_init_async
                as unsafe extern "C" fn(
                    *mut GAsyncInitable,
                    gint,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"async_initable_init_async\0" as *const u8 as *const gchar,
        );
    }
    let mut _task_0: *mut GTask = task;
    if 0 != 0 {
        g_task_set_static_name(
            _task_0,
            b"[gio] D-Bus proxy init\0" as *const u8 as *const gchar,
        );
    } else {
        g_task_set_name(
            _task_0,
            b"[gio] D-Bus proxy init\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_priority(task, io_priority);
    if (*(*proxy).priv_0).bus_type as ::core::ffi::c_int != G_BUS_TYPE_NONE as ::core::ffi::c_int {
        if ({
            let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
            if (*(*proxy).priv_0).connection.is_null() {
                _g_boolean_var_20 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_20 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_20
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusproxy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1805 as ::core::ffi::c_int,
                G_STRFUNC,
                b"proxy->priv->connection == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_bus_get(
            (*(*proxy).priv_0).bus_type,
            cancellable,
            Some(
                safe_c2rust_get_connection_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    } else {
        safe_c2rust_async_initable_init_first(initable);
        safe_c2rust_async_initable_init_second_async(
            initable,
            io_priority,
            cancellable,
            Some(
                safe_c2rust_init_second_async_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_async_initable_init_finish(
    mut initable: *mut GAsyncInitable,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    return g_task_propagate_boolean(res as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_async_initable_iface_init(
    mut async_initable_iface: *mut GAsyncInitableIface,
) {
    (*async_initable_iface).init_async = Some(
        safe_c2rust_async_initable_init_async
            as unsafe extern "C" fn(
                *mut GAsyncInitable,
                gint,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GAsyncInitable,
                ::core::ffi::c_int,
                *mut GCancellable,
                GAsyncReadyCallback,
                gpointer,
            ) -> (),
        >;
    (*async_initable_iface).init_finish = Some(
        safe_c2rust_async_initable_init_finish
            as unsafe extern "C" fn(
                *mut GAsyncInitable,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GAsyncInitable,
                *mut GAsyncResult,
                *mut *mut GError,
            ) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_async_initable_init_async_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut InitableAsyncInitableData = user_data as *mut InitableAsyncInitableData;
    (*data).res = g_object_ref(res as gpointer) as *mut GAsyncResult as *mut GAsyncResult;
    g_main_loop_quit((*data).loop_0);
}
unsafe extern "C" fn safe_c2rust_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut proxy: *mut GDBusProxy = initable as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut data: *mut InitableAsyncInitableData =
        ::core::ptr::null_mut::<InitableAsyncInitableData>();
    let mut ret: gboolean = 0;
    ret = FALSE as gboolean;
    if (*(*proxy).priv_0).bus_type as ::core::ffi::c_int != G_BUS_TYPE_NONE as ::core::ffi::c_int {
        if ({
            let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
            if (*(*proxy).priv_0).connection.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusproxy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1875 as ::core::ffi::c_int,
                G_STRFUNC,
                b"proxy->priv->connection == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*(*proxy).priv_0).connection =
            g_bus_get_sync((*(*proxy).priv_0).bus_type, cancellable, error);
        if (*(*proxy).priv_0).connection.is_null() {
            current_block = 9357053159281113688;
        } else {
            current_block = 13513818773234778473;
        }
    } else {
        current_block = 13513818773234778473;
    }
    match current_block {
        13513818773234778473 => {
            safe_c2rust_async_initable_init_first(
                initable as *mut ::core::ffi::c_void as *mut GAsyncInitable,
            );
            data = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<InitableAsyncInitableData>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut InitableAsyncInitableData;
            (*data).context = g_main_context_new();
            (*data).loop_0 = g_main_loop_new((*data).context, FALSE);
            g_main_context_push_thread_default((*data).context);
            safe_c2rust_async_initable_init_second_async(
                initable as *mut ::core::ffi::c_void as *mut GAsyncInitable,
                G_PRIORITY_DEFAULT,
                cancellable,
                Some(
                    safe_c2rust_async_initable_init_async_cb
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                data as gpointer,
            );
            g_main_loop_run((*data).loop_0);
            ret = safe_c2rust_async_initable_init_second_finish(
                initable as *mut ::core::ffi::c_void as *mut GAsyncInitable,
                (*data).res,
                error,
            );
            g_main_context_pop_thread_default((*data).context);
            g_main_context_unref((*data).context);
            g_main_loop_unref((*data).loop_0);
            g_object_unref((*data).res as gpointer);
            g_free(data as gpointer);
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_initable_iface_init(mut initable_iface: *mut GInitableIface) {
    (*initable_iface).init = Some(
        safe_c2rust_initable_init
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
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_new(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut info: *mut GDBusInterfaceInfo,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    _g_dbus_initialize();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if name.is_null() && g_dbus_connection_get_unique_name(connection).is_null()
            || g_dbus_is_name(name) != 0
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
            b"(name == NULL && g_dbus_connection_get_unique_name (connection) == NULL) || g_dbus_is_name (name)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if g_dbus_is_interface_name(interface_name) != 0 {
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
            b"g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_async_initable_new_async(
        safe_c2rust_g_dbus_proxy_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-interface-info\0" as *const u8 as *const ::core::ffi::c_char,
        info,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        interface_name,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_new_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GDBusProxy {
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    source_object = g_async_result_get_source_object(res);
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !source_object.is_null() {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusproxy.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2017 as ::core::ffi::c_int,
            G_STRFUNC,
            b"source_object != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    object = g_async_initable_new_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        error,
    );
    g_object_unref(source_object as gpointer);
    if !object.is_null() {
        return object as *mut ::core::ffi::c_void as *mut GDBusProxy;
    } else {
        return ::core::ptr::null_mut::<GDBusProxy>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_new_sync(
    mut connection: *mut GDBusConnection,
    mut flags: GDBusProxyFlags,
    mut info: *mut GDBusInterfaceInfo,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusProxy {
    let mut initable: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusProxy>();
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if name.is_null() && g_dbus_connection_get_unique_name(connection).is_null()
            || g_dbus_is_name(name) != 0
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
            b"(name == NULL && g_dbus_connection_get_unique_name (connection) == NULL) || g_dbus_is_name (name)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusProxy>();
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
        return ::core::ptr::null_mut::<GDBusProxy>();
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if g_dbus_is_interface_name(interface_name) != 0 {
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
            b"g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusProxy>();
    }
    initable = g_initable_new(
        safe_c2rust_g_dbus_proxy_get_type(),
        cancellable,
        error,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-interface-info\0" as *const u8 as *const ::core::ffi::c_char,
        info,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        interface_name,
        NULL_0,
    ) as *mut GInitable;
    if !initable.is_null() {
        return initable as *mut ::core::ffi::c_void as *mut GDBusProxy;
    } else {
        return ::core::ptr::null_mut::<GDBusProxy>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_new_for_bus(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut info: *mut GDBusInterfaceInfo,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    _g_dbus_initialize();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if g_dbus_is_name(name) != 0 {
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
            b"g_dbus_is_name (name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if g_dbus_is_interface_name(interface_name) != 0 {
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
            b"g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_async_initable_new_async(
        safe_c2rust_g_dbus_proxy_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-interface-info\0" as *const u8 as *const ::core::ffi::c_char,
        info,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-bus-type\0" as *const u8 as *const ::core::ffi::c_char,
        bus_type as ::core::ffi::c_int,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        interface_name,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_new_for_bus_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GDBusProxy {
    return safe_c2rust_g_dbus_proxy_new_finish(res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_new_for_bus_sync(
    mut bus_type: GBusType,
    mut flags: GDBusProxyFlags,
    mut info: *mut GDBusInterfaceInfo,
    mut name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusProxy {
    let mut initable: *mut GInitable = ::core::ptr::null_mut::<GInitable>();
    _g_dbus_initialize();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if g_dbus_is_name(name) != 0 {
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
            b"g_dbus_is_name (name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusProxy>();
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusProxy>();
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if g_dbus_is_interface_name(interface_name) != 0 {
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
            b"g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusProxy>();
    }
    initable = g_initable_new(
        safe_c2rust_g_dbus_proxy_get_type(),
        cancellable,
        error,
        b"g-flags\0" as *const u8 as *const gchar,
        flags as ::core::ffi::c_uint,
        b"g-interface-info\0" as *const u8 as *const ::core::ffi::c_char,
        info,
        b"g-name\0" as *const u8 as *const ::core::ffi::c_char,
        name,
        b"g-bus-type\0" as *const u8 as *const ::core::ffi::c_char,
        bus_type as ::core::ffi::c_int,
        b"g-object-path\0" as *const u8 as *const ::core::ffi::c_char,
        object_path,
        b"g-interface-name\0" as *const u8 as *const ::core::ffi::c_char,
        interface_name,
        NULL_0,
    ) as *mut GInitable;
    if !initable.is_null() {
        return initable as *mut ::core::ffi::c_void as *mut GDBusProxy;
    } else {
        return ::core::ptr::null_mut::<GDBusProxy>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_connection(
    mut proxy: *mut GDBusProxy,
) -> *mut GDBusConnection {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    return (*(*proxy).priv_0).connection;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_flags(
    mut proxy: *mut GDBusProxy,
) -> GDBusProxyFlags {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DBUS_PROXY_FLAGS_NONE;
    }
    return (*(*proxy).priv_0).flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_name(
    mut proxy: *mut GDBusProxy,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*proxy).priv_0).name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_name_owner(
    mut proxy: *mut GDBusProxy,
) -> *mut gchar {
    let mut ret: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    ret = safe_c2rust_g_strdup_inline((*(*proxy).priv_0).name_owner) as *mut gchar;
    g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_object_path(
    mut proxy: *mut GDBusProxy,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*proxy).priv_0).object_path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_interface_name(
    mut proxy: *mut GDBusProxy,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*proxy).priv_0).interface_name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_default_timeout(
    mut proxy: *mut GDBusProxy,
) -> gint {
    let mut ret: gint = 0;
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    ret = (*(*proxy).priv_0).timeout_msec;
    g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_set_default_timeout(
    mut proxy: *mut GDBusProxy,
    mut timeout_msec: gint,
) {
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if timeout_msec == -(1 as ::core::ffi::c_int) || timeout_msec >= 0 as ::core::ffi::c_int {
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
            b"timeout_msec == -1 || timeout_msec >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    if (*(*proxy).priv_0).timeout_msec != timeout_msec {
        (*(*proxy).priv_0).timeout_msec = timeout_msec;
        g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
        g_object_notify(
            proxy as *mut ::core::ffi::c_void as *mut GObject,
            b"g-default-timeout\0" as *const u8 as *const gchar,
        );
    } else {
        g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_get_interface_info(
    mut proxy: *mut GDBusProxy,
) -> *mut GDBusInterfaceInfo {
    let mut ret: *mut GDBusInterfaceInfo = ::core::ptr::null_mut::<GDBusInterfaceInfo>();
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusInterfaceInfo>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    ret = (*(*proxy).priv_0).expected_interface;
    g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_set_interface_info(
    mut proxy: *mut GDBusProxy,
    mut info: *mut GDBusInterfaceInfo,
) {
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    if !(*(*proxy).priv_0).expected_interface.is_null() {
        g_dbus_interface_info_cache_release((*(*proxy).priv_0).expected_interface);
        g_dbus_interface_info_unref((*(*proxy).priv_0).expected_interface);
    }
    (*(*proxy).priv_0).expected_interface = if !info.is_null() {
        g_dbus_interface_info_ref(info)
    } else {
        ::core::ptr::null_mut::<GDBusInterfaceInfo>()
    };
    if !(*(*proxy).priv_0).expected_interface.is_null() {
        g_dbus_interface_info_cache_build((*(*proxy).priv_0).expected_interface);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
}
unsafe extern "C" fn safe_c2rust_maybe_split_method_name(
    mut method_name: *const gchar,
    mut out_interface_name: *mut *mut gchar,
    mut out_method_name: *mut *const gchar,
) -> gboolean {
    let mut was_split: gboolean = 0;
    was_split = FALSE as gboolean;
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !out_interface_name.is_null() {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusproxy.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2478 as ::core::ffi::c_int,
            G_STRFUNC,
            b"out_interface_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if !out_method_name.is_null() {
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusproxy.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2479 as ::core::ffi::c_int,
            G_STRFUNC,
            b"out_method_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *out_interface_name = ::core::ptr::null_mut::<gchar>();
    *out_method_name = ::core::ptr::null::<gchar>();
    if !strchr(method_name as *const ::core::ffi::c_char, '.' as i32).is_null() {
        let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut last_dot: *mut gchar = ::core::ptr::null_mut::<gchar>();
        p = safe_c2rust_g_strdup_inline(method_name as *const ::core::ffi::c_char) as *mut gchar;
        last_dot = strrchr(p, '.' as i32) as *mut gchar;
        *last_dot = '\0' as i32 as gchar;
        *out_interface_name = p;
        *out_method_name = last_dot.offset(1 as ::core::ffi::c_int as isize);
        was_split = TRUE as gboolean;
    }
    return was_split;
}
unsafe extern "C" fn safe_c2rust_reply_data_free(mut data: *mut ReplyData) {
    g_variant_unref((*data).value);
    if !(*data).fd_list.is_null() {
        g_object_unref((*data).fd_list as gpointer);
    }
    g_slice_free1(
        ::core::mem::size_of::<ReplyData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_reply_cb(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut fd_list: *mut GUnixFDList = ::core::ptr::null_mut::<GUnixFDList>();
    error = ::core::ptr::null_mut::<GError>();
    value = g_dbus_connection_call_with_unix_fd_list_finish(
        connection,
        &raw mut fd_list,
        res,
        &raw mut error,
    );
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        let mut data: *mut ReplyData = ::core::ptr::null_mut::<ReplyData>();
        data = ({
            let mut __s: gsize = ::core::mem::size_of::<ReplyData>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            __p = g_slice_alloc(__s);
            memset(
                __p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                __s as size_t,
            );
            __p
        }) as *mut ReplyData;
        (*data).value = value;
        (*data).fd_list = fd_list;
        g_task_return_pointer(
            task,
            data as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut ReplyData) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_reply_data_free as unsafe extern "C" fn(*mut ReplyData) -> (),
            )),
        );
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_lookup_method_info(
    mut proxy: *mut GDBusProxy,
    mut method_name: *const gchar,
) -> *const GDBusMethodInfo {
    let mut info: *const GDBusMethodInfo = ::core::ptr::null::<GDBusMethodInfo>();
    if !(*(*proxy).priv_0).expected_interface.is_null() {
        info =
            g_dbus_interface_info_lookup_method((*(*proxy).priv_0).expected_interface, method_name);
    }
    return info;
}
unsafe extern "C" fn safe_c2rust_get_destination_for_call(
    mut proxy: *mut GDBusProxy,
) -> *const gchar {
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    ret = ::core::ptr::null::<gchar>();
    ret = (*(*proxy).priv_0).name_owner;
    if ret.is_null() {
        if !((*(*proxy).priv_0).flags as ::core::ffi::c_uint
            & G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0)
        {
            ret = (*(*proxy).priv_0).name;
        }
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_call_internal(
    mut proxy: *mut GDBusProxy,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut current_block: u64;
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut was_split: gboolean = 0;
    let mut split_interface_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut split_method_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut target_method_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut target_interface_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut destination: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut reply_type: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    let mut my_callback: GAsyncReadyCallback = None;
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if g_dbus_is_member_name(method_name) != 0 || g_dbus_is_interface_name(method_name) != 0 {
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
            b"g_dbus_is_member_name (method_name) || g_dbus_is_interface_name (method_name)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if parameters.is_null()
            || g_variant_is_of_type(
                parameters,
                b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
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
            b"parameters == NULL || g_variant_is_of_type (parameters, G_VARIANT_TYPE_TUPLE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if timeout_msec == -(1 as ::core::ffi::c_int) || timeout_msec >= 0 as ::core::ffi::c_int {
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
            b"timeout_msec == -1 || timeout_msec >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if fd_list.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = fd_list as *mut GTypeInstance;
                let mut __t: GType = g_unix_fd_list_get_type();
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
            b"fd_list == NULL || G_IS_UNIX_FD_LIST (fd_list)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    reply_type = ::core::ptr::null_mut::<GVariantType>();
    split_interface_name = ::core::ptr::null_mut::<gchar>();
    if callback.is_some() {
        my_callback = ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusConnection, *mut GAsyncResult, gpointer) -> ()>,
            GAsyncReadyCallback,
        >(Some(
            safe_c2rust_reply_cb
                as unsafe extern "C" fn(*mut GDBusConnection, *mut GAsyncResult, gpointer) -> (),
        ));
        task = g_task_new(proxy as gpointer, cancellable, callback, user_data);
        let mut _task: *mut GTask = task;
        g_task_set_source_tag(
            _task,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut GDBusProxy,
                        *const gchar,
                        *mut GVariant,
                        GDBusCallFlags,
                        gint,
                        *mut GUnixFDList,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
                >,
                gpointer,
            >(Some(
                safe_c2rust_g_dbus_proxy_call_internal
                    as unsafe extern "C" fn(
                        *mut GDBusProxy,
                        *const gchar,
                        *mut GVariant,
                        GDBusCallFlags,
                        gint,
                        *mut GUnixFDList,
                        *mut GCancellable,
                        GAsyncReadyCallback,
                        gpointer,
                    ) -> (),
            )),
        );
        if g_task_get_name(_task).is_null() {
            g_task_set_static_name(
                _task,
                b"g_dbus_proxy_call_internal\0" as *const u8 as *const gchar,
            );
        }
        let mut _task_0: *mut GTask = task;
        if 0 != 0 {
            g_task_set_static_name(
                _task_0,
                b"[gio] D-Bus proxy call\0" as *const u8 as *const gchar,
            );
        } else {
            g_task_set_name(
                _task_0,
                b"[gio] D-Bus proxy call\0" as *const u8 as *const gchar,
            );
        }
    } else {
        my_callback = None;
        task = ::core::ptr::null_mut::<GTask>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    was_split = safe_c2rust_maybe_split_method_name(
        method_name,
        &raw mut split_interface_name,
        &raw mut split_method_name,
    );
    target_method_name = if was_split != 0 {
        split_method_name
    } else {
        method_name
    };
    target_interface_name = if was_split != 0 {
        split_interface_name
    } else {
        (*(*proxy).priv_0).interface_name
    };
    if was_split == 0 {
        let mut expected_method_info: *const GDBusMethodInfo =
            ::core::ptr::null::<GDBusMethodInfo>();
        expected_method_info = safe_c2rust_lookup_method_info(proxy, target_method_name);
        if !expected_method_info.is_null() {
            reply_type = _g_dbus_compute_complete_signature((*expected_method_info).out_args);
        }
    }
    destination = ::core::ptr::null_mut::<gchar>();
    if !(*(*proxy).priv_0).name.is_null() {
        destination = safe_c2rust_g_strdup_inline(
            safe_c2rust_get_destination_for_call(proxy) as *const ::core::ffi::c_char
        ) as *mut gchar;
        if destination.is_null() {
            if !task.is_null() {
                g_task_return_new_error(
                    task,
                    g_io_error_quark(),
                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Cannot invoke method; proxy is for the well-known name %s without an owner, and proxy was constructed with the G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START flag\0"
                            as *const u8 as *const gchar,
                    ) as *const ::core::ffi::c_char,
                    (*(*proxy).priv_0).name,
                );
                g_object_unref(task as gpointer);
            }
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
            current_block = 15385661352742022147;
        } else {
            current_block = 2706659501864706830;
        }
    } else {
        current_block = 2706659501864706830;
    }
    match current_block {
        2706659501864706830 => {
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
            g_dbus_connection_call_with_unix_fd_list(
                (*(*proxy).priv_0).connection,
                destination,
                (*(*proxy).priv_0).object_path,
                target_interface_name,
                target_method_name,
                parameters,
                reply_type,
                flags,
                if timeout_msec == -(1 as ::core::ffi::c_int) {
                    (*(*proxy).priv_0).timeout_msec
                } else {
                    timeout_msec
                },
                fd_list,
                cancellable,
                my_callback,
                task as gpointer,
            );
        }
        _ => {}
    }
    if !reply_type.is_null() {
        g_variant_type_free(reply_type);
    }
    g_free(destination as gpointer);
    g_free(split_interface_name as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_call_finish_internal(
    mut proxy: *mut GDBusProxy,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut data: *mut ReplyData = ::core::ptr::null_mut::<ReplyData>();
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, proxy as gpointer) != 0 {
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
            b"g_task_is_valid (res, proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    value = ::core::ptr::null_mut::<GVariant>();
    data = g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut ReplyData;
    if !data.is_null() {
        value = g_variant_ref((*data).value);
        if !out_fd_list.is_null() {
            *out_fd_list = (if !(*data).fd_list.is_null() {
                g_object_ref((*data).fd_list as gpointer) as *mut GUnixFDList
            } else {
                ::core::ptr::null_mut::<GUnixFDList>()
            }) as *mut GUnixFDList;
        }
        safe_c2rust_reply_data_free(data);
    }
    return value;
}
unsafe extern "C" fn safe_c2rust_g_dbus_proxy_call_sync_internal(
    mut proxy: *mut GDBusProxy,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut fd_list: *mut GUnixFDList,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut current_block: u64;
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut was_split: gboolean = 0;
    let mut split_interface_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut split_method_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut target_method_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut target_interface_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut destination: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut reply_type: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if g_dbus_is_member_name(method_name) != 0 || g_dbus_is_interface_name(method_name) != 0 {
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
            b"g_dbus_is_member_name (method_name) || g_dbus_is_interface_name (method_name)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if parameters.is_null()
            || g_variant_is_of_type(
                parameters,
                b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
            ) != 0
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
            b"parameters == NULL || g_variant_is_of_type (parameters, G_VARIANT_TYPE_TUPLE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if timeout_msec == -(1 as ::core::ffi::c_int) || timeout_msec >= 0 as ::core::ffi::c_int {
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
            b"timeout_msec == -1 || timeout_msec >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if fd_list.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = fd_list as *mut GTypeInstance;
                let mut __t: GType = g_unix_fd_list_get_type();
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
            b"fd_list == NULL || G_IS_UNIX_FD_LIST (fd_list)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    reply_type = ::core::ptr::null_mut::<GVariantType>();
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    was_split = safe_c2rust_maybe_split_method_name(
        method_name,
        &raw mut split_interface_name,
        &raw mut split_method_name,
    );
    target_method_name = if was_split != 0 {
        split_method_name
    } else {
        method_name
    };
    target_interface_name = if was_split != 0 {
        split_interface_name
    } else {
        (*(*proxy).priv_0).interface_name
    };
    if was_split == 0 {
        let mut expected_method_info: *const GDBusMethodInfo =
            ::core::ptr::null::<GDBusMethodInfo>();
        expected_method_info = safe_c2rust_lookup_method_info(proxy, target_method_name);
        if !expected_method_info.is_null() {
            reply_type = _g_dbus_compute_complete_signature((*expected_method_info).out_args);
        }
    }
    destination = ::core::ptr::null_mut::<gchar>();
    if !(*(*proxy).priv_0).name.is_null() {
        destination = safe_c2rust_g_strdup_inline(
            safe_c2rust_get_destination_for_call(proxy) as *const ::core::ffi::c_char
        ) as *mut gchar;
        if destination.is_null() {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Cannot invoke method; proxy is for the well-known name %s without an owner, and proxy was constructed with the G_DBUS_PROXY_FLAGS_DO_NOT_AUTO_START flag\0"
                        as *const u8 as *const gchar,
                ),
                (*(*proxy).priv_0).name,
            );
            ret = ::core::ptr::null_mut::<GVariant>();
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
            current_block = 15948930208847934841;
        } else {
            current_block = 3689906465960840878;
        }
    } else {
        current_block = 3689906465960840878;
    }
    match current_block {
        3689906465960840878 => {
            g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
            ret = g_dbus_connection_call_with_unix_fd_list_sync(
                (*(*proxy).priv_0).connection,
                destination,
                (*(*proxy).priv_0).object_path,
                target_interface_name,
                target_method_name,
                parameters,
                reply_type,
                flags,
                if timeout_msec == -(1 as ::core::ffi::c_int) {
                    (*(*proxy).priv_0).timeout_msec
                } else {
                    timeout_msec
                },
                fd_list,
                out_fd_list,
                cancellable,
                error,
            );
        }
        _ => {}
    }
    if !reply_type.is_null() {
        g_variant_type_free(reply_type);
    }
    g_free(destination as gpointer);
    g_free(split_interface_name as gpointer);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_call(
    mut proxy: *mut GDBusProxy,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    safe_c2rust_g_dbus_proxy_call_internal(
        proxy,
        method_name,
        parameters,
        flags,
        timeout_msec,
        ::core::ptr::null_mut::<GUnixFDList>(),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_call_finish(
    mut proxy: *mut GDBusProxy,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    return safe_c2rust_g_dbus_proxy_call_finish_internal(
        proxy,
        ::core::ptr::null_mut::<*mut GUnixFDList>(),
        res,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_call_sync(
    mut proxy: *mut GDBusProxy,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    return safe_c2rust_g_dbus_proxy_call_sync_internal(
        proxy,
        method_name,
        parameters,
        flags,
        timeout_msec,
        ::core::ptr::null_mut::<GUnixFDList>(),
        ::core::ptr::null_mut::<*mut GUnixFDList>(),
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_call_with_unix_fd_list(
    mut proxy: *mut GDBusProxy,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    safe_c2rust_g_dbus_proxy_call_internal(
        proxy,
        method_name,
        parameters,
        flags,
        timeout_msec,
        fd_list,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_call_with_unix_fd_list_finish(
    mut proxy: *mut GDBusProxy,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    return safe_c2rust_g_dbus_proxy_call_finish_internal(proxy, out_fd_list, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_proxy_call_with_unix_fd_list_sync(
    mut proxy: *mut GDBusProxy,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut fd_list: *mut GUnixFDList,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    return safe_c2rust_g_dbus_proxy_call_sync_internal(
        proxy,
        method_name,
        parameters,
        flags,
        timeout_msec,
        fd_list,
        out_fd_list,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust__g_dbus_proxy_get_info(
    mut interface: *mut GDBusInterface,
) -> *mut GDBusInterfaceInfo {
    let mut proxy: *mut GDBusProxy = interface as *mut ::core::ffi::c_void as *mut GDBusProxy;
    return safe_c2rust_g_dbus_proxy_get_interface_info(proxy);
}
unsafe extern "C" fn safe_c2rust__g_dbus_proxy_get_object(
    mut interface: *mut GDBusInterface,
) -> *mut GDBusObject {
    let mut proxy: *mut GDBusProxy = interface as *mut ::core::ffi::c_void as *mut GDBusProxy;
    return (*(*proxy).priv_0).object;
}
unsafe extern "C" fn safe_c2rust__g_dbus_proxy_dup_object(
    mut interface: *mut GDBusInterface,
) -> *mut GDBusObject {
    let mut proxy: *mut GDBusProxy = interface as *mut ::core::ffi::c_void as *mut GDBusProxy;
    let mut ret: *mut GDBusObject = ::core::ptr::null_mut::<GDBusObject>();
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    if !(*(*proxy).priv_0).object.is_null() {
        ret = g_object_ref((*(*proxy).priv_0).object as gpointer) as *mut GDBusObject
            as *mut GDBusObject;
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust__g_dbus_proxy_set_object(
    mut interface: *mut GDBusInterface,
    mut object: *mut GDBusObject,
) {
    let mut proxy: *mut GDBusProxy = interface as *mut ::core::ffi::c_void as *mut GDBusProxy;
    g_mutex_lock(&raw mut safe_c2rust_g__properties_lock_lock);
    if !(*(*proxy).priv_0).object.is_null() {
        g_object_remove_weak_pointer(
            (*(*proxy).priv_0).object as *mut ::core::ffi::c_void as *mut GObject,
            &raw mut (*(*proxy).priv_0).object as *mut gpointer,
        );
    }
    (*(*proxy).priv_0).object = object;
    if !(*(*proxy).priv_0).object.is_null() {
        g_object_add_weak_pointer(
            (*(*proxy).priv_0).object as *mut ::core::ffi::c_void as *mut GObject,
            &raw mut (*(*proxy).priv_0).object as *mut gpointer,
        );
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__properties_lock_lock);
}
unsafe extern "C" fn safe_c2rust_dbus_interface_iface_init(
    mut dbus_interface_iface: *mut GDBusInterfaceIface,
) {
    (*dbus_interface_iface).get_info = Some(
        safe_c2rust__g_dbus_proxy_get_info
            as unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusInterfaceInfo,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusInterfaceInfo>;
    (*dbus_interface_iface).get_object = Some(
        safe_c2rust__g_dbus_proxy_get_object
            as unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject>;
    (*dbus_interface_iface).dup_object = Some(
        safe_c2rust__g_dbus_proxy_dup_object
            as unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject,
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterface) -> *mut GDBusObject>;
    (*dbus_interface_iface).set_object = Some(
        safe_c2rust__g_dbus_proxy_set_object
            as unsafe extern "C" fn(*mut GDBusInterface, *mut GDBusObject) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusInterface, *mut GDBusObject) -> ()>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
