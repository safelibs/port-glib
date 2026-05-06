extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GSequence;
    pub type _GSequenceNode;
    pub type _GAsyncResult;
    pub type _GCancellable;
    pub type _GMenuModelPrivate;
    pub type _GDBusConnection;
    pub type _GMenuLinkIterPrivate;
    pub type _GMenuAttributeIterPrivate;
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
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
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
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_ref(hash_table: *mut GHashTable) -> *mut GHashTable;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_main_context_ref(context: *mut GMainContext) -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_default() -> *mut GMainContext;
    fn g_main_context_get_thread_default() -> *mut GMainContext;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_n_children(value: *mut GVariant) -> gsize;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_free(iter: *mut GVariantIter);
    fn g_variant_iter_loop(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_variant_new_parsed(format: *const gchar, ...) -> *mut GVariant;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_sequence_new(data_destroy: GDestroyNotify) -> *mut GSequence;
    fn g_sequence_free(seq: *mut GSequence);
    fn g_sequence_get_length(seq: *mut GSequence) -> gint;
    fn g_sequence_is_empty(seq: *mut GSequence) -> gboolean;
    fn g_sequence_get_iter_at_pos(seq: *mut GSequence, pos: gint) -> *mut GSequenceIter;
    fn g_sequence_insert_before(iter: *mut GSequenceIter, data: gpointer) -> *mut GSequenceIter;
    fn g_sequence_remove_range(begin: *mut GSequenceIter, end: *mut GSequenceIter);
    fn g_sequence_get(iter: *mut GSequenceIter) -> gpointer;
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
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
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
    fn g_menu_model_get_type() -> GType;
    fn g_menu_model_items_changed(
        model: *mut GMenuModel,
        position: gint,
        removed: gint,
        added: gint,
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
pub type GMainContext = _GMainContext;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantIter {
    pub x: [guintptr; 16],
}
pub type GVariantIter = _GVariantIter;
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
pub type GSequence = _GSequence;
pub type GSequenceIter = _GSequenceNode;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuModel {
    pub parent_instance: GObject,
    pub priv_0: *mut GMenuModelPrivate,
}
pub type GMenuModelPrivate = _GMenuModelPrivate;
pub type GMenuModel = _GMenuModel;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GDBusConnection = _GDBusConnection;
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
pub struct _GDBusMenuModel {
    pub parent: GMenuModel,
    pub group: *mut GDBusMenuGroup,
    pub id: guint,
    pub items: *mut GSequence,
    pub active: gboolean,
}
pub type GDBusMenuGroup = _GDBusMenuGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMenuGroup {
    pub path: *mut GDBusMenuPath,
    pub id: guint,
    pub proxies: *mut GHashTable,
    pub menus: *mut GHashTable,
    pub ref_count: gint,
    pub state: GroupStatus,
    pub active: gint,
}
pub type GroupStatus = ::core::ffi::c_uint;
pub const GROUP_ONLINE: GroupStatus = 2;
pub const GROUP_PENDING: GroupStatus = 1;
pub const GROUP_OFFLINE: GroupStatus = 0;
pub type GDBusMenuPath = _GDBusMenuPath;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMenuPath {
    pub id: *mut PathIdentifier,
    pub ref_count: gint,
    pub groups: *mut GHashTable,
    pub active: gint,
    pub watch_id: guint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PathIdentifier {
    pub context: *mut GMainContext,
    pub connection: *mut GDBusConnection,
    pub bus_name: *mut gchar,
    pub object_path: *mut gchar,
}
pub type GDBusMenuModel = _GDBusMenuModel;
pub type GDBusMenuModelClass = GMenuModelClass;
pub type GMenuModelClass = _GMenuModelClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuModelClass {
    pub parent_class: GObjectClass,
    pub is_mutable: Option<unsafe extern "C" fn(*mut GMenuModel) -> gboolean>,
    pub get_n_items: Option<unsafe extern "C" fn(*mut GMenuModel) -> gint>,
    pub get_item_attributes:
        Option<unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> ()>,
    pub iterate_item_attributes:
        Option<unsafe extern "C" fn(*mut GMenuModel, gint) -> *mut GMenuAttributeIter>,
    pub get_item_attribute_value: Option<
        unsafe extern "C" fn(
            *mut GMenuModel,
            gint,
            *const gchar,
            *const GVariantType,
        ) -> *mut GVariant,
    >,
    pub get_item_links:
        Option<unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> ()>,
    pub iterate_item_links:
        Option<unsafe extern "C" fn(*mut GMenuModel, gint) -> *mut GMenuLinkIter>,
    pub get_item_link:
        Option<unsafe extern "C" fn(*mut GMenuModel, gint, *const gchar) -> *mut GMenuModel>,
}
pub type GMenuLinkIter = _GMenuLinkIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuLinkIter {
    pub parent_instance: GObject,
    pub priv_0: *mut GMenuLinkIterPrivate,
}
pub type GMenuLinkIterPrivate = _GMenuLinkIterPrivate;
pub type GMenuAttributeIter = _GMenuAttributeIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuAttributeIter {
    pub parent_instance: GObject,
    pub priv_0: *mut GMenuAttributeIterPrivate,
}
pub type GMenuAttributeIterPrivate = _GMenuAttributeIterPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GDBusMenuModelItem {
    pub attributes: *mut GHashTable,
    pub links: *mut GHashTable,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConstPathIdentifier {
    pub context: *mut GMainContext,
    pub connection: *mut GDBusConnection,
    pub bus_name: *const gchar,
    pub object_path: *const gchar,
}
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
unsafe extern "C" fn safe_c2rust_path_identifier_hash(mut data: gconstpointer) -> guint {
    let mut id: *const ConstPathIdentifier = data as *const ConstPathIdentifier;
    return g_str_hash((*id).object_path as gconstpointer);
}
unsafe extern "C" fn safe_c2rust_path_identifier_equal(
    mut a: gconstpointer,
    mut b: gconstpointer,
) -> gboolean {
    let mut id_a: *const ConstPathIdentifier = a as *const ConstPathIdentifier;
    let mut id_b: *const ConstPathIdentifier = b as *const ConstPathIdentifier;
    return ((*id_a).connection == (*id_b).connection
        && g_strcmp0(
            (*id_a).bus_name as *const ::core::ffi::c_char,
            (*id_b).bus_name as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        && strcmp(
            (*id_a).object_path as *const ::core::ffi::c_char,
            (*id_b).object_path as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_path_identifier_free(mut id: *mut PathIdentifier) {
    g_main_context_unref((*id).context);
    g_object_unref((*id).connection as gpointer);
    g_free((*id).bus_name as gpointer);
    g_free((*id).object_path as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<PathIdentifier>() as gsize,
        id as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_path_identifier_new(
    mut cid: *const ConstPathIdentifier,
) -> *mut PathIdentifier {
    let mut id: *mut PathIdentifier = ::core::ptr::null_mut::<PathIdentifier>();
    id = g_slice_alloc(::core::mem::size_of::<PathIdentifier>() as gsize) as *mut PathIdentifier;
    (*id).context = g_main_context_ref((*cid).context);
    (*id).connection = g_object_ref((*cid).connection as gpointer) as *mut GDBusConnection;
    (*id).bus_name =
        safe_c2rust_g_strdup_inline((*cid).bus_name as *const ::core::ffi::c_char) as *mut gchar;
    (*id).object_path =
        safe_c2rust_g_strdup_inline((*cid).object_path as *const ::core::ffi::c_char) as *mut gchar;
    return id;
}
static mut safe_c2rust_g_dbus_menu_paths: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
unsafe extern "C" fn safe_c2rust_g_dbus_menu_path_ref(
    mut path: *mut GDBusMenuPath,
) -> *mut GDBusMenuPath {
    (*path).ref_count += 1;
    return path;
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_path_unref(mut path: *mut GDBusMenuPath) {
    (*path).ref_count -= 1;
    if (*path).ref_count == 0 as ::core::ffi::c_int {
        g_hash_table_remove(safe_c2rust_g_dbus_menu_paths, (*path).id as gconstpointer);
        g_hash_table_unref((*path).groups);
        safe_c2rust_path_identifier_free((*path).id);
        g_slice_free1(
            ::core::mem::size_of::<GDBusMenuPath>() as gsize,
            path as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_path_signal(
    mut connection: *mut GDBusConnection,
    mut sender_name: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut signal_name: *const gchar,
    mut parameters: *mut GVariant,
    mut user_data: gpointer,
) {
    let mut path: *mut GDBusMenuPath = user_data as *mut GDBusMenuPath;
    let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    let mut group_id: guint = 0;
    let mut menu_id: guint = 0;
    let mut position: guint = 0;
    let mut removes: guint = 0;
    let mut adds: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if g_variant_is_of_type(
        parameters,
        g_variant_type_checked_(b"(a(uuuuaa{sv}))\0" as *const u8 as *const gchar),
    ) == 0
    {
        return;
    }
    g_variant_get(
        parameters,
        b"(a(uuuuaa{sv}))\0" as *const u8 as *const gchar,
        &raw mut iter,
    );
    while g_variant_iter_loop(
        iter,
        b"(uuuu@aa{sv})\0" as *const u8 as *const gchar,
        &raw mut group_id,
        &raw mut menu_id,
        &raw mut position,
        &raw mut removes,
        &raw mut adds,
    ) != 0
    {
        let mut group: *mut GDBusMenuGroup = ::core::ptr::null_mut::<GDBusMenuGroup>();
        group = g_hash_table_lookup(
            (*path).groups,
            group_id as glong as gpointer as gconstpointer,
        ) as *mut GDBusMenuGroup;
        if !group.is_null() {
            safe_c2rust_g_dbus_menu_group_changed(
                group,
                menu_id,
                position as gint,
                removes as gint,
                adds,
            );
        }
    }
    g_variant_iter_free(iter);
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_path_activate(mut path: *mut GDBusMenuPath) {
    let fresh1 = (*path).active;
    (*path).active = (*path).active + 1;
    if fresh1 == 0 as ::core::ffi::c_int {
        (*path).watch_id = g_dbus_connection_signal_subscribe(
            (*(*path).id).connection,
            (*(*path).id).bus_name,
            b"org.gtk.Menus\0" as *const u8 as *const gchar,
            b"Changed\0" as *const u8 as *const gchar,
            (*(*path).id).object_path,
            ::core::ptr::null::<gchar>(),
            G_DBUS_SIGNAL_FLAGS_NONE,
            Some(
                safe_c2rust_g_dbus_menu_path_signal
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
            path as gpointer,
            None,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_path_deactivate(mut path: *mut GDBusMenuPath) {
    (*path).active -= 1;
    if (*path).active == 0 as ::core::ffi::c_int {
        g_dbus_connection_signal_unsubscribe((*(*path).id).connection, (*path).watch_id);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_path_get(
    mut context: *mut GMainContext,
    mut connection: *mut GDBusConnection,
    mut bus_name: *const gchar,
    mut object_path: *const gchar,
) -> *mut GDBusMenuPath {
    let cid: ConstPathIdentifier = ConstPathIdentifier {
        context: context,
        connection: connection,
        bus_name: bus_name,
        object_path: object_path,
    };
    let mut path: *mut GDBusMenuPath = ::core::ptr::null_mut::<GDBusMenuPath>();
    if safe_c2rust_g_dbus_menu_paths.is_null() {
        safe_c2rust_g_dbus_menu_paths = g_hash_table_new(
            Some(safe_c2rust_path_identifier_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(
                safe_c2rust_path_identifier_equal
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
            ),
        );
    }
    path = g_hash_table_lookup(
        safe_c2rust_g_dbus_menu_paths,
        &raw const cid as gconstpointer,
    ) as *mut GDBusMenuPath;
    if path.is_null() {
        path =
            g_slice_alloc(::core::mem::size_of::<GDBusMenuPath>() as gsize) as *mut GDBusMenuPath;
        (*path).id = safe_c2rust_path_identifier_new(&raw const cid);
        (*path).groups = g_hash_table_new(None, None);
        (*path).ref_count = 0 as ::core::ffi::c_int as gint;
        (*path).active = 0 as ::core::ffi::c_int as gint;
        g_hash_table_insert(
            safe_c2rust_g_dbus_menu_paths,
            (*path).id as gpointer,
            path as gpointer,
        );
    }
    return safe_c2rust_g_dbus_menu_path_ref(path);
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_group_ref(
    mut group: *mut GDBusMenuGroup,
) -> *mut GDBusMenuGroup {
    (*group).ref_count += 1;
    return group;
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_group_unref(mut group: *mut GDBusMenuGroup) {
    (*group).ref_count -= 1;
    if (*group).ref_count == 0 as ::core::ffi::c_int {
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if (*group).state as ::core::ffi::c_uint
                == GROUP_OFFLINE as ::core::ffi::c_int as ::core::ffi::c_uint
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmenumodel.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                379 as ::core::ffi::c_int,
                G_STRFUNC,
                b"group->state == GROUP_OFFLINE\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
            if (*group).active == 0 as ::core::ffi::c_int {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmenumodel.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                380 as ::core::ffi::c_int,
                G_STRFUNC,
                b"group->active == 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_hash_table_remove(
            (*(*group).path).groups,
            (*group).id as glong as gpointer as gconstpointer,
        );
        g_hash_table_unref((*group).proxies);
        g_hash_table_unref((*group).menus);
        safe_c2rust_g_dbus_menu_path_unref((*group).path);
        g_slice_free1(
            ::core::mem::size_of::<GDBusMenuGroup>() as gsize,
            group as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_item_free(mut data: gpointer) {
    let mut item: *mut GDBusMenuModelItem = data as *mut GDBusMenuModelItem;
    g_hash_table_unref((*item).attributes);
    g_hash_table_unref((*item).links);
    g_slice_free1(
        ::core::mem::size_of::<GDBusMenuModelItem>() as gsize,
        item as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_group_create_item(
    mut description: *mut GVariant,
) -> *mut GDBusMenuModelItem {
    let mut item: *mut GDBusMenuModelItem = ::core::ptr::null_mut::<GDBusMenuModelItem>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut key: *const gchar = ::core::ptr::null::<gchar>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    item = g_slice_alloc(::core::mem::size_of::<GDBusMenuModelItem>() as gsize)
        as *mut GDBusMenuModelItem;
    (*item).attributes = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GVariant) -> ()>, GDestroyNotify>(
            Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()),
        ),
    );
    (*item).links = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GVariant) -> ()>, GDestroyNotify>(
            Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()),
        ),
    );
    g_variant_iter_init(&raw mut iter, description);
    while g_variant_iter_loop(
        &raw mut iter,
        b"{&sv}\0" as *const u8 as *const gchar,
        &raw mut key,
        &raw mut value,
    ) != 0
    {
        if *key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32 {
            g_hash_table_insert(
                (*item).links,
                safe_c2rust_g_strdup_inline(key.offset(1 as ::core::ffi::c_int as isize))
                    as gpointer,
                g_variant_ref(value) as gpointer,
            );
        } else {
            g_hash_table_insert(
                (*item).attributes,
                safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
                g_variant_ref(value) as gpointer,
            );
        }
    }
    return item;
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_group_go_offline(mut group: *mut GDBusMenuGroup) {
    safe_c2rust_g_dbus_menu_path_deactivate((*group).path);
    g_dbus_connection_call(
        (*(*(*group).path).id).connection,
        (*(*(*group).path).id).bus_name,
        (*(*(*group).path).id).object_path,
        b"org.gtk.Menus\0" as *const u8 as *const gchar,
        b"End\0" as *const u8 as *const gchar,
        g_variant_new_parsed(b"([ %u ],)\0" as *const u8 as *const gchar, (*group).id),
        ::core::ptr::null::<GVariantType>(),
        G_DBUS_CALL_FLAGS_NONE,
        -(1 as gint),
        ::core::ptr::null_mut::<GCancellable>(),
        None,
        NULL_0,
    );
    (*group).state = GROUP_OFFLINE;
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_group_start_ready(
    mut source_object: *mut GObject,
    mut result: *mut GAsyncResult,
    mut user_data: gpointer,
) {
    let mut connection: *mut GDBusConnection =
        source_object as *mut ::core::ffi::c_void as *mut GDBusConnection;
    let mut group: *mut GDBusMenuGroup = user_data as *mut GDBusMenuGroup;
    let mut reply: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if (*group).state as ::core::ffi::c_uint
            == GROUP_PENDING as ::core::ffi::c_int as ::core::ffi::c_uint
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmenumodel.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            497 as ::core::ffi::c_int,
            G_STRFUNC,
            b"group->state == GROUP_PENDING\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    reply =
        g_dbus_connection_call_finish(connection, result, ::core::ptr::null_mut::<*mut GError>());
    if (*group).active != 0 {
        (*group).state = GROUP_ONLINE;
        if !reply.is_null() {
            let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
            let mut items: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            let mut group_id: guint = 0;
            let mut menu_id: guint = 0;
            g_variant_get(
                reply,
                b"(a(uuaa{sv}))\0" as *const u8 as *const gchar,
                &raw mut iter,
            );
            while g_variant_iter_loop(
                iter,
                b"(uu@aa{sv})\0" as *const u8 as *const gchar,
                &raw mut group_id,
                &raw mut menu_id,
                &raw mut items,
            ) != 0
            {
                if group_id == (*group).id {
                    safe_c2rust_g_dbus_menu_group_changed(
                        group, menu_id, 0 as gint, 0 as gint, items,
                    );
                }
            }
            g_variant_iter_free(iter);
        }
    } else {
        safe_c2rust_g_dbus_menu_group_go_offline(group);
    }
    if !reply.is_null() {
        g_variant_unref(reply);
    }
    safe_c2rust_g_dbus_menu_group_unref(group);
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_group_activate(mut group: *mut GDBusMenuGroup) {
    let fresh0 = (*group).active;
    (*group).active = (*group).active + 1;
    if fresh0 == 0 as ::core::ffi::c_int {
        if ({
            let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
            if (*group).state as ::core::ffi::c_uint
                != GROUP_ONLINE as ::core::ffi::c_int as ::core::ffi::c_uint
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmenumodel.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                534 as ::core::ffi::c_int,
                G_STRFUNC,
                b"group->state != GROUP_ONLINE\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*group).state as ::core::ffi::c_uint
            == GROUP_OFFLINE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            safe_c2rust_g_dbus_menu_path_activate((*group).path);
            g_dbus_connection_call(
                (*(*(*group).path).id).connection,
                (*(*(*group).path).id).bus_name,
                (*(*(*group).path).id).object_path,
                b"org.gtk.Menus\0" as *const u8 as *const gchar,
                b"Start\0" as *const u8 as *const gchar,
                g_variant_new_parsed(b"([ %u ],)\0" as *const u8 as *const gchar, (*group).id),
                g_variant_type_checked_(b"(a(uuaa{sv}))\0" as *const u8 as *const gchar),
                G_DBUS_CALL_FLAGS_NONE,
                -(1 as gint),
                ::core::ptr::null_mut::<GCancellable>(),
                Some(
                    safe_c2rust_g_dbus_menu_group_start_ready
                        as unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> (),
                ),
                safe_c2rust_g_dbus_menu_group_ref(group) as gpointer,
            );
            (*group).state = GROUP_PENDING;
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_group_deactivate(mut group: *mut GDBusMenuGroup) {
    (*group).active -= 1;
    if (*group).active == 0 as ::core::ffi::c_int {
        if ({
            let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
            if (*group).state as ::core::ffi::c_uint
                != GROUP_OFFLINE as ::core::ffi::c_int as ::core::ffi::c_uint
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmenumodel.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                559 as ::core::ffi::c_int,
                G_STRFUNC,
                b"group->state != GROUP_OFFLINE\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (*group).state as ::core::ffi::c_uint
            == GROUP_ONLINE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            g_hash_table_remove_all((*group).menus);
            safe_c2rust_g_dbus_menu_group_go_offline(group);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_group_changed(
    mut group: *mut GDBusMenuGroup,
    mut menu_id: guint,
    mut position: gint,
    mut removed: gint,
    mut added: *mut GVariant,
) {
    let mut point: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut proxy: *mut GDBusMenuModel = ::core::ptr::null_mut::<GDBusMenuModel>();
    let mut items: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut item: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut n_added: gint = 0;
    let mut n_items: gint = 0;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if g_variant_is_of_type(
            added,
            g_variant_type_checked_(b"aa{sv}\0" as *const u8 as *const gchar),
        ) != 0
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmenumodel.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            591 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_variant_is_of_type (added, G_VARIANT_TYPE (\"aa{sv}\"))\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    n_added = g_variant_n_children(added) as gint;
    if position < 0 as ::core::ffi::c_int
        || position >= 1000 as ::core::ffi::c_int
        || removed < 0 as ::core::ffi::c_int
        || removed >= 1000 as ::core::ffi::c_int
        || n_added >= 1000 as ::core::ffi::c_int
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"invalid arguments\0" as *const u8 as *const gchar,
        );
        return;
    }
    if (*group).state as ::core::ffi::c_uint
        != GROUP_ONLINE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    items = g_hash_table_lookup(
        (*group).menus,
        menu_id as glong as gpointer as gconstpointer,
    ) as *mut GSequence;
    if items.is_null() {
        items = g_sequence_new(Some(
            safe_c2rust_g_dbus_menu_model_item_free as unsafe extern "C" fn(gpointer) -> (),
        ));
        g_hash_table_insert(
            (*group).menus,
            menu_id as glong as gpointer,
            items as gpointer,
        );
    }
    n_items = g_sequence_get_length(items);
    if position + removed > n_items || n_items - removed + n_added > 1000 as ::core::ffi::c_int {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"invalid arguments\0" as *const u8 as *const gchar,
        );
        return;
    }
    point = g_sequence_get_iter_at_pos(items, position + removed);
    if removed != 0 {
        let mut start: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
        start = g_sequence_get_iter_at_pos(items, position);
        g_sequence_remove_range(start, point);
    }
    g_variant_iter_init(&raw mut iter, added);
    while g_variant_iter_loop(
        &raw mut iter,
        b"@a{sv}\0" as *const u8 as *const gchar,
        &raw mut item,
    ) != 0
    {
        g_sequence_insert_before(
            point,
            safe_c2rust_g_dbus_menu_group_create_item(item) as gpointer,
        );
    }
    if g_sequence_is_empty(items) != 0 {
        g_hash_table_remove(
            (*group).menus,
            menu_id as glong as gpointer as gconstpointer,
        );
        items = ::core::ptr::null_mut::<GSequence>();
    }
    proxy = g_hash_table_lookup(
        (*group).proxies,
        menu_id as glong as gpointer as gconstpointer,
    ) as *mut GDBusMenuModel;
    if !proxy.is_null() {
        safe_c2rust_g_dbus_menu_model_changed(proxy, items, position, removed, n_added);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_group_get_from_path(
    mut path: *mut GDBusMenuPath,
    mut group_id: guint,
) -> *mut GDBusMenuGroup {
    let mut group: *mut GDBusMenuGroup = ::core::ptr::null_mut::<GDBusMenuGroup>();
    group = g_hash_table_lookup(
        (*path).groups,
        group_id as glong as gpointer as gconstpointer,
    ) as *mut GDBusMenuGroup;
    if group.is_null() {
        group =
            g_slice_alloc(::core::mem::size_of::<GDBusMenuGroup>() as gsize) as *mut GDBusMenuGroup;
        (*group).path = safe_c2rust_g_dbus_menu_path_ref(path);
        (*group).id = group_id;
        (*group).proxies = g_hash_table_new(None, None);
        (*group).menus = g_hash_table_new_full(
            None,
            None,
            None,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSequence) -> ()>,
                GDestroyNotify,
            >(Some(
                g_sequence_free as unsafe extern "C" fn(*mut GSequence) -> (),
            )),
        );
        (*group).state = GROUP_OFFLINE;
        (*group).active = 0 as ::core::ffi::c_int as gint;
        (*group).ref_count = 0 as ::core::ffi::c_int as gint;
        g_hash_table_insert(
            (*path).groups,
            (*group).id as glong as gpointer,
            group as gpointer,
        );
    }
    return safe_c2rust_g_dbus_menu_group_ref(group);
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_group_get(
    mut context: *mut GMainContext,
    mut connection: *mut GDBusConnection,
    mut bus_name: *const gchar,
    mut object_path: *const gchar,
    mut group_id: guint,
) -> *mut GDBusMenuGroup {
    let mut group: *mut GDBusMenuGroup = ::core::ptr::null_mut::<GDBusMenuGroup>();
    let mut path: *mut GDBusMenuPath = ::core::ptr::null_mut::<GDBusMenuPath>();
    path = safe_c2rust_g_dbus_menu_path_get(context, connection, bus_name, object_path);
    group = safe_c2rust_g_dbus_menu_group_get_from_path(path, group_id);
    safe_c2rust_g_dbus_menu_path_unref(path);
    return group;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_menu_model_get_type(),
        g_intern_static_string(b"GDBusMenuModel\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusMenuModelClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_menu_model_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusMenuModel>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusMenuModel) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_menu_model_init
                    as unsafe extern "C" fn(*mut GDBusMenuModel) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GDBusMenuModel_private_offset: gint = 0;
static mut safe_c2rust_g_dbus_menu_model_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_menu_model_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusMenuModel_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusMenuModel_private_offset,
        );
    }
    safe_c2rust_g_dbus_menu_model_class_init(klass as *mut GDBusMenuModelClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_menu_model_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_is_mutable(
    mut model: *mut GMenuModel,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_get_n_items(mut model: *mut GMenuModel) -> gint {
    let mut proxy: *mut GDBusMenuModel = model as *mut ::core::ffi::c_void as *mut GDBusMenuModel;
    if (*proxy).active == 0 {
        safe_c2rust_g_dbus_menu_group_activate((*proxy).group);
        (*proxy).active = TRUE as gboolean;
    }
    return if !(*proxy).items.is_null() {
        g_sequence_get_length((*proxy).items)
    } else {
        0 as gint
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_get_item_attributes(
    mut model: *mut GMenuModel,
    mut item_index: gint,
    mut table: *mut *mut GHashTable,
) {
    let mut proxy: *mut GDBusMenuModel = model as *mut ::core::ffi::c_void as *mut GDBusMenuModel;
    let mut item: *mut GDBusMenuModelItem = ::core::ptr::null_mut::<GDBusMenuModelItem>();
    let mut iter: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if (*proxy).active != 0 {
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
            b"proxy->active\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !(*proxy).items.is_null() {
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
            b"proxy->items\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iter = g_sequence_get_iter_at_pos((*proxy).items, item_index);
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    item = g_sequence_get(iter) as *mut GDBusMenuModelItem;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !item.is_null() {
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
            b"item\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    *table = g_hash_table_ref((*item).attributes);
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_get_item_links(
    mut model: *mut GMenuModel,
    mut item_index: gint,
    mut table: *mut *mut GHashTable,
) {
    let mut proxy: *mut GDBusMenuModel = model as *mut ::core::ffi::c_void as *mut GDBusMenuModel;
    let mut item: *mut GDBusMenuModelItem = ::core::ptr::null_mut::<GDBusMenuModelItem>();
    let mut iter: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*proxy).active != 0 {
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
            b"proxy->active\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !(*proxy).items.is_null() {
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
            b"proxy->items\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    iter = g_sequence_get_iter_at_pos((*proxy).items, item_index);
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    item = g_sequence_get(iter) as *mut GDBusMenuModelItem;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !item.is_null() {
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
            b"item\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    *table = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    let mut tmp: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    g_hash_table_iter_init(&raw mut tmp, (*item).links);
    while g_hash_table_iter_next(&raw mut tmp, &raw mut key, &raw mut value) != 0 {
        if g_variant_is_of_type(
            value as *mut GVariant,
            g_variant_type_checked_(b"(uu)\0" as *const u8 as *const gchar),
        ) != 0
        {
            let mut group_id: guint = 0;
            let mut menu_id: guint = 0;
            let mut group: *mut GDBusMenuGroup = ::core::ptr::null_mut::<GDBusMenuGroup>();
            let mut link: *mut GDBusMenuModel = ::core::ptr::null_mut::<GDBusMenuModel>();
            g_variant_get(
                value as *mut GVariant,
                b"(uu)\0" as *const u8 as *const gchar,
                &raw mut group_id,
                &raw mut menu_id,
            );
            if (*(*proxy).group).id != group_id {
                group =
                    safe_c2rust_g_dbus_menu_group_get_from_path((*(*proxy).group).path, group_id);
            } else {
                group = safe_c2rust_g_dbus_menu_group_ref((*proxy).group);
            }
            link = safe_c2rust_g_dbus_menu_model_get_from_group(group, menu_id);
            g_hash_table_insert(
                *table,
                safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
                link as gpointer,
            );
            safe_c2rust_g_dbus_menu_group_unref(group);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_finalize(mut object: *mut GObject) {
    let mut proxy: *mut GDBusMenuModel = object as *mut ::core::ffi::c_void as *mut GDBusMenuModel;
    if (*proxy).active != 0 {
        safe_c2rust_g_dbus_menu_group_deactivate((*proxy).group);
    }
    g_hash_table_remove(
        (*(*proxy).group).proxies,
        (*proxy).id as glong as gpointer as gconstpointer,
    );
    safe_c2rust_g_dbus_menu_group_unref((*proxy).group);
    (*(safe_c2rust_g_dbus_menu_model_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_init(mut proxy: *mut GDBusMenuModel) {}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_class_init(mut class: *mut GDBusMenuModelClass) {
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*class).is_mutable = Some(
        safe_c2rust_g_dbus_menu_model_is_mutable
            as unsafe extern "C" fn(*mut GMenuModel) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GMenuModel) -> gboolean>;
    (*class).get_n_items = Some(
        safe_c2rust_g_dbus_menu_model_get_n_items as unsafe extern "C" fn(*mut GMenuModel) -> gint,
    ) as Option<unsafe extern "C" fn(*mut GMenuModel) -> gint>;
    (*class).get_item_attributes = Some(
        safe_c2rust_g_dbus_menu_model_get_item_attributes
            as unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> ()>;
    (*class).get_item_links = Some(
        safe_c2rust_g_dbus_menu_model_get_item_links
            as unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GMenuModel, gint, *mut *mut GHashTable) -> ()>;
    (*object_class).finalize =
        Some(safe_c2rust_g_dbus_menu_model_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_changed(
    mut proxy: *mut GDBusMenuModel,
    mut items: *mut GSequence,
    mut position: gint,
    mut removed: gint,
    mut added: gint,
) {
    (*proxy).items = items;
    if (*proxy).active != 0 && (removed != 0 || added != 0) {
        g_menu_model_items_changed(
            proxy as *mut ::core::ffi::c_void as *mut GMenuModel,
            position,
            removed,
            added,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_get_from_group(
    mut group: *mut GDBusMenuGroup,
    mut menu_id: guint,
) -> *mut GDBusMenuModel {
    let mut proxy: *mut GDBusMenuModel = ::core::ptr::null_mut::<GDBusMenuModel>();
    proxy = g_hash_table_lookup(
        (*group).proxies,
        menu_id as glong as gpointer as gconstpointer,
    ) as *mut GDBusMenuModel;
    if !proxy.is_null() {
        g_object_ref(proxy as gpointer);
    }
    if proxy.is_null() {
        proxy = g_object_new(
            safe_c2rust_g_dbus_menu_model_get_type(),
            ::core::ptr::null::<gchar>(),
        ) as *mut GDBusMenuModel;
        (*proxy).items = g_hash_table_lookup(
            (*group).menus,
            menu_id as glong as gpointer as gconstpointer,
        ) as *mut GSequence;
        g_hash_table_insert(
            (*group).proxies,
            menu_id as glong as gpointer,
            proxy as gpointer,
        );
        (*proxy).group = safe_c2rust_g_dbus_menu_group_ref(group);
        (*proxy).id = menu_id;
    }
    return proxy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_menu_model_get(
    mut connection: *mut GDBusConnection,
    mut bus_name: *const gchar,
    mut object_path: *const gchar,
) -> *mut GDBusMenuModel {
    let mut group: *mut GDBusMenuGroup = ::core::ptr::null_mut::<GDBusMenuGroup>();
    let mut proxy: *mut GDBusMenuModel = ::core::ptr::null_mut::<GDBusMenuModel>();
    let mut context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !bus_name.is_null() || g_dbus_connection_get_unique_name(connection).is_null() {
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
            b"bus_name != NULL || g_dbus_connection_get_unique_name (connection) == NULL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMenuModel>();
    }
    context = g_main_context_get_thread_default();
    if context.is_null() {
        context = g_main_context_default();
    }
    group =
        safe_c2rust_g_dbus_menu_group_get(context, connection, bus_name, object_path, 0 as guint);
    proxy = safe_c2rust_g_dbus_menu_model_get_from_group(group, 0 as guint);
    safe_c2rust_g_dbus_menu_group_unref(group);
    return proxy;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
