use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GSettingsSchemaSource;
    pub type _GSettingsSchema;
    pub type _GAction;
    pub type _GSettingsBackendPrivate;
    pub type _GDelayedSettingsBackendPrivate;
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
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_intern_string(string: *const gchar) -> *const gchar;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_ref_thread_default() -> *mut GMainContext;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_variant_type_peek_string(type_0: *const GVariantType) -> *const gchar;
    fn g_variant_type_dup_string(type_0: *const GVariantType) -> *mut gchar;
    fn g_variant_type_equal(type1: gconstpointer, type2: gconstpointer) -> gboolean;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_take_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type(value: *mut GVariant) -> *const GVariantType;
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new_boolean(value: gboolean) -> *mut GVariant;
    fn g_variant_new_int32(value: gint32) -> *mut GVariant;
    fn g_variant_new_uint32(value: guint32) -> *mut GVariant;
    fn g_variant_new_int64(value: gint64) -> *mut GVariant;
    fn g_variant_new_uint64(value: guint64) -> *mut GVariant;
    fn g_variant_new_double(value: gdouble) -> *mut GVariant;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_strv(strv: *const *const gchar, length: gssize) -> *mut GVariant;
    fn g_variant_get_boolean(value: *mut GVariant) -> gboolean;
    fn g_variant_get_int32(value: *mut GVariant) -> gint32;
    fn g_variant_get_uint32(value: *mut GVariant) -> guint32;
    fn g_variant_get_int64(value: *mut GVariant) -> gint64;
    fn g_variant_get_uint64(value: *mut GVariant) -> guint64;
    fn g_variant_get_double(value: *mut GVariant) -> gdouble;
    fn g_variant_dup_string(value: *mut GVariant, length: *mut gsize) -> *mut gchar;
    fn g_variant_dup_strv(value: *mut GVariant, length: *mut gsize) -> *mut *mut gchar;
    fn g_variant_print(value: *mut GVariant, type_annotate: gboolean) -> *mut gchar;
    fn g_variant_new_va(
        format_string: *const gchar,
        endptr: *mut *const gchar,
        app: *mut ::core::ffi::VaList,
    ) -> *mut GVariant;
    fn g_variant_get_va(
        value: *mut GVariant,
        format_string: *const gchar,
        endptr: *mut *const gchar,
        app: *mut ::core::ffi::VaList,
    );
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
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
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
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
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_signal_handler_is_connected(instance: gpointer, handler_id: gulong) -> gboolean;
    fn g_signal_handlers_disconnect_matched(
        instance: gpointer,
        mask: GSignalMatchType,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        func: gpointer,
        data: gpointer,
    ) -> guint;
    fn g_signal_accumulator_true_handled(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
    ) -> gboolean;
    fn g_value_set_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_dup_boxed(value: *const GValue) -> gpointer;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_class_find_property(
        oclass: *mut GObjectClass,
        property_name: *const gchar,
    ) -> *mut GParamSpec;
    fn g_object_class_override_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        name: *const gchar,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_set(object: gpointer, first_property_name: *const gchar, ...);
    fn g_object_set_property(
        object: *mut GObject,
        property_name: *const gchar,
        value: *const GValue,
    );
    fn g_object_get_property(object: *mut GObject, property_name: *const gchar, value: *mut GValue);
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_set_qdata(object: *mut GObject, quark: GQuark, data: gpointer);
    fn g_object_set_qdata_full(
        object: *mut GObject,
        quark: GQuark,
        data: gpointer,
        destroy: GDestroyNotify,
    );
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
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
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_get_boolean(value: *const GValue) -> gboolean;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_value_take_variant(value: *mut GValue, variant: *mut GVariant);
    fn g_settings_schema_source_get_default() -> *mut GSettingsSchemaSource;
    fn g_settings_schema_source_lookup(
        source: *mut GSettingsSchemaSource,
        schema_id: *const gchar,
        recursive: gboolean,
    ) -> *mut GSettingsSchema;
    fn g_settings_schema_get_type() -> GType;
    fn g_settings_schema_unref(schema: *mut GSettingsSchema);
    fn g_settings_schema_get_id(schema: *mut GSettingsSchema) -> *const gchar;
    fn g_settings_schema_get_path(schema: *mut GSettingsSchema) -> *const gchar;
    fn g_settings_schema_has_key(schema: *mut GSettingsSchema, name: *const gchar) -> gboolean;
    fn g_settings_schema_list_keys(schema: *mut GSettingsSchema) -> *mut *mut gchar;
    fn g_settings_schema_list_children(schema: *mut GSettingsSchema) -> *mut *mut gchar;
    fn g_settings_schema_key_get_default_value(key: *mut GSettingsSchemaKey) -> *mut GVariant;
    fn g_settings_schema_key_get_range(key: *mut GSettingsSchemaKey) -> *mut GVariant;
    fn g_settings_schema_key_range_check(
        key: *mut GSettingsSchemaKey,
        value: *mut GVariant,
    ) -> gboolean;
    fn g_settings_backend_get_type() -> GType;
    fn g_settings_backend_get_default() -> *mut GSettingsBackend;
    fn g_delayed_settings_backend_get_type() -> GType;
    fn g_delayed_settings_backend_new(
        backend: *mut GSettingsBackend,
        owner: gpointer,
        owner_context: *mut GMainContext,
    ) -> *mut GDelayedSettingsBackend;
    fn g_delayed_settings_backend_revert(delayed: *mut GDelayedSettingsBackend);
    fn g_delayed_settings_backend_apply(delayed: *mut GDelayedSettingsBackend);
    fn g_delayed_settings_backend_get_has_unapplied(
        delayed: *mut GDelayedSettingsBackend,
    ) -> gboolean;
    fn g_settings_backend_watch(
        backend: *mut GSettingsBackend,
        vtable: *const GSettingsListenerVTable,
        target: *mut GObject,
        context: *mut GMainContext,
    );
    fn g_settings_backend_unwatch(backend: *mut GSettingsBackend, target: *mut GObject);
    fn g_settings_backend_read(
        backend: *mut GSettingsBackend,
        key: *const gchar,
        expected_type: *const GVariantType,
        default_value: gboolean,
    ) -> *mut GVariant;
    fn g_settings_backend_read_user_value(
        backend: *mut GSettingsBackend,
        key: *const gchar,
        expected_type: *const GVariantType,
    ) -> *mut GVariant;
    fn g_settings_backend_write(
        backend: *mut GSettingsBackend,
        key: *const gchar,
        value: *mut GVariant,
        origin_tag: gpointer,
    ) -> gboolean;
    fn g_settings_backend_reset(
        backend: *mut GSettingsBackend,
        key: *const gchar,
        origin_tag: gpointer,
    );
    fn g_settings_backend_get_writable(
        backend: *mut GSettingsBackend,
        key: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn g_settings_backend_unsubscribe(
        backend: *mut GSettingsBackend,
        name: *const ::core::ffi::c_char,
    );
    fn g_settings_backend_subscribe(
        backend: *mut GSettingsBackend,
        name: *const ::core::ffi::c_char,
    );
    fn g_settings_backend_sync_default();
    fn g_settings_set_mapping(
        value: *const GValue,
        expected_type: *const GVariantType,
        user_data: gpointer,
    ) -> *mut GVariant;
    fn g_settings_get_mapping(
        value: *mut GValue,
        variant: *mut GVariant,
        user_data: gpointer,
    ) -> gboolean;
    fn g_settings_mapping_is_compatible(
        gvalue_type: GType,
        variant_type: *const GVariantType,
    ) -> gboolean;
    fn g_settings_schema_list(schema: *mut GSettingsSchema, n_items: *mut gint) -> *const GQuark;
    fn g_settings_schema_get_child_schema(
        schema: *mut GSettingsSchema,
        name: *const gchar,
    ) -> *mut GSettingsSchema;
    fn g_settings_schema_key_init(
        key: *mut GSettingsSchemaKey,
        schema: *mut GSettingsSchema,
        name: *const gchar,
    );
    fn g_settings_schema_key_clear(key: *mut GSettingsSchemaKey);
    fn g_settings_schema_key_type_check(
        key: *mut GSettingsSchemaKey,
        value: *mut GVariant,
    ) -> gboolean;
    fn g_settings_schema_key_range_fixup(
        key: *mut GSettingsSchemaKey,
        value: *mut GVariant,
    ) -> *mut GVariant;
    fn g_settings_schema_key_get_translated_default(key: *mut GSettingsSchemaKey) -> *mut GVariant;
    fn g_settings_schema_key_get_per_desktop_default(key: *mut GSettingsSchemaKey)
        -> *mut GVariant;
    fn g_settings_schema_key_to_enum(key: *mut GSettingsSchemaKey, value: *mut GVariant) -> gint;
    fn g_settings_schema_key_from_enum(key: *mut GSettingsSchemaKey, value: gint) -> *mut GVariant;
    fn g_settings_schema_key_to_flags(key: *mut GSettingsSchemaKey, value: *mut GVariant) -> guint;
    fn g_settings_schema_key_from_flags(
        key: *mut GSettingsSchemaKey,
        value: guint,
    ) -> *mut GVariant;
    fn g_action_get_type() -> GType;
    fn g_action_change_state(action: *mut GAction, value: *mut GVariant);
    fn _g_cclosure_marshal_BOOLEAN__POINTER_INT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_BOOLEAN__POINTER_INTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
    fn _g_cclosure_marshal_BOOLEAN__UINT(
        closure: *mut GClosure,
        return_value: *mut GValue,
        n_param_values: guint,
        param_values: *const GValue,
        invocation_hint: gpointer,
        marshal_data: gpointer,
    );
    fn _g_cclosure_marshal_BOOLEAN__UINTv(
        closure: *mut GClosure,
        return_value: *mut GValue,
        instance: gpointer,
        args: ::core::ffi::VaList,
        marshal_data: gpointer,
        n_params: ::core::ffi::c_int,
        param_types: *mut GType,
    );
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
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type va_list = __builtin_va_list;
pub type GQuark = guint32;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
pub type GVariantType = _GVariantType;
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
pub type GSettingsSchemaSource = _GSettingsSchemaSource;
pub type GSettingsSchema = _GSettingsSchema;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GSettingsSchemaKey {
    pub schema: *mut GSettingsSchema,
    pub name: *const gchar,
    #[bitfield(name = "is_flags", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "is_enum", ty = "guint", bits = "1..=1")]
    pub is_flags_is_enum: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub strinfo: *const guint32,
    pub strinfo_length: gsize,
    pub unparsed: *const gchar,
    pub lc_char: gchar,
    pub type_0: *const GVariantType,
    pub minimum: *mut GVariant,
    pub maximum: *mut GVariant,
    pub default_value: *mut GVariant,
    pub desktop_overrides: *mut GVariant,
    pub ref_count: gint,
}
pub type GSettingsSchemaKey = _GSettingsSchemaKey;
pub type GAction = _GAction;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsBackend {
    pub parent_instance: GObject,
    pub priv_0: *mut GSettingsBackendPrivate,
}
pub type GSettingsBackendPrivate = _GSettingsBackendPrivate;
pub type GSettingsBackend = _GSettingsBackend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettings {
    pub parent_instance: GObject,
    pub priv_0: *mut GSettingsPrivate,
}
pub type GSettingsPrivate = _GSettingsPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsPrivate {
    pub main_context: *mut GMainContext,
    pub backend: *mut GSettingsBackend,
    pub schema: *mut GSettingsSchema,
    pub path: *mut gchar,
}
pub type GSettings = _GSettings;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsClass {
    pub parent_class: GObjectClass,
    pub writable_changed: Option<unsafe extern "C" fn(*mut GSettings, *const gchar) -> ()>,
    pub changed: Option<unsafe extern "C" fn(*mut GSettings, *const gchar) -> ()>,
    pub writable_change_event: Option<unsafe extern "C" fn(*mut GSettings, GQuark) -> gboolean>,
    pub change_event: Option<unsafe extern "C" fn(*mut GSettings, *const GQuark, gint) -> gboolean>,
    pub padding: [gpointer; 20],
}
pub type GSettingsClass = _GSettingsClass;
pub const PROP_DELAY_APPLY: C2RustUnnamed_0 = 6;
pub const PROP_HAS_UNAPPLIED: C2RustUnnamed_0 = 5;
pub const PROP_PATH: C2RustUnnamed_0 = 4;
pub const PROP_SCHEMA_ID: C2RustUnnamed_0 = 2;
pub const PROP_SCHEMA: C2RustUnnamed_0 = 1;
pub const PROP_BACKEND: C2RustUnnamed_0 = 3;
pub const SIGNAL_WRITABLE_CHANGE_EVENT: C2RustUnnamed_1 = 0;
pub const SIGNAL_WRITABLE_CHANGED: C2RustUnnamed_1 = 1;
pub const SIGNAL_CHANGE_EVENT: C2RustUnnamed_1 = 2;
pub const SIGNAL_CHANGED: C2RustUnnamed_1 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSettingsListenerVTable {
    pub changed: Option<
        unsafe extern "C" fn(*mut GObject, *mut GSettingsBackend, *const gchar, gpointer) -> (),
    >,
    pub path_changed: Option<
        unsafe extern "C" fn(*mut GObject, *mut GSettingsBackend, *const gchar, gpointer) -> (),
    >,
    pub keys_changed: Option<
        unsafe extern "C" fn(
            *mut GObject,
            *mut GSettingsBackend,
            *const gchar,
            gpointer,
            *const *const gchar,
        ) -> (),
    >,
    pub writable_changed:
        Option<unsafe extern "C" fn(*mut GObject, *mut GSettingsBackend, *const gchar) -> ()>,
    pub path_writable_changed:
        Option<unsafe extern "C" fn(*mut GObject, *mut GSettingsBackend, *const gchar) -> ()>,
}
pub type GDelayedSettingsBackend = _GDelayedSettingsBackend;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDelayedSettingsBackend {
    pub parent_instance: GSettingsBackend,
    pub priv_0: *mut GDelayedSettingsBackendPrivate,
}
pub type GDelayedSettingsBackendPrivate = _GDelayedSettingsBackendPrivate;
pub type GSettingsBindSetMapping =
    Option<unsafe extern "C" fn(*const GValue, *const GVariantType, gpointer) -> *mut GVariant>;
pub type GSettingsBindGetMapping =
    Option<unsafe extern "C" fn(*mut GValue, *mut GVariant, gpointer) -> gboolean>;
pub type GSettingsGetMapping =
    Option<unsafe extern "C" fn(*mut GVariant, *mut gpointer, gpointer) -> gboolean>;
pub type GSettingsBindFlags = ::core::ffi::c_uint;
pub const G_SETTINGS_BIND_INVERT_BOOLEAN: GSettingsBindFlags = 16;
pub const G_SETTINGS_BIND_GET_NO_CHANGES: GSettingsBindFlags = 8;
pub const G_SETTINGS_BIND_NO_SENSITIVITY: GSettingsBindFlags = 4;
pub const G_SETTINGS_BIND_SET: GSettingsBindFlags = 2;
pub const G_SETTINGS_BIND_GET: GSettingsBindFlags = 1;
pub const G_SETTINGS_BIND_DEFAULT: GSettingsBindFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSettingsBinding {
    pub key: GSettingsSchemaKey,
    pub settings: *mut GSettings,
    pub object: *mut GObject,
    pub get_mapping: GSettingsBindGetMapping,
    pub set_mapping: GSettingsBindSetMapping,
    pub user_data: gpointer,
    pub destroy: GDestroyNotify,
    pub writable_handler_id: guint,
    pub property_handler_id: guint,
    pub property: *const GParamSpec,
    pub key_handler_id: guint,
    pub running: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSettingsWritableBinding {
    pub settings: *mut GSettings,
    pub object: gpointer,
    pub key: *const gchar,
    pub property: *const gchar,
    pub inverted: gboolean,
    pub handler_id: gulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GSettingsAction {
    pub parent_instance: GObject,
    pub key: GSettingsSchemaKey,
    pub settings: *mut GSettings,
}
pub type GSettingsActionClass = GObjectClass;
pub const ACTION_PROP_STATE: C2RustUnnamed_2 = 5;
pub const ACTION_PROP_STATE_TYPE: C2RustUnnamed_2 = 4;
pub const ACTION_PROP_ENABLED: C2RustUnnamed_2 = 3;
pub const ACTION_PROP_PARAMETER_TYPE: C2RustUnnamed_2 = 2;
pub const ACTION_PROP_NAME: C2RustUnnamed_2 = 1;
pub type GActionInterface = _GActionInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GActionInterface {
    pub g_iface: GTypeInterface,
    pub get_name: Option<unsafe extern "C" fn(*mut GAction) -> *const gchar>,
    pub get_parameter_type: Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>,
    pub get_state_type: Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>,
    pub get_state_hint: Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>,
    pub get_enabled: Option<unsafe extern "C" fn(*mut GAction) -> gboolean>,
    pub get_state: Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>,
    pub change_state: Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>,
    pub activate: Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const N_SIGNALS: C2RustUnnamed_1 = 4;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const ACTION_PROP_0: C2RustUnnamed_2 = 0;
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
pub const G_VARIANT_TYPE_BOOLEAN: *const GVariantType =
    b"b\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_TYPE_FUNDAMENTAL_SHIFT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TYPE_NONE: GType = ((1 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_BOOLEAN: GType = ((5 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_INT: GType = ((6 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_UINT: GType = ((7 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_STRING: GType = ((16 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_POINTER: GType = ((17 as ::core::ffi::c_int) << G_TYPE_FUNDAMENTAL_SHIFT) as GType;
pub const G_TYPE_FLAG_RESERVED_ID_BIT: GType =
    ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType;
pub const G_SIGNAL_TYPE_STATIC_SCOPE: GType =
    ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType;
static mut safe_c2rust_g_settings_signals: [guint; 4] = [0; 4];
unsafe extern "C" fn safe_c2rust_g_settings_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_settings_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSettings_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GSettings_private_offset);
    }
    safe_c2rust_g_settings_class_init(klass as *mut GSettingsClass);
}
static mut safe_c2rust_g_settings_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_settings_get_type_once();
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
static mut safe_c2rust_GSettings_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_settings_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSettings\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSettingsClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_settings_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSettings>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSettings) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_settings_init as unsafe extern "C" fn(*mut GSettings) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GSettings_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GSettingsPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_settings_get_instance_private(
    mut self_0: *mut GSettings,
) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GSettings_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_settings_real_change_event(
    mut settings: *mut GSettings,
    mut keys: *const GQuark,
    mut n_keys: gint,
) -> gboolean {
    let mut i: gint = 0;
    if keys.is_null() {
        keys = g_settings_schema_list((*(*settings).priv_0).schema, &raw mut n_keys);
    }
    i = 0 as ::core::ffi::c_int as gint;
    while i < n_keys {
        let mut key: *const gchar = g_quark_to_string(*keys.offset(i as isize));
        if !(if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = key as *const ::core::ffi::c_char;
                let __suffix: *const ::core::ffi::c_char =
                    b"/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                    if __str.is_null() || __suffix.is_null() {
                        _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_11
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __suffix_len: size_t =
                        strlen(__suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __suffix_len {
                        __result = (memcmp(
                            __str
                                .offset(__str_len as isize)
                                .offset(-(__suffix_len as isize))
                                as *const ::core::ffi::c_void,
                            __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __suffix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_suffix(key, b"/\0" as *const u8 as *const gchar)
        } != 0)
        {
            g_signal_emit(
                settings as gpointer,
                safe_c2rust_g_settings_signals[SIGNAL_CHANGED as ::core::ffi::c_int as usize],
                *keys.offset(i as isize),
                key,
            );
        }
        i += 1;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_settings_real_writable_change_event(
    mut settings: *mut GSettings,
    mut key: GQuark,
) -> gboolean {
    let mut keys: *const GQuark = &raw mut key;
    let mut n_keys: gint = 1 as gint;
    let mut i: gint = 0;
    if key == 0 as GQuark {
        keys = g_settings_schema_list((*(*settings).priv_0).schema, &raw mut n_keys);
    }
    i = 0 as ::core::ffi::c_int as gint;
    while i < n_keys {
        let mut key_name: *const gchar = g_quark_to_string(*keys.offset(i as isize));
        if !(if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = key_name as *const ::core::ffi::c_char;
                let __suffix: *const ::core::ffi::c_char =
                    b"/\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                    if __str.is_null() || __suffix.is_null() {
                        _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_12
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    let __suffix_len: size_t =
                        strlen(__suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
                    if __str_len >= __suffix_len {
                        __result = (memcmp(
                            __str
                                .offset(__str_len as isize)
                                .offset(-(__suffix_len as isize))
                                as *const ::core::ffi::c_void,
                            __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            __suffix_len,
                        ) == 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int as gboolean;
                    }
                }
                __result
            })
        } else {
            g_str_has_suffix(key_name, b"/\0" as *const u8 as *const gchar)
        } != 0)
        {
            g_signal_emit(
                settings as gpointer,
                safe_c2rust_g_settings_signals
                    [SIGNAL_WRITABLE_CHANGED as ::core::ffi::c_int as usize],
                *keys.offset(i as isize),
                key_name,
            );
        }
        i += 1;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_settings_backend_changed(
    mut target: *mut GObject,
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut origin_tag: gpointer,
) {
    let mut settings: *mut GSettings = target as *mut ::core::ffi::c_void as *mut GSettings;
    let mut ignore_this: gboolean = 0;
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while *key.offset(i as isize) as ::core::ffi::c_int
        == *(*(*settings).priv_0).path.offset(i as isize) as ::core::ffi::c_int
    {
        i += 1;
    }
    if *(*(*settings).priv_0).path.offset(i as isize) as ::core::ffi::c_int == '\0' as i32
        && g_settings_schema_has_key((*(*settings).priv_0).schema, key.offset(i as isize)) != 0
    {
        let mut quark: GQuark = 0;
        quark = g_quark_from_string(key.offset(i as isize));
        g_signal_emit(
            settings as gpointer,
            safe_c2rust_g_settings_signals[SIGNAL_CHANGE_EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            &raw mut quark,
            1 as ::core::ffi::c_int,
            &raw mut ignore_this,
        );
    }
}
unsafe extern "C" fn safe_c2rust_settings_backend_path_changed(
    mut target: *mut GObject,
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
    mut origin_tag: gpointer,
) {
    let mut settings: *mut GSettings = target as *mut ::core::ffi::c_void as *mut GSettings;
    let mut ignore_this: gboolean = 0;
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = (*(*settings).priv_0).path;
            let __prefix: *const ::core::ffi::c_char = path as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_13
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
        g_str_has_prefix((*(*settings).priv_0).path, path)
    } != 0
    {
        g_signal_emit(
            settings as gpointer,
            safe_c2rust_g_settings_signals[SIGNAL_CHANGE_EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            NULL_0,
            0 as ::core::ffi::c_int,
            &raw mut ignore_this,
        );
    }
}
unsafe extern "C" fn safe_c2rust_settings_backend_keys_changed(
    mut target: *mut GObject,
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
    mut origin_tag: gpointer,
    mut items: *const *const gchar,
) {
    let mut settings: *mut GSettings = target as *mut ::core::ffi::c_void as *mut GSettings;
    let mut ignore_this: gboolean = 0;
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while *(*(*settings).priv_0).path.offset(i as isize) as ::core::ffi::c_int != 0
        && *(*(*settings).priv_0).path.offset(i as isize) as ::core::ffi::c_int
            == *path.offset(i as isize) as ::core::ffi::c_int
    {
        i += 1;
    }
    if *path.offset(i as isize) as ::core::ffi::c_int == '\0' as i32 {
        let mut quarks: [GQuark; 256] = [0; 256];
        let mut j: gint = 0;
        let mut l: gint = 0 as gint;
        j = 0 as ::core::ffi::c_int as gint;
        while !(*items.offset(j as isize)).is_null() {
            let mut item: *const gchar = *items.offset(j as isize);
            let mut k: gint = 0;
            k = 0 as ::core::ffi::c_int as gint;
            while *item.offset(k as isize) as ::core::ffi::c_int
                == *(*(*settings).priv_0).path.offset((i + k) as isize) as ::core::ffi::c_int
            {
                k += 1;
            }
            if *(*(*settings).priv_0).path.offset((i + k) as isize) as ::core::ffi::c_int
                == '\0' as i32
                && g_settings_schema_has_key((*(*settings).priv_0).schema, item.offset(k as isize))
                    != 0
            {
                let fresh0 = l;
                l = l + 1;
                quarks[fresh0 as usize] = g_quark_from_string(item.offset(k as isize));
            }
            if ({
                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                if l < 256 as ::core::ffi::c_int {
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
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    497 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"l < 256\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            j += 1;
        }
        if l > 0 as ::core::ffi::c_int {
            g_signal_emit(
                settings as gpointer,
                safe_c2rust_g_settings_signals[SIGNAL_CHANGE_EVENT as ::core::ffi::c_int as usize],
                0 as GQuark,
                &raw mut quarks as *mut GQuark,
                l,
                &raw mut ignore_this,
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_settings_backend_writable_changed(
    mut target: *mut GObject,
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
) {
    let mut settings: *mut GSettings = target as *mut ::core::ffi::c_void as *mut GSettings;
    let mut ignore_this: gboolean = 0;
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while *key.offset(i as isize) as ::core::ffi::c_int
        == *(*(*settings).priv_0).path.offset(i as isize) as ::core::ffi::c_int
    {
        i += 1;
    }
    if *(*(*settings).priv_0).path.offset(i as isize) as ::core::ffi::c_int == '\0' as i32
        && g_settings_schema_has_key((*(*settings).priv_0).schema, key.offset(i as isize)) != 0
    {
        g_signal_emit(
            settings as gpointer,
            safe_c2rust_g_settings_signals
                [SIGNAL_WRITABLE_CHANGE_EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            g_quark_from_string(key.offset(i as isize)),
            &raw mut ignore_this,
        );
    }
}
unsafe extern "C" fn safe_c2rust_settings_backend_path_writable_changed(
    mut target: *mut GObject,
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
) {
    let mut settings: *mut GSettings = target as *mut ::core::ffi::c_void as *mut GSettings;
    let mut ignore_this: gboolean = 0;
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = (*(*settings).priv_0).path;
            let __prefix: *const ::core::ffi::c_char = path as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_15
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
        g_str_has_prefix((*(*settings).priv_0).path, path)
    } != 0
    {
        g_signal_emit(
            settings as gpointer,
            safe_c2rust_g_settings_signals
                [SIGNAL_WRITABLE_CHANGE_EVENT as ::core::ffi::c_int as usize],
            0 as GQuark,
            0 as ::core::ffi::c_int as GQuark,
            &raw mut ignore_this,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_settings_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut settings: *mut GSettings = object as *mut ::core::ffi::c_void as *mut GSettings;
    match prop_id {
        1 => {
            let mut schema: *mut GSettingsSchema = ::core::ptr::null_mut::<GSettingsSchema>();
            schema = g_value_dup_boxed(value) as *mut GSettingsSchema;
            if !schema.is_null() {
                if ({
                    let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                    if (*(*settings).priv_0).schema.is_null() {
                        _g_boolean_var_16 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_16 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_16
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        560 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"settings->priv->schema == NULL\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                (*(*settings).priv_0).schema = schema;
            }
        }
        2 => {
            let mut schema_id: *const gchar = ::core::ptr::null::<gchar>();
            schema_id = g_value_get_string(value);
            if !schema_id.is_null() {
                let mut default_source: *mut GSettingsSchemaSource =
                    ::core::ptr::null_mut::<GSettingsSchemaSource>();
                if ({
                    let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
                    if (*(*settings).priv_0).schema.is_null() {
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
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        580 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"settings->priv->schema == NULL\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                default_source = g_settings_schema_source_get_default();
                if default_source.is_null() {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_ERROR,
                        b"No GSettings schemas are installed on the system\0" as *const u8
                            as *const gchar,
                    );
                    loop {}
                }
                (*(*settings).priv_0).schema =
                    g_settings_schema_source_lookup(default_source, schema_id, TRUE);
                if (*(*settings).priv_0).schema.is_null() {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_ERROR,
                        b"Settings schema '%s' is not installed\0" as *const u8 as *const gchar,
                        schema_id,
                    );
                    loop {}
                }
            }
        }
        4 => {
            (*(*settings).priv_0).path = g_value_dup_string(value);
        }
        3 => {
            (*(*settings).priv_0).backend = g_value_dup_object(value) as *mut GSettingsBackend;
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                603 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_settings_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut settings: *mut GSettings = object as *mut ::core::ffi::c_void as *mut GSettings;
    match prop_id {
        1 => {
            g_value_set_boxed(value, (*(*settings).priv_0).schema as gconstpointer);
        }
        2 => {
            g_value_set_string(
                value,
                g_settings_schema_get_id((*(*settings).priv_0).schema),
            );
        }
        3 => {
            g_value_set_object(value, (*(*settings).priv_0).backend as gpointer);
        }
        4 => {
            g_value_set_string(value, (*(*settings).priv_0).path);
        }
        5 => {
            g_value_set_boolean(value, safe_c2rust_g_settings_get_has_unapplied(settings));
        }
        6 => {
            g_value_set_boolean(
                value,
                ({
                    let mut __inst: *mut GTypeInstance =
                        (*(*settings).priv_0).backend as *mut GTypeInstance;
                    let mut __t: GType = g_delayed_settings_backend_get_type();
                    let mut __r: gboolean = 0;
                    if __inst.is_null() {
                        __r = FALSE as gboolean;
                    } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                        __r = TRUE as gboolean;
                    } else {
                        __r = g_type_check_instance_is_a(__inst, __t);
                    }
                    __r
                }),
            );
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                642 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
static mut safe_c2rust_listener_vtable: GSettingsListenerVTable = unsafe {
    GSettingsListenerVTable {
        changed: Some(
            safe_c2rust_settings_backend_changed
                as unsafe extern "C" fn(
                    *mut GObject,
                    *mut GSettingsBackend,
                    *const gchar,
                    gpointer,
                ) -> (),
        ),
        path_changed: Some(
            safe_c2rust_settings_backend_path_changed
                as unsafe extern "C" fn(
                    *mut GObject,
                    *mut GSettingsBackend,
                    *const gchar,
                    gpointer,
                ) -> (),
        ),
        keys_changed: Some(
            safe_c2rust_settings_backend_keys_changed
                as unsafe extern "C" fn(
                    *mut GObject,
                    *mut GSettingsBackend,
                    *const gchar,
                    gpointer,
                    *const *const gchar,
                ) -> (),
        ),
        writable_changed: Some(
            safe_c2rust_settings_backend_writable_changed
                as unsafe extern "C" fn(*mut GObject, *mut GSettingsBackend, *const gchar) -> (),
        ),
        path_writable_changed: Some(
            safe_c2rust_settings_backend_path_writable_changed
                as unsafe extern "C" fn(*mut GObject, *mut GSettingsBackend, *const gchar) -> (),
        ),
    }
};
unsafe extern "C" fn safe_c2rust_g_settings_constructed(mut object: *mut GObject) {
    let mut settings: *mut GSettings = object as *mut ::core::ffi::c_void as *mut GSettings;
    let mut schema_path: *const gchar = ::core::ptr::null::<gchar>();
    schema_path = g_settings_schema_get_path((*(*settings).priv_0).schema);
    if !(*(*settings).priv_0).path.is_null()
        && !schema_path.is_null()
        && strcmp(
            (*(*settings).priv_0).path,
            schema_path as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"settings object created with schema '%s' and path '%s', but path '%s' is specified by schema\0"
                as *const u8 as *const gchar,
            g_settings_schema_get_id((*(*settings).priv_0).schema),
            (*(*settings).priv_0).path,
            schema_path,
        );
        loop {}
    }
    if (*(*settings).priv_0).path.is_null() {
        if schema_path.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"attempting to create schema '%s' without a path\0" as *const u8 as *const gchar,
                g_settings_schema_get_id((*(*settings).priv_0).schema),
            );
            loop {}
        }
        (*(*settings).priv_0).path =
            safe_c2rust_g_strdup_inline(schema_path as *const ::core::ffi::c_char) as *mut gchar;
    }
    if (*(*settings).priv_0).backend.is_null() {
        (*(*settings).priv_0).backend = g_settings_backend_get_default();
    }
    g_settings_backend_watch(
        (*(*settings).priv_0).backend,
        &raw const safe_c2rust_listener_vtable,
        settings as *mut ::core::ffi::c_void as *mut GObject,
        (*(*settings).priv_0).main_context,
    );
    g_settings_backend_subscribe((*(*settings).priv_0).backend, (*(*settings).priv_0).path);
}
unsafe extern "C" fn safe_c2rust_g_settings_finalize(mut object: *mut GObject) {
    let mut settings: *mut GSettings = object as *mut ::core::ffi::c_void as *mut GSettings;
    g_settings_backend_unsubscribe((*(*settings).priv_0).backend, (*(*settings).priv_0).path);
    g_main_context_unref((*(*settings).priv_0).main_context);
    g_object_unref((*(*settings).priv_0).backend as gpointer);
    g_settings_schema_unref((*(*settings).priv_0).schema);
    g_free((*(*settings).priv_0).path as gpointer);
    (*(safe_c2rust_g_settings_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_settings_init(mut settings: *mut GSettings) {
    (*settings).priv_0 =
        safe_c2rust_g_settings_get_instance_private(settings) as *mut GSettingsPrivate;
    (*(*settings).priv_0).main_context = g_main_context_ref_thread_default();
}
unsafe extern "C" fn safe_c2rust_g_settings_class_init(mut class: *mut GSettingsClass) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*class).writable_change_event = Some(
        safe_c2rust_g_settings_real_writable_change_event
            as unsafe extern "C" fn(*mut GSettings, GQuark) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GSettings, GQuark) -> gboolean>;
    (*class).change_event = Some(
        safe_c2rust_g_settings_real_change_event
            as unsafe extern "C" fn(*mut GSettings, *const GQuark, gint) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GSettings, *const GQuark, gint) -> gboolean>;
    (*object_class).set_property = Some(
        safe_c2rust_g_settings_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*object_class).get_property = Some(
        safe_c2rust_g_settings_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*object_class).constructed =
        Some(safe_c2rust_g_settings_constructed as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*object_class).finalize =
        Some(safe_c2rust_g_settings_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    safe_c2rust_g_settings_signals[SIGNAL_CHANGED as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"changed\0" as *const u8 as *const gchar),
        safe_c2rust_g_settings_get_type(),
        (G_SIGNAL_RUN_LAST as ::core::ffi::c_int | G_SIGNAL_DETAILED as ::core::ffi::c_int)
            as GSignalFlags,
        144 as ::core::ffi::c_ulong as glong as guint,
        None,
        NULL_0,
        None,
        G_TYPE_NONE,
        1 as guint,
        G_TYPE_STRING | G_SIGNAL_TYPE_STATIC_SCOPE,
    );
    safe_c2rust_g_settings_signals[SIGNAL_CHANGE_EVENT as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"change-event\0" as *const u8 as *const gchar),
            safe_c2rust_g_settings_get_type(),
            G_SIGNAL_RUN_LAST,
            160 as ::core::ffi::c_ulong as glong as guint,
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
                _g_cclosure_marshal_BOOLEAN__POINTER_INT
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
            G_TYPE_POINTER,
            G_TYPE_INT,
        );
    g_signal_set_va_marshaller(
        safe_c2rust_g_settings_signals[SIGNAL_CHANGE_EVENT as ::core::ffi::c_int as usize],
        (*(class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_BOOLEAN__POINTER_INTv
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
    safe_c2rust_g_settings_signals[SIGNAL_WRITABLE_CHANGED as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"writable-changed\0" as *const u8 as *const gchar),
            safe_c2rust_g_settings_get_type(),
            (G_SIGNAL_RUN_LAST as ::core::ffi::c_int | G_SIGNAL_DETAILED as ::core::ffi::c_int)
                as GSignalFlags,
            136 as ::core::ffi::c_ulong as glong as guint,
            None,
            NULL_0,
            None,
            G_TYPE_NONE,
            1 as guint,
            G_TYPE_STRING | G_SIGNAL_TYPE_STATIC_SCOPE,
        );
    safe_c2rust_g_settings_signals[SIGNAL_WRITABLE_CHANGE_EVENT as ::core::ffi::c_int as usize] =
        g_signal_new(
            g_intern_static_string(b"writable-change-event\0" as *const u8 as *const gchar),
            safe_c2rust_g_settings_get_type(),
            G_SIGNAL_RUN_LAST,
            152 as ::core::ffi::c_ulong as glong as guint,
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
                _g_cclosure_marshal_BOOLEAN__UINT
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
            G_TYPE_UINT,
        );
    g_signal_set_va_marshaller(
        safe_c2rust_g_settings_signals[SIGNAL_WRITABLE_CHANGE_EVENT as ::core::ffi::c_int as usize],
        (*(class as *mut GTypeClass)).g_type,
        Some(
            _g_cclosure_marshal_BOOLEAN__UINTv
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
        object_class,
        PROP_BACKEND as ::core::ffi::c_int as guint,
        g_param_spec_object(
            b"backend\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_settings_backend_get_type(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_SCHEMA as ::core::ffi::c_int as guint,
        g_param_spec_boxed(
            b"settings-schema\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            g_settings_schema_get_type(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_SCHEMA_ID as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"schema\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_DEPRECATED as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_SCHEMA_ID as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"schema-id\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_PATH as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"path\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | G_PARAM_READWRITE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    g_object_class_install_property(
        object_class,
        PROP_HAS_UNAPPLIED as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"has-unapplied\0" as *const u8 as *const gchar,
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
        PROP_DELAY_APPLY as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"delay-apply\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_new(mut schema_id: *const gchar) -> *mut GSettings {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !schema_id.is_null() {
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
            b"schema_id != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    return g_object_new(
        safe_c2rust_g_settings_get_type(),
        b"schema-id\0" as *const u8 as *const gchar,
        schema_id,
        NULL_0,
    ) as *mut GSettings;
}
unsafe extern "C" fn safe_c2rust_path_is_valid(mut path: *const gchar) -> gboolean {
    if path.is_null() {
        return FALSE;
    }
    if *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '/' as i32 {
        return FALSE;
    }
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = path as *const ::core::ffi::c_char;
            let __suffix: *const ::core::ffi::c_char =
                b"/\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                if __str.is_null() || __suffix.is_null() {
                    _g_boolean_var_19 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_19 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_19
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __suffix_len: size_t =
                    strlen(__suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
                if __str_len >= __suffix_len {
                    __result = (memcmp(
                        __str
                            .offset(__str_len as isize)
                            .offset(-(__suffix_len as isize))
                            as *const ::core::ffi::c_void,
                        __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                            as *const ::core::ffi::c_void,
                        __suffix_len,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_suffix(path, b"/\0" as *const u8 as *const gchar)
    } == 0
    {
        return FALSE;
    }
    return (strstr(
        path as *const ::core::ffi::c_char,
        b"//\0" as *const u8 as *const ::core::ffi::c_char,
    ) == NULL_0 as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_new_with_path(
    mut schema_id: *const gchar,
    mut path: *const gchar,
) -> *mut GSettings {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !schema_id.is_null() {
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
            b"schema_id != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if safe_c2rust_path_is_valid(path) != 0 {
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
            b"path_is_valid (path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    return g_object_new(
        safe_c2rust_g_settings_get_type(),
        b"schema-id\0" as *const u8 as *const gchar,
        schema_id,
        b"path\0" as *const u8 as *const ::core::ffi::c_char,
        path,
        NULL_0,
    ) as *mut GSettings;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_new_with_backend(
    mut schema_id: *const gchar,
    mut backend: *mut GSettingsBackend,
) -> *mut GSettings {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !schema_id.is_null() {
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
            b"schema_id != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = g_settings_backend_get_type();
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
            b"G_IS_SETTINGS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    return g_object_new(
        safe_c2rust_g_settings_get_type(),
        b"schema-id\0" as *const u8 as *const gchar,
        schema_id,
        b"backend\0" as *const u8 as *const ::core::ffi::c_char,
        backend,
        NULL_0,
    ) as *mut GSettings;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_new_with_backend_and_path(
    mut schema_id: *const gchar,
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
) -> *mut GSettings {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !schema_id.is_null() {
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
            b"schema_id != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = g_settings_backend_get_type();
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
            b"G_IS_SETTINGS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if safe_c2rust_path_is_valid(path) != 0 {
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
            b"path_is_valid (path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    return g_object_new(
        safe_c2rust_g_settings_get_type(),
        b"schema-id\0" as *const u8 as *const gchar,
        schema_id,
        b"backend\0" as *const u8 as *const ::core::ffi::c_char,
        backend,
        b"path\0" as *const u8 as *const ::core::ffi::c_char,
        path,
        NULL_0,
    ) as *mut GSettings;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_new_full(
    mut schema: *mut GSettingsSchema,
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
) -> *mut GSettings {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !schema.is_null() {
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
            b"schema != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if backend.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
                let mut __t: GType = g_settings_backend_get_type();
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
            b"backend == NULL || G_IS_SETTINGS_BACKEND (backend)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if path.is_null() || safe_c2rust_path_is_valid(path) != 0 {
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
            b"path == NULL || path_is_valid (path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    return g_object_new(
        safe_c2rust_g_settings_get_type(),
        b"settings-schema\0" as *const u8 as *const gchar,
        schema,
        b"backend\0" as *const u8 as *const ::core::ffi::c_char,
        backend,
        b"path\0" as *const u8 as *const ::core::ffi::c_char,
        path,
        NULL_0,
    ) as *mut GSettings;
}
unsafe extern "C" fn safe_c2rust_g_settings_write_to_backend(
    mut settings: *mut GSettings,
    mut key: *mut GSettingsSchemaKey,
    mut value: *mut GVariant,
) -> gboolean {
    let mut success: gboolean = 0;
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    path = g_strconcat((*(*settings).priv_0).path, (*key).name, NULL_0);
    success = g_settings_backend_write((*(*settings).priv_0).backend, path, value, NULL_0);
    g_free(path as gpointer);
    return success;
}
unsafe extern "C" fn safe_c2rust_g_settings_read_from_backend(
    mut settings: *mut GSettings,
    mut key: *mut GSettingsSchemaKey,
    mut user_value_only: gboolean,
    mut default_value: gboolean,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut fixup: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    path = g_strconcat((*(*settings).priv_0).path, (*key).name, NULL_0);
    if user_value_only != 0 {
        value =
            g_settings_backend_read_user_value((*(*settings).priv_0).backend, path, (*key).type_0);
    } else {
        value = g_settings_backend_read(
            (*(*settings).priv_0).backend,
            path,
            (*key).type_0,
            default_value,
        );
    }
    g_free(path as gpointer);
    if !value.is_null() {
        fixup = g_settings_schema_key_range_fixup(key, value);
        g_variant_unref(value);
    } else {
        fixup = ::core::ptr::null_mut::<GVariant>();
    }
    return fixup;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_value(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> *mut GVariant {
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    value = safe_c2rust_g_settings_read_from_backend(settings, &raw mut skey, FALSE, FALSE);
    if value.is_null() {
        value = g_settings_schema_key_get_default_value(&raw mut skey);
    }
    g_settings_schema_key_clear(&raw mut skey);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_user_value(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> *mut GVariant {
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    value = safe_c2rust_g_settings_read_from_backend(settings, &raw mut skey, TRUE, FALSE);
    g_settings_schema_key_clear(&raw mut skey);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_default_value(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> *mut GVariant {
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    value = safe_c2rust_g_settings_read_from_backend(settings, &raw mut skey, FALSE, TRUE);
    if value.is_null() {
        value = g_settings_schema_key_get_default_value(&raw mut skey);
    }
    g_settings_schema_key_clear(&raw mut skey);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_enum(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> gint {
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut result: gint = 0;
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    if skey.is_enum() == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_get_enum() called on key '%s' which is not associated with an enumerated type\0"
                as *const u8 as *const gchar,
            skey.name,
        );
        g_settings_schema_key_clear(&raw mut skey);
        return -(1 as gint);
    }
    value = safe_c2rust_g_settings_read_from_backend(settings, &raw mut skey, FALSE, FALSE);
    if value.is_null() {
        value = g_settings_schema_key_get_default_value(&raw mut skey);
    }
    result = g_settings_schema_key_to_enum(&raw mut skey, value);
    g_settings_schema_key_clear(&raw mut skey);
    g_variant_unref(value);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_enum(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: gint,
) -> gboolean {
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut success: gboolean = 0;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    if skey.is_enum() == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_set_enum() called on key '%s' which is not associated with an enumerated type\0"
                as *const u8 as *const gchar,
            skey.name,
        );
        return FALSE;
    }
    variant = g_settings_schema_key_from_enum(&raw mut skey, value);
    if variant.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_set_enum(): invalid enum value %d for key '%s' in schema '%s'.  Doing nothing.\0"
                as *const u8 as *const gchar,
            value,
            skey.name,
            g_settings_schema_get_id(skey.schema),
        );
        g_settings_schema_key_clear(&raw mut skey);
        return FALSE;
    }
    success = safe_c2rust_g_settings_write_to_backend(
        settings,
        &raw mut skey,
        safe_c2rust_g_steal_pointer(&raw mut variant as gpointer) as *mut GVariant,
    );
    g_settings_schema_key_clear(&raw mut skey);
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_flags(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> guint {
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut result: guint = 0;
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as guint;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as guint;
    }
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    if skey.is_flags() == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_get_flags() called on key '%s' which is not associated with a flags type\0"
                as *const u8 as *const gchar,
            skey.name,
        );
        g_settings_schema_key_clear(&raw mut skey);
        return -(1 as ::core::ffi::c_int) as guint;
    }
    value = safe_c2rust_g_settings_read_from_backend(settings, &raw mut skey, FALSE, FALSE);
    if value.is_null() {
        value = g_settings_schema_key_get_default_value(&raw mut skey);
    }
    result = g_settings_schema_key_to_flags(&raw mut skey, value);
    g_settings_schema_key_clear(&raw mut skey);
    g_variant_unref(value);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_flags(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: guint,
) -> gboolean {
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut success: gboolean = 0;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    if skey.is_flags() == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_set_flags() called on key '%s' which is not associated with a flags type\0"
                as *const u8 as *const gchar,
            skey.name,
        );
        return FALSE;
    }
    variant = g_settings_schema_key_from_flags(&raw mut skey, value);
    if variant.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_set_flags(): invalid flags value 0x%08x for key '%s' in schema '%s'.  Doing nothing.\0"
                as *const u8 as *const gchar,
            value,
            skey.name,
            g_settings_schema_get_id(skey.schema),
        );
        g_settings_schema_key_clear(&raw mut skey);
        return FALSE;
    }
    success = safe_c2rust_g_settings_write_to_backend(
        settings,
        &raw mut skey,
        safe_c2rust_g_steal_pointer(&raw mut variant as gpointer) as *mut GVariant,
    );
    g_settings_schema_key_clear(&raw mut skey);
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_value(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: *mut GVariant,
) -> gboolean {
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut success: gboolean = 0;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_variant_ref_sink(value);
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    if g_settings_schema_key_type_check(&raw mut skey, value) == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_set_value: key '%s' in '%s' expects type '%s', but a GVariant of type '%s' was given\0"
                as *const u8 as *const gchar,
            key,
            g_settings_schema_get_id((*(*settings).priv_0).schema),
            g_variant_type_peek_string(skey.type_0),
            g_variant_get_type_string(value),
        );
        success = FALSE as gboolean;
    } else if g_settings_schema_key_range_check(&raw mut skey, value) == 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"g_settings_set_value: value for key '%s' in schema '%s' is outside of valid range\0"
                as *const u8 as *const gchar,
            key,
            g_settings_schema_get_id((*(*settings).priv_0).schema),
        );
        success = FALSE as gboolean;
    } else {
        success = safe_c2rust_g_settings_write_to_backend(settings, &raw mut skey, value);
    }
    g_settings_schema_key_clear(&raw mut skey);
    g_variant_unref(value);
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut format: *const gchar,
    mut args: ...
) {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ap: ::core::ffi::VaList;
    value = safe_c2rust_g_settings_get_value(settings, key);
    if !strchr(format as *const ::core::ffi::c_char, '&' as i32).is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: the format string may not contain '&' (key '%s' from schema '%s'). This call will probably stop working with a future version of glib.\0"
                as *const u8 as *const gchar,
            b"g_settings_get\0" as *const u8 as *const ::core::ffi::c_char,
            key,
            g_settings_schema_get_id((*(*settings).priv_0).schema),
        );
    }
    ap = args.clone();
    g_variant_get_va(
        value,
        format,
        ::core::ptr::null_mut::<*const gchar>(),
        &raw mut ap,
    );
    g_variant_unref(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut format: *const gchar,
    mut args: ...
) -> gboolean {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ap: ::core::ffi::VaList;
    ap = args.clone();
    value = g_variant_new_va(format, ::core::ptr::null_mut::<*const gchar>(), &raw mut ap);
    return safe_c2rust_g_settings_set_value(
        settings,
        key,
        safe_c2rust_g_steal_pointer(&raw mut value as gpointer) as *mut GVariant,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_mapped(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut mapping: GSettingsGetMapping,
    mut user_data: gpointer,
) -> gpointer {
    let mut current_block: u64;
    let mut result: gpointer = NULL_0;
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut okay: gboolean = 0;
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if mapping.is_some() {
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
            b"mapping != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    value = safe_c2rust_g_settings_read_from_backend(settings, &raw mut skey, FALSE, FALSE);
    if !value.is_null() {
        okay = mapping.expect("non-null function pointer")(value, &raw mut result, user_data);
        g_variant_unref(value);
        if okay != 0 {
            current_block = 3544203180049719045;
        } else {
            current_block = 18317007320854588510;
        }
    } else {
        current_block = 18317007320854588510;
    }
    match current_block {
        18317007320854588510 => {
            value = g_settings_schema_key_get_translated_default(&raw mut skey);
            if !value.is_null() {
                okay =
                    mapping.expect("non-null function pointer")(value, &raw mut result, user_data);
                g_variant_unref(value);
                if okay != 0 {
                    current_block = 3544203180049719045;
                } else {
                    current_block = 6669252993407410313;
                }
            } else {
                current_block = 6669252993407410313;
            }
            match current_block {
                3544203180049719045 => {}
                _ => {
                    value = g_settings_schema_key_get_per_desktop_default(&raw mut skey);
                    if !value.is_null() {
                        okay = mapping.expect("non-null function pointer")(
                            value,
                            &raw mut result,
                            user_data,
                        );
                        g_variant_unref(value);
                        if okay != 0 {
                            current_block = 3544203180049719045;
                        } else {
                            current_block = 4488286894823169796;
                        }
                    } else {
                        current_block = 4488286894823169796;
                    }
                    match current_block {
                        3544203180049719045 => {}
                        _ => {
                            if !(mapping.expect("non-null function pointer")(
                                skey.default_value,
                                &raw mut result,
                                user_data,
                            ) != 0)
                            {
                                if mapping.expect("non-null function pointer")(
                                    ::core::ptr::null_mut::<GVariant>(),
                                    &raw mut result,
                                    user_data,
                                ) == 0
                                {
                                    g_log(
                                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                                        G_LOG_LEVEL_ERROR,
                                        b"The mapping function given to g_settings_get_mapped() for key '%s' in schema '%s' returned FALSE when given a NULL value.\0"
                                            as *const u8 as *const gchar,
                                        key,
                                        g_settings_schema_get_id((*(*settings).priv_0).schema),
                                    );
                                    loop {}
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    g_settings_schema_key_clear(&raw mut skey);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_string(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> *mut gchar {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    value = safe_c2rust_g_settings_get_value(settings, key);
    result = g_variant_dup_string(value, ::core::ptr::null_mut::<gsize>());
    g_variant_unref(value);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_string(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: *const gchar,
) -> gboolean {
    return safe_c2rust_g_settings_set_value(settings, key, g_variant_new_string(value));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_int(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> gint {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut result: gint = 0;
    value = safe_c2rust_g_settings_get_value(settings, key);
    result = g_variant_get_int32(value) as gint;
    g_variant_unref(value);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_int(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: gint,
) -> gboolean {
    return safe_c2rust_g_settings_set_value(settings, key, g_variant_new_int32(value as gint32));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_int64(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> gint64 {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut result: gint64 = 0;
    value = safe_c2rust_g_settings_get_value(settings, key);
    result = g_variant_get_int64(value);
    g_variant_unref(value);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_int64(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: gint64,
) -> gboolean {
    return safe_c2rust_g_settings_set_value(settings, key, g_variant_new_int64(value));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_uint(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> guint {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut result: guint = 0;
    value = safe_c2rust_g_settings_get_value(settings, key);
    result = g_variant_get_uint32(value) as guint;
    g_variant_unref(value);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_uint(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: guint,
) -> gboolean {
    return safe_c2rust_g_settings_set_value(settings, key, g_variant_new_uint32(value as guint32));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_uint64(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> guint64 {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut result: guint64 = 0;
    value = safe_c2rust_g_settings_get_value(settings, key);
    result = g_variant_get_uint64(value);
    g_variant_unref(value);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_uint64(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: guint64,
) -> gboolean {
    return safe_c2rust_g_settings_set_value(settings, key, g_variant_new_uint64(value));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_double(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> gdouble {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut result: gdouble = 0.;
    value = safe_c2rust_g_settings_get_value(settings, key);
    result = g_variant_get_double(value);
    g_variant_unref(value);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_double(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: gdouble,
) -> gboolean {
    return safe_c2rust_g_settings_set_value(settings, key, g_variant_new_double(value));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_boolean(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> gboolean {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut result: gboolean = 0;
    value = safe_c2rust_g_settings_get_value(settings, key);
    result = g_variant_get_boolean(value);
    g_variant_unref(value);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_boolean(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: gboolean,
) -> gboolean {
    return safe_c2rust_g_settings_set_value(settings, key, g_variant_new_boolean(value));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_strv(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> *mut *mut gchar {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut result: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    value = safe_c2rust_g_settings_get_value(settings, key);
    result = g_variant_dup_strv(value, ::core::ptr::null_mut::<gsize>());
    g_variant_unref(value);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_set_strv(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: *const *const gchar,
) -> gboolean {
    let mut array: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if !value.is_null() {
        array = g_variant_new_strv(value, -(1 as ::core::ffi::c_int) as gssize);
    } else {
        array = g_variant_new_strv(::core::ptr::null::<*const gchar>(), 0 as gssize);
    }
    return safe_c2rust_g_settings_set_value(settings, key, array);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_delay(mut settings: *mut GSettings) {
    let mut delayed: *mut GDelayedSettingsBackend =
        ::core::ptr::null_mut::<GDelayedSettingsBackend>();
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut __inst: *mut GTypeInstance = (*(*settings).priv_0).backend as *mut GTypeInstance;
        let mut __t: GType = g_delayed_settings_backend_get_type();
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
        return;
    }
    delayed = g_delayed_settings_backend_new(
        (*(*settings).priv_0).backend,
        settings as gpointer,
        (*(*settings).priv_0).main_context,
    );
    g_settings_backend_unwatch(
        (*(*settings).priv_0).backend,
        settings as *mut ::core::ffi::c_void as *mut GObject,
    );
    g_object_unref((*(*settings).priv_0).backend as gpointer);
    (*(*settings).priv_0).backend = delayed as *mut ::core::ffi::c_void as *mut GSettingsBackend;
    g_settings_backend_watch(
        (*(*settings).priv_0).backend,
        &raw const safe_c2rust_listener_vtable,
        settings as *mut ::core::ffi::c_void as *mut GObject,
        (*(*settings).priv_0).main_context,
    );
    g_object_notify(
        settings as *mut ::core::ffi::c_void as *mut GObject,
        b"delay-apply\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_apply(mut settings: *mut GSettings) {
    if ({
        let mut __inst: *mut GTypeInstance = (*(*settings).priv_0).backend as *mut GTypeInstance;
        let mut __t: GType = g_delayed_settings_backend_get_type();
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
        let mut delayed: *mut GDelayedSettingsBackend =
            ::core::ptr::null_mut::<GDelayedSettingsBackend>();
        delayed = (*(*settings).priv_0).backend as *mut ::core::ffi::c_void
            as *mut GDelayedSettingsBackend;
        g_delayed_settings_backend_apply(delayed);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_revert(mut settings: *mut GSettings) {
    if ({
        let mut __inst: *mut GTypeInstance = (*(*settings).priv_0).backend as *mut GTypeInstance;
        let mut __t: GType = g_delayed_settings_backend_get_type();
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
        let mut delayed: *mut GDelayedSettingsBackend =
            ::core::ptr::null_mut::<GDelayedSettingsBackend>();
        delayed = (*(*settings).priv_0).backend as *mut ::core::ffi::c_void
            as *mut GDelayedSettingsBackend;
        g_delayed_settings_backend_revert(delayed);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_has_unapplied(
    mut settings: *mut GSettings,
) -> gboolean {
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (({
        let mut __inst: *mut GTypeInstance = (*(*settings).priv_0).backend as *mut GTypeInstance;
        let mut __t: GType = g_delayed_settings_backend_get_type();
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
        && g_delayed_settings_backend_get_has_unapplied(
            (*(*settings).priv_0).backend as *mut ::core::ffi::c_void
                as *mut GDelayedSettingsBackend,
        ) != 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_reset(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) {
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    path = g_strconcat((*(*settings).priv_0).path, key, NULL_0);
    g_settings_backend_reset((*(*settings).priv_0).backend, path, NULL_0);
    g_free(path as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_sync() {
    g_settings_backend_sync_default();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_is_writable(
    mut settings: *mut GSettings,
    mut name: *const gchar,
) -> gboolean {
    let mut writable: gboolean = 0;
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    path = g_strconcat((*(*settings).priv_0).path, name, NULL_0);
    writable = g_settings_backend_get_writable((*(*settings).priv_0).backend, path);
    g_free(path as gpointer);
    return writable;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_child(
    mut settings: *mut GSettings,
    mut name: *const gchar,
) -> *mut GSettings {
    let mut child_schema: *mut GSettingsSchema = ::core::ptr::null_mut::<GSettingsSchema>();
    let mut child_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut child: *mut GSettings = ::core::ptr::null_mut::<GSettings>();
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSettings>();
    }
    child_schema = g_settings_schema_get_child_schema((*(*settings).priv_0).schema, name);
    if child_schema.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"Schema '%s' has no child '%s' or child schema not found\0" as *const u8
                as *const gchar,
            g_settings_schema_get_id((*(*settings).priv_0).schema),
            name,
        );
        loop {}
    }
    child_path = g_strconcat(
        (*(*settings).priv_0).path,
        name,
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    child =
        safe_c2rust_g_settings_new_full(child_schema, (*(*settings).priv_0).backend, child_path);
    g_settings_schema_unref(child_schema);
    g_free(child_path as gpointer);
    return child;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_list_keys(
    mut settings: *mut GSettings,
) -> *mut *mut gchar {
    return g_settings_schema_list_keys((*(*settings).priv_0).schema);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_list_children(
    mut settings: *mut GSettings,
) -> *mut *mut gchar {
    return g_settings_schema_list_children((*(*settings).priv_0).schema);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_get_range(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> *mut GVariant {
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut range: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    range = g_settings_schema_key_get_range(&raw mut skey);
    g_settings_schema_key_clear(&raw mut skey);
    return range;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_range_check(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut value: *mut GVariant,
) -> gboolean {
    let mut skey: GSettingsSchemaKey = _GSettingsSchemaKey {
        schema: ::core::ptr::null_mut::<GSettingsSchema>(),
        name: ::core::ptr::null::<gchar>(),
        is_flags_is_enum: [0; 1],
        c2rust_padding: [0; 7],
        strinfo: ::core::ptr::null::<guint32>(),
        strinfo_length: 0,
        unparsed: ::core::ptr::null::<gchar>(),
        lc_char: 0,
        type_0: ::core::ptr::null::<GVariantType>(),
        minimum: ::core::ptr::null_mut::<GVariant>(),
        maximum: ::core::ptr::null_mut::<GVariant>(),
        default_value: ::core::ptr::null_mut::<GVariant>(),
        desktop_overrides: ::core::ptr::null_mut::<GVariant>(),
        ref_count: 0,
    };
    let mut good: gboolean = 0;
    g_settings_schema_key_init(&raw mut skey, (*(*settings).priv_0).schema, key);
    good = g_settings_schema_key_range_check(&raw mut skey, value);
    g_settings_schema_key_clear(&raw mut skey);
    return good;
}
unsafe extern "C" fn safe_c2rust_g_settings_binding_free(mut data: gpointer) {
    let mut binding: *mut GSettingsBinding = data as *mut GSettingsBinding;
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if (*binding).running == 0 {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2571 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!binding->running\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*binding).writable_handler_id != 0 {
        g_signal_handler_disconnect(
            (*binding).settings as gpointer,
            (*binding).writable_handler_id as gulong,
        );
    }
    if (*binding).key_handler_id != 0 {
        g_signal_handler_disconnect(
            (*binding).settings as gpointer,
            (*binding).key_handler_id as gulong,
        );
    }
    if g_signal_handler_is_connected(
        (*binding).object as gpointer,
        (*binding).property_handler_id as gulong,
    ) != 0
    {
        g_signal_handler_disconnect(
            (*binding).object as gpointer,
            (*binding).property_handler_id as gulong,
        );
    }
    g_settings_schema_key_clear(&raw mut (*binding).key);
    if (*binding).destroy.is_some() {
        (*binding).destroy.expect("non-null function pointer")((*binding).user_data);
    }
    g_object_unref((*binding).settings as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GSettingsBinding>() as gsize,
        binding as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_settings_binding_quark(
    mut property: *const ::core::ffi::c_char,
) -> GQuark {
    let mut quark: GQuark = 0;
    let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    tmp = g_strdup_printf(
        b"gsettingsbinding-%s\0" as *const u8 as *const gchar,
        property,
    );
    quark = g_quark_from_string(tmp);
    g_free(tmp as gpointer);
    return quark;
}
unsafe extern "C" fn safe_c2rust_g_settings_binding_key_changed(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut user_data: gpointer,
) {
    let mut binding: *mut GSettingsBinding = user_data as *mut GSettingsBinding;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if settings == (*binding).settings {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2618 as ::core::ffi::c_int,
            G_STRFUNC,
            b"settings == binding->settings\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if key == (*binding).key.name {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2619 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key == binding->key.name\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*binding).running != 0 {
        return;
    }
    (*binding).running = TRUE as gboolean;
    g_value_init(&raw mut value, (*(*binding).property).value_type);
    variant = safe_c2rust_g_settings_read_from_backend(
        (*binding).settings,
        &raw mut (*binding).key,
        FALSE,
        FALSE,
    );
    if !variant.is_null()
        && (*binding).get_mapping.expect("non-null function pointer")(
            &raw mut value,
            variant,
            (*binding).user_data,
        ) == 0
    {
        g_variant_unref(variant);
        variant = ::core::ptr::null_mut::<GVariant>();
    }
    if variant.is_null() {
        variant = g_settings_schema_key_get_translated_default(&raw mut (*binding).key);
        if !variant.is_null()
            && (*binding).get_mapping.expect("non-null function pointer")(
                &raw mut value,
                variant,
                (*binding).user_data,
            ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Translated default '%s' for key '%s' in schema '%s' was rejected by the binding mapping function\0"
                    as *const u8 as *const gchar,
                (*binding).key.unparsed,
                (*binding).key.name,
                g_settings_schema_get_id((*binding).key.schema),
            );
            g_variant_unref(variant);
            variant = ::core::ptr::null_mut::<GVariant>();
        }
    }
    if variant.is_null() {
        variant = g_settings_schema_key_get_per_desktop_default(&raw mut (*binding).key);
        if !variant.is_null()
            && (*binding).get_mapping.expect("non-null function pointer")(
                &raw mut value,
                variant,
                (*binding).user_data,
            ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"Per-desktop default value for key '%s' in schema '%s' was rejected by the binding mapping function.\0"
                    as *const u8 as *const gchar,
                (*binding).key.name,
                g_settings_schema_get_id((*binding).key.schema),
            );
            loop {}
        }
    }
    if variant.is_null() {
        variant = g_variant_ref((*binding).key.default_value);
        if (*binding).get_mapping.expect("non-null function pointer")(
            &raw mut value,
            variant,
            (*binding).user_data,
        ) == 0
        {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"The schema default value for key '%s' in schema '%s' was rejected by the binding mapping function.\0"
                    as *const u8 as *const gchar,
                (*binding).key.name,
                g_settings_schema_get_id((*binding).key.schema),
            );
            loop {}
        }
    }
    g_object_set_property(
        (*binding).object,
        (*(*binding).property).name,
        &raw mut value,
    );
    g_variant_unref(variant);
    g_value_unset(&raw mut value);
    (*binding).running = FALSE as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_settings_binding_property_changed(
    mut object: *mut GObject,
    mut pspec: *const GParamSpec,
    mut user_data: gpointer,
) {
    let mut binding: *mut GSettingsBinding = user_data as *mut GSettingsBinding;
    let mut value: GValue = _GValue {
        g_type: 0 as GType,
        data: [
            C2RustUnnamed {
                v_int: 0 as ::core::ffi::c_int,
            },
            C2RustUnnamed { v_int: 0 },
        ],
    };
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut valid: gboolean = TRUE;
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if object == (*binding).object {
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2692 as ::core::ffi::c_int,
            G_STRFUNC,
            b"object == binding->object\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if pspec == (*binding).property {
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2693 as ::core::ffi::c_int,
            G_STRFUNC,
            b"pspec == binding->property\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*binding).running != 0 {
        return;
    }
    (*binding).running = TRUE as gboolean;
    g_value_init(&raw mut value, (*pspec).value_type);
    g_object_get_property(object, (*pspec).name, &raw mut value);
    variant = (*binding).set_mapping.expect("non-null function pointer")(
        &raw mut value,
        (*binding).key.type_0,
        (*binding).user_data,
    );
    if !variant.is_null() {
        g_variant_take_ref(variant);
        if g_settings_schema_key_type_check(&raw mut (*binding).key, variant) == 0 {
            let mut type_str: *mut gchar = ::core::ptr::null_mut::<gchar>();
            type_str = g_variant_type_dup_string((*binding).key.type_0);
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"binding mapping function for key '%s' returned GVariant of type '%s' when type '%s' was requested\0"
                    as *const u8 as *const gchar,
                (*binding).key.name,
                g_variant_get_type_string(variant),
                type_str,
            );
            g_free(type_str as gpointer);
            valid = FALSE as gboolean;
        }
        if valid != 0 && g_settings_schema_key_range_check(&raw mut (*binding).key, variant) == 0 {
            let mut variant_str: *mut gchar = ::core::ptr::null_mut::<gchar>();
            variant_str = g_variant_print(variant, TRUE);
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"GObject property '%s' on a '%s' object is out of schema-specified range for key '%s' of '%s': %s\0"
                    as *const u8 as *const gchar,
                (*(*binding).property).name,
                g_type_name((*(*binding).property).owner_type),
                (*binding).key.name,
                g_settings_schema_get_id((*binding).key.schema),
                variant_str,
            );
            g_free(variant_str as gpointer);
            valid = FALSE as gboolean;
        }
        if valid != 0 {
            safe_c2rust_g_settings_write_to_backend(
                (*binding).settings,
                &raw mut (*binding).key,
                variant,
            );
        }
        g_variant_unref(variant);
    }
    g_value_unset(&raw mut value);
    (*binding).running = FALSE as gboolean;
}
unsafe extern "C" fn safe_c2rust_g_settings_bind_invert_boolean_get_mapping(
    mut value: *mut GValue,
    mut variant: *mut GVariant,
    mut user_data: gpointer,
) -> gboolean {
    g_value_set_boolean(
        value,
        (g_variant_get_boolean(variant) == 0) as ::core::ffi::c_int,
    );
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_settings_bind_invert_boolean_set_mapping(
    mut value: *const GValue,
    mut expected_type: *const GVariantType,
    mut user_data: gpointer,
) -> *mut GVariant {
    return g_variant_new_boolean((g_value_get_boolean(value) == 0) as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_bind(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut object: gpointer,
    mut property: *const gchar,
    mut flags: GSettingsBindFlags,
) {
    let mut get_mapping: GSettingsBindGetMapping = None;
    let mut set_mapping: GSettingsBindSetMapping = None;
    if flags as ::core::ffi::c_uint
        & G_SETTINGS_BIND_INVERT_BOOLEAN as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        get_mapping = Some(
            safe_c2rust_g_settings_bind_invert_boolean_get_mapping
                as unsafe extern "C" fn(*mut GValue, *mut GVariant, gpointer) -> gboolean,
        ) as GSettingsBindGetMapping;
        set_mapping = Some(
            safe_c2rust_g_settings_bind_invert_boolean_set_mapping
                as unsafe extern "C" fn(
                    *const GValue,
                    *const GVariantType,
                    gpointer,
                ) -> *mut GVariant,
        ) as GSettingsBindSetMapping;
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GSettingsBindFlags>(
            flags as ::core::ffi::c_uint
                & !(G_SETTINGS_BIND_INVERT_BOOLEAN as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    }
    safe_c2rust_g_settings_bind_with_mapping(
        settings,
        key,
        object,
        property,
        flags,
        get_mapping,
        set_mapping,
        NULL_0,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_bind_with_mapping(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut object: gpointer,
    mut property: *const gchar,
    mut flags: GSettingsBindFlags,
    mut get_mapping: GSettingsBindGetMapping,
    mut set_mapping: GSettingsBindSetMapping,
    mut user_data: gpointer,
    mut destroy: GDestroyNotify,
) {
    let mut binding: *mut GSettingsBinding = ::core::ptr::null_mut::<GSettingsBinding>();
    let mut objectclass: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    let mut detailed_signal: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut binding_quark: GQuark = 0;
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if g_type_check_instance_is_fundamentally_a(
            object as *mut GTypeInstance,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
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
            b"G_IS_OBJECT (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if !property.is_null() {
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
            b"property != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if !(flags as ::core::ffi::c_uint)
            & G_SETTINGS_BIND_INVERT_BOOLEAN as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
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
            b"~flags & G_SETTINGS_BIND_INVERT_BOOLEAN\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    objectclass = (*(object as *mut GTypeInstance)).g_class as *mut GObjectClass;
    binding = ({
        let mut __s: gsize = ::core::mem::size_of::<GSettingsBinding>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GSettingsBinding;
    g_settings_schema_key_init(&raw mut (*binding).key, (*(*settings).priv_0).schema, key);
    (*binding).settings = g_object_ref(settings as gpointer) as *mut GSettings as *mut GSettings;
    (*binding).object = object as *mut GObject;
    (*binding).property = g_object_class_find_property(objectclass, property);
    (*binding).user_data = user_data;
    (*binding).destroy = destroy;
    (*binding).get_mapping = (if get_mapping.is_some() {
        get_mapping
            as Option<unsafe extern "C" fn(*mut GValue, *mut GVariant, gpointer) -> gboolean>
    } else {
        Some(
            g_settings_get_mapping
                as unsafe extern "C" fn(*mut GValue, *mut GVariant, gpointer) -> gboolean,
        )
    }) as GSettingsBindGetMapping;
    (*binding).set_mapping = (if set_mapping.is_some() {
        set_mapping
            as Option<
                unsafe extern "C" fn(*const GValue, *const GVariantType, gpointer) -> *mut GVariant,
            >
    } else {
        Some(
            g_settings_set_mapping
                as unsafe extern "C" fn(
                    *const GValue,
                    *const GVariantType,
                    gpointer,
                ) -> *mut GVariant,
        )
    }) as GSettingsBindSetMapping;
    if flags as ::core::ffi::c_uint
        & (G_SETTINGS_BIND_GET as ::core::ffi::c_int | G_SETTINGS_BIND_SET as ::core::ffi::c_int)
            as ::core::ffi::c_uint
        == 0
    {
        flags = ::core::mem::transmute::<::core::ffi::c_uint, GSettingsBindFlags>(
            flags as ::core::ffi::c_uint
                | (G_SETTINGS_BIND_GET as ::core::ffi::c_int
                    | G_SETTINGS_BIND_SET as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
        );
    }
    if (*binding).property.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_bind: no property '%s' on class '%s'\0" as *const u8 as *const gchar,
            property,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
        );
        return;
    }
    if flags as ::core::ffi::c_uint
        & G_SETTINGS_BIND_GET as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && (*(*binding).property).flags as ::core::ffi::c_int
            & G_PARAM_WRITABLE as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_bind: property '%s' on class '%s' is not writable\0" as *const u8
                as *const gchar,
            (*(*binding).property).name,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
        );
        return;
    }
    if flags as ::core::ffi::c_uint
        & G_SETTINGS_BIND_SET as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && (*(*binding).property).flags as ::core::ffi::c_int
            & G_PARAM_READABLE as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_bind: property '%s' on class '%s' is not readable\0" as *const u8
                as *const gchar,
            (*(*binding).property).name,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
        );
        return;
    }
    if get_mapping
        == Some(
            safe_c2rust_g_settings_bind_invert_boolean_get_mapping
                as unsafe extern "C" fn(*mut GValue, *mut GVariant, gpointer) -> gboolean,
        )
    {
        if (*(*binding).property).value_type != G_TYPE_BOOLEAN {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_settings_bind: G_SETTINGS_BIND_INVERT_BOOLEAN was specified, but property '%s' on type '%s' has type '%s'\0"
                    as *const u8 as *const gchar,
                (*(*binding).property).name,
                g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*binding).property).value_type),
            );
            return;
        }
        if g_variant_type_equal(
            (*binding).key.type_0 as gconstpointer,
            G_VARIANT_TYPE_BOOLEAN as gconstpointer,
        ) == 0
        {
            let mut type_string: *mut gchar = g_variant_type_dup_string((*binding).key.type_0);
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_settings_bind: G_SETTINGS_BIND_INVERT_BOOLEAN was specified, but key '%s' on schema '%s' has type '%s'\0"
                    as *const u8 as *const gchar,
                key,
                g_settings_schema_get_id((*(*settings).priv_0).schema),
                type_string,
            );
            g_free(type_string as gpointer);
            return;
        }
    } else if (get_mapping.is_none()
        && flags as ::core::ffi::c_uint
            & G_SETTINGS_BIND_GET as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        || set_mapping.is_none()
            && flags as ::core::ffi::c_uint
                & G_SETTINGS_BIND_SET as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0)
        && g_settings_mapping_is_compatible(
            (*(*binding).property).value_type,
            (*binding).key.type_0,
        ) == 0
    {
        let mut type_string_0: *mut gchar = g_variant_type_dup_string((*binding).key.type_0);
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_bind: property '%s' on class '%s' has type '%s' which is not compatible with type '%s' of key '%s' on schema '%s'\0"
                as *const u8 as *const gchar,
            (*(*binding).property).name,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
            g_type_name((*(*binding).property).value_type),
            type_string_0,
            key,
            g_settings_schema_get_id((*(*settings).priv_0).schema),
        );
        g_free(type_string_0 as gpointer);
        return;
    }
    if flags as ::core::ffi::c_uint
        & G_SETTINGS_BIND_SET as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && !(flags as ::core::ffi::c_uint)
            & G_SETTINGS_BIND_NO_SENSITIVITY as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        let mut sensitive: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
        sensitive =
            g_object_class_find_property(objectclass, b"sensitive\0" as *const u8 as *const gchar);
        if !sensitive.is_null()
            && (*sensitive).value_type == G_TYPE_BOOLEAN
            && (*sensitive).flags as ::core::ffi::c_int & G_PARAM_WRITABLE as ::core::ffi::c_int
                != 0
        {
            safe_c2rust_g_settings_bind_writable(
                settings,
                (*binding).key.name,
                object,
                b"sensitive\0" as *const u8 as *const gchar,
                FALSE,
            );
        }
    }
    if flags as ::core::ffi::c_uint
        & G_SETTINGS_BIND_SET as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        detailed_signal = g_strdup_printf(
            b"notify::%s\0" as *const u8 as *const gchar,
            (*(*binding).property).name,
        );
        (*binding).property_handler_id = g_signal_connect_data(
            object,
            detailed_signal,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GObject, *const GParamSpec, gpointer) -> ()>,
                GCallback,
            >(Some(
                safe_c2rust_g_settings_binding_property_changed
                    as unsafe extern "C" fn(*mut GObject, *const GParamSpec, gpointer) -> (),
            )),
            binding as gpointer,
            None,
            G_CONNECT_DEFAULT,
        ) as guint;
        g_free(detailed_signal as gpointer);
        if !(flags as ::core::ffi::c_uint)
            & G_SETTINGS_BIND_GET as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            safe_c2rust_g_settings_binding_property_changed(
                object as *mut GObject,
                (*binding).property,
                binding as gpointer,
            );
        }
    }
    if flags as ::core::ffi::c_uint
        & G_SETTINGS_BIND_GET as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        if !(flags as ::core::ffi::c_uint)
            & G_SETTINGS_BIND_GET_NO_CHANGES as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            detailed_signal = g_strdup_printf(b"changed::%s\0" as *const u8 as *const gchar, key);
            (*binding).key_handler_id = g_signal_connect_data(
                settings as gpointer,
                detailed_signal,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GSettings, *const gchar, gpointer) -> ()>,
                    GCallback,
                >(Some(
                    safe_c2rust_g_settings_binding_key_changed
                        as unsafe extern "C" fn(*mut GSettings, *const gchar, gpointer) -> (),
                )),
                binding as gpointer,
                None,
                G_CONNECT_DEFAULT,
            ) as guint;
            g_free(detailed_signal as gpointer);
        }
        safe_c2rust_g_settings_binding_key_changed(
            settings,
            (*binding).key.name,
            binding as gpointer,
        );
    }
    binding_quark = safe_c2rust_g_settings_binding_quark(
        (*(*binding).property).name as *const ::core::ffi::c_char,
    );
    g_object_set_qdata_full(
        object as *mut GObject,
        binding_quark,
        binding as gpointer,
        Some(safe_c2rust_g_settings_binding_free as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_g_settings_writable_binding_free(mut data: gpointer) {
    let mut binding: *mut GSettingsWritableBinding = data as *mut GSettingsWritableBinding;
    g_signal_handler_disconnect((*binding).settings as gpointer, (*binding).handler_id);
    g_object_unref((*binding).settings as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GSettingsWritableBinding>() as gsize,
        binding as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_settings_binding_writable_changed(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut user_data: gpointer,
) {
    let mut binding: *mut GSettingsWritableBinding = user_data as *mut GSettingsWritableBinding;
    let mut writable: gboolean = 0;
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if settings == (*binding).settings {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0" as *const u8
                as *const ::core::ffi::c_char,
            3023 as ::core::ffi::c_int,
            G_STRFUNC,
            b"settings == binding->settings\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if key == (*binding).key {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0" as *const u8
                as *const ::core::ffi::c_char,
            3024 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key == binding->key\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    writable = safe_c2rust_g_settings_is_writable(settings, key);
    if (*binding).inverted != 0 {
        writable = (writable == 0) as ::core::ffi::c_int as gboolean;
    }
    g_object_set((*binding).object, (*binding).property, writable, NULL_0);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_bind_writable(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut object: gpointer,
    mut property: *const gchar,
    mut inverted: gboolean,
) {
    let mut binding: *mut GSettingsWritableBinding =
        ::core::ptr::null_mut::<GSettingsWritableBinding>();
    let mut detailed_signal: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    pspec = g_object_class_find_property(
        (*(object as *mut GTypeInstance)).g_class as *mut GObjectClass,
        property,
    );
    if pspec.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_bind_writable: no property '%s' on class '%s'\0" as *const u8
                as *const gchar,
            property,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
        );
        return;
    }
    if (*pspec).flags as ::core::ffi::c_int & G_PARAM_WRITABLE as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_settings_bind_writable: property '%s' on class '%s' is not writable\0" as *const u8
                as *const gchar,
            property,
            g_type_name((*(*(object as *mut GTypeInstance)).g_class).g_type),
        );
        return;
    }
    binding = g_slice_alloc(::core::mem::size_of::<GSettingsWritableBinding>() as gsize)
        as *mut GSettingsWritableBinding;
    (*binding).settings = g_object_ref(settings as gpointer) as *mut GSettings as *mut GSettings;
    (*binding).object = object;
    (*binding).key = g_intern_string(key);
    (*binding).property = g_intern_string(property);
    (*binding).inverted = inverted;
    detailed_signal = g_strdup_printf(b"writable-changed::%s\0" as *const u8 as *const gchar, key);
    (*binding).handler_id = g_signal_connect_data(
        settings as gpointer,
        detailed_signal,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSettings, *const gchar, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_g_settings_binding_writable_changed
                as unsafe extern "C" fn(*mut GSettings, *const gchar, gpointer) -> (),
        )),
        binding as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_free(detailed_signal as gpointer);
    g_object_set_qdata_full(
        object as *mut GObject,
        safe_c2rust_g_settings_binding_quark(property as *const ::core::ffi::c_char),
        binding as gpointer,
        Some(safe_c2rust_g_settings_writable_binding_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    safe_c2rust_g_settings_binding_writable_changed(settings, (*binding).key, binding as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_unbind(
    mut object: gpointer,
    mut property: *const gchar,
) {
    let mut binding_quark: GQuark = 0;
    binding_quark = safe_c2rust_g_settings_binding_quark(property as *const ::core::ffi::c_char);
    g_object_set_qdata(object as *mut GObject, binding_quark, NULL_0);
}
static mut safe_c2rust_g_settings_action_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GSettingsAction_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_settings_action_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_settings_action_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_settings_action_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSettingsAction\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSettingsActionClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_settings_action_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSettingsAction>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSettingsAction) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_settings_action_init
                    as unsafe extern "C" fn(*mut GSettingsAction) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_settings_action_iface_init
                as unsafe extern "C" fn(*mut GActionInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_action_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_settings_action_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_settings_action_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSettingsAction_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSettingsAction_private_offset,
        );
    }
    safe_c2rust_g_settings_action_class_init(klass as *mut GSettingsActionClass);
}
unsafe extern "C" fn safe_c2rust_g_settings_action_get_name(
    mut action: *mut GAction,
) -> *const gchar {
    let mut gsa: *mut GSettingsAction = action as *mut GSettingsAction;
    return (*gsa).key.name;
}
unsafe extern "C" fn safe_c2rust_g_settings_action_get_parameter_type(
    mut action: *mut GAction,
) -> *const GVariantType {
    let mut gsa: *mut GSettingsAction = action as *mut GSettingsAction;
    let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
    type_0 = g_variant_get_type((*gsa).key.default_value);
    if g_variant_type_equal(
        type_0 as gconstpointer,
        G_VARIANT_TYPE_BOOLEAN as gconstpointer,
    ) != 0
    {
        type_0 = ::core::ptr::null::<GVariantType>();
    }
    return type_0;
}
unsafe extern "C" fn safe_c2rust_g_settings_action_get_enabled(
    mut action: *mut GAction,
) -> gboolean {
    let mut gsa: *mut GSettingsAction = action as *mut GSettingsAction;
    return safe_c2rust_g_settings_is_writable((*gsa).settings, (*gsa).key.name);
}
unsafe extern "C" fn safe_c2rust_g_settings_action_get_state_type(
    mut action: *mut GAction,
) -> *const GVariantType {
    let mut gsa: *mut GSettingsAction = action as *mut GSettingsAction;
    return g_variant_get_type((*gsa).key.default_value);
}
unsafe extern "C" fn safe_c2rust_g_settings_action_get_state(
    mut action: *mut GAction,
) -> *mut GVariant {
    let mut gsa: *mut GSettingsAction = action as *mut GSettingsAction;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    value = safe_c2rust_g_settings_read_from_backend(
        (*gsa).settings,
        &raw mut (*gsa).key,
        FALSE,
        FALSE,
    );
    if value.is_null() {
        value = g_settings_schema_key_get_default_value(&raw mut (*gsa).key);
    }
    return value;
}
unsafe extern "C" fn safe_c2rust_g_settings_action_get_state_hint(
    mut action: *mut GAction,
) -> *mut GVariant {
    let mut gsa: *mut GSettingsAction = action as *mut GSettingsAction;
    return g_settings_schema_key_get_range(&raw mut (*gsa).key);
}
unsafe extern "C" fn safe_c2rust_g_settings_action_change_state(
    mut action: *mut GAction,
    mut value: *mut GVariant,
) {
    let mut gsa: *mut GSettingsAction = action as *mut GSettingsAction;
    if g_settings_schema_key_type_check(&raw mut (*gsa).key, value) != 0
        && g_settings_schema_key_range_check(&raw mut (*gsa).key, value) != 0
    {
        safe_c2rust_g_settings_write_to_backend((*gsa).settings, &raw mut (*gsa).key, value);
    }
}
unsafe extern "C" fn safe_c2rust_g_settings_action_activate(
    mut action: *mut GAction,
    mut parameter: *mut GVariant,
) {
    let mut gsa: *mut GSettingsAction = action as *mut GSettingsAction;
    if g_variant_is_of_type((*gsa).key.default_value, G_VARIANT_TYPE_BOOLEAN) != 0 {
        let mut old: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        if !parameter.is_null() {
            return;
        }
        old = safe_c2rust_g_settings_action_get_state(action);
        parameter = g_variant_new_boolean((g_variant_get_boolean(old) == 0) as ::core::ffi::c_int);
        g_variant_unref(old);
    }
    g_action_change_state(action, parameter);
}
unsafe extern "C" fn safe_c2rust_g_settings_action_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut action: *mut GAction = object as *mut ::core::ffi::c_void as *mut GAction;
    match prop_id {
        1 => {
            g_value_set_string(value, safe_c2rust_g_settings_action_get_name(action));
        }
        2 => {
            g_value_set_boxed(
                value,
                safe_c2rust_g_settings_action_get_parameter_type(action) as gconstpointer,
            );
        }
        3 => {
            g_value_set_boolean(value, safe_c2rust_g_settings_action_get_enabled(action));
        }
        4 => {
            g_value_set_boxed(
                value,
                safe_c2rust_g_settings_action_get_state_type(action) as gconstpointer,
            );
        }
        5 => {
            g_value_take_variant(value, safe_c2rust_g_settings_action_get_state(action));
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettings.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                3279 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_settings_action_finalize(mut object: *mut GObject) {
    let mut gsa: *mut GSettingsAction = object as *mut GSettingsAction;
    g_signal_handlers_disconnect_matched(
        (*gsa).settings as gpointer,
        G_SIGNAL_MATCH_DATA,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        NULL_0,
        gsa as gpointer,
    );
    g_object_unref((*gsa).settings as gpointer);
    g_settings_schema_key_clear(&raw mut (*gsa).key);
    (*(safe_c2rust_g_settings_action_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_settings_action_init(mut gsa: *mut GSettingsAction) {}
unsafe extern "C" fn safe_c2rust_g_settings_action_iface_init(mut iface: *mut GActionInterface) {
    (*iface).get_name = Some(
        safe_c2rust_g_settings_action_get_name
            as unsafe extern "C" fn(*mut GAction) -> *const gchar,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> *const gchar>;
    (*iface).get_parameter_type = Some(
        safe_c2rust_g_settings_action_get_parameter_type
            as unsafe extern "C" fn(*mut GAction) -> *const GVariantType,
    )
        as Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>;
    (*iface).get_enabled = Some(
        safe_c2rust_g_settings_action_get_enabled as unsafe extern "C" fn(*mut GAction) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> gboolean>;
    (*iface).get_state_type = Some(
        safe_c2rust_g_settings_action_get_state_type
            as unsafe extern "C" fn(*mut GAction) -> *const GVariantType,
    )
        as Option<unsafe extern "C" fn(*mut GAction) -> *const GVariantType>;
    (*iface).get_state = Some(
        safe_c2rust_g_settings_action_get_state
            as unsafe extern "C" fn(*mut GAction) -> *mut GVariant,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>;
    (*iface).get_state_hint = Some(
        safe_c2rust_g_settings_action_get_state_hint
            as unsafe extern "C" fn(*mut GAction) -> *mut GVariant,
    ) as Option<unsafe extern "C" fn(*mut GAction) -> *mut GVariant>;
    (*iface).change_state = Some(
        safe_c2rust_g_settings_action_change_state
            as unsafe extern "C" fn(*mut GAction, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>;
    (*iface).activate = Some(
        safe_c2rust_g_settings_action_activate
            as unsafe extern "C" fn(*mut GAction, *mut GVariant) -> (),
    ) as Option<unsafe extern "C" fn(*mut GAction, *mut GVariant) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_settings_action_class_init(
    mut class: *mut GSettingsActionClass,
) {
    (*class).get_property = Some(
        safe_c2rust_g_settings_action_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*class).finalize =
        Some(safe_c2rust_g_settings_action_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    g_object_class_override_property(
        class as *mut GObjectClass,
        ACTION_PROP_NAME as ::core::ffi::c_int as guint,
        b"name\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        class as *mut GObjectClass,
        ACTION_PROP_PARAMETER_TYPE as ::core::ffi::c_int as guint,
        b"parameter-type\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        class as *mut GObjectClass,
        ACTION_PROP_ENABLED as ::core::ffi::c_int as guint,
        b"enabled\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        class as *mut GObjectClass,
        ACTION_PROP_STATE_TYPE as ::core::ffi::c_int as guint,
        b"state-type\0" as *const u8 as *const gchar,
    );
    g_object_class_override_property(
        class as *mut GObjectClass,
        ACTION_PROP_STATE as ::core::ffi::c_int as guint,
        b"state\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_settings_action_changed(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut user_data: gpointer,
) {
    g_object_notify(
        user_data as *mut GObject,
        b"state\0" as *const u8 as *const gchar,
    );
}
unsafe extern "C" fn safe_c2rust_g_settings_action_enabled_changed(
    mut settings: *mut GSettings,
    mut key: *const gchar,
    mut user_data: gpointer,
) {
    g_object_notify(
        user_data as *mut GObject,
        b"enabled\0" as *const u8 as *const gchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_create_action(
    mut settings: *mut GSettings,
    mut key: *const gchar,
) -> *mut GAction {
    let mut gsa: *mut GSettingsAction = ::core::ptr::null_mut::<GSettingsAction>();
    let mut detailed_signal: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = settings as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_get_type();
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
            b"G_IS_SETTINGS (settings)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAction>();
    }
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAction>();
    }
    gsa = g_object_new(
        safe_c2rust_g_settings_action_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GSettingsAction;
    (*gsa).settings = g_object_ref(settings as gpointer) as *mut GSettings as *mut GSettings;
    g_settings_schema_key_init(&raw mut (*gsa).key, (*(*settings).priv_0).schema, key);
    detailed_signal = g_strdup_printf(b"changed::%s\0" as *const u8 as *const gchar, key);
    g_signal_connect_data(
        settings as gpointer,
        detailed_signal,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSettings, *const gchar, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_g_settings_action_changed
                as unsafe extern "C" fn(*mut GSettings, *const gchar, gpointer) -> (),
        )),
        gsa as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_free(detailed_signal as gpointer);
    detailed_signal = g_strdup_printf(b"writable-changed::%s\0" as *const u8 as *const gchar, key);
    g_signal_connect_data(
        settings as gpointer,
        detailed_signal,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GSettings, *const gchar, gpointer) -> ()>,
            GCallback,
        >(Some(
            safe_c2rust_g_settings_action_enabled_changed
                as unsafe extern "C" fn(*mut GSettings, *const gchar, gpointer) -> (),
        )),
        gsa as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_free(detailed_signal as gpointer);
    return gsa as *mut ::core::ffi::c_void as *mut GAction;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
