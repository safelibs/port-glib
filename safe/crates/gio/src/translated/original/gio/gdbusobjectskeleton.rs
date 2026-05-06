use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GDBusMethodInvocation;
    pub type _GDBusInterface;
    pub type _GDBusInterfaceSkeletonPrivate;
    pub type _GDBusObject;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_set_free_func(array: *mut GPtrArray, element_free_func: GDestroyNotify);
    fn g_ptr_array_foreach(array: *mut GPtrArray, func: GFunc, user_data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_list_foreach(list: *mut GList, func: GFunc, user_data: gpointer);
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
    fn g_hash_table_get_values(hash_table: *mut GHashTable) -> *mut GList;
    fn g_hash_table_get_values_as_ptr_array(hash_table: *mut GHashTable) -> *mut GPtrArray;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
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
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_param_spec_string(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: *const gchar,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_get_string(value: *const GValue) -> *const gchar;
    fn g_dbus_object_get_type() -> GType;
    fn g_dbus_interface_skeleton_get_type() -> GType;
    fn g_dbus_interface_skeleton_get_info(
        interface_: *mut GDBusInterfaceSkeleton,
    ) -> *mut GDBusInterfaceInfo;
    fn g_dbus_interface_skeleton_flush(interface_: *mut GDBusInterfaceSkeleton);
    fn _g_signal_accumulator_false_handled(
        ihint: *mut GSignalInvocationHint,
        return_accu: *mut GValue,
        handler_return: *const GValue,
        dummy: gpointer,
    ) -> gboolean;
    fn g_dbus_method_invocation_get_type() -> GType;
    fn g_dbus_interface_get_type() -> GType;
    fn g_dbus_interface_set_object(interface_: *mut GDBusInterface, object: *mut GDBusObject);
    fn g_dbus_is_interface_name(string: *const gchar) -> gboolean;
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
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
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
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub type GDBusPropertyInfoFlags = ::core::ffi::c_uint;
pub const G_DBUS_PROPERTY_INFO_FLAGS_WRITABLE: GDBusPropertyInfoFlags = 2;
pub const G_DBUS_PROPERTY_INFO_FLAGS_READABLE: GDBusPropertyInfoFlags = 1;
pub const G_DBUS_PROPERTY_INFO_FLAGS_NONE: GDBusPropertyInfoFlags = 0;
pub type GDBusMethodInvocation = _GDBusMethodInvocation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusAnnotationInfo {
    pub ref_count: gint,
    pub key: *mut gchar,
    pub value: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusAnnotationInfo = _GDBusAnnotationInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusArgInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusArgInfo = _GDBusArgInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMethodInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub in_args: *mut *mut GDBusArgInfo,
    pub out_args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusMethodInfo = _GDBusMethodInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusSignalInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub args: *mut *mut GDBusArgInfo,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusSignalInfo = _GDBusSignalInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusPropertyInfo {
    pub ref_count: gint,
    pub name: *mut gchar,
    pub signature: *mut gchar,
    pub flags: GDBusPropertyInfoFlags,
    pub annotations: *mut *mut GDBusAnnotationInfo,
}
pub type GDBusPropertyInfo = _GDBusPropertyInfo;
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
pub type GDBusInterfaceInfo = _GDBusInterfaceInfo;
pub type GDBusInterface = _GDBusInterface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusInterfaceSkeleton {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusInterfaceSkeletonPrivate,
}
pub type GDBusInterfaceSkeletonPrivate = _GDBusInterfaceSkeletonPrivate;
pub type GDBusInterfaceSkeleton = _GDBusInterfaceSkeleton;
pub type GDBusObject = _GDBusObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectSkeleton {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusObjectSkeletonPrivate,
}
pub type GDBusObjectSkeletonPrivate = _GDBusObjectSkeletonPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectSkeletonPrivate {
    pub lock: GMutex,
    pub object_path: *mut gchar,
    pub map_name_to_iface: *mut GHashTable,
}
pub type GDBusObjectSkeleton = _GDBusObjectSkeleton;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectIface {
    pub parent_iface: GTypeInterface,
    pub get_object_path: Option<unsafe extern "C" fn(*mut GDBusObject) -> *const gchar>,
    pub get_interfaces: Option<unsafe extern "C" fn(*mut GDBusObject) -> *mut GList>,
    pub get_interface:
        Option<unsafe extern "C" fn(*mut GDBusObject, *const gchar) -> *mut GDBusInterface>,
    pub interface_added: Option<unsafe extern "C" fn(*mut GDBusObject, *mut GDBusInterface) -> ()>,
    pub interface_removed:
        Option<unsafe extern "C" fn(*mut GDBusObject, *mut GDBusInterface) -> ()>,
}
pub type GDBusObjectIface = _GDBusObjectIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectSkeletonClass {
    pub parent_class: GObjectClass,
    pub authorize_method: Option<
        unsafe extern "C" fn(
            *mut GDBusObjectSkeleton,
            *mut GDBusInterfaceSkeleton,
            *mut GDBusMethodInvocation,
        ) -> gboolean,
    >,
    pub padding: [gpointer; 8],
}
pub type GDBusObjectSkeletonClass = _GDBusObjectSkeletonClass;
pub const AUTHORIZE_METHOD_SIGNAL: C2RustUnnamed_1 = 0;
pub const PROP_G_OBJECT_PATH: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const LAST_SIGNAL: C2RustUnnamed_1 = 1;
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
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_object_skeleton_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusObjectSkeleton_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusObjectSkeleton_private_offset,
        );
    }
    safe_c2rust_g_dbus_object_skeleton_class_init(klass as *mut GDBusObjectSkeletonClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_object_skeleton_get_type_once();
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
static mut safe_c2rust_g_dbus_object_skeleton_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusObjectSkeleton\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusObjectSkeletonClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_object_skeleton_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusObjectSkeleton>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusObjectSkeleton) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_object_skeleton_init
                    as unsafe extern "C" fn(*mut GDBusObjectSkeleton) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDBusObjectSkeleton_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDBusObjectSkeletonPrivate>() as gsize,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GDBusObjectIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_dbus_object_interface_init
                as unsafe extern "C" fn(*mut GDBusObjectIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_dbus_object_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_get_instance_private(
    mut self_0: *mut GDBusObjectSkeleton,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDBusObjectSkeleton_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GDBusObjectSkeleton_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_finalize(mut _object: *mut GObject) {
    let mut object: *mut GDBusObjectSkeleton =
        _object as *mut ::core::ffi::c_void as *mut GDBusObjectSkeleton;
    g_free((*(*object).priv_0).object_path as gpointer);
    g_hash_table_unref((*(*object).priv_0).map_name_to_iface);
    g_mutex_clear(&raw mut (*(*object).priv_0).lock);
    if (*(safe_c2rust_g_dbus_object_skeleton_parent_class as *mut GObjectClass))
        .finalize
        .is_some()
    {
        (*(safe_c2rust_g_dbus_object_skeleton_parent_class as *mut GObjectClass))
            .finalize
            .expect("non-null function pointer")(_object);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_get_property(
    mut _object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut object: *mut GDBusObjectSkeleton =
        _object as *mut ::core::ffi::c_void as *mut GDBusObjectSkeleton;
    match prop_id {
        1 => {
            g_mutex_lock(&raw mut (*(*object).priv_0).lock);
            g_value_set_string(value, (*(*object).priv_0).object_path);
            g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
        }
        _ => {
            let mut _glib__object: *mut GObject = object as *mut GObject;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectskeleton.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                107 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_set_property(
    mut _object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut object: *mut GDBusObjectSkeleton =
        _object as *mut ::core::ffi::c_void as *mut GDBusObjectSkeleton;
    match prop_id {
        1 => {
            safe_c2rust_g_dbus_object_skeleton_set_object_path(object, g_value_get_string(value));
        }
        _ => {
            let mut _glib__object: *mut GObject = object as *mut GObject;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectskeleton.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_authorize_method_default(
    mut object: *mut GDBusObjectSkeleton,
    mut interface: *mut GDBusInterfaceSkeleton,
    mut invocation: *mut GDBusMethodInvocation,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_class_init(
    mut klass: *mut GDBusObjectSkeletonClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_dbus_object_skeleton_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dbus_object_skeleton_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dbus_object_skeleton_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    (*klass).authorize_method = Some(
        safe_c2rust_g_dbus_object_skeleton_authorize_method_default
            as unsafe extern "C" fn(
                *mut GDBusObjectSkeleton,
                *mut GDBusInterfaceSkeleton,
                *mut GDBusMethodInvocation,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GDBusObjectSkeleton,
                *mut GDBusInterfaceSkeleton,
                *mut GDBusMethodInvocation,
            ) -> gboolean,
        >;
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
                | G_PARAM_CONSTRUCT as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
    safe_c2rust_signals[AUTHORIZE_METHOD_SIGNAL as ::core::ffi::c_int as usize] = g_signal_new(
        g_intern_static_string(b"authorize-method\0" as *const u8 as *const gchar),
        safe_c2rust_g_dbus_object_skeleton_get_type(),
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
        None,
        G_TYPE_BOOLEAN,
        2 as guint,
        g_dbus_interface_skeleton_get_type(),
        g_dbus_method_invocation_get_type(),
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_init(mut object: *mut GDBusObjectSkeleton) {
    (*object).priv_0 = safe_c2rust_g_dbus_object_skeleton_get_instance_private(object)
        as *mut GDBusObjectSkeletonPrivate;
    g_mutex_init(&raw mut (*(*object).priv_0).lock);
    (*(*object).priv_0).map_name_to_iface = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_new(
    mut object_path: *const gchar,
) -> *mut GDBusObjectSkeleton {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectSkeleton>();
    }
    return g_object_new(
        safe_c2rust_g_dbus_object_skeleton_get_type(),
        b"g-object-path\0" as *const u8 as *const gchar,
        object_path,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GDBusObjectSkeleton;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_set_object_path(
    mut object: *mut GDBusObjectSkeleton,
    mut object_path: *const gchar,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_skeleton_get_type();
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
            b"G_IS_DBUS_OBJECT_SKELETON (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if object_path.is_null() || g_variant_is_object_path(object_path) != 0 {
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
            b"object_path == NULL || g_variant_is_object_path (object_path)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*object).priv_0).lock);
    if g_strcmp0(
        (*(*object).priv_0).object_path,
        object_path as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        g_free((*(*object).priv_0).object_path as gpointer);
        (*(*object).priv_0).object_path =
            safe_c2rust_g_strdup_inline(object_path as *const ::core::ffi::c_char) as *mut gchar;
        g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
        g_object_notify(
            object as *mut ::core::ffi::c_void as *mut GObject,
            b"g-object-path\0" as *const u8 as *const gchar,
        );
    } else {
        g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_get_object_path(
    mut _object: *mut GDBusObject,
) -> *const gchar {
    let mut object: *mut GDBusObjectSkeleton =
        _object as *mut ::core::ffi::c_void as *mut GDBusObjectSkeleton;
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut (*(*object).priv_0).lock);
    ret = (*(*object).priv_0).object_path;
    g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_add_interface(
    mut object: *mut GDBusObjectSkeleton,
    mut interface_: *mut GDBusInterfaceSkeleton,
) {
    let mut info: *mut GDBusInterfaceInfo = ::core::ptr::null_mut::<GDBusInterfaceInfo>();
    let mut interface_to_remove: *mut GDBusInterface = ::core::ptr::null_mut::<GDBusInterface>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_skeleton_get_type();
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
            b"G_IS_DBUS_OBJECT_SKELETON (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = g_dbus_interface_skeleton_get_type();
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
        return;
    }
    g_mutex_lock(&raw mut (*(*object).priv_0).lock);
    info = g_dbus_interface_skeleton_get_info(interface_);
    g_object_ref(interface_ as gpointer);
    interface_to_remove = g_hash_table_lookup(
        (*(*object).priv_0).map_name_to_iface,
        (*info).name as gconstpointer,
    ) as *mut GDBusInterface;
    if !interface_to_remove.is_null() {
        g_object_ref(interface_to_remove as gpointer);
        if !(({
            let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*(*object).priv_0).map_name_to_iface,
                (*info).name as gconstpointer,
            ) != 0
            {
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectskeleton.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                305 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (object->priv->map_name_to_iface, info->name)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    g_hash_table_insert(
        (*(*object).priv_0).map_name_to_iface,
        safe_c2rust_g_strdup_inline((*info).name) as gpointer,
        g_object_ref(interface_ as gpointer) as *mut GDBusInterfaceSkeleton as gpointer,
    );
    g_dbus_interface_set_object(
        interface_ as *mut ::core::ffi::c_void as *mut GDBusInterface,
        object as *mut ::core::ffi::c_void as *mut GDBusObject,
    );
    g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
    if !interface_to_remove.is_null() {
        g_dbus_interface_set_object(interface_to_remove, ::core::ptr::null_mut::<GDBusObject>());
        g_signal_emit_by_name(
            object as gpointer,
            b"interface-removed\0" as *const u8 as *const gchar,
            interface_to_remove,
        );
        g_object_unref(interface_to_remove as gpointer);
    }
    g_signal_emit_by_name(
        object as gpointer,
        b"interface-added\0" as *const u8 as *const gchar,
        interface_,
    );
    g_object_unref(interface_ as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_remove_interface(
    mut object: *mut GDBusObjectSkeleton,
    mut interface_: *mut GDBusInterfaceSkeleton,
) {
    let mut other_interface: *mut GDBusInterfaceSkeleton =
        ::core::ptr::null_mut::<GDBusInterfaceSkeleton>();
    let mut info: *mut GDBusInterfaceInfo = ::core::ptr::null_mut::<GDBusInterfaceInfo>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_skeleton_get_type();
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
            b"G_IS_DBUS_OBJECT_SKELETON (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_ as *mut GTypeInstance;
            let mut __t: GType = g_dbus_interface_get_type();
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
            b"G_IS_DBUS_INTERFACE (interface_)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*object).priv_0).lock);
    info = g_dbus_interface_skeleton_get_info(interface_);
    other_interface = g_hash_table_lookup(
        (*(*object).priv_0).map_name_to_iface,
        (*info).name as gconstpointer,
    ) as *mut GDBusInterfaceSkeleton;
    if other_interface.is_null() {
        g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Tried to remove interface with name %s from object at path %s but no such interface exists\0"
                as *const u8 as *const gchar,
            (*info).name,
            (*(*object).priv_0).object_path,
        );
    } else if other_interface != interface_ {
        g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Tried to remove interface %p with name %s from object at path %s but the object has the interface %p\0"
                as *const u8 as *const gchar,
            interface_,
            (*info).name,
            (*(*object).priv_0).object_path,
            other_interface,
        );
    } else {
        g_object_ref(interface_ as gpointer);
        if !(({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*(*object).priv_0).map_name_to_iface,
                (*info).name as gconstpointer,
            ) != 0
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectskeleton.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                374 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (object->priv->map_name_to_iface, info->name)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
        g_dbus_interface_set_object(
            interface_ as *mut ::core::ffi::c_void as *mut GDBusInterface,
            ::core::ptr::null_mut::<GDBusObject>(),
        );
        g_signal_emit_by_name(
            object as gpointer,
            b"interface-removed\0" as *const u8 as *const gchar,
            interface_,
        );
        g_object_unref(interface_ as gpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_remove_interface_by_name(
    mut object: *mut GDBusObjectSkeleton,
    mut interface_name: *const gchar,
) {
    let mut interface: *mut GDBusInterface = ::core::ptr::null_mut::<GDBusInterface>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_skeleton_get_type();
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
            b"G_IS_DBUS_OBJECT_SKELETON (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if g_dbus_is_interface_name(interface_name) != 0 {
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
            b"g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*object).priv_0).lock);
    interface = g_hash_table_lookup(
        (*(*object).priv_0).map_name_to_iface,
        interface_name as gconstpointer,
    ) as *mut GDBusInterface;
    if !interface.is_null() {
        g_object_ref(interface as gpointer);
        if !(({
            let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*(*object).priv_0).map_name_to_iface,
                interface_name as gconstpointer,
            ) != 0
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectskeleton.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                411 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (object->priv->map_name_to_iface, interface_name)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
        g_dbus_interface_set_object(interface, ::core::ptr::null_mut::<GDBusObject>());
        g_signal_emit_by_name(
            object as gpointer,
            b"interface-removed\0" as *const u8 as *const gchar,
            interface,
        );
        g_object_unref(interface as gpointer);
    } else {
        g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_get_interface(
    mut _object: *mut GDBusObject,
    mut interface_name: *const gchar,
) -> *mut GDBusInterface {
    let mut object: *mut GDBusObjectSkeleton =
        _object as *mut ::core::ffi::c_void as *mut GDBusObjectSkeleton;
    let mut ret: *mut GDBusInterface = ::core::ptr::null_mut::<GDBusInterface>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_skeleton_get_type();
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
            b"G_IS_DBUS_OBJECT_SKELETON (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusInterface>();
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if g_dbus_is_interface_name(interface_name) != 0 {
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
            b"g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusInterface>();
    }
    g_mutex_lock(&raw mut (*(*object).priv_0).lock);
    ret = g_hash_table_lookup(
        (*(*object).priv_0).map_name_to_iface,
        interface_name as gconstpointer,
    ) as *mut GDBusInterface;
    if !ret.is_null() {
        g_object_ref(ret as gpointer);
    }
    g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_get_interfaces(
    mut _object: *mut GDBusObject,
) -> *mut GList {
    let mut object: *mut GDBusObjectSkeleton =
        _object as *mut ::core::ffi::c_void as *mut GDBusObjectSkeleton;
    let mut ret: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = object as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_skeleton_get_type();
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
            b"G_IS_DBUS_OBJECT_SKELETON (object)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    ret = ::core::ptr::null_mut::<GList>();
    g_mutex_lock(&raw mut (*(*object).priv_0).lock);
    ret = g_hash_table_get_values((*(*object).priv_0).map_name_to_iface);
    g_list_foreach(
        ret,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gpointer>, GFunc>(Some(
            g_object_ref as unsafe extern "C" fn(gpointer) -> gpointer,
        )),
        NULL_0,
    );
    g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_skeleton_flush(
    mut object: *mut GDBusObjectSkeleton,
) {
    let mut to_flush: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    g_mutex_lock(&raw mut (*(*object).priv_0).lock);
    to_flush = g_hash_table_get_values_as_ptr_array((*(*object).priv_0).map_name_to_iface);
    g_ptr_array_foreach(
        to_flush,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gpointer>, GFunc>(Some(
            g_object_ref as unsafe extern "C" fn(gpointer) -> gpointer,
        )),
        NULL_0,
    );
    g_ptr_array_set_free_func(
        to_flush,
        Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
    );
    g_mutex_unlock(&raw mut (*(*object).priv_0).lock);
    let mut i: guint = 0 as guint;
    while i < (*to_flush).len {
        g_dbus_interface_skeleton_flush(
            *(*to_flush).pdata.offset(i as isize) as *mut GDBusInterfaceSkeleton
        );
        i = i.wrapping_add(1);
    }
    let mut _pp: *mut *mut GPtrArray = &raw mut to_flush;
    let mut _ptr: *mut GPtrArray = *_pp;
    *_pp = ::core::ptr::null_mut::<GPtrArray>();
    if !_ptr.is_null() {
        g_ptr_array_unref(_ptr as *mut GPtrArray);
    }
}
unsafe extern "C" fn safe_c2rust_dbus_object_interface_init(mut iface: *mut GDBusObjectIface) {
    (*iface).get_object_path = Some(
        safe_c2rust_g_dbus_object_skeleton_get_object_path
            as unsafe extern "C" fn(*mut GDBusObject) -> *const gchar,
    )
        as Option<unsafe extern "C" fn(*mut GDBusObject) -> *const gchar>;
    (*iface).get_interfaces = Some(
        safe_c2rust_g_dbus_object_skeleton_get_interfaces
            as unsafe extern "C" fn(*mut GDBusObject) -> *mut GList,
    ) as Option<unsafe extern "C" fn(*mut GDBusObject) -> *mut GList>;
    (*iface).get_interface = Some(
        safe_c2rust_g_dbus_object_skeleton_get_interface
            as unsafe extern "C" fn(*mut GDBusObject, *const gchar) -> *mut GDBusInterface,
    )
        as Option<unsafe extern "C" fn(*mut GDBusObject, *const gchar) -> *mut GDBusInterface>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_object_skeleton_has_authorize_method_handlers(
    mut object: *mut GDBusObjectSkeleton,
) -> gboolean {
    let mut has_handlers: gboolean = 0;
    let mut has_default_class_handler: gboolean = 0;
    has_handlers = g_signal_has_handler_pending(
        object as gpointer,
        safe_c2rust_signals[AUTHORIZE_METHOD_SIGNAL as ::core::ffi::c_int as usize],
        0 as GQuark,
        TRUE,
    );
    has_default_class_handler = ((*((*(object as *mut GTypeInstance)).g_class
        as *mut GDBusObjectSkeletonClass))
        .authorize_method
        == Some(
            safe_c2rust_g_dbus_object_skeleton_authorize_method_default
                as unsafe extern "C" fn(
                    *mut GDBusObjectSkeleton,
                    *mut GDBusInterfaceSkeleton,
                    *mut GDBusMethodInvocation,
                ) -> gboolean,
        )) as ::core::ffi::c_int as gboolean;
    return (has_handlers != 0 || has_default_class_handler == 0) as ::core::ffi::c_int;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
