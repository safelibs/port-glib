use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::c2rust_bitfields;
use ::core::arch::asm;
extern "C" {
    pub type _GVariantType;
    pub type _GVariantTypeInfo;
    pub type _GBytes;
    pub type _GVariant;
    pub type _GHashTable;
    fn g_variant_type_string_scan(
        string: *const gchar,
        limit: *const gchar,
        endptr: *mut *const gchar,
    ) -> gboolean;
    fn g_variant_type_free(type_0: *mut GVariantType);
    fn g_variant_type_copy(type_0: *const GVariantType) -> *mut GVariantType;
    fn g_variant_type_dup_string(type_0: *const GVariantType) -> *mut gchar;
    fn g_variant_type_is_definite(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_container(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_basic(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_maybe(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_array(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_tuple(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_dict_entry(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_variant(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_subtype_of(
        type_0: *const GVariantType,
        supertype: *const GVariantType,
    ) -> gboolean;
    fn g_variant_type_element(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_first(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_next(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_n_items(type_0: *const GVariantType) -> gsize;
    fn g_variant_type_key(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_new_array(element: *const GVariantType) -> *mut GVariantType;
    fn g_variant_type_new_maybe(element: *const GVariantType) -> *mut GVariantType;
    fn g_variant_type_new_tuple(
        items: *const *const GVariantType,
        length: gint,
    ) -> *mut GVariantType;
    fn g_variant_type_new_dict_entry(
        key: *const GVariantType,
        value: *const GVariantType,
    ) -> *mut GVariantType;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_type_info_get_type_string(typeinfo: *mut GVariantTypeInfo) -> *const gchar;
    fn g_variant_type_info_query(
        typeinfo: *mut GVariantTypeInfo,
        alignment: *mut guint,
        size: *mut gsize,
    );
    fn g_variant_type_info_query_element(
        typeinfo: *mut GVariantTypeInfo,
        alignment: *mut guint,
        size: *mut gsize,
    );
    fn g_variant_type_info_get(type_0: *const GVariantType) -> *mut GVariantTypeInfo;
    fn g_variant_type_info_unref(typeinfo: *mut GVariantTypeInfo);
    fn g_variant_serialised_byteswap(value: GVariantSerialised);
    fn g_variant_serialiser_is_string(data: gconstpointer, size: gsize) -> gboolean;
    fn g_variant_serialiser_is_object_path(data: gconstpointer, size: gsize) -> gboolean;
    fn g_variant_serialiser_is_signature(data: gconstpointer, size: gsize) -> gboolean;
    fn g_unichar_isprint(c: gunichar) -> gboolean;
    static safe_c2rust_g_utf8_skip: *const gchar;
    fn g_utf8_get_char(p: *const gchar) -> gunichar;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_bytes_new(data: gconstpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_new_take(data: gpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_new_static(data: gconstpointer, size: gsize) -> *mut GBytes;
    fn g_bytes_new_with_free_func(
        data: gconstpointer,
        size: gsize,
        free_func: GDestroyNotify,
        user_data: gpointer,
    ) -> *mut GBytes;
    fn g_bytes_unref(bytes: *mut GBytes);
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_ascii_dtostr(buffer: *mut gchar, buf_len: gint, d: gdouble) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strescape(source: *const gchar, exceptions: *const gchar) -> *mut gchar;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(string: *mut GString, val: *const gchar, len: gssize) -> *mut GString;
    fn g_string_append_unichar(string: *mut GString, wc: gunichar) -> *mut GString;
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_n_children(value: *mut GVariant) -> gsize;
    fn g_variant_get_child_value(value: *mut GVariant, index_: gsize) -> *mut GVariant;
    fn g_variant_get_size(value: *mut GVariant) -> gsize;
    fn g_variant_get_data(value: *mut GVariant) -> gconstpointer;
    fn g_variant_store(value: *mut GVariant, data: gpointer);
    fn g_variant_is_normal_form(value: *mut GVariant) -> gboolean;
    fn g_variant_new_from_bytes(
        type_0: *const GVariantType,
        bytes: *mut GBytes,
        trusted: gboolean,
    ) -> *mut GVariant;
    fn g_variant_new_from_children(
        type_0: *const GVariantType,
        children: *mut *mut GVariant,
        n_children: gsize,
        trusted: gboolean,
    ) -> *mut GVariant;
    fn g_variant_is_trusted(value: *mut GVariant) -> gboolean;
    fn g_variant_get_type_info(value: *mut GVariant) -> *mut GVariantTypeInfo;
    fn g_variant_get_depth(value: *mut GVariant) -> gsize;
    fn g_variant_maybe_get_child_value(value: *mut GVariant, index_: gsize) -> *mut GVariant;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_copy(block_size: gsize, mem_block: gconstpointer) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
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
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type gint16 = ::core::ffi::c_short;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GVariantType = _GVariantType;
pub type GVariantTypeInfo = _GVariantTypeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVariantSerialised {
    pub type_info: *mut GVariantTypeInfo,
    pub data: *mut guchar,
    pub size: gsize,
    pub depth: gsize,
    pub ordered_offsets_up_to: gsize,
    pub checked_offsets_up_to: gsize,
}
pub type va_list = __builtin_va_list;
pub type gunichar = guint32;
pub type GBytes = _GBytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GVariant = _GVariant;
pub type GVariantClass = ::core::ffi::c_uint;
pub const G_VARIANT_CLASS_DICT_ENTRY: GVariantClass = 123;
pub const G_VARIANT_CLASS_TUPLE: GVariantClass = 40;
pub const G_VARIANT_CLASS_ARRAY: GVariantClass = 97;
pub const G_VARIANT_CLASS_MAYBE: GVariantClass = 109;
pub const G_VARIANT_CLASS_VARIANT: GVariantClass = 118;
pub const G_VARIANT_CLASS_SIGNATURE: GVariantClass = 103;
pub const G_VARIANT_CLASS_OBJECT_PATH: GVariantClass = 111;
pub const G_VARIANT_CLASS_STRING: GVariantClass = 115;
pub const G_VARIANT_CLASS_DOUBLE: GVariantClass = 100;
pub const G_VARIANT_CLASS_HANDLE: GVariantClass = 104;
pub const G_VARIANT_CLASS_UINT64: GVariantClass = 116;
pub const G_VARIANT_CLASS_INT64: GVariantClass = 120;
pub const G_VARIANT_CLASS_UINT32: GVariantClass = 117;
pub const G_VARIANT_CLASS_INT32: GVariantClass = 105;
pub const G_VARIANT_CLASS_UINT16: GVariantClass = 113;
pub const G_VARIANT_CLASS_INT16: GVariantClass = 110;
pub const G_VARIANT_CLASS_BYTE: GVariantClass = 121;
pub const G_VARIANT_CLASS_BOOLEAN: GVariantClass = 98;
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
pub type GVariantIter = _GVariantIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantIter {
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_iter {
    pub value: *mut GVariant,
    pub n: gssize,
    pub i: gssize,
    pub loop_format: *const gchar,
    pub padding: [gsize; 3],
    pub magic: gsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap_iter {
    pub iter: stack_iter,
    pub value_ref: *mut GVariant,
    pub magic: gsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u64_0: guint64,
    pub dbl: gdouble,
}
pub type GVariantBuilder = _GVariantBuilder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantBuilder {
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: C2RustUnnamed_1,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub partial_magic: gsize,
    pub type_0: *const GVariantType,
    pub y: [guintptr; 14],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct stack_builder {
    pub parent: *mut GVariantBuilder,
    pub type_0: *mut GVariantType,
    pub expected_type: *const GVariantType,
    pub prev_item_type: *const GVariantType,
    pub min_items: gsize,
    pub max_items: gsize,
    pub children: *mut *mut GVariant,
    pub allocated_children: gsize,
    pub offset: gsize,
    #[bitfield(name = "uniform_item_types", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "trusted", ty = "guint", bits = "1..=1")]
    pub uniform_item_types_trusted: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub magic: gsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap_builder {
    pub builder: GVariantBuilder,
    pub magic: gsize,
    pub ref_count: gint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantDict {
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub s: C2RustUnnamed_3,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub asv: *mut GVariant,
    pub partial_magic: gsize,
    pub y: [guintptr; 14],
}
pub type GVariantDict = _GVariantDict;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap_dict {
    pub dict: stack_dict,
    pub ref_count: gint,
    pub magic: gsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_dict {
    pub values: *mut GHashTable,
    pub magic: gsize,
}
pub type GHashTable = _GHashTable;
pub type GHashTableIter = _GHashTableIter;
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
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_VARIANT_TYPE_BOOLEAN: *const GVariantType =
    b"b\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_BYTE: *const GVariantType =
    b"y\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_INT16: *const GVariantType =
    b"n\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_UINT16: *const GVariantType =
    b"q\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_INT32: *const GVariantType =
    b"i\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_UINT32: *const GVariantType =
    b"u\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_INT64: *const GVariantType =
    b"x\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_UINT64: *const GVariantType =
    b"t\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_DOUBLE: *const GVariantType =
    b"d\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_STRING: *const GVariantType =
    b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_OBJECT_PATH: *const GVariantType =
    b"o\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_SIGNATURE: *const GVariantType =
    b"g\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_VARIANT: *const GVariantType =
    b"v\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_HANDLE: *const GVariantType =
    b"h\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_TUPLE: *const GVariantType =
    b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_DICT_ENTRY: *const GVariantType =
    b"{?*}\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_STRING_ARRAY: *const GVariantType =
    b"as\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_OBJECT_PATH_ARRAY: *const GVariantType =
    b"ao\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_BYTESTRING: *const GVariantType =
    b"ay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_BYTESTRING_ARRAY: *const GVariantType =
    b"aay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_VARDICT: *const GVariantType =
    b"a{sv}\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_c_inline(
    mut gstring: *mut GString,
    mut c: gchar,
) -> *mut GString {
    if ({
        let mut _g_boolean_var_3: ::core::ffi::c_int = 0;
        if !gstring.is_null() && (*gstring).len.wrapping_add(1 as gsize) < (*gstring).allocated_len
        {
            _g_boolean_var_3 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_3 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_3
    }) as ::core::ffi::c_long
        != 0
    {
        let fresh15 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh15 as isize) = c;
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
    } else {
        g_string_insert_c(gstring, -(1 as ::core::ffi::c_int) as gssize, c);
    }
    return gstring;
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const ::core::ffi::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_4: ::core::ffi::c_int = 0;
        if gstring.is_null() {
            _g_boolean_var_4 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_4 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_4
    }) as ::core::ffi::c_long
        != 0
    {
        return g_string_append_len(gstring, val as *const gchar, len);
    }
    if ({
        let mut _g_boolean_var_5: ::core::ffi::c_int = 0;
        if val.is_null() {
            _g_boolean_var_5 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_5 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_5
    }) as ::core::ffi::c_long
        != 0
    {
        return if len != 0 as gssize {
            g_string_append_len(gstring, val as *const gchar, len)
        } else {
            gstring
        };
    }
    if len < 0 as gssize {
        len_unsigned = strlen(val) as gsize;
    } else {
        len_unsigned = len as gsize;
    }
    if ({
        let mut _g_boolean_var_6: ::core::ffi::c_int = 0;
        if (*gstring).len.wrapping_add(len_unsigned) < (*gstring).allocated_len {
            _g_boolean_var_6 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_6 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_6
    }) as ::core::ffi::c_long
        != 0
    {
        let mut end: *mut ::core::ffi::c_char = (*gstring).str_0.offset((*gstring).len as isize);
        if ({
            let mut _g_boolean_var_7: ::core::ffi::c_int = 0;
            if val.offset(len_unsigned as isize) <= end as *const ::core::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const ::core::ffi::c_char
            {
                _g_boolean_var_7 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_7 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_7
        }) as ::core::ffi::c_long
            != 0
        {
            memcpy(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        } else {
            memmove(
                end as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                len_unsigned as size_t,
            );
        }
        (*gstring).len = (*gstring).len.wrapping_add(len_unsigned);
        *(*gstring).str_0.offset((*gstring).len as isize) = 0 as gchar;
        return gstring;
    } else {
        return g_string_insert_len(
            gstring,
            -(1 as ::core::ffi::c_int) as gssize,
            val as *const gchar,
            len,
        );
    };
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_string_truncate_inline(
    mut gstring: *mut GString,
    mut len: gsize,
) -> *mut GString {
    (*gstring).len = if len < (*gstring).len {
        len
    } else {
        (*gstring).len
    };
    *(*gstring).str_0.offset((*gstring).len as isize) = '\0' as i32 as gchar;
    return gstring;
}
unsafe extern "C" fn safe_c2rust_g_variant_new_from_trusted(
    mut type_0: *const GVariantType,
    mut data: gconstpointer,
    mut size: gsize,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    bytes = g_bytes_new(data, size);
    value = g_variant_new_from_bytes(type_0, bytes, TRUE);
    g_bytes_unref(bytes);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_boolean(mut value: gboolean) -> *mut GVariant {
    let mut v: guchar = value as guchar;
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_BOOLEAN,
        &raw mut v as gconstpointer,
        1 as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_boolean(mut value: *mut GVariant) -> gboolean {
    let mut data: *const guchar = ::core::ptr::null::<guchar>();
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"b\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_BOOLEAN)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    data = g_variant_get_data(value) as *const guchar;
    return if !data.is_null() {
        (*data as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
    } else {
        FALSE
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_byte(mut value: guint8) -> *mut GVariant {
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_BYTE,
        &raw mut value as gconstpointer,
        ::core::mem::size_of::<guint8>() as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_byte(mut value: *mut GVariant) -> guint8 {
    let mut data: *const guint8 = ::core::ptr::null::<guint8>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"y\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_BYTE)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint8;
    }
    data = g_variant_get_data(value) as *const guint8;
    return (if !data.is_null() {
        *data as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as guint8;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_int16(mut value: gint16) -> *mut GVariant {
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_INT16,
        &raw mut value as gconstpointer,
        ::core::mem::size_of::<gint16>() as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_int16(mut value: *mut GVariant) -> gint16 {
    let mut data: *const gint16 = ::core::ptr::null::<gint16>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"n\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_INT16)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint16;
    }
    data = g_variant_get_data(value) as *const gint16;
    return (if !data.is_null() {
        *data as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as gint16;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_uint16(mut value: guint16) -> *mut GVariant {
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_UINT16,
        &raw mut value as gconstpointer,
        ::core::mem::size_of::<guint16>() as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_uint16(mut value: *mut GVariant) -> guint16 {
    let mut data: *const guint16 = ::core::ptr::null::<guint16>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"q\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_UINT16)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint16;
    }
    data = g_variant_get_data(value) as *const guint16;
    return (if !data.is_null() {
        *data as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as guint16;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_int32(mut value: gint32) -> *mut GVariant {
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_INT32,
        &raw mut value as gconstpointer,
        ::core::mem::size_of::<gint32>() as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_int32(mut value: *mut GVariant) -> gint32 {
    let mut data: *const gint32 = ::core::ptr::null::<gint32>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"i\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_INT32)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint32;
    }
    data = g_variant_get_data(value) as *const gint32;
    return if !data.is_null() { *data } else { 0 as gint32 };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_uint32(mut value: guint32) -> *mut GVariant {
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_UINT32,
        &raw mut value as gconstpointer,
        ::core::mem::size_of::<guint32>() as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_uint32(mut value: *mut GVariant) -> guint32 {
    let mut data: *const guint32 = ::core::ptr::null::<guint32>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"u\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_UINT32)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    data = g_variant_get_data(value) as *const guint32;
    return if !data.is_null() { *data } else { 0 as guint32 };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_int64(mut value: gint64) -> *mut GVariant {
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_INT64,
        &raw mut value as gconstpointer,
        ::core::mem::size_of::<gint64>() as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_int64(mut value: *mut GVariant) -> gint64 {
    let mut data: *const gint64 = ::core::ptr::null::<gint64>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"x\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_INT64)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint64;
    }
    data = g_variant_get_data(value) as *const gint64;
    return if !data.is_null() { *data } else { 0 as gint64 };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_uint64(mut value: guint64) -> *mut GVariant {
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_UINT64,
        &raw mut value as gconstpointer,
        ::core::mem::size_of::<guint64>() as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_uint64(mut value: *mut GVariant) -> guint64 {
    let mut data: *const guint64 = ::core::ptr::null::<guint64>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"t\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_UINT64)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as guint64;
    }
    data = g_variant_get_data(value) as *const guint64;
    return if !data.is_null() { *data } else { 0 as guint64 };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_handle(mut value: gint32) -> *mut GVariant {
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_HANDLE,
        &raw mut value as gconstpointer,
        ::core::mem::size_of::<gint32>() as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_handle(mut value: *mut GVariant) -> gint32 {
    let mut data: *const gint32 = ::core::ptr::null::<gint32>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"h\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_HANDLE)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint32;
    }
    data = g_variant_get_data(value) as *const gint32;
    return if !data.is_null() { *data } else { 0 as gint32 };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_double(mut value: gdouble) -> *mut GVariant {
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_DOUBLE,
        &raw mut value as gconstpointer,
        ::core::mem::size_of::<gdouble>() as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_double(mut value: *mut GVariant) -> gdouble {
    let mut data: *const gdouble = ::core::ptr::null::<gdouble>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"d\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_DOUBLE)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int as gdouble;
    }
    data = g_variant_get_data(value) as *const gdouble;
    return if !data.is_null() {
        *data
    } else {
        0 as ::core::ffi::c_int as gdouble
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_maybe(
    mut child_type: *const GVariantType,
    mut child: *mut GVariant,
) -> *mut GVariant {
    let mut maybe_type: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if child_type.is_null() || g_variant_type_is_definite(child_type) != 0 {
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
            b"child_type == NULL || g_variant_type_is_definite (child_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !child_type.is_null() || !child.is_null() {
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
            b"child_type != NULL || child != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if child_type.is_null()
            || child.is_null()
            || safe_c2rust_g_variant_is_of_type(child, child_type) != 0
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
            b"child_type == NULL || child == NULL || g_variant_is_of_type (child, child_type)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if child_type.is_null() {
        child_type = safe_c2rust_g_variant_get_type(child);
    }
    maybe_type = g_variant_type_new_maybe(child_type);
    if !child.is_null() {
        let mut children: *mut *mut GVariant = ::core::ptr::null_mut::<*mut GVariant>();
        let mut trusted: gboolean = 0;
        children = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
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
        let ref mut fresh31 = *children.offset(0 as ::core::ffi::c_int as isize);
        *fresh31 = g_variant_ref_sink(child);
        trusted = g_variant_is_trusted(*children.offset(0 as ::core::ffi::c_int as isize));
        value = g_variant_new_from_children(maybe_type, children, 1 as gsize, trusted);
    } else {
        value = g_variant_new_from_children(
            maybe_type,
            ::core::ptr::null_mut::<*mut GVariant>(),
            0 as gsize,
            TRUE,
        );
    }
    g_variant_type_free(maybe_type);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_maybe(
    mut value: *mut GVariant,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"m*\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_MAYBE)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if g_variant_n_children(value) != 0 {
        return g_variant_get_child_value(value, 0 as gsize);
    }
    return ::core::ptr::null_mut::<GVariant>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_variant(
    mut value: *mut GVariant,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !value.is_null() {
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
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    g_variant_ref_sink(value);
    return g_variant_new_from_children(
        G_VARIANT_TYPE_VARIANT,
        g_memdup2(
            &raw mut value as gconstpointer,
            ::core::mem::size_of::<*mut GVariant>() as gsize,
        ) as *mut *mut GVariant,
        1 as gsize,
        g_variant_is_trusted(value),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_variant(
    mut value: *mut GVariant,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"v\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_VARIANT)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return g_variant_get_child_value(value, 0 as gsize);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_array(
    mut child_type: *const GVariantType,
    mut children: *const *mut GVariant,
    mut n_children: gsize,
) -> *mut GVariant {
    let mut array_type: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    let mut my_children: *mut *mut GVariant = ::core::ptr::null_mut::<*mut GVariant>();
    let mut trusted: gboolean = 0;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if n_children > 0 as gsize || !child_type.is_null() {
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
            b"n_children > 0 || child_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if n_children == 0 as gsize || !children.is_null() {
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
            b"n_children == 0 || children != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if child_type.is_null() || g_variant_type_is_definite(child_type) != 0 {
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
            b"child_type == NULL || g_variant_type_is_definite (child_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    my_children = ({
        let mut __n: gsize = n_children;
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
    trusted = TRUE as gboolean;
    if child_type.is_null() {
        child_type =
            safe_c2rust_g_variant_get_type(*children.offset(0 as ::core::ffi::c_int as isize));
    }
    array_type = g_variant_type_new_array(child_type);
    i = 0 as gsize;
    while i < n_children {
        let mut is_of_child_type: gboolean =
            safe_c2rust_g_variant_is_of_type(*children.offset(i as isize), child_type);
        if ({
            let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
            if is_of_child_type == 0 {
                _g_boolean_var_27 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_27 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_27
        }) as ::core::ffi::c_long
            != 0
        {
            while i != 0 as gsize {
                i = i.wrapping_sub(1);
                g_variant_unref(*my_children.offset(i as isize));
            }
            g_free(my_children as gpointer);
            if ({
                let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
                if is_of_child_type != 0 {
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
                    b"is_of_child_type\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return ::core::ptr::null_mut::<GVariant>();
            }
        }
        let ref mut fresh32 = *my_children.offset(i as isize);
        *fresh32 = g_variant_ref_sink(*children.offset(i as isize));
        trusted &= g_variant_is_trusted(*children.offset(i as isize));
        i = i.wrapping_add(1);
    }
    value = g_variant_new_from_children(array_type, my_children, n_children, trusted);
    g_variant_type_free(array_type);
    return value;
}
unsafe extern "C" fn safe_c2rust_g_variant_make_tuple_type(
    mut children: *const *mut GVariant,
    mut n_children: gsize,
) -> *mut GVariantType {
    let mut types: *mut *const GVariantType = ::core::ptr::null_mut::<*const GVariantType>();
    let mut type_0: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    let mut i: gsize = 0;
    types = ({
        let mut __n: gsize = n_children;
        let mut __s: gsize = ::core::mem::size_of::<*const GVariantType>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut *const GVariantType;
    i = 0 as gsize;
    while i < n_children {
        let ref mut fresh34 = *types.offset(i as isize);
        *fresh34 = safe_c2rust_g_variant_get_type(*children.offset(i as isize));
        i = i.wrapping_add(1);
    }
    type_0 = g_variant_type_new_tuple(types, n_children as gint);
    g_free(types as gpointer);
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_tuple(
    mut children: *const *mut GVariant,
    mut n_children: gsize,
) -> *mut GVariant {
    let mut tuple_type: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    let mut my_children: *mut *mut GVariant = ::core::ptr::null_mut::<*mut GVariant>();
    let mut trusted: gboolean = 0;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if n_children == 0 as gsize || !children.is_null() {
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
            b"n_children == 0 || children != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    my_children = ({
        let mut __n: gsize = n_children;
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
    trusted = TRUE as gboolean;
    i = 0 as gsize;
    while i < n_children {
        let ref mut fresh33 = *my_children.offset(i as isize);
        *fresh33 = g_variant_ref_sink(*children.offset(i as isize));
        trusted &= g_variant_is_trusted(*children.offset(i as isize));
        i = i.wrapping_add(1);
    }
    tuple_type = safe_c2rust_g_variant_make_tuple_type(children, n_children);
    value = g_variant_new_from_children(tuple_type, my_children, n_children, trusted);
    g_variant_type_free(tuple_type);
    return value;
}
unsafe extern "C" fn safe_c2rust_g_variant_make_dict_entry_type(
    mut key: *mut GVariant,
    mut val: *mut GVariant,
) -> *mut GVariantType {
    return g_variant_type_new_dict_entry(
        safe_c2rust_g_variant_get_type(key),
        safe_c2rust_g_variant_get_type(val),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_dict_entry(
    mut key: *mut GVariant,
    mut value: *mut GVariant,
) -> *mut GVariant {
    let mut dict_type: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    let mut children: *mut *mut GVariant = ::core::ptr::null_mut::<*mut GVariant>();
    let mut trusted: gboolean = 0;
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !key.is_null() && !value.is_null() {
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
            b"key != NULL && value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_container(key) == 0 {
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
            b"!g_variant_is_container (key)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    children = ({
        let mut __n: gsize = 2 as ::core::ffi::c_int as gsize;
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
    let ref mut fresh35 = *children.offset(0 as ::core::ffi::c_int as isize);
    *fresh35 = g_variant_ref_sink(key);
    let ref mut fresh36 = *children.offset(1 as ::core::ffi::c_int as isize);
    *fresh36 = g_variant_ref_sink(value);
    trusted = (g_variant_is_trusted(key) != 0 && g_variant_is_trusted(value) != 0)
        as ::core::ffi::c_int as gboolean;
    dict_type = safe_c2rust_g_variant_make_dict_entry_type(key, value);
    value = g_variant_new_from_children(dict_type, children, 2 as gsize, trusted);
    g_variant_type_free(dict_type);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_lookup(
    mut dictionary: *mut GVariant,
    mut key: *const gchar,
    mut format_string: *const gchar,
    mut args: ...
) -> gboolean {
    let mut type_0: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    g_variant_get_data(dictionary);
    type_0 = safe_c2rust_g_variant_format_string_scan_type(
        format_string,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*const gchar>(),
    );
    value = safe_c2rust_g_variant_lookup_value(dictionary, key, type_0);
    g_variant_type_free(type_0);
    if !value.is_null() {
        let mut ap: ::core::ffi::VaList;
        ap = args.clone();
        safe_c2rust_g_variant_get_va(
            value,
            format_string,
            ::core::ptr::null_mut::<*const gchar>(),
            &raw mut ap,
        );
        g_variant_unref(value);
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_lookup_value(
    mut dictionary: *mut GVariant,
    mut key: *const gchar,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut entry: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            dictionary,
            g_variant_type_checked_(b"a{s*}\0" as *const u8 as *const gchar),
        ) != 0
            || safe_c2rust_g_variant_is_of_type(
                dictionary,
                g_variant_type_checked_(b"a{o*}\0" as *const u8 as *const gchar),
            ) != 0
        {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (dictionary, G_VARIANT_TYPE (\"a{s*}\")) || g_variant_is_of_type (dictionary, G_VARIANT_TYPE (\"a{o*}\"))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    safe_c2rust_g_variant_iter_init(&raw mut iter, dictionary);
    loop {
        entry = safe_c2rust_g_variant_iter_next_value(&raw mut iter);
        if entry.is_null() {
            break;
        }
        let mut entry_key: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        let mut matches: gboolean = 0;
        entry_key = g_variant_get_child_value(entry, 0 as gsize);
        matches = (strcmp(
            safe_c2rust_g_variant_get_string(entry_key, ::core::ptr::null_mut::<gsize>())
                as *const ::core::ffi::c_char,
            key as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
        g_variant_unref(entry_key);
        if matches != 0 {
            break;
        }
        g_variant_unref(entry);
    }
    if entry.is_null() {
        return ::core::ptr::null_mut::<GVariant>();
    }
    value = g_variant_get_child_value(entry, 1 as gsize);
    g_variant_unref(entry);
    if safe_c2rust_g_variant_is_of_type(value, G_VARIANT_TYPE_VARIANT) != 0 {
        let mut tmp: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        tmp = safe_c2rust_g_variant_get_variant(value);
        g_variant_unref(value);
        if !expected_type.is_null() && safe_c2rust_g_variant_is_of_type(tmp, expected_type) == 0 {
            g_variant_unref(tmp);
            tmp = ::core::ptr::null_mut::<GVariant>();
        }
        value = tmp;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if expected_type.is_null()
            || value.is_null()
            || safe_c2rust_g_variant_is_of_type(value, expected_type) != 0
        {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"expected_type == NULL || value == NULL || g_variant_is_of_type (value, expected_type)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_fixed_array(
    mut value: *mut GVariant,
    mut n_elements: *mut gsize,
    mut element_size: gsize,
) -> gconstpointer {
    let mut array_info: *mut GVariantTypeInfo = ::core::ptr::null_mut::<GVariantTypeInfo>();
    let mut array_element_size: gsize = 0;
    let mut data: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
    let mut size: gsize = 0;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"a*\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_ARRAY)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !n_elements.is_null() {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"n_elements != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if element_size > 0 as gsize {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"element_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    array_info = g_variant_get_type_info(value);
    g_variant_type_info_query_element(
        array_info,
        ::core::ptr::null_mut::<guint>(),
        &raw mut array_element_size,
    );
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if array_element_size != 0 {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"array_element_size\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if array_element_size != element_size {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
        if array_element_size != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_variant_get_fixed_array: assertion 'g_variant_array_has_fixed_size (value, element_size)' failed: array size %lu does not match given element_size %lu.\0"
                    as *const u8 as *const gchar,
                array_element_size,
                element_size,
            );
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_variant_get_fixed_array: assertion 'g_variant_array_has_fixed_size (value, element_size)' failed: array does not have fixed size.\0"
                    as *const u8 as *const gchar,
            );
        }
    }
    data = g_variant_get_data(value);
    size = g_variant_get_size(value);
    if size.wrapping_rem(element_size) != 0 {
        *n_elements = 0 as gsize;
    } else {
        *n_elements = size.wrapping_div(element_size);
    }
    if *n_elements != 0 {
        return data;
    }
    return ::core::ptr::null::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_fixed_array(
    mut element_type: *const GVariantType,
    mut elements: gconstpointer,
    mut n_elements: gsize,
    mut element_size: gsize,
) -> *mut GVariant {
    let mut array_type: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    let mut array_element_size: gsize = 0;
    let mut array_info: *mut GVariantTypeInfo = ::core::ptr::null_mut::<GVariantTypeInfo>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if g_variant_type_is_definite(element_type) != 0 {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_type_is_definite (element_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if element_size > 0 as gsize {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"element_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    array_type = g_variant_type_new_array(element_type);
    array_info = g_variant_type_info_get(array_type);
    g_variant_type_info_query_element(
        array_info,
        ::core::ptr::null_mut::<guint>(),
        &raw mut array_element_size,
    );
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if array_element_size != element_size {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
        if array_element_size != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_variant_new_fixed_array: array size %lu does not match given element_size %lu.\0"
                    as *const u8 as *const gchar,
                array_element_size,
                element_size,
            );
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_variant_get_fixed_array: array does not have fixed size.\0" as *const u8
                    as *const gchar,
            );
        }
        return ::core::ptr::null_mut::<GVariant>();
    }
    data = g_memdup2(elements, n_elements.wrapping_mul(element_size));
    value = safe_c2rust_g_variant_new_from_data(
        array_type,
        data as gconstpointer,
        n_elements.wrapping_mul(element_size),
        FALSE,
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        data,
    );
    g_variant_type_free(array_type);
    g_variant_type_info_unref(array_info);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_string(
    mut string: *const gchar,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if g_utf8_validate(
            string,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<*const gchar>(),
        ) != 0
        {
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_utf8_validate (string, -1, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_STRING,
        string as gconstpointer,
        (strlen(string as *const ::core::ffi::c_char) as gsize).wrapping_add(1 as gsize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_take_string(
    mut string: *mut gchar,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if g_utf8_validate(
            string,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<*const gchar>(),
        ) != 0
        {
            _g_boolean_var_45 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_45 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_45
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_utf8_validate (string, -1, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    bytes = g_bytes_new_take(
        string as gpointer,
        (strlen(string) as gsize).wrapping_add(1 as gsize),
    );
    value = g_variant_new_from_bytes(G_VARIANT_TYPE_STRING, bytes, TRUE);
    g_bytes_unref(bytes);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_printf(
    mut format_string: *const gchar,
    mut args: ...
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    let mut string: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !format_string.is_null() {
            _g_boolean_var_46 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_46 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_46
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"format_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    ap = args.clone();
    string = g_strdup_vprintf(format_string, ap.clone());
    bytes = g_bytes_new_take(
        string as gpointer,
        (strlen(string) as gsize).wrapping_add(1 as gsize),
    );
    value = g_variant_new_from_bytes(G_VARIANT_TYPE_STRING, bytes, TRUE);
    g_bytes_unref(bytes);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_object_path(
    mut object_path: *const gchar,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_object_path(object_path) != 0 {
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_object_path (object_path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_OBJECT_PATH,
        object_path as gconstpointer,
        (strlen(object_path as *const ::core::ffi::c_char) as gsize).wrapping_add(1 as gsize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_is_object_path(
    mut string: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_variant_serialiser_is_object_path(
        string as gconstpointer,
        (strlen(string as *const ::core::ffi::c_char) as gsize).wrapping_add(1 as gsize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_signature(
    mut signature: *const gchar,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_signature(signature) != 0 {
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_signature (signature)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_SIGNATURE,
        signature as gconstpointer,
        (strlen(signature as *const ::core::ffi::c_char) as gsize).wrapping_add(1 as gsize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_is_signature(mut string: *const gchar) -> gboolean {
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return g_variant_serialiser_is_signature(
        string as gconstpointer,
        (strlen(string as *const ::core::ffi::c_char) as gsize).wrapping_add(1 as gsize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_string(
    mut value: *mut GVariant,
    mut length: *mut gsize,
) -> *const gchar {
    let mut data: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
    let mut size: gsize = 0;
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) != 0
            || safe_c2rust_g_variant_is_of_type(
                value,
                b"o\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
            ) != 0
            || safe_c2rust_g_variant_is_of_type(
                value,
                b"g\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
            ) != 0
        {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_STRING) || g_variant_is_of_type (value, G_VARIANT_TYPE_OBJECT_PATH) || g_variant_is_of_type (value, G_VARIANT_TYPE_SIGNATURE)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    data = g_variant_get_data(value);
    size = g_variant_get_size(value);
    if g_variant_is_trusted(value) == 0 {
        match safe_c2rust_g_variant_classify(value) as ::core::ffi::c_uint {
            115 => {
                if !(g_variant_serialiser_is_string(data, size) != 0) {
                    data = b"\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer;
                    size = 1 as gsize;
                }
            }
            111 => {
                if !(g_variant_serialiser_is_object_path(data, size) != 0) {
                    data = b"/\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer;
                    size = 2 as gsize;
                }
            }
            103 => {
                if !(g_variant_serialiser_is_signature(data, size) != 0) {
                    data = b"\0" as *const u8 as *const ::core::ffi::c_char as gconstpointer;
                    size = 1 as gsize;
                }
            }
            _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1517 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
    }
    if !length.is_null() {
        *length = size.wrapping_sub(1 as gsize);
    }
    return data as *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dup_string(
    mut value: *mut GVariant,
    mut length: *mut gsize,
) -> *mut gchar {
    return safe_c2rust_g_strdup_inline(
        safe_c2rust_g_variant_get_string(value, length) as *const ::core::ffi::c_char
    ) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_strv(
    mut strv: *const *const gchar,
    mut length: gssize,
) -> *mut GVariant {
    let mut strings: *mut *mut GVariant = ::core::ptr::null_mut::<*mut GVariant>();
    let mut i: gsize = 0;
    let mut length_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if length == 0 as gssize || !strv.is_null() {
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length == 0 || strv != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if length < 0 as gssize {
        length = g_strv_length(strv as *mut *mut gchar) as gssize;
    }
    length_unsigned = length as gsize;
    strings = ({
        let mut __n: gsize = length_unsigned;
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
    i = 0 as gsize;
    while i < length_unsigned {
        let ref mut fresh16 = *strings.offset(i as isize);
        *fresh16 = g_variant_ref_sink(safe_c2rust_g_variant_new_string(*strv.offset(i as isize)));
        i = i.wrapping_add(1);
    }
    return g_variant_new_from_children(
        G_VARIANT_TYPE_STRING_ARRAY,
        strings,
        length_unsigned,
        TRUE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_strv(
    mut value: *mut GVariant,
    mut length: *mut gsize,
) -> *mut *const gchar {
    let mut strv: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut n: gsize = 0;
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"as\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_STRING_ARRAY)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*const gchar>();
    }
    g_variant_get_data(value);
    n = g_variant_n_children(value);
    strv = ({
        let mut __n: gsize = n.wrapping_add(1 as gsize);
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
    i = 0 as gsize;
    while i < n {
        let mut string: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        string = g_variant_get_child_value(value, i);
        let ref mut fresh19 = *strv.offset(i as isize);
        *fresh19 = safe_c2rust_g_variant_get_string(string, ::core::ptr::null_mut::<gsize>());
        g_variant_unref(string);
        i = i.wrapping_add(1);
    }
    let ref mut fresh20 = *strv.offset(i as isize);
    *fresh20 = ::core::ptr::null::<gchar>();
    if !length.is_null() {
        *length = n;
    }
    return strv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dup_strv(
    mut value: *mut GVariant,
    mut length: *mut gsize,
) -> *mut *mut gchar {
    let mut strv: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut n: gsize = 0;
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"as\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_STRING_ARRAY)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    n = g_variant_n_children(value);
    strv = ({
        let mut __n: gsize = n.wrapping_add(1 as gsize);
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
    i = 0 as gsize;
    while i < n {
        let mut string: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        string = g_variant_get_child_value(value, i);
        let ref mut fresh21 = *strv.offset(i as isize);
        *fresh21 = safe_c2rust_g_variant_dup_string(string, ::core::ptr::null_mut::<gsize>());
        g_variant_unref(string);
        i = i.wrapping_add(1);
    }
    let ref mut fresh22 = *strv.offset(i as isize);
    *fresh22 = ::core::ptr::null_mut::<gchar>();
    if !length.is_null() {
        *length = n;
    }
    return strv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_objv(
    mut strv: *const *const gchar,
    mut length: gssize,
) -> *mut GVariant {
    let mut strings: *mut *mut GVariant = ::core::ptr::null_mut::<*mut GVariant>();
    let mut i: gsize = 0;
    let mut length_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if length == 0 as gssize || !strv.is_null() {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length == 0 || strv != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if length < 0 as gssize {
        length = g_strv_length(strv as *mut *mut gchar) as gssize;
    }
    length_unsigned = length as gsize;
    strings = ({
        let mut __n: gsize = length_unsigned;
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
    i = 0 as gsize;
    while i < length_unsigned {
        let ref mut fresh17 = *strings.offset(i as isize);
        *fresh17 = g_variant_ref_sink(safe_c2rust_g_variant_new_object_path(
            *strv.offset(i as isize),
        ));
        i = i.wrapping_add(1);
    }
    return g_variant_new_from_children(
        G_VARIANT_TYPE_OBJECT_PATH_ARRAY,
        strings,
        length_unsigned,
        TRUE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_objv(
    mut value: *mut GVariant,
    mut length: *mut gsize,
) -> *mut *const gchar {
    let mut strv: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut n: gsize = 0;
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"ao\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_OBJECT_PATH_ARRAY)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*const gchar>();
    }
    g_variant_get_data(value);
    n = g_variant_n_children(value);
    strv = ({
        let mut __n: gsize = n.wrapping_add(1 as gsize);
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
    i = 0 as gsize;
    while i < n {
        let mut string: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        string = g_variant_get_child_value(value, i);
        let ref mut fresh23 = *strv.offset(i as isize);
        *fresh23 = safe_c2rust_g_variant_get_string(string, ::core::ptr::null_mut::<gsize>());
        g_variant_unref(string);
        i = i.wrapping_add(1);
    }
    let ref mut fresh24 = *strv.offset(i as isize);
    *fresh24 = ::core::ptr::null::<gchar>();
    if !length.is_null() {
        *length = n;
    }
    return strv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dup_objv(
    mut value: *mut GVariant,
    mut length: *mut gsize,
) -> *mut *mut gchar {
    let mut strv: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut n: gsize = 0;
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"ao\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_OBJECT_PATH_ARRAY)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    n = g_variant_n_children(value);
    strv = ({
        let mut __n: gsize = n.wrapping_add(1 as gsize);
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
    i = 0 as gsize;
    while i < n {
        let mut string: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        string = g_variant_get_child_value(value, i);
        let ref mut fresh25 = *strv.offset(i as isize);
        *fresh25 = safe_c2rust_g_variant_dup_string(string, ::core::ptr::null_mut::<gsize>());
        g_variant_unref(string);
        i = i.wrapping_add(1);
    }
    let ref mut fresh26 = *strv.offset(i as isize);
    *fresh26 = ::core::ptr::null_mut::<gchar>();
    if !length.is_null() {
        *length = n;
    }
    return strv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_bytestring(
    mut string: *const gchar,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !string.is_null() {
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return safe_c2rust_g_variant_new_from_trusted(
        G_VARIANT_TYPE_BYTESTRING,
        string as gconstpointer,
        (strlen(string as *const ::core::ffi::c_char) as gsize).wrapping_add(1 as gsize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_bytestring(
    mut value: *mut GVariant,
) -> *const gchar {
    let mut string: *const gchar = ::core::ptr::null::<gchar>();
    let mut size: gsize = 0;
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"ay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_BYTESTRING)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    string = g_variant_get_data(value) as *const gchar;
    size = g_variant_get_size(value);
    if size != 0
        && *string.offset(size.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
            == '\0' as i32
    {
        return string;
    } else {
        return b"\0" as *const u8 as *const gchar;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dup_bytestring(
    mut value: *mut GVariant,
    mut length: *mut gsize,
) -> *mut gchar {
    let mut original: *const gchar = safe_c2rust_g_variant_get_bytestring(value);
    let mut size: gsize = 0;
    if original.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    size = strlen(original as *const ::core::ffi::c_char) as gsize;
    if !length.is_null() {
        *length = size;
    }
    return g_memdup2(original as gconstpointer, size.wrapping_add(1 as gsize)) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_bytestring_array(
    mut strv: *const *const gchar,
    mut length: gssize,
) -> *mut GVariant {
    let mut strings: *mut *mut GVariant = ::core::ptr::null_mut::<*mut GVariant>();
    let mut i: gsize = 0;
    let mut length_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if length == 0 as gssize || !strv.is_null() {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length == 0 || strv != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if length < 0 as gssize {
        length = g_strv_length(strv as *mut *mut gchar) as gssize;
    }
    length_unsigned = length as gsize;
    strings = ({
        let mut __n: gsize = length_unsigned;
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
    i = 0 as gsize;
    while i < length_unsigned {
        let ref mut fresh18 = *strings.offset(i as isize);
        *fresh18 = g_variant_ref_sink(safe_c2rust_g_variant_new_bytestring(
            *strv.offset(i as isize),
        ));
        i = i.wrapping_add(1);
    }
    return g_variant_new_from_children(
        G_VARIANT_TYPE_BYTESTRING_ARRAY,
        strings,
        length_unsigned,
        TRUE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_bytestring_array(
    mut value: *mut GVariant,
    mut length: *mut gsize,
) -> *mut *const gchar {
    let mut strv: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut n: gsize = 0;
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"aay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_BYTESTRING_ARRAY)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*const gchar>();
    }
    g_variant_get_data(value);
    n = g_variant_n_children(value);
    strv = ({
        let mut __n: gsize = n.wrapping_add(1 as gsize);
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
    i = 0 as gsize;
    while i < n {
        let mut string: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        string = g_variant_get_child_value(value, i);
        let ref mut fresh27 = *strv.offset(i as isize);
        *fresh27 = safe_c2rust_g_variant_get_bytestring(string);
        g_variant_unref(string);
        i = i.wrapping_add(1);
    }
    let ref mut fresh28 = *strv.offset(i as isize);
    *fresh28 = ::core::ptr::null::<gchar>();
    if !length.is_null() {
        *length = n;
    }
    return strv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dup_bytestring_array(
    mut value: *mut GVariant,
    mut length: *mut gsize,
) -> *mut *mut gchar {
    let mut strv: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut n: gsize = 0;
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_is_of_type(
            value,
            b"aay\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
        ) == 0
        {
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_is_of_type (value, G_VARIANT_TYPE_BYTESTRING_ARRAY)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    g_variant_get_data(value);
    n = g_variant_n_children(value);
    strv = ({
        let mut __n: gsize = n.wrapping_add(1 as gsize);
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
    i = 0 as gsize;
    while i < n {
        let mut string: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        string = g_variant_get_child_value(value, i);
        let ref mut fresh29 = *strv.offset(i as isize);
        *fresh29 = safe_c2rust_g_variant_dup_bytestring(string, ::core::ptr::null_mut::<gsize>());
        g_variant_unref(string);
        i = i.wrapping_add(1);
    }
    let ref mut fresh30 = *strv.offset(i as isize);
    *fresh30 = ::core::ptr::null_mut::<gchar>();
    if !length.is_null() {
        *length = n;
    }
    return strv;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_type(
    mut value: *mut GVariant,
) -> *const GVariantType {
    let mut type_info: *mut GVariantTypeInfo = ::core::ptr::null_mut::<GVariantTypeInfo>();
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<GVariantType>();
    }
    type_info = g_variant_get_type_info(value);
    return g_variant_type_info_get_type_string(type_info) as *mut GVariantType;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_type_string(
    mut value: *mut GVariant,
) -> *const gchar {
    let mut type_info: *mut GVariantTypeInfo = ::core::ptr::null_mut::<GVariantTypeInfo>();
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    type_info = g_variant_get_type_info(value);
    return g_variant_type_info_get_type_string(type_info);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_is_of_type(
    mut value: *mut GVariant,
    mut type_0: *const GVariantType,
) -> gboolean {
    return g_variant_type_is_subtype_of(safe_c2rust_g_variant_get_type(value), type_0);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_is_container(mut value: *mut GVariant) -> gboolean {
    return g_variant_type_is_container(safe_c2rust_g_variant_get_type(value));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_classify(mut value: *mut GVariant) -> GVariantClass {
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GVariantClass;
    }
    return *safe_c2rust_g_variant_get_type_string(value) as GVariantClass;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_print_string(
    mut value: *mut GVariant,
    mut string: *mut GString,
    mut type_annotate: gboolean,
) -> *mut GString {
    let mut value_type_string: *const gchar = safe_c2rust_g_variant_get_type_string(value);
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if string.is_null() {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
        string = g_string_new(::core::ptr::null::<gchar>());
    }
    let mut current_block_164: u64;
    match *value_type_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
        109 => {
            if type_annotate != 0 {
                g_string_append_printf(
                    string,
                    b"@%s \0" as *const u8 as *const gchar,
                    value_type_string,
                );
            }
            if g_variant_n_children(value) != 0 {
                let mut base_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
                let mut i: guint = 0;
                let mut depth: guint = 0;
                let mut element: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                depth = 0 as guint;
                base_type = safe_c2rust_g_variant_get_type(value);
                while g_variant_type_is_maybe(base_type) != 0 {
                    depth = depth.wrapping_add(1);
                    base_type = g_variant_type_element(base_type);
                }
                element = g_variant_ref(value);
                i = 0 as guint;
                while i < depth && !element.is_null() {
                    let mut new_element: *mut GVariant = if g_variant_n_children(element) != 0 {
                        g_variant_get_child_value(element, 0 as gsize)
                    } else {
                        ::core::ptr::null_mut::<GVariant>()
                    };
                    g_variant_unref(element);
                    element = safe_c2rust_g_steal_pointer(&raw mut new_element as gpointer)
                        as *mut GVariant as *mut GVariant;
                    i = i.wrapping_add(1);
                }
                if element.is_null() {
                    while i > 1 as guint {
                        if 0 != 0 {
                            ({
                                let __val: *const ::core::ffi::c_char =
                                    b"just \0" as *const u8 as *const ::core::ffi::c_char;
                                safe_c2rust_g_string_append_len_inline(
                                    string,
                                    __val,
                                    if ({
                                        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
                                        if !__val.is_null() {
                                            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
                                        } else {
                                            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
                                        }
                                        _g_boolean_var_68
                                    }) as ::core::ffi::c_long
                                        != 0
                                    {
                                        strlen(
                                            __val.offset(
                                                __val.is_null() as ::core::ffi::c_int as isize
                                            ),
                                        ) as gssize
                                    } else {
                                        -(1 as ::core::ffi::c_int) as gssize
                                    },
                                );
                            });
                        } else {
                            safe_c2rust_g_string_append_len_inline(
                                string,
                                b"just \0" as *const u8 as *const ::core::ffi::c_char,
                                -(1 as ::core::ffi::c_int) as gssize,
                            );
                        };
                        i = i.wrapping_sub(1);
                    }
                    if 0 != 0 {
                        ({
                            let __val: *const ::core::ffi::c_char =
                                b"nothing\0" as *const u8 as *const ::core::ffi::c_char;
                            safe_c2rust_g_string_append_len_inline(
                                string,
                                __val,
                                if ({
                                    let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
                                    if !__val.is_null() {
                                        _g_boolean_var_69 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_69 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_69
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                    strlen(
                                        __val
                                            .offset(__val.is_null() as ::core::ffi::c_int as isize),
                                    ) as gssize
                                } else {
                                    -(1 as ::core::ffi::c_int) as gssize
                                },
                            );
                        });
                    } else {
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            b"nothing\0" as *const u8 as *const ::core::ffi::c_char,
                            -(1 as ::core::ffi::c_int) as gssize,
                        );
                    };
                } else {
                    safe_c2rust_g_variant_print_string(element, string, FALSE);
                }
                let mut _pp: *mut *mut GVariant = &raw mut element;
                let mut _ptr: *mut GVariant = *_pp;
                *_pp = ::core::ptr::null_mut::<GVariant>();
                if !_ptr.is_null() {
                    g_variant_unref(_ptr as *mut GVariant);
                }
            } else {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"nothing\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_70 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_70 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_70
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"nothing\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
        }
        97 => {
            if *value_type_string.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 'y' as i32
            {
                let mut str: *const gchar = ::core::ptr::null::<gchar>();
                let mut size: gsize = 0;
                let mut i_0: gsize = 0;
                str = g_variant_get_data(value) as *const gchar;
                size = g_variant_get_size(value);
                i_0 = 0 as gsize;
                while i_0 < size {
                    if *str.offset(i_0 as isize) as ::core::ffi::c_int == '\0' as i32 {
                        break;
                    }
                    i_0 = i_0.wrapping_add(1);
                }
                if i_0 == size.wrapping_sub(1 as gsize) {
                    let mut escaped: *mut gchar = g_strescape(str, ::core::ptr::null::<gchar>());
                    if !strchr(str as *const ::core::ffi::c_char, '\'' as i32).is_null() {
                        g_string_append_printf(
                            string,
                            b"b\"%s\"\0" as *const u8 as *const gchar,
                            escaped,
                        );
                    } else {
                        g_string_append_printf(
                            string,
                            b"b'%s'\0" as *const u8 as *const gchar,
                            escaped,
                        );
                    }
                    g_free(escaped as gpointer);
                    current_block_164 = 15663981557480382173;
                } else {
                    current_block_164 = 17784502470059252271;
                }
            } else {
                current_block_164 = 17784502470059252271;
            }
            match current_block_164 {
                15663981557480382173 => {}
                _ => {
                    if *value_type_string.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '{' as i32
                    {
                        let mut comma: *const gchar = b"\0" as *const u8 as *const gchar;
                        let mut n: gsize = 0;
                        let mut i_1: gsize = 0;
                        n = g_variant_n_children(value);
                        if n == 0 as gsize {
                            if type_annotate != 0 {
                                g_string_append_printf(
                                    string,
                                    b"@%s \0" as *const u8 as *const gchar,
                                    value_type_string,
                                );
                            }
                            if 0 != 0 {
                                ({
                                    let __val: *const ::core::ffi::c_char =
                                        b"{}\0" as *const u8 as *const ::core::ffi::c_char;
                                    safe_c2rust_g_string_append_len_inline(
                                        string,
                                        __val,
                                        if ({
                                            let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
                                            if !__val.is_null() {
                                                _g_boolean_var_71 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_71 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_71
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                            strlen(__val.offset(__val.is_null()
                                                as ::core::ffi::c_int
                                                as isize))
                                                as gssize
                                        } else {
                                            -(1 as ::core::ffi::c_int) as gssize
                                        },
                                    );
                                });
                            } else {
                                safe_c2rust_g_string_append_len_inline(
                                    string,
                                    b"{}\0" as *const u8 as *const ::core::ffi::c_char,
                                    -(1 as ::core::ffi::c_int) as gssize,
                                );
                            };
                        } else {
                            safe_c2rust_g_string_append_c_inline(string, '{' as i32 as gchar);
                            i_1 = 0 as gsize;
                            while i_1 < n {
                                let mut entry: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                                let mut key: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                                let mut val: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                                if 0 != 0 {
                                    ({
                                        let __val: *const ::core::ffi::c_char =
                                            comma as *const ::core::ffi::c_char;
                                        safe_c2rust_g_string_append_len_inline(
                                            string,
                                            __val,
                                            if ({
                                                let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
                                                if !__val.is_null() {
                                                    _g_boolean_var_72 = 1 as ::core::ffi::c_int;
                                                } else {
                                                    _g_boolean_var_72 = 0 as ::core::ffi::c_int;
                                                }
                                                _g_boolean_var_72
                                            })
                                                as ::core::ffi::c_long
                                                != 0
                                            {
                                                strlen(
                                                    __val.offset(__val.is_null()
                                                        as ::core::ffi::c_int
                                                        as isize),
                                                )
                                                    as gssize
                                            } else {
                                                -(1 as ::core::ffi::c_int) as gssize
                                            },
                                        );
                                    });
                                } else {
                                    safe_c2rust_g_string_append_len_inline(
                                        string,
                                        comma as *const ::core::ffi::c_char,
                                        -(1 as ::core::ffi::c_int) as gssize,
                                    );
                                };
                                comma = b", \0" as *const u8 as *const ::core::ffi::c_char
                                    as *const gchar;
                                entry = g_variant_get_child_value(value, i_1);
                                key = g_variant_get_child_value(entry, 0 as gsize);
                                val = g_variant_get_child_value(entry, 1 as gsize);
                                g_variant_unref(entry);
                                safe_c2rust_g_variant_print_string(key, string, type_annotate);
                                g_variant_unref(key);
                                if 0 != 0 {
                                    ({
                                        let __val: *const ::core::ffi::c_char =
                                            b": \0" as *const u8 as *const ::core::ffi::c_char;
                                        safe_c2rust_g_string_append_len_inline(
                                            string,
                                            __val,
                                            if ({
                                                let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
                                                if !__val.is_null() {
                                                    _g_boolean_var_73 = 1 as ::core::ffi::c_int;
                                                } else {
                                                    _g_boolean_var_73 = 0 as ::core::ffi::c_int;
                                                }
                                                _g_boolean_var_73
                                            })
                                                as ::core::ffi::c_long
                                                != 0
                                            {
                                                strlen(
                                                    __val.offset(__val.is_null()
                                                        as ::core::ffi::c_int
                                                        as isize),
                                                )
                                                    as gssize
                                            } else {
                                                -(1 as ::core::ffi::c_int) as gssize
                                            },
                                        );
                                    });
                                } else {
                                    safe_c2rust_g_string_append_len_inline(
                                        string,
                                        b": \0" as *const u8 as *const ::core::ffi::c_char,
                                        -(1 as ::core::ffi::c_int) as gssize,
                                    );
                                };
                                safe_c2rust_g_variant_print_string(val, string, type_annotate);
                                g_variant_unref(val);
                                type_annotate = FALSE as gboolean;
                                i_1 = i_1.wrapping_add(1);
                            }
                            safe_c2rust_g_string_append_c_inline(string, '}' as i32 as gchar);
                        }
                    } else {
                        let mut comma_0: *const gchar = b"\0" as *const u8 as *const gchar;
                        let mut n_0: gsize = 0;
                        let mut i_2: gsize = 0;
                        n_0 = g_variant_n_children(value);
                        if n_0 == 0 as gsize {
                            if type_annotate != 0 {
                                g_string_append_printf(
                                    string,
                                    b"@%s \0" as *const u8 as *const gchar,
                                    value_type_string,
                                );
                            }
                            if 0 != 0 {
                                ({
                                    let __val: *const ::core::ffi::c_char =
                                        b"[]\0" as *const u8 as *const ::core::ffi::c_char;
                                    safe_c2rust_g_string_append_len_inline(
                                        string,
                                        __val,
                                        if ({
                                            let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
                                            if !__val.is_null() {
                                                _g_boolean_var_74 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_74 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_74
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                            strlen(__val.offset(__val.is_null()
                                                as ::core::ffi::c_int
                                                as isize))
                                                as gssize
                                        } else {
                                            -(1 as ::core::ffi::c_int) as gssize
                                        },
                                    );
                                });
                            } else {
                                safe_c2rust_g_string_append_len_inline(
                                    string,
                                    b"[]\0" as *const u8 as *const ::core::ffi::c_char,
                                    -(1 as ::core::ffi::c_int) as gssize,
                                );
                            };
                        } else {
                            safe_c2rust_g_string_append_c_inline(string, '[' as i32 as gchar);
                            i_2 = 0 as gsize;
                            while i_2 < n_0 {
                                let mut element_0: *mut GVariant =
                                    ::core::ptr::null_mut::<GVariant>();
                                if 0 != 0 {
                                    ({
                                        let __val: *const ::core::ffi::c_char =
                                            comma_0 as *const ::core::ffi::c_char;
                                        safe_c2rust_g_string_append_len_inline(
                                            string,
                                            __val,
                                            if ({
                                                let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
                                                if !__val.is_null() {
                                                    _g_boolean_var_75 = 1 as ::core::ffi::c_int;
                                                } else {
                                                    _g_boolean_var_75 = 0 as ::core::ffi::c_int;
                                                }
                                                _g_boolean_var_75
                                            })
                                                as ::core::ffi::c_long
                                                != 0
                                            {
                                                strlen(
                                                    __val.offset(__val.is_null()
                                                        as ::core::ffi::c_int
                                                        as isize),
                                                )
                                                    as gssize
                                            } else {
                                                -(1 as ::core::ffi::c_int) as gssize
                                            },
                                        );
                                    });
                                } else {
                                    safe_c2rust_g_string_append_len_inline(
                                        string,
                                        comma_0 as *const ::core::ffi::c_char,
                                        -(1 as ::core::ffi::c_int) as gssize,
                                    );
                                };
                                comma_0 = b", \0" as *const u8 as *const ::core::ffi::c_char
                                    as *const gchar;
                                element_0 = g_variant_get_child_value(value, i_2);
                                safe_c2rust_g_variant_print_string(
                                    element_0,
                                    string,
                                    type_annotate,
                                );
                                g_variant_unref(element_0);
                                type_annotate = FALSE as gboolean;
                                i_2 = i_2.wrapping_add(1);
                            }
                            safe_c2rust_g_string_append_c_inline(string, ']' as i32 as gchar);
                        }
                    }
                }
            }
        }
        40 => {
            let mut n_1: gsize = 0;
            let mut i_3: gsize = 0;
            n_1 = g_variant_n_children(value);
            safe_c2rust_g_string_append_c_inline(string, '(' as i32 as gchar);
            i_3 = 0 as gsize;
            while i_3 < n_1 {
                let mut element_1: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                element_1 = g_variant_get_child_value(value, i_3);
                safe_c2rust_g_variant_print_string(element_1, string, type_annotate);
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b", \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_76 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_76 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_76
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b", \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
                g_variant_unref(element_1);
                i_3 = i_3.wrapping_add(1);
            }
            safe_c2rust_g_string_truncate_inline(
                string,
                (*string)
                    .len
                    .wrapping_sub((n_1 > 0 as gsize) as ::core::ffi::c_int as gsize)
                    .wrapping_sub((n_1 > 1 as gsize) as ::core::ffi::c_int as gsize),
            );
            safe_c2rust_g_string_append_c_inline(string, ')' as i32 as gchar);
        }
        123 => {
            let mut element_2: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            safe_c2rust_g_string_append_c_inline(string, '{' as i32 as gchar);
            element_2 = g_variant_get_child_value(value, 0 as gsize);
            safe_c2rust_g_variant_print_string(element_2, string, type_annotate);
            g_variant_unref(element_2);
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        b", \0" as *const u8 as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        __val,
                        if ({
                            let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_77 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_77 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_77
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as ::core::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                safe_c2rust_g_string_append_len_inline(
                    string,
                    b", \0" as *const u8 as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
            element_2 = g_variant_get_child_value(value, 1 as gsize);
            safe_c2rust_g_variant_print_string(element_2, string, type_annotate);
            g_variant_unref(element_2);
            safe_c2rust_g_string_append_c_inline(string, '}' as i32 as gchar);
        }
        118 => {
            let mut child: *mut GVariant = safe_c2rust_g_variant_get_variant(value);
            safe_c2rust_g_string_append_c_inline(string, '<' as i32 as gchar);
            safe_c2rust_g_variant_print_string(child, string, TRUE);
            safe_c2rust_g_string_append_c_inline(string, '>' as i32 as gchar);
            g_variant_unref(child);
        }
        98 => {
            if safe_c2rust_g_variant_get_boolean(value) != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"true\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_78 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_78 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_78
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"true\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            } else {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"false\0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_79 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_79 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_79
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"false\0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
        }
        115 => {
            let mut str_0: *const gchar =
                safe_c2rust_g_variant_get_string(value, ::core::ptr::null_mut::<gsize>());
            let mut quote: gunichar =
                (if !strchr(str_0 as *const ::core::ffi::c_char, '\'' as i32).is_null() {
                    '"' as i32
                } else {
                    '\'' as i32
                }) as gunichar;
            safe_c2rust_g_string_append_c_inline(string, quote as gchar);
            while *str_0 != 0 {
                let mut c: gunichar = g_utf8_get_char(str_0);
                if c == quote || c == '\\' as i32 as gunichar {
                    safe_c2rust_g_string_append_c_inline(string, '\\' as i32 as gchar);
                }
                if g_unichar_isprint(c) != 0 {
                    g_string_append_unichar(string, c);
                } else {
                    safe_c2rust_g_string_append_c_inline(string, '\\' as i32 as gchar);
                    if c < 0x10000 as ::core::ffi::c_int as gunichar {
                        match c {
                            7 => {
                                safe_c2rust_g_string_append_c_inline(string, 'a' as i32 as gchar);
                            }
                            8 => {
                                safe_c2rust_g_string_append_c_inline(string, 'b' as i32 as gchar);
                            }
                            12 => {
                                safe_c2rust_g_string_append_c_inline(string, 'f' as i32 as gchar);
                            }
                            10 => {
                                safe_c2rust_g_string_append_c_inline(string, 'n' as i32 as gchar);
                            }
                            13 => {
                                safe_c2rust_g_string_append_c_inline(string, 'r' as i32 as gchar);
                            }
                            9 => {
                                safe_c2rust_g_string_append_c_inline(string, 't' as i32 as gchar);
                            }
                            11 => {
                                safe_c2rust_g_string_append_c_inline(string, 'v' as i32 as gchar);
                            }
                            _ => {
                                g_string_append_printf(
                                    string,
                                    b"u%04x\0" as *const u8 as *const gchar,
                                    c,
                                );
                            }
                        }
                    } else {
                        g_string_append_printf(string, b"U%08x\0" as *const u8 as *const gchar, c);
                    }
                }
                str_0 = str_0.offset(
                    *safe_c2rust_g_utf8_skip.offset(*(str_0 as *const guchar) as isize)
                        as ::core::ffi::c_int as isize,
                ) as *mut ::core::ffi::c_char;
            }
            safe_c2rust_g_string_append_c_inline(string, quote as gchar);
        }
        121 => {
            if type_annotate != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"byte \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_80 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_80 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_80
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"byte \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            g_string_append_printf(
                string,
                b"0x%02x\0" as *const u8 as *const gchar,
                safe_c2rust_g_variant_get_byte(value) as ::core::ffi::c_int,
            );
        }
        110 => {
            if type_annotate != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"int16 \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_81 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_81 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_81
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"int16 \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            g_string_append_printf(
                string,
                b"%hi\0" as *const u8 as *const gchar,
                safe_c2rust_g_variant_get_int16(value) as ::core::ffi::c_int,
            );
        }
        113 => {
            if type_annotate != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"uint16 \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_82 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_82 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_82
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"uint16 \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            g_string_append_printf(
                string,
                b"%hu\0" as *const u8 as *const gchar,
                safe_c2rust_g_variant_get_uint16(value) as ::core::ffi::c_int,
            );
        }
        105 => {
            g_string_append_printf(
                string,
                b"%i\0" as *const u8 as *const gchar,
                safe_c2rust_g_variant_get_int32(value),
            );
        }
        104 => {
            if type_annotate != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"handle \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_83 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_83 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_83
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"handle \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            g_string_append_printf(
                string,
                b"%i\0" as *const u8 as *const gchar,
                safe_c2rust_g_variant_get_handle(value),
            );
        }
        117 => {
            if type_annotate != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"uint32 \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_84 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_84 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_84
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"uint32 \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            g_string_append_printf(
                string,
                b"%u\0" as *const u8 as *const gchar,
                safe_c2rust_g_variant_get_uint32(value),
            );
        }
        120 => {
            if type_annotate != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"int64 \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_85 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_85 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_85
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"int64 \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            g_string_append_printf(
                string,
                b"%li\0" as *const u8 as *const gchar,
                safe_c2rust_g_variant_get_int64(value),
            );
        }
        116 => {
            if type_annotate != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"uint64 \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_86 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_86 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_86
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"uint64 \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            g_string_append_printf(
                string,
                b"%lu\0" as *const u8 as *const gchar,
                safe_c2rust_g_variant_get_uint64(value),
            );
        }
        100 => {
            let mut buffer: [gchar; 100] = [0; 100];
            let mut i_4: gint = 0;
            g_ascii_dtostr(
                &raw mut buffer as *mut gchar,
                ::core::mem::size_of::<[gchar; 100]>() as gint,
                safe_c2rust_g_variant_get_double(value),
            );
            i_4 = 0 as ::core::ffi::c_int as gint;
            while buffer[i_4 as usize] != 0 {
                if buffer[i_4 as usize] as ::core::ffi::c_int == '.' as i32
                    || buffer[i_4 as usize] as ::core::ffi::c_int == 'e' as i32
                    || buffer[i_4 as usize] as ::core::ffi::c_int == 'n' as i32
                    || buffer[i_4 as usize] as ::core::ffi::c_int == 'N' as i32
                {
                    break;
                }
                i_4 += 1;
            }
            if buffer[i_4 as usize] as ::core::ffi::c_int == '\0' as i32 {
                let fresh46 = i_4;
                i_4 = i_4 + 1;
                buffer[fresh46 as usize] = '.' as i32 as gchar;
                let fresh47 = i_4;
                i_4 = i_4 + 1;
                buffer[fresh47 as usize] = '0' as i32 as gchar;
                let fresh48 = i_4;
                i_4 = i_4 + 1;
                buffer[fresh48 as usize] = '\0' as i32 as gchar;
            }
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = &raw mut buffer as *mut gchar;
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        __val,
                        if ({
                            let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_87 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_87 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_87
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                as gssize
                        } else {
                            -(1 as ::core::ffi::c_int) as gssize
                        },
                    );
                });
            } else {
                safe_c2rust_g_string_append_len_inline(
                    string,
                    &raw mut buffer as *mut gchar,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
        111 => {
            if type_annotate != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"objectpath \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_88 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_88 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_88
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"objectpath \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            g_string_append_printf(
                string,
                b"'%s'\0" as *const u8 as *const gchar,
                safe_c2rust_g_variant_get_string(value, ::core::ptr::null_mut::<gsize>()),
            );
        }
        103 => {
            if type_annotate != 0 {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            b"signature \0" as *const u8 as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            string,
                            __val,
                            if ({
                                let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_89 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_89 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_89
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize))
                                    as gssize
                            } else {
                                -(1 as ::core::ffi::c_int) as gssize
                            },
                        );
                    });
                } else {
                    safe_c2rust_g_string_append_len_inline(
                        string,
                        b"signature \0" as *const u8 as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            }
            g_string_append_printf(
                string,
                b"'%s'\0" as *const u8 as *const gchar,
                safe_c2rust_g_variant_get_string(value, ::core::ptr::null_mut::<gsize>()),
            );
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                2630 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_print(
    mut value: *mut GVariant,
    mut type_annotate: gboolean,
) -> *mut gchar {
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(
                safe_c2rust_g_variant_print_string(
                    value,
                    ::core::ptr::null_mut::<GString>(),
                    type_annotate,
                ),
                0 as gboolean,
            )
        } else {
            g_string_free_and_steal(safe_c2rust_g_variant_print_string(
                value,
                ::core::ptr::null_mut::<GString>(),
                type_annotate,
            ))
        }
    } else {
        g_string_free(
            safe_c2rust_g_variant_print_string(
                value,
                ::core::ptr::null_mut::<GString>(),
                type_annotate,
            ),
            0 as gboolean,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_hash(mut value_: gconstpointer) -> guint {
    let mut value: *mut GVariant = value_ as *mut GVariant;
    match safe_c2rust_g_variant_classify(value) as ::core::ffi::c_uint {
        115 | 111 | 103 => {
            return g_str_hash(safe_c2rust_g_variant_get_string(
                value,
                ::core::ptr::null_mut::<gsize>(),
            ) as gconstpointer);
        }
        98 => return safe_c2rust_g_variant_get_boolean(value) as guint,
        121 => return safe_c2rust_g_variant_get_byte(value) as guint,
        110 | 113 => {
            let mut ptr: *const guint16 = ::core::ptr::null::<guint16>();
            ptr = g_variant_get_data(value) as *const guint16;
            if !ptr.is_null() {
                return *ptr as guint;
            } else {
                return 0 as guint;
            }
        }
        105 | 117 | 104 => {
            let mut ptr_0: *const guint = ::core::ptr::null::<guint>();
            ptr_0 = g_variant_get_data(value) as *const guint;
            if !ptr_0.is_null() {
                return *ptr_0;
            } else {
                return 0 as guint;
            }
        }
        120 | 116 | 100 => {
            let mut ptr_1: *const guint = ::core::ptr::null::<guint>();
            ptr_1 = g_variant_get_data(value) as *const guint;
            if !ptr_1.is_null() {
                return (*ptr_1.offset(0 as ::core::ffi::c_int as isize))
                    .wrapping_add(*ptr_1.offset(1 as ::core::ffi::c_int as isize));
            } else {
                return 0 as guint;
            }
        }
        _ => {
            if ({
                let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
                if safe_c2rust_g_variant_is_container(value) == 0 {
                    _g_boolean_var_90 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_90 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_90
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_return_if_fail_warning(
                    G_LOG_DOMAIN.as_ptr(),
                    G_STRFUNC,
                    b"!g_variant_is_container (value)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return 0 as guint;
            }
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                2745 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_equal(
    mut one: gconstpointer,
    mut two: gconstpointer,
) -> gboolean {
    let mut equal: gboolean = 0;
    if ({
        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
        if !one.is_null() && !two.is_null() {
            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_91
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"one != NULL && two != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if g_variant_get_type_info(one as *mut GVariant)
        != g_variant_get_type_info(two as *mut GVariant)
    {
        return FALSE;
    }
    if g_variant_is_trusted(one as *mut GVariant) != 0
        && g_variant_is_trusted(two as *mut GVariant) != 0
    {
        let mut data_one: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
        let mut data_two: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
        let mut size_one: gsize = 0;
        let mut size_two: gsize = 0;
        size_one = g_variant_get_size(one as *mut GVariant);
        size_two = g_variant_get_size(two as *mut GVariant);
        if size_one != size_two {
            return FALSE;
        }
        data_one = g_variant_get_data(one as *mut GVariant);
        data_two = g_variant_get_data(two as *mut GVariant);
        if size_one != 0 {
            equal = (memcmp(
                data_one as *const ::core::ffi::c_void,
                data_two as *const ::core::ffi::c_void,
                size_one as size_t,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
        } else {
            equal = TRUE as gboolean;
        }
    } else {
        let mut strone: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut strtwo: *mut gchar = ::core::ptr::null_mut::<gchar>();
        strone = safe_c2rust_g_variant_print(one as *mut GVariant, FALSE);
        strtwo = safe_c2rust_g_variant_print(two as *mut GVariant, FALSE);
        equal =
            (strcmp(strone, strtwo) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
        g_free(strone as gpointer);
        g_free(strtwo as gpointer);
    }
    return equal;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_compare(
    mut one: gconstpointer,
    mut two: gconstpointer,
) -> gint {
    let mut a: *mut GVariant = one as *mut GVariant;
    let mut b: *mut GVariant = two as *mut GVariant;
    if ({
        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_classify(a) as ::core::ffi::c_uint
            == safe_c2rust_g_variant_classify(b) as ::core::ffi::c_uint
        {
            _g_boolean_var_92 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_92 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_92
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_classify (a) == g_variant_classify (b)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    match safe_c2rust_g_variant_classify(a) as ::core::ffi::c_uint {
        98 => {
            return safe_c2rust_g_variant_get_boolean(a) as gint
                - safe_c2rust_g_variant_get_boolean(b) as gint;
        }
        121 => {
            return safe_c2rust_g_variant_get_byte(a) as gint
                - safe_c2rust_g_variant_get_byte(b) as gint;
        }
        110 => {
            return safe_c2rust_g_variant_get_int16(a) as gint
                - safe_c2rust_g_variant_get_int16(b) as gint;
        }
        113 => {
            return safe_c2rust_g_variant_get_uint16(a) as gint
                - safe_c2rust_g_variant_get_uint16(b) as gint;
        }
        105 => {
            let mut a_val: gint32 = safe_c2rust_g_variant_get_int32(a);
            let mut b_val: gint32 = safe_c2rust_g_variant_get_int32(b);
            return if a_val == b_val {
                0 as gint
            } else if a_val > b_val {
                1 as gint
            } else {
                -(1 as gint)
            };
        }
        117 => {
            let mut a_val_0: guint32 = safe_c2rust_g_variant_get_uint32(a);
            let mut b_val_0: guint32 = safe_c2rust_g_variant_get_uint32(b);
            return if a_val_0 == b_val_0 {
                0 as gint
            } else if a_val_0 > b_val_0 {
                1 as gint
            } else {
                -(1 as gint)
            };
        }
        120 => {
            let mut a_val_1: gint64 = safe_c2rust_g_variant_get_int64(a);
            let mut b_val_1: gint64 = safe_c2rust_g_variant_get_int64(b);
            return if a_val_1 == b_val_1 {
                0 as gint
            } else if a_val_1 > b_val_1 {
                1 as gint
            } else {
                -(1 as gint)
            };
        }
        116 => {
            let mut a_val_2: guint64 = safe_c2rust_g_variant_get_uint64(a);
            let mut b_val_2: guint64 = safe_c2rust_g_variant_get_uint64(b);
            return if a_val_2 == b_val_2 {
                0 as gint
            } else if a_val_2 > b_val_2 {
                1 as gint
            } else {
                -(1 as gint)
            };
        }
        100 => {
            let mut a_val_3: gdouble = safe_c2rust_g_variant_get_double(a);
            let mut b_val_3: gdouble = safe_c2rust_g_variant_get_double(b);
            return if a_val_3 == b_val_3 {
                0 as gint
            } else if a_val_3 > b_val_3 {
                1 as gint
            } else {
                -(1 as gint)
            };
        }
        115 | 111 | 103 => {
            return strcmp(
                safe_c2rust_g_variant_get_string(a, ::core::ptr::null_mut::<gsize>())
                    as *const ::core::ffi::c_char,
                safe_c2rust_g_variant_get_string(b, ::core::ptr::null_mut::<gsize>())
                    as *const ::core::ffi::c_char,
            ) as gint;
        }
        _ => {
            if ({
                let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
                if safe_c2rust_g_variant_is_container(a) == 0 {
                    _g_boolean_var_93 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_93 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_93
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_return_if_fail_warning(
                    G_LOG_DOMAIN.as_ptr(),
                    G_STRFUNC,
                    b"!g_variant_is_container (a)\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return 0 as gint;
            }
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                2924 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
pub const GVSI_MAGIC: gsize = 3579507750 as ::core::ffi::c_uint as gsize;
pub const GVHI_MAGIC: gsize = 1450270775 as ::core::ffi::c_uint as gsize;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_iter_new(
    mut value: *mut GVariant,
) -> *mut GVariantIter {
    let mut iter: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    iter = g_slice_alloc(::core::mem::size_of::<heap_iter>() as gsize) as *mut heap_iter
        as *mut GVariantIter;
    let ref mut fresh42 = (*(iter as *mut heap_iter)).value_ref;
    *fresh42 = g_variant_ref(value);
    (*(iter as *mut heap_iter)).magic = GVHI_MAGIC;
    safe_c2rust_g_variant_iter_init(iter, value);
    return iter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_iter_init(
    mut iter: *mut GVariantIter,
    mut value: *mut GVariant,
) -> gsize {
    (*(iter as *mut stack_iter)).magic = GVSI_MAGIC;
    let ref mut fresh43 = (*(iter as *mut stack_iter)).value;
    *fresh43 = value;
    (*(iter as *mut stack_iter)).n = g_variant_n_children(value) as gssize;
    (*(iter as *mut stack_iter)).i = -(1 as ::core::ffi::c_int) as gssize;
    let ref mut fresh44 = (*(iter as *mut stack_iter)).loop_format;
    *fresh44 = ::core::ptr::null::<gchar>();
    return (*(iter as *mut stack_iter)).n as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_iter_copy(
    mut iter: *mut GVariantIter,
) -> *mut GVariantIter {
    let mut copy: *mut GVariantIter = ::core::ptr::null_mut::<GVariantIter>();
    if ({
        let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
        if !iter.is_null()
            && (*(iter as *mut stack_iter)).magic == 3579507750 as ::core::ffi::c_uint as gsize
        {
            _g_boolean_var_94 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_94 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_94
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_iter (iter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantIter>();
    }
    copy = safe_c2rust_g_variant_iter_new((*(iter as *mut stack_iter)).value);
    (*(copy as *mut stack_iter)).i = (*(iter as *mut stack_iter)).i;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_iter_n_children(
    mut iter: *mut GVariantIter,
) -> gsize {
    if ({
        let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
        if !iter.is_null()
            && (*(iter as *mut stack_iter)).magic == 3579507750 as ::core::ffi::c_uint as gsize
        {
            _g_boolean_var_95 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_95 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_95
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_iter (iter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return (*(iter as *mut stack_iter)).n as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_iter_free(mut iter: *mut GVariantIter) {
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if !iter.is_null()
            && (*(iter as *mut stack_iter)).magic == 3579507750 as ::core::ffi::c_uint as gsize
            && (*(iter as *mut heap_iter)).magic == 1450270775 as ::core::ffi::c_uint as gsize
        {
            _g_boolean_var_96 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_96 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_96
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_heap_iter (iter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_variant_unref((*(iter as *mut heap_iter)).value_ref);
    (*(iter as *mut heap_iter)).magic = 0 as gsize;
    g_slice_free1(
        ::core::mem::size_of::<heap_iter>() as gsize,
        iter as *mut heap_iter as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_iter_next_value(
    mut iter: *mut GVariantIter,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if !iter.is_null()
            && (*(iter as *mut stack_iter)).magic == 3579507750 as ::core::ffi::c_uint as gsize
        {
            _g_boolean_var_97 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_97 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_97
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_iter (iter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if (*(iter as *mut stack_iter)).i >= (*(iter as *mut stack_iter)).n {
            _g_boolean_var_98 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_98 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_98
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"g_variant_iter_next_value: must not be called again after NULL has already been returned.\0"
                as *const u8 as *const gchar,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    let ref mut fresh45 = (*(iter as *mut stack_iter)).i;
    *fresh45 += 1;
    if (*(iter as *mut stack_iter)).i < (*(iter as *mut stack_iter)).n {
        return g_variant_get_child_value(
            (*(iter as *mut stack_iter)).value,
            (*(iter as *mut stack_iter)).i as gsize,
        );
    }
    return ::core::ptr::null_mut::<GVariant>();
}
pub const GVSB_MAGIC: gsize = 1033660112 as ::core::ffi::c_uint as gsize;
pub const GVSB_MAGIC_PARTIAL: gsize = 2942751021 as ::core::ffi::c_uint as gsize;
pub const GVHB_MAGIC: gsize = 3087242682 as ::core::ffi::c_uint as gsize;
unsafe extern "C" fn safe_c2rust_ensure_valid_builder(
    mut builder: *mut GVariantBuilder,
) -> gboolean {
    if builder.is_null() {
        return FALSE;
    } else if (*(builder as *mut stack_builder)).magic == GVSB_MAGIC {
        return TRUE;
    }
    if (*builder).u.s.partial_magic == GVSB_MAGIC_PARTIAL {
        static mut safe_c2rust_cleared_builder: GVariantBuilder = _GVariantBuilder {
            u: C2RustUnnamed_0 {
                s: C2RustUnnamed_1 {
                    partial_magic: 0,
                    type_0: ::core::ptr::null::<GVariantType>(),
                    y: [0; 14],
                },
            },
        };
        if memcmp(
            &raw mut safe_c2rust_cleared_builder.u.s.y as *mut guintptr
                as *const ::core::ffi::c_void,
            &raw mut (*builder).u.s.y as *mut guintptr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[guintptr; 14]>() as size_t,
        ) != 0
        {
            return FALSE;
        }
        safe_c2rust_g_variant_builder_init(builder, (*builder).u.s.type_0);
    }
    return ((*(builder as *mut stack_builder)).magic == GVSB_MAGIC) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_new(
    mut type_0: *const GVariantType,
) -> *mut GVariantBuilder {
    let mut builder: *mut GVariantBuilder = ::core::ptr::null_mut::<GVariantBuilder>();
    builder = g_slice_alloc(::core::mem::size_of::<heap_builder>() as gsize) as *mut heap_builder
        as *mut GVariantBuilder;
    safe_c2rust_g_variant_builder_init(builder, type_0);
    (*(builder as *mut heap_builder)).magic = GVHB_MAGIC;
    (*(builder as *mut heap_builder)).ref_count = 1 as ::core::ffi::c_int as gint;
    return builder;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_unref(mut builder: *mut GVariantBuilder) {
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if (*(builder as *mut heap_builder)).magic == 3087242682 as ::core::ffi::c_uint as gsize {
            _g_boolean_var_99 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_99 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_99
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_heap_builder (builder)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let ref mut fresh91 = (*(builder as *mut heap_builder)).ref_count;
    *fresh91 -= 1;
    if *fresh91 != 0 {
        return;
    }
    safe_c2rust_g_variant_builder_clear(builder);
    (*(builder as *mut heap_builder)).magic = 0 as gsize;
    g_slice_free1(
        ::core::mem::size_of::<heap_builder>() as gsize,
        builder as *mut heap_builder as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_ref(
    mut builder: *mut GVariantBuilder,
) -> *mut GVariantBuilder {
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if (*(builder as *mut heap_builder)).magic == 3087242682 as ::core::ffi::c_uint as gsize {
            _g_boolean_var_100 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_100 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_100
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_heap_builder (builder)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantBuilder>();
    }
    let ref mut fresh92 = (*(builder as *mut heap_builder)).ref_count;
    *fresh92 += 1;
    return builder;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_clear(mut builder: *mut GVariantBuilder) {
    let mut i: gsize = 0;
    if (*(builder as *mut stack_builder)).magic == 0 as gsize {
        return;
    }
    let mut valid_builder: gboolean = safe_c2rust_ensure_valid_builder(builder);
    if ({
        let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
        if valid_builder != 0 {
            _g_boolean_var_101 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_101 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_101
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_builder\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_variant_type_free((*(builder as *mut stack_builder)).type_0);
    i = 0 as gsize;
    while i < (*(builder as *mut stack_builder)).offset {
        g_variant_unref(
            *(*(builder as *mut stack_builder))
                .children
                .offset(i as isize),
        );
        i = i.wrapping_add(1);
    }
    g_free((*(builder as *mut stack_builder)).children as gpointer);
    if !(*(builder as *mut stack_builder)).parent.is_null() {
        safe_c2rust_g_variant_builder_clear((*(builder as *mut stack_builder)).parent);
        g_slice_free1(
            ::core::mem::size_of::<GVariantBuilder>() as gsize,
            (*(builder as *mut stack_builder)).parent as gpointer,
        );
    }
    memset(
        builder as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GVariantBuilder>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_init(
    mut builder: *mut GVariantBuilder,
    mut type_0: *const GVariantType,
) {
    if ({
        let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
        if !type_0.is_null() {
            _g_boolean_var_102 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_102 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_102
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
        if g_variant_type_is_container(type_0) != 0 {
            _g_boolean_var_103 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_103 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_103
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_type_is_container (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    memset(
        builder as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GVariantBuilder>() as size_t,
    );
    let ref mut fresh68 = (*(builder as *mut stack_builder)).type_0;
    *fresh68 = g_variant_type_copy(type_0);
    (*(builder as *mut stack_builder)).magic = GVSB_MAGIC;
    let ref mut fresh69 = *(builder as *mut stack_builder);
    (*fresh69).set_trusted(TRUE as guint as guint);
    match *(type_0 as *const gchar) as ::core::ffi::c_int {
        118 => {
            let ref mut fresh70 = *(builder as *mut stack_builder);
            (*fresh70).set_uniform_item_types(TRUE as guint as guint);
            (*(builder as *mut stack_builder)).allocated_children = 1 as gsize;
            let ref mut fresh71 = (*(builder as *mut stack_builder)).expected_type;
            *fresh71 = ::core::ptr::null::<GVariantType>();
            (*(builder as *mut stack_builder)).min_items = 1 as gsize;
            (*(builder as *mut stack_builder)).max_items = 1 as gsize;
        }
        97 => {
            let ref mut fresh72 = *(builder as *mut stack_builder);
            (*fresh72).set_uniform_item_types(TRUE as guint as guint);
            (*(builder as *mut stack_builder)).allocated_children = 8 as gsize;
            let ref mut fresh73 = (*(builder as *mut stack_builder)).expected_type;
            *fresh73 = g_variant_type_element((*(builder as *mut stack_builder)).type_0);
            (*(builder as *mut stack_builder)).min_items = 0 as gsize;
            (*(builder as *mut stack_builder)).max_items = -(1 as ::core::ffi::c_int) as gsize;
        }
        109 => {
            let ref mut fresh74 = *(builder as *mut stack_builder);
            (*fresh74).set_uniform_item_types(TRUE as guint as guint);
            (*(builder as *mut stack_builder)).allocated_children = 1 as gsize;
            let ref mut fresh75 = (*(builder as *mut stack_builder)).expected_type;
            *fresh75 = g_variant_type_element((*(builder as *mut stack_builder)).type_0);
            (*(builder as *mut stack_builder)).min_items = 0 as gsize;
            (*(builder as *mut stack_builder)).max_items = 1 as gsize;
        }
        123 => {
            let ref mut fresh76 = *(builder as *mut stack_builder);
            (*fresh76).set_uniform_item_types(FALSE as guint as guint);
            (*(builder as *mut stack_builder)).allocated_children = 2 as gsize;
            let ref mut fresh77 = (*(builder as *mut stack_builder)).expected_type;
            *fresh77 = g_variant_type_key((*(builder as *mut stack_builder)).type_0);
            (*(builder as *mut stack_builder)).min_items = 2 as gsize;
            (*(builder as *mut stack_builder)).max_items = 2 as gsize;
        }
        114 => {
            let ref mut fresh78 = *(builder as *mut stack_builder);
            (*fresh78).set_uniform_item_types(FALSE as guint as guint);
            (*(builder as *mut stack_builder)).allocated_children = 8 as gsize;
            let ref mut fresh79 = (*(builder as *mut stack_builder)).expected_type;
            *fresh79 = ::core::ptr::null::<GVariantType>();
            (*(builder as *mut stack_builder)).min_items = 0 as gsize;
            (*(builder as *mut stack_builder)).max_items = -(1 as ::core::ffi::c_int) as gsize;
        }
        40 => {
            (*(builder as *mut stack_builder)).allocated_children = g_variant_type_n_items(type_0);
            let ref mut fresh80 = (*(builder as *mut stack_builder)).expected_type;
            *fresh80 = g_variant_type_first((*(builder as *mut stack_builder)).type_0);
            (*(builder as *mut stack_builder)).min_items =
                (*(builder as *mut stack_builder)).allocated_children;
            (*(builder as *mut stack_builder)).max_items =
                (*(builder as *mut stack_builder)).allocated_children;
            let ref mut fresh81 = *(builder as *mut stack_builder);
            (*fresh81).set_uniform_item_types(FALSE as guint as guint);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                3504 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    let ref mut fresh82 = (*(builder as *mut stack_builder)).children;
    *fresh82 = ({
        let mut __n: gsize = (*(builder as *mut stack_builder)).allocated_children;
        let mut __s: gsize = ::core::mem::size_of::<*mut GVariant>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut *mut GVariant;
}
unsafe extern "C" fn safe_c2rust_g_variant_builder_make_room(mut builder: *mut stack_builder) {
    if (*builder).offset == (*builder).allocated_children {
        (*builder).allocated_children = (*builder).allocated_children.wrapping_mul(2 as gsize);
        (*builder).children = ({
            let mut __n: gsize = (*builder).allocated_children;
            let mut __s: gsize = ::core::mem::size_of::<*mut GVariant>() as gsize;
            let mut __p: gpointer = (*builder).children as gpointer;
            if __s == 1 as gsize {
                __p = g_realloc(__p, __n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_realloc(__p, __n.wrapping_mul(__s));
            } else {
                __p = g_realloc_n(__p, __n, __s);
            }
            __p
        }) as *mut *mut GVariant;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_add_value(
    mut builder: *mut GVariantBuilder,
    mut value: *mut GVariant,
) {
    let mut valid_builder: gboolean = safe_c2rust_ensure_valid_builder(builder);
    if ({
        let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
        if valid_builder != 0 {
            _g_boolean_var_104 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_104 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_104
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_builder\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_105: ::core::ffi::c_int = 0;
        if (*(builder as *mut stack_builder)).offset < (*(builder as *mut stack_builder)).max_items
        {
            _g_boolean_var_105 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_105 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_105
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"GVSB(builder)->offset < GVSB(builder)->max_items\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_106: ::core::ffi::c_int = 0;
        if (*(builder as *mut stack_builder)).expected_type.is_null()
            || safe_c2rust_g_variant_is_of_type(
                value,
                (*(builder as *mut stack_builder)).expected_type,
            ) != 0
        {
            _g_boolean_var_106 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_106 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_106
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!GVSB(builder)->expected_type || g_variant_is_of_type (value, GVSB(builder)->expected_type)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_107: ::core::ffi::c_int = 0;
        if (*(builder as *mut stack_builder)).prev_item_type.is_null()
            || safe_c2rust_g_variant_is_of_type(
                value,
                (*(builder as *mut stack_builder)).prev_item_type,
            ) != 0
        {
            _g_boolean_var_107 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_107 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_107
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!GVSB(builder)->prev_item_type || g_variant_is_of_type (value, GVSB(builder)->prev_item_type)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let ref mut fresh83 = *(builder as *mut stack_builder);
    (*fresh83).set_trusted(
        (*fresh83).trusted() & g_variant_is_trusted(value) as ::core::ffi::c_int as guint,
    );
    if (*(builder as *mut stack_builder)).uniform_item_types() == 0 {
        if !(*(builder as *mut stack_builder)).expected_type.is_null() {
            let ref mut fresh84 = (*(builder as *mut stack_builder)).expected_type;
            *fresh84 = g_variant_type_next((*(builder as *mut stack_builder)).expected_type);
        }
        if !(*(builder as *mut stack_builder)).prev_item_type.is_null() {
            let ref mut fresh85 = (*(builder as *mut stack_builder)).prev_item_type;
            *fresh85 = g_variant_type_next((*(builder as *mut stack_builder)).prev_item_type);
        }
    } else {
        let ref mut fresh86 = (*(builder as *mut stack_builder)).prev_item_type;
        *fresh86 = safe_c2rust_g_variant_get_type(value);
    }
    safe_c2rust_g_variant_builder_make_room(builder as *mut stack_builder);
    let ref mut fresh87 = (*(builder as *mut stack_builder)).offset;
    let fresh88 = *fresh87;
    *fresh87 = (*fresh87).wrapping_add(1);
    let ref mut fresh89 = *(*(builder as *mut stack_builder))
        .children
        .offset(fresh88 as isize);
    *fresh89 = g_variant_ref_sink(value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_open(
    mut builder: *mut GVariantBuilder,
    mut type_0: *const GVariantType,
) {
    let mut parent: *mut GVariantBuilder = ::core::ptr::null_mut::<GVariantBuilder>();
    let mut valid_builder: gboolean = safe_c2rust_ensure_valid_builder(builder);
    if ({
        let mut _g_boolean_var_108: ::core::ffi::c_int = 0;
        if valid_builder != 0 {
            _g_boolean_var_108 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_108 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_108
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_builder\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_109: ::core::ffi::c_int = 0;
        if (*(builder as *mut stack_builder)).offset < (*(builder as *mut stack_builder)).max_items
        {
            _g_boolean_var_109 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_109 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_109
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"GVSB(builder)->offset < GVSB(builder)->max_items\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_110: ::core::ffi::c_int = 0;
        if (*(builder as *mut stack_builder)).expected_type.is_null()
            || g_variant_type_is_subtype_of(
                type_0,
                (*(builder as *mut stack_builder)).expected_type,
            ) != 0
        {
            _g_boolean_var_110 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_110 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_110
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!GVSB(builder)->expected_type || g_variant_type_is_subtype_of (type, GVSB(builder)->expected_type)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_111: ::core::ffi::c_int = 0;
        if (*(builder as *mut stack_builder)).prev_item_type.is_null()
            || g_variant_type_is_subtype_of(
                (*(builder as *mut stack_builder)).prev_item_type,
                type_0,
            ) != 0
        {
            _g_boolean_var_111 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_111 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_111
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!GVSB(builder)->prev_item_type || g_variant_type_is_subtype_of (GVSB(builder)->prev_item_type, type)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    parent = if 1 as ::core::ffi::c_int != 0 {
        g_slice_copy(
            ::core::mem::size_of::<GVariantBuilder>() as gsize,
            builder as gconstpointer,
        ) as *mut GVariantBuilder
    } else {
        ::core::ptr::null_mut::<GVariantBuilder>()
    };
    safe_c2rust_g_variant_builder_init(builder, type_0);
    let ref mut fresh93 = (*(builder as *mut stack_builder)).parent;
    *fresh93 = parent;
    if !(*(parent as *mut stack_builder)).prev_item_type.is_null() {
        if (*(builder as *mut stack_builder)).uniform_item_types() == 0 {
            let ref mut fresh94 = (*(builder as *mut stack_builder)).prev_item_type;
            *fresh94 = g_variant_type_first((*(parent as *mut stack_builder)).prev_item_type);
        } else if g_variant_type_is_variant((*(builder as *mut stack_builder)).type_0) == 0 {
            let ref mut fresh95 = (*(builder as *mut stack_builder)).prev_item_type;
            *fresh95 = g_variant_type_element((*(parent as *mut stack_builder)).prev_item_type);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_close(mut builder: *mut GVariantBuilder) {
    let mut parent: *mut GVariantBuilder = ::core::ptr::null_mut::<GVariantBuilder>();
    let mut valid_builder: gboolean = safe_c2rust_ensure_valid_builder(builder);
    if ({
        let mut _g_boolean_var_112: ::core::ffi::c_int = 0;
        if valid_builder != 0 {
            _g_boolean_var_112 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_112 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_112
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_builder\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_113: ::core::ffi::c_int = 0;
        if !(*(builder as *mut stack_builder)).parent.is_null() {
            _g_boolean_var_113 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_113 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_113
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"GVSB(builder)->parent != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    parent = (*(builder as *mut stack_builder)).parent;
    let ref mut fresh96 = (*(builder as *mut stack_builder)).parent;
    *fresh96 = ::core::ptr::null_mut::<GVariantBuilder>();
    safe_c2rust_g_variant_builder_add_value(parent, safe_c2rust_g_variant_builder_end(builder));
    *builder = *parent;
    g_slice_free1(
        ::core::mem::size_of::<GVariantBuilder>() as gsize,
        parent as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_variant_make_maybe_type(
    mut element: *mut GVariant,
) -> *mut GVariantType {
    return g_variant_type_new_maybe(safe_c2rust_g_variant_get_type(element));
}
unsafe extern "C" fn safe_c2rust_g_variant_make_array_type(
    mut element: *mut GVariant,
) -> *mut GVariantType {
    return g_variant_type_new_array(safe_c2rust_g_variant_get_type(element));
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_end(
    mut builder: *mut GVariantBuilder,
) -> *mut GVariant {
    let mut my_type: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut valid_builder: gboolean = safe_c2rust_ensure_valid_builder(builder);
    if ({
        let mut _g_boolean_var_114: ::core::ffi::c_int = 0;
        if valid_builder != 0 {
            _g_boolean_var_114 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_114 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_114
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_builder\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_115: ::core::ffi::c_int = 0;
        if (*(builder as *mut stack_builder)).offset >= (*(builder as *mut stack_builder)).min_items
        {
            _g_boolean_var_115 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_115 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_115
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"GVSB(builder)->offset >= GVSB(builder)->min_items\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_116: ::core::ffi::c_int = 0;
        if (*(builder as *mut stack_builder)).uniform_item_types() == 0
            || !(*(builder as *mut stack_builder)).prev_item_type.is_null()
            || g_variant_type_is_definite((*(builder as *mut stack_builder)).type_0) != 0
        {
            _g_boolean_var_116 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_116 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_116
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!GVSB(builder)->uniform_item_types || GVSB(builder)->prev_item_type != NULL || g_variant_type_is_definite (GVSB(builder)->type)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if g_variant_type_is_definite((*(builder as *mut stack_builder)).type_0) != 0 {
        my_type = g_variant_type_copy((*(builder as *mut stack_builder)).type_0);
    } else if g_variant_type_is_maybe((*(builder as *mut stack_builder)).type_0) != 0 {
        my_type = safe_c2rust_g_variant_make_maybe_type(
            *(*(builder as *mut stack_builder))
                .children
                .offset(0 as ::core::ffi::c_int as isize),
        );
    } else if g_variant_type_is_array((*(builder as *mut stack_builder)).type_0) != 0 {
        my_type = safe_c2rust_g_variant_make_array_type(
            *(*(builder as *mut stack_builder))
                .children
                .offset(0 as ::core::ffi::c_int as isize),
        );
    } else if g_variant_type_is_tuple((*(builder as *mut stack_builder)).type_0) != 0 {
        my_type = safe_c2rust_g_variant_make_tuple_type(
            (*(builder as *mut stack_builder)).children,
            (*(builder as *mut stack_builder)).offset,
        );
    } else if g_variant_type_is_dict_entry((*(builder as *mut stack_builder)).type_0) != 0 {
        my_type = safe_c2rust_g_variant_make_dict_entry_type(
            *(*(builder as *mut stack_builder))
                .children
                .offset(0 as ::core::ffi::c_int as isize),
            *(*(builder as *mut stack_builder))
                .children
                .offset(1 as ::core::ffi::c_int as isize),
        );
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
            3775 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    value = g_variant_new_from_children(
        my_type,
        ({
            let mut __n: gsize = (*(builder as *mut stack_builder)).offset;
            let mut __s: gsize = ::core::mem::size_of::<*mut GVariant>() as gsize;
            let mut __p: gpointer = (*(builder as *mut stack_builder)).children as gpointer;
            if __s == 1 as gsize {
                __p = g_realloc(__p, __n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_realloc(__p, __n.wrapping_mul(__s));
            } else {
                __p = g_realloc_n(__p, __n, __s);
            }
            __p
        }) as *mut *mut GVariant,
        (*(builder as *mut stack_builder)).offset,
        (*(builder as *mut stack_builder)).trusted() as gboolean,
    );
    let ref mut fresh67 = (*(builder as *mut stack_builder)).children;
    *fresh67 = ::core::ptr::null_mut::<*mut GVariant>();
    (*(builder as *mut stack_builder)).offset = 0 as gsize;
    safe_c2rust_g_variant_builder_clear(builder);
    g_variant_type_free(my_type);
    return value;
}
pub const GVSD_MAGIC: gsize = 2579507750 as ::core::ffi::c_uint as gsize;
pub const GVSD_MAGIC_PARTIAL: gsize = 3488698669 as ::core::ffi::c_uint as gsize;
pub const GVHD_MAGIC: gsize = 2450270775 as ::core::ffi::c_uint as gsize;
unsafe extern "C" fn safe_c2rust_ensure_valid_dict(mut dict: *mut GVariantDict) -> gboolean {
    if dict.is_null() {
        return FALSE;
    } else if (*(dict as *mut stack_dict)).magic == GVSD_MAGIC {
        return TRUE;
    }
    if (*dict).u.s.partial_magic == GVSD_MAGIC_PARTIAL {
        static mut safe_c2rust_cleared_dict: GVariantDict = _GVariantDict {
            u: C2RustUnnamed_2 {
                s: C2RustUnnamed_3 {
                    asv: ::core::ptr::null::<GVariant>() as *mut GVariant,
                    partial_magic: 0,
                    y: [0; 14],
                },
            },
        };
        if memcmp(
            &raw mut safe_c2rust_cleared_dict.u.s.y as *mut guintptr as *const ::core::ffi::c_void,
            &raw mut (*dict).u.s.y as *mut guintptr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[guintptr; 14]>() as size_t,
        ) != 0
        {
            return FALSE;
        }
        safe_c2rust_g_variant_dict_init(dict, (*dict).u.s.asv);
    }
    return ((*(dict as *mut stack_dict)).magic == GVSD_MAGIC) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_new(
    mut from_asv: *mut GVariant,
) -> *mut GVariantDict {
    let mut dict: *mut GVariantDict = ::core::ptr::null_mut::<GVariantDict>();
    dict = g_slice_alloc(::core::mem::size_of::<heap_dict>() as gsize) as *mut GVariantDict;
    safe_c2rust_g_variant_dict_init(dict, from_asv);
    (*(dict as *mut heap_dict)).magic = GVHD_MAGIC;
    (*(dict as *mut heap_dict)).ref_count = 1 as ::core::ffi::c_int as gint;
    return dict;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_init(
    mut dict: *mut GVariantDict,
    mut from_asv: *mut GVariant,
) {
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let ref mut fresh103 = (*(dict as *mut stack_dict)).values;
    *fresh103 = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GVariant) -> ()>, GDestroyNotify>(
            Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()),
        ),
    );
    (*(dict as *mut stack_dict)).magic = GVSD_MAGIC;
    if !from_asv.is_null() {
        safe_c2rust_g_variant_iter_init(&raw mut iter, from_asv);
        while safe_c2rust_g_variant_iter_next(
            &raw mut iter,
            b"{sv}\0" as *const u8 as *const gchar,
            &raw mut key,
            &raw mut value,
        ) != 0
        {
            g_hash_table_insert(
                (*(dict as *mut stack_dict)).values,
                key as gpointer,
                value as gpointer,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_lookup(
    mut dict: *mut GVariantDict,
    mut key: *const gchar,
    mut format_string: *const gchar,
    mut args: ...
) -> gboolean {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ap: ::core::ffi::VaList;
    let mut valid_dict: gboolean = safe_c2rust_ensure_valid_dict(dict);
    if ({
        let mut _g_boolean_var_117: ::core::ffi::c_int = 0;
        if valid_dict != 0 {
            _g_boolean_var_117 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_117 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_117
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_dict\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_118: ::core::ffi::c_int = 0;
        if !key.is_null() {
            _g_boolean_var_118 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_118 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_118
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
        let mut _g_boolean_var_119: ::core::ffi::c_int = 0;
        if !format_string.is_null() {
            _g_boolean_var_119 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_119 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_119
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"format_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    value = g_hash_table_lookup((*(dict as *mut stack_dict)).values, key as gconstpointer)
        as *mut GVariant;
    if value.is_null()
        || safe_c2rust_g_variant_check_format_string(value, format_string, FALSE) == 0
    {
        return FALSE;
    }
    ap = args.clone();
    safe_c2rust_g_variant_get_va(
        value,
        format_string,
        ::core::ptr::null_mut::<*const gchar>(),
        &raw mut ap,
    );
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_lookup_value(
    mut dict: *mut GVariantDict,
    mut key: *const gchar,
    mut expected_type: *const GVariantType,
) -> *mut GVariant {
    let mut result: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut valid_dict: gboolean = safe_c2rust_ensure_valid_dict(dict);
    if ({
        let mut _g_boolean_var_120: ::core::ffi::c_int = 0;
        if valid_dict != 0 {
            _g_boolean_var_120 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_120 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_120
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_dict\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_121: ::core::ffi::c_int = 0;
        if !key.is_null() {
            _g_boolean_var_121 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_121 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_121
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    result = g_hash_table_lookup((*(dict as *mut stack_dict)).values, key as gconstpointer)
        as *mut GVariant;
    if !result.is_null()
        && (expected_type.is_null() || safe_c2rust_g_variant_is_of_type(result, expected_type) != 0)
    {
        return g_variant_ref(result);
    }
    return ::core::ptr::null_mut::<GVariant>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_contains(
    mut dict: *mut GVariantDict,
    mut key: *const gchar,
) -> gboolean {
    let mut valid_dict: gboolean = safe_c2rust_ensure_valid_dict(dict);
    if ({
        let mut _g_boolean_var_122: ::core::ffi::c_int = 0;
        if valid_dict != 0 {
            _g_boolean_var_122 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_122 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_122
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_dict\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_123: ::core::ffi::c_int = 0;
        if !key.is_null() {
            _g_boolean_var_123 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_123 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_123
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
    return g_hash_table_contains((*(dict as *mut stack_dict)).values, key as gconstpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_insert(
    mut dict: *mut GVariantDict,
    mut key: *const gchar,
    mut format_string: *const gchar,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaList;
    let mut valid_dict: gboolean = safe_c2rust_ensure_valid_dict(dict);
    if ({
        let mut _g_boolean_var_124: ::core::ffi::c_int = 0;
        if valid_dict != 0 {
            _g_boolean_var_124 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_124 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_124
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_dict\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_125: ::core::ffi::c_int = 0;
        if !key.is_null() {
            _g_boolean_var_125 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_125 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_125
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_126: ::core::ffi::c_int = 0;
        if !format_string.is_null() {
            _g_boolean_var_126 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_126 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_126
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"format_string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    ap = args.clone();
    safe_c2rust_g_variant_dict_insert_value(
        dict,
        key,
        safe_c2rust_g_variant_new_va(
            format_string,
            ::core::ptr::null_mut::<*const gchar>(),
            &raw mut ap,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_insert_value(
    mut dict: *mut GVariantDict,
    mut key: *const gchar,
    mut value: *mut GVariant,
) {
    let mut valid_dict: gboolean = safe_c2rust_ensure_valid_dict(dict);
    if ({
        let mut _g_boolean_var_127: ::core::ffi::c_int = 0;
        if valid_dict != 0 {
            _g_boolean_var_127 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_127 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_127
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_dict\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_128: ::core::ffi::c_int = 0;
        if !key.is_null() {
            _g_boolean_var_128 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_128 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_128
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_129: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_129 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_129 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_129
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_hash_table_insert(
        (*(dict as *mut stack_dict)).values,
        safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as gpointer,
        g_variant_ref_sink(value) as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_remove(
    mut dict: *mut GVariantDict,
    mut key: *const gchar,
) -> gboolean {
    let mut valid_dict: gboolean = safe_c2rust_ensure_valid_dict(dict);
    if ({
        let mut _g_boolean_var_130: ::core::ffi::c_int = 0;
        if valid_dict != 0 {
            _g_boolean_var_130 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_130 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_130
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_dict\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_131: ::core::ffi::c_int = 0;
        if !key.is_null() {
            _g_boolean_var_131 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_131 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_131
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
    return g_hash_table_remove((*(dict as *mut stack_dict)).values, key as gconstpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_clear(mut dict: *mut GVariantDict) {
    if (*(dict as *mut stack_dict)).magic == 0 as gsize {
        return;
    }
    let mut valid_dict: gboolean = safe_c2rust_ensure_valid_dict(dict);
    if ({
        let mut _g_boolean_var_132: ::core::ffi::c_int = 0;
        if valid_dict != 0 {
            _g_boolean_var_132 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_132 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_132
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_dict\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_hash_table_unref((*(dict as *mut stack_dict)).values);
    let ref mut fresh104 = (*(dict as *mut stack_dict)).values;
    *fresh104 = ::core::ptr::null_mut::<GHashTable>();
    (*(dict as *mut stack_dict)).magic = 0 as gsize;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_end(
    mut dict: *mut GVariantDict,
) -> *mut GVariant {
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed_0 {
            s: C2RustUnnamed_1 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut value: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut valid_dict: gboolean = safe_c2rust_ensure_valid_dict(dict);
    if ({
        let mut _g_boolean_var_133: ::core::ffi::c_int = 0;
        if valid_dict != 0 {
            _g_boolean_var_133 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_133 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_133
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_dict\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    safe_c2rust_g_variant_builder_init(&raw mut builder, G_VARIANT_TYPE_VARDICT);
    g_hash_table_iter_init(&raw mut iter, (*(dict as *mut stack_dict)).values);
    while g_hash_table_iter_next(&raw mut iter, &raw mut key, &raw mut value) != 0 {
        safe_c2rust_g_variant_builder_add(
            &raw mut builder,
            b"{sv}\0" as *const u8 as *const gchar,
            key as *const gchar,
            value as *mut GVariant,
        );
    }
    safe_c2rust_g_variant_dict_clear(dict);
    return safe_c2rust_g_variant_builder_end(&raw mut builder);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_ref(
    mut dict: *mut GVariantDict,
) -> *mut GVariantDict {
    if ({
        let mut _g_boolean_var_134: ::core::ffi::c_int = 0;
        if (*(dict as *mut heap_dict)).magic == 2450270775 as ::core::ffi::c_uint as gsize {
            _g_boolean_var_134 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_134 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_134
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_heap_dict (dict)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariantDict>();
    }
    let ref mut fresh105 = (*(dict as *mut heap_dict)).ref_count;
    *fresh105 += 1;
    return dict;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_dict_unref(mut dict: *mut GVariantDict) {
    if ({
        let mut _g_boolean_var_135: ::core::ffi::c_int = 0;
        if (*(dict as *mut heap_dict)).magic == 2450270775 as ::core::ffi::c_uint as gsize {
            _g_boolean_var_135 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_135 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_135
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"is_valid_heap_dict (dict)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let ref mut fresh106 = (*(dict as *mut heap_dict)).ref_count;
    *fresh106 -= 1;
    if *fresh106 == 0 as ::core::ffi::c_int {
        safe_c2rust_g_variant_dict_clear(dict);
        g_slice_free1(
            ::core::mem::size_of::<heap_dict>() as gsize,
            dict as *mut heap_dict as gpointer,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_format_string_scan(
    mut string: *const gchar,
    mut limit: *const gchar,
    mut endptr: *mut *const gchar,
) -> gboolean {
    let mut c: ::core::ffi::c_char = 0;
    let mut current_block_25: u64;
    match if string == limit {
        '\0' as i32
    } else {
        let fresh0 = string;
        string = string.offset(1);
        *fresh0 as ::core::ffi::c_int
    } {
        98 | 121 | 110 | 113 | 105 | 117 | 120 | 116 | 104 | 100 | 115 | 111 | 103 | 118 | 42
        | 63 | 114 => {}
        109 => return safe_c2rust_g_variant_format_string_scan(string, limit, endptr),
        97 | 64 => return g_variant_type_string_scan(string, limit, endptr),
        40 => {
            while (if string == limit {
                '\0' as i32
            } else {
                *string as ::core::ffi::c_int
            }) != ')' as i32
            {
                if safe_c2rust_g_variant_format_string_scan(string, limit, &raw mut string) == 0 {
                    return FALSE;
                }
            }
            if string == limit {
            } else {
                let fresh1 = string;
                string = string.offset(1);
                *fresh1;
            };
        }
        123 => {
            c = (if string == limit {
                '\0' as i32
            } else {
                let fresh2 = string;
                string = string.offset(1);
                *fresh2 as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
            if c as ::core::ffi::c_int == '&' as i32 {
                c = (if string == limit {
                    '\0' as i32
                } else {
                    let fresh3 = string;
                    string = string.offset(1);
                    *fresh3 as ::core::ffi::c_int
                }) as ::core::ffi::c_char;
                if c as ::core::ffi::c_int != 's' as i32
                    && c as ::core::ffi::c_int != 'o' as i32
                    && c as ::core::ffi::c_int != 'g' as i32
                {
                    return FALSE;
                }
            } else {
                if c as ::core::ffi::c_int == '@' as i32 {
                    c = (if string == limit {
                        '\0' as i32
                    } else {
                        let fresh4 = string;
                        string = string.offset(1);
                        *fresh4 as ::core::ffi::c_int
                    }) as ::core::ffi::c_char;
                }
                if c as ::core::ffi::c_int != '\0' as i32
                    && strchr(
                        b"bynqiuxthdsog?\0" as *const u8 as *const ::core::ffi::c_char,
                        c as ::core::ffi::c_int,
                    )
                    .is_null()
                {
                    return FALSE;
                }
            }
            if safe_c2rust_g_variant_format_string_scan(string, limit, &raw mut string) == 0 {
                return FALSE;
            }
            if (if string == limit {
                '\0' as i32
            } else {
                let fresh5 = string;
                string = string.offset(1);
                *fresh5 as ::core::ffi::c_int
            }) != '}' as i32
            {
                return FALSE;
            }
        }
        94 => {
            c = (if string == limit {
                '\0' as i32
            } else {
                let fresh6 = string;
                string = string.offset(1);
                *fresh6 as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
            if c as ::core::ffi::c_int == 'a' as i32 {
                c = (if string == limit {
                    '\0' as i32
                } else {
                    let fresh7 = string;
                    string = string.offset(1);
                    *fresh7 as ::core::ffi::c_int
                }) as ::core::ffi::c_char;
                if c as ::core::ffi::c_int == '&' as i32 {
                    c = (if string == limit {
                        '\0' as i32
                    } else {
                        let fresh8 = string;
                        string = string.offset(1);
                        *fresh8 as ::core::ffi::c_int
                    }) as ::core::ffi::c_char;
                    if c as ::core::ffi::c_int == 'a' as i32 {
                        c = (if string == limit {
                            '\0' as i32
                        } else {
                            let fresh9 = string;
                            string = string.offset(1);
                            *fresh9 as ::core::ffi::c_int
                        }) as ::core::ffi::c_char;
                        if c as ::core::ffi::c_int == 'y' as i32 {
                            current_block_25 = 15512526488502093901;
                        } else {
                            current_block_25 = 5529461102203738653;
                        }
                    } else if c as ::core::ffi::c_int == 's' as i32
                        || c as ::core::ffi::c_int == 'o' as i32
                    {
                        current_block_25 = 15512526488502093901;
                    } else {
                        current_block_25 = 5529461102203738653;
                    }
                } else if c as ::core::ffi::c_int == 'a' as i32 {
                    c = (if string == limit {
                        '\0' as i32
                    } else {
                        let fresh10 = string;
                        string = string.offset(1);
                        *fresh10 as ::core::ffi::c_int
                    }) as ::core::ffi::c_char;
                    if c as ::core::ffi::c_int == 'y' as i32 {
                        current_block_25 = 15512526488502093901;
                    } else {
                        current_block_25 = 5529461102203738653;
                    }
                } else if c as ::core::ffi::c_int == 's' as i32
                    || c as ::core::ffi::c_int == 'o' as i32
                {
                    current_block_25 = 15512526488502093901;
                } else if c as ::core::ffi::c_int == 'y' as i32 {
                    current_block_25 = 15512526488502093901;
                } else {
                    current_block_25 = 5529461102203738653;
                }
            } else if c as ::core::ffi::c_int == '&' as i32 {
                c = (if string == limit {
                    '\0' as i32
                } else {
                    let fresh11 = string;
                    string = string.offset(1);
                    *fresh11 as ::core::ffi::c_int
                }) as ::core::ffi::c_char;
                if c as ::core::ffi::c_int == 'a' as i32 {
                    c = (if string == limit {
                        '\0' as i32
                    } else {
                        let fresh12 = string;
                        string = string.offset(1);
                        *fresh12 as ::core::ffi::c_int
                    }) as ::core::ffi::c_char;
                    if c as ::core::ffi::c_int == 'y' as i32 {
                        current_block_25 = 15512526488502093901;
                    } else {
                        current_block_25 = 5529461102203738653;
                    }
                } else {
                    current_block_25 = 5529461102203738653;
                }
            } else {
                current_block_25 = 5529461102203738653;
            }
            match current_block_25 {
                15512526488502093901 => {}
                _ => return FALSE,
            }
        }
        38 => {
            c = (if string == limit {
                '\0' as i32
            } else {
                let fresh13 = string;
                string = string.offset(1);
                *fresh13 as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
            if c as ::core::ffi::c_int != 's' as i32
                && c as ::core::ffi::c_int != 'o' as i32
                && c as ::core::ffi::c_int != 'g' as i32
            {
                return FALSE;
            }
        }
        _ => return FALSE,
    }
    if !endptr.is_null() {
        *endptr = string;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_check_format_string(
    mut value: *mut GVariant,
    mut format_string: *const gchar,
    mut copy_only: gboolean,
) -> gboolean {
    let mut original_format: *const gchar = format_string;
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    type_string = safe_c2rust_g_variant_get_type_string(value);
    while *type_string as ::core::ffi::c_int != 0 || *format_string as ::core::ffi::c_int != 0 {
        let fresh100 = format_string;
        format_string = format_string.offset(1);
        let mut format: gchar = *fresh100;
        match format as ::core::ffi::c_int {
            38 => {
                if ({
                    let mut _g_boolean_var_136: ::core::ffi::c_int = 0;
                    if copy_only != 0 {
                        _g_boolean_var_136 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_136 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_136
                }) as ::core::ffi::c_long
                    != 0
                {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_CRITICAL,
                        b"g_variant_check_format_string() is being called by a function with a GVariant varargs interface to validate the passed format string for type safety.  The passed format (%s) contains a '&' character which would result in a pointer being returned to the data inside of a GVariant instance that may no longer exist by the time the function returns.  Modify your code to use a format string without '&'.\0"
                            as *const u8 as *const gchar,
                        original_format,
                    );
                    return FALSE;
                }
                continue;
            }
            94 | 64 => {
                continue;
            }
            63 => {
                let fresh101 = type_string;
                type_string = type_string.offset(1);
                let mut s: ::core::ffi::c_char = *fresh101;
                if s as ::core::ffi::c_int == '\0' as i32
                    || strchr(
                        b"bynqiuxthdsog\0" as *const u8 as *const ::core::ffi::c_char,
                        s as ::core::ffi::c_int,
                    )
                    .is_null()
                {
                    return FALSE;
                }
                continue;
            }
            114 => {
                if *type_string as ::core::ffi::c_int != '(' as i32 {
                    return FALSE;
                }
            }
            42 => {}
            _ => {
                let fresh102 = type_string;
                type_string = type_string.offset(1);
                if format as ::core::ffi::c_int != *fresh102 as ::core::ffi::c_int {
                    return FALSE;
                }
                continue;
            }
        }
        if g_variant_type_string_scan(
            type_string,
            ::core::ptr::null::<gchar>(),
            &raw mut type_string,
        ) == 0
        {
            return FALSE;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_format_string_scan_type(
    mut string: *const gchar,
    mut limit: *const gchar,
    mut endptr: *mut *const gchar,
) -> *mut GVariantType {
    let mut my_end: *const gchar = ::core::ptr::null::<gchar>();
    let mut dest: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut new: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if endptr.is_null() {
        endptr = &raw mut my_end;
    }
    if safe_c2rust_g_variant_format_string_scan(string, limit, endptr) == 0 {
        return ::core::ptr::null_mut::<GVariantType>();
    }
    new = g_malloc(
        ((*endptr).offset_from(string) as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as gsize,
    ) as *mut gchar;
    dest = new;
    while string != *endptr {
        if *string as ::core::ffi::c_int != '@' as i32
            && *string as ::core::ffi::c_int != '&' as i32
            && *string as ::core::ffi::c_int != '^' as i32
        {
            let fresh14 = dest;
            dest = dest.offset(1);
            *fresh14 = *string;
        }
        string = string.offset(1);
    }
    *dest = '\0' as i32 as gchar;
    return g_variant_type_checked_(new) as *mut GVariantType;
}
unsafe extern "C" fn safe_c2rust_valid_format_string(
    mut format_string: *const gchar,
    mut single: gboolean,
    mut value: *mut GVariant,
) -> gboolean {
    let mut endptr: *const gchar = ::core::ptr::null::<gchar>();
    let mut type_0: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
    type_0 = safe_c2rust_g_variant_format_string_scan_type(
        format_string,
        ::core::ptr::null::<gchar>(),
        &raw mut endptr,
    );
    if ({
        let mut _g_boolean_var_137: ::core::ffi::c_int = 0;
        if type_0.is_null() || single != 0 && *endptr as ::core::ffi::c_int != '\0' as i32 {
            _g_boolean_var_137 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_137 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_137
    }) as ::core::ffi::c_long
        != 0
    {
        if single != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"'%s' is not a valid GVariant format string\0" as *const u8 as *const gchar,
                format_string,
            );
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"'%s' does not have a valid GVariant format string as a prefix\0" as *const u8
                    as *const gchar,
                format_string,
            );
        }
        if !type_0.is_null() {
            g_variant_type_free(type_0);
        }
        return FALSE;
    }
    if ({
        let mut _g_boolean_var_138: ::core::ffi::c_int = 0;
        if !value.is_null() && safe_c2rust_g_variant_is_of_type(value, type_0) == 0 {
            _g_boolean_var_138 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_138 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_138
    }) as ::core::ffi::c_long
        != 0
    {
        let mut fragment: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut typestr: *mut gchar = ::core::ptr::null_mut::<gchar>();
        fragment = g_strndup(
            format_string,
            endptr.offset_from(format_string) as ::core::ffi::c_long as gsize,
        );
        typestr = g_variant_type_dup_string(type_0);
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"the GVariant format string '%s' has a type of '%s' but the given value has a type of '%s'\0"
                as *const u8 as *const gchar,
            fragment,
            typestr,
            safe_c2rust_g_variant_get_type_string(value),
        );
        g_variant_type_free(type_0);
        g_free(fragment as gpointer);
        g_free(typestr as gpointer);
        return FALSE;
    }
    g_variant_type_free(type_0);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_variant_format_string_is_leaf(
    mut str: *const gchar,
) -> gboolean {
    return (*str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'm' as i32
        && *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '(' as i32
        && *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '{' as i32)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_variant_format_string_is_nnp(mut str: *const gchar) -> gboolean {
    return (*str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'a' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 's' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'o' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'g' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '^' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '@' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '*' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '?' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'r' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'v' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '&' as i32)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_variant_valist_free_nnp(
    mut str: *const gchar,
    mut ptr: gpointer,
) {
    match *str as ::core::ffi::c_int {
        97 => {
            safe_c2rust_g_variant_iter_free(ptr as *mut GVariantIter);
        }
        94 => {
            if if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = str as *const ::core::ffi::c_char;
                    let __suffix: *const ::core::ffi::c_char =
                        b"y\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_139: ::core::ffi::c_int = 0;
                        if __str.is_null() || __suffix.is_null() {
                            _g_boolean_var_139 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_139 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_139
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __suffix_len: size_t = strlen(
                            __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __suffix_len {
                            __result = (memcmp(
                                __str
                                    .offset(__str_len as isize)
                                    .offset(-(__suffix_len as isize))
                                    as *const ::core::ffi::c_void,
                                __suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __suffix_len,
                            ) == 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as gboolean;
                        }
                    }
                    __result
                })
            } else {
                g_str_has_suffix(str, b"y\0" as *const u8 as *const gchar)
            } != 0
            {
                if *str.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'a' as i32
                {
                    g_free(ptr);
                } else if *str.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'a' as i32
                {
                    g_strfreev(ptr as *mut *mut gchar);
                }
            } else if *str.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '&' as i32
            {
                g_strfreev(ptr as *mut *mut gchar);
            } else {
                g_free(ptr);
            }
        }
        115 | 111 | 103 => {
            g_free(ptr);
        }
        64 | 42 | 63 | 118 => {
            g_variant_unref(ptr as *mut GVariant);
        }
        38 => {}
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                4790 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_variant_scan_convenience(
    mut str: *mut *const gchar,
    mut constant: *mut gboolean,
    mut arrays: *mut guint,
) -> gchar {
    *constant = FALSE as gboolean;
    *arrays = 0 as guint;
    loop {
        let fresh41 = *str;
        *str = (*str).offset(1);
        let mut c: ::core::ffi::c_char = *fresh41;
        if c as ::core::ffi::c_int == '&' as i32 {
            *constant = TRUE as gboolean;
        } else if c as ::core::ffi::c_int == 'a' as i32 {
            *arrays = (*arrays).wrapping_add(1);
        } else {
            return c as gchar;
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_variant_valist_new_nnp(
    mut str: *mut *const gchar,
    mut ptr: gpointer,
) -> *mut GVariant {
    if **str as ::core::ffi::c_int == '&' as i32 {
        *str = (*str).offset(1);
    }
    let fresh98 = *str;
    *str = (*str).offset(1);
    match *fresh98 as ::core::ffi::c_int {
        97 => {
            if !ptr.is_null() {
                let mut type_0: *const GVariantType = ::core::ptr::null::<GVariantType>();
                let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                value = safe_c2rust_g_variant_builder_end(ptr as *mut GVariantBuilder);
                type_0 = safe_c2rust_g_variant_get_type(value);
                if ({
                    let mut _g_boolean_var_140: ::core::ffi::c_int = 0;
                    if g_variant_type_is_array(type_0) == 0 {
                        _g_boolean_var_140 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_140 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_140
                }) as ::core::ffi::c_long
                    != 0
                {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_ERROR,
                        b"g_variant_new: expected array GVariantBuilder but the built value has type '%s'\0"
                            as *const u8 as *const gchar,
                        safe_c2rust_g_variant_get_type_string(value),
                    );
                    loop {}
                }
                type_0 = g_variant_type_element(type_0);
                if ({
                    let mut _g_boolean_var_141: ::core::ffi::c_int = 0;
                    if g_variant_type_is_subtype_of(type_0, *str as *mut GVariantType) == 0 {
                        _g_boolean_var_141 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_141 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_141
                }) as ::core::ffi::c_long
                    != 0
                {
                    let mut type_string: *mut gchar =
                        g_variant_type_dup_string(*str as *mut GVariantType);
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_ERROR,
                        b"g_variant_new: expected GVariantBuilder array element type '%s' but the built value has element type '%s'\0"
                            as *const u8 as *const gchar,
                        type_string,
                        safe_c2rust_g_variant_get_type_string(value)
                            .offset(1 as ::core::ffi::c_int as isize),
                    );
                    loop {}
                }
                g_variant_type_string_scan(*str, ::core::ptr::null::<gchar>(), str);
                return value;
            } else {
                let mut type_1: *const GVariantType = *str as *mut GVariantType;
                g_variant_type_string_scan(*str, ::core::ptr::null::<gchar>(), str);
                if ({
                    let mut _g_boolean_var_142: ::core::ffi::c_int = 0;
                    if g_variant_type_is_definite(type_1) == 0 {
                        _g_boolean_var_142 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_142 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_142
                }) as ::core::ffi::c_long
                    != 0
                {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_ERROR,
                        b"g_variant_new: NULL pointer given with indefinite array type; unable to determine which type of empty array to construct.\0"
                            as *const u8 as *const gchar,
                    );
                    loop {}
                }
                return safe_c2rust_g_variant_new_array(
                    type_1,
                    ::core::ptr::null::<*mut GVariant>(),
                    0 as gsize,
                );
            }
        }
        115 => {
            let mut value_0: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            value_0 = safe_c2rust_g_variant_new_string(ptr as *const gchar);
            if value_0.is_null() {
                value_0 = safe_c2rust_g_variant_new_string(
                    b"[Invalid UTF-8]\0" as *const u8 as *const gchar,
                );
            }
            return value_0;
        }
        111 => return safe_c2rust_g_variant_new_object_path(ptr as *const gchar),
        103 => return safe_c2rust_g_variant_new_signature(ptr as *const gchar),
        94 => {
            let mut constant: gboolean = 0;
            let mut arrays: guint = 0;
            let mut type_2: gchar = 0;
            type_2 =
                safe_c2rust_g_variant_scan_convenience(str, &raw mut constant, &raw mut arrays);
            if type_2 as ::core::ffi::c_int == 's' as i32 {
                return safe_c2rust_g_variant_new_strv(
                    ptr as *const *const gchar,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            }
            if type_2 as ::core::ffi::c_int == 'o' as i32 {
                return safe_c2rust_g_variant_new_objv(
                    ptr as *const *const gchar,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            }
            if arrays > 1 as guint {
                return safe_c2rust_g_variant_new_bytestring_array(
                    ptr as *const *const gchar,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            }
            return safe_c2rust_g_variant_new_bytestring(ptr as *const gchar);
        }
        64 => {
            if ({
                let mut _g_boolean_var_143: ::core::ffi::c_int = 0;
                if safe_c2rust_g_variant_is_of_type(ptr as *mut GVariant, *str as *mut GVariantType)
                    == 0
                {
                    _g_boolean_var_143 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_143 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_143
            }) as ::core::ffi::c_long
                != 0
            {
                let mut type_string_0: *mut gchar =
                    g_variant_type_dup_string(*str as *mut GVariantType);
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"g_variant_new: expected GVariant of type '%s' but received value has type '%s'\0"
                        as *const u8 as *const gchar,
                    type_string_0,
                    safe_c2rust_g_variant_get_type_string(ptr as *mut GVariant),
                );
                loop {}
            }
            g_variant_type_string_scan(*str, ::core::ptr::null::<gchar>(), str);
            return ptr as *mut GVariant;
        }
        42 => return ptr as *mut GVariant,
        63 => {
            if ({
                let mut _g_boolean_var_144: ::core::ffi::c_int = 0;
                if g_variant_type_is_basic(safe_c2rust_g_variant_get_type(ptr as *mut GVariant))
                    == 0
                {
                    _g_boolean_var_144 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_144 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_144
            }) as ::core::ffi::c_long
                != 0
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"g_variant_new: format string '?' expects basic-typed GVariant, but received value has type '%s'\0"
                        as *const u8 as *const gchar,
                    safe_c2rust_g_variant_get_type_string(ptr as *mut GVariant),
                );
                loop {}
            }
            return ptr as *mut GVariant;
        }
        114 => {
            if ({
                let mut _g_boolean_var_145: ::core::ffi::c_int = 0;
                if g_variant_type_is_tuple(safe_c2rust_g_variant_get_type(ptr as *mut GVariant))
                    == 0
                {
                    _g_boolean_var_145 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_145 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_145
            }) as ::core::ffi::c_long
                != 0
            {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"g_variant_new: format string 'r' expects tuple-typed GVariant, but received value has type '%s'\0"
                        as *const u8 as *const gchar,
                    safe_c2rust_g_variant_get_type_string(ptr as *mut GVariant),
                );
                loop {}
            }
            return ptr as *mut GVariant;
        }
        118 => return safe_c2rust_g_variant_new_variant(ptr as *mut GVariant),
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                4946 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_variant_valist_get_nnp(
    mut str: *mut *const gchar,
    mut value: *mut GVariant,
) -> gpointer {
    let fresh40 = *str;
    *str = (*str).offset(1);
    match *fresh40 as ::core::ffi::c_int {
        97 => {
            g_variant_type_string_scan(*str, ::core::ptr::null::<gchar>(), str);
            return safe_c2rust_g_variant_iter_new(value) as gpointer;
        }
        38 => {
            *str = (*str).offset(1);
            return safe_c2rust_g_variant_get_string(value, ::core::ptr::null_mut::<gsize>())
                as *mut gchar as gpointer;
        }
        115 | 111 | 103 => {
            return safe_c2rust_g_variant_dup_string(value, ::core::ptr::null_mut::<gsize>())
                as gpointer;
        }
        94 => {
            let mut constant: gboolean = 0;
            let mut arrays: guint = 0;
            let mut type_0: gchar = 0;
            type_0 =
                safe_c2rust_g_variant_scan_convenience(str, &raw mut constant, &raw mut arrays);
            if type_0 as ::core::ffi::c_int == 's' as i32 {
                if constant != 0 {
                    return safe_c2rust_g_variant_get_strv(value, ::core::ptr::null_mut::<gsize>())
                        as gpointer;
                } else {
                    return safe_c2rust_g_variant_dup_strv(value, ::core::ptr::null_mut::<gsize>())
                        as gpointer;
                }
            } else if type_0 as ::core::ffi::c_int == 'o' as i32 {
                if constant != 0 {
                    return safe_c2rust_g_variant_get_objv(value, ::core::ptr::null_mut::<gsize>())
                        as gpointer;
                } else {
                    return safe_c2rust_g_variant_dup_objv(value, ::core::ptr::null_mut::<gsize>())
                        as gpointer;
                }
            } else if arrays > 1 as guint {
                if constant != 0 {
                    return safe_c2rust_g_variant_get_bytestring_array(
                        value,
                        ::core::ptr::null_mut::<gsize>(),
                    ) as gpointer;
                } else {
                    return safe_c2rust_g_variant_dup_bytestring_array(
                        value,
                        ::core::ptr::null_mut::<gsize>(),
                    ) as gpointer;
                }
            } else if constant != 0 {
                return safe_c2rust_g_variant_get_bytestring(value) as *mut gchar as gpointer;
            } else {
                return safe_c2rust_g_variant_dup_bytestring(value, ::core::ptr::null_mut::<gsize>())
                    as gpointer;
            }
        }
        64 => {
            g_variant_type_string_scan(*str, ::core::ptr::null::<gchar>(), str);
        }
        42 | 63 | 114 => {}
        118 => return safe_c2rust_g_variant_get_variant(value) as gpointer,
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                5023 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    return g_variant_ref(value) as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_variant_valist_skip_leaf(
    mut str: *mut *const gchar,
    mut app: *mut ::core::ffi::VaList,
) {
    if safe_c2rust_g_variant_format_string_is_nnp(*str) != 0 {
        safe_c2rust_g_variant_format_string_scan(*str, ::core::ptr::null::<gchar>(), str);
        (*app).arg::<gpointer>();
        return;
    }
    let fresh97 = *str;
    *str = (*str).offset(1);
    match *fresh97 as ::core::ffi::c_int {
        98 | 121 | 110 | 113 | 105 | 117 | 104 => {
            (*app).arg::<::core::ffi::c_int>();
            return;
        }
        120 | 116 => {
            (*app).arg::<guint64>();
            return;
        }
        100 => {
            (*app).arg::<gdouble>();
            return;
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                5061 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_variant_valist_new_leaf(
    mut str: *mut *const gchar,
    mut app: *mut ::core::ffi::VaList,
) -> *mut GVariant {
    if safe_c2rust_g_variant_format_string_is_nnp(*str) != 0 {
        return safe_c2rust_g_variant_valist_new_nnp(str, (*app).arg::<gpointer>());
    }
    let fresh99 = *str;
    *str = (*str).offset(1);
    match *fresh99 as ::core::ffi::c_int {
        98 => return safe_c2rust_g_variant_new_boolean((*app).arg::<gboolean>()),
        121 => return safe_c2rust_g_variant_new_byte((*app).arg::<guint>() as guint8),
        110 => return safe_c2rust_g_variant_new_int16((*app).arg::<gint>() as gint16),
        113 => return safe_c2rust_g_variant_new_uint16((*app).arg::<guint>() as guint16),
        105 => return safe_c2rust_g_variant_new_int32((*app).arg::<gint>()),
        117 => return safe_c2rust_g_variant_new_uint32((*app).arg::<guint>()),
        120 => return safe_c2rust_g_variant_new_int64((*app).arg::<gint64>()),
        116 => return safe_c2rust_g_variant_new_uint64((*app).arg::<guint64>()),
        104 => return safe_c2rust_g_variant_new_handle((*app).arg::<gint>()),
        100 => return safe_c2rust_g_variant_new_double((*app).arg::<gdouble>()),
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                5105 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_variant_valist_get_leaf(
    mut str: *mut *const gchar,
    mut value: *mut GVariant,
    mut free: gboolean,
    mut app: *mut ::core::ffi::VaList,
) {
    let mut ptr: gpointer = (*app).arg::<gpointer>();
    if ptr.is_null() {
        safe_c2rust_g_variant_format_string_scan(*str, ::core::ptr::null::<gchar>(), str);
        return;
    }
    if safe_c2rust_g_variant_format_string_is_nnp(*str) != 0 {
        let mut nnp: *mut gpointer = ptr as *mut gpointer;
        if free != 0 && !(*nnp).is_null() {
            safe_c2rust_g_variant_valist_free_nnp(*str, *nnp);
        }
        *nnp = NULL as gpointer;
        if !value.is_null() {
            *nnp = safe_c2rust_g_variant_valist_get_nnp(str, value);
        } else {
            safe_c2rust_g_variant_format_string_scan(*str, ::core::ptr::null::<gchar>(), str);
        }
        return;
    }
    if !value.is_null() {
        let fresh38 = *str;
        *str = (*str).offset(1);
        match *fresh38 as ::core::ffi::c_int {
            98 => {
                *(ptr as *mut gboolean) = safe_c2rust_g_variant_get_boolean(value);
                return;
            }
            121 => {
                *(ptr as *mut guint8) = safe_c2rust_g_variant_get_byte(value);
                return;
            }
            110 => {
                *(ptr as *mut gint16) = safe_c2rust_g_variant_get_int16(value);
                return;
            }
            113 => {
                *(ptr as *mut guint16) = safe_c2rust_g_variant_get_uint16(value);
                return;
            }
            105 => {
                *(ptr as *mut gint32) = safe_c2rust_g_variant_get_int32(value);
                return;
            }
            117 => {
                *(ptr as *mut guint32) = safe_c2rust_g_variant_get_uint32(value);
                return;
            }
            120 => {
                *(ptr as *mut gint64) = safe_c2rust_g_variant_get_int64(value);
                return;
            }
            116 => {
                *(ptr as *mut guint64) = safe_c2rust_g_variant_get_uint64(value);
                return;
            }
            104 => {
                *(ptr as *mut gint32) = safe_c2rust_g_variant_get_handle(value);
                return;
            }
            100 => {
                *(ptr as *mut gdouble) = safe_c2rust_g_variant_get_double(value);
                return;
            }
            _ => {}
        }
    } else {
        let fresh39 = *str;
        *str = (*str).offset(1);
        match *fresh39 as ::core::ffi::c_int {
            121 => {
                *(ptr as *mut guint8) = 0 as guint8;
                return;
            }
            110 | 113 => {
                *(ptr as *mut guint16) = 0 as guint16;
                return;
            }
            105 | 117 | 104 | 98 => {
                *(ptr as *mut guint32) = 0 as guint32;
                return;
            }
            120 | 116 | 100 => {
                *(ptr as *mut guint64) = 0 as guint64;
                return;
            }
            _ => {}
        }
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
        5217 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn safe_c2rust_g_variant_valist_skip(
    mut str: *mut *const gchar,
    mut app: *mut ::core::ffi::VaList,
) {
    if safe_c2rust_g_variant_format_string_is_leaf(*str) != 0 {
        safe_c2rust_g_variant_valist_skip_leaf(str, app);
    } else if **str as ::core::ffi::c_int == 'm' as i32 {
        *str = (*str).offset(1);
        if safe_c2rust_g_variant_format_string_is_nnp(*str) == 0 {
            (*app).arg::<gboolean>();
        }
        safe_c2rust_g_variant_valist_skip(str, app);
    } else {
        if ({
            let mut _g_boolean_var_146: ::core::ffi::c_int = 0;
            if **str as ::core::ffi::c_int == '(' as i32
                || **str as ::core::ffi::c_int == '{' as i32
            {
                _g_boolean_var_146 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_146 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_146
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                5239 as ::core::ffi::c_int,
                G_STRFUNC,
                b"**str == '(' || **str == '{'\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        *str = (*str).offset(1);
        while **str as ::core::ffi::c_int != ')' as i32 && **str as ::core::ffi::c_int != '}' as i32
        {
            safe_c2rust_g_variant_valist_skip(str, app);
        }
        *str = (*str).offset(1);
    };
}
unsafe extern "C" fn safe_c2rust_g_variant_valist_new(
    mut str: *mut *const gchar,
    mut app: *mut ::core::ffi::VaList,
) -> *mut GVariant {
    if safe_c2rust_g_variant_format_string_is_leaf(*str) != 0 {
        return safe_c2rust_g_variant_valist_new_leaf(str, app);
    }
    if **str as ::core::ffi::c_int == 'm' as i32 {
        let mut type_0: *mut GVariantType = ::core::ptr::null_mut::<GVariantType>();
        let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
        *str = (*str).offset(1);
        if safe_c2rust_g_variant_format_string_is_nnp(*str) != 0 {
            let mut nnp: gpointer = (*app).arg::<gpointer>();
            if !nnp.is_null() {
                value = safe_c2rust_g_variant_valist_new_nnp(str, nnp);
            } else {
                type_0 = safe_c2rust_g_variant_format_string_scan_type(
                    *str,
                    ::core::ptr::null::<gchar>(),
                    str,
                );
            }
        } else {
            let mut just: gboolean = (*app).arg::<gboolean>();
            if just != 0 {
                value = safe_c2rust_g_variant_valist_new(str, app);
            } else {
                type_0 = safe_c2rust_g_variant_format_string_scan_type(
                    *str,
                    ::core::ptr::null::<gchar>(),
                    ::core::ptr::null_mut::<*const gchar>(),
                );
                safe_c2rust_g_variant_valist_skip(str, app);
            }
        }
        value = safe_c2rust_g_variant_new_maybe(type_0, value);
        if !type_0.is_null() {
            g_variant_type_free(type_0);
        }
        return value;
    } else {
        let mut b: GVariantBuilder = _GVariantBuilder {
            u: C2RustUnnamed_0 {
                s: C2RustUnnamed_1 {
                    partial_magic: 0,
                    type_0: ::core::ptr::null::<GVariantType>(),
                    y: [0; 14],
                },
            },
        };
        if **str as ::core::ffi::c_int == '(' as i32 {
            safe_c2rust_g_variant_builder_init(&raw mut b, G_VARIANT_TYPE_TUPLE);
        } else {
            if ({
                let mut _g_boolean_var_147: ::core::ffi::c_int = 0;
                if **str as ::core::ffi::c_int == '{' as i32 {
                    _g_boolean_var_147 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_147 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_147
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                    5298 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"**str == '{'\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            safe_c2rust_g_variant_builder_init(&raw mut b, G_VARIANT_TYPE_DICT_ENTRY);
        }
        *str = (*str).offset(1);
        while **str as ::core::ffi::c_int != ')' as i32 && **str as ::core::ffi::c_int != '}' as i32
        {
            safe_c2rust_g_variant_builder_add_value(
                &raw mut b,
                safe_c2rust_g_variant_valist_new(str, app),
            );
        }
        *str = (*str).offset(1);
        return safe_c2rust_g_variant_builder_end(&raw mut b);
    };
}
unsafe extern "C" fn safe_c2rust_g_variant_valist_get(
    mut str: *mut *const gchar,
    mut value: *mut GVariant,
    mut free: gboolean,
    mut app: *mut ::core::ffi::VaList,
) {
    if safe_c2rust_g_variant_format_string_is_leaf(*str) != 0 {
        safe_c2rust_g_variant_valist_get_leaf(str, value, free, app);
    } else if **str as ::core::ffi::c_int == 'm' as i32 {
        *str = (*str).offset(1);
        if !value.is_null() {
            value = safe_c2rust_g_variant_get_maybe(value);
        }
        if safe_c2rust_g_variant_format_string_is_nnp(*str) == 0 {
            let mut ptr: *mut gboolean = (*app).arg::<*mut gboolean>();
            if !ptr.is_null() {
                *ptr = (value != NULL as *mut GVariant) as ::core::ffi::c_int as gboolean;
            }
        }
        safe_c2rust_g_variant_valist_get(str, value, free, app);
        if !value.is_null() {
            g_variant_unref(value);
        }
    } else {
        let mut index: gint = 0 as gint;
        if ({
            let mut _g_boolean_var_148: ::core::ffi::c_int = 0;
            if **str as ::core::ffi::c_int == '(' as i32
                || **str as ::core::ffi::c_int == '{' as i32
            {
                _g_boolean_var_148 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_148 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_148
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
                5345 as ::core::ffi::c_int,
                G_STRFUNC,
                b"**str == '(' || **str == '{'\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        *str = (*str).offset(1);
        while **str as ::core::ffi::c_int != ')' as i32 && **str as ::core::ffi::c_int != '}' as i32
        {
            if !value.is_null() {
                let fresh37 = index;
                index = index + 1;
                let mut child: *mut GVariant = g_variant_get_child_value(value, fresh37 as gsize);
                safe_c2rust_g_variant_valist_get(str, child, free, app);
                g_variant_unref(child);
            } else {
                safe_c2rust_g_variant_valist_get(
                    str,
                    ::core::ptr::null_mut::<GVariant>(),
                    free,
                    app,
                );
            }
        }
        *str = (*str).offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new(
    mut format_string: *const gchar,
    mut args: ...
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_149: ::core::ffi::c_int = 0;
        if safe_c2rust_valid_format_string(
            format_string,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            ::core::ptr::null_mut::<GVariant>(),
        ) != 0
            && *format_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '?' as i32
            && *format_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '@' as i32
            && *format_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '*' as i32
            && *format_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 'r' as i32
        {
            _g_boolean_var_149 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_149 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_149
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_format_string (format_string, TRUE, NULL) && format_string[0] != '?' && format_string[0] != '@' && format_string[0] != '*' && format_string[0] != 'r'\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    ap = args.clone();
    value = safe_c2rust_g_variant_new_va(
        format_string,
        ::core::ptr::null_mut::<*const gchar>(),
        &raw mut ap,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_va(
    mut format_string: *const gchar,
    mut endptr: *mut *const gchar,
    mut app: *mut ::core::ffi::VaList,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_150: ::core::ffi::c_int = 0;
        if safe_c2rust_valid_format_string(
            format_string,
            endptr.is_null() as ::core::ffi::c_int,
            ::core::ptr::null_mut::<GVariant>(),
        ) != 0
        {
            _g_boolean_var_150 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_150 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_150
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_format_string (format_string, !endptr, NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_151: ::core::ffi::c_int = 0;
        if !app.is_null() {
            _g_boolean_var_151 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_151 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_151
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"app != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    value = safe_c2rust_g_variant_valist_new(&raw mut format_string, app);
    if !endptr.is_null() {
        *endptr = format_string;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get(
    mut value: *mut GVariant,
    mut format_string: *const gchar,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_152: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_152 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_152 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_152
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_153: ::core::ffi::c_int = 0;
        if safe_c2rust_valid_format_string(
            format_string,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            value,
        ) != 0
        {
            _g_boolean_var_153 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_153 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_153
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_format_string (format_string, TRUE, value)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if !strchr(format_string as *const ::core::ffi::c_char, '&' as i32).is_null() {
        g_variant_get_data(value);
    }
    ap = args.clone();
    safe_c2rust_g_variant_get_va(
        value,
        format_string,
        ::core::ptr::null_mut::<*const gchar>(),
        &raw mut ap,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_va(
    mut value: *mut GVariant,
    mut format_string: *const gchar,
    mut endptr: *mut *const gchar,
    mut app: *mut ::core::ffi::VaList,
) {
    if ({
        let mut _g_boolean_var_154: ::core::ffi::c_int = 0;
        if safe_c2rust_valid_format_string(
            format_string,
            endptr.is_null() as ::core::ffi::c_int,
            value,
        ) != 0
        {
            _g_boolean_var_154 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_154 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_154
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_format_string (format_string, !endptr, value)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_155: ::core::ffi::c_int = 0;
        if !value.is_null() {
            _g_boolean_var_155 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_155 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_155
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_156: ::core::ffi::c_int = 0;
        if !app.is_null() {
            _g_boolean_var_156 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_156 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_156
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"app != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !strchr(format_string as *const ::core::ffi::c_char, '&' as i32).is_null() {
        g_variant_get_data(value);
    }
    safe_c2rust_g_variant_valist_get(&raw mut format_string, value, FALSE, app);
    if !endptr.is_null() {
        *endptr = format_string;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_builder_add(
    mut builder: *mut GVariantBuilder,
    mut format_string: *const gchar,
    mut args: ...
) {
    let mut variant: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ap: ::core::ffi::VaList;
    ap = args.clone();
    variant = safe_c2rust_g_variant_new_va(
        format_string,
        ::core::ptr::null_mut::<*const gchar>(),
        &raw mut ap,
    );
    safe_c2rust_g_variant_builder_add_value(builder, variant);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_child(
    mut value: *mut GVariant,
    mut index_: gsize,
    mut format_string: *const gchar,
    mut args: ...
) {
    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ap: ::core::ffi::VaList;
    if !strchr(format_string as *const ::core::ffi::c_char, '&' as i32).is_null() {
        g_variant_get_data(value);
    }
    child = g_variant_get_child_value(value, index_);
    if ({
        let mut _g_boolean_var_157: ::core::ffi::c_int = 0;
        if safe_c2rust_valid_format_string(
            format_string,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            child,
        ) != 0
        {
            _g_boolean_var_157 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_157 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_157
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_format_string (format_string, TRUE, child)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    ap = args.clone();
    safe_c2rust_g_variant_get_va(
        child,
        format_string,
        ::core::ptr::null_mut::<*const gchar>(),
        &raw mut ap,
    );
    g_variant_unref(child);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_iter_next(
    mut iter: *mut GVariantIter,
    mut format_string: *const gchar,
    mut args: ...
) -> gboolean {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    value = safe_c2rust_g_variant_iter_next_value(iter);
    if ({
        let mut _g_boolean_var_158: ::core::ffi::c_int = 0;
        if safe_c2rust_valid_format_string(
            format_string,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            value,
        ) != 0
        {
            _g_boolean_var_158 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_158 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_158
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"valid_format_string (format_string, TRUE, value)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !value.is_null() {
        let mut ap: ::core::ffi::VaList;
        ap = args.clone();
        safe_c2rust_g_variant_valist_get(&raw mut format_string, value, FALSE, &raw mut ap);
        g_variant_unref(value);
    }
    return (value != NULL as *mut GVariant) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_iter_loop(
    mut iter: *mut GVariantIter,
    mut format_string: *const gchar,
    mut args: ...
) -> gboolean {
    let mut first_time: gboolean =
        ((*(iter as *mut stack_iter)).loop_format == NULL as *const gchar) as ::core::ffi::c_int;
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ap: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_159: ::core::ffi::c_int = 0;
        if first_time != 0 || format_string == (*(iter as *mut stack_iter)).loop_format {
            _g_boolean_var_159 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_159 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_159
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"first_time || format_string == GVSI(iter)->loop_format\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if first_time != 0 {
        if ({
            let mut _g_boolean_var_160: ::core::ffi::c_int = 0;
            if safe_c2rust_g_variant_is_of_type(
                (*(iter as *mut stack_iter)).value,
                b"a*\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
            ) == 0
            {
                _g_boolean_var_160 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_160 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_160
        }) as ::core::ffi::c_long
            != 0
        {
            g_return_if_fail_warning(
                G_LOG_DOMAIN.as_ptr(),
                G_STRFUNC,
                b"g_variant_is_of_type (GVSI(iter)->value, G_VARIANT_TYPE_ARRAY)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return 0 as gboolean;
        }
        let ref mut fresh90 = (*(iter as *mut stack_iter)).loop_format;
        *fresh90 = format_string;
        if !strchr(format_string as *const ::core::ffi::c_char, '&' as i32).is_null() {
            g_variant_get_data((*(iter as *mut stack_iter)).value);
        }
    }
    value = safe_c2rust_g_variant_iter_next_value(iter);
    if ({
        let mut _g_boolean_var_161: ::core::ffi::c_int = 0;
        if first_time == 0
            || safe_c2rust_valid_format_string(
                format_string,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                value,
            ) != 0
        {
            _g_boolean_var_161 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_161 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_161
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!first_time || valid_format_string (format_string, TRUE, value)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ap = args.clone();
    safe_c2rust_g_variant_valist_get(
        &raw mut format_string,
        value,
        (first_time == 0) as ::core::ffi::c_int,
        &raw mut ap,
    );
    if !value.is_null() {
        g_variant_unref(value);
    }
    return (value != NULL as *mut GVariant) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_variant_deep_copy(
    mut value: *mut GVariant,
    mut byteswap: gboolean,
) -> *mut GVariant {
    match safe_c2rust_g_variant_classify(value) as ::core::ffi::c_uint {
        109 | 40 | 123 | 118 => {
            let mut builder: GVariantBuilder = _GVariantBuilder {
                u: C2RustUnnamed_0 {
                    s: C2RustUnnamed_1 {
                        partial_magic: 0,
                        type_0: ::core::ptr::null::<GVariantType>(),
                        y: [0; 14],
                    },
                },
            };
            let mut i: gsize = 0;
            let mut n_children: gsize = 0;
            safe_c2rust_g_variant_builder_init(
                &raw mut builder,
                safe_c2rust_g_variant_get_type(value),
            );
            i = 0 as gsize;
            n_children = g_variant_n_children(value);
            while i < n_children {
                let mut child: *mut GVariant = g_variant_get_child_value(value, i);
                safe_c2rust_g_variant_builder_add_value(
                    &raw mut builder,
                    safe_c2rust_g_variant_deep_copy(child, byteswap),
                );
                g_variant_unref(child);
                i = i.wrapping_add(1);
            }
            return safe_c2rust_g_variant_builder_end(&raw mut builder);
        }
        97 => {
            let mut builder_0: GVariantBuilder = _GVariantBuilder {
                u: C2RustUnnamed_0 {
                    s: C2RustUnnamed_1 {
                        partial_magic: 0,
                        type_0: ::core::ptr::null::<GVariantType>(),
                        y: [0; 14],
                    },
                },
            };
            let mut i_0: gsize = 0;
            let mut n_children_0: gsize = 0;
            let mut first_invalid_child_deep_copy: *mut GVariant =
                ::core::ptr::null_mut::<GVariant>();
            safe_c2rust_g_variant_builder_init(
                &raw mut builder_0,
                safe_c2rust_g_variant_get_type(value),
            );
            i_0 = 0 as gsize;
            n_children_0 = g_variant_n_children(value);
            while i_0 < n_children_0 {
                let mut child_0: *mut GVariant = g_variant_maybe_get_child_value(value, i_0);
                if !child_0.is_null() {
                    safe_c2rust_g_variant_builder_add_value(
                        &raw mut builder_0,
                        safe_c2rust_g_variant_deep_copy(child_0, byteswap),
                    );
                } else if child_0.is_null() && !first_invalid_child_deep_copy.is_null() {
                    safe_c2rust_g_variant_builder_add_value(
                        &raw mut builder_0,
                        first_invalid_child_deep_copy,
                    );
                } else if child_0.is_null() {
                    child_0 = g_variant_get_child_value(value, i_0);
                    first_invalid_child_deep_copy =
                        g_variant_ref_sink(safe_c2rust_g_variant_deep_copy(child_0, byteswap));
                    safe_c2rust_g_variant_builder_add_value(
                        &raw mut builder_0,
                        first_invalid_child_deep_copy,
                    );
                }
                let mut _pp: *mut *mut GVariant = &raw mut child_0;
                let mut _ptr: *mut GVariant = *_pp;
                *_pp = ::core::ptr::null_mut::<GVariant>();
                if !_ptr.is_null() {
                    g_variant_unref(_ptr as *mut GVariant);
                }
                i_0 = i_0.wrapping_add(1);
            }
            let mut _pp_0: *mut *mut GVariant = &raw mut first_invalid_child_deep_copy;
            let mut _ptr_0: *mut GVariant = *_pp_0;
            *_pp_0 = ::core::ptr::null_mut::<GVariant>();
            if !_ptr_0.is_null() {
                g_variant_unref(_ptr_0 as *mut GVariant);
            }
            return safe_c2rust_g_variant_builder_end(&raw mut builder_0);
        }
        98 => {
            return safe_c2rust_g_variant_new_boolean(safe_c2rust_g_variant_get_boolean(value));
        }
        121 => {
            return safe_c2rust_g_variant_new_byte(safe_c2rust_g_variant_get_byte(value));
        }
        110 => {
            if byteswap != 0 {
                return safe_c2rust_g_variant_new_int16(
                    ((safe_c2rust_g_variant_get_int16(value) as guint16 as ::core::ffi::c_int
                        >> 8 as ::core::ffi::c_int) as guint16
                        as ::core::ffi::c_int
                        | ((safe_c2rust_g_variant_get_int16(value) as guint16
                            as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int) as guint16
                            as ::core::ffi::c_int) as guint16 as gint16,
                );
            } else {
                return safe_c2rust_g_variant_new_int16(safe_c2rust_g_variant_get_int16(value));
            }
        }
        113 => {
            if byteswap != 0 {
                return safe_c2rust_g_variant_new_uint16(
                    ((safe_c2rust_g_variant_get_uint16(value) as ::core::ffi::c_int
                        >> 8 as ::core::ffi::c_int) as guint16
                        as ::core::ffi::c_int
                        | ((safe_c2rust_g_variant_get_uint16(value) as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int) as guint16
                            as ::core::ffi::c_int) as guint16,
                );
            } else {
                return safe_c2rust_g_variant_new_uint16(safe_c2rust_g_variant_get_uint16(value));
            }
        }
        105 => {
            if byteswap != 0 {
                return safe_c2rust_g_variant_new_int32(
                    ({
                        let mut __v: guint32 = 0;
                        let mut __x: guint32 = safe_c2rust_g_variant_get_int32(value) as guint32;
                        if 0 != 0 {
                            __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                                | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                                | (__x & 0xff0000 as ::core::ffi::c_uint)
                                    >> 8 as ::core::ffi::c_int
                                | (__x & 0xff000000 as ::core::ffi::c_uint)
                                    >> 24 as ::core::ffi::c_int;
                        } else {
                            let fresh49 = &mut __v;
                            let fresh50;
                            let fresh51 = __x;
                            asm!(
                                "bswapl {0:e}\n", inlateout(reg)
                                c2rust_asm_casts::AsmCast::cast_in(fresh49, fresh51) =>
                                fresh50, options(preserves_flags, pure, readonly,
                                att_syntax)
                            );
                            c2rust_asm_casts::AsmCast::cast_out(fresh49, fresh51, fresh50);
                        }
                        __v
                    }) as gint32,
                );
            } else {
                return safe_c2rust_g_variant_new_int32(safe_c2rust_g_variant_get_int32(value));
            }
        }
        117 => {
            if byteswap != 0 {
                return safe_c2rust_g_variant_new_uint32(
                    ({
                        let mut __v: guint32 = 0;
                        let mut __x: guint32 = safe_c2rust_g_variant_get_uint32(value);
                        if 0 != 0 {
                            __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                                | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                                | (__x & 0xff0000 as ::core::ffi::c_uint)
                                    >> 8 as ::core::ffi::c_int
                                | (__x & 0xff000000 as ::core::ffi::c_uint)
                                    >> 24 as ::core::ffi::c_int;
                        } else {
                            let fresh52 = &mut __v;
                            let fresh53;
                            let fresh54 = __x;
                            asm!(
                                "bswapl {0:e}\n", inlateout(reg)
                                c2rust_asm_casts::AsmCast::cast_in(fresh52, fresh54) =>
                                fresh53, options(preserves_flags, pure, readonly,
                                att_syntax)
                            );
                            c2rust_asm_casts::AsmCast::cast_out(fresh52, fresh54, fresh53);
                        }
                        __v
                    }),
                );
            } else {
                return safe_c2rust_g_variant_new_uint32(safe_c2rust_g_variant_get_uint32(value));
            }
        }
        120 => {
            if byteswap != 0 {
                return safe_c2rust_g_variant_new_int64(
                    ({
                        let mut __v: guint64 = 0;
                        let mut __x: guint64 = safe_c2rust_g_variant_get_int64(value) as guint64;
                        if 0 != 0 {
                            __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                                | (__x & 0xff00 as ::core::ffi::c_ulong)
                                    << 40 as ::core::ffi::c_int
                                | (__x & 0xff0000 as ::core::ffi::c_ulong)
                                    << 24 as ::core::ffi::c_int
                                | (__x & 0xff000000 as ::core::ffi::c_ulong)
                                    << 8 as ::core::ffi::c_int
                                | (__x & 0xff00000000 as ::core::ffi::c_ulong)
                                    >> 8 as ::core::ffi::c_int
                                | (__x & 0xff0000000000 as ::core::ffi::c_ulong)
                                    >> 24 as ::core::ffi::c_int
                                | (__x & 0xff000000000000 as ::core::ffi::c_ulong)
                                    >> 40 as ::core::ffi::c_int
                                | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                                    >> 56 as ::core::ffi::c_int;
                        } else {
                            let fresh55 = &mut __v;
                            let fresh56;
                            let fresh57 = __x;
                            asm!(
                                "bswapq {0}\n", inlateout(reg)
                                c2rust_asm_casts::AsmCast::cast_in(fresh55, fresh57) =>
                                fresh56, options(preserves_flags, pure, readonly,
                                att_syntax)
                            );
                            c2rust_asm_casts::AsmCast::cast_out(fresh55, fresh57, fresh56);
                        }
                        __v
                    }) as gint64,
                );
            } else {
                return safe_c2rust_g_variant_new_int64(safe_c2rust_g_variant_get_int64(value));
            }
        }
        116 => {
            if byteswap != 0 {
                return safe_c2rust_g_variant_new_uint64(
                    ({
                        let mut __v: guint64 = 0;
                        let mut __x: guint64 = safe_c2rust_g_variant_get_uint64(value);
                        if 0 != 0 {
                            __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                                | (__x & 0xff00 as ::core::ffi::c_ulong)
                                    << 40 as ::core::ffi::c_int
                                | (__x & 0xff0000 as ::core::ffi::c_ulong)
                                    << 24 as ::core::ffi::c_int
                                | (__x & 0xff000000 as ::core::ffi::c_ulong)
                                    << 8 as ::core::ffi::c_int
                                | (__x & 0xff00000000 as ::core::ffi::c_ulong)
                                    >> 8 as ::core::ffi::c_int
                                | (__x & 0xff0000000000 as ::core::ffi::c_ulong)
                                    >> 24 as ::core::ffi::c_int
                                | (__x & 0xff000000000000 as ::core::ffi::c_ulong)
                                    >> 40 as ::core::ffi::c_int
                                | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                                    >> 56 as ::core::ffi::c_int;
                        } else {
                            let fresh58 = &mut __v;
                            let fresh59;
                            let fresh60 = __x;
                            asm!(
                                "bswapq {0}\n", inlateout(reg)
                                c2rust_asm_casts::AsmCast::cast_in(fresh58, fresh60) =>
                                fresh59, options(preserves_flags, pure, readonly,
                                att_syntax)
                            );
                            c2rust_asm_casts::AsmCast::cast_out(fresh58, fresh60, fresh59);
                        }
                        __v
                    }),
                );
            } else {
                return safe_c2rust_g_variant_new_uint64(safe_c2rust_g_variant_get_uint64(value));
            }
        }
        104 => {
            if byteswap != 0 {
                return safe_c2rust_g_variant_new_handle(
                    ({
                        let mut __v: guint32 = 0;
                        let mut __x: guint32 = safe_c2rust_g_variant_get_handle(value) as guint32;
                        if 0 != 0 {
                            __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                                | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                                | (__x & 0xff0000 as ::core::ffi::c_uint)
                                    >> 8 as ::core::ffi::c_int
                                | (__x & 0xff000000 as ::core::ffi::c_uint)
                                    >> 24 as ::core::ffi::c_int;
                        } else {
                            let fresh61 = &mut __v;
                            let fresh62;
                            let fresh63 = __x;
                            asm!(
                                "bswapl {0:e}\n", inlateout(reg)
                                c2rust_asm_casts::AsmCast::cast_in(fresh61, fresh63) =>
                                fresh62, options(preserves_flags, pure, readonly,
                                att_syntax)
                            );
                            c2rust_asm_casts::AsmCast::cast_out(fresh61, fresh63, fresh62);
                        }
                        __v
                    }) as gint32,
                );
            } else {
                return safe_c2rust_g_variant_new_handle(safe_c2rust_g_variant_get_handle(value));
            }
        }
        100 => {
            if byteswap != 0 {
                let mut u1: C2RustUnnamed = C2RustUnnamed { u64_0: 0 };
                let mut u2: C2RustUnnamed = C2RustUnnamed { u64_0: 0 };
                u1.dbl = safe_c2rust_g_variant_get_double(value);
                u2.u64_0 = ({
                    let mut __v: guint64 = 0;
                    let mut __x: guint64 = u1.u64_0;
                    if 0 != 0 {
                        __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                            | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                            | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                            | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                            | (__x & 0xff00000000 as ::core::ffi::c_ulong)
                                >> 8 as ::core::ffi::c_int
                            | (__x & 0xff0000000000 as ::core::ffi::c_ulong)
                                >> 24 as ::core::ffi::c_int
                            | (__x & 0xff000000000000 as ::core::ffi::c_ulong)
                                >> 40 as ::core::ffi::c_int
                            | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                                >> 56 as ::core::ffi::c_int;
                    } else {
                        let fresh64 = &mut __v;
                        let fresh65;
                        let fresh66 = __x;
                        asm!(
                            "bswapq {0}\n", inlateout(reg)
                            c2rust_asm_casts::AsmCast::cast_in(fresh64, fresh66) =>
                            fresh65, options(preserves_flags, pure, readonly, att_syntax)
                        );
                        c2rust_asm_casts::AsmCast::cast_out(fresh64, fresh66, fresh65);
                    }
                    __v
                });
                return safe_c2rust_g_variant_new_double(u2.dbl);
            } else {
                return safe_c2rust_g_variant_new_double(safe_c2rust_g_variant_get_double(value));
            }
        }
        115 => {
            return safe_c2rust_g_variant_new_string(safe_c2rust_g_variant_get_string(
                value,
                ::core::ptr::null_mut::<gsize>(),
            ));
        }
        111 => {
            return safe_c2rust_g_variant_new_object_path(safe_c2rust_g_variant_get_string(
                value,
                ::core::ptr::null_mut::<gsize>(),
            ));
        }
        103 => {
            return safe_c2rust_g_variant_new_signature(safe_c2rust_g_variant_get_string(
                value,
                ::core::ptr::null_mut::<gsize>(),
            ));
        }
        _ => {}
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
        6042 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_get_normal_form(
    mut value: *mut GVariant,
) -> *mut GVariant {
    let mut trusted: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if g_variant_is_normal_form(value) != 0 {
        return g_variant_ref(value);
    }
    trusted = safe_c2rust_g_variant_deep_copy(value, FALSE);
    if ({
        let mut _g_boolean_var_162: ::core::ffi::c_int = 0;
        if g_variant_is_trusted(trusted) != 0 {
            _g_boolean_var_162 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_162 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_162
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
            6088 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_variant_is_trusted (trusted)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return g_variant_ref_sink(trusted);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_byteswap(mut value: *mut GVariant) -> *mut GVariant {
    let mut type_info: *mut GVariantTypeInfo = ::core::ptr::null_mut::<GVariantTypeInfo>();
    let mut alignment: guint = 0;
    let mut new: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    type_info = g_variant_get_type_info(value);
    g_variant_type_info_query(
        type_info,
        &raw mut alignment,
        ::core::ptr::null_mut::<gsize>(),
    );
    if alignment != 0 && g_variant_is_normal_form(value) != 0 {
        let mut serialised: GVariantSerialised = GVariantSerialised {
            type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
            data: ::core::ptr::null_mut::<guchar>(),
            size: 0,
            depth: 0,
            ordered_offsets_up_to: 0,
            checked_offsets_up_to: 0,
        };
        let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
        serialised.type_info = g_variant_get_type_info(value);
        serialised.size = g_variant_get_size(value);
        serialised.data = g_malloc(serialised.size) as *mut guchar;
        serialised.depth = g_variant_get_depth(value);
        serialised.ordered_offsets_up_to = G_MAXSIZE as gsize;
        serialised.checked_offsets_up_to = G_MAXSIZE as gsize;
        g_variant_store(value, serialised.data as gpointer);
        g_variant_serialised_byteswap(serialised);
        bytes = g_bytes_new_take(serialised.data as gpointer, serialised.size);
        new = g_variant_ref_sink(g_variant_new_from_bytes(
            safe_c2rust_g_variant_get_type(value),
            bytes,
            TRUE,
        ));
        g_bytes_unref(bytes);
    } else if alignment != 0 {
        new = g_variant_ref_sink(safe_c2rust_g_variant_deep_copy(value, TRUE));
    } else {
        new = safe_c2rust_g_variant_get_normal_form(value);
    }
    if ({
        let mut _g_boolean_var_163: ::core::ffi::c_int = 0;
        if g_variant_is_trusted(new) != 0 {
            _g_boolean_var_163 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_163 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_163
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant.c\0" as *const u8 as *const ::core::ffi::c_char,
            6159 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_variant_is_trusted (new)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return safe_c2rust_g_steal_pointer(&raw mut new as gpointer) as *mut GVariant;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_new_from_data(
    mut type_0: *const GVariantType,
    mut data: gconstpointer,
    mut size: gsize,
    mut trusted: gboolean,
    mut notify: GDestroyNotify,
    mut user_data: gpointer,
) -> *mut GVariant {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut bytes: *mut GBytes = ::core::ptr::null_mut::<GBytes>();
    if ({
        let mut _g_boolean_var_164: ::core::ffi::c_int = 0;
        if g_variant_type_is_definite(type_0) != 0 {
            _g_boolean_var_164 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_164 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_164
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"g_variant_type_is_definite (type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_165: ::core::ffi::c_int = 0;
        if !data.is_null() || size == 0 as gsize {
            _g_boolean_var_165 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_165 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_165
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"data != NULL || size == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if notify.is_some() {
        bytes = g_bytes_new_with_free_func(data, size, notify, user_data);
    } else {
        bytes = g_bytes_new_static(data, size);
    }
    value = g_variant_new_from_bytes(type_0, bytes, trusted);
    g_bytes_unref(bytes);
    return value;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_variant_get_type\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
