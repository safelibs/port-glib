use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::c2rust_bitfields;
use ::core::arch::asm;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GMainLoop;
    pub type _GSourcePrivate;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GAsyncInitable;
    pub type _GCancellablePrivate;
    pub type _GIOStreamPrivate;
    pub type _GInitable;
    pub type _GSocketPrivate;
    pub type _GSocketConnectionPrivate;
    pub type _GTask;
    pub type _GCredentials;
    pub type _GUnixFDListPrivate;
    pub type _GDBusMessage;
    pub type _GDBusAuthObserver;
    pub type GDBusWorker;
    pub type _GDBusAuthPrivate;
    pub type _GDBusMethodInvocation;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_array_new(zero_terminated: gboolean, clear_: gboolean, element_size: guint)
        -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_remove_index(array: *mut GPtrArray, index_: guint) -> gpointer;
    fn g_ptr_array_remove_index_fast(array: *mut GPtrArray, index_: guint) -> gpointer;
    fn g_ptr_array_remove(array: *mut GPtrArray, data: gpointer) -> gboolean;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_copy(error: *const GError) -> *mut GError;
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_clear_error(err: *mut *mut GError);
    fn g_thread_self() -> *mut GThread;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_trylock(mutex: *mut GMutex) -> gboolean;
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn raise(__sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_replace(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_steal_all_keys(hash_table: *mut GHashTable) -> *mut GPtrArray;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_foreach_remove(
        hash_table: *mut GHashTable,
        func: GHRFunc,
        user_data: gpointer,
    ) -> guint;
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
    fn g_direct_hash(v: gconstpointer) -> guint;
    fn g_direct_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_main_context_new() -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_push_thread_default(context: *mut GMainContext);
    fn g_main_context_pop_thread_default(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_main_loop_new(context: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
    fn g_main_loop_run(loop_0: *mut GMainLoop);
    fn g_main_loop_quit(loop_0: *mut GMainLoop);
    fn g_main_loop_unref(loop_0: *mut GMainLoop);
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_destroy(source: *mut GSource);
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_idle_source_new() -> *mut GSource;
    fn g_timeout_source_new(interval: guint) -> *mut GSource;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strjoin(separator: *const gchar, ...) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_contains(strv: *const *const gchar, str: *const gchar) -> gboolean;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_prepend_c(string: *mut GString, c: gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_variant_type_free(type_0: *mut GVariantType);
    fn g_variant_type_copy(type_0: *const GVariantType) -> *mut GVariantType;
    fn g_variant_type_dup_string(type_0: *const GVariantType) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_take_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_is_object_path(string: *const gchar) -> gboolean;
    fn g_variant_get_child(value: *mut GVariant, index_: gsize, format_string: *const gchar, ...);
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_open(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_close(builder: *mut GVariantBuilder);
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
    fn g_print(format: *const gchar, ...);
    fn g_ref_count_init(rc: *mut grefcount);
    fn g_ref_count_inc(rc: *mut grefcount);
    fn g_ref_count_dec(rc: *mut grefcount) -> gboolean;
    fn g_atomic_ref_count_inc(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_dec(arc: *mut gatomicrefcount) -> gboolean;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_assertion_message(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        message: *const ::core::ffi::c_char,
    );
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_assertion_message_error(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        error: *const GError,
        error_domain: GQuark,
        error_code: ::core::ffi::c_int,
    );
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn g_closure_ref(closure: *mut GClosure) -> *mut GClosure;
    fn g_closure_sink(closure: *mut GClosure);
    fn g_closure_unref(closure: *mut GClosure);
    fn g_closure_set_marshal(closure: *mut GClosure, marshal: GClosureMarshal);
    fn g_closure_invoke(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
    );
    fn g_cclosure_marshal_generic(
        closure: *mut GClosure,
        return_gvalue: *mut GValue,
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
    fn g_error_get_type() -> GType;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_get_data(object: *mut GObject, key: *const gchar) -> gpointer;
    fn g_object_set_data(object: *mut GObject, key: *const gchar, data: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> gpointer;
    fn g_weak_ref_set(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
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
    fn g_param_spec_object(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        object_type: GType,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_value_set_variant(value: *mut GValue, variant: *mut GVariant);
    fn g_value_get_variant(value: *const GValue) -> *mut GVariant;
    fn _g_dbus_auth_new(stream: *mut GIOStream) -> *mut GDBusAuth;
    fn _g_dbus_auth_run_server(
        auth: *mut GDBusAuth,
        observer: *mut GDBusAuthObserver,
        guid: *const gchar,
        allow_anonymous: gboolean,
        require_same_user: gboolean,
        offered_capabilities: GDBusCapabilityFlags,
        out_negotiated_capabilities: *mut GDBusCapabilityFlags,
        out_received_credentials: *mut *mut GCredentials,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn _g_dbus_auth_run_client(
        auth: *mut GDBusAuth,
        observer: *mut GDBusAuthObserver,
        conn_flags: GDBusConnectionFlags,
        offered_capabilities: GDBusCapabilityFlags,
        out_negotiated_capabilities: *mut GDBusCapabilityFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_dbus_is_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_unique_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_member_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_interface_name(string: *const gchar) -> gboolean;
    fn g_dbus_address_get_stream_sync(
        address: *const gchar,
        out_guid: *mut *mut gchar,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GIOStream;
    fn g_dbus_address_get_for_bus_sync(
        bus_type: GBusType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_dbus_message_get_type() -> GType;
    fn g_dbus_message_new_signal(
        path: *const gchar,
        interface_: *const gchar,
        signal: *const gchar,
    ) -> *mut GDBusMessage;
    fn g_dbus_message_new_method_call(
        name: *const gchar,
        path: *const gchar,
        interface_: *const gchar,
        method: *const gchar,
    ) -> *mut GDBusMessage;
    fn g_dbus_message_new_method_reply(method_call_message: *mut GDBusMessage)
        -> *mut GDBusMessage;
    fn g_dbus_message_new_method_error(
        method_call_message: *mut GDBusMessage,
        error_name: *const gchar,
        error_message_format: *const gchar,
        ...
    ) -> *mut GDBusMessage;
    fn g_dbus_message_new_method_error_literal(
        method_call_message: *mut GDBusMessage,
        error_name: *const gchar,
        error_message: *const gchar,
    ) -> *mut GDBusMessage;
    fn g_dbus_message_get_locked(message: *mut GDBusMessage) -> gboolean;
    fn g_dbus_message_lock(message: *mut GDBusMessage);
    fn g_dbus_message_get_message_type(message: *mut GDBusMessage) -> GDBusMessageType;
    fn g_dbus_message_get_flags(message: *mut GDBusMessage) -> GDBusMessageFlags;
    fn g_dbus_message_set_flags(message: *mut GDBusMessage, flags: GDBusMessageFlags);
    fn g_dbus_message_get_serial(message: *mut GDBusMessage) -> guint32;
    fn g_dbus_message_set_serial(message: *mut GDBusMessage, serial: guint32);
    fn g_dbus_message_set_header(
        message: *mut GDBusMessage,
        header_field: GDBusMessageHeaderField,
        value: *mut GVariant,
    );
    fn g_dbus_message_get_body(message: *mut GDBusMessage) -> *mut GVariant;
    fn g_dbus_message_set_body(message: *mut GDBusMessage, body: *mut GVariant);
    fn g_dbus_message_get_unix_fd_list(message: *mut GDBusMessage) -> *mut GUnixFDList;
    fn g_dbus_message_set_unix_fd_list(message: *mut GDBusMessage, fd_list: *mut GUnixFDList);
    fn g_dbus_message_get_reply_serial(message: *mut GDBusMessage) -> guint32;
    fn g_dbus_message_get_interface(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_get_member(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_get_path(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_get_sender(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_get_error_name(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_get_signature(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_get_arg0(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_get_arg0_path(message: *mut GDBusMessage) -> *const gchar;
    fn g_dbus_message_to_blob(
        message: *mut GDBusMessage,
        out_size: *mut gsize,
        capabilities: GDBusCapabilityFlags,
        error: *mut *mut GError,
    ) -> *mut guchar;
    fn g_dbus_message_to_gerror(message: *mut GDBusMessage, error: *mut *mut GError) -> gboolean;
    fn g_dbus_error_quark() -> GQuark;
    fn g_dbus_error_encode_gerror(error: *const GError) -> *mut gchar;
    fn g_dbus_connection_flags_get_type() -> GType;
    fn g_dbus_capability_flags_get_type() -> GType;
    fn g_dbus_interface_info_lookup_method(
        info: *mut GDBusInterfaceInfo,
        name: *const gchar,
    ) -> *mut GDBusMethodInfo;
    fn g_dbus_interface_info_lookup_property(
        info: *mut GDBusInterfaceInfo,
        name: *const gchar,
    ) -> *mut GDBusPropertyInfo;
    fn g_dbus_interface_info_cache_build(info: *mut GDBusInterfaceInfo);
    fn g_dbus_interface_info_cache_release(info: *mut GDBusInterfaceInfo);
    fn g_dbus_interface_info_generate_xml(
        info: *mut GDBusInterfaceInfo,
        indent: guint,
        string_builder: *mut GString,
    );
    fn g_dbus_interface_info_ref(info: *mut GDBusInterfaceInfo) -> *mut GDBusInterfaceInfo;
    fn g_dbus_interface_info_unref(info: *mut GDBusInterfaceInfo);
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
    fn g_dbus_method_invocation_get_message(
        invocation: *mut GDBusMethodInvocation,
    ) -> *mut GDBusMessage;
    fn g_dbus_method_invocation_get_parameters(
        invocation: *mut GDBusMethodInvocation,
    ) -> *mut GVariant;
    fn g_dbus_method_invocation_get_user_data(invocation: *mut GDBusMethodInvocation) -> gpointer;
    fn _g_dbus_worker_new(
        stream: *mut GIOStream,
        capabilities: GDBusCapabilityFlags,
        initially_frozen: gboolean,
        message_received_callback: GDBusWorkerMessageReceivedCallback,
        message_about_to_be_sent_callback: GDBusWorkerMessageAboutToBeSentCallback,
        disconnected_callback: GDBusWorkerDisconnectedCallback,
        user_data: gpointer,
    ) -> *mut GDBusWorker;
    fn _g_dbus_worker_send_message(
        worker: *mut GDBusWorker,
        message: *mut GDBusMessage,
        blob: *mut gchar,
        blob_len: gsize,
    );
    fn _g_dbus_worker_stop(worker: *mut GDBusWorker);
    fn _g_dbus_worker_unfreeze(worker: *mut GDBusWorker);
    fn _g_dbus_worker_flush_sync(
        worker: *mut GDBusWorker,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn _g_dbus_worker_close(worker: *mut GDBusWorker, task: *mut GTask);
    fn _g_dbus_initialize();
    fn _g_dbus_debug_call() -> gboolean;
    fn _g_dbus_debug_signal() -> gboolean;
    fn _g_dbus_debug_incoming() -> gboolean;
    fn _g_dbus_debug_emission() -> gboolean;
    fn _g_dbus_debug_print_lock();
    fn _g_dbus_debug_print_unlock();
    fn _g_dbus_compute_complete_signature(args: *mut *mut GDBusArgInfo) -> *mut GVariantType;
    fn _g_dbus_get_machine_id(error: *mut *mut GError) -> *mut gchar;
    fn _g_dbus_method_invocation_new(
        sender: *const gchar,
        object_path: *const gchar,
        interface_name: *const gchar,
        method_name: *const gchar,
        method_info: *const GDBusMethodInfo,
        property_info: *const GDBusPropertyInfo,
        connection: *mut GDBusConnection,
        message: *mut GDBusMessage,
        parameters: *mut GVariant,
        user_data: gpointer,
    ) -> *mut GDBusMethodInvocation;
    fn g_dbus_auth_observer_get_type() -> GType;
    fn g_initable_get_type() -> GType;
    fn g_initable_init(
        initable: *mut GInitable,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_initable_new(
        object_type: GType,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
        first_property_name: *const gchar,
        ...
    ) -> gpointer;
    fn g_async_initable_get_type() -> GType;
    fn g_async_initable_init_async(
        initable: *mut GAsyncInitable,
        io_priority: ::core::ffi::c_int,
        cancellable: *mut GCancellable,
        callback: GAsyncReadyCallback,
        user_data: gpointer,
    );
    fn g_async_initable_init_finish(
        initable: *mut GAsyncInitable,
        res: *mut GAsyncResult,
        error: *mut *mut GError,
    ) -> gboolean;
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
    fn g_cancellable_connect(
        cancellable: *mut GCancellable,
        callback: GCallback,
        data: gpointer,
        data_destroy_func: GDestroyNotify,
    ) -> gulong;
    fn g_cancellable_disconnect(cancellable: *mut GCancellable, handler_id: gulong);
    fn g_io_error_quark() -> GQuark;
    fn g_io_stream_get_type() -> GType;
    fn g_async_result_get_type() -> GType;
    fn g_async_result_get_source_object(res: *mut GAsyncResult) -> *mut GObject;
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
    fn g_task_get_source_object(task: *mut GTask) -> gpointer;
    fn g_task_get_task_data(task: *mut GTask) -> gpointer;
    fn g_task_get_cancellable(task: *mut GTask) -> *mut GCancellable;
    fn g_task_get_name(task: *mut GTask) -> *const gchar;
    fn g_task_is_valid(result: gpointer, source_object: gpointer) -> gboolean;
    fn g_task_run_in_thread(task: *mut GTask, task_func: GTaskThreadFunc);
    fn g_task_attach_source(task: *mut GTask, source: *mut GSource, callback: GSourceFunc);
    fn g_task_return_pointer(task: *mut GTask, result: gpointer, result_destroy: GDestroyNotify);
    fn g_task_return_boolean(task: *mut GTask, result: gboolean);
    fn g_task_return_error(task: *mut GTask, error: *mut GError);
    fn g_task_return_new_error_literal(
        task: *mut GTask,
        domain: GQuark,
        code: gint,
        message: *const ::core::ffi::c_char,
    );
    fn g_task_return_error_if_cancelled(task: *mut GTask) -> gboolean;
    fn g_task_propagate_pointer(task: *mut GTask, error: *mut *mut GError) -> gpointer;
    fn g_task_propagate_boolean(task: *mut GTask, error: *mut *mut GError) -> gboolean;
    fn _g_cclosure_marshal_VOID__BOOLEAN_BOXED(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__BOOLEAN_BOXEDv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn g_socket_set_blocking(socket: *mut GSocket, blocking: gboolean);
    fn g_socket_connection_get_type() -> GType;
    fn g_socket_connection_get_socket(connection: *mut GSocketConnection) -> *mut GSocket;
    fn g_unix_fd_list_get_type() -> GType;
    fn g_unix_connection_get_type() -> GType;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type grefcount = gint;
pub type gatomicrefcount = gint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
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
pub type GThreadFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThread {
    pub func: GThreadFunc,
    pub data: gpointer,
    pub joinable: gboolean,
    pub priority: GThreadPriority,
}
pub type GThreadPriority = ::core::ffi::c_uint;
pub const G_THREAD_PRIORITY_URGENT: GThreadPriority = 3;
pub const G_THREAD_PRIORITY_HIGH: GThreadPriority = 2;
pub const G_THREAD_PRIORITY_NORMAL: GThreadPriority = 1;
pub const G_THREAD_PRIORITY_LOW: GThreadPriority = 0;
pub type GThread = _GThread;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GData = _GData;
pub type GHashTable = _GHashTable;
pub type GHRFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
pub type GDBusCapabilityFlags = ::core::ffi::c_uint;
pub const G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING: GDBusCapabilityFlags = 1;
pub const G_DBUS_CAPABILITY_FLAGS_NONE: GDBusCapabilityFlags = 0;
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GDBusMessageType = ::core::ffi::c_uint;
pub const G_DBUS_MESSAGE_TYPE_SIGNAL: GDBusMessageType = 4;
pub const G_DBUS_MESSAGE_TYPE_ERROR: GDBusMessageType = 3;
pub const G_DBUS_MESSAGE_TYPE_METHOD_RETURN: GDBusMessageType = 2;
pub const G_DBUS_MESSAGE_TYPE_METHOD_CALL: GDBusMessageType = 1;
pub const G_DBUS_MESSAGE_TYPE_INVALID: GDBusMessageType = 0;
pub type GDBusMessageFlags = ::core::ffi::c_uint;
pub const G_DBUS_MESSAGE_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusMessageFlags = 4;
pub const G_DBUS_MESSAGE_FLAGS_NO_AUTO_START: GDBusMessageFlags = 2;
pub const G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED: GDBusMessageFlags = 1;
pub const G_DBUS_MESSAGE_FLAGS_NONE: GDBusMessageFlags = 0;
pub type GDBusMessageHeaderField = ::core::ffi::c_uint;
pub const G_DBUS_MESSAGE_HEADER_FIELD_NUM_UNIX_FDS: GDBusMessageHeaderField = 9;
pub const G_DBUS_MESSAGE_HEADER_FIELD_SIGNATURE: GDBusMessageHeaderField = 8;
pub const G_DBUS_MESSAGE_HEADER_FIELD_SENDER: GDBusMessageHeaderField = 7;
pub const G_DBUS_MESSAGE_HEADER_FIELD_DESTINATION: GDBusMessageHeaderField = 6;
pub const G_DBUS_MESSAGE_HEADER_FIELD_REPLY_SERIAL: GDBusMessageHeaderField = 5;
pub const G_DBUS_MESSAGE_HEADER_FIELD_ERROR_NAME: GDBusMessageHeaderField = 4;
pub const G_DBUS_MESSAGE_HEADER_FIELD_MEMBER: GDBusMessageHeaderField = 3;
pub const G_DBUS_MESSAGE_HEADER_FIELD_INTERFACE: GDBusMessageHeaderField = 2;
pub const G_DBUS_MESSAGE_HEADER_FIELD_PATH: GDBusMessageHeaderField = 1;
pub const G_DBUS_MESSAGE_HEADER_FIELD_INVALID: GDBusMessageHeaderField = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GDBusSubtreeFlags = ::core::ffi::c_uint;
pub const G_DBUS_SUBTREE_FLAGS_DISPATCH_TO_UNENUMERATED_NODES: GDBusSubtreeFlags = 1;
pub const G_DBUS_SUBTREE_FLAGS_NONE: GDBusSubtreeFlags = 0;
pub type GDBusSignalFlags = ::core::ffi::c_uint;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH: GDBusSignalFlags = 4;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE: GDBusSignalFlags = 2;
pub const G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE: GDBusSignalFlags = 1;
pub const G_DBUS_SIGNAL_FLAGS_NONE: GDBusSignalFlags = 0;
pub type GDBusSendMessageFlags = ::core::ffi::c_uint;
pub const G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL: GDBusSendMessageFlags = 1;
pub const G_DBUS_SEND_MESSAGE_FLAGS_NONE: GDBusSendMessageFlags = 0;
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
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GInitable = _GInitable;
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
pub type GTask = _GTask;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GCredentials = _GCredentials;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
pub type GDBusMessage = _GDBusMessage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusConnection {
    pub parent_instance: GObject,
    pub lock: GMutex,
    pub init_lock: GMutex,
    pub machine_id: *mut gchar,
    pub stream: *mut GIOStream,
    pub auth: *mut GDBusAuth,
    pub last_serial: guint32,
    pub worker: *mut GDBusWorker,
    pub bus_unique_name: *mut gchar,
    pub guid: *mut gchar,
    pub atomic_flags: gint,
    pub initialization_error: *mut GError,
    pub main_context_at_construction: *mut GMainContext,
    pub address: *mut gchar,
    pub flags: GDBusConnectionFlags,
    pub map_method_serial_to_task: *mut GHashTable,
    pub map_method_serial_to_name_watcher: *mut GHashTable,
    pub map_rule_to_signal_data: *mut GHashTable,
    pub map_id_to_signal_data: *mut GHashTable,
    pub map_sender_unique_name_to_signal_data_array: *mut GHashTable,
    pub map_object_path_to_eo: *mut GHashTable,
    pub map_id_to_ei: *mut GHashTable,
    pub map_object_path_to_es: *mut GHashTable,
    pub map_id_to_es: *mut GHashTable,
    pub map_thread_to_last_serial: *mut GHashTable,
    pub filters: *mut GPtrArray,
    pub capabilities: GDBusCapabilityFlags,
    pub authentication_observer: *mut GDBusAuthObserver,
    pub credentials: *mut GCredentials,
    pub finalizing: gboolean,
}
pub type GDBusAuthObserver = _GDBusAuthObserver;
pub type GDBusAuth = _GDBusAuth;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAuth {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusAuthPrivate,
}
pub type GDBusAuthPrivate = _GDBusAuthPrivate;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusSubtreeVTable {
    pub enumerate: GDBusSubtreeEnumerateFunc,
    pub introspect: GDBusSubtreeIntrospectFunc,
    pub dispatch: GDBusSubtreeDispatchFunc,
    pub padding: [gpointer; 8],
}
pub type GDBusSubtreeDispatchFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        *const gchar,
        *mut gpointer,
        gpointer,
    ) -> *const GDBusInterfaceVTable,
>;
pub type GDBusSubtreeIntrospectFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        *const gchar,
        gpointer,
    ) -> *mut *mut GDBusInterfaceInfo,
>;
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
pub type GDBusSubtreeEnumerateFunc = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *const gchar,
        *const gchar,
        gpointer,
    ) -> *mut *mut gchar,
>;
pub type GDBusSubtreeVTable = _GDBusSubtreeVTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportedSubtree {
    pub refcount: gint,
    pub id: guint,
    pub object_path: *mut gchar,
    pub connection: *mut GDBusConnection,
    pub vtable: *mut GDBusSubtreeVTable,
    pub flags: GDBusSubtreeFlags,
    pub context: *mut GMainContext,
    pub user_data: gpointer,
    pub user_data_free_func: GDestroyNotify,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallDestroyNotifyData {
    pub callback: GDestroyNotify,
    pub user_data: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportedObject {
    pub object_path: *mut gchar,
    pub connection: *mut GDBusConnection,
    pub map_if_name_to_ei: *mut GHashTable,
}
pub type GDBusConnectionClass = _GDBusConnectionClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusConnectionClass {
    pub parent_class: GObjectClass,
    pub closed: Option<unsafe extern "C" fn(*mut GDBusConnection, gboolean, *mut GError) -> ()>,
}
pub const CLOSED_SIGNAL: C2RustUnnamed_6 = 0;
pub const PROP_AUTHENTICATION_OBSERVER: C2RustUnnamed_7 = 9;
pub const PROP_CAPABILITY_FLAGS: C2RustUnnamed_7 = 8;
pub const PROP_EXIT_ON_CLOSE: C2RustUnnamed_7 = 7;
pub const PROP_CLOSED: C2RustUnnamed_7 = 6;
pub const PROP_UNIQUE_NAME: C2RustUnnamed_7 = 5;
pub const PROP_GUID: C2RustUnnamed_7 = 4;
pub const PROP_FLAGS: C2RustUnnamed_7 = 3;
pub const PROP_ADDRESS: C2RustUnnamed_7 = 2;
pub const PROP_STREAM: C2RustUnnamed_7 = 1;
pub const FLAG_INITIALIZED: C2RustUnnamed_5 = 1;
pub const FLAG_EXIT_ON_CLOSE: C2RustUnnamed_5 = 2;
pub const FLAG_CLOSED: C2RustUnnamed_5 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FilterData {
    pub id: guint,
    pub ref_count: guint,
    pub filter_function: GDBusMessageFilterFunction,
    pub user_data: gpointer,
    pub user_data_free_func: GDestroyNotify,
    pub context: *mut GMainContext,
}
pub type GDBusMessageFilterFunction = Option<
    unsafe extern "C" fn(
        *mut GDBusConnection,
        *mut GDBusMessage,
        gboolean,
        gpointer,
    ) -> *mut GDBusMessage,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalData {
    pub rule: *mut gchar,
    pub sender: *mut gchar,
    pub interface_name: *mut gchar,
    pub member: *mut gchar,
    pub object_path: *mut gchar,
    pub arg0: *mut gchar,
    pub flags: GDBusSignalFlags,
    pub subscribers: *mut GPtrArray,
    pub shared_name_watcher: *mut SignalData,
    pub watched_name: *mut WatchedName,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WatchedName {
    pub ref_count: grefcount,
    pub owner: *mut gchar,
    pub get_name_owner_serial: guint32,
}
pub type CheckUnclosedFlags = ::core::ffi::c_uint;
pub const MAY_BE_UNINITIALIZED: CheckUnclosedFlags = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalSubscriber {
    pub ref_count: gatomicrefcount,
    pub callback: GDBusSignalCallback,
    pub user_data: gpointer,
    pub user_data_free_func: GDestroyNotify,
    pub id: guint,
    pub context: *mut GMainContext,
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
pub struct SendMessageSyncData {
    pub res: *mut GAsyncResult,
    pub context: *mut GMainContext,
    pub loop_0: *mut GMainLoop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SendMessageData {
    pub serial: guint32,
    pub cancellable_handler_id: gulong,
    pub cancelled_idle_source: *mut GSource,
    pub timeout_source: *mut GSource,
    pub delivered: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EmitClosedData {
    pub connection: *mut GDBusConnection,
    pub error: *mut GError,
    pub remote_peer_vanished: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubtreeDeferredData {
    pub message: *mut GDBusMessage,
    pub es: *mut ExportedSubtree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PropertyGetAllData {
    pub connection: *mut GDBusConnection,
    pub message: *mut GDBusMessage,
    pub user_data: gpointer,
    pub vtable: *const GDBusInterfaceVTable,
    pub interface_info: *mut GDBusInterfaceInfo,
    pub registration_id: guint,
    pub subtree_registration_id: guint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExportedInterface {
    pub eo: *mut ExportedObject,
    pub refcount: gint,
    pub id: guint,
    pub interface_name: *mut gchar,
    pub vtable: *mut GDBusInterfaceVTable,
    pub interface_info: *mut GDBusInterfaceInfo,
    pub context: *mut GMainContext,
    pub user_data: gpointer,
    pub user_data_free_func: GDestroyNotify,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PropertyData {
    pub connection: *mut GDBusConnection,
    pub message: *mut GDBusMessage,
    pub user_data: gpointer,
    pub property_name: *const gchar,
    pub vtable: *const GDBusInterfaceVTable,
    pub interface_info: *mut GDBusInterfaceInfo,
    pub property_info: *const GDBusPropertyInfo,
    pub registration_id: guint,
    pub subtree_registration_id: guint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SignalInstance {
    pub subscriber: *mut SignalSubscriber,
    pub message: *mut GDBusMessage,
    pub connection: *mut GDBusConnection,
    pub sender: *const gchar,
    pub path: *const gchar,
    pub interface: *const gchar,
    pub member: *const gchar,
}
pub type GDBusWorkerDisconnectedCallback =
    Option<unsafe extern "C" fn(*mut GDBusWorker, gboolean, *mut GError, gpointer) -> ()>;
pub type GDBusWorkerMessageAboutToBeSentCallback = Option<
    unsafe extern "C" fn(*mut GDBusWorker, *mut GDBusMessage, gpointer) -> *mut GDBusMessage,
>;
pub type GDBusWorkerMessageReceivedCallback =
    Option<unsafe extern "C" fn(*mut GDBusWorker, *mut GDBusMessage, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SyncCloseData {
    pub loop_0: *mut GMainLoop,
    pub result: *mut GAsyncResult,
}
pub type GTaskThreadFunc =
    Option<unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallState {
    pub reply_type: *mut GVariantType,
    pub method_name: *mut gchar,
    pub fd_list: *mut GUnixFDList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RegisterObjectData {
    pub method_call_closure: *mut GClosure,
    pub get_property_closure: *mut GClosure,
    pub set_property_closure: *mut GClosure,
}
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_6 = 1;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_7 = 0;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
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
pub const G_PRIORITY_HIGH: ::core::ffi::c_int = -(100 as ::core::ffi::c_int);
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn safe_c2rust_g_set_str(
    mut str_pointer: *mut *mut ::core::ffi::c_char,
    mut new_str: *const ::core::ffi::c_char,
) -> gboolean {
    let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if *str_pointer == new_str as *mut ::core::ffi::c_char
        || !(*str_pointer).is_null()
            && !new_str.is_null()
            && strcmp(*str_pointer, new_str) == 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    copy = safe_c2rust_g_strdup_inline(new_str);
    g_free(*str_pointer as gpointer);
    *str_pointer = copy;
    return TRUE;
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_4: ::core::ffi::c_int = 0;
        if gstring.is_null() {
            _g_boolean_var_4 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_4 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_4
    }) as ::core::ffi::c_long
        != 0
    {
        return g_string_append_len(gstring, val as *const gchar, len);
    }
    if ({
        let mut _g_boolean_var_5: ::core::ffi::c_int = 0;
        if val.is_null() {
            _g_boolean_var_5 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_5 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_5
    }) as ::core::ffi::c_long
        != 0
    {
        return if len != 0 as gssize {
            g_string_append_len(gstring, val as *const gchar, len)
        } else {
            gstring
        };
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    if ({
        let mut _g_boolean_var_6: ::core::ffi::c_int = 0;
        if (*gstring).len.wrapping_add(len_unsigned) < (*gstring).allocated_len {
            _g_boolean_var_6 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_6 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_6
    }) as ::core::ffi::c_long
        != 0
    {
        let mut end: *mut ::core::ffi::c_char = (*gstring).str_0.offset((*gstring).len as isize);
        if ({
            let mut _g_boolean_var_7: ::core::ffi::c_int = 0;
            if val.offset(len_unsigned as isize) <= end as *const ::core::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const ::core::ffi::c_char
            {
                _g_boolean_var_7 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_7 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_7
        }) as ::core::ffi::c_long
            != 0
        {
            memcpy(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        } else {
            memmove(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        }
        (*gstring).len = (*gstring).len.wrapping_add(len_unsigned);
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
        return gstring;
    } else {
        return g_string_insert_len(
            gstring,
            -(1 as ::core::ffi::c_int) as gssize,
            val as *const gchar,
            len,
        );
    };
}
pub const G_VARIANT_TYPE_ANY: *const GVariantType =
    b"*\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_VARIANT: GType = ((21 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const DBUS_SERVICE_DBUS: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"org.freedesktop.DBus\0")
};
pub const DBUS_INTERFACE_DBUS: [::core::ffi::c_char; 21] = DBUS_SERVICE_DBUS;
pub const DBUS_PATH_DBUS: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"/org/freedesktop/DBus\0")
};
static mut safe_c2rust_g__message_bus_lock_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_the_session_bus: GWeakRef = GWeakRef {
    priv_0: C2RustUnnamed_2 {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
};
static mut safe_c2rust_the_system_bus: GWeakRef = GWeakRef {
    priv_0: C2RustUnnamed_2 {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
};
pub const SEND_MESSAGE_FLAGS_INITIALIZING: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int;
pub const CALL_FLAGS_INITIALIZING: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_call_destroy_notify_data_in_idle(
    mut user_data: gpointer,
) -> gboolean {
    let mut data: *mut CallDestroyNotifyData = user_data as *mut CallDestroyNotifyData;
    (*data).callback.expect("non-null function pointer")((*data).user_data);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_call_destroy_notify_data_free(
    mut data: *mut CallDestroyNotifyData,
) {
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_call_destroy_notify(
    mut context: *mut GMainContext,
    mut callback: GDestroyNotify,
    mut user_data: gpointer,
) {
    let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut data: *mut CallDestroyNotifyData = ::core::ptr::null_mut::<CallDestroyNotifyData>();
    if callback.is_none() {
        return;
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<CallDestroyNotifyData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut CallDestroyNotifyData;
    (*data).callback = callback;
    (*data).user_data = user_data;
    idle_source = g_idle_source_new();
    g_source_set_priority(idle_source, G_PRIORITY_DEFAULT);
    g_source_set_callback(
        idle_source,
        Some(
            safe_c2rust_call_destroy_notify_data_in_idle
                as unsafe extern "C" fn(gpointer) -> gboolean,
        ),
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut CallDestroyNotifyData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_call_destroy_notify_data_free
                as unsafe extern "C" fn(*mut CallDestroyNotifyData) -> (),
        )),
    );
    g_source_set_static_name(
        idle_source,
        b"[gio] call_destroy_notify_data_in_idle\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(idle_source, context);
    g_source_unref(idle_source);
}
unsafe extern "C" fn safe_c2rust_signal_subscriber_ref(
    mut subscriber: *mut SignalSubscriber,
) -> *mut SignalSubscriber {
    g_atomic_ref_count_inc(&raw mut (*subscriber).ref_count);
    return subscriber;
}
unsafe extern "C" fn safe_c2rust_signal_subscriber_unref(mut subscriber: *mut SignalSubscriber) {
    if g_atomic_ref_count_dec(&raw mut (*subscriber).ref_count) != 0 {
        safe_c2rust_call_destroy_notify(
            (*subscriber).context,
            (*subscriber).user_data_free_func,
            (*subscriber).user_data,
        );
        g_main_context_unref((*subscriber).context);
        g_free(subscriber as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_watched_name_new() -> *mut WatchedName {
    let mut watched_name: *mut WatchedName = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<WatchedName>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut WatchedName;
    g_ref_count_init(&raw mut (*watched_name).ref_count);
    (*watched_name).owner = ::core::ptr::null_mut::<gchar>();
    return safe_c2rust_g_steal_pointer(&raw mut watched_name as gpointer) as *mut WatchedName;
}
unsafe extern "C" fn safe_c2rust_signal_data_new_take(
    mut rule: *mut gchar,
    mut sender: *mut gchar,
    mut interface_name: *mut gchar,
    mut member: *mut gchar,
    mut object_path: *mut gchar,
    mut arg0: *mut gchar,
    mut flags: GDBusSignalFlags,
) -> *mut SignalData {
    let mut signal_data: *mut SignalData = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<SignalData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut SignalData;
    (*signal_data).rule = rule;
    (*signal_data).sender = sender;
    (*signal_data).interface_name = interface_name;
    (*signal_data).member = member;
    (*signal_data).object_path = object_path;
    (*signal_data).arg0 = arg0;
    (*signal_data).flags = flags;
    (*signal_data).subscribers = g_ptr_array_new_with_free_func(::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut SignalSubscriber) -> ()>,
        GDestroyNotify,
    >(Some(
        safe_c2rust_signal_subscriber_unref as unsafe extern "C" fn(*mut SignalSubscriber) -> (),
    )));
    return safe_c2rust_g_steal_pointer(&raw mut signal_data as gpointer) as *mut SignalData;
}
unsafe extern "C" fn safe_c2rust_signal_data_free(mut signal_data: *mut SignalData) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*(*signal_data).subscribers).len == 0 as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            414 as ::core::ffi::c_int,
            G_STRFUNC,
            b"signal_data->subscribers->len == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*signal_data).watched_name.is_null() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            418 as ::core::ffi::c_int,
            G_STRFUNC,
            b"signal_data->watched_name == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if (*signal_data).shared_name_watcher.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            422 as ::core::ffi::c_int,
            G_STRFUNC,
            b"signal_data->shared_name_watcher == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    g_free((*signal_data).rule as gpointer);
    g_free((*signal_data).sender as gpointer);
    g_free((*signal_data).interface_name as gpointer);
    g_free((*signal_data).member as gpointer);
    g_free((*signal_data).object_path as gpointer);
    g_free((*signal_data).arg0 as gpointer);
    g_ptr_array_unref((*signal_data).subscribers);
    g_free(signal_data as gpointer);
}
static mut safe_c2rust_signals: [guint; 1] = [0 as ::core::ffi::c_int as guint];
static mut safe_c2rust_g_dbus_connection_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusConnection\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusConnectionClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_connection_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusConnection>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusConnection) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_connection_init
                    as unsafe extern "C" fn(*mut GDBusConnection) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
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
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
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
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GDBusConnection_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_connection_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dbus_connection_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_connection_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusConnection_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusConnection_private_offset,
        );
    }
    safe_c2rust_g_dbus_connection_class_init(klass as *mut GDBusConnectionClass);
}
unsafe extern "C" fn safe_c2rust_check_initialized(
    mut connection: *mut GDBusConnection,
) -> gboolean {
    let mut flags: gint = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*connection).atomic_flags;
            (*connection).atomic_flags;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*connection).atomic_flags);
        gaig_temp
    });
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_int & FLAG_INITIALIZED as ::core::ffi::c_int != 0 {
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
            b"flags & FLAG_INITIALIZED\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*connection).initialization_error.is_null() {
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
            b"connection->initialization_error == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_check_unclosed(
    mut connection: *mut GDBusConnection,
    mut check: CheckUnclosedFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut flags: gint = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*connection).atomic_flags;
            (*connection).atomic_flags;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*connection).atomic_flags);
        gaig_temp
    });
    if check as ::core::ffi::c_uint
        & MAY_BE_UNINITIALIZED as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
    {
        if ({
            let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
            if flags as ::core::ffi::c_int & FLAG_INITIALIZED as ::core::ffi::c_int != 0 {
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
                b"flags & FLAG_INITIALIZED\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
        if ({
            let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
            if (*connection).initialization_error.is_null() {
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
                b"connection->initialization_error == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
    }
    if flags as ::core::ffi::c_int & FLAG_CLOSED as ::core::ffi::c_int != 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
            glib_gettext(b"The connection is closed\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    return TRUE;
}
static mut safe_c2rust_alive_connections: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
unsafe extern "C" fn safe_c2rust_g_dbus_connection_dispose(mut object: *mut GObject) {
    let mut connection: *mut GDBusConnection =
        object as *mut ::core::ffi::c_void as *mut GDBusConnection;
    g_mutex_lock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    g_mutex_lock(&raw mut (*connection).lock);
    if !(*connection).worker.is_null() {
        _g_dbus_worker_stop((*connection).worker);
        (*connection).worker = ::core::ptr::null_mut::<GDBusWorker>();
        if !safe_c2rust_alive_connections.is_null() {
            if !(({
                let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                if g_hash_table_remove(safe_c2rust_alive_connections, connection as gconstpointer)
                    != 0
                {
                    _g_boolean_var_17 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_17 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_17
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    754 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_hash_table_remove (alive_connections, connection)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
    } else if !safe_c2rust_alive_connections.is_null() {
        if !(({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if g_hash_table_contains(safe_c2rust_alive_connections, connection as gconstpointer)
                == 0
            {
                _g_boolean_var_18 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_18 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_18
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                759 as ::core::ffi::c_int,
                G_STRFUNC,
                b"!g_hash_table_contains (alive_connections, connection)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    g_mutex_unlock(&raw mut (*connection).lock);
    g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    if (*(safe_c2rust_g_dbus_connection_parent_class as *mut GObjectClass))
        .dispose
        .is_some()
    {
        (*(safe_c2rust_g_dbus_connection_parent_class as *mut GObjectClass))
            .dispose
            .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_finalize(mut object: *mut GObject) {
    let mut connection: *mut GDBusConnection =
        object as *mut ::core::ffi::c_void as *mut GDBusConnection;
    (*connection).finalizing = TRUE as gboolean;
    safe_c2rust_purge_all_signal_subscriptions(connection);
    safe_c2rust_purge_all_filters(connection);
    g_ptr_array_unref((*connection).filters);
    if !(*connection).authentication_observer.is_null() {
        g_object_unref((*connection).authentication_observer as gpointer);
    }
    if !(*connection).auth.is_null() {
        g_object_unref((*connection).auth as gpointer);
    }
    if !(*connection).credentials.is_null() {
        g_object_unref((*connection).credentials as gpointer);
    }
    if !(*connection).stream.is_null() {
        g_object_unref((*connection).stream as gpointer);
        (*connection).stream = ::core::ptr::null_mut::<GIOStream>();
    }
    g_free((*connection).address as gpointer);
    g_free((*connection).guid as gpointer);
    g_free((*connection).bus_unique_name as gpointer);
    if !(*connection).initialization_error.is_null() {
        g_error_free((*connection).initialization_error);
    }
    g_hash_table_unref((*connection).map_method_serial_to_task);
    g_hash_table_unref((*connection).map_method_serial_to_name_watcher);
    g_hash_table_unref((*connection).map_rule_to_signal_data);
    g_hash_table_unref((*connection).map_id_to_signal_data);
    g_hash_table_unref((*connection).map_sender_unique_name_to_signal_data_array);
    g_hash_table_unref((*connection).map_id_to_ei);
    g_hash_table_unref((*connection).map_object_path_to_eo);
    g_hash_table_unref((*connection).map_id_to_es);
    g_hash_table_unref((*connection).map_object_path_to_es);
    g_hash_table_unref((*connection).map_thread_to_last_serial);
    g_main_context_unref((*connection).main_context_at_construction);
    g_free((*connection).machine_id as gpointer);
    g_mutex_clear(&raw mut (*connection).init_lock);
    g_mutex_clear(&raw mut (*connection).lock);
    (*(safe_c2rust_g_dbus_connection_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut connection: *mut GDBusConnection =
        object as *mut ::core::ffi::c_void as *mut GDBusConnection;
    match prop_id {
        1 => {
            g_value_set_object(
                value,
                safe_c2rust_g_dbus_connection_get_stream(connection) as gpointer,
            );
        }
        4 => {
            g_value_set_string(value, safe_c2rust_g_dbus_connection_get_guid(connection));
        }
        5 => {
            g_value_set_string(
                value,
                safe_c2rust_g_dbus_connection_get_unique_name(connection),
            );
        }
        6 => {
            g_value_set_boolean(value, safe_c2rust_g_dbus_connection_is_closed(connection));
        }
        7 => {
            g_value_set_boolean(
                value,
                safe_c2rust_g_dbus_connection_get_exit_on_close(connection),
            );
        }
        8 => {
            g_value_set_flags(
                value,
                safe_c2rust_g_dbus_connection_get_capabilities(connection) as guint,
            );
        }
        3 => {
            g_value_set_flags(
                value,
                safe_c2rust_g_dbus_connection_get_flags(connection) as guint,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                867 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut connection: *mut GDBusConnection =
        object as *mut ::core::ffi::c_void as *mut GDBusConnection;
    match prop_id {
        1 => {
            (*connection).stream = g_value_dup_object(value) as *mut GIOStream;
        }
        4 => {
            (*connection).guid = g_value_dup_string(value);
        }
        2 => {
            (*connection).address = g_value_dup_string(value);
        }
        3 => {
            (*connection).flags = g_value_get_flags(value) as GDBusConnectionFlags;
        }
        7 => {
            safe_c2rust_g_dbus_connection_set_exit_on_close(connection, g_value_get_boolean(value));
        }
        9 => {
            (*connection).authentication_observer =
                g_value_dup_object(value) as *mut GDBusAuthObserver;
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                908 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_real_closed(
    mut connection: *mut GDBusConnection,
    mut remote_peer_vanished: gboolean,
    mut error: *mut GError,
) {
    let mut flags: gint = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*connection).atomic_flags;
            (*connection).atomic_flags;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*connection).atomic_flags);
        gaig_temp
    });
    if remote_peer_vanished != 0
        && flags as ::core::ffi::c_int & FLAG_EXIT_ON_CLOSE as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        && flags as ::core::ffi::c_int & FLAG_INITIALIZED as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        && (*connection).initialization_error.is_null()
    {
        raise(SIGTERM);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_class_init(
    mut klass: *mut GDBusConnectionClass,
) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_dbus_connection_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).dispose =
        Some(safe_c2rust_g_dbus_connection_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dbus_connection_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dbus_connection_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*klass).closed = Some(
        safe_c2rust_g_dbus_connection_real_closed
            as unsafe extern "C" fn(*mut GDBusConnection, gboolean, *mut GError) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GDBusConnection, gboolean, *mut GError) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_STREAM as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"stream\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_io_stream_get_type(),
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
        PROP_ADDRESS as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"address\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_connection_flags_get_type(),
            G_DBUS_CONNECTION_FLAGS_NONE as ::core::ffi::c_int as guint,
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
        PROP_GUID as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"guid\0" as *const u8 as *const gchar,
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
        PROP_UNIQUE_NAME as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"unique-name\0" as *const u8 as *const gchar,
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
        PROP_CLOSED as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"closed\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_EXIT_ON_CLOSE as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"exit-on-close\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_CAPABILITY_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"capabilities\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_capability_flags_get_type(),
            G_DBUS_CAPABILITY_FLAGS_NONE as ::core::ffi::c_int as guint,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        gobject_class,
        PROP_AUTHENTICATION_OBSERVER as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"authentication-observer\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_dbus_auth_observer_get_type(),
            (G_PARAM_WRITABLE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    safe_c2rust_signals[CLOSED_SIGNAL as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"closed\0" as *const u8 as *const gchar),
        safe_c2rust_g_dbus_connection_get_type(),
        G_SIGNAL_RUN_LAST,
        136 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__BOOLEAN_BOXED
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
        G_TYPE_BOOLEAN,
        g_error_get_type(),
    );
    g_signal_set_va_marshaller(
        safe_c2rust_signals[CLOSED_SIGNAL as ::core::ffi::c_int as usize],
        (*(klass as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__BOOLEAN_BOXEDv
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
unsafe extern "C" fn safe_c2rust_g_dbus_connection_init(mut connection: *mut GDBusConnection) {
    g_mutex_init(&raw mut (*connection).lock);
    g_mutex_init(&raw mut (*connection).init_lock);
    (*connection).map_method_serial_to_task = g_hash_table_new_full(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*connection).map_method_serial_to_name_watcher = g_hash_table_new_full(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        None,
    );
    (*connection).map_rule_to_signal_data = g_hash_table_new(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    (*connection).map_id_to_signal_data = g_hash_table_new(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    (*connection).map_sender_unique_name_to_signal_data_array = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GPtrArray) -> ()>, GDestroyNotify>(
            Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()),
        ),
    );
    (*connection).map_object_path_to_eo = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ExportedObject) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_exported_object_free as unsafe extern "C" fn(*mut ExportedObject) -> (),
        )),
    );
    (*connection).map_id_to_ei = g_hash_table_new(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    (*connection).map_object_path_to_es = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut ExportedSubtree) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_exported_subtree_unref as unsafe extern "C" fn(*mut ExportedSubtree) -> (),
        )),
    );
    (*connection).map_id_to_es = g_hash_table_new(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    (*connection).map_thread_to_last_serial = g_hash_table_new(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    (*connection).main_context_at_construction = g_main_context_ref_thread_default();
    (*connection).filters = g_ptr_array_new();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_stream(
    mut connection: *mut GDBusConnection,
) -> *mut GIOStream {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIOStream>();
    }
    if safe_c2rust_check_initialized(connection) == 0 {
        return ::core::ptr::null_mut::<GIOStream>();
    }
    return (*connection).stream;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_start_message_processing(
    mut connection: *mut GDBusConnection,
) {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_check_initialized(connection) == 0 {
        return;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !(*connection).worker.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1272 as ::core::ffi::c_int,
            G_STRFUNC,
            b"connection->worker != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    _g_dbus_worker_unfreeze((*connection).worker);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_is_closed(
    mut connection: *mut GDBusConnection,
) -> gboolean {
    let mut flags: gint = 0;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
        return 0 as gboolean;
    }
    flags = ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*connection).atomic_flags;
            (*connection).atomic_flags;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*connection).atomic_flags);
        gaig_temp
    });
    return if flags as ::core::ffi::c_int & FLAG_CLOSED as ::core::ffi::c_int != 0 {
        TRUE
    } else {
        FALSE
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_capabilities(
    mut connection: *mut GDBusConnection,
) -> GDBusCapabilityFlags {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DBUS_CAPABILITY_FLAGS_NONE;
    }
    if safe_c2rust_check_initialized(connection) == 0 {
        return G_DBUS_CAPABILITY_FLAGS_NONE;
    }
    return (*connection).capabilities;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_flags(
    mut connection: *mut GDBusConnection,
) -> GDBusConnectionFlags {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DBUS_CONNECTION_FLAGS_NONE;
    }
    if safe_c2rust_check_initialized(connection) == 0 {
        return G_DBUS_CONNECTION_FLAGS_NONE;
    }
    return (*connection).flags;
}
unsafe extern "C" fn safe_c2rust_flush_in_thread_func(
    mut task: *mut GTask,
    mut source_object: gpointer,
    mut task_data: gpointer,
    mut cancellable: *mut GCancellable,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_dbus_connection_flush_sync(
        source_object as *mut GDBusConnection,
        cancellable,
        &raw mut error,
    ) != 0
    {
        g_task_return_boolean(task, TRUE);
    } else {
        g_task_return_error(task, error);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_flush(
    mut connection: *mut GDBusConnection,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
        return;
    }
    task = g_task_new(connection as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_dbus_connection_flush
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_dbus_connection_flush\0" as *const u8 as *const gchar,
        );
    }
    g_task_run_in_thread(
        task,
        Some(
            safe_c2rust_flush_in_thread_func
                as unsafe extern "C" fn(*mut GTask, gpointer, gpointer, *mut GCancellable) -> (),
        ),
    );
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_flush_finish(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, connection as gpointer) != 0 {
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
            b"g_task_is_valid (res, connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(res as *mut ::core::ffi::c_void as *mut GTask, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_flush_sync(
    mut connection: *mut GDBusConnection,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
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
    ret = FALSE as gboolean;
    if !(safe_c2rust_check_unclosed(connection, 0 as CheckUnclosedFlags, error) == 0) {
        if ({
            let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
            if !(*connection).worker.is_null() {
                _g_boolean_var_31 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_31 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_31
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1464 as ::core::ffi::c_int,
                G_STRFUNC,
                b"connection->worker != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        ret = _g_dbus_worker_flush_sync((*connection).worker, cancellable, error);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_emit_closed_data_free(mut data: *mut EmitClosedData) {
    g_object_unref((*data).connection as gpointer);
    if !(*data).error.is_null() {
        g_error_free((*data).error);
    }
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_emit_closed_in_idle(mut user_data: gpointer) -> gboolean {
    let mut data: *mut EmitClosedData = user_data as *mut EmitClosedData;
    let mut result: gboolean = 0;
    g_object_notify(
        (*data).connection as *mut ::core::ffi::c_void as *mut GObject,
        b"closed\0" as *const u8 as *const gchar,
    );
    g_signal_emit(
        (*data).connection as gpointer,
        safe_c2rust_signals[CLOSED_SIGNAL as ::core::ffi::c_int as usize],
        0 as GQuark,
        (*data).remote_peer_vanished,
        (*data).error,
        &raw mut result,
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_schedule_closed_unlocked(
    mut connection: *mut GDBusConnection,
    mut remote_peer_vanished: gboolean,
    mut error: *mut GError,
) {
    let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut data: *mut EmitClosedData = ::core::ptr::null_mut::<EmitClosedData>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if g_mutex_trylock(&raw mut (*connection).lock) != 0 {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
        g_assertion_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1522 as ::core::ffi::c_int,
            G_STRFUNC,
            b"CONNECTION_ENSURE_LOCK: GDBusConnection object lock is not locked\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<EmitClosedData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut EmitClosedData;
    (*data).connection =
        g_object_ref(connection as gpointer) as *mut GDBusConnection as *mut GDBusConnection;
    (*data).remote_peer_vanished = remote_peer_vanished;
    (*data).error = if !error.is_null() {
        g_error_copy(error)
    } else {
        ::core::ptr::null_mut::<GError>()
    };
    idle_source = g_idle_source_new();
    g_source_set_priority(idle_source, G_PRIORITY_DEFAULT);
    g_source_set_callback(
        idle_source,
        Some(safe_c2rust_emit_closed_in_idle as unsafe extern "C" fn(gpointer) -> gboolean),
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut EmitClosedData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_emit_closed_data_free as unsafe extern "C" fn(*mut EmitClosedData) -> (),
        )),
    );
    g_source_set_static_name(
        idle_source,
        b"[gio] emit_closed_in_idle\0" as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(idle_source, (*connection).main_context_at_construction);
    g_source_unref(idle_source);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_close(
    mut connection: *mut GDBusConnection,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_check_initialized(connection) == 0 {
        return;
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !(*connection).worker.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1591 as ::core::ffi::c_int,
            G_STRFUNC,
            b"connection->worker != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    task = g_task_new(connection as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_dbus_connection_close
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_dbus_connection_close\0" as *const u8 as *const gchar,
        );
    }
    _g_dbus_worker_close((*connection).worker, task);
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_close_finish(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, connection as gpointer) != 0 {
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
            b"g_task_is_valid (res, connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_task_propagate_boolean(res as *mut ::core::ffi::c_void as *mut GTask, error);
}
unsafe extern "C" fn safe_c2rust_sync_close_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut SyncCloseData = user_data as *mut SyncCloseData;
    (*data).result = g_object_ref(res as gpointer) as *mut GAsyncResult as *mut GAsyncResult;
    g_main_loop_quit((*data).loop_0);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_close_sync(
    mut connection: *mut GDBusConnection,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    if safe_c2rust_check_unclosed(connection, 0 as CheckUnclosedFlags, error) != 0 {
        let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
        let mut data: SyncCloseData = SyncCloseData {
            loop_0: ::core::ptr::null_mut::<GMainLoop>(),
            result: ::core::ptr::null_mut::<GAsyncResult>(),
        };
        context = g_main_context_new();
        g_main_context_push_thread_default(context);
        data.loop_0 = g_main_loop_new(context, TRUE);
        data.result = ::core::ptr::null_mut::<GAsyncResult>();
        safe_c2rust_g_dbus_connection_close(
            connection,
            cancellable,
            Some(
                safe_c2rust_sync_close_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            &raw mut data as gpointer,
        );
        g_main_loop_run(data.loop_0);
        ret = safe_c2rust_g_dbus_connection_close_finish(connection, data.result, error);
        g_object_unref(data.result as gpointer);
        g_main_loop_unref(data.loop_0);
        g_main_context_pop_thread_default(context);
        g_main_context_unref(context);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_last_serial(
    mut connection: *mut GDBusConnection,
) -> guint32 {
    let mut ret: guint32 = 0;
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    g_mutex_lock(&raw mut (*connection).lock);
    ret = g_hash_table_lookup(
        (*connection).map_thread_to_last_serial,
        g_thread_self() as gconstpointer,
    ) as gulong as guint as guint32;
    g_mutex_unlock(&raw mut (*connection).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_send_message_unlocked(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut flags: GDBusSendMessageFlags,
    mut out_serial: *mut guint32,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut blob: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut blob_size: gsize = 0;
    let mut serial_to_use: guint32 = 0;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if g_mutex_trylock(&raw mut (*connection).lock) != 0 {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
        g_assertion_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1737 as ::core::ffi::c_int,
            G_STRFUNC,
            b"CONNECTION_ENSURE_LOCK: GDBusConnection object lock is not locked\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !out_serial.is_null() {
        *out_serial = 0 as guint32;
    }
    if safe_c2rust_check_unclosed(
        connection,
        (if flags as ::core::ffi::c_uint & SEND_MESSAGE_FLAGS_INITIALIZING != 0 {
            MAY_BE_UNINITIALIZED as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as CheckUnclosedFlags,
        error,
    ) == 0
    {
        return FALSE;
    }
    blob = g_dbus_message_to_blob(
        message,
        &raw mut blob_size,
        (*connection).capabilities,
        error,
    );
    if blob.is_null() {
        return FALSE;
    }
    if flags as ::core::ffi::c_uint
        & G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        serial_to_use = g_dbus_message_get_serial(message);
    } else {
        (*connection).last_serial = (*connection).last_serial.wrapping_add(1);
        serial_to_use = (*connection).last_serial;
    }
    match *blob.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
        108 => {
            *(blob as *mut guint32).offset(2 as ::core::ffi::c_int as isize) = serial_to_use;
        }
        66 => {
            *(blob as *mut guint32).offset(2 as ::core::ffi::c_int as isize) = ({
                let mut __v: guint32 = 0;
                let mut __x: guint32 = serial_to_use;
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
                } else {
                    let fresh0 = &mut __v;
                    let fresh1;
                    let fresh2 = __x;
                    asm!(
                        "bswapl {0:e}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
                }
                __v
            });
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1778 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    if !out_serial.is_null() {
        *out_serial = serial_to_use;
    }
    g_hash_table_replace(
        (*connection).map_thread_to_last_serial,
        g_thread_self() as gpointer,
        serial_to_use as gulong as gpointer,
    );
    if flags as ::core::ffi::c_uint
        & G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
    {
        g_dbus_message_set_serial(message, serial_to_use);
    }
    g_dbus_message_lock(message);
    _g_dbus_worker_send_message((*connection).worker, message, blob as *mut gchar, blob_size);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_send_message(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut flags: GDBusSendMessageFlags,
    mut out_serial: *mut guint32,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            || g_dbus_message_get_locked(message) == 0
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
            b"(flags & G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL) || !g_dbus_message_get_locked (message)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
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
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut (*connection).lock);
    ret = safe_c2rust_g_dbus_connection_send_message_unlocked(
        connection,
        message,
        flags,
        out_serial as *mut guint32,
        error,
    );
    g_mutex_unlock(&raw mut (*connection).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_send_message_data_free(mut data: *mut SendMessageData) {
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if (*data).timeout_source.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1892 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data->timeout_source == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if (*data).cancellable_handler_id == 0 as gulong {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1893 as ::core::ffi::c_int,
            G_STRFUNC,
            b"data->cancellable_handler_id == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_slice_free1(
        ::core::mem::size_of::<SendMessageData>() as gsize,
        data as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_send_message_with_reply_cleanup(
    mut task: *mut GTask,
    mut remove: gboolean,
) {
    let mut connection: *mut GDBusConnection =
        g_task_get_source_object(task) as *mut GDBusConnection;
    let mut data: *mut SendMessageData = g_task_get_task_data(task) as *mut SendMessageData;
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if g_mutex_trylock(&raw mut (*connection).lock) != 0 {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0
    {
        g_assertion_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1907 as ::core::ffi::c_int,
            G_STRFUNC,
            b"CONNECTION_ENSURE_LOCK: GDBusConnection object lock is not locked\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if (*data).delivered == 0 {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1909 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!data->delivered\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*data).delivered = TRUE as gboolean;
    if !(*data).timeout_source.is_null() {
        g_source_destroy((*data).timeout_source);
        let mut _pp: *mut *mut GSource = &raw mut (*data).timeout_source;
        let mut _ptr: *mut GSource = *_pp;
        *_pp = ::core::ptr::null_mut::<GSource>();
        if !_ptr.is_null() {
            g_source_unref(_ptr as *mut GSource);
        }
    }
    if (*data).cancellable_handler_id > 0 as gulong {
        g_cancellable_disconnect(g_task_get_cancellable(task), (*data).cancellable_handler_id);
        (*data).cancellable_handler_id = 0 as gulong;
    }
    if !(*data).cancelled_idle_source.is_null() {
        g_source_destroy((*data).cancelled_idle_source);
        let mut _pp_0: *mut *mut GSource = &raw mut (*data).cancelled_idle_source;
        let mut _ptr_0: *mut GSource = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<GSource>();
        if !_ptr_0.is_null() {
            g_source_unref(_ptr_0 as *mut GSource);
        }
    }
    if remove != 0 {
        let mut removed: gboolean = g_hash_table_remove(
            (*connection).map_method_serial_to_task,
            (*data).serial as gulong as gpointer as gconstpointer,
        );
        if !(({
            let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
            if removed != 0 {
                _g_boolean_var_52 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_52 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_52
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1933 as ::core::ffi::c_int,
                G_STRFUNC,
                b"removed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_send_message_data_deliver_reply_unlocked(
    mut task: *mut GTask,
    mut reply: *mut GDBusMessage,
) {
    let mut data: *mut SendMessageData = g_task_get_task_data(task) as *mut SendMessageData;
    if !((*data).delivered != 0) {
        g_task_return_pointer(
            task,
            g_object_ref(reply as gpointer) as *mut GDBusMessage as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
        safe_c2rust_send_message_with_reply_cleanup(task, TRUE);
    }
}
unsafe extern "C" fn safe_c2rust_send_message_data_deliver_error(
    mut task: *mut GTask,
    mut domain: GQuark,
    mut code: gint,
    mut message: *const ::core::ffi::c_char,
) {
    let mut connection: *mut GDBusConnection =
        g_task_get_source_object(task) as *mut GDBusConnection;
    let mut data: *mut SendMessageData = g_task_get_task_data(task) as *mut SendMessageData;
    g_mutex_lock(&raw mut (*connection).lock);
    if (*data).delivered != 0 {
        g_mutex_unlock(&raw mut (*connection).lock);
        return;
    }
    g_object_ref(task as gpointer);
    safe_c2rust_send_message_with_reply_cleanup(task, TRUE);
    g_mutex_unlock(&raw mut (*connection).lock);
    g_task_return_new_error_literal(task, domain, code, message);
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_send_message_with_reply_cancelled_idle_cb(
    mut user_data: gpointer,
) -> gboolean {
    let mut task: *mut GTask = user_data as *mut GTask;
    safe_c2rust_send_message_data_deliver_error(
        task,
        g_io_error_quark(),
        G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
        glib_gettext(b"Operation was cancelled\0" as *const u8 as *const gchar)
            as *const ::core::ffi::c_char,
    );
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_send_message_with_reply_cancelled_cb(
    mut cancellable: *mut GCancellable,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut data: *mut SendMessageData = g_task_get_task_data(task) as *mut SendMessageData;
    if !(*data).cancelled_idle_source.is_null() {
        return;
    }
    (*data).cancelled_idle_source = g_idle_source_new();
    g_source_set_static_name(
        (*data).cancelled_idle_source,
        b"[gio] send_message_with_reply_cancelled_idle_cb\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    g_task_attach_source(
        task,
        (*data).cancelled_idle_source,
        Some(
            safe_c2rust_send_message_with_reply_cancelled_idle_cb
                as unsafe extern "C" fn(gpointer) -> gboolean,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_send_message_with_reply_timeout_cb(
    mut user_data: gpointer,
) -> gboolean {
    let mut task: *mut GTask = user_data as *mut GTask;
    safe_c2rust_send_message_data_deliver_error(
        task,
        g_io_error_quark(),
        G_IO_ERROR_TIMED_OUT as ::core::ffi::c_int as gint,
        glib_gettext(b"Timeout was reached\0" as *const u8 as *const gchar)
            as *const ::core::ffi::c_char,
    );
    return G_SOURCE_REMOVE;
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_send_message_with_reply_unlocked(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut flags: GDBusSendMessageFlags,
    mut timeout_msec: gint,
    mut out_serial: *mut guint32,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut data: *mut SendMessageData = ::core::ptr::null_mut::<SendMessageData>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut serial: guint32 = 0;
    if out_serial.is_null() {
        out_serial = &raw mut serial;
    }
    if timeout_msec == -(1 as ::core::ffi::c_int) {
        timeout_msec = (25 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int) as gint;
    }
    data = ({
        let mut __s: gsize = ::core::mem::size_of::<SendMessageData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut SendMessageData;
    task = g_task_new(connection as gpointer, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *mut GDBusMessage,
                    GDBusSendMessageFlags,
                    gint,
                    *mut guint32,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_dbus_connection_send_message_with_reply_unlocked
                as unsafe extern "C" fn(
                    *mut GDBusConnection,
                    *mut GDBusMessage,
                    GDBusSendMessageFlags,
                    gint,
                    *mut guint32,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(
            _task,
            b"g_dbus_connection_send_message_with_reply_unlocked\0" as *const u8 as *const gchar,
        );
    }
    g_task_set_task_data(
        task,
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut SendMessageData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_send_message_data_free as unsafe extern "C" fn(*mut SendMessageData) -> (),
        )),
    );
    if g_task_return_error_if_cancelled(task) != 0 {
        g_object_unref(task as gpointer);
        return;
    }
    if safe_c2rust_g_dbus_connection_send_message_unlocked(
        connection,
        message,
        flags,
        out_serial,
        &raw mut error,
    ) == 0
    {
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
        return;
    }
    (*data).serial = *out_serial;
    if !cancellable.is_null() {
        (*data).cancellable_handler_id = g_cancellable_connect(
            cancellable,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GCancellable, gpointer) -> ()>,
                GCallback,
            >(Some(
                safe_c2rust_send_message_with_reply_cancelled_cb
                    as unsafe extern "C" fn(*mut GCancellable, gpointer) -> (),
            )),
            g_object_ref(task as gpointer) as *mut GTask as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    if timeout_msec != G_MAXINT {
        (*data).timeout_source = g_timeout_source_new(timeout_msec as guint);
        g_source_set_static_name(
            (*data).timeout_source,
            b"[gio] send_message_with_reply_unlocked\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_task_attach_source(
            task,
            (*data).timeout_source,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gboolean>, GSourceFunc>(
                Some(
                    safe_c2rust_send_message_with_reply_timeout_cb
                        as unsafe extern "C" fn(gpointer) -> gboolean,
                ),
            ),
        );
    }
    g_hash_table_insert(
        (*connection).map_method_serial_to_task,
        *out_serial as gulong as gpointer,
        safe_c2rust_g_steal_pointer(&raw mut task as gpointer) as *mut GTask as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_send_message_with_reply(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut flags: GDBusSendMessageFlags,
    mut timeout_msec: gint,
    mut out_serial: *mut guint32,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            || g_dbus_message_get_locked(message) == 0
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
            b"(flags & G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL) || !g_dbus_message_get_locked (message)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if timeout_msec >= 0 as ::core::ffi::c_int || timeout_msec == -(1 as ::core::ffi::c_int) {
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
            b"timeout_msec >= 0 || timeout_msec == -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*connection).lock);
    safe_c2rust_g_dbus_connection_send_message_with_reply_unlocked(
        connection,
        message,
        flags,
        timeout_msec,
        out_serial as *mut guint32,
        cancellable,
        callback,
        user_data,
    );
    g_mutex_unlock(&raw mut (*connection).lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_send_message_with_reply_finish(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GDBusMessage {
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, connection as gpointer) != 0 {
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
            b"g_task_is_valid (res, connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GDBusMessage;
}
unsafe extern "C" fn safe_c2rust_send_message_with_reply_sync_cb(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut data: *mut SendMessageSyncData = user_data as *mut SendMessageSyncData;
    (*data).res = g_object_ref(res as gpointer) as *mut GAsyncResult as *mut GAsyncResult;
    g_main_loop_quit((*data).loop_0);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_send_message_with_reply_sync(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut flags: GDBusSendMessageFlags,
    mut timeout_msec: gint,
    mut out_serial: *mut guint32,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusMessage {
    let mut data: SendMessageSyncData = SendMessageSyncData {
        res: ::core::ptr::null_mut::<GAsyncResult>(),
        context: ::core::ptr::null_mut::<GMainContext>(),
        loop_0: ::core::ptr::null_mut::<GMainLoop>(),
    };
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            || g_dbus_message_get_locked(message) == 0
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
            b"(flags & G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL) || !g_dbus_message_get_locked (message)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if timeout_msec >= 0 as ::core::ffi::c_int || timeout_msec == -(1 as ::core::ffi::c_int) {
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
            b"timeout_msec >= 0 || timeout_msec == -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    data.res = ::core::ptr::null_mut::<GAsyncResult>();
    data.context = g_main_context_new();
    data.loop_0 = g_main_loop_new(data.context, FALSE);
    g_main_context_push_thread_default(data.context);
    safe_c2rust_g_dbus_connection_send_message_with_reply(
        connection,
        message,
        flags,
        timeout_msec,
        out_serial,
        cancellable,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusConnection, *mut GAsyncResult, gpointer) -> ()>,
            GAsyncReadyCallback,
        >(Some(
            safe_c2rust_send_message_with_reply_sync_cb
                as unsafe extern "C" fn(*mut GDBusConnection, *mut GAsyncResult, gpointer) -> (),
        )),
        &raw mut data as gpointer,
    );
    g_main_loop_run(data.loop_0);
    reply =
        safe_c2rust_g_dbus_connection_send_message_with_reply_finish(connection, data.res, error);
    g_main_context_pop_thread_default(data.context);
    g_main_context_unref(data.context);
    g_main_loop_unref(data.loop_0);
    if !data.res.is_null() {
        g_object_unref(data.res as gpointer);
    }
    return reply;
}
unsafe extern "C" fn safe_c2rust_name_watcher_unref_watched_name(
    mut connection: *mut GDBusConnection,
    mut name_watcher: *mut SignalData,
) {
    let mut watched_name: *mut WatchedName = (*name_watcher).watched_name;
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if !watched_name.is_null() {
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2330 as ::core::ffi::c_int,
            G_STRFUNC,
            b"watched_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if g_ref_count_dec(&raw mut (*watched_name).ref_count) == 0 {
        return;
    }
    if (*watched_name).get_name_owner_serial != 0 as guint32 {
        g_hash_table_remove(
            (*connection).map_method_serial_to_name_watcher,
            (*watched_name).get_name_owner_serial as gulong as gpointer as gconstpointer,
        );
    }
    (*name_watcher).watched_name = ::core::ptr::null_mut::<WatchedName>();
    g_free((*watched_name).owner as gpointer);
    g_free(watched_name as gpointer);
}
unsafe extern "C" fn safe_c2rust_name_watcher_set_name_owner_unlocked(
    mut name_watcher: *mut SignalData,
    mut new_owner: *const ::core::ffi::c_char,
) {
    if !new_owner.is_null()
        && *new_owner.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
    {
        new_owner = ::core::ptr::null::<::core::ffi::c_char>();
    }
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if !(*name_watcher).watched_name.is_null() {
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2359 as ::core::ffi::c_int,
            G_STRFUNC,
            b"name_watcher->watched_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_g_set_str(&raw mut (*(*name_watcher).watched_name).owner, new_owner);
}
unsafe extern "C" fn safe_c2rust_name_watcher_deliver_name_owner_changed_unlocked(
    mut name_watcher: *mut SignalData,
    mut message: *mut GDBusMessage,
) {
    let mut body: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    body = g_dbus_message_get_body(message);
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if !body.is_null()
            && g_variant_is_of_type(
                body,
                g_variant_type_checked_(b"(sss)\0" as *const u8 as *const gchar),
            ) != 0
        {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
        let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut new_owner: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        g_variant_get(
            body,
            b"(&s&s&s)\0" as *const u8 as *const gchar,
            &raw mut name,
            NULL_0,
            &raw mut new_owner,
        );
        if ({
            let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
            if g_strcmp0((*name_watcher).arg0, name) == 0 as ::core::ffi::c_int {
                _g_boolean_var_68 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_68 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_68
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                2380 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_strcmp0 (name_watcher->arg0, name) == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
            if *new_owner.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32
                || g_dbus_is_name(new_owner as *const gchar) != 0
            {
                _g_boolean_var_69 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_69 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_69
        }) as ::core::ffi::c_long
            != 0
        {
            safe_c2rust_name_watcher_set_name_owner_unlocked(name_watcher, new_owner);
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Received NameOwnerChanged signal with invalid owner \"%s\" for \"%s\"\0"
                    as *const u8 as *const gchar,
                new_owner,
                name,
            );
        }
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Received NameOwnerChanged signal with unexpected signature %s\0" as *const u8
                as *const gchar,
            if body.is_null() {
                b"()\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                g_variant_get_type_string(body) as *const ::core::ffi::c_char
            },
        );
    };
}
unsafe extern "C" fn safe_c2rust_name_watcher_deliver_get_name_owner_reply_unlocked(
    mut name_watcher: *mut SignalData,
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
) {
    let mut type_0: GDBusMessageType = G_DBUS_MESSAGE_TYPE_INVALID;
    let mut body: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut watched_name: *mut WatchedName = ::core::ptr::null_mut::<WatchedName>();
    watched_name = (*name_watcher).watched_name;
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if !watched_name.is_null() {
            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_70
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2411 as ::core::ffi::c_int,
            G_STRFUNC,
            b"watched_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if (*watched_name).get_name_owner_serial != 0 as guint32 {
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2412 as ::core::ffi::c_int,
            G_STRFUNC,
            b"watched_name->get_name_owner_serial != 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    type_0 = g_dbus_message_get_message_type(message);
    body = g_dbus_message_get_body(message);
    if type_0 as ::core::ffi::c_uint
        == G_DBUS_MESSAGE_TYPE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if g_strcmp0(
            g_dbus_message_get_error_name(message) as *const ::core::ffi::c_char,
            b"org.freedesktop.DBus.Error.NameHasNoOwner\0" as *const u8
                as *const ::core::ffi::c_char,
        ) != 0
        {
            safe_c2rust_name_watcher_set_name_owner_unlocked(
                name_watcher,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    } else if type_0 as ::core::ffi::c_uint
        != G_DBUS_MESSAGE_TYPE_METHOD_RETURN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Received GetNameOwner reply with unexpected type %d\0" as *const u8 as *const gchar,
            type_0 as ::core::ffi::c_uint,
        );
    } else if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if !body.is_null()
            && g_variant_is_of_type(
                body,
                g_variant_type_checked_(b"(s)\0" as *const u8 as *const gchar),
            ) != 0
        {
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
        let mut new_owner: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        g_variant_get(
            body,
            b"(&s)\0" as *const u8 as *const gchar,
            &raw mut new_owner,
        );
        if ({
            let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
            if g_dbus_is_name(new_owner as *const gchar) != 0 {
                _g_boolean_var_73 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_73 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_73
        }) as ::core::ffi::c_long
            != 0
        {
            safe_c2rust_name_watcher_set_name_owner_unlocked(name_watcher, new_owner);
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Received GetNameOwner reply with invalid owner \"%s\" for \"%s\"\0" as *const u8
                    as *const gchar,
                new_owner,
                (*name_watcher).arg0,
            );
        }
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Received GetNameOwner reply with unexpected signature %s\0" as *const u8
                as *const gchar,
            if body.is_null() {
                b"()\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                g_variant_get_type_string(body) as *const ::core::ffi::c_char
            },
        );
    }
    g_hash_table_remove(
        (*connection).map_method_serial_to_name_watcher,
        (*watched_name).get_name_owner_serial as gulong as gpointer as gconstpointer,
    );
    (*watched_name).get_name_owner_serial = 0 as guint32;
}
unsafe extern "C" fn safe_c2rust_name_watcher_call_get_name_owner_unlocked(
    mut connection: *mut GDBusConnection,
    mut name_watcher: *mut SignalData,
) {
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut watched_name: *mut WatchedName = ::core::ptr::null_mut::<WatchedName>();
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if g_strcmp0(
            (*name_watcher).sender,
            b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            _g_boolean_var_74 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_74 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_74
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2466 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_strcmp0 (name_watcher->sender, DBUS_SERVICE_DBUS) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if g_strcmp0(
            (*name_watcher).interface_name,
            b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_75
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2467 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_strcmp0 (name_watcher->interface_name, DBUS_INTERFACE_DBUS) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if g_strcmp0(
            (*name_watcher).member,
            b"NameOwnerChanged\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_76
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2468 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_strcmp0 (name_watcher->member, \"NameOwnerChanged\") == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if g_strcmp0(
            (*name_watcher).object_path,
            b"/org/freedesktop/DBus\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            _g_boolean_var_77 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_77 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_77
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2469 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_strcmp0 (name_watcher->object_path, DBUS_PATH_DBUS) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if g_dbus_is_name((*name_watcher).arg0) != 0 {
            _g_boolean_var_78 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_78 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_78
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2472 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_dbus_is_name (name_watcher->arg0)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if (*name_watcher).flags as ::core::ffi::c_uint
            == G_DBUS_SIGNAL_FLAGS_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _g_boolean_var_79 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_79 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_79
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2473 as ::core::ffi::c_int,
            G_STRFUNC,
            b"name_watcher->flags == G_DBUS_SIGNAL_FLAGS_NONE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    watched_name = (*name_watcher).watched_name;
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if !watched_name.is_null() {
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2476 as ::core::ffi::c_int,
            G_STRFUNC,
            b"watched_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if (*watched_name).owner.is_null() {
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2477 as ::core::ffi::c_int,
            G_STRFUNC,
            b"watched_name->owner == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if (*watched_name).get_name_owner_serial == 0 as guint32 {
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2478 as ::core::ffi::c_int,
            G_STRFUNC,
            b"watched_name->get_name_owner_serial == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if (*name_watcher).shared_name_watcher.is_null() {
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2479 as ::core::ffi::c_int,
            G_STRFUNC,
            b"name_watcher->shared_name_watcher == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    message = g_dbus_message_new_method_call(
        DBUS_SERVICE_DBUS.as_ptr() as *const gchar,
        DBUS_PATH_DBUS.as_ptr() as *const gchar,
        DBUS_INTERFACE_DBUS.as_ptr() as *const gchar,
        b"GetNameOwner\0" as *const u8 as *const gchar,
    );
    g_dbus_message_set_body(
        message,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, (*name_watcher).arg0),
    );
    if safe_c2rust_g_dbus_connection_send_message_unlocked(
        connection,
        message,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        &raw mut (*watched_name).get_name_owner_serial,
        &raw mut local_error,
    ) != 0
    {
        if ({
            let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
            if (*watched_name).get_name_owner_serial != 0 as guint32 {
                _g_boolean_var_84 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_84 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_84
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                2492 as ::core::ffi::c_int,
                G_STRFUNC,
                b"watched_name->get_name_owner_serial != 0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        g_hash_table_insert(
            (*connection).map_method_serial_to_name_watcher,
            (*watched_name).get_name_owner_serial as gulong as gpointer,
            name_watcher as gpointer,
        );
    } else {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Error while sending GetNameOwner() message: %s\0" as *const u8 as *const gchar,
            (*local_error).message,
        );
        g_clear_error(&raw mut local_error);
        if ({
            let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
            if (*watched_name).get_name_owner_serial == 0 as guint32 {
                _g_boolean_var_85 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_85 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_85
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                2502 as ::core::ffi::c_int,
                G_STRFUNC,
                b"watched_name->get_name_owner_serial == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    g_object_unref(message as gpointer);
}
unsafe extern "C" fn safe_c2rust_filter_data_destroy(
    mut filter: *mut FilterData,
    mut notify_sync: gboolean,
) {
    if notify_sync != 0 {
        if (*filter).user_data_free_func.is_some() {
            (*filter)
                .user_data_free_func
                .expect("non-null function pointer")((*filter).user_data);
        }
    } else {
        safe_c2rust_call_destroy_notify(
            (*filter).context,
            (*filter).user_data_free_func,
            (*filter).user_data,
        );
    }
    g_main_context_unref((*filter).context);
    g_free(filter as gpointer);
}
unsafe extern "C" fn safe_c2rust_copy_filter_list(
    mut filters: *mut GPtrArray,
) -> *mut *mut FilterData {
    let mut copy: *mut *mut FilterData = ::core::ptr::null_mut::<*mut FilterData>();
    let mut n: guint = 0;
    copy = ({
        let mut __n: gsize = (*filters).len.wrapping_add(1 as guint) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut FilterData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut *mut FilterData;
    n = 0 as guint;
    while n < (*filters).len {
        let ref mut fresh4 = *copy.offset(n as isize);
        *fresh4 = *(*filters).pdata.offset(n as isize) as *mut FilterData;
        let ref mut fresh5 = (**copy.offset(n as isize)).ref_count;
        *fresh5 = (*fresh5).wrapping_add(1);
        n = n.wrapping_add(1);
    }
    let ref mut fresh6 = *copy.offset(n as isize);
    *fresh6 = ::core::ptr::null_mut::<FilterData>();
    return copy;
}
unsafe extern "C" fn safe_c2rust_free_filter_list(mut filters: *mut *mut FilterData) {
    let mut n: guint = 0;
    n = 0 as guint;
    while !(*filters.offset(n as isize)).is_null() {
        let ref mut fresh3 = (**filters.offset(n as isize)).ref_count;
        *fresh3 = (*fresh3).wrapping_sub(1);
        if (**filters.offset(n as isize)).ref_count == 0 as guint {
            safe_c2rust_filter_data_destroy(*filters.offset(n as isize), FALSE);
        }
        n = n.wrapping_add(1);
    }
    g_free(filters as gpointer);
}
unsafe extern "C" fn safe_c2rust_on_worker_message_received(
    mut worker: *mut GDBusWorker,
    mut message: *mut GDBusMessage,
    mut user_data: gpointer,
) {
    let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    let mut filters: *mut *mut FilterData = ::core::ptr::null_mut::<*mut FilterData>();
    let mut n: guint = 0;
    let mut alive: gboolean = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    alive = g_hash_table_contains(safe_c2rust_alive_connections, user_data as gconstpointer);
    if alive == 0 {
        g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
        return;
    }
    connection = user_data as *mut GDBusConnection;
    g_object_ref(connection as gpointer);
    g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    g_object_ref(message as gpointer);
    g_dbus_message_lock(message);
    g_mutex_lock(&raw mut (*connection).lock);
    filters = safe_c2rust_copy_filter_list((*connection).filters);
    g_mutex_unlock(&raw mut (*connection).lock);
    n = 0 as guint;
    while !(*filters.offset(n as isize)).is_null() {
        message = (**filters.offset(n as isize))
            .filter_function
            .expect("non-null function pointer")(
            connection,
            message,
            TRUE,
            (**filters.offset(n as isize)).user_data,
        );
        if message.is_null() {
            break;
        }
        g_dbus_message_lock(message);
        n = n.wrapping_add(1);
    }
    g_mutex_lock(&raw mut (*connection).lock);
    safe_c2rust_free_filter_list(filters);
    g_mutex_unlock(&raw mut (*connection).lock);
    if !message.is_null() {
        let mut message_type: GDBusMessageType = G_DBUS_MESSAGE_TYPE_INVALID;
        message_type = g_dbus_message_get_message_type(message);
        if message_type as ::core::ffi::c_uint
            == G_DBUS_MESSAGE_TYPE_METHOD_RETURN as ::core::ffi::c_int as ::core::ffi::c_uint
            || message_type as ::core::ffi::c_uint
                == G_DBUS_MESSAGE_TYPE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut reply_serial: guint32 = 0;
            let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
            let mut name_watcher: *mut SignalData = ::core::ptr::null_mut::<SignalData>();
            reply_serial = g_dbus_message_get_reply_serial(message);
            g_mutex_lock(&raw mut (*connection).lock);
            task = g_hash_table_lookup(
                (*connection).map_method_serial_to_task,
                reply_serial as gulong as gpointer as gconstpointer,
            ) as *mut GTask;
            if !task.is_null() {
                safe_c2rust_send_message_data_deliver_reply_unlocked(task, message);
            }
            name_watcher = g_hash_table_lookup(
                (*connection).map_method_serial_to_name_watcher,
                reply_serial as gulong as gpointer as gconstpointer,
            ) as *mut SignalData;
            if !name_watcher.is_null() {
                if ({
                    let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
                    if !(*name_watcher).watched_name.is_null() {
                        _g_boolean_var_86 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_86 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_86
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        2655 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"name_watcher->watched_name != NULL\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if ({
                    let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
                    if (*(*name_watcher).watched_name).get_name_owner_serial == reply_serial {
                        _g_boolean_var_87 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_87 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_87
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        2656 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"name_watcher->watched_name->get_name_owner_serial == reply_serial\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                safe_c2rust_name_watcher_deliver_get_name_owner_reply_unlocked(
                    name_watcher,
                    connection,
                    message,
                );
            }
            g_mutex_unlock(&raw mut (*connection).lock);
        } else if message_type as ::core::ffi::c_uint
            == G_DBUS_MESSAGE_TYPE_SIGNAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            g_mutex_lock(&raw mut (*connection).lock);
            safe_c2rust_distribute_signals(connection, message);
            g_mutex_unlock(&raw mut (*connection).lock);
        } else if message_type as ::core::ffi::c_uint
            == G_DBUS_MESSAGE_TYPE_METHOD_CALL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            g_mutex_lock(&raw mut (*connection).lock);
            safe_c2rust_distribute_method_call(connection, message);
            g_mutex_unlock(&raw mut (*connection).lock);
        }
    }
    if !message.is_null() {
        g_object_unref(message as gpointer);
    }
    g_object_unref(connection as gpointer);
}
unsafe extern "C" fn safe_c2rust_on_worker_message_about_to_be_sent(
    mut worker: *mut GDBusWorker,
    mut message: *mut GDBusMessage,
    mut user_data: gpointer,
) -> *mut GDBusMessage {
    let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    let mut filters: *mut *mut FilterData = ::core::ptr::null_mut::<*mut FilterData>();
    let mut n: guint = 0;
    let mut alive: gboolean = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    alive = g_hash_table_contains(safe_c2rust_alive_connections, user_data as gconstpointer);
    if alive == 0 {
        g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
        return message;
    }
    connection = user_data as *mut GDBusConnection;
    g_object_ref(connection as gpointer);
    g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    g_mutex_lock(&raw mut (*connection).lock);
    filters = safe_c2rust_copy_filter_list((*connection).filters);
    g_mutex_unlock(&raw mut (*connection).lock);
    n = 0 as guint;
    while !(*filters.offset(n as isize)).is_null() {
        g_dbus_message_lock(message);
        message = (**filters.offset(n as isize))
            .filter_function
            .expect("non-null function pointer")(
            connection,
            message,
            FALSE,
            (**filters.offset(n as isize)).user_data,
        );
        if message.is_null() {
            break;
        }
        n = n.wrapping_add(1);
    }
    g_mutex_lock(&raw mut (*connection).lock);
    safe_c2rust_free_filter_list(filters);
    g_mutex_unlock(&raw mut (*connection).lock);
    g_object_unref(connection as gpointer);
    return message;
}
unsafe extern "C" fn safe_c2rust_cancel_method_on_close(
    mut key: gpointer,
    mut value: gpointer,
    mut user_data: gpointer,
) -> gboolean {
    let mut task: *mut GTask = value as *mut GTask;
    let mut data: *mut SendMessageData = g_task_get_task_data(task) as *mut SendMessageData;
    if (*data).delivered != 0 {
        return FALSE;
    }
    g_task_return_new_error_literal(
        task,
        g_io_error_quark(),
        G_IO_ERROR_CLOSED as ::core::ffi::c_int as gint,
        glib_gettext(b"The connection is closed\0" as *const u8 as *const gchar)
            as *const ::core::ffi::c_char,
    );
    safe_c2rust_send_message_with_reply_cleanup(task, FALSE);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_on_worker_closed(
    mut worker: *mut GDBusWorker,
    mut remote_peer_vanished: gboolean,
    mut error: *mut GError,
    mut user_data: gpointer,
) {
    let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    let mut alive: gboolean = 0;
    let mut old_atomic_flags: guint = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    alive = g_hash_table_contains(safe_c2rust_alive_connections, user_data as gconstpointer);
    if alive == 0 {
        g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
        return;
    }
    connection = user_data as *mut GDBusConnection;
    g_object_ref(connection as gpointer);
    g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    g_mutex_lock(&raw mut (*connection).lock);
    old_atomic_flags = ({
        if 0 as ::core::ffi::c_int != 0 {
            (*connection).atomic_flags;
        } else {
        };
        crate::translated::compat::atomic_or_seqcst(
            &raw mut (*connection).atomic_flags,
            FLAG_CLOSED as ::core::ffi::c_int,
        ) as guint
    });
    if old_atomic_flags & FLAG_CLOSED as ::core::ffi::c_int as guint == 0 {
        g_hash_table_foreach_remove(
            (*connection).map_method_serial_to_task,
            Some(
                safe_c2rust_cancel_method_on_close
                    as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
            ),
            NULL_0,
        );
        safe_c2rust_schedule_closed_unlocked(connection, remote_peer_vanished, error);
    }
    g_mutex_unlock(&raw mut (*connection).lock);
    g_object_unref(connection as gpointer);
}
unsafe extern "C" fn safe_c2rust_get_offered_capabilities_max(
    mut connection: *mut GDBusConnection,
) -> GDBusCapabilityFlags {
    let mut ret: GDBusCapabilityFlags = G_DBUS_CAPABILITY_FLAGS_NONE;
    ret = G_DBUS_CAPABILITY_FLAGS_NONE;
    if ({
        let mut __inst: *mut GTypeInstance = (*connection).stream as *mut GTypeInstance;
        let mut __t: GType = g_unix_connection_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) != 0
    {
        ret = ::core::mem::transmute::<::core::ffi::c_uint, GDBusCapabilityFlags>(
            ret as ::core::ffi::c_uint
                | G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
        );
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_initable_init(
    mut initable: *mut GInitable,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut connection: *mut GDBusConnection =
        initable as *mut ::core::ffi::c_void as *mut GDBusConnection;
    let mut ret: gboolean = 0;
    g_mutex_lock(&raw mut (*connection).init_lock);
    ret = FALSE as gboolean;
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*connection).atomic_flags;
            (*connection).atomic_flags;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*connection).atomic_flags);
        gaig_temp
    }) & FLAG_INITIALIZED as ::core::ffi::c_int
        != 0
    {
        ret = ((*connection).initialization_error == NULL_0 as *mut GError) as ::core::ffi::c_int
            as gboolean;
    } else {
        if ({
            let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
            if (*connection).initialization_error.is_null() {
                _g_boolean_var_88 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_88 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_88
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                2848 as ::core::ffi::c_int,
                G_STRFUNC,
                b"connection->initialization_error == NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if !(*connection).address.is_null() {
            if ({
                let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
                if (*connection).stream.is_null() {
                    _g_boolean_var_89 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_89 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_89
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2861 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"connection->stream == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if (*connection).flags as ::core::ffi::c_uint
                & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
                || (*connection).flags as ::core::ffi::c_uint
                    & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                || (*connection).flags as ::core::ffi::c_uint
                    & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
            {
                g_set_error_literal(
                    &raw mut (*connection).initialization_error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Unsupported flags encountered when constructing a client-side connection\0"
                            as *const u8 as *const gchar,
                    ),
                );
                current_block = 10326684607915539853;
            } else {
                (*connection).stream = g_dbus_address_get_stream_sync(
                    (*connection).address,
                    ::core::ptr::null_mut::<*mut gchar>(),
                    cancellable,
                    &raw mut (*connection).initialization_error,
                );
                if (*connection).stream.is_null() {
                    current_block = 10326684607915539853;
                } else {
                    current_block = 2668756484064249700;
                }
            }
        } else {
            if !(*connection).stream.is_null() {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2887 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
            current_block = 2668756484064249700;
        }
        match current_block {
            10326684607915539853 => {}
            _ => {
                if (*connection).flags as ::core::ffi::c_uint
                    & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                {
                    if ({
                        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
                        if (*connection).flags as ::core::ffi::c_uint
                            & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            == 0
                        {
                            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_90
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            2893 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"!(connection->flags & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    if ({
                        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
                        if !(*connection).guid.is_null() {
                            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_91
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            2894 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"connection->guid != NULL\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    (*connection).auth = _g_dbus_auth_new((*connection).stream);
                    if _g_dbus_auth_run_server(
                        (*connection).auth,
                        (*connection).authentication_observer,
                        (*connection).guid,
                        ((*connection).flags as ::core::ffi::c_uint
                            & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint) as gboolean,
                        ((*connection).flags as ::core::ffi::c_uint
                            & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint) as gboolean,
                        safe_c2rust_get_offered_capabilities_max(connection),
                        &raw mut (*connection).capabilities,
                        &raw mut (*connection).credentials,
                        cancellable,
                        &raw mut (*connection).initialization_error,
                    ) == 0
                    {
                        current_block = 10326684607915539853;
                    } else {
                        current_block = 6717214610478484138;
                    }
                } else if (*connection).flags as ::core::ffi::c_uint
                    & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                {
                    if ({
                        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
                        if (*connection).flags as ::core::ffi::c_uint
                            & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            == 0
                        {
                            _g_boolean_var_92 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_92 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_92
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            2910 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"!(connection->flags & G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    if ({
                        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
                        if (*connection).guid.is_null() {
                            _g_boolean_var_93 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_93 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_93
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            2911 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"connection->guid == NULL\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    (*connection).auth = _g_dbus_auth_new((*connection).stream);
                    (*connection).guid = _g_dbus_auth_run_client(
                        (*connection).auth,
                        (*connection).authentication_observer,
                        (*connection).flags,
                        safe_c2rust_get_offered_capabilities_max(connection),
                        &raw mut (*connection).capabilities,
                        cancellable,
                        &raw mut (*connection).initialization_error,
                    );
                    if (*connection).guid.is_null() {
                        current_block = 10326684607915539853;
                    } else {
                        current_block = 6717214610478484138;
                    }
                } else {
                    current_block = 6717214610478484138;
                }
                match current_block {
                    10326684607915539853 => {}
                    _ => {
                        if !(*connection).authentication_observer.is_null() {
                            g_object_unref((*connection).authentication_observer as gpointer);
                            (*connection).authentication_observer =
                                ::core::ptr::null_mut::<GDBusAuthObserver>();
                        }
                        if ({
                            let mut __inst: *mut GTypeInstance =
                                (*connection).stream as *mut GTypeInstance;
                            let mut __t: GType = g_socket_connection_get_type();
                            let mut __r: gboolean = 0;
                            if __inst.is_null() {
                                __r = FALSE as gboolean;
                            } else if !(*__inst).g_class.is_null()
                                && (*(*__inst).g_class).g_type == __t
                            {
                                __r = TRUE as gboolean;
                            } else {
                                __r = g_type_check_instance_is_a(__inst, __t);
                            }
                            __r
                        }) != 0
                        {
                            g_socket_set_blocking(
                                g_socket_connection_get_socket(
                                    (*connection).stream as *mut ::core::ffi::c_void
                                        as *mut GSocketConnection,
                                ),
                                FALSE,
                            );
                        }
                        g_mutex_lock(&raw mut safe_c2rust_g__message_bus_lock_lock);
                        if safe_c2rust_alive_connections.is_null() {
                            safe_c2rust_alive_connections = g_hash_table_new(
                                Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                                Some(
                                    g_direct_equal
                                        as unsafe extern "C" fn(
                                            gconstpointer,
                                            gconstpointer,
                                        )
                                            -> gboolean,
                                ),
                            );
                        }
                        g_hash_table_add(safe_c2rust_alive_connections, connection as gpointer);
                        g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
                        (*connection).worker = _g_dbus_worker_new(
                            (*connection).stream,
                            (*connection).capabilities,
                            ((*connection).flags as ::core::ffi::c_uint
                                & G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                != 0 as ::core::ffi::c_uint)
                                as ::core::ffi::c_int,
                            Some(
                                safe_c2rust_on_worker_message_received
                                    as unsafe extern "C" fn(
                                        *mut GDBusWorker,
                                        *mut GDBusMessage,
                                        gpointer,
                                    )
                                        -> (),
                            ),
                            Some(
                                safe_c2rust_on_worker_message_about_to_be_sent
                                    as unsafe extern "C" fn(
                                        *mut GDBusWorker,
                                        *mut GDBusMessage,
                                        gpointer,
                                    )
                                        -> *mut GDBusMessage,
                            ),
                            Some(
                                safe_c2rust_on_worker_closed
                                    as unsafe extern "C" fn(
                                        *mut GDBusWorker,
                                        gboolean,
                                        *mut GError,
                                        gpointer,
                                    )
                                        -> (),
                            ),
                            connection as gpointer,
                        );
                        if (*connection).flags as ::core::ffi::c_uint
                            & G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            != 0
                        {
                            let mut hello_result: *mut GVariant =
                                ::core::ptr::null_mut::<GVariant>();
                            if (*connection).flags as ::core::ffi::c_uint
                                & G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING
                                    as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                != 0
                            {
                                g_set_error_literal(
                                    &raw mut (*connection).initialization_error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                                    b"Cannot use DELAY_MESSAGE_PROCESSING with MESSAGE_BUS_CONNECTION\0"
                                        as *const u8 as *const gchar,
                                );
                                current_block = 10326684607915539853;
                            } else {
                                hello_result = safe_c2rust_g_dbus_connection_call_sync(
                                    connection,
                                    b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
                                    b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
                                    b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
                                    b"Hello\0" as *const u8 as *const gchar,
                                    ::core::ptr::null_mut::<GVariant>(),
                                    g_variant_type_checked_(b"(s)\0" as *const u8 as *const gchar),
                                    CALL_FLAGS_INITIALIZING as GDBusCallFlags,
                                    -(1 as gint),
                                    ::core::ptr::null_mut::<GCancellable>(),
                                    &raw mut (*connection).initialization_error,
                                );
                                if hello_result.is_null() {
                                    current_block = 10326684607915539853;
                                } else {
                                    g_variant_get(
                                        hello_result,
                                        b"(s)\0" as *const u8 as *const gchar,
                                        &raw mut (*connection).bus_unique_name,
                                    );
                                    g_variant_unref(hello_result);
                                    current_block = 2706659501864706830;
                                }
                            }
                        } else {
                            current_block = 2706659501864706830;
                        }
                        match current_block {
                            10326684607915539853 => {}
                            _ => {
                                ret = TRUE as gboolean;
                            }
                        }
                    }
                }
            }
        }
    }
    if ret == 0 {
        if ({
            let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
            if !(*connection).initialization_error.is_null() {
                _g_boolean_var_94 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_94 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_94
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                2996 as ::core::ffi::c_int,
                G_STRFUNC,
                b"connection->initialization_error != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        g_propagate_error(error, g_error_copy((*connection).initialization_error));
    }
    if g_error_matches(
        (*connection).initialization_error,
        g_io_error_quark(),
        G_IO_ERROR_CANCELLED as ::core::ffi::c_int as gint,
    ) != 0
    {
        if !(*connection).worker.is_null() {
            _g_dbus_worker_stop((*connection).worker);
            (*connection).worker = ::core::ptr::null_mut::<GDBusWorker>();
            if !safe_c2rust_alive_connections.is_null() {
                if !(({
                    let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
                    if g_hash_table_remove(
                        safe_c2rust_alive_connections,
                        connection as gconstpointer,
                    ) != 0
                    {
                        _g_boolean_var_95 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_95 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_95
                }) as ::core::ffi::c_long
                    != 0)
                {
                    g_warn_message(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        3009 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"g_hash_table_remove (alive_connections, connection)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        g_clear_error(&raw mut (*connection).initialization_error);
        let mut _pp: *mut *mut GIOStream = &raw mut (*connection).stream;
        let mut _ptr: *mut GIOStream = *_pp;
        *_pp = ::core::ptr::null_mut::<GIOStream>();
        if !_ptr.is_null() {
            g_object_unref(_ptr as gpointer);
        }
        let mut _pp_0: *mut *mut GDBusAuth = &raw mut (*connection).auth;
        let mut _ptr_0: *mut GDBusAuth = *_pp_0;
        *_pp_0 = ::core::ptr::null_mut::<GDBusAuth>();
        if !_ptr_0.is_null() {
            g_object_unref(_ptr_0 as gpointer);
        }
        let mut _pp_1: *mut *mut GCredentials = &raw mut (*connection).credentials;
        let mut _ptr_1: *mut GCredentials = *_pp_1;
        *_pp_1 = ::core::ptr::null_mut::<GCredentials>();
        if !_ptr_1.is_null() {
            g_object_unref(_ptr_1 as gpointer);
        }
        let mut _pp_2: *mut *mut gchar = &raw mut (*connection).guid;
        let mut _ptr_2: *mut gchar = *_pp_2;
        *_pp_2 = ::core::ptr::null_mut::<gchar>();
        if !_ptr_2.is_null() {
            g_free(_ptr_2 as gpointer);
        }
        (*connection).capabilities = G_DBUS_CAPABILITY_FLAGS_NONE;
    } else {
        if 0 as ::core::ffi::c_int != 0 {
            (*connection).atomic_flags;
        } else {
        };
        crate::translated::compat::atomic_or_seqcst(
            &raw mut (*connection).atomic_flags,
            FLAG_INITIALIZED as ::core::ffi::c_int,
        );
    }
    g_mutex_unlock(&raw mut (*connection).init_lock);
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
unsafe extern "C" fn safe_c2rust_async_initable_iface_init(
    mut async_initable_iface: *mut GAsyncInitableIface,
) {
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_new(
    mut stream: *mut GIOStream,
    mut guid: *const gchar,
    mut flags: GDBusConnectionFlags,
    mut observer: *mut GDBusAuthObserver,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    _g_dbus_initialize();
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = g_io_stream_get_type();
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
            _g_boolean_var_96 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_96 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_96
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & !(G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            _g_boolean_var_97 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_97 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_97
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(flags & ~G_DBUS_CONNECTION_FLAGS_ALL) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_async_initable_new_async(
        safe_c2rust_g_dbus_connection_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"stream\0" as *const u8 as *const gchar,
        stream,
        b"guid\0" as *const u8 as *const ::core::ffi::c_char,
        guid,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        b"authentication-observer\0" as *const u8 as *const ::core::ffi::c_char,
        observer,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_new_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GDBusConnection {
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            _g_boolean_var_98 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_98 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_98
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_99 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_99 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_99
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    source_object = g_async_result_get_source_object(res);
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if !source_object.is_null() {
            _g_boolean_var_100 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_100 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_100
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            3124 as ::core::ffi::c_int,
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
        return object as *mut ::core::ffi::c_void as *mut GDBusConnection;
    } else {
        return ::core::ptr::null_mut::<GDBusConnection>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_new_sync(
    mut stream: *mut GIOStream,
    mut guid: *const gchar,
    mut flags: GDBusConnectionFlags,
    mut observer: *mut GDBusAuthObserver,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusConnection {
    _g_dbus_initialize();
    if ({
        let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = stream as *mut GTypeInstance;
            let mut __t: GType = g_io_stream_get_type();
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
            _g_boolean_var_101 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_101 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_101
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_IO_STREAM (stream)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    if ({
        let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & !(G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            _g_boolean_var_102 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_102 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_102
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(flags & ~G_DBUS_CONNECTION_FLAGS_ALL) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    if ({
        let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_103 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_103 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_103
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    return g_initable_new(
        safe_c2rust_g_dbus_connection_get_type(),
        cancellable,
        error,
        b"stream\0" as *const u8 as *const gchar,
        stream,
        b"guid\0" as *const u8 as *const ::core::ffi::c_char,
        guid,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        b"authentication-observer\0" as *const u8 as *const ::core::ffi::c_char,
        observer,
        NULL_0,
    ) as *mut GDBusConnection;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_new_for_address(
    mut address: *const gchar,
    mut flags: GDBusConnectionFlags,
    mut observer: *mut GDBusAuthObserver,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    _g_dbus_initialize();
    if ({
        let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
        if !address.is_null() {
            _g_boolean_var_104 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_104 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_104
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"address != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_105: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & !(G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            _g_boolean_var_105 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_105 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_105
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(flags & ~G_DBUS_CONNECTION_FLAGS_ALL) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_async_initable_new_async(
        safe_c2rust_g_dbus_connection_get_type(),
        G_PRIORITY_DEFAULT,
        cancellable,
        callback,
        user_data,
        b"address\0" as *const u8 as *const gchar,
        address,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        b"authentication-observer\0" as *const u8 as *const ::core::ffi::c_char,
        observer,
        NULL_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_new_for_address_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GDBusConnection {
    let mut object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    let mut source_object: *mut GObject = ::core::ptr::null_mut::<GObject>();
    if ({
        let mut _g_boolean_var_106: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = res as *mut GTypeInstance;
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
            _g_boolean_var_106 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_106 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_106
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ASYNC_RESULT (res)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    if ({
        let mut _g_boolean_var_107: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_107 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_107 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_107
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    source_object = g_async_result_get_source_object(res);
    if ({
        let mut _g_boolean_var_108: ::core::ffi::c_int = 0;
        if !source_object.is_null() {
            _g_boolean_var_108 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_108 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_108
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            3271 as ::core::ffi::c_int,
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
        return object as *mut ::core::ffi::c_void as *mut GDBusConnection;
    } else {
        return ::core::ptr::null_mut::<GDBusConnection>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_new_for_address_sync(
    mut address: *const gchar,
    mut flags: GDBusConnectionFlags,
    mut observer: *mut GDBusAuthObserver,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusConnection {
    _g_dbus_initialize();
    if ({
        let mut _g_boolean_var_109: ::core::ffi::c_int = 0;
        if !address.is_null() {
            _g_boolean_var_109 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_109 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_109
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"address != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    if ({
        let mut _g_boolean_var_110: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            & !(G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_SERVER as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_ALLOW_ANONYMOUS as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_DELAY_MESSAGE_PROCESSING as ::core::ffi::c_int
                | G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_REQUIRE_SAME_USER as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            _g_boolean_var_110 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_110 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_110
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(flags & ~G_DBUS_CONNECTION_FLAGS_ALL) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    if ({
        let mut _g_boolean_var_111: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_111 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_111 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_111
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    return g_initable_new(
        safe_c2rust_g_dbus_connection_get_type(),
        cancellable,
        error,
        b"address\0" as *const u8 as *const gchar,
        address,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        b"authentication-observer\0" as *const u8 as *const ::core::ffi::c_char,
        observer,
        NULL_0,
    ) as *mut GDBusConnection;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_set_exit_on_close(
    mut connection: *mut GDBusConnection,
    mut exit_on_close: gboolean,
) {
    if ({
        let mut _g_boolean_var_112: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_112 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_112 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_112
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
    if exit_on_close != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            (*connection).atomic_flags;
        } else {
        };
        crate::translated::compat::atomic_or_seqcst(
            &raw mut (*connection).atomic_flags,
            FLAG_EXIT_ON_CLOSE as ::core::ffi::c_int,
        );
    } else {
        if 0 as ::core::ffi::c_int != 0 {
            (*connection).atomic_flags;
            !(FLAG_EXIT_ON_CLOSE as ::core::ffi::c_int);
        } else {
        };
        crate::translated::compat::atomic_and_seqcst(
            &raw mut (*connection).atomic_flags,
            !(FLAG_EXIT_ON_CLOSE as ::core::ffi::c_int),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_exit_on_close(
    mut connection: *mut GDBusConnection,
) -> gboolean {
    if ({
        let mut _g_boolean_var_113: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_113 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_113 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_113
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
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            (*connection).atomic_flags;
            (*connection).atomic_flags;
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut (*connection).atomic_flags);
        gaig_temp
    }) & FLAG_EXIT_ON_CLOSE as ::core::ffi::c_int
        != 0
    {
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_guid(
    mut connection: *mut GDBusConnection,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_114: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_114 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_114 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_114
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*connection).guid;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_unique_name(
    mut connection: *mut GDBusConnection,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_115: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_115 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_115 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_115
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if safe_c2rust_check_initialized(connection) == 0 {
        return ::core::ptr::null::<gchar>();
    }
    return (*connection).bus_unique_name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_get_peer_credentials(
    mut connection: *mut GDBusConnection,
) -> *mut GCredentials {
    if ({
        let mut _g_boolean_var_116: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_116 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_116 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_116
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GCredentials>();
    }
    if safe_c2rust_check_initialized(connection) == 0 {
        return ::core::ptr::null_mut::<GCredentials>();
    }
    return (*connection).credentials;
}
static mut safe_c2rust__global_filter_id: guint = 1 as guint;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_add_filter(
    mut connection: *mut GDBusConnection,
    mut filter_function: GDBusMessageFilterFunction,
    mut user_data: gpointer,
    mut user_data_free_func: GDestroyNotify,
) -> guint {
    let mut data: *mut FilterData = ::core::ptr::null_mut::<FilterData>();
    if ({
        let mut _g_boolean_var_117: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_117 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_117 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_117
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_118: ::core::ffi::c_int = 0;
        if filter_function.is_some() {
            _g_boolean_var_118 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_118 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_118
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"filter_function != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_119: ::core::ffi::c_int = 0;
        if safe_c2rust_check_initialized(connection) != 0 {
            _g_boolean_var_119 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_119 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_119
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    g_mutex_lock(&raw mut (*connection).lock);
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<FilterData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut FilterData;
    (*data).id = ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust__global_filter_id;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            &raw mut safe_c2rust__global_filter_id,
            1 as ::core::ffi::c_int as guint,
        ) as gint
    }) as guint;
    (*data).ref_count = 1 as guint;
    (*data).filter_function = filter_function;
    (*data).user_data = user_data;
    (*data).user_data_free_func = user_data_free_func;
    (*data).context = g_main_context_ref_thread_default();
    g_ptr_array_add((*connection).filters, data as gpointer);
    g_mutex_unlock(&raw mut (*connection).lock);
    return (*data).id;
}
unsafe extern "C" fn safe_c2rust_purge_all_filters(mut connection: *mut GDBusConnection) {
    let mut n: guint = 0;
    n = 0 as guint;
    while n < (*(*connection).filters).len {
        safe_c2rust_filter_data_destroy(
            *(*(*connection).filters).pdata.offset(n as isize) as *mut FilterData,
            FALSE,
        );
        n = n.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_remove_filter(
    mut connection: *mut GDBusConnection,
    mut filter_id: guint,
) {
    let mut n: guint = 0;
    let mut found: gboolean = 0;
    let mut to_destroy: *mut FilterData = ::core::ptr::null_mut::<FilterData>();
    if ({
        let mut _g_boolean_var_120: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_120 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_120 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_120
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
        let mut _g_boolean_var_121: ::core::ffi::c_int = 0;
        if safe_c2rust_check_initialized(connection) != 0 {
            _g_boolean_var_121 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_121 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_121
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*connection).lock);
    found = FALSE as gboolean;
    to_destroy = ::core::ptr::null_mut::<FilterData>();
    n = 0 as guint;
    while n < (*(*connection).filters).len {
        let mut data: *mut FilterData =
            *(*(*connection).filters).pdata.offset(n as isize) as *mut FilterData;
        if (*data).id == filter_id {
            found = TRUE as gboolean;
            g_ptr_array_remove_index((*connection).filters, n);
            (*data).ref_count = (*data).ref_count.wrapping_sub(1);
            if (*data).ref_count == 0 as guint {
                to_destroy = data;
            }
            break;
        } else {
            n = n.wrapping_add(1);
        }
    }
    g_mutex_unlock(&raw mut (*connection).lock);
    if !to_destroy.is_null() {
        safe_c2rust_filter_data_destroy(to_destroy, TRUE);
    } else if found == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"g_dbus_connection_remove_filter: No filter found for filter_id %d\0" as *const u8
                as *const gchar,
            filter_id,
        );
    }
}
unsafe extern "C" fn safe_c2rust_args_to_rule(
    mut sender: *const gchar,
    mut interface_name: *const gchar,
    mut member: *const gchar,
    mut object_path: *const gchar,
    mut arg0: *const gchar,
    mut flags: GDBusSignalFlags,
) -> *mut gchar {
    let mut rule: *mut GString = ::core::ptr::null_mut::<GString>();
    rule = g_string_new(b"type='signal'\0" as *const u8 as *const gchar);
    if flags as ::core::ffi::c_uint
        & G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        g_string_prepend_c(rule, '-' as i32 as gchar);
    }
    if !sender.is_null() {
        g_string_append_printf(rule, b",sender='%s'\0" as *const u8 as *const gchar, sender);
    }
    if !interface_name.is_null() {
        g_string_append_printf(
            rule,
            b",interface='%s'\0" as *const u8 as *const gchar,
            interface_name,
        );
    }
    if !member.is_null() {
        g_string_append_printf(rule, b",member='%s'\0" as *const u8 as *const gchar, member);
    }
    if !object_path.is_null() {
        g_string_append_printf(
            rule,
            b",path='%s'\0" as *const u8 as *const gchar,
            object_path,
        );
    }
    if !arg0.is_null() {
        if flags as ::core::ffi::c_uint
            & G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            g_string_append_printf(rule, b",arg0path='%s'\0" as *const u8 as *const gchar, arg0);
        } else if flags as ::core::ffi::c_uint
            & G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            g_string_append_printf(
                rule,
                b",arg0namespace='%s'\0" as *const u8 as *const gchar,
                arg0,
            );
        } else {
            g_string_append_printf(rule, b",arg0='%s'\0" as *const u8 as *const gchar, arg0);
        }
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(rule, 0 as gboolean)
        } else {
            g_string_free_and_steal(rule)
        }
    } else {
        g_string_free(rule, 0 as gboolean)
    };
}
static mut safe_c2rust__global_subscriber_id: guint = 1 as guint;
static mut safe_c2rust__global_registration_id: guint = 1 as guint;
static mut safe_c2rust__global_subtree_registration_id: guint = 1 as guint;
unsafe extern "C" fn safe_c2rust_add_match_rule(
    mut connection: *mut GDBusConnection,
    mut match_rule: *const gchar,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    if *match_rule.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32 {
        return;
    }
    message = g_dbus_message_new_method_call(
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"AddMatch\0" as *const u8 as *const gchar,
    );
    g_dbus_message_set_body(
        message,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, match_rule),
    );
    error = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_dbus_connection_send_message_unlocked(
        connection,
        message,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        ::core::ptr::null_mut::<guint32>(),
        &raw mut error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Error while sending AddMatch() message: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
    }
    g_object_unref(message as gpointer);
}
unsafe extern "C" fn safe_c2rust_remove_match_rule(
    mut connection: *mut GDBusConnection,
    mut match_rule: *const gchar,
) {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    if *match_rule.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32 {
        return;
    }
    message = g_dbus_message_new_method_call(
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"/org/freedesktop/DBus\0" as *const u8 as *const gchar,
        b"org.freedesktop.DBus\0" as *const u8 as *const gchar,
        b"RemoveMatch\0" as *const u8 as *const gchar,
    );
    g_dbus_message_set_body(
        message,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, match_rule),
    );
    error = ::core::ptr::null_mut::<GError>();
    if safe_c2rust_g_dbus_connection_send_message_unlocked(
        connection,
        message,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        ::core::ptr::null_mut::<guint32>(),
        &raw mut error,
    ) == 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Error while sending RemoveMatch() message: %s\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_error_free(error);
    }
    g_object_unref(message as gpointer);
}
unsafe extern "C" fn safe_c2rust_is_signal_data_for_name_lost_or_acquired(
    mut signal_data: *mut SignalData,
) -> gboolean {
    return (g_strcmp0(
        (*signal_data).sender,
        b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            (*signal_data).interface_name,
            b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            (*signal_data).object_path,
            b"/org/freedesktop/DBus\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && (g_strcmp0(
            (*signal_data).member,
            b"NameLost\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || g_strcmp0(
                (*signal_data).member,
                b"NameAcquired\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_add_signal_data(
    mut connection: *mut GDBusConnection,
    mut signal_data: *mut SignalData,
    mut sender_unique_name: *const ::core::ffi::c_char,
) {
    let mut signal_data_array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    g_hash_table_insert(
        (*connection).map_rule_to_signal_data,
        (*signal_data).rule as gpointer,
        signal_data as gpointer,
    );
    if (*connection).flags as ::core::ffi::c_uint
        & G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
            as ::core::ffi::c_uint
        != 0
    {
        if safe_c2rust_is_signal_data_for_name_lost_or_acquired(signal_data) == 0 {
            safe_c2rust_add_match_rule(connection, (*signal_data).rule);
        }
    }
    signal_data_array = g_hash_table_lookup(
        (*connection).map_sender_unique_name_to_signal_data_array,
        sender_unique_name as gconstpointer,
    ) as *mut GPtrArray;
    if signal_data_array.is_null() {
        signal_data_array = g_ptr_array_new();
        g_hash_table_insert(
            (*connection).map_sender_unique_name_to_signal_data_array,
            safe_c2rust_g_strdup_inline(sender_unique_name) as gpointer,
            signal_data_array as gpointer,
        );
    }
    g_ptr_array_add(signal_data_array, signal_data as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_signal_subscribe(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut interface_name: *const gchar,
    mut member: *const gchar,
    mut object_path: *const gchar,
    mut arg0: *const gchar,
    mut flags: GDBusSignalFlags,
    mut callback: GDBusSignalCallback,
    mut user_data: gpointer,
    mut user_data_free_func: GDestroyNotify,
) -> guint {
    let mut rule: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut signal_data: *mut SignalData = ::core::ptr::null_mut::<SignalData>();
    let mut name_watcher: *mut SignalData = ::core::ptr::null_mut::<SignalData>();
    let mut subscriber: *mut SignalSubscriber = ::core::ptr::null_mut::<SignalSubscriber>();
    let mut sender_is_its_own_owner: gboolean = 0;
    let mut sender_unique_name: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_122: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_122 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_122 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_122
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_123: ::core::ffi::c_int = 0;
        if sender.is_null()
            || g_dbus_is_name(sender) != 0
                && (*connection).flags as ::core::ffi::c_uint
                    & G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
        {
            _g_boolean_var_123 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_123 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_123
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"sender == NULL || (g_dbus_is_name (sender) && (connection->flags & G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_124: ::core::ffi::c_int = 0;
        if interface_name.is_null() || g_dbus_is_interface_name(interface_name) != 0 {
            _g_boolean_var_124 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_124 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_124
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"interface_name == NULL || g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_125: ::core::ffi::c_int = 0;
        if member.is_null() || g_dbus_is_member_name(member) != 0 {
            _g_boolean_var_125 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_125 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_125
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"member == NULL || g_dbus_is_member_name (member)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_126: ::core::ffi::c_int = 0;
        if object_path.is_null() || g_variant_is_object_path(object_path) != 0 {
            _g_boolean_var_126 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_126 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_126
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"object_path == NULL || g_variant_is_object_path (object_path)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_127: ::core::ffi::c_int = 0;
        if callback.is_some() {
            _g_boolean_var_127 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_127 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_127
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"callback != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_128: ::core::ffi::c_int = 0;
        if safe_c2rust_check_initialized(connection) != 0 {
            _g_boolean_var_128 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_128 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_128
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_129: ::core::ffi::c_int = 0;
        if !(flags as ::core::ffi::c_uint
            & G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && flags as ::core::ffi::c_uint
                & G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0)
        {
            _g_boolean_var_129 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_129 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_129
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!((flags & G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH) && (flags & G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_130: ::core::ffi::c_int = 0;
        if !(arg0.is_null()
            && flags as ::core::ffi::c_uint
                & (G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH as ::core::ffi::c_int
                    | G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
                != 0)
        {
            _g_boolean_var_130 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_130 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_130
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!(arg0 == NULL && (flags & (G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH | G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE)))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    g_mutex_lock(&raw mut (*connection).lock);
    rule = safe_c2rust_args_to_rule(sender, interface_name, member, object_path, arg0, flags);
    if !sender.is_null()
        && (g_dbus_is_unique_name(sender) != 0
            || g_strcmp0(
                sender as *const ::core::ffi::c_char,
                b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
    {
        sender_is_its_own_owner = TRUE as gboolean;
    } else {
        sender_is_its_own_owner = FALSE as gboolean;
    }
    if sender_is_its_own_owner != 0 {
        sender_unique_name = sender;
    } else {
        sender_unique_name = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    subscriber = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<SignalSubscriber>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut SignalSubscriber;
    (*subscriber).ref_count = 1 as ::core::ffi::c_int as gatomicrefcount;
    (*subscriber).callback = callback;
    (*subscriber).user_data = user_data;
    (*subscriber).user_data_free_func = user_data_free_func;
    (*subscriber).id = ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust__global_subscriber_id;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            &raw mut safe_c2rust__global_subscriber_id,
            1 as ::core::ffi::c_int as guint,
        ) as gint
    }) as guint;
    (*subscriber).context = g_main_context_ref_thread_default();
    signal_data = g_hash_table_lookup((*connection).map_rule_to_signal_data, rule as gconstpointer)
        as *mut SignalData;
    if !signal_data.is_null() {
        g_ptr_array_add((*signal_data).subscribers, subscriber as gpointer);
        g_free(rule as gpointer);
    } else {
        signal_data = safe_c2rust_signal_data_new_take(
            safe_c2rust_g_steal_pointer(&raw mut rule as gpointer) as *mut gchar,
            safe_c2rust_g_strdup_inline(sender as *const ::core::ffi::c_char) as *mut gchar,
            safe_c2rust_g_strdup_inline(interface_name as *const ::core::ffi::c_char) as *mut gchar,
            safe_c2rust_g_strdup_inline(member as *const ::core::ffi::c_char) as *mut gchar,
            safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as *mut gchar,
            safe_c2rust_g_strdup_inline(arg0 as *const ::core::ffi::c_char) as *mut gchar,
            flags,
        );
        g_ptr_array_add((*signal_data).subscribers, subscriber as gpointer);
        if !sender.is_null() && sender_is_its_own_owner == 0 {
            let mut name_owner_rule: *mut gchar = ::core::ptr::null_mut::<gchar>();
            if ({
                let mut _g_boolean_var_131: ::core::ffi::c_int = 0;
                if (*connection).flags as ::core::ffi::c_uint
                    & G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                {
                    _g_boolean_var_131 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_131 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_131
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    3932 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"connection->flags & G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            name_owner_rule = safe_c2rust_args_to_rule(
                DBUS_SERVICE_DBUS.as_ptr() as *const gchar,
                DBUS_INTERFACE_DBUS.as_ptr() as *const gchar,
                b"NameOwnerChanged\0" as *const u8 as *const gchar,
                DBUS_PATH_DBUS.as_ptr() as *const gchar,
                sender,
                G_DBUS_SIGNAL_FLAGS_NONE,
            );
            name_watcher = g_hash_table_lookup(
                (*connection).map_rule_to_signal_data,
                name_owner_rule as gconstpointer,
            ) as *mut SignalData;
            if name_watcher.is_null() {
                name_watcher = safe_c2rust_signal_data_new_take(
                    safe_c2rust_g_steal_pointer(&raw mut name_owner_rule as gpointer) as *mut gchar,
                    safe_c2rust_g_strdup_inline(
                        b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as *mut gchar,
                    safe_c2rust_g_strdup_inline(
                        b"org.freedesktop.DBus\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as *mut gchar,
                    safe_c2rust_g_strdup_inline(
                        b"NameOwnerChanged\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as *mut gchar,
                    safe_c2rust_g_strdup_inline(
                        b"/org/freedesktop/DBus\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as *mut gchar,
                    safe_c2rust_g_strdup_inline(sender as *const ::core::ffi::c_char) as *mut gchar,
                    G_DBUS_SIGNAL_FLAGS_NONE,
                );
                safe_c2rust_add_signal_data(connection, name_watcher, DBUS_SERVICE_DBUS.as_ptr());
            }
            if (*name_watcher).watched_name.is_null() {
                (*name_watcher).watched_name = safe_c2rust_watched_name_new();
                safe_c2rust_name_watcher_call_get_name_owner_unlocked(connection, name_watcher);
            } else {
                g_ref_count_inc(&raw mut (*(*name_watcher).watched_name).ref_count);
            }
            (*signal_data).shared_name_watcher = name_watcher;
            let mut _pp: *mut *mut gchar = &raw mut name_owner_rule;
            let mut _ptr: *mut gchar = *_pp;
            *_pp = ::core::ptr::null_mut::<gchar>();
            if !_ptr.is_null() {
                g_free(_ptr as gpointer);
            }
        }
        safe_c2rust_add_signal_data(
            connection,
            signal_data,
            sender_unique_name as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_insert(
        (*connection).map_id_to_signal_data,
        (*subscriber).id as gulong as gpointer,
        signal_data as gpointer,
    );
    g_mutex_unlock(&raw mut (*connection).lock);
    return (*subscriber).id;
}
unsafe extern "C" fn safe_c2rust_remove_signal_data_if_unused(
    mut connection: *mut GDBusConnection,
    mut signal_data: *mut SignalData,
) {
    let mut sender_unique_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut signal_data_array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    if (*(*signal_data).subscribers).len != 0 as guint {
        return;
    }
    if !(*signal_data).watched_name.is_null() {
        return;
    }
    if !(*signal_data).sender.is_null() && (*signal_data).shared_name_watcher.is_null() {
        sender_unique_name = (*signal_data).sender;
    } else {
        sender_unique_name = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    if !(({
        let mut _g_boolean_var_132: ::core::ffi::c_int = 0;
        if g_hash_table_remove(
            (*connection).map_rule_to_signal_data,
            (*signal_data).rule as gconstpointer,
        ) != 0
        {
            _g_boolean_var_132 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_132 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_132
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            4012 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_hash_table_remove (connection->map_rule_to_signal_data, signal_data->rule)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    signal_data_array = g_hash_table_lookup(
        (*connection).map_sender_unique_name_to_signal_data_array,
        sender_unique_name as gconstpointer,
    ) as *mut GPtrArray;
    if !(({
        let mut _g_boolean_var_133: ::core::ffi::c_int = 0;
        if !signal_data_array.is_null() {
            _g_boolean_var_133 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_133 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_133
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            4016 as ::core::ffi::c_int,
            G_STRFUNC,
            b"signal_data_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(({
        let mut _g_boolean_var_134: ::core::ffi::c_int = 0;
        if g_ptr_array_remove(signal_data_array, signal_data as gpointer) != 0 {
            _g_boolean_var_134 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_134 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_134
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            4017 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_ptr_array_remove (signal_data_array, signal_data)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*signal_data_array).len == 0 as guint {
        if !(({
            let mut _g_boolean_var_135: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*connection).map_sender_unique_name_to_signal_data_array,
                sender_unique_name as gconstpointer,
            ) != 0
            {
                _g_boolean_var_135 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_135 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_135
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                4022 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (connection->map_sender_unique_name_to_signal_data_array, sender_unique_name)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if (*connection).flags as ::core::ffi::c_uint
        & G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int
            as ::core::ffi::c_uint
        != 0
        && safe_c2rust_is_signal_data_for_name_lost_or_acquired(signal_data) == 0
        && safe_c2rust_g_dbus_connection_is_closed(connection) == 0
        && (*connection).finalizing == 0
    {
        safe_c2rust_remove_match_rule(connection, (*signal_data).rule);
    }
    if !(*signal_data).shared_name_watcher.is_null() {
        let mut name_watcher: *mut SignalData =
            safe_c2rust_g_steal_pointer(&raw mut (*signal_data).shared_name_watcher as gpointer)
                as *mut SignalData;
        safe_c2rust_name_watcher_unref_watched_name(connection, name_watcher);
        safe_c2rust_remove_signal_data_if_unused(connection, name_watcher);
    }
    safe_c2rust_signal_data_free(signal_data);
}
unsafe extern "C" fn safe_c2rust_unsubscribe_id_internal(
    mut connection: *mut GDBusConnection,
    mut subscription_id: guint,
) -> guint {
    let mut current_block: u64;
    let mut signal_data: *mut SignalData = ::core::ptr::null_mut::<SignalData>();
    let mut n: guint = 0;
    let mut n_removed: guint = 0 as guint;
    signal_data = g_hash_table_lookup(
        (*connection).map_id_to_signal_data,
        subscription_id as gulong as gpointer as gconstpointer,
    ) as *mut SignalData;
    if !signal_data.is_null() {
        n = 0 as guint;
        loop {
            if !(n < (*(*signal_data).subscribers).len) {
                current_block = 13586036798005543211;
                break;
            }
            let mut subscriber: *mut SignalSubscriber =
                *(*(*signal_data).subscribers).pdata.offset(n as isize) as *mut SignalSubscriber;
            if (*subscriber).id != subscription_id {
                n = n.wrapping_add(1);
            } else {
                if !(({
                    let mut _g_boolean_var_136: ::core::ffi::c_int = 0;
                    if g_hash_table_remove(
                        (*connection).map_id_to_signal_data,
                        subscription_id as gulong as gpointer as gconstpointer,
                    ) != 0
                    {
                        _g_boolean_var_136 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_136 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_136
                }) as ::core::ffi::c_long
                    != 0)
                {
                    g_warn_message(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        4083 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"g_hash_table_remove (connection->map_id_to_signal_data, GUINT_TO_POINTER (subscription_id))\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                n_removed = n_removed.wrapping_add(1);
                g_ptr_array_remove_index_fast((*signal_data).subscribers, n);
                safe_c2rust_remove_signal_data_if_unused(connection, signal_data);
                current_block = 5414749123229727957;
                break;
            }
        }
        match current_block {
            5414749123229727957 => {}
            _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    4091 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
    }
    return n_removed;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_signal_unsubscribe(
    mut connection: *mut GDBusConnection,
    mut subscription_id: guint,
) {
    let mut n_subscribers_removed: guint = 0;
    if ({
        let mut _g_boolean_var_137: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_137 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_137 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_137
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
        let mut _g_boolean_var_138: ::core::ffi::c_int = 0;
        if safe_c2rust_check_initialized(connection) != 0 {
            _g_boolean_var_138 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_138 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_138
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*connection).lock);
    n_subscribers_removed = safe_c2rust_unsubscribe_id_internal(connection, subscription_id);
    g_mutex_unlock(&raw mut (*connection).lock);
    if ({
        let mut _g_boolean_var_139: ::core::ffi::c_int = 0;
        if n_subscribers_removed == 0 as guint || n_subscribers_removed == 1 as guint {
            _g_boolean_var_139 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_139 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_139
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            4132 as ::core::ffi::c_int,
            G_STRFUNC,
            b"n_subscribers_removed == 0 || n_subscribers_removed == 1\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn safe_c2rust_emit_signal_instance_in_idle_cb(mut data: gpointer) -> gboolean {
    let mut signal_instance: *mut SignalInstance = data as *mut SignalInstance;
    let mut parameters: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut has_subscription: gboolean = 0;
    parameters = g_dbus_message_get_body((*signal_instance).message);
    if parameters.is_null() {
        parameters = g_variant_new(b"()\0" as *const u8 as *const gchar);
        g_variant_ref_sink(parameters);
    } else {
        g_variant_ref_sink(parameters);
    }
    g_mutex_lock(&raw mut (*(*signal_instance).connection).lock);
    has_subscription = FALSE as gboolean;
    if !g_hash_table_lookup(
        (*(*signal_instance).connection).map_id_to_signal_data,
        (*(*signal_instance).subscriber).id as gulong as gpointer as gconstpointer,
    )
    .is_null()
    {
        has_subscription = TRUE as gboolean;
    }
    g_mutex_unlock(&raw mut (*(*signal_instance).connection).lock);
    if has_subscription != 0 {
        (*(*signal_instance).subscriber)
            .callback
            .expect("non-null function pointer")(
            (*signal_instance).connection,
            (*signal_instance).sender,
            (*signal_instance).path,
            (*signal_instance).interface,
            (*signal_instance).member,
            parameters,
            (*(*signal_instance).subscriber).user_data,
        );
    }
    g_variant_unref(parameters);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_signal_instance_free(mut signal_instance: *mut SignalInstance) {
    let mut _pp: *mut *mut GDBusMessage = &raw mut (*signal_instance).message;
    let mut _ptr: *mut GDBusMessage = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusMessage>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_object_unref((*signal_instance).connection as gpointer);
    safe_c2rust_signal_subscriber_unref((*signal_instance).subscriber);
    g_free(signal_instance as gpointer);
}
unsafe extern "C" fn safe_c2rust_namespace_rule_matches(
    mut namespace: *const gchar,
    mut name: *const gchar,
) -> gboolean {
    let mut len_namespace: gint = 0;
    let mut len_name: gint = 0;
    len_namespace = strlen(namespace as *const ::core::ffi::c_char) as gint;
    len_name = strlen(name as *const ::core::ffi::c_char) as gint;
    if len_name < len_namespace {
        return FALSE;
    }
    if memcmp(
        namespace as *const ::core::ffi::c_void,
        name as *const ::core::ffi::c_void,
        len_namespace as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    return (len_namespace == len_name
        || *name.offset(len_namespace as isize) as ::core::ffi::c_int == '.' as i32)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_path_rule_matches(
    mut path_a: *const gchar,
    mut path_b: *const gchar,
) -> gboolean {
    let mut len_a: gint = 0;
    let mut len_b: gint = 0;
    len_a = strlen(path_a as *const ::core::ffi::c_char) as gint;
    len_b = strlen(path_b as *const ::core::ffi::c_char) as gint;
    if len_a < len_b
        && (len_a == 0 as ::core::ffi::c_int
            || *path_a.offset((len_a as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                != '/' as i32)
    {
        return FALSE;
    }
    if len_b < len_a
        && (len_b == 0 as ::core::ffi::c_int
            || *path_b.offset((len_b as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                != '/' as i32)
    {
        return FALSE;
    }
    return (memcmp(
        path_a as *const ::core::ffi::c_void,
        path_b as *const ::core::ffi::c_void,
        (if len_a < len_b { len_a } else { len_b }) as size_t,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_schedule_callbacks(
    mut connection: *mut GDBusConnection,
    mut signal_data_array: *mut GPtrArray,
    mut message: *mut GDBusMessage,
    mut sender: *const gchar,
) {
    let mut n: guint = 0;
    let mut m: guint = 0;
    let mut interface: *const gchar = ::core::ptr::null::<gchar>();
    let mut member: *const gchar = ::core::ptr::null::<gchar>();
    let mut path: *const gchar = ::core::ptr::null::<gchar>();
    let mut arg0: *const gchar = ::core::ptr::null::<gchar>();
    let mut arg0_path: *const gchar = ::core::ptr::null::<gchar>();
    interface = ::core::ptr::null::<gchar>();
    member = ::core::ptr::null::<gchar>();
    path = ::core::ptr::null::<gchar>();
    arg0 = ::core::ptr::null::<gchar>();
    arg0_path = ::core::ptr::null::<gchar>();
    interface = g_dbus_message_get_interface(message);
    member = g_dbus_message_get_member(message);
    path = g_dbus_message_get_path(message);
    arg0 = g_dbus_message_get_arg0(message);
    arg0_path = g_dbus_message_get_arg0_path(message);
    if ({
        let mut _g_boolean_var_140: ::core::ffi::c_int = 0;
        if arg0.is_null() || arg0_path.is_null() {
            _g_boolean_var_140 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_140 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_140
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            4276 as ::core::ffi::c_int,
            G_STRFUNC,
            b"arg0 == NULL || arg0_path == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut current_block_89: u64;
    n = (*signal_data_array).len;
    while n > 0 as guint {
        n = n.wrapping_sub(1);
        let mut signal_data: *mut SignalData =
            *(*signal_data_array).pdata.offset(n as isize) as *mut SignalData;
        if !(!(*signal_data).interface_name.is_null()
            && g_strcmp0(
                (*signal_data).interface_name,
                interface as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int)
        {
            if !(!(*signal_data).member.is_null()
                && g_strcmp0((*signal_data).member, member as *const ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int)
            {
                if !(!(*signal_data).object_path.is_null()
                    && g_strcmp0(
                        (*signal_data).object_path,
                        path as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int)
                {
                    if !(*signal_data).shared_name_watcher.is_null() {
                        let mut watched_name: *const WatchedName =
                            ::core::ptr::null::<WatchedName>();
                        let mut current_owner: *const ::core::ffi::c_char =
                            ::core::ptr::null::<::core::ffi::c_char>();
                        if ({
                            let mut _g_boolean_var_141: ::core::ffi::c_int = 0;
                            if !(*signal_data).sender.is_null() {
                                _g_boolean_var_141 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_141 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_141
                        }) as ::core::ffi::c_long
                            != 0
                        {
                        } else {
                            g_assertion_message_expr(
                                G_LOG_DOMAIN.as_ptr(),
                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                4318 as ::core::ffi::c_int,
                                G_STRFUNC,
                                b"signal_data->sender != NULL\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        if ({
                            let mut _g_boolean_var_142: ::core::ffi::c_int = 0;
                            if g_dbus_is_unique_name((*signal_data).sender) == 0 {
                                _g_boolean_var_142 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_142 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_142
                        }) as ::core::ffi::c_long
                            != 0
                        {
                        } else {
                            g_assertion_message_expr(
                                G_LOG_DOMAIN.as_ptr(),
                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                4322 as ::core::ffi::c_int,
                                G_STRFUNC,
                                b"!g_dbus_is_unique_name (signal_data->sender)\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                        if ({
                            let mut _g_boolean_var_143: ::core::ffi::c_int = 0;
                            if g_strcmp0(
                                (*signal_data).sender,
                                b"org.freedesktop.DBus\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ) != 0 as ::core::ffi::c_int
                            {
                                _g_boolean_var_143 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_143 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_143
                        }) as ::core::ffi::c_long
                            != 0
                        {
                        } else {
                            g_assertion_message_expr(
                                G_LOG_DOMAIN.as_ptr(),
                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                4323 as ::core::ffi::c_int,
                                G_STRFUNC,
                                b"g_strcmp0 (signal_data->sender, DBUS_SERVICE_DBUS) != 0\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                        watched_name = (*(*signal_data).shared_name_watcher).watched_name;
                        if ({
                            let mut _g_boolean_var_144: ::core::ffi::c_int = 0;
                            if !watched_name.is_null() {
                                _g_boolean_var_144 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_144 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_144
                        }) as ::core::ffi::c_long
                            != 0
                        {
                        } else {
                            g_assertion_message_expr(
                                G_LOG_DOMAIN.as_ptr(),
                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                    as *const u8 as *const ::core::ffi::c_char,
                                4326 as ::core::ffi::c_int,
                                G_STRFUNC,
                                b"watched_name != NULL\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        current_owner = (*watched_name).owner;
                        if current_owner.is_null()
                            || g_strcmp0(current_owner, sender as *const ::core::ffi::c_char)
                                != 0 as ::core::ffi::c_int
                        {
                            current_block_89 = 13586036798005543211;
                        } else {
                            current_block_89 = 13460095289871124136;
                        }
                    } else {
                        if !(*signal_data).sender.is_null() {
                            if ({
                                let mut _g_boolean_var_145: ::core::ffi::c_int = 0;
                                if g_dbus_is_unique_name((*signal_data).sender) != 0
                                    || strcmp(
                                        (*signal_data).sender as *const ::core::ffi::c_char,
                                        b"org.freedesktop.DBus\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int
                                {
                                    _g_boolean_var_145 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_145 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_145
                            }) as ::core::ffi::c_long
                                != 0
                            {
                            } else {
                                g_assertion_message_expr(
                                    G_LOG_DOMAIN.as_ptr(),
                                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    4338 as ::core::ffi::c_int,
                                    G_STRFUNC,
                                    b"g_dbus_is_unique_name (signal_data->sender) || g_str_equal (signal_data->sender, DBUS_SERVICE_DBUS)\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                );
                            }
                            if ({
                                let mut _g_boolean_var_146: ::core::ffi::c_int = 0;
                                if g_strcmp0(
                                    (*signal_data).sender,
                                    sender as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    _g_boolean_var_146 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_146 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_146
                            }) as ::core::ffi::c_long
                                != 0
                            {
                            } else {
                                g_assertion_message_expr(
                                    G_LOG_DOMAIN.as_ptr(),
                                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                    4344 as ::core::ffi::c_int,
                                    G_STRFUNC,
                                    b"g_strcmp0 (signal_data->sender, sender) == 0\0"
                                        as *const u8 as *const ::core::ffi::c_char,
                                );
                            }
                        }
                        current_block_89 = 13460095289871124136;
                    }
                    match current_block_89 {
                        13586036798005543211 => {}
                        _ => {
                            if !(*signal_data).arg0.is_null() {
                                if (*signal_data).flags as ::core::ffi::c_uint
                                    & G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    != 0
                                {
                                    if arg0.is_null()
                                        || safe_c2rust_namespace_rule_matches(
                                            (*signal_data).arg0,
                                            arg0,
                                        ) == 0
                                    {
                                        current_block_89 = 13586036798005543211;
                                    } else {
                                        current_block_89 = 16415152177862271243;
                                    }
                                } else if (*signal_data).flags as ::core::ffi::c_uint
                                    & G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    != 0
                                {
                                    if (arg0.is_null()
                                        || safe_c2rust_path_rule_matches((*signal_data).arg0, arg0)
                                            == 0)
                                        && (arg0_path.is_null()
                                            || safe_c2rust_path_rule_matches(
                                                (*signal_data).arg0,
                                                arg0_path,
                                            ) == 0)
                                    {
                                        current_block_89 = 13586036798005543211;
                                    } else {
                                        current_block_89 = 16415152177862271243;
                                    }
                                } else if arg0.is_null()
                                    || !(strcmp(
                                        (*signal_data).arg0 as *const ::core::ffi::c_char,
                                        arg0 as *const ::core::ffi::c_char,
                                    ) == 0 as ::core::ffi::c_int)
                                {
                                    current_block_89 = 13586036798005543211;
                                } else {
                                    current_block_89 = 16415152177862271243;
                                }
                            } else {
                                current_block_89 = 16415152177862271243;
                            }
                            match current_block_89 {
                                13586036798005543211 => {}
                                _ => {
                                    if !(*signal_data).watched_name.is_null() {
                                        if ({
                                            let mut _g_boolean_var_147: ::core::ffi::c_int = 0;
                                            if g_strcmp0(
                                                sender as *const ::core::ffi::c_char,
                                                b"org.freedesktop.DBus\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                _g_boolean_var_147 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_147 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_147
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                        } else {
                                            g_assertion_message_expr(
                                                G_LOG_DOMAIN.as_ptr(),
                                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                                4369 as ::core::ffi::c_int,
                                                G_STRFUNC,
                                                b"g_strcmp0 (sender, DBUS_SERVICE_DBUS) == 0\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        }
                                        if ({
                                            let mut _g_boolean_var_148: ::core::ffi::c_int = 0;
                                            if g_strcmp0(
                                                interface as *const ::core::ffi::c_char,
                                                b"org.freedesktop.DBus\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                _g_boolean_var_148 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_148 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_148
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                        } else {
                                            g_assertion_message_expr(
                                                G_LOG_DOMAIN.as_ptr(),
                                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                                4370 as ::core::ffi::c_int,
                                                G_STRFUNC,
                                                b"g_strcmp0 (interface, DBUS_INTERFACE_DBUS) == 0\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                            );
                                        }
                                        if ({
                                            let mut _g_boolean_var_149: ::core::ffi::c_int = 0;
                                            if g_strcmp0(
                                                path as *const ::core::ffi::c_char,
                                                b"/org/freedesktop/DBus\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                _g_boolean_var_149 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_149 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_149
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                        } else {
                                            g_assertion_message_expr(
                                                G_LOG_DOMAIN.as_ptr(),
                                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                                4371 as ::core::ffi::c_int,
                                                G_STRFUNC,
                                                b"g_strcmp0 (path, DBUS_PATH_DBUS) == 0\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        }
                                        if ({
                                            let mut _g_boolean_var_150: ::core::ffi::c_int = 0;
                                            if g_strcmp0(
                                                member as *const ::core::ffi::c_char,
                                                b"NameOwnerChanged\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            ) == 0 as ::core::ffi::c_int
                                            {
                                                _g_boolean_var_150 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_150 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_150
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                        } else {
                                            g_assertion_message_expr(
                                                G_LOG_DOMAIN.as_ptr(),
                                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                                4372 as ::core::ffi::c_int,
                                                G_STRFUNC,
                                                b"g_strcmp0 (member, \"NameOwnerChanged\") == 0\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                            );
                                        }
                                        safe_c2rust_name_watcher_deliver_name_owner_changed_unlocked(
                                            signal_data,
                                            message,
                                        );
                                    }
                                    m = 0 as guint;
                                    while m < (*(*signal_data).subscribers).len {
                                        let mut subscriber: *mut SignalSubscriber =
                                            *(*(*signal_data).subscribers).pdata.offset(m as isize)
                                                as *mut SignalSubscriber;
                                        let mut idle_source: *mut GSource =
                                            ::core::ptr::null_mut::<GSource>();
                                        let mut signal_instance: *mut SignalInstance =
                                            ::core::ptr::null_mut::<SignalInstance>();
                                        signal_instance = ({
                                            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                                            let mut __s: gsize =
                                                ::core::mem::size_of::<SignalInstance>() as gsize;
                                            let mut __p: gpointer =
                                                ::core::ptr::null_mut::<::core::ffi::c_void>();
                                            if __s == 1 as gsize {
                                                __p = g_malloc0(__n);
                                            } else if 0 != 0
                                                && (__s == 0 as gsize
                                                    || __n <= G_MAXSIZE.wrapping_div(__s))
                                            {
                                                __p = g_malloc0(__n.wrapping_mul(__s));
                                            } else {
                                                __p = g_malloc0_n(__n, __s);
                                            }
                                            __p
                                        })
                                            as *mut SignalInstance;
                                        (*signal_instance).subscriber =
                                            safe_c2rust_signal_subscriber_ref(subscriber);
                                        (*signal_instance).message =
                                            g_object_ref(message as gpointer) as *mut GDBusMessage
                                                as *mut GDBusMessage;
                                        (*signal_instance).connection =
                                            g_object_ref(connection as gpointer)
                                                as *mut GDBusConnection
                                                as *mut GDBusConnection;
                                        (*signal_instance).sender = sender;
                                        (*signal_instance).path = path;
                                        (*signal_instance).interface = interface;
                                        (*signal_instance).member = member;
                                        idle_source = g_idle_source_new();
                                        g_source_set_priority(idle_source, G_PRIORITY_DEFAULT);
                                        g_source_set_callback(
                                            idle_source,
                                            Some(
                                                safe_c2rust_emit_signal_instance_in_idle_cb
                                                    as unsafe extern "C" fn(gpointer) -> gboolean,
                                            ),
                                            signal_instance as gpointer,
                                            ::core::mem::transmute::<
                                                Option<
                                                    unsafe extern "C" fn(*mut SignalInstance) -> (),
                                                >,
                                                GDestroyNotify,
                                            >(Some(
                                                safe_c2rust_signal_instance_free
                                                    as unsafe extern "C" fn(
                                                        *mut SignalInstance,
                                                    )
                                                        -> (),
                                            )),
                                        );
                                        g_source_set_static_name(
                                            idle_source,
                                            b"[gio] emit_signal_instance_in_idle_cb\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                        g_source_attach(idle_source, (*subscriber).context);
                                        g_source_unref(idle_source);
                                        m = m.wrapping_add(1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn safe_c2rust_distribute_signals(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
) {
    let mut signal_data_array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut sender: *const gchar = ::core::ptr::null::<gchar>();
    let mut interface: *const gchar = ::core::ptr::null::<gchar>();
    let mut member: *const gchar = ::core::ptr::null::<gchar>();
    let mut path: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_151: ::core::ffi::c_int = 0;
        if g_dbus_message_get_message_type(message) as ::core::ffi::c_uint
            == G_DBUS_MESSAGE_TYPE_SIGNAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _g_boolean_var_151 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_151 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_151
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            4412 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_dbus_message_get_message_type (message) == G_DBUS_MESSAGE_TYPE_SIGNAL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    sender = g_dbus_message_get_sender(message);
    interface = g_dbus_message_get_interface(message);
    member = g_dbus_message_get_member(message);
    path = g_dbus_message_get_path(message);
    if ({
        let mut _g_boolean_var_152: ::core::ffi::c_int = 0;
        if !interface.is_null() {
            _g_boolean_var_152 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_152 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_152
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            4422 as ::core::ffi::c_int,
            G_STRFUNC,
            b"interface != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_153: ::core::ffi::c_int = 0;
        if !member.is_null() {
            _g_boolean_var_153 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_153 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_153
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            4423 as ::core::ffi::c_int,
            G_STRFUNC,
            b"member != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_154: ::core::ffi::c_int = 0;
        if !path.is_null() {
            _g_boolean_var_154 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_154 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_154
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            4424 as ::core::ffi::c_int,
            G_STRFUNC,
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_155: ::core::ffi::c_int = 0;
        if _g_dbus_debug_signal() != 0 {
            _g_boolean_var_155 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_155 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_155
    }) as ::core::ffi::c_long
        != 0
    {
        _g_dbus_debug_print_lock();
        g_print(
            b"========================================================================\nGDBus-debug:Signal:\n <<<< RECEIVED SIGNAL %s.%s\n      on object %s\n      sent by name %s\n\0"
                as *const u8 as *const gchar,
            interface,
            member,
            path,
            if !sender.is_null() {
                sender as *const ::core::ffi::c_char
            } else {
                b"(none)\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        _g_dbus_debug_print_unlock();
    }
    if !sender.is_null() {
        signal_data_array = g_hash_table_lookup(
            (*connection).map_sender_unique_name_to_signal_data_array,
            sender as gconstpointer,
        ) as *mut GPtrArray;
        if !signal_data_array.is_null() {
            safe_c2rust_schedule_callbacks(connection, signal_data_array, message, sender);
        }
    }
    signal_data_array = g_hash_table_lookup(
        (*connection).map_sender_unique_name_to_signal_data_array,
        b"\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer,
    ) as *mut GPtrArray;
    if !signal_data_array.is_null() {
        safe_c2rust_schedule_callbacks(connection, signal_data_array, message, sender);
    }
}
unsafe extern "C" fn safe_c2rust_purge_all_signal_subscriptions(
    mut connection: *mut GDBusConnection,
) {
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut ids: *mut GArray = ::core::ptr::null_mut::<GArray>();
    let mut n: guint = 0;
    ids = g_array_new(FALSE, FALSE, ::core::mem::size_of::<guint>() as guint);
    g_hash_table_iter_init(&raw mut iter, (*connection).map_id_to_signal_data);
    while g_hash_table_iter_next(
        &raw mut iter,
        &raw mut key,
        ::core::ptr::null_mut::<gpointer>(),
    ) != 0
    {
        let mut subscription_id: guint = key as gulong as guint;
        g_array_append_vals(ids, &raw mut subscription_id as gconstpointer, 1 as guint);
    }
    n = 0 as guint;
    while n < (*ids).len {
        let mut subscription_id_0: guint =
            *((*ids).data as *mut ::core::ffi::c_void as *mut guint).offset(n as isize);
        safe_c2rust_unsubscribe_id_internal(connection, subscription_id_0);
        n = n.wrapping_add(1);
    }
    g_array_free(ids, TRUE);
}
unsafe extern "C" fn safe_c2rust__g_dbus_interface_vtable_copy(
    mut vtable: *const GDBusInterfaceVTable,
) -> *mut GDBusInterfaceVTable {
    return g_memdup2(
        vtable as gconstpointer,
        (3 as gsize).wrapping_mul(::core::mem::size_of::<gpointer>() as gsize),
    ) as *mut GDBusInterfaceVTable;
}
unsafe extern "C" fn safe_c2rust__g_dbus_interface_vtable_free(
    mut vtable: *mut GDBusInterfaceVTable,
) {
    g_free(vtable as gpointer);
}
unsafe extern "C" fn safe_c2rust__g_dbus_subtree_vtable_copy(
    mut vtable: *const GDBusSubtreeVTable,
) -> *mut GDBusSubtreeVTable {
    return g_memdup2(
        vtable as gconstpointer,
        (3 as gsize).wrapping_mul(::core::mem::size_of::<gpointer>() as gsize),
    ) as *mut GDBusSubtreeVTable;
}
unsafe extern "C" fn safe_c2rust__g_dbus_subtree_vtable_free(mut vtable: *mut GDBusSubtreeVTable) {
    g_free(vtable as gpointer);
}
unsafe extern "C" fn safe_c2rust_exported_object_free(mut eo: *mut ExportedObject) {
    g_free((*eo).object_path as gpointer);
    g_hash_table_unref((*eo).map_if_name_to_ei);
    g_free(eo as gpointer);
}
unsafe extern "C" fn safe_c2rust_exported_interface_ref(
    mut ei: *mut ExportedInterface,
) -> *mut ExportedInterface {
    if 0 as ::core::ffi::c_int != 0 {
        (*ei).refcount;
        (*ei).refcount;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*ei).refcount, 1 as ::core::ffi::c_int);
    return ei;
}
unsafe extern "C" fn safe_c2rust_exported_interface_unref(mut ei: *mut ExportedInterface) {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*ei).refcount;
            (*ei).refcount;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(&raw mut (*ei).refcount, 1 as ::core::ffi::c_int)
            == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) == 0
    {
        return;
    }
    g_dbus_interface_info_cache_release((*ei).interface_info);
    g_dbus_interface_info_unref((*ei).interface_info);
    safe_c2rust_call_destroy_notify((*ei).context, (*ei).user_data_free_func, (*ei).user_data);
    g_main_context_unref((*ei).context);
    g_free((*ei).interface_name as gpointer);
    safe_c2rust__g_dbus_interface_vtable_free((*ei).vtable);
    g_free(ei as gpointer);
}
unsafe extern "C" fn safe_c2rust_exported_subtree_ref(
    mut es: *mut ExportedSubtree,
) -> *mut ExportedSubtree {
    if 0 as ::core::ffi::c_int != 0 {
        (*es).refcount;
        (*es).refcount;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*es).refcount, 1 as ::core::ffi::c_int);
    return es;
}
unsafe extern "C" fn safe_c2rust_exported_subtree_unref(mut es: *mut ExportedSubtree) {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*es).refcount;
            (*es).refcount;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(&raw mut (*es).refcount, 1 as ::core::ffi::c_int)
            == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) == 0
    {
        return;
    }
    safe_c2rust_call_destroy_notify((*es).context, (*es).user_data_free_func, (*es).user_data);
    g_main_context_unref((*es).context);
    safe_c2rust__g_dbus_subtree_vtable_free((*es).vtable);
    g_free((*es).object_path as gpointer);
    g_free(es as gpointer);
}
unsafe extern "C" fn safe_c2rust_has_object_been_unregistered(
    mut connection: *mut GDBusConnection,
    mut registration_id: guint,
    mut out_ei: *mut *mut ExportedInterface,
    mut subtree_registration_id: guint,
    mut out_es: *mut *mut ExportedSubtree,
) -> gboolean {
    let mut ret: gboolean = 0;
    let mut ei: *mut ExportedInterface = ::core::ptr::null_mut::<ExportedInterface>();
    let mut es: gpointer = NULL_0;
    if ({
        let mut _g_boolean_var_156: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_156 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_156 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_156
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
    ret = FALSE as gboolean;
    g_mutex_lock(&raw mut (*connection).lock);
    if registration_id != 0 as guint {
        ei = g_hash_table_lookup(
            (*connection).map_id_to_ei,
            registration_id as gulong as gpointer as gconstpointer,
        ) as *mut ExportedInterface;
        if ei.is_null() {
            ret = TRUE as gboolean;
        } else if !out_ei.is_null() {
            *out_ei = safe_c2rust_exported_interface_ref(ei);
        }
    }
    if subtree_registration_id != 0 as guint {
        es = g_hash_table_lookup(
            (*connection).map_id_to_es,
            subtree_registration_id as gulong as gpointer as gconstpointer,
        );
        if es.is_null() {
            ret = TRUE as gboolean;
        } else if !out_es.is_null() {
            *out_es = safe_c2rust_exported_subtree_ref(es as *mut ExportedSubtree);
        }
    }
    g_mutex_unlock(&raw mut (*connection).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_property_data_free(mut data: *mut PropertyData) {
    g_object_unref((*data).connection as gpointer);
    let mut _pp: *mut *mut GDBusMessage = &raw mut (*data).message;
    let mut _ptr: *mut GDBusMessage = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusMessage>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_invoke_get_property_in_idle_cb(mut _data: gpointer) -> gboolean {
    let mut data: *mut PropertyData = _data as *mut PropertyData;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut ei: *mut ExportedInterface = ::core::ptr::null_mut::<ExportedInterface>();
    let mut es: *mut ExportedSubtree = ::core::ptr::null_mut::<ExportedSubtree>();
    if safe_c2rust_has_object_been_unregistered(
        (*data).connection,
        (*data).registration_id,
        &raw mut ei,
        (*data).subtree_registration_id,
        &raw mut es,
    ) != 0
    {
        reply = g_dbus_message_new_method_error(
            (*data).message,
            b"org.freedesktop.DBus.Error.UnknownMethod\0" as *const u8 as *const gchar,
            glib_gettext(
                b"No such interface \xE2\x80\x9Corg.freedesktop.DBus.Properties\xE2\x80\x9D on object at path %s\0"
                    as *const u8 as *const gchar,
            ),
            g_dbus_message_get_path((*data).message),
        );
        safe_c2rust_g_dbus_connection_send_message(
            (*data).connection,
            reply,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            ::core::ptr::null_mut::<guint32>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(reply as gpointer);
    } else {
        error = ::core::ptr::null_mut::<GError>();
        value = (*(*data).vtable)
            .get_property
            .expect("non-null function pointer")(
            (*data).connection,
            g_dbus_message_get_sender((*data).message),
            g_dbus_message_get_path((*data).message),
            (*(*data).interface_info).name,
            (*data).property_name,
            &raw mut error,
            (*data).user_data,
        );
        if !value.is_null() {
            if !error.is_null() {
                g_assertion_message_error(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    4737 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error\0" as *const u8 as *const ::core::ffi::c_char,
                    error,
                    0 as GQuark,
                    0 as ::core::ffi::c_int,
                );
            }
            g_variant_take_ref(value);
            reply = g_dbus_message_new_method_reply((*data).message);
            g_dbus_message_set_body(
                reply,
                g_variant_new(b"(v)\0" as *const u8 as *const gchar, value),
            );
            safe_c2rust_g_dbus_connection_send_message(
                (*data).connection,
                reply,
                G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                ::core::ptr::null_mut::<guint32>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_variant_unref(value);
            g_object_unref(reply as gpointer);
        } else {
            let mut dbus_error_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
            if ({
                let mut _g_boolean_var_157: ::core::ffi::c_int = 0;
                if !error.is_null() {
                    _g_boolean_var_157 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_157 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_157
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    4749 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            dbus_error_name = g_dbus_error_encode_gerror(error);
            reply = g_dbus_message_new_method_error_literal(
                (*data).message,
                dbus_error_name,
                (*error).message,
            );
            safe_c2rust_g_dbus_connection_send_message(
                (*data).connection,
                reply,
                G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                ::core::ptr::null_mut::<guint32>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_free(dbus_error_name as gpointer);
            g_error_free(error);
            g_object_unref(reply as gpointer);
        }
    }
    let mut _pp: *mut *mut ExportedInterface = &raw mut ei;
    let mut _ptr: *mut ExportedInterface = *_pp;
    *_pp = ::core::ptr::null_mut::<ExportedInterface>();
    if !_ptr.is_null() {
        safe_c2rust_exported_interface_unref(_ptr as *mut ExportedInterface);
    }
    let mut _pp_0: *mut *mut ExportedSubtree = &raw mut es;
    let mut _ptr_0: *mut ExportedSubtree = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<ExportedSubtree>();
    if !_ptr_0.is_null() {
        safe_c2rust_exported_subtree_unref(_ptr_0 as *mut ExportedSubtree);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_invoke_set_property_in_idle_cb(mut _data: gpointer) -> gboolean {
    let mut data: *mut PropertyData = _data as *mut PropertyData;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    error = ::core::ptr::null_mut::<GError>();
    value = ::core::ptr::null_mut::<GVariant>();
    g_variant_get(
        g_dbus_message_get_body((*data).message),
        b"(ssv)\0" as *const u8 as *const gchar,
        NULL_0,
        NULL_0,
        &raw mut value,
    );
    if (*(*data).vtable)
        .set_property
        .expect("non-null function pointer")(
        (*data).connection,
        g_dbus_message_get_sender((*data).message),
        g_dbus_message_get_path((*data).message),
        (*(*data).interface_info).name,
        (*data).property_name,
        value,
        &raw mut error,
        (*data).user_data,
    ) == 0
    {
        let mut dbus_error_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if ({
            let mut _g_boolean_var_158: ::core::ffi::c_int = 0;
            if !error.is_null() {
                _g_boolean_var_158 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_158 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_158
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                4795 as ::core::ffi::c_int,
                G_STRFUNC,
                b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        dbus_error_name = g_dbus_error_encode_gerror(error);
        reply = g_dbus_message_new_method_error_literal(
            (*data).message,
            dbus_error_name,
            (*error).message,
        );
        g_free(dbus_error_name as gpointer);
        g_error_free(error);
    } else {
        reply = g_dbus_message_new_method_reply((*data).message);
    }
    if ({
        let mut _g_boolean_var_159: ::core::ffi::c_int = 0;
        if !reply.is_null() {
            _g_boolean_var_159 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_159 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_159
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            4808 as ::core::ffi::c_int,
            G_STRFUNC,
            b"reply != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_g_dbus_connection_send_message(
        (*data).connection,
        reply,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        ::core::ptr::null_mut::<guint32>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_object_unref(reply as gpointer);
    g_variant_unref(value);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_validate_and_maybe_schedule_property_getset(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut registration_id: guint,
    mut subtree_registration_id: guint,
    mut is_get: gboolean,
    mut interface_info: *mut GDBusInterfaceInfo,
    mut vtable: *const GDBusInterfaceVTable,
    mut main_context: *mut GMainContext,
    mut user_data: gpointer,
) -> gboolean {
    let mut current_block: u64;
    let mut handled: gboolean = 0;
    let mut interface_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut property_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut property_info: *const GDBusPropertyInfo = ::core::ptr::null::<GDBusPropertyInfo>();
    let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut property_data: *mut PropertyData = ::core::ptr::null_mut::<PropertyData>();
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    handled = FALSE as gboolean;
    if is_get != 0 {
        g_variant_get(
            g_dbus_message_get_body(message),
            b"(&s&s)\0" as *const u8 as *const gchar,
            &raw mut interface_name,
            &raw mut property_name,
        );
    } else {
        g_variant_get(
            g_dbus_message_get_body(message),
            b"(&s&sv)\0" as *const u8 as *const gchar,
            &raw mut interface_name,
            &raw mut property_name,
            NULL_0,
        );
    }
    if !vtable.is_null() {
        property_info = ::core::ptr::null::<GDBusPropertyInfo>();
        property_info =
            g_dbus_interface_info_lookup_property(interface_info, property_name as *const gchar);
        if property_info.is_null() {
            reply = g_dbus_message_new_method_error(
                message,
                b"org.freedesktop.DBus.Error.InvalidArgs\0" as *const u8 as *const gchar,
                glib_gettext(
                    b"No such property \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8 as *const gchar,
                ),
                property_name,
            );
            safe_c2rust_g_dbus_connection_send_message_unlocked(
                connection,
                reply,
                G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                ::core::ptr::null_mut::<guint32>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_object_unref(reply as gpointer);
            handled = TRUE as gboolean;
        } else if is_get != 0
            && (*property_info).flags as ::core::ffi::c_uint
                & G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
            reply = g_dbus_message_new_method_error(
                message,
                b"org.freedesktop.DBus.Error.InvalidArgs\0" as *const u8 as *const gchar,
                glib_gettext(
                    b"Property \xE2\x80\x9C%s\xE2\x80\x9D is not readable\0" as *const u8
                        as *const gchar,
                ),
                property_name,
            );
            safe_c2rust_g_dbus_connection_send_message_unlocked(
                connection,
                reply,
                G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                ::core::ptr::null_mut::<guint32>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_object_unref(reply as gpointer);
            handled = TRUE as gboolean;
        } else if is_get == 0
            && (*property_info).flags as ::core::ffi::c_uint
                & G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
            reply = g_dbus_message_new_method_error(
                message,
                b"org.freedesktop.DBus.Error.InvalidArgs\0" as *const u8 as *const gchar,
                glib_gettext(
                    b"Property \xE2\x80\x9C%s\xE2\x80\x9D is not writable\0" as *const u8
                        as *const gchar,
                ),
                property_name,
            );
            safe_c2rust_g_dbus_connection_send_message_unlocked(
                connection,
                reply,
                G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                ::core::ptr::null_mut::<guint32>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_object_unref(reply as gpointer);
            handled = TRUE as gboolean;
        } else {
            if is_get == 0 {
                let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                g_variant_get_child(
                    g_dbus_message_get_body(message),
                    2 as gsize,
                    b"v\0" as *const u8 as *const gchar,
                    &raw mut value,
                );
                if g_strcmp0(
                    g_variant_get_type_string(value) as *const ::core::ffi::c_char,
                    (*property_info).signature,
                ) != 0 as ::core::ffi::c_int
                {
                    reply = g_dbus_message_new_method_error(
                        message,
                        b"org.freedesktop.DBus.Error.InvalidArgs\0" as *const u8
                            as *const gchar,
                        glib_gettext(
                            b"Error setting property \xE2\x80\x9C%s\xE2\x80\x9D: Expected type \xE2\x80\x9C%s\xE2\x80\x9D but got \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                as *const u8 as *const gchar,
                        ),
                        property_name,
                        (*property_info).signature,
                        g_variant_get_type_string(value),
                    );
                    safe_c2rust_g_dbus_connection_send_message_unlocked(
                        connection,
                        reply,
                        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                        ::core::ptr::null_mut::<guint32>(),
                        ::core::ptr::null_mut::<*mut GError>(),
                    );
                    g_variant_unref(value);
                    g_object_unref(reply as gpointer);
                    handled = TRUE as gboolean;
                    current_block = 4480321469763654056;
                } else {
                    g_variant_unref(value);
                    current_block = 11057878835866523405;
                }
            } else {
                current_block = 11057878835866523405;
            }
            match current_block {
                4480321469763654056 => {}
                _ => {
                    if is_get != 0 {
                        if (*vtable).get_property.is_none() {
                            safe_c2rust_schedule_method_call(
                                connection,
                                message,
                                registration_id,
                                subtree_registration_id,
                                interface_info,
                                ::core::ptr::null::<GDBusMethodInfo>(),
                                property_info,
                                g_dbus_message_get_body(message),
                                vtable,
                                main_context,
                                user_data,
                            );
                            handled = TRUE as gboolean;
                            current_block = 4480321469763654056;
                        } else {
                            current_block = 18153031941552419006;
                        }
                    } else if (*vtable).set_property.is_none() {
                        safe_c2rust_schedule_method_call(
                            connection,
                            message,
                            registration_id,
                            subtree_registration_id,
                            interface_info,
                            ::core::ptr::null::<GDBusMethodInfo>(),
                            property_info,
                            g_dbus_message_get_body(message),
                            vtable,
                            main_context,
                            user_data,
                        );
                        handled = TRUE as gboolean;
                        current_block = 4480321469763654056;
                    } else {
                        current_block = 18153031941552419006;
                    }
                    match current_block {
                        4480321469763654056 => {}
                        _ => {
                            property_data = ({
                                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                                let mut __s: gsize =
                                    ::core::mem::size_of::<PropertyData>() as gsize;
                                let mut __p: gpointer =
                                    ::core::ptr::null_mut::<::core::ffi::c_void>();
                                if __s == 1 as gsize {
                                    __p = g_malloc0(__n);
                                } else if 0 != 0
                                    && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s))
                                {
                                    __p = g_malloc0(__n.wrapping_mul(__s));
                                } else {
                                    __p = g_malloc0_n(__n, __s);
                                }
                                __p
                            }) as *mut PropertyData;
                            (*property_data).connection = g_object_ref(connection as gpointer)
                                as *mut GDBusConnection
                                as *mut GDBusConnection;
                            (*property_data).message = g_object_ref(message as gpointer)
                                as *mut GDBusMessage
                                as *mut GDBusMessage;
                            (*property_data).user_data = user_data;
                            (*property_data).property_name = property_name as *const gchar;
                            (*property_data).vtable = vtable;
                            (*property_data).interface_info = interface_info;
                            (*property_data).property_info = property_info;
                            (*property_data).registration_id = registration_id;
                            (*property_data).subtree_registration_id = subtree_registration_id;
                            idle_source = g_idle_source_new();
                            g_source_set_priority(idle_source, G_PRIORITY_DEFAULT);
                            g_source_set_callback(
                                idle_source,
                                if is_get != 0 {
                                    Some(
                                        safe_c2rust_invoke_get_property_in_idle_cb
                                            as unsafe extern "C" fn(gpointer) -> gboolean,
                                    )
                                } else {
                                    Some(
                                        safe_c2rust_invoke_set_property_in_idle_cb
                                            as unsafe extern "C" fn(gpointer) -> gboolean,
                                    )
                                },
                                property_data as gpointer,
                                ::core::mem::transmute::<
                                    Option<unsafe extern "C" fn(*mut PropertyData) -> ()>,
                                    GDestroyNotify,
                                >(Some(
                                    safe_c2rust_property_data_free
                                        as unsafe extern "C" fn(*mut PropertyData) -> (),
                                )),
                            );
                            if is_get != 0 {
                                g_source_set_static_name(
                                    idle_source,
                                    b"[gio] invoke_get_property_in_idle_cb\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            } else {
                                g_source_set_static_name(
                                    idle_source,
                                    b"[gio] invoke_set_property_in_idle_cb\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                            g_source_attach(idle_source, main_context);
                            g_source_unref(idle_source);
                            handled = TRUE as gboolean;
                        }
                    }
                }
            }
        }
    }
    return handled;
}
unsafe extern "C" fn safe_c2rust_handle_getset_property(
    mut connection: *mut GDBusConnection,
    mut eo: *mut ExportedObject,
    mut message: *mut GDBusMessage,
    mut is_get: gboolean,
) -> gboolean {
    let mut ei: *mut ExportedInterface = ::core::ptr::null_mut::<ExportedInterface>();
    let mut handled: gboolean = 0;
    let mut interface_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut property_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    handled = FALSE as gboolean;
    if is_get != 0 {
        g_variant_get(
            g_dbus_message_get_body(message),
            b"(&s&s)\0" as *const u8 as *const gchar,
            &raw mut interface_name,
            &raw mut property_name,
        );
    } else {
        g_variant_get(
            g_dbus_message_get_body(message),
            b"(&s&sv)\0" as *const u8 as *const gchar,
            &raw mut interface_name,
            &raw mut property_name,
            NULL_0,
        );
    }
    ei = g_hash_table_lookup((*eo).map_if_name_to_ei, interface_name as gconstpointer)
        as *mut ExportedInterface;
    if ei.is_null() {
        let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
        reply = g_dbus_message_new_method_error(
            message,
            b"org.freedesktop.DBus.Error.InvalidArgs\0" as *const u8 as *const gchar,
            glib_gettext(
                b"No such interface \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8 as *const gchar,
            ),
            interface_name,
        );
        safe_c2rust_g_dbus_connection_send_message_unlocked(
            (*eo).connection,
            reply,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            ::core::ptr::null_mut::<guint32>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(reply as gpointer);
        handled = TRUE as gboolean;
    } else {
        handled = safe_c2rust_validate_and_maybe_schedule_property_getset(
            (*eo).connection,
            message,
            (*ei).id,
            0 as guint,
            is_get,
            (*ei).interface_info,
            (*ei).vtable,
            (*ei).context,
            (*ei).user_data,
        );
    }
    return handled;
}
unsafe extern "C" fn safe_c2rust_property_get_all_data_free(mut data: *mut PropertyGetAllData) {
    g_object_unref((*data).connection as gpointer);
    let mut _pp: *mut *mut GDBusMessage = &raw mut (*data).message;
    let mut _ptr: *mut GDBusMessage = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusMessage>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_invoke_get_all_properties_in_idle_cb(
    mut _data: gpointer,
) -> gboolean {
    let mut data: *mut PropertyGetAllData = _data as *mut PropertyGetAllData;
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut n: guint = 0;
    let mut ei: *mut ExportedInterface = ::core::ptr::null_mut::<ExportedInterface>();
    let mut es: *mut ExportedSubtree = ::core::ptr::null_mut::<ExportedSubtree>();
    if safe_c2rust_has_object_been_unregistered(
        (*data).connection,
        (*data).registration_id,
        &raw mut ei,
        (*data).subtree_registration_id,
        &raw mut es,
    ) != 0
    {
        reply = g_dbus_message_new_method_error(
            (*data).message,
            b"org.freedesktop.DBus.Error.UnknownMethod\0" as *const u8 as *const gchar,
            glib_gettext(
                b"No such interface \xE2\x80\x9Corg.freedesktop.DBus.Properties\xE2\x80\x9D on object at path %s\0"
                    as *const u8 as *const gchar,
            ),
            g_dbus_message_get_path((*data).message),
        );
        safe_c2rust_g_dbus_connection_send_message(
            (*data).connection,
            reply,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            ::core::ptr::null_mut::<guint32>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(reply as gpointer);
    } else {
        g_variant_builder_init(
            &raw mut builder,
            g_variant_type_checked_(b"(a{sv})\0" as *const u8 as *const gchar),
        );
        g_variant_builder_open(
            &raw mut builder,
            g_variant_type_checked_(b"a{sv}\0" as *const u8 as *const gchar),
        );
        n = 0 as guint;
        while !(*(*data).interface_info).properties.is_null()
            && !(*(*(*data).interface_info).properties.offset(n as isize)).is_null()
        {
            let mut property_info: *const GDBusPropertyInfo =
                *(*(*data).interface_info).properties.offset(n as isize);
            let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            if !((*property_info).flags as ::core::ffi::c_uint
                & G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0)
            {
                value = (*(*data).vtable)
                    .get_property
                    .expect("non-null function pointer")(
                    (*data).connection,
                    g_dbus_message_get_sender((*data).message),
                    g_dbus_message_get_path((*data).message),
                    (*(*data).interface_info).name,
                    (*property_info).name,
                    ::core::ptr::null_mut::<*mut GError>(),
                    (*data).user_data,
                );
                if !value.is_null() {
                    g_variant_take_ref(value);
                    g_variant_builder_add(
                        &raw mut builder,
                        b"{sv}\0" as *const u8 as *const gchar,
                        (*property_info).name,
                        value,
                    );
                    g_variant_unref(value);
                }
            }
            n = n.wrapping_add(1);
        }
        g_variant_builder_close(&raw mut builder);
        reply = g_dbus_message_new_method_reply((*data).message);
        g_dbus_message_set_body(reply, g_variant_builder_end(&raw mut builder));
        safe_c2rust_g_dbus_connection_send_message(
            (*data).connection,
            reply,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            ::core::ptr::null_mut::<guint32>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(reply as gpointer);
    }
    let mut _pp: *mut *mut ExportedInterface = &raw mut ei;
    let mut _ptr: *mut ExportedInterface = *_pp;
    *_pp = ::core::ptr::null_mut::<ExportedInterface>();
    if !_ptr.is_null() {
        safe_c2rust_exported_interface_unref(_ptr as *mut ExportedInterface);
    }
    let mut _pp_0: *mut *mut ExportedSubtree = &raw mut es;
    let mut _ptr_0: *mut ExportedSubtree = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<ExportedSubtree>();
    if !_ptr_0.is_null() {
        safe_c2rust_exported_subtree_unref(_ptr_0 as *mut ExportedSubtree);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_interface_has_readable_properties(
    mut interface_info: *mut GDBusInterfaceInfo,
) -> gboolean {
    let mut i: gint = 0;
    if (*interface_info).properties.is_null() {
        return FALSE;
    }
    i = 0 as ::core::ffi::c_int as gint;
    while !(*(*interface_info).properties.offset(i as isize)).is_null() {
        if (**(*interface_info).properties.offset(i as isize)).flags as ::core::ffi::c_uint
            & G_DBUS_PROPERTY_INFO_FLAGS_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_validate_and_maybe_schedule_property_get_all(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut registration_id: guint,
    mut subtree_registration_id: guint,
    mut interface_info: *mut GDBusInterfaceInfo,
    mut vtable: *const GDBusInterfaceVTable,
    mut main_context: *mut GMainContext,
    mut user_data: gpointer,
) -> gboolean {
    let mut handled: gboolean = 0;
    let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut property_get_all_data: *mut PropertyGetAllData =
        ::core::ptr::null_mut::<PropertyGetAllData>();
    handled = FALSE as gboolean;
    if !vtable.is_null() {
        if (*vtable).get_property.is_none()
            && safe_c2rust_interface_has_readable_properties(interface_info) != 0
        {
            safe_c2rust_schedule_method_call(
                connection,
                message,
                registration_id,
                subtree_registration_id,
                interface_info,
                ::core::ptr::null::<GDBusMethodInfo>(),
                ::core::ptr::null::<GDBusPropertyInfo>(),
                g_dbus_message_get_body(message),
                vtable,
                main_context,
                user_data,
            );
            handled = TRUE as gboolean;
        } else {
            property_get_all_data = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<PropertyGetAllData>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut PropertyGetAllData;
            (*property_get_all_data).connection = g_object_ref(connection as gpointer)
                as *mut GDBusConnection
                as *mut GDBusConnection;
            (*property_get_all_data).message =
                g_object_ref(message as gpointer) as *mut GDBusMessage as *mut GDBusMessage;
            (*property_get_all_data).user_data = user_data;
            (*property_get_all_data).vtable = vtable;
            (*property_get_all_data).interface_info = interface_info;
            (*property_get_all_data).registration_id = registration_id;
            (*property_get_all_data).subtree_registration_id = subtree_registration_id;
            idle_source = g_idle_source_new();
            g_source_set_priority(idle_source, G_PRIORITY_DEFAULT);
            g_source_set_callback(
                idle_source,
                Some(
                    safe_c2rust_invoke_get_all_properties_in_idle_cb
                        as unsafe extern "C" fn(gpointer) -> gboolean,
                ),
                property_get_all_data as gpointer,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut PropertyGetAllData) -> ()>,
                    GDestroyNotify,
                >(Some(
                    safe_c2rust_property_get_all_data_free
                        as unsafe extern "C" fn(*mut PropertyGetAllData) -> (),
                )),
            );
            g_source_set_static_name(
                idle_source,
                b"[gio] invoke_get_all_properties_in_idle_cb\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            g_source_attach(idle_source, main_context);
            g_source_unref(idle_source);
            handled = TRUE as gboolean;
        }
    }
    return handled;
}
unsafe extern "C" fn safe_c2rust_handle_get_all_properties(
    mut connection: *mut GDBusConnection,
    mut eo: *mut ExportedObject,
    mut message: *mut GDBusMessage,
) -> gboolean {
    let mut ei: *mut ExportedInterface = ::core::ptr::null_mut::<ExportedInterface>();
    let mut handled: gboolean = 0;
    let mut interface_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    handled = FALSE as gboolean;
    g_variant_get(
        g_dbus_message_get_body(message),
        b"(&s)\0" as *const u8 as *const gchar,
        &raw mut interface_name,
    );
    ei = g_hash_table_lookup((*eo).map_if_name_to_ei, interface_name as gconstpointer)
        as *mut ExportedInterface;
    if ei.is_null() {
        let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
        reply = g_dbus_message_new_method_error(
            message,
            b"org.freedesktop.DBus.Error.InvalidArgs\0" as *const u8 as *const gchar,
            glib_gettext(
                b"No such interface \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8 as *const gchar,
            ),
            interface_name,
        );
        safe_c2rust_g_dbus_connection_send_message_unlocked(
            (*eo).connection,
            reply,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            ::core::ptr::null_mut::<guint32>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(reply as gpointer);
        handled = TRUE as gboolean;
    } else {
        handled = safe_c2rust_validate_and_maybe_schedule_property_get_all(
            (*eo).connection,
            message,
            (*ei).id,
            0 as guint,
            (*ei).interface_info,
            (*ei).vtable,
            (*ei).context,
            (*ei).user_data,
        );
    }
    return handled;
}
static mut safe_c2rust_introspect_header: [gchar; 195] = unsafe {
    ::core::mem::transmute::<
        [u8; 195],
        [gchar; 195],
    >(
        *b"<!DOCTYPE node PUBLIC \"-//freedesktop//DTD D-BUS Object Introspection 1.0//EN\"\n                      \"http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd\">\n<!-- GDBus 2.80.0 -->\n<node>\n\0",
    )
};
static mut safe_c2rust_introspect_tail: [gchar; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [gchar; 9]>(*b"</node>\n\0") };
static mut safe_c2rust_introspect_properties_interface: [gchar; 841] = unsafe {
    ::core::mem::transmute::<
        [u8; 841],
        [gchar; 841],
    >(
        *b"  <interface name=\"org.freedesktop.DBus.Properties\">\n    <method name=\"Get\">\n      <arg type=\"s\" name=\"interface_name\" direction=\"in\"/>\n      <arg type=\"s\" name=\"property_name\" direction=\"in\"/>\n      <arg type=\"v\" name=\"value\" direction=\"out\"/>\n    </method>\n    <method name=\"GetAll\">\n      <arg type=\"s\" name=\"interface_name\" direction=\"in\"/>\n      <arg type=\"a{sv}\" name=\"properties\" direction=\"out\"/>\n    </method>\n    <method name=\"Set\">\n      <arg type=\"s\" name=\"interface_name\" direction=\"in\"/>\n      <arg type=\"s\" name=\"property_name\" direction=\"in\"/>\n      <arg type=\"v\" name=\"value\" direction=\"in\"/>\n    </method>\n    <signal name=\"PropertiesChanged\">\n      <arg type=\"s\" name=\"interface_name\"/>\n      <arg type=\"a{sv}\" name=\"changed_properties\"/>\n      <arg type=\"as\" name=\"invalidated_properties\"/>\n    </signal>\n  </interface>\n\0",
    )
};
static mut safe_c2rust_introspect_introspectable_interface: [gchar; 365] = unsafe {
    ::core::mem::transmute::<
        [u8; 365],
        [gchar; 365],
    >(
        *b"  <interface name=\"org.freedesktop.DBus.Introspectable\">\n    <method name=\"Introspect\">\n      <arg type=\"s\" name=\"xml_data\" direction=\"out\"/>\n    </method>\n  </interface>\n  <interface name=\"org.freedesktop.DBus.Peer\">\n    <method name=\"Ping\"/>\n    <method name=\"GetMachineId\">\n      <arg type=\"s\" name=\"machine_uuid\" direction=\"out\"/>\n    </method>\n  </interface>\n\0",
    )
};
unsafe extern "C" fn safe_c2rust_introspect_append_header(mut s: *mut GString) {
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                &raw const safe_c2rust_introspect_header as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                s,
                __val,
                if ({
                    let mut _g_boolean_var_160: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_160 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_160 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_160
                }) as ::core::ffi::c_long
                    != 0
                {
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            s,
            &raw const safe_c2rust_introspect_header as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
}
unsafe extern "C" fn safe_c2rust_maybe_add_path(
    mut path: *const gchar,
    mut path_len: gsize,
    mut object_path: *const gchar,
    mut set: *mut GHashTable,
) {
    if (if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = object_path as *const ::core::ffi::c_char;
            let __prefix: *const ::core::ffi::c_char = path as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_161: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_161 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_161 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_161
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __prefix_len: size_t =
                    strlen(__prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __prefix_len {
                    __result = (memcmp(
                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __prefix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_prefix(object_path, path)
    }) != 0
        && strlen(object_path as *const ::core::ffi::c_char) > path_len as size_t
        && *object_path.offset(path_len.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        let mut begin: *const gchar = ::core::ptr::null::<gchar>();
        let mut end: *const gchar = ::core::ptr::null::<gchar>();
        let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
        begin = object_path.offset(path_len as isize);
        end = strchr(begin as *const ::core::ffi::c_char, '/' as i32);
        if !end.is_null() {
            s = g_strndup(
                begin,
                end.offset_from(begin) as ::core::ffi::c_long as gsize,
            );
        } else {
            s = safe_c2rust_g_strdup_inline(begin as *const ::core::ffi::c_char) as *mut gchar;
        }
        if g_hash_table_contains(set, s as gconstpointer) == 0 {
            g_hash_table_add(set, s as gpointer);
        } else {
            g_free(s as gpointer);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_list_registered_unlocked(
    mut connection: *mut GDBusConnection,
    mut path: *const gchar,
) -> *mut *mut gchar {
    let mut p: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut ret: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut hash_iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut object_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut path_len: gsize = 0;
    let mut set: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    if ({
        let mut _g_boolean_var_162: ::core::ffi::c_int = 0;
        if g_mutex_trylock(&raw mut (*connection).lock) != 0 {
            _g_boolean_var_162 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_162 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_162
    }) as ::core::ffi::c_long
        != 0
    {
        g_assertion_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            5335 as ::core::ffi::c_int,
            G_STRFUNC,
            b"CONNECTION_ENSURE_LOCK: GDBusConnection object lock is not locked\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    path_len = strlen(path as *const ::core::ffi::c_char) as gsize;
    if path_len > 1 as gsize {
        path_len = path_len.wrapping_add(1);
    }
    set = g_hash_table_new(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    g_hash_table_iter_init(&raw mut hash_iter, (*connection).map_object_path_to_eo);
    while g_hash_table_iter_next(
        &raw mut hash_iter,
        &raw mut object_path as gpointer as *mut gpointer,
        ::core::ptr::null_mut::<gpointer>(),
    ) != 0
    {
        safe_c2rust_maybe_add_path(path, path_len, object_path, set);
    }
    g_hash_table_iter_init(&raw mut hash_iter, (*connection).map_object_path_to_es);
    while g_hash_table_iter_next(
        &raw mut hash_iter,
        &raw mut object_path as gpointer as *mut gpointer,
        ::core::ptr::null_mut::<gpointer>(),
    ) != 0
    {
        safe_c2rust_maybe_add_path(path, path_len, object_path, set);
    }
    p = g_hash_table_steal_all_keys(set);
    g_hash_table_unref(set);
    g_ptr_array_add(p, NULL_0);
    ret = g_ptr_array_free(p, FALSE) as *mut *mut gchar;
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_list_registered(
    mut connection: *mut GDBusConnection,
    mut path: *const gchar,
) -> *mut *mut gchar {
    let mut ret: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    g_mutex_lock(&raw mut (*connection).lock);
    ret = safe_c2rust_g_dbus_connection_list_registered_unlocked(connection, path);
    g_mutex_unlock(&raw mut (*connection).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_handle_introspect(
    mut connection: *mut GDBusConnection,
    mut eo: *mut ExportedObject,
    mut message: *mut GDBusMessage,
) -> gboolean {
    let mut n: guint = 0;
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut hash_iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut ei: *mut ExportedInterface = ::core::ptr::null_mut::<ExportedInterface>();
    let mut registered: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    s = g_string_sized_new(
        (::core::mem::size_of::<[gchar; 195]>() as gsize)
            .wrapping_add(::core::mem::size_of::<[gchar; 841]>() as gsize)
            .wrapping_add(::core::mem::size_of::<[gchar; 365]>() as gsize)
            .wrapping_add(::core::mem::size_of::<[gchar; 9]>() as gsize),
    );
    safe_c2rust_introspect_append_header(s);
    if g_hash_table_lookup(
        (*eo).map_if_name_to_ei,
        b"org.freedesktop.DBus.Properties\0" as *const u8 as *const ::core::ffi::c_char
            as gconstpointer,
    )
    .is_null()
    {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    &raw const safe_c2rust_introspect_properties_interface
                        as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    s,
                    __val,
                    if ({
                        let mut _g_boolean_var_163: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_163 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_163 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_163
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as ::core::ffi::c_int) as gssize
                    },
                );
            });
        } else {
            safe_c2rust_g_string_append_len_inline(
                s,
                &raw const safe_c2rust_introspect_properties_interface
                    as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if g_hash_table_lookup(
        (*eo).map_if_name_to_ei,
        b"org.freedesktop.DBus.Introspectable\0" as *const u8 as *const ::core::ffi::c_char
            as gconstpointer,
    )
    .is_null()
    {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    &raw const safe_c2rust_introspect_introspectable_interface
                        as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    s,
                    __val,
                    if ({
                        let mut _g_boolean_var_164: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_164 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_164 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_164
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as ::core::ffi::c_int) as gssize
                    },
                );
            });
        } else {
            safe_c2rust_g_string_append_len_inline(
                s,
                &raw const safe_c2rust_introspect_introspectable_interface
                    as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    g_hash_table_iter_init(&raw mut hash_iter, (*eo).map_if_name_to_ei);
    while g_hash_table_iter_next(
        &raw mut hash_iter,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut ei as gpointer as *mut gpointer,
    ) != 0
    {
        g_dbus_interface_info_generate_xml((*ei).interface_info, 2 as guint, s);
    }
    registered =
        safe_c2rust_g_dbus_connection_list_registered_unlocked(connection, (*eo).object_path);
    n = 0 as guint;
    while !registered.is_null() && !(*registered.offset(n as isize)).is_null() {
        g_string_append_printf(
            s,
            b"  <node name=\"%s\"/>\n\0" as *const u8 as *const gchar,
            *registered.offset(n as isize),
        );
        n = n.wrapping_add(1);
    }
    g_strfreev(registered);
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                &raw const safe_c2rust_introspect_tail as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                s,
                __val,
                if ({
                    let mut _g_boolean_var_165: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_165 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_165 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_165
                }) as ::core::ffi::c_long
                    != 0
                {
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            s,
            &raw const safe_c2rust_introspect_tail as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    reply = g_dbus_message_new_method_reply(message);
    g_dbus_message_set_body(
        reply,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, (*s).str_0),
    );
    safe_c2rust_g_dbus_connection_send_message_unlocked(
        connection,
        reply,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        ::core::ptr::null_mut::<guint32>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_object_unref(reply as gpointer);
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        } else {
            g_string_free_and_steal(s);
        };
    } else {
        g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
    };
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_call_in_idle_cb(mut user_data: gpointer) -> gboolean {
    let mut invocation: *mut GDBusMethodInvocation = user_data as *mut GDBusMethodInvocation;
    let mut vtable: *mut GDBusInterfaceVTable = ::core::ptr::null_mut::<GDBusInterfaceVTable>();
    let mut registration_id: guint = 0;
    let mut subtree_registration_id: guint = 0;
    let mut ei: *mut ExportedInterface = ::core::ptr::null_mut::<ExportedInterface>();
    let mut es: *mut ExportedSubtree = ::core::ptr::null_mut::<ExportedSubtree>();
    registration_id = g_object_get_data(
        invocation as *mut ::core::ffi::c_void as *mut GObject,
        b"g-dbus-registration-id\0" as *const u8 as *const gchar,
    ) as gulong as guint;
    subtree_registration_id = g_object_get_data(
        invocation as *mut ::core::ffi::c_void as *mut GObject,
        b"g-dbus-subtree-registration-id\0" as *const u8 as *const gchar,
    ) as gulong as guint;
    if safe_c2rust_has_object_been_unregistered(
        g_dbus_method_invocation_get_connection(invocation),
        registration_id,
        &raw mut ei,
        subtree_registration_id,
        &raw mut es,
    ) != 0
    {
        let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
        reply = g_dbus_message_new_method_error(
            g_dbus_method_invocation_get_message(invocation),
            b"org.freedesktop.DBus.Error.UnknownMethod\0" as *const u8 as *const gchar,
            glib_gettext(
                b"No such interface \xE2\x80\x9C%s\xE2\x80\x9D on object at path %s\0" as *const u8
                    as *const gchar,
            ),
            g_dbus_method_invocation_get_interface_name(invocation),
            g_dbus_method_invocation_get_object_path(invocation),
        );
        safe_c2rust_g_dbus_connection_send_message(
            g_dbus_method_invocation_get_connection(invocation),
            reply,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            ::core::ptr::null_mut::<guint32>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(reply as gpointer);
    } else {
        vtable = g_object_get_data(
            invocation as *mut ::core::ffi::c_void as *mut GObject,
            b"g-dbus-interface-vtable\0" as *const u8 as *const gchar,
        ) as *mut GDBusInterfaceVTable;
        if ({
            let mut _g_boolean_var_166: ::core::ffi::c_int = 0;
            if !vtable.is_null() && (*vtable).method_call.is_some() {
                _g_boolean_var_166 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_166 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_166
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                5451 as ::core::ffi::c_int,
                G_STRFUNC,
                b"vtable != NULL && vtable->method_call != NULL\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        (*vtable).method_call.expect("non-null function pointer")(
            g_dbus_method_invocation_get_connection(invocation),
            g_dbus_method_invocation_get_sender(invocation),
            g_dbus_method_invocation_get_object_path(invocation),
            g_dbus_method_invocation_get_interface_name(invocation),
            g_dbus_method_invocation_get_method_name(invocation),
            g_dbus_method_invocation_get_parameters(invocation),
            g_object_ref(invocation as gpointer) as *mut GDBusMethodInvocation,
            g_dbus_method_invocation_get_user_data(invocation),
        );
    }
    let mut _pp: *mut *mut ExportedInterface = &raw mut ei;
    let mut _ptr: *mut ExportedInterface = *_pp;
    *_pp = ::core::ptr::null_mut::<ExportedInterface>();
    if !_ptr.is_null() {
        safe_c2rust_exported_interface_unref(_ptr as *mut ExportedInterface);
    }
    let mut _pp_0: *mut *mut ExportedSubtree = &raw mut es;
    let mut _ptr_0: *mut ExportedSubtree = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<ExportedSubtree>();
    if !_ptr_0.is_null() {
        safe_c2rust_exported_subtree_unref(_ptr_0 as *mut ExportedSubtree);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_schedule_method_call(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut registration_id: guint,
    mut subtree_registration_id: guint,
    mut interface_info: *const GDBusInterfaceInfo,
    mut method_info: *const GDBusMethodInfo,
    mut property_info: *const GDBusPropertyInfo,
    mut parameters: *mut GVariant,
    mut vtable: *const GDBusInterfaceVTable,
    mut main_context: *mut GMainContext,
    mut user_data: gpointer,
) {
    let mut invocation: *mut GDBusMethodInvocation =
        ::core::ptr::null_mut::<GDBusMethodInvocation>();
    let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    invocation = _g_dbus_method_invocation_new(
        g_dbus_message_get_sender(message),
        g_dbus_message_get_path(message),
        g_dbus_message_get_interface(message),
        g_dbus_message_get_member(message),
        method_info,
        property_info,
        connection,
        message,
        parameters,
        user_data,
    );
    g_object_set_data(
        invocation as *mut ::core::ffi::c_void as *mut GObject,
        b"g-dbus-interface-vtable\0" as *const u8 as *const gchar,
        vtable as gpointer,
    );
    g_object_set_data(
        invocation as *mut ::core::ffi::c_void as *mut GObject,
        b"g-dbus-registration-id\0" as *const u8 as *const gchar,
        registration_id as gulong as gpointer,
    );
    g_object_set_data(
        invocation as *mut ::core::ffi::c_void as *mut GObject,
        b"g-dbus-subtree-registration-id\0" as *const u8 as *const gchar,
        subtree_registration_id as gulong as gpointer,
    );
    idle_source = g_idle_source_new();
    g_source_set_priority(idle_source, G_PRIORITY_DEFAULT);
    g_source_set_callback(
        idle_source,
        Some(safe_c2rust_call_in_idle_cb as unsafe extern "C" fn(gpointer) -> gboolean),
        safe_c2rust_g_steal_pointer(&raw mut invocation as gpointer) as *mut GDBusMethodInvocation
            as gpointer,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_source_set_static_name(
        idle_source,
        b"[gio, /home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c] call_in_idle_cb\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    g_source_attach(idle_source, main_context);
    g_source_unref(idle_source);
}
unsafe extern "C" fn safe_c2rust_validate_and_maybe_schedule_method_call(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
    mut registration_id: guint,
    mut subtree_registration_id: guint,
    mut interface_info: *mut GDBusInterfaceInfo,
    mut vtable: *const GDBusInterfaceVTable,
    mut main_context: *mut GMainContext,
    mut user_data: gpointer,
) -> gboolean {
    let mut method_info: *mut GDBusMethodInfo = ::core::ptr::null_mut::<GDBusMethodInfo>();
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut parameters: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut handled: gboolean = 0;
    let mut in_type: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    handled = FALSE as gboolean;
    method_info =
        g_dbus_interface_info_lookup_method(interface_info, g_dbus_message_get_member(message));
    if method_info.is_null() {
        reply = g_dbus_message_new_method_error(
            message,
            b"org.freedesktop.DBus.Error.UnknownMethod\0" as *const u8 as *const gchar,
            glib_gettext(
                b"No such method \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8 as *const gchar,
            ),
            g_dbus_message_get_member(message),
        );
        safe_c2rust_g_dbus_connection_send_message_unlocked(
            connection,
            reply,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            ::core::ptr::null_mut::<guint32>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(reply as gpointer);
        handled = TRUE as gboolean;
    } else {
        parameters = g_dbus_message_get_body(message);
        if parameters.is_null() {
            parameters = g_variant_new(b"()\0" as *const u8 as *const gchar);
            g_variant_ref_sink(parameters);
        } else {
            g_variant_ref(parameters);
        }
        in_type = _g_dbus_compute_complete_signature((*method_info).in_args);
        if g_variant_is_of_type(parameters, in_type) == 0 {
            let mut type_string: *mut gchar = ::core::ptr::null_mut::<gchar>();
            type_string = g_variant_type_dup_string(in_type);
            reply = g_dbus_message_new_method_error(
                message,
                b"org.freedesktop.DBus.Error.InvalidArgs\0" as *const u8 as *const gchar,
                glib_gettext(
                    b"Type of message, \xE2\x80\x9C%s\xE2\x80\x9D, does not match expected type \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                g_variant_get_type_string(parameters),
                type_string,
            );
            safe_c2rust_g_dbus_connection_send_message_unlocked(
                connection,
                reply,
                G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                ::core::ptr::null_mut::<guint32>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_variant_type_free(in_type);
            g_variant_unref(parameters);
            g_object_unref(reply as gpointer);
            g_free(type_string as gpointer);
            handled = TRUE as gboolean;
        } else {
            g_variant_type_free(in_type);
            safe_c2rust_schedule_method_call(
                connection,
                message,
                registration_id,
                subtree_registration_id,
                interface_info,
                method_info,
                ::core::ptr::null::<GDBusPropertyInfo>(),
                parameters,
                vtable,
                main_context,
                user_data,
            );
            g_variant_unref(parameters);
            handled = TRUE as gboolean;
        }
    }
    return handled;
}
unsafe extern "C" fn safe_c2rust_obj_message_func(
    mut connection: *mut GDBusConnection,
    mut eo: *mut ExportedObject,
    mut message: *mut GDBusMessage,
    mut object_found: *mut gboolean,
) -> gboolean {
    let mut current_block: u64;
    let mut interface_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut member: *const gchar = ::core::ptr::null::<gchar>();
    let mut signature: *const gchar = ::core::ptr::null::<gchar>();
    let mut handled: gboolean = 0;
    handled = FALSE as gboolean;
    interface_name = g_dbus_message_get_interface(message);
    member = g_dbus_message_get_member(message);
    signature = g_dbus_message_get_signature(message);
    if !interface_name.is_null() {
        let mut ei: *mut ExportedInterface = ::core::ptr::null_mut::<ExportedInterface>();
        ei = g_hash_table_lookup((*eo).map_if_name_to_ei, interface_name as gconstpointer)
            as *mut ExportedInterface;
        if !ei.is_null() {
            if (*ei).vtable.is_null() || (*(*ei).vtable).method_call.is_none() {
                current_block = 520815703870273440;
            } else {
                handled = safe_c2rust_validate_and_maybe_schedule_method_call(
                    connection,
                    message,
                    (*ei).id,
                    0 as guint,
                    (*ei).interface_info,
                    (*ei).vtable,
                    (*ei).context,
                    (*ei).user_data,
                );
                current_block = 520815703870273440;
            }
        } else {
            *object_found = TRUE as gboolean;
            current_block = 11650488183268122163;
        }
    } else {
        current_block = 11650488183268122163;
    }
    match current_block {
        11650488183268122163 => {
            if g_strcmp0(
                interface_name as *const ::core::ffi::c_char,
                b"org.freedesktop.DBus.Introspectable\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && g_strcmp0(
                    member as *const ::core::ffi::c_char,
                    b"Introspect\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                && g_strcmp0(
                    signature as *const ::core::ffi::c_char,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                handled = safe_c2rust_handle_introspect(connection, eo, message);
            } else if g_strcmp0(
                interface_name as *const ::core::ffi::c_char,
                b"org.freedesktop.DBus.Properties\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && g_strcmp0(
                    member as *const ::core::ffi::c_char,
                    b"Get\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                && g_strcmp0(
                    signature as *const ::core::ffi::c_char,
                    b"ss\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                handled = safe_c2rust_handle_getset_property(connection, eo, message, TRUE);
            } else if g_strcmp0(
                interface_name as *const ::core::ffi::c_char,
                b"org.freedesktop.DBus.Properties\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && g_strcmp0(
                    member as *const ::core::ffi::c_char,
                    b"Set\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                && g_strcmp0(
                    signature as *const ::core::ffi::c_char,
                    b"ssv\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                handled = safe_c2rust_handle_getset_property(connection, eo, message, FALSE);
            } else if g_strcmp0(
                interface_name as *const ::core::ffi::c_char,
                b"org.freedesktop.DBus.Properties\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && g_strcmp0(
                    member as *const ::core::ffi::c_char,
                    b"GetAll\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                && g_strcmp0(
                    signature as *const ::core::ffi::c_char,
                    b"s\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                handled = safe_c2rust_handle_get_all_properties(connection, eo, message);
            }
        }
        _ => {}
    }
    return handled;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_register_object(
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
    mut interface_info: *mut GDBusInterfaceInfo,
    mut vtable: *const GDBusInterfaceVTable,
    mut user_data: gpointer,
    mut user_data_free_func: GDestroyNotify,
    mut error: *mut *mut GError,
) -> guint {
    let mut eo: *mut ExportedObject = ::core::ptr::null_mut::<ExportedObject>();
    let mut ei: *mut ExportedInterface = ::core::ptr::null_mut::<ExportedInterface>();
    let mut ret: guint = 0;
    if ({
        let mut _g_boolean_var_167: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_167 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_167 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_167
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_168: ::core::ffi::c_int = 0;
        if !object_path.is_null() && g_variant_is_object_path(object_path) != 0 {
            _g_boolean_var_168 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_168 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_168
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"object_path != NULL && g_variant_is_object_path (object_path)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_169: ::core::ffi::c_int = 0;
        if !interface_info.is_null() {
            _g_boolean_var_169 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_169 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_169
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"interface_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_170: ::core::ffi::c_int = 0;
        if g_dbus_is_interface_name((*interface_info).name) != 0 {
            _g_boolean_var_170 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_170 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_170
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_dbus_is_interface_name (interface_info->name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_171: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_171 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_171 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_171
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_172: ::core::ffi::c_int = 0;
        if safe_c2rust_check_initialized(connection) != 0 {
            _g_boolean_var_172 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_172 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_172
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    ret = 0 as guint;
    g_mutex_lock(&raw mut (*connection).lock);
    eo = g_hash_table_lookup(
        (*connection).map_object_path_to_eo,
        object_path as gconstpointer,
    ) as *mut ExportedObject;
    if eo.is_null() {
        eo = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<ExportedObject>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut ExportedObject;
        (*eo).object_path =
            safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as *mut gchar;
        (*eo).connection = connection;
        (*eo).map_if_name_to_ei = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            None,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut ExportedInterface) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_exported_interface_unref
                    as unsafe extern "C" fn(*mut ExportedInterface) -> (),
            )),
        );
        g_hash_table_insert(
            (*connection).map_object_path_to_eo,
            (*eo).object_path as gpointer,
            eo as gpointer,
        );
    }
    ei = g_hash_table_lookup(
        (*eo).map_if_name_to_ei,
        (*interface_info).name as gconstpointer,
    ) as *mut ExportedInterface;
    if !ei.is_null() {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_EXISTS as ::core::ffi::c_int as gint,
            glib_gettext(
                b"An object is already exported for the interface %s at %s\0" as *const u8
                    as *const gchar,
            ),
            (*interface_info).name,
            object_path,
        );
    } else {
        ei = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<ExportedInterface>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut ExportedInterface;
        (*ei).refcount = 1 as ::core::ffi::c_int as gint;
        (*ei).id = ({
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust__global_registration_id;
            } else {
            };
            crate::translated::compat::atomic_xadd_seqcst(
                &raw mut safe_c2rust__global_registration_id,
                1 as ::core::ffi::c_int as guint,
            ) as gint
        }) as guint;
        (*ei).eo = eo;
        (*ei).user_data = user_data;
        (*ei).user_data_free_func = user_data_free_func;
        (*ei).vtable = safe_c2rust__g_dbus_interface_vtable_copy(vtable);
        (*ei).interface_info = g_dbus_interface_info_ref(interface_info);
        g_dbus_interface_info_cache_build((*ei).interface_info);
        (*ei).interface_name = safe_c2rust_g_strdup_inline((*interface_info).name) as *mut gchar;
        (*ei).context = g_main_context_ref_thread_default();
        g_hash_table_insert(
            (*eo).map_if_name_to_ei,
            (*ei).interface_name as gpointer,
            ei as gpointer,
        );
        g_hash_table_insert(
            (*connection).map_id_to_ei,
            (*ei).id as gulong as gpointer,
            ei as gpointer,
        );
        ret = (*ei).id;
    }
    g_mutex_unlock(&raw mut (*connection).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_unregister_object(
    mut connection: *mut GDBusConnection,
    mut registration_id: guint,
) -> gboolean {
    let mut ei: *mut ExportedInterface = ::core::ptr::null_mut::<ExportedInterface>();
    let mut eo: *mut ExportedObject = ::core::ptr::null_mut::<ExportedObject>();
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_173: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_173 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_173 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_173
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
        let mut _g_boolean_var_174: ::core::ffi::c_int = 0;
        if safe_c2rust_check_initialized(connection) != 0 {
            _g_boolean_var_174 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_174 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_174
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    g_mutex_lock(&raw mut (*connection).lock);
    ei = g_hash_table_lookup(
        (*connection).map_id_to_ei,
        registration_id as gulong as gpointer as gconstpointer,
    ) as *mut ExportedInterface;
    if !ei.is_null() {
        eo = (*ei).eo;
        if !(({
            let mut _g_boolean_var_175: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*connection).map_id_to_ei,
                (*ei).id as gulong as gpointer as gconstpointer,
            ) != 0
            {
                _g_boolean_var_175 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_175 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_175
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                5844 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (connection->map_id_to_ei, GUINT_TO_POINTER (ei->id))\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !(({
            let mut _g_boolean_var_176: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*eo).map_if_name_to_ei,
                (*ei).interface_name as gconstpointer,
            ) != 0
            {
                _g_boolean_var_176 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_176 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_176
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                5845 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (eo->map_if_name_to_ei, ei->interface_name)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if g_hash_table_size((*eo).map_if_name_to_ei) == 0 as guint {
            if !(({
                let mut _g_boolean_var_177: ::core::ffi::c_int = 0;
                if g_hash_table_remove(
                    (*connection).map_object_path_to_eo,
                    (*eo).object_path as gconstpointer,
                ) != 0
                {
                    _g_boolean_var_177 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_177 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_177
            }) as ::core::ffi::c_long
                != 0)
            {
                g_warn_message(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    5849 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"g_hash_table_remove (connection->map_object_path_to_eo, eo->object_path)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        ret = TRUE as gboolean;
    }
    g_mutex_unlock(&raw mut (*connection).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_register_object_data_new(
    mut method_call_closure: *mut GClosure,
    mut get_property_closure: *mut GClosure,
    mut set_property_closure: *mut GClosure,
) -> *mut RegisterObjectData {
    let mut data: *mut RegisterObjectData = ::core::ptr::null_mut::<RegisterObjectData>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<RegisterObjectData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut RegisterObjectData;
    if !method_call_closure.is_null() {
        (*data).method_call_closure = g_closure_ref(method_call_closure);
        g_closure_sink(method_call_closure);
        if (*method_call_closure).marshal.is_none() {
            g_closure_set_marshal(
                method_call_closure,
                Some(
                    g_cclosure_marshal_generic
                        as unsafe extern "C" fn(
                            *mut GClosure,
                            *mut GValue,
                            guint,
                            *const GValue,
                            gpointer,
                            gpointer,
                        ) -> (),
                ),
            );
        }
    }
    if !get_property_closure.is_null() {
        (*data).get_property_closure = g_closure_ref(get_property_closure);
        g_closure_sink(get_property_closure);
        if (*get_property_closure).marshal.is_none() {
            g_closure_set_marshal(
                get_property_closure,
                Some(
                    g_cclosure_marshal_generic
                        as unsafe extern "C" fn(
                            *mut GClosure,
                            *mut GValue,
                            guint,
                            *const GValue,
                            gpointer,
                            gpointer,
                        ) -> (),
                ),
            );
        }
    }
    if !set_property_closure.is_null() {
        (*data).set_property_closure = g_closure_ref(set_property_closure);
        g_closure_sink(set_property_closure);
        if (*set_property_closure).marshal.is_none() {
            g_closure_set_marshal(
                set_property_closure,
                Some(
                    g_cclosure_marshal_generic
                        as unsafe extern "C" fn(
                            *mut GClosure,
                            *mut GValue,
                            guint,
                            *const GValue,
                            gpointer,
                            gpointer,
                        ) -> (),
                ),
            );
        }
    }
    return data;
}
unsafe extern "C" fn safe_c2rust_register_object_free_func(mut user_data: gpointer) {
    let mut data: *mut RegisterObjectData = user_data as *mut RegisterObjectData;
    let mut _pp: *mut *mut GClosure = &raw mut (*data).method_call_closure;
    let mut _ptr: *mut GClosure = *_pp;
    *_pp = ::core::ptr::null_mut::<GClosure>();
    if !_ptr.is_null() {
        g_closure_unref(_ptr as *mut GClosure);
    }
    let mut _pp_0: *mut *mut GClosure = &raw mut (*data).get_property_closure;
    let mut _ptr_0: *mut GClosure = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<GClosure>();
    if !_ptr_0.is_null() {
        g_closure_unref(_ptr_0 as *mut GClosure);
    }
    let mut _pp_1: *mut *mut GClosure = &raw mut (*data).set_property_closure;
    let mut _ptr_1: *mut GClosure = *_pp_1;
    *_pp_1 = ::core::ptr::null_mut::<GClosure>();
    if !_ptr_1.is_null() {
        g_closure_unref(_ptr_1 as *mut GClosure);
    }
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_register_with_closures_on_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut data: *mut RegisterObjectData = user_data as *mut RegisterObjectData;
    let mut params: [GValue; 7] = [
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
    ];
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        safe_c2rust_g_dbus_connection_get_type(),
    );
    g_value_set_object(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        connection as gpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        sender,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        object_path,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(3 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(3 as ::core::ffi::c_int as isize) as *mut GValue,
        interface_name,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(4 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(4 as ::core::ffi::c_int as isize) as *mut GValue,
        method_name,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(5 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_VARIANT,
    );
    g_value_set_variant(
        (&raw mut params as *mut GValue).offset(5 as ::core::ffi::c_int as isize) as *mut GValue,
        parameters,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(6 as ::core::ffi::c_int as isize) as *mut GValue,
        g_dbus_method_invocation_get_type(),
    );
    g_value_set_object(
        (&raw mut params as *mut GValue).offset(6 as ::core::ffi::c_int as isize) as *mut GValue,
        invocation as gpointer,
    );
    g_closure_invoke(
        (*data).method_call_closure,
        ::core::ptr::null_mut::<GValue>(),
        (::core::mem::size_of::<[GValue; 7]>() as usize)
            .wrapping_div(::core::mem::size_of::<GValue>() as usize) as guint,
        &raw mut params as *mut GValue,
        NULL_0,
    );
    g_value_unset((&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(3 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(4 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(5 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(6 as ::core::ffi::c_int as isize));
}
unsafe extern "C" fn safe_c2rust_register_with_closures_on_get_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> *mut GVariant {
    let mut data: *mut RegisterObjectData = user_data as *mut RegisterObjectData;
    let mut params: [GValue; 5] = [
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
    ];
    let mut result_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        safe_c2rust_g_dbus_connection_get_type(),
    );
    g_value_set_object(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        connection as gpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        sender,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        object_path,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(3 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(3 as ::core::ffi::c_int as isize) as *mut GValue,
        interface_name,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(4 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(4 as ::core::ffi::c_int as isize) as *mut GValue,
        property_name,
    );
    g_value_init(&raw mut result_value, G_TYPE_VARIANT);
    g_closure_invoke(
        (*data).get_property_closure,
        &raw mut result_value,
        (::core::mem::size_of::<[GValue; 5]>() as usize)
            .wrapping_div(::core::mem::size_of::<GValue>() as usize) as guint,
        &raw mut params as *mut GValue,
        NULL_0,
    );
    result = g_value_get_variant(&raw mut result_value);
    if !result.is_null() {
        g_variant_ref(result);
    }
    g_value_unset((&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(3 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(4 as ::core::ffi::c_int as isize));
    g_value_unset(&raw mut result_value);
    if result.is_null() {
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Unable to retrieve property %s.%s\0" as *const u8 as *const gchar),
            interface_name,
            property_name,
        );
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_register_with_closures_on_set_property(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut property_name: *const gchar,
    mut value: *mut GVariant,
    mut error: *mut *mut GError,
    mut user_data: gpointer,
) -> gboolean {
    let mut data: *mut RegisterObjectData = user_data as *mut RegisterObjectData;
    let mut params: [GValue; 6] = [
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
        _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed_1 {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed_1 { v_int: 0 },
            ],
        },
    ];
    let mut result_value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed_1 {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed_1 { v_int: 0 },
        ],
    };
    let mut result: gboolean = 0;
    g_value_init(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        safe_c2rust_g_dbus_connection_get_type(),
    );
    g_value_set_object(
        (&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize) as *mut GValue,
        connection as gpointer,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize) as *mut GValue,
        sender,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize) as *mut GValue,
        object_path,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(3 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(3 as ::core::ffi::c_int as isize) as *mut GValue,
        interface_name,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(4 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_STRING,
    );
    g_value_set_string(
        (&raw mut params as *mut GValue).offset(4 as ::core::ffi::c_int as isize) as *mut GValue,
        property_name,
    );
    g_value_init(
        (&raw mut params as *mut GValue).offset(5 as ::core::ffi::c_int as isize) as *mut GValue,
        G_TYPE_VARIANT,
    );
    g_value_set_variant(
        (&raw mut params as *mut GValue).offset(5 as ::core::ffi::c_int as isize) as *mut GValue,
        value,
    );
    g_value_init(&raw mut result_value, G_TYPE_BOOLEAN);
    g_closure_invoke(
        (*data).set_property_closure,
        &raw mut result_value,
        (::core::mem::size_of::<[GValue; 6]>() as usize)
            .wrapping_div(::core::mem::size_of::<GValue>() as usize) as guint,
        &raw mut params as *mut GValue,
        NULL_0,
    );
    result = g_value_get_boolean(&raw mut result_value);
    g_value_unset((&raw mut params as *mut GValue).offset(0 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(1 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(2 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(3 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(4 as ::core::ffi::c_int as isize));
    g_value_unset((&raw mut params as *mut GValue).offset(5 as ::core::ffi::c_int as isize));
    g_value_unset(&raw mut result_value);
    if result == 0 {
        g_set_error(
            error,
            g_dbus_error_quark(),
            G_DBUS_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Unable to set property %s.%s\0" as *const u8 as *const gchar),
            interface_name,
            property_name,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_register_object_with_closures(
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
    mut interface_info: *mut GDBusInterfaceInfo,
    mut method_call_closure: *mut GClosure,
    mut get_property_closure: *mut GClosure,
    mut set_property_closure: *mut GClosure,
    mut error: *mut *mut GError,
) -> guint {
    let mut data: *mut RegisterObjectData = ::core::ptr::null_mut::<RegisterObjectData>();
    let mut vtable: GDBusInterfaceVTable = _GDBusInterfaceVTable {
        method_call: if !method_call_closure.is_null() {
            Some(
                safe_c2rust_register_with_closures_on_method_call
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
        } else {
            None
        },
        get_property: if !get_property_closure.is_null() {
            Some(
                safe_c2rust_register_with_closures_on_get_property
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut *mut GError,
                        gpointer,
                    ) -> *mut GVariant,
            )
        } else {
            None
        },
        set_property: if !set_property_closure.is_null() {
            Some(
                safe_c2rust_register_with_closures_on_set_property
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut GVariant,
                        *mut *mut GError,
                        gpointer,
                    ) -> gboolean,
            )
        } else {
            None
        },
        padding: [
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ],
    };
    data = safe_c2rust_register_object_data_new(
        method_call_closure,
        get_property_closure,
        set_property_closure,
    );
    return safe_c2rust_g_dbus_connection_register_object(
        connection,
        object_path,
        interface_info,
        &raw mut vtable,
        data as gpointer,
        Some(safe_c2rust_register_object_free_func as unsafe extern "C" fn(gpointer) -> ()),
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_emit_signal(
    mut connection: *mut GDBusConnection,
    mut destination_bus_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut ret: gboolean = 0;
    message = ::core::ptr::null_mut::<GDBusMessage>();
    ret = FALSE as gboolean;
    if ({
        let mut _g_boolean_var_178: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_178 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_178 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_178
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
        let mut _g_boolean_var_179: ::core::ffi::c_int = 0;
        if destination_bus_name.is_null() || g_dbus_is_name(destination_bus_name) != 0 {
            _g_boolean_var_179 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_179 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_179
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"destination_bus_name == NULL || g_dbus_is_name (destination_bus_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_180: ::core::ffi::c_int = 0;
        if !object_path.is_null() && g_variant_is_object_path(object_path) != 0 {
            _g_boolean_var_180 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_180 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_180
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"object_path != NULL && g_variant_is_object_path (object_path)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_181: ::core::ffi::c_int = 0;
        if !interface_name.is_null() && g_dbus_is_interface_name(interface_name) != 0 {
            _g_boolean_var_181 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_181 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_181
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"interface_name != NULL && g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_182: ::core::ffi::c_int = 0;
        if !signal_name.is_null() && g_dbus_is_member_name(signal_name) != 0 {
            _g_boolean_var_182 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_182 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_182
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"signal_name != NULL && g_dbus_is_member_name (signal_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_183: ::core::ffi::c_int = 0;
        if parameters.is_null()
            || g_variant_is_of_type(
                parameters,
                b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
            ) != 0
        {
            _g_boolean_var_183 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_183 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_183
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
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_184: ::core::ffi::c_int = 0;
        if safe_c2rust_check_initialized(connection) != 0 {
            _g_boolean_var_184 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_184 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_184
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_185: ::core::ffi::c_int = 0;
        if _g_dbus_debug_emission() != 0 {
            _g_boolean_var_185 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_185 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_185
    }) as ::core::ffi::c_long
        != 0
    {
        _g_dbus_debug_print_lock();
        g_print(
            b"========================================================================\nGDBus-debug:Emission:\n >>>> SIGNAL EMISSION %s.%s()\n      on object %s\n      destination %s\n\0"
                as *const u8 as *const gchar,
            interface_name,
            signal_name,
            object_path,
            if !destination_bus_name.is_null() {
                destination_bus_name as *const ::core::ffi::c_char
            } else {
                b"(none)\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        _g_dbus_debug_print_unlock();
    }
    message = g_dbus_message_new_signal(object_path, interface_name, signal_name);
    if !destination_bus_name.is_null() {
        g_dbus_message_set_header(
            message,
            G_DBUS_MESSAGE_HEADER_FIELD_DESTINATION,
            g_variant_new_string(destination_bus_name),
        );
    }
    if !parameters.is_null() {
        g_dbus_message_set_body(message, parameters);
    }
    ret = safe_c2rust_g_dbus_connection_send_message(
        connection,
        message,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        ::core::ptr::null_mut::<guint32>(),
        error,
    );
    g_object_unref(message as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_add_call_flags(
    mut message: *mut GDBusMessage,
    mut flags: GDBusCallFlags,
) {
    let mut msg_flags: GDBusMessageFlags = G_DBUS_MESSAGE_FLAGS_NONE;
    if flags as ::core::ffi::c_uint
        & G_DBUS_CALL_FLAGS_NO_AUTO_START as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        msg_flags = ::core::mem::transmute::<::core::ffi::c_uint, GDBusMessageFlags>(
            msg_flags as ::core::ffi::c_uint
                | G_DBUS_MESSAGE_FLAGS_NO_AUTO_START as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    }
    if flags as ::core::ffi::c_uint
        & G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION as ::core::ffi::c_int
            as ::core::ffi::c_uint
        != 0
    {
        msg_flags = ::core::mem::transmute::<::core::ffi::c_uint, GDBusMessageFlags>(
            msg_flags as ::core::ffi::c_uint
                | G_DBUS_MESSAGE_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
        );
    }
    if msg_flags as u64 != 0 {
        g_dbus_message_set_flags(message, msg_flags);
    }
}
unsafe extern "C" fn safe_c2rust_decode_method_reply(
    mut reply: *mut GDBusMessage,
    mut method_name: *const gchar,
    mut reply_type: *const GVariantType,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    result = ::core::ptr::null_mut::<GVariant>();
    match g_dbus_message_get_message_type(reply) as ::core::ffi::c_uint {
        2 => {
            result = g_dbus_message_get_body(reply);
            if result.is_null() {
                result = g_variant_new(b"()\0" as *const u8 as *const gchar);
                g_variant_ref_sink(result);
            } else {
                g_variant_ref(result);
            }
            if g_variant_is_of_type(result, reply_type) == 0 {
                let mut type_string: *mut gchar = g_variant_type_dup_string(reply_type);
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Method \xE2\x80\x9C%s\xE2\x80\x9D returned type \xE2\x80\x9C%s\xE2\x80\x9D, but expected \xE2\x80\x9C%s\xE2\x80\x9D\0"
                            as *const u8 as *const gchar,
                    ),
                    method_name,
                    g_variant_get_type_string(result),
                    type_string,
                );
                g_variant_unref(result);
                g_free(type_string as gpointer);
                result = ::core::ptr::null_mut::<GVariant>();
            }
            if !result.is_null() {
                if !out_fd_list.is_null() {
                    *out_fd_list = g_dbus_message_get_unix_fd_list(reply);
                    if !(*out_fd_list).is_null() {
                        g_object_ref(*out_fd_list as gpointer);
                    }
                }
            }
        }
        3 => {
            g_dbus_message_to_gerror(reply, error);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                6265 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    return result;
}
unsafe extern "C" fn safe_c2rust_call_state_free(mut state: *mut CallState) {
    g_variant_type_free((*state).reply_type);
    g_free((*state).method_name as gpointer);
    if !(*state).fd_list.is_null() {
        g_object_unref((*state).fd_list as gpointer);
    }
    g_slice_free1(
        ::core::mem::size_of::<CallState>() as gsize,
        state as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_call_done(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut connection: *mut GDBusConnection =
        source as *mut ::core::ffi::c_void as *mut GDBusConnection;
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut state: *mut CallState = g_task_get_task_data(task) as *mut CallState;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    reply = safe_c2rust_g_dbus_connection_send_message_with_reply_finish(
        connection,
        result,
        &raw mut error,
    );
    if ({
        let mut _g_boolean_var_186: ::core::ffi::c_int = 0;
        if _g_dbus_debug_call() != 0 {
            _g_boolean_var_186 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_186 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_186
    }) as ::core::ffi::c_long
        != 0
    {
        _g_dbus_debug_print_lock();
        g_print(
            b"========================================================================\nGDBus-debug:Call:\n <<<< ASYNC COMPLETE %s()\0"
                as *const u8 as *const gchar,
            (*state).method_name,
        );
        if !reply.is_null() {
            g_print(
                b" (serial %d)\n      SUCCESS\n\0" as *const u8 as *const gchar,
                g_dbus_message_get_reply_serial(reply),
            );
        } else {
            g_print(
                b"\n      FAILED: %s\n\0" as *const u8 as *const gchar,
                (*error).message,
            );
        }
        _g_dbus_debug_print_unlock();
    }
    if !reply.is_null() {
        value = safe_c2rust_decode_method_reply(
            reply,
            (*state).method_name,
            (*state).reply_type,
            &raw mut (*state).fd_list,
            &raw mut error,
        );
    }
    if !error.is_null() {
        g_task_return_error(task, error);
    } else {
        g_task_return_pointer(
            task,
            value as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVariant) -> ()>,
                GDestroyNotify,
            >(Some(
                g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> (),
            )),
        );
    }
    let mut _pp: *mut *mut GDBusMessage = &raw mut reply;
    let mut _ptr: *mut GDBusMessage = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusMessage>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_object_unref(task as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_call_internal(
    mut connection: *mut GDBusConnection,
    mut bus_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut reply_type: *const GVariantType,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut serial: guint32 = 0;
    if ({
        let mut _g_boolean_var_187: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_187 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_187 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_187
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
        let mut _g_boolean_var_188: ::core::ffi::c_int = 0;
        if bus_name.is_null() || g_dbus_is_name(bus_name) != 0 {
            _g_boolean_var_188 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_188 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_188
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bus_name == NULL || g_dbus_is_name (bus_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_189: ::core::ffi::c_int = 0;
        if !object_path.is_null() && g_variant_is_object_path(object_path) != 0 {
            _g_boolean_var_189 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_189 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_189
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"object_path != NULL && g_variant_is_object_path (object_path)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_190: ::core::ffi::c_int = 0;
        if !interface_name.is_null() && g_dbus_is_interface_name(interface_name) != 0 {
            _g_boolean_var_190 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_190 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_190
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"interface_name != NULL && g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_191: ::core::ffi::c_int = 0;
        if !method_name.is_null() && g_dbus_is_member_name(method_name) != 0 {
            _g_boolean_var_191 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_191 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_191
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"method_name != NULL && g_dbus_is_member_name (method_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_192: ::core::ffi::c_int = 0;
        if timeout_msec >= 0 as ::core::ffi::c_int || timeout_msec == -(1 as ::core::ffi::c_int) {
            _g_boolean_var_192 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_192 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_192
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"timeout_msec >= 0 || timeout_msec == -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_193: ::core::ffi::c_int = 0;
        if parameters.is_null()
            || g_variant_is_of_type(
                parameters,
                b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
            ) != 0
        {
            _g_boolean_var_193 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_193 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_193
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(parameters == NULL) || g_variant_is_of_type (parameters, G_VARIANT_TYPE_TUPLE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_194: ::core::ffi::c_int = 0;
        if safe_c2rust_check_initialized(connection) != 0 {
            _g_boolean_var_194 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_194 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_194
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_195: ::core::ffi::c_int = 0;
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
            _g_boolean_var_195 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_195 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_195
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
    message = g_dbus_message_new_method_call(bus_name, object_path, interface_name, method_name);
    safe_c2rust_add_call_flags(message, flags);
    if !parameters.is_null() {
        g_dbus_message_set_body(message, parameters);
    }
    if !fd_list.is_null() {
        g_dbus_message_set_unix_fd_list(message, fd_list);
    }
    if callback.is_some() {
        let mut state: *mut CallState = ::core::ptr::null_mut::<CallState>();
        let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
        state = ({
            let mut __s: gsize = ::core::mem::size_of::<CallState>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            __p = g_slice_alloc(__s);
            memset(
                __p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                __s as size_t,
            );
            __p
        }) as *mut CallState;
        (*state).method_name = g_strjoin(
            b".\0" as *const u8 as *const gchar,
            interface_name,
            method_name,
            NULL_0,
        );
        if reply_type.is_null() {
            reply_type = G_VARIANT_TYPE_ANY;
        }
        (*state).reply_type = g_variant_type_copy(reply_type);
        task = g_task_new(connection as gpointer, cancellable, callback, user_data);
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
                        *const GVariantType,
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
                safe_c2rust_g_dbus_connection_call_internal
                    as unsafe extern "C" fn(
                        *mut GDBusConnection,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *const gchar,
                        *mut GVariant,
                        *const GVariantType,
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
                b"g_dbus_connection_call_internal\0" as *const u8 as *const gchar,
            );
        }
        g_task_set_task_data(
            task,
            state as gpointer,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut CallState) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_call_state_free as unsafe extern "C" fn(*mut CallState) -> (),
            )),
        );
        safe_c2rust_g_dbus_connection_send_message_with_reply(
            connection,
            message,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            timeout_msec,
            &raw mut serial as *mut guint32,
            cancellable,
            Some(
                safe_c2rust_g_dbus_connection_call_done
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    } else {
        let mut msg_flags: GDBusMessageFlags = G_DBUS_MESSAGE_FLAGS_NONE;
        msg_flags = g_dbus_message_get_flags(message);
        msg_flags = ::core::mem::transmute::<::core::ffi::c_uint, GDBusMessageFlags>(
            msg_flags as ::core::ffi::c_uint
                | G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
        );
        g_dbus_message_set_flags(message, msg_flags);
        safe_c2rust_g_dbus_connection_send_message(
            connection,
            message,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            &raw mut serial as *mut guint32,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if ({
        let mut _g_boolean_var_196: ::core::ffi::c_int = 0;
        if _g_dbus_debug_call() != 0 {
            _g_boolean_var_196 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_196 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_196
    }) as ::core::ffi::c_long
        != 0
    {
        _g_dbus_debug_print_lock();
        g_print(
            b"========================================================================\nGDBus-debug:Call:\n >>>> ASYNC %s.%s()\n      on object %s\n      owned by name %s (serial %d)\n\0"
                as *const u8 as *const gchar,
            interface_name,
            method_name,
            object_path,
            if !bus_name.is_null() {
                bus_name as *const ::core::ffi::c_char
            } else {
                b"(none)\0" as *const u8 as *const ::core::ffi::c_char
            },
            serial,
        );
        _g_dbus_debug_print_unlock();
    }
    if !message.is_null() {
        g_object_unref(message as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_call_finish_internal(
    mut connection: *mut GDBusConnection,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut state: *mut CallState = ::core::ptr::null_mut::<CallState>();
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_197: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_197 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_197 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_197
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_198: ::core::ffi::c_int = 0;
        if g_task_is_valid(res as gpointer, connection as gpointer) != 0 {
            _g_boolean_var_198 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_198 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_198
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_199: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_199 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_199 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_199
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
    task = res as *mut ::core::ffi::c_void as *mut GTask;
    state = g_task_get_task_data(task) as *mut CallState;
    ret = g_task_propagate_pointer(task, error) as *mut GVariant;
    if ret.is_null() {
        return ::core::ptr::null_mut::<GVariant>();
    }
    if !out_fd_list.is_null() {
        *out_fd_list = (if !(*state).fd_list.is_null() {
            g_object_ref((*state).fd_list as gpointer) as *mut GUnixFDList
        } else {
            ::core::ptr::null_mut::<GUnixFDList>()
        }) as *mut GUnixFDList;
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_connection_call_sync_internal(
    mut connection: *mut GDBusConnection,
    mut bus_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut reply_type: *const GVariantType,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut fd_list: *mut GUnixFDList,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut send_flags: GDBusSendMessageFlags = G_DBUS_SEND_MESSAGE_FLAGS_NONE;
    message = ::core::ptr::null_mut::<GDBusMessage>();
    reply = ::core::ptr::null_mut::<GDBusMessage>();
    result = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_200: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_200 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_200 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_200
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_201: ::core::ffi::c_int = 0;
        if bus_name.is_null() || g_dbus_is_name(bus_name) != 0 {
            _g_boolean_var_201 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_201 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_201
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"bus_name == NULL || g_dbus_is_name (bus_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_202: ::core::ffi::c_int = 0;
        if !object_path.is_null() && g_variant_is_object_path(object_path) != 0 {
            _g_boolean_var_202 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_202 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_202
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"object_path != NULL && g_variant_is_object_path (object_path)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_203: ::core::ffi::c_int = 0;
        if !interface_name.is_null() && g_dbus_is_interface_name(interface_name) != 0 {
            _g_boolean_var_203 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_203 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_203
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"interface_name != NULL && g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_204: ::core::ffi::c_int = 0;
        if !method_name.is_null() && g_dbus_is_member_name(method_name) != 0 {
            _g_boolean_var_204 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_204 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_204
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"method_name != NULL && g_dbus_is_member_name (method_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_205: ::core::ffi::c_int = 0;
        if timeout_msec >= 0 as ::core::ffi::c_int || timeout_msec == -(1 as ::core::ffi::c_int) {
            _g_boolean_var_205 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_205 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_205
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"timeout_msec >= 0 || timeout_msec == -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_206: ::core::ffi::c_int = 0;
        if parameters.is_null()
            || g_variant_is_of_type(
                parameters,
                b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
            ) != 0
        {
            _g_boolean_var_206 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_206 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_206
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(parameters == NULL) || g_variant_is_of_type (parameters, G_VARIANT_TYPE_TUPLE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_207: ::core::ffi::c_int = 0;
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
            _g_boolean_var_207 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_207 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_207
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
        let mut _g_boolean_var_208: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_208 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_208 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_208
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
    if flags as ::core::ffi::c_uint & CALL_FLAGS_INITIALIZING == 0 {
        if ({
            let mut _g_boolean_var_209: ::core::ffi::c_int = 0;
            if safe_c2rust_check_initialized(connection) != 0 {
                _g_boolean_var_209 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_209 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_209
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<GVariant>();
        }
    }
    if reply_type.is_null() {
        reply_type = G_VARIANT_TYPE_ANY;
    }
    message = g_dbus_message_new_method_call(bus_name, object_path, interface_name, method_name);
    safe_c2rust_add_call_flags(message, flags);
    if !parameters.is_null() {
        g_dbus_message_set_body(message, parameters);
    }
    if !fd_list.is_null() {
        g_dbus_message_set_unix_fd_list(message, fd_list);
    }
    if ({
        let mut _g_boolean_var_210: ::core::ffi::c_int = 0;
        if _g_dbus_debug_call() != 0 {
            _g_boolean_var_210 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_210 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_210
    }) as ::core::ffi::c_long
        != 0
    {
        _g_dbus_debug_print_lock();
        g_print(
            b"========================================================================\nGDBus-debug:Call:\n >>>> SYNC %s.%s()\n      on object %s\n      owned by name %s\n\0"
                as *const u8 as *const gchar,
            interface_name,
            method_name,
            object_path,
            if !bus_name.is_null() {
                bus_name as *const ::core::ffi::c_char
            } else {
                b"(none)\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        _g_dbus_debug_print_unlock();
    }
    local_error = ::core::ptr::null_mut::<GError>();
    send_flags = G_DBUS_SEND_MESSAGE_FLAGS_NONE;
    if flags as ::core::ffi::c_uint & CALL_FLAGS_INITIALIZING != 0 {
        send_flags = ::core::mem::transmute::<::core::ffi::c_uint, GDBusSendMessageFlags>(
            send_flags as ::core::ffi::c_uint | SEND_MESSAGE_FLAGS_INITIALIZING,
        );
    }
    reply = safe_c2rust_g_dbus_connection_send_message_with_reply_sync(
        connection,
        message,
        send_flags,
        timeout_msec,
        ::core::ptr::null_mut::<guint32>(),
        cancellable,
        &raw mut local_error,
    );
    if ({
        let mut _g_boolean_var_211: ::core::ffi::c_int = 0;
        if _g_dbus_debug_call() != 0 {
            _g_boolean_var_211 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_211 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_211
    }) as ::core::ffi::c_long
        != 0
    {
        _g_dbus_debug_print_lock();
        g_print(
            b"========================================================================\nGDBus-debug:Call:\n <<<< SYNC COMPLETE %s.%s()\n      \0"
                as *const u8 as *const gchar,
            interface_name,
            method_name,
        );
        if !reply.is_null() {
            g_print(b"SUCCESS\n\0" as *const u8 as *const gchar);
        } else {
            g_print(
                b"FAILED: %s\n\0" as *const u8 as *const gchar,
                (*local_error).message,
            );
        }
        _g_dbus_debug_print_unlock();
    }
    if reply.is_null() {
        if !error.is_null() {
            *error = local_error;
        } else {
            g_error_free(local_error);
        }
    } else {
        result =
            safe_c2rust_decode_method_reply(reply, method_name, reply_type, out_fd_list, error);
    }
    if !message.is_null() {
        g_object_unref(message as gpointer);
    }
    if !reply.is_null() {
        g_object_unref(reply as gpointer);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_call(
    mut connection: *mut GDBusConnection,
    mut bus_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut reply_type: *const GVariantType,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    safe_c2rust_g_dbus_connection_call_internal(
        connection,
        bus_name,
        object_path,
        interface_name,
        method_name,
        parameters,
        reply_type,
        flags,
        timeout_msec,
        ::core::ptr::null_mut::<GUnixFDList>(),
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_call_finish(
    mut connection: *mut GDBusConnection,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    return safe_c2rust_g_dbus_connection_call_finish_internal(
        connection,
        ::core::ptr::null_mut::<*mut GUnixFDList>(),
        res,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_call_sync(
    mut connection: *mut GDBusConnection,
    mut bus_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut reply_type: *const GVariantType,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    return safe_c2rust_g_dbus_connection_call_sync_internal(
        connection,
        bus_name,
        object_path,
        interface_name,
        method_name,
        parameters,
        reply_type,
        flags,
        timeout_msec,
        ::core::ptr::null_mut::<GUnixFDList>(),
        ::core::ptr::null_mut::<*mut GUnixFDList>(),
        cancellable,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_call_with_unix_fd_list(
    mut connection: *mut GDBusConnection,
    mut bus_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut reply_type: *const GVariantType,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut fd_list: *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    safe_c2rust_g_dbus_connection_call_internal(
        connection,
        bus_name,
        object_path,
        interface_name,
        method_name,
        parameters,
        reply_type,
        flags,
        timeout_msec,
        fd_list,
        cancellable,
        callback,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_call_with_unix_fd_list_finish(
    mut connection: *mut GDBusConnection,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    return safe_c2rust_g_dbus_connection_call_finish_internal(connection, out_fd_list, res, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_call_with_unix_fd_list_sync(
    mut connection: *mut GDBusConnection,
    mut bus_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut reply_type: *const GVariantType,
    mut flags: GDBusCallFlags,
    mut timeout_msec: gint,
    mut fd_list: *mut GUnixFDList,
    mut out_fd_list: *mut *mut GUnixFDList,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    return safe_c2rust_g_dbus_connection_call_sync_internal(
        connection,
        bus_name,
        object_path,
        interface_name,
        method_name,
        parameters,
        reply_type,
        flags,
        timeout_msec,
        fd_list,
        out_fd_list,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_handle_subtree_introspect(
    mut connection: *mut GDBusConnection,
    mut es: *mut ExportedSubtree,
    mut message: *mut GDBusMessage,
) -> gboolean {
    let mut current_block: u64;
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut handled: gboolean = 0;
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut children: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut is_root: gboolean = 0;
    let mut sender: *const gchar = ::core::ptr::null::<gchar>();
    let mut requested_object_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut requested_node: *const gchar = ::core::ptr::null::<gchar>();
    let mut interfaces: *mut *mut GDBusInterfaceInfo =
        ::core::ptr::null_mut::<*mut GDBusInterfaceInfo>();
    let mut n: guint = 0;
    let mut subnode_paths: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut has_properties_interface: gboolean = 0;
    let mut has_introspectable_interface: gboolean = 0;
    handled = FALSE as gboolean;
    requested_object_path = g_dbus_message_get_path(message);
    sender = g_dbus_message_get_sender(message);
    is_root = (g_strcmp0(
        requested_object_path as *const ::core::ffi::c_char,
        (*es).object_path,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    s = g_string_new(::core::ptr::null::<gchar>());
    safe_c2rust_introspect_append_header(s);
    children = (*(*es).vtable)
        .enumerate
        .expect("non-null function pointer")(
        (*es).connection,
        sender,
        (*es).object_path,
        (*es).user_data,
    );
    if is_root == 0 {
        requested_node = strrchr(
            requested_object_path as *const ::core::ffi::c_char,
            '/' as i32,
        )
        .offset(1 as ::core::ffi::c_int as isize);
        if (*es).flags as ::core::ffi::c_uint
            & G_DBUS_SUBTREE_FLAGS_DISPATCH_TO_UNENUMERATED_NODES as ::core::ffi::c_int
                as ::core::ffi::c_uint
            == 0
            && g_strv_contains(children as *const *const gchar, requested_node) == 0
        {
            current_block = 10670893614424213133;
        } else {
            current_block = 4166486009154926805;
        }
    } else {
        requested_node = ::core::ptr::null::<gchar>();
        current_block = 4166486009154926805;
    }
    match current_block {
        4166486009154926805 => {
            interfaces = (*(*es).vtable)
                .introspect
                .expect("non-null function pointer")(
                (*es).connection,
                sender,
                (*es).object_path,
                requested_node,
                (*es).user_data,
            );
            if !interfaces.is_null() {
                has_properties_interface = FALSE as gboolean;
                has_introspectable_interface = FALSE as gboolean;
                n = 0 as guint;
                while !(*interfaces.offset(n as isize)).is_null() {
                    if strcmp(
                        (**interfaces.offset(n as isize)).name,
                        b"org.freedesktop.DBus.Properties\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        has_properties_interface = TRUE as gboolean;
                    } else if strcmp(
                        (**interfaces.offset(n as isize)).name,
                        b"org.freedesktop.DBus.Introspectable\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        has_introspectable_interface = TRUE as gboolean;
                    }
                    n = n.wrapping_add(1);
                }
                if has_properties_interface == 0 {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                &raw const safe_c2rust_introspect_properties_interface
                                    as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                s,
                                __val,
                                if ({
                                    let mut _g_boolean_var_212: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_212 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_212 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_212
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    strlen(
                                        __val
                                            .offset(__val.is_null() as ::core::ffi::c_int as isize),
                                    ) as gssize
                                } else {
                                    -(1 as ::core::ffi::c_int) as gssize
                                },
                            );
                        });
                    } else {
                        safe_c2rust_g_string_append_len_inline(
                            s,
                            &raw const safe_c2rust_introspect_properties_interface
                                as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                }
                if has_introspectable_interface == 0 {
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                &raw const safe_c2rust_introspect_introspectable_interface
                                    as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                s,
                                __val,
                                if ({
                                    let mut _g_boolean_var_213: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_213 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_213 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_213
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    strlen(
                                        __val
                                            .offset(__val.is_null() as ::core::ffi::c_int as isize),
                                    ) as gssize
                                } else {
                                    -(1 as ::core::ffi::c_int) as gssize
                                },
                            );
                        });
                    } else {
                        safe_c2rust_g_string_append_len_inline(
                            s,
                            &raw const safe_c2rust_introspect_introspectable_interface
                                as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                }
                n = 0 as guint;
                while !(*interfaces.offset(n as isize)).is_null() {
                    g_dbus_interface_info_generate_xml(
                        *interfaces.offset(n as isize),
                        2 as guint,
                        s,
                    );
                    g_dbus_interface_info_unref(*interfaces.offset(n as isize));
                    n = n.wrapping_add(1);
                }
                g_free(interfaces as gpointer);
            }
            if is_root != 0 {
                n = 0 as guint;
                while !children.is_null() && !(*children.offset(n as isize)).is_null() {
                    g_string_append_printf(
                        s,
                        b"  <node name=\"%s\"/>\n\0" as *const u8 as *const gchar,
                        *children.offset(n as isize),
                    );
                    n = n.wrapping_add(1);
                }
            }
            subnode_paths = safe_c2rust_g_dbus_connection_list_registered(
                (*es).connection,
                requested_object_path,
            );
            n = 0 as guint;
            while !subnode_paths.is_null() && !(*subnode_paths.offset(n as isize)).is_null() {
                g_string_append_printf(
                    s,
                    b"  <node name=\"%s\"/>\n\0" as *const u8 as *const gchar,
                    *subnode_paths.offset(n as isize),
                );
                n = n.wrapping_add(1);
            }
            g_strfreev(subnode_paths);
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b"</node>\n\0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        s,
                        __val,
                        if ({
                            let mut _g_boolean_var_214: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_214 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_214 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_214
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as ::core::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                safe_c2rust_g_string_append_len_inline(
                    s,
                    b"</node>\n\0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            reply = g_dbus_message_new_method_reply(message);
            g_dbus_message_set_body(
                reply,
                g_variant_new(b"(s)\0" as *const u8 as *const gchar, (*s).str_0),
            );
            safe_c2rust_g_dbus_connection_send_message(
                connection,
                reply,
                G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                ::core::ptr::null_mut::<guint32>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            g_object_unref(reply as gpointer);
            handled = TRUE as gboolean;
        }
        _ => {}
    }
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        } else {
            g_string_free_and_steal(s);
        };
    } else {
        g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
    };
    g_strfreev(children);
    return handled;
}
unsafe extern "C" fn safe_c2rust_handle_subtree_method_invocation(
    mut connection: *mut GDBusConnection,
    mut es: *mut ExportedSubtree,
    mut message: *mut GDBusMessage,
) -> gboolean {
    let mut current_block: u64;
    let mut handled: gboolean = 0;
    let mut sender: *const gchar = ::core::ptr::null::<gchar>();
    let mut interface_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut member: *const gchar = ::core::ptr::null::<gchar>();
    let mut signature: *const gchar = ::core::ptr::null::<gchar>();
    let mut requested_object_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut requested_node: *const gchar = ::core::ptr::null::<gchar>();
    let mut is_root: gboolean = 0;
    let mut interface_info: *mut GDBusInterfaceInfo = ::core::ptr::null_mut::<GDBusInterfaceInfo>();
    let mut interface_vtable: *const GDBusInterfaceVTable =
        ::core::ptr::null::<GDBusInterfaceVTable>();
    let mut interface_user_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut n: guint = 0;
    let mut interfaces: *mut *mut GDBusInterfaceInfo =
        ::core::ptr::null_mut::<*mut GDBusInterfaceInfo>();
    let mut is_property_get: gboolean = 0;
    let mut is_property_set: gboolean = 0;
    let mut is_property_get_all: gboolean = 0;
    handled = FALSE as gboolean;
    interfaces = ::core::ptr::null_mut::<*mut GDBusInterfaceInfo>();
    requested_object_path = g_dbus_message_get_path(message);
    sender = g_dbus_message_get_sender(message);
    interface_name = g_dbus_message_get_interface(message);
    member = g_dbus_message_get_member(message);
    signature = g_dbus_message_get_signature(message);
    is_root = (g_strcmp0(
        requested_object_path as *const ::core::ffi::c_char,
        (*es).object_path,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    is_property_get = FALSE as gboolean;
    is_property_set = FALSE as gboolean;
    is_property_get_all = FALSE as gboolean;
    if g_strcmp0(
        interface_name as *const ::core::ffi::c_char,
        b"org.freedesktop.DBus.Properties\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if g_strcmp0(
            member as *const ::core::ffi::c_char,
            b"Get\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && g_strcmp0(
                signature as *const ::core::ffi::c_char,
                b"ss\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            is_property_get = TRUE as gboolean;
        } else if g_strcmp0(
            member as *const ::core::ffi::c_char,
            b"Set\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && g_strcmp0(
                signature as *const ::core::ffi::c_char,
                b"ssv\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            is_property_set = TRUE as gboolean;
        } else if g_strcmp0(
            member as *const ::core::ffi::c_char,
            b"GetAll\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            && g_strcmp0(
                signature as *const ::core::ffi::c_char,
                b"s\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            is_property_get_all = TRUE as gboolean;
        }
    }
    if is_root == 0 {
        requested_node = strrchr(
            requested_object_path as *const ::core::ffi::c_char,
            '/' as i32,
        )
        .offset(1 as ::core::ffi::c_int as isize);
        if !((*es).flags as ::core::ffi::c_uint)
            & G_DBUS_SUBTREE_FLAGS_DISPATCH_TO_UNENUMERATED_NODES as ::core::ffi::c_int
                as ::core::ffi::c_uint
            != 0
        {
            let mut children: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
            let mut exists: gboolean = 0;
            children = (*(*es).vtable)
                .enumerate
                .expect("non-null function pointer")(
                (*es).connection,
                sender,
                (*es).object_path,
                (*es).user_data,
            );
            exists = g_strv_contains(children as *const *const gchar, requested_node);
            g_strfreev(children);
            if exists == 0 {
                current_block = 18211489521268282274;
            } else {
                current_block = 3437258052017859086;
            }
        } else {
            current_block = 3437258052017859086;
        }
    } else {
        requested_node = ::core::ptr::null::<gchar>();
        current_block = 3437258052017859086;
    }
    match current_block {
        3437258052017859086 => {
            interfaces = (*(*es).vtable)
                .introspect
                .expect("non-null function pointer")(
                (*es).connection,
                sender,
                requested_object_path,
                requested_node,
                (*es).user_data,
            );
            if !interfaces.is_null() {
                interface_info = ::core::ptr::null_mut::<GDBusInterfaceInfo>();
                n = 0 as guint;
                while !(*interfaces.offset(n as isize)).is_null() {
                    if g_strcmp0(
                        (**interfaces.offset(n as isize)).name,
                        interface_name as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        interface_info = *interfaces.offset(n as isize);
                    }
                    n = n.wrapping_add(1);
                }
                if !interface_info.is_null() {
                    interface_user_data = NULL_0 as gpointer;
                    interface_vtable = (*(*es).vtable).dispatch.expect("non-null function pointer")(
                        (*es).connection,
                        sender,
                        (*es).object_path,
                        interface_name,
                        requested_node,
                        &raw mut interface_user_data,
                        (*es).user_data,
                    );
                    if !interface_vtable.is_null() {
                        g_mutex_lock(&raw mut (*connection).lock);
                        handled = safe_c2rust_validate_and_maybe_schedule_method_call(
                            (*es).connection,
                            message,
                            0 as guint,
                            (*es).id,
                            interface_info,
                            interface_vtable,
                            (*es).context,
                            interface_user_data,
                        );
                        g_mutex_unlock(&raw mut (*connection).lock);
                    }
                } else if is_property_get != 0 || is_property_set != 0 || is_property_get_all != 0 {
                    if is_property_get != 0 {
                        g_variant_get(
                            g_dbus_message_get_body(message),
                            b"(&s&s)\0" as *const u8 as *const gchar,
                            &raw mut interface_name,
                            NULL_0,
                        );
                    } else if is_property_set != 0 {
                        g_variant_get(
                            g_dbus_message_get_body(message),
                            b"(&s&sv)\0" as *const u8 as *const gchar,
                            &raw mut interface_name,
                            NULL_0,
                            NULL_0,
                        );
                    } else if is_property_get_all != 0 {
                        g_variant_get(
                            g_dbus_message_get_body(message),
                            b"(&s)\0" as *const u8 as *const gchar,
                            &raw mut interface_name,
                            NULL_0,
                            NULL_0,
                        );
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            7186 as ::core::ffi::c_int,
                            G_STRFUNC,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                    n = 0 as guint;
                    while !(*interfaces.offset(n as isize)).is_null() {
                        if g_strcmp0(
                            (**interfaces.offset(n as isize)).name,
                            interface_name as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            interface_info = *interfaces.offset(n as isize);
                        }
                        n = n.wrapping_add(1);
                    }
                    if interface_info.is_null() {
                        let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
                        reply = g_dbus_message_new_method_error(
                            message,
                            b"org.freedesktop.DBus.Error.InvalidArgs\0" as *const u8
                                as *const gchar,
                            glib_gettext(
                                b"No such interface \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                                    as *const gchar,
                            ),
                            interface_name,
                        );
                        safe_c2rust_g_dbus_connection_send_message(
                            (*es).connection,
                            reply,
                            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                            ::core::ptr::null_mut::<guint32>(),
                            ::core::ptr::null_mut::<*mut GError>(),
                        );
                        g_object_unref(reply as gpointer);
                        handled = TRUE as gboolean;
                    } else {
                        interface_user_data = NULL_0 as gpointer;
                        interface_vtable =
                            (*(*es).vtable).dispatch.expect("non-null function pointer")(
                                (*es).connection,
                                sender,
                                (*es).object_path,
                                interface_name,
                                requested_node,
                                &raw mut interface_user_data,
                                (*es).user_data,
                            );
                        if interface_vtable.is_null() {
                            g_log(
                                G_LOG_DOMAIN.as_ptr() as *const gchar,
                                G_LOG_LEVEL_WARNING,
                                b"The subtree introspection function indicates that '%s' is a valid interface name, but calling the dispatch function on that interface gave us NULL\0"
                                    as *const u8 as *const gchar,
                                interface_name,
                            );
                        } else if is_property_get != 0 || is_property_set != 0 {
                            g_mutex_lock(&raw mut (*connection).lock);
                            handled = safe_c2rust_validate_and_maybe_schedule_property_getset(
                                (*es).connection,
                                message,
                                0 as guint,
                                (*es).id,
                                is_property_get,
                                interface_info,
                                interface_vtable,
                                (*es).context,
                                interface_user_data,
                            );
                            g_mutex_unlock(&raw mut (*connection).lock);
                        } else if is_property_get_all != 0 {
                            g_mutex_lock(&raw mut (*connection).lock);
                            handled = safe_c2rust_validate_and_maybe_schedule_property_get_all(
                                (*es).connection,
                                message,
                                0 as guint,
                                (*es).id,
                                interface_info,
                                interface_vtable,
                                (*es).context,
                                interface_user_data,
                            );
                            g_mutex_unlock(&raw mut (*connection).lock);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !interfaces.is_null() {
        n = 0 as guint;
        while !(*interfaces.offset(n as isize)).is_null() {
            g_dbus_interface_info_unref(*interfaces.offset(n as isize));
            n = n.wrapping_add(1);
        }
        g_free(interfaces as gpointer);
    }
    return handled;
}
unsafe extern "C" fn safe_c2rust_subtree_deferred_data_free(mut data: *mut SubtreeDeferredData) {
    let mut _pp: *mut *mut GDBusMessage = &raw mut (*data).message;
    let mut _ptr: *mut GDBusMessage = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusMessage>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    safe_c2rust_exported_subtree_unref((*data).es);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_process_subtree_vtable_message_in_idle_cb(
    mut _data: gpointer,
) -> gboolean {
    let mut data: *mut SubtreeDeferredData = _data as *mut SubtreeDeferredData;
    let mut handled: gboolean = 0;
    handled = FALSE as gboolean;
    if g_strcmp0(
        g_dbus_message_get_interface((*data).message) as *const ::core::ffi::c_char,
        b"org.freedesktop.DBus.Introspectable\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            g_dbus_message_get_member((*data).message) as *const ::core::ffi::c_char,
            b"Introspect\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            g_dbus_message_get_signature((*data).message) as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        handled = safe_c2rust_handle_subtree_introspect(
            (*(*data).es).connection,
            (*data).es,
            (*data).message,
        );
    } else {
        handled = safe_c2rust_handle_subtree_method_invocation(
            (*(*data).es).connection,
            (*data).es,
            (*data).message,
        );
    }
    if handled == 0 {
        g_mutex_lock(&raw mut (*(*(*data).es).connection).lock);
        handled = safe_c2rust_handle_generic_unlocked((*(*data).es).connection, (*data).message);
        g_mutex_unlock(&raw mut (*(*(*data).es).connection).lock);
    }
    if handled == 0 {
        let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
        reply = g_dbus_message_new_method_error(
            (*data).message,
            b"org.freedesktop.DBus.Error.UnknownMethod\0" as *const u8 as *const gchar,
            glib_gettext(
                b"Method \xE2\x80\x9C%s\xE2\x80\x9D on interface \xE2\x80\x9C%s\xE2\x80\x9D with signature \xE2\x80\x9C%s\xE2\x80\x9D does not exist\0"
                    as *const u8 as *const gchar,
            ),
            g_dbus_message_get_member((*data).message),
            g_dbus_message_get_interface((*data).message),
            g_dbus_message_get_signature((*data).message),
        );
        safe_c2rust_g_dbus_connection_send_message(
            (*(*data).es).connection,
            reply,
            G_DBUS_SEND_MESSAGE_FLAGS_NONE,
            ::core::ptr::null_mut::<guint32>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_object_unref(reply as gpointer);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_subtree_message_func(
    mut connection: *mut GDBusConnection,
    mut es: *mut ExportedSubtree,
    mut message: *mut GDBusMessage,
) -> gboolean {
    let mut idle_source: *mut GSource = ::core::ptr::null_mut::<GSource>();
    let mut data: *mut SubtreeDeferredData = ::core::ptr::null_mut::<SubtreeDeferredData>();
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<SubtreeDeferredData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut SubtreeDeferredData;
    (*data).message = g_object_ref(message as gpointer) as *mut GDBusMessage as *mut GDBusMessage;
    (*data).es = safe_c2rust_exported_subtree_ref(es);
    idle_source = g_idle_source_new();
    g_source_set_priority(idle_source, G_PRIORITY_HIGH);
    g_source_set_callback(
        idle_source,
        Some(
            safe_c2rust_process_subtree_vtable_message_in_idle_cb
                as unsafe extern "C" fn(gpointer) -> gboolean,
        ),
        data as gpointer,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut SubtreeDeferredData) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_subtree_deferred_data_free
                as unsafe extern "C" fn(*mut SubtreeDeferredData) -> (),
        )),
    );
    g_source_set_static_name(
        idle_source,
        b"[gio] process_subtree_vtable_message_in_idle_cb\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    g_source_attach(idle_source, (*es).context);
    g_source_unref(idle_source);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_register_subtree(
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
    mut vtable: *const GDBusSubtreeVTable,
    mut flags: GDBusSubtreeFlags,
    mut user_data: gpointer,
    mut user_data_free_func: GDestroyNotify,
    mut error: *mut *mut GError,
) -> guint {
    let mut ret: guint = 0;
    let mut es: *mut ExportedSubtree = ::core::ptr::null_mut::<ExportedSubtree>();
    if ({
        let mut _g_boolean_var_215: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_215 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_215 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_215
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_216: ::core::ffi::c_int = 0;
        if !object_path.is_null() && g_variant_is_object_path(object_path) != 0 {
            _g_boolean_var_216 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_216 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_216
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"object_path != NULL && g_variant_is_object_path (object_path)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_217: ::core::ffi::c_int = 0;
        if !vtable.is_null() {
            _g_boolean_var_217 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_217 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_217
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"vtable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_218: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_218 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_218 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_218
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_219: ::core::ffi::c_int = 0;
        if safe_c2rust_check_initialized(connection) != 0 {
            _g_boolean_var_219 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_219 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_219
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    ret = 0 as guint;
    g_mutex_lock(&raw mut (*connection).lock);
    es = g_hash_table_lookup(
        (*connection).map_object_path_to_es,
        object_path as gconstpointer,
    ) as *mut ExportedSubtree;
    if !es.is_null() {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_EXISTS as ::core::ffi::c_int as gint,
            glib_gettext(b"A subtree is already exported for %s\0" as *const u8 as *const gchar),
            object_path,
        );
    } else {
        es = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<ExportedSubtree>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut ExportedSubtree;
        (*es).refcount = 1 as ::core::ffi::c_int as gint;
        (*es).object_path =
            safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as *mut gchar;
        (*es).connection = connection;
        (*es).vtable = safe_c2rust__g_dbus_subtree_vtable_copy(vtable);
        (*es).flags = flags;
        (*es).id = ({
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust__global_subtree_registration_id;
            } else {
            };
            crate::translated::compat::atomic_xadd_seqcst(
                &raw mut safe_c2rust__global_subtree_registration_id,
                1 as ::core::ffi::c_int as guint,
            ) as gint
        }) as guint;
        (*es).user_data = user_data;
        (*es).user_data_free_func = user_data_free_func;
        (*es).context = g_main_context_ref_thread_default();
        g_hash_table_insert(
            (*connection).map_object_path_to_es,
            (*es).object_path as gpointer,
            es as gpointer,
        );
        g_hash_table_insert(
            (*connection).map_id_to_es,
            (*es).id as gulong as gpointer,
            es as gpointer,
        );
        ret = (*es).id;
    }
    g_mutex_unlock(&raw mut (*connection).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_unregister_subtree(
    mut connection: *mut GDBusConnection,
    mut registration_id: guint,
) -> gboolean {
    let mut es: *mut ExportedSubtree = ::core::ptr::null_mut::<ExportedSubtree>();
    let mut ret: gboolean = 0;
    if ({
        let mut _g_boolean_var_220: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = connection as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_connection_get_type();
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
            _g_boolean_var_220 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_220 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_220
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
        let mut _g_boolean_var_221: ::core::ffi::c_int = 0;
        if safe_c2rust_check_initialized(connection) != 0 {
            _g_boolean_var_221 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_221 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_221
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"check_initialized (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    g_mutex_lock(&raw mut (*connection).lock);
    es = g_hash_table_lookup(
        (*connection).map_id_to_es,
        registration_id as gulong as gpointer as gconstpointer,
    ) as *mut ExportedSubtree;
    if !es.is_null() {
        if !(({
            let mut _g_boolean_var_222: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*connection).map_id_to_es,
                (*es).id as gulong as gpointer as gconstpointer,
            ) != 0
            {
                _g_boolean_var_222 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_222 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_222
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                7499 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (connection->map_id_to_es, GUINT_TO_POINTER (es->id))\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !(({
            let mut _g_boolean_var_223: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*connection).map_object_path_to_es,
                (*es).object_path as gconstpointer,
            ) != 0
            {
                _g_boolean_var_223 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_223 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_223
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                7500 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (connection->map_object_path_to_es, es->object_path)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        ret = TRUE as gboolean;
    }
    g_mutex_unlock(&raw mut (*connection).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_handle_generic_ping_unlocked(
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
    mut message: *mut GDBusMessage,
) {
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    reply = g_dbus_message_new_method_reply(message);
    safe_c2rust_g_dbus_connection_send_message_unlocked(
        connection,
        reply,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        ::core::ptr::null_mut::<guint32>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_object_unref(reply as gpointer);
}
unsafe extern "C" fn safe_c2rust_handle_generic_get_machine_id_unlocked(
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
    mut message: *mut GDBusMessage,
) {
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    reply = ::core::ptr::null_mut::<GDBusMessage>();
    if (*connection).machine_id.is_null() {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        error = ::core::ptr::null_mut::<GError>();
        (*connection).machine_id = _g_dbus_get_machine_id(&raw mut error);
        if (*connection).machine_id.is_null() {
            reply = g_dbus_message_new_method_error_literal(
                message,
                b"org.freedesktop.DBus.Error.Failed\0" as *const u8 as *const gchar,
                (*error).message,
            );
            g_error_free(error);
        }
    }
    if reply.is_null() {
        reply = g_dbus_message_new_method_reply(message);
        g_dbus_message_set_body(
            reply,
            g_variant_new(
                b"(s)\0" as *const u8 as *const gchar,
                (*connection).machine_id,
            ),
        );
    }
    safe_c2rust_g_dbus_connection_send_message_unlocked(
        connection,
        reply,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        ::core::ptr::null_mut::<guint32>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_object_unref(reply as gpointer);
}
unsafe extern "C" fn safe_c2rust_handle_generic_introspect_unlocked(
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
    mut message: *mut GDBusMessage,
) {
    let mut n: guint = 0;
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut registered: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    s = g_string_new(::core::ptr::null::<gchar>());
    safe_c2rust_introspect_append_header(s);
    registered = safe_c2rust_g_dbus_connection_list_registered_unlocked(connection, object_path);
    n = 0 as guint;
    while !registered.is_null() && !(*registered.offset(n as isize)).is_null() {
        g_string_append_printf(
            s,
            b"  <node name=\"%s\"/>\n\0" as *const u8 as *const gchar,
            *registered.offset(n as isize),
        );
        n = n.wrapping_add(1);
    }
    g_strfreev(registered);
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"</node>\n\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                s,
                __val,
                if ({
                    let mut _g_boolean_var_224: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_224 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_224 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_224
                }) as ::core::ffi::c_long
                    != 0
                {
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            s,
            b"</node>\n\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    reply = g_dbus_message_new_method_reply(message);
    g_dbus_message_set_body(
        reply,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, (*s).str_0),
    );
    safe_c2rust_g_dbus_connection_send_message_unlocked(
        connection,
        reply,
        G_DBUS_SEND_MESSAGE_FLAGS_NONE,
        ::core::ptr::null_mut::<guint32>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_object_unref(reply as gpointer);
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        } else {
            g_string_free_and_steal(s);
        };
    } else {
        g_string_free(s, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn safe_c2rust_handle_generic_unlocked(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
) -> gboolean {
    let mut handled: gboolean = 0;
    let mut interface_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut member: *const gchar = ::core::ptr::null::<gchar>();
    let mut signature: *const gchar = ::core::ptr::null::<gchar>();
    let mut path: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_225: ::core::ffi::c_int = 0;
        if g_mutex_trylock(&raw mut (*connection).lock) != 0 {
            _g_boolean_var_225 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_225 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_225
    }) as ::core::ffi::c_long
        != 0
    {
        g_assertion_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            7596 as ::core::ffi::c_int,
            G_STRFUNC,
            b"CONNECTION_ENSURE_LOCK: GDBusConnection object lock is not locked\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    handled = FALSE as gboolean;
    interface_name = g_dbus_message_get_interface(message);
    member = g_dbus_message_get_member(message);
    signature = g_dbus_message_get_signature(message);
    path = g_dbus_message_get_path(message);
    if g_strcmp0(
        interface_name as *const ::core::ffi::c_char,
        b"org.freedesktop.DBus.Introspectable\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            member as *const ::core::ffi::c_char,
            b"Introspect\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            signature as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        safe_c2rust_handle_generic_introspect_unlocked(connection, path, message);
        handled = TRUE as gboolean;
    } else if g_strcmp0(
        interface_name as *const ::core::ffi::c_char,
        b"org.freedesktop.DBus.Peer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            member as *const ::core::ffi::c_char,
            b"Ping\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            signature as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        safe_c2rust_handle_generic_ping_unlocked(connection, path, message);
        handled = TRUE as gboolean;
    } else if g_strcmp0(
        interface_name as *const ::core::ffi::c_char,
        b"org.freedesktop.DBus.Peer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            member as *const ::core::ffi::c_char,
            b"GetMachineId\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            signature as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        safe_c2rust_handle_generic_get_machine_id_unlocked(connection, path, message);
        handled = TRUE as gboolean;
    }
    return handled;
}
unsafe extern "C" fn safe_c2rust_distribute_method_call(
    mut connection: *mut GDBusConnection,
    mut message: *mut GDBusMessage,
) {
    let mut current_block: u64;
    let mut reply: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut eo: *mut ExportedObject = ::core::ptr::null_mut::<ExportedObject>();
    let mut es: *mut ExportedSubtree = ::core::ptr::null_mut::<ExportedSubtree>();
    let mut path: *const gchar = ::core::ptr::null::<gchar>();
    let mut interface_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut member: *const gchar = ::core::ptr::null::<gchar>();
    let mut subtree_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut needle: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut object_found: gboolean = FALSE;
    if ({
        let mut _g_boolean_var_226: ::core::ffi::c_int = 0;
        if g_dbus_message_get_message_type(message) as ::core::ffi::c_uint
            == G_DBUS_MESSAGE_TYPE_METHOD_CALL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _g_boolean_var_226 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_226 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_226
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            7647 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_dbus_message_get_message_type (message) == G_DBUS_MESSAGE_TYPE_METHOD_CALL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    member = g_dbus_message_get_member(message);
    path = g_dbus_message_get_path(message);
    if ({
        let mut _g_boolean_var_227: ::core::ffi::c_int = 0;
        if !member.is_null() {
            _g_boolean_var_227 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_227 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_227
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            7654 as ::core::ffi::c_int,
            G_STRFUNC,
            b"member != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_228: ::core::ffi::c_int = 0;
        if !path.is_null() {
            _g_boolean_var_228 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_228 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_228
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            7655 as ::core::ffi::c_int,
            G_STRFUNC,
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    interface_name = g_dbus_message_get_interface(message);
    subtree_path = safe_c2rust_g_strdup_inline(path as *const ::core::ffi::c_char) as *mut gchar;
    needle = strrchr(subtree_path, '/' as i32) as *mut gchar;
    if !needle.is_null() && needle != subtree_path {
        *needle = '\0' as i32 as gchar;
    } else {
        g_free(subtree_path as gpointer);
        subtree_path = ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_229: ::core::ffi::c_int = 0;
        if _g_dbus_debug_incoming() != 0 {
            _g_boolean_var_229 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_229 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_229
    }) as ::core::ffi::c_long
        != 0
    {
        _g_dbus_debug_print_lock();
        g_print(
            b"========================================================================\nGDBus-debug:Incoming:\n <<<< METHOD INVOCATION %s.%s()\n      on object %s\n      invoked by name %s\n      serial %d\n\0"
                as *const u8 as *const gchar,
            interface_name,
            member,
            path,
            if !g_dbus_message_get_sender(message).is_null() {
                g_dbus_message_get_sender(message) as *const ::core::ffi::c_char
            } else {
                b"(none)\0" as *const u8 as *const ::core::ffi::c_char
            },
            g_dbus_message_get_serial(message),
        );
        _g_dbus_debug_print_unlock();
    }
    eo = g_hash_table_lookup((*connection).map_object_path_to_eo, path as gconstpointer)
        as *mut ExportedObject;
    if !eo.is_null() {
        if safe_c2rust_obj_message_func(connection, eo, message, &raw mut object_found) != 0 {
            current_block = 2797465446588703679;
        } else {
            current_block = 11932355480408055363;
        }
    } else {
        current_block = 11932355480408055363;
    }
    match current_block {
        11932355480408055363 => {
            es = g_hash_table_lookup((*connection).map_object_path_to_es, path as gconstpointer)
                as *mut ExportedSubtree;
            if !es.is_null() {
                if safe_c2rust_subtree_message_func(connection, es, message) != 0 {
                    current_block = 2797465446588703679;
                } else {
                    current_block = 17500079516916021833;
                }
            } else {
                current_block = 17500079516916021833;
            }
            match current_block {
                2797465446588703679 => {}
                _ => {
                    if !subtree_path.is_null() {
                        es = g_hash_table_lookup(
                            (*connection).map_object_path_to_es,
                            subtree_path as gconstpointer,
                        ) as *mut ExportedSubtree;
                        if !es.is_null() {
                            if safe_c2rust_subtree_message_func(connection, es, message) != 0 {
                                current_block = 2797465446588703679;
                            } else {
                                current_block = 15597372965620363352;
                            }
                        } else {
                            current_block = 15597372965620363352;
                        }
                    } else {
                        current_block = 15597372965620363352;
                    }
                    match current_block {
                        2797465446588703679 => {}
                        _ => {
                            if !(safe_c2rust_handle_generic_unlocked(connection, message) != 0) {
                                if object_found == TRUE {
                                    reply = g_dbus_message_new_method_error(
                                        message,
                                        b"org.freedesktop.DBus.Error.UnknownMethod\0" as *const u8
                                            as *const gchar,
                                        glib_gettext(
                                            b"No such interface \xE2\x80\x9C%s\xE2\x80\x9D on object at path %s\0"
                                                as *const u8 as *const gchar,
                                        ),
                                        interface_name,
                                        path,
                                    );
                                } else {
                                    reply = g_dbus_message_new_method_error(
                                        message,
                                        b"org.freedesktop.DBus.Error.UnknownMethod\0" as *const u8
                                            as *const gchar,
                                        glib_gettext(
                                            b"Object does not exist at path \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                                as *const u8 as *const gchar,
                                        ),
                                        path,
                                    );
                                }
                                safe_c2rust_g_dbus_connection_send_message_unlocked(
                                    connection,
                                    reply,
                                    G_DBUS_SEND_MESSAGE_FLAGS_NONE,
                                    ::core::ptr::null_mut::<guint32>(),
                                    ::core::ptr::null_mut::<*mut GError>(),
                                );
                                g_object_unref(reply as gpointer);
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    g_free(subtree_path as gpointer);
}
unsafe extern "C" fn safe_c2rust_message_bus_get_singleton(
    mut bus_type: GBusType,
    mut error: *mut *mut GError,
) -> *mut GWeakRef {
    let mut ret: *mut GWeakRef = ::core::ptr::null_mut::<GWeakRef>();
    let mut starter_bus: *const gchar = ::core::ptr::null::<gchar>();
    ret = ::core::ptr::null_mut::<GWeakRef>();
    match bus_type as ::core::ffi::c_int {
        2 => {
            ret = &raw mut safe_c2rust_the_session_bus;
        }
        1 => {
            ret = &raw mut safe_c2rust_the_system_bus;
        }
        -1 => {
            starter_bus = g_getenv(b"DBUS_STARTER_BUS_TYPE\0" as *const u8 as *const gchar);
            if g_strcmp0(
                starter_bus as *const ::core::ffi::c_char,
                b"session\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                ret = safe_c2rust_message_bus_get_singleton(G_BUS_TYPE_SESSION, error);
            } else if g_strcmp0(
                starter_bus as *const ::core::ffi::c_char,
                b"system\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                ret = safe_c2rust_message_bus_get_singleton(G_BUS_TYPE_SYSTEM, error);
            } else if !starter_bus.is_null() {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Cannot determine bus address from DBUS_STARTER_BUS_TYPE environment variable \xE2\x80\x94 unknown value \xE2\x80\x9C%s\xE2\x80\x9D\0"
                            as *const u8 as *const gchar,
                    ),
                    starter_bus,
                );
            } else {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Cannot determine bus address because the DBUS_STARTER_BUS_TYPE environment variable is not set\0"
                            as *const u8 as *const gchar,
                    ),
                );
            }
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                7796 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_get_uninitialized_connection(
    mut bus_type: GBusType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusConnection {
    let mut current_block: u64;
    let mut singleton: *mut GWeakRef = ::core::ptr::null_mut::<GWeakRef>();
    let mut ret: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    ret = ::core::ptr::null_mut::<GDBusConnection>();
    g_mutex_lock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    singleton = safe_c2rust_message_bus_get_singleton(bus_type, error);
    if !singleton.is_null() {
        ret = g_weak_ref_get(singleton) as *mut GDBusConnection;
        if ret.is_null() {
            let mut address: *mut gchar = ::core::ptr::null_mut::<gchar>();
            address = g_dbus_address_get_for_bus_sync(bus_type, cancellable, error);
            if address.is_null() {
                current_block = 14711672155730489944;
            } else {
                ret = g_object_new(
                    safe_c2rust_g_dbus_connection_get_type(),
                    b"address\0" as *const u8 as *const gchar,
                    address,
                    b"flags\0" as *const u8 as *const ::core::ffi::c_char,
                    G_DBUS_CONNECTION_FLAGS_AUTHENTICATION_CLIENT as ::core::ffi::c_int
                        | G_DBUS_CONNECTION_FLAGS_CROSS_NAMESPACE as ::core::ffi::c_int
                        | G_DBUS_CONNECTION_FLAGS_MESSAGE_BUS_CONNECTION as ::core::ffi::c_int,
                    b"exit-on-close\0" as *const u8 as *const ::core::ffi::c_char,
                    TRUE,
                    NULL_0,
                ) as *mut GDBusConnection;
                g_weak_ref_set(singleton, ret as gpointer);
                g_free(address as gpointer);
                current_block = 3276175668257526147;
            }
        } else {
            current_block = 3276175668257526147;
        }
        match current_block {
            14711672155730489944 => {}
            _ => {
                if ({
                    let mut _g_boolean_var_230: ::core::ffi::c_int = 0;
                    if !ret.is_null() {
                        _g_boolean_var_230 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_230 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_230
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        7842 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"ret != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
        }
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_bus_get_singleton_if_exists(
    mut bus_type: GBusType,
) -> *mut GDBusConnection {
    let mut singleton: *mut GWeakRef = ::core::ptr::null_mut::<GWeakRef>();
    let mut ret: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    g_mutex_lock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    singleton =
        safe_c2rust_message_bus_get_singleton(bus_type, ::core::ptr::null_mut::<*mut GError>());
    if !singleton.is_null() {
        ret = g_weak_ref_get(singleton) as *mut GDBusConnection;
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_bus_forget_singleton(mut bus_type: GBusType) {
    let mut singleton: *mut GWeakRef = ::core::ptr::null_mut::<GWeakRef>();
    g_mutex_lock(&raw mut safe_c2rust_g__message_bus_lock_lock);
    singleton =
        safe_c2rust_message_bus_get_singleton(bus_type, ::core::ptr::null_mut::<*mut GError>());
    if !singleton.is_null() {
        g_weak_ref_set(singleton, NULL_0);
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__message_bus_lock_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_get_sync(
    mut bus_type: GBusType,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GDBusConnection {
    let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    _g_dbus_initialize();
    if ({
        let mut _g_boolean_var_231: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_231 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_231 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_231
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    connection = safe_c2rust_get_uninitialized_connection(bus_type, cancellable, error);
    if !connection.is_null() {
        if g_initable_init(
            connection as *mut ::core::ffi::c_void as *mut GInitable,
            cancellable,
            error,
        ) == 0
        {
            g_object_unref(connection as gpointer);
            connection = ::core::ptr::null_mut::<GDBusConnection>();
        }
    }
    return connection;
}
unsafe extern "C" fn safe_c2rust_bus_get_async_initable_cb(
    mut source_object: *mut GObject,
    mut res: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut task: *mut GTask = user_data as *mut GTask;
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    if g_async_initable_init_finish(
        source_object as *mut ::core::ffi::c_void as *mut GAsyncInitable,
        res,
        &raw mut error,
    ) == 0
    {
        if ({
            let mut _g_boolean_var_232: ::core::ffi::c_int = 0;
            if !error.is_null() {
                _g_boolean_var_232 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_232 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_232
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                7951 as ::core::ffi::c_int,
                G_STRFUNC,
                b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_task_return_error(task, error);
        g_object_unref(source_object as gpointer);
    } else {
        g_task_return_pointer(
            task,
            source_object as gpointer,
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    g_object_unref(task as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_get(
    mut bus_type: GBusType,
    mut cancellable: *mut GCancellable,
    mut callback: GAsyncReadyCallback,
    mut user_data: gpointer,
) {
    let mut connection: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    let mut task: *mut GTask = ::core::ptr::null_mut::<GTask>();
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    _g_dbus_initialize();
    task = g_task_new(NULL_0, cancellable, callback, user_data);
    let mut _task: *mut GTask = task;
    g_task_set_source_tag(
        _task,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    GBusType,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >(Some(
            safe_c2rust_g_bus_get
                as unsafe extern "C" fn(
                    GBusType,
                    *mut GCancellable,
                    GAsyncReadyCallback,
                    gpointer,
                ) -> (),
        )),
    );
    if g_task_get_name(_task).is_null() {
        g_task_set_static_name(_task, b"g_bus_get\0" as *const u8 as *const gchar);
    }
    connection = safe_c2rust_get_uninitialized_connection(bus_type, cancellable, &raw mut error);
    if connection.is_null() {
        if ({
            let mut _g_boolean_var_233: ::core::ffi::c_int = 0;
            if !error.is_null() {
                _g_boolean_var_233 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_233 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_233
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusconnection.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                7997 as ::core::ffi::c_int,
                G_STRFUNC,
                b"error != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_task_return_error(task, error);
        g_object_unref(task as gpointer);
    } else {
        g_async_initable_init_async(
            connection as *mut ::core::ffi::c_void as *mut GAsyncInitable,
            G_PRIORITY_DEFAULT,
            cancellable,
            Some(
                safe_c2rust_bus_get_async_initable_cb
                    as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
            ),
            task as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bus_get_finish(
    mut res: *mut GAsyncResult,
    mut error: *mut *mut GError,
) -> *mut GDBusConnection {
    if ({
        let mut _g_boolean_var_234: ::core::ffi::c_int = 0;
        if g_task_is_valid(
            res as gpointer,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ) != 0
        {
            _g_boolean_var_234 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_234 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_234
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_task_is_valid (res, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    if ({
        let mut _g_boolean_var_235: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_235 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_235 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_235
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    return g_task_propagate_pointer(res as *mut ::core::ffi::c_void as *mut GTask, error)
        as *mut GDBusConnection;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
