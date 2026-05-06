use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    pub type _GVariantType;
    pub type _GVariantTypeInfo;
    fn g_variant_type_string_scan(
        string: *const gchar,
        limit: *const gchar,
        endptr: *mut *const gchar,
    ) -> gboolean;
    fn g_variant_type_is_definite(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_info_get_type_string(typeinfo: *mut GVariantTypeInfo) -> *const gchar;
    fn g_variant_type_info_query(
        typeinfo: *mut GVariantTypeInfo,
        alignment: *mut guint,
        size: *mut gsize,
    );
    fn g_variant_type_info_query_depth(typeinfo: *mut GVariantTypeInfo) -> gsize;
    fn g_variant_type_info_element(typeinfo: *mut GVariantTypeInfo) -> *mut GVariantTypeInfo;
    fn g_variant_type_info_query_element(
        typeinfo: *mut GVariantTypeInfo,
        alignment: *mut guint,
        size: *mut gsize,
    );
    fn g_variant_type_info_n_members(typeinfo: *mut GVariantTypeInfo) -> gsize;
    fn g_variant_type_info_member_info(
        typeinfo: *mut GVariantTypeInfo,
        index: gsize,
    ) -> *const GVariantMemberInfo;
    fn g_variant_type_info_get(type_0: *const GVariantType) -> *mut GVariantTypeInfo;
    fn g_variant_type_info_ref(typeinfo: *mut GVariantTypeInfo) -> *mut GVariantTypeInfo;
    fn g_variant_type_info_unref(typeinfo: *mut GVariantTypeInfo);
    fn g_utf8_validate_len(str: *const gchar, max_len: gsize, end: *mut *const gchar) -> gboolean;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
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
}
pub type size_t = usize;
pub type gint8 = ::core::ffi::c_schar;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GVariantType = _GVariantType;
pub type GVariantTypeInfo = _GVariantTypeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVariantMemberInfo {
    pub type_info: *mut GVariantTypeInfo,
    pub i: gsize,
    pub a: gsize,
    pub b: gint8,
    pub c: gint8,
    pub ending_type: guint8,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Offsets {
    pub data_size: gsize,
    pub array: *mut guchar,
    pub length: gsize,
    pub offset_size: guint,
    pub is_normal: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub bytes: [guchar; 8],
    pub integer: gsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub a: ::core::ffi::c_char,
    pub b: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub x: guint64,
    pub y: *mut ::core::ffi::c_void,
    pub z: gdouble,
}
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
pub type GVariantSerialisedFiller =
    Option<unsafe extern "C" fn(*mut GVariantSerialised, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub bytes: [guchar; 8],
    pub integer: gsize,
}
pub const G_ASCII_ALNUM: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_3 = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed_3 = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed_3 = 512;
pub const G_ASCII_SPACE: C2RustUnnamed_3 = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed_3 = 128;
pub const G_ASCII_PRINT: C2RustUnnamed_3 = 64;
pub const G_ASCII_LOWER: C2RustUnnamed_3 = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed_3 = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed_3 = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed_3 = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed_3 = 2;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXUINT8: guint8 = 0xff as ::core::ffi::c_int as guint8;
pub const G_MAXUINT16: guint16 = 0xffff as ::core::ffi::c_int as guint16;
pub const G_MAXUINT32: guint32 = 0xffffffff as ::core::ffi::c_uint;
pub const G_VARIANT_TYPE_UNIT: *const GVariantType =
    b"()\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_INFO_CHAR_MAYBE: ::core::ffi::c_int = 109;
pub const G_VARIANT_TYPE_INFO_CHAR_ARRAY: ::core::ffi::c_int = 97;
pub const G_VARIANT_TYPE_INFO_CHAR_TUPLE: ::core::ffi::c_int = 40;
pub const G_VARIANT_TYPE_INFO_CHAR_DICT_ENTRY: ::core::ffi::c_int = 123;
pub const G_VARIANT_TYPE_INFO_CHAR_VARIANT: ::core::ffi::c_int = 118;
pub const G_VARIANT_MEMBER_ENDING_FIXED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_VARIANT_MEMBER_ENDING_LAST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const G_VARIANT_MEMBER_ENDING_OFFSET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_VARIANT_MAX_RECURSION_DEPTH: gsize = 128 as ::core::ffi::c_int as gsize;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_serialised_check(
    mut serialised: GVariantSerialised,
) -> gboolean {
    let mut fixed_size: gsize = 0;
    let mut alignment: guint = 0;
    if serialised.type_info.is_null() {
        return FALSE;
    }
    g_variant_type_info_query(
        serialised.type_info,
        &raw mut alignment,
        &raw mut fixed_size,
    );
    if fixed_size != 0 as gsize && serialised.size != fixed_size {
        return FALSE;
    } else if fixed_size == 0 as gsize
        && !(serialised.size == 0 as gsize || !serialised.data.is_null())
    {
        return FALSE;
    }
    if serialised.ordered_offsets_up_to > serialised.checked_offsets_up_to {
        return FALSE;
    }
    alignment = (alignment as ::core::ffi::c_ulong
        & (::core::mem::size_of::<C2RustUnnamed_0>() as usize).wrapping_sub(9 as usize)
            as ::core::ffi::c_ulong) as guint;
    return (serialised.size <= alignment as gsize
        || alignment as gsize & serialised.data as gsize == 0 as gsize)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_gvs_fixed_sized_maybe_n_children(
    mut value: GVariantSerialised,
) -> gsize {
    let mut element_fixed_size: gsize = 0;
    g_variant_type_info_query_element(
        value.type_info,
        ::core::ptr::null_mut::<guint>(),
        &raw mut element_fixed_size,
    );
    return (if element_fixed_size == value.size {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as gsize;
}
unsafe extern "C" fn safe_c2rust_gvs_fixed_sized_maybe_get_child(
    mut value: GVariantSerialised,
    mut index_: gsize,
) -> GVariantSerialised {
    value.type_info = g_variant_type_info_element(value.type_info);
    g_variant_type_info_ref(value.type_info);
    value.depth = value.depth.wrapping_add(1);
    value.ordered_offsets_up_to = 0 as gsize;
    value.checked_offsets_up_to = 0 as gsize;
    return value;
}
unsafe extern "C" fn safe_c2rust_gvs_fixed_sized_maybe_needed_size(
    mut type_info: *mut GVariantTypeInfo,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) -> gsize {
    if n_children != 0 {
        let mut element_fixed_size: gsize = 0;
        g_variant_type_info_query_element(
            type_info,
            ::core::ptr::null_mut::<guint>(),
            &raw mut element_fixed_size,
        );
        return element_fixed_size;
    } else {
        return 0 as gsize;
    };
}
unsafe extern "C" fn safe_c2rust_gvs_fixed_sized_maybe_serialise(
    mut value: GVariantSerialised,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) {
    if n_children != 0 {
        let mut child: GVariantSerialised = GVariantSerialised {
            type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
            data: value.data,
            size: value.size,
            depth: value.depth.wrapping_add(1 as gsize),
            ordered_offsets_up_to: 0 as gsize,
            checked_offsets_up_to: 0 as gsize,
        };
        gvs_filler.expect("non-null function pointer")(
            &raw mut child,
            *children.offset(0 as ::core::ffi::c_int as isize),
        );
    }
}
unsafe extern "C" fn safe_c2rust_gvs_fixed_sized_maybe_is_normal(
    mut value: GVariantSerialised,
) -> gboolean {
    if value.size > 0 as gsize {
        let mut element_fixed_size: gsize = 0;
        g_variant_type_info_query_element(
            value.type_info,
            ::core::ptr::null_mut::<guint>(),
            &raw mut element_fixed_size,
        );
        if value.size != element_fixed_size {
            return FALSE;
        }
        value.type_info = g_variant_type_info_element(value.type_info);
        value.depth = value.depth.wrapping_add(1);
        value.ordered_offsets_up_to = 0 as gsize;
        value.checked_offsets_up_to = 0 as gsize;
        return safe_c2rust_g_variant_serialised_is_normal(value);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_maybe_n_children(
    mut value: GVariantSerialised,
) -> gsize {
    return (if value.size > 0 as gsize {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as gsize;
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_maybe_get_child(
    mut value: GVariantSerialised,
    mut index_: gsize,
) -> GVariantSerialised {
    value.type_info = g_variant_type_info_element(value.type_info);
    g_variant_type_info_ref(value.type_info);
    value.size = value.size.wrapping_sub(1);
    if value.size == 0 as gsize {
        value.data = ::core::ptr::null_mut::<guchar>();
    }
    value.depth = value.depth.wrapping_add(1);
    value.ordered_offsets_up_to = 0 as gsize;
    value.checked_offsets_up_to = 0 as gsize;
    return value;
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_maybe_needed_size(
    mut type_info: *mut GVariantTypeInfo,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) -> gsize {
    if n_children != 0 {
        let mut child: GVariantSerialised = GVariantSerialised {
            type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
            data: ::core::ptr::null_mut::<guchar>(),
            size: 0,
            depth: 0,
            ordered_offsets_up_to: 0,
            checked_offsets_up_to: 0,
        };
        gvs_filler.expect("non-null function pointer")(
            &raw mut child,
            *children.offset(0 as ::core::ffi::c_int as isize),
        );
        return child.size.wrapping_add(1 as gsize);
    } else {
        return 0 as gsize;
    };
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_maybe_serialise(
    mut value: GVariantSerialised,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) {
    if n_children != 0 {
        let mut child: GVariantSerialised = GVariantSerialised {
            type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
            data: value.data,
            size: value.size.wrapping_sub(1 as gsize),
            depth: value.depth.wrapping_add(1 as gsize),
            ordered_offsets_up_to: 0 as gsize,
            checked_offsets_up_to: 0 as gsize,
        };
        gvs_filler.expect("non-null function pointer")(
            &raw mut child,
            *children.offset(0 as ::core::ffi::c_int as isize),
        );
        *value.data.offset(child.size as isize) = '\0' as i32 as guchar;
    }
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_maybe_is_normal(
    mut value: GVariantSerialised,
) -> gboolean {
    if value.size == 0 as gsize {
        return TRUE;
    }
    if *value
        .data
        .offset(value.size.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
        != '\0' as i32
    {
        return FALSE;
    }
    value.type_info = g_variant_type_info_element(value.type_info);
    value.size = value.size.wrapping_sub(1);
    value.depth = value.depth.wrapping_add(1);
    value.ordered_offsets_up_to = 0 as gsize;
    value.checked_offsets_up_to = 0 as gsize;
    return safe_c2rust_g_variant_serialised_is_normal(value);
}
unsafe extern "C" fn safe_c2rust_gvs_fixed_sized_array_n_children(
    mut value: GVariantSerialised,
) -> gsize {
    let mut element_fixed_size: gsize = 0;
    g_variant_type_info_query_element(
        value.type_info,
        ::core::ptr::null_mut::<guint>(),
        &raw mut element_fixed_size,
    );
    if value.size.wrapping_rem(element_fixed_size) == 0 as gsize {
        return value.size.wrapping_div(element_fixed_size);
    }
    return 0 as gsize;
}
unsafe extern "C" fn safe_c2rust_gvs_fixed_sized_array_get_child(
    mut value: GVariantSerialised,
    mut index_: gsize,
) -> GVariantSerialised {
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    child.type_info = g_variant_type_info_element(value.type_info);
    g_variant_type_info_query(
        child.type_info,
        ::core::ptr::null_mut::<guint>(),
        &raw mut child.size,
    );
    child.data = value.data.offset(child.size.wrapping_mul(index_) as isize);
    g_variant_type_info_ref(child.type_info);
    child.depth = value.depth.wrapping_add(1 as gsize);
    return child;
}
unsafe extern "C" fn safe_c2rust_gvs_fixed_sized_array_needed_size(
    mut type_info: *mut GVariantTypeInfo,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) -> gsize {
    let mut element_fixed_size: gsize = 0;
    g_variant_type_info_query_element(
        type_info,
        ::core::ptr::null_mut::<guint>(),
        &raw mut element_fixed_size,
    );
    return element_fixed_size.wrapping_mul(n_children);
}
unsafe extern "C" fn safe_c2rust_gvs_fixed_sized_array_serialise(
    mut value: GVariantSerialised,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) {
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    let mut i: gsize = 0;
    child.type_info = g_variant_type_info_element(value.type_info);
    g_variant_type_info_query(
        child.type_info,
        ::core::ptr::null_mut::<guint>(),
        &raw mut child.size,
    );
    child.data = value.data;
    child.depth = value.depth.wrapping_add(1 as gsize);
    i = 0 as gsize;
    while i < n_children {
        gvs_filler.expect("non-null function pointer")(
            &raw mut child,
            *children.offset(i as isize),
        );
        child.data = child.data.offset(child.size as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn safe_c2rust_gvs_fixed_sized_array_is_normal(
    mut value: GVariantSerialised,
) -> gboolean {
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    child.type_info = g_variant_type_info_element(value.type_info);
    g_variant_type_info_query(
        child.type_info,
        ::core::ptr::null_mut::<guint>(),
        &raw mut child.size,
    );
    child.depth = value.depth.wrapping_add(1 as gsize);
    if value.size.wrapping_rem(child.size) != 0 as gsize {
        return FALSE;
    }
    child.data = value.data;
    while child.data < value.data.offset(value.size as isize) {
        if safe_c2rust_g_variant_serialised_is_normal(child) == 0 {
            return FALSE;
        }
        child.data = child.data.offset(child.size as isize);
    }
    return TRUE;
}
#[inline]
unsafe extern "C" fn safe_c2rust_gvs_read_unaligned_le(
    mut bytes: *mut guchar,
    mut size: guint,
) -> gsize {
    let mut tmpvalue: C2RustUnnamed = C2RustUnnamed { bytes: [0; 8] };
    tmpvalue.integer = 0 as gsize;
    if !bytes.is_null() {
        memcpy(
            &raw mut tmpvalue.bytes as *mut ::core::ffi::c_void,
            bytes as *const ::core::ffi::c_void,
            size as size_t,
        );
    }
    return tmpvalue.integer;
}
#[inline]
unsafe extern "C" fn safe_c2rust_gvs_write_unaligned_le(
    mut bytes: *mut guchar,
    mut value: gsize,
    mut size: guint,
) {
    let mut tmpvalue: C2RustUnnamed_2 = C2RustUnnamed_2 { bytes: [0; 8] };
    tmpvalue.integer = value;
    memcpy(
        bytes as *mut ::core::ffi::c_void,
        &raw mut tmpvalue.bytes as *const ::core::ffi::c_void,
        size as size_t,
    );
}
unsafe extern "C" fn safe_c2rust_gvs_get_offset_size(mut size: gsize) -> guint {
    if size > G_MAXUINT32 as gsize {
        return 8 as guint;
    } else if size > G_MAXUINT16 as gsize {
        return 4 as guint;
    } else if size > G_MAXUINT8 as gsize {
        return 2 as guint;
    } else if size > 0 as gsize {
        return 1 as guint;
    }
    return 0 as guint;
}
unsafe extern "C" fn safe_c2rust_gvs_calculate_total_size(
    mut body_size: gsize,
    mut offsets: gsize,
) -> gsize {
    if body_size.wrapping_add((1 as gsize).wrapping_mul(offsets)) <= G_MAXUINT8 as gsize {
        return body_size.wrapping_add((1 as gsize).wrapping_mul(offsets));
    }
    if body_size.wrapping_add((2 as gsize).wrapping_mul(offsets)) <= G_MAXUINT16 as gsize {
        return body_size.wrapping_add((2 as gsize).wrapping_mul(offsets));
    }
    if body_size.wrapping_add((4 as gsize).wrapping_mul(offsets)) <= G_MAXUINT32 as gsize {
        return body_size.wrapping_add((4 as gsize).wrapping_mul(offsets));
    }
    return body_size.wrapping_add((8 as gsize).wrapping_mul(offsets));
}
unsafe extern "C" fn safe_c2rust_gvs_offsets_get_offset_n(
    mut offsets: *mut Offsets,
    mut n: gsize,
) -> gsize {
    return safe_c2rust_gvs_read_unaligned_le(
        (*offsets)
            .array
            .offset(((*offsets).offset_size as gsize).wrapping_mul(n) as isize),
        (*offsets).offset_size,
    );
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_array_get_frame_offsets(
    mut value: GVariantSerialised,
) -> Offsets {
    let mut out: Offsets = Offsets {
        data_size: 0 as gsize,
        array: ::core::ptr::null_mut::<guchar>(),
        length: 0,
        offset_size: 0,
        is_normal: 0,
    };
    let mut offsets_array_size: gsize = 0;
    let mut last_end: gsize = 0;
    if value.size == 0 as gsize {
        out.is_normal = TRUE as gboolean;
        return out;
    }
    out.offset_size = safe_c2rust_gvs_get_offset_size(value.size);
    last_end = safe_c2rust_gvs_read_unaligned_le(
        value
            .data
            .offset(value.size as isize)
            .offset(-(out.offset_size as isize)),
        out.offset_size,
    );
    if last_end > value.size {
        return out;
    }
    offsets_array_size = value.size.wrapping_sub(last_end);
    if offsets_array_size.wrapping_rem(out.offset_size as gsize) != 0 {
        return out;
    }
    out.data_size = last_end;
    out.array = value.data.offset(last_end as isize);
    out.length = offsets_array_size.wrapping_div(out.offset_size as gsize);
    if out.length > 0 as gsize
        && safe_c2rust_gvs_calculate_total_size(last_end, out.length) != value.size
    {
        return out;
    }
    out.is_normal = TRUE as gboolean;
    return out;
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_array_n_children(
    mut value: GVariantSerialised,
) -> gsize {
    return safe_c2rust_gvs_variable_sized_array_get_frame_offsets(value).length;
}
unsafe extern "C" fn safe_c2rust_find_unordered_guint8(
    mut data: *const guint8,
    mut start: gsize,
    mut len: gsize,
) -> gsize {
    let mut off: gsize = 0;
    let mut current_le: guint8 = 0;
    let mut previous_le: guint8 = 0;
    let mut current: guint8 = 0;
    let mut previous: guint8 = 0;
    memcpy(
        &raw mut previous_le as *mut ::core::ffi::c_void,
        data.offset(
            (start as usize).wrapping_mul(::core::mem::size_of::<guint8>() as usize) as isize,
        ) as *const ::core::ffi::c_void,
        ::core::mem::size_of::<guint8>() as size_t,
    );
    previous = previous_le;
    off = (start as usize)
        .wrapping_add(1 as usize)
        .wrapping_mul(::core::mem::size_of::<guint8>() as usize) as gsize;
    while (off as usize) < (len as usize).wrapping_mul(::core::mem::size_of::<guint8>() as usize) {
        memcpy(
            &raw mut current_le as *mut ::core::ffi::c_void,
            data.offset(off as isize) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<guint8>() as size_t,
        );
        current = current_le;
        if (current as ::core::ffi::c_int) < previous as ::core::ffi::c_int {
            break;
        }
        previous = current;
        off = (off as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<guint8>() as usize as ::core::ffi::c_ulong)
            as gsize as gsize;
    }
    return off
        .wrapping_div(::core::mem::size_of::<guint8>() as gsize)
        .wrapping_sub(1 as gsize);
}
unsafe extern "C" fn safe_c2rust_find_unordered_guint16(
    mut data: *const guint8,
    mut start: gsize,
    mut len: gsize,
) -> gsize {
    let mut off: gsize = 0;
    let mut current_le: guint16 = 0;
    let mut previous_le: guint16 = 0;
    let mut current: guint16 = 0;
    let mut previous: guint16 = 0;
    memcpy(
        &raw mut previous_le as *mut ::core::ffi::c_void,
        data.offset(
            (start as usize).wrapping_mul(::core::mem::size_of::<guint16>() as usize) as isize,
        ) as *const ::core::ffi::c_void,
        ::core::mem::size_of::<guint16>() as size_t,
    );
    previous = previous_le;
    off = (start as usize)
        .wrapping_add(1 as usize)
        .wrapping_mul(::core::mem::size_of::<guint16>() as usize) as gsize;
    while (off as usize) < (len as usize).wrapping_mul(::core::mem::size_of::<guint16>() as usize) {
        memcpy(
            &raw mut current_le as *mut ::core::ffi::c_void,
            data.offset(off as isize) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<guint16>() as size_t,
        );
        current = current_le;
        if (current as ::core::ffi::c_int) < previous as ::core::ffi::c_int {
            break;
        }
        previous = current;
        off = (off as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<guint16>() as usize as ::core::ffi::c_ulong)
            as gsize as gsize;
    }
    return off
        .wrapping_div(::core::mem::size_of::<guint16>() as gsize)
        .wrapping_sub(1 as gsize);
}
unsafe extern "C" fn safe_c2rust_find_unordered_guint32(
    mut data: *const guint8,
    mut start: gsize,
    mut len: gsize,
) -> gsize {
    let mut off: gsize = 0;
    let mut current_le: guint32 = 0;
    let mut previous_le: guint32 = 0;
    let mut current: guint32 = 0;
    let mut previous: guint32 = 0;
    memcpy(
        &raw mut previous_le as *mut ::core::ffi::c_void,
        data.offset(
            (start as usize).wrapping_mul(::core::mem::size_of::<guint32>() as usize) as isize,
        ) as *const ::core::ffi::c_void,
        ::core::mem::size_of::<guint32>() as size_t,
    );
    previous = previous_le;
    off = (start as usize)
        .wrapping_add(1 as usize)
        .wrapping_mul(::core::mem::size_of::<guint32>() as usize) as gsize;
    while (off as usize) < (len as usize).wrapping_mul(::core::mem::size_of::<guint32>() as usize) {
        memcpy(
            &raw mut current_le as *mut ::core::ffi::c_void,
            data.offset(off as isize) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<guint32>() as size_t,
        );
        current = current_le;
        if current < previous {
            break;
        }
        previous = current;
        off = (off as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<guint32>() as usize as ::core::ffi::c_ulong)
            as gsize as gsize;
    }
    return off
        .wrapping_div(::core::mem::size_of::<guint32>() as gsize)
        .wrapping_sub(1 as gsize);
}
unsafe extern "C" fn safe_c2rust_find_unordered_guint64(
    mut data: *const guint8,
    mut start: gsize,
    mut len: gsize,
) -> gsize {
    let mut off: gsize = 0;
    let mut current_le: guint64 = 0;
    let mut previous_le: guint64 = 0;
    let mut current: guint64 = 0;
    let mut previous: guint64 = 0;
    memcpy(
        &raw mut previous_le as *mut ::core::ffi::c_void,
        data.offset(
            (start as usize).wrapping_mul(::core::mem::size_of::<guint64>() as usize) as isize,
        ) as *const ::core::ffi::c_void,
        ::core::mem::size_of::<guint64>() as size_t,
    );
    previous = previous_le;
    off = (start as usize)
        .wrapping_add(1 as usize)
        .wrapping_mul(::core::mem::size_of::<guint64>() as usize) as gsize;
    while (off as usize) < (len as usize).wrapping_mul(::core::mem::size_of::<guint64>() as usize) {
        memcpy(
            &raw mut current_le as *mut ::core::ffi::c_void,
            data.offset(off as isize) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<guint64>() as size_t,
        );
        current = current_le;
        if current < previous {
            break;
        }
        previous = current;
        off = (off as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<guint64>() as usize as ::core::ffi::c_ulong)
            as gsize as gsize;
    }
    return off
        .wrapping_div(::core::mem::size_of::<guint64>() as gsize)
        .wrapping_sub(1 as gsize);
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_array_get_child(
    mut value: GVariantSerialised,
    mut index_: gsize,
) -> GVariantSerialised {
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    let mut offsets: Offsets = safe_c2rust_gvs_variable_sized_array_get_frame_offsets(value);
    let mut start: gsize = 0;
    let mut end: gsize = 0;
    child.type_info = g_variant_type_info_element(value.type_info);
    g_variant_type_info_ref(child.type_info);
    child.depth = value.depth.wrapping_add(1 as gsize);
    if index_ > value.checked_offsets_up_to
        && value.ordered_offsets_up_to == value.checked_offsets_up_to
    {
        match offsets.offset_size {
            1 => {
                value.ordered_offsets_up_to = safe_c2rust_find_unordered_guint8(
                    offsets.array,
                    value.checked_offsets_up_to,
                    index_.wrapping_add(1 as gsize),
                );
            }
            2 => {
                value.ordered_offsets_up_to = safe_c2rust_find_unordered_guint16(
                    offsets.array,
                    value.checked_offsets_up_to,
                    index_.wrapping_add(1 as gsize),
                );
            }
            4 => {
                value.ordered_offsets_up_to = safe_c2rust_find_unordered_guint32(
                    offsets.array,
                    value.checked_offsets_up_to,
                    index_.wrapping_add(1 as gsize),
                );
            }
            8 => {
                value.ordered_offsets_up_to = safe_c2rust_find_unordered_guint64(
                    offsets.array,
                    value.checked_offsets_up_to,
                    index_.wrapping_add(1 as gsize),
                );
            }
            _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gvariant-serialiser.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    796 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
        value.checked_offsets_up_to = index_;
    }
    if index_ > value.ordered_offsets_up_to {
        return child;
    }
    if index_ > 0 as gsize {
        let mut alignment: guint = 0;
        start =
            safe_c2rust_gvs_offsets_get_offset_n(&raw mut offsets, index_.wrapping_sub(1 as gsize));
        g_variant_type_info_query(
            child.type_info,
            &raw mut alignment,
            ::core::ptr::null_mut::<gsize>(),
        );
        start = start.wrapping_add(start.wrapping_neg() & alignment as gsize);
    } else {
        start = 0 as gsize;
    }
    end = safe_c2rust_gvs_offsets_get_offset_n(&raw mut offsets, index_);
    if start < end && end <= value.size && end <= offsets.data_size {
        child.data = value.data.offset(start as isize);
        child.size = end.wrapping_sub(start);
    }
    return child;
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_array_needed_size(
    mut type_info: *mut GVariantTypeInfo,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) -> gsize {
    let mut alignment: guint = 0;
    let mut offset: gsize = 0;
    let mut i: gsize = 0;
    g_variant_type_info_query(
        type_info,
        &raw mut alignment,
        ::core::ptr::null_mut::<gsize>(),
    );
    offset = 0 as gsize;
    i = 0 as gsize;
    while i < n_children {
        let mut child: GVariantSerialised = GVariantSerialised {
            type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
            data: ::core::ptr::null_mut::<guchar>(),
            size: 0,
            depth: 0,
            ordered_offsets_up_to: 0,
            checked_offsets_up_to: 0,
        };
        offset = offset.wrapping_add(offset.wrapping_neg() & alignment as gsize);
        gvs_filler.expect("non-null function pointer")(
            &raw mut child,
            *children.offset(i as isize),
        );
        offset = offset.wrapping_add(child.size);
        i = i.wrapping_add(1);
    }
    return safe_c2rust_gvs_calculate_total_size(offset, n_children);
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_array_serialise(
    mut value: GVariantSerialised,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) {
    let mut offset_ptr: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut offset_size: gsize = 0;
    let mut alignment: guint = 0;
    let mut offset: gsize = 0;
    let mut i: gsize = 0;
    g_variant_type_info_query(
        value.type_info,
        &raw mut alignment,
        ::core::ptr::null_mut::<gsize>(),
    );
    offset_size = safe_c2rust_gvs_get_offset_size(value.size) as gsize;
    offset = 0 as gsize;
    offset_ptr = value
        .data
        .offset(value.size as isize)
        .offset(-(offset_size.wrapping_mul(n_children) as isize));
    i = 0 as gsize;
    while i < n_children {
        let mut child: GVariantSerialised = GVariantSerialised {
            type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
            data: ::core::ptr::null_mut::<guchar>(),
            size: 0,
            depth: 0,
            ordered_offsets_up_to: 0,
            checked_offsets_up_to: 0,
        };
        while offset & alignment as gsize != 0 {
            let fresh2 = offset;
            offset = offset.wrapping_add(1);
            *value.data.offset(fresh2 as isize) = '\0' as i32 as guchar;
        }
        child.data = value.data.offset(offset as isize);
        gvs_filler.expect("non-null function pointer")(
            &raw mut child,
            *children.offset(i as isize),
        );
        offset = offset.wrapping_add(child.size);
        safe_c2rust_gvs_write_unaligned_le(offset_ptr, offset, offset_size as guint);
        offset_ptr = offset_ptr.offset(offset_size as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn safe_c2rust_gvs_variable_sized_array_is_normal(
    mut value: GVariantSerialised,
) -> gboolean {
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    let mut alignment: guint = 0;
    let mut offset: gsize = 0;
    let mut i: gsize = 0;
    let mut offsets: Offsets = safe_c2rust_gvs_variable_sized_array_get_frame_offsets(value);
    if offsets.is_normal == 0 {
        return FALSE;
    }
    if value.size != 0 as gsize && offsets.length == 0 as gsize {
        return FALSE;
    }
    child.type_info = g_variant_type_info_element(value.type_info);
    g_variant_type_info_query(
        child.type_info,
        &raw mut alignment,
        ::core::ptr::null_mut::<gsize>(),
    );
    child.depth = value.depth.wrapping_add(1 as gsize);
    offset = 0 as gsize;
    i = 0 as gsize;
    while i < offsets.length {
        let mut this_end: gsize = 0;
        this_end = safe_c2rust_gvs_read_unaligned_le(
            offsets
                .array
                .offset((offsets.offset_size as gsize).wrapping_mul(i) as isize),
            offsets.offset_size,
        );
        if this_end < offset || this_end > offsets.data_size {
            return FALSE;
        }
        while offset & alignment as gsize != 0 {
            if !(offset < this_end
                && *value.data.offset(offset as isize) as ::core::ffi::c_int == '\0' as i32)
            {
                return FALSE;
            }
            offset = offset.wrapping_add(1);
        }
        child.data = value.data.offset(offset as isize);
        child.size = this_end.wrapping_sub(offset);
        if child.size == 0 as gsize {
            child.data = ::core::ptr::null_mut::<guchar>();
        }
        if safe_c2rust_g_variant_serialised_is_normal(child) == 0 {
            return FALSE;
        }
        offset = this_end;
        i = i.wrapping_add(1);
    }
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if offset == offsets.data_size {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-serialiser.c\0" as *const u8 as *const ::core::ffi::c_char,
            940 as ::core::ffi::c_int,
            G_STRFUNC,
            b"offset == offsets.data_size\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    value.ordered_offsets_up_to = G_MAXSIZE as gsize;
    value.checked_offsets_up_to = G_MAXSIZE as gsize;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_gvs_tuple_get_member_bounds(
    mut value: GVariantSerialised,
    mut index_: gsize,
    mut offset_size: gsize,
    mut out_member_start: *mut gsize,
    mut out_member_end: *mut gsize,
) {
    let mut member_info: *const GVariantMemberInfo = ::core::ptr::null::<GVariantMemberInfo>();
    let mut member_start: gsize = 0;
    let mut member_end: gsize = 0;
    member_info = g_variant_type_info_member_info(value.type_info, index_);
    if (*member_info).i.wrapping_add(1 as gsize) != 0
        && offset_size.wrapping_mul((*member_info).i.wrapping_add(1 as gsize)) <= value.size
    {
        member_start = safe_c2rust_gvs_read_unaligned_le(
            value.data.offset(value.size as isize).offset(
                -(offset_size.wrapping_mul((*member_info).i.wrapping_add(1 as gsize)) as isize),
            ),
            offset_size as guint,
        );
    } else {
        member_start = 0 as gsize;
    }
    member_start = member_start.wrapping_add((*member_info).a);
    member_start &= (*member_info).b as gsize;
    member_start |= (*member_info).c as gsize;
    if (*member_info).ending_type as ::core::ffi::c_int == G_VARIANT_MEMBER_ENDING_LAST
        && offset_size.wrapping_mul((*member_info).i.wrapping_add(1 as gsize)) <= value.size
    {
        member_end = value
            .size
            .wrapping_sub(offset_size.wrapping_mul((*member_info).i.wrapping_add(1 as gsize)));
    } else if (*member_info).ending_type as ::core::ffi::c_int == G_VARIANT_MEMBER_ENDING_FIXED {
        let mut fixed_size: gsize = 0;
        g_variant_type_info_query(
            (*member_info).type_info,
            ::core::ptr::null_mut::<guint>(),
            &raw mut fixed_size,
        );
        member_end = member_start.wrapping_add(fixed_size);
    } else if (*member_info).ending_type as ::core::ffi::c_int == G_VARIANT_MEMBER_ENDING_OFFSET
        && offset_size.wrapping_mul((*member_info).i.wrapping_add(2 as gsize)) <= value.size
    {
        member_end = safe_c2rust_gvs_read_unaligned_le(
            value.data.offset(value.size as isize).offset(
                -(offset_size.wrapping_mul((*member_info).i.wrapping_add(2 as gsize)) as isize),
            ),
            offset_size as guint,
        );
    } else {
        member_end = G_MAXSIZE as gsize;
    }
    if !out_member_start.is_null() {
        *out_member_start = member_start;
    }
    if !out_member_end.is_null() {
        *out_member_end = member_end;
    }
}
unsafe extern "C" fn safe_c2rust_gvs_tuple_n_children(mut value: GVariantSerialised) -> gsize {
    return g_variant_type_info_n_members(value.type_info);
}
unsafe extern "C" fn safe_c2rust_gvs_tuple_get_child(
    mut value: GVariantSerialised,
    mut index_: gsize,
) -> GVariantSerialised {
    let mut member_info: *const GVariantMemberInfo = ::core::ptr::null::<GVariantMemberInfo>();
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    let mut offset_size: gsize = 0;
    let mut start: gsize = 0;
    let mut end: gsize = 0;
    let mut last_end: gsize = 0;
    member_info = g_variant_type_info_member_info(value.type_info, index_);
    child.type_info = g_variant_type_info_ref((*member_info).type_info);
    child.depth = value.depth.wrapping_add(1 as gsize);
    offset_size = safe_c2rust_gvs_get_offset_size(value.size) as gsize;
    if (*member_info).ending_type as ::core::ffi::c_int == G_VARIANT_MEMBER_ENDING_FIXED {
        g_variant_type_info_query(
            child.type_info,
            ::core::ptr::null_mut::<guint>(),
            &raw mut child.size,
        );
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if value.data.is_null() && value.size != 0 as gsize {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
        if ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if child.size != 0 as gsize {
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
                b"../original/glib/gvariant-serialiser.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1061 as ::core::ffi::c_int,
                G_STRFUNC,
                b"child.size != 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        child.data = ::core::ptr::null_mut::<guchar>();
        return child;
    }
    if index_ > value.checked_offsets_up_to
        && value.ordered_offsets_up_to == value.checked_offsets_up_to
    {
        let mut i: gsize = 0;
        let mut prev_i_end: gsize = 0 as gsize;
        if value.checked_offsets_up_to > 0 as gsize {
            safe_c2rust_gvs_tuple_get_member_bounds(
                value,
                value.checked_offsets_up_to.wrapping_sub(1 as gsize),
                offset_size,
                ::core::ptr::null_mut::<gsize>(),
                &raw mut prev_i_end,
            );
        }
        i = value.checked_offsets_up_to;
        while i <= index_ {
            let mut i_start: gsize = 0;
            let mut i_end: gsize = 0;
            safe_c2rust_gvs_tuple_get_member_bounds(
                value,
                i,
                offset_size,
                &raw mut i_start,
                &raw mut i_end,
            );
            if i_start > i_end || i_start < prev_i_end || i_end > value.size {
                break;
            }
            prev_i_end = i_end;
            i = i.wrapping_add(1);
        }
        value.ordered_offsets_up_to = i.wrapping_sub(1 as gsize);
        value.checked_offsets_up_to = index_;
    }
    if index_ > value.ordered_offsets_up_to {
        return child;
    }
    if (*member_info).ending_type as ::core::ffi::c_int == G_VARIANT_MEMBER_ENDING_OFFSET {
        if offset_size.wrapping_mul((*member_info).i.wrapping_add(2 as gsize)) > value.size {
            return child;
        }
    } else if offset_size.wrapping_mul((*member_info).i.wrapping_add(1 as gsize)) > value.size {
        return child;
    }
    safe_c2rust_gvs_tuple_get_member_bounds(
        value,
        index_,
        offset_size,
        &raw mut start,
        &raw mut end,
    );
    safe_c2rust_gvs_tuple_get_member_bounds(
        value,
        g_variant_type_info_n_members(value.type_info).wrapping_sub(1 as gsize),
        offset_size,
        ::core::ptr::null_mut::<gsize>(),
        &raw mut last_end,
    );
    if start < end && end <= value.size && end <= last_end {
        child.data = value.data.offset(start as isize);
        child.size = end.wrapping_sub(start);
    }
    return child;
}
unsafe extern "C" fn safe_c2rust_gvs_tuple_needed_size(
    mut type_info: *mut GVariantTypeInfo,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) -> gsize {
    let mut member_info: *const GVariantMemberInfo = ::core::ptr::null::<GVariantMemberInfo>();
    let mut fixed_size: gsize = 0;
    let mut offset: gsize = 0;
    let mut i: gsize = 0;
    g_variant_type_info_query(
        type_info,
        ::core::ptr::null_mut::<guint>(),
        &raw mut fixed_size,
    );
    if fixed_size != 0 {
        return fixed_size;
    }
    offset = 0 as gsize;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if n_children > 0 as gsize {
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
            b"../original/glib/gvariant-serialiser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1149 as ::core::ffi::c_int,
            G_STRFUNC,
            b"n_children > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    i = 0 as gsize;
    while i < n_children {
        let mut alignment: guint = 0;
        member_info = g_variant_type_info_member_info(type_info, i);
        g_variant_type_info_query(
            (*member_info).type_info,
            &raw mut alignment,
            &raw mut fixed_size,
        );
        offset = offset.wrapping_add(offset.wrapping_neg() & alignment as gsize);
        if fixed_size != 0 {
            offset = offset.wrapping_add(fixed_size);
        } else {
            let mut child: GVariantSerialised = GVariantSerialised {
                type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
                data: ::core::ptr::null_mut::<guchar>(),
                size: 0,
                depth: 0,
                ordered_offsets_up_to: 0,
                checked_offsets_up_to: 0,
            };
            gvs_filler.expect("non-null function pointer")(
                &raw mut child,
                *children.offset(i as isize),
            );
            offset = offset.wrapping_add(child.size);
        }
        i = i.wrapping_add(1);
    }
    return safe_c2rust_gvs_calculate_total_size(offset, (*member_info).i.wrapping_add(1 as gsize));
}
unsafe extern "C" fn safe_c2rust_gvs_tuple_serialise(
    mut value: GVariantSerialised,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) {
    let mut offset_size: gsize = 0;
    let mut offset: gsize = 0;
    let mut i: gsize = 0;
    offset_size = safe_c2rust_gvs_get_offset_size(value.size) as gsize;
    offset = 0 as gsize;
    i = 0 as gsize;
    while i < n_children {
        let mut member_info: *const GVariantMemberInfo = ::core::ptr::null::<GVariantMemberInfo>();
        let mut child: GVariantSerialised = GVariantSerialised {
            type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
            data: ::core::ptr::null_mut::<guchar>(),
            size: 0,
            depth: 0,
            ordered_offsets_up_to: 0,
            checked_offsets_up_to: 0,
        };
        let mut alignment: guint = 0;
        member_info = g_variant_type_info_member_info(value.type_info, i);
        g_variant_type_info_query(
            (*member_info).type_info,
            &raw mut alignment,
            ::core::ptr::null_mut::<gsize>(),
        );
        while offset & alignment as gsize != 0 {
            let fresh0 = offset;
            offset = offset.wrapping_add(1);
            *value.data.offset(fresh0 as isize) = '\0' as i32 as guchar;
        }
        child.data = value.data.offset(offset as isize);
        gvs_filler.expect("non-null function pointer")(
            &raw mut child,
            *children.offset(i as isize),
        );
        offset = offset.wrapping_add(child.size);
        if (*member_info).ending_type as ::core::ffi::c_int == G_VARIANT_MEMBER_ENDING_OFFSET {
            value.size = value.size.wrapping_sub(offset_size);
            safe_c2rust_gvs_write_unaligned_le(
                value.data.offset(value.size as isize),
                offset,
                offset_size as guint,
            );
        }
        i = i.wrapping_add(1);
    }
    while offset < value.size {
        let fresh1 = offset;
        offset = offset.wrapping_add(1);
        *value.data.offset(fresh1 as isize) = '\0' as i32 as guchar;
    }
}
unsafe extern "C" fn safe_c2rust_gvs_tuple_is_normal(mut value: GVariantSerialised) -> gboolean {
    let mut offset_size: guint = 0;
    let mut offset_ptr: gsize = 0;
    let mut length: gsize = 0;
    let mut offset: gsize = 0;
    let mut i: gsize = 0;
    let mut offset_table_size: gsize = 0;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if value.data.is_null() && value.size != 0 as gsize {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
        return FALSE;
    }
    offset_size = safe_c2rust_gvs_get_offset_size(value.size);
    length = g_variant_type_info_n_members(value.type_info);
    offset_ptr = value.size;
    offset = 0 as gsize;
    i = 0 as gsize;
    while i < length {
        let mut member_info: *const GVariantMemberInfo = ::core::ptr::null::<GVariantMemberInfo>();
        let mut child: GVariantSerialised = GVariantSerialised {
            type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
            data: ::core::ptr::null_mut::<guchar>(),
            size: 0,
            depth: 0,
            ordered_offsets_up_to: 0,
            checked_offsets_up_to: 0,
        };
        let mut fixed_size: gsize = 0;
        let mut alignment: guint = 0;
        let mut end: gsize = 0;
        member_info = g_variant_type_info_member_info(value.type_info, i);
        child.type_info = (*member_info).type_info;
        child.depth = value.depth.wrapping_add(1 as gsize);
        g_variant_type_info_query(child.type_info, &raw mut alignment, &raw mut fixed_size);
        while offset & alignment as gsize != 0 {
            if offset > value.size
                || *value.data.offset(offset as isize) as ::core::ffi::c_int != '\0' as i32
            {
                return FALSE;
            }
            offset = offset.wrapping_add(1);
        }
        child.data = value.data.offset(offset as isize);
        match (*member_info).ending_type as ::core::ffi::c_int {
            G_VARIANT_MEMBER_ENDING_FIXED => {
                end = offset.wrapping_add(fixed_size);
            }
            G_VARIANT_MEMBER_ENDING_LAST => {
                end = offset_ptr;
            }
            G_VARIANT_MEMBER_ENDING_OFFSET => {
                if offset_ptr < offset_size as gsize {
                    return FALSE;
                }
                offset_ptr = offset_ptr.wrapping_sub(offset_size as gsize);
                if offset_ptr < offset {
                    return FALSE;
                }
                end = safe_c2rust_gvs_read_unaligned_le(
                    value.data.offset(offset_ptr as isize),
                    offset_size,
                );
            }
            _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gvariant-serialiser.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1280 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
        if end < offset || end > offset_ptr {
            return FALSE;
        }
        child.size = end.wrapping_sub(offset);
        if child.size == 0 as gsize {
            child.data = ::core::ptr::null_mut::<guchar>();
        }
        if safe_c2rust_g_variant_serialised_is_normal(child) == 0 {
            return FALSE;
        }
        offset = end;
        i = i.wrapping_add(1);
    }
    value.ordered_offsets_up_to = G_MAXSIZE as gsize;
    value.checked_offsets_up_to = G_MAXSIZE as gsize;
    let mut fixed_size_0: gsize = 0;
    let mut alignment_0: guint = 0;
    g_variant_type_info_query(value.type_info, &raw mut alignment_0, &raw mut fixed_size_0);
    if fixed_size_0 != 0 {
        if ({
            let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
            if fixed_size_0 == value.size {
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
                b"../original/glib/gvariant-serialiser.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1309 as ::core::ffi::c_int,
                G_STRFUNC,
                b"fixed_size == value.size\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
            if offset_ptr == value.size {
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
                b"../original/glib/gvariant-serialiser.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1310 as ::core::ffi::c_int,
                G_STRFUNC,
                b"offset_ptr == value.size\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if i == 0 as gsize {
            let fresh3 = offset;
            offset = offset.wrapping_add(1);
            if *value.data.offset(fresh3 as isize) as ::core::ffi::c_int != '\0' as i32 {
                return FALSE;
            }
        } else {
            while offset & alignment_0 as gsize != 0 {
                let fresh4 = offset;
                offset = offset.wrapping_add(1);
                if *value.data.offset(fresh4 as isize) as ::core::ffi::c_int != '\0' as i32 {
                    return FALSE;
                }
            }
        }
        if ({
            let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
            if offset == value.size {
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
                b"../original/glib/gvariant-serialiser.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1324 as ::core::ffi::c_int,
                G_STRFUNC,
                b"offset == value.size\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if offset_ptr != offset {
        return FALSE;
    }
    offset_table_size = value.size.wrapping_sub(offset_ptr);
    if value.size > 0 as gsize
        && safe_c2rust_gvs_calculate_total_size(
            offset,
            offset_table_size.wrapping_div(offset_size as gsize),
        ) != value.size
    {
        return FALSE;
    }
    return TRUE;
}
#[inline]
unsafe extern "C" fn safe_c2rust_gvs_variant_n_children(mut value: GVariantSerialised) -> gsize {
    return 1 as gsize;
}
#[inline]
unsafe extern "C" fn safe_c2rust_gvs_variant_get_child(
    mut value: GVariantSerialised,
    mut index_: gsize,
) -> GVariantSerialised {
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    if value.size != 0 {
        child.size = value.size.wrapping_sub(1 as gsize);
        while child.size != 0 {
            if *value.data.offset(child.size as isize) as ::core::ffi::c_int == '\0' as i32 {
                break;
            }
            child.size = child.size.wrapping_sub(1);
        }
        if *value.data.offset(child.size as isize) as ::core::ffi::c_int == '\0' as i32 {
            let mut type_string: *const gchar = value
                .data
                .offset(child.size.wrapping_add(1 as gsize) as isize)
                as *mut guchar as *mut gchar;
            let mut limit: *const gchar =
                value.data.offset(value.size as isize) as *mut guchar as *mut gchar;
            let mut end: *const gchar = ::core::ptr::null::<gchar>();
            if g_variant_type_string_scan(type_string, limit, &raw mut end) != 0 && end == limit {
                let mut type_0: *const GVariantType = type_string as *mut GVariantType;
                if g_variant_type_is_definite(type_0) != 0 {
                    let mut fixed_size: gsize = 0;
                    let mut child_type_depth: gsize = 0;
                    child.type_info = g_variant_type_info_get(type_0);
                    child.depth = value.depth.wrapping_add(1 as gsize);
                    if child.size != 0 as gsize {
                        child.data = value.data;
                    }
                    g_variant_type_info_query(
                        child.type_info,
                        ::core::ptr::null_mut::<guint>(),
                        &raw mut fixed_size,
                    );
                    child_type_depth = g_variant_type_info_query_depth(child.type_info);
                    if (fixed_size == 0 || fixed_size == child.size)
                        && value.depth
                            < G_VARIANT_MAX_RECURSION_DEPTH.wrapping_sub(child_type_depth)
                    {
                        return child;
                    }
                    g_variant_type_info_unref(child.type_info);
                }
            }
        }
    }
    child.type_info = g_variant_type_info_get(G_VARIANT_TYPE_UNIT);
    child.data = ::core::ptr::null_mut::<guchar>();
    child.size = 1 as gsize;
    child.depth = value.depth.wrapping_add(1 as gsize);
    return child;
}
#[inline]
unsafe extern "C" fn safe_c2rust_gvs_variant_needed_size(
    mut type_info: *mut GVariantTypeInfo,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) -> gsize {
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    gvs_filler.expect("non-null function pointer")(
        &raw mut child,
        *children.offset(0 as ::core::ffi::c_int as isize),
    );
    type_string = g_variant_type_info_get_type_string(child.type_info);
    return child
        .size
        .wrapping_add(1 as gsize)
        .wrapping_add(strlen(type_string as *const ::core::ffi::c_char) as gsize);
}
#[inline]
unsafe extern "C" fn safe_c2rust_gvs_variant_serialise(
    mut value: GVariantSerialised,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) {
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    child.data = value.data;
    gvs_filler.expect("non-null function pointer")(
        &raw mut child,
        *children.offset(0 as ::core::ffi::c_int as isize),
    );
    type_string = g_variant_type_info_get_type_string(child.type_info);
    *value.data.offset(child.size as isize) = '\0' as i32 as guchar;
    memcpy(
        value
            .data
            .offset(child.size as isize)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        type_string as *const ::core::ffi::c_void,
        strlen(type_string as *const ::core::ffi::c_char),
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_gvs_variant_is_normal(mut value: GVariantSerialised) -> gboolean {
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    let mut normal: gboolean = 0;
    let mut child_type_depth: gsize = 0;
    child = safe_c2rust_gvs_variant_get_child(value, 0 as gsize);
    child_type_depth = g_variant_type_info_query_depth(child.type_info);
    normal = (value.depth < G_VARIANT_MAX_RECURSION_DEPTH.wrapping_sub(child_type_depth)
        && (!child.data.is_null() || child.size == 0 as gsize)
        && safe_c2rust_g_variant_serialised_is_normal(child) != 0)
        as ::core::ffi::c_int as gboolean;
    g_variant_type_info_unref(child.type_info);
    return normal;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_serialised_n_children(
    mut serialised: GVariantSerialised,
) -> gsize {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_serialised_check(serialised) != 0 {
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
            b"../original/glib/gvariant-serialiser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1544 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_variant_serialised_check (serialised)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    match *g_variant_type_info_get_type_string(serialised.type_info)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
    {
        G_VARIANT_TYPE_INFO_CHAR_MAYBE => {
            let mut fixed_size: gsize = 0;
            g_variant_type_info_query_element(
                serialised.type_info,
                ::core::ptr::null_mut::<guint>(),
                &raw mut fixed_size,
            );
            if fixed_size != 0 {
                return safe_c2rust_gvs_fixed_sized_maybe_n_children(serialised);
            } else {
                return safe_c2rust_gvs_variable_sized_maybe_n_children(serialised);
            }
        }
        G_VARIANT_TYPE_INFO_CHAR_ARRAY => {
            let mut fixed_size_0: gsize = 0;
            g_variant_type_info_query_element(
                serialised.type_info,
                ::core::ptr::null_mut::<guint>(),
                &raw mut fixed_size_0,
            );
            if fixed_size_0 != 0 {
                return safe_c2rust_gvs_fixed_sized_array_n_children(serialised);
            } else {
                return safe_c2rust_gvs_variable_sized_array_n_children(serialised);
            }
        }
        G_VARIANT_TYPE_INFO_CHAR_DICT_ENTRY | G_VARIANT_TYPE_INFO_CHAR_TUPLE => {
            return safe_c2rust_gvs_tuple_n_children(serialised);
        }
        G_VARIANT_TYPE_INFO_CHAR_VARIANT => {
            return safe_c2rust_gvs_variant_n_children(serialised);
        }
        _ => {}
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"../original/glib/gvariant-serialiser.c\0" as *const u8 as *const ::core::ffi::c_char,
        1551 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_serialised_get_child(
    mut serialised: GVariantSerialised,
    mut index_: gsize,
) -> GVariantSerialised {
    let mut child: GVariantSerialised = GVariantSerialised {
        type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
        data: ::core::ptr::null_mut::<guchar>(),
        size: 0,
        depth: 0,
        ordered_offsets_up_to: 0,
        checked_offsets_up_to: 0,
    };
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_serialised_check(serialised) != 0 {
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
            b"../original/glib/gvariant-serialiser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1581 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_variant_serialised_check (serialised)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if index_ < safe_c2rust_g_variant_serialised_n_children(serialised) {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
        match *g_variant_type_info_get_type_string(serialised.type_info)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        {
            G_VARIANT_TYPE_INFO_CHAR_MAYBE => {
                let mut fixed_size: gsize = 0;
                g_variant_type_info_query_element(
                    serialised.type_info,
                    ::core::ptr::null_mut::<guint>(),
                    &raw mut fixed_size,
                );
                if fixed_size != 0 {
                    child = safe_c2rust_gvs_fixed_sized_maybe_get_child(serialised, index_);
                    if ({
                        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                        if child.size != 0 || child.data.is_null() {
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
                            b"../original/glib/gvariant-serialiser.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1592 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"child.size || child.data == NULL\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    if ({
                        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
                        if safe_c2rust_g_variant_serialised_check(child) != 0 {
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
                            b"../original/glib/gvariant-serialiser.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1592 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"g_variant_serialised_check (child)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    return child;
                } else {
                    child = safe_c2rust_gvs_variable_sized_maybe_get_child(serialised, index_);
                    if ({
                        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
                        if child.size != 0 || child.data.is_null() {
                            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_21
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/gvariant-serialiser.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1592 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"child.size || child.data == NULL\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    if ({
                        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                        if safe_c2rust_g_variant_serialised_check(child) != 0 {
                            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_22
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/gvariant-serialiser.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1592 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"g_variant_serialised_check (child)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    return child;
                }
            }
            G_VARIANT_TYPE_INFO_CHAR_ARRAY => {
                let mut fixed_size_0: gsize = 0;
                g_variant_type_info_query_element(
                    serialised.type_info,
                    ::core::ptr::null_mut::<guint>(),
                    &raw mut fixed_size_0,
                );
                if fixed_size_0 != 0 {
                    child = safe_c2rust_gvs_fixed_sized_array_get_child(serialised, index_);
                    if ({
                        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                        if child.size != 0 || child.data.is_null() {
                            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_23
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/gvariant-serialiser.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1592 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"child.size || child.data == NULL\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    if ({
                        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
                        if safe_c2rust_g_variant_serialised_check(child) != 0 {
                            _g_boolean_var_24 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_24 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_24
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/gvariant-serialiser.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1592 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"g_variant_serialised_check (child)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    return child;
                } else {
                    child = safe_c2rust_gvs_variable_sized_array_get_child(serialised, index_);
                    if ({
                        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
                        if child.size != 0 || child.data.is_null() {
                            _g_boolean_var_25 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_25 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_25
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/gvariant-serialiser.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1592 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"child.size || child.data == NULL\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    if ({
                        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
                        if safe_c2rust_g_variant_serialised_check(child) != 0 {
                            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_26
                    }) as ::core::ffi::c_long
                        != 0
                    {
                    } else {
                        g_assertion_message_expr(
                            G_LOG_DOMAIN.as_ptr(),
                            b"../original/glib/gvariant-serialiser.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            1592 as ::core::ffi::c_int,
                            G_STRFUNC,
                            b"g_variant_serialised_check (child)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                    return child;
                }
            }
            G_VARIANT_TYPE_INFO_CHAR_DICT_ENTRY | G_VARIANT_TYPE_INFO_CHAR_TUPLE => {
                child = safe_c2rust_gvs_tuple_get_child(serialised, index_);
                if ({
                    let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
                    if child.size != 0 || child.data.is_null() {
                        _g_boolean_var_27 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_27 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_27
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/gvariant-serialiser.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1592 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"child.size || child.data == NULL\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if ({
                    let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
                    if safe_c2rust_g_variant_serialised_check(child) != 0 {
                        _g_boolean_var_28 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_28 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_28
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/gvariant-serialiser.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1592 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"g_variant_serialised_check (child)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                return child;
            }
            G_VARIANT_TYPE_INFO_CHAR_VARIANT => {
                child = safe_c2rust_gvs_variant_get_child(serialised, index_);
                if ({
                    let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
                    if child.size != 0 || child.data.is_null() {
                        _g_boolean_var_29 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_29 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_29
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/gvariant-serialiser.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1592 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"child.size || child.data == NULL\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                if ({
                    let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
                    if safe_c2rust_g_variant_serialised_check(child) != 0 {
                        _g_boolean_var_30 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_30 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_30
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/gvariant-serialiser.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1592 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"g_variant_serialised_check (child)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                return child;
            }
            _ => {}
        }
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-serialiser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1593 as ::core::ffi::c_int,
            G_STRFUNC,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    g_log(
        G_LOG_DOMAIN.as_ptr() as *const gchar,
        G_LOG_LEVEL_ERROR,
        b"Attempt to access item %lu in a container with only %lu items\0" as *const u8
            as *const gchar,
        index_,
        safe_c2rust_g_variant_serialised_n_children(serialised),
    );
    loop {}
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_serialiser_serialise(
    mut serialised: GVariantSerialised,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_serialised_check(serialised) != 0 {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-serialiser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1630 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_variant_serialised_check (serialised)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    match *g_variant_type_info_get_type_string(serialised.type_info)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
    {
        G_VARIANT_TYPE_INFO_CHAR_MAYBE => {
            let mut fixed_size: gsize = 0;
            g_variant_type_info_query_element(
                serialised.type_info,
                ::core::ptr::null_mut::<guint>(),
                &raw mut fixed_size,
            );
            if fixed_size != 0 {
                safe_c2rust_gvs_fixed_sized_maybe_serialise(
                    serialised, gvs_filler, children, n_children,
                );
                return;
            } else {
                safe_c2rust_gvs_variable_sized_maybe_serialise(
                    serialised, gvs_filler, children, n_children,
                );
                return;
            }
        }
        G_VARIANT_TYPE_INFO_CHAR_ARRAY => {
            let mut fixed_size_0: gsize = 0;
            g_variant_type_info_query_element(
                serialised.type_info,
                ::core::ptr::null_mut::<guint>(),
                &raw mut fixed_size_0,
            );
            if fixed_size_0 != 0 {
                safe_c2rust_gvs_fixed_sized_array_serialise(
                    serialised, gvs_filler, children, n_children,
                );
                return;
            } else {
                safe_c2rust_gvs_variable_sized_array_serialise(
                    serialised, gvs_filler, children, n_children,
                );
                return;
            }
        }
        G_VARIANT_TYPE_INFO_CHAR_DICT_ENTRY | G_VARIANT_TYPE_INFO_CHAR_TUPLE => {
            safe_c2rust_gvs_tuple_serialise(serialised, gvs_filler, children, n_children);
            return;
        }
        G_VARIANT_TYPE_INFO_CHAR_VARIANT => {
            safe_c2rust_gvs_variant_serialise(serialised, gvs_filler, children, n_children);
            return;
        }
        _ => {}
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"../original/glib/gvariant-serialiser.c\0" as *const u8 as *const ::core::ffi::c_char,
        1639 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_serialiser_needed_size(
    mut type_info: *mut GVariantTypeInfo,
    mut gvs_filler: GVariantSerialisedFiller,
    mut children: *const gpointer,
    mut n_children: gsize,
) -> gsize {
    match *g_variant_type_info_get_type_string(type_info).offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
    {
        G_VARIANT_TYPE_INFO_CHAR_MAYBE => {
            let mut fixed_size: gsize = 0;
            g_variant_type_info_query_element(
                type_info,
                ::core::ptr::null_mut::<guint>(),
                &raw mut fixed_size,
            );
            if fixed_size != 0 {
                return safe_c2rust_gvs_fixed_sized_maybe_needed_size(
                    type_info, gvs_filler, children, n_children,
                );
            } else {
                return safe_c2rust_gvs_variable_sized_maybe_needed_size(
                    type_info, gvs_filler, children, n_children,
                );
            }
        }
        G_VARIANT_TYPE_INFO_CHAR_ARRAY => {
            let mut fixed_size_0: gsize = 0;
            g_variant_type_info_query_element(
                type_info,
                ::core::ptr::null_mut::<guint>(),
                &raw mut fixed_size_0,
            );
            if fixed_size_0 != 0 {
                return safe_c2rust_gvs_fixed_sized_array_needed_size(
                    type_info, gvs_filler, children, n_children,
                );
            } else {
                return safe_c2rust_gvs_variable_sized_array_needed_size(
                    type_info, gvs_filler, children, n_children,
                );
            }
        }
        G_VARIANT_TYPE_INFO_CHAR_DICT_ENTRY | G_VARIANT_TYPE_INFO_CHAR_TUPLE => {
            return safe_c2rust_gvs_tuple_needed_size(type_info, gvs_filler, children, n_children);
        }
        G_VARIANT_TYPE_INFO_CHAR_VARIANT => {
            return safe_c2rust_gvs_variant_needed_size(
                type_info, gvs_filler, children, n_children,
            );
        }
        _ => {}
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"../original/glib/gvariant-serialiser.c\0" as *const u8 as *const ::core::ffi::c_char,
        1667 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_serialised_byteswap(
    mut serialised: GVariantSerialised,
) {
    let mut fixed_size: gsize = 0;
    let mut alignment: guint = 0;
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if safe_c2rust_g_variant_serialised_check(serialised) != 0 {
            _g_boolean_var_32 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_32 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_32
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gvariant-serialiser.c\0" as *const u8 as *const ::core::ffi::c_char,
            1685 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_variant_serialised_check (serialised)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if serialised.data.is_null() {
        return;
    }
    g_variant_type_info_query(
        serialised.type_info,
        &raw mut alignment,
        &raw mut fixed_size,
    );
    if alignment == 0 {
        return;
    }
    if alignment.wrapping_add(1 as guint) as gsize == fixed_size {
        match fixed_size {
            2 => {
                let mut ptr: *mut guint16 = serialised.data as *mut guint16;
                let mut __n1: gint64 = serialised.size as gint64;
                let mut __n2: gint64 = 2 as gint64;
                if !(__n1 == __n2) {
                    g_assertion_message_cmpint(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/gvariant-serialiser.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1710 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"serialised.size == 2\0" as *const u8 as *const ::core::ffi::c_char,
                        __n1 as guint64,
                        b"==\0" as *const u8 as *const ::core::ffi::c_char,
                        __n2 as guint64,
                        'i' as i32 as ::core::ffi::c_char,
                    );
                }
                *ptr = ((*ptr as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16
                    as ::core::ffi::c_int
                    | ((*ptr as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                        as ::core::ffi::c_int) as guint16;
                return;
            }
            4 => {
                let mut ptr_0: *mut guint32 = serialised.data as *mut guint32;
                let mut __n1_0: gint64 = serialised.size as gint64;
                let mut __n2_0: gint64 = 4 as gint64;
                if !(__n1_0 == __n2_0) {
                    g_assertion_message_cmpint(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/gvariant-serialiser.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1719 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"serialised.size == 4\0" as *const u8 as *const ::core::ffi::c_char,
                        __n1_0 as guint64,
                        b"==\0" as *const u8 as *const ::core::ffi::c_char,
                        __n2_0 as guint64,
                        'i' as i32 as ::core::ffi::c_char,
                    );
                }
                *ptr_0 = ({
                    let mut __v: guint32 = 0;
                    let mut __x: guint32 = *ptr_0;
                    if 0 != 0 {
                        __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                            | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                            | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                            | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
                    } else {
                        let fresh5 = &mut __v;
                        let fresh6;
                        let fresh7 = __x;
                        asm!(
                            "bswapl {0:e}\n", inlateout(reg)
                            c2rust_asm_casts::AsmCast::cast_in(fresh5, fresh7) => fresh6,
                            options(preserves_flags, pure, readonly, att_syntax)
                        );
                        c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
                    }
                    __v
                });
                return;
            }
            8 => {
                let mut ptr_1: *mut guint64 = serialised.data as *mut guint64;
                let mut __n1_1: gint64 = serialised.size as gint64;
                let mut __n2_1: gint64 = 8 as gint64;
                if !(__n1_1 == __n2_1) {
                    g_assertion_message_cmpint(
                        G_LOG_DOMAIN.as_ptr(),
                        b"../original/glib/gvariant-serialiser.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1728 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"serialised.size == 8\0" as *const u8 as *const ::core::ffi::c_char,
                        __n1_1 as guint64,
                        b"==\0" as *const u8 as *const ::core::ffi::c_char,
                        __n2_1 as guint64,
                        'i' as i32 as ::core::ffi::c_char,
                    );
                }
                *ptr_1 = ({
                    let mut __v: guint64 = 0;
                    let mut __x: guint64 = *ptr_1;
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
                        let fresh8 = &mut __v;
                        let fresh9;
                        let fresh10 = __x;
                        asm!(
                            "bswapq {0}\n", inlateout(reg)
                            c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh10) =>
                            fresh9, options(preserves_flags, pure, readonly, att_syntax)
                        );
                        c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
                    }
                    __v
                });
                return;
            }
            _ => {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gvariant-serialiser.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1734 as ::core::ffi::c_int,
                    G_STRFUNC,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
    } else {
        let mut children: gsize = 0;
        let mut i: gsize = 0;
        children = safe_c2rust_g_variant_serialised_n_children(serialised);
        i = 0 as gsize;
        while i < children {
            let mut child: GVariantSerialised = GVariantSerialised {
                type_info: ::core::ptr::null_mut::<GVariantTypeInfo>(),
                data: ::core::ptr::null_mut::<guchar>(),
                size: 0,
                depth: 0,
                ordered_offsets_up_to: 0,
                checked_offsets_up_to: 0,
            };
            child = safe_c2rust_g_variant_serialised_get_child(serialised, i);
            safe_c2rust_g_variant_serialised_byteswap(child);
            g_variant_type_info_unref(child.type_info);
            i = i.wrapping_add(1);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_serialised_is_normal(
    mut serialised: GVariantSerialised,
) -> gboolean {
    if serialised.depth >= G_VARIANT_MAX_RECURSION_DEPTH {
        return FALSE;
    }
    match *g_variant_type_info_get_type_string(serialised.type_info)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
    {
        G_VARIANT_TYPE_INFO_CHAR_MAYBE => {
            let mut fixed_size: gsize = 0;
            g_variant_type_info_query_element(
                serialised.type_info,
                ::core::ptr::null_mut::<guint>(),
                &raw mut fixed_size,
            );
            if fixed_size != 0 {
                return safe_c2rust_gvs_fixed_sized_maybe_is_normal(serialised);
            } else {
                return safe_c2rust_gvs_variable_sized_maybe_is_normal(serialised);
            }
        }
        G_VARIANT_TYPE_INFO_CHAR_ARRAY => {
            let mut fixed_size_0: gsize = 0;
            g_variant_type_info_query_element(
                serialised.type_info,
                ::core::ptr::null_mut::<guint>(),
                &raw mut fixed_size_0,
            );
            if fixed_size_0 != 0 {
                return safe_c2rust_gvs_fixed_sized_array_is_normal(serialised);
            } else {
                return safe_c2rust_gvs_variable_sized_array_is_normal(serialised);
            }
        }
        G_VARIANT_TYPE_INFO_CHAR_DICT_ENTRY | G_VARIANT_TYPE_INFO_CHAR_TUPLE => {
            return safe_c2rust_gvs_tuple_is_normal(serialised);
        }
        G_VARIANT_TYPE_INFO_CHAR_VARIANT => {
            return safe_c2rust_gvs_variant_is_normal(serialised);
        }
        _ => {}
    }
    if serialised.data.is_null() {
        return FALSE;
    }
    match *g_variant_type_info_get_type_string(serialised.type_info)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
    {
        98 => {
            return ((*serialised.data.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int)
                < 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        115 => {
            return safe_c2rust_g_variant_serialiser_is_string(
                serialised.data as gconstpointer,
                serialised.size,
            );
        }
        111 => {
            return safe_c2rust_g_variant_serialiser_is_object_path(
                serialised.data as gconstpointer,
                serialised.size,
            );
        }
        103 => {
            return safe_c2rust_g_variant_serialiser_is_signature(
                serialised.data as gconstpointer,
                serialised.size,
            );
        }
        _ => return TRUE,
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_serialiser_is_string(
    mut data: gconstpointer,
    mut size: gsize,
) -> gboolean {
    let mut expected_end: *const gchar = ::core::ptr::null::<gchar>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    if size == 0 as gsize {
        return FALSE;
    }
    expected_end = (data as *mut gchar)
        .offset(size as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    if *expected_end as ::core::ffi::c_int != '\0' as i32 {
        return FALSE;
    }
    g_utf8_validate_len(data as *const gchar, size, &raw mut end);
    return (end == expected_end) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_serialiser_is_object_path(
    mut data: gconstpointer,
    mut size: gsize,
) -> gboolean {
    let mut string: *const gchar = data as *const gchar;
    let mut i: gsize = 0;
    if safe_c2rust_g_variant_serialiser_is_string(data, size) == 0 {
        return FALSE;
    }
    if *string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '/' as i32 {
        return FALSE;
    }
    i = 1 as gsize;
    while *string.offset(i as isize) != 0 {
        if !(*safe_c2rust_g_ascii_table.offset(*string.offset(i as isize) as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_ALNUM as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
            || *string.offset(i as isize) as ::core::ffi::c_int == '_' as i32)
        {
            if *string.offset(i as isize) as ::core::ffi::c_int == '/' as i32 {
                if *string.offset(i.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
                    == '/' as i32
                {
                    return FALSE;
                }
            } else {
                return FALSE;
            }
        }
        i = i.wrapping_add(1);
    }
    if i > 1 as gsize
        && *string.offset(i.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int == '/' as i32
    {
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_variant_serialiser_is_signature(
    mut data: gconstpointer,
    mut size: gsize,
) -> gboolean {
    let mut string: *const gchar = data as *const gchar;
    let mut first_invalid: gsize = 0;
    if safe_c2rust_g_variant_serialiser_is_string(data, size) == 0 {
        return FALSE;
    }
    first_invalid = strspn(
        string as *const ::core::ffi::c_char,
        b"ybnqiuxthdvasog(){}\0" as *const u8 as *const ::core::ffi::c_char,
    ) as gsize;
    if *string.offset(first_invalid as isize) != 0 {
        return FALSE;
    }
    while *string != 0 {
        if g_variant_type_string_scan(string, ::core::ptr::null::<gchar>(), &raw mut string) == 0 {
            return FALSE;
        }
    }
    return TRUE;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_variant_serialised_n_children\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
