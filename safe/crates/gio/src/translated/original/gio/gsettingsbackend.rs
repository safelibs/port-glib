extern "C" {
    pub type _GData;
    pub type _GMainContext;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GTree;
    pub type _GPermission;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_mutex_init(mutex: *mut GMutex);
    fn g_mutex_clear(mutex: *mut GMutex);
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_delete_link(list: *mut GSList, link_: *mut GSList) -> *mut GSList;
    fn g_main_context_ref(context: *mut GMainContext) -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_context_invoke(context: *mut GMainContext, function: GSourceFunc, data: gpointer);
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_take_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_tree_new_full(
        key_compare_func: GCompareDataFunc,
        key_compare_data: gpointer,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GTree;
    fn g_tree_foreach(tree: *mut GTree, func: GTraverseFunc, user_data: gpointer);
    fn g_tree_nnodes(tree: *mut GTree) -> gint;
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
    fn g_type_add_instance_private(class_type: GType, private_size: gsize) -> gint;
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_object_weak_ref(object: *mut GObject, notify: GWeakNotify, data: gpointer);
    fn g_object_weak_unref(object: *mut GObject, notify: GWeakNotify, data: gpointer);
    fn g_weak_ref_init(weak_ref: *mut GWeakRef, object: gpointer);
    fn g_weak_ref_clear(weak_ref: *mut GWeakRef);
    fn g_weak_ref_get(weak_ref: *mut GWeakRef) -> gpointer;
    fn g_simple_permission_new(allowed: gboolean) -> *mut GPermission;
    fn _g_io_module_get_default(
        extension_point: *const gchar,
        envvar: *const gchar,
        verify_func: GIOModuleVerifyFunc,
    ) -> gpointer;
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
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
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
pub struct GWeakRef {
    pub priv_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub p: gpointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsBackend {
    pub parent_instance: GObject,
    pub priv_0: *mut GSettingsBackendPrivate,
}
pub type GSettingsBackendPrivate = _GSettingsBackendPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsBackendPrivate {
    pub watches: *mut GSettingsBackendWatch,
    pub lock: GMutex,
}
pub type GSettingsBackendWatch = _GSettingsBackendWatch;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsBackendWatch {
    pub target: GWeakRef,
    pub target_ptr: *mut GObject,
    pub vtable: *const GSettingsListenerVTable,
    pub context: *mut GMainContext,
    pub next: *mut GSettingsBackendWatch,
}
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
pub type GSettingsBackendClosure = _GSettingsBackendClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsBackendClosure {
    pub function: Option<
        unsafe extern "C" fn(
            *mut GObject,
            *mut GSettingsBackend,
            *const gchar,
            gpointer,
            *mut *mut gchar,
        ) -> (),
    >,
    pub context: *mut GMainContext,
    pub target: *mut GObject,
    pub backend: *mut GSettingsBackend,
    pub name: *mut gchar,
    pub origin_tag: gpointer,
    pub names: *mut *mut gchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FlattenState {
    pub keys: *mut *const gchar,
    pub values: *mut *mut GVariant,
    pub prefix_len: gint,
    pub prefix: *mut gchar,
}
pub type GIOModuleVerifyFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
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
pub const G_SETTINGS_BACKEND_EXTENSION_POINT_NAME: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"gsettings-backend\0")
};
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_settings_backend_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GSettingsBackend\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GSettingsBackendClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_settings_backend_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GSettingsBackend>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GSettingsBackend) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_settings_backend_init
                    as unsafe extern "C" fn(*mut GSettingsBackend) -> (),
            )),
        ),
        G_TYPE_FLAG_ABSTRACT,
    );
    safe_c2rust_GSettingsBackend_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GSettingsBackendPrivate>() as gsize,
    );
    return g_define_type_id;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_settings_backend_get_instance_private(
    mut self_0: *mut GSettingsBackend,
) -> gpointer {
    return (self_0 as *mut guint8)
        .offset(safe_c2rust_GSettingsBackend_private_offset as glong as isize)
        as gpointer;
}
static mut safe_c2rust_GSettingsBackend_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_settings_backend_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_settings_backend_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GSettingsBackend_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GSettingsBackend_private_offset,
        );
    }
    safe_c2rust_g_settings_backend_class_init(klass as *mut GSettingsBackendClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_settings_backend_get_type_once();
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
static mut safe_c2rust_g_settings_backend_parent_class: gpointer = NULL_0;
static mut safe_c2rust_g_settings_has_backend: gboolean = 0;
unsafe extern "C" fn safe_c2rust_is_key(mut key: *const gchar) -> gboolean {
    let mut length: gint = 0;
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if *key.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
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
            b"key[0] == '/'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    i = 1 as ::core::ffi::c_int as gint;
    while *key.offset(i as isize) != 0 {
        if ({
            let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
            if *key.offset(i as isize) as ::core::ffi::c_int != '/' as i32
                || *key.offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    != '/' as i32
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
                b"key[i] != '/' || key[i + 1] != '/'\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
        i += 1;
    }
    length = i;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if *key.offset((length as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            != '/' as i32
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
            b"key[length - 1] != '/'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_is_path(mut path: *const gchar) -> gboolean {
    let mut length: gint = 0;
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !path.is_null() {
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
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
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
            b"path[0] == '/'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    i = 1 as ::core::ffi::c_int as gint;
    while *path.offset(i as isize) != 0 {
        if ({
            let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
            if *path.offset(i as isize) as ::core::ffi::c_int != '/' as i32
                || *path.offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    != '/' as i32
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
                b"path[i] != '/' || path[i + 1] != '/'\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
        i += 1;
    }
    length = i;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if *path.offset((length as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            == '/' as i32
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
            b"path[length - 1] == '/'\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_settings_backend_watch_weak_notify(
    mut data: gpointer,
    mut where_the_object_was: *mut GObject,
) {
    let mut backend: *mut GSettingsBackend = data as *mut GSettingsBackend;
    let mut ptr: *mut *mut GSettingsBackendWatch =
        ::core::ptr::null_mut::<*mut GSettingsBackendWatch>();
    g_mutex_lock(&raw mut (*(*backend).priv_0).lock);
    ptr = &raw mut (*(*backend).priv_0).watches;
    while !(*ptr).is_null() {
        if (**ptr).target_ptr == where_the_object_was {
            let mut tmp: *mut GSettingsBackendWatch = *ptr;
            *ptr = (*tmp).next;
            g_weak_ref_clear(&raw mut (*tmp).target);
            g_slice_free1(
                ::core::mem::size_of::<GSettingsBackendWatch>() as gsize,
                tmp as gpointer,
            );
            g_mutex_unlock(&raw mut (*(*backend).priv_0).lock);
            return;
        }
        ptr = &raw mut (**ptr).next;
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gsettingsbackend.c\0"
            as *const u8 as *const ::core::ffi::c_char,
        173 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_watch(
    mut backend: *mut GSettingsBackend,
    mut vtable: *const GSettingsListenerVTable,
    mut target: *mut GObject,
    mut context: *mut GMainContext,
) {
    let mut watch: *mut GSettingsBackendWatch = ::core::ptr::null_mut::<GSettingsBackendWatch>();
    watch = g_slice_alloc(::core::mem::size_of::<GSettingsBackendWatch>() as gsize)
        as *mut GSettingsBackendWatch;
    (*watch).context = context;
    (*watch).vtable = vtable;
    g_weak_ref_init(&raw mut (*watch).target, target as gpointer);
    (*watch).target_ptr = target;
    g_object_weak_ref(
        target,
        Some(
            safe_c2rust_g_settings_backend_watch_weak_notify
                as unsafe extern "C" fn(gpointer, *mut GObject) -> (),
        ),
        backend as gpointer,
    );
    g_mutex_lock(&raw mut (*(*backend).priv_0).lock);
    (*watch).next = (*(*backend).priv_0).watches;
    (*(*backend).priv_0).watches = watch;
    g_mutex_unlock(&raw mut (*(*backend).priv_0).lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_unwatch(
    mut backend: *mut GSettingsBackend,
    mut target: *mut GObject,
) {
    g_object_weak_unref(
        target,
        Some(
            safe_c2rust_g_settings_backend_watch_weak_notify
                as unsafe extern "C" fn(gpointer, *mut GObject) -> (),
        ),
        backend as gpointer,
    );
    safe_c2rust_g_settings_backend_watch_weak_notify(backend as gpointer, target);
}
unsafe extern "C" fn safe_c2rust_g_settings_backend_invoke_closure(
    mut user_data: gpointer,
) -> gboolean {
    let mut closure: *mut GSettingsBackendClosure = user_data as *mut GSettingsBackendClosure;
    (*closure).function.expect("non-null function pointer")(
        (*closure).target,
        (*closure).backend,
        (*closure).name,
        (*closure).origin_tag,
        (*closure).names,
    );
    if !(*closure).context.is_null() {
        g_main_context_unref((*closure).context);
    }
    g_object_unref((*closure).backend as gpointer);
    g_object_unref((*closure).target as gpointer);
    g_strfreev((*closure).names);
    g_free((*closure).name as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GSettingsBackendClosure>() as gsize,
        closure as gpointer,
    );
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_settings_backend_dispatch_signal(
    mut backend: *mut GSettingsBackend,
    mut function_offset: gsize,
    mut name: *const gchar,
    mut origin_tag: gpointer,
    mut names: *const *const gchar,
) {
    let mut watch: *mut GSettingsBackendWatch = ::core::ptr::null_mut::<GSettingsBackendWatch>();
    let mut closures: *mut GSList = ::core::ptr::null_mut::<GSList>();
    g_mutex_lock(&raw mut (*(*backend).priv_0).lock);
    watch = (*(*backend).priv_0).watches;
    while !watch.is_null() {
        let mut closure: *mut GSettingsBackendClosure =
            ::core::ptr::null_mut::<GSettingsBackendClosure>();
        let mut target: *mut GObject = g_weak_ref_get(&raw mut (*watch).target) as *mut GObject;
        if !target.is_null() {
            closure = g_slice_alloc(::core::mem::size_of::<GSettingsBackendClosure>() as gsize)
                as *mut GSettingsBackendClosure;
            (*closure).context = (*watch).context;
            if !(*closure).context.is_null() {
                g_main_context_ref((*closure).context);
            }
            (*closure).backend =
                g_object_ref(backend as gpointer) as *mut GSettingsBackend as *mut GSettingsBackend;
            (*closure).target = safe_c2rust_g_steal_pointer(&raw mut target as gpointer)
                as *mut GObject as *mut GObject;
            (*closure).function = ::core::mem::transmute::<
                *mut ::core::ffi::c_void,
                Option<
                    unsafe extern "C" fn(
                        *mut GObject,
                        *mut GSettingsBackend,
                        *const gchar,
                        gpointer,
                        *mut *mut gchar,
                    ) -> (),
                >,
            >(
                *(((*watch).vtable as *mut guint8).offset(function_offset as glong as isize)
                    as gpointer as *mut *mut ::core::ffi::c_void),
            );
            (*closure).name =
                safe_c2rust_g_strdup_inline(name as *const ::core::ffi::c_char) as *mut gchar;
            (*closure).origin_tag = origin_tag;
            (*closure).names = g_strdupv(names as *mut *mut gchar);
            closures = g_slist_prepend(closures, closure as gpointer);
        }
        watch = (*watch).next;
    }
    g_mutex_unlock(&raw mut (*(*backend).priv_0).lock);
    while !closures.is_null() {
        let mut closure_0: *mut GSettingsBackendClosure =
            (*closures).data as *mut GSettingsBackendClosure;
        if !(*closure_0).context.is_null() {
            g_main_context_invoke(
                (*closure_0).context,
                Some(
                    safe_c2rust_g_settings_backend_invoke_closure
                        as unsafe extern "C" fn(gpointer) -> gboolean,
                ),
                closure_0 as gpointer,
            );
        } else {
            safe_c2rust_g_settings_backend_invoke_closure(closure_0 as gpointer);
        }
        closures = g_slist_delete_link(closures, closures);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_changed(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut origin_tag: gpointer,
) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_backend_get_type();
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
            b"G_IS_SETTINGS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if safe_c2rust_is_key(key) != 0 {
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
            b"is_key (key)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_settings_backend_dispatch_signal(
        backend,
        0 as ::core::ffi::c_ulong as glong as gsize,
        key,
        origin_tag,
        ::core::ptr::null::<*const gchar>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_keys_changed(
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
    mut items: *const *const gchar,
    mut origin_tag: gpointer,
) {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_backend_get_type();
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
            b"G_IS_SETTINGS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if safe_c2rust_is_path(path) != 0 {
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
            b"is_path (path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !items.is_null() {
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
            b"items != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_settings_backend_dispatch_signal(
        backend,
        16 as ::core::ffi::c_ulong as glong as gsize,
        path,
        origin_tag,
        items,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_path_changed(
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
    mut origin_tag: gpointer,
) {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_backend_get_type();
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
        return;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if safe_c2rust_is_path(path) != 0 {
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
            b"is_path (path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_settings_backend_dispatch_signal(
        backend,
        8 as ::core::ffi::c_ulong as glong as gsize,
        path,
        origin_tag,
        ::core::ptr::null::<*const gchar>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_writable_changed(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
) {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_backend_get_type();
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
        return;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if safe_c2rust_is_key(key) != 0 {
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
            b"is_key (key)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_settings_backend_dispatch_signal(
        backend,
        24 as ::core::ffi::c_ulong as glong as gsize,
        key,
        NULL_0,
        ::core::ptr::null::<*const gchar>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_path_writable_changed(
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
) {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_backend_get_type();
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
            b"G_IS_SETTINGS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if safe_c2rust_is_path(path) != 0 {
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
            b"is_path (path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_settings_backend_dispatch_signal(
        backend,
        32 as ::core::ffi::c_ulong as glong as gsize,
        path,
        NULL_0,
        ::core::ptr::null::<*const gchar>(),
    );
}
unsafe extern "C" fn safe_c2rust_g_settings_backend_flatten_one(
    mut key: gpointer,
    mut value: gpointer,
    mut user_data: gpointer,
) -> gboolean {
    let mut state: *mut FlattenState = user_data as *mut FlattenState;
    let mut skey: *const gchar = key as *const gchar;
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if safe_c2rust_is_key(key as *const gchar) != 0 {
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
            b"is_key (key)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if (*state).prefix.is_null() {
        let mut last_byte: *mut gchar = ::core::ptr::null_mut::<gchar>();
        (*state).prefix =
            safe_c2rust_g_strdup_inline(skey as *const ::core::ffi::c_char) as *mut gchar;
        last_byte = strrchr((*state).prefix, '/' as i32).offset(1 as ::core::ffi::c_int as isize)
            as *mut gchar;
        (*state).prefix_len = last_byte.offset_from((*state).prefix) as ::core::ffi::c_long as gint;
        *last_byte = '\0' as i32 as gchar;
    } else {
        i = 0 as ::core::ffi::c_int as gint;
        while *(*state).prefix.offset(i as isize) as ::core::ffi::c_int
            == *skey.offset(i as isize) as ::core::ffi::c_int
        {
            i += 1;
        }
        if *(*state).prefix.offset(i as isize) as ::core::ffi::c_int != '\0' as i32 {
            while *(*state)
                .prefix
                .offset((i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                != '/' as i32
            {
                i -= 1;
            }
            *(*state).prefix.offset(i as isize) = '\0' as i32 as gchar;
            (*state).prefix_len = i;
        }
    }
    let fresh3 = (*state).keys;
    (*state).keys = (*state).keys.offset(1);
    *fresh3 = key as *const gchar;
    if !(*state).values.is_null() {
        let fresh4 = (*state).values;
        (*state).values = (*state).values.offset(1);
        *fresh4 = value as *mut GVariant;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_flatten_tree(
    mut tree: *mut GTree,
    mut path: *mut *mut gchar,
    mut keys: *mut *mut *const gchar,
    mut values: *mut *mut *mut GVariant,
) {
    let mut state: FlattenState = FlattenState {
        keys: ::core::ptr::null_mut::<*const gchar>(),
        values: ::core::ptr::null_mut::<*mut GVariant>(),
        prefix_len: 0,
        prefix: ::core::ptr::null_mut::<gchar>(),
    };
    let mut nnodes: gsize = 0;
    nnodes = g_tree_nnodes(tree) as gsize;
    state.keys = ({
        let mut __n: gsize = nnodes.wrapping_add(1 as gsize);
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
    }) as *mut *const gchar;
    *keys = state.keys;
    let ref mut fresh0 = *state.keys.offset(nnodes as isize);
    *fresh0 = ::core::ptr::null::<gchar>();
    if !values.is_null() {
        state.values = ({
            let mut __n: gsize = nnodes.wrapping_add(1 as gsize);
            let mut __s: gsize = ::core::mem::size_of::<*mut GVariant>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut *mut GVariant;
        *values = state.values;
        let ref mut fresh1 = *state.values.offset(nnodes as isize);
        *fresh1 = ::core::ptr::null_mut::<GVariant>();
    }
    g_tree_foreach(
        tree,
        Some(
            safe_c2rust_g_settings_backend_flatten_one
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
        ),
        &raw mut state as gpointer,
    );
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if (*keys).offset(nnodes as isize) == state.keys {
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
            b"*keys + nnodes == state.keys\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    *path = state.prefix;
    loop {
        let fresh2 = nnodes;
        nnodes = nnodes.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        state.keys = state.keys.offset(-1);
        *state.keys = (*state.keys).offset(state.prefix_len as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_changed_tree(
    mut backend: *mut GSettingsBackend,
    mut tree: *mut GTree,
    mut origin_tag: gpointer,
) {
    let mut keys: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = backend as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_settings_backend_get_type();
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
            b"G_IS_SETTINGS_BACKEND (backend)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_settings_backend_flatten_tree(
        tree,
        &raw mut path,
        &raw mut keys,
        ::core::ptr::null_mut::<*mut *mut GVariant>(),
    );
    safe_c2rust_g_settings_backend_keys_changed(backend, path, keys, origin_tag);
    g_free(path as gpointer);
    g_free(keys as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_read(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut expected_type: *const GVariantType,
    mut default_value: gboolean,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    value = (*((*(backend as *mut GTypeInstance)).g_class as *mut GSettingsBackendClass))
        .read
        .expect("non-null function pointer")(backend, key, expected_type, default_value);
    if !value.is_null() {
        value = g_variant_take_ref(value);
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !value.is_null() && g_variant_is_of_type(value, expected_type) == 0 {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
        g_variant_unref(value);
        value = ::core::ptr::null_mut::<GVariant>();
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_read_user_value(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    value = (*((*(backend as *mut GTypeInstance)).g_class as *mut GSettingsBackendClass))
        .read_user_value
        .expect("non-null function pointer")(backend, key, expected_type);
    if !value.is_null() {
        value = g_variant_take_ref(value);
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !value.is_null() && g_variant_is_of_type(value, expected_type) == 0 {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
        g_variant_unref(value);
        value = ::core::ptr::null_mut::<GVariant>();
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_write(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut value: *mut GVariant,
    mut origin_tag: gpointer,
) -> gboolean {
    let mut success: gboolean = 0;
    g_variant_ref_sink(value);
    success = (*((*(backend as *mut GTypeInstance)).g_class as *mut GSettingsBackendClass))
        .write
        .expect("non-null function pointer")(backend, key, value, origin_tag);
    g_variant_unref(value);
    return success;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_write_tree(
    mut backend: *mut GSettingsBackend,
    mut tree: *mut GTree,
    mut origin_tag: gpointer,
) -> gboolean {
    return (*((*(backend as *mut GTypeInstance)).g_class as *mut GSettingsBackendClass))
        .write_tree
        .expect("non-null function pointer")(backend, tree, origin_tag);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_reset(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut origin_tag: gpointer,
) {
    (*((*(backend as *mut GTypeInstance)).g_class as *mut GSettingsBackendClass))
        .reset
        .expect("non-null function pointer")(backend, key, origin_tag);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_get_writable(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
) -> gboolean {
    return (*((*(backend as *mut GTypeInstance)).g_class as *mut GSettingsBackendClass))
        .get_writable
        .expect("non-null function pointer")(backend, key);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_unsubscribe(
    mut backend: *mut GSettingsBackend,
    mut name: *const ::core::ffi::c_char,
) {
    (*((*(backend as *mut GTypeInstance)).g_class as *mut GSettingsBackendClass))
        .unsubscribe
        .expect("non-null function pointer")(backend, name as *const gchar);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_subscribe(
    mut backend: *mut GSettingsBackend,
    mut name: *const gchar,
) {
    (*((*(backend as *mut GTypeInstance)).g_class as *mut GSettingsBackendClass))
        .subscribe
        .expect("non-null function pointer")(backend, name);
}
unsafe extern "C" fn safe_c2rust_g_settings_backend_finalize(mut object: *mut GObject) {
    let mut backend: *mut GSettingsBackend =
        object as *mut ::core::ffi::c_void as *mut GSettingsBackend;
    g_mutex_clear(&raw mut (*(*backend).priv_0).lock);
    (*(safe_c2rust_g_settings_backend_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_ignore_subscription(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
) {
}
unsafe extern "C" fn safe_c2rust_g_settings_backend_real_read_user_value(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    return safe_c2rust_g_settings_backend_read(backend, key, expected_type, FALSE);
}
unsafe extern "C" fn safe_c2rust_g_settings_backend_init(mut backend: *mut GSettingsBackend) {
    (*backend).priv_0 = safe_c2rust_g_settings_backend_get_instance_private(backend)
        as *mut GSettingsBackendPrivate;
    g_mutex_init(&raw mut (*(*backend).priv_0).lock);
}
unsafe extern "C" fn safe_c2rust_g_settings_backend_class_init(
    mut class: *mut GSettingsBackendClass,
) {
    let mut gobject_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*class).subscribe = Some(
        safe_c2rust_ignore_subscription
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> ()>;
    (*class).unsubscribe = Some(
        safe_c2rust_ignore_subscription
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> ()>;
    (*class).read_user_value = Some(
        safe_c2rust_g_settings_backend_real_read_user_value
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
    (*gobject_class).finalize =
        Some(safe_c2rust_g_settings_backend_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_g_settings_backend_variant_unref0(mut data: gpointer) {
    if !data.is_null() {
        g_variant_unref(data as *mut GVariant);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_create_tree() -> *mut GTree {
    return g_tree_new_full(
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >,
            GCompareDataFunc,
        >(Some(
            strcmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )),
        NULL_0,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(safe_c2rust_g_settings_backend_variant_unref0 as unsafe extern "C" fn(gpointer) -> ()),
    );
}
unsafe extern "C" fn safe_c2rust_g_settings_backend_verify(mut impl_0: gpointer) -> gboolean {
    let mut backend: *mut GSettingsBackend = impl_0 as *mut GSettingsBackend;
    if strcmp(
        g_type_name((*(*(backend as *mut GTypeInstance)).g_class).g_type)
            as *const ::core::ffi::c_char,
        b"GMemorySettingsBackend\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && g_strcmp0(
            g_getenv(b"GSETTINGS_BACKEND\0" as *const u8 as *const gchar)
                as *const ::core::ffi::c_char,
            b"memory\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_MESSAGE,
            b"Using the 'memory' GSettings backend.  Your settings will not be saved or shared with other applications.\0"
                as *const u8 as *const gchar,
        );
    }
    safe_c2rust_g_settings_has_backend = TRUE as gboolean;
    return TRUE;
}
static mut safe_c2rust_settings_backend_default_singleton: *mut GSettingsBackend =
    ::core::ptr::null::<GSettingsBackend>() as *mut GSettingsBackend;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_get_default() -> *mut GSettingsBackend {
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_settings_backend_default_singleton;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GSettingsBackend =
                ::core::ptr::null_mut::<GSettingsBackend>();
            let mut gapg_temp_atomic: *mut *mut GSettingsBackend =
                &raw mut safe_c2rust_settings_backend_default_singleton;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_settings_backend_default_singleton as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut singleton: *mut GSettingsBackend = ::core::ptr::null_mut::<GSettingsBackend>();
        singleton = _g_io_module_get_default(
            G_SETTINGS_BACKEND_EXTENSION_POINT_NAME.as_ptr() as *const gchar,
            b"GSETTINGS_BACKEND\0" as *const u8 as *const gchar,
            Some(
                safe_c2rust_g_settings_backend_verify as unsafe extern "C" fn(gpointer) -> gboolean,
            ),
        ) as *mut GSettingsBackend;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_settings_backend_default_singleton = singleton;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_settings_backend_default_singleton as *mut ::core::ffi::c_void,
            singleton as guintptr as gpointer,
        );
    }
    return g_object_ref(safe_c2rust_settings_backend_default_singleton as gpointer)
        as *mut GSettingsBackend;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_get_permission(
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
) -> *mut GPermission {
    let mut class: *mut GSettingsBackendClass =
        (*(backend as *mut GTypeInstance)).g_class as *mut GSettingsBackendClass;
    if (*class).get_permission.is_some() {
        return (*class).get_permission.expect("non-null function pointer")(backend, path);
    }
    return g_simple_permission_new(TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_settings_backend_sync_default() {
    if safe_c2rust_g_settings_has_backend != 0 {
        let mut class: *mut GSettingsBackendClass =
            ::core::ptr::null_mut::<GSettingsBackendClass>();
        let mut backend: *mut GSettingsBackend = ::core::ptr::null_mut::<GSettingsBackend>();
        backend = safe_c2rust_g_settings_backend_get_default();
        class = (*(backend as *mut GTypeInstance)).g_class as *mut GSettingsBackendClass;
        if (*class).sync.is_some() {
            (*class).sync.expect("non-null function pointer")(backend);
        }
        g_object_unref(backend as gpointer);
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
