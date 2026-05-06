extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GRemoteActionGroup;
    pub type _GDBusConnection;
    pub type _GActionGroup;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
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
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
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
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_type_free(type_0: *mut GVariantType);
    fn g_variant_type_copy(type_0: *const GVariantType) -> *mut GVariantType;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type(value: *mut GVariant) -> *const GVariantType;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_n_children(value: *mut GVariant) -> gsize;
    fn g_variant_get_child(value: *mut GVariant, index_: gsize, format_string: *const gchar, ...);
    fn g_variant_equal(one: gconstpointer, two: gconstpointer) -> gboolean;
    fn g_variant_iter_free(iter: *mut GVariantIter);
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
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
    fn g_assertion_message_cmpint(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        arg1: guint64,
        cmp: *const ::core::ffi::c_char,
        arg2: guint64,
        numtype: ::core::ffi::c_char,
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_remote_action_group_get_type() -> GType;
    fn g_dbus_connection_get_unique_name(connection: *mut GDBusConnection) -> *const gchar;
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
    fn g_dbus_connection_call_sync(
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
    fn g_action_group_get_type() -> GType;
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
}
pub type size_t = usize;
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
pub type GDBusCallFlags = ::core::ffi::c_uint;
pub const G_DBUS_CALL_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusCallFlags = 2;
pub const G_DBUS_CALL_FLAGS_NO_AUTO_START: GDBusCallFlags = 1;
pub const G_DBUS_CALL_FLAGS_NONE: GDBusCallFlags = 0;
pub type GDBusSignalFlags = ::core::ffi::c_uint;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_PATH: GDBusSignalFlags = 4;
pub const G_DBUS_SIGNAL_FLAGS_MATCH_ARG0_NAMESPACE: GDBusSignalFlags = 2;
pub const G_DBUS_SIGNAL_FLAGS_NO_MATCH_RULE: GDBusSignalFlags = 1;
pub const G_DBUS_SIGNAL_FLAGS_NONE: GDBusSignalFlags = 0;
pub type GAsyncResult = _GAsyncResult;
pub type GCancellable = _GCancellable;
pub type GRemoteActionGroup = _GRemoteActionGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusActionGroup {
    pub parent_instance: GObject,
    pub connection: *mut GDBusConnection,
    pub bus_name: *mut gchar,
    pub object_path: *mut gchar,
    pub subscription_id: guint,
    pub actions: *mut GHashTable,
    pub strict: gboolean,
}
pub type GDBusConnection = _GDBusConnection;
pub type GDBusActionGroup = _GDBusActionGroup;
pub type GActionGroup = _GActionGroup;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GDBusActionGroupClass = GObjectClass;
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
pub struct ActionInfo {
    pub name: *mut gchar,
    pub parameter_type: *mut GVariantType,
    pub enabled: gboolean,
    pub state: *mut GVariant,
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
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
unsafe extern "C" fn safe_c2rust_action_info_free(mut user_data: gpointer) {
    let mut info: *mut ActionInfo = user_data as *mut ActionInfo;
    g_free((*info).name as gpointer);
    if !(*info).state.is_null() {
        g_variant_unref((*info).state);
    }
    if !(*info).parameter_type.is_null() {
        g_variant_type_free((*info).parameter_type);
    }
    g_slice_free1(
        ::core::mem::size_of::<ActionInfo>() as gsize,
        info as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_action_info_new_from_iter(
    mut iter: *mut GVariantIter,
) -> *mut ActionInfo {
    let mut param_str: *const gchar = ::core::ptr::null::<gchar>();
    let mut info: *mut ActionInfo = ::core::ptr::null_mut::<ActionInfo>();
    let mut enabled: gboolean = 0;
    let mut state: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if g_variant_iter_next(
        iter,
        b"{s(b&g@av)}\0" as *const u8 as *const gchar,
        &raw mut name,
        &raw mut enabled,
        &raw mut param_str,
        &raw mut state,
    ) == 0
    {
        return ::core::ptr::null_mut::<ActionInfo>();
    }
    info = g_slice_alloc(::core::mem::size_of::<ActionInfo>() as gsize) as *mut ActionInfo;
    (*info).name = name;
    (*info).enabled = enabled;
    if g_variant_n_children(state) != 0 {
        g_variant_get_child(
            state,
            0 as gsize,
            b"v\0" as *const u8 as *const gchar,
            &raw mut (*info).state,
        );
    } else {
        (*info).state = ::core::ptr::null_mut::<GVariant>();
    }
    g_variant_unref(state);
    if *param_str.offset(0 as ::core::ffi::c_int as isize) != 0 {
        (*info).parameter_type = g_variant_type_copy(param_str as *mut GVariantType);
    } else {
        (*info).parameter_type = ::core::ptr::null_mut::<GVariantType>();
    }
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_action_group_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_action_group_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusActionGroup\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusActionGroupClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_action_group_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusActionGroup>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusActionGroup) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_action_group_init
                    as unsafe extern "C" fn(*mut GDBusActionGroup) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroupInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_dbus_action_group_iface_init
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
            Option<unsafe extern "C" fn(*mut GRemoteActionGroupInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_dbus_action_group_remote_iface_init
                as unsafe extern "C" fn(*mut GRemoteActionGroupInterface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_remote_action_group_get_type(),
        &raw const g_implement_interface_info_0,
    );
    return g_define_type_id;
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_action_group_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusActionGroup_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusActionGroup_private_offset,
        );
    }
    safe_c2rust_g_dbus_action_group_class_init(klass as *mut GDBusActionGroupClass);
}
static mut safe_c2rust_g_dbus_action_group_parent_class: gpointer = NULL_0;
static mut safe_c2rust_GDBusActionGroup_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_changed(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut group: *mut GDBusActionGroup = user_data as *mut GDBusActionGroup;
    let mut g_group: *mut GActionGroup = user_data as *mut GActionGroup;
    if (*group).actions.is_null() {
        return;
    }
    if strcmp(
        signal_name as *const ::core::ffi::c_char,
        b"Changed\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_variant_is_of_type(
            parameters,
            g_variant_type_checked_(b"(asa{sb}a{sv}a{s(bgav)})\0" as *const u8 as *const gchar),
        ) != 0
    {
        let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
        let mut name: *const gchar = ::core::ptr::null::<gchar>();
        g_variant_get_child(
            parameters,
            0 as gsize,
            b"as\0" as *const u8 as *const gchar,
            &raw mut iter,
        );
        while g_variant_iter_next(iter, b"&s\0" as *const u8 as *const gchar, &raw mut name) != 0 {
            if !g_hash_table_lookup((*group).actions, name as gconstpointer).is_null() {
                g_hash_table_remove((*group).actions, name as gconstpointer);
                g_action_group_action_removed(g_group, name);
            }
        }
        g_variant_iter_free(iter);
        let mut iter_0: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
        let mut name_0: *const gchar = ::core::ptr::null::<gchar>();
        let mut enabled: gboolean = 0;
        g_variant_get_child(
            parameters,
            1 as gsize,
            b"a{sb}\0" as *const u8 as *const gchar,
            &raw mut iter_0,
        );
        while g_variant_iter_next(
            iter_0,
            b"{&sb}\0" as *const u8 as *const gchar,
            &raw mut name_0,
            &raw mut enabled,
        ) != 0
        {
            let mut info: *mut ActionInfo = ::core::ptr::null_mut::<ActionInfo>();
            info =
                g_hash_table_lookup((*group).actions, name_0 as gconstpointer) as *mut ActionInfo;
            if !info.is_null() && (*info).enabled != enabled {
                (*info).enabled = enabled;
                g_action_group_action_enabled_changed(g_group, name_0, enabled);
            }
        }
        g_variant_iter_free(iter_0);
        let mut iter_1: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
        let mut name_1: *const gchar = ::core::ptr::null::<gchar>();
        let mut state: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        g_variant_get_child(
            parameters,
            2 as gsize,
            b"a{sv}\0" as *const u8 as *const gchar,
            &raw mut iter_1,
        );
        while g_variant_iter_next(
            iter_1,
            b"{&sv}\0" as *const u8 as *const gchar,
            &raw mut name_1,
            &raw mut state,
        ) != 0
        {
            let mut info_0: *mut ActionInfo = ::core::ptr::null_mut::<ActionInfo>();
            info_0 =
                g_hash_table_lookup((*group).actions, name_1 as gconstpointer) as *mut ActionInfo;
            if !info_0.is_null()
                && !(*info_0).state.is_null()
                && g_variant_equal(state as gconstpointer, (*info_0).state as gconstpointer) == 0
                && g_variant_is_of_type(state, g_variant_get_type((*info_0).state)) != 0
            {
                g_variant_unref((*info_0).state);
                (*info_0).state = g_variant_ref(state);
                g_action_group_action_state_changed(g_group, name_1, state);
            }
            g_variant_unref(state);
        }
        g_variant_iter_free(iter_1);
        let mut iter_2: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
        let mut info_1: *mut ActionInfo = ::core::ptr::null_mut::<ActionInfo>();
        g_variant_get_child(
            parameters,
            3 as gsize,
            b"a{s(bgav)}\0" as *const u8 as *const gchar,
            &raw mut iter_2,
        );
        loop {
            info_1 = safe_c2rust_action_info_new_from_iter(iter_2);
            if info_1.is_null() {
                break;
            }
            if g_hash_table_lookup((*group).actions, (*info_1).name as gconstpointer).is_null() {
                g_hash_table_insert(
                    (*group).actions,
                    (*info_1).name as gpointer,
                    info_1 as gpointer,
                );
                if (*group).strict != 0 {
                    g_action_group_action_added(g_group, (*info_1).name);
                }
            } else {
                safe_c2rust_action_info_free(info_1 as gpointer);
            }
        }
        g_variant_iter_free(iter_2);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_describe_all_done(
    mut source: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut group: *mut GDBusActionGroup = user_data as *mut GDBusActionGroup;
    let mut reply: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*group).actions.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusactiongroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_int,
            G_STRFUNC,
            b"group->actions == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*group).actions = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        Some(safe_c2rust_action_info_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*group).connection == source as gpointer as *mut GDBusConnection {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusactiongroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            251 as ::core::ffi::c_int,
            G_STRFUNC,
            b"group->connection == (gpointer) source\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    reply = g_dbus_connection_call_finish(
        (*group).connection,
        result,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !reply.is_null() {
        let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
        let mut action: *mut ActionInfo = ::core::ptr::null_mut::<ActionInfo>();
        g_variant_get(
            reply,
            b"(a{s(bgav)})\0" as *const u8 as *const gchar,
            &raw mut iter,
        );
        loop {
            action = safe_c2rust_action_info_new_from_iter(iter);
            if action.is_null() {
                break;
            }
            g_hash_table_insert(
                (*group).actions,
                (*action).name as gpointer,
                action as gpointer,
            );
            if (*group).strict != 0 {
                g_action_group_action_added(
                    group as *mut ::core::ffi::c_void as *mut GActionGroup,
                    (*action).name,
                );
            }
        }
        g_variant_iter_free(iter);
        g_variant_unref(reply);
    }
    g_object_unref(group as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_async_init(mut group: *mut GDBusActionGroup) {
    if (*group).subscription_id != 0 as guint {
        return;
    }
    (*group).subscription_id = g_dbus_connection_signal_subscribe(
        (*group).connection,
        (*group).bus_name,
        b"org.gtk.Actions\0" as *const u8 as *const gchar,
        b"Changed\0" as *const u8 as *const gchar,
        (*group).object_path,
        ::core::ptr::null::<gchar>(),
        G_DBUS_SIGNAL_FLAGS_NONE,
        Some(
            safe_c2rust_g_dbus_action_group_changed
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
        group as gpointer,
        None,
    );
    g_dbus_connection_call(
        (*group).connection,
        (*group).bus_name,
        (*group).object_path,
        b"org.gtk.Actions\0" as *const u8 as *const gchar,
        b"DescribeAll\0" as *const u8 as *const gchar,
        ::core::ptr::null_mut::<GVariant>(),
        g_variant_type_checked_(b"(a{s(bgav)})\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        Some(
            safe_c2rust_g_dbus_action_group_describe_all_done
                as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
        ),
        g_object_ref(group as gpointer) as *mut GDBusActionGroup as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_list_actions(
    mut g_group: *mut GActionGroup,
) -> *mut *mut gchar {
    let mut group: *mut GDBusActionGroup =
        g_group as *mut ::core::ffi::c_void as *mut GDBusActionGroup;
    let mut keys: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    if !(*group).actions.is_null() {
        let mut iter: GHashTableIter = _GHashTableIter {
            dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy4: 0,
            dummy5: 0,
            dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        let mut n: gint = 0;
        let mut i: gint = 0 as gint;
        let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        n = g_hash_table_size((*group).actions) as gint;
        keys = ({
            let mut __n: gsize = (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
        g_hash_table_iter_init(&raw mut iter, (*group).actions);
        while g_hash_table_iter_next(
            &raw mut iter,
            &raw mut key,
            ::core::ptr::null_mut::<gpointer>(),
        ) != 0
        {
            let fresh0 = i;
            i = i + 1;
            let ref mut fresh1 = *keys.offset(fresh0 as isize);
            *fresh1 = safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as *mut gchar;
        }
        let mut __n1: gint64 = i as gint64;
        let mut __n2: gint64 = n as gint64;
        if !(__n1 == __n2) {
            g_assertion_message_cmpint(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusactiongroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                308 as ::core::ffi::c_int,
                G_STRFUNC,
                b"i == n\0" as *const u8 as *const ::core::ffi::c_char,
                __n1 as guint64,
                b"==\0" as *const u8 as *const ::core::ffi::c_char,
                __n2 as guint64,
                'i' as i32 as ::core::ffi::c_char,
            );
        }
        let ref mut fresh2 = *keys.offset(n as isize);
        *fresh2 = ::core::ptr::null_mut::<gchar>();
    } else {
        safe_c2rust_g_dbus_action_group_async_init(group);
        keys = ({
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
    }
    (*group).strict = TRUE as gboolean;
    return keys;
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_query_action(
    mut g_group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut enabled: *mut gboolean,
    mut parameter_type: *mut *const GVariantType,
    mut state_type: *mut *const GVariantType,
    mut state_hint: *mut *mut GVariant,
    mut state: *mut *mut GVariant,
) -> gboolean {
    let mut group: *mut GDBusActionGroup =
        g_group as *mut ::core::ffi::c_void as *mut GDBusActionGroup;
    let mut info: *mut ActionInfo = ::core::ptr::null_mut::<ActionInfo>();
    if !(*group).actions.is_null() {
        info =
            g_hash_table_lookup((*group).actions, action_name as gconstpointer) as *mut ActionInfo;
        if info.is_null() {
            (*group).strict = TRUE as gboolean;
            return FALSE;
        }
        if !enabled.is_null() {
            *enabled = (*info).enabled;
        }
        if !parameter_type.is_null() {
            *parameter_type = (*info).parameter_type;
        }
        if !state_type.is_null() {
            *state_type = if !(*info).state.is_null() {
                g_variant_get_type((*info).state)
            } else {
                ::core::ptr::null::<GVariantType>()
            };
        }
        if !state_hint.is_null() {
            *state_hint = ::core::ptr::null_mut::<GVariant>();
        }
        if !state.is_null() {
            *state = if !(*info).state.is_null() {
                g_variant_ref((*info).state)
            } else {
                ::core::ptr::null_mut::<GVariant>()
            };
        }
        return TRUE;
    } else {
        safe_c2rust_g_dbus_action_group_async_init(group);
        (*group).strict = TRUE as gboolean;
        return FALSE;
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_activate_action_full(
    mut remote: *mut GRemoteActionGroup,
    mut action_name: *const gchar,
    mut parameter: *mut GVariant,
    mut platform_data: *mut GVariant,
) {
    let mut group: *mut GDBusActionGroup =
        remote as *mut ::core::ffi::c_void as *mut GDBusActionGroup;
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"av\0" as *const u8 as *const gchar),
    );
    if !parameter.is_null() {
        g_variant_builder_add(
            &raw mut builder,
            b"v\0" as *const u8 as *const gchar,
            parameter,
        );
    }
    g_dbus_connection_call(
        (*group).connection,
        (*group).bus_name,
        (*group).object_path,
        b"org.gtk.Actions\0" as *const u8 as *const gchar,
        b"Activate\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(sav@a{sv})\0" as *const u8 as *const gchar,
            action_name,
            &raw mut builder,
            platform_data,
        ),
        ::core::ptr::null::<GVariantType>(),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        None,
        NULL_0,
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_change_action_state_full(
    mut remote: *mut GRemoteActionGroup,
    mut action_name: *const gchar,
    mut value: *mut GVariant,
    mut platform_data: *mut GVariant,
) {
    let mut group: *mut GDBusActionGroup =
        remote as *mut ::core::ffi::c_void as *mut GDBusActionGroup;
    g_dbus_connection_call(
        (*group).connection,
        (*group).bus_name,
        (*group).object_path,
        b"org.gtk.Actions\0" as *const u8 as *const gchar,
        b"SetState\0" as *const u8 as *const gchar,
        g_variant_new(
            b"(sv@a{sv})\0" as *const u8 as *const gchar,
            action_name,
            value,
            platform_data,
        ),
        ::core::ptr::null::<GVariantType>(),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        None,
        NULL_0,
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_change_state(
    mut group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut value: *mut GVariant,
) {
    safe_c2rust_g_dbus_action_group_change_action_state_full(
        group as *mut ::core::ffi::c_void as *mut GRemoteActionGroup,
        action_name,
        value,
        g_variant_new(b"a{sv}\0" as *const u8 as *const gchar, NULL_0),
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_activate(
    mut group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut parameter: *mut GVariant,
) {
    safe_c2rust_g_dbus_action_group_activate_action_full(
        group as *mut ::core::ffi::c_void as *mut GRemoteActionGroup,
        action_name,
        parameter,
        g_variant_new(b"a{sv}\0" as *const u8 as *const gchar, NULL_0),
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_finalize(mut object: *mut GObject) {
    let mut group: *mut GDBusActionGroup =
        object as *mut ::core::ffi::c_void as *mut GDBusActionGroup;
    if (*group).subscription_id != 0 {
        g_dbus_connection_signal_unsubscribe((*group).connection, (*group).subscription_id);
    }
    if !(*group).actions.is_null() {
        g_hash_table_unref((*group).actions);
    }
    g_object_unref((*group).connection as gpointer);
    g_free((*group).object_path as gpointer);
    g_free((*group).bus_name as gpointer);
    (*(safe_c2rust_g_dbus_action_group_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_init(mut group: *mut GDBusActionGroup) {}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_class_init(
    mut class: *mut GDBusActionGroupClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize =
        Some(safe_c2rust_g_dbus_action_group_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_remote_iface_init(
    mut iface: *mut GRemoteActionGroupInterface,
) {
    (*iface).activate_action_full = Some(
        safe_c2rust_g_dbus_action_group_activate_action_full
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
        safe_c2rust_g_dbus_action_group_change_action_state_full
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
unsafe extern "C" fn safe_c2rust_g_dbus_action_group_iface_init(
    mut iface: *mut GActionGroupInterface,
) {
    (*iface).list_actions = Some(
        safe_c2rust_g_dbus_action_group_list_actions
            as unsafe extern "C" fn(*mut GActionGroup) -> *mut *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup) -> *mut *mut gchar>;
    (*iface).query_action = Some(
        safe_c2rust_g_dbus_action_group_query_action
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
        safe_c2rust_g_dbus_action_group_change_state
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>;
    (*iface).activate_action = Some(
        safe_c2rust_g_dbus_action_group_activate
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_action_group_get(
    mut connection: *mut GDBusConnection,
    mut bus_name: *const gchar,
    mut object_path: *const gchar,
) -> *mut GDBusActionGroup {
    let mut group: *mut GDBusActionGroup = ::core::ptr::null_mut::<GDBusActionGroup>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !bus_name.is_null() || g_dbus_connection_get_unique_name(connection).is_null() {
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
            b"bus_name != NULL || g_dbus_connection_get_unique_name (connection) == NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusActionGroup>();
    }
    group = g_object_new(
        safe_c2rust_g_dbus_action_group_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GDBusActionGroup;
    (*group).connection =
        g_object_ref(connection as gpointer) as *mut GDBusConnection as *mut GDBusConnection;
    (*group).bus_name =
        safe_c2rust_g_strdup_inline(bus_name as *const ::core::ffi::c_char) as *mut gchar;
    (*group).object_path =
        safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as *mut gchar;
    return group;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_action_group_sync(
    mut group: *mut GDBusActionGroup,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut reply: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*group).subscription_id == 0 as guint {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusactiongroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            517 as ::core::ffi::c_int,
            G_STRFUNC,
            b"group->subscription_id == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*group).subscription_id = g_dbus_connection_signal_subscribe(
        (*group).connection,
        (*group).bus_name,
        b"org.gtk.Actions\0" as *const u8 as *const gchar,
        b"Changed\0" as *const u8 as *const gchar,
        (*group).object_path,
        ::core::ptr::null::<gchar>(),
        G_DBUS_SIGNAL_FLAGS_NONE,
        Some(
            safe_c2rust_g_dbus_action_group_changed
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
        group as gpointer,
        None,
    );
    reply = g_dbus_connection_call_sync(
        (*group).connection,
        (*group).bus_name,
        (*group).object_path,
        b"org.gtk.Actions\0" as *const u8 as *const gchar,
        b"DescribeAll\0" as *const u8 as *const gchar,
        ::core::ptr::null_mut::<GVariant>(),
        g_variant_type_checked_(b"(a{s(bgav)})\0" as *const u8 as *const gchar),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        cancellable,
        error,
    );
    if !reply.is_null() {
        let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
        let mut action: *mut ActionInfo = ::core::ptr::null_mut::<ActionInfo>();
        if ({
            let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
            if (*group).actions.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusactiongroup.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                532 as ::core::ffi::c_int,
                G_STRFUNC,
                b"group->actions == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*group).actions = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            None,
            Some(safe_c2rust_action_info_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        g_variant_get(
            reply,
            b"(a{s(bgav)})\0" as *const u8 as *const gchar,
            &raw mut iter,
        );
        loop {
            action = safe_c2rust_action_info_new_from_iter(iter);
            if action.is_null() {
                break;
            }
            g_hash_table_insert(
                (*group).actions,
                (*action).name as gpointer,
                action as gpointer,
            );
        }
        g_variant_iter_free(iter);
        g_variant_unref(reply);
    }
    return (reply != NULL_0 as *mut GVariant) as ::core::ffi::c_int;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
