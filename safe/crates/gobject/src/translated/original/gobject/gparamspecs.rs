// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type _GVariantType;
    pub type _GVariant;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_unichar_validate(ch: gunichar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_type_free(type_0: *mut GVariantType);
    fn g_variant_type_copy(type_0: *const GVariantType) -> *mut GVariantType;
    fn g_variant_type_equal(type1: gconstpointer, type2: gconstpointer) -> gboolean;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type(value: *mut GVariant) -> *const GVariantType;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_classify(value: *mut GVariant) -> GVariantClass;
    fn g_variant_equal(one: gconstpointer, two: gconstpointer) -> gboolean;
    fn g_variant_compare(one: gconstpointer, two: gconstpointer) -> gint;
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
    fn g_type_parent(type_0: GType) -> GType;
    fn g_type_is_a(type_0: GType, is_a_type: GType) -> gboolean;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_peek(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_check_instance_is_fundamentally_a(
        instance: *mut GTypeInstance,
        fundamental_type: GType,
    ) -> gboolean;
    fn g_type_check_is_value_type(type_0: GType) -> gboolean;
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_copy(src_value: *const GValue, dest_value: *mut GValue);
    fn g_value_unset(value: *mut GValue);
    fn g_value_type_compatible(src_type: GType, dest_type: GType) -> gboolean;
    fn g_enum_get_value(enum_class: *mut GEnumClass, value: gint) -> *mut GEnumValue;
    fn g_param_spec_ref(pspec: *mut GParamSpec) -> *mut GParamSpec;
    fn g_param_spec_unref(pspec: *mut GParamSpec);
    fn g_param_spec_sink(pspec: *mut GParamSpec);
    fn g_param_spec_get_redirect_target(pspec: *mut GParamSpec) -> *mut GParamSpec;
    fn g_param_value_set_default(pspec: *mut GParamSpec, value: *mut GValue);
    fn g_param_value_validate(pspec: *mut GParamSpec, value: *mut GValue) -> gboolean;
    fn g_param_value_is_valid(pspec: *mut GParamSpec, value: *const GValue) -> gboolean;
    fn g_param_values_cmp(
        pspec: *mut GParamSpec,
        value1: *const GValue,
        value2: *const GValue,
    ) -> gint;
    fn g_param_type_register_static(
        name: *const gchar,
        pspec_info: *const GParamSpecTypeInfo,
    ) -> GType;
    fn g_param_spec_internal(
        param_type: GType,
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        flags: GParamFlags,
    ) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_array_get_type() -> GType;
    fn g_value_array_new(n_prealloced: guint) -> *mut GValueArray;
    fn g_value_array_append(
        value_array: *mut GValueArray,
        value: *const GValue,
    ) -> *mut GValueArray;
    fn g_value_array_remove(value_array: *mut GValueArray, index_: guint) -> *mut GValueArray;
    fn g_gtype_get_type() -> GType;
}
pub type size_t = usize;
pub type gint8 = ::core::ffi::c_schar;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
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
pub type GData = _GData;
pub type gunichar = guint32;
pub type GVariantType = _GVariantType;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumClass {
    pub g_type_class: GTypeClass,
    pub minimum: gint,
    pub maximum: gint,
    pub n_values: guint,
    pub values: *mut GEnumValue,
}
pub type GEnumValue = _GEnumValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GEnumValue {
    pub value: gint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GEnumClass = _GEnumClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFlagsClass {
    pub g_type_class: GTypeClass,
    pub mask: guint,
    pub n_values: guint,
    pub values: *mut GFlagsValue,
}
pub type GFlagsValue = _GFlagsValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFlagsValue {
    pub value: guint,
    pub value_name: *const gchar,
    pub value_nick: *const gchar,
}
pub type GFlagsClass = _GFlagsClass;
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
pub struct _GParamSpecClass {
    pub g_type_class: GTypeClass,
    pub value_type: GType,
    pub finalize: Option<unsafe extern "C" fn(*mut GParamSpec) -> ()>,
    pub value_set_default: Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> ()>,
    pub value_validate: Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean>,
    pub values_cmp:
        Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint>,
    pub value_is_valid: Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>,
    pub dummy: [gpointer; 3],
}
pub type GParamSpecClass = _GParamSpecClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecTypeInfo {
    pub instance_size: guint16,
    pub n_preallocs: guint16,
    pub instance_init: Option<unsafe extern "C" fn(*mut GParamSpec) -> ()>,
    pub value_type: GType,
    pub finalize: Option<unsafe extern "C" fn(*mut GParamSpec) -> ()>,
    pub value_set_default: Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> ()>,
    pub value_validate: Option<unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean>,
    pub values_cmp:
        Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint>,
}
pub type GParamSpecTypeInfo = _GParamSpecTypeInfo;
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
pub struct _GParamSpecChar {
    pub parent_instance: GParamSpec,
    pub minimum: gint8,
    pub maximum: gint8,
    pub default_value: gint8,
}
pub type GParamSpecChar = _GParamSpecChar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecUChar {
    pub parent_instance: GParamSpec,
    pub minimum: guint8,
    pub maximum: guint8,
    pub default_value: guint8,
}
pub type GParamSpecUChar = _GParamSpecUChar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecBoolean {
    pub parent_instance: GParamSpec,
    pub default_value: gboolean,
}
pub type GParamSpecBoolean = _GParamSpecBoolean;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecInt {
    pub parent_instance: GParamSpec,
    pub minimum: gint,
    pub maximum: gint,
    pub default_value: gint,
}
pub type GParamSpecInt = _GParamSpecInt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecUInt {
    pub parent_instance: GParamSpec,
    pub minimum: guint,
    pub maximum: guint,
    pub default_value: guint,
}
pub type GParamSpecUInt = _GParamSpecUInt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecLong {
    pub parent_instance: GParamSpec,
    pub minimum: glong,
    pub maximum: glong,
    pub default_value: glong,
}
pub type GParamSpecLong = _GParamSpecLong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecULong {
    pub parent_instance: GParamSpec,
    pub minimum: gulong,
    pub maximum: gulong,
    pub default_value: gulong,
}
pub type GParamSpecULong = _GParamSpecULong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecInt64 {
    pub parent_instance: GParamSpec,
    pub minimum: gint64,
    pub maximum: gint64,
    pub default_value: gint64,
}
pub type GParamSpecInt64 = _GParamSpecInt64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecUInt64 {
    pub parent_instance: GParamSpec,
    pub minimum: guint64,
    pub maximum: guint64,
    pub default_value: guint64,
}
pub type GParamSpecUInt64 = _GParamSpecUInt64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecUnichar {
    pub parent_instance: GParamSpec,
    pub default_value: gunichar,
}
pub type GParamSpecUnichar = _GParamSpecUnichar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecEnum {
    pub parent_instance: GParamSpec,
    pub enum_class: *mut GEnumClass,
    pub default_value: gint,
}
pub type GParamSpecEnum = _GParamSpecEnum;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecFlags {
    pub parent_instance: GParamSpec,
    pub flags_class: *mut GFlagsClass,
    pub default_value: guint,
}
pub type GParamSpecFlags = _GParamSpecFlags;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecFloat {
    pub parent_instance: GParamSpec,
    pub minimum: gfloat,
    pub maximum: gfloat,
    pub default_value: gfloat,
    pub epsilon: gfloat,
}
pub type GParamSpecFloat = _GParamSpecFloat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecDouble {
    pub parent_instance: GParamSpec,
    pub minimum: gdouble,
    pub maximum: gdouble,
    pub default_value: gdouble,
    pub epsilon: gdouble,
}
pub type GParamSpecDouble = _GParamSpecDouble;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GParamSpecString {
    pub parent_instance: GParamSpec,
    pub default_value: *mut gchar,
    pub cset_first: *mut gchar,
    pub cset_nth: *mut gchar,
    pub substitutor: gchar,
    #[bitfield(name = "null_fold_if_empty", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "ensure_non_null", ty = "guint", bits = "1..=1")]
    pub null_fold_if_empty_ensure_non_null: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
pub type GParamSpecString = _GParamSpecString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecParam {
    pub parent_instance: GParamSpec,
}
pub type GParamSpecParam = _GParamSpecParam;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecBoxed {
    pub parent_instance: GParamSpec,
}
pub type GParamSpecBoxed = _GParamSpecBoxed;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecPointer {
    pub parent_instance: GParamSpec,
}
pub type GParamSpecPointer = _GParamSpecPointer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecValueArray {
    pub parent_instance: GParamSpec,
    pub element_spec: *mut GParamSpec,
    pub fixed_n_elements: guint,
}
pub type GParamSpecValueArray = _GParamSpecValueArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecObject {
    pub parent_instance: GParamSpec,
}
pub type GParamSpecObject = _GParamSpecObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecOverride {
    pub parent_instance: GParamSpec,
    pub overridden: *mut GParamSpec,
}
pub type GParamSpecOverride = _GParamSpecOverride;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecGType {
    pub parent_instance: GParamSpec,
    pub is_a_type: GType,
}
pub type GParamSpecGType = _GParamSpecGType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpecVariant {
    pub parent_instance: GParamSpec,
    pub type_0: *mut GVariantType,
    pub default_value: *mut GVariant,
    pub padding: [gpointer; 4],
}
pub type GParamSpecVariant = _GParamSpecVariant;
pub type GValueArray = _GValueArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValueArray {
    pub n_values: guint,
    pub values: *mut GValue,
    pub n_prealloced: guint,
}
#[inline(always)]
unsafe extern "C" fn g_strdup_inline(
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
unsafe extern "C" fn param_char_init(mut pspec: *mut GParamSpec) {
    let mut cspec: *mut GParamSpecChar = pspec as *mut ::core::ffi::c_void as *mut GParamSpecChar;
    (*cspec).minimum = 0x7f as gint8;
    (*cspec).maximum = 0x80 as ::core::ffi::c_int as gint8;
    (*cspec).default_value = 0 as gint8;
}
unsafe extern "C" fn param_char_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_int =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecChar)).default_value as gint;
}
unsafe extern "C" fn param_char_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut cspec: *mut GParamSpecChar = pspec as *mut ::core::ffi::c_void as *mut GParamSpecChar;
    let mut oval: gint = (*value).data[0 as ::core::ffi::c_int as usize].v_int;
    return ((*cspec).minimum as ::core::ffi::c_int <= oval
        && oval <= (*cspec).maximum as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_char_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut cspec: *mut GParamSpecChar = pspec as *mut ::core::ffi::c_void as *mut GParamSpecChar;
    let mut oval: gint = (*value).data[0 as ::core::ffi::c_int as usize].v_int;
    (*value).data[0 as ::core::ffi::c_int as usize].v_int =
        (if (*value).data[0 as ::core::ffi::c_int as usize].v_int
            > (*cspec).maximum as ::core::ffi::c_int
        {
            (*cspec).maximum as ::core::ffi::c_int
        } else if (*value).data[0 as ::core::ffi::c_int as usize].v_int
            < (*cspec).minimum as ::core::ffi::c_int
        {
            (*cspec).minimum as ::core::ffi::c_int
        } else {
            (*value).data[0 as ::core::ffi::c_int as usize].v_int as ::core::ffi::c_int
        }) as gint;
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_int != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_uchar_init(mut pspec: *mut GParamSpec) {
    let mut uspec: *mut GParamSpecUChar = pspec as *mut ::core::ffi::c_void as *mut GParamSpecUChar;
    (*uspec).minimum = 0 as guint8;
    (*uspec).maximum = 0xff as guint8;
    (*uspec).default_value = 0 as guint8;
}
unsafe extern "C" fn param_uchar_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_uint =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecUChar)).default_value as guint;
}
unsafe extern "C" fn param_uchar_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut uspec: *mut GParamSpecUChar = pspec as *mut ::core::ffi::c_void as *mut GParamSpecUChar;
    let mut oval: guint = (*value).data[0 as ::core::ffi::c_int as usize].v_uint;
    return ((*uspec).minimum as guint <= oval && oval <= (*uspec).maximum as guint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn param_uchar_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut uspec: *mut GParamSpecUChar = pspec as *mut ::core::ffi::c_void as *mut GParamSpecUChar;
    let mut oval: guint = (*value).data[0 as ::core::ffi::c_int as usize].v_uint;
    (*value).data[0 as ::core::ffi::c_int as usize].v_uint =
        if (*value).data[0 as ::core::ffi::c_int as usize].v_uint > (*uspec).maximum as guint {
            (*uspec).maximum as guint
        } else if (*value).data[0 as ::core::ffi::c_int as usize].v_uint < (*uspec).minimum as guint
        {
            (*uspec).minimum as guint
        } else {
            (*value).data[0 as ::core::ffi::c_int as usize].v_uint
        };
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_uint != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_boolean_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_int =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecBoolean)).default_value as gint;
}
unsafe extern "C" fn param_boolean_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut oval: ::core::ffi::c_int =
        (*value).data[0 as ::core::ffi::c_int as usize].v_int as ::core::ffi::c_int;
    return (oval == 0 as ::core::ffi::c_int
        || oval == (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn param_boolean_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut oval: gint = (*value).data[0 as ::core::ffi::c_int as usize].v_int;
    (*value).data[0 as ::core::ffi::c_int as usize].v_int =
        ((*value).data[0 as ::core::ffi::c_int as usize].v_int != 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as gint;
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_int != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_int_init(mut pspec: *mut GParamSpec) {
    let mut ispec: *mut GParamSpecInt = pspec as *mut ::core::ffi::c_void as *mut GParamSpecInt;
    (*ispec).minimum = 0x7fffffff as ::core::ffi::c_int as gint;
    (*ispec).maximum = 0x80000000 as ::core::ffi::c_uint as gint;
    (*ispec).default_value = 0 as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn param_int_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_int =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecInt)).default_value;
}
unsafe extern "C" fn param_int_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut ispec: *mut GParamSpecInt = pspec as *mut ::core::ffi::c_void as *mut GParamSpecInt;
    let mut oval: ::core::ffi::c_int =
        (*value).data[0 as ::core::ffi::c_int as usize].v_int as ::core::ffi::c_int;
    return ((*ispec).minimum <= oval && oval <= (*ispec).maximum) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_int_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut ispec: *mut GParamSpecInt = pspec as *mut ::core::ffi::c_void as *mut GParamSpecInt;
    let mut oval: gint = (*value).data[0 as ::core::ffi::c_int as usize].v_int;
    (*value).data[0 as ::core::ffi::c_int as usize].v_int =
        if (*value).data[0 as ::core::ffi::c_int as usize].v_int > (*ispec).maximum {
            (*ispec).maximum
        } else if (*value).data[0 as ::core::ffi::c_int as usize].v_int < (*ispec).minimum {
            (*ispec).minimum
        } else {
            (*value).data[0 as ::core::ffi::c_int as usize].v_int
        };
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_int != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_int_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    if (*value1).data[0 as ::core::ffi::c_int as usize].v_int
        < (*value2).data[0 as ::core::ffi::c_int as usize].v_int
    {
        return -(1 as gint);
    } else {
        return ((*value1).data[0 as ::core::ffi::c_int as usize].v_int
            > (*value2).data[0 as ::core::ffi::c_int as usize].v_int)
            as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn param_uint_init(mut pspec: *mut GParamSpec) {
    let mut uspec: *mut GParamSpecUInt = pspec as *mut ::core::ffi::c_void as *mut GParamSpecUInt;
    (*uspec).minimum = 0 as guint;
    (*uspec).maximum = 0xffffffff as ::core::ffi::c_uint as guint;
    (*uspec).default_value = 0 as guint;
}
unsafe extern "C" fn param_uint_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_uint =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecUInt)).default_value;
}
unsafe extern "C" fn param_uint_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut uspec: *mut GParamSpecUInt = pspec as *mut ::core::ffi::c_void as *mut GParamSpecUInt;
    let mut oval: guint = (*value).data[0 as ::core::ffi::c_int as usize].v_uint;
    return ((*uspec).minimum <= oval && oval <= (*uspec).maximum) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_uint_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut uspec: *mut GParamSpecUInt = pspec as *mut ::core::ffi::c_void as *mut GParamSpecUInt;
    let mut oval: guint = (*value).data[0 as ::core::ffi::c_int as usize].v_uint;
    (*value).data[0 as ::core::ffi::c_int as usize].v_uint =
        if (*value).data[0 as ::core::ffi::c_int as usize].v_uint > (*uspec).maximum {
            (*uspec).maximum
        } else if (*value).data[0 as ::core::ffi::c_int as usize].v_uint < (*uspec).minimum {
            (*uspec).minimum
        } else {
            (*value).data[0 as ::core::ffi::c_int as usize].v_uint
        };
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_uint != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_uint_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    if (*value1).data[0 as ::core::ffi::c_int as usize].v_uint
        < (*value2).data[0 as ::core::ffi::c_int as usize].v_uint
    {
        return -(1 as gint);
    } else {
        return ((*value1).data[0 as ::core::ffi::c_int as usize].v_uint
            > (*value2).data[0 as ::core::ffi::c_int as usize].v_uint)
            as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn param_long_init(mut pspec: *mut GParamSpec) {
    let mut lspec: *mut GParamSpecLong = pspec as *mut ::core::ffi::c_void as *mut GParamSpecLong;
    (*lspec).minimum = 0x7fffffffffffffff as ::core::ffi::c_long as glong;
    (*lspec).maximum = 0x8000000000000000 as ::core::ffi::c_ulong as glong;
    (*lspec).default_value = 0 as glong;
}
unsafe extern "C" fn param_long_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_long =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecLong)).default_value;
}
unsafe extern "C" fn param_long_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut lspec: *mut GParamSpecLong = pspec as *mut ::core::ffi::c_void as *mut GParamSpecLong;
    let mut oval: glong = (*value).data[0 as ::core::ffi::c_int as usize].v_long;
    return ((*lspec).minimum <= oval && oval <= (*lspec).maximum) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_long_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut lspec: *mut GParamSpecLong = pspec as *mut ::core::ffi::c_void as *mut GParamSpecLong;
    let mut oval: glong = (*value).data[0 as ::core::ffi::c_int as usize].v_long;
    (*value).data[0 as ::core::ffi::c_int as usize].v_long =
        if (*value).data[0 as ::core::ffi::c_int as usize].v_long > (*lspec).maximum {
            (*lspec).maximum
        } else if (*value).data[0 as ::core::ffi::c_int as usize].v_long < (*lspec).minimum {
            (*lspec).minimum
        } else {
            (*value).data[0 as ::core::ffi::c_int as usize].v_long
        };
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_long != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_long_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    if (*value1).data[0 as ::core::ffi::c_int as usize].v_long
        < (*value2).data[0 as ::core::ffi::c_int as usize].v_long
    {
        return -(1 as gint);
    } else {
        return ((*value1).data[0 as ::core::ffi::c_int as usize].v_long
            > (*value2).data[0 as ::core::ffi::c_int as usize].v_long)
            as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn param_ulong_init(mut pspec: *mut GParamSpec) {
    let mut uspec: *mut GParamSpecULong = pspec as *mut ::core::ffi::c_void as *mut GParamSpecULong;
    (*uspec).minimum = 0 as gulong;
    (*uspec).maximum = 0xffffffffffffffff as ::core::ffi::c_ulong as gulong;
    (*uspec).default_value = 0 as gulong;
}
unsafe extern "C" fn param_ulong_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_ulong =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecULong)).default_value;
}
unsafe extern "C" fn param_ulong_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut uspec: *mut GParamSpecULong = pspec as *mut ::core::ffi::c_void as *mut GParamSpecULong;
    let mut oval: gulong = (*value).data[0 as ::core::ffi::c_int as usize].v_ulong;
    return ((*uspec).minimum <= oval && oval <= (*uspec).maximum) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_ulong_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut uspec: *mut GParamSpecULong = pspec as *mut ::core::ffi::c_void as *mut GParamSpecULong;
    let mut oval: gulong = (*value).data[0 as ::core::ffi::c_int as usize].v_ulong;
    (*value).data[0 as ::core::ffi::c_int as usize].v_ulong =
        if (*value).data[0 as ::core::ffi::c_int as usize].v_ulong > (*uspec).maximum {
            (*uspec).maximum
        } else if (*value).data[0 as ::core::ffi::c_int as usize].v_ulong < (*uspec).minimum {
            (*uspec).minimum
        } else {
            (*value).data[0 as ::core::ffi::c_int as usize].v_ulong
        };
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_ulong != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_ulong_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    if (*value1).data[0 as ::core::ffi::c_int as usize].v_ulong
        < (*value2).data[0 as ::core::ffi::c_int as usize].v_ulong
    {
        return -(1 as gint);
    } else {
        return ((*value1).data[0 as ::core::ffi::c_int as usize].v_ulong
            > (*value2).data[0 as ::core::ffi::c_int as usize].v_ulong)
            as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn param_int64_init(mut pspec: *mut GParamSpec) {
    let mut lspec: *mut GParamSpecInt64 = pspec as *mut ::core::ffi::c_void as *mut GParamSpecInt64;
    (*lspec).minimum = -(0x7fffffffffffffff as ::core::ffi::c_long) - 1 as ::core::ffi::c_long;
    (*lspec).maximum = 0x7fffffffffffffff as ::core::ffi::c_long as gint64;
    (*lspec).default_value = 0 as gint64;
}
unsafe extern "C" fn param_int64_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_int64 =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecInt64)).default_value;
}
unsafe extern "C" fn param_int64_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut lspec: *mut GParamSpecInt64 = pspec as *mut ::core::ffi::c_void as *mut GParamSpecInt64;
    let mut oval: gint64 = (*value).data[0 as ::core::ffi::c_int as usize].v_int64;
    return ((*lspec).minimum <= oval && oval <= (*lspec).maximum) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_int64_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut lspec: *mut GParamSpecInt64 = pspec as *mut ::core::ffi::c_void as *mut GParamSpecInt64;
    let mut oval: gint64 = (*value).data[0 as ::core::ffi::c_int as usize].v_int64;
    (*value).data[0 as ::core::ffi::c_int as usize].v_int64 =
        if (*value).data[0 as ::core::ffi::c_int as usize].v_int64 > (*lspec).maximum {
            (*lspec).maximum
        } else if (*value).data[0 as ::core::ffi::c_int as usize].v_int64 < (*lspec).minimum {
            (*lspec).minimum
        } else {
            (*value).data[0 as ::core::ffi::c_int as usize].v_int64
        };
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_int64 != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_int64_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    if (*value1).data[0 as ::core::ffi::c_int as usize].v_int64
        < (*value2).data[0 as ::core::ffi::c_int as usize].v_int64
    {
        return -(1 as gint);
    } else {
        return ((*value1).data[0 as ::core::ffi::c_int as usize].v_int64
            > (*value2).data[0 as ::core::ffi::c_int as usize].v_int64)
            as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn param_uint64_init(mut pspec: *mut GParamSpec) {
    let mut uspec: *mut GParamSpecUInt64 =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecUInt64;
    (*uspec).minimum = 0 as guint64;
    (*uspec).maximum = 0xffffffffffffffff as ::core::ffi::c_ulong as guint64;
    (*uspec).default_value = 0 as guint64;
}
unsafe extern "C" fn param_uint64_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_uint64 =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecUInt64)).default_value;
}
unsafe extern "C" fn param_uint64_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut uspec: *mut GParamSpecUInt64 =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecUInt64;
    let mut oval: guint64 = (*value).data[0 as ::core::ffi::c_int as usize].v_uint64;
    return ((*uspec).minimum <= oval && oval <= (*uspec).maximum) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_uint64_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut uspec: *mut GParamSpecUInt64 =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecUInt64;
    let mut oval: guint64 = (*value).data[0 as ::core::ffi::c_int as usize].v_uint64;
    (*value).data[0 as ::core::ffi::c_int as usize].v_uint64 =
        if (*value).data[0 as ::core::ffi::c_int as usize].v_uint64 > (*uspec).maximum {
            (*uspec).maximum
        } else if (*value).data[0 as ::core::ffi::c_int as usize].v_uint64 < (*uspec).minimum {
            (*uspec).minimum
        } else {
            (*value).data[0 as ::core::ffi::c_int as usize].v_uint64
        };
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_uint64 != oval)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn param_uint64_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    if (*value1).data[0 as ::core::ffi::c_int as usize].v_uint64
        < (*value2).data[0 as ::core::ffi::c_int as usize].v_uint64
    {
        return -(1 as gint);
    } else {
        return ((*value1).data[0 as ::core::ffi::c_int as usize].v_uint64
            > (*value2).data[0 as ::core::ffi::c_int as usize].v_uint64)
            as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn param_unichar_init(mut pspec: *mut GParamSpec) {
    let mut uspec: *mut GParamSpecUnichar =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecUnichar;
    (*uspec).default_value = 0 as gunichar;
}
unsafe extern "C" fn param_unichar_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_uint =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecUnichar)).default_value as guint;
}
unsafe extern "C" fn param_unichar_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    return g_unichar_validate((*value).data[0 as ::core::ffi::c_int as usize].v_uint as gunichar);
}
unsafe extern "C" fn param_unichar_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut oval: gunichar = (*value).data[0 as ::core::ffi::c_int as usize].v_uint as gunichar;
    let mut changed: gboolean = 0 as gboolean;
    if g_unichar_validate(oval) == 0 {
        (*value).data[0 as ::core::ffi::c_int as usize].v_uint = 0 as guint;
        changed = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
    }
    return changed;
}
unsafe extern "C" fn param_unichar_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    if (*value1).data[0 as ::core::ffi::c_int as usize].v_uint
        < (*value2).data[0 as ::core::ffi::c_int as usize].v_uint
    {
        return -(1 as gint);
    } else {
        return ((*value1).data[0 as ::core::ffi::c_int as usize].v_uint
            > (*value2).data[0 as ::core::ffi::c_int as usize].v_uint)
            as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn param_enum_init(mut pspec: *mut GParamSpec) {
    let mut espec: *mut GParamSpecEnum = pspec as *mut ::core::ffi::c_void as *mut GParamSpecEnum;
    (*espec).enum_class = ::core::ptr::null_mut::<GEnumClass>();
    (*espec).default_value = 0 as ::core::ffi::c_int as gint;
}
unsafe extern "C" fn param_enum_finalize(mut pspec: *mut GParamSpec) {
    let mut espec: *mut GParamSpecEnum = pspec as *mut ::core::ffi::c_void as *mut GParamSpecEnum;
    let mut parent_class: *mut GParamSpecClass = g_type_class_peek(g_type_parent(
        *g_param_spec_types.offset(10 as ::core::ffi::c_int as isize),
    )) as *mut GParamSpecClass;
    if !(*espec).enum_class.is_null() {
        g_type_class_unref((*espec).enum_class as gpointer);
        (*espec).enum_class = ::core::ptr::null_mut::<GEnumClass>();
    }
    (*parent_class).finalize.expect("non-null function pointer")(pspec);
}
unsafe extern "C" fn param_enum_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_long =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecEnum)).default_value as glong;
}
unsafe extern "C" fn param_enum_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut espec: *mut GParamSpecEnum = pspec as *mut ::core::ffi::c_void as *mut GParamSpecEnum;
    let mut oval: glong = (*value).data[0 as ::core::ffi::c_int as usize].v_long;
    return (g_enum_get_value((*espec).enum_class, oval as gint)
        != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut GEnumValue)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn param_enum_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut espec: *mut GParamSpecEnum = pspec as *mut ::core::ffi::c_void as *mut GParamSpecEnum;
    let mut oval: glong = (*value).data[0 as ::core::ffi::c_int as usize].v_long;
    if (*espec).enum_class.is_null()
        || g_enum_get_value(
            (*espec).enum_class,
            (*value).data[0 as ::core::ffi::c_int as usize].v_long as gint,
        )
        .is_null()
    {
        (*value).data[0 as ::core::ffi::c_int as usize].v_long = (*espec).default_value as glong;
    }
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_long != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_flags_init(mut pspec: *mut GParamSpec) {
    let mut fspec: *mut GParamSpecFlags = pspec as *mut ::core::ffi::c_void as *mut GParamSpecFlags;
    (*fspec).flags_class = ::core::ptr::null_mut::<GFlagsClass>();
    (*fspec).default_value = 0 as guint;
}
unsafe extern "C" fn param_flags_finalize(mut pspec: *mut GParamSpec) {
    let mut fspec: *mut GParamSpecFlags = pspec as *mut ::core::ffi::c_void as *mut GParamSpecFlags;
    let mut parent_class: *mut GParamSpecClass = g_type_class_peek(g_type_parent(
        *g_param_spec_types.offset(11 as ::core::ffi::c_int as isize),
    )) as *mut GParamSpecClass;
    if !(*fspec).flags_class.is_null() {
        g_type_class_unref((*fspec).flags_class as gpointer);
        (*fspec).flags_class = ::core::ptr::null_mut::<GFlagsClass>();
    }
    (*parent_class).finalize.expect("non-null function pointer")(pspec);
}
unsafe extern "C" fn param_flags_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_ulong =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecFlags)).default_value as gulong;
}
unsafe extern "C" fn param_flags_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut fspec: *mut GParamSpecFlags = pspec as *mut ::core::ffi::c_void as *mut GParamSpecFlags;
    let mut oval: gulong = (*value).data[0 as ::core::ffi::c_int as usize].v_ulong;
    return (oval & !(*(*fspec).flags_class).mask as gulong == 0 as gulong) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_flags_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut fspec: *mut GParamSpecFlags = pspec as *mut ::core::ffi::c_void as *mut GParamSpecFlags;
    let mut oval: gulong = (*value).data[0 as ::core::ffi::c_int as usize].v_ulong;
    if !(*fspec).flags_class.is_null() {
        (*value).data[0 as ::core::ffi::c_int as usize].v_ulong &=
            (*(*fspec).flags_class).mask as gulong;
    } else {
        (*value).data[0 as ::core::ffi::c_int as usize].v_ulong = (*fspec).default_value as gulong;
    }
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_ulong != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_float_init(mut pspec: *mut GParamSpec) {
    let mut fspec: *mut GParamSpecFloat = pspec as *mut ::core::ffi::c_void as *mut GParamSpecFloat;
    (*fspec).minimum = -3.40282347e+38f32 as gfloat;
    (*fspec).maximum = 3.40282347e+38f32 as gfloat;
    (*fspec).default_value = 0 as ::core::ffi::c_int as gfloat;
    (*fspec).epsilon = 1e-30f32;
}
unsafe extern "C" fn param_float_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_float =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecFloat)).default_value;
}
unsafe extern "C" fn param_float_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut fspec: *mut GParamSpecFloat = pspec as *mut ::core::ffi::c_void as *mut GParamSpecFloat;
    let mut oval: gfloat = (*value).data[0 as ::core::ffi::c_int as usize].v_float;
    return ((*fspec).minimum <= oval && oval <= (*fspec).maximum) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_float_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut fspec: *mut GParamSpecFloat = pspec as *mut ::core::ffi::c_void as *mut GParamSpecFloat;
    let mut oval: gfloat = (*value).data[0 as ::core::ffi::c_int as usize].v_float;
    (*value).data[0 as ::core::ffi::c_int as usize].v_float =
        if (*value).data[0 as ::core::ffi::c_int as usize].v_float > (*fspec).maximum {
            (*fspec).maximum
        } else if (*value).data[0 as ::core::ffi::c_int as usize].v_float < (*fspec).minimum {
            (*fspec).minimum
        } else {
            (*value).data[0 as ::core::ffi::c_int as usize].v_float
        };
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_float != oval) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_float_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    let mut epsilon: gfloat =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecFloat)).epsilon;
    if (*value1).data[0 as ::core::ffi::c_int as usize].v_float
        < (*value2).data[0 as ::core::ffi::c_int as usize].v_float
    {
        return -(((*value2).data[0 as ::core::ffi::c_int as usize].v_float
            - (*value1).data[0 as ::core::ffi::c_int as usize].v_float
            > epsilon) as ::core::ffi::c_int);
    } else {
        return ((*value1).data[0 as ::core::ffi::c_int as usize].v_float
            - (*value2).data[0 as ::core::ffi::c_int as usize].v_float
            > epsilon) as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn param_double_init(mut pspec: *mut GParamSpec) {
    let mut dspec: *mut GParamSpecDouble =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecDouble;
    (*dspec).minimum = -1.7976931348623157e+308f64 as gdouble;
    (*dspec).maximum = 1.7976931348623157e+308f64 as gdouble;
    (*dspec).default_value = 0 as ::core::ffi::c_int as gdouble;
    (*dspec).epsilon = 1e-90f64 as gdouble;
}
unsafe extern "C" fn param_double_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_double =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecDouble)).default_value;
}
unsafe extern "C" fn param_double_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut dspec: *mut GParamSpecDouble =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecDouble;
    let mut oval: gfloat = (*value).data[0 as ::core::ffi::c_int as usize].v_double as gfloat;
    return ((*dspec).minimum <= oval as gdouble && oval as gdouble <= (*dspec).maximum)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn param_double_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut dspec: *mut GParamSpecDouble =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecDouble;
    let mut oval: gdouble = (*value).data[0 as ::core::ffi::c_int as usize].v_double;
    (*value).data[0 as ::core::ffi::c_int as usize].v_double =
        if (*value).data[0 as ::core::ffi::c_int as usize].v_double > (*dspec).maximum {
            (*dspec).maximum
        } else if (*value).data[0 as ::core::ffi::c_int as usize].v_double < (*dspec).minimum {
            (*dspec).minimum
        } else {
            (*value).data[0 as ::core::ffi::c_int as usize].v_double
        };
    return ((*value).data[0 as ::core::ffi::c_int as usize].v_double != oval)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn param_double_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    let mut epsilon: gdouble =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecDouble)).epsilon;
    if (*value1).data[0 as ::core::ffi::c_int as usize].v_double
        < (*value2).data[0 as ::core::ffi::c_int as usize].v_double
    {
        return -(((*value2).data[0 as ::core::ffi::c_int as usize].v_double
            - (*value1).data[0 as ::core::ffi::c_int as usize].v_double
            > epsilon) as ::core::ffi::c_int);
    } else {
        return ((*value1).data[0 as ::core::ffi::c_int as usize].v_double
            - (*value2).data[0 as ::core::ffi::c_int as usize].v_double
            > epsilon) as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn param_string_init(mut pspec: *mut GParamSpec) {
    let mut sspec: *mut GParamSpecString =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecString;
    (*sspec).default_value = ::core::ptr::null_mut::<gchar>();
    (*sspec).cset_first = ::core::ptr::null_mut::<gchar>();
    (*sspec).cset_nth = ::core::ptr::null_mut::<gchar>();
    (*sspec).substitutor = '_' as i32 as gchar;
    (*sspec).set_null_fold_if_empty(0 as guint as guint);
    (*sspec).set_ensure_non_null(0 as guint as guint);
}
unsafe extern "C" fn param_string_finalize(mut pspec: *mut GParamSpec) {
    let mut sspec: *mut GParamSpecString =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecString;
    let mut parent_class: *mut GParamSpecClass = g_type_class_peek(g_type_parent(
        *g_param_spec_types.offset(14 as ::core::ffi::c_int as isize),
    )) as *mut GParamSpecClass;
    g_free((*sspec).default_value as gpointer);
    g_free((*sspec).cset_first as gpointer);
    g_free((*sspec).cset_nth as gpointer);
    (*sspec).default_value = ::core::ptr::null_mut::<gchar>();
    (*sspec).cset_first = ::core::ptr::null_mut::<gchar>();
    (*sspec).cset_nth = ::core::ptr::null_mut::<gchar>();
    (*parent_class).finalize.expect("non-null function pointer")(pspec);
}
unsafe extern "C" fn param_string_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = g_strdup_inline(
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecString)).default_value,
    ) as gpointer;
}
unsafe extern "C" fn param_string_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut sspec: *mut GParamSpecString =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecString;
    let mut string: *mut gchar =
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut gchar;
    let mut changed: guint = 0 as guint;
    if !string.is_null()
        && *string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
        if !(*sspec).cset_first.is_null()
            && strchr(
                (*sspec).cset_first,
                *string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            )
            .is_null()
        {
            if (*value).data[1 as ::core::ffi::c_int as usize].v_uint
                & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
                != 0
            {
                (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
                    g_strdup_inline(string) as gpointer;
                string = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut gchar;
                (*value).data[1 as ::core::ffi::c_int as usize].v_uint &=
                    !((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
            }
            *string.offset(0 as ::core::ffi::c_int as isize) = (*sspec).substitutor;
            changed = changed.wrapping_add(1);
        }
        if !(*sspec).cset_nth.is_null() {
            s = string.offset(1 as ::core::ffi::c_int as isize);
            while *s != 0 {
                if strchr((*sspec).cset_nth, *s as ::core::ffi::c_int).is_null() {
                    if (*value).data[1 as ::core::ffi::c_int as usize].v_uint
                        & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
                        != 0
                    {
                        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
                            g_strdup_inline(string) as gpointer;
                        s = ((*value).data[0 as ::core::ffi::c_int as usize].v_pointer
                            as *mut gchar)
                            .offset(s.offset_from(string) as ::core::ffi::c_long as isize);
                        string =
                            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut gchar;
                        (*value).data[1 as ::core::ffi::c_int as usize].v_uint &=
                            !((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
                    }
                    *s = (*sspec).substitutor;
                    changed = changed.wrapping_add(1);
                }
                s = s.offset(1);
            }
        }
    }
    if (*sspec).null_fold_if_empty() as ::core::ffi::c_int != 0
        && !string.is_null()
        && *string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        if (*value).data[1 as ::core::ffi::c_int as usize].v_uint
            & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
            == 0
        {
            g_free((*value).data[0 as ::core::ffi::c_int as usize].v_pointer);
        } else {
            (*value).data[1 as ::core::ffi::c_int as usize].v_uint &=
                !((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
        }
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        changed = changed.wrapping_add(1);
        string = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut gchar;
    }
    if (*sspec).ensure_non_null() as ::core::ffi::c_int != 0 && string.is_null() {
        (*value).data[1 as ::core::ffi::c_int as usize].v_uint &=
            !((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char) as gpointer;
        changed = changed.wrapping_add(1);
        string = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut gchar;
    }
    return changed as gboolean;
}
unsafe extern "C" fn param_string_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut sspec: *mut GParamSpecString =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecString;
    let mut ret: gboolean = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    if !(*sspec).cset_first.is_null()
        || !(*sspec).cset_nth.is_null()
        || (*sspec).ensure_non_null() as ::core::ffi::c_int != 0
        || (*sspec).null_fold_if_empty() as ::core::ffi::c_int != 0
    {
        let mut tmp_value: GValue = _GValue {
            g_type: 0 as GType,
            data: [
                C2RustUnnamed {
                    v_int: 0 as ::core::ffi::c_int,
                },
                C2RustUnnamed { v_int: 0 },
            ],
        };
        g_value_init(&raw mut tmp_value, (*(value as *mut GValue)).g_type);
        g_value_copy(value, &raw mut tmp_value);
        ret = (param_string_validate(pspec, &raw mut tmp_value) == 0) as ::core::ffi::c_int
            as gboolean;
        g_value_unset(&raw mut tmp_value);
    }
    return ret;
}
unsafe extern "C" fn param_string_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    if (*value1).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        return if !(*value2).data[0 as ::core::ffi::c_int as usize]
            .v_pointer
            .is_null()
        {
            -(1 as gint)
        } else {
            0 as gint
        };
    } else if (*value2).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        return ((*value1).data[0 as ::core::ffi::c_int as usize].v_pointer
            != ::core::ptr::null_mut::<::core::ffi::c_void>()) as ::core::ffi::c_int;
    } else {
        return strcmp(
            (*value1).data[0 as ::core::ffi::c_int as usize].v_pointer
                as *const ::core::ffi::c_char,
            (*value2).data[0 as ::core::ffi::c_int as usize].v_pointer
                as *const ::core::ffi::c_char,
        ) as gint;
    };
}
unsafe extern "C" fn param_param_init(mut pspec: *mut GParamSpec) {}
unsafe extern "C" fn param_param_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
}
unsafe extern "C" fn param_param_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut param: *mut GParamSpec =
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec;
    if param.is_null() {
        return 0 as gboolean;
    }
    return g_value_type_compatible(
        (*(*(param as *mut GTypeInstance)).g_class).g_type,
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
    );
}
unsafe extern "C" fn param_param_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut param: *mut GParamSpec =
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GParamSpec;
    let mut changed: guint = 0 as guint;
    if !param.is_null()
        && g_value_type_compatible(
            (*(*(param as *mut GTypeInstance)).g_class).g_type,
            (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
        ) == 0
    {
        g_param_spec_unref(param);
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        changed = changed.wrapping_add(1);
    }
    return changed as gboolean;
}
unsafe extern "C" fn param_boxed_init(mut pspec: *mut GParamSpec) {}
unsafe extern "C" fn param_boxed_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
}
unsafe extern "C" fn param_boxed_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    let mut p1: *mut guint8 =
        (*value1).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut guint8;
    let mut p2: *mut guint8 =
        (*value2).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut guint8;
    return if p1 < p2 {
        -(1 as gint)
    } else {
        (p1 > p2) as ::core::ffi::c_int
    };
}
unsafe extern "C" fn param_pointer_init(mut pspec: *mut GParamSpec) {}
unsafe extern "C" fn param_pointer_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
}
unsafe extern "C" fn param_pointer_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    let mut p1: *mut guint8 =
        (*value1).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut guint8;
    let mut p2: *mut guint8 =
        (*value2).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut guint8;
    return if p1 < p2 {
        -(1 as gint)
    } else {
        (p1 > p2) as ::core::ffi::c_int
    };
}
unsafe extern "C" fn param_value_array_init(mut pspec: *mut GParamSpec) {
    let mut aspec: *mut GParamSpecValueArray =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecValueArray;
    (*aspec).element_spec = ::core::ptr::null_mut::<GParamSpec>();
    (*aspec).fixed_n_elements = 0 as guint;
}
#[inline]
unsafe extern "C" fn value_array_ensure_size(
    mut value_array: *mut GValueArray,
    mut fixed_n_elements: guint,
) -> guint {
    let mut changed: guint = 0 as guint;
    if fixed_n_elements != 0 {
        while (*value_array).n_values < fixed_n_elements {
            g_value_array_append(value_array, ::core::ptr::null::<GValue>());
            changed = changed.wrapping_add(1);
        }
        while (*value_array).n_values > fixed_n_elements {
            g_value_array_remove(
                value_array,
                (*value_array).n_values.wrapping_sub(1 as guint),
            );
            changed = changed.wrapping_add(1);
        }
    }
    return changed;
}
unsafe extern "C" fn param_value_array_finalize(mut pspec: *mut GParamSpec) {
    let mut aspec: *mut GParamSpecValueArray =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecValueArray;
    let mut parent_class: *mut GParamSpecClass = g_type_class_peek(g_type_parent(
        *g_param_spec_types.offset(18 as ::core::ffi::c_int as isize),
    )) as *mut GParamSpecClass;
    if !(*aspec).element_spec.is_null() {
        g_param_spec_unref((*aspec).element_spec);
        (*aspec).element_spec = ::core::ptr::null_mut::<GParamSpec>();
    }
    (*parent_class).finalize.expect("non-null function pointer")(pspec);
}
unsafe extern "C" fn param_value_array_set_default(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) {
    let mut aspec: *mut GParamSpecValueArray =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecValueArray;
    if (*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
        && (*aspec).fixed_n_elements != 0
    {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_value_array_new((*aspec).fixed_n_elements) as gpointer;
    }
    if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        value_array_ensure_size(
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GValueArray,
            (*aspec).fixed_n_elements,
        );
    }
}
unsafe extern "C" fn param_value_array_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut aspec: *mut GParamSpecValueArray =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecValueArray;
    let mut value_array: *mut GValueArray =
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GValueArray;
    let mut changed: guint = 0 as guint;
    if (*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
        && (*aspec).fixed_n_elements != 0
    {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            g_value_array_new((*aspec).fixed_n_elements) as gpointer;
    }
    if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        changed = changed.wrapping_add(value_array_ensure_size(
            value_array,
            (*aspec).fixed_n_elements,
        ));
        if !(*aspec).element_spec.is_null() {
            let mut element_spec: *mut GParamSpec = (*aspec).element_spec;
            let mut i: guint = 0;
            i = 0 as guint;
            while i < (*value_array).n_values {
                let mut element: *mut GValue = (*value_array).values.offset(i as isize);
                if g_value_type_compatible(
                    (*element).g_type,
                    (*(element_spec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
                ) == 0
                {
                    if (*element).g_type != 0 as GType {
                        g_value_unset(element);
                    }
                    g_value_init(
                        element,
                        (*(element_spec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
                    );
                    g_param_value_set_default(element_spec, element);
                    changed = changed.wrapping_add(1);
                } else {
                    changed = changed
                        .wrapping_add(g_param_value_validate(element_spec, element) as guint);
                }
                i = i.wrapping_add(1);
            }
        }
    }
    return changed as gboolean;
}
unsafe extern "C" fn param_value_array_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    let mut aspec: *mut GParamSpecValueArray =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecValueArray;
    let mut value_array1: *mut GValueArray =
        (*value1).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GValueArray;
    let mut value_array2: *mut GValueArray =
        (*value2).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GValueArray;
    if value_array1.is_null() || value_array2.is_null() {
        return if !value_array2.is_null() {
            -(1 as gint)
        } else {
            (value_array1 != value_array2) as ::core::ffi::c_int
        };
    }
    if (*value_array1).n_values != (*value_array2).n_values {
        return if (*value_array1).n_values < (*value_array2).n_values {
            -(1 as gint)
        } else {
            1 as gint
        };
    } else if (*aspec).element_spec.is_null() {
        return if (*value_array1).n_values < (*value_array2).n_values {
            -(1 as gint)
        } else {
            ((*value_array1).n_values > (*value_array2).n_values) as ::core::ffi::c_int
        };
    } else {
        let mut i: guint = 0;
        i = 0 as guint;
        while i < (*value_array1).n_values {
            let mut element1: *mut GValue = (*value_array1).values.offset(i as isize);
            let mut element2: *mut GValue = (*value_array2).values.offset(i as isize);
            let mut cmp: gint = 0;
            if (*element1).g_type != (*element2).g_type {
                return if (*element1).g_type < (*element2).g_type {
                    -(1 as gint)
                } else {
                    1 as gint
                };
            }
            cmp = g_param_values_cmp((*aspec).element_spec, element1, element2);
            if cmp != 0 {
                return cmp;
            }
            i = i.wrapping_add(1);
        }
        return 0 as gint;
    };
}
unsafe extern "C" fn param_object_init(mut pspec: *mut GParamSpec) {}
unsafe extern "C" fn param_object_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
}
unsafe extern "C" fn param_object_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut ospec: *mut GParamSpecObject =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecObject;
    let mut object: *mut GObject =
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GObject;
    return (!object.is_null()
        && g_value_type_compatible(
            (*(*(object as *mut GTypeInstance)).g_class).g_type,
            (*(ospec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
        ) != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn param_object_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut ospec: *mut GParamSpecObject =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecObject;
    let mut object: *mut GObject =
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GObject;
    let mut changed: guint = 0 as guint;
    if !object.is_null()
        && g_value_type_compatible(
            (*(*(object as *mut GTypeInstance)).g_class).g_type,
            (*(ospec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type,
        ) == 0
    {
        g_object_unref(object as gpointer);
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        changed = changed.wrapping_add(1);
    }
    return changed as gboolean;
}
unsafe extern "C" fn param_object_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    let mut p1: *mut guint8 =
        (*value1).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut guint8;
    let mut p2: *mut guint8 =
        (*value2).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut guint8;
    return if p1 < p2 {
        -(1 as gint)
    } else {
        (p1 > p2) as ::core::ffi::c_int
    };
}
unsafe extern "C" fn param_override_init(mut pspec: *mut GParamSpec) {}
unsafe extern "C" fn param_override_finalize(mut pspec: *mut GParamSpec) {
    let mut ospec: *mut GParamSpecOverride =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecOverride;
    let mut parent_class: *mut GParamSpecClass = g_type_class_peek(g_type_parent(
        *g_param_spec_types.offset(20 as ::core::ffi::c_int as isize),
    )) as *mut GParamSpecClass;
    if !(*ospec).overridden.is_null() {
        g_param_spec_unref((*ospec).overridden);
        (*ospec).overridden = ::core::ptr::null_mut::<GParamSpec>();
    }
    (*parent_class).finalize.expect("non-null function pointer")(pspec);
}
unsafe extern "C" fn param_override_set_default(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) {
    let mut ospec: *mut GParamSpecOverride =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecOverride;
    g_param_value_set_default((*ospec).overridden, value);
}
unsafe extern "C" fn param_override_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut ospec: *mut GParamSpecOverride =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecOverride;
    return g_param_value_is_valid((*ospec).overridden, value);
}
unsafe extern "C" fn param_override_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut ospec: *mut GParamSpecOverride =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecOverride;
    return g_param_value_validate((*ospec).overridden, value);
}
unsafe extern "C" fn param_override_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    let mut ospec: *mut GParamSpecOverride =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecOverride;
    return g_param_values_cmp((*ospec).overridden, value1, value2);
}
unsafe extern "C" fn param_gtype_init(mut pspec: *mut GParamSpec) {}
unsafe extern "C" fn param_gtype_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    let mut tspec: *mut GParamSpecGType = pspec as *mut ::core::ffi::c_void as *mut GParamSpecGType;
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = (*tspec).is_a_type as gpointer;
}
unsafe extern "C" fn param_gtype_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut tspec: *mut GParamSpecGType = pspec as *mut ::core::ffi::c_void as *mut GParamSpecGType;
    let mut gtype: GType = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as guintptr;
    return ((*tspec).is_a_type == ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        || (gtype == (*tspec).is_a_type || g_type_is_a(gtype, (*tspec).is_a_type) != 0))
        as ::core::ffi::c_int;
}
unsafe extern "C" fn param_gtype_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut tspec: *mut GParamSpecGType = pspec as *mut ::core::ffi::c_void as *mut GParamSpecGType;
    let mut gtype: GType = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as guintptr;
    let mut changed: guint = 0 as guint;
    if (*tspec).is_a_type != ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        && !(gtype == (*tspec).is_a_type || g_type_is_a(gtype, (*tspec).is_a_type) != 0)
    {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = (*tspec).is_a_type as gpointer;
        changed = changed.wrapping_add(1);
    }
    return changed as gboolean;
}
unsafe extern "C" fn param_gtype_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    let mut p1: GType = (*value1).data[0 as ::core::ffi::c_int as usize].v_pointer as guintptr;
    let mut p2: GType = (*value2).data[0 as ::core::ffi::c_int as usize].v_pointer as guintptr;
    return if p1 < p2 {
        -(1 as gint)
    } else {
        (p1 > p2) as ::core::ffi::c_int
    };
}
unsafe extern "C" fn param_variant_init(mut pspec: *mut GParamSpec) {
    let mut vspec: *mut GParamSpecVariant =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecVariant;
    (*vspec).type_0 = ::core::ptr::null_mut::<GVariantType>();
    (*vspec).default_value = ::core::ptr::null_mut::<GVariant>();
}
unsafe extern "C" fn param_variant_finalize(mut pspec: *mut GParamSpec) {
    let mut vspec: *mut GParamSpecVariant =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecVariant;
    let mut parent_class: *mut GParamSpecClass = g_type_class_peek(g_type_parent(
        *g_param_spec_types.offset(22 as ::core::ffi::c_int as isize),
    )) as *mut GParamSpecClass;
    if !(*vspec).default_value.is_null() {
        g_variant_unref((*vspec).default_value);
    }
    g_variant_type_free((*vspec).type_0);
    (*parent_class).finalize.expect("non-null function pointer")(pspec);
}
unsafe extern "C" fn param_variant_set_default(mut pspec: *mut GParamSpec, mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecVariant)).default_value as gpointer;
    (*value).data[1 as ::core::ffi::c_int as usize].v_uint |=
        ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
}
unsafe extern "C" fn param_variant_is_valid(
    mut pspec: *mut GParamSpec,
    mut value: *const GValue,
) -> gboolean {
    let mut vspec: *mut GParamSpecVariant =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecVariant;
    let mut variant: *mut GVariant =
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant;
    if variant.is_null() {
        return ((*vspec).default_value
            == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut GVariant)
            as ::core::ffi::c_int;
    } else {
        return g_variant_is_of_type(variant, (*vspec).type_0);
    };
}
unsafe extern "C" fn param_variant_validate(
    mut pspec: *mut GParamSpec,
    mut value: *mut GValue,
) -> gboolean {
    let mut vspec: *mut GParamSpecVariant =
        pspec as *mut ::core::ffi::c_void as *mut GParamSpecVariant;
    let mut variant: *mut GVariant =
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant;
    if variant.is_null() && !(*vspec).default_value.is_null()
        || !variant.is_null() && g_variant_is_of_type(variant, (*vspec).type_0) == 0
    {
        g_param_value_set_default(pspec, value);
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return 0 as gboolean;
}
unsafe extern "C" fn variant_is_incomparable(mut v: *mut GVariant) -> gboolean {
    let mut v_class: GVariantClass = g_variant_classify(v);
    return (v_class as ::core::ffi::c_uint
        == G_VARIANT_CLASS_HANDLE as ::core::ffi::c_int as ::core::ffi::c_uint
        || v_class as ::core::ffi::c_uint
            == G_VARIANT_CLASS_VARIANT as ::core::ffi::c_int as ::core::ffi::c_uint
        || v_class as ::core::ffi::c_uint
            == G_VARIANT_CLASS_MAYBE as ::core::ffi::c_int as ::core::ffi::c_uint
        || v_class as ::core::ffi::c_uint
            == G_VARIANT_CLASS_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint
        || v_class as ::core::ffi::c_uint
            == G_VARIANT_CLASS_TUPLE as ::core::ffi::c_int as ::core::ffi::c_uint
        || v_class as ::core::ffi::c_uint
            == G_VARIANT_CLASS_DICT_ENTRY as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn param_variant_values_cmp(
    mut pspec: *mut GParamSpec,
    mut value1: *const GValue,
    mut value2: *const GValue,
) -> gint {
    let mut v1: *mut GVariant =
        (*value1).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant;
    let mut v2: *mut GVariant =
        (*value2).data[0 as ::core::ffi::c_int as usize].v_pointer as *mut GVariant;
    if v1.is_null() && v2.is_null() {
        return 0 as gint;
    } else if v1.is_null() && !v2.is_null() {
        return -(1 as gint);
    } else if !v1.is_null() && v2.is_null() {
        return 1 as gint;
    }
    if g_variant_type_equal(
        g_variant_get_type(v1) as gconstpointer,
        g_variant_get_type(v2) as gconstpointer,
    ) == 0
        || variant_is_incomparable(v1) != 0
        || variant_is_incomparable(v2) != 0
    {
        return if g_variant_equal(v1 as gconstpointer, v2 as gconstpointer) != 0 {
            0 as gint
        } else if v1 < v2 {
            -(1 as gint)
        } else {
            1 as gint
        };
    }
    return g_variant_compare(v1 as gconstpointer, v2 as gconstpointer);
}
#[no_mangle]
pub static mut g_param_spec_types: *mut GType = ::core::ptr::null::<GType>() as *mut GType;
#[inline]
unsafe extern "C" fn ensure_param_spec_types() {
    if g_param_spec_types.is_null() {
        crate::translated::original::gobject::gtype::bootstrap_type_system();
    }
}
#[no_mangle]
pub unsafe extern "C" fn _g_param_spec_types_init() {
    let n_types: guint = 23 as guint;
    let mut type_0: GType = 0;
    let mut spec_types: *mut GType = ::core::ptr::null_mut::<GType>();
    let mut spec_types_bound: *mut GType = ::core::ptr::null_mut::<GType>();
    g_param_spec_types =
        g_malloc0_n(n_types as gsize, ::core::mem::size_of::<GType>() as gsize) as *mut GType;
    spec_types = g_param_spec_types;
    spec_types_bound = g_param_spec_types.offset(n_types as isize);
    let pspec_info: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecChar>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_char_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_char_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_char_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_int_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamChar\0" as *const u8 as *const gchar),
        &raw const pspec_info,
    );
    let mut class: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class).value_is_valid = Some(
        param_char_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class as gpointer);
    let fresh1 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh1 = type_0;
    if type_0 == *g_param_spec_types.offset(0 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1407 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_CHAR\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_0: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecUChar>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_uchar_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((4 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_uchar_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_uchar_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_uint_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamUChar\0" as *const u8 as *const gchar),
        &raw const pspec_info_0,
    );
    let mut class_0: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_0).value_is_valid = Some(
        param_uchar_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_0 as gpointer);
    let fresh2 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh2 = type_0;
    if type_0 == *g_param_spec_types.offset(1 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1426 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_UCHAR\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_1: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecBoolean>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: None,
        value_type: ((5 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_boolean_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_boolean_validate
                as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_int_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamBoolean\0" as *const u8 as *const gchar),
        &raw const pspec_info_1,
    );
    let mut class_1: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_1).value_is_valid = Some(
        param_boolean_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_1 as gpointer);
    let fresh3 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh3 = type_0;
    if type_0 == *g_param_spec_types.offset(2 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1445 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_BOOLEAN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_2: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecInt>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_int_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((6 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_int_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_int_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_int_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamInt\0" as *const u8 as *const gchar),
        &raw const pspec_info_2,
    );
    let mut class_2: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_2).value_is_valid = Some(
        param_int_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_2 as gpointer);
    let fresh4 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh4 = type_0;
    if type_0 == *g_param_spec_types.offset(3 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1464 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_INT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_3: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecUInt>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_uint_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_uint_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_uint_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_uint_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamUInt\0" as *const u8 as *const gchar),
        &raw const pspec_info_3,
    );
    let mut class_3: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_3).value_is_valid = Some(
        param_uint_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_3 as gpointer);
    let fresh5 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh5 = type_0;
    if type_0 == *g_param_spec_types.offset(4 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1483 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_UINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_4: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecLong>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_long_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((8 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_long_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_long_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_long_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamLong\0" as *const u8 as *const gchar),
        &raw const pspec_info_4,
    );
    let mut class_4: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_4).value_is_valid = Some(
        param_long_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_4 as gpointer);
    let fresh6 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh6 = type_0;
    if type_0 == *g_param_spec_types.offset(5 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1502 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_LONG\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_5: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecULong>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_ulong_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((9 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_ulong_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_ulong_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_ulong_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamULong\0" as *const u8 as *const gchar),
        &raw const pspec_info_5,
    );
    let mut class_5: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_5).value_is_valid = Some(
        param_ulong_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_5 as gpointer);
    let fresh7 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh7 = type_0;
    if type_0 == *g_param_spec_types.offset(6 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1521 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_ULONG\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_6: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecInt64>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_int64_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((10 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_int64_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_int64_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_int64_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamInt64\0" as *const u8 as *const gchar),
        &raw const pspec_info_6,
    );
    let mut class_6: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_6).value_is_valid = Some(
        param_int64_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_6 as gpointer);
    let fresh8 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh8 = type_0;
    if type_0 == *g_param_spec_types.offset(7 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1540 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_INT64\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_7: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecUInt64>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_uint64_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((11 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_uint64_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_uint64_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_uint64_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamUInt64\0" as *const u8 as *const gchar),
        &raw const pspec_info_7,
    );
    let mut class_7: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_7).value_is_valid = Some(
        param_uint64_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_7 as gpointer);
    let fresh9 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh9 = type_0;
    if type_0 == *g_param_spec_types.offset(8 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1559 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_UINT64\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_8: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecUnichar>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_unichar_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((7 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_unichar_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_unichar_validate
                as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_unichar_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamUnichar\0" as *const u8 as *const gchar),
        &raw const pspec_info_8,
    );
    let mut class_8: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_8).value_is_valid = Some(
        param_unichar_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_8 as gpointer);
    let fresh10 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh10 = type_0;
    if type_0 == *g_param_spec_types.offset(9 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1578 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_UNICHAR\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_9: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecEnum>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_enum_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: Some(param_enum_finalize as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_set_default: Some(
            param_enum_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_enum_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_long_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamEnum\0" as *const u8 as *const gchar),
        &raw const pspec_info_9,
    );
    let mut class_9: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_9).value_is_valid = Some(
        param_enum_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_9 as gpointer);
    let fresh11 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh11 = type_0;
    if type_0 == *g_param_spec_types.offset(10 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1597 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_ENUM\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_10: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecFlags>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_flags_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: Some(param_flags_finalize as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_set_default: Some(
            param_flags_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_flags_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_ulong_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamFlags\0" as *const u8 as *const gchar),
        &raw const pspec_info_10,
    );
    let mut class_10: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_10).value_is_valid = Some(
        param_flags_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_10 as gpointer);
    let fresh12 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh12 = type_0;
    if type_0 == *g_param_spec_types.offset(11 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1616 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_FLAGS\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_11: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecFloat>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_float_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((14 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_float_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_float_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_float_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamFloat\0" as *const u8 as *const gchar),
        &raw const pspec_info_11,
    );
    let mut class_11: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_11).value_is_valid = Some(
        param_float_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_11 as gpointer);
    let fresh13 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh13 = type_0;
    if type_0 == *g_param_spec_types.offset(12 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1635 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_FLOAT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_12: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecDouble>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_double_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((15 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_double_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_double_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_double_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamDouble\0" as *const u8 as *const gchar),
        &raw const pspec_info_12,
    );
    let mut class_12: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_12).value_is_valid = Some(
        param_double_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_12 as gpointer);
    let fresh14 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh14 = type_0;
    if type_0 == *g_param_spec_types.offset(13 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1654 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_DOUBLE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_13: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecString>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_string_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: Some(param_string_finalize as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_set_default: Some(
            param_string_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_string_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_string_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamString\0" as *const u8 as *const gchar),
        &raw const pspec_info_13,
    );
    let mut class_13: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_13).value_is_valid = Some(
        param_string_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_13 as gpointer);
    let fresh15 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh15 = type_0;
    if type_0 == *g_param_spec_types.offset(14 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1673 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_STRING\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_14: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecParam>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_param_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_param_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_param_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_pointer_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamParam\0" as *const u8 as *const gchar),
        &raw const pspec_info_14,
    );
    let mut class_14: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_14).value_is_valid = Some(
        param_param_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_14 as gpointer);
    let fresh16 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh16 = type_0;
    if type_0 == *g_param_spec_types.offset(15 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1692 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_PARAM\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_15: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecBoxed>() as guint16,
        n_preallocs: 4 as guint16,
        instance_init: Some(param_boxed_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_boxed_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: None,
        values_cmp: Some(
            param_boxed_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamBoxed\0" as *const u8 as *const gchar),
        &raw const pspec_info_15,
    );
    let fresh17 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh17 = type_0;
    if type_0 == *g_param_spec_types.offset(16 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1710 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_BOXED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_16: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecPointer>() as guint16,
        n_preallocs: 0 as guint16,
        instance_init: Some(param_pointer_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((17 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_pointer_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: None,
        values_cmp: Some(
            param_pointer_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamPointer\0" as *const u8 as *const gchar),
        &raw const pspec_info_16,
    );
    let fresh18 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh18 = type_0;
    if type_0 == *g_param_spec_types.offset(17 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1728 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_POINTER\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut pspec_info_17: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecValueArray>() as guint16,
        n_preallocs: 0 as guint16,
        instance_init: Some(param_value_array_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: 0xdeadbeef as GType,
        finalize: Some(param_value_array_finalize as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_set_default: Some(
            param_value_array_set_default
                as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_value_array_validate
                as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_value_array_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    pspec_info_17.value_type = g_value_array_get_type();
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamValueArray\0" as *const u8 as *const gchar),
        &raw mut pspec_info_17,
    );
    let fresh19 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh19 = type_0;
    if type_0 == *g_param_spec_types.offset(18 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1747 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_VALUE_ARRAY\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_18: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecObject>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_object_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: None,
        value_set_default: Some(
            param_object_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_object_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_object_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamObject\0" as *const u8 as *const gchar),
        &raw const pspec_info_18,
    );
    let mut class_15: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_15).value_is_valid = Some(
        param_object_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_15 as gpointer);
    let fresh20 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh20 = type_0;
    if type_0 == *g_param_spec_types.offset(19 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1766 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_OBJECT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_19: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecOverride>() as guint16,
        n_preallocs: 16 as guint16,
        instance_init: Some(param_override_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: Some(param_override_finalize as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_set_default: Some(
            param_override_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_override_validate
                as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_override_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamOverride\0" as *const u8 as *const gchar),
        &raw const pspec_info_19,
    );
    let mut class_16: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_16).value_is_valid = Some(
        param_override_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_16 as gpointer);
    let fresh21 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh21 = type_0;
    if type_0 == *g_param_spec_types.offset(20 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1785 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_OVERRIDE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut pspec_info_20: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecGType>() as guint16,
        n_preallocs: 0 as guint16,
        instance_init: Some(param_gtype_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: 0xdeadbeef as GType,
        finalize: None,
        value_set_default: Some(
            param_gtype_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_gtype_validate as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_gtype_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    pspec_info_20.value_type = g_gtype_get_type();
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamGType\0" as *const u8 as *const gchar),
        &raw mut pspec_info_20,
    );
    let mut class_17: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_17).value_is_valid = Some(
        param_gtype_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_17 as gpointer);
    let fresh22 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh22 = type_0;
    if type_0 == *g_param_spec_types.offset(21 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1805 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_GTYPE\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let pspec_info_21: GParamSpecTypeInfo = _GParamSpecTypeInfo {
        instance_size: ::core::mem::size_of::<GParamSpecVariant>() as guint16,
        n_preallocs: 0 as guint16,
        instance_init: Some(param_variant_init as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_type: ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        finalize: Some(param_variant_finalize as unsafe extern "C" fn(*mut GParamSpec) -> ()),
        value_set_default: Some(
            param_variant_set_default as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> (),
        ),
        value_validate: Some(
            param_variant_validate
                as unsafe extern "C" fn(*mut GParamSpec, *mut GValue) -> gboolean,
        ),
        values_cmp: Some(
            param_variant_values_cmp
                as unsafe extern "C" fn(*mut GParamSpec, *const GValue, *const GValue) -> gint,
        ),
    };
    type_0 = g_param_type_register_static(
        g_intern_static_string(b"GParamVariant\0" as *const u8 as *const gchar),
        &raw const pspec_info_21,
    );
    let mut class_18: *mut GParamSpecClass = g_type_class_ref(type_0) as *mut GParamSpecClass;
    (*class_18).value_is_valid = Some(
        param_variant_is_valid as unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GParamSpec, *const GValue) -> gboolean>;
    g_type_class_unref(class_18 as gpointer);
    let fresh23 = spec_types;
    spec_types = spec_types.offset(1);
    *fresh23 = type_0;
    if type_0 == *g_param_spec_types.offset(22 as ::core::ffi::c_int as isize) {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1824 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_PARAM_VARIANT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if spec_types == spec_types_bound {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gparamspecs.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1827 as ::core::ffi::c_int,
            b"_g_param_spec_types_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"spec_types == spec_types_bound\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_char(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut minimum: gint8,
    mut maximum: gint8,
    mut default_value: gint8,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut cspec: *mut GParamSpecChar = ::core::ptr::null_mut::<GParamSpecChar>();
    if default_value as ::core::ffi::c_int >= minimum as ::core::ffi::c_int
        && default_value as ::core::ffi::c_int <= maximum as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_char\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value >= minimum && default_value <= maximum\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    cspec = g_param_spec_internal(
        *g_param_spec_types.offset(0 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecChar;
    (*cspec).minimum = minimum;
    (*cspec).maximum = maximum;
    (*cspec).default_value = default_value;
    return cspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_uchar(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut minimum: guint8,
    mut maximum: guint8,
    mut default_value: guint8,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut uspec: *mut GParamSpecUChar = ::core::ptr::null_mut::<GParamSpecUChar>();
    if default_value as ::core::ffi::c_int >= minimum as ::core::ffi::c_int
        && default_value as ::core::ffi::c_int <= maximum as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_uchar\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value >= minimum && default_value <= maximum\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    uspec = g_param_spec_internal(
        *g_param_spec_types.offset(1 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecUChar;
    (*uspec).minimum = minimum;
    (*uspec).maximum = maximum;
    (*uspec).default_value = default_value;
    return uspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_boolean(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut default_value: gboolean,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut bspec: *mut GParamSpecBoolean = ::core::ptr::null_mut::<GParamSpecBoolean>();
    if default_value == (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
        || default_value == 0 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_boolean\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value == TRUE || default_value == FALSE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    bspec = g_param_spec_internal(
        *g_param_spec_types.offset(2 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecBoolean;
    (*bspec).default_value = default_value;
    return bspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_int(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut minimum: gint,
    mut maximum: gint,
    mut default_value: gint,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut ispec: *mut GParamSpecInt = ::core::ptr::null_mut::<GParamSpecInt>();
    if default_value >= minimum && default_value <= maximum {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_int\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value >= minimum && default_value <= maximum\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    ispec = g_param_spec_internal(
        *g_param_spec_types.offset(3 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecInt;
    (*ispec).minimum = minimum;
    (*ispec).maximum = maximum;
    (*ispec).default_value = default_value;
    return ispec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_uint(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut minimum: guint,
    mut maximum: guint,
    mut default_value: guint,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut uspec: *mut GParamSpecUInt = ::core::ptr::null_mut::<GParamSpecUInt>();
    if default_value >= minimum && default_value <= maximum {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_uint\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value >= minimum && default_value <= maximum\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    uspec = g_param_spec_internal(
        *g_param_spec_types.offset(4 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecUInt;
    (*uspec).minimum = minimum;
    (*uspec).maximum = maximum;
    (*uspec).default_value = default_value;
    return uspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_long(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut minimum: glong,
    mut maximum: glong,
    mut default_value: glong,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut lspec: *mut GParamSpecLong = ::core::ptr::null_mut::<GParamSpecLong>();
    if default_value >= minimum && default_value <= maximum {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_long\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value >= minimum && default_value <= maximum\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    lspec = g_param_spec_internal(
        *g_param_spec_types.offset(5 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecLong;
    (*lspec).minimum = minimum;
    (*lspec).maximum = maximum;
    (*lspec).default_value = default_value;
    return lspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_ulong(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut minimum: gulong,
    mut maximum: gulong,
    mut default_value: gulong,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut uspec: *mut GParamSpecULong = ::core::ptr::null_mut::<GParamSpecULong>();
    if default_value >= minimum && default_value <= maximum {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_ulong\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value >= minimum && default_value <= maximum\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    uspec = g_param_spec_internal(
        *g_param_spec_types.offset(6 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecULong;
    (*uspec).minimum = minimum;
    (*uspec).maximum = maximum;
    (*uspec).default_value = default_value;
    return uspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_int64(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut minimum: gint64,
    mut maximum: gint64,
    mut default_value: gint64,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut lspec: *mut GParamSpecInt64 = ::core::ptr::null_mut::<GParamSpecInt64>();
    if default_value >= minimum && default_value <= maximum {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_int64\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value >= minimum && default_value <= maximum\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    lspec = g_param_spec_internal(
        *g_param_spec_types.offset(7 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecInt64;
    (*lspec).minimum = minimum;
    (*lspec).maximum = maximum;
    (*lspec).default_value = default_value;
    return lspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_uint64(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut minimum: guint64,
    mut maximum: guint64,
    mut default_value: guint64,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut uspec: *mut GParamSpecUInt64 = ::core::ptr::null_mut::<GParamSpecUInt64>();
    if default_value >= minimum && default_value <= maximum {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_uint64\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value >= minimum && default_value <= maximum\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    uspec = g_param_spec_internal(
        *g_param_spec_types.offset(8 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecUInt64;
    (*uspec).minimum = minimum;
    (*uspec).maximum = maximum;
    (*uspec).default_value = default_value;
    return uspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_unichar(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut default_value: gunichar,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut uspec: *mut GParamSpecUnichar = ::core::ptr::null_mut::<GParamSpecUnichar>();
    uspec = g_param_spec_internal(
        *g_param_spec_types.offset(9 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecUnichar;
    (*uspec).default_value = default_value;
    return uspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_enum(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut enum_type: GType,
    mut default_value: gint,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut espec: *mut GParamSpecEnum = ::core::ptr::null_mut::<GParamSpecEnum>();
    let mut enum_class: *mut GEnumClass = ::core::ptr::null_mut::<GEnumClass>();
    if g_type_fundamental(enum_type)
        == ((12 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_enum\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_ENUM (enum_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    enum_class = g_type_class_ref(enum_type) as *mut GEnumClass;
    if !g_enum_get_value(enum_class, default_value).is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_enum\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_enum_get_value (enum_class, default_value) != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    espec = g_param_spec_internal(
        *g_param_spec_types.offset(10 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecEnum;
    (*espec).enum_class = enum_class;
    (*espec).default_value = default_value;
    (*(espec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type = enum_type;
    return espec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_flags(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut flags_type: GType,
    mut default_value: guint,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut fspec: *mut GParamSpecFlags = ::core::ptr::null_mut::<GParamSpecFlags>();
    let mut flags_class: *mut GFlagsClass = ::core::ptr::null_mut::<GFlagsClass>();
    if g_type_fundamental(flags_type)
        == ((13 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_flags\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_FLAGS (flags_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    flags_class = g_type_class_ref(flags_type) as *mut GFlagsClass;
    if default_value & (*flags_class).mask == default_value {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_flags\0" as *const u8 as *const ::core::ffi::c_char,
            b"(default_value & flags_class->mask) == default_value\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    fspec = g_param_spec_internal(
        *g_param_spec_types.offset(11 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecFlags;
    (*fspec).flags_class = flags_class;
    (*fspec).default_value = default_value;
    (*(fspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type = flags_type;
    return fspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_float(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut minimum: gfloat,
    mut maximum: gfloat,
    mut default_value: gfloat,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut fspec: *mut GParamSpecFloat = ::core::ptr::null_mut::<GParamSpecFloat>();
    if default_value >= minimum && default_value <= maximum {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_float\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value >= minimum && default_value <= maximum\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    fspec = g_param_spec_internal(
        *g_param_spec_types.offset(12 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecFloat;
    (*fspec).minimum = minimum;
    (*fspec).maximum = maximum;
    (*fspec).default_value = default_value;
    return fspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_double(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut minimum: gdouble,
    mut maximum: gdouble,
    mut default_value: gdouble,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut dspec: *mut GParamSpecDouble = ::core::ptr::null_mut::<GParamSpecDouble>();
    if default_value >= minimum && default_value <= maximum {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_double\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value >= minimum && default_value <= maximum\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    dspec = g_param_spec_internal(
        *g_param_spec_types.offset(13 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecDouble;
    (*dspec).minimum = minimum;
    (*dspec).maximum = maximum;
    (*dspec).default_value = default_value;
    return dspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_string(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut default_value: *const gchar,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut sspec: *mut GParamSpecString = g_param_spec_internal(
        *g_param_spec_types.offset(14 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecString;
    g_free((*sspec).default_value as gpointer);
    (*sspec).default_value =
        g_strdup_inline(default_value as *const ::core::ffi::c_char) as *mut gchar;
    return sspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_param(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut param_type: GType,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut pspec: *mut GParamSpecParam = ::core::ptr::null_mut::<GParamSpecParam>();
    if g_type_fundamental(param_type)
        == ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_param\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_PARAM (param_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    pspec = g_param_spec_internal(
        *g_param_spec_types.offset(15 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecParam;
    (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type = param_type;
    return pspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_boxed(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut boxed_type: GType,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut bspec: *mut GParamSpecBoxed = ::core::ptr::null_mut::<GParamSpecBoxed>();
    if g_type_fundamental(boxed_type)
        == ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_BOXED (boxed_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    if g_type_check_is_value_type(boxed_type) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_VALUE_TYPE (boxed_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    bspec = g_param_spec_internal(
        *g_param_spec_types.offset(16 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecBoxed;
    (*(bspec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type = boxed_type;
    return bspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_pointer(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut pspec: *mut GParamSpecPointer = ::core::ptr::null_mut::<GParamSpecPointer>();
    pspec = g_param_spec_internal(
        *g_param_spec_types.offset(17 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecPointer;
    return pspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_gtype(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut is_a_type: GType,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut tspec: *mut GParamSpecGType = ::core::ptr::null_mut::<GParamSpecGType>();
    tspec = g_param_spec_internal(
        *g_param_spec_types.offset(21 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecGType;
    (*tspec).is_a_type = is_a_type;
    return tspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_value_array(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut element_spec: *mut GParamSpec,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut aspec: *mut GParamSpecValueArray = ::core::ptr::null_mut::<GParamSpecValueArray>();
    if element_spec.is_null()
        || g_type_check_instance_is_fundamentally_a(
            element_spec as *mut GTypeInstance,
            ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_value_array\0" as *const u8 as *const ::core::ffi::c_char,
            b"element_spec == NULL || G_IS_PARAM_SPEC (element_spec)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    aspec = g_param_spec_internal(
        *g_param_spec_types.offset(18 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecValueArray;
    if !element_spec.is_null() {
        (*aspec).element_spec = g_param_spec_ref(element_spec);
        g_param_spec_sink(element_spec);
    }
    return aspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_object(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut object_type: GType,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut ospec: *mut GParamSpecObject = ::core::ptr::null_mut::<GParamSpecObject>();
    if object_type == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        || g_type_is_a(
            object_type,
            ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_object\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_is_a (object_type, G_TYPE_OBJECT)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    ospec = g_param_spec_internal(
        *g_param_spec_types.offset(19 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecObject;
    (*(ospec as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type = object_type;
    return ospec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_override(
    mut name: *const gchar,
    mut overridden: *mut GParamSpec,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut pspec: *mut GParamSpec = ::core::ptr::null_mut::<GParamSpec>();
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_override\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    if g_type_check_instance_is_fundamentally_a(
        overridden as *mut GTypeInstance,
        ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_override\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_IS_PARAM_SPEC (overridden)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    while 0 as ::core::ffi::c_int == 0 {
        let mut indirect: *mut GParamSpec = g_param_spec_get_redirect_target(overridden);
        if indirect.is_null() {
            break;
        }
        overridden = indirect;
    }
    pspec = g_param_spec_internal(
        *g_param_spec_types.offset(20 as ::core::ffi::c_int as isize),
        name,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null::<gchar>(),
        (*overridden).flags,
    ) as *mut GParamSpec;
    (*pspec).value_type = (*(overridden as *mut ::core::ffi::c_void as *mut GParamSpec)).value_type;
    let ref mut fresh0 =
        (*(pspec as *mut ::core::ffi::c_void as *mut GParamSpecOverride)).overridden;
    *fresh0 = g_param_spec_ref(overridden);
    return pspec;
}
#[no_mangle]
pub unsafe extern "C" fn g_param_spec_variant(
    mut name: *const gchar,
    mut nick: *const gchar,
    mut blurb: *const gchar,
    mut type_0: *const GVariantType,
    mut default_value: *mut GVariant,
    mut flags: GParamFlags,
) -> *mut GParamSpec {
    ensure_param_spec_types();
    let mut vspec: *mut GParamSpecVariant = ::core::ptr::null_mut::<GParamSpecVariant>();
    if !type_0.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_variant\0" as *const u8 as *const ::core::ffi::c_char,
            b"type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    if default_value.is_null() || g_variant_is_of_type(default_value, type_0) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_param_spec_variant\0" as *const u8 as *const ::core::ffi::c_char,
            b"default_value == NULL || g_variant_is_of_type (default_value, type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GParamSpec>();
    }
    vspec = g_param_spec_internal(
        *g_param_spec_types.offset(22 as ::core::ffi::c_int as isize),
        name,
        nick,
        blurb,
        flags,
    ) as *mut GParamSpecVariant;
    (*vspec).type_0 = g_variant_type_copy(type_0);
    if !default_value.is_null() {
        (*vspec).default_value = g_variant_ref_sink(default_value);
    }
    return vspec as *mut ::core::ffi::c_void as *mut GParamSpec;
}
