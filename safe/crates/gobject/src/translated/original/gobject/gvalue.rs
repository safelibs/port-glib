// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
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
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_parent(type_0: GType) -> GType;
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
    fn g_type_interface_instantiatable_prerequisite(interface_type: GType) -> GType;
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_value_table_peek(type_0: GType) -> *mut GTypeValueTable;
    fn g_type_check_instance(instance: *mut GTypeInstance) -> gboolean;
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_object_ref(object: gpointer) -> gpointer;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
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
pub union _GTypeCValue {
    pub v_int: gint,
    pub v_long: glong,
    pub v_int64: gint64,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GTypeCValue = _GTypeCValue;
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
pub type GTypeValueTable = _GTypeValueTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeValueTable {
    pub value_init: GTypeValueInitFunc,
    pub value_free: GTypeValueFreeFunc,
    pub value_copy: GTypeValueCopyFunc,
    pub value_peek_pointer: GTypeValuePeekPointerFunc,
    pub collect_format: *const gchar,
    pub collect_value: GTypeValueCollectFunc,
    pub lcopy_format: *const gchar,
    pub lcopy_value: GTypeValueLCopyFunc,
}
pub type GTypeValueLCopyFunc =
    Option<unsafe extern "C" fn(*const GValue, guint, *mut GTypeCValue, guint) -> *mut gchar>;
pub type GTypeValueCollectFunc =
    Option<unsafe extern "C" fn(*mut GValue, guint, *mut GTypeCValue, guint) -> *mut gchar>;
pub type GTypeValuePeekPointerFunc = Option<unsafe extern "C" fn(*const GValue) -> gpointer>;
pub type GTypeValueCopyFunc = Option<unsafe extern "C" fn(*const GValue, *mut GValue) -> ()>;
pub type GTypeValueFreeFunc = Option<unsafe extern "C" fn(*mut GValue) -> ()>;
pub type GTypeValueInitFunc = Option<unsafe extern "C" fn(*mut GValue) -> ()>;
pub type GValueTransform = Option<unsafe extern "C" fn(*const GValue, *mut GValue) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransformEntry {
    pub src_type: GType,
    pub dest_type: GType,
    pub func: GValueTransform,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GBSearchConfig {
    pub sizeof_node: guint,
    pub cmp_nodes: GBSearchCompareFunc,
    pub flags: guint,
}
pub type GBSearchCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub const G_BSEARCH_ARRAY_ALIGN_POWER2: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union GBSearchArray {
    pub n_nodes: guint,
    pub alignment_dummy1: gpointer,
    pub alignment_dummy2: glong,
    pub alignment_dummy3: gdouble,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_BSEARCH_ARRAY_AUTO_SHRINK: C2RustUnnamed_0 = 2;
#[inline]
unsafe extern "C" fn g_bit_storage_impl(mut number: gulong) -> guint {
    let mut n_bits: guint = 0 as guint;
    loop {
        n_bits = n_bits.wrapping_add(1);
        number >>= 1 as ::core::ffi::c_int;
        if !(number != 0) {
            break;
        }
    }
    return n_bits;
}
#[inline]
unsafe extern "C" fn g_bsearch_array_create(
    mut bconfig: *const GBSearchConfig,
) -> *mut GBSearchArray {
    let mut barray: *mut GBSearchArray = ::core::ptr::null_mut::<GBSearchArray>();
    let mut size: guint = 0;
    if !bconfig.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_bsearch_array_create\0" as *const u8 as *const ::core::ffi::c_char,
            b"bconfig != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBSearchArray>();
    }
    size = (::core::mem::size_of::<GBSearchArray>() as usize)
        .wrapping_add((*bconfig).sizeof_node as usize) as guint;
    if (*bconfig).flags & G_BSEARCH_ARRAY_ALIGN_POWER2 as ::core::ffi::c_int as guint != 0 {
        size = (if size != 0 {
            (1 as ::core::ffi::c_int) << g_bit_storage_impl(size.wrapping_sub(1 as guint) as gulong)
        } else {
            0 as ::core::ffi::c_int
        }) as guint;
    }
    barray = g_malloc(size as gsize) as *mut GBSearchArray;
    memset(
        barray as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GBSearchArray>() as size_t,
    );
    return barray;
}
#[inline]
unsafe extern "C" fn g_bsearch_array_lookup_fuzzy(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
    mut key_node: gconstpointer,
    sibling_or_after: guint,
) -> gpointer {
    let mut cmp_nodes: GBSearchCompareFunc = (*bconfig).cmp_nodes;
    let mut check: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut nodes: *mut guint8 =
        (barray as *mut guint8).offset(::core::mem::size_of::<GBSearchArray>() as usize as isize);
    let mut n_nodes: guint = (*barray).n_nodes;
    let mut offs: guint = 0 as guint;
    let mut sizeof_node: guint = (*bconfig).sizeof_node;
    let mut cmp: gint = 0 as gint;
    while offs < n_nodes {
        let mut i: guint = offs.wrapping_add(n_nodes) >> 1 as ::core::ffi::c_int;
        check = nodes.offset(i.wrapping_mul(sizeof_node) as isize);
        cmp = cmp_nodes.expect("non-null function pointer")(key_node, check as gconstpointer);
        if cmp == 0 as ::core::ffi::c_int {
            return (if sibling_or_after > 1 as guint {
                ::core::ptr::null_mut::<guint8>()
            } else {
                check
            }) as gpointer;
        } else if cmp < 0 as ::core::ffi::c_int {
            n_nodes = i;
        } else {
            offs = i.wrapping_add(1 as guint);
        }
    }
    return (if sibling_or_after == 0 {
        ::core::ptr::null_mut::<guint8>()
    } else if sibling_or_after > 1 as guint && cmp > 0 as ::core::ffi::c_int {
        check.offset(sizeof_node as isize)
    } else {
        check
    }) as gpointer;
}
#[inline]
unsafe extern "C" fn g_bsearch_array_get_index(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
    mut node_in_array: gconstpointer,
) -> guint {
    let mut distance: guint = (node_in_array as *mut guint8).offset_from(
        (barray as *mut guint8).offset(::core::mem::size_of::<GBSearchArray>() as usize as isize),
    ) as ::core::ffi::c_long as guint;
    if !node_in_array.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_bsearch_array_get_index\0" as *const u8 as *const ::core::ffi::c_char,
            b"node_in_array != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (*barray).n_nodes;
    }
    distance = distance.wrapping_div((*bconfig).sizeof_node);
    return if distance < (*barray).n_nodes.wrapping_add(1 as guint) {
        distance
    } else {
        (*barray).n_nodes.wrapping_add(1 as guint)
    };
}
#[inline]
unsafe extern "C" fn g_bsearch_array_grow(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
    mut index_: guint,
) -> *mut GBSearchArray {
    let mut old_size: guint = (*barray).n_nodes.wrapping_mul((*bconfig).sizeof_node);
    let mut new_size: guint = old_size.wrapping_add((*bconfig).sizeof_node);
    let mut node: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    if index_ <= (*barray).n_nodes {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_bsearch_array_grow\0" as *const u8 as *const ::core::ffi::c_char,
            b"index_ <= barray->n_nodes\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBSearchArray>();
    }
    if (*bconfig).flags & G_BSEARCH_ARRAY_ALIGN_POWER2 as ::core::ffi::c_int as guint != 0 {
        new_size = (if (::core::mem::size_of::<GBSearchArray>() as usize)
            .wrapping_add(new_size as usize)
            != 0
        {
            (1 as ::core::ffi::c_int)
                << g_bit_storage_impl(
                    (::core::mem::size_of::<GBSearchArray>() as gulong)
                        .wrapping_add(new_size as gulong)
                        .wrapping_sub(1 as gulong),
                )
        } else {
            0 as ::core::ffi::c_int
        }) as guint;
        old_size = (if (::core::mem::size_of::<GBSearchArray>() as usize)
            .wrapping_add(old_size as usize)
            != 0
        {
            (1 as ::core::ffi::c_int)
                << g_bit_storage_impl(
                    (::core::mem::size_of::<GBSearchArray>() as gulong)
                        .wrapping_add(old_size as gulong)
                        .wrapping_sub(1 as gulong),
                )
        } else {
            0 as ::core::ffi::c_int
        }) as guint;
        if old_size != new_size {
            barray = g_realloc(barray as gpointer, new_size as gsize) as *mut GBSearchArray;
        }
    } else {
        barray = g_realloc(
            barray as gpointer,
            (::core::mem::size_of::<GBSearchArray>() as gsize).wrapping_add(new_size as gsize),
        ) as *mut GBSearchArray;
    }
    node = (barray as *mut guint8)
        .offset(::core::mem::size_of::<GBSearchArray>() as usize as isize)
        .offset(index_.wrapping_mul((*bconfig).sizeof_node) as isize);
    memmove(
        node.offset((*bconfig).sizeof_node as isize) as *mut ::core::ffi::c_void,
        node as *const ::core::ffi::c_void,
        (*barray)
            .n_nodes
            .wrapping_sub(index_)
            .wrapping_mul((*bconfig).sizeof_node) as size_t,
    );
    (*barray).n_nodes = (*barray).n_nodes.wrapping_add(1 as guint);
    return barray;
}
#[inline]
unsafe extern "C" fn g_bsearch_array_insert(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
    mut key_node: gconstpointer,
) -> *mut GBSearchArray {
    let mut node: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    if (*barray).n_nodes == 0 {
        barray = g_bsearch_array_grow(barray, bconfig, 0 as guint);
        node = (barray as *mut guint8)
            .offset(::core::mem::size_of::<GBSearchArray>() as usize as isize);
    } else {
        node = g_bsearch_array_lookup_fuzzy(barray, bconfig, key_node, 2 as guint) as *mut guint8;
        if !node.is_null() {
            let mut index_: guint =
                g_bsearch_array_get_index(barray, bconfig, node as gconstpointer);
            barray = g_bsearch_array_grow(barray, bconfig, index_);
            node = (barray as *mut guint8)
                .offset(::core::mem::size_of::<GBSearchArray>() as usize as isize)
                .offset(index_.wrapping_mul((*bconfig).sizeof_node) as isize);
        } else {
            return barray;
        }
    }
    memcpy(
        node as *mut ::core::ffi::c_void,
        key_node as *const ::core::ffi::c_void,
        (*bconfig).sizeof_node as size_t,
    );
    return barray;
}
#[inline]
unsafe extern "C" fn g_bsearch_array_replace(
    mut barray: *mut GBSearchArray,
    mut bconfig: *const GBSearchConfig,
    mut key_node: gconstpointer,
) -> *mut GBSearchArray {
    let mut node: *mut guint8 =
        g_bsearch_array_lookup_fuzzy(barray, bconfig, key_node, 0 as guint) as *mut guint8;
    if !node.is_null() {
        memcpy(
            node as *mut ::core::ffi::c_void,
            key_node as *const ::core::ffi::c_void,
            (*bconfig).sizeof_node as size_t,
        );
    } else {
        barray = g_bsearch_array_insert(barray, bconfig, key_node);
    }
    return barray;
}
static mut transform_array: *mut GBSearchArray =
    ::core::ptr::null::<GBSearchArray>() as *mut GBSearchArray;
static mut transform_bconfig: GBSearchConfig = unsafe {
    GBSearchConfig {
        sizeof_node: ::core::mem::size_of::<TransformEntry>() as guint,
        cmp_nodes: Some(
            transform_entries_cmp as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
        ),
        flags: G_BSEARCH_ARRAY_ALIGN_POWER2 as ::core::ffi::c_int as guint,
    }
};
#[no_mangle]
pub unsafe extern "C" fn _g_value_c_init() {
    transform_array = g_bsearch_array_create(&raw mut transform_bconfig);
}
#[inline]
unsafe extern "C" fn value_meminit(mut value: *mut GValue, mut value_type: GType) {
    (*value).g_type = value_type;
    memset(
        &raw mut (*value).data as *mut C2RustUnnamed as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[C2RustUnnamed; 2]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_value_init(mut value: *mut GValue, mut g_type: GType) -> *mut GValue {
    let mut value_table: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
    if !value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValue>();
    }
    value_table = g_type_value_table_peek(g_type);
    if !value_table.is_null() && (*value).g_type == 0 as GType {
        value_meminit(value, g_type);
        (*value_table)
            .value_init
            .expect("non-null function pointer")(value);
    } else if (*value).g_type != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: cannot initialize GValue with type '%s', the value has already been initialized as '%s'\0"
                as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvalue.c:99\0"
                as *const u8 as *const ::core::ffi::c_char,
            g_type_name(g_type),
            g_type_name((*value).g_type),
        );
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: cannot initialize GValue with type '%s', %s\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvalue.c:104\0"
                as *const u8 as *const ::core::ffi::c_char,
            g_type_name(g_type),
            if !value_table.is_null() {
                b"this type is abstract with regards to GValue use, use a more specific (derived) type\0"
                    as *const u8 as *const ::core::ffi::c_char
            } else {
                b"this type has no GTypeValueTable implementation\0" as *const u8
                    as *const ::core::ffi::c_char
            },
        );
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_copy(mut src_value: *const GValue, mut dest_value: *mut GValue) {
    if !src_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_copy\0" as *const u8 as *const ::core::ffi::c_char,
            b"src_value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !dest_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_copy\0" as *const u8 as *const ::core::ffi::c_char,
            b"dest_value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_value_type_compatible((*(src_value as *mut GValue)).g_type, (*dest_value).g_type) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_copy\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_type_compatible (G_VALUE_TYPE (src_value), G_VALUE_TYPE (dest_value))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if src_value != dest_value as *const GValue {
        let mut dest_type: GType = (*dest_value).g_type;
        let mut value_table: *mut GTypeValueTable = g_type_value_table_peek(dest_type);
        if !value_table.is_null() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_copy\0" as *const u8 as *const ::core::ffi::c_char,
                b"value_table\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if (*value_table).value_free.is_some() {
            (*value_table)
                .value_free
                .expect("non-null function pointer")(dest_value);
        }
        value_meminit(dest_value, dest_type);
        (*value_table)
            .value_copy
            .expect("non-null function pointer")(src_value, dest_value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_value_reset(mut value: *mut GValue) -> *mut GValue {
    let mut value_table: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
    let mut g_type: GType = 0;
    if !value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_reset\0" as *const u8 as *const ::core::ffi::c_char,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValue>();
    }
    g_type = (*value).g_type;
    value_table = g_type_value_table_peek(g_type);
    if !value_table.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_reset\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_table\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GValue>();
    }
    if (*value_table).value_free.is_some() {
        (*value_table)
            .value_free
            .expect("non-null function pointer")(value);
    }
    value_meminit(value, g_type);
    (*value_table)
        .value_init
        .expect("non-null function pointer")(value);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_unset(mut value: *mut GValue) {
    let mut value_table: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
    if (*value).g_type == 0 as GType {
        return;
    }
    if !value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_unset\0" as *const u8 as *const ::core::ffi::c_char,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value_table = g_type_value_table_peek((*value).g_type);
    if !value_table.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_unset\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_table\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*value_table).value_free.is_some() {
        (*value_table)
            .value_free
            .expect("non-null function pointer")(value);
    }
    memset(
        value as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GValue>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_value_fits_pointer(mut value: *const GValue) -> gboolean {
    let mut value_table: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
    if !value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_fits_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    value_table = g_type_value_table_peek((*(value as *mut GValue)).g_type);
    if !value_table.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_fits_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_table\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*value_table).value_peek_pointer.is_some() as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_peek_pointer(mut value: *const GValue) -> gpointer {
    let mut value_table: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
    if !value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_peek_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    value_table = g_type_value_table_peek((*(value as *mut GValue)).g_type);
    if !value_table.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_peek_pointer\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_table\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*value_table).value_peek_pointer.is_none() {
        if g_value_fits_pointer(value) == (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_peek_pointer\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_fits_pointer (value) == TRUE\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*value_table)
        .value_peek_pointer
        .expect("non-null function pointer")(value);
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_instance(mut value: *mut GValue, mut instance: gpointer) {
    let mut g_type: GType = 0;
    let mut value_table: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
    let mut cvalue: GTypeCValue = _GTypeCValue { v_int: 0 };
    let mut error_msg: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if !value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_set_instance\0" as *const u8 as *const ::core::ffi::c_char,
            b"value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_type = (*value).g_type;
    value_table = g_type_value_table_peek(g_type);
    if !value_table.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_set_instance\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_table\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !instance.is_null() {
        if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_set_instance\0" as *const u8 as *const ::core::ffi::c_char,
                b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if g_value_type_compatible(
            (*(*(instance as *mut GTypeInstance)).g_class).g_type,
            (*value).g_type,
        ) != 0
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_set_instance\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_type_compatible (G_TYPE_FROM_INSTANCE (instance), G_VALUE_TYPE (value))\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if strcmp(
        (*value_table).collect_format as *const ::core::ffi::c_char,
        b"p\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_set_instance\0" as *const u8 as *const ::core::ffi::c_char,
            b"strcmp (value_table->collect_format, \"p\") == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    memset(
        &raw mut cvalue as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GTypeCValue>() as size_t,
    );
    cvalue.v_pointer = instance;
    if (*value_table).value_free.is_some() {
        (*value_table)
            .value_free
            .expect("non-null function pointer")(value);
    }
    value_meminit(value, g_type);
    error_msg = (*value_table)
        .collect_value
        .expect("non-null function pointer")(
        value, 1 as guint, &raw mut cvalue, 0 as guint
    );
    if !error_msg.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"%s: %s\0" as *const u8 as *const gchar,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvalue.c:294\0"
                as *const u8 as *const ::core::ffi::c_char,
            error_msg,
        );
        g_free(error_msg as gpointer);
        value_meminit(value, g_type);
        (*value_table)
            .value_init
            .expect("non-null function pointer")(value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_value_init_from_instance(
    mut value: *mut GValue,
    mut instance: gpointer,
) {
    if !value.is_null() && (*value).g_type == 0 as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_init_from_instance\0" as *const u8 as *const ::core::ffi::c_char,
            b"value != NULL && G_VALUE_TYPE(value) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_type_check_instance_is_fundamentally_a(
        instance as *mut GTypeInstance,
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
        value_meminit(value, (*(*(instance as *mut GTypeInstance)).g_class).g_type);
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_object_ref(instance) as gpointer;
    } else {
        let mut g_type: GType = 0;
        let mut value_table: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
        let mut cvalue: GTypeCValue = _GTypeCValue { v_int: 0 };
        let mut error_msg: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if g_type_check_instance(instance as *mut GTypeInstance) != 0 {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_init_from_instance\0" as *const u8 as *const ::core::ffi::c_char,
                b"G_TYPE_CHECK_INSTANCE (instance)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        g_type = (*(*(instance as *mut GTypeInstance)).g_class).g_type;
        value_table = g_type_value_table_peek(g_type);
        if strcmp(
            (*value_table).collect_format as *const ::core::ffi::c_char,
            b"p\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_value_init_from_instance\0" as *const u8 as *const ::core::ffi::c_char,
                b"strcmp (value_table->collect_format, \"p\") == 0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        memset(
            &raw mut cvalue as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<GTypeCValue>() as size_t,
        );
        cvalue.v_pointer = instance;
        value_meminit(value, g_type);
        (*value_table)
            .value_init
            .expect("non-null function pointer")(value);
        error_msg = (*value_table)
            .collect_value
            .expect("non-null function pointer")(
            value, 1 as guint, &raw mut cvalue, 0 as guint
        );
        if !error_msg.is_null() {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"%s: %s\0" as *const u8 as *const gchar,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gvalue.c:358\0"
                    as *const u8 as *const ::core::ffi::c_char,
                error_msg,
            );
            g_free(error_msg as gpointer);
            value_meminit(value, g_type);
            (*value_table)
                .value_init
                .expect("non-null function pointer")(value);
        }
    };
}
unsafe extern "C" fn transform_lookup_get_parent_type(mut type_0: GType) -> GType {
    if g_type_fundamental(type_0) == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        return g_type_interface_instantiatable_prerequisite(type_0);
    }
    return g_type_parent(type_0);
}
unsafe extern "C" fn transform_func_lookup(
    mut src_type: GType,
    mut dest_type: GType,
) -> GValueTransform {
    let mut entry: TransformEntry = TransformEntry {
        src_type: 0,
        dest_type: 0,
        func: None,
    };
    entry.src_type = src_type;
    loop {
        entry.dest_type = dest_type;
        loop {
            let mut e: *mut TransformEntry = ::core::ptr::null_mut::<TransformEntry>();
            e = g_bsearch_array_lookup_fuzzy(
                transform_array,
                &raw mut transform_bconfig,
                &raw mut entry as gconstpointer,
                0 as guint,
            ) as *mut TransformEntry;
            if !e.is_null() {
                if g_type_value_table_peek(entry.dest_type) == g_type_value_table_peek(dest_type)
                    && g_type_value_table_peek(entry.src_type) == g_type_value_table_peek(src_type)
                {
                    return (*e).func;
                }
            }
            entry.dest_type = transform_lookup_get_parent_type(entry.dest_type);
            if !(entry.dest_type != 0) {
                break;
            }
        }
        entry.src_type = transform_lookup_get_parent_type(entry.src_type);
        if !(entry.src_type != 0) {
            break;
        }
    }
    return None;
}
unsafe extern "C" fn transform_entries_cmp(
    mut bsearch_node1: gconstpointer,
    mut bsearch_node2: gconstpointer,
) -> gint {
    let mut e1: *const TransformEntry = bsearch_node1 as *const TransformEntry;
    let mut e2: *const TransformEntry = bsearch_node2 as *const TransformEntry;
    let mut cmp: gint = if (*e1).src_type > (*e2).src_type {
        1 as gint
    } else if (*e1).src_type == (*e2).src_type {
        0 as gint
    } else {
        -(1 as gint)
    };
    if cmp != 0 {
        return cmp;
    } else {
        return if (*e1).dest_type > (*e2).dest_type {
            1 as gint
        } else if (*e1).dest_type == (*e2).dest_type {
            0 as gint
        } else {
            -(1 as gint)
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_value_register_transform_func(
    mut src_type: GType,
    mut dest_type: GType,
    mut transform_func: GValueTransform,
) {
    let mut entry: TransformEntry = TransformEntry {
        src_type: 0,
        dest_type: 0,
        func: None,
    };
    if transform_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_register_transform_func\0" as *const u8 as *const ::core::ffi::c_char,
            b"transform_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    entry.src_type = src_type;
    entry.dest_type = dest_type;
    entry.func = transform_func;
    transform_array = g_bsearch_array_replace(
        transform_array,
        &raw mut transform_bconfig,
        &raw mut entry as gconstpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_value_type_transformable(
    mut src_type: GType,
    mut dest_type: GType,
) -> gboolean {
    if src_type != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_type_transformable\0" as *const u8 as *const ::core::ffi::c_char,
            b"src_type\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if dest_type != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_type_transformable\0" as *const u8 as *const ::core::ffi::c_char,
            b"dest_type\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (g_value_type_compatible(src_type, dest_type) != 0
        || transform_func_lookup(src_type, dest_type).is_some()) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_type_compatible(
    mut src_type: GType,
    mut dest_type: GType,
) -> gboolean {
    if src_type != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_type_compatible\0" as *const u8 as *const ::core::ffi::c_char,
            b"src_type\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if dest_type != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_type_compatible\0" as *const u8 as *const ::core::ffi::c_char,
            b"dest_type\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if src_type == dest_type {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return ((src_type == dest_type || g_type_is_a(src_type, dest_type) != 0)
        && g_type_value_table_peek(dest_type) == g_type_value_table_peek(src_type))
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_transform(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) -> gboolean {
    let mut dest_type: GType = 0;
    if !src_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_transform\0" as *const u8 as *const ::core::ffi::c_char,
            b"src_value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !dest_value.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_transform\0" as *const u8 as *const ::core::ffi::c_char,
            b"dest_value\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    dest_type = (*dest_value).g_type;
    if g_value_type_compatible((*(src_value as *mut GValue)).g_type, dest_type) != 0 {
        g_value_copy(src_value, dest_value);
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    } else {
        let mut transform: GValueTransform =
            transform_func_lookup((*(src_value as *mut GValue)).g_type, dest_type);
        if transform.is_some() {
            g_value_unset(dest_value);
            value_meminit(dest_value, dest_type);
            transform.expect("non-null function pointer")(src_value, dest_value);
            return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        }
    }
    return 0 as gboolean;
}
