use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    pub type _GData;
    pub type _GHashTable;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GUnixFDListPrivate;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn g_array_sized_new(
        zero_terminated: gboolean,
        clear_: gboolean,
        element_size: guint,
        reserved_size: guint,
    ) -> *mut GArray;
    fn g_array_free(array: *mut GArray, free_segment: gboolean) -> *mut gchar;
    fn g_array_append_vals(array: *mut GArray, data: gconstpointer, len: guint) -> *mut GArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_prefix_error(err: *mut *mut GError, format: *const gchar, ...);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_dngettext(
        domain: *const gchar,
        msgid: *const gchar,
        msgid_plural: *const gchar,
        n: gulong,
    ) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_sort(list: *mut GList, compare_func: GCompareFunc) -> *mut GList;
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
    fn g_hash_table_get_keys(hash_table: *mut GHashTable) -> *mut GList;
    fn g_hash_table_get_keys_as_ptr_array(hash_table: *mut GHashTable) -> *mut GPtrArray;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_direct_hash(v: gconstpointer) -> guint;
    fn g_direct_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strdup_vprintf(format: *const gchar, args: ::core::ffi::VaList) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
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
    fn g_string_insert_c(string: *mut GString, pos: gssize, c: gchar) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_variant_type_string_is_valid(type_string: *const gchar) -> gboolean;
    fn g_variant_type_free(type_0: *mut GVariantType);
    fn g_variant_type_new(type_string: *const gchar) -> *mut GVariantType;
    fn g_variant_type_peek_string(type_0: *const GVariantType) -> *const gchar;
    fn g_variant_type_dup_string(type_0: *const GVariantType) -> *mut gchar;
    fn g_variant_type_is_tuple(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_dict_entry(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_is_variant(type_0: *const GVariantType) -> gboolean;
    fn g_variant_type_element(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_first(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_next(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_key(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_value(type_0: *const GVariantType) -> *const GVariantType;
    fn g_variant_type_checked_(type_string: *const gchar) -> *const GVariantType;
    fn g_variant_type_string_get_depth_(type_string: *const gchar) -> gsize;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_take_ref(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_get_type(value: *mut GVariant) -> *const GVariantType;
    fn g_variant_get_type_string(value: *mut GVariant) -> *const gchar;
    fn g_variant_is_of_type(value: *mut GVariant, type_0: *const GVariantType) -> gboolean;
    fn g_variant_new_boolean(value: gboolean) -> *mut GVariant;
    fn g_variant_new_byte(value: guint8) -> *mut GVariant;
    fn g_variant_new_int16(value: gint16) -> *mut GVariant;
    fn g_variant_new_uint16(value: guint16) -> *mut GVariant;
    fn g_variant_new_int32(value: gint32) -> *mut GVariant;
    fn g_variant_new_uint32(value: guint32) -> *mut GVariant;
    fn g_variant_new_int64(value: gint64) -> *mut GVariant;
    fn g_variant_new_uint64(value: guint64) -> *mut GVariant;
    fn g_variant_new_handle(value: gint32) -> *mut GVariant;
    fn g_variant_new_double(value: gdouble) -> *mut GVariant;
    fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
    fn g_variant_new_object_path(object_path: *const gchar) -> *mut GVariant;
    fn g_variant_is_object_path(string: *const gchar) -> gboolean;
    fn g_variant_new_signature(signature: *const gchar) -> *mut GVariant;
    fn g_variant_is_signature(string: *const gchar) -> gboolean;
    fn g_variant_new_variant(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_new_fixed_array(
        element_type: *const GVariantType,
        elements: gconstpointer,
        n_elements: gsize,
        element_size: gsize,
    ) -> *mut GVariant;
    fn g_variant_get_boolean(value: *mut GVariant) -> gboolean;
    fn g_variant_get_byte(value: *mut GVariant) -> guint8;
    fn g_variant_get_int16(value: *mut GVariant) -> gint16;
    fn g_variant_get_uint16(value: *mut GVariant) -> guint16;
    fn g_variant_get_int32(value: *mut GVariant) -> gint32;
    fn g_variant_get_uint32(value: *mut GVariant) -> guint32;
    fn g_variant_get_int64(value: *mut GVariant) -> gint64;
    fn g_variant_get_uint64(value: *mut GVariant) -> guint64;
    fn g_variant_get_handle(value: *mut GVariant) -> gint32;
    fn g_variant_get_double(value: *mut GVariant) -> gdouble;
    fn g_variant_get_string(value: *mut GVariant, length: *mut gsize) -> *const gchar;
    fn g_variant_new_dict_entry(key: *mut GVariant, value: *mut GVariant) -> *mut GVariant;
    fn g_variant_n_children(value: *mut GVariant) -> gsize;
    fn g_variant_get_child_value(value: *mut GVariant, index_: gsize) -> *mut GVariant;
    fn g_variant_get_size(value: *mut GVariant) -> gsize;
    fn g_variant_get_data(value: *mut GVariant) -> gconstpointer;
    fn g_variant_print(value: *mut GVariant, type_annotate: gboolean) -> *mut gchar;
    fn g_variant_print_string(
        value: *mut GVariant,
        string: *mut GString,
        type_annotate: gboolean,
    ) -> *mut GString;
    fn g_variant_byteswap(value: *mut GVariant) -> *mut GVariant;
    fn g_variant_iter_init(iter: *mut GVariantIter, value: *mut GVariant) -> gsize;
    fn g_variant_iter_next_value(iter: *mut GVariantIter) -> *mut GVariant;
    fn g_variant_builder_init(builder: *mut GVariantBuilder, type_0: *const GVariantType);
    fn g_variant_builder_end(builder: *mut GVariantBuilder) -> *mut GVariant;
    fn g_variant_builder_clear(builder: *mut GVariantBuilder);
    fn g_variant_builder_add_value(builder: *mut GVariantBuilder, value: *mut GVariant);
    fn g_variant_builder_add(builder: *mut GVariantBuilder, format_string: *const gchar, ...);
    fn g_variant_new(format_string: *const gchar, ...) -> *mut GVariant;
    fn g_variant_get(value: *mut GVariant, format_string: *const gchar, ...);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
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
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_class_ref(type_0: GType) -> gpointer;
    fn g_type_class_unref(g_class: gpointer);
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
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_class_install_property(
        oclass: *mut GObjectClass,
        property_id: guint,
        pspec: *mut GParamSpec,
    );
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_notify(object: *mut GObject, property_name: *const gchar);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_flags_get_first_value(flags_class: *mut GFlagsClass, value: guint) -> *mut GFlagsValue;
    fn g_param_spec_boolean(
        name: *const gchar,
        nick: *const gchar,
        blurb: *const gchar,
        default_value: gboolean,
        flags: GParamFlags,
    ) -> *mut GParamSpec;
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_dbus_is_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_member_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_interface_name(string: *const gchar) -> gboolean;
    fn g_dbus_is_error_name(string: *const gchar) -> gboolean;
    fn g_dbus_error_set_dbus_error(
        error: *mut *mut GError,
        dbus_error_name: *const gchar,
        dbus_error_message: *const gchar,
        format: *const gchar,
        ...
    );
    fn g_dbus_message_type_get_type() -> GType;
    fn g_dbus_message_flags_get_type() -> GType;
    fn g_dbus_message_header_field_get_type() -> GType;
    fn g_io_error_quark() -> GQuark;
    fn _g_dbus_enum_to_string(enum_type: GType, value: gint) -> *mut gchar;
    fn g_unix_fd_list_get_type() -> GType;
    fn g_unix_fd_list_new() -> *mut GUnixFDList;
    fn g_unix_fd_list_append(list: *mut GUnixFDList, fd: gint, error: *mut *mut GError) -> gint;
    fn g_unix_fd_list_get_length(list: *mut GUnixFDList) -> gint;
    fn g_unix_fd_list_peek_fds(list: *mut GUnixFDList, length: *mut gint) -> *const gint;
    fn glib_gettext(str: *const gchar) -> *const gchar;
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
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type guint8 = ::core::ffi::c_uchar;
pub type gint16 = ::core::ffi::c_short;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type goffset = gint64;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
pub type va_list = __builtin_va_list;
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
pub struct _GHashTableIter {
    pub dummy1: gpointer,
    pub dummy2: gpointer,
    pub dummy3: gpointer,
    pub dummy4: ::core::ffi::c_int,
    pub dummy5: gboolean,
    pub dummy6: gpointer,
}
pub type GHashTableIter = _GHashTableIter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
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
pub type GDataStreamByteOrder = ::core::ffi::c_uint;
pub const G_DATA_STREAM_BYTE_ORDER_HOST_ENDIAN: GDataStreamByteOrder = 2;
pub const G_DATA_STREAM_BYTE_ORDER_LITTLE_ENDIAN: GDataStreamByteOrder = 1;
pub const G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN: GDataStreamByteOrder = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: C2RustUnnamed_2 = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: C2RustUnnamed_2 = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: C2RustUnnamed_2 = 46;
pub const G_IO_ERROR_NOT_CONNECTED: C2RustUnnamed_2 = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: C2RustUnnamed_2 = 44;
pub const G_IO_ERROR_BROKEN_PIPE: C2RustUnnamed_2 = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: C2RustUnnamed_2 = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: C2RustUnnamed_2 = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: C2RustUnnamed_2 = 41;
pub const G_IO_ERROR_PROXY_FAILED: C2RustUnnamed_2 = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: C2RustUnnamed_2 = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: C2RustUnnamed_2 = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: C2RustUnnamed_2 = 37;
pub const G_IO_ERROR_DBUS_ERROR: C2RustUnnamed_2 = 36;
pub const G_IO_ERROR_INVALID_DATA: C2RustUnnamed_2 = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: C2RustUnnamed_2 = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: C2RustUnnamed_2 = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: C2RustUnnamed_2 = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: C2RustUnnamed_2 = 31;
pub const G_IO_ERROR_FAILED_HANDLED: C2RustUnnamed_2 = 30;
pub const G_IO_ERROR_WOULD_MERGE: C2RustUnnamed_2 = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: C2RustUnnamed_2 = 28;
pub const G_IO_ERROR_WOULD_BLOCK: C2RustUnnamed_2 = 27;
pub const G_IO_ERROR_BUSY: C2RustUnnamed_2 = 26;
pub const G_IO_ERROR_WOULD_RECURSE: C2RustUnnamed_2 = 25;
pub const G_IO_ERROR_TIMED_OUT: C2RustUnnamed_2 = 24;
pub const G_IO_ERROR_WRONG_ETAG: C2RustUnnamed_2 = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: C2RustUnnamed_2 = 22;
pub const G_IO_ERROR_READ_ONLY: C2RustUnnamed_2 = 21;
pub const G_IO_ERROR_PENDING: C2RustUnnamed_2 = 20;
pub const G_IO_ERROR_CANCELLED: C2RustUnnamed_2 = 19;
pub const G_IO_ERROR_CLOSED: C2RustUnnamed_2 = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: C2RustUnnamed_2 = 17;
pub const G_IO_ERROR_NOT_MOUNTED: C2RustUnnamed_2 = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: C2RustUnnamed_2 = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: C2RustUnnamed_2 = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: C2RustUnnamed_2 = 13;
pub const G_IO_ERROR_NO_SPACE: C2RustUnnamed_2 = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: C2RustUnnamed_2 = 11;
pub const G_IO_ERROR_INVALID_FILENAME: C2RustUnnamed_2 = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: C2RustUnnamed_2 = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: C2RustUnnamed_2 = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: C2RustUnnamed_2 = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: C2RustUnnamed_2 = 6;
pub const G_IO_ERROR_NOT_EMPTY: C2RustUnnamed_2 = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: C2RustUnnamed_2 = 4;
pub const G_IO_ERROR_IS_DIRECTORY: C2RustUnnamed_2 = 3;
pub const G_IO_ERROR_EXISTS: C2RustUnnamed_2 = 2;
pub const G_IO_ERROR_NOT_FOUND: C2RustUnnamed_2 = 1;
pub const G_IO_ERROR_FAILED: C2RustUnnamed_2 = 0;
pub type GDBusCapabilityFlags = ::core::ffi::c_uint;
pub const G_DBUS_CAPABILITY_FLAGS_UNIX_FD_PASSING: GDBusCapabilityFlags = 1;
pub const G_DBUS_CAPABILITY_FLAGS_NONE: GDBusCapabilityFlags = 0;
pub type GDBusMessageType = ::core::ffi::c_uint;
pub const G_DBUS_MESSAGE_TYPE_SIGNAL: GDBusMessageType = 4;
pub const G_DBUS_MESSAGE_TYPE_ERROR: GDBusMessageType = 3;
pub const G_DBUS_MESSAGE_TYPE_METHOD_RETURN: GDBusMessageType = 2;
pub const G_DBUS_MESSAGE_TYPE_METHOD_CALL: GDBusMessageType = 1;
pub const G_DBUS_MESSAGE_TYPE_INVALID: GDBusMessageType = 0;
pub type GDBusMessageFlags = ::core::ffi::c_uint;
pub const G_DBUS_MESSAGE_FLAGS_ALLOW_INTERACTIVE_AUTHORIZATION: GDBusMessageFlags = 4;
pub const G_DBUS_MESSAGE_FLAGS_NO_AUTO_START: GDBusMessageFlags = 2;
pub const G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED: GDBusMessageFlags = 1;
pub const G_DBUS_MESSAGE_FLAGS_NONE: GDBusMessageFlags = 0;
pub type GDBusMessageHeaderField = ::core::ffi::c_uint;
pub const G_DBUS_MESSAGE_HEADER_FIELD_NUM_UNIX_FDS: GDBusMessageHeaderField = 9;
pub const G_DBUS_MESSAGE_HEADER_FIELD_SIGNATURE: GDBusMessageHeaderField = 8;
pub const G_DBUS_MESSAGE_HEADER_FIELD_SENDER: GDBusMessageHeaderField = 7;
pub const G_DBUS_MESSAGE_HEADER_FIELD_DESTINATION: GDBusMessageHeaderField = 6;
pub const G_DBUS_MESSAGE_HEADER_FIELD_REPLY_SERIAL: GDBusMessageHeaderField = 5;
pub const G_DBUS_MESSAGE_HEADER_FIELD_ERROR_NAME: GDBusMessageHeaderField = 4;
pub const G_DBUS_MESSAGE_HEADER_FIELD_MEMBER: GDBusMessageHeaderField = 3;
pub const G_DBUS_MESSAGE_HEADER_FIELD_INTERFACE: GDBusMessageHeaderField = 2;
pub const G_DBUS_MESSAGE_HEADER_FIELD_PATH: GDBusMessageHeaderField = 1;
pub const G_DBUS_MESSAGE_HEADER_FIELD_INVALID: GDBusMessageHeaderField = 0;
pub type GDBusMessageByteOrder = ::core::ffi::c_uint;
pub const G_DBUS_MESSAGE_BYTE_ORDER_LITTLE_ENDIAN: GDBusMessageByteOrder = 108;
pub const G_DBUS_MESSAGE_BYTE_ORDER_BIG_ENDIAN: GDBusMessageByteOrder = 66;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GUnixFDList {
    pub parent_instance: GObject,
    pub priv_0: *mut GUnixFDListPrivate,
}
pub type GUnixFDListPrivate = _GUnixFDListPrivate;
pub type GUnixFDList = _GUnixFDList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMessage {
    pub parent_instance: GObject,
    pub type_0: GDBusMessageType,
    pub flags: GDBusMessageFlags,
    pub locked: gboolean,
    pub byte_order: GDBusMessageByteOrder,
    pub major_protocol_version: guchar,
    pub serial: guint32,
    pub headers: *mut GHashTable,
    pub body: *mut GVariant,
    pub arg0_cache: *mut GVariant,
    pub fd_list: *mut GUnixFDList,
}
pub type GDBusMessage = _GDBusMessage;
pub type GDBusMessageClass = _GDBusMessageClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GDBusMessageClass {
    pub parent_class: GObjectClass,
}
pub const PROP_LOCKED: C2RustUnnamed_5 = 1;
pub type GMemoryBuffer = _GMemoryBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMemoryBuffer {
    pub len: gsize,
    pub valid_len: gsize,
    pub pos: gsize,
    pub data: *mut gchar,
    pub byte_order: GDataStreamByteOrder,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub v_uint64: guint64,
    pub v_double: gdouble,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub v_uint64: guint64,
    pub v_double: gdouble,
}
pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const PROP_0: C2RustUnnamed_5 = 0;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_gnu_dev_major(mut __dev: __dev_t) -> ::core::ffi::c_uint {
    let mut __major: ::core::ffi::c_uint = 0;
    __major = ((__dev & 0xfff00 as ::core::ffi::c_uint as __dev_t) >> 8 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    __major = (__major as __dev_t
        | (__dev & 0xfffff00000000000 as ::core::ffi::c_ulong) >> 32 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    return __major;
}
#[inline]
unsafe extern "C" fn safe_c2rust_gnu_dev_minor(mut __dev: __dev_t) -> ::core::ffi::c_uint {
    let mut __minor: ::core::ffi::c_uint = 0;
    __minor = ((__dev & 0xff as ::core::ffi::c_uint as __dev_t) >> 0 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    __minor = (__minor as __dev_t
        | (__dev & 0xffffff00000 as ::core::ffi::c_ulong) >> 12 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    return __minor;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
        let fresh0 = (*gstring).len;
        (*gstring).len = (*gstring).len.wrapping_add(1);
        *(*gstring).str_0.offset(fresh0 as isize) = c;
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
pub const G_VARIANT_TYPE_UINT32: *const GVariantType =
    b"u\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_STRING: *const GVariantType =
    b"s\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_OBJECT_PATH: *const GVariantType =
    b"o\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_SIGNATURE: *const GVariantType =
    b"g\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
pub const G_VARIANT_TYPE_TUPLE: *const GVariantType =
    b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType;
#[inline]
unsafe extern "C" fn safe_c2rust_g_nearest_pow(mut num: gsize) -> gsize {
    let mut n: gsize = num.wrapping_sub(1 as gsize);
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if num > 0 as gsize
            && num
                <= (9223372036854775807 as ::core::ffi::c_long as ::core::ffi::c_ulong)
                    .wrapping_mul(2 as ::core::ffi::c_ulong)
                    .wrapping_add(1 as ::core::ffi::c_ulong)
                    .wrapping_div(2 as ::core::ffi::c_ulong)
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/glib/gutilsprivate.h\0"
                as *const u8 as *const ::core::ffi::c_char,
            44 as ::core::ffi::c_int,
            G_STRFUNC,
            b"num > 0 && num <= G_MAXSIZE / 2\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    n |= n >> 1 as ::core::ffi::c_int;
    n |= n >> 2 as ::core::ffi::c_int;
    n |= n >> 4 as ::core::ffi::c_int;
    n |= n >> 8 as ::core::ffi::c_int;
    n |= n >> 16 as ::core::ffi::c_int;
    n |= n >> 32 as ::core::ffi::c_int;
    return n.wrapping_add(1 as gsize);
}
pub const G_DBUS_MAX_TYPE_DEPTH: ::core::ffi::c_int =
    64 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_memory_buffer_is_byteswapped(
    mut mbuf: *mut GMemoryBuffer,
) -> gboolean {
    return ((*mbuf).byte_order as ::core::ffi::c_uint
        == G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_read_byte(
    mut mbuf: *mut GMemoryBuffer,
    mut error: *mut *mut GError,
) -> guchar {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guchar;
    }
    if (*mbuf).pos >= (*mbuf).valid_len {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Unexpected end of message while reading byte.\0" as *const u8 as *const gchar,
        );
        return 0 as guchar;
    }
    let fresh1 = (*mbuf).pos;
    (*mbuf).pos = (*mbuf).pos.wrapping_add(1);
    return *(*mbuf).data.offset(fresh1 as isize) as guchar;
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_read_int16(
    mut mbuf: *mut GMemoryBuffer,
    mut error: *mut *mut GError,
) -> gint16 {
    let mut v: gint16 = 0;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gint16;
    }
    if (*mbuf).pos > (*mbuf).valid_len.wrapping_sub(2 as gsize) {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Unexpected end of message while reading int16.\0" as *const u8 as *const gchar,
        );
        return 0 as gint16;
    }
    memcpy(
        &raw mut v as *mut ::core::ffi::c_void,
        (*mbuf).data.offset((*mbuf).pos as isize) as *const ::core::ffi::c_void,
        2 as size_t,
    );
    (*mbuf).pos = (*mbuf).pos.wrapping_add(2 as gsize);
    if safe_c2rust_g_memory_buffer_is_byteswapped(mbuf) != 0 {
        v = ((v as guint16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16
            as ::core::ffi::c_int
            | ((v as guint16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                as ::core::ffi::c_int) as guint16 as gint16;
    }
    return v;
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_read_uint16(
    mut mbuf: *mut GMemoryBuffer,
    mut error: *mut *mut GError,
) -> guint16 {
    let mut v: guint16 = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint16;
    }
    if (*mbuf).pos > (*mbuf).valid_len.wrapping_sub(2 as gsize) {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Unexpected end of message while reading uint16.\0" as *const u8 as *const gchar,
        );
        return 0 as guint16;
    }
    memcpy(
        &raw mut v as *mut ::core::ffi::c_void,
        (*mbuf).data.offset((*mbuf).pos as isize) as *const ::core::ffi::c_void,
        2 as size_t,
    );
    (*mbuf).pos = (*mbuf).pos.wrapping_add(2 as gsize);
    if safe_c2rust_g_memory_buffer_is_byteswapped(mbuf) != 0 {
        v = ((v as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16 as ::core::ffi::c_int
            | ((v as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                as ::core::ffi::c_int) as guint16;
    }
    return v;
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_read_int32(
    mut mbuf: *mut GMemoryBuffer,
    mut error: *mut *mut GError,
) -> gint32 {
    let mut v: gint32 = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint32);
    }
    if (*mbuf).pos > (*mbuf).valid_len.wrapping_sub(4 as gsize) {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Unexpected end of message while reading int32.\0" as *const u8 as *const gchar,
        );
        return 0 as gint32;
    }
    memcpy(
        &raw mut v as *mut ::core::ffi::c_void,
        (*mbuf).data.offset((*mbuf).pos as isize) as *const ::core::ffi::c_void,
        4 as size_t,
    );
    (*mbuf).pos = (*mbuf).pos.wrapping_add(4 as gsize);
    if safe_c2rust_g_memory_buffer_is_byteswapped(mbuf) != 0 {
        v = ({
            let mut __v: guint32 = 0;
            let mut __x: guint32 = v as guint32;
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
        }) as gint32;
    }
    return v;
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_read_uint32(
    mut mbuf: *mut GMemoryBuffer,
    mut error: *mut *mut GError,
) -> guint32 {
    let mut v: guint32 = 0;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    if (*mbuf).pos > (*mbuf).valid_len.wrapping_sub(4 as gsize) {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Unexpected end of message while reading uint32.\0" as *const u8 as *const gchar,
        );
        return 0 as guint32;
    }
    memcpy(
        &raw mut v as *mut ::core::ffi::c_void,
        (*mbuf).data.offset((*mbuf).pos as isize) as *const ::core::ffi::c_void,
        4 as size_t,
    );
    (*mbuf).pos = (*mbuf).pos.wrapping_add(4 as gsize);
    if safe_c2rust_g_memory_buffer_is_byteswapped(mbuf) != 0 {
        v = ({
            let mut __v: guint32 = 0;
            let mut __x: guint32 = v;
            if 0 != 0 {
                __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                    | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                    | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                    | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
            } else {
                let fresh2 = &mut __v;
                let fresh3;
                let fresh4 = __x;
                asm!(
                    "bswapl {0:e}\n", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh2, fresh4) => fresh3,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
            }
            __v
        });
    }
    return v;
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_read_int64(
    mut mbuf: *mut GMemoryBuffer,
    mut error: *mut *mut GError,
) -> gint64 {
    let mut v: gint64 = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gint64;
    }
    if (*mbuf).pos > (*mbuf).valid_len.wrapping_sub(8 as gsize) {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Unexpected end of message while reading int64.\0" as *const u8 as *const gchar,
        );
        return 0 as gint64;
    }
    memcpy(
        &raw mut v as *mut ::core::ffi::c_void,
        (*mbuf).data.offset((*mbuf).pos as isize) as *const ::core::ffi::c_void,
        8 as size_t,
    );
    (*mbuf).pos = (*mbuf).pos.wrapping_add(8 as gsize);
    if safe_c2rust_g_memory_buffer_is_byteswapped(mbuf) != 0 {
        v = ({
            let mut __v: guint64 = 0;
            let mut __x: guint64 = v as guint64;
            if 0 != 0 {
                __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                    | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                    | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                    | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                    | (__x & 0xff00000000 as ::core::ffi::c_ulong) >> 8 as ::core::ffi::c_int
                    | (__x & 0xff0000000000 as ::core::ffi::c_ulong) >> 24 as ::core::ffi::c_int
                    | (__x & 0xff000000000000 as ::core::ffi::c_ulong) >> 40 as ::core::ffi::c_int
                    | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                        >> 56 as ::core::ffi::c_int;
            } else {
                let fresh11 = &mut __v;
                let fresh12;
                let fresh13 = __x;
                asm!(
                    "bswapq {0}\n", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh11, fresh13) => fresh12,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
            }
            __v
        }) as gint64;
    }
    return v;
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_read_uint64(
    mut mbuf: *mut GMemoryBuffer,
    mut error: *mut *mut GError,
) -> guint64 {
    let mut v: guint64 = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint64;
    }
    if (*mbuf).pos > (*mbuf).valid_len.wrapping_sub(8 as gsize) {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Unexpected end of message while reading uint64.\0" as *const u8 as *const gchar,
        );
        return 0 as guint64;
    }
    memcpy(
        &raw mut v as *mut ::core::ffi::c_void,
        (*mbuf).data.offset((*mbuf).pos as isize) as *const ::core::ffi::c_void,
        8 as size_t,
    );
    (*mbuf).pos = (*mbuf).pos.wrapping_add(8 as gsize);
    if safe_c2rust_g_memory_buffer_is_byteswapped(mbuf) != 0 {
        v = ({
            let mut __v: guint64 = 0;
            let mut __x: guint64 = v;
            if 0 != 0 {
                __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                    | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                    | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                    | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                    | (__x & 0xff00000000 as ::core::ffi::c_ulong) >> 8 as ::core::ffi::c_int
                    | (__x & 0xff0000000000 as ::core::ffi::c_ulong) >> 24 as ::core::ffi::c_int
                    | (__x & 0xff000000000000 as ::core::ffi::c_ulong) >> 40 as ::core::ffi::c_int
                    | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                        >> 56 as ::core::ffi::c_int;
            } else {
                let fresh8 = &mut __v;
                let fresh9;
                let fresh10 = __x;
                asm!(
                    "bswapq {0}\n", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh10) => fresh9,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
            }
            __v
        });
    }
    return v;
}
pub const MIN_ARRAY_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_array_resize(mut mbuf: *mut GMemoryBuffer, mut size: gsize) {
    let mut data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut len: gsize = 0;
    if (*mbuf).len == size {
        return;
    }
    len = (*mbuf).len;
    data = g_realloc((*mbuf).data as gpointer, size);
    if size > len {
        memset(
            (data as *mut guint8).offset(len as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (size as size_t).wrapping_sub(len as size_t),
        );
    }
    (*mbuf).data = data as *mut gchar;
    (*mbuf).len = size;
    if (*mbuf).len < (*mbuf).valid_len {
        (*mbuf).valid_len = (*mbuf).len;
    }
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_write(
    mut mbuf: *mut GMemoryBuffer,
    mut buffer: *const ::core::ffi::c_void,
    mut count: gsize,
) -> gboolean {
    let mut dest: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut new_size: gsize = 0;
    if count == 0 as gsize {
        return TRUE;
    }
    if (*mbuf).pos.wrapping_add(count) < (*mbuf).pos {
        return FALSE;
    }
    if (*mbuf).pos.wrapping_add(count) > (*mbuf).len {
        new_size = safe_c2rust_g_nearest_pow((*mbuf).pos.wrapping_add(count));
        if new_size == 0 as gsize {
            return FALSE;
        }
        new_size = if new_size > 128 as gsize {
            new_size
        } else {
            128 as gsize
        };
        safe_c2rust_array_resize(mbuf, new_size);
    }
    dest = ((*mbuf).data as *mut guint8).offset((*mbuf).pos as isize);
    memcpy(dest as *mut ::core::ffi::c_void, buffer, count as size_t);
    (*mbuf).pos = (*mbuf).pos.wrapping_add(count);
    if (*mbuf).pos > (*mbuf).valid_len {
        (*mbuf).valid_len = (*mbuf).pos;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_put_byte(
    mut mbuf: *mut GMemoryBuffer,
    mut data: guchar,
) -> gboolean {
    return safe_c2rust_g_memory_buffer_write(
        mbuf,
        &raw mut data as *const ::core::ffi::c_void,
        1 as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_put_int16(
    mut mbuf: *mut GMemoryBuffer,
    mut data: gint16,
) -> gboolean {
    match (*mbuf).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ((data as guint16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16
                as ::core::ffi::c_int
                | ((data as guint16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                    as ::core::ffi::c_int) as guint16 as gint16;
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return safe_c2rust_g_memory_buffer_write(
        mbuf,
        &raw mut data as *const ::core::ffi::c_void,
        2 as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_put_uint16(
    mut mbuf: *mut GMemoryBuffer,
    mut data: guint16,
) -> gboolean {
    match (*mbuf).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ((data as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as guint16
                as ::core::ffi::c_int
                | ((data as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as guint16
                    as ::core::ffi::c_int) as guint16;
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return safe_c2rust_g_memory_buffer_write(
        mbuf,
        &raw mut data as *const ::core::ffi::c_void,
        2 as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_put_int32(
    mut mbuf: *mut GMemoryBuffer,
    mut data: gint32,
) -> gboolean {
    match (*mbuf).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ({
                let mut __v: guint32 = 0;
                let mut __x: guint32 = data as guint32;
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
                } else {
                    let fresh23 = &mut __v;
                    let fresh24;
                    let fresh25 = __x;
                    asm!(
                        "bswapl {0:e}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh23, fresh25) => fresh24,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh23, fresh25, fresh24);
                }
                __v
            }) as gint32;
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return safe_c2rust_g_memory_buffer_write(
        mbuf,
        &raw mut data as *const ::core::ffi::c_void,
        4 as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_put_uint32(
    mut mbuf: *mut GMemoryBuffer,
    mut data: guint32,
) -> gboolean {
    match (*mbuf).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ({
                let mut __v: guint32 = 0;
                let mut __x: guint32 = data;
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
                } else {
                    let fresh20 = &mut __v;
                    let fresh21;
                    let fresh22 = __x;
                    asm!(
                        "bswapl {0:e}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh20, fresh22) => fresh21,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh20, fresh22, fresh21);
                }
                __v
            });
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return safe_c2rust_g_memory_buffer_write(
        mbuf,
        &raw mut data as *const ::core::ffi::c_void,
        4 as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_put_int64(
    mut mbuf: *mut GMemoryBuffer,
    mut data: gint64,
) -> gboolean {
    match (*mbuf).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ({
                let mut __v: guint64 = 0;
                let mut __x: guint64 = data as guint64;
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                        | (__x & 0xff00000000 as ::core::ffi::c_ulong) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff0000000000 as ::core::ffi::c_ulong)
                            >> 24 as ::core::ffi::c_int
                        | (__x & 0xff000000000000 as ::core::ffi::c_ulong)
                            >> 40 as ::core::ffi::c_int
                        | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                            >> 56 as ::core::ffi::c_int;
                } else {
                    let fresh29 = &mut __v;
                    let fresh30;
                    let fresh31 = __x;
                    asm!(
                        "bswapq {0}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh29, fresh31) => fresh30,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh29, fresh31, fresh30);
                }
                __v
            }) as gint64;
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return safe_c2rust_g_memory_buffer_write(
        mbuf,
        &raw mut data as *const ::core::ffi::c_void,
        8 as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_put_uint64(
    mut mbuf: *mut GMemoryBuffer,
    mut data: guint64,
) -> gboolean {
    match (*mbuf).byte_order as ::core::ffi::c_uint {
        0 => {
            data = ({
                let mut __v: guint64 = 0;
                let mut __x: guint64 = data;
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_ulong) << 56 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_ulong) << 40 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int
                        | (__x & 0xff00000000 as ::core::ffi::c_ulong) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff0000000000 as ::core::ffi::c_ulong)
                            >> 24 as ::core::ffi::c_int
                        | (__x & 0xff000000000000 as ::core::ffi::c_ulong)
                            >> 40 as ::core::ffi::c_int
                        | (__x & 0xff00000000000000 as ::core::ffi::c_ulong)
                            >> 56 as ::core::ffi::c_int;
                } else {
                    let fresh26 = &mut __v;
                    let fresh27;
                    let fresh28 = __x;
                    asm!(
                        "bswapq {0}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh26, fresh28) => fresh27,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh26, fresh28, fresh27);
                }
                __v
            });
        }
        1 => {
            data = data;
        }
        2 | _ => {}
    }
    return safe_c2rust_g_memory_buffer_write(
        mbuf,
        &raw mut data as *const ::core::ffi::c_void,
        8 as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_g_memory_buffer_put_string(
    mut mbuf: *mut GMemoryBuffer,
    mut str: *const ::core::ffi::c_char,
) -> gboolean {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !str.is_null() {
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
            b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return safe_c2rust_g_memory_buffer_write(
        mbuf,
        str as *const ::core::ffi::c_void,
        strlen(str) as gsize,
    );
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_dbus_message_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GDBusMessage\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GDBusMessageClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_message_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GDBusMessage>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GDBusMessage) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_dbus_message_init as unsafe extern "C" fn(*mut GDBusMessage) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut safe_c2rust_GDBusMessage_private_offset: gint = 0;
static mut safe_c2rust_g_dbus_message_parent_class: gpointer = NULL;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_dbus_message_get_type_once();
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
unsafe extern "C" fn safe_c2rust_g_dbus_message_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_dbus_message_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GDBusMessage_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GDBusMessage_private_offset);
    }
    safe_c2rust_g_dbus_message_class_init(klass as *mut GDBusMessageClass);
}
unsafe extern "C" fn safe_c2rust_g_dbus_message_finalize(mut object: *mut GObject) {
    let mut message: *mut GDBusMessage = object as *mut ::core::ffi::c_void as *mut GDBusMessage;
    if !(*message).headers.is_null() {
        g_hash_table_unref((*message).headers);
    }
    if !(*message).body.is_null() {
        g_variant_unref((*message).body);
    }
    let mut _pp: *mut *mut GVariant = &raw mut (*message).arg0_cache;
    let mut _ptr: *mut GVariant = *_pp;
    *_pp = ::core::ptr::null_mut::<GVariant>();
    if !_ptr.is_null() {
        g_variant_unref(_ptr as *mut GVariant);
    }
    if !(*message).fd_list.is_null() {
        g_object_unref((*message).fd_list as gpointer);
    }
    if (*(safe_c2rust_g_dbus_message_parent_class as *mut GObjectClass))
        .finalize
        .is_some()
    {
        (*(safe_c2rust_g_dbus_message_parent_class as *mut GObjectClass))
            .finalize
            .expect("non-null function pointer")(object);
    }
}
unsafe extern "C" fn safe_c2rust_g_dbus_message_get_property(
    mut object: *mut GObject,
    mut prop_id: guint,
    mut value: *mut GValue,
    mut pspec: *mut GParamSpec,
) {
    let mut message: *mut GDBusMessage = object as *mut ::core::ffi::c_void as *mut GDBusMessage;
    match prop_id {
        1 => {
            g_value_set_boolean(value, safe_c2rust_g_dbus_message_get_locked(message));
        }
        _ => {
            let mut _glib__object: *mut GObject = object;
            let mut _glib__pspec: *mut GParamSpec = pspec;
            let mut _glib__property_id: guint = prop_id;
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s:%d: invalid %s id %u for \"%s\" of type '%s' in '%s'\0" as *const u8
                    as *const gchar,
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                549 as ::core::ffi::c_int,
                b"property\0" as *const u8 as *const ::core::ffi::c_char,
                _glib__property_id,
                (*_glib__pspec).name,
                g_type_name((*(*(_glib__pspec as *mut GTypeInstance)).g_class).g_type),
                g_type_name((*(*(_glib__object as *mut GTypeInstance)).g_class).g_type),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_dbus_message_class_init(mut klass: *mut GDBusMessageClass) {
    let mut gobject_class: *mut GObjectClass = ::core::ptr::null_mut::<GObjectClass>();
    gobject_class = klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*gobject_class).finalize =
        Some(safe_c2rust_g_dbus_message_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*gobject_class).get_property = Some(
        safe_c2rust_g_dbus_message_get_property
            as unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
    )
        as Option<unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> ()>;
    g_object_class_install_property(
        gobject_class,
        PROP_LOCKED as ::core::ffi::c_int as guint,
        g_param_spec_boolean(
            b"locked\0" as *const u8 as *const gchar,
            ::core::ptr::null::<gchar>(),
            ::core::ptr::null::<gchar>(),
            FALSE,
            (G_PARAM_READABLE as ::core::ffi::c_int
                | G_PARAM_STATIC_NAME as ::core::ffi::c_int
                | G_PARAM_STATIC_BLURB as ::core::ffi::c_int
                | G_PARAM_STATIC_NICK as ::core::ffi::c_int) as GParamFlags,
        ),
    );
}
unsafe extern "C" fn safe_c2rust_g_dbus_message_init(mut message: *mut GDBusMessage) {
    (*message).byte_order = G_DBUS_MESSAGE_BYTE_ORDER_LITTLE_ENDIAN;
    (*message).headers = g_hash_table_new_full(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        None,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GVariant) -> ()>, GDestroyNotify>(
            Some(g_variant_unref as unsafe extern "C" fn(*mut GVariant) -> ()),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_new() -> *mut GDBusMessage {
    return g_object_new(
        safe_c2rust_g_dbus_message_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GDBusMessage;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_new_method_call(
    mut name: *const gchar,
    mut path: *const gchar,
    mut interface_: *const gchar,
    mut method: *const gchar,
) -> *mut GDBusMessage {
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if name.is_null() || g_dbus_is_name(name) != 0 {
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
            b"name == NULL || g_dbus_is_name (name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(path) != 0 {
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
            b"g_variant_is_object_path (path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if g_dbus_is_member_name(method) != 0 {
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
            b"g_dbus_is_member_name (method)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if interface_.is_null() || g_dbus_is_interface_name(interface_) != 0 {
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
            b"interface_ == NULL || g_dbus_is_interface_name (interface_)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    message = safe_c2rust_g_dbus_message_new();
    (*message).type_0 = G_DBUS_MESSAGE_TYPE_METHOD_CALL;
    if !name.is_null() {
        safe_c2rust_g_dbus_message_set_destination(message, name);
    }
    safe_c2rust_g_dbus_message_set_path(message, path);
    safe_c2rust_g_dbus_message_set_member(message, method);
    if !interface_.is_null() {
        safe_c2rust_g_dbus_message_set_interface(message, interface_);
    }
    return message;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_new_signal(
    mut path: *const gchar,
    mut interface_: *const gchar,
    mut signal: *const gchar,
) -> *mut GDBusMessage {
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if g_variant_is_object_path(path) != 0 {
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
            b"g_variant_is_object_path (path)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if g_dbus_is_member_name(signal) != 0 {
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
            b"g_dbus_is_member_name (signal)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if g_dbus_is_interface_name(interface_) != 0 {
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
            b"g_dbus_is_interface_name (interface_)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    message = safe_c2rust_g_dbus_message_new();
    (*message).type_0 = G_DBUS_MESSAGE_TYPE_SIGNAL;
    (*message).flags = G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED;
    safe_c2rust_g_dbus_message_set_path(message, path);
    safe_c2rust_g_dbus_message_set_member(message, signal);
    safe_c2rust_g_dbus_message_set_interface(message, interface_);
    return message;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_new_method_reply(
    mut method_call_message: *mut GDBusMessage,
) -> *mut GDBusMessage {
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut sender: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = method_call_message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (method_call_message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if safe_c2rust_g_dbus_message_get_message_type(method_call_message) as ::core::ffi::c_uint
            == G_DBUS_MESSAGE_TYPE_METHOD_CALL as ::core::ffi::c_int as ::core::ffi::c_uint
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
            b"g_dbus_message_get_message_type (method_call_message) == G_DBUS_MESSAGE_TYPE_METHOD_CALL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if safe_c2rust_g_dbus_message_get_serial(method_call_message) != 0 as guint32 {
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
            b"g_dbus_message_get_serial (method_call_message) != 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    message = safe_c2rust_g_dbus_message_new();
    (*message).type_0 = G_DBUS_MESSAGE_TYPE_METHOD_RETURN;
    (*message).flags = G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED;
    (*message).byte_order = (*method_call_message).byte_order;
    safe_c2rust_g_dbus_message_set_reply_serial(
        message,
        safe_c2rust_g_dbus_message_get_serial(method_call_message),
    );
    sender = safe_c2rust_g_dbus_message_get_sender(method_call_message);
    if !sender.is_null() {
        safe_c2rust_g_dbus_message_set_destination(message, sender);
    }
    return message;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_new_method_error(
    mut method_call_message: *mut GDBusMessage,
    mut error_name: *const gchar,
    mut error_message_format: *const gchar,
    mut args: ...
) -> *mut GDBusMessage {
    let mut ret: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut var_args: ::core::ffi::VaList;
    var_args = args.clone();
    ret = safe_c2rust_g_dbus_message_new_method_error_valist(
        method_call_message,
        error_name,
        error_message_format,
        var_args,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_new_method_error_literal(
    mut method_call_message: *mut GDBusMessage,
    mut error_name: *const gchar,
    mut error_message: *const gchar,
) -> *mut GDBusMessage {
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut sender: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = method_call_message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (method_call_message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if safe_c2rust_g_dbus_message_get_message_type(method_call_message) as ::core::ffi::c_uint
            == G_DBUS_MESSAGE_TYPE_METHOD_CALL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"g_dbus_message_get_message_type (method_call_message) == G_DBUS_MESSAGE_TYPE_METHOD_CALL\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if safe_c2rust_g_dbus_message_get_serial(method_call_message) != 0 as guint32 {
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
            b"g_dbus_message_get_serial (method_call_message) != 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if g_dbus_is_name(error_name) != 0 {
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
            b"g_dbus_is_name (error_name)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !error_message.is_null() {
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
            b"error_message != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    message = safe_c2rust_g_dbus_message_new();
    (*message).type_0 = G_DBUS_MESSAGE_TYPE_ERROR;
    (*message).flags = G_DBUS_MESSAGE_FLAGS_NO_REPLY_EXPECTED;
    (*message).byte_order = (*method_call_message).byte_order;
    safe_c2rust_g_dbus_message_set_reply_serial(
        message,
        safe_c2rust_g_dbus_message_get_serial(method_call_message),
    );
    safe_c2rust_g_dbus_message_set_error_name(message, error_name);
    safe_c2rust_g_dbus_message_set_body(
        message,
        g_variant_new(b"(s)\0" as *const u8 as *const gchar, error_message),
    );
    sender = safe_c2rust_g_dbus_message_get_sender(method_call_message);
    if !sender.is_null() {
        safe_c2rust_g_dbus_message_set_destination(message, sender);
    }
    return message;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_new_method_error_valist(
    mut method_call_message: *mut GDBusMessage,
    mut error_name: *const gchar,
    mut error_message_format: *const gchar,
    mut var_args: ::core::ffi::VaList,
) -> *mut GDBusMessage {
    let mut ret: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut error_message: *mut gchar = ::core::ptr::null_mut::<gchar>();
    error_message = g_strdup_vprintf(error_message_format, var_args);
    ret = safe_c2rust_g_dbus_message_new_method_error_literal(
        method_call_message,
        error_name,
        error_message,
    );
    g_free(error_message as gpointer);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_byte_order(
    mut message: *mut GDBusMessage,
) -> GDBusMessageByteOrder {
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GDBusMessageByteOrder;
    }
    return (*message).byte_order;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_byte_order(
    mut message: *mut GDBusMessage,
    mut byte_order: GDBusMessageByteOrder,
) {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*message).locked != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: Attempted to modify a locked message\0" as *const u8 as *const gchar,
            b"g_dbus_message_set_byte_order\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*message).byte_order = byte_order;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_message_type(
    mut message: *mut GDBusMessage,
) -> GDBusMessageType {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DBUS_MESSAGE_TYPE_INVALID;
    }
    return (*message).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_message_type(
    mut message: *mut GDBusMessage,
    mut type_0: GDBusMessageType,
) {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if (type_0 as guint) < 256 as guint {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(guint) type < 256\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*message).locked != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: Attempted to modify a locked message\0" as *const u8 as *const gchar,
            b"g_dbus_message_set_message_type\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*message).type_0 = type_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_flags(
    mut message: *mut GDBusMessage,
) -> GDBusMessageFlags {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return G_DBUS_MESSAGE_FLAGS_NONE;
    }
    return (*message).flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_flags(
    mut message: *mut GDBusMessage,
    mut flags: GDBusMessageFlags,
) {
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if (flags as guint) < 256 as guint {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"(guint) flags < 256\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*message).locked != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: Attempted to modify a locked message\0" as *const u8 as *const gchar,
            b"g_dbus_message_set_flags\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*message).flags = flags;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_serial(
    mut message: *mut GDBusMessage,
) -> guint32 {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    return (*message).serial;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_serial(
    mut message: *mut GDBusMessage,
    mut serial: guint32,
) {
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*message).locked != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: Attempted to modify a locked message\0" as *const u8 as *const gchar,
            b"g_dbus_message_set_serial\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*message).serial = serial;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_header(
    mut message: *mut GDBusMessage,
    mut header_field: GDBusMessageHeaderField,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if (header_field as guint) < 256 as guint {
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
            b"(guint) header_field < 256\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return g_hash_table_lookup(
        (*message).headers,
        header_field as gulong as gpointer as gconstpointer,
    ) as *mut GVariant;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_header(
    mut message: *mut GDBusMessage,
    mut header_field: GDBusMessageHeaderField,
    mut value: *mut GVariant,
) {
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if (header_field as guint) < 256 as guint {
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
            b"(guint) header_field < 256\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*message).locked != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: Attempted to modify a locked message\0" as *const u8 as *const gchar,
            b"g_dbus_message_set_header\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if value.is_null() {
        g_hash_table_remove(
            (*message).headers,
            header_field as gulong as gpointer as gconstpointer,
        );
    } else {
        g_hash_table_insert(
            (*message).headers,
            header_field as gulong as gpointer,
            g_variant_ref_sink(value) as gpointer,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_header_fields(
    mut message: *mut GDBusMessage,
) -> *mut guchar {
    let mut keys: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut array: *mut GArray = ::core::ptr::null_mut::<GArray>();
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guchar>();
    }
    keys = g_hash_table_get_keys_as_ptr_array((*message).headers);
    array = g_array_sized_new(
        FALSE,
        FALSE,
        ::core::mem::size_of::<guchar>() as guint,
        (*keys).len.wrapping_add(1 as guint),
    );
    let mut i: guint = 0 as guint;
    while i < (*keys).len {
        let mut val: guchar = *(*keys).pdata.offset(i as isize) as gulong as guint as guchar;
        g_array_append_vals(array, &raw mut val as gconstpointer, 1 as guint);
        i = i.wrapping_add(1);
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if (*array).len == (*keys).len {
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1101 as ::core::ffi::c_int,
            G_STRFUNC,
            b"array->len == keys->len\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut _pp: *mut *mut GPtrArray = &raw mut keys;
    let mut _ptr: *mut GPtrArray = *_pp;
    *_pp = ::core::ptr::null_mut::<GPtrArray>();
    if !_ptr.is_null() {
        g_ptr_array_unref(_ptr as *mut GPtrArray);
    }
    let mut invalid_field: guchar =
        G_DBUS_MESSAGE_HEADER_FIELD_INVALID as ::core::ffi::c_int as guchar;
    g_array_append_vals(array, &raw mut invalid_field as gconstpointer, 1 as guint);
    return g_array_free(array, FALSE) as *mut guchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_body(
    mut message: *mut GDBusMessage,
) -> *mut GVariant {
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GVariant>();
    }
    return (*message).body;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_body(
    mut message: *mut GDBusMessage,
    mut body: *mut GVariant,
) {
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if body.is_null()
            || g_variant_is_of_type(
                body,
                b"r\0" as *const u8 as *const ::core::ffi::c_char as *const GVariantType,
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
            b"(body == NULL) || g_variant_is_of_type (body, G_VARIANT_TYPE_TUPLE)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*message).locked != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: Attempted to modify a locked message\0" as *const u8 as *const gchar,
            b"g_dbus_message_set_body\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*message).body.is_null() {
        g_variant_unref((*message).body);
    }
    if body.is_null() {
        (*message).body = ::core::ptr::null_mut::<GVariant>();
        (*message).arg0_cache = ::core::ptr::null_mut::<GVariant>();
        safe_c2rust_g_dbus_message_set_signature(message, ::core::ptr::null::<gchar>());
    } else {
        let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
        let mut type_string_len: gsize = 0;
        let mut signature: *mut gchar = ::core::ptr::null_mut::<gchar>();
        (*message).body = g_variant_ref_sink(body);
        if g_variant_is_of_type((*message).body, G_VARIANT_TYPE_TUPLE) != 0
            && g_variant_n_children((*message).body) > 0 as gsize
        {
            (*message).arg0_cache = g_variant_get_child_value((*message).body, 0 as gsize);
        } else {
            (*message).arg0_cache = ::core::ptr::null_mut::<GVariant>();
        }
        type_string = g_variant_get_type_string(body);
        type_string_len = strlen(type_string as *const ::core::ffi::c_char) as gsize;
        if ({
            let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
            if type_string_len >= 2 as gsize {
                _g_boolean_var_53 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_53 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_53
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1180 as ::core::ffi::c_int,
                G_STRFUNC,
                b"type_string_len >= 2\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        signature = g_strndup(
            type_string.offset(1 as ::core::ffi::c_int as isize),
            type_string_len.wrapping_sub(2 as gsize),
        );
        safe_c2rust_g_dbus_message_set_signature(message, signature);
        g_free(signature as gpointer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_unix_fd_list(
    mut message: *mut GDBusMessage,
) -> *mut GUnixFDList {
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GUnixFDList>();
    }
    return (*message).fd_list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_unix_fd_list(
    mut message: *mut GDBusMessage,
    mut fd_list: *mut GUnixFDList,
) {
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if fd_list.is_null()
            || ({
                let mut __inst: *mut GTypeInstance = fd_list as *mut GTypeInstance;
                let mut __t: GType = g_unix_fd_list_get_type();
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
            b"fd_list == NULL || G_IS_UNIX_FD_LIST (fd_list)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*message).locked != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"%s: Attempted to modify a locked message\0" as *const u8 as *const gchar,
            b"g_dbus_message_set_unix_fd_list\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*message).fd_list.is_null() {
        g_object_unref((*message).fd_list as gpointer);
    }
    if !fd_list.is_null() {
        (*message).fd_list =
            g_object_ref(fd_list as gpointer) as *mut GUnixFDList as *mut GUnixFDList;
        safe_c2rust_g_dbus_message_set_num_unix_fds(
            message,
            g_unix_fd_list_get_length(fd_list) as guint32,
        );
    } else {
        (*message).fd_list = ::core::ptr::null_mut::<GUnixFDList>();
        safe_c2rust_g_dbus_message_set_num_unix_fds(message, 0 as guint32);
    };
}
unsafe extern "C" fn safe_c2rust_get_type_fixed_size(mut type_0: *const GVariantType) -> guint {
    match *g_variant_type_peek_string(type_0) as ::core::ffi::c_int {
        121 => return 1 as guint,
        110 | 113 => return 2 as guint,
        105 | 117 | 104 => return 4 as guint,
        120 | 116 | 100 => return 8 as guint,
        _ => return 0 as guint,
    };
}
unsafe extern "C" fn safe_c2rust_message_type_to_string(
    mut message_type: GDBusMessageType,
) -> *const ::core::ffi::c_char {
    match message_type as ::core::ffi::c_uint {
        0 => return b"INVALID\0" as *const u8 as *const ::core::ffi::c_char,
        1 => return b"METHOD_CALL\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"METHOD_RETURN\0" as *const u8 as *const ::core::ffi::c_char,
        3 => return b"ERROR\0" as *const u8 as *const ::core::ffi::c_char,
        4 => return b"SIGNAL\0" as *const u8 as *const ::core::ffi::c_char,
        _ => return b"unknown-type\0" as *const u8 as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn safe_c2rust_message_header_field_to_string(
    mut field: GDBusMessageHeaderField,
) -> *const ::core::ffi::c_char {
    match field as ::core::ffi::c_uint {
        0 => return b"INVALID\0" as *const u8 as *const ::core::ffi::c_char,
        1 => return b"PATH\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"INTERFACE\0" as *const u8 as *const ::core::ffi::c_char,
        3 => return b"MEMBER\0" as *const u8 as *const ::core::ffi::c_char,
        4 => return b"ERROR_NAME\0" as *const u8 as *const ::core::ffi::c_char,
        5 => return b"REPLY_SERIAL\0" as *const u8 as *const ::core::ffi::c_char,
        6 => return b"DESTINATION\0" as *const u8 as *const ::core::ffi::c_char,
        7 => return b"SENDER\0" as *const u8 as *const ::core::ffi::c_char,
        8 => return b"SIGNATURE\0" as *const u8 as *const ::core::ffi::c_char,
        9 => return b"NUM_UNIX_FDS\0" as *const u8 as *const ::core::ffi::c_char,
        _ => return b"unknown-field\0" as *const u8 as *const ::core::ffi::c_char,
    };
}
unsafe extern "C" fn safe_c2rust_validate_header(
    mut message: *mut GDBusMessage,
    mut field: GDBusMessageHeaderField,
    mut header_value: *mut GVariant,
    mut expected_type: *const GVariantType,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !header_value.is_null() {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1343 as ::core::ffi::c_int,
            G_STRFUNC,
            b"header_value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if g_variant_is_of_type(header_value, expected_type) == 0 {
        let mut expected_type_string: *mut ::core::ffi::c_char =
            g_variant_type_dup_string(expected_type) as *mut ::core::ffi::c_char;
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"%s message: %s header field is invalid; expected a value of type \xE2\x80\x98%s\xE2\x80\x99\0"
                    as *const u8 as *const gchar,
            ),
            safe_c2rust_message_type_to_string((*message).type_0),
            safe_c2rust_message_header_field_to_string(field),
            expected_type_string,
        );
        g_free(expected_type_string as gpointer);
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_require_header(
    mut message: *mut GDBusMessage,
    mut field: GDBusMessageHeaderField,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut header_value: *mut GVariant = safe_c2rust_g_dbus_message_get_header(message, field);
    if header_value.is_null() {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"%s message: %s header field is missing or invalid\0" as *const u8 as *const gchar,
            ),
            safe_c2rust_message_type_to_string((*message).type_0),
            safe_c2rust_message_header_field_to_string(field),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_validate_headers(
    mut message: *mut GDBusMessage,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut ret: gboolean = 0;
    let mut headers_iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut header_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    g_hash_table_iter_init(&raw mut headers_iter, (*message).headers);
    loop {
        if !(g_hash_table_iter_next(
            &raw mut headers_iter,
            &raw mut key,
            &raw mut header_value as gpointer as *mut gpointer,
        ) != 0)
        {
            current_block = 9441801433784995173;
            break;
        }
        let mut field_type: GDBusMessageHeaderField =
            key as glong as gint as GDBusMessageHeaderField;
        match field_type as ::core::ffi::c_uint {
            0 => {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"%s message: INVALID header field supplied\0" as *const u8 as *const gchar,
                    ),
                    safe_c2rust_message_type_to_string((*message).type_0),
                );
                current_block = 575382179471392814;
                break;
            }
            1 => {
                if safe_c2rust_validate_header(
                    message,
                    field_type,
                    header_value,
                    G_VARIANT_TYPE_OBJECT_PATH,
                    error,
                ) == 0
                {
                    current_block = 575382179471392814;
                    break;
                }
                if !(g_strcmp0(
                    g_variant_get_string(header_value, ::core::ptr::null_mut::<gsize>())
                        as *const ::core::ffi::c_char,
                    b"/org/freedesktop/DBus/Local\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
                {
                    continue;
                }
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"%s message: PATH header field is using the reserved value /org/freedesktop/DBus/Local\0"
                            as *const u8 as *const gchar,
                    ),
                    safe_c2rust_message_type_to_string((*message).type_0),
                );
                current_block = 575382179471392814;
                break;
            }
            2 => {
                if safe_c2rust_validate_header(
                    message,
                    field_type,
                    header_value,
                    G_VARIANT_TYPE_STRING,
                    error,
                ) == 0
                {
                    current_block = 575382179471392814;
                    break;
                }
                if g_dbus_is_interface_name(g_variant_get_string(
                    header_value,
                    ::core::ptr::null_mut::<gsize>(),
                )) == 0
                {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"%s message: INTERFACE header field does not contain a valid interface name\0"
                                as *const u8 as *const gchar,
                        ),
                        safe_c2rust_message_type_to_string((*message).type_0),
                    );
                    current_block = 575382179471392814;
                    break;
                } else {
                    if !(g_strcmp0(
                        g_variant_get_string(header_value, ::core::ptr::null_mut::<gsize>())
                            as *const ::core::ffi::c_char,
                        b"org.freedesktop.DBus.Local\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int)
                    {
                        continue;
                    }
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"%s message: INTERFACE header field is using the reserved value org.freedesktop.DBus.Local\0"
                                as *const u8 as *const gchar,
                        ),
                        safe_c2rust_message_type_to_string((*message).type_0),
                    );
                    current_block = 575382179471392814;
                    break;
                }
            }
            3 => {
                if safe_c2rust_validate_header(
                    message,
                    field_type,
                    header_value,
                    G_VARIANT_TYPE_STRING,
                    error,
                ) == 0
                {
                    current_block = 575382179471392814;
                    break;
                }
                if !(g_dbus_is_member_name(g_variant_get_string(
                    header_value,
                    ::core::ptr::null_mut::<gsize>(),
                )) == 0)
                {
                    continue;
                }
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"%s message: MEMBER header field does not contain a valid member name\0"
                            as *const u8 as *const gchar,
                    ),
                    safe_c2rust_message_type_to_string((*message).type_0),
                );
                current_block = 575382179471392814;
                break;
            }
            4 => {
                if safe_c2rust_validate_header(
                    message,
                    field_type,
                    header_value,
                    G_VARIANT_TYPE_STRING,
                    error,
                ) == 0
                {
                    current_block = 575382179471392814;
                    break;
                }
                if !(g_dbus_is_error_name(g_variant_get_string(
                    header_value,
                    ::core::ptr::null_mut::<gsize>(),
                )) == 0)
                {
                    continue;
                }
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"%s message: ERROR_NAME header field does not contain a valid error name\0"
                            as *const u8 as *const gchar,
                    ),
                    safe_c2rust_message_type_to_string((*message).type_0),
                );
                current_block = 575382179471392814;
                break;
            }
            5 => {
                if safe_c2rust_validate_header(
                    message,
                    field_type,
                    header_value,
                    G_VARIANT_TYPE_UINT32,
                    error,
                ) == 0
                {
                    current_block = 575382179471392814;
                    break;
                }
            }
            6 => {
                if safe_c2rust_validate_header(
                    message,
                    field_type,
                    header_value,
                    G_VARIANT_TYPE_STRING,
                    error,
                ) == 0
                {
                    current_block = 575382179471392814;
                    break;
                }
            }
            7 => {
                if safe_c2rust_validate_header(
                    message,
                    field_type,
                    header_value,
                    G_VARIANT_TYPE_STRING,
                    error,
                ) == 0
                {
                    current_block = 575382179471392814;
                    break;
                }
            }
            8 => {
                if safe_c2rust_validate_header(
                    message,
                    field_type,
                    header_value,
                    G_VARIANT_TYPE_SIGNATURE,
                    error,
                ) == 0
                {
                    current_block = 575382179471392814;
                    break;
                }
            }
            9 => {
                if safe_c2rust_validate_header(
                    message,
                    field_type,
                    header_value,
                    G_VARIANT_TYPE_UINT32,
                    error,
                ) == 0
                {
                    current_block = 575382179471392814;
                    break;
                }
            }
            _ => {}
        }
    }
    match current_block {
        9441801433784995173 => {
            match (*message).type_0 as ::core::ffi::c_uint {
                0 => {
                    g_set_error_literal(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(b"type is INVALID\0" as *const u8 as *const gchar),
                    );
                    current_block = 575382179471392814;
                }
                1 => {
                    if safe_c2rust_require_header(message, G_DBUS_MESSAGE_HEADER_FIELD_PATH, error)
                        == 0
                        || safe_c2rust_require_header(
                            message,
                            G_DBUS_MESSAGE_HEADER_FIELD_MEMBER,
                            error,
                        ) == 0
                    {
                        current_block = 575382179471392814;
                    } else {
                        current_block = 17233182392562552756;
                    }
                }
                2 => {
                    if safe_c2rust_require_header(
                        message,
                        G_DBUS_MESSAGE_HEADER_FIELD_REPLY_SERIAL,
                        error,
                    ) == 0
                    {
                        current_block = 575382179471392814;
                    } else {
                        current_block = 17233182392562552756;
                    }
                }
                3 => {
                    if safe_c2rust_require_header(
                        message,
                        G_DBUS_MESSAGE_HEADER_FIELD_ERROR_NAME,
                        error,
                    ) == 0
                        || safe_c2rust_require_header(
                            message,
                            G_DBUS_MESSAGE_HEADER_FIELD_REPLY_SERIAL,
                            error,
                        ) == 0
                    {
                        current_block = 575382179471392814;
                    } else {
                        current_block = 17233182392562552756;
                    }
                }
                4 => {
                    if safe_c2rust_require_header(message, G_DBUS_MESSAGE_HEADER_FIELD_PATH, error)
                        == 0
                        || safe_c2rust_require_header(
                            message,
                            G_DBUS_MESSAGE_HEADER_FIELD_INTERFACE,
                            error,
                        ) == 0
                        || safe_c2rust_require_header(
                            message,
                            G_DBUS_MESSAGE_HEADER_FIELD_MEMBER,
                            error,
                        ) == 0
                    {
                        current_block = 575382179471392814;
                    } else {
                        current_block = 17233182392562552756;
                    }
                }
                _ => {
                    current_block = 17233182392562552756;
                }
            }
            match current_block {
                575382179471392814 => {}
                _ => {
                    ret = TRUE as gboolean;
                }
            }
        }
        _ => {}
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if ret != 0 || (error.is_null() || !(*error).is_null()) {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1546 as ::core::ffi::c_int,
            G_STRFUNC,
            b"ret || (error == NULL || *error != NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_ensure_input_padding(
    mut buf: *mut GMemoryBuffer,
    mut padding_size: gsize,
) -> gboolean {
    let mut offset: gsize = 0;
    let mut wanted_offset: gsize = 0;
    offset = (*buf).pos;
    wanted_offset = offset
        .wrapping_add(padding_size)
        .wrapping_sub(1 as gsize)
        .wrapping_div(padding_size)
        .wrapping_mul(padding_size);
    (*buf).pos = wanted_offset;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_read_string(
    mut mbuf: *mut GMemoryBuffer,
    mut len: gsize,
    mut error: *mut *mut GError,
) -> *const gchar {
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut end_valid: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if (*mbuf).pos.wrapping_add(len) >= (*mbuf).valid_len
            || (*mbuf).pos.wrapping_add(len) < (*mbuf).pos
        {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
        (*mbuf).pos = (*mbuf).valid_len;
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            g_dngettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"Wanted to read %lu byte but only got %lu\0" as *const u8 as *const gchar,
                b"Wanted to read %lu bytes but only got %lu\0" as *const u8 as *const gchar,
                len,
            ),
            len,
            (*mbuf).valid_len.wrapping_sub((*mbuf).pos),
        );
        return ::core::ptr::null::<gchar>();
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if *(*mbuf).data.offset((*mbuf).pos.wrapping_add(len) as isize) as ::core::ffi::c_int
            != '\0' as i32
        {
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
        str = g_strndup((*mbuf).data.offset((*mbuf).pos as isize), len);
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Expected NUL byte after the string \xE2\x80\x9C%s\xE2\x80\x9D but found byte %d\0"
                    as *const u8 as *const gchar,
            ),
            str,
            *(*mbuf).data.offset((*mbuf).pos.wrapping_add(len) as isize) as ::core::ffi::c_int,
        );
        g_free(str as gpointer);
        (*mbuf).pos = (*mbuf).pos.wrapping_add(len.wrapping_add(1 as gsize));
        return ::core::ptr::null::<gchar>();
    }
    str = (*mbuf).data.offset((*mbuf).pos as isize);
    (*mbuf).pos = (*mbuf).pos.wrapping_add(len.wrapping_add(1 as gsize));
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if g_utf8_validate(
            str,
            -(1 as ::core::ffi::c_int) as gssize,
            &raw mut end_valid,
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
        let mut offset: gint = 0;
        let mut valid_str: *mut gchar = ::core::ptr::null_mut::<gchar>();
        offset = end_valid.offset_from(str) as ::core::ffi::c_long as gint;
        valid_str = g_strndup(str, offset as gsize);
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Expected valid UTF-8 string but found invalid bytes at byte offset %d (length of string is %d). The valid UTF-8 string up until that point was \xE2\x80\x9C%s\xE2\x80\x9D\0"
                    as *const u8 as *const gchar,
            ),
            offset,
            len as gint,
            valid_str,
        );
        g_free(valid_str as gpointer);
        return ::core::ptr::null::<gchar>();
    }
    return str;
}
unsafe extern "C" fn safe_c2rust_read_bytes(
    mut mbuf: *mut GMemoryBuffer,
    mut len: gsize,
    mut error: *mut *mut GError,
) -> gconstpointer {
    let mut result: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if (*mbuf).pos.wrapping_add(len) > (*mbuf).valid_len
            || (*mbuf).pos.wrapping_add(len) < (*mbuf).pos
        {
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
        (*mbuf).pos = (*mbuf).valid_len;
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            g_dngettext(
                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                b"Wanted to read %lu byte but only got %lu\0" as *const u8 as *const gchar,
                b"Wanted to read %lu bytes but only got %lu\0" as *const u8 as *const gchar,
                len,
            ),
            len,
            (*mbuf).valid_len.wrapping_sub((*mbuf).pos),
        );
        return ::core::ptr::null::<::core::ffi::c_void>();
    }
    result = (*mbuf).data.offset((*mbuf).pos as isize) as gconstpointer;
    (*mbuf).pos = (*mbuf).pos.wrapping_add(len);
    return result;
}
unsafe extern "C" fn safe_c2rust_parse_value_from_blob(
    mut buf: *mut GMemoryBuffer,
    mut type_0: *const GVariantType,
    mut max_depth: guint,
    mut just_align: gboolean,
    mut indent: guint,
    mut error: *mut *mut GError,
) -> *mut GVariant {
    let mut current_block: u64;
    let mut ret: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    if max_depth == 0 as guint {
        g_set_error_literal(
            &raw mut local_error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Value nested too deeply\0" as *const u8 as *const gchar),
        );
    } else {
        type_string = g_variant_type_peek_string(type_0);
        match *type_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
            98 => {
                safe_c2rust_ensure_input_padding(buf, 4 as gsize);
                if just_align == 0 {
                    let mut v: gboolean = 0;
                    v = safe_c2rust_g_memory_buffer_read_uint32(buf, &raw mut local_error)
                        as gboolean;
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        ret = g_variant_new_boolean(v);
                        current_block = 242220637564940144;
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            121 => {
                if just_align == 0 {
                    let mut v_0: guchar = 0;
                    v_0 = safe_c2rust_g_memory_buffer_read_byte(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        ret = g_variant_new_byte(v_0 as guint8);
                        current_block = 242220637564940144;
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            110 => {
                safe_c2rust_ensure_input_padding(buf, 2 as gsize);
                if just_align == 0 {
                    let mut v_1: gint16 = 0;
                    v_1 = safe_c2rust_g_memory_buffer_read_int16(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        ret = g_variant_new_int16(v_1);
                        current_block = 242220637564940144;
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            113 => {
                safe_c2rust_ensure_input_padding(buf, 2 as gsize);
                if just_align == 0 {
                    let mut v_2: guint16 = 0;
                    v_2 = safe_c2rust_g_memory_buffer_read_uint16(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        ret = g_variant_new_uint16(v_2);
                        current_block = 242220637564940144;
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            105 => {
                safe_c2rust_ensure_input_padding(buf, 4 as gsize);
                if just_align == 0 {
                    let mut v_3: gint32 = 0;
                    v_3 = safe_c2rust_g_memory_buffer_read_int32(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        ret = g_variant_new_int32(v_3);
                        current_block = 242220637564940144;
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            117 => {
                safe_c2rust_ensure_input_padding(buf, 4 as gsize);
                if just_align == 0 {
                    let mut v_4: guint32 = 0;
                    v_4 = safe_c2rust_g_memory_buffer_read_uint32(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        ret = g_variant_new_uint32(v_4);
                        current_block = 242220637564940144;
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            120 => {
                safe_c2rust_ensure_input_padding(buf, 8 as gsize);
                if just_align == 0 {
                    let mut v_5: gint64 = 0;
                    v_5 = safe_c2rust_g_memory_buffer_read_int64(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        ret = g_variant_new_int64(v_5);
                        current_block = 242220637564940144;
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            116 => {
                safe_c2rust_ensure_input_padding(buf, 8 as gsize);
                if just_align == 0 {
                    let mut v_6: guint64 = 0;
                    v_6 = safe_c2rust_g_memory_buffer_read_uint64(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        ret = g_variant_new_uint64(v_6);
                        current_block = 242220637564940144;
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            100 => {
                safe_c2rust_ensure_input_padding(buf, 8 as gsize);
                if just_align == 0 {
                    let mut u: C2RustUnnamed_3 = C2RustUnnamed_3 { v_uint64: 0 };
                    u.v_uint64 = safe_c2rust_g_memory_buffer_read_uint64(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        ret = g_variant_new_double(u.v_double);
                        current_block = 242220637564940144;
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            115 => {
                safe_c2rust_ensure_input_padding(buf, 4 as gsize);
                if just_align == 0 {
                    let mut len: guint32 = 0;
                    let mut v_7: *const gchar = ::core::ptr::null::<gchar>();
                    len = safe_c2rust_g_memory_buffer_read_uint32(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        v_7 = safe_c2rust_read_string(buf, len as gsize, &raw mut local_error);
                        if v_7.is_null() {
                            current_block = 18386514359787761805;
                        } else {
                            ret = g_variant_new_string(v_7);
                            current_block = 242220637564940144;
                        }
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            111 => {
                safe_c2rust_ensure_input_padding(buf, 4 as gsize);
                if just_align == 0 {
                    let mut len_0: guint32 = 0;
                    let mut v_8: *const gchar = ::core::ptr::null::<gchar>();
                    len_0 = safe_c2rust_g_memory_buffer_read_uint32(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        v_8 = safe_c2rust_read_string(buf, len_0 as gsize, &raw mut local_error);
                        if v_8.is_null() {
                            current_block = 18386514359787761805;
                        } else if g_variant_is_object_path(v_8) == 0 {
                            g_set_error(
                                &raw mut local_error,
                                g_io_error_quark(),
                                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"Parsed value \xE2\x80\x9C%s\xE2\x80\x9D is not a valid D-Bus object path\0"
                                        as *const u8 as *const gchar,
                                ),
                                v_8,
                            );
                            current_block = 18386514359787761805;
                        } else {
                            ret = g_variant_new_object_path(v_8);
                            current_block = 242220637564940144;
                        }
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            103 => {
                if just_align == 0 {
                    let mut len_1: guchar = 0;
                    let mut v_9: *const gchar = ::core::ptr::null::<gchar>();
                    len_1 = safe_c2rust_g_memory_buffer_read_byte(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        v_9 = safe_c2rust_read_string(buf, len_1 as gsize, &raw mut local_error);
                        if v_9.is_null() {
                            current_block = 18386514359787761805;
                        } else if g_variant_is_signature(v_9) == 0 {
                            g_set_error(
                                &raw mut local_error,
                                g_io_error_quark(),
                                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"Parsed value \xE2\x80\x9C%s\xE2\x80\x9D is not a valid D-Bus signature\0"
                                        as *const u8 as *const gchar,
                                ),
                                v_9,
                            );
                            current_block = 18386514359787761805;
                        } else {
                            ret = g_variant_new_signature(v_9);
                            current_block = 242220637564940144;
                        }
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            104 => {
                safe_c2rust_ensure_input_padding(buf, 4 as gsize);
                if just_align == 0 {
                    let mut v_10: gint32 = 0;
                    v_10 = safe_c2rust_g_memory_buffer_read_int32(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else {
                        ret = g_variant_new_handle(v_10);
                        current_block = 242220637564940144;
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            97 => {
                safe_c2rust_ensure_input_padding(buf, 4 as gsize);
                if just_align == 0 {
                    let mut array_len: guint32 = 0;
                    let mut element_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
                    let mut fixed_size: guint = 0;
                    array_len = safe_c2rust_g_memory_buffer_read_uint32(buf, &raw mut local_error);
                    if !local_error.is_null() {
                        current_block = 18386514359787761805;
                    } else if array_len
                        > ((2 as ::core::ffi::c_int) << 26 as ::core::ffi::c_int) as guint32
                    {
                        g_set_error(
                            &raw mut local_error,
                            g_io_error_quark(),
                            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                            g_dngettext(
                                GETTEXT_PACKAGE.as_ptr() as *const gchar,
                                b"Encountered array of length %u byte. Maximum length is 2<<26 bytes (64 MiB).\0"
                                    as *const u8 as *const gchar,
                                b"Encountered array of length %u bytes. Maximum length is 2<<26 bytes (64 MiB).\0"
                                    as *const u8 as *const gchar,
                                array_len as gulong,
                            ),
                            array_len,
                        );
                        current_block = 18386514359787761805;
                    } else {
                        element_type = g_variant_type_element(type_0);
                        fixed_size = safe_c2rust_get_type_fixed_size(element_type);
                        if fixed_size != 0 as guint {
                            let mut array_data: gconstpointer =
                                ::core::ptr::null::<::core::ffi::c_void>();
                            if (array_len as guint).wrapping_rem(fixed_size)
                                != 0 as ::core::ffi::c_uint
                            {
                                g_set_error(
                                    &raw mut local_error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"Encountered array of type \xE2\x80\x9Ca%c\xE2\x80\x9D, expected to have a length a multiple of %u bytes, but found to be %u bytes in length\0"
                                            as *const u8 as *const gchar,
                                    ),
                                    *g_variant_type_peek_string(element_type)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int,
                                    fixed_size,
                                    array_len,
                                );
                                current_block = 18386514359787761805;
                            } else if max_depth == 1 as guint {
                                g_set_error_literal(
                                    &raw mut local_error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"Value nested too deeply\0" as *const u8 as *const gchar,
                                    ),
                                );
                                current_block = 18386514359787761805;
                            } else {
                                safe_c2rust_ensure_input_padding(buf, fixed_size as gsize);
                                array_data = safe_c2rust_read_bytes(
                                    buf,
                                    array_len as gsize,
                                    &raw mut local_error,
                                );
                                if array_data.is_null() {
                                    current_block = 18386514359787761805;
                                } else {
                                    ret = g_variant_new_fixed_array(
                                        element_type,
                                        array_data,
                                        (array_len as guint).wrapping_div(fixed_size) as gsize,
                                        fixed_size as gsize,
                                    );
                                    if safe_c2rust_g_memory_buffer_is_byteswapped(buf) != 0 {
                                        let mut tmp: *mut GVariant = g_variant_ref_sink(ret);
                                        ret = g_variant_byteswap(tmp);
                                        g_variant_unref(tmp);
                                    }
                                    current_block = 242220637564940144;
                                }
                            }
                        } else {
                            let mut builder: GVariantBuilder = _GVariantBuilder {
                                u: C2RustUnnamed {
                                    s: C2RustUnnamed_0 {
                                        partial_magic: 0,
                                        type_0: ::core::ptr::null::<GVariantType>(),
                                        y: [0; 14],
                                    },
                                },
                            };
                            let mut offset: goffset = 0;
                            let mut target: goffset = 0;
                            g_variant_builder_init(&raw mut builder, type_0);
                            if array_len == 0 as guint32 {
                                let mut item: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                                item = safe_c2rust_parse_value_from_blob(
                                    buf,
                                    element_type,
                                    max_depth.wrapping_sub(1 as guint),
                                    TRUE,
                                    indent.wrapping_add(2 as guint),
                                    ::core::ptr::null_mut::<*mut GError>(),
                                );
                                if ({
                                    let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
                                    if item.is_null() {
                                        _g_boolean_var_65 = 1 as ::core::ffi::c_int;
                                    } else {
                                        _g_boolean_var_65 = 0 as ::core::ffi::c_int;
                                    }
                                    _g_boolean_var_65
                                }) as ::core::ffi::c_long
                                    != 0
                                {
                                } else {
                                    g_assertion_message_expr(
                                        G_LOG_DOMAIN.as_ptr(),
                                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        1989 as ::core::ffi::c_int,
                                        G_STRFUNC,
                                        b"item == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                                    );
                                }
                                current_block = 7728257318064351663;
                            } else {
                                offset = (*buf).pos as goffset;
                                target = offset + array_len as goffset;
                                loop {
                                    if !(offset < target) {
                                        current_block = 7728257318064351663;
                                        break;
                                    }
                                    let mut item_0: *mut GVariant =
                                        ::core::ptr::null_mut::<GVariant>();
                                    item_0 = safe_c2rust_parse_value_from_blob(
                                        buf,
                                        element_type,
                                        max_depth.wrapping_sub(1 as guint),
                                        FALSE,
                                        indent.wrapping_add(2 as guint),
                                        &raw mut local_error,
                                    );
                                    if item_0.is_null() {
                                        g_variant_builder_clear(&raw mut builder);
                                        current_block = 18386514359787761805;
                                        break;
                                    } else {
                                        g_variant_builder_add_value(&raw mut builder, item_0);
                                        g_variant_unref(item_0);
                                        if ({
                                            let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
                                            if (*buf).pos > offset as gsize {
                                                _g_boolean_var_66 = 1 as ::core::ffi::c_int;
                                            } else {
                                                _g_boolean_var_66 = 0 as ::core::ffi::c_int;
                                            }
                                            _g_boolean_var_66
                                        })
                                            as ::core::ffi::c_long
                                            != 0
                                        {
                                        } else {
                                            g_assertion_message_expr(
                                                G_LOG_DOMAIN.as_ptr(),
                                                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                                                    as *const u8 as *const ::core::ffi::c_char,
                                                2019 as ::core::ffi::c_int,
                                                G_STRFUNC,
                                                b"buf->pos > (gsize) offset\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        }
                                        offset = (*buf).pos as goffset;
                                    }
                                }
                            }
                            match current_block {
                                18386514359787761805 => {}
                                _ => {
                                    ret = g_variant_builder_end(&raw mut builder);
                                    current_block = 242220637564940144;
                                }
                            }
                        }
                    }
                } else {
                    current_block = 242220637564940144;
                }
            }
            _ => {
                if g_variant_type_is_dict_entry(type_0) != 0 {
                    let mut key_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
                    let mut value_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
                    let mut key: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                    safe_c2rust_ensure_input_padding(buf, 8 as gsize);
                    if just_align == 0 {
                        key_type = g_variant_type_key(type_0);
                        key = safe_c2rust_parse_value_from_blob(
                            buf,
                            key_type,
                            max_depth.wrapping_sub(1 as guint),
                            FALSE,
                            indent.wrapping_add(2 as guint),
                            &raw mut local_error,
                        );
                        if key.is_null() {
                            current_block = 18386514359787761805;
                        } else {
                            value_type = g_variant_type_value(type_0);
                            value = safe_c2rust_parse_value_from_blob(
                                buf,
                                value_type,
                                max_depth.wrapping_sub(1 as guint),
                                FALSE,
                                indent.wrapping_add(2 as guint),
                                &raw mut local_error,
                            );
                            if value.is_null() {
                                g_variant_unref(key);
                                current_block = 18386514359787761805;
                            } else {
                                ret = g_variant_new_dict_entry(key, value);
                                g_variant_unref(key);
                                g_variant_unref(value);
                                current_block = 242220637564940144;
                            }
                        }
                    } else {
                        current_block = 242220637564940144;
                    }
                } else if g_variant_type_is_tuple(type_0) != 0 {
                    safe_c2rust_ensure_input_padding(buf, 8 as gsize);
                    if just_align == 0 {
                        let mut element_type_0: *const GVariantType =
                            ::core::ptr::null::<GVariantType>();
                        let mut builder_0: GVariantBuilder = _GVariantBuilder {
                            u: C2RustUnnamed {
                                s: C2RustUnnamed_0 {
                                    partial_magic: 0,
                                    type_0: ::core::ptr::null::<GVariantType>(),
                                    y: [0; 14],
                                },
                            },
                        };
                        g_variant_builder_init(&raw mut builder_0, type_0);
                        element_type_0 = g_variant_type_first(type_0);
                        if element_type_0.is_null() {
                            g_variant_builder_clear(&raw mut builder_0);
                            g_set_error_literal(
                                &raw mut local_error,
                                g_io_error_quark(),
                                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"Empty structures (tuples) are not allowed in D-Bus\0"
                                        as *const u8
                                        as *const gchar,
                                ),
                            );
                            current_block = 18386514359787761805;
                        } else {
                            loop {
                                if element_type_0.is_null() {
                                    current_block = 8880031775101799352;
                                    break;
                                }
                                let mut item_1: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                                item_1 = safe_c2rust_parse_value_from_blob(
                                    buf,
                                    element_type_0,
                                    max_depth.wrapping_sub(1 as guint),
                                    FALSE,
                                    indent.wrapping_add(2 as guint),
                                    &raw mut local_error,
                                );
                                if item_1.is_null() {
                                    g_variant_builder_clear(&raw mut builder_0);
                                    current_block = 18386514359787761805;
                                    break;
                                } else {
                                    g_variant_builder_add_value(&raw mut builder_0, item_1);
                                    g_variant_unref(item_1);
                                    element_type_0 = g_variant_type_next(element_type_0);
                                }
                            }
                            match current_block {
                                18386514359787761805 => {}
                                _ => {
                                    ret = g_variant_builder_end(&raw mut builder_0);
                                    current_block = 242220637564940144;
                                }
                            }
                        }
                    } else {
                        current_block = 242220637564940144;
                    }
                } else if g_variant_type_is_variant(type_0) != 0 {
                    if just_align == 0 {
                        let mut siglen: guchar = 0;
                        let mut sig: *const gchar = ::core::ptr::null::<gchar>();
                        let mut variant_type: *mut GVariantType =
                            ::core::ptr::null_mut::<GVariantType>();
                        let mut value_0: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                        siglen = safe_c2rust_g_memory_buffer_read_byte(buf, &raw mut local_error);
                        if !local_error.is_null() {
                            current_block = 18386514359787761805;
                        } else {
                            sig =
                                safe_c2rust_read_string(buf, siglen as gsize, &raw mut local_error);
                            if sig.is_null() {
                                current_block = 18386514359787761805;
                            } else if g_variant_is_signature(sig) == 0
                                || g_variant_type_string_is_valid(sig) == 0
                            {
                                g_set_error(
                                    &raw mut local_error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"Parsed value \xE2\x80\x9C%s\xE2\x80\x9D for variant is not a valid D-Bus signature\0"
                                            as *const u8 as *const gchar,
                                    ),
                                    sig,
                                );
                                current_block = 18386514359787761805;
                            } else if max_depth as gsize <= g_variant_type_string_get_depth_(sig) {
                                g_set_error_literal(
                                    &raw mut local_error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"Value nested too deeply\0" as *const u8 as *const gchar,
                                    ),
                                );
                                current_block = 18386514359787761805;
                            } else {
                                variant_type = g_variant_type_new(sig);
                                value_0 = safe_c2rust_parse_value_from_blob(
                                    buf,
                                    variant_type,
                                    max_depth.wrapping_sub(1 as guint),
                                    FALSE,
                                    indent.wrapping_add(2 as guint),
                                    &raw mut local_error,
                                );
                                g_variant_type_free(variant_type);
                                if value_0.is_null() {
                                    current_block = 18386514359787761805;
                                } else {
                                    ret = g_variant_new_variant(value_0);
                                    g_variant_unref(value_0);
                                    current_block = 242220637564940144;
                                }
                            }
                        }
                    } else {
                        current_block = 242220637564940144;
                    }
                } else {
                    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
                    s = g_variant_type_dup_string(type_0);
                    g_set_error(
                        &raw mut local_error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Error deserializing GVariant with type string \xE2\x80\x9C%s\xE2\x80\x9D from the D-Bus wire format\0"
                                as *const u8 as *const gchar,
                        ),
                        s,
                    );
                    g_free(s as gpointer);
                    current_block = 18386514359787761805;
                }
            }
        }
        match current_block {
            18386514359787761805 => {}
            _ => {
                if ({
                    let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
                    if just_align != 0 && ret.is_null() || just_align == 0 && !ret.is_null() {
                        _g_boolean_var_67 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_67 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_67
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        2198 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"(just_align && ret == NULL) || (!just_align && ret != NULL)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                if !ret.is_null() {
                    g_variant_take_ref(ret);
                }
                return ret;
            }
        }
    }
    g_propagate_error(error, local_error);
    return ::core::ptr::null_mut::<GVariant>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_bytes_needed(
    mut blob: *mut guchar,
    mut blob_len: gsize,
    mut error: *mut *mut GError,
) -> gssize {
    let mut ret: gssize = 0;
    ret = -(1 as ::core::ffi::c_int) as gssize;
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if !blob.is_null() {
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"blob != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if blob_len >= 16 as gsize {
            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_70
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"blob_len >= 16\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gssize;
    }
    if *blob.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'l' as i32 {
        ret = ((12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as guint32)
            .wrapping_add(*(blob as *mut guint32).offset(3 as ::core::ffi::c_int as isize))
            as gssize;
        ret = 8 as gssize * ((ret + 7 as gssize) / 8 as gssize);
        ret += *(blob as *mut guint32).offset(1 as ::core::ffi::c_int as isize) as gssize;
    } else if *blob.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'B' as i32 {
        ret = ((12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as guint32).wrapping_add(
            ({
                let mut __v: guint32 = 0;
                let mut __x: guint32 =
                    *(blob as *mut guint32).offset(3 as ::core::ffi::c_int as isize);
                if 0 != 0 {
                    __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                        | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                        | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                        | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
                } else {
                    let fresh14 = &mut __v;
                    let fresh15;
                    let fresh16 = __x;
                    asm!(
                        "bswapl {0:e}\n", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh16) =>
                        fresh15, options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
                }
                __v
            }),
        ) as gssize;
        ret = 8 as gssize * ((ret + 7 as gssize) / 8 as gssize);
        ret += ({
            let mut __v: guint32 = 0;
            let mut __x: guint32 = *(blob as *mut guint32).offset(1 as ::core::ffi::c_int as isize);
            if 0 != 0 {
                __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                    | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                    | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                    | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
            } else {
                let fresh17 = &mut __v;
                let fresh18;
                let fresh19 = __x;
                asm!(
                    "bswapl {0:e}\n", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh17, fresh19) => fresh18,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh17, fresh19, fresh18);
            }
            __v
        }) as gssize;
    } else {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Unable to determine message blob length - given blob is malformed\0" as *const u8
                as *const gchar,
        );
    }
    if ret > ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as gssize {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Blob indicates that message exceeds maximum message length (128MiB)\0" as *const u8
                as *const gchar,
        );
        ret = -(1 as ::core::ffi::c_int) as gssize;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_new_from_blob(
    mut blob: *mut guchar,
    mut blob_len: gsize,
    mut capabilities: GDBusCapabilityFlags,
    mut error: *mut *mut GError,
) -> *mut GDBusMessage {
    let mut current_block: u64;
    let mut local_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut mbuf: GMemoryBuffer = _GMemoryBuffer {
        len: 0,
        valid_len: 0,
        pos: 0,
        data: ::core::ptr::null_mut::<gchar>(),
        byte_order: G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN,
    };
    let mut message: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut endianness: guchar = 0;
    let mut major_protocol_version: guchar = 0;
    let mut message_body_len: guint32 = 0;
    let mut headers: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut item: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    let mut signature: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if !blob.is_null() {
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"blob != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_72 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_72 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_72
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    message = safe_c2rust_g_dbus_message_new();
    memset(
        &raw mut mbuf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GMemoryBuffer>() as size_t,
    );
    mbuf.data = blob as *mut gchar;
    mbuf.valid_len = blob_len;
    mbuf.len = mbuf.valid_len;
    endianness = safe_c2rust_g_memory_buffer_read_byte(&raw mut mbuf, &raw mut local_error);
    if local_error.is_null() {
        match endianness as ::core::ffi::c_int {
            108 => {
                mbuf.byte_order = G_DATA_STREAM_BYTE_ORDER_LITTLE_ENDIAN;
                (*message).byte_order = G_DBUS_MESSAGE_BYTE_ORDER_LITTLE_ENDIAN;
                current_block = 16203760046146113240;
            }
            66 => {
                mbuf.byte_order = G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN;
                (*message).byte_order = G_DBUS_MESSAGE_BYTE_ORDER_BIG_ENDIAN;
                current_block = 16203760046146113240;
            }
            _ => {
                g_set_error(
                    &raw mut local_error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Invalid endianness value. Expected 0x6c (\xE2\x80\x9Cl\xE2\x80\x9D) or 0x42 (\xE2\x80\x9CB\xE2\x80\x9D) but found value 0x%02x\0"
                            as *const u8 as *const gchar,
                    ),
                    endianness as ::core::ffi::c_int,
                );
                current_block = 18402572457850173932;
            }
        }
        match current_block {
            18402572457850173932 => {}
            _ => {
                (*message).type_0 =
                    safe_c2rust_g_memory_buffer_read_byte(&raw mut mbuf, &raw mut local_error)
                        as GDBusMessageType;
                if local_error.is_null() {
                    (*message).flags =
                        safe_c2rust_g_memory_buffer_read_byte(&raw mut mbuf, &raw mut local_error)
                            as GDBusMessageFlags;
                    if local_error.is_null() {
                        major_protocol_version = safe_c2rust_g_memory_buffer_read_byte(
                            &raw mut mbuf,
                            &raw mut local_error,
                        );
                        if local_error.is_null() {
                            if major_protocol_version as ::core::ffi::c_int
                                != 1 as ::core::ffi::c_int
                            {
                                g_set_error(
                                    &raw mut local_error,
                                    g_io_error_quark(),
                                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                    glib_gettext(
                                        b"Invalid major protocol version. Expected 1 but found %d\0"
                                            as *const u8
                                            as *const gchar,
                                    ),
                                    major_protocol_version as ::core::ffi::c_int,
                                );
                            } else {
                                message_body_len = safe_c2rust_g_memory_buffer_read_uint32(
                                    &raw mut mbuf,
                                    &raw mut local_error,
                                );
                                if local_error.is_null() {
                                    (*message).serial = safe_c2rust_g_memory_buffer_read_uint32(
                                        &raw mut mbuf,
                                        &raw mut local_error,
                                    );
                                    if local_error.is_null() {
                                        headers = safe_c2rust_parse_value_from_blob(
                                            &raw mut mbuf,
                                            g_variant_type_checked_(
                                                b"a{yv}\0" as *const u8 as *const gchar,
                                            ),
                                            (G_DBUS_MAX_TYPE_DEPTH + 2 as ::core::ffi::c_int)
                                                as guint,
                                            FALSE,
                                            2 as guint,
                                            &raw mut local_error,
                                        );
                                        if !headers.is_null() {
                                            g_variant_iter_init(&raw mut iter, headers);
                                            loop {
                                                item = g_variant_iter_next_value(&raw mut iter);
                                                if item.is_null() {
                                                    break;
                                                }
                                                let mut header_field: guchar = 0;
                                                let mut value: *mut GVariant =
                                                    ::core::ptr::null_mut::<GVariant>();
                                                g_variant_get(
                                                    item,
                                                    b"{yv}\0" as *const u8 as *const gchar,
                                                    &raw mut header_field,
                                                    &raw mut value,
                                                );
                                                safe_c2rust_g_dbus_message_set_header(
                                                    message,
                                                    header_field as GDBusMessageHeaderField,
                                                    value,
                                                );
                                                g_variant_unref(value);
                                                g_variant_unref(item);
                                            }
                                            g_variant_unref(headers);
                                            signature = safe_c2rust_g_dbus_message_get_header(
                                                message,
                                                G_DBUS_MESSAGE_HEADER_FIELD_SIGNATURE,
                                            );
                                            if !signature.is_null() {
                                                let mut signature_str: *const gchar =
                                                    ::core::ptr::null::<gchar>();
                                                let mut signature_str_len: gsize = 0;
                                                if g_variant_is_of_type(
                                                    signature,
                                                    G_VARIANT_TYPE_SIGNATURE,
                                                ) == 0
                                                {
                                                    g_set_error_literal(
                                                        &raw mut local_error,
                                                        g_io_error_quark(),
                                                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                                        glib_gettext(
                                                            b"Signature header found but is not of type signature\0"
                                                                as *const u8 as *const gchar,
                                                        ),
                                                    );
                                                    current_block = 18402572457850173932;
                                                } else {
                                                    signature_str = g_variant_get_string(
                                                        signature,
                                                        &raw mut signature_str_len,
                                                    );
                                                    if message_body_len == 0 as guint32
                                                        && signature_str_len > 0 as gsize
                                                    {
                                                        g_set_error(
                                                            &raw mut local_error,
                                                            g_io_error_quark(),
                                                            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                                            glib_gettext(
                                                                b"Signature header with signature \xE2\x80\x9C%s\xE2\x80\x9D found but message body is empty\0"
                                                                    as *const u8 as *const gchar,
                                                            ),
                                                            signature_str,
                                                        );
                                                        current_block = 18402572457850173932;
                                                    } else if signature_str_len > 0 as gsize {
                                                        let mut variant_type: *mut GVariantType =
                                                            ::core::ptr::null_mut::<GVariantType>();
                                                        let mut tupled_signature_str: *mut gchar =
                                                            g_strdup_printf(
                                                                b"(%s)\0" as *const u8
                                                                    as *const gchar,
                                                                signature_str,
                                                            );
                                                        if g_variant_is_signature(signature_str)
                                                            == 0
                                                            || g_variant_type_string_is_valid(
                                                                tupled_signature_str,
                                                            ) == 0
                                                        {
                                                            g_set_error(
                                                                &raw mut local_error,
                                                                g_io_error_quark(),
                                                                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                                                glib_gettext(
                                                                    b"Parsed value \xE2\x80\x9C%s\xE2\x80\x9D is not a valid D-Bus signature (for body)\0"
                                                                        as *const u8 as *const gchar,
                                                                ),
                                                                signature_str,
                                                            );
                                                            g_free(
                                                                tupled_signature_str as gpointer,
                                                            );
                                                            current_block = 18402572457850173932;
                                                        } else {
                                                            variant_type = g_variant_type_new(
                                                                tupled_signature_str,
                                                            );
                                                            g_free(
                                                                tupled_signature_str as gpointer,
                                                            );
                                                            (*message).body =
                                                                safe_c2rust_parse_value_from_blob(
                                                                    &raw mut mbuf,
                                                                    variant_type,
                                                                    (G_DBUS_MAX_TYPE_DEPTH
                                                                        + 1 as ::core::ffi::c_int)
                                                                        as guint,
                                                                    FALSE,
                                                                    2 as guint,
                                                                    &raw mut local_error,
                                                                );
                                                            g_variant_type_free(variant_type);
                                                            if !(*message).body.is_null()
                                                                && g_variant_is_of_type(
                                                                    (*message).body,
                                                                    G_VARIANT_TYPE_TUPLE,
                                                                ) != 0
                                                                && g_variant_n_children(
                                                                    (*message).body,
                                                                ) > 0 as gsize
                                                            {
                                                                (*message).arg0_cache =
                                                                    g_variant_get_child_value(
                                                                        (*message).body,
                                                                        0 as gsize,
                                                                    );
                                                            } else {
                                                                (*message).arg0_cache =
                                                                    ::core::ptr::null_mut::<GVariant>(
                                                                    );
                                                            }
                                                            if (*message).body.is_null() {
                                                                current_block =
                                                                    18402572457850173932;
                                                            } else {
                                                                current_block = 5181772461570869434;
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 5181772461570869434;
                                                    }
                                                }
                                            } else if message_body_len != 0 as guint32 {
                                                g_set_error(
                                                    &raw mut local_error,
                                                    g_io_error_quark(),
                                                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                                    g_dngettext(
                                                        GETTEXT_PACKAGE.as_ptr() as *const gchar,
                                                        b"No signature header in message but the message body is %u byte\0"
                                                            as *const u8 as *const gchar,
                                                        b"No signature header in message but the message body is %u bytes\0"
                                                            as *const u8 as *const gchar,
                                                        message_body_len as gulong,
                                                    ),
                                                    message_body_len,
                                                );
                                                current_block = 18402572457850173932;
                                            } else {
                                                current_block = 5181772461570869434;
                                            }
                                            match current_block {
                                                18402572457850173932 => {}
                                                _ => {
                                                    if safe_c2rust_validate_headers(
                                                        message,
                                                        &raw mut local_error,
                                                    ) == 0
                                                    {
                                                        g_prefix_error(
                                                            &raw mut local_error,
                                                            glib_gettext(
                                                                b"Cannot deserialize message: \0"
                                                                    as *const u8
                                                                    as *const gchar,
                                                            ),
                                                        );
                                                    } else {
                                                        return message;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut _pp: *mut *mut GDBusMessage = &raw mut message;
    let mut _ptr: *mut GDBusMessage = *_pp;
    *_pp = ::core::ptr::null_mut::<GDBusMessage>();
    if !_ptr.is_null() {
        g_object_unref(_ptr as gpointer);
    }
    g_propagate_error(error, local_error);
    return ::core::ptr::null_mut::<GDBusMessage>();
}
unsafe extern "C" fn safe_c2rust_ensure_output_padding(
    mut mbuf: *mut GMemoryBuffer,
    mut padding_size: gsize,
) -> gsize {
    let mut offset: gsize = 0;
    let mut wanted_offset: gsize = 0;
    let mut padding_needed: gsize = 0;
    let mut n: guint = 0;
    offset = (*mbuf).pos;
    wanted_offset = offset
        .wrapping_add(padding_size)
        .wrapping_sub(1 as gsize)
        .wrapping_div(padding_size)
        .wrapping_mul(padding_size);
    padding_needed = wanted_offset.wrapping_sub(offset);
    n = 0 as guint;
    while (n as gsize) < padding_needed {
        safe_c2rust_g_memory_buffer_put_byte(mbuf, '\0' as i32 as guchar);
        n = n.wrapping_add(1);
    }
    return padding_needed;
}
unsafe extern "C" fn safe_c2rust_append_value_to_blob(
    mut value: *mut GVariant,
    mut type_0: *const GVariantType,
    mut mbuf: *mut GMemoryBuffer,
    mut out_padding_added: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut padding_added: gsize = 0;
    let mut type_string: *const gchar = ::core::ptr::null::<gchar>();
    type_string = g_variant_type_peek_string(type_0);
    padding_added = 0 as gsize;
    match *type_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
        98 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 4 as gsize);
            if !value.is_null() {
                let mut v: gboolean = g_variant_get_boolean(value);
                safe_c2rust_g_memory_buffer_put_uint32(mbuf, v as guint32);
            }
            current_block = 11702799181856929651;
        }
        121 => {
            if !value.is_null() {
                let mut v_0: guint8 = g_variant_get_byte(value);
                safe_c2rust_g_memory_buffer_put_byte(mbuf, v_0 as guchar);
            }
            current_block = 11702799181856929651;
        }
        110 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 2 as gsize);
            if !value.is_null() {
                let mut v_1: gint16 = g_variant_get_int16(value);
                safe_c2rust_g_memory_buffer_put_int16(mbuf, v_1);
            }
            current_block = 11702799181856929651;
        }
        113 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 2 as gsize);
            if !value.is_null() {
                let mut v_2: guint16 = g_variant_get_uint16(value);
                safe_c2rust_g_memory_buffer_put_uint16(mbuf, v_2);
            }
            current_block = 11702799181856929651;
        }
        105 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 4 as gsize);
            if !value.is_null() {
                let mut v_3: gint32 = g_variant_get_int32(value);
                safe_c2rust_g_memory_buffer_put_int32(mbuf, v_3);
            }
            current_block = 11702799181856929651;
        }
        117 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 4 as gsize);
            if !value.is_null() {
                let mut v_4: guint32 = g_variant_get_uint32(value);
                safe_c2rust_g_memory_buffer_put_uint32(mbuf, v_4);
            }
            current_block = 11702799181856929651;
        }
        120 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 8 as gsize);
            if !value.is_null() {
                let mut v_5: gint64 = g_variant_get_int64(value);
                safe_c2rust_g_memory_buffer_put_int64(mbuf, v_5);
            }
            current_block = 11702799181856929651;
        }
        116 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 8 as gsize);
            if !value.is_null() {
                let mut v_6: guint64 = g_variant_get_uint64(value);
                safe_c2rust_g_memory_buffer_put_uint64(mbuf, v_6);
            }
            current_block = 11702799181856929651;
        }
        100 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 8 as gsize);
            if !value.is_null() {
                let mut u: C2RustUnnamed_4 = C2RustUnnamed_4 { v_uint64: 0 };
                u.v_double = g_variant_get_double(value);
                safe_c2rust_g_memory_buffer_put_uint64(mbuf, u.v_uint64);
            }
            current_block = 11702799181856929651;
        }
        115 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 4 as gsize);
            if !value.is_null() {
                let mut len: gsize = 0;
                let mut v_7: *const gchar = ::core::ptr::null::<gchar>();
                let mut end: *const gchar = ::core::ptr::null::<gchar>();
                v_7 = g_variant_get_string(value, &raw mut len);
                if ({
                    let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
                    if g_utf8_validate(v_7, -(1 as ::core::ffi::c_int) as gssize, &raw mut end) != 0
                        && end == v_7.offset(len as isize)
                    {
                        _g_boolean_var_73 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_73 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_73
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        2675 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"g_utf8_validate (v, -1, &end) && (end == v + len)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                safe_c2rust_g_memory_buffer_put_uint32(mbuf, len as guint32);
                safe_c2rust_g_memory_buffer_put_string(mbuf, v_7 as *const ::core::ffi::c_char);
                safe_c2rust_g_memory_buffer_put_byte(mbuf, '\0' as i32 as guchar);
            }
            current_block = 11702799181856929651;
        }
        111 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 4 as gsize);
            if !value.is_null() {
                let mut len_0: gsize = 0;
                let mut v_8: *const gchar = g_variant_get_string(value, &raw mut len_0);
                if ({
                    let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
                    if g_variant_is_object_path(v_8) != 0 {
                        _g_boolean_var_74 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_74 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_74
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        2688 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"g_variant_is_object_path (v)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                safe_c2rust_g_memory_buffer_put_uint32(mbuf, len_0 as guint32);
                safe_c2rust_g_memory_buffer_put_string(mbuf, v_8 as *const ::core::ffi::c_char);
                safe_c2rust_g_memory_buffer_put_byte(mbuf, '\0' as i32 as guchar);
            }
            current_block = 11702799181856929651;
        }
        103 => {
            if !value.is_null() {
                let mut len_1: gsize = 0;
                let mut v_9: *const gchar = g_variant_get_string(value, &raw mut len_1);
                if ({
                    let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
                    if g_variant_is_signature(v_9) != 0 {
                        _g_boolean_var_75 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_75 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_75
                }) as ::core::ffi::c_long
                    != 0
                {
                } else {
                    g_assertion_message_expr(
                        G_LOG_DOMAIN.as_ptr(),
                        b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        2700 as ::core::ffi::c_int,
                        G_STRFUNC,
                        b"g_variant_is_signature (v)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                safe_c2rust_g_memory_buffer_put_byte(mbuf, len_1 as guchar);
                safe_c2rust_g_memory_buffer_put_string(mbuf, v_9 as *const ::core::ffi::c_char);
                safe_c2rust_g_memory_buffer_put_byte(mbuf, '\0' as i32 as guchar);
            }
            current_block = 11702799181856929651;
        }
        104 => {
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 4 as gsize);
            if !value.is_null() {
                let mut v_10: gint32 = g_variant_get_handle(value);
                safe_c2rust_g_memory_buffer_put_int32(mbuf, v_10);
            }
            current_block = 11702799181856929651;
        }
        97 => {
            let mut element_type: *const GVariantType = ::core::ptr::null::<GVariantType>();
            let mut item: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
            let mut array_len_offset: goffset = 0;
            let mut array_payload_begin_offset: goffset = 0;
            let mut cur_offset: goffset = 0;
            let mut array_len: gsize = 0;
            let mut fixed_size: guint = 0;
            padding_added = safe_c2rust_ensure_output_padding(mbuf, 4 as gsize);
            if !value.is_null() {
                array_len_offset = (*mbuf).valid_len as goffset;
                safe_c2rust_g_memory_buffer_put_uint32(mbuf, 0xf00dface as guint32);
                array_payload_begin_offset = (*mbuf).valid_len as goffset;
                element_type = g_variant_type_element(type_0);
                fixed_size = safe_c2rust_get_type_fixed_size(element_type);
                if g_variant_n_children(value) == 0 as gsize {
                    let mut padding_added_for_item: gsize = 0;
                    if safe_c2rust_append_value_to_blob(
                        ::core::ptr::null_mut::<GVariant>(),
                        element_type,
                        mbuf,
                        &raw mut padding_added_for_item,
                        error,
                    ) == 0
                    {
                        current_block = 17793135415593099369;
                    } else {
                        array_payload_begin_offset = (array_payload_begin_offset as gsize)
                            .wrapping_add(padding_added_for_item)
                            as goffset
                            as goffset;
                        current_block = 15447629348493591490;
                    }
                } else if fixed_size != 0 as guint {
                    let mut use_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                    if safe_c2rust_g_memory_buffer_is_byteswapped(mbuf) != 0 {
                        use_value = g_variant_byteswap(value);
                    } else {
                        use_value = g_variant_ref(value);
                    }
                    array_payload_begin_offset = (array_payload_begin_offset as gsize)
                        .wrapping_add(safe_c2rust_ensure_output_padding(mbuf, fixed_size as gsize))
                        as goffset as goffset;
                    array_len = g_variant_get_size(use_value);
                    safe_c2rust_g_memory_buffer_write(
                        mbuf,
                        g_variant_get_data(use_value) as *const ::core::ffi::c_void,
                        array_len,
                    );
                    g_variant_unref(use_value);
                    current_block = 15447629348493591490;
                } else {
                    let mut n: guint = 0;
                    n = 0 as guint;
                    g_variant_iter_init(&raw mut iter, value);
                    loop {
                        item = g_variant_iter_next_value(&raw mut iter);
                        if item.is_null() {
                            current_block = 15447629348493591490;
                            break;
                        }
                        let mut padding_added_for_item_0: gsize = 0;
                        if safe_c2rust_append_value_to_blob(
                            item,
                            g_variant_get_type(item),
                            mbuf,
                            &raw mut padding_added_for_item_0,
                            error,
                        ) == 0
                        {
                            g_variant_unref(item);
                            current_block = 17793135415593099369;
                            break;
                        } else {
                            g_variant_unref(item);
                            if n == 0 as guint {
                                array_payload_begin_offset = (array_payload_begin_offset as gsize)
                                    .wrapping_add(padding_added_for_item_0)
                                    as goffset
                                    as goffset;
                            }
                            n = n.wrapping_add(1);
                        }
                    }
                }
                match current_block {
                    17793135415593099369 => {}
                    _ => {
                        cur_offset = (*mbuf).valid_len as goffset;
                        array_len = (cur_offset - array_payload_begin_offset) as gsize;
                        (*mbuf).pos = array_len_offset as gsize;
                        safe_c2rust_g_memory_buffer_put_uint32(mbuf, array_len as guint32);
                        (*mbuf).pos = cur_offset as gsize;
                        current_block = 11702799181856929651;
                    }
                }
            } else {
                current_block = 11702799181856929651;
            }
        }
        _ => {
            if g_variant_type_is_dict_entry(type_0) != 0 || g_variant_type_is_tuple(type_0) != 0 {
                if g_variant_type_first(type_0).is_null() {
                    g_set_error_literal(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Empty structures (tuples) are not allowed in D-Bus\0" as *const u8
                                as *const gchar,
                        ),
                    );
                    current_block = 17793135415593099369;
                } else {
                    padding_added = safe_c2rust_ensure_output_padding(mbuf, 8 as gsize);
                    if !value.is_null() {
                        let mut item_0: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                        let mut iter_0: GVariantIter = _GVariantIter { x: [0; 16] };
                        g_variant_iter_init(&raw mut iter_0, value);
                        loop {
                            item_0 = g_variant_iter_next_value(&raw mut iter_0);
                            if item_0.is_null() {
                                current_block = 11702799181856929651;
                                break;
                            }
                            if safe_c2rust_append_value_to_blob(
                                item_0,
                                g_variant_get_type(item_0),
                                mbuf,
                                ::core::ptr::null_mut::<gsize>(),
                                error,
                            ) == 0
                            {
                                g_variant_unref(item_0);
                                current_block = 17793135415593099369;
                                break;
                            } else {
                                g_variant_unref(item_0);
                            }
                        }
                    } else {
                        current_block = 11702799181856929651;
                    }
                }
            } else if g_variant_type_is_variant(type_0) != 0 {
                if !value.is_null() {
                    let mut child: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
                    let mut signature: *const gchar = ::core::ptr::null::<gchar>();
                    child = g_variant_get_child_value(value, 0 as gsize);
                    signature = g_variant_get_type_string(child);
                    safe_c2rust_g_memory_buffer_put_byte(
                        mbuf,
                        strlen(signature as *const ::core::ffi::c_char) as guchar,
                    );
                    safe_c2rust_g_memory_buffer_put_string(
                        mbuf,
                        signature as *const ::core::ffi::c_char,
                    );
                    safe_c2rust_g_memory_buffer_put_byte(mbuf, '\0' as i32 as guchar);
                    if safe_c2rust_append_value_to_blob(
                        child,
                        g_variant_get_type(child),
                        mbuf,
                        ::core::ptr::null_mut::<gsize>(),
                        error,
                    ) == 0
                    {
                        g_variant_unref(child);
                        current_block = 17793135415593099369;
                    } else {
                        g_variant_unref(child);
                        current_block = 11702799181856929651;
                    }
                } else {
                    current_block = 11702799181856929651;
                }
            } else {
                g_set_error(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Error serializing GVariant with type string \xE2\x80\x9C%s\xE2\x80\x9D to the D-Bus wire format\0"
                            as *const u8 as *const gchar,
                    ),
                    g_variant_get_type_string(value),
                );
                current_block = 17793135415593099369;
            }
        }
    }
    match current_block {
        17793135415593099369 => return FALSE,
        _ => {
            if !out_padding_added.is_null() {
                *out_padding_added = padding_added;
            }
            return TRUE;
        }
    };
}
unsafe extern "C" fn safe_c2rust_append_body_to_blob(
    mut value: *mut GVariant,
    mut mbuf: *mut GMemoryBuffer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut item: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut iter: GVariantIter = _GVariantIter { x: [0; 16] };
    if g_variant_is_of_type(value, G_VARIANT_TYPE_TUPLE) == 0 {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            b"Expected a tuple for the body of the GDBusMessage.\0" as *const u8 as *const gchar,
        );
    } else {
        g_variant_iter_init(&raw mut iter, value);
        loop {
            item = g_variant_iter_next_value(&raw mut iter);
            if item.is_null() {
                current_block = 7351195479953500246;
                break;
            }
            if safe_c2rust_append_value_to_blob(
                item,
                g_variant_get_type(item),
                mbuf,
                ::core::ptr::null_mut::<gsize>(),
                error,
            ) == 0
            {
                g_variant_unref(item);
                current_block = 2105953587386888737;
                break;
            } else {
                g_variant_unref(item);
            }
        }
        match current_block {
            2105953587386888737 => {}
            _ => return TRUE,
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_to_blob(
    mut message: *mut GDBusMessage,
    mut out_size: *mut gsize,
    mut capabilities: GDBusCapabilityFlags,
    mut error: *mut *mut GError,
) -> *mut guchar {
    let mut current_block: u64;
    let mut mbuf: GMemoryBuffer = _GMemoryBuffer {
        len: 0,
        valid_len: 0,
        pos: 0,
        data: ::core::ptr::null_mut::<gchar>(),
        byte_order: G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN,
    };
    let mut ret: *mut guchar = ::core::ptr::null_mut::<guchar>();
    let mut size: gsize = 0;
    let mut body_len_offset: goffset = 0;
    let mut body_start_offset: goffset = 0;
    let mut body_size: gsize = 0;
    let mut header_fields: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut builder: GVariantBuilder = _GVariantBuilder {
        u: C2RustUnnamed {
            s: C2RustUnnamed_0 {
                partial_magic: 0,
                type_0: ::core::ptr::null::<GVariantType>(),
                y: [0; 14],
            },
        },
    };
    let mut hash_iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut header_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut signature: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut signature_str: *const gchar = ::core::ptr::null::<gchar>();
    let mut num_fds_in_message: gint = 0;
    let mut num_fds_according_to_header: gint = 0;
    ret = ::core::ptr::null_mut::<guchar>();
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_76 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_76 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_76
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guchar>();
    }
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if !out_size.is_null() {
            _g_boolean_var_77 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_77 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_77
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"out_size != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guchar>();
    }
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_78 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_78 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_78
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<guchar>();
    }
    memset(
        &raw mut mbuf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<GMemoryBuffer>() as size_t,
    );
    mbuf.len = MIN_ARRAY_SIZE as gsize;
    mbuf.data = g_malloc(mbuf.len) as *mut gchar;
    mbuf.byte_order = G_DATA_STREAM_BYTE_ORDER_HOST_ENDIAN;
    match (*message).byte_order as ::core::ffi::c_uint {
        66 => {
            mbuf.byte_order = G_DATA_STREAM_BYTE_ORDER_BIG_ENDIAN;
        }
        108 => {
            mbuf.byte_order = G_DATA_STREAM_BYTE_ORDER_LITTLE_ENDIAN;
        }
        _ => {}
    }
    safe_c2rust_g_memory_buffer_put_byte(&raw mut mbuf, (*message).byte_order as guchar);
    safe_c2rust_g_memory_buffer_put_byte(&raw mut mbuf, (*message).type_0 as guchar);
    safe_c2rust_g_memory_buffer_put_byte(&raw mut mbuf, (*message).flags as guchar);
    safe_c2rust_g_memory_buffer_put_byte(&raw mut mbuf, 1 as guchar);
    body_len_offset = mbuf.valid_len as goffset;
    safe_c2rust_g_memory_buffer_put_uint32(&raw mut mbuf, 0xf00dface as guint32);
    safe_c2rust_g_memory_buffer_put_uint32(&raw mut mbuf, (*message).serial);
    num_fds_in_message = 0 as ::core::ffi::c_int as gint;
    if !(*message).fd_list.is_null() {
        num_fds_in_message = g_unix_fd_list_get_length((*message).fd_list);
    }
    num_fds_according_to_header = safe_c2rust_g_dbus_message_get_num_unix_fds(message) as gint;
    if num_fds_in_message != num_fds_according_to_header {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Number of file descriptors in message (%d) differs from header field (%d)\0"
                    as *const u8 as *const gchar,
            ),
            num_fds_in_message,
            num_fds_according_to_header,
        );
    } else if safe_c2rust_validate_headers(message, error) == 0 {
        g_prefix_error(
            error,
            glib_gettext(b"Cannot serialize message: \0" as *const u8 as *const gchar),
        );
    } else {
        g_variant_builder_init(
            &raw mut builder,
            g_variant_type_checked_(b"a{yv}\0" as *const u8 as *const gchar),
        );
        g_hash_table_iter_init(&raw mut hash_iter, (*message).headers);
        while g_hash_table_iter_next(
            &raw mut hash_iter,
            &raw mut key,
            &raw mut header_value as gpointer as *mut gpointer,
        ) != 0
        {
            g_variant_builder_add(
                &raw mut builder,
                b"{yv}\0" as *const u8 as *const gchar,
                key as gulong as guint as guchar as ::core::ffi::c_int,
                header_value,
            );
        }
        header_fields = g_variant_builder_end(&raw mut builder);
        if safe_c2rust_append_value_to_blob(
            header_fields,
            g_variant_get_type(header_fields),
            &raw mut mbuf,
            ::core::ptr::null_mut::<gsize>(),
            error,
        ) == 0
        {
            g_variant_unref(header_fields);
        } else {
            g_variant_unref(header_fields);
            safe_c2rust_ensure_output_padding(&raw mut mbuf, 8 as gsize);
            body_start_offset = mbuf.valid_len as goffset;
            signature = safe_c2rust_g_dbus_message_get_header(
                message,
                G_DBUS_MESSAGE_HEADER_FIELD_SIGNATURE,
            );
            if !signature.is_null()
                && g_variant_is_of_type(signature, G_VARIANT_TYPE_SIGNATURE) == 0
            {
                g_set_error_literal(
                    error,
                    g_io_error_quark(),
                    G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Signature header found but is not of type signature\0" as *const u8
                            as *const gchar,
                    ),
                );
            } else {
                signature_str = ::core::ptr::null::<gchar>();
                if !signature.is_null() {
                    signature_str =
                        g_variant_get_string(signature, ::core::ptr::null_mut::<gsize>());
                }
                if !(*message).body.is_null() {
                    let mut tupled_signature_str: *mut gchar = ::core::ptr::null_mut::<gchar>();
                    if signature.is_null() {
                        g_set_error(
                            error,
                            g_io_error_quark(),
                            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                            glib_gettext(
                                b"Message body has signature \xE2\x80\x9C%s\xE2\x80\x9D but there is no signature header\0"
                                    as *const u8 as *const gchar,
                            ),
                            g_variant_get_type_string((*message).body),
                        );
                        current_block = 13859201457851849047;
                    } else {
                        tupled_signature_str =
                            g_strdup_printf(b"(%s)\0" as *const u8 as *const gchar, signature_str);
                        if g_strcmp0(
                            tupled_signature_str,
                            g_variant_get_type_string((*message).body)
                                as *const ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int
                        {
                            g_set_error(
                                error,
                                g_io_error_quark(),
                                G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                                glib_gettext(
                                    b"Message body has type signature \xE2\x80\x9C%s\xE2\x80\x9D but signature in the header field is \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                        as *const u8 as *const gchar,
                                ),
                                g_variant_get_type_string((*message).body),
                                tupled_signature_str,
                            );
                            g_free(tupled_signature_str as gpointer);
                            current_block = 13859201457851849047;
                        } else {
                            g_free(tupled_signature_str as gpointer);
                            if safe_c2rust_append_body_to_blob(
                                (*message).body,
                                &raw mut mbuf,
                                error,
                            ) == 0
                            {
                                current_block = 13859201457851849047;
                            } else {
                                current_block = 13707613154239713890;
                            }
                        }
                    }
                } else if !signature.is_null()
                    && strlen(signature_str as *const ::core::ffi::c_char) > 0 as size_t
                {
                    g_set_error(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Message body is empty but signature in the header field is \xE2\x80\x9C(%s)\xE2\x80\x9D\0"
                                as *const u8 as *const gchar,
                        ),
                        signature_str,
                    );
                    current_block = 13859201457851849047;
                } else {
                    current_block = 13707613154239713890;
                }
                match current_block {
                    13859201457851849047 => {}
                    _ => {
                        size = mbuf.valid_len;
                        body_size = size.wrapping_sub(body_start_offset as gsize);
                        mbuf.pos = body_len_offset as gsize;
                        safe_c2rust_g_memory_buffer_put_uint32(&raw mut mbuf, body_size as guint32);
                        *out_size = size;
                        ret = mbuf.data as *mut guchar;
                    }
                }
            }
        }
    }
    if ret.is_null() {
        g_free(mbuf.data as gpointer);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_get_uint32_header(
    mut message: *mut GDBusMessage,
    mut header_field: GDBusMessageHeaderField,
) -> guint32 {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ret: guint32 = 0;
    ret = 0 as guint32;
    value = g_hash_table_lookup(
        (*message).headers,
        header_field as gulong as gpointer as gconstpointer,
    ) as *mut GVariant;
    if !value.is_null() && g_variant_is_of_type(value, G_VARIANT_TYPE_UINT32) != 0 {
        ret = g_variant_get_uint32(value);
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_get_string_header(
    mut message: *mut GDBusMessage,
    mut header_field: GDBusMessageHeaderField,
) -> *const gchar {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    ret = ::core::ptr::null::<gchar>();
    value = g_hash_table_lookup(
        (*message).headers,
        header_field as gulong as gpointer as gconstpointer,
    ) as *mut GVariant;
    if !value.is_null() && g_variant_is_of_type(value, G_VARIANT_TYPE_STRING) != 0 {
        ret = g_variant_get_string(value, ::core::ptr::null_mut::<gsize>());
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_get_object_path_header(
    mut message: *mut GDBusMessage,
    mut header_field: GDBusMessageHeaderField,
) -> *const gchar {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    ret = ::core::ptr::null::<gchar>();
    value = g_hash_table_lookup(
        (*message).headers,
        header_field as gulong as gpointer as gconstpointer,
    ) as *mut GVariant;
    if !value.is_null() && g_variant_is_of_type(value, G_VARIANT_TYPE_OBJECT_PATH) != 0 {
        ret = g_variant_get_string(value, ::core::ptr::null_mut::<gsize>());
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_get_signature_header(
    mut message: *mut GDBusMessage,
    mut header_field: GDBusMessageHeaderField,
) -> *const gchar {
    let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    ret = ::core::ptr::null::<gchar>();
    value = g_hash_table_lookup(
        (*message).headers,
        header_field as gulong as gpointer as gconstpointer,
    ) as *mut GVariant;
    if !value.is_null() && g_variant_is_of_type(value, G_VARIANT_TYPE_SIGNATURE) != 0 {
        ret = g_variant_get_string(value, ::core::ptr::null_mut::<gsize>());
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_set_uint32_header(
    mut message: *mut GDBusMessage,
    mut header_field: GDBusMessageHeaderField,
    mut value: guint32,
) {
    safe_c2rust_g_dbus_message_set_header(message, header_field, g_variant_new_uint32(value));
}
unsafe extern "C" fn safe_c2rust_set_string_header(
    mut message: *mut GDBusMessage,
    mut header_field: GDBusMessageHeaderField,
    mut value: *const gchar,
) {
    safe_c2rust_g_dbus_message_set_header(
        message,
        header_field,
        if value.is_null() {
            ::core::ptr::null_mut::<GVariant>()
        } else {
            g_variant_new_string(value)
        },
    );
}
unsafe extern "C" fn safe_c2rust_set_object_path_header(
    mut message: *mut GDBusMessage,
    mut header_field: GDBusMessageHeaderField,
    mut value: *const gchar,
) {
    safe_c2rust_g_dbus_message_set_header(
        message,
        header_field,
        if value.is_null() {
            ::core::ptr::null_mut::<GVariant>()
        } else {
            g_variant_new_object_path(value)
        },
    );
}
unsafe extern "C" fn safe_c2rust_set_signature_header(
    mut message: *mut GDBusMessage,
    mut header_field: GDBusMessageHeaderField,
    mut value: *const gchar,
) {
    safe_c2rust_g_dbus_message_set_header(
        message,
        header_field,
        if value.is_null() {
            ::core::ptr::null_mut::<GVariant>()
        } else {
            g_variant_new_signature(value)
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_reply_serial(
    mut message: *mut GDBusMessage,
) -> guint32 {
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_79 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_79 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_79
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    return safe_c2rust_get_uint32_header(message, G_DBUS_MESSAGE_HEADER_FIELD_REPLY_SERIAL);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_reply_serial(
    mut message: *mut GDBusMessage,
    mut value: guint32,
) {
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_80 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_80 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_80
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_set_uint32_header(message, G_DBUS_MESSAGE_HEADER_FIELD_REPLY_SERIAL, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_interface(
    mut message: *mut GDBusMessage,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_81: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_81 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_81 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_81
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return safe_c2rust_get_string_header(message, G_DBUS_MESSAGE_HEADER_FIELD_INTERFACE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_interface(
    mut message: *mut GDBusMessage,
    mut value: *const gchar,
) {
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_82 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_82 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_82
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if value.is_null() || g_dbus_is_interface_name(value) != 0 {
            _g_boolean_var_83 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_83 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_83
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value == NULL || g_dbus_is_interface_name (value)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_set_string_header(message, G_DBUS_MESSAGE_HEADER_FIELD_INTERFACE, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_member(
    mut message: *mut GDBusMessage,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_84 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_84 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_84
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return safe_c2rust_get_string_header(message, G_DBUS_MESSAGE_HEADER_FIELD_MEMBER);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_member(
    mut message: *mut GDBusMessage,
    mut value: *const gchar,
) {
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_85 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_85 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_85
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if value.is_null() || g_dbus_is_member_name(value) != 0 {
            _g_boolean_var_86 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_86 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_86
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value == NULL || g_dbus_is_member_name (value)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_set_string_header(message, G_DBUS_MESSAGE_HEADER_FIELD_MEMBER, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_path(
    mut message: *mut GDBusMessage,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_87 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_87 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_87
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return safe_c2rust_get_object_path_header(message, G_DBUS_MESSAGE_HEADER_FIELD_PATH);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_path(
    mut message: *mut GDBusMessage,
    mut value: *const gchar,
) {
    if ({
        let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            _g_boolean_var_88 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_88 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_88
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
        if value.is_null() || g_variant_is_object_path(value) != 0 {
            _g_boolean_var_89 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_89 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_89
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value == NULL || g_variant_is_object_path (value)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_set_object_path_header(message, G_DBUS_MESSAGE_HEADER_FIELD_PATH, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_sender(
    mut message: *mut GDBusMessage,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return safe_c2rust_get_string_header(message, G_DBUS_MESSAGE_HEADER_FIELD_SENDER);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_sender(
    mut message: *mut GDBusMessage,
    mut value: *const gchar,
) {
    if ({
        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
        if value.is_null() || g_dbus_is_name(value) != 0 {
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
            b"value == NULL || g_dbus_is_name (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_set_string_header(message, G_DBUS_MESSAGE_HEADER_FIELD_SENDER, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_destination(
    mut message: *mut GDBusMessage,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return safe_c2rust_get_string_header(message, G_DBUS_MESSAGE_HEADER_FIELD_DESTINATION);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_destination(
    mut message: *mut GDBusMessage,
    mut value: *const gchar,
) {
    if ({
        let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
        if value.is_null() || g_dbus_is_name(value) != 0 {
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
            b"value == NULL || g_dbus_is_name (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_set_string_header(message, G_DBUS_MESSAGE_HEADER_FIELD_DESTINATION, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_error_name(
    mut message: *mut GDBusMessage,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    return safe_c2rust_get_string_header(message, G_DBUS_MESSAGE_HEADER_FIELD_ERROR_NAME);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_error_name(
    mut message: *mut GDBusMessage,
    mut value: *const gchar,
) {
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if value.is_null() || g_dbus_is_error_name(value) != 0 {
            _g_boolean_var_98 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_98 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_98
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"value == NULL || g_dbus_is_error_name (value)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_set_string_header(message, G_DBUS_MESSAGE_HEADER_FIELD_ERROR_NAME, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_signature(
    mut message: *mut GDBusMessage,
) -> *const gchar {
    let mut ret: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    ret = safe_c2rust_get_signature_header(message, G_DBUS_MESSAGE_HEADER_FIELD_SIGNATURE);
    if ret.is_null() {
        ret = b"\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_signature(
    mut message: *mut GDBusMessage,
    mut value: *const gchar,
) {
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
        if value.is_null() || g_variant_is_signature(value) != 0 {
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
            b"value == NULL || g_variant_is_signature (value)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_set_signature_header(message, G_DBUS_MESSAGE_HEADER_FIELD_SIGNATURE, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_arg0(
    mut message: *mut GDBusMessage,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if !(*message).arg0_cache.is_null()
        && g_variant_is_of_type((*message).arg0_cache, G_VARIANT_TYPE_STRING) != 0
    {
        return g_variant_get_string((*message).arg0_cache, ::core::ptr::null_mut::<gsize>());
    }
    return ::core::ptr::null::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_arg0_path(
    mut message: *mut GDBusMessage,
) -> *const gchar {
    if ({
        let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if !(*message).arg0_cache.is_null()
        && g_variant_is_of_type((*message).arg0_cache, G_VARIANT_TYPE_OBJECT_PATH) != 0
    {
        return g_variant_get_string((*message).arg0_cache, ::core::ptr::null_mut::<gsize>());
    }
    return ::core::ptr::null::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_num_unix_fds(
    mut message: *mut GDBusMessage,
) -> guint32 {
    if ({
        let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint32;
    }
    return safe_c2rust_get_uint32_header(message, G_DBUS_MESSAGE_HEADER_FIELD_NUM_UNIX_FDS);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_set_num_unix_fds(
    mut message: *mut GDBusMessage,
    mut value: guint32,
) {
    if ({
        let mut _g_boolean_var_105: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_set_uint32_header(message, G_DBUS_MESSAGE_HEADER_FIELD_NUM_UNIX_FDS, value);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_to_gerror(
    mut message: *mut GDBusMessage,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut ret: gboolean = 0;
    let mut error_name: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_106: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    ret = FALSE as gboolean;
    if !((*message).type_0 as ::core::ffi::c_uint
        != G_DBUS_MESSAGE_TYPE_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        error_name = safe_c2rust_g_dbus_message_get_error_name(message);
        if !error_name.is_null() {
            let mut body: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            body = safe_c2rust_g_dbus_message_get_body(message);
            if !body.is_null()
                && g_variant_is_of_type(
                    body,
                    g_variant_type_checked_(b"(s)\0" as *const u8 as *const gchar),
                ) != 0
            {
                let mut error_message: *const gchar = ::core::ptr::null::<gchar>();
                g_variant_get(
                    body,
                    b"(&s)\0" as *const u8 as *const gchar,
                    &raw mut error_message,
                );
                g_dbus_error_set_dbus_error(
                    error,
                    error_name,
                    error_message,
                    ::core::ptr::null::<gchar>(),
                );
            } else if !body.is_null() {
                g_dbus_error_set_dbus_error(
                    error,
                    error_name,
                    b"\0" as *const u8 as *const gchar,
                    glib_gettext(
                        b"Error return with body of type \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                            as *const gchar,
                    ),
                    g_variant_get_type_string(body),
                );
            } else {
                g_dbus_error_set_dbus_error(
                    error,
                    error_name,
                    b"\0" as *const u8 as *const gchar,
                    glib_gettext(b"Error return with empty body\0" as *const u8 as *const gchar),
                );
            }
        } else {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
                b"Error return without error-name header!\0" as *const u8 as *const gchar,
            );
        }
        ret = TRUE as gboolean;
    }
    return ret;
}
unsafe extern "C" fn safe_c2rust_flags_to_string(
    mut flags_type: GType,
    mut value: guint,
) -> *mut gchar {
    let mut s: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut klass: *mut GFlagsClass = ::core::ptr::null_mut::<GFlagsClass>();
    let mut n: guint = 0;
    klass = g_type_class_ref(flags_type) as *mut GFlagsClass;
    s = g_string_new(::core::ptr::null::<gchar>());
    n = 0 as guint;
    while n < 32 as guint {
        if value & ((1 as ::core::ffi::c_int) << n) as guint != 0 as guint {
            let mut flags_value: *mut GFlagsValue = ::core::ptr::null_mut::<GFlagsValue>();
            flags_value = g_flags_get_first_value(klass, ((1 as ::core::ffi::c_int) << n) as guint);
            if (*s).len > 0 as gsize {
                safe_c2rust_g_string_append_c_inline(s, ',' as i32 as gchar);
            }
            if !flags_value.is_null() {
                if 0 != 0 {
                    ({
                        let __val: *const ::core::ffi::c_char =
                            (*flags_value).value_nick as *const ::core::ffi::c_char;
                        safe_c2rust_g_string_append_len_inline(
                            s,
                            __val,
                            if ({
                                let mut _g_boolean_var_107: ::core::ffi::c_int = 0;
                                if !__val.is_null() {
                                    _g_boolean_var_107 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_107 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_107
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
                        s,
                        (*flags_value).value_nick as *const ::core::ffi::c_char,
                        -(1 as ::core::ffi::c_int) as gssize,
                    );
                };
            } else {
                g_string_append_printf(s, b"unknown (bit %d)\0" as *const u8 as *const gchar, n);
            }
        }
        n = n.wrapping_add(1);
    }
    if (*s).len == 0 as gsize {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"none\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    s,
                    __val,
                    if ({
                        let mut _g_boolean_var_108: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_108 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_108 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_108
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
                s,
                b"none\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    g_type_class_unref(klass as gpointer);
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(s, 0 as gboolean)
        } else {
            g_string_free_and_steal(s)
        }
    } else {
        g_string_free(s, 0 as gboolean)
    };
}
unsafe extern "C" fn safe_c2rust__sort_keys_func(
    mut a: gconstpointer,
    mut b: gconstpointer,
) -> gint {
    let mut ia: gint = 0;
    let mut ib: gint = 0;
    ia = a as glong as gint;
    ib = b as glong as gint;
    return ia - ib;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_print(
    mut message: *mut GDBusMessage,
    mut indent: guint,
) -> *mut gchar {
    let mut str: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut keys: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_109: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    str = g_string_new(::core::ptr::null::<gchar>());
    s = _g_dbus_enum_to_string(g_dbus_message_type_get_type(), (*message).type_0 as gint);
    g_string_append_printf(
        str,
        b"%*sType:    %s\n\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        s,
    );
    g_free(s as gpointer);
    s = safe_c2rust_flags_to_string(g_dbus_message_flags_get_type(), (*message).flags as guint);
    g_string_append_printf(
        str,
        b"%*sFlags:   %s\n\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        s,
    );
    g_free(s as gpointer);
    g_string_append_printf(
        str,
        b"%*sVersion: %d\n\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        (*message).major_protocol_version as ::core::ffi::c_int,
    );
    g_string_append_printf(
        str,
        b"%*sSerial:  %d\n\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        (*message).serial,
    );
    g_string_append_printf(
        str,
        b"%*sHeaders:\n\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    keys = g_hash_table_get_keys((*message).headers);
    keys = g_list_sort(
        keys,
        Some(
            safe_c2rust__sort_keys_func
                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
        ),
    );
    if !keys.is_null() {
        l = keys;
        while !l.is_null() {
            let mut key: gint = (*l).data as glong as gint;
            let mut value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
            let mut value_str: *mut gchar = ::core::ptr::null_mut::<gchar>();
            value = g_hash_table_lookup((*message).headers, (*l).data as gconstpointer)
                as *mut GVariant;
            if ({
                let mut _g_boolean_var_110: ::core::ffi::c_int = 0;
                if !value.is_null() {
                    _g_boolean_var_110 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_110 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_110
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gdbusmessage.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    3824 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            s = _g_dbus_enum_to_string(g_dbus_message_header_field_get_type(), key);
            value_str = g_variant_print(value, TRUE);
            g_string_append_printf(
                str,
                b"%*s  %s -> %s\n\0" as *const u8 as *const gchar,
                indent,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
                s,
                value_str,
            );
            g_free(s as gpointer);
            g_free(value_str as gpointer);
            l = (*l).next;
        }
    } else {
        g_string_append_printf(
            str,
            b"%*s  (none)\n\0" as *const u8 as *const gchar,
            indent,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_list_free(keys);
    g_string_append_printf(
        str,
        b"%*sBody: \0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !(*message).body.is_null() {
        g_variant_print_string((*message).body, str, TRUE);
    } else {
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"()\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    str,
                    __val,
                    if ({
                        let mut _g_boolean_var_111: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_111 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_111 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_111
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
                str,
                b"()\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char =
                b"\n\0" as *const u8 as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                str,
                __val,
                if ({
                    let mut _g_boolean_var_112: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_112 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_112 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_112
                }) as ::core::ffi::c_long
                    != 0
                {
                    strlen(__val.offset(__val.is_null() as ::core::ffi::c_int as isize)) as gssize
                } else {
                    -(1 as ::core::ffi::c_int) as gssize
                },
            );
        });
    } else {
        safe_c2rust_g_string_append_len_inline(
            str,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    g_string_append_printf(
        str,
        b"%*sUNIX File Descriptors:\n\0" as *const u8 as *const gchar,
        indent,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !(*message).fd_list.is_null() {
        let mut num_fds: gint = 0;
        let mut fds: *const gint = ::core::ptr::null::<gint>();
        let mut n: gint = 0;
        fds = g_unix_fd_list_peek_fds((*message).fd_list, &raw mut num_fds);
        if num_fds > 0 as ::core::ffi::c_int {
            n = 0 as ::core::ffi::c_int as gint;
            while n < num_fds {
                let mut fs: *mut GString = ::core::ptr::null_mut::<GString>();
                let mut statbuf: stat = stat {
                    st_dev: 0,
                    st_ino: 0,
                    st_nlink: 0,
                    st_mode: 0,
                    st_uid: 0,
                    st_gid: 0,
                    __pad0: 0,
                    st_rdev: 0,
                    st_size: 0,
                    st_blksize: 0,
                    st_blocks: 0,
                    st_atim: timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    },
                    st_mtim: timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    },
                    st_ctim: timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    },
                    __glibc_reserved: [0; 3],
                };
                fs = g_string_new(::core::ptr::null::<gchar>());
                if fstat(
                    *fds.offset(n as isize) as ::core::ffi::c_int,
                    &raw mut statbuf,
                ) == 0 as ::core::ffi::c_int
                {
                    g_string_append_printf(
                        fs,
                        b"%sdev=%d:%d\0" as *const u8 as *const gchar,
                        if (*fs).len > 0 as gsize {
                            b",\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        safe_c2rust_gnu_dev_major(statbuf.st_dev) as gint,
                        safe_c2rust_gnu_dev_minor(statbuf.st_dev) as gint,
                    );
                    g_string_append_printf(
                        fs,
                        b"%smode=0%o\0" as *const u8 as *const gchar,
                        if (*fs).len > 0 as gsize {
                            b",\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        statbuf.st_mode,
                    );
                    g_string_append_printf(
                        fs,
                        b"%sino=%lu\0" as *const u8 as *const gchar,
                        if (*fs).len > 0 as gsize {
                            b",\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        statbuf.st_ino,
                    );
                    g_string_append_printf(
                        fs,
                        b"%suid=%u\0" as *const u8 as *const gchar,
                        if (*fs).len > 0 as gsize {
                            b",\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        statbuf.st_uid,
                    );
                    g_string_append_printf(
                        fs,
                        b"%sgid=%u\0" as *const u8 as *const gchar,
                        if (*fs).len > 0 as gsize {
                            b",\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        statbuf.st_gid,
                    );
                    g_string_append_printf(
                        fs,
                        b"%srdev=%d:%d\0" as *const u8 as *const gchar,
                        if (*fs).len > 0 as gsize {
                            b",\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        safe_c2rust_gnu_dev_major(statbuf.st_rdev) as gint,
                        safe_c2rust_gnu_dev_minor(statbuf.st_rdev) as gint,
                    );
                    g_string_append_printf(
                        fs,
                        b"%ssize=%lu\0" as *const u8 as *const gchar,
                        if (*fs).len > 0 as gsize {
                            b",\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        statbuf.st_size as guint64,
                    );
                    g_string_append_printf(
                        fs,
                        b"%satime=%lu\0" as *const u8 as *const gchar,
                        if (*fs).len > 0 as gsize {
                            b",\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        statbuf.st_atim.tv_sec as guint64,
                    );
                    g_string_append_printf(
                        fs,
                        b"%smtime=%lu\0" as *const u8 as *const gchar,
                        if (*fs).len > 0 as gsize {
                            b",\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        statbuf.st_mtim.tv_sec as guint64,
                    );
                    g_string_append_printf(
                        fs,
                        b"%sctime=%lu\0" as *const u8 as *const gchar,
                        if (*fs).len > 0 as gsize {
                            b",\0" as *const u8 as *const ::core::ffi::c_char
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        statbuf.st_ctim.tv_sec as guint64,
                    );
                } else {
                    let mut errsv: ::core::ffi::c_int = *__errno_location();
                    g_string_append_printf(
                        fs,
                        b"(fstat failed: %s)\0" as *const u8 as *const gchar,
                        g_strerror(errsv as gint),
                    );
                }
                g_string_append_printf(
                    str,
                    b"%*s  fd %d: %s\n\0" as *const u8 as *const gchar,
                    indent,
                    b"\0" as *const u8 as *const ::core::ffi::c_char,
                    *fds.offset(n as isize),
                    (*fs).str_0,
                );
                if 0 != 0 {
                    if 0 as ::core::ffi::c_int == 0 {
                        g_string_free(fs, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                    } else {
                        g_string_free_and_steal(fs);
                    };
                } else {
                    g_string_free(fs, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
                };
                n += 1;
            }
        } else {
            g_string_append_printf(
                str,
                b"%*s  (empty)\n\0" as *const u8 as *const gchar,
                indent,
                b"\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        g_string_append_printf(
            str,
            b"%*s  (none)\n\0" as *const u8 as *const gchar,
            indent,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(str, 0 as gboolean)
        } else {
            g_string_free_and_steal(str)
        }
    } else {
        g_string_free(str, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_get_locked(
    mut message: *mut GDBusMessage,
) -> gboolean {
    if ({
        let mut _g_boolean_var_113: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (*message).locked;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_lock(mut message: *mut GDBusMessage) {
    if ({
        let mut _g_boolean_var_114: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !((*message).locked != 0) {
        (*message).locked = TRUE as gboolean;
        g_object_notify(
            message as *mut ::core::ffi::c_void as *mut GObject,
            b"locked\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dbus_message_copy(
    mut message: *mut GDBusMessage,
    mut error: *mut *mut GError,
) -> *mut GDBusMessage {
    let mut current_block: u64;
    let mut ret: *mut GDBusMessage = ::core::ptr::null_mut::<GDBusMessage>();
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut header_key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut header_value: *mut GVariant = ::core::ptr::null_mut::<GVariant>();
    if ({
        let mut _g_boolean_var_115: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = message as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_dbus_message_get_type();
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
            b"G_IS_DBUS_MESSAGE (message)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    if ({
        let mut _g_boolean_var_116: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDBusMessage>();
    }
    ret = safe_c2rust_g_dbus_message_new();
    (*ret).type_0 = (*message).type_0;
    (*ret).flags = (*message).flags;
    (*ret).byte_order = (*message).byte_order;
    (*ret).major_protocol_version = (*message).major_protocol_version;
    (*ret).serial = (*message).serial;
    if !(*message).fd_list.is_null() {
        let mut n: gint = 0;
        let mut num_fds: gint = 0;
        let mut fds: *const gint = ::core::ptr::null::<gint>();
        (*ret).fd_list = g_unix_fd_list_new();
        fds = g_unix_fd_list_peek_fds((*message).fd_list, &raw mut num_fds);
        n = 0 as ::core::ffi::c_int as gint;
        loop {
            if !(n < num_fds) {
                current_block = 9828876828309294594;
                break;
            }
            if g_unix_fd_list_append((*ret).fd_list, *fds.offset(n as isize), error)
                == -(1 as ::core::ffi::c_int)
            {
                g_object_unref(ret as gpointer);
                ret = ::core::ptr::null_mut::<GDBusMessage>();
                current_block = 10571691993352772611;
                break;
            } else {
                n += 1;
            }
        }
    } else {
        current_block = 9828876828309294594;
    }
    match current_block {
        9828876828309294594 => {
            (*ret).body = if !(*message).body.is_null() {
                g_variant_ref((*message).body)
            } else {
                ::core::ptr::null_mut::<GVariant>()
            };
            (*ret).arg0_cache = if !(*message).arg0_cache.is_null() {
                g_variant_ref((*message).arg0_cache)
            } else {
                ::core::ptr::null_mut::<GVariant>()
            };
            g_hash_table_iter_init(&raw mut iter, (*message).headers);
            while g_hash_table_iter_next(
                &raw mut iter,
                &raw mut header_key,
                &raw mut header_value as gpointer as *mut gpointer,
            ) != 0
            {
                g_hash_table_insert(
                    (*ret).headers,
                    header_key,
                    g_variant_ref(header_value) as gpointer,
                );
            }
        }
        _ => {}
    }
    return ret;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const GETTEXT_PACKAGE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"glib20\0") };
