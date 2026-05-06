use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GOptionContext;
    pub type _GOptionGroup;
    pub type _GCancellable;
    pub type _GSimpleActionGroupPrivate;
    pub type _GRemoteActionGroup;
    pub type _GActionMap;
    pub type _GActionGroup;
    pub type _GAction;
    pub type _GDBusConnection;
    pub type _GApplicationImpl;
    pub type _GApplicationCommandLinePrivate;
    pub type _GNotification;
    pub type _GFile;
    pub type _GWakeup;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_get_prgname() -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_get_environ() -> *mut *mut gchar;
    fn g_get_current_dir() -> *mut gchar;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_main_context_default() -> *mut GMainContext;
    fn g_main_context_iteration(context: *mut GMainContext, may_block: gboolean) -> gboolean;
    fn g_main_context_acquire(context: *mut GMainContext) -> gboolean;
    fn g_main_context_release(context: *mut GMainContext);
    fn g_source_remove(tag: guint) -> gboolean;
    fn g_timeout_add(interval: guint, function: GSourceFunc, data: gpointer) -> guint;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_new_boolean(value: gboolean) -> *mut GVariant;
    fn g_variant_new_int32(value: gint32) -> *mut GVariant;
    fn g_variant_new_int64(value: gint64) -> *mut GVariant;
    fn g_variant_new_double(value: gdouble) -> *mut GVariant;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_strv(strv: *const *const gchar, length: gssize) -> *mut GVariant;
    fn g_variant_new_bytestring(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_bytestring_array(strv: *const *const gchar, length: gssize) -> *mut GVariant;
    fn g_variant_builder_new(type_0: *const GVariantType) -> *mut GVariantBuilder;
    fn g_variant_builder_unref(builder: *mut GVariantBuilder);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_dict_new(from_asv: *mut GVariant) -> *mut GVariantDict;
    fn g_variant_dict_insert_value(
        dict: *mut GVariantDict,
        key: *const gchar,
        value: *mut GVariant,
    );
    fn g_variant_dict_end(dict: *mut GVariantDict) -> *mut GVariant;
    fn g_variant_dict_unref(dict: *mut GVariantDict);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_print(format: *const gchar, ...);
    fn g_printerr(format: *const gchar, ...);
    fn g_option_context_new(parameter_string: *const gchar) -> *mut GOptionContext;
    fn g_option_context_set_summary(context: *mut GOptionContext, summary: *const gchar);
    fn g_option_context_set_description(context: *mut GOptionContext, description: *const gchar);
    fn g_option_context_free(context: *mut GOptionContext);
    fn g_option_context_set_help_enabled(context: *mut GOptionContext, help_enabled: gboolean);
    fn g_option_context_set_ignore_unknown_options(
        context: *mut GOptionContext,
        ignore_unknown: gboolean,
    );
    fn g_option_context_parse_strv(
        context: *mut GOptionContext,
        arguments: *mut *mut *mut gchar,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_option_context_add_group(context: *mut GOptionContext, group: *mut GOptionGroup);
    fn g_option_context_set_main_group(context: *mut GOptionContext, group: *mut GOptionGroup);
    fn g_option_group_new(
        name: *const gchar,
        description: *const gchar,
        help_description: *const gchar,
        user_data: gpointer,
        destroy: GDestroyNotify,
    ) -> *mut GOptionGroup;
    fn g_option_group_unref(group: *mut GOptionGroup);
    fn g_option_group_add_entries(group: *mut GOptionGroup, entries: *const GOptionEntry);
    fn g_option_group_set_translation_domain(group: *mut GOptionGroup, domain: *const gchar);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_copy(block_size: gsize, mem_block: gconstpointer) -> gpointer;
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
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_cclosure_new(
        callback_func: GCallback,
        user_data: gpointer,
        destroy_data: GClosureNotify,
    ) -> *mut GClosure;
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
    fn g_signal_lookup(name: *const gchar, itype: GType) -> guint;
    fn g_signal_has_handler_pending(
        instance: gpointer,
        signal_id: guint,
        detail: GQuark,
        may_be_blocked: gboolean,
    ) -> gboolean;
    fn g_signal_connect_closure_by_id(
        instance: gpointer,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        after: gboolean,
    ) -> gulong;
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_signal_handler_find(
        instance: gpointer,
        mask: GSignalMatchType,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        func: gpointer,
        data: gpointer,
    ) -> gulong;
    fn g_signal_accumulator_true_handled(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
    ) -> gboolean;
    fn g_signal_accumulator_first_wins(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
    ) -> gboolean;
    fn g_variant_dict_get_type() -> GType;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_class_find_property(
        oclass: *mut GObjectClass,
        property_name: *const gchar,
    ) -> *mut GParamSpec;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_get(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_param_spec_uint(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        minimum: guint,
        maximum: guint,
        default_value: guint,
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
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_get_int(value: *const GValue) -> gint;
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_get_uint(value: *const GValue) -> guint;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_application_command_line_get_type() -> GType;
    fn g_application_command_line_get_exit_status(
        cmdline: *mut GApplicationCommandLine,
    ) -> ::core::ffi::c_int;
    fn g_application_command_line_set_exit_status(
        cmdline: *mut GApplicationCommandLine,
        exit_status: ::core::ffi::c_int,
    );
    fn g_action_group_get_type() -> GType;
    fn g_action_group_list_actions(action_group: *mut GActionGroup) -> *mut *mut gchar;
    fn g_action_group_change_action_state(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
        value: *mut GVariant,
    );
    fn g_action_group_activate_action(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
        parameter: *mut GVariant,
    );
    fn g_action_group_action_added(action_group: *mut GActionGroup, action_name: *const gchar);
    fn g_action_group_action_removed(action_group: *mut GActionGroup, action_name: *const gchar);
    fn g_action_group_action_enabled_changed(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
        enabled: gboolean,
    );
    fn g_action_group_action_state_changed(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
        state: *mut GVariant,
    );
    fn g_action_group_query_action(
        action_group: *mut GActionGroup,
        action_name: *const gchar,
        enabled: *mut gboolean,
        parameter_type: *mut *const GVariantType,
        state_type: *mut *const GVariantType,
        state_hint: *mut *mut GVariant,
        state: *mut *mut GVariant,
    ) -> gboolean;
    fn g_action_map_get_type() -> GType;
    fn g_action_map_lookup_action(
        action_map: *mut GActionMap,
        action_name: *const gchar,
    ) -> *mut GAction;
    fn g_action_map_add_action(action_map: *mut GActionMap, action: *mut GAction);
    fn g_action_map_remove_action(action_map: *mut GActionMap, action_name: *const gchar);
    fn g_simple_action_group_get_type() -> GType;
    fn g_remote_action_group_get_type() -> GType;
    fn g_remote_action_group_activate_action_full(
        remote: *mut GRemoteActionGroup,
        action_name: *const gchar,
        parameter: *mut GVariant,
        platform_data: *mut GVariant,
    );
    fn g_remote_action_group_change_action_state_full(
        remote: *mut GRemoteActionGroup,
        action_name: *const gchar,
        value: *mut GVariant,
        platform_data: *mut GVariant,
    );
    fn g_application_impl_destroy(impl_0: *mut GApplicationImpl);
    fn g_application_impl_register(
        application: *mut GApplication,
        appid: *const gchar,
        flags: GApplicationFlags,
        exported_actions: *mut GActionGroup,
        remote_actions: *mut *mut GRemoteActionGroup,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GApplicationImpl;
    fn g_application_impl_activate(impl_0: *mut GApplicationImpl, platform_data: *mut GVariant);
    fn g_application_impl_open(
        impl_0: *mut GApplicationImpl,
        files: *mut *mut GFile,
        n_files: gint,
        hint: *const gchar,
        platform_data: *mut GVariant,
    );
    fn g_application_impl_command_line(
        impl_0: *mut GApplicationImpl,
        arguments: *const *const gchar,
        platform_data: *mut GVariant,
    ) -> ::core::ffi::c_int;
    fn g_application_impl_flush(impl_0: *mut GApplicationImpl);
    fn g_application_impl_get_dbus_connection(
        impl_0: *mut GApplicationImpl,
    ) -> *mut GDBusConnection;
    fn g_application_impl_get_dbus_object_path(impl_0: *mut GApplicationImpl) -> *const gchar;
    fn g_application_impl_set_busy_state(impl_0: *mut GApplicationImpl, busy: gboolean);
    fn g_settings_sync();
    fn g_notification_get_type() -> GType;
    fn g_notification_backend_new_default(
        application: *mut GApplication,
    ) -> *mut GNotificationBackend;
    fn g_notification_backend_send_notification(
        backend: *mut GNotificationBackend,
        id: *const gchar,
        notification: *mut GNotification,
    );
    fn g_notification_backend_withdraw_notification(
        backend: *mut GNotificationBackend,
        id: *const gchar,
    );
    fn g_dbus_generate_guid() -> *mut gchar;
    fn g_dbus_is_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_unique_name(string: *const gchar) -> gboolean;
    fn g_application_flags_get_type() -> GType;
    fn g_file_new_for_commandline_arg(arg: *const ::core::ffi::c_char) -> *mut GFile;
    fn glib__private__() -> *const GLibPrivateVTable;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn _g_cclosure_marshal_BOOLEAN__VOID(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_BOOLEAN__VOIDv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_cclosure_marshal_INT__BOXED(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_INT__BOXEDv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_cclosure_marshal_INT__OBJECT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_INT__OBJECTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_cclosure_marshal_VOID__POINTER_INT_STRING(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_VOID__POINTER_INT_STRINGv(
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
pub type gint32 = ::core::ffi::c_int;
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
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
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
pub type GDir = _GDir;
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
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantDict {
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: C2RustUnnamed_2,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub asv: *mut GVariant,
    pub partial_magic: gsize,
    pub y: [guintptr; 14],
}
pub type GVariantDict = _GVariantDict;
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
pub type GOptionContext = _GOptionContext;
pub type GOptionGroup = _GOptionGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOptionEntry {
    pub long_name: *const gchar,
    pub short_name: gchar,
    pub flags: gint,
    pub arg: GOptionArg,
    pub arg_data: gpointer,
    pub description: *const gchar,
    pub arg_description: *const gchar,
}
pub type GOptionArg = ::core::ffi::c_uint;
pub const G_OPTION_ARG_INT64: GOptionArg = 8;
pub const G_OPTION_ARG_DOUBLE: GOptionArg = 7;
pub const G_OPTION_ARG_FILENAME_ARRAY: GOptionArg = 6;
pub const G_OPTION_ARG_STRING_ARRAY: GOptionArg = 5;
pub const G_OPTION_ARG_FILENAME: GOptionArg = 4;
pub const G_OPTION_ARG_CALLBACK: GOptionArg = 3;
pub const G_OPTION_ARG_INT: GOptionArg = 2;
pub const G_OPTION_ARG_STRING: GOptionArg = 1;
pub const G_OPTION_ARG_NONE: GOptionArg = 0;
pub type GOptionEntry = _GOptionEntry;
pub type GOptionFlags = ::core::ffi::c_uint;
pub const G_OPTION_FLAG_NOALIAS: GOptionFlags = 64;
pub const G_OPTION_FLAG_OPTIONAL_ARG: GOptionFlags = 32;
pub const G_OPTION_FLAG_FILENAME: GOptionFlags = 16;
pub const G_OPTION_FLAG_NO_ARG: GOptionFlags = 8;
pub const G_OPTION_FLAG_REVERSE: GOptionFlags = 4;
pub const G_OPTION_FLAG_IN_MAIN: GOptionFlags = 2;
pub const G_OPTION_FLAG_HIDDEN: GOptionFlags = 1;
pub const G_OPTION_FLAG_NONE: GOptionFlags = 0;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed_3; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub type GApplicationFlags = ::core::ffi::c_uint;
pub const G_APPLICATION_REPLACE: GApplicationFlags = 256;
pub const G_APPLICATION_ALLOW_REPLACEMENT: GApplicationFlags = 128;
pub const G_APPLICATION_CAN_OVERRIDE_APP_ID: GApplicationFlags = 64;
pub const G_APPLICATION_NON_UNIQUE: GApplicationFlags = 32;
pub const G_APPLICATION_SEND_ENVIRONMENT: GApplicationFlags = 16;
pub const G_APPLICATION_HANDLES_COMMAND_LINE: GApplicationFlags = 8;
pub const G_APPLICATION_HANDLES_OPEN: GApplicationFlags = 4;
pub const G_APPLICATION_IS_LAUNCHER: GApplicationFlags = 2;
pub const G_APPLICATION_IS_SERVICE: GApplicationFlags = 1;
pub const G_APPLICATION_DEFAULT_FLAGS: GApplicationFlags = 0;
pub const G_APPLICATION_FLAGS_NONE: GApplicationFlags = 0;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSimpleActionGroup {
    pub parent_instance: GObject,
    pub priv_0: *mut GSimpleActionGroupPrivate,
}
pub type GSimpleActionGroupPrivate = _GSimpleActionGroupPrivate;
pub type GSimpleActionGroup = _GSimpleActionGroup;
pub type GRemoteActionGroup = _GRemoteActionGroup;
pub type GActionMap = _GActionMap;
pub type GActionGroup = _GActionGroup;
pub type GAction = _GAction;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplication {
    pub parent_instance: GObject,
    pub priv_0: *mut GApplicationPrivate,
}
pub type GApplicationPrivate = _GApplicationPrivate;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GApplicationPrivate {
    pub flags: GApplicationFlags,
    pub id: *mut gchar,
    pub version: *mut gchar,
    pub resource_path: *mut gchar,
    pub actions: *mut GActionGroup,
    pub inactivity_timeout_id: guint,
    pub inactivity_timeout: guint,
    pub use_count: guint,
    pub busy_count: guint,
    #[bitfield(name = "is_registered", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "is_remote", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "did_startup", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "did_shutdown", ty = "guint", bits = "3..=3")]
    #[bitfield(name = "must_quit_now", ty = "guint", bits = "4..=4")]
    pub is_registered_is_remote_did_startup_did_shutdown_must_quit_now: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub remote_actions: *mut GRemoteActionGroup,
    pub impl_0: *mut GApplicationImpl,
    pub notifications: *mut GNotificationBackend,
    pub main_options: *mut GOptionGroup,
    pub option_groups: *mut GSList,
    pub packed_options: *mut GHashTable,
    pub options_parsed: gboolean,
    pub parameter_string: *mut gchar,
    pub summary: *mut gchar,
    pub description: *mut gchar,
    pub option_strings: *mut GSList,
}
pub type GNotificationBackend = _GNotificationBackend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNotificationBackend {
    pub parent_instance: GObject,
    pub application: *mut GApplication,
    pub dbus_connection: *mut GDBusConnection,
}
pub type GDBusConnection = _GDBusConnection;
pub type GApplication = _GApplication;
pub type GApplicationImpl = _GApplicationImpl;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplicationCommandLine {
    pub parent_instance: GObject,
    pub priv_0: *mut GApplicationCommandLinePrivate,
}
pub type GApplicationCommandLinePrivate = _GApplicationCommandLinePrivate;
pub type GApplicationCommandLine = _GApplicationCommandLine;
pub type GNotification = _GNotification;
pub type GFile = _GFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GApplicationClass {
    pub parent_class: GObjectClass,
    pub startup: Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
    pub activate: Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
    pub open:
        Option<unsafe extern "C" fn(*mut GApplication, *mut *mut GFile, gint, *const gchar) -> ()>,
    pub command_line: Option<
        unsafe extern "C" fn(*mut GApplication, *mut GApplicationCommandLine) -> ::core::ffi::c_int,
    >,
    pub local_command_line: Option<
        unsafe extern "C" fn(
            *mut GApplication,
            *mut *mut *mut gchar,
            *mut ::core::ffi::c_int,
        ) -> gboolean,
    >,
    pub before_emit: Option<unsafe extern "C" fn(*mut GApplication, *mut GVariant) -> ()>,
    pub after_emit: Option<unsafe extern "C" fn(*mut GApplication, *mut GVariant) -> ()>,
    pub add_platform_data:
        Option<unsafe extern "C" fn(*mut GApplication, *mut GVariantBuilder) -> ()>,
    pub quit_mainloop: Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
    pub run_mainloop: Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
    pub shutdown: Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
    pub dbus_register: Option<
        unsafe extern "C" fn(
            *mut GApplication,
            *mut GDBusConnection,
            *const gchar,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub dbus_unregister:
        Option<unsafe extern "C" fn(*mut GApplication, *mut GDBusConnection, *const gchar) -> ()>,
    pub handle_local_options:
        Option<unsafe extern "C" fn(*mut GApplication, *mut GVariantDict) -> gint>,
    pub name_lost: Option<unsafe extern "C" fn(*mut GApplication) -> gboolean>,
    pub padding: [gpointer; 7],
}
pub type GApplicationClass = _GApplicationClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GApplicationExportedActions {
    pub parent_instance: GSimpleActionGroup,
    pub application: *mut GApplication,
}
pub type GApplicationExportedActionsClass = GSimpleActionGroupClass;
pub type GSimpleActionGroupClass = _GSimpleActionGroupClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSimpleActionGroupClass {
    pub parent_class: GObjectClass,
    pub padding: [gpointer; 12],
}
pub type GRemoteActionGroupInterface = _GRemoteActionGroupInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRemoteActionGroupInterface {
    pub g_iface: GTypeInterface,
    pub activate_action_full: Option<
        unsafe extern "C" fn(
            *mut GRemoteActionGroup,
            *const gchar,
            *mut GVariant,
            *mut GVariant,
        ) -> (),
    >,
    pub change_action_state_full: Option<
        unsafe extern "C" fn(
            *mut GRemoteActionGroup,
            *const gchar,
            *mut GVariant,
            *mut GVariant,
        ) -> (),
    >,
}
pub const SIGNAL_NAME_LOST: C2RustUnnamed_5 = 7;
pub const SIGNAL_HANDLE_LOCAL_OPTIONS: C2RustUnnamed_5 = 6;
pub const SIGNAL_COMMAND_LINE: C2RustUnnamed_5 = 5;
pub const SIGNAL_OPEN: C2RustUnnamed_5 = 3;
pub const SIGNAL_ACTIVATE: C2RustUnnamed_5 = 2;
pub const SIGNAL_SHUTDOWN: C2RustUnnamed_5 = 1;
pub const SIGNAL_STARTUP: C2RustUnnamed_5 = 0;
pub const PROP_IS_BUSY: C2RustUnnamed_4 = 9;
pub const PROP_ACTION_GROUP: C2RustUnnamed_4 = 8;
pub const PROP_INACTIVITY_TIMEOUT: C2RustUnnamed_4 = 7;
pub const PROP_IS_REMOTE: C2RustUnnamed_4 = 6;
pub const PROP_IS_REGISTERED: C2RustUnnamed_4 = 5;
pub const PROP_RESOURCE_BASE_PATH: C2RustUnnamed_4 = 4;
pub const PROP_FLAGS: C2RustUnnamed_4 = 3;
pub const PROP_VERSION: C2RustUnnamed_4 = 2;
pub const PROP_APPLICATION_ID: C2RustUnnamed_4 = 1;
pub type GActionMapInterface = _GActionMapInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GActionMapInterface {
    pub g_iface: GTypeInterface,
    pub lookup_action: Option<unsafe extern "C" fn(*mut GActionMap, *const gchar) -> *mut GAction>,
    pub add_action: Option<unsafe extern "C" fn(*mut GActionMap, *mut GAction) -> ()>,
    pub remove_action: Option<unsafe extern "C" fn(*mut GActionMap, *const gchar) -> ()>,
}
pub type GActionGroupInterface = _GActionGroupInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GActionGroupInterface {
    pub g_iface: GTypeInterface,
    pub has_action: Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> gboolean>,
    pub list_actions: Option<unsafe extern "C" fn(*mut GActionGroup) -> *mut *mut gchar>,
    pub get_action_enabled:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> gboolean>,
    pub get_action_parameter_type:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *const GVariantType>,
    pub get_action_state_type:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *const GVariantType>,
    pub get_action_state_hint:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *mut GVariant>,
    pub get_action_state:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> *mut GVariant>,
    pub change_action_state:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>,
    pub activate_action:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>,
    pub action_added: Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> ()>,
    pub action_removed: Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> ()>,
    pub action_enabled_changed:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, gboolean) -> ()>,
    pub action_state_changed:
        Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>,
    pub query_action: Option<
        unsafe extern "C" fn(
            *mut GActionGroup,
            *const gchar,
            *mut gboolean,
            *mut *const GVariantType,
            *mut *const GVariantType,
            *mut *mut GVariant,
            *mut *mut GVariant,
        ) -> gboolean,
    >,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GApplicationBusyBinding {
    pub app: *mut GApplication,
    pub is_busy: gboolean,
}
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const PROP_NONE: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const NR_SIGNALS: C2RustUnnamed_5 = 8;
pub const SIGNAL_ACTION: C2RustUnnamed_5 = 4;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INT: GType = ((6 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_POINTER: GType = ((17 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_OBJECT: GType = ((20 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
static mut safe_c2rust_g_application_signals: [guint; 8] = [0; 8];
static mut safe_c2rust_GApplication_private_offset: gint = 0;
#[inline]
unsafe extern "C" fn safe_c2rust_g_application_get_instance_private(
    mut self_0: *mut GApplication,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GApplication_private_offset as glong as isize)
        as gpointer;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_application_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GApplication\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GApplicationClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_application_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GApplication>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GApplication) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_application_init as unsafe extern "C" fn(*mut GApplication) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GApplication_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GApplicationPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroupInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_application_action_group_iface_init
                as unsafe extern "C" fn(*mut GActionGroupInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_action_group_get_type(),
        &raw const g_implement_interface_info,
    );
    let g_implement_interface_info_0: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionMapInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_application_action_map_iface_init
                as unsafe extern "C" fn(*mut GActionMapInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_action_map_get_type(),
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_application_get_type_once();
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
static mut safe_c2rust_g_application_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_application_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_application_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GApplication_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GApplication_private_offset);
    }
    safe_c2rust_g_application_class_init(klass as *mut GApplicationClass);
}
unsafe extern "C" fn safe_c2rust_g_application_exported_actions_class_intern_init(
    mut klass: gpointer,
) {
    safe_c2rust_g_application_exported_actions_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GApplicationExportedActions_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GApplicationExportedActions_private_offset,
        );
    }
    safe_c2rust_g_application_exported_actions_class_init(
        klass as *mut GApplicationExportedActionsClass,
    );
}
static mut safe_c2rust_g_application_exported_actions_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GApplicationExportedActions_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_application_exported_actions_get_type() -> GType {
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
        let mut g_define_type_id: GType =
            safe_c2rust_g_application_exported_actions_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_application_exported_actions_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_simple_action_group_get_type(),
        g_intern_static_string(b"GApplicationExportedActions\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GApplicationExportedActionsClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_application_exported_actions_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GApplicationExportedActions>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GApplicationExportedActions) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_application_exported_actions_init
                    as unsafe extern "C" fn(*mut GApplicationExportedActions) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GRemoteActionGroupInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_application_exported_actions_iface_init
                as unsafe extern "C" fn(*mut GRemoteActionGroupInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_remote_action_group_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_application_exported_actions_activate_action_full(
    mut remote: *mut GRemoteActionGroup,
    mut action_name: *const gchar,
    mut parameter: *mut GVariant,
    mut platform_data: *mut GVariant,
) {
    let mut exported: *mut GApplicationExportedActions = remote as *mut GApplicationExportedActions;
    (*((*((*exported).application as *mut GTypeInstance)).g_class as *mut GApplicationClass))
        .before_emit
        .expect("non-null function pointer")((*exported).application, platform_data);
    g_action_group_activate_action(
        exported as *mut ::core::ffi::c_void as *mut GActionGroup,
        action_name,
        parameter,
    );
    (*((*((*exported).application as *mut GTypeInstance)).g_class as *mut GApplicationClass))
        .after_emit
        .expect("non-null function pointer")((*exported).application, platform_data);
}
unsafe extern "C" fn safe_c2rust_g_application_exported_actions_change_action_state_full(
    mut remote: *mut GRemoteActionGroup,
    mut action_name: *const gchar,
    mut value: *mut GVariant,
    mut platform_data: *mut GVariant,
) {
    let mut exported: *mut GApplicationExportedActions = remote as *mut GApplicationExportedActions;
    (*((*((*exported).application as *mut GTypeInstance)).g_class as *mut GApplicationClass))
        .before_emit
        .expect("non-null function pointer")((*exported).application, platform_data);
    g_action_group_change_action_state(
        exported as *mut ::core::ffi::c_void as *mut GActionGroup,
        action_name,
        value,
    );
    (*((*((*exported).application as *mut GTypeInstance)).g_class as *mut GApplicationClass))
        .after_emit
        .expect("non-null function pointer")((*exported).application, platform_data);
}
unsafe extern "C" fn safe_c2rust_g_application_exported_actions_init(
    mut actions: *mut GApplicationExportedActions,
) {
}
unsafe extern "C" fn safe_c2rust_g_application_exported_actions_iface_init(
    mut iface: *mut GRemoteActionGroupInterface,
) {
    (*iface).activate_action_full = Some(
        safe_c2rust_g_application_exported_actions_activate_action_full
            as unsafe extern "C" fn(
                *mut GRemoteActionGroup,
                *const gchar,
                *mut GVariant,
                *mut GVariant,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GRemoteActionGroup,
                *const gchar,
                *mut GVariant,
                *mut GVariant,
            ) -> (),
        >;
    (*iface).change_action_state_full = Some(
        safe_c2rust_g_application_exported_actions_change_action_state_full
            as unsafe extern "C" fn(
                *mut GRemoteActionGroup,
                *const gchar,
                *mut GVariant,
                *mut GVariant,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut GRemoteActionGroup,
                *const gchar,
                *mut GVariant,
                *mut GVariant,
            ) -> (),
        >;
}
unsafe extern "C" fn safe_c2rust_g_application_exported_actions_class_init(
    mut class: *mut GApplicationExportedActionsClass,
) {
}
unsafe extern "C" fn safe_c2rust_g_application_exported_actions_new(
    mut application: *mut GApplication,
) -> *mut GActionGroup {
    let mut actions: *mut GApplicationExportedActions =
        ::core::ptr::null_mut::<GApplicationExportedActions>();
    actions = g_object_new(
        safe_c2rust_g_application_exported_actions_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GApplicationExportedActions;
    (*actions).application = application;
    return actions as *mut ::core::ffi::c_void as *mut GActionGroup;
}
unsafe extern "C" fn safe_c2rust_free_option_entry(mut data: gpointer) {
    let mut entry: *mut GOptionEntry = data as *mut GOptionEntry;
    match (*entry).arg as ::core::ffi::c_uint {
        1 | 4 => {
            g_free(*((*entry).arg_data as *mut *mut gchar) as gpointer);
        }
        5 | 6 => {
            g_strfreev(*((*entry).arg_data as *mut *mut *mut gchar));
        }
        _ => {}
    }
    g_free((*entry).arg_data);
    g_slice_free1(
        ::core::mem::size_of::<GOptionEntry>() as gsize,
        entry as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_application_pack_option_entries(
    mut application: *mut GApplication,
    mut dict: *mut GVariantDict,
) {
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut item: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    g_hash_table_iter_init(&raw mut iter, (*(*application).priv_0).packed_options);
    while g_hash_table_iter_next(
        &raw mut iter,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut item,
    ) != 0
    {
        let mut entry: *mut GOptionEntry = item as *mut GOptionEntry;
        let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        match (*entry).arg as ::core::ffi::c_uint {
            0 => {
                if *((*entry).arg_data as *mut gboolean) != 2 as ::core::ffi::c_int {
                    value = g_variant_new_boolean(*((*entry).arg_data as *mut gboolean));
                }
            }
            1 => {
                if !(*((*entry).arg_data as *mut *mut gchar)).is_null() {
                    value = g_variant_new_string(*((*entry).arg_data as *mut *mut gchar));
                }
            }
            2 => {
                if *((*entry).arg_data as *mut gint32) != 0 {
                    value = g_variant_new_int32(*((*entry).arg_data as *mut gint32));
                }
            }
            4 => {
                if !(*((*entry).arg_data as *mut *mut gchar)).is_null() {
                    value = g_variant_new_bytestring(*((*entry).arg_data as *mut *mut gchar));
                }
            }
            5 => {
                if !(*((*entry).arg_data as *mut *mut *mut gchar)).is_null() {
                    value = g_variant_new_strv(
                        *((*entry).arg_data as *mut *mut *const gchar),
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                }
            }
            6 => {
                if !(*((*entry).arg_data as *mut *mut *mut gchar)).is_null() {
                    value = g_variant_new_bytestring_array(
                        *((*entry).arg_data as *mut *mut *const gchar),
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                }
            }
            7 => {
                if *((*entry).arg_data as *mut gdouble) != 0. {
                    value = g_variant_new_double(*((*entry).arg_data as *mut gdouble));
                }
            }
            8 => {
                if *((*entry).arg_data as *mut gint64) != 0 {
                    value = g_variant_new_int64(*((*entry).arg_data as *mut gint64));
                }
            }
            _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplication.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    472 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
        if !value.is_null() {
            g_variant_dict_insert_value(dict, (*entry).long_name, value);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_application_parse_command_line(
    mut application: *mut GApplication,
    mut arguments: *mut *mut *mut gchar,
    mut print_version: *mut gboolean,
    mut error: *mut *mut GError,
) -> *mut GVariantDict {
    let mut become_service: gboolean = FALSE;
    let mut app_id: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut replace: gboolean = FALSE;
    let mut version: gboolean = FALSE;
    let mut dict: *mut GVariantDict = ::core::ptr::null_mut::<GVariantDict>();
    let mut context: *mut GOptionContext = ::core::ptr::null_mut::<GOptionContext>();
    let mut gapplication_group: *mut GOptionGroup = ::core::ptr::null_mut::<GOptionGroup>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).options_parsed == 0 {
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
            b"!application->priv->options_parsed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantDict>();
    }
    context = g_option_context_new((*(*application).priv_0).parameter_string);
    g_option_context_set_summary(context, (*(*application).priv_0).summary);
    g_option_context_set_description(context, (*(*application).priv_0).description);
    gapplication_group = g_option_group_new(
        b"gapplication\0" as *const u8 as *const gchar,
        glib_gettext(b"GApplication Options:\0" as *const u8 as *const gchar),
        glib_gettext(b"Show GApplication options\0" as *const u8 as *const gchar),
        NULL_0,
        None,
    );
    g_option_group_set_translation_domain(
        gapplication_group,
        GETTEXT_PACKAGE.as_ptr() as *const gchar,
    );
    g_option_context_add_group(context, gapplication_group);
    if (*(*application).priv_0).main_options.is_null()
        && (*(*application).priv_0).flags as ::core::ffi::c_uint
            & G_APPLICATION_HANDLES_COMMAND_LINE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        g_option_context_set_ignore_unknown_options(context, TRUE);
        g_option_context_set_help_enabled(context, FALSE);
    }
    if !(*(*application).priv_0).main_options.is_null() {
        g_option_context_set_main_group(context, (*(*application).priv_0).main_options);
        (*(*application).priv_0).main_options = ::core::ptr::null_mut::<GOptionGroup>();
    }
    while !(*(*application).priv_0).option_groups.is_null() {
        g_option_context_add_group(
            context,
            (*(*(*application).priv_0).option_groups).data as *mut GOptionGroup,
        );
        (*(*application).priv_0).option_groups = g_slist_delete_link(
            (*(*application).priv_0).option_groups,
            (*(*application).priv_0).option_groups,
        );
    }
    if (*(*application).priv_0).flags as ::core::ffi::c_uint
        & (G_APPLICATION_IS_SERVICE as ::core::ffi::c_int
            | G_APPLICATION_IS_LAUNCHER as ::core::ffi::c_int) as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
        let mut entries: [GOptionEntry; 2] = [
            _GOptionEntry {
                long_name: b"gapplication-service\0" as *const u8 as *const gchar,
                short_name: '\0' as i32 as gchar,
                flags: 0 as gint,
                arg: G_OPTION_ARG_NONE,
                arg_data: &raw mut become_service as gpointer,
                description: b"Enter GApplication service mode (use from D-Bus service files)\0"
                    as *const u8 as *const gchar,
                arg_description: ::core::ptr::null::<gchar>(),
            },
            _GOptionEntry {
                long_name: ::core::ptr::null::<gchar>(),
                short_name: 0 as gchar,
                flags: 0 as gint,
                arg: G_OPTION_ARG_NONE,
                arg_data: NULL_0,
                description: ::core::ptr::null::<gchar>(),
                arg_description: ::core::ptr::null::<gchar>(),
            },
        ];
        g_option_group_add_entries(gapplication_group, &raw mut entries as *mut GOptionEntry);
    }
    if (*(*application).priv_0).flags as ::core::ffi::c_uint
        & G_APPLICATION_CAN_OVERRIDE_APP_ID as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        let mut entries_0: [GOptionEntry; 2] = [
            _GOptionEntry {
                long_name: b"gapplication-app-id\0" as *const u8 as *const gchar,
                short_name: '\0' as i32 as gchar,
                flags: 0 as gint,
                arg: G_OPTION_ARG_STRING,
                arg_data: &raw mut app_id as gpointer,
                description: b"Override the application\xE2\x80\x99s ID\0" as *const u8
                    as *const gchar,
                arg_description: ::core::ptr::null::<gchar>(),
            },
            _GOptionEntry {
                long_name: ::core::ptr::null::<gchar>(),
                short_name: 0 as gchar,
                flags: 0 as gint,
                arg: G_OPTION_ARG_NONE,
                arg_data: NULL_0,
                description: ::core::ptr::null::<gchar>(),
                arg_description: ::core::ptr::null::<gchar>(),
            },
        ];
        g_option_group_add_entries(gapplication_group, &raw mut entries_0 as *mut GOptionEntry);
    }
    if !(*(*application).priv_0).version.is_null() {
        let mut entries_1: [GOptionEntry; 2] = [
            _GOptionEntry {
                long_name: b"version\0" as *const u8 as *const gchar,
                short_name: '\0' as i32 as gchar,
                flags: 0 as gint,
                arg: G_OPTION_ARG_NONE,
                arg_data: &raw mut version as gpointer,
                description: b"Print the application version\0" as *const u8 as *const gchar,
                arg_description: ::core::ptr::null::<gchar>(),
            },
            _GOptionEntry {
                long_name: ::core::ptr::null::<gchar>(),
                short_name: 0 as gchar,
                flags: 0 as gint,
                arg: G_OPTION_ARG_NONE,
                arg_data: NULL_0,
                description: ::core::ptr::null::<gchar>(),
                arg_description: ::core::ptr::null::<gchar>(),
            },
        ];
        g_option_group_add_entries(gapplication_group, &raw mut entries_1 as *mut GOptionEntry);
    }
    if (*(*application).priv_0).flags as ::core::ffi::c_uint
        & G_APPLICATION_ALLOW_REPLACEMENT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        let mut entries_2: [GOptionEntry; 2] = [
            _GOptionEntry {
                long_name: b"gapplication-replace\0" as *const u8 as *const gchar,
                short_name: '\0' as i32 as gchar,
                flags: 0 as gint,
                arg: G_OPTION_ARG_NONE,
                arg_data: &raw mut replace as gpointer,
                description: b"Replace the running instance\0" as *const u8 as *const gchar,
                arg_description: ::core::ptr::null::<gchar>(),
            },
            _GOptionEntry {
                long_name: ::core::ptr::null::<gchar>(),
                short_name: 0 as gchar,
                flags: 0 as gint,
                arg: G_OPTION_ARG_NONE,
                arg_data: NULL_0,
                description: ::core::ptr::null::<gchar>(),
                arg_description: ::core::ptr::null::<gchar>(),
            },
        ];
        g_option_group_add_entries(gapplication_group, &raw mut entries_2 as *mut GOptionEntry);
    }
    if !(g_option_context_parse_strv(context, arguments, error) == 0) {
        *print_version = version;
        if become_service != 0 {
            (*(*application).priv_0).flags =
                ::core::mem::transmute::<::core::ffi::c_uint, GApplicationFlags>(
                    (*(*application).priv_0).flags as ::core::ffi::c_uint
                        | G_APPLICATION_IS_SERVICE as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
        }
        if !app_id.is_null() {
            safe_c2rust_g_application_set_application_id(application, app_id);
        }
        if replace != 0 {
            (*(*application).priv_0).flags =
                ::core::mem::transmute::<::core::ffi::c_uint, GApplicationFlags>(
                    (*(*application).priv_0).flags as ::core::ffi::c_uint
                        | G_APPLICATION_REPLACE as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
        }
        dict = g_variant_dict_new(::core::ptr::null_mut::<GVariant>());
        if !(*(*application).priv_0).packed_options.is_null() {
            safe_c2rust_g_application_pack_option_entries(application, dict);
            g_hash_table_unref((*(*application).priv_0).packed_options);
            (*(*application).priv_0).packed_options = ::core::ptr::null_mut::<GHashTable>();
        }
    }
    (*(*application).priv_0).options_parsed = TRUE as gboolean;
    g_option_context_free(context);
    g_free(app_id as gpointer);
    return dict;
}
unsafe extern "C" fn safe_c2rust_add_packed_option(
    mut application: *mut GApplication,
    mut entry: *mut GOptionEntry,
) {
    match (*entry).arg as ::core::ffi::c_uint {
        0 => {
            (*entry).arg_data = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<gboolean>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc_n(__n, __s);
                }
                __p
            }) as *mut gboolean as gpointer;
            *((*entry).arg_data as *mut gboolean) = 2 as ::core::ffi::c_int as gboolean;
        }
        2 => {
            (*entry).arg_data = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<gint>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut gint as gpointer;
        }
        1 | 4 | 5 | 6 => {
            (*entry).arg_data = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<gpointer>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut gpointer as gpointer;
        }
        8 => {
            (*entry).arg_data = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<gint64>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut gint64 as gpointer;
        }
        7 => {
            (*entry).arg_data = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<gdouble>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut gdouble as gpointer;
        }
        _ => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"file %s: line %d (%s): should not be reached\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplication.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                661 as ::core::ffi::c_int,
                G_STRFUNC,
            );
            return;
        }
    }
    if (*(*application).priv_0).packed_options.is_null() {
        (*(*application).priv_0).packed_options = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            Some(safe_c2rust_free_option_entry as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    g_hash_table_insert(
        (*(*application).priv_0).packed_options,
        safe_c2rust_g_strdup_inline((*entry).long_name as *const ::core::ffi::c_char) as gpointer,
        (if 1 as ::core::ffi::c_int != 0 {
            g_slice_copy(
                ::core::mem::size_of::<GOptionEntry>() as gsize,
                entry as gconstpointer,
            ) as *mut GOptionEntry
        } else {
            ::core::ptr::null_mut::<GOptionEntry>()
        }) as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_add_main_option_entries(
    mut application: *mut GApplication,
    mut entries: *const GOptionEntry,
) {
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !entries.is_null() {
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
            b"entries != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*application).priv_0).main_options.is_null() {
        (*(*application).priv_0).main_options = g_option_group_new(
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            NULL_0,
            None,
        );
        g_option_group_set_translation_domain(
            (*(*application).priv_0).main_options,
            ::core::ptr::null::<gchar>(),
        );
    }
    i = 0 as ::core::ffi::c_int as gint;
    while !(*entries.offset(i as isize)).long_name.is_null() {
        let mut my_entries: [GOptionEntry; 2] = [
            _GOptionEntry {
                long_name: ::core::ptr::null::<gchar>(),
                short_name: 0 as gchar,
                flags: 0 as gint,
                arg: G_OPTION_ARG_NONE,
                arg_data: NULL_0,
                description: ::core::ptr::null::<gchar>(),
                arg_description: ::core::ptr::null::<gchar>(),
            },
            _GOptionEntry {
                long_name: ::core::ptr::null::<gchar>(),
                short_name: 0 as gchar,
                flags: 0 as gint,
                arg: G_OPTION_ARG_NONE,
                arg_data: NULL_0,
                description: ::core::ptr::null::<gchar>(),
                arg_description: ::core::ptr::null::<gchar>(),
            },
        ];
        my_entries[0 as ::core::ffi::c_int as usize] = *entries.offset(i as isize);
        if my_entries[0 as ::core::ffi::c_int as usize]
            .arg_data
            .is_null()
        {
            safe_c2rust_add_packed_option(
                application,
                (&raw mut my_entries as *mut GOptionEntry).offset(0 as ::core::ffi::c_int as isize)
                    as *mut GOptionEntry,
            );
        }
        g_option_group_add_entries(
            (*(*application).priv_0).main_options,
            &raw mut my_entries as *mut GOptionEntry,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_add_main_option(
    mut application: *mut GApplication,
    mut long_name: *const ::core::ffi::c_char,
    mut short_name: ::core::ffi::c_char,
    mut flags: GOptionFlags,
    mut arg: GOptionArg,
    mut description: *const ::core::ffi::c_char,
    mut arg_description: *const ::core::ffi::c_char,
) {
    let mut dup_string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut my_entry: [GOptionEntry; 2] = [
        _GOptionEntry {
            long_name: ::core::ptr::null::<gchar>(),
            short_name: short_name as gchar,
            flags: flags as gint,
            arg: arg,
            arg_data: NULL_0,
            description: ::core::ptr::null::<gchar>(),
            arg_description: ::core::ptr::null::<gchar>(),
        },
        _GOptionEntry {
            long_name: ::core::ptr::null::<gchar>(),
            short_name: 0 as gchar,
            flags: 0 as gint,
            arg: G_OPTION_ARG_NONE,
            arg_data: NULL_0,
            description: ::core::ptr::null::<gchar>(),
            arg_description: ::core::ptr::null::<gchar>(),
        },
    ];
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !long_name.is_null() {
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
            b"long_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !description.is_null() {
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
            b"description != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    dup_string = safe_c2rust_g_strdup_inline(long_name) as *mut gchar;
    my_entry[0 as ::core::ffi::c_int as usize].long_name = dup_string;
    (*(*application).priv_0).option_strings = g_slist_prepend(
        (*(*application).priv_0).option_strings,
        dup_string as gpointer,
    );
    dup_string = safe_c2rust_g_strdup_inline(description) as *mut gchar;
    my_entry[0 as ::core::ffi::c_int as usize].description = dup_string;
    (*(*application).priv_0).option_strings = g_slist_prepend(
        (*(*application).priv_0).option_strings,
        dup_string as gpointer,
    );
    dup_string = safe_c2rust_g_strdup_inline(arg_description) as *mut gchar;
    my_entry[0 as ::core::ffi::c_int as usize].arg_description = dup_string;
    (*(*application).priv_0).option_strings = g_slist_prepend(
        (*(*application).priv_0).option_strings,
        dup_string as gpointer,
    );
    safe_c2rust_g_application_add_main_option_entries(
        application,
        &raw mut my_entry as *mut GOptionEntry,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_add_option_group(
    mut application: *mut GApplication,
    mut group: *mut GOptionGroup,
) {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !group.is_null() {
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
            b"group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*application).priv_0).option_groups =
        g_slist_prepend((*(*application).priv_0).option_groups, group as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_set_option_context_parameter_string(
    mut application: *mut GApplication,
    mut parameter_string: *const gchar,
) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*(*application).priv_0).parameter_string as gpointer);
    (*(*application).priv_0).parameter_string =
        safe_c2rust_g_strdup_inline(parameter_string as *const ::core::ffi::c_char) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_set_option_context_summary(
    mut application: *mut GApplication,
    mut summary: *const gchar,
) {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*(*application).priv_0).summary as gpointer);
    (*(*application).priv_0).summary =
        safe_c2rust_g_strdup_inline(summary as *const ::core::ffi::c_char) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_set_option_context_description(
    mut application: *mut GApplication,
    mut description: *const gchar,
) {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*(*application).priv_0).description as gpointer);
    (*(*application).priv_0).description =
        safe_c2rust_g_strdup_inline(description as *const ::core::ffi::c_char) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_g_application_real_before_emit(
    mut application: *mut GApplication,
    mut platform_data: *mut GVariant,
) {
}
unsafe extern "C" fn safe_c2rust_g_application_real_after_emit(
    mut application: *mut GApplication,
    mut platform_data: *mut GVariant,
) {
}
unsafe extern "C" fn safe_c2rust_g_application_real_startup(mut application: *mut GApplication) {
    (*(*application).priv_0).set_did_startup(TRUE as guint as guint);
}
unsafe extern "C" fn safe_c2rust_g_application_real_shutdown(mut application: *mut GApplication) {
    (*(*application).priv_0).set_did_shutdown(TRUE as guint as guint);
}
unsafe extern "C" fn safe_c2rust_g_application_real_activate(mut application: *mut GApplication) {
    if g_signal_has_handler_pending(
        application as gpointer,
        safe_c2rust_g_application_signals[SIGNAL_ACTIVATE as ::core::ffi::c_int as usize],
        0 as GQuark,
        TRUE,
    ) == 0
        && (*((*(application as *mut GTypeInstance)).g_class as *mut GApplicationClass)).activate
            == Some(
                safe_c2rust_g_application_real_activate
                    as unsafe extern "C" fn(*mut GApplication) -> (),
            )
    {
        static mut safe_c2rust_warned: gboolean = 0;
        if safe_c2rust_warned != 0 {
            return;
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Your application does not implement g_application_activate() and has no handlers connected to the 'activate' signal.  It should do one of these.\0"
                as *const u8 as *const gchar,
        );
        safe_c2rust_warned = TRUE as gboolean;
    }
}
unsafe extern "C" fn safe_c2rust_g_application_real_open(
    mut application: *mut GApplication,
    mut files: *mut *mut GFile,
    mut n_files: gint,
    mut hint: *const gchar,
) {
    if g_signal_has_handler_pending(
        application as gpointer,
        safe_c2rust_g_application_signals[SIGNAL_OPEN as ::core::ffi::c_int as usize],
        0 as GQuark,
        TRUE,
    ) == 0
        && (*((*(application as *mut GTypeInstance)).g_class as *mut GApplicationClass)).open
            == Some(
                safe_c2rust_g_application_real_open
                    as unsafe extern "C" fn(
                        *mut GApplication,
                        *mut *mut GFile,
                        gint,
                        *const gchar,
                    ) -> (),
            )
    {
        static mut safe_c2rust_warned: gboolean = 0;
        if safe_c2rust_warned != 0 {
            return;
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Your application claims to support opening files but does not implement g_application_open() and has no handlers connected to the 'open' signal.\0"
                as *const u8 as *const gchar,
        );
        safe_c2rust_warned = TRUE as gboolean;
    }
}
unsafe extern "C" fn safe_c2rust_g_application_real_command_line(
    mut application: *mut GApplication,
    mut cmdline: *mut GApplicationCommandLine,
) -> ::core::ffi::c_int {
    if g_signal_has_handler_pending(
        application as gpointer,
        safe_c2rust_g_application_signals[SIGNAL_COMMAND_LINE as ::core::ffi::c_int as usize],
        0 as GQuark,
        TRUE,
    ) == 0
        && (*((*(application as *mut GTypeInstance)).g_class as *mut GApplicationClass))
            .command_line
            == Some(
                safe_c2rust_g_application_real_command_line
                    as unsafe extern "C" fn(
                        *mut GApplication,
                        *mut GApplicationCommandLine,
                    ) -> ::core::ffi::c_int,
            )
    {
        static mut safe_c2rust_warned: gboolean = 0;
        if safe_c2rust_warned != 0 {
            return 1 as ::core::ffi::c_int;
        }
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Your application claims to support custom command line handling but does not implement g_application_command_line() and has no handlers connected to the 'command-line' signal.\0"
                as *const u8 as *const gchar,
        );
        safe_c2rust_warned = TRUE as gboolean;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_application_real_handle_local_options(
    mut application: *mut GApplication,
    mut options: *mut GVariantDict,
) -> gint {
    return -(1 as gint);
}
unsafe extern "C" fn safe_c2rust_get_platform_data(
    mut application: *mut GApplication,
    mut options: *mut GVariant,
) -> *mut GVariant {
    let mut builder: *mut GVariantBuilder = ::core::ptr::null_mut::<GVariantBuilder>();
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    builder = g_variant_builder_new(g_variant_type_checked_(
        b"a{sv}\0" as *const u8 as *const gchar,
    ));
    let mut cwd: *mut gchar = g_get_current_dir();
    g_variant_builder_add(
        builder,
        b"{sv}\0" as *const u8 as *const gchar,
        b"cwd\0" as *const u8 as *const ::core::ffi::c_char,
        g_variant_new_bytestring(cwd),
    );
    g_free(cwd as gpointer);
    if (*(*application).priv_0).flags as ::core::ffi::c_uint
        & G_APPLICATION_SEND_ENVIRONMENT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        let mut array: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut envp: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        envp = g_get_environ();
        array = g_variant_new_bytestring_array(
            envp as *mut *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
        );
        g_strfreev(envp);
        g_variant_builder_add(
            builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"environ\0" as *const u8 as *const ::core::ffi::c_char,
            array,
        );
    }
    if !options.is_null() {
        g_variant_builder_add(
            builder,
            b"{sv}\0" as *const u8 as *const gchar,
            b"options\0" as *const u8 as *const ::core::ffi::c_char,
            options,
        );
    }
    (*((*(application as *mut GTypeInstance)).g_class as *mut GApplicationClass))
        .add_platform_data
        .expect("non-null function pointer")(application, builder);
    result = g_variant_builder_end(builder);
    g_variant_builder_unref(builder);
    return result;
}
unsafe extern "C" fn safe_c2rust_g_application_call_command_line(
    mut application: *mut GApplication,
    mut arguments: *const *const gchar,
    mut options: *mut GVariant,
    mut exit_status: *mut gint,
) {
    if (*(*application).priv_0).is_remote() != 0 {
        let mut platform_data: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        platform_data = safe_c2rust_get_platform_data(application, options);
        *exit_status = g_application_impl_command_line(
            (*(*application).priv_0).impl_0,
            arguments,
            platform_data,
        ) as gint;
    } else {
        let mut cmdline: *mut GApplicationCommandLine =
            ::core::ptr::null_mut::<GApplicationCommandLine>();
        let mut v: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut handler_exit_status: gint = 0;
        v = g_variant_new_bytestring_array(
            arguments as *mut *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
        );
        cmdline = g_object_new(
            g_application_command_line_get_type(),
            b"arguments\0" as *const u8 as *const gchar,
            v,
            b"options\0" as *const u8 as *const ::core::ffi::c_char,
            options,
            NULL_0,
        ) as *mut GApplicationCommandLine;
        g_signal_emit(
            application as gpointer,
            safe_c2rust_g_application_signals[SIGNAL_COMMAND_LINE as ::core::ffi::c_int as usize],
            0 as GQuark,
            cmdline,
            &raw mut handler_exit_status,
        );
        g_application_command_line_set_exit_status(
            cmdline,
            handler_exit_status as ::core::ffi::c_int,
        );
        *exit_status = g_application_command_line_get_exit_status(cmdline) as gint;
        g_object_unref(cmdline as gpointer);
    };
}
unsafe extern "C" fn safe_c2rust_g_application_real_local_command_line(
    mut application: *mut GApplication,
    mut arguments: *mut *mut *mut gchar,
    mut exit_status: *mut ::core::ffi::c_int,
) -> gboolean {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut options: *mut GVariantDict = ::core::ptr::null_mut::<GVariantDict>();
    let mut n_args: gint = 0;
    let mut print_version: gboolean = FALSE;
    options = safe_c2rust_g_application_parse_command_line(
        application,
        arguments,
        &raw mut print_version,
        &raw mut error,
    );
    if options.is_null() {
        g_printerr(b"%s\n\0" as *const u8 as *const gchar, (*error).message);
        g_error_free(error);
        *exit_status = 1 as ::core::ffi::c_int;
        return TRUE;
    }
    if print_version != 0 {
        let mut prgname: *const ::core::ffi::c_char = g_get_prgname() as *const ::core::ffi::c_char;
        if ({
            let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
            if !(*(*application).priv_0).version.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplication.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1137 as ::core::ffi::c_int,
                G_STRFUNC,
                b"application->priv->version != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !prgname.is_null() {
            g_print(
                b"%s %s\n\0" as *const u8 as *const gchar,
                prgname,
                (*(*application).priv_0).version,
            );
        } else {
            g_print(
                b"%s\n\0" as *const u8 as *const gchar,
                (*(*application).priv_0).version,
            );
        }
        *exit_status = EXIT_SUCCESS;
        return TRUE;
    }
    g_signal_emit(
        application as gpointer,
        safe_c2rust_g_application_signals
            [SIGNAL_HANDLE_LOCAL_OPTIONS as ::core::ffi::c_int as usize],
        0 as GQuark,
        options,
        exit_status,
    );
    if *exit_status >= 0 as ::core::ffi::c_int {
        g_variant_dict_unref(options);
        return TRUE;
    }
    if safe_c2rust_g_application_register(
        application,
        ::core::ptr::null_mut::<GCancellable>(),
        &raw mut error,
    ) == 0
    {
        g_printerr(
            b"Failed to register: %s\n\0" as *const u8 as *const gchar,
            (*error).message,
        );
        g_variant_dict_unref(options);
        g_error_free(error);
        *exit_status = 1 as ::core::ffi::c_int;
        return TRUE;
    }
    n_args = g_strv_length(*arguments) as gint;
    if (*(*application).priv_0).flags as ::core::ffi::c_uint
        & G_APPLICATION_IS_SERVICE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        *exit_status = (n_args > 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if *exit_status != 0 {
            g_printerr(
                b"GApplication service mode takes no arguments.\n\0" as *const u8 as *const gchar,
            );
            (*(*application).priv_0).flags =
                ::core::mem::transmute::<::core::ffi::c_uint, GApplicationFlags>(
                    (*(*application).priv_0).flags as ::core::ffi::c_uint
                        & !(G_APPLICATION_IS_SERVICE as ::core::ffi::c_int) as ::core::ffi::c_uint,
                );
            *exit_status = 1 as ::core::ffi::c_int;
        } else {
            *exit_status = 0 as ::core::ffi::c_int;
        }
    } else if (*(*application).priv_0).flags as ::core::ffi::c_uint
        & G_APPLICATION_HANDLES_COMMAND_LINE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        safe_c2rust_g_application_call_command_line(
            application,
            *arguments as *mut *const gchar,
            g_variant_dict_end(options),
            exit_status as *mut gint,
        );
    } else if n_args <= 1 as ::core::ffi::c_int {
        safe_c2rust_g_application_activate(application);
        *exit_status = 0 as ::core::ffi::c_int;
    } else if !((*(*application).priv_0).flags as ::core::ffi::c_uint)
        & G_APPLICATION_HANDLES_OPEN as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"This application can not open files.\0" as *const u8 as *const gchar,
        );
        *exit_status = 1 as ::core::ffi::c_int;
    } else {
        let mut files: *mut *mut GFile = ::core::ptr::null_mut::<*mut GFile>();
        let mut n_files: gint = 0;
        let mut i: gint = 0;
        n_files = (n_args as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint;
        files = ({
            let mut __n: gsize = n_files as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut GFile>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut GFile;
        i = 0 as ::core::ffi::c_int as gint;
        while i < n_files {
            let ref mut fresh0 = *files.offset(i as isize);
            *fresh0 = g_file_new_for_commandline_arg(
                *(*arguments).offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize),
            );
            i += 1;
        }
        safe_c2rust_g_application_open(
            application,
            files,
            n_files,
            b"\0" as *const u8 as *const gchar,
        );
        i = 0 as ::core::ffi::c_int as gint;
        while i < n_files {
            g_object_unref(*files.offset(i as isize) as gpointer);
            i += 1;
        }
        g_free(files as gpointer);
        *exit_status = 0 as ::core::ffi::c_int;
    }
    g_variant_dict_unref(options);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_application_real_add_platform_data(
    mut application: *mut GApplication,
    mut builder: *mut GVariantBuilder,
) {
}
unsafe extern "C" fn safe_c2rust_g_application_real_dbus_register(
    mut application: *mut GApplication,
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_application_real_dbus_unregister(
    mut application: *mut GApplication,
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
) {
}
unsafe extern "C" fn safe_c2rust_g_application_real_name_lost(
    mut application: *mut GApplication,
) -> gboolean {
    safe_c2rust_g_application_quit(application);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_application_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut application: *mut GApplication =
        object as *mut ::core::ffi::c_void as *mut GApplication;
    match prop_id {
        1 => {
            safe_c2rust_g_application_set_application_id(application, g_value_get_string(value));
        }
        2 => {
            safe_c2rust_g_application_set_version(application, g_value_get_string(value));
        }
        3 => {
            safe_c2rust_g_application_set_flags(
                application,
                g_value_get_flags(value) as GApplicationFlags,
            );
        }
        4 => {
            safe_c2rust_g_application_set_resource_base_path(
                application,
                g_value_get_string(value),
            );
        }
        7 => {
            safe_c2rust_g_application_set_inactivity_timeout(application, g_value_get_uint(value));
        }
        8 => {
            let mut _pp: *mut *mut GActionGroup = &raw mut (*(*application).priv_0).actions;
            let mut _ptr: *mut GActionGroup = *_pp;
            *_pp = ::core::ptr::null_mut::<GActionGroup>();
            if !_ptr.is_null() {
                g_object_unref(_ptr as gpointer);
            }
            (*(*application).priv_0).actions = g_value_dup_object(value) as *mut GActionGroup;
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplication.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1295 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_set_action_group(
    mut application: *mut GApplication,
    mut action_group: *mut GActionGroup,
) {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() == 0 {
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
            b"!application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*(*application).priv_0).actions.is_null() {
        g_object_unref((*(*application).priv_0).actions as gpointer);
    }
    (*(*application).priv_0).actions = action_group;
    if !(*(*application).priv_0).actions.is_null() {
        g_object_ref((*(*application).priv_0).actions as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_application_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut application: *mut GApplication =
        object as *mut ::core::ffi::c_void as *mut GApplication;
    match prop_id {
        1 => {
            g_value_set_string(
                value,
                safe_c2rust_g_application_get_application_id(application),
            );
        }
        2 => {
            g_value_set_string(value, safe_c2rust_g_application_get_version(application));
        }
        3 => {
            g_value_set_flags(
                value,
                safe_c2rust_g_application_get_flags(application) as guint,
            );
        }
        4 => {
            g_value_set_string(
                value,
                safe_c2rust_g_application_get_resource_base_path(application),
            );
        }
        5 => {
            g_value_set_boolean(
                value,
                safe_c2rust_g_application_get_is_registered(application),
            );
        }
        6 => {
            g_value_set_boolean(value, safe_c2rust_g_application_get_is_remote(application));
        }
        7 => {
            g_value_set_uint(
                value,
                safe_c2rust_g_application_get_inactivity_timeout(application),
            );
        }
        9 => {
            g_value_set_boolean(value, safe_c2rust_g_application_get_is_busy(application));
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplication.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1380 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_application_constructed(mut object: *mut GObject) {
    let mut application: *mut GApplication =
        object as *mut ::core::ffi::c_void as *mut GApplication;
    if safe_c2rust_g_application_get_default().is_null() {
        safe_c2rust_g_application_set_default(application);
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).resource_path.is_null() {
            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_24
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gapplication.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1393 as ::core::ffi::c_int,
            G_STRFUNC,
            b"application->priv->resource_path == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if !(*(*application).priv_0).id.is_null() {
        let mut i: gint = 0;
        (*(*application).priv_0).resource_path = g_strconcat(
            b"/\0" as *const u8 as *const gchar,
            (*(*application).priv_0).id,
            NULL_0,
        );
        i = 1 as ::core::ffi::c_int as gint;
        while *(*(*application).priv_0).resource_path.offset(i as isize) != 0 {
            if *(*(*application).priv_0).resource_path.offset(i as isize) as ::core::ffi::c_int
                == '.' as i32
            {
                *(*(*application).priv_0).resource_path.offset(i as isize) = '/' as i32 as gchar;
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_application_dispose(mut object: *mut GObject) {
    let mut application: *mut GApplication =
        object as *mut ::core::ffi::c_void as *mut GApplication;
    if !(*(*application).priv_0).impl_0.is_null()
        && (*((*(application as *mut GTypeInstance)).g_class as *mut GApplicationClass))
            .dbus_unregister
            != Some(
                safe_c2rust_g_application_real_dbus_unregister
                    as unsafe extern "C" fn(
                        *mut GApplication,
                        *mut GDBusConnection,
                        *const gchar,
                    ) -> (),
            )
    {
        static mut safe_c2rust_warned: gboolean = 0;
        if safe_c2rust_warned == 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Your application did not unregister from D-Bus before destruction. Consider using g_application_run().\0"
                    as *const u8 as *const gchar,
            );
        }
        safe_c2rust_warned = TRUE as gboolean;
    }
    (*(safe_c2rust_g_application_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_application_finalize(mut object: *mut GObject) {
    let mut application: *mut GApplication =
        object as *mut ::core::ffi::c_void as *mut GApplication;
    if (*(*application).priv_0).inactivity_timeout_id != 0 {
        g_source_remove((*(*application).priv_0).inactivity_timeout_id);
    }
    g_slist_free_full(
        (*(*application).priv_0).option_groups,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GOptionGroup) -> ()>,
            GDestroyNotify,
        >(Some(
            g_option_group_unref as unsafe extern "C" fn(*mut GOptionGroup) -> (),
        )),
    );
    if !(*(*application).priv_0).main_options.is_null() {
        g_option_group_unref((*(*application).priv_0).main_options);
    }
    if !(*(*application).priv_0).packed_options.is_null() {
        g_hash_table_unref((*(*application).priv_0).packed_options);
    }
    g_free((*(*application).priv_0).parameter_string as gpointer);
    g_free((*(*application).priv_0).summary as gpointer);
    g_free((*(*application).priv_0).description as gpointer);
    g_free((*(*application).priv_0).version as gpointer);
    g_slist_free_full(
        (*(*application).priv_0).option_strings,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    if !(*(*application).priv_0).impl_0.is_null() {
        g_application_impl_destroy((*(*application).priv_0).impl_0);
    }
    g_free((*(*application).priv_0).id as gpointer);
    if safe_c2rust_g_application_get_default() == application {
        safe_c2rust_g_application_set_default(::core::ptr::null_mut::<GApplication>());
    }
    if !(*(*application).priv_0).actions.is_null() {
        g_object_unref((*(*application).priv_0).actions as gpointer);
    }
    let mut _pp: *mut *mut GRemoteActionGroup = &raw mut (*(*application).priv_0).remote_actions;
    let mut _ptr: *mut GRemoteActionGroup = *_pp;
    *_pp = ::core::ptr::null_mut::<GRemoteActionGroup>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    if !(*(*application).priv_0).notifications.is_null() {
        g_object_unref((*(*application).priv_0).notifications as gpointer);
    }
    g_free((*(*application).priv_0).resource_path as gpointer);
    (*(safe_c2rust_g_application_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_application_init(mut application: *mut GApplication) {
    (*application).priv_0 =
        safe_c2rust_g_application_get_instance_private(application) as *mut GApplicationPrivate;
    (*(*application).priv_0).actions = safe_c2rust_g_application_exported_actions_new(application);
    g_signal_connect_data(
        (*(*application).priv_0).actions as gpointer,
        b"action-added\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> ()>,
            GCallback,
        >(Some(
            g_action_group_action_added
                as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> (),
        )),
        application as gpointer,
        None,
        G_CONNECT_SWAPPED,
    );
    g_signal_connect_data(
        (*(*application).priv_0).actions as gpointer,
        b"action-enabled-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, gboolean) -> ()>,
            GCallback,
        >(Some(
            g_action_group_action_enabled_changed
                as unsafe extern "C" fn(*mut GActionGroup, *const gchar, gboolean) -> (),
        )),
        application as gpointer,
        None,
        G_CONNECT_SWAPPED,
    );
    g_signal_connect_data(
        (*(*application).priv_0).actions as gpointer,
        b"action-state-changed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>,
            GCallback,
        >(Some(
            g_action_group_action_state_changed
                as unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> (),
        )),
        application as gpointer,
        None,
        G_CONNECT_SWAPPED,
    );
    g_signal_connect_data(
        (*(*application).priv_0).actions as gpointer,
        b"action-removed\0" as *const u8 as *const gchar,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> ()>,
            GCallback,
        >(Some(
            g_action_group_action_removed
                as unsafe extern "C" fn(*mut GActionGroup, *const gchar) -> (),
        )),
        application as gpointer,
        None,
        G_CONNECT_SWAPPED,
    );
}
unsafe extern "C" fn safe_c2rust_g_application_handle_local_options_accumulator(
    mut ihint: *mut GSignalInvocationHint,
    mut return_accu: *mut GValue,
    mut handler_return: *const GValue,
    mut dummy: gpointer,
) -> gboolean {
    let mut value: gint = 0;
    value = g_value_get_int(handler_return);
    g_value_set_int(return_accu, value);
    return (value < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_application_class_init(mut class: *mut GApplicationClass) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).constructed =
        Some(safe_c2rust_g_application_constructed as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).dispose =
        Some(safe_c2rust_g_application_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).finalize =
        Some(safe_c2rust_g_application_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).get_property = Some(
        safe_c2rust_g_application_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).set_property = Some(
        safe_c2rust_g_application_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*class).before_emit = Some(
        safe_c2rust_g_application_real_before_emit
            as unsafe extern "C" fn(*mut GApplication, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GApplication, *mut GVariant) -> ()>;
    (*class).after_emit = Some(
        safe_c2rust_g_application_real_after_emit
            as unsafe extern "C" fn(*mut GApplication, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GApplication, *mut GVariant) -> ()>;
    (*class).startup = Some(
        safe_c2rust_g_application_real_startup as unsafe extern "C" fn(*mut GApplication) -> (),
    ) as Option<unsafe extern "C" fn(*mut GApplication) -> ()>;
    (*class).shutdown = Some(
        safe_c2rust_g_application_real_shutdown as unsafe extern "C" fn(*mut GApplication) -> (),
    ) as Option<unsafe extern "C" fn(*mut GApplication) -> ()>;
    (*class).activate = Some(
        safe_c2rust_g_application_real_activate as unsafe extern "C" fn(*mut GApplication) -> (),
    ) as Option<unsafe extern "C" fn(*mut GApplication) -> ()>;
    (*class).open = Some(
        safe_c2rust_g_application_real_open
            as unsafe extern "C" fn(*mut GApplication, *mut *mut GFile, gint, *const gchar) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut GApplication, *mut *mut GFile, gint, *const gchar) -> (),
        >;
    (*class).command_line = Some(
        safe_c2rust_g_application_real_command_line
            as unsafe extern "C" fn(
                *mut GApplication,
                *mut GApplicationCommandLine,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GApplication,
                *mut GApplicationCommandLine,
            ) -> ::core::ffi::c_int,
        >;
    (*class).local_command_line = Some(
        safe_c2rust_g_application_real_local_command_line
            as unsafe extern "C" fn(
                *mut GApplication,
                *mut *mut *mut gchar,
                *mut ::core::ffi::c_int,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GApplication,
                *mut *mut *mut gchar,
                *mut ::core::ffi::c_int,
            ) -> gboolean,
        >;
    (*class).handle_local_options = Some(
        safe_c2rust_g_application_real_handle_local_options
            as unsafe extern "C" fn(*mut GApplication, *mut GVariantDict) -> gint,
    )
        as Option<unsafe extern "C" fn(*mut GApplication, *mut GVariantDict) -> gint>;
    (*class).add_platform_data = Some(
        safe_c2rust_g_application_real_add_platform_data
            as unsafe extern "C" fn(*mut GApplication, *mut GVariantBuilder) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GApplication, *mut GVariantBuilder) -> ()>;
    (*class).dbus_register = Some(
        safe_c2rust_g_application_real_dbus_register
            as unsafe extern "C" fn(
                *mut GApplication,
                *mut GDBusConnection,
                *const gchar,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GApplication,
                *mut GDBusConnection,
                *const gchar,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*class).dbus_unregister = Some(
        safe_c2rust_g_application_real_dbus_unregister
            as unsafe extern "C" fn(*mut GApplication, *mut GDBusConnection, *const gchar) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut GApplication, *mut GDBusConnection, *const gchar) -> (),
        >;
    (*class).name_lost = Some(
        safe_c2rust_g_application_real_name_lost
            as unsafe extern "C" fn(*mut GApplication) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GApplication) -> gboolean>;
    g_object_class_install_property(
        object_class,
        PROP_APPLICATION_ID as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"application-id\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_VERSION as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"version\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)
                | G_PARAM_EXPLICIT_NOTIFY as ::core::ffi::c_int) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_FLAGS as ::core::ffi::c_int as guint,
        g_param_spec_flags(
            b"flags\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_application_flags_get_type(),
            G_APPLICATION_DEFAULT_FLAGS as ::core::ffi::c_int as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_RESOURCE_BASE_PATH as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"resource-base-path\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_IS_REGISTERED as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-registered\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_IS_REMOTE as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-remote\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_INACTIVITY_TIMEOUT as ::core::ffi::c_int as guint,
        g_param_spec_uint(
            b"inactivity-timeout\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            0 as guint,
            G_MAXUINT,
            0 as guint,
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_ACTION_GROUP as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"action-group\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_action_group_get_type(),
            (G_PARAM_DEPRECATED as ::core::ffi::c_int
                | G_PARAM_WRITABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_IS_BUSY as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"is-busy\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    safe_c2rust_g_application_signals[SIGNAL_STARTUP as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"startup\0" as *const u8 as *const gchar),
        safe_c2rust_g_application_get_type(),
        G_SIGNAL_RUN_FIRST,
        136 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        None,
        G_TYPE_NONE,
        0 as guint,
    );
    safe_c2rust_g_application_signals[SIGNAL_SHUTDOWN as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"shutdown\0" as *const u8 as *const gchar),
            safe_c2rust_g_application_get_type(),
            G_SIGNAL_RUN_LAST,
            216 as ::core::ffi::c_ulong as glong as guint,
            None,
            NULL_0,
            None,
            G_TYPE_NONE,
            0 as guint,
        );
    safe_c2rust_g_application_signals[SIGNAL_ACTIVATE as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"activate\0" as *const u8 as *const gchar),
            safe_c2rust_g_application_get_type(),
            G_SIGNAL_RUN_LAST,
            144 as ::core::ffi::c_ulong as glong as guint,
            None,
            NULL_0,
            None,
            G_TYPE_NONE,
            0 as guint,
        );
    safe_c2rust_g_application_signals[SIGNAL_OPEN as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"open\0" as *const u8 as *const gchar),
        safe_c2rust_g_application_get_type(),
        G_SIGNAL_RUN_LAST,
        152 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        Some(
            _g_cclosure_marshal_VOID__POINTER_INT_STRING
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
        G_TYPE_POINTER,
        G_TYPE_INT,
        G_TYPE_STRING,
    );
    g_signal_set_va_marshaller(
        safe_c2rust_g_application_signals[SIGNAL_OPEN as ::core::ffi::c_int as usize],
        (*(class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_VOID__POINTER_INT_STRINGv
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
    safe_c2rust_g_application_signals[SIGNAL_COMMAND_LINE as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"command-line\0" as *const u8 as *const gchar),
            safe_c2rust_g_application_get_type(),
            G_SIGNAL_RUN_LAST,
            160 as ::core::ffi::c_ulong as glong as guint,
            Some(
                g_signal_accumulator_first_wins
                    as unsafe extern "C" fn(
                        *mut GSignalInvocationHint,
                        *mut GValue,
                        *const GValue,
                        gpointer,
                    ) -> gboolean,
            ),
            NULL_0,
            Some(
                _g_cclosure_marshal_INT__OBJECT
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ),
            G_TYPE_INT,
            1 as guint,
            g_application_command_line_get_type(),
        );
    g_signal_set_va_marshaller(
        safe_c2rust_g_application_signals[SIGNAL_COMMAND_LINE as ::core::ffi::c_int as usize],
        (*(class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_INT__OBJECTv
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
    safe_c2rust_g_application_signals[SIGNAL_HANDLE_LOCAL_OPTIONS as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"handle-local-options\0" as *const u8 as *const gchar),
            safe_c2rust_g_application_get_type(),
            G_SIGNAL_RUN_LAST,
            240 as ::core::ffi::c_ulong as glong as guint,
            Some(
                safe_c2rust_g_application_handle_local_options_accumulator
                    as unsafe extern "C" fn(
                        *mut GSignalInvocationHint,
                        *mut GValue,
                        *const GValue,
                        gpointer,
                    ) -> gboolean,
            ),
            NULL_0,
            Some(
                _g_cclosure_marshal_INT__BOXED
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ),
            G_TYPE_INT,
            1 as guint,
            g_variant_dict_get_type(),
        );
    g_signal_set_va_marshaller(
        safe_c2rust_g_application_signals
            [SIGNAL_HANDLE_LOCAL_OPTIONS as ::core::ffi::c_int as usize],
        (*(class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_INT__BOXEDv
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
    safe_c2rust_g_application_signals[SIGNAL_NAME_LOST as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"name-lost\0" as *const u8 as *const gchar),
            safe_c2rust_g_application_get_type(),
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
            NULL_0,
            Some(
                _g_cclosure_marshal_BOOLEAN__VOID
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
            0 as guint,
        );
    g_signal_set_va_marshaller(
        safe_c2rust_g_application_signals[SIGNAL_NAME_LOST as ::core::ffi::c_int as usize],
        (*(class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_BOOLEAN__VOIDv
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_id_is_valid(
    mut application_id: *const gchar,
) -> gboolean {
    return (g_dbus_is_name(application_id) != 0 && g_dbus_is_unique_name(application_id) == 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_new(
    mut application_id: *const gchar,
    mut flags: GApplicationFlags,
) -> *mut GApplication {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if application_id.is_null() || safe_c2rust_g_application_id_is_valid(application_id) != 0 {
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
            b"application_id == NULL || g_application_id_is_valid (application_id)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GApplication>();
    }
    return g_object_new(
        safe_c2rust_g_application_get_type(),
        b"application-id\0" as *const u8 as *const gchar,
        application_id,
        b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        flags as ::core::ffi::c_uint,
        NULL_0,
    ) as *mut GApplication;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_application_id(
    mut application: *mut GApplication,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*application).priv_0).id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_set_application_id(
    mut application: *mut GApplication,
    mut application_id: *const gchar,
) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_strcmp0(
        (*(*application).priv_0).id,
        application_id as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        if ({
            let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
            if application_id.is_null()
                || safe_c2rust_g_application_id_is_valid(application_id) != 0
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
                b"application_id == NULL || g_application_id_is_valid (application_id)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if ({
            let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
            if (*(*application).priv_0).is_registered() == 0 {
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
                b"!application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        g_free((*(*application).priv_0).id as gpointer);
        (*(*application).priv_0).id =
            safe_c2rust_g_strdup_inline(application_id as *const ::core::ffi::c_char) as *mut gchar;
        g_object_notify(
            application as *mut ::core::ffi::c_void as *mut GObject,
            b"application-id\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_version(
    mut application: *mut GApplication,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*application).priv_0).version;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_set_version(
    mut application: *mut GApplication,
    mut version: *const gchar,
) {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !version.is_null() {
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
            b"version != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() == 0 {
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
            b"!application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if safe_c2rust_g_set_str(
        &raw mut (*(*application).priv_0).version,
        version as *const ::core::ffi::c_char,
    ) != 0
    {
        g_object_notify(
            application as *mut ::core::ffi::c_void as *mut GObject,
            b"version\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_flags(
    mut application: *mut GApplication,
) -> GApplicationFlags {
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_APPLICATION_FLAGS_NONE;
    }
    return (*(*application).priv_0).flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_set_flags(
    mut application: *mut GApplication,
    mut flags: GApplicationFlags,
) {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*application).priv_0).flags as ::core::ffi::c_uint != flags as ::core::ffi::c_uint {
        if ({
            let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
            if (*(*application).priv_0).is_registered() == 0 {
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
                b"!application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        (*(*application).priv_0).flags = flags;
        g_object_notify(
            application as *mut ::core::ffi::c_void as *mut GObject,
            b"flags\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_resource_base_path(
    mut application: *mut GApplication,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return (*(*application).priv_0).resource_path;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_set_resource_base_path(
    mut application: *mut GApplication,
    mut resource_path: *const gchar,
) {
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if resource_path.is_null()
            || (if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char =
                        resource_path as *const ::core::ffi::c_char;
                    let __prefix: *const ::core::ffi::c_char =
                        b"/\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = 0 as gboolean;
                    if ({
                        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_39
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix_len,
                            ) == 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as gboolean;
                        }
                    }
                    __result
                })
            } else {
                g_str_has_prefix(resource_path, b"/\0" as *const u8 as *const gchar)
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
            b"resource_path == NULL || g_str_has_prefix (resource_path, \"/\")\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_strcmp0(
        (*(*application).priv_0).resource_path,
        resource_path as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        g_free((*(*application).priv_0).resource_path as gpointer);
        (*(*application).priv_0).resource_path =
            safe_c2rust_g_strdup_inline(resource_path as *const ::core::ffi::c_char) as *mut gchar;
        g_object_notify(
            application as *mut ::core::ffi::c_void as *mut GObject,
            b"resource-base-path\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_inactivity_timeout(
    mut application: *mut GApplication,
) -> guint {
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*(*application).priv_0).inactivity_timeout;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_set_inactivity_timeout(
    mut application: *mut GApplication,
    mut inactivity_timeout: guint,
) {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*application).priv_0).inactivity_timeout != inactivity_timeout {
        (*(*application).priv_0).inactivity_timeout = inactivity_timeout;
        g_object_notify(
            application as *mut ::core::ffi::c_void as *mut GObject,
            b"inactivity-timeout\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_is_registered(
    mut application: *mut GApplication,
) -> gboolean {
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*application).priv_0).is_registered() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_is_remote(
    mut application: *mut GApplication,
) -> gboolean {
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() != 0 {
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
            b"application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*(*application).priv_0).is_remote() as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_dbus_connection(
    mut application: *mut GApplication,
) -> *mut GDBusConnection {
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() != 0 {
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
            b"application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    return g_application_impl_get_dbus_connection((*(*application).priv_0).impl_0);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_dbus_object_path(
    mut application: *mut GApplication,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() != 0 {
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
            b"application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return g_application_impl_get_dbus_object_path((*(*application).priv_0).impl_0);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_register(
    mut application: *mut GApplication,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*(*application).priv_0).is_registered() == 0 {
        if (*(*application).priv_0).id.is_null() {
            (*(*application).priv_0).flags =
                ::core::mem::transmute::<::core::ffi::c_uint, GApplicationFlags>(
                    (*(*application).priv_0).flags as ::core::ffi::c_uint
                        | G_APPLICATION_NON_UNIQUE as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
        }
        (*(*application).priv_0).impl_0 = g_application_impl_register(
            application,
            (*(*application).priv_0).id,
            (*(*application).priv_0).flags,
            (*(*application).priv_0).actions,
            &raw mut (*(*application).priv_0).remote_actions,
            cancellable,
            error,
        );
        if (*(*application).priv_0).impl_0.is_null() {
            return FALSE;
        }
        (*(*application).priv_0).set_is_remote(
            ((*(*application).priv_0).remote_actions != NULL_0 as *mut GRemoteActionGroup)
                as ::core::ffi::c_int as guint as guint,
        );
        (*(*application).priv_0).set_is_registered(TRUE as guint as guint);
        g_object_notify(
            application as *mut ::core::ffi::c_void as *mut GObject,
            b"is-registered\0" as *const u8 as *const gchar,
        );
        if (*(*application).priv_0).is_remote() == 0 {
            g_signal_emit(
                application as gpointer,
                safe_c2rust_g_application_signals[SIGNAL_STARTUP as ::core::ffi::c_int as usize],
                0 as GQuark,
            );
            if (*(*application).priv_0).did_startup() == 0 {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"GApplication subclass '%s' failed to chain up on ::startup (from start of override function)\0"
                        as *const u8 as *const gchar,
                    g_type_name((*(*(application as *mut GTypeInstance)).g_class).g_type),
                );
            }
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_hold(mut application: *mut GApplication) {
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*application).priv_0).inactivity_timeout_id != 0 {
        g_source_remove((*(*application).priv_0).inactivity_timeout_id);
        (*(*application).priv_0).inactivity_timeout_id = 0 as guint;
    }
    (*(*application).priv_0).use_count = (*(*application).priv_0).use_count.wrapping_add(1);
}
unsafe extern "C" fn safe_c2rust_inactivity_timeout_expired(mut data: gpointer) -> gboolean {
    let mut application: *mut GApplication = data as *mut GApplication;
    (*(*application).priv_0).inactivity_timeout_id = 0 as guint;
    return G_SOURCE_REMOVE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_release(mut application: *mut GApplication) {
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).use_count > 0 as guint {
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
            b"application->priv->use_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*application).priv_0).use_count = (*(*application).priv_0).use_count.wrapping_sub(1);
    if (*(*application).priv_0).use_count == 0 as guint
        && (*(*application).priv_0).inactivity_timeout != 0
    {
        (*(*application).priv_0).inactivity_timeout_id = g_timeout_add(
            (*(*application).priv_0).inactivity_timeout,
            Some(
                safe_c2rust_inactivity_timeout_expired
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            application as gpointer,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_activate(mut application: *mut GApplication) {
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() != 0 {
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
            b"application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*application).priv_0).is_remote() != 0 {
        g_application_impl_activate(
            (*(*application).priv_0).impl_0,
            safe_c2rust_get_platform_data(application, ::core::ptr::null_mut::<GVariant>()),
        );
    } else {
        g_signal_emit(
            application as gpointer,
            safe_c2rust_g_application_signals[SIGNAL_ACTIVATE as ::core::ffi::c_int as usize],
            0 as GQuark,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_open(
    mut application: *mut GApplication,
    mut files: *mut *mut GFile,
    mut n_files: gint,
    mut hint: *const gchar,
) {
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).flags as ::core::ffi::c_uint
            & G_APPLICATION_HANDLES_OPEN as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
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
            b"application->priv->flags & G_APPLICATION_HANDLES_OPEN\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() != 0 {
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
            b"application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*application).priv_0).is_remote() != 0 {
        g_application_impl_open(
            (*(*application).priv_0).impl_0,
            files,
            n_files,
            hint,
            safe_c2rust_get_platform_data(application, ::core::ptr::null_mut::<GVariant>()),
        );
    } else {
        g_signal_emit(
            application as gpointer,
            safe_c2rust_g_application_signals[SIGNAL_OPEN as ::core::ffi::c_int as usize],
            0 as GQuark,
            files,
            n_files,
            hint,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_run(
    mut application: *mut GApplication,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut arguments: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut status: ::core::ffi::c_int = 0;
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    let mut acquired_context: gboolean = 0;
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if argc == 0 as ::core::ffi::c_int || !argv.is_null() {
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
            b"argc == 0 || argv != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).must_quit_now() == 0 {
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
            b"!application->priv->must_quit_now\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut i: gint = 0;
    arguments = ({
        let mut __n: gsize = (argc + 1 as ::core::ffi::c_int) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut *mut gchar;
    i = 0 as ::core::ffi::c_int as gint;
    while i < argc {
        let ref mut fresh1 = *arguments.offset(i as isize);
        *fresh1 = safe_c2rust_g_strdup_inline(*argv.offset(i as isize)) as *mut gchar;
        i += 1;
    }
    let ref mut fresh2 = *arguments.offset(i as isize);
    *fresh2 = ::core::ptr::null_mut::<gchar>();
    if g_get_prgname().is_null() && argc > 0 as ::core::ffi::c_int {
        let mut prgname: *mut gchar = ::core::ptr::null_mut::<gchar>();
        prgname = g_path_get_basename(*argv.offset(0 as ::core::ffi::c_int as isize));
        (*glib__private__())
            .g_set_prgname_once
            .expect("non-null function pointer")(prgname);
        g_free(prgname as gpointer);
    }
    context = g_main_context_default();
    acquired_context = g_main_context_acquire(context);
    if acquired_context == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_application_run() cannot acquire the default main context because it is already acquired by another thread!\0"
                as *const u8 as *const gchar,
        );
        g_strfreev(arguments);
        return 1 as ::core::ffi::c_int;
    }
    if (*((*(application as *mut GTypeInstance)).g_class as *mut GApplicationClass))
        .local_command_line
        .expect("non-null function pointer")(application, &raw mut arguments, &raw mut status)
        == 0
    {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        if safe_c2rust_g_application_register(
            application,
            ::core::ptr::null_mut::<GCancellable>(),
            &raw mut error,
        ) == 0
        {
            g_printerr(
                b"Failed to register: %s\n\0" as *const u8 as *const gchar,
                (*error).message,
            );
            g_error_free(error);
            return 1 as ::core::ffi::c_int;
        }
        safe_c2rust_g_application_call_command_line(
            application,
            arguments as *mut *const gchar,
            ::core::ptr::null_mut::<GVariant>(),
            &raw mut status,
        );
    }
    g_strfreev(arguments);
    if (*(*application).priv_0).flags as ::core::ffi::c_uint
        & G_APPLICATION_IS_SERVICE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && (*(*application).priv_0).is_registered() as ::core::ffi::c_int != 0
        && (*(*application).priv_0).use_count == 0
        && (*(*application).priv_0).inactivity_timeout_id == 0
    {
        (*(*application).priv_0).inactivity_timeout_id = g_timeout_add(
            10000 as guint,
            Some(
                safe_c2rust_inactivity_timeout_expired
                    as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
            application as gpointer,
        );
    }
    while (*(*application).priv_0).use_count != 0
        || (*(*application).priv_0).inactivity_timeout_id != 0
    {
        if (*(*application).priv_0).must_quit_now() != 0 {
            break;
        }
        g_main_context_iteration(context, TRUE);
        status = 0 as ::core::ffi::c_int;
    }
    if (*(*application).priv_0).is_registered() as ::core::ffi::c_int != 0
        && (*(*application).priv_0).is_remote() == 0
    {
        g_signal_emit(
            application as gpointer,
            safe_c2rust_g_application_signals[SIGNAL_SHUTDOWN as ::core::ffi::c_int as usize],
            0 as GQuark,
        );
        if (*(*application).priv_0).did_shutdown() == 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"GApplication subclass '%s' failed to chain up on ::shutdown (from end of override function)\0"
                    as *const u8 as *const gchar,
                g_type_name((*(*(application as *mut GTypeInstance)).g_class).g_type),
            );
        }
    }
    if !(*(*application).priv_0).impl_0.is_null() {
        if (*(*application).priv_0).is_registered() != 0 {
            (*(*application).priv_0).set_is_registered(FALSE as guint as guint);
            g_object_notify(
                application as *mut ::core::ffi::c_void as *mut GObject,
                b"is-registered\0" as *const u8 as *const gchar,
            );
        }
        g_application_impl_flush((*(*application).priv_0).impl_0);
        g_application_impl_destroy((*(*application).priv_0).impl_0);
        (*(*application).priv_0).impl_0 = ::core::ptr::null_mut::<GApplicationImpl>();
    }
    g_settings_sync();
    if (*(*application).priv_0).must_quit_now() == 0 {
        while g_main_context_iteration(context, FALSE) != 0 {}
    }
    g_main_context_release(context);
    return status;
}
unsafe extern "C" fn safe_c2rust_g_application_list_actions(
    mut action_group: *mut GActionGroup,
) -> *mut *mut gchar {
    let mut application: *mut GApplication =
        action_group as *mut ::core::ffi::c_void as *mut GApplication;
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() != 0 {
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
            b"application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if !(*(*application).priv_0).remote_actions.is_null() {
        return g_action_group_list_actions(
            (*(*application).priv_0).remote_actions as *mut ::core::ffi::c_void
                as *mut GActionGroup,
        );
    } else if !(*(*application).priv_0).actions.is_null() {
        return g_action_group_list_actions((*(*application).priv_0).actions);
    } else {
        return ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut gchar>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut *mut gchar;
    };
}
unsafe extern "C" fn safe_c2rust_g_application_query_action(
    mut group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut enabled: *mut gboolean,
    mut parameter_type: *mut *const GVariantType,
    mut state_type: *mut *const GVariantType,
    mut state_hint: *mut *mut GVariant,
    mut state: *mut *mut GVariant,
) -> gboolean {
    let mut application: *mut GApplication = group as *mut ::core::ffi::c_void as *mut GApplication;
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() != 0 {
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
            b"application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !(*(*application).priv_0).remote_actions.is_null() {
        return g_action_group_query_action(
            (*(*application).priv_0).remote_actions as *mut ::core::ffi::c_void
                as *mut GActionGroup,
            action_name,
            enabled,
            parameter_type,
            state_type,
            state_hint,
            state,
        );
    }
    if !(*(*application).priv_0).actions.is_null() {
        return g_action_group_query_action(
            (*(*application).priv_0).actions,
            action_name,
            enabled,
            parameter_type,
            state_type,
            state_hint,
            state,
        );
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_application_change_action_state(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut value: *mut GVariant,
) {
    let mut application: *mut GApplication =
        action_group as *mut ::core::ffi::c_void as *mut GApplication;
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_remote() as ::core::ffi::c_int != 0
            || !(*(*application).priv_0).actions.is_null()
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
            b"application->priv->is_remote || application->priv->actions != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() != 0 {
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
            b"application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*(*application).priv_0).remote_actions.is_null() {
        g_remote_action_group_change_action_state_full(
            (*(*application).priv_0).remote_actions,
            action_name,
            value,
            safe_c2rust_get_platform_data(application, ::core::ptr::null_mut::<GVariant>()),
        );
    } else {
        g_action_group_change_action_state((*(*application).priv_0).actions, action_name, value);
    };
}
unsafe extern "C" fn safe_c2rust_g_application_activate_action(
    mut action_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut parameter: *mut GVariant,
) {
    let mut application: *mut GApplication =
        action_group as *mut ::core::ffi::c_void as *mut GApplication;
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_remote() as ::core::ffi::c_int != 0
            || !(*(*application).priv_0).actions.is_null()
        {
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
            b"application->priv->is_remote || application->priv->actions != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() != 0 {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*(*application).priv_0).remote_actions.is_null() {
        g_remote_action_group_activate_action_full(
            (*(*application).priv_0).remote_actions,
            action_name,
            parameter,
            safe_c2rust_get_platform_data(application, ::core::ptr::null_mut::<GVariant>()),
        );
    } else {
        g_action_group_activate_action((*(*application).priv_0).actions, action_name, parameter);
    };
}
unsafe extern "C" fn safe_c2rust_g_application_lookup_action(
    mut action_map: *mut GActionMap,
    mut action_name: *const gchar,
) -> *mut GAction {
    let mut application: *mut GApplication =
        action_map as *mut ::core::ffi::c_void as *mut GApplication;
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance =
                (*(*application).priv_0).actions as *mut GTypeInstance;
            let mut __t: GType = g_action_map_get_type();
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
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ACTION_MAP (application->priv->actions)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAction>();
    }
    return g_action_map_lookup_action(
        (*(*application).priv_0).actions as *mut ::core::ffi::c_void as *mut GActionMap,
        action_name,
    );
}
unsafe extern "C" fn safe_c2rust_g_application_add_action(
    mut action_map: *mut GActionMap,
    mut action: *mut GAction,
) {
    let mut application: *mut GApplication =
        action_map as *mut ::core::ffi::c_void as *mut GApplication;
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance =
                (*(*application).priv_0).actions as *mut GTypeInstance;
            let mut __t: GType = g_action_map_get_type();
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
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ACTION_MAP (application->priv->actions)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_action_map_add_action(
        (*(*application).priv_0).actions as *mut ::core::ffi::c_void as *mut GActionMap,
        action,
    );
}
unsafe extern "C" fn safe_c2rust_g_application_remove_action(
    mut action_map: *mut GActionMap,
    mut action_name: *const gchar,
) {
    let mut application: *mut GApplication =
        action_map as *mut ::core::ffi::c_void as *mut GApplication;
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance =
                (*(*application).priv_0).actions as *mut GTypeInstance;
            let mut __t: GType = g_action_map_get_type();
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
            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_70
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_ACTION_MAP (application->priv->actions)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_action_map_remove_action(
        (*(*application).priv_0).actions as *mut ::core::ffi::c_void as *mut GActionMap,
        action_name,
    );
}
unsafe extern "C" fn safe_c2rust_g_application_action_group_iface_init(
    mut iface: *mut GActionGroupInterface,
) {
    (*iface).list_actions = Some(
        safe_c2rust_g_application_list_actions
            as unsafe extern "C" fn(*mut GActionGroup) -> *mut *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup) -> *mut *mut gchar>;
    (*iface).query_action = Some(
        safe_c2rust_g_application_query_action
            as unsafe extern "C" fn(
                *mut GActionGroup,
                *const gchar,
                *mut gboolean,
                *mut *const GVariantType,
                *mut *const GVariantType,
                *mut *mut GVariant,
                *mut *mut GVariant,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GActionGroup,
                *const gchar,
                *mut gboolean,
                *mut *const GVariantType,
                *mut *const GVariantType,
                *mut *mut GVariant,
                *mut *mut GVariant,
            ) -> gboolean,
        >;
    (*iface).change_action_state = Some(
        safe_c2rust_g_application_change_action_state
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>;
    (*iface).activate_action = Some(
        safe_c2rust_g_application_activate_action
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_application_action_map_iface_init(
    mut iface: *mut GActionMapInterface,
) {
    (*iface).lookup_action = Some(
        safe_c2rust_g_application_lookup_action
            as unsafe extern "C" fn(*mut GActionMap, *const gchar) -> *mut GAction,
    )
        as Option<unsafe extern "C" fn(*mut GActionMap, *const gchar) -> *mut GAction>;
    (*iface).add_action = Some(
        safe_c2rust_g_application_add_action
            as unsafe extern "C" fn(*mut GActionMap, *mut GAction) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GActionMap, *mut GAction) -> ()>;
    (*iface).remove_action = Some(
        safe_c2rust_g_application_remove_action
            as unsafe extern "C" fn(*mut GActionMap, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GActionMap, *const gchar) -> ()>;
}
static mut safe_c2rust_default_app: *mut GApplication =
    ::core::ptr::null::<GApplication>() as *mut GApplication;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_default() -> *mut GApplication {
    return safe_c2rust_default_app;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_set_default(mut application: *mut GApplication) {
    safe_c2rust_default_app = application;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_quit(mut application: *mut GApplication) {
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*application).priv_0).set_must_quit_now(TRUE as guint as guint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_mark_busy(mut application: *mut GApplication) {
    let mut was_busy: gboolean = 0;
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).is_registered() != 0 {
            _g_boolean_var_73 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_73 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_73
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"application->priv->is_registered\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    was_busy = ((*(*application).priv_0).busy_count > 0 as guint) as ::core::ffi::c_int as gboolean;
    (*(*application).priv_0).busy_count = (*(*application).priv_0).busy_count.wrapping_add(1);
    if was_busy == 0 {
        g_application_impl_set_busy_state((*(*application).priv_0).impl_0, TRUE);
        g_object_notify(
            application as *mut ::core::ffi::c_void as *mut GObject,
            b"is-busy\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_unmark_busy(mut application: *mut GApplication) {
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            _g_boolean_var_74 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_74 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_74
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if (*(*application).priv_0).busy_count > 0 as guint {
            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_75
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"application->priv->busy_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*(*application).priv_0).busy_count = (*(*application).priv_0).busy_count.wrapping_sub(1);
    if (*(*application).priv_0).busy_count == 0 as guint {
        g_application_impl_set_busy_state((*(*application).priv_0).impl_0, FALSE);
        g_object_notify(
            application as *mut ::core::ffi::c_void as *mut GObject,
            b"is-busy\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_get_is_busy(
    mut application: *mut GApplication,
) -> gboolean {
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_76
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return ((*(*application).priv_0).busy_count > 0 as guint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_send_notification(
    mut application: *mut GApplication,
    mut id: *const gchar,
    mut notification: *mut GNotification,
) {
    let mut generated_id: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            _g_boolean_var_77 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_77 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_77
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = notification as *mut GTypeInstance;
            let mut __t: GType = g_notification_get_type();
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
            _g_boolean_var_78 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_78 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_78
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_NOTIFICATION (notification)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if safe_c2rust_g_application_get_is_registered(application) != 0 {
            _g_boolean_var_79 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_79 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_79
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_application_get_is_registered (application)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if safe_c2rust_g_application_get_is_remote(application) == 0 {
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!g_application_get_is_remote (application)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if !safe_c2rust_g_application_get_application_id(application).is_null() {
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_application_get_application_id (application) != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*application).priv_0).notifications.is_null() {
        (*(*application).priv_0).notifications = g_notification_backend_new_default(application);
    }
    if id.is_null() {
        generated_id = g_dbus_generate_guid();
        id = generated_id;
    }
    g_notification_backend_send_notification(
        (*(*application).priv_0).notifications,
        id,
        notification,
    );
    g_free(generated_id as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_withdraw_notification(
    mut application: *mut GApplication,
    mut id: *const gchar,
) {
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if !id.is_null() {
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"id != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*application).priv_0).notifications.is_null() {
        (*(*application).priv_0).notifications = g_notification_backend_new_default(application);
    }
    g_notification_backend_withdraw_notification((*(*application).priv_0).notifications, id);
}
unsafe extern "C" fn safe_c2rust_g_application_busy_binding_destroy(
    mut data: gpointer,
    mut closure: *mut GClosure,
) {
    let mut binding: *mut GApplicationBusyBinding = data as *mut GApplicationBusyBinding;
    if (*binding).is_busy != 0 {
        safe_c2rust_g_application_unmark_busy((*binding).app);
    }
    g_object_unref((*binding).app as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GApplicationBusyBinding>() as gsize,
        binding as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_application_notify_busy_binding(
    mut object: *mut GObject,
    mut pspec: *mut GParamSpec,
    mut user_data: gpointer,
) {
    let mut binding: *mut GApplicationBusyBinding = user_data as *mut GApplicationBusyBinding;
    let mut is_busy: gboolean = 0;
    g_object_get(object as gpointer, (*pspec).name, &raw mut is_busy, NULL_0);
    if is_busy != 0 && (*binding).is_busy == 0 {
        safe_c2rust_g_application_mark_busy((*binding).app);
    } else if is_busy == 0 && (*binding).is_busy != 0 {
        safe_c2rust_g_application_unmark_busy((*binding).app);
    }
    (*binding).is_busy = is_busy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_bind_busy_property(
    mut application: *mut GApplication,
    mut object: gpointer,
    mut property: *const gchar,
) {
    let mut notify_id: guint = 0;
    let mut property_quark: GQuark = 0;
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    let mut binding: *mut GApplicationBusyBinding =
        ::core::ptr::null_mut::<GApplicationBusyBinding>();
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            _g_boolean_var_84 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_84 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_84
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if g_type_check_instance_is_fundamentally_a(
            object as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
        {
            _g_boolean_var_85 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_85 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_85
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if !property.is_null() {
            _g_boolean_var_86 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_86 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_86
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    notify_id = g_signal_lookup(b"notify\0" as *const u8 as *const gchar, G_TYPE_OBJECT);
    property_quark = g_quark_from_string(property);
    pspec = g_object_class_find_property(
        (*(object as *mut GTypeInstance)).g_class as *mut GObjectClass,
        property,
    );
    if ({
        let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
        if !pspec.is_null()
            && (*pspec).value_type
                == ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        {
            _g_boolean_var_87 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_87 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_87
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"pspec != NULL && pspec->value_type == G_TYPE_BOOLEAN\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_signal_handler_find(
        object,
        (G_SIGNAL_MATCH_ID as ::core::ffi::c_int
            | G_SIGNAL_MATCH_DETAIL as ::core::ffi::c_int
            | G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int) as GSignalMatchType,
        notify_id,
        property_quark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> ()>,
            gpointer,
        >(Some(
            safe_c2rust_g_application_notify_busy_binding
                as unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> (),
        )),
        NULL_0,
    ) > 0 as gulong
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: '%s' is already bound to the busy state of the application\0" as *const u8
                as *const gchar,
            b"g_application_bind_busy_property\0" as *const u8 as *const ::core::ffi::c_char,
            property,
        );
        return;
    }
    binding = g_slice_alloc(::core::mem::size_of::<GApplicationBusyBinding>() as gsize)
        as *mut GApplicationBusyBinding;
    (*binding).app =
        g_object_ref(application as gpointer) as *mut GApplication as *mut GApplication;
    (*binding).is_busy = FALSE as gboolean;
    closure = g_cclosure_new(
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_g_application_notify_busy_binding
                as unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> (),
        )),
        binding as gpointer,
        Some(
            safe_c2rust_g_application_busy_binding_destroy
                as unsafe extern "C" fn(gpointer, *mut GClosure) -> (),
        ),
    );
    g_signal_connect_closure_by_id(object, notify_id, property_quark, closure, FALSE);
    safe_c2rust_g_application_notify_busy_binding(
        object as *mut GObject,
        pspec,
        binding as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_application_unbind_busy_property(
    mut application: *mut GApplication,
    mut object: gpointer,
    mut property: *const gchar,
) {
    let mut notify_id: guint = 0;
    let mut property_quark: GQuark = 0;
    let mut handler_id: gulong = 0;
    if ({
        let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = application as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_application_get_type();
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
            _g_boolean_var_88 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_88 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_88
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_APPLICATION (application)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
        if g_type_check_instance_is_fundamentally_a(
            object as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
        {
            _g_boolean_var_89 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_89 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_89
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if !property.is_null() {
            _g_boolean_var_90 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_90 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_90
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    notify_id = g_signal_lookup(b"notify\0" as *const u8 as *const gchar, G_TYPE_OBJECT);
    property_quark = g_quark_from_string(property);
    handler_id = g_signal_handler_find(
        object,
        (G_SIGNAL_MATCH_ID as ::core::ffi::c_int
            | G_SIGNAL_MATCH_DETAIL as ::core::ffi::c_int
            | G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int) as GSignalMatchType,
        notify_id,
        property_quark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> ()>,
            gpointer,
        >(Some(
            safe_c2rust_g_application_notify_busy_binding
                as unsafe extern "C" fn(*mut GObject, *mut GParamSpec, gpointer) -> (),
        )),
        NULL_0,
    );
    if handler_id == 0 as gulong {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: '%s' is not bound to the busy state of the application\0" as *const u8
                as *const gchar,
            b"g_application_unbind_busy_property\0" as *const u8 as *const ::core::ffi::c_char,
            property,
        );
        return;
    }
    g_signal_handler_disconnect(object, handler_id);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const GETTEXT_PACKAGE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"glib20\0") };
