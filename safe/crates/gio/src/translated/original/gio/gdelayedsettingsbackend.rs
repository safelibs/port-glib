extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GTree;
    pub type _GSettingsBackendPrivate;
    pub type _GPermission;
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
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_main_context_invoke(context: *mut GMainContext, function: GSourceFunc, data: gpointer);
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_tree_unref(tree: *mut GTree);
    fn g_tree_insert(tree: *mut GTree, key: gpointer, value: gpointer);
    fn g_tree_remove(tree: *mut GTree, key: gconstpointer) -> gboolean;
    fn g_tree_lookup(tree: *mut GTree, key: gconstpointer) -> gpointer;
    fn g_tree_lookup_extended(
        tree: *mut GTree,
        lookup_key: gconstpointer,
        orig_key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_tree_foreach(tree: *mut GTree, func: GTraverseFunc, user_data: gpointer);
    fn g_tree_nnodes(tree: *mut GTree) -> gint;
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_weak_ref(object: *mut GObject, notify: GWeakNotify, data: gpointer);
    fn g_settings_backend_get_type() -> GType;
    fn g_settings_backend_changed(
        backend: *mut GSettingsBackend,
        key: *const gchar,
        origin_tag: gpointer,
    );
    fn g_settings_backend_path_changed(
        backend: *mut GSettingsBackend,
        path: *const gchar,
        origin_tag: gpointer,
    );
    fn g_settings_backend_keys_changed(
        backend: *mut GSettingsBackend,
        path: *const gchar,
        items: *const *const gchar,
        origin_tag: gpointer,
    );
    fn g_settings_backend_path_writable_changed(backend: *mut GSettingsBackend, path: *const gchar);
    fn g_settings_backend_writable_changed(backend: *mut GSettingsBackend, key: *const gchar);
    fn g_settings_backend_changed_tree(
        backend: *mut GSettingsBackend,
        tree: *mut GTree,
        origin_tag: gpointer,
    );
    fn g_settings_backend_watch(
        backend: *mut GSettingsBackend,
        vtable: *const GSettingsListenerVTable,
        target: *mut GObject,
        context: *mut GMainContext,
    );
    fn g_settings_backend_create_tree() -> *mut GTree;
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
    fn g_settings_backend_write_tree(
        backend: *mut GSettingsBackend,
        tree: *mut GTree,
        origin_tag: gpointer,
    ) -> gboolean;
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
    fn g_settings_backend_get_permission(
        backend: *mut GSettingsBackend,
        path: *const gchar,
    ) -> *mut GPermission;
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
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
pub type GTree = _GTree;
pub type GTraverseFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>;
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
pub type GWeakNotify = Option<unsafe extern "C" fn(gpointer, *mut GObject) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsBackend {
    pub parent_instance: GObject,
    pub priv_0: *mut GSettingsBackendPrivate,
}
pub type GSettingsBackendPrivate = _GSettingsBackendPrivate;
pub type GSettingsBackend = _GSettingsBackend;
pub type GPermission = _GPermission;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsBackendClass {
    pub parent_class: GObjectClass,
    pub read: Option<
        unsafe extern "C" fn(
            *mut GSettingsBackend,
            *const gchar,
            *const GVariantType,
            gboolean,
        ) -> *mut GVariant,
    >,
    pub get_writable: Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> gboolean>,
    pub write: Option<
        unsafe extern "C" fn(
            *mut GSettingsBackend,
            *const gchar,
            *mut GVariant,
            gpointer,
        ) -> gboolean,
    >,
    pub write_tree:
        Option<unsafe extern "C" fn(*mut GSettingsBackend, *mut GTree, gpointer) -> gboolean>,
    pub reset: Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar, gpointer) -> ()>,
    pub subscribe: Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> ()>,
    pub unsubscribe: Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> ()>,
    pub sync: Option<unsafe extern "C" fn(*mut GSettingsBackend) -> ()>,
    pub get_permission:
        Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> *mut GPermission>,
    pub read_user_value: Option<
        unsafe extern "C" fn(
            *mut GSettingsBackend,
            *const gchar,
            *const GVariantType,
        ) -> *mut GVariant,
    >,
    pub padding: [gpointer; 23],
}
pub type GSettingsBackendClass = _GSettingsBackendClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDelayedSettingsBackendPrivate {
    pub backend: *mut GSettingsBackend,
    pub lock: GMutex,
    pub delayed: *mut GTree,
    pub owner_context: *mut GMainContext,
    pub owner: gpointer,
}
pub type GDelayedSettingsBackendPrivate = _GDelayedSettingsBackendPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDelayedSettingsBackendClass {
    pub parent_class: GSettingsBackendClass,
}
pub type GDelayedSettingsBackendClass = _GDelayedSettingsBackendClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDelayedSettingsBackend {
    pub parent_instance: GSettingsBackend,
    pub priv_0: *mut GDelayedSettingsBackendPrivate,
}
pub type GDelayedSettingsBackend = _GDelayedSettingsBackend;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CheckPrefixState {
    pub path: *const gchar,
    pub keys: *mut *const gchar,
    pub index: gsize,
}
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_delayed_settings_backend_get_type_once();
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
static mut safe_c2rust_g_delayed_settings_backend_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_delayed_settings_backend_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDelayedSettingsBackend_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GDelayedSettingsBackend_private_offset,
        );
    }
    safe_c2rust_g_delayed_settings_backend_class_init(klass as *mut GDelayedSettingsBackendClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_settings_backend_get_type(),
        g_intern_static_string(b"GDelayedSettingsBackend\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDelayedSettingsBackendClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_delayed_settings_backend_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDelayedSettingsBackend>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDelayedSettingsBackend) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_delayed_settings_backend_init
                    as unsafe extern "C" fn(*mut GDelayedSettingsBackend) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GDelayedSettingsBackend_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GDelayedSettingsBackendPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_get_instance_private(
    mut self_0: *mut GDelayedSettingsBackend,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GDelayedSettingsBackend_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GDelayedSettingsBackend_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_invoke_notify_unapplied(mut data: gpointer) -> gboolean {
    g_object_notify(
        data as *mut GObject,
        b"has-unapplied\0" as *const u8 as *const gchar,
    );
    g_object_unref(data);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_notify_unapplied(
    mut delayed: *mut GDelayedSettingsBackend,
) {
    let mut target_context: *mut GMainContext = ::core::ptr::null_mut::<GMainContext>();
    let mut target: *mut GObject = ::core::ptr::null_mut::<GObject>();
    g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
    if !(*(*delayed).priv_0).owner.is_null() {
        target_context = (*(*delayed).priv_0).owner_context;
        target = g_object_ref((*(*delayed).priv_0).owner) as *mut GObject;
    } else {
        target_context = ::core::ptr::null_mut::<GMainContext>();
        target = ::core::ptr::null_mut::<GObject>();
    }
    g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
    if !target.is_null() {
        g_main_context_invoke(
            target_context,
            Some(safe_c2rust_invoke_notify_unapplied as unsafe extern "C" fn(gpointer) -> gboolean),
            target as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_read(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut expected_type: *const GVariantType,
    mut default_value: gboolean,
) -> *mut GVariant {
    let mut delayed: *mut GDelayedSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    let mut result: gpointer = NULL_0;
    if default_value == 0 {
        g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
        if g_tree_lookup_extended(
            (*(*delayed).priv_0).delayed,
            key as gconstpointer,
            ::core::ptr::null_mut::<gpointer>(),
            &raw mut result,
        ) != 0
        {
            if !result.is_null() {
                g_variant_ref(result as *mut GVariant);
            } else {
                default_value = TRUE as gboolean;
            }
        }
        g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
    }
    if result.is_null() {
        result = g_settings_backend_read(
            (*(*delayed).priv_0).backend,
            key,
            expected_type,
            default_value,
        ) as gpointer;
    }
    return result as *mut GVariant;
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_read_user_value(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    let mut delayed: *mut GDelayedSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    let mut value_found: gboolean = FALSE;
    let mut result: gpointer = NULL_0;
    g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
    value_found = g_tree_lookup_extended(
        (*(*delayed).priv_0).delayed,
        key as gconstpointer,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut result,
    );
    if !result.is_null() {
        g_variant_ref(result as *mut GVariant);
    }
    g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
    if value_found != 0 {
        return result as *mut GVariant;
    }
    return g_settings_backend_read_user_value((*(*delayed).priv_0).backend, key, expected_type);
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_write(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut value: *mut GVariant,
    mut origin_tag: gpointer,
) -> gboolean {
    let mut delayed: *mut GDelayedSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    let mut was_empty: gboolean = 0;
    g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
    was_empty = (g_tree_nnodes((*(*delayed).priv_0).delayed) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as gboolean;
    g_tree_insert(
        (*(*delayed).priv_0).delayed,
        safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
        g_variant_ref_sink(value) as gpointer,
    );
    g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
    g_settings_backend_changed(backend, key, origin_tag);
    if was_empty != 0 {
        safe_c2rust_g_delayed_settings_backend_notify_unapplied(delayed);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_add_to_tree(
    mut key: gpointer,
    mut value: gpointer,
    mut user_data: gpointer,
) -> gboolean {
    g_tree_insert(
        user_data as *mut GTree,
        safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
        (if !value.is_null() {
            g_variant_ref(value as *mut GVariant)
        } else {
            ::core::ptr::null_mut::<GVariant>()
        }) as gpointer,
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_write_tree(
    mut backend: *mut GSettingsBackend,
    mut tree: *mut GTree,
    mut origin_tag: gpointer,
) -> gboolean {
    let mut delayed: *mut GDelayedSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    let mut was_empty: gboolean = 0;
    g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
    was_empty = (g_tree_nnodes((*(*delayed).priv_0).delayed) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as gboolean;
    g_tree_foreach(
        tree,
        Some(
            safe_c2rust_add_to_tree
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
        ),
        (*(*delayed).priv_0).delayed as gpointer,
    );
    g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
    g_settings_backend_changed_tree(backend, tree, origin_tag);
    if was_empty != 0 {
        safe_c2rust_g_delayed_settings_backend_notify_unapplied(delayed);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_get_writable(
    mut backend: *mut GSettingsBackend,
    mut name: *const gchar,
) -> gboolean {
    let mut delayed: *mut GDelayedSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    return g_settings_backend_get_writable(
        (*(*delayed).priv_0).backend,
        name as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_reset(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut origin_tag: gpointer,
) {
    let mut delayed: *mut GDelayedSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    let mut was_empty: gboolean = 0;
    g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
    was_empty = (g_tree_nnodes((*(*delayed).priv_0).delayed) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as gboolean;
    g_tree_insert(
        (*(*delayed).priv_0).delayed,
        safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
        NULL_0,
    );
    g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
    g_settings_backend_changed(backend, key, origin_tag);
    if was_empty != 0 {
        safe_c2rust_g_delayed_settings_backend_notify_unapplied(delayed);
    }
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_subscribe(
    mut backend: *mut GSettingsBackend,
    mut name: *const ::core::ffi::c_char,
) {
    let mut delayed: *mut GDelayedSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    g_settings_backend_subscribe((*(*delayed).priv_0).backend, name);
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_unsubscribe(
    mut backend: *mut GSettingsBackend,
    mut name: *const ::core::ffi::c_char,
) {
    let mut delayed: *mut GDelayedSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    g_settings_backend_unsubscribe((*(*delayed).priv_0).backend, name);
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_get_permission(
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
) -> *mut GPermission {
    let mut delayed: *mut GDelayedSettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    return g_settings_backend_get_permission((*(*delayed).priv_0).backend, path);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_get_has_unapplied(
    mut delayed: *mut GDelayedSettingsBackend,
) -> gboolean {
    return (g_tree_nnodes((*(*delayed).priv_0).delayed) > 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_apply(
    mut delayed: *mut GDelayedSettingsBackend,
) {
    if g_tree_nnodes((*(*delayed).priv_0).delayed) > 0 as ::core::ffi::c_int {
        let mut success: gboolean = 0;
        let mut tmp: *mut GTree = ::core::ptr::null_mut::<GTree>();
        g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
        tmp = (*(*delayed).priv_0).delayed;
        (*(*delayed).priv_0).delayed = g_settings_backend_create_tree();
        success = g_settings_backend_write_tree(
            (*(*delayed).priv_0).backend,
            tmp,
            (*delayed).priv_0 as gpointer,
        );
        g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
        if success == 0 {
            g_settings_backend_changed_tree(
                delayed as *mut ::core::ffi::c_void as *mut GSettingsBackend,
                tmp,
                NULL_0,
            );
        }
        g_tree_unref(tmp);
        safe_c2rust_g_delayed_settings_backend_notify_unapplied(delayed);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_revert(
    mut delayed: *mut GDelayedSettingsBackend,
) {
    if g_tree_nnodes((*(*delayed).priv_0).delayed) > 0 as ::core::ffi::c_int {
        let mut tmp: *mut GTree = ::core::ptr::null_mut::<GTree>();
        g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
        tmp = (*(*delayed).priv_0).delayed;
        (*(*delayed).priv_0).delayed = g_settings_backend_create_tree();
        g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
        g_settings_backend_changed_tree(
            delayed as *mut ::core::ffi::c_void as *mut GSettingsBackend,
            tmp,
            NULL_0,
        );
        g_tree_unref(tmp);
        safe_c2rust_g_delayed_settings_backend_notify_unapplied(delayed);
    }
}
unsafe extern "C" fn safe_c2rust_delayed_backend_changed(
    mut target: *mut GObject,
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut origin_tag: gpointer,
) {
    let mut delayed: *mut GDelayedSettingsBackend =
        target as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    if origin_tag != (*delayed).priv_0 as gpointer {
        g_settings_backend_changed(
            delayed as *mut ::core::ffi::c_void as *mut GSettingsBackend,
            key,
            origin_tag,
        );
    }
}
unsafe extern "C" fn safe_c2rust_delayed_backend_keys_changed(
    mut target: *mut GObject,
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
    mut origin_tag: gpointer,
    mut items: *const *const gchar,
) {
    let mut delayed: *mut GDelayedSettingsBackend =
        target as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    if origin_tag != (*delayed).priv_0 as gpointer {
        g_settings_backend_keys_changed(
            delayed as *mut ::core::ffi::c_void as *mut GSettingsBackend,
            path,
            items,
            origin_tag,
        );
    }
}
unsafe extern "C" fn safe_c2rust_delayed_backend_path_changed(
    mut target: *mut GObject,
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
    mut origin_tag: gpointer,
) {
    let mut delayed: *mut GDelayedSettingsBackend =
        target as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    if origin_tag != (*delayed).priv_0 as gpointer {
        g_settings_backend_path_changed(
            delayed as *mut ::core::ffi::c_void as *mut GSettingsBackend,
            path,
            origin_tag,
        );
    }
}
unsafe extern "C" fn safe_c2rust_delayed_backend_writable_changed(
    mut target: *mut GObject,
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
) {
    let mut delayed: *mut GDelayedSettingsBackend =
        target as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    let mut last_one: gboolean = FALSE;
    g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
    if !g_tree_lookup((*(*delayed).priv_0).delayed, key as gconstpointer).is_null()
        && g_settings_backend_get_writable(
            (*(*delayed).priv_0).backend,
            key as *const ::core::ffi::c_char,
        ) == 0
    {
        g_tree_remove((*(*delayed).priv_0).delayed, key as gconstpointer);
        last_one = (g_tree_nnodes((*(*delayed).priv_0).delayed) == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as gboolean;
    }
    g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
    if last_one != 0 {
        safe_c2rust_g_delayed_settings_backend_notify_unapplied(delayed);
    }
    g_settings_backend_writable_changed(
        delayed as *mut ::core::ffi::c_void as *mut GSettingsBackend,
        key,
    );
}
unsafe extern "C" fn safe_c2rust_check_prefix(
    mut key: gpointer,
    mut value: gpointer,
    mut data: gpointer,
) -> gboolean {
    let mut state: *mut CheckPrefixState = data as *mut CheckPrefixState;
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = key as *const ::core::ffi::c_char;
            let __prefix: *const ::core::ffi::c_char = (*state).path as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_10
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
        g_str_has_prefix(key as *const gchar, (*state).path)
    } != 0
    {
        let fresh0 = (*state).index;
        (*state).index = (*state).index.wrapping_add(1);
        let ref mut fresh1 = *(*state).keys.offset(fresh0 as isize);
        *fresh1 = key as *const gchar;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_delayed_backend_path_writable_changed(
    mut target: *mut GObject,
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
) {
    let mut delayed: *mut GDelayedSettingsBackend =
        target as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    let mut last_one: gboolean = FALSE;
    let mut n_keys: gsize = 0;
    g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
    n_keys = g_tree_nnodes((*(*delayed).priv_0).delayed) as gsize;
    if n_keys > 0 as gsize {
        let mut state: CheckPrefixState = CheckPrefixState {
            path: path,
            keys: ({
                let mut __n: gsize = n_keys;
                let mut __s: gsize = ::core::mem::size_of::<*const gchar>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc_n(__n, __s);
                }
                __p
            }) as *mut *const gchar,
            index: 0 as gsize,
        };
        let mut i: gsize = 0;
        g_tree_foreach(
            (*(*delayed).priv_0).delayed,
            Some(
                safe_c2rust_check_prefix
                    as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
            ),
            &raw mut state as gpointer,
        );
        i = 0 as gsize;
        while i < state.index {
            if !g_tree_lookup(
                (*(*delayed).priv_0).delayed,
                *state.keys.offset(i as isize) as gconstpointer,
            )
            .is_null()
                && g_settings_backend_get_writable(
                    (*(*delayed).priv_0).backend,
                    *state.keys.offset(i as isize) as *const ::core::ffi::c_char,
                ) == 0
            {
                g_tree_remove(
                    (*(*delayed).priv_0).delayed,
                    *state.keys.offset(i as isize) as gconstpointer,
                );
            }
            i = i.wrapping_add(1);
        }
        g_free(state.keys as gpointer);
        last_one = (g_tree_nnodes((*(*delayed).priv_0).delayed) == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as gboolean;
    }
    g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
    if last_one != 0 {
        safe_c2rust_g_delayed_settings_backend_notify_unapplied(delayed);
    }
    g_settings_backend_path_writable_changed(
        delayed as *mut ::core::ffi::c_void as *mut GSettingsBackend,
        path,
    );
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_finalize(mut object: *mut GObject) {
    let mut delayed: *mut GDelayedSettingsBackend =
        object as *mut ::core::ffi::c_void as *mut GDelayedSettingsBackend;
    g_mutex_clear(&raw mut (*(*delayed).priv_0).lock);
    g_object_unref((*(*delayed).priv_0).backend as gpointer);
    g_tree_unref((*(*delayed).priv_0).delayed);
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if (*(*delayed).priv_0).owner.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdelayedsettingsbackend.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            448 as ::core::ffi::c_int,
            G_STRFUNC,
            b"delayed->priv->owner == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(safe_c2rust_g_delayed_settings_backend_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_class_init(
    mut class: *mut GDelayedSettingsBackendClass,
) {
    let mut backend_class: *mut GSettingsBackendClass =
        class as *mut ::core::ffi::c_void as *mut GSettingsBackendClass;
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*backend_class).read = Some(
        safe_c2rust_g_delayed_settings_backend_read
            as unsafe extern "C" fn(
                *mut GSettingsBackend,
                *const gchar,
                *const GVariantType,
                gboolean,
            ) -> *mut GVariant,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSettingsBackend,
                *const gchar,
                *const GVariantType,
                gboolean,
            ) -> *mut GVariant,
        >;
    (*backend_class).read_user_value = Some(
        safe_c2rust_g_delayed_settings_backend_read_user_value
            as unsafe extern "C" fn(
                *mut GSettingsBackend,
                *const gchar,
                *const GVariantType,
            ) -> *mut GVariant,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSettingsBackend,
                *const gchar,
                *const GVariantType,
            ) -> *mut GVariant,
        >;
    (*backend_class).write = Some(
        safe_c2rust_g_delayed_settings_backend_write
            as unsafe extern "C" fn(
                *mut GSettingsBackend,
                *const gchar,
                *mut GVariant,
                gpointer,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GSettingsBackend,
                *const gchar,
                *mut GVariant,
                gpointer,
            ) -> gboolean,
        >;
    (*backend_class).write_tree = Some(
        safe_c2rust_g_delayed_settings_backend_write_tree
            as unsafe extern "C" fn(*mut GSettingsBackend, *mut GTree, gpointer) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *mut GTree, gpointer) -> gboolean>;
    (*backend_class).reset = Some(
        safe_c2rust_g_delayed_settings_backend_reset
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar, gpointer) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar, gpointer) -> ()>;
    (*backend_class).get_writable = Some(
        safe_c2rust_g_delayed_settings_backend_get_writable
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> gboolean>;
    (*backend_class).subscribe = Some(
        safe_c2rust_g_delayed_settings_backend_subscribe
            as unsafe extern "C" fn(*mut GSettingsBackend, *const ::core::ffi::c_char) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> ()>;
    (*backend_class).unsubscribe = Some(
        safe_c2rust_g_delayed_settings_backend_unsubscribe
            as unsafe extern "C" fn(*mut GSettingsBackend, *const ::core::ffi::c_char) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> ()>;
    (*backend_class).get_permission = Some(
        safe_c2rust_g_delayed_settings_backend_get_permission
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> *mut GPermission,
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> *mut GPermission>;
    (*object_class).finalize = Some(
        safe_c2rust_g_delayed_settings_backend_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_init(
    mut delayed: *mut GDelayedSettingsBackend,
) {
    (*delayed).priv_0 = safe_c2rust_g_delayed_settings_backend_get_instance_private(delayed)
        as *mut GDelayedSettingsBackendPrivate;
    (*(*delayed).priv_0).delayed = g_settings_backend_create_tree();
    g_mutex_init(&raw mut (*(*delayed).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_disown(
    mut data: gpointer,
    mut where_the_object_was: *mut GObject,
) {
    let mut delayed: *mut GDelayedSettingsBackend = data as *mut GDelayedSettingsBackend;
    g_mutex_lock(&raw mut (*(*delayed).priv_0).lock);
    (*(*delayed).priv_0).owner_context = ::core::ptr::null_mut::<GMainContext>();
    (*(*delayed).priv_0).owner = NULL_0 as gpointer;
    g_mutex_unlock(&raw mut (*(*delayed).priv_0).lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_delayed_settings_backend_new(
    mut backend: *mut GSettingsBackend,
    mut owner: gpointer,
    mut owner_context: *mut GMainContext,
) -> *mut GDelayedSettingsBackend {
    static mut safe_c2rust_vtable: GSettingsListenerVTable = unsafe {
        GSettingsListenerVTable {
            changed: Some(
                safe_c2rust_delayed_backend_changed
                    as unsafe extern "C" fn(
                        *mut GObject,
                        *mut GSettingsBackend,
                        *const gchar,
                        gpointer,
                    ) -> (),
            ),
            path_changed: Some(
                safe_c2rust_delayed_backend_path_changed
                    as unsafe extern "C" fn(
                        *mut GObject,
                        *mut GSettingsBackend,
                        *const gchar,
                        gpointer,
                    ) -> (),
            ),
            keys_changed: Some(
                safe_c2rust_delayed_backend_keys_changed
                    as unsafe extern "C" fn(
                        *mut GObject,
                        *mut GSettingsBackend,
                        *const gchar,
                        gpointer,
                        *const *const gchar,
                    ) -> (),
            ),
            writable_changed: Some(
                safe_c2rust_delayed_backend_writable_changed
                    as unsafe extern "C" fn(
                        *mut GObject,
                        *mut GSettingsBackend,
                        *const gchar,
                    ) -> (),
            ),
            path_writable_changed: Some(
                safe_c2rust_delayed_backend_path_writable_changed
                    as unsafe extern "C" fn(
                        *mut GObject,
                        *mut GSettingsBackend,
                        *const gchar,
                    ) -> (),
            ),
        }
    };
    let mut delayed: *mut GDelayedSettingsBackend =
        ::core::ptr::null_mut::<GDelayedSettingsBackend>();
    delayed = g_object_new(
        safe_c2rust_g_delayed_settings_backend_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GDelayedSettingsBackend;
    (*(*delayed).priv_0).backend =
        g_object_ref(backend as gpointer) as *mut GSettingsBackend as *mut GSettingsBackend;
    (*(*delayed).priv_0).owner_context = owner_context;
    (*(*delayed).priv_0).owner = owner;
    g_object_weak_ref(
        owner as *mut GObject,
        Some(
            safe_c2rust_g_delayed_settings_backend_disown
                as unsafe extern "C" fn(gpointer, *mut GObject) -> (),
        ),
        delayed as gpointer,
    );
    g_settings_backend_watch(
        (*(*delayed).priv_0).backend,
        &raw mut safe_c2rust_vtable,
        delayed as *mut ::core::ffi::c_void as *mut GObject,
        ::core::ptr::null_mut::<GMainContext>(),
    );
    return delayed;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
