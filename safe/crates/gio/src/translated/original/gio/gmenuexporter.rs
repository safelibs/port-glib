use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GSequence;
    pub type _GSequenceNode;
    pub type _GMenuModelPrivate;
    pub type _GDBusConnection;
    pub type _GDBusMethodInvocation;
    pub type _GMenuLinkIterPrivate;
    pub type _GMenuAttributeIterPrivate;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
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
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_get_child_value(value: *mut GVariant, index_: gsize) -> *mut GVariant;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_next(iter: *mut GVariantIter, format_string: *const gchar, ...) -> gboolean;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_open(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_close(builder: *mut GVariantBuilder);
    fn g_variant_builder_add_value(builder: *mut GVariantBuilder, value: *mut GVariant);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
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
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_signal_connect_data(
        instance: gpointer,
        detailed_signal: *const gchar,
        c_handler: GCallback,
        data: gpointer,
        destroy_data: GClosureNotify,
        connect_flags: GConnectFlags,
    ) -> gulong;
    fn g_signal_handler_disconnect(instance: gpointer, handler_id: gulong);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
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
    fn g_menu_model_is_mutable(model: *mut GMenuModel) -> gboolean;
    fn g_menu_model_get_n_items(model: *mut GMenuModel) -> gint;
    fn g_menu_model_iterate_item_attributes(
        model: *mut GMenuModel,
        item_index: gint,
    ) -> *mut GMenuAttributeIter;
    fn g_menu_model_iterate_item_links(
        model: *mut GMenuModel,
        item_index: gint,
    ) -> *mut GMenuLinkIter;
    fn g_menu_attribute_iter_get_next(
        iter: *mut GMenuAttributeIter,
        out_name: *mut *const gchar,
        value: *mut *mut GVariant,
    ) -> gboolean;
    fn g_menu_link_iter_get_next(
        iter: *mut GMenuLinkIter,
        out_link: *mut *const gchar,
        value: *mut *mut GMenuModel,
    ) -> gboolean;
    fn g_dbus_method_invocation_return_value(
        invocation: *mut GDBusMethodInvocation,
        parameters: *mut GVariant,
    );
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
    fn g_bus_watch_name_on_connection(
        connection: *mut GDBusConnection,
        name: *const gchar,
        flags: GBusNameWatcherFlags,
        name_appeared_handler: GBusNameAppearedCallback,
        name_vanished_handler: GBusNameVanishedCallback,
        user_data: gpointer,
        user_data_free_func: GDestroyNotify,
    ) -> guint;
    fn g_bus_unwatch_name(watcher_id: guint);
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
pub type GSequence = _GSequence;
pub type GSequenceIter = _GSequenceNode;
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
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
pub type GBusNameWatcherFlags = ::core::ffi::c_uint;
pub const G_BUS_NAME_WATCHER_FLAGS_AUTO_START: GBusNameWatcherFlags = 1;
pub const G_BUS_NAME_WATCHER_FLAGS_NONE: GBusNameWatcherFlags = 0;
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuModel {
    pub parent_instance: GObject,
    pub priv_0: *mut GMenuModelPrivate,
}
pub type GMenuModelPrivate = _GMenuModelPrivate;
pub type GMenuModel = _GMenuModel;
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
pub type GMenuExporterMenu = _GMenuExporterMenu;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuExporterMenu {
    pub group: *mut GMenuExporterGroup,
    pub id: guint,
    pub model: *mut GMenuModel,
    pub handler_id: gulong,
    pub item_links: *mut GSequence,
}
pub type GMenuExporterGroup = _GMenuExporterGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuExporterGroup {
    pub exporter: *mut GMenuExporter,
    pub id: guint,
    pub menus: *mut GHashTable,
    pub next_menu_id: guint,
    pub prepared: gboolean,
    pub subscribed: gint,
}
pub type GMenuExporter = _GMenuExporter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuExporter {
    pub connection: *mut GDBusConnection,
    pub object_path: *mut gchar,
    pub registration_id: guint,
    pub groups: *mut GHashTable,
    pub next_group_id: guint,
    pub root: *mut GMenuExporterMenu,
    pub peer_remote: *mut GMenuExporterRemote,
    pub remotes: *mut GHashTable,
}
pub type GMenuExporterRemote = _GMenuExporterRemote;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuExporterRemote {
    pub exporter: *mut GMenuExporter,
    pub watches: *mut GHashTable,
    pub watch_id: guint,
}
pub type GMenuExporterLink = _GMenuExporterLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMenuExporterLink {
    pub name: *mut gchar,
    pub menu: *mut GMenuExporterMenu,
    pub next: *mut GMenuExporterLink,
}
pub type GBusNameAppearedCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, *const gchar, gpointer) -> ()>;
pub type GBusNameVanishedCallback =
    Option<unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> ()>;
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
pub const G_VARIANT_TYPE_ARRAY: *const GVariantType =
    b"a*\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_TUPLE: *const GVariantType =
    b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_VARDICT: *const GVariantType =
    b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
unsafe extern "C" fn safe_c2rust_org_gtk_Menus_get_interface() -> *mut GDBusInterfaceInfo {
    static mut safe_c2rust_interface_info: *mut GDBusInterfaceInfo =
        ::core::ptr::null::<GDBusInterfaceInfo>() as *mut GDBusInterfaceInfo;
    static mut safe_c2rust_interface_info_initialized: gsize = 0 as gsize;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_interface_info_initialized;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_interface_info_initialized;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(
                &raw mut safe_c2rust_interface_info_initialized as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut info: *mut GDBusNodeInfo = ::core::ptr::null_mut::<GDBusNodeInfo>();
        info = g_dbus_node_info_new_for_xml(
            b"<node>  <interface name='org.gtk.Menus'>    <method name='Start'>      <arg type='au' name='groups' direction='in'/>      <arg type='a(uuaa{sv})' name='content' direction='out'/>    </method>    <method name='End'>      <arg type='au' name='groups' direction='in'/>    </method>    <signal name='Changed'>      arg type='a(uuuuaa{sv})' name='changes'/>    </signal>  </interface></node>\0"
                as *const u8 as *const gchar,
            &raw mut error,
        );
        if info.is_null() {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"%s\0" as *const u8 as *const gchar,
                (*error).message,
            );
            loop {}
        }
        safe_c2rust_interface_info = g_dbus_node_info_lookup_interface(
            info,
            b"org.gtk.Menus\0" as *const u8 as *const gchar,
        );
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if !safe_c2rust_interface_info.is_null() {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int,
                G_STRFUNC,
                b"interface_info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_dbus_interface_info_ref(safe_c2rust_interface_info);
        g_dbus_node_info_unref(info);
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_interface_info_initialized = 1 as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_interface_info_initialized as *mut ::core::ffi::c_void,
            1 as ::core::ffi::c_int as gsize,
        );
    }
    return safe_c2rust_interface_info;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_menu_free(mut menu: *mut GMenuExporterMenu) {
    safe_c2rust_g_menu_exporter_group_remove_menu((*menu).group, (*menu).id);
    if (*menu).handler_id != 0 as gulong {
        g_signal_handler_disconnect((*menu).model as gpointer, (*menu).handler_id);
    }
    if !(*menu).item_links.is_null() {
        g_sequence_free((*menu).item_links);
    }
    g_object_unref((*menu).model as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GMenuExporterMenu>() as gsize,
        menu as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_link_free(mut data: gpointer) {
    let mut link: *mut GMenuExporterLink = data as *mut GMenuExporterLink;
    while !link.is_null() {
        let mut tmp: *mut GMenuExporterLink = link;
        link = (*tmp).next;
        safe_c2rust_g_menu_exporter_menu_free((*tmp).menu);
        g_free((*tmp).name as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<GMenuExporterLink>() as gsize,
            tmp as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_menu_create_links(
    mut menu: *mut GMenuExporterMenu,
    mut position: gint,
) -> *mut GMenuExporterLink {
    let mut list: *mut GMenuExporterLink = ::core::ptr::null_mut::<GMenuExporterLink>();
    let mut iter: *mut GMenuLinkIter = ::core::ptr::null_mut::<GMenuLinkIter>();
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut model: *mut GMenuModel = ::core::ptr::null_mut::<GMenuModel>();
    iter = g_menu_model_iterate_item_links((*menu).model, position);
    while g_menu_link_iter_get_next(iter, &raw mut name, &raw mut model) != 0 {
        let mut group: *mut GMenuExporterGroup = ::core::ptr::null_mut::<GMenuExporterGroup>();
        let mut tmp: *mut GMenuExporterLink = ::core::ptr::null_mut::<GMenuExporterLink>();
        if !(strcmp(
            name,
            b"section\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            group = safe_c2rust_g_menu_exporter_create_group(
                safe_c2rust_g_menu_exporter_group_get_exporter((*menu).group),
            );
        } else {
            group = (*menu).group;
        }
        tmp = g_slice_alloc(::core::mem::size_of::<GMenuExporterLink>() as gsize)
            as *mut GMenuExporterLink;
        (*tmp).name = g_strconcat(b":\0" as *const u8 as *const gchar, name, NULL_0);
        (*tmp).menu = safe_c2rust_g_menu_exporter_group_add_menu(group, model);
        (*tmp).next = list;
        list = tmp;
        g_object_unref(model as gpointer);
    }
    g_object_unref(iter as gpointer);
    return list;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_menu_describe_item(
    mut menu: *mut GMenuExporterMenu,
    mut position: gint,
) -> *mut GVariant {
    let mut attr_iter: *mut GMenuAttributeIter = ::core::ptr::null_mut::<GMenuAttributeIter>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut iter: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    let mut link: *mut GMenuExporterLink = ::core::ptr::null_mut::<GMenuExporterLink>();
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    g_variant_builder_init(&raw mut builder, G_VARIANT_TYPE_VARDICT);
    attr_iter = g_menu_model_iterate_item_attributes((*menu).model, position);
    while g_menu_attribute_iter_get_next(attr_iter, &raw mut name, &raw mut value) != 0 {
        g_variant_builder_add(
            &raw mut builder,
            b"{sv}\0" as *const u8 as *const gchar,
            name,
            value,
        );
        g_variant_unref(value);
    }
    g_object_unref(attr_iter as gpointer);
    iter = g_sequence_get_iter_at_pos((*menu).item_links, position);
    link = g_sequence_get(iter) as *mut GMenuExporterLink;
    while !link.is_null() {
        g_variant_builder_add(
            &raw mut builder,
            b"{sv}\0" as *const u8 as *const gchar,
            (*link).name,
            g_variant_new(
                b"(uu)\0" as *const u8 as *const gchar,
                safe_c2rust_g_menu_exporter_group_get_id((*(*link).menu).group),
                (*(*link).menu).id,
            ),
        );
        link = (*link).next;
    }
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_menu_list(
    mut menu: *mut GMenuExporterMenu,
) -> *mut GVariant {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut i: gint = 0;
    let mut n: gint = 0;
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"aa{sv}\0" as *const u8 as *const gchar),
    );
    n = g_sequence_get_length((*menu).item_links);
    i = 0 as ::core::ffi::c_int as gint;
    while i < n {
        g_variant_builder_add_value(
            &raw mut builder,
            safe_c2rust_g_menu_exporter_menu_describe_item(menu, i),
        );
        i += 1;
    }
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_menu_items_changed(
    mut model: *mut GMenuModel,
    mut position: gint,
    mut removed: gint,
    mut added: gint,
    mut user_data: gpointer,
) {
    let mut menu: *mut GMenuExporterMenu = user_data as *mut GMenuExporterMenu;
    let mut point: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    let mut i: gint = 0;
    let mut n_items: gint = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*menu).model == model {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            247 as ::core::ffi::c_int,
            G_STRFUNC,
            b"menu->model == model\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !(*menu).item_links.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_int,
            G_STRFUNC,
            b"menu->item_links != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    n_items = g_sequence_get_length((*menu).item_links);
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if position >= 0 as ::core::ffi::c_int && position < 1000 as ::core::ffi::c_int {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            253 as ::core::ffi::c_int,
            G_STRFUNC,
            b"position >= 0 && position < G_MENU_EXPORTER_MAX_SECTION_SIZE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if removed >= 0 as ::core::ffi::c_int && removed < 1000 as ::core::ffi::c_int {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            254 as ::core::ffi::c_int,
            G_STRFUNC,
            b"removed >= 0 && removed < G_MENU_EXPORTER_MAX_SECTION_SIZE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if added < 1000 as ::core::ffi::c_int {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            255 as ::core::ffi::c_int,
            G_STRFUNC,
            b"added < G_MENU_EXPORTER_MAX_SECTION_SIZE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if position + removed <= n_items {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            256 as ::core::ffi::c_int,
            G_STRFUNC,
            b"position + removed <= n_items\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if n_items - removed + added < 1000 as ::core::ffi::c_int {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            257 as ::core::ffi::c_int,
            G_STRFUNC,
            b"n_items - removed + added < G_MENU_EXPORTER_MAX_SECTION_SIZE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    point = g_sequence_get_iter_at_pos((*menu).item_links, position + removed);
    g_sequence_remove_range(
        g_sequence_get_iter_at_pos((*menu).item_links, position),
        point,
    );
    i = position;
    while i < position + added {
        g_sequence_insert_before(
            point,
            safe_c2rust_g_menu_exporter_menu_create_links(menu, i) as gpointer,
        );
        i += 1;
    }
    if safe_c2rust_g_menu_exporter_group_is_subscribed((*menu).group) != 0 {
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
            g_variant_type_checked_(b"(uuuuaa{sv})\0" as *const u8 as *const gchar),
        );
        g_variant_builder_add(
            &raw mut builder,
            b"u\0" as *const u8 as *const gchar,
            safe_c2rust_g_menu_exporter_group_get_id((*menu).group),
        );
        g_variant_builder_add(
            &raw mut builder,
            b"u\0" as *const u8 as *const gchar,
            (*menu).id,
        );
        g_variant_builder_add(
            &raw mut builder,
            b"u\0" as *const u8 as *const gchar,
            position,
        );
        g_variant_builder_add(
            &raw mut builder,
            b"u\0" as *const u8 as *const gchar,
            removed,
        );
        g_variant_builder_open(
            &raw mut builder,
            g_variant_type_checked_(b"aa{sv}\0" as *const u8 as *const gchar),
        );
        i = position;
        while i < position + added {
            g_variant_builder_add_value(
                &raw mut builder,
                safe_c2rust_g_menu_exporter_menu_describe_item(menu, i),
            );
            i += 1;
        }
        g_variant_builder_close(&raw mut builder);
        safe_c2rust_g_menu_exporter_report(
            safe_c2rust_g_menu_exporter_group_get_exporter((*menu).group),
            g_variant_builder_end(&raw mut builder),
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_menu_prepare(mut menu: *mut GMenuExporterMenu) {
    let mut n_items: gint = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if (*menu).item_links.is_null() {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            289 as ::core::ffi::c_int,
            G_STRFUNC,
            b"menu->item_links == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if g_menu_model_is_mutable((*menu).model) != 0 {
        (*menu).handler_id = g_signal_connect_data(
            (*menu).model as gpointer,
            b"items-changed\0" as *const u8 as *const gchar,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMenuModel, gint, gint, gint, gpointer) -> ()>,
                GCallback,
            >(Some(
                safe_c2rust_g_menu_exporter_menu_items_changed
                    as unsafe extern "C" fn(*mut GMenuModel, gint, gint, gint, gpointer) -> (),
            )),
            menu as gpointer,
            None,
            G_CONNECT_DEFAULT,
        );
    }
    (*menu).item_links = g_sequence_new(Some(
        safe_c2rust_g_menu_exporter_link_free as unsafe extern "C" fn(gpointer) -> (),
    ));
    n_items = g_menu_model_get_n_items((*menu).model);
    if n_items != 0 {
        safe_c2rust_g_menu_exporter_menu_items_changed(
            (*menu).model,
            0 as gint,
            0 as gint,
            n_items,
            menu as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_menu_new(
    mut group: *mut GMenuExporterGroup,
    mut id: guint,
    mut model: *mut GMenuModel,
) -> *mut GMenuExporterMenu {
    let mut menu: *mut GMenuExporterMenu = ::core::ptr::null_mut::<GMenuExporterMenu>();
    menu = ({
        let mut __s: gsize = ::core::mem::size_of::<GMenuExporterMenu>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GMenuExporterMenu;
    (*menu).group = group;
    (*menu).id = id;
    (*menu).model = g_object_ref(model as gpointer) as *mut GMenuModel as *mut GMenuModel;
    return menu;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_group_check_if_useless(
    mut group: *mut GMenuExporterGroup,
) {
    if g_hash_table_size((*group).menus) == 0 as guint
        && (*group).subscribed == 0 as ::core::ffi::c_int
    {
        safe_c2rust_g_menu_exporter_remove_group((*group).exporter, (*group).id);
        g_hash_table_unref((*group).menus);
        g_slice_free1(
            ::core::mem::size_of::<GMenuExporterGroup>() as gsize,
            group as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_group_subscribe(
    mut group: *mut GMenuExporterGroup,
    mut builder: *mut GVariantBuilder,
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
    let mut val: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if (*group).prepared == 0 {
        let mut menu: *mut GMenuExporterMenu = ::core::ptr::null_mut::<GMenuExporterMenu>();
        (*group).prepared = TRUE as gboolean;
        menu = g_hash_table_lookup((*group).menus, ::core::ptr::null::<::core::ffi::c_void>())
            as *mut GMenuExporterMenu;
        if !menu.is_null() {
            safe_c2rust_g_menu_exporter_menu_prepare(menu);
        }
    }
    (*group).subscribed += 1;
    g_hash_table_iter_init(&raw mut iter, (*group).menus);
    while g_hash_table_iter_next(&raw mut iter, &raw mut key, &raw mut val) != 0 {
        let mut id: guint = key as glong as gint as guint;
        let mut menu_0: *mut GMenuExporterMenu = val as *mut GMenuExporterMenu;
        if g_sequence_is_empty((*menu_0).item_links) == 0 {
            g_variant_builder_open(
                builder,
                g_variant_type_checked_(b"(uuaa{sv})\0" as *const u8 as *const gchar),
            );
            g_variant_builder_add(builder, b"u\0" as *const u8 as *const gchar, (*group).id);
            g_variant_builder_add(builder, b"u\0" as *const u8 as *const gchar, id);
            g_variant_builder_add_value(builder, safe_c2rust_g_menu_exporter_menu_list(menu_0));
            g_variant_builder_close(builder);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_group_unsubscribe(
    mut group: *mut GMenuExporterGroup,
    mut count: gint,
) {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if (*group).subscribed >= count {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            396 as ::core::ffi::c_int,
            G_STRFUNC,
            b"group->subscribed >= count\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*group).subscribed -= count;
    safe_c2rust_g_menu_exporter_group_check_if_useless(group);
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_group_get_exporter(
    mut group: *mut GMenuExporterGroup,
) -> *mut GMenuExporter {
    return (*group).exporter;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_group_is_subscribed(
    mut group: *mut GMenuExporterGroup,
) -> gboolean {
    return ((*group).subscribed > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_group_get_id(
    mut group: *mut GMenuExporterGroup,
) -> guint {
    return (*group).id;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_group_remove_menu(
    mut group: *mut GMenuExporterGroup,
    mut id: guint,
) {
    g_hash_table_remove((*group).menus, id as glong as gpointer as gconstpointer);
    safe_c2rust_g_menu_exporter_group_check_if_useless(group);
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_group_add_menu(
    mut group: *mut GMenuExporterGroup,
    mut model: *mut GMenuModel,
) -> *mut GMenuExporterMenu {
    let mut menu: *mut GMenuExporterMenu = ::core::ptr::null_mut::<GMenuExporterMenu>();
    let mut id: guint = 0;
    let fresh1 = (*group).next_menu_id;
    (*group).next_menu_id = (*group).next_menu_id.wrapping_add(1);
    id = fresh1;
    menu = safe_c2rust_g_menu_exporter_menu_new(group, id, model);
    g_hash_table_insert((*group).menus, id as glong as gpointer, menu as gpointer);
    if (*group).prepared != 0 {
        safe_c2rust_g_menu_exporter_menu_prepare(menu);
    }
    return menu;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_group_new(
    mut exporter: *mut GMenuExporter,
    mut id: guint,
) -> *mut GMenuExporterGroup {
    let mut group: *mut GMenuExporterGroup = ::core::ptr::null_mut::<GMenuExporterGroup>();
    group = ({
        let mut __s: gsize = ::core::mem::size_of::<GMenuExporterGroup>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GMenuExporterGroup;
    (*group).menus = g_hash_table_new(None, None);
    (*group).exporter = exporter;
    (*group).id = id;
    return group;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_remote_subscribe(
    mut remote: *mut GMenuExporterRemote,
    mut group_id: guint,
    mut builder: *mut GVariantBuilder,
) {
    let mut group: *mut GMenuExporterGroup = ::core::ptr::null_mut::<GMenuExporterGroup>();
    let mut count: guint = 0;
    count = g_hash_table_lookup(
        (*remote).watches,
        group_id as glong as gpointer as gconstpointer,
    ) as gsize as guint;
    g_hash_table_insert(
        (*remote).watches,
        group_id as glong as gpointer,
        count.wrapping_add(1 as guint) as glong as gpointer,
    );
    group = safe_c2rust_g_menu_exporter_lookup_group((*remote).exporter, group_id);
    safe_c2rust_g_menu_exporter_group_subscribe(group, builder);
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_remote_unsubscribe(
    mut remote: *mut GMenuExporterRemote,
    mut group_id: guint,
) {
    let mut group: *mut GMenuExporterGroup = ::core::ptr::null_mut::<GMenuExporterGroup>();
    let mut count: guint = 0;
    count = g_hash_table_lookup(
        (*remote).watches,
        group_id as glong as gpointer as gconstpointer,
    ) as gsize as guint;
    if count == 0 as guint {
        return;
    }
    if count != 1 as guint {
        g_hash_table_insert(
            (*remote).watches,
            group_id as glong as gpointer,
            count.wrapping_sub(1 as guint) as glong as gpointer,
        );
    } else {
        g_hash_table_remove(
            (*remote).watches,
            group_id as glong as gpointer as gconstpointer,
        );
    }
    group = safe_c2rust_g_menu_exporter_lookup_group((*remote).exporter, group_id);
    safe_c2rust_g_menu_exporter_group_unsubscribe(group, 1 as gint);
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_remote_has_subscriptions(
    mut remote: *mut GMenuExporterRemote,
) -> gboolean {
    return (g_hash_table_size((*remote).watches) != 0 as guint) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_remote_free(mut data: gpointer) {
    let mut remote: *mut GMenuExporterRemote = data as *mut GMenuExporterRemote;
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut val: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    g_hash_table_iter_init(&raw mut iter, (*remote).watches);
    while g_hash_table_iter_next(&raw mut iter, &raw mut key, &raw mut val) != 0 {
        let mut group: *mut GMenuExporterGroup = ::core::ptr::null_mut::<GMenuExporterGroup>();
        group = safe_c2rust_g_menu_exporter_lookup_group(
            (*remote).exporter,
            key as glong as gint as guint,
        );
        safe_c2rust_g_menu_exporter_group_unsubscribe(group, val as glong as gint);
    }
    if (*remote).watch_id > 0 as guint {
        g_bus_unwatch_name((*remote).watch_id);
    }
    g_hash_table_unref((*remote).watches);
    g_slice_free1(
        ::core::mem::size_of::<GMenuExporterRemote>() as gsize,
        remote as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_remote_new(
    mut exporter: *mut GMenuExporter,
    mut watch_id: guint,
) -> *mut GMenuExporterRemote {
    let mut remote: *mut GMenuExporterRemote = ::core::ptr::null_mut::<GMenuExporterRemote>();
    remote = ({
        let mut __s: gsize = ::core::mem::size_of::<GMenuExporterRemote>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GMenuExporterRemote;
    (*remote).exporter = exporter;
    (*remote).watches = g_hash_table_new(None, None);
    (*remote).watch_id = watch_id;
    return remote;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_name_vanished(
    mut connection: *mut GDBusConnection,
    mut name: *const gchar,
    mut user_data: gpointer,
) {
    let mut exporter: *mut GMenuExporter = user_data as *mut GMenuExporter;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*exporter).connection == connection || connection.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            574 as ::core::ffi::c_int,
            G_STRFUNC,
            b"exporter->connection == connection || connection == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_remove((*exporter).remotes, name as gconstpointer);
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_subscribe(
    mut exporter: *mut GMenuExporter,
    mut sender: *const gchar,
    mut group_ids: *mut GVariant,
) -> *mut GVariant {
    let mut remote: *mut GMenuExporterRemote = ::core::ptr::null_mut::<GMenuExporterRemote>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut id: guint32 = 0;
    if !sender.is_null() {
        remote = g_hash_table_lookup((*exporter).remotes, sender as gconstpointer)
            as *mut GMenuExporterRemote;
    } else {
        remote = (*exporter).peer_remote;
    }
    if remote.is_null() {
        if !sender.is_null() {
            let mut watch_id: guint = 0;
            watch_id = g_bus_watch_name_on_connection(
                (*exporter).connection,
                sender,
                G_BUS_NAME_WATCHER_FLAGS_NONE,
                None,
                Some(
                    safe_c2rust_g_menu_exporter_name_vanished
                        as unsafe extern "C" fn(*mut GDBusConnection, *const gchar, gpointer) -> (),
                ),
                exporter as gpointer,
                None,
            );
            remote = safe_c2rust_g_menu_exporter_remote_new(exporter, watch_id);
            g_hash_table_insert(
                (*exporter).remotes,
                safe_c2rust_g_strdup_inline(sender as *const ::core::ffi::c_char) as gpointer,
                remote as gpointer,
            );
        } else {
            (*exporter).peer_remote = safe_c2rust_g_menu_exporter_remote_new(exporter, 0 as guint);
            remote = (*exporter).peer_remote;
        }
    }
    g_variant_builder_init(
        &raw mut builder,
        g_variant_type_checked_(b"(a(uuaa{sv}))\0" as *const u8 as *const gchar),
    );
    g_variant_builder_open(
        &raw mut builder,
        g_variant_type_checked_(b"a(uuaa{sv})\0" as *const u8 as *const gchar),
    );
    g_variant_iter_init(&raw mut iter, group_ids);
    while g_variant_iter_next(
        &raw mut iter,
        b"u\0" as *const u8 as *const gchar,
        &raw mut id,
    ) != 0
    {
        safe_c2rust_g_menu_exporter_remote_subscribe(remote, id as guint, &raw mut builder);
    }
    g_variant_builder_close(&raw mut builder);
    return g_variant_builder_end(&raw mut builder);
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_unsubscribe(
    mut exporter: *mut GMenuExporter,
    mut sender: *const gchar,
    mut group_ids: *mut GVariant,
) {
    let mut remote: *mut GMenuExporterRemote = ::core::ptr::null_mut::<GMenuExporterRemote>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut id: guint32 = 0;
    if !sender.is_null() {
        remote = g_hash_table_lookup((*exporter).remotes, sender as gconstpointer)
            as *mut GMenuExporterRemote;
    } else {
        remote = (*exporter).peer_remote;
    }
    if remote.is_null() {
        return;
    }
    g_variant_iter_init(&raw mut iter, group_ids);
    while g_variant_iter_next(
        &raw mut iter,
        b"u\0" as *const u8 as *const gchar,
        &raw mut id,
    ) != 0
    {
        safe_c2rust_g_menu_exporter_remote_unsubscribe(remote, id as guint);
    }
    if safe_c2rust_g_menu_exporter_remote_has_subscriptions(remote) == 0 {
        if !sender.is_null() {
            g_hash_table_remove((*exporter).remotes, sender as gconstpointer);
        } else {
            let mut _pp: *mut *mut GMenuExporterRemote = &raw mut (*exporter).peer_remote;
            let mut _ptr: *mut GMenuExporterRemote = *_pp;
            *_pp = ::core::ptr::null_mut::<GMenuExporterRemote>();
            if !_ptr.is_null() {
                safe_c2rust_g_menu_exporter_remote_free(_ptr as gpointer);
            }
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_report(
    mut exporter: *mut GMenuExporter,
    mut report: *mut GVariant,
) {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    g_variant_builder_init(&raw mut builder, G_VARIANT_TYPE_TUPLE);
    g_variant_builder_open(&raw mut builder, G_VARIANT_TYPE_ARRAY);
    g_variant_builder_add_value(&raw mut builder, report);
    g_variant_builder_close(&raw mut builder);
    g_dbus_connection_emit_signal(
        (*exporter).connection,
        ::core::ptr::null::<gchar>(),
        (*exporter).object_path,
        b"org.gtk.Menus\0" as *const u8 as *const gchar,
        b"Changed\0" as *const u8 as *const gchar,
        g_variant_builder_end(&raw mut builder),
        ::core::ptr::null_mut::<*mut GError>(),
    );
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_remove_group(
    mut exporter: *mut GMenuExporter,
    mut id: guint,
) {
    g_hash_table_remove((*exporter).groups, id as glong as gpointer as gconstpointer);
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_lookup_group(
    mut exporter: *mut GMenuExporter,
    mut group_id: guint,
) -> *mut GMenuExporterGroup {
    let mut group: *mut GMenuExporterGroup = ::core::ptr::null_mut::<GMenuExporterGroup>();
    group = g_hash_table_lookup(
        (*exporter).groups,
        group_id as glong as gpointer as gconstpointer,
    ) as *mut GMenuExporterGroup;
    if group.is_null() {
        group = safe_c2rust_g_menu_exporter_group_new(exporter, group_id);
        g_hash_table_insert(
            (*exporter).groups,
            group_id as glong as gpointer,
            group as gpointer,
        );
    }
    return group;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_create_group(
    mut exporter: *mut GMenuExporter,
) -> *mut GMenuExporterGroup {
    let mut group: *mut GMenuExporterGroup = ::core::ptr::null_mut::<GMenuExporterGroup>();
    let mut id: guint = 0;
    let fresh0 = (*exporter).next_group_id;
    (*exporter).next_group_id = (*exporter).next_group_id.wrapping_add(1);
    id = fresh0;
    group = safe_c2rust_g_menu_exporter_group_new(exporter, id);
    g_hash_table_insert(
        (*exporter).groups,
        id as glong as gpointer,
        group as gpointer,
    );
    return group;
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_free(mut user_data: gpointer) {
    let mut exporter: *mut GMenuExporter = user_data as *mut GMenuExporter;
    safe_c2rust_g_menu_exporter_menu_free((*exporter).root);
    let mut _pp: *mut *mut GMenuExporterRemote = &raw mut (*exporter).peer_remote;
    let mut _ptr: *mut GMenuExporterRemote = *_pp;
    *_pp = ::core::ptr::null_mut::<GMenuExporterRemote>();
    if !_ptr.is_null() {
        safe_c2rust_g_menu_exporter_remote_free(_ptr as gpointer);
    }
    g_hash_table_unref((*exporter).remotes);
    g_hash_table_unref((*exporter).groups);
    g_object_unref((*exporter).connection as gpointer);
    g_free((*exporter).object_path as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GMenuExporter>() as gsize,
        exporter as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_menu_exporter_method_call(
    mut connection: *mut GDBusConnection,
    mut sender: *const gchar,
    mut object_path: *const gchar,
    mut interface_name: *const gchar,
    mut method_name: *const gchar,
    mut parameters: *mut GVariant,
    mut invocation: *mut GDBusMethodInvocation,
    mut user_data: gpointer,
) {
    let mut exporter: *mut GMenuExporter = user_data as *mut GMenuExporter;
    let mut group_ids: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    group_ids = g_variant_get_child_value(parameters, 0 as gsize);
    if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"Start\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        g_dbus_method_invocation_return_value(
            invocation,
            safe_c2rust_g_menu_exporter_subscribe(exporter, sender, group_ids),
        );
    } else if strcmp(
        method_name as *const ::core::ffi::c_char,
        b"End\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        safe_c2rust_g_menu_exporter_unsubscribe(exporter, sender, group_ids);
        g_dbus_method_invocation_return_value(invocation, ::core::ptr::null_mut::<GVariant>());
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gmenuexporter.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            749 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    g_variant_unref(group_ids);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_export_menu_model(
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
    mut menu: *mut GMenuModel,
    mut error: *mut *mut GError,
) -> guint {
    let vtable: GDBusInterfaceVTable = _GDBusInterfaceVTable {
        method_call: Some(
            safe_c2rust_g_menu_exporter_method_call
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
        get_property: None,
        set_property: None,
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
    let mut exporter: *mut GMenuExporter = ::core::ptr::null_mut::<GMenuExporter>();
    let mut id: guint = 0;
    exporter = ({
        let mut __s: gsize = ::core::mem::size_of::<GMenuExporter>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GMenuExporter;
    id = g_dbus_connection_register_object(
        connection,
        object_path,
        safe_c2rust_org_gtk_Menus_get_interface(),
        &raw const vtable,
        exporter as gpointer,
        Some(safe_c2rust_g_menu_exporter_free as unsafe extern "C" fn(gpointer) -> ()),
        error,
    );
    if id == 0 as guint {
        g_slice_free1(
            ::core::mem::size_of::<GMenuExporter>() as gsize,
            exporter as gpointer,
        );
        return 0 as guint;
    }
    (*exporter).connection =
        g_object_ref(connection as gpointer) as *mut GDBusConnection as *mut GDBusConnection;
    (*exporter).object_path =
        safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as *mut gchar;
    (*exporter).groups = g_hash_table_new(None, None);
    (*exporter).remotes = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(safe_c2rust_g_menu_exporter_remote_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    (*exporter).root = safe_c2rust_g_menu_exporter_group_add_menu(
        safe_c2rust_g_menu_exporter_create_group(exporter),
        menu,
    );
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_connection_unexport_menu_model(
    mut connection: *mut GDBusConnection,
    mut export_id: guint,
) {
    g_dbus_connection_unregister_object(connection, export_id);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
