extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GDBusConnection;
    pub type _GDBusProxyPrivate;
    pub type _GDBusInterface;
    pub type _GDBusObject;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    fn g_signal_emit_by_name(instance: gpointer, detailed_signal: *const gchar, ...);
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    fn g_value_dup_object(value: *const GValue) -> gpointer;
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
    fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    fn g_value_dup_string(value: *const GValue) -> *mut gchar;
    fn g_dbus_object_get_type() -> GType;
    fn g_dbus_connection_get_type() -> GType;
    fn g_dbus_is_interface_name(string: *const gchar) -> gboolean;
    fn g_dbus_proxy_get_type() -> GType;
    fn g_dbus_proxy_get_interface_name(proxy: *mut GDBusProxy) -> *const gchar;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
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
pub type GDBusConnection = _GDBusConnection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusProxy {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusProxyPrivate,
}
pub type GDBusProxyPrivate = _GDBusProxyPrivate;
pub type GDBusProxy = _GDBusProxy;
pub type GDBusInterface = _GDBusInterface;
pub type GDBusObject = _GDBusObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectProxy {
    pub parent_instance: GObject,
    pub priv_0: *mut GDBusObjectProxyPrivate,
}
pub type GDBusObjectProxyPrivate = _GDBusObjectProxyPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusObjectProxyPrivate {
    pub lock: GMutex,
    pub map_name_to_iface: *mut GHashTable,
    pub object_path: *mut gchar,
    pub connection: *mut GDBusConnection,
}
pub type GDBusObjectProxy = _GDBusObjectProxy;
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
pub struct _GDBusObjectProxyClass {
    pub parent_class: GObjectClass,
    pub padding: [gpointer; 8],
}
pub type GDBusObjectProxyClass = _GDBusObjectProxyClass;
pub const PROP_G_CONNECTION: C2RustUnnamed_0 = 2;
pub const PROP_G_OBJECT_PATH: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_0 = 0;
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
#[inline]
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_get_instance_private(
    mut self_0: *mut GDBusObjectProxy,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDBusObjectProxy_private_offset as glong as isize)
        as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_object_proxy_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_object_proxy_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusObjectProxy_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDBusObjectProxy_private_offset,
        );
    }
    safe_c2rust_g_dbus_object_proxy_class_init(klass as *mut GDBusObjectProxyClass);
}
static mut safe_c2rust_GDBusObjectProxy_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusObjectProxy\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusObjectProxyClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_object_proxy_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusObjectProxy>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusObjectProxy) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_object_proxy_init
                    as unsafe extern "C" fn(*mut GDBusObjectProxy) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDBusObjectProxy_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDBusObjectProxyPrivate>() as gsize,
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
static mut safe_c2rust_g_dbus_object_proxy_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_finalize(mut object: *mut GObject) {
    let mut proxy: *mut GDBusObjectProxy =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectProxy;
    g_hash_table_unref((*(*proxy).priv_0).map_name_to_iface);
    let mut _pp: *mut *mut GDBusConnection = &raw mut (*(*proxy).priv_0).connection;
    let mut _ptr: *mut GDBusConnection = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusConnection>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_free((*(*proxy).priv_0).object_path as gpointer);
    g_mutex_clear(&raw mut (*(*proxy).priv_0).lock);
    if (*(safe_c2rust_g_dbus_object_proxy_parent_class as *mut GObjectClass))
        .finalize
        .is_some()
    {
        (*(safe_c2rust_g_dbus_object_proxy_parent_class as *mut GObjectClass))
            .finalize
            .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut proxy: *mut GDBusObjectProxy =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectProxy;
    match prop_id {
        1 => {
            g_mutex_lock(&raw mut (*(*proxy).priv_0).lock);
            g_value_set_string(value, (*(*proxy).priv_0).object_path);
            g_mutex_unlock(&raw mut (*(*proxy).priv_0).lock);
        }
        2 => {
            g_value_set_object(
                value,
                safe_c2rust_g_dbus_object_proxy_get_connection(proxy) as gpointer,
            );
        }
        _ => {
            let mut _glib__object: *mut GObject = proxy as *mut GObject;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectproxy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_set_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *const GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut proxy: *mut GDBusObjectProxy =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectProxy;
    match prop_id {
        1 => {
            g_mutex_lock(&raw mut (*(*proxy).priv_0).lock);
            (*(*proxy).priv_0).object_path = g_value_dup_string(value);
            g_mutex_unlock(&raw mut (*(*proxy).priv_0).lock);
        }
        2 => {
            g_mutex_lock(&raw mut (*(*proxy).priv_0).lock);
            (*(*proxy).priv_0).connection = g_value_dup_object(value) as *mut GDBusConnection;
            g_mutex_unlock(&raw mut (*(*proxy).priv_0).lock);
        }
        _ => {
            let mut _glib__object: *mut GObject = proxy as *mut GObject;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectproxy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                132 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_class_init(
    mut klass: *mut GDBusObjectProxyClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_dbus_object_proxy_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).set_property = Some(
        safe_c2rust_g_dbus_object_proxy_set_property
            as unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *const GValue, *mut GParamSpec) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dbus_object_proxy_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_G_OBJECT_PATH as ::core::ffi::c_int as guint,
        g_param_spec_string(
            b"g-object-path\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
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
            (G_PARAM_READWRITE as ::core::ffi::c_int
                | G_PARAM_CONSTRUCT_ONLY as ::core::ffi::c_int
                | (G_PARAM_STATIC_NAME as ::core::ffi::c_int
                    | G_PARAM_STATIC_NICK as ::core::ffi::c_int
                    | G_PARAM_STATIC_BLURB as ::core::ffi::c_int)) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_init(mut proxy: *mut GDBusObjectProxy) {
    (*proxy).priv_0 =
        safe_c2rust_g_dbus_object_proxy_get_instance_private(proxy) as *mut GDBusObjectProxyPrivate;
    g_mutex_init(&raw mut (*(*proxy).priv_0).lock);
    (*(*proxy).priv_0).map_name_to_iface = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
            Some(g_object_unref as unsafe extern "C" fn(gpointer) -> ()),
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_get_object_path(
    mut object: *mut GDBusObject,
) -> *const gchar {
    let mut proxy: *mut GDBusObjectProxy =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectProxy;
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    g_mutex_lock(&raw mut (*(*proxy).priv_0).lock);
    ret = (*(*proxy).priv_0).object_path;
    g_mutex_unlock(&raw mut (*(*proxy).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_get_connection(
    mut proxy: *mut GDBusObjectProxy,
) -> *mut GDBusConnection {
    let mut ret: *mut GDBusConnection = ::core::ptr::null_mut::<GDBusConnection>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_proxy_get_type();
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
            b"G_IS_DBUS_OBJECT_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusConnection>();
    }
    g_mutex_lock(&raw mut (*(*proxy).priv_0).lock);
    ret = (*(*proxy).priv_0).connection;
    g_mutex_unlock(&raw mut (*(*proxy).priv_0).lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_get_interface(
    mut object: *mut GDBusObject,
    mut interface_name: *const gchar,
) -> *mut GDBusInterface {
    let mut proxy: *mut GDBusObjectProxy =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectProxy;
    let mut ret: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_proxy_get_type();
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
            b"G_IS_DBUS_OBJECT_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusInterface>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if g_dbus_is_interface_name(interface_name) != 0 {
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
            b"g_dbus_is_interface_name (interface_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusInterface>();
    }
    g_mutex_lock(&raw mut (*(*proxy).priv_0).lock);
    ret = g_hash_table_lookup(
        (*(*proxy).priv_0).map_name_to_iface,
        interface_name as gconstpointer,
    ) as *mut GDBusProxy;
    if !ret.is_null() {
        g_object_ref(ret as gpointer);
    }
    g_mutex_unlock(&raw mut (*(*proxy).priv_0).lock);
    return ret as *mut GDBusInterface;
}
unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_get_interfaces(
    mut object: *mut GDBusObject,
) -> *mut GList {
    let mut proxy: *mut GDBusObjectProxy =
        object as *mut ::core::ffi::c_void as *mut GDBusObjectProxy;
    let mut ret: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_proxy_get_type();
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
            b"G_IS_DBUS_OBJECT_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    ret = ::core::ptr::null_mut::<GList>();
    g_mutex_lock(&raw mut (*(*proxy).priv_0).lock);
    ret = g_hash_table_get_values((*(*proxy).priv_0).map_name_to_iface);
    g_list_foreach(
        ret,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> gpointer>, GFunc>(Some(
            g_object_ref as unsafe extern "C" fn(gpointer) -> gpointer,
        )),
        NULL_0,
    );
    g_mutex_unlock(&raw mut (*(*proxy).priv_0).lock);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_object_proxy_new(
    mut connection: *mut GDBusConnection,
    mut object_path: *const gchar,
) -> *mut GDBusObjectProxy {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
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
            b"G_IS_DBUS_CONNECTION (connection)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectProxy>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(object_path) != 0 {
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
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusObjectProxy>();
    }
    return g_object_new(
        safe_c2rust_g_dbus_object_proxy_get_type(),
        b"g-object-path\0" as *const u8 as *const gchar,
        object_path,
        b"g-connection\0" as *const u8 as *const ::core::ffi::c_char,
        connection,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as *mut GDBusObjectProxy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_object_proxy_add_interface(
    mut proxy: *mut GDBusObjectProxy,
    mut interface_proxy: *mut GDBusProxy,
) {
    let mut interface_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut interface_proxy_to_remove: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_proxy_get_type();
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
            b"G_IS_DBUS_OBJECT_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = interface_proxy as *mut GTypeInstance;
            let mut __t: GType = g_dbus_proxy_get_type();
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
            b"G_IS_DBUS_PROXY (interface_proxy)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_mutex_lock(&raw mut (*(*proxy).priv_0).lock);
    interface_name = g_dbus_proxy_get_interface_name(interface_proxy);
    interface_proxy_to_remove = g_hash_table_lookup(
        (*(*proxy).priv_0).map_name_to_iface,
        interface_name as gconstpointer,
    ) as *mut GDBusProxy;
    if !interface_proxy_to_remove.is_null() {
        g_object_ref(interface_proxy_to_remove as gpointer);
        if !(({
            let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*(*proxy).priv_0).map_name_to_iface,
                interface_name as gconstpointer,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectproxy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (proxy->priv->map_name_to_iface, interface_name)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    g_hash_table_insert(
        (*(*proxy).priv_0).map_name_to_iface,
        safe_c2rust_g_strdup_inline(interface_name as *const ::core::ffi::c_char) as gpointer,
        g_object_ref(interface_proxy as gpointer) as *mut GDBusProxy as gpointer,
    );
    g_object_ref(interface_proxy as gpointer);
    g_mutex_unlock(&raw mut (*(*proxy).priv_0).lock);
    if !interface_proxy_to_remove.is_null() {
        g_signal_emit_by_name(
            proxy as gpointer,
            b"interface-removed\0" as *const u8 as *const gchar,
            interface_proxy_to_remove,
        );
        g_object_unref(interface_proxy_to_remove as gpointer);
    }
    g_signal_emit_by_name(
        proxy as gpointer,
        b"interface-added\0" as *const u8 as *const gchar,
        interface_proxy,
    );
    g_object_unref(interface_proxy as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_dbus_object_proxy_remove_interface(
    mut proxy: *mut GDBusObjectProxy,
    mut interface_name: *const gchar,
) {
    let mut interface_proxy: *mut GDBusProxy = ::core::ptr::null_mut::<GDBusProxy>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = proxy as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_object_proxy_get_type();
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
            b"G_IS_DBUS_OBJECT_PROXY (proxy)\0" as *const u8 as *const ::core::ffi::c_char,
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
    g_mutex_lock(&raw mut (*(*proxy).priv_0).lock);
    interface_proxy = g_hash_table_lookup(
        (*(*proxy).priv_0).map_name_to_iface,
        interface_name as gconstpointer,
    ) as *mut GDBusProxy;
    if !interface_proxy.is_null() {
        g_object_ref(interface_proxy as gpointer);
        if !(({
            let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
            if g_hash_table_remove(
                (*(*proxy).priv_0).map_name_to_iface,
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
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusobjectproxy.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                335 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_hash_table_remove (proxy->priv->map_name_to_iface, interface_name)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_mutex_unlock(&raw mut (*(*proxy).priv_0).lock);
        g_signal_emit_by_name(
            proxy as gpointer,
            b"interface-removed\0" as *const u8 as *const gchar,
            interface_proxy,
        );
        g_object_unref(interface_proxy as gpointer);
    } else {
        g_mutex_unlock(&raw mut (*(*proxy).priv_0).lock);
    };
}
unsafe extern "C" fn safe_c2rust_dbus_object_interface_init(mut iface: *mut GDBusObjectIface) {
    (*iface).get_object_path = Some(
        safe_c2rust_g_dbus_object_proxy_get_object_path
            as unsafe extern "C" fn(*mut GDBusObject) -> *const gchar,
    )
        as Option<unsafe extern "C" fn(*mut GDBusObject) -> *const gchar>;
    (*iface).get_interfaces = Some(
        safe_c2rust_g_dbus_object_proxy_get_interfaces
            as unsafe extern "C" fn(*mut GDBusObject) -> *mut GList,
    ) as Option<unsafe extern "C" fn(*mut GDBusObject) -> *mut GList>;
    (*iface).get_interface = Some(
        safe_c2rust_g_dbus_object_proxy_get_interface
            as unsafe extern "C" fn(*mut GDBusObject, *const gchar) -> *mut GDBusInterface,
    )
        as Option<unsafe extern "C" fn(*mut GDBusObject, *const gchar) -> *mut GDBusInterface>;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
