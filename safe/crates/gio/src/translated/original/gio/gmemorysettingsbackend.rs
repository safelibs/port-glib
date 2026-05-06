extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GTree;
    pub type _GSettingsBackendPrivate;
    pub type _GPermission;
    pub type _GIOExtension;
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
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_equal(one: gconstpointer, two: gconstpointer) -> gboolean;
    fn g_tree_foreach(tree: *mut GTree, func: GTraverseFunc, user_data: gpointer);
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
    fn g_simple_permission_new(allowed: gboolean) -> *mut GPermission;
    fn g_settings_backend_get_type() -> GType;
    fn g_settings_backend_changed(
        backend: *mut GSettingsBackend,
        key: *const gchar,
        origin_tag: gpointer,
    );
    fn g_settings_backend_changed_tree(
        backend: *mut GSettingsBackend,
        tree: *mut GTree,
        origin_tag: gpointer,
    );
    fn g_io_extension_point_implement(
        extension_point_name: *const ::core::ffi::c_char,
        type_0: GType,
        extension_name: *const ::core::ffi::c_char,
        priority: gint,
    ) -> *mut GIOExtension;
    fn _g_io_modules_ensure_extension_points_registered();
}
pub type size_t = usize;
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
pub type GData = _GData;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSettingsBackend {
    pub parent_instance: GObject,
    pub priv_0: *mut GSettingsBackendPrivate,
}
pub type GSettingsBackendPrivate = _GSettingsBackendPrivate;
pub type GSettingsBackend = _GSettingsBackend;
pub type GPermission = _GPermission;
pub type GIOExtension = _GIOExtension;
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
pub struct GMemorySettingsBackend {
    pub parent_instance: GSettingsBackend,
    pub table: *mut GHashTable,
}
pub type GMemorySettingsBackendClass = GSettingsBackendClass;
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
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_memory_settings_backend_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GMemorySettingsBackend_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GMemorySettingsBackend_private_offset,
        );
    }
    safe_c2rust_g_memory_settings_backend_class_init(klass as *mut GMemorySettingsBackendClass);
}
static mut safe_c2rust_GMemorySettingsBackend_private_offset: gint = 0;
static mut safe_c2rust_g_memory_settings_backend_parent_class: gpointer = NULL_0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_settings_backend_get_type(),
        g_intern_static_string(b"GMemorySettingsBackend\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GMemorySettingsBackendClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_memory_settings_backend_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GMemorySettingsBackend>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GMemorySettingsBackend) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_memory_settings_backend_init
                    as unsafe extern "C" fn(*mut GMemorySettingsBackend) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    _g_io_modules_ensure_extension_points_registered();
    g_io_extension_point_implement(
        b"gsettings-backend\0" as *const u8 as *const ::core::ffi::c_char,
        g_define_type_id,
        b"memory\0" as *const u8 as *const ::core::ffi::c_char,
        10 as gint,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_memory_settings_backend_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_read(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut expected_type: *const GVariantType,
    mut default_value: gboolean,
) -> *mut GVariant {
    let mut memory: *mut GMemorySettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GMemorySettingsBackend;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if default_value != 0 {
        return ::core::ptr::null_mut::<GVariant>();
    }
    value = g_hash_table_lookup((*memory).table, key as gconstpointer) as *mut GVariant;
    if !value.is_null() {
        g_variant_ref(value);
    }
    return value;
}
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_write(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut value: *mut GVariant,
    mut origin_tag: gpointer,
) -> gboolean {
    let mut memory: *mut GMemorySettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GMemorySettingsBackend;
    let mut old_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    old_value = g_hash_table_lookup((*memory).table, key as gconstpointer) as *mut GVariant;
    g_variant_ref_sink(value);
    if old_value.is_null()
        || g_variant_equal(value as gconstpointer, old_value as gconstpointer) == 0
    {
        g_hash_table_insert(
            (*memory).table,
            safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
            value as gpointer,
        );
        g_settings_backend_changed(backend, key, origin_tag);
    } else {
        g_variant_unref(value);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_write_one(
    mut key: gpointer,
    mut value: gpointer,
    mut data: gpointer,
) -> gboolean {
    let mut memory: *mut GMemorySettingsBackend = data as *mut GMemorySettingsBackend;
    if !value.is_null() {
        g_hash_table_insert(
            (*memory).table,
            safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
            g_variant_ref(value as *mut GVariant) as gpointer,
        );
    } else {
        g_hash_table_remove((*memory).table, key as gconstpointer);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_write_tree(
    mut backend: *mut GSettingsBackend,
    mut tree: *mut GTree,
    mut origin_tag: gpointer,
) -> gboolean {
    g_tree_foreach(
        tree,
        Some(
            safe_c2rust_g_memory_settings_backend_write_one
                as unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean,
        ),
        backend as gpointer,
    );
    g_settings_backend_changed_tree(backend, tree, origin_tag);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_reset(
    mut backend: *mut GSettingsBackend,
    mut key: *const gchar,
    mut origin_tag: gpointer,
) {
    let mut memory: *mut GMemorySettingsBackend =
        backend as *mut ::core::ffi::c_void as *mut GMemorySettingsBackend;
    if !g_hash_table_lookup((*memory).table, key as gconstpointer).is_null() {
        g_hash_table_remove((*memory).table, key as gconstpointer);
        g_settings_backend_changed(backend, key, origin_tag);
    }
}
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_get_writable(
    mut backend: *mut GSettingsBackend,
    mut name: *const gchar,
) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_get_permission(
    mut backend: *mut GSettingsBackend,
    mut path: *const gchar,
) -> *mut GPermission {
    return g_simple_permission_new(TRUE);
}
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_finalize(mut object: *mut GObject) {
    let mut memory: *mut GMemorySettingsBackend =
        object as *mut ::core::ffi::c_void as *mut GMemorySettingsBackend;
    g_hash_table_unref((*memory).table);
    (*(safe_c2rust_g_memory_settings_backend_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_init(
    mut memory: *mut GMemorySettingsBackend,
) {
    (*memory).table = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GVariant) -> ()>, GDestroyNotify>(
            Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()),
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_class_init(
    mut class: *mut GMemorySettingsBackendClass,
) {
    let mut backend_class: *mut GSettingsBackendClass =
        class as *mut ::core::ffi::c_void as *mut GSettingsBackendClass;
    let mut object_class: *mut GObjectClass =
        class as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*backend_class).read = Some(
        safe_c2rust_g_memory_settings_backend_read
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
    (*backend_class).write = Some(
        safe_c2rust_g_memory_settings_backend_write
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
        safe_c2rust_g_memory_settings_backend_write_tree
            as unsafe extern "C" fn(*mut GSettingsBackend, *mut GTree, gpointer) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *mut GTree, gpointer) -> gboolean>;
    (*backend_class).reset = Some(
        safe_c2rust_g_memory_settings_backend_reset
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar, gpointer) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar, gpointer) -> ()>;
    (*backend_class).get_writable = Some(
        safe_c2rust_g_memory_settings_backend_get_writable
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> gboolean>;
    (*backend_class).get_permission = Some(
        safe_c2rust_g_memory_settings_backend_get_permission
            as unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> *mut GPermission,
    )
        as Option<unsafe extern "C" fn(*mut GSettingsBackend, *const gchar) -> *mut GPermission>;
    (*object_class).finalize = Some(
        safe_c2rust_g_memory_settings_backend_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_memory_settings_backend_new() -> *mut GSettingsBackend {
    return g_object_new(
        safe_c2rust_g_memory_settings_backend_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GSettingsBackend;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
