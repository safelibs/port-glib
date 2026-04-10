// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_bitfields;
extern "C" {
    pub type _GBytes;
    pub type _GTimeZone;
    pub type _GDateTime;
    pub type _GBookmarkFile;
    pub type _GChecksum;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GHmac;
    pub type _GMainContext;
    pub type _GMainLoop;
    pub type _GSourcePrivate;
    pub type _GKeyFile;
    pub type _GMappedFile;
    pub type _GMarkupParseContext;
    pub type _GVariantType;
    pub type _GVariant;
    pub type _GOptionGroup;
    pub type _GPatternSpec;
    pub type _GRand;
    pub type _GRegex;
    pub type _GMatchInfo;
    pub type _GStrvBuilder;
    pub type _GTree;
    pub type _GUri;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_array_ref(array: *mut GArray) -> *mut GArray;
    fn g_array_unref(array: *mut GArray);
    fn g_ptr_array_ref(array: *mut GPtrArray) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_byte_array_ref(array: *mut GByteArray) -> *mut GByteArray;
    fn g_byte_array_unref(array: *mut GByteArray);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_copy(error: *const GError) -> *mut GError;
    fn g_thread_ref(thread: *mut GThread) -> *mut GThread;
    fn g_thread_unref(thread: *mut GThread);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_time_zone_ref(tz: *mut GTimeZone) -> *mut GTimeZone;
    fn g_time_zone_unref(tz: *mut GTimeZone);
    fn g_date_time_unref(datetime: *mut GDateTime);
    fn g_date_time_ref(datetime: *mut GDateTime) -> *mut GDateTime;
    fn g_bookmark_file_free(bookmark: *mut GBookmarkFile);
    fn g_bookmark_file_copy(bookmark: *mut GBookmarkFile) -> *mut GBookmarkFile;
    fn g_bytes_ref(bytes: *mut GBytes) -> *mut GBytes;
    fn g_bytes_unref(bytes: *mut GBytes);
    fn g_checksum_copy(checksum: *const GChecksum) -> *mut GChecksum;
    fn g_checksum_free(checksum: *mut GChecksum);
    fn g_date_free(date: *mut GDate);
    fn g_date_copy(date: *const GDate) -> *mut GDate;
    fn g_dir_ref(dir: *mut GDir) -> *mut GDir;
    fn g_dir_unref(dir: *mut GDir);
    fn g_free(mem: gpointer);
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_ref(hash_table: *mut GHashTable) -> *mut GHashTable;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_hmac_ref(hmac: *mut GHmac) -> *mut GHmac;
    fn g_hmac_unref(hmac: *mut GHmac);
    fn g_main_context_ref(context: *mut GMainContext) -> *mut GMainContext;
    fn g_main_context_unref(context: *mut GMainContext);
    fn g_main_loop_ref(loop_0: *mut GMainLoop) -> *mut GMainLoop;
    fn g_main_loop_unref(loop_0: *mut GMainLoop);
    fn g_source_ref(source: *mut GSource) -> *mut GSource;
    fn g_source_unref(source: *mut GSource);
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_string_new_len(init: *const gchar, len: gssize) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_key_file_ref(key_file: *mut GKeyFile) -> *mut GKeyFile;
    fn g_key_file_unref(key_file: *mut GKeyFile);
    fn g_mapped_file_ref(file: *mut GMappedFile) -> *mut GMappedFile;
    fn g_mapped_file_unref(file: *mut GMappedFile);
    fn g_markup_parse_context_ref(context: *mut GMarkupParseContext) -> *mut GMarkupParseContext;
    fn g_markup_parse_context_unref(context: *mut GMarkupParseContext);
    fn g_variant_type_free(type_0: *mut GVariantType);
    fn g_variant_type_copy(type_0: *const GVariantType) -> *mut GVariantType;
    fn g_variant_builder_unref(builder: *mut GVariantBuilder);
    fn g_variant_builder_ref(builder: *mut GVariantBuilder) -> *mut GVariantBuilder;
    fn g_variant_dict_ref(dict: *mut GVariantDict) -> *mut GVariantDict;
    fn g_variant_dict_unref(dict: *mut GVariantDict);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_option_group_ref(group: *mut GOptionGroup) -> *mut GOptionGroup;
    fn g_option_group_unref(group: *mut GOptionGroup);
    fn g_pattern_spec_free(pspec: *mut GPatternSpec);
    fn g_pattern_spec_copy(pspec: *mut GPatternSpec) -> *mut GPatternSpec;
    fn g_rand_free(rand_: *mut GRand);
    fn g_rand_copy(rand_: *mut GRand) -> *mut GRand;
    fn g_regex_ref(regex: *mut GRegex) -> *mut GRegex;
    fn g_regex_unref(regex: *mut GRegex);
    fn g_match_info_ref(match_info: *mut GMatchInfo) -> *mut GMatchInfo;
    fn g_match_info_unref(match_info: *mut GMatchInfo);
    fn g_strv_builder_unref(builder: *mut GStrvBuilder);
    fn g_strv_builder_ref(builder: *mut GStrvBuilder) -> *mut GStrvBuilder;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_tree_ref(tree: *mut GTree) -> *mut GTree;
    fn g_tree_unref(tree: *mut GTree);
    fn g_uri_ref(uri: *mut GUri) -> *mut GUri;
    fn g_uri_unref(uri: *mut GUri);
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_from_name(name: *const gchar) -> GType;
    fn g_type_register_static(
        parent_type: GType,
        type_name: *const gchar,
        info: *const GTypeInfo,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_register_fundamental(
        type_id: GType,
        type_name: *const gchar,
        info: *const GTypeInfo,
        finfo: *const GTypeFundamentalInfo,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_value_table_peek(type_0: GType) -> *mut GTypeValueTable;
    fn g_type_check_is_value_type(type_0: GType) -> gboolean;
    fn g_type_check_value_holds(value: *const GValue, type_0: GType) -> gboolean;
    fn g_type_test_flags(type_0: GType, flags: guint) -> gboolean;
    fn g_closure_ref(closure: *mut GClosure) -> *mut GClosure;
    fn g_closure_unref(closure: *mut GClosure);
    fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;
    fn g_value_copy(src_value: *const GValue, dest_value: *mut GValue);
    fn g_value_reset(value: *mut GValue) -> *mut GValue;
    fn g_value_unset(value: *mut GValue);
    fn _g_type_boxed_copy(type_0: GType, value: gpointer) -> gpointer;
    fn _g_type_boxed_free(type_0: GType, value: gpointer);
    fn _g_type_boxed_init(type_0: GType, copy_func: GBoxedCopyFunc, free_func: GBoxedFreeFunc);
    fn g_value_array_free(value_array: *mut GValueArray);
    fn g_value_array_copy(value_array: *const GValueArray) -> *mut GValueArray;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gushort = ::core::ffi::c_ushort;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GBytes = _GBytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GArray {
    pub data: *mut gchar,
    pub len: guint,
}
pub type GArray = _GArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GByteArray {
    pub data: *mut guint8,
    pub len: guint,
}
pub type GByteArray = _GByteArray;
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
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GThreadFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GThread {
    pub func: GThreadFunc,
    pub data: gpointer,
    pub joinable: gboolean,
    pub priority: GThreadPriority,
}
pub type GThreadPriority = ::core::ffi::c_uint;
pub const G_THREAD_PRIORITY_URGENT: GThreadPriority = 3;
pub const G_THREAD_PRIORITY_HIGH: GThreadPriority = 2;
pub const G_THREAD_PRIORITY_NORMAL: GThreadPriority = 1;
pub const G_THREAD_PRIORITY_LOW: GThreadPriority = 0;
pub type GThread = _GThread;
pub type GTimeZone = _GTimeZone;
pub type GDateTime = _GDateTime;
pub type GBookmarkFile = _GBookmarkFile;
pub type GChecksum = _GChecksum;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GDate {
    #[bitfield(name = "julian_days", ty = "guint", bits = "0..=31")]
    #[bitfield(name = "julian", ty = "guint", bits = "32..=32")]
    #[bitfield(name = "dmy", ty = "guint", bits = "33..=33")]
    #[bitfield(name = "day", ty = "guint", bits = "34..=39")]
    #[bitfield(name = "month", ty = "guint", bits = "40..=43")]
    #[bitfield(name = "year", ty = "guint", bits = "44..=59")]
    pub julian_days_julian_dmy_day_month_year: [u8; 8],
}
pub type GDate = _GDate;
pub type GDir = _GDir;
pub type GHashTable = _GHashTable;
pub type GHmac = _GHmac;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPollFD {
    pub fd: gint,
    pub events: gushort,
    pub revents: gushort,
}
pub type GPollFD = _GPollFD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub type GMainContext = _GMainContext;
pub type GMainLoop = _GMainLoop;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSource {
    pub callback_data: gpointer,
    pub callback_funcs: *mut GSourceCallbackFuncs,
    pub source_funcs: *const GSourceFuncs,
    pub ref_count: guint,
    pub context: *mut GMainContext,
    pub priority: gint,
    pub flags: guint,
    pub source_id: guint,
    pub poll_fds: *mut GSList,
    pub prev: *mut GSource,
    pub next: *mut GSource,
    pub name: *mut ::core::ffi::c_char,
    pub priv_0: *mut GSourcePrivate,
}
pub type GSourcePrivate = _GSourcePrivate;
pub type GSource = _GSource;
pub type GSourceFuncs = _GSourceFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceFuncs {
    pub prepare: Option<unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean>,
    pub check: Option<unsafe extern "C" fn(*mut GSource) -> gboolean>,
    pub dispatch: Option<unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean>,
    pub finalize: Option<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub closure_callback: GSourceFunc,
    pub closure_marshal: GSourceDummyMarshal,
}
pub type GSourceDummyMarshal = Option<unsafe extern "C" fn() -> ()>;
pub type GSourceFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSourceCallbackFuncs {
    pub ref_0: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub unref: Option<unsafe extern "C" fn(gpointer) -> ()>,
    pub get:
        Option<unsafe extern "C" fn(gpointer, *mut GSource, *mut GSourceFunc, *mut gpointer) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GKeyFile = _GKeyFile;
pub type GMappedFile = _GMappedFile;
pub type GMarkupParseContext = _GMarkupParseContext;
pub type GVariantType = _GVariantType;
pub type GVariant = _GVariant;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVariantDict {
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: C2RustUnnamed_2,
    pub x: [guintptr; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub asv: *mut GVariant,
    pub partial_magic: gsize,
    pub y: [guintptr; 14],
}
pub type GVariantDict = _GVariantDict;
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
pub type GOptionGroup = _GOptionGroup;
pub type GPatternSpec = _GPatternSpec;
pub type GRand = _GRand;
pub type GRegex = _GRegex;
pub type GMatchInfo = _GMatchInfo;
pub type GStrvBuilder = _GStrvBuilder;
pub type GTree = _GTree;
pub type GUri = _GUri;
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed_3; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInfo {
    pub class_size: guint16,
    pub base_init: GBaseInitFunc,
    pub base_finalize: GBaseFinalizeFunc,
    pub class_init: GClassInitFunc,
    pub class_finalize: GClassFinalizeFunc,
    pub class_data: gconstpointer,
    pub instance_size: guint16,
    pub n_preallocs: guint16,
    pub instance_init: GInstanceInitFunc,
    pub value_table: *const GTypeValueTable,
}
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
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassFinalizeFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GBaseFinalizeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GBaseInitFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GTypeInfo = _GTypeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeFundamentalInfo {
    pub type_flags: GTypeFundamentalFlags,
}
pub type GTypeFundamentalFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEEP_DERIVABLE: GTypeFundamentalFlags = 8;
pub const G_TYPE_FLAG_DERIVABLE: GTypeFundamentalFlags = 4;
pub const G_TYPE_FLAG_INSTANTIATABLE: GTypeFundamentalFlags = 2;
pub const G_TYPE_FLAG_CLASSED: GTypeFundamentalFlags = 1;
pub type GTypeFundamentalInfo = _GTypeFundamentalInfo;
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GDate) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
pub type GBoxedFreeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GDate) -> *mut GDate>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GDate) -> *mut GDate>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
pub type GBoxedCopyFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GString) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GString) -> *mut GString>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GString) -> *mut GString>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GHashTable) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GHashTable) -> *mut GHashTable>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GHashTable) -> *mut GHashTable>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GArray) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GArray) -> *mut GArray>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GArray) -> *mut GArray>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GByteArray) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GByteArray) -> *mut GByteArray>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GByteArray) -> *mut GByteArray>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GPtrArray) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GPtrArray) -> *mut GPtrArray>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GPtrArray) -> *mut GPtrArray>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GBytes) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GBytes) -> *mut GBytes>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GBytes) -> *mut GBytes>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GVariantType) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_19 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GVariantType) -> *mut GVariantType>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GVariantType) -> *mut GVariantType>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GRegex) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GRegex) -> *mut GRegex>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GRegex) -> *mut GRegex>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GMatchInfo) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GMatchInfo) -> *mut GMatchInfo>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GMatchInfo) -> *mut GMatchInfo>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GError) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GError) -> *mut GError>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GError) -> *mut GError>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_26 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GDateTime) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_27 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GDateTime) -> *mut GDateTime>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GDateTime) -> *mut GDateTime>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_28 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GTimeZone) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_29 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GTimeZone) -> *mut GTimeZone>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GTimeZone) -> *mut GTimeZone>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_30 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GVariantBuilder) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_31 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GVariantBuilder) -> *mut GVariantBuilder>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GVariantBuilder) -> *mut GVariantBuilder>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_32 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GVariantDict) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GVariantDict) -> *mut GVariantDict>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GVariantDict) -> *mut GVariantDict>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_34 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GKeyFile) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_35 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GKeyFile) -> *mut GKeyFile>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GKeyFile) -> *mut GKeyFile>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_36 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GMainLoop) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_37 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GMainLoop) -> *mut GMainLoop>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GMainLoop) -> *mut GMainLoop>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_38 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GMainContext) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_39 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GMainContext) -> *mut GMainContext>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GMainContext) -> *mut GMainContext>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_40 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GSource) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_41 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GSource) -> *mut GSource>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GSource) -> *mut GSource>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_42 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GPollFD) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_43 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GPollFD) -> *mut GPollFD>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GPollFD) -> *mut GPollFD>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_44 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GThread) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_45 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GThread) -> *mut GThread>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GThread) -> *mut GThread>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_46 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GChecksum) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_47 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GChecksum) -> *mut GChecksum>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GChecksum) -> *mut GChecksum>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_48 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GMarkupParseContext) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_49 {
    pub do_copy_type:
        Option<unsafe extern "C" fn(*mut GMarkupParseContext) -> *mut GMarkupParseContext>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GMarkupParseContext) -> *mut GMarkupParseContext>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_50 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GMappedFile) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_51 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GMappedFile) -> *mut GMappedFile>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GMappedFile) -> *mut GMappedFile>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_52 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GOptionGroup) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_53 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GOptionGroup) -> *mut GOptionGroup>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GOptionGroup) -> *mut GOptionGroup>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_54 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GUri) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_55 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GUri) -> *mut GUri>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GUri) -> *mut GUri>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_56 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GTree) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_57 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GTree) -> *mut GTree>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GTree) -> *mut GTree>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_58 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GPatternSpec) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_59 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GPatternSpec) -> *mut GPatternSpec>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GPatternSpec) -> *mut GPatternSpec>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_60 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GBookmarkFile) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_61 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GBookmarkFile) -> *mut GBookmarkFile>,
    pub do_const_copy_type:
        Option<unsafe extern "C" fn(*const GBookmarkFile) -> *mut GBookmarkFile>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_62 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GHmac) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_63 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GHmac) -> *mut GHmac>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GHmac) -> *mut GHmac>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_64 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GDir) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_65 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GDir) -> *mut GDir>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GDir) -> *mut GDir>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_66 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GRand) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_67 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GRand) -> *mut GRand>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GRand) -> *mut GRand>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_68 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GStrvBuilder) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_69 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GStrvBuilder) -> *mut GStrvBuilder>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GStrvBuilder) -> *mut GStrvBuilder>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
pub type GClosure = _GClosure;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_70 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GClosure) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_71 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GClosure) -> *mut GClosure>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GClosure) -> *mut GClosure>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_72 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GValue) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_73 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GValue) -> *mut GValue>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GValue) -> *mut GValue>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValueArray {
    pub n_values: guint,
    pub values: *mut GValue,
    pub n_prealloced: guint,
}
pub type GValueArray = _GValueArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_74 {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GValueArray) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_75 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GValueArray) -> *mut GValueArray>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GValueArray) -> *mut GValueArray>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
#[inline]
unsafe extern "C" fn value_meminit(mut value: *mut GValue, mut value_type: GType) {
    (*value).g_type = value_type;
    memset(
        &raw mut (*value).data as *mut C2RustUnnamed_3 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[C2RustUnnamed_3; 2]>() as size_t,
    );
}
unsafe extern "C" fn value_copy(mut src_value: *mut GValue) -> *mut GValue {
    let mut dest_value: *mut GValue =
        g_malloc0_n(1 as gsize, ::core::mem::size_of::<GValue>() as gsize) as *mut GValue;
    if (*src_value).g_type != 0 {
        g_value_init(dest_value, (*src_value).g_type);
        g_value_copy(src_value, dest_value);
    }
    return dest_value;
}
unsafe extern "C" fn value_free(mut value: *mut GValue) {
    if (*value).g_type != 0 {
        g_value_unset(value);
    }
    g_free(value as gpointer);
}
unsafe extern "C" fn pollfd_copy(mut src: *mut GPollFD) -> *mut GPollFD {
    let mut dest: *mut GPollFD =
        g_malloc0_n(1 as gsize, ::core::mem::size_of::<GPollFD>() as gsize) as *mut GPollFD;
    memcpy(
        dest as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<GPollFD>() as size_t,
    );
    return dest;
}
#[no_mangle]
pub unsafe extern "C" fn _g_boxed_type_init() {
    let info: GTypeInfo = _GTypeInfo {
        class_size: 0 as guint16,
        base_init: None,
        base_finalize: None,
        class_init: None,
        class_finalize: None,
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: 0 as guint16,
        n_preallocs: 0 as guint16,
        instance_init: None,
        value_table: ::core::ptr::null::<GTypeValueTable>(),
    };
    let finfo: GTypeFundamentalInfo = _GTypeFundamentalInfo {
        type_flags: G_TYPE_FLAG_DERIVABLE,
    };
    let mut type_0: GType = 0;
    type_0 = g_type_register_fundamental(
        ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GBoxed\0" as *const u8 as *const gchar),
        &raw const info,
        &raw const finfo,
        (G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int
            | G_TYPE_FLAG_VALUE_ABSTRACT as ::core::ffi::c_int) as GTypeFlags,
    );
    if type_0 == ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gboxed.c\0" as *const u8
                as *const ::core::ffi::c_char,
            96 as ::core::ffi::c_int,
            b"_g_boxed_type_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"type == G_TYPE_BOXED\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn gstring_copy(mut src_gstring: *mut GString) -> *mut GString {
    return g_string_new_len((*src_gstring).str_0, (*src_gstring).len as gssize);
}
unsafe extern "C" fn gstring_free(mut gstring: *mut GString) {
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                gstring,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(gstring);
        };
    } else {
        g_string_free(
            gstring,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_closure_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_closure_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_71, C2RustUnnamed_70) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_71, C2RustUnnamed_70) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GClosure\0" as *const u8 as *const gchar),
        C2RustUnnamed_71 {
            do_copy_type: Some(
                g_closure_ref as unsafe extern "C" fn(*mut GClosure) -> *mut GClosure,
            ),
        },
        C2RustUnnamed_70 {
            do_free_type: Some(g_closure_unref as unsafe extern "C" fn(*mut GClosure) -> ()),
        },
    );
    return g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_value_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_73, C2RustUnnamed_72) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_73, C2RustUnnamed_72) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GValue\0" as *const u8 as *const gchar),
        C2RustUnnamed_73 {
            do_copy_type: Some(value_copy as unsafe extern "C" fn(*mut GValue) -> *mut GValue),
        },
        C2RustUnnamed_72 {
            do_free_type: Some(value_free as unsafe extern "C" fn(*mut GValue) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_value_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_value_array_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_75, C2RustUnnamed_74) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_75, C2RustUnnamed_74) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GValueArray\0" as *const u8 as *const gchar),
        C2RustUnnamed_75 {
            do_const_copy_type: Some(
                g_value_array_copy as unsafe extern "C" fn(*const GValueArray) -> *mut GValueArray,
            ),
        },
        C2RustUnnamed_74 {
            do_free_type: Some(g_value_array_free as unsafe extern "C" fn(*mut GValueArray) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_array_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_value_array_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_date_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_5, C2RustUnnamed_4) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_5, C2RustUnnamed_4) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GDate\0" as *const u8 as *const gchar),
        C2RustUnnamed_5 {
            do_const_copy_type: Some(
                g_date_copy as unsafe extern "C" fn(*const GDate) -> *mut GDate,
            ),
        },
        C2RustUnnamed_4 {
            do_free_type: Some(g_date_free as unsafe extern "C" fn(*mut GDate) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_date_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_date_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_gstring_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_7, C2RustUnnamed_6) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_7, C2RustUnnamed_6) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GString\0" as *const u8 as *const gchar),
        C2RustUnnamed_7 {
            do_copy_type: Some(gstring_copy as unsafe extern "C" fn(*mut GString) -> *mut GString),
        },
        C2RustUnnamed_6 {
            do_free_type: Some(gstring_free as unsafe extern "C" fn(*mut GString) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_gstring_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_gstring_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_hash_table_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_9, C2RustUnnamed_8) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_9, C2RustUnnamed_8) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GHashTable\0" as *const u8 as *const gchar),
        C2RustUnnamed_9 {
            do_copy_type: Some(
                g_hash_table_ref as unsafe extern "C" fn(*mut GHashTable) -> *mut GHashTable,
            ),
        },
        C2RustUnnamed_8 {
            do_free_type: Some(g_hash_table_unref as unsafe extern "C" fn(*mut GHashTable) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_hash_table_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_hash_table_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_array_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_array_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_array_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_11, C2RustUnnamed_10) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_11, C2RustUnnamed_10) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GArray\0" as *const u8 as *const gchar),
        C2RustUnnamed_11 {
            do_copy_type: Some(g_array_ref as unsafe extern "C" fn(*mut GArray) -> *mut GArray),
        },
        C2RustUnnamed_10 {
            do_free_type: Some(g_array_unref as unsafe extern "C" fn(*mut GArray) -> ()),
        },
    );
    return g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_ptr_array_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_15, C2RustUnnamed_14) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_15, C2RustUnnamed_14) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GPtrArray\0" as *const u8 as *const gchar),
        C2RustUnnamed_15 {
            do_copy_type: Some(
                g_ptr_array_ref as unsafe extern "C" fn(*mut GPtrArray) -> *mut GPtrArray,
            ),
        },
        C2RustUnnamed_14 {
            do_free_type: Some(g_ptr_array_unref as unsafe extern "C" fn(*mut GPtrArray) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_ptr_array_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_ptr_array_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_byte_array_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_13, C2RustUnnamed_12) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_13, C2RustUnnamed_12) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GByteArray\0" as *const u8 as *const gchar),
        C2RustUnnamed_13 {
            do_copy_type: Some(
                g_byte_array_ref as unsafe extern "C" fn(*mut GByteArray) -> *mut GByteArray,
            ),
        },
        C2RustUnnamed_12 {
            do_free_type: Some(g_byte_array_unref as unsafe extern "C" fn(*mut GByteArray) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_byte_array_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_byte_array_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_bytes_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_17, C2RustUnnamed_16) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_17, C2RustUnnamed_16) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GBytes\0" as *const u8 as *const gchar),
        C2RustUnnamed_17 {
            do_copy_type: Some(g_bytes_ref as unsafe extern "C" fn(*mut GBytes) -> *mut GBytes),
        },
        C2RustUnnamed_16 {
            do_free_type: Some(g_bytes_unref as unsafe extern "C" fn(*mut GBytes) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_bytes_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_bytes_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_tree_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_tree_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_tree_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_57, C2RustUnnamed_56) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_57, C2RustUnnamed_56) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GTree\0" as *const u8 as *const gchar),
        C2RustUnnamed_57 {
            do_copy_type: Some(g_tree_ref as unsafe extern "C" fn(*mut GTree) -> *mut GTree),
        },
        C2RustUnnamed_56 {
            do_free_type: Some(g_tree_unref as unsafe extern "C" fn(*mut GTree) -> ()),
        },
    );
    return g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_regex_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_21, C2RustUnnamed_20) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_21, C2RustUnnamed_20) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GRegex\0" as *const u8 as *const gchar),
        C2RustUnnamed_21 {
            do_copy_type: Some(g_regex_ref as unsafe extern "C" fn(*mut GRegex) -> *mut GRegex),
        },
        C2RustUnnamed_20 {
            do_free_type: Some(g_regex_unref as unsafe extern "C" fn(*mut GRegex) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_regex_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_regex_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_match_info_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_23, C2RustUnnamed_22) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_23, C2RustUnnamed_22) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GMatchInfo\0" as *const u8 as *const gchar),
        C2RustUnnamed_23 {
            do_copy_type: Some(
                g_match_info_ref as unsafe extern "C" fn(*mut GMatchInfo) -> *mut GMatchInfo,
            ),
        },
        C2RustUnnamed_22 {
            do_free_type: Some(g_match_info_unref as unsafe extern "C" fn(*mut GMatchInfo) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_match_info_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_match_info_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_variant_type_get_gtype() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_variant_type_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_variant_type_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_19, C2RustUnnamed_18) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_19, C2RustUnnamed_18) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GVariantType\0" as *const u8 as *const gchar),
        C2RustUnnamed_19 {
            do_const_copy_type: Some(
                g_variant_type_copy
                    as unsafe extern "C" fn(*const GVariantType) -> *mut GVariantType,
            ),
        },
        C2RustUnnamed_18 {
            do_free_type: Some(
                g_variant_type_free as unsafe extern "C" fn(*mut GVariantType) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_variant_builder_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_variant_builder_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_variant_builder_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_31, C2RustUnnamed_30) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_31, C2RustUnnamed_30) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GVariantBuilder\0" as *const u8 as *const gchar),
        C2RustUnnamed_31 {
            do_copy_type: Some(
                g_variant_builder_ref
                    as unsafe extern "C" fn(*mut GVariantBuilder) -> *mut GVariantBuilder,
            ),
        },
        C2RustUnnamed_30 {
            do_free_type: Some(
                g_variant_builder_unref as unsafe extern "C" fn(*mut GVariantBuilder) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_variant_dict_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_variant_dict_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_variant_dict_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_33, C2RustUnnamed_32) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_33, C2RustUnnamed_32) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GVariantDict\0" as *const u8 as *const gchar),
        C2RustUnnamed_33 {
            do_copy_type: Some(
                g_variant_dict_ref as unsafe extern "C" fn(*mut GVariantDict) -> *mut GVariantDict,
            ),
        },
        C2RustUnnamed_32 {
            do_free_type: Some(
                g_variant_dict_unref as unsafe extern "C" fn(*mut GVariantDict) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_error_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_25, C2RustUnnamed_24) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_25, C2RustUnnamed_24) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GError\0" as *const u8 as *const gchar),
        C2RustUnnamed_25 {
            do_const_copy_type: Some(
                g_error_copy as unsafe extern "C" fn(*const GError) -> *mut GError,
            ),
        },
        C2RustUnnamed_24 {
            do_free_type: Some(g_error_free as unsafe extern "C" fn(*mut GError) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_error_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_error_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_date_time_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_date_time_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_date_time_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_27, C2RustUnnamed_26) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_27, C2RustUnnamed_26) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GDateTime\0" as *const u8 as *const gchar),
        C2RustUnnamed_27 {
            do_copy_type: Some(
                g_date_time_ref as unsafe extern "C" fn(*mut GDateTime) -> *mut GDateTime,
            ),
        },
        C2RustUnnamed_26 {
            do_free_type: Some(g_date_time_unref as unsafe extern "C" fn(*mut GDateTime) -> ()),
        },
    );
    return g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_time_zone_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_29, C2RustUnnamed_28) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_29, C2RustUnnamed_28) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GTimeZone\0" as *const u8 as *const gchar),
        C2RustUnnamed_29 {
            do_copy_type: Some(
                g_time_zone_ref as unsafe extern "C" fn(*mut GTimeZone) -> *mut GTimeZone,
            ),
        },
        C2RustUnnamed_28 {
            do_free_type: Some(g_time_zone_unref as unsafe extern "C" fn(*mut GTimeZone) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_time_zone_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_time_zone_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_key_file_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_key_file_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_key_file_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_35, C2RustUnnamed_34) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_35, C2RustUnnamed_34) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GKeyFile\0" as *const u8 as *const gchar),
        C2RustUnnamed_35 {
            do_copy_type: Some(
                g_key_file_ref as unsafe extern "C" fn(*mut GKeyFile) -> *mut GKeyFile,
            ),
        },
        C2RustUnnamed_34 {
            do_free_type: Some(g_key_file_unref as unsafe extern "C" fn(*mut GKeyFile) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_mapped_file_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_mapped_file_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_mapped_file_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_51, C2RustUnnamed_50) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_51, C2RustUnnamed_50) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GMappedFile\0" as *const u8 as *const gchar),
        C2RustUnnamed_51 {
            do_copy_type: Some(
                g_mapped_file_ref as unsafe extern "C" fn(*mut GMappedFile) -> *mut GMappedFile,
            ),
        },
        C2RustUnnamed_50 {
            do_free_type: Some(g_mapped_file_unref as unsafe extern "C" fn(*mut GMappedFile) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_bookmark_file_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_bookmark_file_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_bookmark_file_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_61, C2RustUnnamed_60) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_61, C2RustUnnamed_60) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GBookmarkFile\0" as *const u8 as *const gchar),
        C2RustUnnamed_61 {
            do_copy_type: Some(
                g_bookmark_file_copy
                    as unsafe extern "C" fn(*mut GBookmarkFile) -> *mut GBookmarkFile,
            ),
        },
        C2RustUnnamed_60 {
            do_free_type: Some(
                g_bookmark_file_free as unsafe extern "C" fn(*mut GBookmarkFile) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_hmac_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_63, C2RustUnnamed_62) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_63, C2RustUnnamed_62) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GHmac\0" as *const u8 as *const gchar),
        C2RustUnnamed_63 {
            do_copy_type: Some(g_hmac_ref as unsafe extern "C" fn(*mut GHmac) -> *mut GHmac),
        },
        C2RustUnnamed_62 {
            do_free_type: Some(g_hmac_unref as unsafe extern "C" fn(*mut GHmac) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_hmac_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_hmac_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_dir_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_dir_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_dir_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_65, C2RustUnnamed_64) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_65, C2RustUnnamed_64) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GDir\0" as *const u8 as *const gchar),
        C2RustUnnamed_65 {
            do_copy_type: Some(g_dir_ref as unsafe extern "C" fn(*mut GDir) -> *mut GDir),
        },
        C2RustUnnamed_64 {
            do_free_type: Some(g_dir_unref as unsafe extern "C" fn(*mut GDir) -> ()),
        },
    );
    return g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_rand_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_67, C2RustUnnamed_66) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_67, C2RustUnnamed_66) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GRand\0" as *const u8 as *const gchar),
        C2RustUnnamed_67 {
            do_copy_type: Some(g_rand_copy as unsafe extern "C" fn(*mut GRand) -> *mut GRand),
        },
        C2RustUnnamed_66 {
            do_free_type: Some(g_rand_free as unsafe extern "C" fn(*mut GRand) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_rand_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_rand_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_main_loop_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_main_loop_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_main_loop_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_37, C2RustUnnamed_36) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_37, C2RustUnnamed_36) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GMainLoop\0" as *const u8 as *const gchar),
        C2RustUnnamed_37 {
            do_copy_type: Some(
                g_main_loop_ref as unsafe extern "C" fn(*mut GMainLoop) -> *mut GMainLoop,
            ),
        },
        C2RustUnnamed_36 {
            do_free_type: Some(g_main_loop_unref as unsafe extern "C" fn(*mut GMainLoop) -> ()),
        },
    );
    return g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_main_context_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_39, C2RustUnnamed_38) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_39, C2RustUnnamed_38) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GMainContext\0" as *const u8 as *const gchar),
        C2RustUnnamed_39 {
            do_copy_type: Some(
                g_main_context_ref as unsafe extern "C" fn(*mut GMainContext) -> *mut GMainContext,
            ),
        },
        C2RustUnnamed_38 {
            do_free_type: Some(
                g_main_context_unref as unsafe extern "C" fn(*mut GMainContext) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_main_context_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_main_context_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_source_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_source_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_source_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_41, C2RustUnnamed_40) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_41, C2RustUnnamed_40) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GSource\0" as *const u8 as *const gchar),
        C2RustUnnamed_41 {
            do_copy_type: Some(g_source_ref as unsafe extern "C" fn(*mut GSource) -> *mut GSource),
        },
        C2RustUnnamed_40 {
            do_free_type: Some(g_source_unref as unsafe extern "C" fn(*mut GSource) -> ()),
        },
    );
    return g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_pollfd_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_43, C2RustUnnamed_42) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_43, C2RustUnnamed_42) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GPollFD\0" as *const u8 as *const gchar),
        C2RustUnnamed_43 {
            do_copy_type: Some(pollfd_copy as unsafe extern "C" fn(*mut GPollFD) -> *mut GPollFD),
        },
        C2RustUnnamed_42 {
            do_free_boxed: Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_pollfd_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_pollfd_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_markup_parse_context_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_49, C2RustUnnamed_48) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_49, C2RustUnnamed_48) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GMarkupParseContext\0" as *const u8 as *const gchar),
        C2RustUnnamed_49 {
            do_copy_type: Some(
                g_markup_parse_context_ref
                    as unsafe extern "C" fn(*mut GMarkupParseContext) -> *mut GMarkupParseContext,
            ),
        },
        C2RustUnnamed_48 {
            do_free_type: Some(
                g_markup_parse_context_unref
                    as unsafe extern "C" fn(*mut GMarkupParseContext) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_markup_parse_context_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_markup_parse_context_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_thread_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_45, C2RustUnnamed_44) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_45, C2RustUnnamed_44) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GThread\0" as *const u8 as *const gchar),
        C2RustUnnamed_45 {
            do_copy_type: Some(g_thread_ref as unsafe extern "C" fn(*mut GThread) -> *mut GThread),
        },
        C2RustUnnamed_44 {
            do_free_type: Some(g_thread_unref as unsafe extern "C" fn(*mut GThread) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_thread_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_thread_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_checksum_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_47, C2RustUnnamed_46) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_47, C2RustUnnamed_46) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GChecksum\0" as *const u8 as *const gchar),
        C2RustUnnamed_47 {
            do_const_copy_type: Some(
                g_checksum_copy as unsafe extern "C" fn(*const GChecksum) -> *mut GChecksum,
            ),
        },
        C2RustUnnamed_46 {
            do_free_type: Some(g_checksum_free as unsafe extern "C" fn(*mut GChecksum) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_checksum_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_checksum_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_uri_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_55, C2RustUnnamed_54) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_55, C2RustUnnamed_54) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GUri\0" as *const u8 as *const gchar),
        C2RustUnnamed_55 {
            do_copy_type: Some(g_uri_ref as unsafe extern "C" fn(*mut GUri) -> *mut GUri),
        },
        C2RustUnnamed_54 {
            do_free_type: Some(g_uri_unref as unsafe extern "C" fn(*mut GUri) -> ()),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_uri_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_uri_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_option_group_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_option_group_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_option_group_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_53, C2RustUnnamed_52) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_53, C2RustUnnamed_52) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GOptionGroup\0" as *const u8 as *const gchar),
        C2RustUnnamed_53 {
            do_copy_type: Some(
                g_option_group_ref as unsafe extern "C" fn(*mut GOptionGroup) -> *mut GOptionGroup,
            ),
        },
        C2RustUnnamed_52 {
            do_free_type: Some(
                g_option_group_unref as unsafe extern "C" fn(*mut GOptionGroup) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_pattern_spec_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_pattern_spec_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_pattern_spec_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_59, C2RustUnnamed_58) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_59, C2RustUnnamed_58) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GPatternSpec\0" as *const u8 as *const gchar),
        C2RustUnnamed_59 {
            do_copy_type: Some(
                g_pattern_spec_copy as unsafe extern "C" fn(*mut GPatternSpec) -> *mut GPatternSpec,
            ),
        },
        C2RustUnnamed_58 {
            do_free_type: Some(
                g_pattern_spec_free as unsafe extern "C" fn(*mut GPatternSpec) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_strv_builder_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_strv_builder_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn g_strv_builder_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_69, C2RustUnnamed_68) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_69, C2RustUnnamed_68) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GStrvBuilder\0" as *const u8 as *const gchar),
        C2RustUnnamed_69 {
            do_copy_type: Some(
                g_strv_builder_ref as unsafe extern "C" fn(*mut GStrvBuilder) -> *mut GStrvBuilder,
            ),
        },
        C2RustUnnamed_68 {
            do_free_type: Some(
                g_strv_builder_unref as unsafe extern "C" fn(*mut GStrvBuilder) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_strv_get_type() -> GType {
    static mut static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = g_boxed_type_register_static(
            g_intern_static_string(b"GStrv\0" as *const u8 as *const gchar),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut *mut gchar) -> *mut *mut gchar>,
                GBoxedCopyFunc,
            >(Some(
                g_strdupv as unsafe extern "C" fn(*mut *mut gchar) -> *mut *mut gchar,
            )),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut *mut gchar) -> ()>,
                GBoxedFreeFunc,
            >(Some(
                g_strfreev as unsafe extern "C" fn(*mut *mut gchar) -> (),
            )),
        );
        if 0 as ::core::ffi::c_int != 0 {
            static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return static_g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn g_variant_get_gtype() -> GType {
    return ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
}
unsafe extern "C" fn boxed_proxy_value_init(mut value: *mut GValue) {
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
        ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
}
unsafe extern "C" fn boxed_proxy_value_free(mut value: *mut GValue) {
    if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
        && (*value).data[1 as ::core::ffi::c_int as usize].v_uint
            & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
            == 0
    {
        _g_type_boxed_free(
            (*value).g_type,
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer,
        );
    }
}
unsafe extern "C" fn boxed_proxy_value_copy(
    mut src_value: *const GValue,
    mut dest_value: *mut GValue,
) {
    if !(*src_value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer = _g_type_boxed_copy(
            (*(src_value as *mut GValue)).g_type,
            (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer,
        );
    } else {
        (*dest_value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            (*src_value).data[0 as ::core::ffi::c_int as usize].v_pointer;
    };
}
unsafe extern "C" fn boxed_proxy_value_peek_pointer(mut value: *const GValue) -> gpointer {
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer;
}
unsafe extern "C" fn boxed_proxy_collect_value(
    mut value: *mut GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    if (*collect_values.offset(0 as ::core::ffi::c_int as isize))
        .v_pointer
        .is_null()
    {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    } else if collect_flags & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint != 0
    {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer =
            (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer;
        (*value).data[1 as ::core::ffi::c_int as usize].v_uint =
            ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
    } else {
        (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = _g_type_boxed_copy(
            (*value).g_type,
            (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer,
        );
    }
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn boxed_proxy_lcopy_value(
    mut value: *const GValue,
    mut n_collect_values: guint,
    mut collect_values: *mut GTypeCValue,
    mut collect_flags: guint,
) -> *mut gchar {
    let mut boxed_p: *mut gpointer =
        (*collect_values.offset(0 as ::core::ffi::c_int as isize)).v_pointer as *mut gpointer;
    if !boxed_p.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"boxed_proxy_lcopy_value\0" as *const u8 as *const ::core::ffi::c_char,
            b"boxed_p != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return g_strdup_printf(
            b"value location for '%s' passed as NULL\0" as *const u8 as *const gchar,
            g_type_name((*(value as *mut GValue)).g_type),
        );
    }
    if (*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        *boxed_p = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    } else if collect_flags & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint != 0
    {
        *boxed_p = (*value).data[0 as ::core::ffi::c_int as usize].v_pointer;
    } else {
        *boxed_p = _g_type_boxed_copy(
            (*(value as *mut GValue)).g_type,
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer,
        );
    }
    return ::core::ptr::null_mut::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn g_boxed_type_register_static(
    mut name: *const gchar,
    mut boxed_copy: GBoxedCopyFunc,
    mut boxed_free: GBoxedFreeFunc,
) -> GType {
    static mut vtable: GTypeValueTable = unsafe {
        _GTypeValueTable {
            value_init: Some(boxed_proxy_value_init as unsafe extern "C" fn(*mut GValue) -> ()),
            value_free: Some(boxed_proxy_value_free as unsafe extern "C" fn(*mut GValue) -> ()),
            value_copy: Some(
                boxed_proxy_value_copy as unsafe extern "C" fn(*const GValue, *mut GValue) -> (),
            ),
            value_peek_pointer: Some(
                boxed_proxy_value_peek_pointer as unsafe extern "C" fn(*const GValue) -> gpointer,
            ),
            collect_format: b"p\0" as *const u8 as *const gchar,
            collect_value: Some(
                boxed_proxy_collect_value
                    as unsafe extern "C" fn(
                        *mut GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
            lcopy_format: b"p\0" as *const u8 as *const gchar,
            lcopy_value: Some(
                boxed_proxy_lcopy_value
                    as unsafe extern "C" fn(
                        *const GValue,
                        guint,
                        *mut GTypeCValue,
                        guint,
                    ) -> *mut gchar,
            ),
        }
    };
    let mut type_info: GTypeInfo = _GTypeInfo {
        class_size: 0 as guint16,
        base_init: None,
        base_finalize: None,
        class_init: None,
        class_finalize: None,
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: 0 as guint16,
        n_preallocs: 0 as guint16,
        instance_init: None,
        value_table: &raw const vtable,
    };
    let mut type_0: GType = 0;
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_boxed_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if boxed_copy.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_boxed_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"boxed_copy != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if boxed_free.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_boxed_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"boxed_free != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if g_type_from_name(name) == 0 as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_boxed_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_from_name (name) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    type_0 = g_type_register_static(
        ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        name,
        &raw mut type_info,
        G_TYPE_FLAG_NONE,
    );
    if type_0 != 0 {
        _g_type_boxed_init(type_0, boxed_copy, boxed_free);
    }
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn g_boxed_copy(
    mut boxed_type: GType,
    mut src_boxed: gconstpointer,
) -> gpointer {
    let mut value_table: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
    let mut dest_boxed: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if g_type_fundamental(boxed_type)
        == ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_boxed_copy\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_BOXED (boxed_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if g_type_test_flags(
        boxed_type,
        G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int as guint,
    ) == 0 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_boxed_copy\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_ABSTRACT (boxed_type) == FALSE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !src_boxed.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_boxed_copy\0" as *const u8 as *const ::core::ffi::c_char,
            b"src_boxed != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    value_table = g_type_value_table_peek(boxed_type);
    if !value_table.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gboxed.c\0" as *const u8
                as *const ::core::ffi::c_char,
            339 as ::core::ffi::c_int,
            b"g_boxed_copy\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*value_table).value_copy
        == Some(boxed_proxy_value_copy as unsafe extern "C" fn(*const GValue, *mut GValue) -> ())
    {
        dest_boxed = _g_type_boxed_copy(boxed_type, src_boxed as gpointer);
    } else {
        let mut src_value: GValue = _GValue {
            g_type: 0,
            data: [C2RustUnnamed_3 { v_int: 0 }; 2],
        };
        let mut dest_value: GValue = _GValue {
            g_type: 0,
            data: [C2RustUnnamed_3 { v_int: 0 }; 2],
        };
        value_meminit(&raw mut src_value, boxed_type);
        src_value.data[0 as ::core::ffi::c_int as usize].v_pointer = src_boxed as gpointer;
        src_value.data[1 as ::core::ffi::c_int as usize].v_uint =
            ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint;
        value_meminit(&raw mut dest_value, boxed_type);
        (*value_table)
            .value_copy
            .expect("non-null function pointer")(&raw mut src_value, &raw mut dest_value);
        if dest_value.data[1 as ::core::ffi::c_int as usize].v_ulong != 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"the copy_value() implementation of type '%s' seems to make use of reserved GValue fields\0"
                    as *const u8 as *const gchar,
                g_type_name(boxed_type),
            );
        }
        dest_boxed = dest_value.data[0 as ::core::ffi::c_int as usize].v_pointer;
    }
    return dest_boxed;
}
#[no_mangle]
pub unsafe extern "C" fn g_boxed_free(mut boxed_type: GType, mut boxed: gpointer) {
    let mut value_table: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
    if g_type_fundamental(boxed_type)
        == ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_boxed_free\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_BOXED (boxed_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_type_test_flags(
        boxed_type,
        G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int as guint,
    ) == 0 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_boxed_free\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_ABSTRACT (boxed_type) == FALSE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if !boxed.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_boxed_free\0" as *const u8 as *const ::core::ffi::c_char,
            b"boxed != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value_table = g_type_value_table_peek(boxed_type);
    if !value_table.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gboxed.c\0" as *const u8
                as *const ::core::ffi::c_char,
            396 as ::core::ffi::c_int,
            b"g_boxed_free\0" as *const u8 as *const ::core::ffi::c_char,
            b"value_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*value_table).value_free
        == Some(boxed_proxy_value_free as unsafe extern "C" fn(*mut GValue) -> ())
    {
        _g_type_boxed_free(boxed_type, boxed);
    } else {
        let mut value: GValue = _GValue {
            g_type: 0,
            data: [C2RustUnnamed_3 { v_int: 0 }; 2],
        };
        value_meminit(&raw mut value, boxed_type);
        value.data[0 as ::core::ffi::c_int as usize].v_pointer = boxed;
        (*value_table)
            .value_free
            .expect("non-null function pointer")(&raw mut value);
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_value_get_boxed(mut value: *const GValue) -> gpointer {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_get_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_BOXED (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if g_type_check_is_value_type((*(value as *mut GValue)).g_type) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_get_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_VALUE (G_VALUE_TYPE (value))\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*value).data[0 as ::core::ffi::c_int as usize].v_pointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_dup_boxed(mut value: *const GValue) -> gpointer {
    if ({
        let mut __val: *const GValue = value;
        let mut __t: GType = ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_dup_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_BOXED (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if g_type_check_is_value_type((*(value as *mut GValue)).g_type) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_dup_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_VALUE (G_VALUE_TYPE (value))\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
    {
        g_boxed_copy(
            (*(value as *mut GValue)).g_type,
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer as gconstpointer,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    };
}
#[inline]
unsafe extern "C" fn value_set_boxed_internal(
    mut value: *mut GValue,
    mut boxed: gconstpointer,
    mut need_copy: gboolean,
    mut need_free: gboolean,
) {
    if boxed.is_null() {
        g_value_reset(value);
        return;
    }
    if !(*value).data[0 as ::core::ffi::c_int as usize]
        .v_pointer
        .is_null()
        && (*value).data[1 as ::core::ffi::c_int as usize].v_uint
            & ((1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int) as guint
            == 0
    {
        g_boxed_free(
            (*value).g_type,
            (*value).data[0 as ::core::ffi::c_int as usize].v_pointer,
        );
    }
    (*value).data[1 as ::core::ffi::c_int as usize].v_uint = (if need_free != 0 {
        0 as ::core::ffi::c_int
    } else {
        (1 as ::core::ffi::c_int) << 27 as ::core::ffi::c_int
    }) as guint;
    (*value).data[0 as ::core::ffi::c_int as usize].v_pointer = (if need_copy != 0 {
        g_boxed_copy((*value).g_type, boxed) as *mut ::core::ffi::c_void
    } else {
        boxed as *mut ::core::ffi::c_void
    }) as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_boxed(mut value: *mut GValue, mut boxed: gconstpointer) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_set_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_BOXED (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_type_check_is_value_type((*value).g_type) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_set_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_VALUE (G_VALUE_TYPE (value))\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value_set_boxed_internal(
        value,
        boxed,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_static_boxed(
    mut value: *mut GValue,
    mut boxed: gconstpointer,
) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_set_static_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_BOXED (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_type_check_is_value_type((*value).g_type) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_set_static_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_VALUE (G_VALUE_TYPE (value))\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value_set_boxed_internal(value, boxed, 0 as gboolean, 0 as gboolean);
}
#[no_mangle]
pub unsafe extern "C" fn g_value_set_boxed_take_ownership(
    mut value: *mut GValue,
    mut boxed: gconstpointer,
) {
    g_value_take_boxed(value, boxed);
}
#[no_mangle]
pub unsafe extern "C" fn g_value_take_boxed(mut value: *mut GValue, mut boxed: gconstpointer) {
    if ({
        let mut __val: *const GValue = value as *const GValue;
        let mut __t: GType = ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
        let mut __r: gboolean = 0;
        if __val.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if (*__val).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_value_holds(__val, __t);
        }
        __r
    }) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_take_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_VALUE_HOLDS_BOXED (value)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_type_check_is_value_type((*value).g_type) != 0 {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_value_take_boxed\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_VALUE (G_VALUE_TYPE (value))\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value_set_boxed_internal(
        value,
        boxed,
        0 as gboolean,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
}
