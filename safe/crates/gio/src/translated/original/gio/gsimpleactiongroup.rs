use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GActionMap;
    pub type _GActionGroup;
    pub type _GSimpleAction;
    pub type _GAction;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
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
    fn g_hash_table_foreach(hash_table: *mut GHashTable, func: GHFunc, user_data: gpointer);
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
    fn g_variant_unref(value: *mut GVariant);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handlers_disconnect_matched(
        instance: gpointer,
        mask: GSignalMatchType,
        signal_id: guint,
        detail: GQuark,
        closure: *mut GClosure,
        func: gpointer,
        data: gpointer,
    ) -> guint;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
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
    fn g_action_map_get_type() -> GType;
    fn g_action_map_lookup_action(
        action_map: *mut GActionMap,
        action_name: *const gchar,
    ) -> *mut GAction;
    fn g_action_map_add_action(action_map: *mut GActionMap, action: *mut GAction);
    fn g_action_map_remove_action(action_map: *mut GActionMap, action_name: *const gchar);
    fn g_action_map_add_action_entries(
        action_map: *mut GActionMap,
        entries: *const GActionEntry,
        n_entries: gint,
        user_data: gpointer,
    );
    fn g_action_get_name(action: *mut GAction) -> *const gchar;
    fn g_action_get_parameter_type(action: *mut GAction) -> *const GVariantType;
    fn g_action_get_state_type(action: *mut GAction) -> *const GVariantType;
    fn g_action_get_state_hint(action: *mut GAction) -> *mut GVariant;
    fn g_action_get_enabled(action: *mut GAction) -> gboolean;
    fn g_action_get_state(action: *mut GAction) -> *mut GVariant;
    fn g_action_change_state(action: *mut GAction, value: *mut GVariant);
    fn g_action_activate(action: *mut GAction, parameter: *mut GVariant);
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
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GHFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> ()>;
pub type GQuark = guint32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSimpleActionGroup {
    pub parent_instance: GObject,
    pub priv_0: *mut GSimpleActionGroupPrivate,
}
pub type GSimpleActionGroupPrivate = _GSimpleActionGroupPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSimpleActionGroupPrivate {
    pub table: *mut GHashTable,
}
pub type GSimpleActionGroup = _GSimpleActionGroup;
pub type GActionMap = _GActionMap;
pub type GActionGroup = _GActionGroup;
pub type GSimpleAction = _GSimpleAction;
pub type GAction = _GAction;
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
pub type GActionGroupInterface = _GActionGroupInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GActionMapInterface {
    pub g_iface: GTypeInterface,
    pub lookup_action: Option<unsafe extern "C" fn(*mut GActionMap, *const gchar) -> *mut GAction>,
    pub add_action: Option<unsafe extern "C" fn(*mut GActionMap, *mut GAction) -> ()>,
    pub remove_action: Option<unsafe extern "C" fn(*mut GActionMap, *const gchar) -> ()>,
}
pub type GActionMapInterface = _GActionMapInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GActionEntry {
    pub name: *const gchar,
    pub activate: Option<unsafe extern "C" fn(*mut GSimpleAction, *mut GVariant, gpointer) -> ()>,
    pub parameter_type: *const gchar,
    pub state: *const gchar,
    pub change_state:
        Option<unsafe extern "C" fn(*mut GSimpleAction, *mut GVariant, gpointer) -> ()>,
    pub padding: [gsize; 3],
}
pub type GActionEntry = _GActionEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSimpleActionGroupClass {
    pub parent_class: GObjectClass,
    pub padding: [gpointer; 12],
}
pub type GSimpleActionGroupClass = _GSimpleActionGroupClass;
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
unsafe extern "C" fn safe_c2rust_g_simple_action_group_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_simple_action_group_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSimpleActionGroup_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSimpleActionGroup_private_offset,
        );
    }
    safe_c2rust_g_simple_action_group_class_init(klass as *mut GSimpleActionGroupClass);
}
static mut safe_c2rust_GSimpleActionGroup_private_offset: gint = 0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_group_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_simple_action_group_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_simple_action_group_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSimpleActionGroup\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSimpleActionGroupClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_simple_action_group_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSimpleActionGroup>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSimpleActionGroup) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_simple_action_group_init
                    as unsafe extern "C" fn(*mut GSimpleActionGroup) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GSimpleActionGroup_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GSimpleActionGroupPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GActionGroupInterface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_simple_action_group_iface_init
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
            safe_c2rust_g_simple_action_group_map_iface_init
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
#[inline]
unsafe extern "C" fn safe_c2rust_g_simple_action_group_get_instance_private(
    mut self_0: *mut GSimpleActionGroup,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GSimpleActionGroup_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_g_simple_action_group_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_simple_action_group_list_actions(
    mut group: *mut GActionGroup,
) -> *mut *mut gchar {
    let mut simple: *mut GSimpleActionGroup =
        group as *mut ::core::ffi::c_void as *mut GSimpleActionGroup;
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
    let mut keys: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    n = g_hash_table_size((*(*simple).priv_0).table) as gint;
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
    g_hash_table_iter_init(&raw mut iter, (*(*simple).priv_0).table);
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsimpleactiongroup.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            70 as ::core::ffi::c_int,
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
    return keys;
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_query_action(
    mut group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut enabled: *mut gboolean,
    mut parameter_type: *mut *const GVariantType,
    mut state_type: *mut *const GVariantType,
    mut state_hint: *mut *mut GVariant,
    mut state: *mut *mut GVariant,
) -> gboolean {
    let mut simple: *mut GSimpleActionGroup =
        group as *mut ::core::ffi::c_void as *mut GSimpleActionGroup;
    let mut action: *mut GAction = ::core::ptr::null_mut::<GAction>();
    action = g_hash_table_lookup((*(*simple).priv_0).table, action_name as gconstpointer)
        as *mut GAction;
    if action.is_null() {
        return FALSE;
    }
    if !enabled.is_null() {
        *enabled = g_action_get_enabled(action);
    }
    if !parameter_type.is_null() {
        *parameter_type = g_action_get_parameter_type(action);
    }
    if !state_type.is_null() {
        *state_type = g_action_get_state_type(action);
    }
    if !state_hint.is_null() {
        *state_hint = g_action_get_state_hint(action);
    }
    if !state.is_null() {
        *state = g_action_get_state(action);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_change_state(
    mut group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut value: *mut GVariant,
) {
    let mut simple: *mut GSimpleActionGroup =
        group as *mut ::core::ffi::c_void as *mut GSimpleActionGroup;
    let mut action: *mut GAction = ::core::ptr::null_mut::<GAction>();
    action = g_hash_table_lookup((*(*simple).priv_0).table, action_name as gconstpointer)
        as *mut GAction;
    if action.is_null() {
        return;
    }
    g_action_change_state(action, value);
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_activate(
    mut group: *mut GActionGroup,
    mut action_name: *const gchar,
    mut parameter: *mut GVariant,
) {
    let mut simple: *mut GSimpleActionGroup =
        group as *mut ::core::ffi::c_void as *mut GSimpleActionGroup;
    let mut action: *mut GAction = ::core::ptr::null_mut::<GAction>();
    action = g_hash_table_lookup((*(*simple).priv_0).table, action_name as gconstpointer)
        as *mut GAction;
    if action.is_null() {
        return;
    }
    g_action_activate(action, parameter);
}
unsafe extern "C" fn safe_c2rust_action_enabled_notify(
    mut action: *mut GAction,
    mut pspec: *mut GParamSpec,
    mut user_data: gpointer,
) {
    g_action_group_action_enabled_changed(
        user_data as *mut GActionGroup,
        g_action_get_name(action),
        g_action_get_enabled(action),
    );
}
unsafe extern "C" fn safe_c2rust_action_state_notify(
    mut action: *mut GAction,
    mut pspec: *mut GParamSpec,
    mut user_data: gpointer,
) {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    value = g_action_get_state(action);
    g_action_group_action_state_changed(
        user_data as *mut GActionGroup,
        g_action_get_name(action),
        value,
    );
    g_variant_unref(value);
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_disconnect(
    mut key: gpointer,
    mut value: gpointer,
    mut user_data: gpointer,
) {
    g_signal_handlers_disconnect_matched(
        value,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GAction, *mut GParamSpec, gpointer) -> ()>,
            gpointer,
        >(Some(
            safe_c2rust_action_enabled_notify
                as unsafe extern "C" fn(*mut GAction, *mut GParamSpec, gpointer) -> (),
        )),
        user_data,
    );
    g_signal_handlers_disconnect_matched(
        value,
        (G_SIGNAL_MATCH_FUNC as ::core::ffi::c_int | G_SIGNAL_MATCH_DATA as ::core::ffi::c_int)
            as GSignalMatchType,
        0 as guint,
        0 as GQuark,
        ::core::ptr::null_mut::<GClosure>(),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GAction, *mut GParamSpec, gpointer) -> ()>,
            gpointer,
        >(Some(
            safe_c2rust_action_state_notify
                as unsafe extern "C" fn(*mut GAction, *mut GParamSpec, gpointer) -> (),
        )),
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_lookup_action(
    mut action_map: *mut GActionMap,
    mut action_name: *const gchar,
) -> *mut GAction {
    let mut simple: *mut GSimpleActionGroup =
        action_map as *mut ::core::ffi::c_void as *mut GSimpleActionGroup;
    return g_hash_table_lookup((*(*simple).priv_0).table, action_name as gconstpointer)
        as *mut GAction;
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_add_action(
    mut action_map: *mut GActionMap,
    mut action: *mut GAction,
) {
    let mut simple: *mut GSimpleActionGroup =
        action_map as *mut ::core::ffi::c_void as *mut GSimpleActionGroup;
    let mut action_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut old_action: *mut GAction = ::core::ptr::null_mut::<GAction>();
    action_name = g_action_get_name(action);
    if action_name.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"The supplied action has no name. You must set the GAction:name property when creating an action.\0"
                as *const u8 as *const gchar,
        );
        return;
    }
    old_action = g_hash_table_lookup((*(*simple).priv_0).table, action_name as gconstpointer)
        as *mut GAction;
    if old_action != action {
        if !old_action.is_null() {
            g_action_group_action_removed(
                simple as *mut ::core::ffi::c_void as *mut GActionGroup,
                action_name,
            );
            safe_c2rust_g_simple_action_group_disconnect(
                NULL_0,
                old_action as gpointer,
                simple as gpointer,
            );
        }
        g_signal_connect_data(
            action as gpointer,
            b"notify::enabled\0" as *const u8 as *const gchar,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GAction, *mut GParamSpec, gpointer) -> ()>,
                GCallback,
            >(Some(
                safe_c2rust_action_enabled_notify
                    as unsafe extern "C" fn(*mut GAction, *mut GParamSpec, gpointer) -> (),
            )),
            simple as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
        if !g_action_get_state_type(action).is_null() {
            g_signal_connect_data(
                action as gpointer,
                b"notify::state\0" as *const u8 as *const gchar,
                ::core::mem::transmute::<
                    Option<unsafe extern "C" fn(*mut GAction, *mut GParamSpec, gpointer) -> ()>,
                    GCallback,
                >(Some(
                    safe_c2rust_action_state_notify
                        as unsafe extern "C" fn(*mut GAction, *mut GParamSpec, gpointer) -> (),
                )),
                simple as gpointer,
                None,
                G_CONNECT_DEFAULT,
            );
        }
        g_hash_table_insert(
            (*(*simple).priv_0).table,
            safe_c2rust_g_strdup_inline(action_name as *const ::core::ffi::c_char) as gpointer,
            g_object_ref(action as gpointer) as *mut GAction as gpointer,
        );
        g_action_group_action_added(
            simple as *mut ::core::ffi::c_void as *mut GActionGroup,
            action_name,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_remove_action(
    mut action_map: *mut GActionMap,
    mut action_name: *const gchar,
) {
    let mut simple: *mut GSimpleActionGroup =
        action_map as *mut ::core::ffi::c_void as *mut GSimpleActionGroup;
    let mut action: *mut GAction = ::core::ptr::null_mut::<GAction>();
    action = g_hash_table_lookup((*(*simple).priv_0).table, action_name as gconstpointer)
        as *mut GAction;
    if !action.is_null() {
        g_action_group_action_removed(
            simple as *mut ::core::ffi::c_void as *mut GActionGroup,
            action_name,
        );
        safe_c2rust_g_simple_action_group_disconnect(
            NULL_0,
            action as gpointer,
            simple as gpointer,
        );
        g_hash_table_remove((*(*simple).priv_0).table, action_name as gconstpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_finalize(mut object: *mut GObject) {
    let mut simple: *mut GSimpleActionGroup =
        object as *mut ::core::ffi::c_void as *mut GSimpleActionGroup;
    g_hash_table_foreach(
        (*(*simple).priv_0).table,
        Some(
            safe_c2rust_g_simple_action_group_disconnect
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> (),
        ),
        simple as gpointer,
    );
    g_hash_table_unref((*(*simple).priv_0).table);
    (*(safe_c2rust_g_simple_action_group_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_init(mut simple: *mut GSimpleActionGroup) {
    (*simple).priv_0 = safe_c2rust_g_simple_action_group_get_instance_private(simple)
        as *mut GSimpleActionGroupPrivate;
    (*(*simple).priv_0).table = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_class_init(
    mut class: *mut GSimpleActionGroupClass,
) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).finalize = Some(
        safe_c2rust_g_simple_action_group_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_iface_init(
    mut iface: *mut GActionGroupInterface,
) {
    (*iface).list_actions = Some(
        safe_c2rust_g_simple_action_group_list_actions
            as unsafe extern "C" fn(*mut GActionGroup) -> *mut *mut gchar,
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup) -> *mut *mut gchar>;
    (*iface).query_action = Some(
        safe_c2rust_g_simple_action_group_query_action
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
        safe_c2rust_g_simple_action_group_change_state
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>;
    (*iface).activate_action = Some(
        safe_c2rust_g_simple_action_group_activate
            as unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GActionGroup, *const gchar, *mut GVariant) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_simple_action_group_map_iface_init(
    mut iface: *mut GActionMapInterface,
) {
    (*iface).add_action = Some(
        safe_c2rust_g_simple_action_group_add_action
            as unsafe extern "C" fn(*mut GActionMap, *mut GAction) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GActionMap, *mut GAction) -> ()>;
    (*iface).remove_action = Some(
        safe_c2rust_g_simple_action_group_remove_action
            as unsafe extern "C" fn(*mut GActionMap, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GActionMap, *const gchar) -> ()>;
    (*iface).lookup_action = Some(
        safe_c2rust_g_simple_action_group_lookup_action
            as unsafe extern "C" fn(*mut GActionMap, *const gchar) -> *mut GAction,
    )
        as Option<unsafe extern "C" fn(*mut GActionMap, *const gchar) -> *mut GAction>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_group_new() -> *mut GSimpleActionGroup {
    return g_object_new(
        safe_c2rust_g_simple_action_group_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GSimpleActionGroup;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_group_lookup(
    mut simple: *mut GSimpleActionGroup,
    mut action_name: *const gchar,
) -> *mut GAction {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_action_group_get_type();
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
            b"G_IS_SIMPLE_ACTION_GROUP (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GAction>();
    }
    return g_action_map_lookup_action(
        simple as *mut ::core::ffi::c_void as *mut GActionMap,
        action_name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_group_insert(
    mut simple: *mut GSimpleActionGroup,
    mut action: *mut GAction,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_action_group_get_type();
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
            b"G_IS_SIMPLE_ACTION_GROUP (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_action_map_add_action(
        simple as *mut ::core::ffi::c_void as *mut GActionMap,
        action,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_group_remove(
    mut simple: *mut GSimpleActionGroup,
    mut action_name: *const gchar,
) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = simple as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_simple_action_group_get_type();
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
            b"G_IS_SIMPLE_ACTION_GROUP (simple)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_action_map_remove_action(
        simple as *mut ::core::ffi::c_void as *mut GActionMap,
        action_name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_simple_action_group_add_entries(
    mut simple: *mut GSimpleActionGroup,
    mut entries: *const GActionEntry,
    mut n_entries: gint,
    mut user_data: gpointer,
) {
    g_action_map_add_action_entries(
        simple as *mut ::core::ffi::c_void as *mut GActionMap,
        entries,
        n_entries,
        user_data,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
