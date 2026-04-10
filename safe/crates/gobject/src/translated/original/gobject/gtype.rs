// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::c2rust_bitfields;
use ::core::arch::asm;
use std::sync::Once;
extern "C" {
    pub type _GHashTable;
    pub type _GTypePlugin;
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
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_quark_from_string(string: *const gchar) -> GQuark;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_quark_to_string(quark: GQuark) -> *const gchar;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_rw_lock_writer_lock(rw_lock: *mut GRWLock);
    fn g_rw_lock_writer_unlock(rw_lock: *mut GRWLock);
    fn g_rw_lock_reader_lock(rw_lock: *mut GRWLock);
    fn g_rw_lock_reader_unlock(rw_lock: *mut GRWLock);
    fn g_rec_mutex_lock(rec_mutex: *mut GRecMutex);
    fn g_rec_mutex_unlock(rec_mutex: *mut GRecMutex);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_free_sized(mem: gpointer, size: size_t);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_memdup2(mem: gconstpointer, byte_size: gsize) -> gpointer;
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
    fn g_type_plugin_get_type() -> GType;
    fn g_type_plugin_use(plugin: *mut GTypePlugin);
    fn g_type_plugin_unuse(plugin: *mut GTypePlugin);
    fn g_type_plugin_complete_type_info(
        plugin: *mut GTypePlugin,
        g_type: GType,
        info: *mut GTypeInfo,
        value_table: *mut GTypeValueTable,
    );
    fn g_type_plugin_complete_interface_info(
        plugin: *mut GTypePlugin,
        instance_type: GType,
        interface_type: GType,
        info: *mut GInterfaceInfo,
    );
    fn _g_atomic_array_init(array: *mut GAtomicArray);
    fn _g_atomic_array_copy(
        array: *mut GAtomicArray,
        header_size: gsize,
        additional_element_size: gsize,
    ) -> gpointer;
    fn _g_atomic_array_update(array: *mut GAtomicArray, new_data: gpointer);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
}
pub type uintptr_t = usize;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const VG_USERREQ__INNER_THREADS: C2RustUnnamed = 6402;
pub const VG_USERREQ__VEX_INIT_FOR_IRI: C2RustUnnamed = 6401;
pub const VG_USERREQ__CHANGE_ERR_DISABLEMENT: C2RustUnnamed = 6145;
pub const VG_USERREQ__MAP_IP_TO_SRCLOC: C2RustUnnamed = 5889;
pub const VG_USERREQ__LOAD_PDB_DEBUGINFO: C2RustUnnamed = 5633;
pub const VG_USERREQ__STACK_CHANGE: C2RustUnnamed = 5379;
pub const VG_USERREQ__STACK_DEREGISTER: C2RustUnnamed = 5378;
pub const VG_USERREQ__STACK_REGISTER: C2RustUnnamed = 5377;
pub const VG_USERREQ__PRINTF_BACKTRACE_VALIST_BY_REF: C2RustUnnamed = 5124;
pub const VG_USERREQ__PRINTF_VALIST_BY_REF: C2RustUnnamed = 5123;
pub const VG_USERREQ__PRINTF_BACKTRACE: C2RustUnnamed = 5122;
pub const VG_USERREQ__PRINTF: C2RustUnnamed = 5121;
pub const VG_USERREQ__MEMPOOL_EXISTS: C2RustUnnamed = 4874;
pub const VG_USERREQ__MEMPOOL_CHANGE: C2RustUnnamed = 4873;
pub const VG_USERREQ__MOVE_MEMPOOL: C2RustUnnamed = 4872;
pub const VG_USERREQ__MEMPOOL_TRIM: C2RustUnnamed = 4871;
pub const VG_USERREQ__MEMPOOL_FREE: C2RustUnnamed = 4870;
pub const VG_USERREQ__MEMPOOL_ALLOC: C2RustUnnamed = 4869;
pub const VG_USERREQ__DESTROY_MEMPOOL: C2RustUnnamed = 4868;
pub const VG_USERREQ__CREATE_MEMPOOL: C2RustUnnamed = 4867;
pub const VG_USERREQ__FREELIKE_BLOCK: C2RustUnnamed = 4866;
pub const VG_USERREQ__RESIZEINPLACE_BLOCK: C2RustUnnamed = 4875;
pub const VG_USERREQ__MALLOCLIKE_BLOCK: C2RustUnnamed = 4865;
pub const VG_USERREQ__GDB_MONITOR_COMMAND: C2RustUnnamed = 4610;
pub const VG_USERREQ__COUNT_ERRORS: C2RustUnnamed = 4609;
pub const VG_USERREQ__CLIENT_CALL3: C2RustUnnamed = 4356;
pub const VG_USERREQ__CLIENT_CALL2: C2RustUnnamed = 4355;
pub const VG_USERREQ__CLIENT_CALL1: C2RustUnnamed = 4354;
pub const VG_USERREQ__CLIENT_CALL0: C2RustUnnamed = 4353;
pub const VG_USERREQ__DISCARD_TRANSLATIONS: C2RustUnnamed = 4098;
pub const VG_USERREQ__RUNNING_ON_VALGRIND: C2RustUnnamed = 4097;
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
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRecMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GRecMutex = _GRecMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRWLock {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GRWLock = _GRWLock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GData {
    pub n_qdatas: guint,
    pub qdatas: *mut QData,
}
pub type QData = _QData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QData {
    pub quark: GQuark,
    pub data: gpointer,
}
pub type GData = _GData;
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
    pub data: [C2RustUnnamed_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type GTypePlugin = _GTypePlugin;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeQuery {
    pub type_0: GType,
    pub type_name: *const gchar,
    pub class_size: guint,
    pub instance_size: guint,
}
pub type GTypeQuery = _GTypeQuery;
pub type GTypeDebugFlags = ::core::ffi::c_uint;
pub const G_TYPE_DEBUG_MASK: GTypeDebugFlags = 7;
pub const G_TYPE_DEBUG_INSTANCE_COUNT: GTypeDebugFlags = 4;
pub const G_TYPE_DEBUG_SIGNALS: GTypeDebugFlags = 2;
pub const G_TYPE_DEBUG_OBJECTS: GTypeDebugFlags = 1;
pub const G_TYPE_DEBUG_NONE: GTypeDebugFlags = 0;
pub type TypeNode = _TypeNode;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _TypeNode {
    pub ref_count: guint,
    pub plugin: *mut GTypePlugin,
    pub n_children: guint,
    #[bitfield(name = "n_supers", ty = "guint", bits = "0..=7")]
    #[bitfield(name = "n_prerequisites", ty = "guint", bits = "8..=16")]
    #[bitfield(name = "is_abstract", ty = "guint", bits = "17..=17")]
    #[bitfield(name = "is_classed", ty = "guint", bits = "18..=18")]
    #[bitfield(name = "is_deprecated", ty = "guint", bits = "19..=19")]
    #[bitfield(name = "is_instantiatable", ty = "guint", bits = "20..=20")]
    #[bitfield(name = "is_final", ty = "guint", bits = "21..=21")]
    #[bitfield(name = "mutatable_check_cache", ty = "guint", bits = "22..=22")]
    pub n_supers_n_prerequisites_is_abstract_is_classed_is_deprecated_is_instantiatable_is_final_mutatable_check_cache:
        [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub children: *mut GType,
    pub data: *mut TypeData,
    pub qname: GQuark,
    pub global_gdata: *mut GData,
    pub _prot: C2RustUnnamed_1,
    pub prerequisites: *mut GType,
    pub supers: [GType; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub iface_entries: GAtomicArray,
    pub offsets: GAtomicArray,
}
pub type GAtomicArray = _GAtomicArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAtomicArray {
    pub data: gpointer,
}
pub type TypeData = _TypeData;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _TypeData {
    pub common: CommonData,
    pub boxed: BoxedData,
    pub iface: IFaceData,
    pub class: ClassData,
    pub instance: InstanceData,
}
pub type InstanceData = _InstanceData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _InstanceData {
    pub common: CommonData,
    pub class_size: guint16,
    pub class_private_size: guint16,
    pub init_state: ::core::ffi::c_int,
    pub class_init_base: GBaseInitFunc,
    pub class_finalize_base: GBaseFinalizeFunc,
    pub class_init: GClassInitFunc,
    pub class_finalize: GClassFinalizeFunc,
    pub class_data: gconstpointer,
    pub class: gpointer,
    pub instance_size: guint16,
    pub private_size: guint16,
    pub instance_init: GInstanceInitFunc,
}
pub type CommonData = _CommonData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CommonData {
    pub value_table: *mut GTypeValueTable,
}
pub type ClassData = _ClassData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ClassData {
    pub common: CommonData,
    pub class_size: guint16,
    pub class_private_size: guint16,
    pub init_state: ::core::ffi::c_int,
    pub class_init_base: GBaseInitFunc,
    pub class_finalize_base: GBaseFinalizeFunc,
    pub class_init: GClassInitFunc,
    pub class_finalize: GClassFinalizeFunc,
    pub class_data: gconstpointer,
    pub class: gpointer,
}
pub type IFaceData = _IFaceData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IFaceData {
    pub common: CommonData,
    pub vtable_size: guint16,
    pub vtable_init_base: GBaseInitFunc,
    pub vtable_finalize_base: GBaseFinalizeFunc,
    pub dflt_init: GClassInitFunc,
    pub dflt_finalize: GClassFinalizeFunc,
    pub dflt_data: gconstpointer,
    pub dflt_vtable: gpointer,
}
pub type BoxedData = _BoxedData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BoxedData {
    pub data: CommonData,
    pub copy_func: GBoxedCopyFunc,
    pub free_func: GBoxedFreeFunc,
}
pub type GBoxedFreeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GBoxedCopyFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type IFaceEntries = _IFaceEntries;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IFaceEntries {
    pub offset_index: gsize,
    pub entry: [IFaceEntry; 1],
}
pub type IFaceEntry = _IFaceEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IFaceEntry {
    pub iface_type: GType,
    pub vtable: *mut GTypeInterface,
    pub init_state: InitState,
}
pub type InitState = ::core::ffi::c_uint;
pub const INITIALIZED: InitState = 5;
pub const IFACE_INIT: InitState = 4;
pub const CLASS_INIT: InitState = 3;
pub const BASE_IFACE_INIT: InitState = 2;
pub const BASE_CLASS_INIT: InitState = 1;
pub const UNINITIALIZED: InitState = 0;
pub type GAtomicArrayMetadata = _GAtomicArrayMetadata;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GAtomicArrayMetadata {
    pub size: gsize,
    pub _alignment_padding: gpointer,
}
pub type IFaceHolder = _IFaceHolder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IFaceHolder {
    pub instance_type: GType,
    pub info: *mut GInterfaceInfo,
    pub plugin: *mut GTypePlugin,
    pub next: *mut IFaceHolder,
}
pub const G_VALUE_COLLECT_POINTER: C2RustUnnamed_2 = 112;
pub const G_VALUE_COLLECT_DOUBLE: C2RustUnnamed_2 = 100;
pub const G_VALUE_COLLECT_INT64: C2RustUnnamed_2 = 113;
pub const G_VALUE_COLLECT_LONG: C2RustUnnamed_2 = 108;
pub const G_VALUE_COLLECT_INT: C2RustUnnamed_2 = 105;
pub const G_TYPE_FLAG_ABSTRACT: GTypeFlags = 16;
pub const G_TYPE_FLAG_VALUE_ABSTRACT: GTypeFlags = 32;
pub type GTypeClassCacheFunc = Option<unsafe extern "C" fn(gpointer, *mut GTypeClass) -> gboolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClassCacheFunc {
    pub cache_data: gpointer,
    pub cache_func: GTypeClassCacheFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IFaceCheckFunc {
    pub check_data: gpointer,
    pub check_func: GTypeInterfaceCheckFunc,
}
pub type GTypeInterfaceCheckFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GTypeFlags = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEPRECATED: GTypeFlags = 128;
pub const G_TYPE_FLAG_FINAL: GTypeFlags = 64;
pub const G_TYPE_FLAG_NONE: GTypeFlags = 0;
pub type GTypePluginCompleteInterfaceInfo =
    Option<unsafe extern "C" fn(*mut GTypePlugin, GType, GType, *mut GInterfaceInfo) -> ()>;
pub type GTypePluginClass = _GTypePluginClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypePluginClass {
    pub base_iface: GTypeInterface,
    pub use_plugin: GTypePluginUse,
    pub unuse_plugin: GTypePluginUnuse,
    pub complete_type_info: GTypePluginCompleteTypeInfo,
    pub complete_interface_info: GTypePluginCompleteInterfaceInfo,
}
pub type GTypePluginCompleteTypeInfo = Option<
    unsafe extern "C" fn(*mut GTypePlugin, GType, *mut GTypeInfo, *mut GTypeValueTable) -> (),
>;
pub type GTypePluginUnuse = Option<unsafe extern "C" fn(*mut GTypePlugin) -> ()>;
pub type GTypePluginUse = Option<unsafe extern "C" fn(*mut GTypePlugin) -> ()>;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
static mut type_rw_lock: GRWLock = _GRWLock {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    i: [0; 2],
};
static mut class_init_rec_mutex: GRecMutex = _GRecMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    i: [0; 2],
};
static mut static_n_class_cache_funcs: guint = 0 as guint;
static mut static_class_cache_funcs: *mut ClassCacheFunc =
    ::core::ptr::null::<ClassCacheFunc>() as *mut ClassCacheFunc;
static mut static_n_iface_check_funcs: guint = 0 as guint;
static mut static_iface_check_funcs: *mut IFaceCheckFunc =
    ::core::ptr::null::<IFaceCheckFunc>() as *mut IFaceCheckFunc;
static mut static_quark_type_flags: GQuark = 0 as GQuark;
static mut static_quark_iface_holder: GQuark = 0 as GQuark;
static mut static_quark_dependants_array: GQuark = 0 as GQuark;
static mut type_registration_serial: guint = 0 as guint;
#[no_mangle]
pub static mut _g_type_debug_flags: GTypeDebugFlags = G_TYPE_DEBUG_NONE;
static mut static_type_nodes_ht: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut static_fundamental_type_nodes: [*mut TypeNode; 256] = [
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
    ::core::ptr::null::<TypeNode>() as *mut TypeNode,
];
static mut static_fundamental_next: GType = 49 as GType;
static TYPE_SYSTEM_INIT: Once = Once::new();
#[inline]
unsafe extern "C" fn lookup_type_node_I(mut utype: GType) -> *mut TypeNode {
    if static_quark_type_flags == 0 {
        bootstrap_type_system();
    }
    if utype > ((255 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
        return (utype
            & !((((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as GType)) as *mut TypeNode;
    } else {
        return static_fundamental_type_nodes[(utype >> 2 as ::core::ffi::c_int) as usize];
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_get_type_registration_serial() -> guint {
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            *(&raw mut type_registration_serial as *mut gint);
            *(&raw mut type_registration_serial as *mut gint);
        } else {
        };
        *&raw mut gaig_temp =
            crate::translated::compat::atomic_load_seqcst(&raw mut type_registration_serial as *mut gint);
        gaig_temp
    }) as guint;
}
unsafe extern "C" fn type_node_any_new_W(
    mut pnode: *mut TypeNode,
    mut ftype: GType,
    mut name: *const gchar,
    mut plugin: *mut GTypePlugin,
    mut type_flags: GTypeFundamentalFlags,
) -> *mut TypeNode {
    let mut n_supers: guint = 0;
    let mut type_0: GType = 0;
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut i: guint = 0;
    let mut node_size: guint = 0 as guint;
    n_supers = (if !pnode.is_null() {
        (*pnode).n_supers() as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as guint;
    if pnode.is_null() {
        node_size = (node_size as gssize
            + (if (if ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                > ::core::mem::size_of::<gpointer>() as usize
            {
                ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
            } else {
                ::core::mem::size_of::<gpointer>() as usize
            }) > ::core::mem::size_of::<glong>() as usize
            {
                if ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                    > ::core::mem::size_of::<gpointer>() as usize
                {
                    ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                } else {
                    ::core::mem::size_of::<gpointer>() as usize
                }
            } else {
                ::core::mem::size_of::<glong>() as usize
            }) as gssize) as guint;
    }
    node_size = (node_size as glong + 72 as ::core::ffi::c_ulong as glong) as guint;
    node_size = (node_size as ::core::ffi::c_ulong).wrapping_add(
        (::core::mem::size_of::<GType>() as usize)
            .wrapping_mul((1 as guint).wrapping_add(n_supers).wrapping_add(1 as guint) as usize)
            as ::core::ffi::c_ulong,
    ) as guint as guint;
    node = g_malloc0(node_size as gsize) as *mut TypeNode;
    if pnode.is_null() {
        node = (node as *mut guint8).offset(
            (if (if ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                > ::core::mem::size_of::<gpointer>() as usize
            {
                ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
            } else {
                ::core::mem::size_of::<gpointer>() as usize
            }) > ::core::mem::size_of::<glong>() as usize
            {
                (if ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                    > ::core::mem::size_of::<gpointer>() as usize
                {
                    ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                } else {
                    ::core::mem::size_of::<gpointer>() as usize
                })
            } else {
                ::core::mem::size_of::<glong>() as usize
            }) as gssize as isize,
        ) as gpointer as *mut TypeNode;
        static_fundamental_type_nodes[(ftype >> 2 as ::core::ffi::c_int) as usize] = node;
        type_0 = ftype;
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__MALLOCLIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                node as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (node_size as gssize
                    - (if (if ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                        > ::core::mem::size_of::<gpointer>() as usize
                    {
                        ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                    } else {
                        ::core::mem::size_of::<gpointer>() as usize
                    }) > ::core::mem::size_of::<glong>() as usize
                    {
                        (if ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                            > ::core::mem::size_of::<gpointer>() as usize
                        {
                            ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                        } else {
                            ::core::mem::size_of::<gpointer>() as usize
                        })
                    } else {
                        ::core::mem::size_of::<glong>() as usize
                    }) as gssize) as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh5 = &mut _zzq_result;
            let fresh6;
            let fresh7 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh5, fresh7) =>
                fresh6, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t).offset(0
                as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
        });
    } else {
        type_0 = node as guintptr;
    }
    if type_0
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
            as GType
        == 0 as GType
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            427 as ::core::ffi::c_int,
            b"type_node_any_new_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"(type & TYPE_ID_MASK) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*node).set_n_supers(n_supers as guint);
    if pnode.is_null() {
        *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize) = type_0;
        *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize) =
            0 as GType;
        (*node).set_is_abstract(
            (type_flags as ::core::ffi::c_uint
                & G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint as guint,
        );
        (*node).set_is_classed(
            (type_flags as ::core::ffi::c_uint
                & G_TYPE_FLAG_CLASSED as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint as guint,
        );
        (*node).set_is_deprecated(
            (type_flags as ::core::ffi::c_uint
                & G_TYPE_FLAG_DEPRECATED as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint as guint,
        );
        (*node).set_is_instantiatable(
            (type_flags as ::core::ffi::c_uint
                & G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint as guint,
        );
        if *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        {
            (*node).set_n_prerequisites(0 as guint as guint);
            (*node).prerequisites = ::core::ptr::null_mut::<GType>();
        } else {
            _g_atomic_array_init(&raw mut (*node)._prot.iface_entries);
        }
    } else {
        *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize) = type_0;
        memcpy(
            (&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            &raw mut (*pnode).supers as *mut GType as *const ::core::ffi::c_void,
            (::core::mem::size_of::<GType>() as size_t).wrapping_mul(
                (1 as ::core::ffi::c_int
                    + (*pnode).n_supers() as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as size_t,
            ),
        );
        (*node).set_is_abstract(
            (type_flags as ::core::ffi::c_uint
                & G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint as guint,
        );
        (*node).set_is_classed((*pnode).is_classed() as guint);
        (*node).set_is_deprecated(
            (type_flags as ::core::ffi::c_uint
                & G_TYPE_FLAG_DEPRECATED as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint as guint,
        );
        (*node).set_is_instantiatable((*pnode).is_instantiatable() as guint);
        (*node).set_is_deprecated(
            (*node).is_deprecated() | (*pnode).is_deprecated() as ::core::ffi::c_int as guint,
        );
        if *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        {
            (*node).set_n_prerequisites(0 as guint as guint);
            (*node).prerequisites = ::core::ptr::null_mut::<GType>();
        } else {
            let mut j: guint = 0;
            let mut entries: *mut IFaceEntries = ::core::ptr::null_mut::<IFaceEntries>();
            entries = _g_atomic_array_copy(
                &raw mut (*pnode)._prot.iface_entries,
                (::core::mem::size_of::<IFaceEntries>() as gsize)
                    .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as gsize),
                0 as gsize,
            ) as *mut IFaceEntries;
            if !entries.is_null() {
                j = 0 as guint;
                while (j as usize)
                    < ((*(entries as *mut GAtomicArrayMetadata)
                        .offset(-(1 as ::core::ffi::c_int as isize)))
                    .size as usize)
                        .wrapping_sub(
                            (::core::mem::size_of::<IFaceEntries>() as usize)
                                .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                        )
                        .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize)
                {
                    let ref mut fresh8 =
                        (*(&raw mut (*entries).entry as *mut IFaceEntry).offset(j as isize)).vtable;
                    *fresh8 = ::core::ptr::null_mut::<GTypeInterface>();
                    (*(&raw mut (*entries).entry as *mut IFaceEntry).offset(j as isize))
                        .init_state = UNINITIALIZED;
                    j = j.wrapping_add(1);
                }
                _g_atomic_array_update(&raw mut (*node)._prot.iface_entries, entries as gpointer);
            }
        }
        let fresh9 = (*pnode).n_children;
        (*pnode).n_children = (*pnode).n_children.wrapping_add(1);
        i = fresh9;
        (*pnode).children = g_realloc_n(
            (*pnode).children as gpointer,
            (*pnode).n_children as gsize,
            ::core::mem::size_of::<GType>() as gsize,
        ) as *mut GType;
        *(*pnode).children.offset(i as isize) = type_0;
    }
    (*node).plugin = plugin;
    (*node).n_children = 0 as guint;
    (*node).children = ::core::ptr::null_mut::<GType>();
    (*node).data = ::core::ptr::null_mut::<TypeData>();
    (*node).qname = g_quark_from_string(name);
    (*node).global_gdata = ::core::ptr::null_mut::<GData>();
    g_hash_table_insert(
        static_type_nodes_ht,
        g_quark_to_string((*node).qname) as gpointer,
        type_0 as gpointer,
    );
    if 0 as ::core::ffi::c_int != 0 {
        *(&raw mut type_registration_serial as *mut gint);
        *(&raw mut type_registration_serial as *mut gint);
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(
        &raw mut type_registration_serial as *mut gint,
        1 as ::core::ffi::c_int,
    );
    return node;
}
#[inline]
unsafe extern "C" fn type_node_fundamental_info_I(
    mut node: *mut TypeNode,
) -> *mut GTypeFundamentalInfo {
    let mut ftype: GType =
        *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize);
    if ftype != *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize) {
        node = lookup_type_node_I(ftype);
    }
    return (if !node.is_null() {
        (node as *mut guint8).offset(
            -((if (if ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                > ::core::mem::size_of::<gpointer>() as usize
            {
                ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
            } else {
                ::core::mem::size_of::<gpointer>() as usize
            }) > ::core::mem::size_of::<glong>() as usize
            {
                (if ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                    > ::core::mem::size_of::<gpointer>() as usize
                {
                    ::core::mem::size_of::<GTypeFundamentalInfo>() as usize
                } else {
                    ::core::mem::size_of::<gpointer>() as usize
                })
            } else {
                ::core::mem::size_of::<glong>() as usize
            }) as gssize) as isize,
        ) as gpointer
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    }) as *mut GTypeFundamentalInfo;
}
unsafe extern "C" fn type_node_fundamental_new_W(
    mut ftype: GType,
    mut name: *const gchar,
    mut type_flags: GTypeFundamentalFlags,
) -> *mut TypeNode {
    let mut finfo: *mut GTypeFundamentalInfo = ::core::ptr::null_mut::<GTypeFundamentalInfo>();
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if ftype
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
            as GType
        == 0 as GType
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            526 as ::core::ffi::c_int,
            b"type_node_fundamental_new_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"(ftype & TYPE_ID_MASK) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ftype <= ((255 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            527 as ::core::ffi::c_int,
            b"type_node_fundamental_new_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"ftype <= G_TYPE_FUNDAMENTAL_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ftype >> 2 as ::core::ffi::c_int == static_fundamental_next {
        static_fundamental_next = static_fundamental_next.wrapping_add(1);
    }
    node = type_node_any_new_W(
        ::core::ptr::null_mut::<TypeNode>(),
        ftype,
        name,
        ::core::ptr::null_mut::<GTypePlugin>(),
        type_flags,
    );
    finfo = type_node_fundamental_info_I(node);
    (*finfo).type_flags = (type_flags as ::core::ffi::c_uint
        & (G_TYPE_FLAG_CLASSED as ::core::ffi::c_int
            | G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int
            | G_TYPE_FLAG_DERIVABLE as ::core::ffi::c_int
            | G_TYPE_FLAG_DEEP_DERIVABLE as ::core::ffi::c_int) as ::core::ffi::c_uint)
        as GTypeFundamentalFlags;
    return node;
}
unsafe extern "C" fn type_node_new_W(
    mut pnode: *mut TypeNode,
    mut name: *const gchar,
    mut plugin: *mut GTypePlugin,
) -> *mut TypeNode {
    if !pnode.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            546 as ::core::ffi::c_int,
            b"type_node_new_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"pnode\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ((*pnode).n_supers() as ::core::ffi::c_int) < 255 as ::core::ffi::c_int {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            547 as ::core::ffi::c_int,
            b"type_node_new_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"pnode->n_supers < MAX_N_SUPERS\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*pnode).n_children
        < (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
            .wrapping_mul(2 as ::core::ffi::c_uint)
            .wrapping_add(1 as ::core::ffi::c_uint)
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            548 as ::core::ffi::c_int,
            b"type_node_new_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"pnode->n_children < MAX_N_CHILDREN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return type_node_any_new_W(
        pnode,
        *(&raw mut (*pnode).supers as *mut GType).offset((*pnode).n_supers() as isize),
        name,
        plugin,
        0 as GTypeFundamentalFlags,
    );
}
#[inline]
unsafe extern "C" fn lookup_iface_entry_I(
    mut entries: *mut IFaceEntries,
    mut iface_node: *mut TypeNode,
) -> *mut IFaceEntry {
    let mut offsets: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut offset_index: gsize = 0;
    let mut check: *mut IFaceEntry = ::core::ptr::null_mut::<IFaceEntry>();
    let mut index: gsize = 0;
    let mut entry: *mut IFaceEntry = ::core::ptr::null_mut::<IFaceEntry>();
    if entries.is_null() {
        return ::core::ptr::null_mut::<IFaceEntry>();
    }
    let mut _datap: *mut gpointer = &raw mut (*iface_node)._prot.offsets.data;
    let mut transaction_data: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut __check: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    __check = ({
        let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut gapg_temp_atomic: *mut gpointer = _datap as *mut gpointer;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as *mut guint8;
    loop {
        transaction_data = __check;
        entry = ::core::ptr::null_mut::<IFaceEntry>();
        offsets = transaction_data;
        offset_index = (*entries).offset_index;
        if !offsets.is_null()
            && offset_index
                < (*(offsets as *mut GAtomicArrayMetadata)
                    .offset(-(1 as ::core::ffi::c_int as isize)))
                .size
        {
            index = *offsets.offset(offset_index as isize) as gsize;
            if index > 0 as gsize {
                index = index.wrapping_sub(1 as gsize);
                if (index as usize)
                    < ((*(entries as *mut GAtomicArrayMetadata)
                        .offset(-(1 as ::core::ffi::c_int as isize)))
                    .size as usize)
                        .wrapping_sub(
                            (::core::mem::size_of::<IFaceEntries>() as usize)
                                .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                        )
                        .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize)
                {
                    check = (&raw mut (*entries).entry as *mut IFaceEntry).offset(index as isize)
                        as *mut IFaceEntry;
                    if (*check).iface_type
                        == *(&raw mut (*iface_node).supers as *mut GType)
                            .offset(0 as ::core::ffi::c_int as isize)
                    {
                        entry = check;
                    }
                }
            }
        }
        __check = ({
            let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut gapg_temp_atomic: *mut gpointer = _datap as *mut gpointer;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) as *mut guint8;
        if !(transaction_data != __check) {
            break;
        }
    }
    return entry;
}
#[inline]
unsafe extern "C" fn type_lookup_iface_entry_L(
    mut node: *mut TypeNode,
    mut iface_node: *mut TypeNode,
) -> *mut IFaceEntry {
    if !(*(&raw mut (*iface_node).supers as *mut GType).offset((*iface_node).n_supers() as isize)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
    {
        return ::core::ptr::null_mut::<IFaceEntry>();
    }
    return lookup_iface_entry_I(
        (*node)._prot.iface_entries.data as *mut IFaceEntries,
        iface_node,
    );
}
#[inline]
unsafe extern "C" fn type_lookup_iface_vtable_I(
    mut node: *mut TypeNode,
    mut iface_node: *mut TypeNode,
    mut vtable_ptr: *mut gpointer,
) -> gboolean {
    let mut entry: *mut IFaceEntry = ::core::ptr::null_mut::<IFaceEntry>();
    let mut res: gboolean = 0;
    if !(*(&raw mut (*iface_node).supers as *mut GType).offset((*iface_node).n_supers() as isize)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
    {
        if !vtable_ptr.is_null() {
            *vtable_ptr = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        }
        return 0 as gboolean;
    }
    let mut _datap: *mut gpointer = &raw mut (*node)._prot.iface_entries.data;
    let mut transaction_data: *mut IFaceEntries = ::core::ptr::null_mut::<IFaceEntries>();
    let mut __check: *mut IFaceEntries = ::core::ptr::null_mut::<IFaceEntries>();
    __check = ({
        let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut gapg_temp_atomic: *mut gpointer = _datap as *mut gpointer;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as *mut IFaceEntries;
    loop {
        transaction_data = __check;
        entry = lookup_iface_entry_I(transaction_data, iface_node);
        res = (entry != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut IFaceEntry)
            as ::core::ffi::c_int as gboolean;
        if !vtable_ptr.is_null() {
            if !entry.is_null() {
                *vtable_ptr = (*entry).vtable as gpointer;
            } else {
                *vtable_ptr = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
            }
        }
        __check = ({
            let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut gapg_temp_atomic: *mut gpointer = _datap as *mut gpointer;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) as *mut IFaceEntries;
        if !(transaction_data != __check) {
            break;
        }
    }
    return res;
}
#[inline]
unsafe extern "C" fn type_lookup_prerequisite_L(
    mut iface: *mut TypeNode,
    mut prerequisite_type: GType,
) -> gboolean {
    if *(&raw mut (*iface).supers as *mut GType).offset((*iface).n_supers() as isize)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        && (*iface).n_prerequisites() as ::core::ffi::c_int != 0
    {
        let mut prerequisites: *mut GType = (*iface)
            .prerequisites
            .offset(-(1 as ::core::ffi::c_int as isize));
        let mut n_prerequisites: guint = (*iface).n_prerequisites();
        loop {
            let mut i: guint = 0;
            let mut check: *mut GType = ::core::ptr::null_mut::<GType>();
            i = n_prerequisites.wrapping_add(1 as guint) >> 1 as ::core::ffi::c_int;
            check = prerequisites.offset(i as isize);
            if prerequisite_type == *check {
                return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
            } else if prerequisite_type > *check {
                n_prerequisites = n_prerequisites.wrapping_sub(i);
                prerequisites = check;
            } else {
                n_prerequisites = i.wrapping_sub(1 as guint);
            }
            if !(n_prerequisites != 0) {
                break;
            }
        }
    }
    return 0 as gboolean;
}
unsafe extern "C" fn type_descriptive_name_I(mut type_0: GType) -> *const gchar {
    if type_0 != 0 {
        let mut node: *mut TypeNode = lookup_type_node_I(type_0);
        return if !node.is_null() {
            g_quark_to_string((*node).qname)
        } else {
            b"<unknown>\0" as *const u8 as *const gchar
        };
    } else {
        return b"<invalid>\0" as *const u8 as *const gchar;
    };
}
unsafe extern "C" fn check_plugin_U(
    mut plugin: *mut GTypePlugin,
    mut need_complete_type_info: gboolean,
    mut need_complete_interface_info: gboolean,
    mut type_name: *const gchar,
) -> gboolean {
    if plugin.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"plugin handle for type '%s' is NULL\0" as *const u8 as *const gchar,
            type_name,
        );
        return 0 as gboolean;
    }
    if ({
        let mut __inst: *mut GTypeInstance = plugin as *mut GTypeInstance;
        let mut __t: GType = g_type_plugin_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = 0 as ::core::ffi::c_int as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"plugin pointer (%p) for type '%s' is invalid\0" as *const u8 as *const gchar,
            plugin,
            type_name,
        );
        return 0 as gboolean;
    }
    if need_complete_type_info != 0
        && (*(g_type_interface_peek(
            (*(plugin as *mut GTypeInstance)).g_class as gpointer,
            g_type_plugin_get_type(),
        ) as *mut GTypePluginClass))
            .complete_type_info
            .is_none()
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"plugin for type '%s' has no complete_type_info() implementation\0" as *const u8
                as *const gchar,
            type_name,
        );
        return 0 as gboolean;
    }
    if need_complete_interface_info != 0
        && (*(g_type_interface_peek(
            (*(plugin as *mut GTypeInstance)).g_class as gpointer,
            g_type_plugin_get_type(),
        ) as *mut GTypePluginClass))
            .complete_interface_info
            .is_none()
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"plugin for type '%s' has no complete_interface_info() implementation\0" as *const u8
                as *const gchar,
            type_name,
        );
        return 0 as gboolean;
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn check_type_name_I(mut type_name: *const gchar) -> gboolean {
    static mut extra_chars: [gchar; 4] =
        unsafe { ::core::mem::transmute::<[u8; 4], [gchar; 4]>(*b"-_+\0") };
    let mut p: *const gchar = type_name;
    let mut name_valid: gboolean = 0;
    if *type_name.offset(0 as ::core::ffi::c_int as isize) == 0
        || *type_name.offset(1 as ::core::ffi::c_int as isize) == 0
        || *type_name.offset(2 as ::core::ffi::c_int as isize) == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"type name '%s' is too short\0" as *const u8 as *const gchar,
            type_name,
        );
        return 0 as gboolean;
    }
    name_valid = (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'A' as i32
        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'Z' as i32
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'a' as i32
            && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'z' as i32
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '_' as i32)
        as ::core::ffi::c_int as gboolean;
    p = type_name.offset(1 as ::core::ffi::c_int as isize);
    while *p != 0 {
        name_valid &= (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >= 'A' as i32
            && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'Z' as i32
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= 'a' as i32
                && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= 'z' as i32
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
                && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '9' as i32
            || !strchr(
                &raw const extra_chars as *const ::core::ffi::c_char,
                *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            )
            .is_null()) as ::core::ffi::c_int;
        p = p.offset(1);
    }
    if name_valid == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"type name '%s' contains invalid characters\0" as *const u8 as *const gchar,
            type_name,
        );
        return 0 as gboolean;
    }
    if g_type_from_name(type_name) != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot register existing type '%s'\0" as *const u8 as *const gchar,
            type_name,
        );
        return 0 as gboolean;
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn check_derivation_I(
    mut parent_type: GType,
    mut type_name: *const gchar,
) -> gboolean {
    let mut pnode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut finfo: *mut GTypeFundamentalInfo = ::core::ptr::null_mut::<GTypeFundamentalInfo>();
    pnode = lookup_type_node_I(parent_type);
    if pnode.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot derive type '%s' from invalid parent type '%s'\0" as *const u8 as *const gchar,
            type_name,
            type_descriptive_name_I(parent_type),
        );
        return 0 as gboolean;
    }
    if (*pnode).is_final() != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot derive '%s' from final parent type '%s'\0" as *const u8 as *const gchar,
            type_name,
            g_quark_to_string((*pnode).qname),
        );
        return 0 as gboolean;
    }
    finfo = type_node_fundamental_info_I(pnode);
    if (*finfo).type_flags as ::core::ffi::c_uint
        & G_TYPE_FLAG_DERIVABLE as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot derive '%s' from non-derivable parent type '%s'\0" as *const u8
                as *const gchar,
            type_name,
            g_quark_to_string((*pnode).qname),
        );
        return 0 as gboolean;
    }
    if parent_type != *(&raw mut (*pnode).supers as *mut GType).offset((*pnode).n_supers() as isize)
        && (*finfo).type_flags as ::core::ffi::c_uint
            & G_TYPE_FLAG_DEEP_DERIVABLE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot derive '%s' from non-fundamental parent type '%s'\0" as *const u8
                as *const gchar,
            type_name,
            g_quark_to_string((*pnode).qname),
        );
        return 0 as gboolean;
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn check_collect_format_I(mut collect_format: *const gchar) -> gboolean {
    let mut p: *const gchar = collect_format;
    let mut valid_format: [gchar; 6] = [
        G_VALUE_COLLECT_INT as ::core::ffi::c_int as gchar,
        G_VALUE_COLLECT_LONG as ::core::ffi::c_int as gchar,
        G_VALUE_COLLECT_INT64 as ::core::ffi::c_int as gchar,
        G_VALUE_COLLECT_DOUBLE as ::core::ffi::c_int as gchar,
        G_VALUE_COLLECT_POINTER as ::core::ffi::c_int as gchar,
        0 as ::core::ffi::c_int as gchar,
    ];
    while *p != 0 {
        let fresh1 = p;
        p = p.offset(1);
        if strchr(
            &raw mut valid_format as *mut gchar,
            *fresh1 as ::core::ffi::c_int,
        )
        .is_null()
        {
            return 0 as gboolean;
        }
    }
    return (p.offset_from(collect_format) as ::core::ffi::c_long <= 8 as ::core::ffi::c_long)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn check_value_table_I(
    mut type_name: *const gchar,
    mut value_table: *const GTypeValueTable,
) -> gboolean {
    if value_table.is_null() {
        return 0 as gboolean;
    } else if (*value_table).value_init.is_none() {
        if (*value_table).value_free.is_some()
            || (*value_table).value_copy.is_some()
            || (*value_table).value_peek_pointer.is_some()
            || !(*value_table).collect_format.is_null()
            || (*value_table).collect_value.is_some()
            || !(*value_table).lcopy_format.is_null()
            || (*value_table).lcopy_value.is_some()
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"cannot handle uninitializable values of type '%s'\0" as *const u8 as *const gchar,
                type_name,
            );
        }
        return 0 as gboolean;
    } else {
        (*value_table).value_free.is_none();
        if (*value_table).value_copy.is_none() {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"missing 'value_copy()' for type '%s'\0" as *const u8 as *const gchar,
                type_name,
            );
            return 0 as gboolean;
        }
        if (!(*value_table).collect_format.is_null() || (*value_table).collect_value.is_some())
            && ((*value_table).collect_format.is_null() || (*value_table).collect_value.is_none())
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"one of 'collect_format' and 'collect_value()' is unspecified for type '%s'\0"
                    as *const u8 as *const gchar,
                type_name,
            );
            return 0 as gboolean;
        }
        if !(*value_table).collect_format.is_null()
            && check_collect_format_I((*value_table).collect_format) == 0
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"the '%s' specification for type '%s' is too long or invalid\0" as *const u8
                    as *const gchar,
                b"collect_format\0" as *const u8 as *const ::core::ffi::c_char,
                type_name,
            );
            return 0 as gboolean;
        }
        if (!(*value_table).lcopy_format.is_null() || (*value_table).lcopy_value.is_some())
            && ((*value_table).lcopy_format.is_null() || (*value_table).lcopy_value.is_none())
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"one of 'lcopy_format' and 'lcopy_value()' is unspecified for type '%s'\0"
                    as *const u8 as *const gchar,
                type_name,
            );
            return 0 as gboolean;
        }
        if !(*value_table).lcopy_format.is_null()
            && check_collect_format_I((*value_table).lcopy_format) == 0
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"the '%s' specification for type '%s' is too long or invalid\0" as *const u8
                    as *const gchar,
                b"lcopy_format\0" as *const u8 as *const ::core::ffi::c_char,
                type_name,
            );
            return 0 as gboolean;
        }
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn check_type_info_I(
    mut pnode: *mut TypeNode,
    mut ftype: GType,
    mut type_name: *const gchar,
    mut info: *const GTypeInfo,
) -> gboolean {
    let mut finfo: *mut GTypeFundamentalInfo =
        type_node_fundamental_info_I(lookup_type_node_I(ftype));
    let mut is_interface: gboolean = (ftype
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
        as ::core::ffi::c_int;
    if ftype <= ((255 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        && ftype
            & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as GType
            == 0
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            881 as ::core::ffi::c_int,
            b"check_type_info_I\0" as *const u8 as *const ::core::ffi::c_char,
            b"ftype <= G_TYPE_FUNDAMENTAL_MAX && !(ftype & TYPE_ID_MASK)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*finfo).type_flags as ::core::ffi::c_uint
        & G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
        && ((*info).instance_size as ::core::ffi::c_int != 0 || (*info).instance_init.is_some())
    {
        if !pnode.is_null() {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"cannot instantiate '%s', derived from non-instantiatable parent type '%s'\0"
                    as *const u8 as *const gchar,
                type_name,
                g_quark_to_string((*pnode).qname),
            );
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"cannot instantiate '%s' as non-instantiatable fundamental\0" as *const u8
                    as *const gchar,
                type_name,
            );
        }
        return 0 as gboolean;
    }
    if !((*finfo).type_flags as ::core::ffi::c_uint
        & G_TYPE_FLAG_CLASSED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        || is_interface != 0)
        && ((*info).class_init.is_some()
            || (*info).class_finalize.is_some()
            || !(*info).class_data.is_null()
            || (*info).class_size as ::core::ffi::c_int != 0
            || (*info).base_init.is_some()
            || (*info).base_finalize.is_some())
    {
        if !pnode.is_null() {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"cannot create class for '%s', derived from non-classed parent type '%s'\0"
                    as *const u8 as *const gchar,
                type_name,
                g_quark_to_string((*pnode).qname),
            );
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"cannot create class for '%s' as non-classed fundamental\0" as *const u8
                    as *const gchar,
                type_name,
            );
        }
        return 0 as gboolean;
    }
    if is_interface != 0
        && ((*info).class_size as usize) < ::core::mem::size_of::<GTypeInterface>() as usize
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"specified interface size for type '%s' is smaller than 'GTypeInterface' size\0"
                as *const u8 as *const gchar,
            type_name,
        );
        return 0 as gboolean;
    }
    if (*finfo).type_flags as ::core::ffi::c_uint
        & G_TYPE_FLAG_CLASSED as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        if ((*info).class_size as usize) < ::core::mem::size_of::<GTypeClass>() as usize {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"specified class size for type '%s' is smaller than 'GTypeClass' size\0"
                    as *const u8 as *const gchar,
                type_name,
            );
            return 0 as gboolean;
        }
        if !pnode.is_null()
            && ((*info).class_size as ::core::ffi::c_int)
                < (*(*pnode).data).class.class_size as ::core::ffi::c_int
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"specified class size for type '%s' is smaller than the parent type's '%s' class size\0"
                    as *const u8 as *const gchar,
                type_name,
                g_quark_to_string((*pnode).qname),
            );
            return 0 as gboolean;
        }
    }
    if (*finfo).type_flags as ::core::ffi::c_uint
        & G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        if ((*info).instance_size as usize) < ::core::mem::size_of::<GTypeInstance>() as usize {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"specified instance size for type '%s' is smaller than 'GTypeInstance' size\0"
                    as *const u8 as *const gchar,
                type_name,
            );
            return 0 as gboolean;
        }
        if !pnode.is_null()
            && ((*info).instance_size as ::core::ffi::c_int)
                < (*(*pnode).data).instance.instance_size as ::core::ffi::c_int
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"specified instance size for type '%s' is smaller than the parent type's '%s' instance size\0"
                    as *const u8 as *const gchar,
                type_name,
                g_quark_to_string((*pnode).qname),
            );
            return 0 as gboolean;
        }
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn find_conforming_child_type_L(
    mut pnode: *mut TypeNode,
    mut iface: *mut TypeNode,
) -> *mut TypeNode {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut i: guint = 0;
    if !type_lookup_iface_entry_L(pnode, iface).is_null() {
        return pnode;
    }
    i = 0 as guint;
    while i < (*pnode).n_children && node.is_null() {
        node = find_conforming_child_type_L(
            lookup_type_node_I(*(*pnode).children.offset(i as isize)),
            iface,
        );
        i = i.wrapping_add(1);
    }
    return node;
}
unsafe extern "C" fn check_add_interface_L(
    mut instance_type: GType,
    mut iface_type: GType,
) -> gboolean {
    let mut node: *mut TypeNode = lookup_type_node_I(instance_type);
    let mut iface: *mut TypeNode = lookup_type_node_I(iface_type);
    let mut entry: *mut IFaceEntry = ::core::ptr::null_mut::<IFaceEntry>();
    let mut tnode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut prerequisites: *mut GType = ::core::ptr::null_mut::<GType>();
    let mut i: guint = 0;
    if node.is_null() || (*node).is_instantiatable() == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot add interfaces to invalid (non-instantiatable) type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(instance_type),
        );
        return 0 as gboolean;
    }
    if iface.is_null()
        || !(*(&raw mut (*iface).supers as *mut GType).offset((*iface).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot add invalid (non-interface) type '%s' to type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(iface_type),
            g_quark_to_string((*node).qname),
        );
        return 0 as gboolean;
    }
    if !(*node).data.is_null() && !(*(*node).data).class.class.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"attempting to add an interface (%s) to class (%s) after class_init\0" as *const u8
                as *const gchar,
            g_quark_to_string((*iface).qname),
            g_quark_to_string((*node).qname),
        );
        return 0 as gboolean;
    }
    tnode = lookup_type_node_I(
        *(&raw mut (*iface).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
    );
    if *(&raw mut (*tnode).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize) != 0
        && type_lookup_iface_entry_L(node, tnode).is_null()
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot add sub-interface '%s' to type '%s' which does not conform to super-interface '%s'\0"
                as *const u8 as *const gchar,
            g_quark_to_string((*iface).qname),
            g_quark_to_string((*node).qname),
            g_quark_to_string((*tnode).qname),
        );
        return 0 as gboolean;
    }
    entry = type_lookup_iface_entry_L(node, iface);
    if !entry.is_null()
        && (*entry).vtable.is_null()
        && type_iface_peek_holder_L(
            iface,
            *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
        )
        .is_null()
    {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    tnode = find_conforming_child_type_L(node, iface);
    if !tnode.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot add interface type '%s' to type '%s', since type '%s' already conforms to interface\0"
                as *const u8 as *const gchar,
            g_quark_to_string((*iface).qname),
            g_quark_to_string((*node).qname),
            g_quark_to_string((*tnode).qname),
        );
        return 0 as gboolean;
    }
    prerequisites = (*iface).prerequisites;
    i = 0 as guint;
    while i < (*iface).n_prerequisites() {
        tnode = lookup_type_node_I(*prerequisites.offset(i as isize));
        if type_node_is_a_L(node, tnode) == 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"cannot add interface type '%s' to type '%s' which does not conform to prerequisite '%s'\0"
                    as *const u8 as *const gchar,
                g_quark_to_string((*iface).qname),
                g_quark_to_string((*node).qname),
                g_quark_to_string((*tnode).qname),
            );
            return 0 as gboolean;
        }
        i = i.wrapping_add(1);
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn check_interface_info_I(
    mut iface: *mut TypeNode,
    mut instance_type: GType,
    mut info: *const GInterfaceInfo,
) -> gboolean {
    if ((*info).interface_finalize.is_some() || !(*info).interface_data.is_null())
        && (*info).interface_init.is_none()
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"interface type '%s' for type '%s' comes without initializer\0" as *const u8
                as *const gchar,
            g_quark_to_string((*iface).qname),
            type_descriptive_name_I(instance_type),
        );
        return 0 as gboolean;
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn type_data_make_W(
    mut node: *mut TypeNode,
    mut info: *const GTypeInfo,
    mut value_table: *const GTypeValueTable,
) {
    let mut data: *mut TypeData = ::core::ptr::null_mut::<TypeData>();
    let mut vtable: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
    let mut vtable_size: guint = 0 as guint;
    if (*node).data.is_null() && !info.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1078 as ::core::ffi::c_int,
            b"type_data_make_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"node->data == NULL && info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if value_table.is_null() {
        let mut pnode: *mut TypeNode = lookup_type_node_I(
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
        if !pnode.is_null() {
            vtable = (*(*pnode).data).common.value_table;
        } else {
            static mut zero_vtable: GTypeValueTable = _GTypeValueTable {
                value_init: None,
                value_free: None,
                value_copy: None,
                value_peek_pointer: None,
                collect_format: ::core::ptr::null::<gchar>(),
                collect_value: None,
                lcopy_format: ::core::ptr::null::<gchar>(),
                lcopy_value: None,
            };
            value_table = &raw const zero_vtable;
        }
    }
    if !value_table.is_null() {
        vtable_size = ::core::mem::size_of::<GTypeValueTable>() as guint;
        if !(*value_table).collect_format.is_null() {
            vtable_size = (vtable_size as size_t).wrapping_add(strlen(
                (*value_table).collect_format as *const ::core::ffi::c_char,
            )) as guint as guint;
        }
        if !(*value_table).lcopy_format.is_null() {
            vtable_size = (vtable_size as size_t).wrapping_add(strlen(
                (*value_table).lcopy_format as *const ::core::ffi::c_char,
            )) as guint as guint;
        }
        vtable_size = vtable_size.wrapping_add(2 as guint);
    }
    if (*node).is_instantiatable() != 0 {
        let mut pnode_0: *mut TypeNode = lookup_type_node_I(
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
        data = g_malloc0(
            (::core::mem::size_of::<InstanceData>() as gsize).wrapping_add(vtable_size as gsize),
        ) as *mut TypeData;
        if vtable_size != 0 {
            vtable = (data as *mut guint8)
                .offset(::core::mem::size_of::<InstanceData>() as glong as isize)
                as gpointer as *mut GTypeValueTable;
        }
        (*data).instance.class_size = (*info).class_size;
        (*data).instance.class_init_base = (*info).base_init;
        (*data).instance.class_finalize_base = (*info).base_finalize;
        (*data).instance.class_init = (*info).class_init;
        (*data).instance.class_finalize = (*info).class_finalize;
        (*data).instance.class_data = (*info).class_data;
        (*data).instance.class = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        (*data).instance.init_state = UNINITIALIZED as ::core::ffi::c_int;
        (*data).instance.instance_size = (*info).instance_size;
        (*data).instance.private_size = 0 as guint16;
        (*data).instance.class_private_size = 0 as guint16;
        if !pnode_0.is_null() {
            (*data).instance.class_private_size = (*(*pnode_0).data).instance.class_private_size;
        }
        (*data).instance.instance_init = (*info).instance_init;
    } else if (*node).is_classed() != 0 {
        let mut pnode_1: *mut TypeNode = lookup_type_node_I(
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
        data = g_malloc0(
            (::core::mem::size_of::<ClassData>() as gsize).wrapping_add(vtable_size as gsize),
        ) as *mut TypeData;
        if vtable_size != 0 {
            vtable = (data as *mut guint8)
                .offset(::core::mem::size_of::<ClassData>() as glong as isize)
                as gpointer as *mut GTypeValueTable;
        }
        (*data).class.class_size = (*info).class_size;
        (*data).class.class_init_base = (*info).base_init;
        (*data).class.class_finalize_base = (*info).base_finalize;
        (*data).class.class_init = (*info).class_init;
        (*data).class.class_finalize = (*info).class_finalize;
        (*data).class.class_data = (*info).class_data;
        (*data).class.class = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        (*data).class.class_private_size = 0 as guint16;
        if !pnode_1.is_null() {
            (*data).class.class_private_size = (*(*pnode_1).data).class.class_private_size;
        }
        (*data).class.init_state = UNINITIALIZED as ::core::ffi::c_int;
    } else if *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        data = g_malloc0(
            (::core::mem::size_of::<IFaceData>() as gsize).wrapping_add(vtable_size as gsize),
        ) as *mut TypeData;
        if vtable_size != 0 {
            vtable = (data as *mut guint8)
                .offset(::core::mem::size_of::<IFaceData>() as glong as isize)
                as gpointer as *mut GTypeValueTable;
        }
        (*data).iface.vtable_size = (*info).class_size;
        (*data).iface.vtable_init_base = (*info).base_init;
        (*data).iface.vtable_finalize_base = (*info).base_finalize;
        (*data).iface.dflt_init = (*info).class_init;
        (*data).iface.dflt_finalize = (*info).class_finalize;
        (*data).iface.dflt_data = (*info).class_data;
        (*data).iface.dflt_vtable = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    } else if *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
        == ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        data = g_malloc0(
            (::core::mem::size_of::<BoxedData>() as gsize).wrapping_add(vtable_size as gsize),
        ) as *mut TypeData;
        if vtable_size != 0 {
            vtable = (data as *mut guint8)
                .offset(::core::mem::size_of::<BoxedData>() as glong as isize)
                as gpointer as *mut GTypeValueTable;
        }
    } else {
        data = g_malloc0(
            (::core::mem::size_of::<CommonData>() as gsize).wrapping_add(vtable_size as gsize),
        ) as *mut TypeData;
        if vtable_size != 0 {
            vtable = (data as *mut guint8)
                .offset(::core::mem::size_of::<CommonData>() as glong as isize)
                as gpointer as *mut GTypeValueTable;
        }
    }
    (*node).data = data;
    if vtable_size != 0 {
        let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
        *vtable = *value_table;
        p = (vtable as *mut guint8)
            .offset(::core::mem::size_of::<GTypeValueTable>() as glong as isize)
            as gpointer as *mut gchar;
        *p.offset(0 as ::core::ffi::c_int as isize) = 0 as gchar;
        (*vtable).collect_format = p;
        if !(*value_table).collect_format.is_null() {
            strcat(
                p as *mut ::core::ffi::c_char,
                (*value_table).collect_format as *const ::core::ffi::c_char,
            );
            p = p.offset(
                strlen((*value_table).collect_format as *const ::core::ffi::c_char) as isize,
            );
        }
        p = p.offset(1);
        *p.offset(0 as ::core::ffi::c_int as isize) = 0 as gchar;
        (*vtable).lcopy_format = p;
        if !(*value_table).lcopy_format.is_null() {
            strcat(
                p as *mut ::core::ffi::c_char,
                (*value_table).lcopy_format as *const ::core::ffi::c_char,
            );
        }
    }
    (*(*node).data).common.value_table = vtable;
    (*node).set_mutatable_check_cache(
        ((*(*(*node).data).common.value_table).value_init.is_some()
            && (G_TYPE_FLAG_VALUE_ABSTRACT as ::core::ffi::c_int
                | G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int) as guint
                & type_get_qdata_L(node, static_quark_type_flags) as gulong as guint
                == 0) as ::core::ffi::c_int as guint as guint,
    );
    if !(*(*node).data).common.value_table.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1205 as ::core::ffi::c_int,
            b"type_data_make_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"node->data->common.value_table != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut gais_temp: gint = 1 as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
        *&raw mut gais_temp,
    );
}
#[inline]
unsafe extern "C" fn type_data_ref_Wm(mut node: *mut TypeNode) {
    if (*node).data.is_null() {
        let mut pnode: *mut TypeNode = lookup_type_node_I(
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
        let mut tmp_info: GTypeInfo = _GTypeInfo {
            class_size: 0,
            base_init: None,
            base_finalize: None,
            class_init: None,
            class_finalize: None,
            class_data: ::core::ptr::null::<::core::ffi::c_void>(),
            instance_size: 0,
            n_preallocs: 0,
            instance_init: None,
            value_table: ::core::ptr::null::<GTypeValueTable>(),
        };
        let mut tmp_value_table: GTypeValueTable = _GTypeValueTable {
            value_init: None,
            value_free: None,
            value_copy: None,
            value_peek_pointer: None,
            collect_format: ::core::ptr::null::<gchar>(),
            collect_value: None,
            lcopy_format: ::core::ptr::null::<gchar>(),
            lcopy_value: None,
        };
        if !(*node).plugin.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1219 as ::core::ffi::c_int,
                b"type_data_ref_Wm\0" as *const u8 as *const ::core::ffi::c_char,
                b"node->plugin != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !pnode.is_null() {
            type_data_ref_Wm(pnode);
            if !(*node).data.is_null() {
                static mut _action: [gchar; 26] = unsafe {
                    ::core::mem::transmute::<[u8; 26], [gchar; 26]>(*b" invalidly modified type \0")
                };
                let mut _arg: gpointer = (*node).plugin as gpointer;
                let mut _tname: *const gchar = g_quark_to_string((*node).qname);
                let mut _fname: *const gchar = b"g_type_plugin_*\0" as *const u8 as *const gchar;
                if !_arg.is_null() {
                    g_log(
                        b"GLib-GObject\0" as *const u8 as *const gchar,
                        G_LOG_LEVEL_ERROR,
                        b"%s(%p)%s'%s'\0" as *const u8 as *const gchar,
                        _fname,
                        _arg,
                        &raw const _action as *const gchar,
                        _tname,
                    );
                    loop {}
                } else {
                    g_log(
                        b"GLib-GObject\0" as *const u8 as *const gchar,
                        G_LOG_LEVEL_ERROR,
                        b"%s()%s'%s'\0" as *const u8 as *const gchar,
                        _fname,
                        &raw const _action as *const gchar,
                        _tname,
                    );
                    loop {}
                }
            }
        }
        memset(
            &raw mut tmp_info as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<GTypeInfo>() as size_t,
        );
        memset(
            &raw mut tmp_value_table as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<GTypeValueTable>() as size_t,
        );
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        g_type_plugin_use((*node).plugin);
        g_type_plugin_complete_type_info(
            (*node).plugin,
            *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
            &raw mut tmp_info,
            &raw mut tmp_value_table,
        );
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
        if !(*node).data.is_null() {
            static mut _action_0: [gchar; 26] = unsafe {
                ::core::mem::transmute::<[u8; 26], [gchar; 26]>(*b" invalidly modified type \0")
            };
            let mut _arg_0: gpointer = (*node).plugin as gpointer;
            let mut _tname_0: *const gchar = g_quark_to_string((*node).qname);
            let mut _fname_0: *const gchar = b"g_type_plugin_*\0" as *const u8 as *const gchar;
            if !_arg_0.is_null() {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"%s(%p)%s'%s'\0" as *const u8 as *const gchar,
                    _fname_0,
                    _arg_0,
                    &raw const _action_0 as *const gchar,
                    _tname_0,
                );
                loop {}
            } else {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"%s()%s'%s'\0" as *const u8 as *const gchar,
                    _fname_0,
                    &raw const _action_0 as *const gchar,
                    _tname_0,
                );
                loop {}
            }
        }
        check_type_info_I(
            pnode,
            *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize),
            g_quark_to_string((*node).qname),
            &raw mut tmp_info,
        );
        type_data_make_W(
            node,
            &raw mut tmp_info,
            if check_value_table_I(g_quark_to_string((*node).qname), &raw mut tmp_value_table) != 0
            {
                &raw mut tmp_value_table
            } else {
                ::core::ptr::null_mut::<GTypeValueTable>()
            },
        );
    } else {
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint
            > 0 as guint
        {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1245 as ::core::ffi::c_int,
                b"type_data_ref_Wm\0" as *const u8 as *const ::core::ffi::c_char,
                b"NODE_REFCOUNT (node) > 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if 0 as ::core::ffi::c_int != 0 {
            *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            &raw mut (*node).ref_count as *mut ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    };
}
#[inline]
unsafe extern "C" fn type_data_ref_U(mut node: *mut TypeNode) -> gboolean {
    let mut current: guint = 0;
    loop {
        current = ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint;
        if current < 1 as guint {
            return 0 as gboolean;
        }
        if !(({
            let mut gaicae_oldval: gint = current as gint;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            let fresh2 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int),
                current.wrapping_add(1 as guint) as ::core::ffi::c_int,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int) =
                fresh2.0;
            if fresh2.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) == 0)
        {
            break;
        }
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn iface_node_has_available_offset_L(
    mut iface_node: *mut TypeNode,
    mut offset: gsize,
    mut for_index: ::core::ffi::c_int,
) -> gboolean {
    let mut offsets: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    offsets = (*iface_node)._prot.offsets.data as *mut guint8;
    if offsets.is_null() {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if (*(offsets as *mut GAtomicArrayMetadata).offset(-(1 as ::core::ffi::c_int as isize))).size
        <= offset
    {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    if *offsets.offset(offset as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || *offsets.offset(offset as isize) as ::core::ffi::c_int
            == for_index + 1 as ::core::ffi::c_int
    {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return 0 as gboolean;
}
unsafe extern "C" fn find_free_iface_offset_L(mut entries: *mut IFaceEntries) -> gsize {
    let mut entry: *mut IFaceEntry = ::core::ptr::null_mut::<IFaceEntry>();
    let mut iface_node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut offset: gsize = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut n_entries: ::core::ffi::c_int = 0;
    n_entries = ((*(entries as *mut GAtomicArrayMetadata)
        .offset(-(1 as ::core::ffi::c_int as isize)))
    .size as usize)
        .wrapping_sub(
            (::core::mem::size_of::<IFaceEntries>() as usize)
                .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
        )
        .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize)
        as ::core::ffi::c_int;
    offset = 0 as gsize;
    loop {
        i = 0 as ::core::ffi::c_int;
        while i < n_entries {
            entry = (&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize)
                as *mut IFaceEntry;
            iface_node = lookup_type_node_I((*entry).iface_type);
            if iface_node_has_available_offset_L(iface_node, offset, i) == 0 {
                offset = offset.wrapping_add(1);
                break;
            } else {
                i += 1;
            }
        }
        if !(i != n_entries) {
            break;
        }
    }
    return offset;
}
unsafe extern "C" fn iface_node_set_offset_L(
    mut iface_node: *mut TypeNode,
    mut offset: gsize,
    mut index: ::core::ffi::c_int,
) {
    let mut offsets: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut old_offsets: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut new_size: gsize = 0;
    let mut old_size: gsize = 0;
    let mut i: gsize = 0;
    old_offsets = (*iface_node)._prot.offsets.data as *mut guint8;
    if old_offsets.is_null() {
        old_size = 0 as gsize;
    } else {
        old_size = (*(old_offsets as *mut GAtomicArrayMetadata)
            .offset(-(1 as ::core::ffi::c_int as isize)))
        .size;
        if offset < old_size
            && *old_offsets.offset(offset as isize) as ::core::ffi::c_int
                == index + 1 as ::core::ffi::c_int
        {
            return;
        }
    }
    new_size = if old_size > offset.wrapping_add(1 as gsize) {
        old_size
    } else {
        offset.wrapping_add(1 as gsize)
    };
    offsets = _g_atomic_array_copy(
        &raw mut (*iface_node)._prot.offsets,
        0 as gsize,
        new_size.wrapping_sub(old_size),
    ) as *mut guint8;
    i = old_size;
    while i < new_size {
        *offsets.offset(i as isize) = 0 as guint8;
        i = i.wrapping_add(1);
    }
    *offsets.offset(offset as isize) = (index + 1 as ::core::ffi::c_int) as guint8;
    _g_atomic_array_update(&raw mut (*iface_node)._prot.offsets, offsets as gpointer);
}
unsafe extern "C" fn type_node_add_iface_entry_W(
    mut node: *mut TypeNode,
    mut iface_type: GType,
    mut parent_entry: *mut IFaceEntry,
) {
    let mut entries: *mut IFaceEntries = ::core::ptr::null_mut::<IFaceEntries>();
    let mut entry: *mut IFaceEntry = ::core::ptr::null_mut::<IFaceEntry>();
    let mut iface_node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut i: guint = 0;
    let mut j: guint = 0;
    let mut num_entries: guint = 0;
    if (*node).is_instantiatable() != 0 {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1361 as ::core::ffi::c_int,
            b"type_node_add_iface_entry_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"node->is_instantiatable\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    entries = (*node)._prot.iface_entries.data as *mut IFaceEntries;
    if !entries.is_null() {
        num_entries =
            ((*(entries as *mut GAtomicArrayMetadata).offset(-(1 as ::core::ffi::c_int as isize)))
                .size as usize)
                .wrapping_sub(
                    (::core::mem::size_of::<IFaceEntries>() as usize)
                        .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                )
                .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize) as guint;
        if num_entries < 255 as guint {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1368 as ::core::ffi::c_int,
                b"type_node_add_iface_entry_W\0" as *const u8 as *const ::core::ffi::c_char,
                b"num_entries < MAX_N_INTERFACES\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        i = 0 as guint;
        while i < num_entries {
            entry = (&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize)
                as *mut IFaceEntry;
            if (*entry).iface_type == iface_type {
                if parent_entry.is_null() {
                    if (*entry).vtable.is_null()
                        && (*entry).init_state as ::core::ffi::c_uint
                            == UNINITIALIZED as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                    } else {
                        g_assertion_message_expr(
                            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            1384 as ::core::ffi::c_int,
                            b"type_node_add_iface_entry_W\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"entry->vtable == NULL && entry->init_state == UNINITIALIZED\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
                return;
            }
            i = i.wrapping_add(1);
        }
    }
    entries = _g_atomic_array_copy(
        &raw mut (*node)._prot.iface_entries,
        (::core::mem::size_of::<IFaceEntries>() as gsize)
            .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as gsize),
        ::core::mem::size_of::<IFaceEntry>() as gsize,
    ) as *mut IFaceEntries;
    num_entries = ((*(entries as *mut GAtomicArrayMetadata)
        .offset(-(1 as ::core::ffi::c_int as isize)))
    .size as usize)
        .wrapping_sub(
            (::core::mem::size_of::<IFaceEntries>() as usize)
                .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
        )
        .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize) as guint;
    i = num_entries.wrapping_sub(1 as guint);
    if i == 0 as guint {
        (*entries).offset_index = 0 as gsize;
    }
    (*(&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize)).iface_type = iface_type;
    let ref mut fresh10 =
        (*(&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize)).vtable;
    *fresh10 = ::core::ptr::null_mut::<GTypeInterface>();
    (*(&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize)).init_state = UNINITIALIZED;
    if !parent_entry.is_null() {
        if !(*node).data.is_null()
            && ({
                let mut gaig_temp: gint = 0;
                if 0 as ::core::ffi::c_int != 0 {
                    (*(*node).data).class.init_state;
                    (*(*node).data).class.init_state;
                } else {
                };
                *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                    &raw mut (*(*node).data).class.init_state as *mut gint,
                );
                gaig_temp
            }) >= BASE_IFACE_INIT as ::core::ffi::c_int
        {
            (*(&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize)).init_state =
                INITIALIZED;
            let ref mut fresh11 =
                (*(&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize)).vtable;
            *fresh11 = (*parent_entry).vtable;
        }
    }
    iface_node = lookup_type_node_I(iface_type);
    if iface_node_has_available_offset_L(
        iface_node,
        (*entries).offset_index,
        i as ::core::ffi::c_int,
    ) != 0
    {
        iface_node_set_offset_L(iface_node, (*entries).offset_index, i as ::core::ffi::c_int);
    } else {
        (*entries).offset_index = find_free_iface_offset_L(entries);
        j = 0 as guint;
        while (j as usize)
            < ((*(entries as *mut GAtomicArrayMetadata).offset(-(1 as ::core::ffi::c_int as isize)))
                .size as usize)
                .wrapping_sub(
                    (::core::mem::size_of::<IFaceEntries>() as usize)
                        .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                )
                .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize)
        {
            entry = (&raw mut (*entries).entry as *mut IFaceEntry).offset(j as isize)
                as *mut IFaceEntry;
            iface_node = lookup_type_node_I((*entry).iface_type);
            iface_node_set_offset_L(iface_node, (*entries).offset_index, j as ::core::ffi::c_int);
            j = j.wrapping_add(1);
        }
    }
    _g_atomic_array_update(&raw mut (*node)._prot.iface_entries, entries as gpointer);
    if !parent_entry.is_null() {
        i = 0 as guint;
        while i < (*node).n_children {
            type_node_add_iface_entry_W(
                lookup_type_node_I(*(*node).children.offset(i as isize)),
                iface_type,
                (&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize)
                    as *mut IFaceEntry,
            );
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn type_add_interface_Wm(
    mut node: *mut TypeNode,
    mut iface: *mut TypeNode,
    mut info: *const GInterfaceInfo,
    mut plugin: *mut GTypePlugin,
) {
    let mut iholder: *mut IFaceHolder =
        g_malloc0_n(1 as gsize, ::core::mem::size_of::<IFaceHolder>() as gsize) as *mut IFaceHolder;
    let mut entry: *mut IFaceEntry = ::core::ptr::null_mut::<IFaceEntry>();
    let mut i: guint = 0;
    if (*node).is_instantiatable() as ::core::ffi::c_int != 0
        && *(&raw mut (*iface).supers as *mut GType).offset((*iface).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        && (!info.is_null() && plugin.is_null() || info.is_null() && !plugin.is_null())
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1459 as ::core::ffi::c_int,
            b"type_add_interface_Wm\0" as *const u8 as *const ::core::ffi::c_char,
            b"node->is_instantiatable && NODE_IS_IFACE (iface) && ((info && !plugin) || (!info && plugin))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*iholder).next = type_get_qdata_L(iface, static_quark_iface_holder) as *mut IFaceHolder;
    type_set_qdata_W(iface, static_quark_iface_holder, iholder as gpointer);
    (*iholder).instance_type =
        *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
    (*iholder).info = (if !info.is_null() {
        g_memdup2(
            info as gconstpointer,
            ::core::mem::size_of::<GInterfaceInfo>() as gsize,
        )
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    }) as *mut GInterfaceInfo;
    (*iholder).plugin = plugin;
    type_node_add_iface_entry_W(
        node,
        *(&raw mut (*iface).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
        ::core::ptr::null_mut::<IFaceEntry>(),
    );
    if !(*node).data.is_null() {
        let mut class_state: InitState = ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*(*node).data).class.init_state;
                (*(*node).data).class.init_state;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*(*node).data).class.init_state as *mut gint,
            );
            gaig_temp
        }) as InitState;
        if class_state as ::core::ffi::c_uint
            >= BASE_IFACE_INIT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            type_iface_vtable_base_init_Wm(iface, node);
        }
        if class_state as ::core::ffi::c_uint
            >= IFACE_INIT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            type_iface_vtable_iface_init_Wm(iface, node);
        }
    }
    entry = type_lookup_iface_entry_L(node, iface);
    i = 0 as guint;
    while i < (*node).n_children {
        type_node_add_iface_entry_W(
            lookup_type_node_I(*(*node).children.offset(i as isize)),
            *(&raw mut (*iface).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
            entry,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn type_iface_add_prerequisite_W(
    mut iface: *mut TypeNode,
    mut prerequisite_node: *mut TypeNode,
) {
    let mut prerequisite_type: GType = *(&raw mut (*prerequisite_node).supers as *mut GType)
        .offset(0 as ::core::ffi::c_int as isize);
    let mut prerequisites: *mut GType = ::core::ptr::null_mut::<GType>();
    let mut dependants: *mut GType = ::core::ptr::null_mut::<GType>();
    let mut n_dependants: guint = 0;
    let mut i: guint = 0;
    if *(&raw mut (*iface).supers as *mut GType).offset((*iface).n_supers() as isize)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        && ((*iface).n_prerequisites() as ::core::ffi::c_int) < 511 as ::core::ffi::c_int
        && ((*prerequisite_node).is_instantiatable() as ::core::ffi::c_int != 0
            || *(&raw mut (*prerequisite_node).supers as *mut GType)
                .offset((*prerequisite_node).n_supers() as isize)
                == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1500 as ::core::ffi::c_int,
            b"type_iface_add_prerequisite_W\0" as *const u8
                as *const ::core::ffi::c_char,
            b"NODE_IS_IFACE (iface) && IFACE_NODE_N_PREREQUISITES (iface) < MAX_N_PREREQUISITES && (prerequisite_node->is_instantiatable || NODE_IS_IFACE (prerequisite_node))\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    prerequisites = (*iface).prerequisites;
    i = 0 as guint;
    while i < (*iface).n_prerequisites() {
        if *prerequisites.offset(i as isize) == prerequisite_type {
            return;
        } else {
            if *prerequisites.offset(i as isize) > prerequisite_type {
                break;
            }
            i = i.wrapping_add(1);
        }
    }
    (*iface).set_n_prerequisites((*iface).n_prerequisites() + 1 as ::core::ffi::c_int as guint);
    (*iface).prerequisites = g_realloc_n(
        (*iface).prerequisites as gpointer,
        (*iface).n_prerequisites() as gsize,
        ::core::mem::size_of::<GType>() as gsize,
    ) as *mut GType;
    prerequisites = (*iface).prerequisites;
    memmove(
        prerequisites
            .offset(i as isize)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        prerequisites.offset(i as isize) as *const ::core::ffi::c_void,
        (::core::mem::size_of::<GType>() as size_t).wrapping_mul(
            (*iface)
                .n_prerequisites()
                .wrapping_sub(i)
                .wrapping_sub(1 as guint) as size_t,
        ),
    );
    *prerequisites.offset(i as isize) = prerequisite_type;
    if *(&raw mut (*prerequisite_node).supers as *mut GType)
        .offset((*prerequisite_node).n_supers() as isize)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        dependants =
            type_get_qdata_L(prerequisite_node, static_quark_dependants_array) as *mut GType;
        n_dependants = (if !dependants.is_null() {
            *dependants.offset(0 as ::core::ffi::c_int as isize)
        } else {
            0 as GType
        }) as guint;
        n_dependants = n_dependants.wrapping_add(1 as guint);
        dependants = g_realloc_n(
            dependants as gpointer,
            n_dependants.wrapping_add(1 as guint) as gsize,
            ::core::mem::size_of::<GType>() as gsize,
        ) as *mut GType;
        *dependants.offset(n_dependants as isize) =
            *(&raw mut (*iface).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
        *dependants.offset(0 as ::core::ffi::c_int as isize) = n_dependants as GType;
        type_set_qdata_W(
            prerequisite_node,
            static_quark_dependants_array,
            dependants as gpointer,
        );
    }
    dependants = type_get_qdata_L(iface, static_quark_dependants_array) as *mut GType;
    n_dependants = (if !dependants.is_null() {
        *dependants.offset(0 as ::core::ffi::c_int as isize)
    } else {
        0 as GType
    }) as guint;
    i = 1 as guint;
    while i <= n_dependants {
        type_iface_add_prerequisite_W(
            lookup_type_node_I(*dependants.offset(i as isize)),
            prerequisite_node,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_type_interface_add_prerequisite(
    mut interface_type: GType,
    mut prerequisite_type: GType,
) {
    let mut iface: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut prerequisite_node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut holders: *mut IFaceHolder = ::core::ptr::null_mut::<IFaceHolder>();
    if g_type_fundamental(interface_type)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_interface_add_prerequisite\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INTERFACE (interface_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(interface_type == prerequisite_type || g_type_is_a(interface_type, prerequisite_type) != 0)
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_interface_add_prerequisite\0" as *const u8 as *const ::core::ffi::c_char,
            b"!g_type_is_a (interface_type, prerequisite_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(prerequisite_type == interface_type || g_type_is_a(prerequisite_type, interface_type) != 0)
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_interface_add_prerequisite\0" as *const u8 as *const ::core::ffi::c_char,
            b"!g_type_is_a (prerequisite_type, interface_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    iface = lookup_type_node_I(interface_type);
    prerequisite_node = lookup_type_node_I(prerequisite_type);
    if iface.is_null()
        || prerequisite_node.is_null()
        || !(*(&raw mut (*iface).supers as *mut GType).offset((*iface).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"interface type '%s' or prerequisite type '%s' invalid\0" as *const u8 as *const gchar,
            type_descriptive_name_I(interface_type),
            type_descriptive_name_I(prerequisite_type),
        );
        return;
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    holders = type_get_qdata_L(iface, static_quark_iface_holder) as *mut IFaceHolder;
    if !holders.is_null() {
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"unable to add prerequisite '%s' to interface '%s' which is already in use for '%s'\0"
                as *const u8 as *const gchar,
            type_descriptive_name_I(prerequisite_type),
            type_descriptive_name_I(interface_type),
            type_descriptive_name_I((*holders).instance_type),
        );
        return;
    }
    if (*prerequisite_node).is_instantiatable() != 0 {
        let mut i: guint = 0;
        i = 0 as guint;
        while i < (*iface).n_prerequisites() {
            let mut prnode: *mut TypeNode =
                lookup_type_node_I(*(*iface).prerequisites.offset(i as isize));
            if (*prnode).is_instantiatable() != 0 {
                g_rw_lock_writer_unlock(&raw mut type_rw_lock);
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"adding prerequisite '%s' to interface '%s' conflicts with existing prerequisite '%s'\0"
                        as *const u8 as *const gchar,
                    type_descriptive_name_I(prerequisite_type),
                    type_descriptive_name_I(interface_type),
                    type_descriptive_name_I(
                        *(&raw mut (*prnode).supers as *mut GType)
                            .offset(0 as ::core::ffi::c_int as isize),
                    ),
                );
                return;
            }
            i = i.wrapping_add(1);
        }
        i = 0 as guint;
        while i
            < (*prerequisite_node)
                .n_supers()
                .wrapping_add(1 as ::core::ffi::c_uint)
        {
            type_iface_add_prerequisite_W(
                iface,
                lookup_type_node_I(
                    *(&raw mut (*prerequisite_node).supers as *mut GType).offset(i as isize),
                ),
            );
            i = i.wrapping_add(1);
        }
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    } else if *(&raw mut (*prerequisite_node).supers as *mut GType)
        .offset((*prerequisite_node).n_supers() as isize)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        let mut prerequisites: *mut GType = ::core::ptr::null_mut::<GType>();
        let mut i_0: guint = 0;
        prerequisites = (*prerequisite_node).prerequisites;
        i_0 = 0 as guint;
        while i_0 < (*prerequisite_node).n_prerequisites() {
            type_iface_add_prerequisite_W(
                iface,
                lookup_type_node_I(*prerequisites.offset(i_0 as isize)),
            );
            i_0 = i_0.wrapping_add(1);
        }
        type_iface_add_prerequisite_W(iface, prerequisite_node);
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    } else {
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"prerequisite '%s' for interface '%s' is neither instantiatable nor interface\0"
                as *const u8 as *const gchar,
            type_descriptive_name_I(prerequisite_type),
            type_descriptive_name_I(interface_type),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_interface_prerequisites(
    mut interface_type: GType,
    mut n_prerequisites: *mut guint,
) -> *mut GType {
    let mut iface: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if g_type_fundamental(interface_type)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_interface_prerequisites\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INTERFACE (interface_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GType>();
    }
    iface = lookup_type_node_I(interface_type);
    if !iface.is_null() {
        let mut types: *mut GType = ::core::ptr::null_mut::<GType>();
        let mut inode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
        let mut i: guint = 0;
        let mut n: guint = 0 as guint;
        g_rw_lock_reader_lock(&raw mut type_rw_lock);
        types = g_malloc0_n(
            ((*iface).n_prerequisites() as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize,
            ::core::mem::size_of::<GType>() as gsize,
        ) as *mut GType;
        i = 0 as guint;
        while i < (*iface).n_prerequisites() {
            let mut prerequisite: GType = *(*iface).prerequisites.offset(i as isize);
            let mut node: *mut TypeNode = lookup_type_node_I(prerequisite);
            if (*node).is_instantiatable() != 0 {
                if inode.is_null() || type_node_is_a_L(node, inode) != 0 {
                    inode = node;
                }
            } else {
                let fresh12 = n;
                n = n.wrapping_add(1);
                *types.offset(fresh12 as isize) = *(&raw mut (*node).supers as *mut GType)
                    .offset(0 as ::core::ffi::c_int as isize);
            }
            i = i.wrapping_add(1);
        }
        if !inode.is_null() {
            let fresh13 = n;
            n = n.wrapping_add(1);
            *types.offset(fresh13 as isize) =
                *(&raw mut (*inode).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
        }
        if !n_prerequisites.is_null() {
            *n_prerequisites = n;
        }
        g_rw_lock_reader_unlock(&raw mut type_rw_lock);
        return types;
    } else {
        if !n_prerequisites.is_null() {
            *n_prerequisites = 0 as guint;
        }
        return ::core::ptr::null_mut::<GType>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_interface_instantiatable_prerequisite(
    mut interface_type: GType,
) -> GType {
    let mut inode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut iface: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut i: guint = 0;
    if g_type_fundamental(interface_type)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_interface_instantiatable_prerequisite\0" as *const u8
                as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INTERFACE (interface_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
    }
    iface = lookup_type_node_I(interface_type);
    if iface.is_null() {
        return ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
    }
    g_rw_lock_reader_lock(&raw mut type_rw_lock);
    i = 0 as guint;
    while i < (*iface).n_prerequisites() {
        let mut prerequisite: GType = *(*iface).prerequisites.offset(i as isize);
        let mut node: *mut TypeNode = lookup_type_node_I(prerequisite);
        if (*node).is_instantiatable() != 0 {
            if inode.is_null() || type_node_is_a_L(node, inode) != 0 {
                inode = node;
            }
        }
        i = i.wrapping_add(1);
    }
    g_rw_lock_reader_unlock(&raw mut type_rw_lock);
    if !inode.is_null() {
        return *(&raw mut (*inode).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
    } else {
        return ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
    };
}
unsafe extern "C" fn type_iface_peek_holder_L(
    mut iface: *mut TypeNode,
    mut instance_type: GType,
) -> *mut IFaceHolder {
    let mut iholder: *mut IFaceHolder = ::core::ptr::null_mut::<IFaceHolder>();
    if *(&raw mut (*iface).supers as *mut GType).offset((*iface).n_supers() as isize)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1738 as ::core::ffi::c_int,
            b"type_iface_peek_holder_L\0" as *const u8 as *const ::core::ffi::c_char,
            b"NODE_IS_IFACE (iface)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    iholder = type_get_qdata_L(iface, static_quark_iface_holder) as *mut IFaceHolder;
    while !iholder.is_null() && (*iholder).instance_type != instance_type {
        iholder = (*iholder).next;
    }
    return iholder;
}
unsafe extern "C" fn type_iface_retrieve_holder_info_Wm(
    mut iface: *mut TypeNode,
    mut instance_type: GType,
    mut need_info: gboolean,
) -> *mut IFaceHolder {
    let mut iholder: *mut IFaceHolder = type_iface_peek_holder_L(iface, instance_type);
    if !iholder.is_null() && (*iholder).info.is_null() && need_info != 0 {
        let mut tmp_info: GInterfaceInfo = _GInterfaceInfo {
            interface_init: None,
            interface_finalize: None,
            interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        if !(*iholder).plugin.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1757 as ::core::ffi::c_int,
                b"type_iface_retrieve_holder_info_Wm\0" as *const u8 as *const ::core::ffi::c_char,
                b"iholder->plugin != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        type_data_ref_Wm(iface);
        if !(*iholder).info.is_null() {
            static mut _action: [gchar; 26] = unsafe {
                ::core::mem::transmute::<[u8; 26], [gchar; 26]>(*b" invalidly modified type \0")
            };
            let mut _arg: gpointer = (*iface).plugin as gpointer;
            let mut _tname: *const gchar = g_quark_to_string((*iface).qname);
            let mut _fname: *const gchar = b"g_type_plugin_*\0" as *const u8 as *const gchar;
            if !_arg.is_null() {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"%s(%p)%s'%s'\0" as *const u8 as *const gchar,
                    _fname,
                    _arg,
                    &raw const _action as *const gchar,
                    _tname,
                );
                loop {}
            } else {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"%s()%s'%s'\0" as *const u8 as *const gchar,
                    _fname,
                    &raw const _action as *const gchar,
                    _tname,
                );
                loop {}
            }
        }
        memset(
            &raw mut tmp_info as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<GInterfaceInfo>() as size_t,
        );
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        g_type_plugin_use((*iholder).plugin);
        g_type_plugin_complete_interface_info(
            (*iholder).plugin,
            instance_type,
            *(&raw mut (*iface).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
            &raw mut tmp_info,
        );
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
        if !(*iholder).info.is_null() {
            static mut _action_0: [gchar; 26] = unsafe {
                ::core::mem::transmute::<[u8; 26], [gchar; 26]>(*b" invalidly modified type \0")
            };
            let mut _arg_0: gpointer = (*iholder).plugin as gpointer;
            let mut _tname_0: *const gchar = g_quark_to_string((*iface).qname);
            let mut _fname_0: *const gchar = b"g_type_plugin_*\0" as *const u8 as *const gchar;
            if !_arg_0.is_null() {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"%s(%p)%s'%s'\0" as *const u8 as *const gchar,
                    _fname_0,
                    _arg_0,
                    &raw const _action_0 as *const gchar,
                    _tname_0,
                );
                loop {}
            } else {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_ERROR,
                    b"%s()%s'%s'\0" as *const u8 as *const gchar,
                    _fname_0,
                    &raw const _action_0 as *const gchar,
                    _tname_0,
                );
                loop {}
            }
        }
        check_interface_info_I(iface, instance_type, &raw mut tmp_info);
        (*iholder).info = g_memdup2(
            &raw mut tmp_info as gconstpointer,
            ::core::mem::size_of::<GInterfaceInfo>() as gsize,
        ) as *mut GInterfaceInfo;
    }
    return iholder;
}
unsafe extern "C" fn type_iface_blow_holder_info_Wm(
    mut iface: *mut TypeNode,
    mut instance_type: GType,
) {
    let mut iholder: *mut IFaceHolder =
        type_get_qdata_L(iface, static_quark_iface_holder) as *mut IFaceHolder;
    if *(&raw mut (*iface).supers as *mut GType).offset((*iface).n_supers() as isize)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1785 as ::core::ffi::c_int,
            b"type_iface_blow_holder_info_Wm\0" as *const u8 as *const ::core::ffi::c_char,
            b"NODE_IS_IFACE (iface)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    while (*iholder).instance_type != instance_type {
        iholder = (*iholder).next;
    }
    if !(*iholder).info.is_null() && !(*iholder).plugin.is_null() {
        g_free((*iholder).info as gpointer);
        (*iholder).info = ::core::ptr::null_mut::<GInterfaceInfo>();
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        g_type_plugin_unuse((*iholder).plugin);
        type_data_unref_U(iface, 0 as gboolean);
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
    }
}
unsafe extern "C" fn maybe_issue_deprecation_warning(mut type_0: GType) {
    static mut already_warned_table: *mut GHashTable =
        ::core::ptr::null::<GHashTable>() as *mut GHashTable;
    static mut enable_diagnostic: *const gchar = ::core::ptr::null::<gchar>();
    static mut already_warned_lock: GMutex = _GMutex {
        p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    };
    let mut already: gboolean = 0;
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            enable_diagnostic;
        } else {
        };
        (({
            let mut gapg_temp_newval: *const gchar = ::core::ptr::null::<gchar>();
            let mut gapg_temp_atomic: *mut *const gchar = &raw mut enable_diagnostic;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(&raw mut enable_diagnostic as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut value: *const gchar =
            g_getenv(b"G_ENABLE_DIAGNOSTIC\0" as *const u8 as *const gchar);
        if value.is_null() {
            value = b"0\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
        }
        if 0 as ::core::ffi::c_int != 0 {
            enable_diagnostic = value;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut enable_diagnostic as *mut ::core::ffi::c_void,
            value as guintptr as gpointer,
        );
    }
    if *enable_diagnostic.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '0' as i32
    {
        return;
    }
    g_mutex_lock(&raw mut already_warned_lock);
    if already_warned_table.is_null() {
        already_warned_table = g_hash_table_new(None, None);
    }
    name = g_type_name(type_0) as *const ::core::ffi::c_char;
    already = g_hash_table_contains(already_warned_table, name as gpointer as gconstpointer);
    if already == 0 {
        g_hash_table_add(already_warned_table, name as gpointer);
    }
    g_mutex_unlock(&raw mut already_warned_lock);
    if already == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"The type %s is deprecated and shouldn\xE2\x80\x99t be used any more. It may be removed in a future version.\0"
                as *const u8 as *const gchar,
            name,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_type_create_instance(mut type_0: GType) -> *mut GTypeInstance {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut instance: *mut GTypeInstance = ::core::ptr::null_mut::<GTypeInstance>();
    let mut class: *mut GTypeClass = ::core::ptr::null_mut::<GTypeClass>();
    let mut allocated: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut private_size: gint = 0;
    let mut ivar_size: gint = 0;
    let mut i: guint = 0;
    node = lookup_type_node_I(type_0);
    if node.is_null() || (*node).is_instantiatable() == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"cannot create new instance of invalid (non-instantiatable) type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(type_0),
        );
        loop {}
    }
    if (*node).mutatable_check_cache() == 0
        && _g_type_test_flags(type_0, G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int as guint) != 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"cannot create instance of abstract (non-instantiatable) type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(type_0),
        );
        loop {}
    }
    if _g_type_test_flags(
        type_0,
        G_TYPE_FLAG_DEPRECATED as ::core::ffi::c_int as guint,
    ) != 0
    {
        maybe_issue_deprecation_warning(type_0);
    }
    class = g_type_class_ref(type_0) as *mut GTypeClass;
    private_size = (*(*node).data).instance.private_size as gint;
    ivar_size = (*(*node).data).instance.instance_size as gint;
    if private_size != 0
        && ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__RUNNING_ON_VALGRIND as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh14 = &mut _zzq_result;
            let fresh15;
            let fresh16 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh16) =>
                fresh15, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh16, fresh15);
            _zzq_result
        }) as ::core::ffi::c_uint
            != 0
    {
        private_size = (private_size as ::core::ffi::c_ulong).wrapping_add(
            ((1 as usize).wrapping_add(
                (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                    .wrapping_sub(1 as usize),
            ) & (2 as usize)
                .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                .wrapping_neg()) as ::core::ffi::c_ulong,
        ) as gint as gint;
        allocated = g_malloc0(
            ((private_size + ivar_size) as gsize)
                .wrapping_add(::core::mem::size_of::<gpointer>() as gsize),
        ) as *mut gchar;
        let ref mut fresh17 = *(allocated
            .offset(private_size as isize)
            .offset(ivar_size as isize) as *mut gpointer);
        *fresh17 = allocated.offset(
            ((1 as usize).wrapping_add(
                (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                    .wrapping_sub(1 as usize),
            ) & (2 as usize)
                .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                .wrapping_neg()) as isize,
        ) as gpointer;
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__MALLOCLIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                allocated.offset(private_size as isize) as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (ivar_size as usize).wrapping_add(::core::mem::size_of::<gpointer>() as usize)
                    as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh18 = &mut _zzq_result;
            let fresh19;
            let fresh20 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh18, fresh20) =>
                fresh19, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
        });
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__MALLOCLIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                allocated.offset(
                    ((1 as usize).wrapping_add(
                        (2 as usize)
                            .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                            .wrapping_sub(1 as usize),
                    ) & (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                        .wrapping_neg()) as isize,
                ) as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (private_size as usize).wrapping_sub(
                    (1 as usize).wrapping_add(
                        (2 as usize)
                            .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                            .wrapping_sub(1 as usize),
                    ) & (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                        .wrapping_neg(),
                ) as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh21 = &mut _zzq_result;
            let fresh22;
            let fresh23 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23) =>
                fresh22, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
        });
    } else {
        allocated = g_malloc0((private_size + ivar_size) as gsize) as *mut gchar;
    }
    instance = allocated.offset(private_size as isize) as *mut GTypeInstance;
    i = (*node).n_supers();
    while i > 0 as guint {
        let mut pnode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
        pnode = lookup_type_node_I(*(&raw mut (*node).supers as *mut GType).offset(i as isize));
        if (*(*pnode).data).instance.instance_init.is_some() {
            (*instance).g_class = (*(*pnode).data).instance.class as *mut GTypeClass;
            (*(*pnode).data)
                .instance
                .instance_init
                .expect("non-null function pointer")(instance, class as gpointer);
        }
        i = i.wrapping_sub(1);
    }
    (*instance).g_class = class;
    if (*(*node).data).instance.instance_init.is_some() {
        (*(*node).data)
            .instance
            .instance_init
            .expect("non-null function pointer")(instance, class as gpointer);
    }
    return instance;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_free_instance(mut instance: *mut GTypeInstance) {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut class: *mut GTypeClass = ::core::ptr::null_mut::<GTypeClass>();
    let mut allocated: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut private_size: gint = 0;
    let mut ivar_size: gint = 0;
    if !instance.is_null() && !(*instance).g_class.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_free_instance\0" as *const u8 as *const ::core::ffi::c_char,
            b"instance != NULL && instance->g_class != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    class = (*instance).g_class;
    node = lookup_type_node_I((*class).g_type);
    if node.is_null()
        || (*node).is_instantiatable() == 0
        || (*node).data.is_null()
        || (*(*node).data).class.class != class as gpointer
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot free instance of invalid (non-instantiatable) type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I((*class).g_type),
        );
        return;
    }
    if (*node).mutatable_check_cache() == 0
        && _g_type_test_flags(
            *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
            G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int as guint,
        ) != 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot free instance of abstract (non-instantiatable) type '%s'\0" as *const u8
                as *const gchar,
            g_quark_to_string((*node).qname),
        );
        return;
    }
    (*instance).g_class = ::core::ptr::null_mut::<GTypeClass>();
    private_size = (*(*node).data).instance.private_size as gint;
    ivar_size = (*(*node).data).instance.instance_size as gint;
    allocated = (instance as *mut gchar).offset(-(private_size as isize));
    if private_size != 0
        && ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__RUNNING_ON_VALGRIND as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh24 = &mut _zzq_result;
            let fresh25;
            let fresh26 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26) =>
                fresh25, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
            _zzq_result
        }) as ::core::ffi::c_uint
            != 0
    {
        private_size = (private_size as ::core::ffi::c_ulong).wrapping_add(
            ((1 as usize).wrapping_add(
                (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                    .wrapping_sub(1 as usize),
            ) & (2 as usize)
                .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                .wrapping_neg()) as ::core::ffi::c_ulong,
        ) as gint as gint;
        allocated = allocated.offset(
            -(((1 as usize).wrapping_add(
                (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                    .wrapping_sub(1 as usize),
            ) & (2 as usize)
                .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                .wrapping_neg()) as isize),
        );
        let ref mut fresh27 = *(allocated
            .offset(private_size as isize)
            .offset(ivar_size as isize) as *mut gpointer);
        *fresh27 = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        g_free_sized(
            allocated as gpointer,
            ((private_size + ivar_size) as size_t)
                .wrapping_add(::core::mem::size_of::<gpointer>() as size_t),
        );
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__FREELIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                allocated.offset(
                    ((1 as usize).wrapping_add(
                        (2 as usize)
                            .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                            .wrapping_sub(1 as usize),
                    ) & (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                        .wrapping_neg()) as isize,
                ) as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh28 = &mut _zzq_result;
            let fresh29;
            let fresh30 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh28, fresh30) =>
                fresh29, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh28, fresh30, fresh29);
        });
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__FREELIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                instance as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh31 = &mut _zzq_result;
            let fresh32;
            let fresh33 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh31, fresh33) =>
                fresh32, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh31, fresh33, fresh32);
        });
    } else {
        g_free_sized(allocated as gpointer, (private_size + ivar_size) as size_t);
    }
    g_type_class_unref(class as gpointer);
}
unsafe extern "C" fn type_iface_ensure_dflt_vtable_Wm(mut iface: *mut TypeNode) {
    if !(*iface).data.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2045 as ::core::ffi::c_int,
            b"type_iface_ensure_dflt_vtable_Wm\0" as *const u8 as *const ::core::ffi::c_char,
            b"iface->data\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*(*iface).data).iface.dflt_vtable.is_null() {
        let mut vtable: *mut GTypeInterface =
            g_malloc0((*(*iface).data).iface.vtable_size as gsize) as *mut GTypeInterface;
        (*(*iface).data).iface.dflt_vtable = vtable as gpointer;
        (*vtable).g_type =
            *(&raw mut (*iface).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
        (*vtable).g_instance_type = 0 as GType;
        if (*(*iface).data).iface.vtable_init_base.is_some()
            || (*(*iface).data).iface.dflt_init.is_some()
        {
            g_rw_lock_writer_unlock(&raw mut type_rw_lock);
            if (*(*iface).data).iface.vtable_init_base.is_some() {
                (*(*iface).data)
                    .iface
                    .vtable_init_base
                    .expect("non-null function pointer")(vtable as gpointer);
            }
            if (*(*iface).data).iface.dflt_init.is_some() {
                (*(*iface).data)
                    .iface
                    .dflt_init
                    .expect("non-null function pointer")(
                    vtable as gpointer,
                    (*(*iface).data).iface.dflt_data as gpointer,
                );
            }
            g_rw_lock_writer_lock(&raw mut type_rw_lock);
        }
    }
}
unsafe extern "C" fn type_iface_vtable_base_init_Wm(
    mut iface: *mut TypeNode,
    mut node: *mut TypeNode,
) -> gboolean {
    let mut entry: *mut IFaceEntry = ::core::ptr::null_mut::<IFaceEntry>();
    let mut iholder: *mut IFaceHolder = ::core::ptr::null_mut::<IFaceHolder>();
    let mut vtable: *mut GTypeInterface = ::core::ptr::null_mut::<GTypeInterface>();
    let mut pnode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    iholder = type_iface_retrieve_holder_info_Wm(
        iface,
        *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
    if iholder.is_null() {
        return 0 as gboolean;
    }
    type_iface_ensure_dflt_vtable_Wm(iface);
    entry = type_lookup_iface_entry_L(node, iface);
    if !(*iface).data.is_null()
        && !entry.is_null()
        && (*entry).vtable.is_null()
        && !iholder.is_null()
        && !(*iholder).info.is_null()
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2093 as ::core::ffi::c_int,
            b"type_iface_vtable_base_init_Wm\0" as *const u8 as *const ::core::ffi::c_char,
            b"iface->data && entry && entry->vtable == NULL && iholder && iholder->info\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*entry).init_state = IFACE_INIT;
    pnode = lookup_type_node_I(
        *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
    );
    if !pnode.is_null() {
        let mut pentry: *mut IFaceEntry = type_lookup_iface_entry_L(pnode, iface);
        if !pentry.is_null() {
            vtable = g_memdup2(
                (*pentry).vtable as gconstpointer,
                (*(*iface).data).iface.vtable_size as gsize,
            ) as *mut GTypeInterface;
        }
    }
    if vtable.is_null() {
        vtable = g_memdup2(
            (*(*iface).data).iface.dflt_vtable as gconstpointer,
            (*(*iface).data).iface.vtable_size as gsize,
        ) as *mut GTypeInterface;
    }
    (*entry).vtable = vtable;
    (*vtable).g_type =
        *(&raw mut (*iface).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
    (*vtable).g_instance_type =
        *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
    if (*(*iface).data).iface.vtable_init_base.is_some() {
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        (*(*iface).data)
            .iface
            .vtable_init_base
            .expect("non-null function pointer")(vtable as gpointer);
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
    }
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn type_iface_vtable_iface_init_Wm(
    mut iface: *mut TypeNode,
    mut node: *mut TypeNode,
) {
    let mut entry: *mut IFaceEntry = type_lookup_iface_entry_L(node, iface);
    let mut iholder: *mut IFaceHolder = type_iface_peek_holder_L(
        iface,
        *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
    );
    let mut vtable: *mut GTypeInterface = ::core::ptr::null_mut::<GTypeInterface>();
    let mut i: guint = 0;
    if !(*iface).data.is_null()
        && !entry.is_null()
        && !iholder.is_null()
        && !(*iholder).info.is_null()
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2136 as ::core::ffi::c_int,
            b"type_iface_vtable_iface_init_Wm\0" as *const u8 as *const ::core::ffi::c_char,
            b"iface->data && entry && iholder && iholder->info\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*entry).init_state as ::core::ffi::c_uint
        == IFACE_INIT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2137 as ::core::ffi::c_int,
            b"type_iface_vtable_iface_init_Wm\0" as *const u8 as *const ::core::ffi::c_char,
            b"entry->init_state == IFACE_INIT\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*entry).init_state = INITIALIZED;
    vtable = (*entry).vtable;
    if (*(*iholder).info).interface_init.is_some() {
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        if (*(*iholder).info).interface_init.is_some() {
            (*(*iholder).info)
                .interface_init
                .expect("non-null function pointer")(
                vtable as gpointer,
                (*(*iholder).info).interface_data,
            );
        }
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
    }
    i = 0 as guint;
    while i < static_n_iface_check_funcs {
        let mut check_func: GTypeInterfaceCheckFunc =
            (*static_iface_check_funcs.offset(i as isize)).check_func;
        let mut check_data: gpointer = (*static_iface_check_funcs.offset(i as isize)).check_data;
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        check_func.expect("non-null function pointer")(check_data, vtable as gpointer);
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn type_iface_vtable_finalize_Wm(
    mut iface: *mut TypeNode,
    mut node: *mut TypeNode,
    mut vtable: *mut GTypeInterface,
) -> gboolean {
    let mut entry: *mut IFaceEntry = type_lookup_iface_entry_L(node, iface);
    let mut iholder: *mut IFaceHolder = ::core::ptr::null_mut::<IFaceHolder>();
    iholder = type_iface_retrieve_holder_info_Wm(
        iface,
        *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
        0 as gboolean,
    );
    if iholder.is_null() {
        return 0 as gboolean;
    }
    if !entry.is_null() && (*entry).vtable == vtable && !(*iholder).info.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2175 as ::core::ffi::c_int,
            b"type_iface_vtable_finalize_Wm\0" as *const u8 as *const ::core::ffi::c_char,
            b"entry && entry->vtable == vtable && iholder->info\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*entry).vtable = ::core::ptr::null_mut::<GTypeInterface>();
    (*entry).init_state = UNINITIALIZED;
    if (*(*iholder).info).interface_finalize.is_some()
        || (*(*iface).data).iface.vtable_finalize_base.is_some()
    {
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        if (*(*iholder).info).interface_finalize.is_some() {
            (*(*iholder).info)
                .interface_finalize
                .expect("non-null function pointer")(
                vtable as gpointer,
                (*(*iholder).info).interface_data,
            );
        }
        if (*(*iface).data).iface.vtable_finalize_base.is_some() {
            (*(*iface).data)
                .iface
                .vtable_finalize_base
                .expect("non-null function pointer")(vtable as gpointer);
        }
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
    }
    (*vtable).g_type = 0 as GType;
    (*vtable).g_instance_type = 0 as GType;
    g_free(vtable as gpointer);
    type_iface_blow_holder_info_Wm(
        iface,
        *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
    );
    return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn type_class_init_Wm(mut node: *mut TypeNode, mut pclass: *mut GTypeClass) {
    let mut slist: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut init_slist: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut class: *mut GTypeClass = ::core::ptr::null_mut::<GTypeClass>();
    let mut entries: *mut IFaceEntries = ::core::ptr::null_mut::<IFaceEntries>();
    let mut entry: *mut IFaceEntry = ::core::ptr::null_mut::<IFaceEntry>();
    let mut bnode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut pnode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut i: guint = 0;
    if (*node).is_classed() as ::core::ffi::c_int != 0
        && !(*node).data.is_null()
        && (*(*node).data).class.class_size as ::core::ffi::c_int != 0
        && (*(*node).data).class.class.is_null()
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*(*node).data).class.init_state;
                (*(*node).data).class.init_state;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*(*node).data).class.init_state as *mut gint,
            );
            gaig_temp
        }) == UNINITIALIZED as ::core::ffi::c_int
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2214 as ::core::ffi::c_int,
            b"type_class_init_Wm\0" as *const u8 as *const ::core::ffi::c_char,
            b"node->is_classed && node->data && node->data->class.class_size && !node->data->class.class && g_atomic_int_get (&node->data->class.init_state) == UNINITIALIZED\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*(*node).data).class.class_private_size != 0 {
        class = g_malloc0(
            (((*(*node).data).class.class_size as gsize).wrapping_add(
                (2 as gsize)
                    .wrapping_mul(::core::mem::size_of::<gsize>() as gsize)
                    .wrapping_sub(1 as gsize),
            ) & (2 as gsize)
                .wrapping_mul(::core::mem::size_of::<gsize>() as gsize)
                .wrapping_neg())
            .wrapping_add((*(*node).data).class.class_private_size as gsize),
        ) as *mut GTypeClass;
    } else {
        class = g_malloc0((*(*node).data).class.class_size as gsize) as *mut GTypeClass;
    }
    (*(*node).data).class.class = class as gpointer;
    let mut gais_temp: gint = BASE_CLASS_INIT as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        (*(*node).data).class.init_state;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut (*(*node).data).class.init_state as *mut gint,
        *&raw mut gais_temp,
    );
    if !pclass.is_null() {
        pnode = lookup_type_node_I((*pclass).g_type);
        memcpy(
            class as *mut ::core::ffi::c_void,
            pclass as *const ::core::ffi::c_void,
            (*(*pnode).data).class.class_size as size_t,
        );
        memcpy(
            (class as *mut guint8).offset(
                (((*(*node).data).class.class_size as usize).wrapping_add(
                    (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                        .wrapping_sub(1 as usize),
                ) & (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                    .wrapping_neg()) as glong as isize,
            ) as *mut ::core::ffi::c_void,
            (pclass as *mut guint8).offset(
                (((*(*pnode).data).class.class_size as usize).wrapping_add(
                    (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                        .wrapping_sub(1 as usize),
                ) & (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                    .wrapping_neg()) as glong as isize,
            ) as gpointer as *const ::core::ffi::c_void,
            (*(*pnode).data).class.class_private_size as size_t,
        );
        if (*node).is_instantiatable() != 0 {
            (*(*node).data).instance.private_size = (*(*pnode).data).instance.private_size;
        }
    }
    (*class).g_type =
        *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    bnode = node;
    while !bnode.is_null() {
        if (*(*bnode).data).class.class_init_base.is_some() {
            init_slist = g_slist_prepend(
                init_slist,
                ::core::mem::transmute::<GBaseInitFunc, gpointer>(
                    (*(*bnode).data).class.class_init_base,
                ),
            );
        }
        bnode = lookup_type_node_I(
            *(&raw mut (*bnode).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
    }
    slist = init_slist;
    while !slist.is_null() {
        let mut class_init_base: GBaseInitFunc =
            ::core::mem::transmute::<gpointer, GBaseInitFunc>((*slist).data);
        class_init_base.expect("non-null function pointer")(class as gpointer);
        slist = (*slist).next;
    }
    g_slist_free(init_slist);
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    let mut gais_temp_0: gint = BASE_IFACE_INIT as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        (*(*node).data).class.init_state;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut (*(*node).data).class.init_state as *mut gint,
        *&raw mut gais_temp_0,
    );
    pnode = lookup_type_node_I(
        *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
    );
    i = 0 as guint;
    loop {
        entries = (*node)._prot.iface_entries.data as *mut IFaceEntries;
        if !(!entries.is_null()
            && (i as usize)
                < ((*(entries as *mut GAtomicArrayMetadata)
                    .offset(-(1 as ::core::ffi::c_int as isize)))
                .size as usize)
                    .wrapping_sub(
                        (::core::mem::size_of::<IFaceEntries>() as usize)
                            .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                    )
                    .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize))
        {
            break;
        }
        entry =
            (&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize) as *mut IFaceEntry;
        while (i as usize)
            < ((*(entries as *mut GAtomicArrayMetadata).offset(-(1 as ::core::ffi::c_int as isize)))
                .size as usize)
                .wrapping_sub(
                    (::core::mem::size_of::<IFaceEntries>() as usize)
                        .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                )
                .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize)
            && (*entry).init_state as ::core::ffi::c_uint
                == IFACE_INIT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            entry = entry.offset(1);
            i = i.wrapping_add(1);
        }
        if i as usize
            == ((*(entries as *mut GAtomicArrayMetadata)
                .offset(-(1 as ::core::ffi::c_int as isize)))
            .size as usize)
                .wrapping_sub(
                    (::core::mem::size_of::<IFaceEntries>() as usize)
                        .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                )
                .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize)
        {
            break;
        }
        if type_iface_vtable_base_init_Wm(lookup_type_node_I((*entry).iface_type), node) == 0 {
            let mut j: guint = 0;
            let mut pentries: *mut IFaceEntries =
                (*pnode)._prot.iface_entries.data as *mut IFaceEntries;
            if !pnode.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2288 as ::core::ffi::c_int,
                    b"type_class_init_Wm\0" as *const u8 as *const ::core::ffi::c_char,
                    b"pnode != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !pentries.is_null() {
                j = 0 as guint;
                while (j as usize)
                    < ((*(pentries as *mut GAtomicArrayMetadata)
                        .offset(-(1 as ::core::ffi::c_int as isize)))
                    .size as usize)
                        .wrapping_sub(
                            (::core::mem::size_of::<IFaceEntries>() as usize)
                                .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                        )
                        .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize)
                {
                    let mut pentry: *mut IFaceEntry =
                        (&raw mut (*pentries).entry as *mut IFaceEntry).offset(j as isize)
                            as *mut IFaceEntry;
                    if (*pentry).iface_type == (*entry).iface_type {
                        (*entry).vtable = (*pentry).vtable;
                        (*entry).init_state = INITIALIZED;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                    }
                }
            }
            if !(*entry).vtable.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    2302 as ::core::ffi::c_int,
                    b"type_class_init_Wm\0" as *const u8 as *const ::core::ffi::c_char,
                    b"entry->vtable != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        i = i.wrapping_add(1);
    }
    let mut gais_temp_1: gint = CLASS_INIT as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        (*(*node).data).class.init_state;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut (*(*node).data).class.init_state as *mut gint,
        *&raw mut gais_temp_1,
    );
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    if (*(*node).data).class.class_init.is_some() {
        (*(*node).data)
            .class
            .class_init
            .expect("non-null function pointer")(
            class as gpointer,
            (*(*node).data).class.class_data as gpointer,
        );
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    let mut gais_temp_2: gint = IFACE_INIT as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        (*(*node).data).class.init_state;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut (*(*node).data).class.init_state as *mut gint,
        *&raw mut gais_temp_2,
    );
    i = 0 as guint;
    loop {
        entries = (*node)._prot.iface_entries.data as *mut IFaceEntries;
        if entries.is_null() {
            break;
        }
        entry =
            (&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize) as *mut IFaceEntry;
        while (i as usize)
            < ((*(entries as *mut GAtomicArrayMetadata).offset(-(1 as ::core::ffi::c_int as isize)))
                .size as usize)
                .wrapping_sub(
                    (::core::mem::size_of::<IFaceEntries>() as usize)
                        .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                )
                .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize)
            && (*entry).init_state as ::core::ffi::c_uint
                == INITIALIZED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            entry = entry.offset(1);
            i = i.wrapping_add(1);
        }
        if i as usize
            == ((*(entries as *mut GAtomicArrayMetadata)
                .offset(-(1 as ::core::ffi::c_int as isize)))
            .size as usize)
                .wrapping_sub(
                    (::core::mem::size_of::<IFaceEntries>() as usize)
                        .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                )
                .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize)
        {
            break;
        }
        type_iface_vtable_iface_init_Wm(lookup_type_node_I((*entry).iface_type), node);
        i = i.wrapping_add(1);
    }
    let mut gais_temp_3: gint = INITIALIZED as ::core::ffi::c_int;
    if 0 as ::core::ffi::c_int != 0 {
        (*(*node).data).class.init_state;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(
        &raw mut (*(*node).data).class.init_state as *mut gint,
        *&raw mut gais_temp_3,
    );
}
unsafe extern "C" fn type_data_finalize_class_ifaces_Wm(mut node: *mut TypeNode) {
    let mut i: guint = 0;
    let mut entries: *mut IFaceEntries = ::core::ptr::null_mut::<IFaceEntries>();
    if (*node).is_instantiatable() as ::core::ffi::c_int != 0
        && !(*node).data.is_null()
        && !(*(*node).data).class.class.is_null()
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint
            == 0 as guint
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            2363 as ::core::ffi::c_int,
            b"type_data_finalize_class_ifaces_Wm\0" as *const u8
                as *const ::core::ffi::c_char,
            b"node->is_instantiatable && node->data && node->data->class.class && NODE_REFCOUNT (node) == 0\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    '_reiterate: loop {
        entries = (*node)._prot.iface_entries.data as *mut IFaceEntries;
        i = 0 as guint;
        loop {
            if !(!entries.is_null()
                && (i as usize)
                    < ((*(entries as *mut GAtomicArrayMetadata)
                        .offset(-(1 as ::core::ffi::c_int as isize)))
                    .size as usize)
                        .wrapping_sub(
                            (::core::mem::size_of::<IFaceEntries>() as usize)
                                .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                        )
                        .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize))
            {
                break '_reiterate;
            }
            let mut entry: *mut IFaceEntry = (&raw mut (*entries).entry as *mut IFaceEntry)
                .offset(i as isize) as *mut IFaceEntry;
            if !(*entry).vtable.is_null() {
                if type_iface_vtable_finalize_Wm(
                    lookup_type_node_I((*entry).iface_type),
                    node,
                    (*entry).vtable,
                ) != 0
                {
                    break;
                }
                (*entry).vtable = ::core::ptr::null_mut::<GTypeInterface>();
                (*entry).init_state = UNINITIALIZED;
            }
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn type_data_finalize_class_U(
    mut node: *mut TypeNode,
    mut cdata: *mut ClassData,
) {
    let mut class: *mut GTypeClass = (*cdata).class as *mut GTypeClass;
    let mut bnode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if !(*cdata).class.is_null()
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint
            == 0 as guint
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2396 as ::core::ffi::c_int,
            b"type_data_finalize_class_U\0" as *const u8 as *const ::core::ffi::c_char,
            b"cdata->class && NODE_REFCOUNT (node) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*cdata).class_finalize.is_some() {
        (*cdata).class_finalize.expect("non-null function pointer")(
            class as gpointer,
            (*cdata).class_data as gpointer,
        );
    }
    if (*cdata).class_finalize_base.is_some() {
        (*cdata)
            .class_finalize_base
            .expect("non-null function pointer")(class as gpointer);
    }
    bnode = lookup_type_node_I(
        *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
    );
    while !bnode.is_null() {
        if (*(*bnode).data).class.class_finalize_base.is_some() {
            (*(*bnode).data)
                .class
                .class_finalize_base
                .expect("non-null function pointer")(class as gpointer);
        }
        bnode = lookup_type_node_I(
            *(&raw mut (*bnode).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
    }
    g_free((*cdata).class);
}
unsafe extern "C" fn type_data_last_unref_Wm(mut node: *mut TypeNode, mut uncached: gboolean) {
    if !node.is_null() && !(*node).plugin.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"type_data_last_unref_Wm\0" as *const u8 as *const ::core::ffi::c_char,
            b"node != NULL && node->plugin != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*node).data.is_null()
        || ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint
            == 0 as guint
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot drop last reference to unreferenced type '%s'\0" as *const u8 as *const gchar,
            g_quark_to_string((*node).qname),
        );
        return;
    }
    if (*node).is_classed() as ::core::ffi::c_int != 0
        && !(*node).data.is_null()
        && !(*(*node).data).class.class.is_null()
        && static_n_class_cache_funcs != 0
        && uncached == 0
    {
        let mut i: guint = 0;
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        g_rw_lock_reader_lock(&raw mut type_rw_lock);
        i = 0 as guint;
        while i < static_n_class_cache_funcs {
            let mut cache_func: GTypeClassCacheFunc =
                (*static_class_cache_funcs.offset(i as isize)).cache_func;
            let mut cache_data: gpointer =
                (*static_class_cache_funcs.offset(i as isize)).cache_data;
            let mut need_break: gboolean = 0;
            g_rw_lock_reader_unlock(&raw mut type_rw_lock);
            need_break = cache_func.expect("non-null function pointer")(
                cache_data,
                (*(*node).data).class.class as *mut GTypeClass,
            );
            g_rw_lock_reader_lock(&raw mut type_rw_lock);
            if (*node).data.is_null()
                || ({
                    let mut gaig_temp: gint = 0;
                    if 0 as ::core::ffi::c_int != 0 {
                        *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                        *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                    } else {
                    };
                    *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                        &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
                    );
                    gaig_temp
                }) as guint
                    == 0 as guint
            {
                static mut _action: [gchar; 26] = unsafe {
                    ::core::mem::transmute::<[u8; 26], [gchar; 26]>(*b" invalidly modified type \0")
                };
                let mut _arg: gpointer =
                    ::core::mem::transmute::<GTypeClassCacheFunc, gpointer>(cache_func);
                let mut _tname: *const gchar = g_quark_to_string((*node).qname);
                let mut _fname: *const gchar =
                    b"GType class cache function \0" as *const u8 as *const gchar;
                if !_arg.is_null() {
                    g_log(
                        b"GLib-GObject\0" as *const u8 as *const gchar,
                        G_LOG_LEVEL_ERROR,
                        b"%s(%p)%s'%s'\0" as *const u8 as *const gchar,
                        _fname,
                        _arg,
                        &raw const _action as *const gchar,
                        _tname,
                    );
                    loop {}
                } else {
                    g_log(
                        b"GLib-GObject\0" as *const u8 as *const gchar,
                        G_LOG_LEVEL_ERROR,
                        b"%s()%s'%s'\0" as *const u8 as *const gchar,
                        _fname,
                        &raw const _action as *const gchar,
                        _tname,
                    );
                    loop {}
                }
            }
            if need_break != 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        g_rw_lock_reader_unlock(&raw mut type_rw_lock);
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*node).ref_count as *mut ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        let mut ptype: GType =
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize);
        let mut tdata: *mut TypeData = ::core::ptr::null_mut::<TypeData>();
        (*node).is_instantiatable() != 0;
        tdata = (*node).data;
        if (*node).is_classed() as ::core::ffi::c_int != 0 && !(*tdata).class.class.is_null() {
            if !((*node)._prot.iface_entries.data as *mut IFaceEntries).is_null() {
                type_data_finalize_class_ifaces_Wm(node);
            }
            (*node).set_mutatable_check_cache(0 as guint as guint);
            (*node).data = ::core::ptr::null_mut::<TypeData>();
            g_rw_lock_writer_unlock(&raw mut type_rw_lock);
            type_data_finalize_class_U(node, &raw mut (*tdata).class);
            g_rw_lock_writer_lock(&raw mut type_rw_lock);
        } else if *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            && !(*tdata).iface.dflt_vtable.is_null()
        {
            (*node).set_mutatable_check_cache(0 as guint as guint);
            (*node).data = ::core::ptr::null_mut::<TypeData>();
            if (*tdata).iface.dflt_finalize.is_some()
                || (*tdata).iface.vtable_finalize_base.is_some()
            {
                g_rw_lock_writer_unlock(&raw mut type_rw_lock);
                if (*tdata).iface.dflt_finalize.is_some() {
                    (*tdata)
                        .iface
                        .dflt_finalize
                        .expect("non-null function pointer")(
                        (*tdata).iface.dflt_vtable,
                        (*tdata).iface.dflt_data as gpointer,
                    );
                }
                if (*tdata).iface.vtable_finalize_base.is_some() {
                    (*tdata)
                        .iface
                        .vtable_finalize_base
                        .expect("non-null function pointer")(
                        (*tdata).iface.dflt_vtable
                    );
                }
                g_rw_lock_writer_lock(&raw mut type_rw_lock);
            }
            g_free((*tdata).iface.dflt_vtable);
        } else {
            (*node).set_mutatable_check_cache(0 as guint as guint);
            (*node).data = ::core::ptr::null_mut::<TypeData>();
        }
        g_free(tdata as gpointer);
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        g_type_plugin_unuse((*node).plugin);
        if ptype != 0 {
            type_data_unref_U(lookup_type_node_I(ptype), 0 as gboolean);
        }
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
    }
}
#[inline]
unsafe extern "C" fn type_data_unref_U(mut node: *mut TypeNode, mut uncached: gboolean) {
    let mut current: guint = 0;
    loop {
        current = ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint;
        if current <= 1 as guint {
            if (*node).plugin.is_null() {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"static type '%s' unreferenced too often\0" as *const u8 as *const gchar,
                    g_quark_to_string((*node).qname),
                );
                return;
            } else {
                return;
            }
        }
        if !(({
            let mut gaicae_oldval: gint = current as gint;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            let fresh0 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int),
                current.wrapping_sub(1 as guint) as ::core::ffi::c_int,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int) =
                fresh0.0;
            if fresh0.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) == 0)
        {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_type_add_class_cache_func(
    mut cache_data: gpointer,
    mut cache_func: GTypeClassCacheFunc,
) {
    let mut i: guint = 0;
    if cache_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_add_class_cache_func\0" as *const u8 as *const ::core::ffi::c_char,
            b"cache_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    let fresh34 = static_n_class_cache_funcs;
    static_n_class_cache_funcs = static_n_class_cache_funcs.wrapping_add(1);
    i = fresh34;
    static_class_cache_funcs = g_realloc_n(
        static_class_cache_funcs as gpointer,
        static_n_class_cache_funcs as gsize,
        ::core::mem::size_of::<ClassCacheFunc>() as gsize,
    ) as *mut ClassCacheFunc;
    let ref mut fresh35 = (*static_class_cache_funcs.offset(i as isize)).cache_data;
    *fresh35 = cache_data;
    let ref mut fresh36 = (*static_class_cache_funcs.offset(i as isize)).cache_func;
    *fresh36 = cache_func;
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_remove_class_cache_func(
    mut cache_data: gpointer,
    mut cache_func: GTypeClassCacheFunc,
) {
    let mut found_it: gboolean = 0 as gboolean;
    let mut i: guint = 0;
    if cache_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_remove_class_cache_func\0" as *const u8 as *const ::core::ffi::c_char,
            b"cache_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    i = 0 as guint;
    while i < static_n_class_cache_funcs {
        if (*static_class_cache_funcs.offset(i as isize)).cache_data == cache_data
            && (*static_class_cache_funcs.offset(i as isize)).cache_func == cache_func
        {
            static_n_class_cache_funcs = static_n_class_cache_funcs.wrapping_sub(1);
            memmove(
                static_class_cache_funcs.offset(i as isize) as *mut ::core::ffi::c_void,
                static_class_cache_funcs
                    .offset(i as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (::core::mem::size_of::<ClassCacheFunc>() as size_t)
                    .wrapping_mul(static_n_class_cache_funcs.wrapping_sub(i) as size_t),
            );
            static_class_cache_funcs = g_realloc_n(
                static_class_cache_funcs as gpointer,
                static_n_class_cache_funcs as gsize,
                ::core::mem::size_of::<ClassCacheFunc>() as gsize,
            ) as *mut ClassCacheFunc;
            found_it = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    if found_it == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c:2607: cannot remove unregistered class cache func %p with data %p\0"
                as *const u8 as *const gchar,
            cache_func,
            cache_data,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_type_add_interface_check(
    mut check_data: gpointer,
    mut check_func: GTypeInterfaceCheckFunc,
) {
    let mut i: guint = 0;
    if check_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_add_interface_check\0" as *const u8 as *const ::core::ffi::c_char,
            b"check_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    let fresh37 = static_n_iface_check_funcs;
    static_n_iface_check_funcs = static_n_iface_check_funcs.wrapping_add(1);
    i = fresh37;
    static_iface_check_funcs = g_realloc_n(
        static_iface_check_funcs as gpointer,
        static_n_iface_check_funcs as gsize,
        ::core::mem::size_of::<IFaceCheckFunc>() as gsize,
    ) as *mut IFaceCheckFunc;
    let ref mut fresh38 = (*static_iface_check_funcs.offset(i as isize)).check_data;
    *fresh38 = check_data;
    let ref mut fresh39 = (*static_iface_check_funcs.offset(i as isize)).check_func;
    *fresh39 = check_func;
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_remove_interface_check(
    mut check_data: gpointer,
    mut check_func: GTypeInterfaceCheckFunc,
) {
    let mut found_it: gboolean = 0 as gboolean;
    let mut i: guint = 0;
    if check_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_remove_interface_check\0" as *const u8 as *const ::core::ffi::c_char,
            b"check_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    i = 0 as guint;
    while i < static_n_iface_check_funcs {
        if (*static_iface_check_funcs.offset(i as isize)).check_data == check_data
            && (*static_iface_check_funcs.offset(i as isize)).check_func == check_func
        {
            static_n_iface_check_funcs = static_n_iface_check_funcs.wrapping_sub(1);
            memmove(
                static_iface_check_funcs.offset(i as isize) as *mut ::core::ffi::c_void,
                static_iface_check_funcs
                    .offset(i as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (::core::mem::size_of::<IFaceCheckFunc>() as size_t)
                    .wrapping_mul(static_n_iface_check_funcs.wrapping_sub(i) as size_t),
            );
            static_iface_check_funcs = g_realloc_n(
                static_iface_check_funcs as gpointer,
                static_n_iface_check_funcs as gsize,
                ::core::mem::size_of::<IFaceCheckFunc>() as gsize,
            ) as *mut IFaceCheckFunc;
            found_it = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    if found_it == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c:2681: cannot remove unregistered class check func %p with data %p\0"
                as *const u8 as *const gchar,
            check_func,
            check_data,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_type_register_fundamental(
    mut type_id: GType,
    mut type_name: *const gchar,
    mut info: *const GTypeInfo,
    mut finfo: *const GTypeFundamentalInfo,
    mut flags: GTypeFlags,
) -> GType {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if static_quark_type_flags == 0 {
        bootstrap_type_system();
    }
    if static_quark_type_flags != 0 {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2713 as ::core::ffi::c_int,
            b"g_type_register_fundamental\0" as *const u8 as *const ::core::ffi::c_char,
            b"static_quark_type_flags\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if type_id > 0 as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_fundamental\0" as *const u8 as *const ::core::ffi::c_char,
            b"type_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !type_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_fundamental\0" as *const u8 as *const ::core::ffi::c_char,
            b"type_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !info.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_fundamental\0" as *const u8 as *const ::core::ffi::c_char,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !finfo.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_fundamental\0" as *const u8 as *const ::core::ffi::c_char,
            b"finfo != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if check_type_name_I(type_name) == 0 {
        return 0 as GType;
    }
    if type_id
        & (((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
            as GType
        != 0
        || type_id > ((255 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"attempt to register fundamental type '%s' with invalid type id (%lu)\0" as *const u8
                as *const gchar,
            type_name,
            type_id,
        );
        return 0 as GType;
    }
    if (*finfo).type_flags as ::core::ffi::c_uint
        & G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && (*finfo).type_flags as ::core::ffi::c_uint
            & G_TYPE_FLAG_CLASSED as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot register instantiatable fundamental type '%s' as non-classed\0" as *const u8
                as *const gchar,
            type_name,
        );
        return 0 as GType;
    }
    if !lookup_type_node_I(type_id).is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot register existing fundamental type '%s' (as '%s')\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(type_id),
            type_name,
        );
        return 0 as GType;
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    node = type_node_fundamental_new_W(type_id, type_name, (*finfo).type_flags);
    type_add_flags_W(node, flags);
    if check_type_info_I(
        ::core::ptr::null_mut::<TypeNode>(),
        *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize),
        type_name,
        info,
    ) != 0
    {
        type_data_make_W(
            node,
            info,
            if check_value_table_I(type_name, (*info).value_table) != 0 {
                (*info).value_table
            } else {
                ::core::ptr::null::<GTypeValueTable>()
            },
        );
    }
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    return *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_register_static_simple(
    mut parent_type: GType,
    mut type_name: *const gchar,
    mut class_size: guint,
    mut class_init: GClassInitFunc,
    mut instance_size: guint,
    mut instance_init: GInstanceInitFunc,
    mut flags: GTypeFlags,
) -> GType {
    let mut info: GTypeInfo = _GTypeInfo {
        class_size: 0,
        base_init: None,
        base_finalize: None,
        class_init: None,
        class_finalize: None,
        class_data: ::core::ptr::null::<::core::ffi::c_void>(),
        instance_size: 0,
        n_preallocs: 0,
        instance_init: None,
        value_table: ::core::ptr::null::<GTypeValueTable>(),
    };
    if class_size <= 0xffff as ::core::ffi::c_int as guint16 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_static_simple\0" as *const u8 as *const ::core::ffi::c_char,
            b"class_size <= G_MAXUINT16\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
    }
    if instance_size <= 0xffff as ::core::ffi::c_int as guint16 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_static_simple\0" as *const u8 as *const ::core::ffi::c_char,
            b"instance_size <= G_MAXUINT16\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType;
    }
    info.class_size = class_size as guint16;
    info.base_init = None;
    info.base_finalize = None;
    info.class_init = class_init;
    info.class_finalize = None;
    info.class_data = ::core::ptr::null::<::core::ffi::c_void>();
    info.instance_size = instance_size as guint16;
    info.n_preallocs = 0 as guint16;
    info.instance_init = instance_init;
    info.value_table = ::core::ptr::null::<GTypeValueTable>();
    return g_type_register_static(parent_type, type_name, &raw mut info, flags);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_register_static(
    mut parent_type: GType,
    mut type_name: *const gchar,
    mut info: *const GTypeInfo,
    mut flags: GTypeFlags,
) -> GType {
    let mut pnode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut type_0: GType = 0 as GType;
    if static_quark_type_flags == 0 {
        bootstrap_type_system();
    }
    if static_quark_type_flags != 0 {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2830 as ::core::ffi::c_int,
            b"g_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"static_quark_type_flags\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if parent_type > 0 as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"parent_type > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !type_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"type_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !info.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"info != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if check_type_name_I(type_name) == 0 || check_derivation_I(parent_type, type_name) == 0 {
        return 0 as GType;
    }
    if (*info).class_finalize.is_some() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"class finalizer specified for static type '%s'\0" as *const u8 as *const gchar,
            type_name,
        );
        return 0 as GType;
    }
    pnode = lookup_type_node_I(parent_type);
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    type_data_ref_Wm(pnode);
    if check_type_info_I(
        pnode,
        *(&raw mut (*pnode).supers as *mut GType).offset((*pnode).n_supers() as isize),
        type_name,
        info,
    ) != 0
    {
        node = type_node_new_W(pnode, type_name, ::core::ptr::null_mut::<GTypePlugin>());
        type_add_flags_W(node, flags);
        type_0 = *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
        type_data_make_W(
            node,
            info,
            if check_value_table_I(type_name, (*info).value_table) != 0 {
                (*info).value_table
            } else {
                ::core::ptr::null::<GTypeValueTable>()
            },
        );
    }
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_register_dynamic(
    mut parent_type: GType,
    mut type_name: *const gchar,
    mut plugin: *mut GTypePlugin,
    mut flags: GTypeFlags,
) -> GType {
    let mut pnode: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut type_0: GType = 0;
    if static_quark_type_flags == 0 {
        bootstrap_type_system();
    }
    if static_quark_type_flags != 0 {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2885 as ::core::ffi::c_int,
            b"g_type_register_dynamic\0" as *const u8 as *const ::core::ffi::c_char,
            b"static_quark_type_flags\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if parent_type > 0 as GType {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_dynamic\0" as *const u8 as *const ::core::ffi::c_char,
            b"parent_type > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !type_name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_dynamic\0" as *const u8 as *const ::core::ffi::c_char,
            b"type_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if !plugin.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_register_dynamic\0" as *const u8 as *const ::core::ffi::c_char,
            b"plugin != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if check_type_name_I(type_name) == 0
        || check_derivation_I(parent_type, type_name) == 0
        || check_plugin_U(
            plugin,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            0 as gboolean,
            type_name,
        ) == 0
    {
        return 0 as GType;
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    pnode = lookup_type_node_I(parent_type);
    node = type_node_new_W(pnode, type_name, plugin);
    type_add_flags_W(node, flags);
    type_0 = *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_add_interface_static(
    mut instance_type: GType,
    mut interface_type: GType,
    mut info: *const GInterfaceInfo,
) {
    if _g_type_test_flags(
        instance_type,
        G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as guint,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_add_interface_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INSTANTIATABLE (instance_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_type_parent(interface_type)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_add_interface_static\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_parent (interface_type) == G_TYPE_INTERFACE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    g_rec_mutex_lock(&raw mut class_init_rec_mutex);
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    if check_add_interface_L(instance_type, interface_type) != 0 {
        let mut node: *mut TypeNode = lookup_type_node_I(instance_type);
        let mut iface: *mut TypeNode = lookup_type_node_I(interface_type);
        if check_interface_info_I(
            iface,
            *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
            info,
        ) != 0
        {
            type_add_interface_Wm(node, iface, info, ::core::ptr::null_mut::<GTypePlugin>());
        }
    }
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    g_rec_mutex_unlock(&raw mut class_init_rec_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_add_interface_dynamic(
    mut instance_type: GType,
    mut interface_type: GType,
    mut plugin: *mut GTypePlugin,
) {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if _g_type_test_flags(
        instance_type,
        G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as guint,
    ) != 0
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_add_interface_dynamic\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INSTANTIATABLE (instance_type)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_type_parent(interface_type)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_add_interface_dynamic\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_parent (interface_type) == G_TYPE_INTERFACE\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    node = lookup_type_node_I(instance_type);
    if check_plugin_U(
        plugin,
        0 as gboolean,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        g_quark_to_string((*node).qname),
    ) == 0
    {
        return;
    }
    g_rec_mutex_lock(&raw mut class_init_rec_mutex);
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    if check_add_interface_L(instance_type, interface_type) != 0 {
        let mut iface: *mut TypeNode = lookup_type_node_I(interface_type);
        type_add_interface_Wm(node, iface, ::core::ptr::null::<GInterfaceInfo>(), plugin);
    }
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    g_rec_mutex_unlock(&raw mut class_init_rec_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_class_ref(mut type_0: GType) -> gpointer {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut ptype: GType = 0;
    let mut holds_ref: gboolean = 0;
    let mut pclass: *mut GTypeClass = ::core::ptr::null_mut::<GTypeClass>();
    node = lookup_type_node_I(type_0);
    if node.is_null() || (*node).is_classed() == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot retrieve class for invalid (unclassed) type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(type_0),
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if type_data_ref_U(node) != 0 {
        if ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*(*node).data).class.init_state;
                (*(*node).data).class.init_state;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*(*node).data).class.init_state as *mut gint,
            );
            gaig_temp
        }) == INITIALIZED as ::core::ffi::c_int
        {
            return (*(*node).data).class.class;
        }
        holds_ref = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
    } else {
        holds_ref = 0 as ::core::ffi::c_int as gboolean;
    }
    g_rec_mutex_lock(&raw mut class_init_rec_mutex);
    ptype = *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize);
    pclass = (if ptype != 0 {
        g_type_class_ref(ptype)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    }) as *mut GTypeClass;
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    if holds_ref == 0 {
        type_data_ref_Wm(node);
    }
    if (*(*node).data).class.class.is_null() {
        type_class_init_Wm(node, pclass);
    }
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    if !pclass.is_null() {
        g_type_class_unref(pclass as gpointer);
    }
    g_rec_mutex_unlock(&raw mut class_init_rec_mutex);
    return (*(*node).data).class.class;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_class_unref(mut g_class: gpointer) {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut class: *mut GTypeClass = g_class as *mut GTypeClass;
    if !g_class.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_class_unref\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_class != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    node = lookup_type_node_I((*class).g_type);
    if !node.is_null()
        && (*node).is_classed() as ::core::ffi::c_int != 0
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint
            != 0
    {
        type_data_unref_U(node, 0 as gboolean);
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot unreference class of invalid (unclassed) type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I((*class).g_type),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_class_unref_uncached(mut g_class: gpointer) {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut class: *mut GTypeClass = g_class as *mut GTypeClass;
    if !g_class.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_class_unref_uncached\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_class != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    node = lookup_type_node_I((*class).g_type);
    if !node.is_null()
        && (*node).is_classed() as ::core::ffi::c_int != 0
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint
            != 0
    {
        type_data_unref_U(node, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot unreference class of invalid (unclassed) type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I((*class).g_type),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_class_peek(mut type_0: GType) -> gpointer {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut class: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    node = lookup_type_node_I(type_0);
    if !node.is_null()
        && (*node).is_classed() as ::core::ffi::c_int != 0
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint
            != 0
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*(*node).data).class.init_state;
                (*(*node).data).class.init_state;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*(*node).data).class.init_state as *mut gint,
            );
            gaig_temp
        }) == INITIALIZED as ::core::ffi::c_int
    {
        class = (*(*node).data).class.class;
    } else {
        class = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    }
    return class;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_class_peek_static(mut type_0: GType) -> gpointer {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut class: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    node = lookup_type_node_I(type_0);
    if !node.is_null()
        && (*node).is_classed() as ::core::ffi::c_int != 0
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint
            != 0
        && (*node).plugin.is_null()
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                (*(*node).data).class.init_state;
                (*(*node).data).class.init_state;
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*(*node).data).class.init_state as *mut gint,
            );
            gaig_temp
        }) == INITIALIZED as ::core::ffi::c_int
    {
        class = (*(*node).data).class.class;
    } else {
        class = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    }
    return class;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_class_peek_parent(mut g_class: gpointer) -> gpointer {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut class: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if !g_class.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_class_peek_parent\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_class != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    node = lookup_type_node_I((*(g_class as *mut GTypeClass)).g_type);
    if !node.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_class_peek_parent\0" as *const u8 as *const ::core::ffi::c_char,
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*node).is_classed() as ::core::ffi::c_int != 0
        && !(*node).data.is_null()
        && *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize) != 0
    {
        node = lookup_type_node_I(
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
        class = (*(*node).data).class.class;
    } else if *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize) != 0
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c:3197: invalid class pointer '%p'\0"
                as *const u8 as *const gchar,
            g_class,
        );
    }
    return class;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_interface_peek(
    mut instance_class: gpointer,
    mut iface_type: GType,
) -> gpointer {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut iface: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut vtable: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut class: *mut GTypeClass = instance_class as *mut GTypeClass;
    if !instance_class.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_interface_peek\0" as *const u8 as *const ::core::ffi::c_char,
            b"instance_class != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    node = lookup_type_node_I((*class).g_type);
    iface = lookup_type_node_I(iface_type);
    if !node.is_null() && (*node).is_instantiatable() as ::core::ffi::c_int != 0 && !iface.is_null()
    {
        type_lookup_iface_vtable_I(node, iface, &raw mut vtable);
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c:3230: invalid class pointer '%p'\0"
                as *const u8 as *const gchar,
            class,
        );
    }
    return vtable;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_interface_peek_parent(mut g_iface: gpointer) -> gpointer {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut iface: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut vtable: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut iface_class: *mut GTypeInterface = g_iface as *mut GTypeInterface;
    if !g_iface.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_interface_peek_parent\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_iface != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    iface = lookup_type_node_I((*iface_class).g_type);
    node = lookup_type_node_I((*iface_class).g_instance_type);
    if !node.is_null() {
        node = lookup_type_node_I(
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
    }
    if !node.is_null() && (*node).is_instantiatable() as ::core::ffi::c_int != 0 && !iface.is_null()
    {
        type_lookup_iface_vtable_I(node, iface, &raw mut vtable);
    } else if !node.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c:3266: invalid interface pointer '%p'\0"
                as *const u8 as *const gchar,
            g_iface,
        );
    }
    return vtable;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_default_interface_ref(mut g_type: GType) -> gpointer {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut dflt_vtable: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    node = lookup_type_node_I(g_type);
    if node.is_null()
        || !(*(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
        || !(*node).data.is_null()
            && ({
                let mut gaig_temp: gint = 0;
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                    *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                } else {
                };
                *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                    &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
                );
                gaig_temp
            }) as guint
                == 0 as guint
    {
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot retrieve default vtable for invalid or non-interface type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(g_type),
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if (*node).data.is_null() || (*(*node).data).iface.dflt_vtable.is_null() {
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
        g_rec_mutex_lock(&raw mut class_init_rec_mutex);
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
        node = lookup_type_node_I(g_type);
        type_data_ref_Wm(node);
        type_iface_ensure_dflt_vtable_Wm(node);
        g_rec_mutex_unlock(&raw mut class_init_rec_mutex);
    } else {
        type_data_ref_Wm(node);
    }
    dflt_vtable = (*(*node).data).iface.dflt_vtable;
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    return dflt_vtable;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_default_interface_peek(mut g_type: GType) -> gpointer {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut vtable: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    node = lookup_type_node_I(g_type);
    if !node.is_null()
        && *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint
            != 0
    {
        vtable = (*(*node).data).iface.dflt_vtable;
    } else {
        vtable = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    }
    return vtable;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_default_interface_unref(mut g_iface: gpointer) {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut vtable: *mut GTypeInterface = g_iface as *mut GTypeInterface;
    if !g_iface.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_default_interface_unref\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_iface != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    node = lookup_type_node_I((*vtable).g_type);
    if !node.is_null()
        && *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        type_data_unref_U(node, 0 as gboolean);
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot unreference invalid interface default vtable for '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I((*vtable).g_type),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_name(mut type_0: GType) -> *const gchar {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if static_quark_type_flags == 0 {
        bootstrap_type_system();
    }
    node = lookup_type_node_I(type_0);
    return if !node.is_null() {
        g_quark_to_string((*node).qname)
    } else {
        ::core::ptr::null::<gchar>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_qname(mut type_0: GType) -> GQuark {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    node = lookup_type_node_I(type_0);
    return if !node.is_null() {
        (*node).qname
    } else {
        0 as GQuark
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_from_name(mut name: *const gchar) -> GType {
    let mut type_0: GType = 0 as GType;
    if !name.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_from_name\0" as *const u8 as *const ::core::ffi::c_char,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as GType;
    }
    if static_quark_type_flags == 0 {
        bootstrap_type_system();
    }
    g_rw_lock_reader_lock(&raw mut type_rw_lock);
    type_0 = g_hash_table_lookup(static_type_nodes_ht, name as gconstpointer) as guintptr;
    g_rw_lock_reader_unlock(&raw mut type_rw_lock);
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_parent(mut type_0: GType) -> GType {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    node = lookup_type_node_I(type_0);
    return if !node.is_null() {
        *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize)
    } else {
        0 as GType
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_depth(mut type_0: GType) -> guint {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    node = lookup_type_node_I(type_0);
    return (if !node.is_null() {
        (*node).n_supers() as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_next_base(mut type_0: GType, mut base_type: GType) -> GType {
    let mut atype: GType = 0 as GType;
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    node = lookup_type_node_I(type_0);
    if !node.is_null() {
        let mut base_node: *mut TypeNode = lookup_type_node_I(base_type);
        if !base_node.is_null()
            && ((*base_node).n_supers() as ::core::ffi::c_int)
                < (*node).n_supers() as ::core::ffi::c_int
        {
            let mut n: guint = ((*node).n_supers() as ::core::ffi::c_int
                - (*base_node).n_supers() as ::core::ffi::c_int)
                as guint;
            if *(&raw mut (*node).supers as *mut GType).offset(n as isize) == base_type {
                atype = *(&raw mut (*node).supers as *mut GType)
                    .offset(n.wrapping_sub(1 as guint) as isize);
            }
        }
    }
    return atype;
}
#[inline]
unsafe extern "C" fn type_node_check_conformities_UorL(
    mut node: *mut TypeNode,
    mut iface_node: *mut TypeNode,
    mut support_interfaces: gboolean,
    mut support_prerequisites: gboolean,
    mut have_lock: gboolean,
) -> gboolean {
    let mut match_0: gboolean = 0;
    if (*iface_node).n_supers() as ::core::ffi::c_int <= (*node).n_supers() as ::core::ffi::c_int
        && *(&raw mut (*node).supers as *mut GType).offset(
            ((*node).n_supers() as ::core::ffi::c_int
                - (*iface_node).n_supers() as ::core::ffi::c_int) as isize,
        ) == *(&raw mut (*iface_node).supers as *mut GType)
            .offset(0 as ::core::ffi::c_int as isize)
    {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    support_interfaces = (support_interfaces != 0
        && (*node).is_instantiatable() as ::core::ffi::c_int != 0
        && *(&raw mut (*iface_node).supers as *mut GType).offset((*iface_node).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
        as ::core::ffi::c_int as gboolean;
    support_prerequisites = (support_prerequisites != 0
        && *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
        as ::core::ffi::c_int as gboolean;
    match_0 = 0 as ::core::ffi::c_int as gboolean;
    if support_interfaces != 0 {
        if have_lock != 0 {
            if !type_lookup_iface_entry_L(node, iface_node).is_null() {
                match_0 = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
            }
        } else if type_lookup_iface_vtable_I(node, iface_node, ::core::ptr::null_mut::<gpointer>())
            != 0
        {
            match_0 = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        }
    }
    if match_0 == 0 && support_prerequisites != 0 {
        if have_lock == 0 {
            g_rw_lock_reader_lock(&raw mut type_rw_lock);
        }
        if support_prerequisites != 0
            && type_lookup_prerequisite_L(
                node,
                *(&raw mut (*iface_node).supers as *mut GType)
                    .offset(0 as ::core::ffi::c_int as isize),
            ) != 0
        {
            match_0 = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        }
        if have_lock == 0 {
            g_rw_lock_reader_unlock(&raw mut type_rw_lock);
        }
    }
    return match_0;
}
unsafe extern "C" fn type_node_is_a_L(
    mut node: *mut TypeNode,
    mut iface_node: *mut TypeNode,
) -> gboolean {
    return type_node_check_conformities_UorL(
        node,
        iface_node,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C" fn type_node_conforms_to_U(
    mut node: *mut TypeNode,
    mut iface_node: *mut TypeNode,
    mut support_interfaces: gboolean,
    mut support_prerequisites: gboolean,
) -> gboolean {
    return type_node_check_conformities_UorL(
        node,
        iface_node,
        support_interfaces,
        support_prerequisites,
        0 as gboolean,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_type_is_a(mut type_0: GType, mut iface_type: GType) -> gboolean {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut iface_node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut is_a: gboolean = 0;
    if type_0 == iface_type {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    node = lookup_type_node_I(type_0);
    iface_node = lookup_type_node_I(iface_type);
    is_a = (!node.is_null()
        && !iface_node.is_null()
        && type_node_conforms_to_U(
            node,
            iface_node,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        ) != 0) as ::core::ffi::c_int as gboolean;
    return is_a;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_children(
    mut type_0: GType,
    mut n_children: *mut guint,
) -> *mut GType {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    node = lookup_type_node_I(type_0);
    if !node.is_null() {
        let mut children: *mut GType = ::core::ptr::null_mut::<GType>();
        g_rw_lock_reader_lock(&raw mut type_rw_lock);
        children = g_malloc_n(
            (*node).n_children.wrapping_add(1 as guint) as gsize,
            ::core::mem::size_of::<GType>() as gsize,
        ) as *mut GType;
        if (*node).n_children != 0 as guint {
            memcpy(
                children as *mut ::core::ffi::c_void,
                (*node).children as *const ::core::ffi::c_void,
                (::core::mem::size_of::<GType>() as size_t)
                    .wrapping_mul((*node).n_children as size_t),
            );
        }
        *children.offset((*node).n_children as isize) = 0 as GType;
        if !n_children.is_null() {
            *n_children = (*node).n_children;
        }
        g_rw_lock_reader_unlock(&raw mut type_rw_lock);
        return children;
    } else {
        if !n_children.is_null() {
            *n_children = 0 as guint;
        }
        return ::core::ptr::null_mut::<GType>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_interfaces(
    mut type_0: GType,
    mut n_interfaces: *mut guint,
) -> *mut GType {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    node = lookup_type_node_I(type_0);
    if !node.is_null() && (*node).is_instantiatable() as ::core::ffi::c_int != 0 {
        let mut entries: *mut IFaceEntries = ::core::ptr::null_mut::<IFaceEntries>();
        let mut ifaces: *mut GType = ::core::ptr::null_mut::<GType>();
        let mut i: guint = 0;
        g_rw_lock_reader_lock(&raw mut type_rw_lock);
        entries = (*node)._prot.iface_entries.data as *mut IFaceEntries;
        if !entries.is_null() {
            ifaces = g_malloc_n(
                (*(entries as *mut GAtomicArrayMetadata)
                    .offset(-(1 as ::core::ffi::c_int as isize)))
                .size
                .wrapping_sub(
                    (::core::mem::size_of::<IFaceEntries>() as gsize)
                        .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as gsize),
                )
                .wrapping_div(::core::mem::size_of::<IFaceEntry>() as gsize)
                .wrapping_add(1 as gsize),
                ::core::mem::size_of::<GType>() as gsize,
            ) as *mut GType;
            i = 0 as guint;
            while (i as usize)
                < ((*(entries as *mut GAtomicArrayMetadata)
                    .offset(-(1 as ::core::ffi::c_int as isize)))
                .size as usize)
                    .wrapping_sub(
                        (::core::mem::size_of::<IFaceEntries>() as usize)
                            .wrapping_sub(::core::mem::size_of::<IFaceEntry>() as usize),
                    )
                    .wrapping_div(::core::mem::size_of::<IFaceEntry>() as usize)
            {
                *ifaces.offset(i as isize) =
                    (*(&raw mut (*entries).entry as *mut IFaceEntry).offset(i as isize)).iface_type;
                i = i.wrapping_add(1);
            }
        } else {
            ifaces = g_malloc_n(1 as gsize, ::core::mem::size_of::<GType>() as gsize) as *mut GType;
            i = 0 as guint;
        }
        *ifaces.offset(i as isize) = 0 as GType;
        if !n_interfaces.is_null() {
            *n_interfaces = i;
        }
        g_rw_lock_reader_unlock(&raw mut type_rw_lock);
        return ifaces;
    } else {
        if !n_interfaces.is_null() {
            *n_interfaces = 0 as guint;
        }
        return ::core::ptr::null_mut::<GType>();
    };
}
#[inline]
unsafe extern "C" fn type_get_qdata_L(mut node: *mut TypeNode, mut quark: GQuark) -> gpointer {
    let mut gdata: *mut GData = (*node).global_gdata;
    if quark != 0 && !gdata.is_null() && (*gdata).n_qdatas != 0 {
        let mut qdatas: *mut QData = (*gdata).qdatas.offset(-(1 as ::core::ffi::c_int as isize));
        let mut n_qdatas: guint = (*gdata).n_qdatas;
        loop {
            let mut i: guint = 0;
            let mut check: *mut QData = ::core::ptr::null_mut::<QData>();
            i = n_qdatas.wrapping_add(1 as guint).wrapping_div(2 as guint);
            check = qdatas.offset(i as isize);
            if quark == (*check).quark {
                return (*check).data;
            } else if quark > (*check).quark {
                n_qdatas = n_qdatas.wrapping_sub(i);
                qdatas = check;
            } else {
                n_qdatas = i.wrapping_sub(1 as guint);
            }
            if !(n_qdatas != 0) {
                break;
            }
        }
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn g_type_get_qdata(mut type_0: GType, mut quark: GQuark) -> gpointer {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    node = lookup_type_node_I(type_0);
    if !node.is_null() {
        g_rw_lock_reader_lock(&raw mut type_rw_lock);
        data = type_get_qdata_L(node, quark);
        g_rw_lock_reader_unlock(&raw mut type_rw_lock);
    } else {
        if !node.is_null() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_type_get_qdata\0" as *const u8 as *const ::core::ffi::c_char,
                b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        data = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
    }
    return data;
}
#[inline]
unsafe extern "C" fn type_set_qdata_W(
    mut node: *mut TypeNode,
    mut quark: GQuark,
    mut data: gpointer,
) {
    let mut gdata: *mut GData = ::core::ptr::null_mut::<GData>();
    let mut qdata: *mut QData = ::core::ptr::null_mut::<QData>();
    let mut i: guint = 0;
    if (*node).global_gdata.is_null() {
        (*node).global_gdata =
            g_malloc0_n(1 as gsize, ::core::mem::size_of::<GData>() as gsize) as *mut GData;
    }
    gdata = (*node).global_gdata;
    qdata = (*gdata).qdatas;
    i = 0 as guint;
    while i < (*gdata).n_qdatas {
        if (*qdata.offset(i as isize)).quark == quark {
            let ref mut fresh3 = (*qdata.offset(i as isize)).data;
            *fresh3 = data;
            return;
        }
        i = i.wrapping_add(1);
    }
    (*gdata).n_qdatas = (*gdata).n_qdatas.wrapping_add(1);
    (*gdata).qdatas = g_realloc_n(
        (*gdata).qdatas as gpointer,
        (*gdata).n_qdatas as gsize,
        ::core::mem::size_of::<QData>() as gsize,
    ) as *mut QData;
    qdata = (*gdata).qdatas;
    i = 0 as guint;
    while i < (*gdata).n_qdatas.wrapping_sub(1 as guint) {
        if (*qdata.offset(i as isize)).quark > quark {
            break;
        }
        i = i.wrapping_add(1);
    }
    memmove(
        qdata
            .offset(i as isize)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        qdata.offset(i as isize) as *const ::core::ffi::c_void,
        (::core::mem::size_of::<QData>() as size_t)
            .wrapping_mul((*gdata).n_qdatas.wrapping_sub(i).wrapping_sub(1 as guint) as size_t),
    );
    (*qdata.offset(i as isize)).quark = quark;
    let ref mut fresh4 = (*qdata.offset(i as isize)).data;
    *fresh4 = data;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_set_qdata(
    mut type_0: GType,
    mut quark: GQuark,
    mut data: gpointer,
) {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if quark != 0 as GQuark {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_set_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"quark != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    node = lookup_type_node_I(type_0);
    if !node.is_null() {
        g_rw_lock_writer_lock(&raw mut type_rw_lock);
        type_set_qdata_W(node, quark, data);
        g_rw_lock_writer_unlock(&raw mut type_rw_lock);
    } else if !node.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_set_qdata\0" as *const u8 as *const ::core::ffi::c_char,
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    };
}
unsafe extern "C" fn type_add_flags_W(mut node: *mut TypeNode, mut flags: GTypeFlags) {
    let mut dflags: guint = 0;
    if flags as ::core::ffi::c_uint
        & !(G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int
            | G_TYPE_FLAG_VALUE_ABSTRACT as ::core::ffi::c_int
            | G_TYPE_FLAG_FINAL as ::core::ffi::c_int
            | G_TYPE_FLAG_DEPRECATED as ::core::ffi::c_int) as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"type_add_flags_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"(flags & ~TYPE_FLAG_MASK) == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !node.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"type_add_flags_W\0" as *const u8 as *const ::core::ffi::c_char,
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if flags as ::core::ffi::c_uint
        & (G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int
            | G_TYPE_FLAG_VALUE_ABSTRACT as ::core::ffi::c_int
            | G_TYPE_FLAG_FINAL as ::core::ffi::c_int
            | G_TYPE_FLAG_DEPRECATED as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
        && (*node).is_classed() as ::core::ffi::c_int != 0
        && !(*node).data.is_null()
        && !(*(*node).data).class.class.is_null()
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"tagging type '%s' as abstract after class initialization\0" as *const u8
                as *const gchar,
            g_quark_to_string((*node).qname),
        );
    }
    dflags = type_get_qdata_L(node, static_quark_type_flags) as gulong as guint;
    dflags |= flags as ::core::ffi::c_uint;
    type_set_qdata_W(node, static_quark_type_flags, dflags as gulong as gpointer);
    (*node).set_is_abstract(
        (flags as ::core::ffi::c_uint
            & G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint as guint,
    );
    (*node).set_is_deprecated(
        (*node).is_deprecated()
            | (flags as ::core::ffi::c_uint
                & G_TYPE_FLAG_DEPRECATED as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint,
    );
    (*node).set_is_final(
        (flags as ::core::ffi::c_uint
            & G_TYPE_FLAG_FINAL as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int as guint as guint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn g_type_query(mut type_0: GType, mut query: *mut GTypeQuery) {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if !query.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_query\0" as *const u8 as *const ::core::ffi::c_char,
            b"query != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*query).type_0 = 0 as GType;
    node = lookup_type_node_I(type_0);
    if !node.is_null() && (*node).is_classed() as ::core::ffi::c_int != 0 {
        g_rw_lock_reader_lock(&raw mut type_rw_lock);
        if !(*node).data.is_null() {
            (*query).type_0 =
                *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize);
            (*query).type_name = g_quark_to_string((*node).qname);
            (*query).class_size = (*(*node).data).class.class_size as guint;
            (*query).instance_size = (if (*node).is_instantiatable() as ::core::ffi::c_int != 0 {
                (*(*node).data).instance.instance_size as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as guint;
        }
        g_rw_lock_reader_unlock(&raw mut type_rw_lock);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_type_get_instance_count(mut type_0: GType) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn _g_type_test_flags(mut type_0: GType, mut flags: guint) -> gboolean {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut result: gboolean = 0 as gboolean;
    node = lookup_type_node_I(type_0);
    if !node.is_null() {
        if flags
            & !(G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int
                | G_TYPE_FLAG_CLASSED as ::core::ffi::c_int
                | G_TYPE_FLAG_DEPRECATED as ::core::ffi::c_int
                | G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int
                | G_TYPE_FLAG_FINAL as ::core::ffi::c_int) as guint
            == 0 as guint
        {
            if flags & G_TYPE_FLAG_CLASSED as ::core::ffi::c_int as guint != 0
                && (*node).is_classed() == 0
            {
                return 0 as gboolean;
            }
            if flags & G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int as guint != 0
                && (*node).is_instantiatable() == 0
            {
                return 0 as gboolean;
            }
            if flags & G_TYPE_FLAG_FINAL as ::core::ffi::c_int as guint != 0
                && (*node).is_final() == 0
            {
                return 0 as gboolean;
            }
            if flags & G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int as guint != 0
                && (*node).is_abstract() == 0
            {
                return 0 as gboolean;
            }
            if flags & G_TYPE_FLAG_DEPRECATED as ::core::ffi::c_int as guint != 0
                && (*node).is_deprecated() == 0
            {
                return 0 as gboolean;
            }
            return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        }
        let mut fflags: guint = flags
            & (G_TYPE_FLAG_CLASSED as ::core::ffi::c_int
                | G_TYPE_FLAG_INSTANTIATABLE as ::core::ffi::c_int
                | G_TYPE_FLAG_DERIVABLE as ::core::ffi::c_int
                | G_TYPE_FLAG_DEEP_DERIVABLE as ::core::ffi::c_int) as guint;
        let mut tflags: guint = flags
            & (G_TYPE_FLAG_ABSTRACT as ::core::ffi::c_int
                | G_TYPE_FLAG_VALUE_ABSTRACT as ::core::ffi::c_int
                | G_TYPE_FLAG_FINAL as ::core::ffi::c_int
                | G_TYPE_FLAG_DEPRECATED as ::core::ffi::c_int) as guint;
        if fflags != 0 {
            let mut finfo: *mut GTypeFundamentalInfo = type_node_fundamental_info_I(node);
            fflags =
                ((*finfo).type_flags as guint & fflags == fflags) as ::core::ffi::c_int as guint;
        } else {
            fflags = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint;
        }
        if tflags != 0 {
            g_rw_lock_reader_lock(&raw mut type_rw_lock);
            tflags = (tflags & type_get_qdata_L(node, static_quark_type_flags) as gulong as guint
                == tflags) as ::core::ffi::c_int as guint;
            g_rw_lock_reader_unlock(&raw mut type_rw_lock);
        } else {
            tflags = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint;
        }
        result = (tflags != 0 && fflags != 0) as ::core::ffi::c_int as gboolean;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_test_flags(mut type_0: GType, mut flags: guint) -> gboolean {
    return _g_type_test_flags(type_0, flags);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_get_plugin(mut type_0: GType) -> *mut GTypePlugin {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    node = lookup_type_node_I(type_0);
    return if !node.is_null() {
        (*node).plugin
    } else {
        ::core::ptr::null_mut::<GTypePlugin>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_interface_get_plugin(
    mut instance_type: GType,
    mut interface_type: GType,
) -> *mut GTypePlugin {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut iface: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if g_type_fundamental(interface_type)
        == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_interface_get_plugin\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_INTERFACE (interface_type)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTypePlugin>();
    }
    node = lookup_type_node_I(instance_type);
    iface = lookup_type_node_I(interface_type);
    if !node.is_null() && !iface.is_null() {
        let mut iholder: *mut IFaceHolder = ::core::ptr::null_mut::<IFaceHolder>();
        let mut plugin: *mut GTypePlugin = ::core::ptr::null_mut::<GTypePlugin>();
        g_rw_lock_reader_lock(&raw mut type_rw_lock);
        iholder = type_get_qdata_L(iface, static_quark_iface_holder) as *mut IFaceHolder;
        while !iholder.is_null() && (*iholder).instance_type != instance_type {
            iholder = (*iholder).next;
        }
        plugin = if !iholder.is_null() {
            (*iholder).plugin
        } else {
            ::core::ptr::null_mut::<GTypePlugin>()
        };
        g_rw_lock_reader_unlock(&raw mut type_rw_lock);
        return plugin;
    }
    if node.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_interface_get_plugin\0" as *const u8 as *const ::core::ffi::c_char,
            b"node == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTypePlugin>();
    }
    if iface.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_interface_get_plugin\0" as *const u8 as *const ::core::ffi::c_char,
            b"iface == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTypePlugin>();
    }
    g_log(
        b"GLib-GObject\0" as *const u8 as *const gchar,
        G_LOG_LEVEL_CRITICAL,
        b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c:4081: attempt to look up plugin for invalid instance/interface type pair.\0"
            as *const u8 as *const gchar,
    );
    return ::core::ptr::null_mut::<GTypePlugin>();
}
#[no_mangle]
pub unsafe extern "C" fn g_type_fundamental_next() -> GType {
    let mut type_0: GType = 0;
    if static_quark_type_flags == 0 {
        bootstrap_type_system();
    }
    g_rw_lock_reader_lock(&raw mut type_rw_lock);
    type_0 = static_fundamental_next;
    g_rw_lock_reader_unlock(&raw mut type_rw_lock);
    type_0 = type_0 << 2 as ::core::ffi::c_int;
    return if type_0 <= ((255 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
        type_0
    } else {
        0 as GType
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_fundamental(mut type_id: GType) -> GType {
    let mut node: *mut TypeNode = lookup_type_node_I(type_id);
    return if !node.is_null() {
        *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
    } else {
        0 as GType
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_check_instance_is_a(
    mut type_instance: *mut GTypeInstance,
    mut iface_type: GType,
) -> gboolean {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut iface: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut check: gboolean = 0;
    if type_instance.is_null() || (*type_instance).g_class.is_null() {
        return 0 as gboolean;
    }
    iface = lookup_type_node_I(iface_type);
    if !iface.is_null() && (*iface).is_final() as ::core::ffi::c_int != 0 {
        return ((*(*type_instance).g_class).g_type == iface_type) as ::core::ffi::c_int;
    }
    node = lookup_type_node_I((*(*type_instance).g_class).g_type);
    check = (!node.is_null()
        && (*node).is_instantiatable() as ::core::ffi::c_int != 0
        && !iface.is_null()
        && type_node_conforms_to_U(
            node,
            iface,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            0 as gboolean,
        ) != 0) as ::core::ffi::c_int as gboolean;
    return check;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_check_instance_is_fundamentally_a(
    mut type_instance: *mut GTypeInstance,
    mut fundamental_type: GType,
) -> gboolean {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if type_instance.is_null() || (*type_instance).g_class.is_null() {
        return 0 as gboolean;
    }
    node = lookup_type_node_I((*(*type_instance).g_class).g_type);
    return (!node.is_null()
        && *(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
            == fundamental_type) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_check_class_is_a(
    mut type_class: *mut GTypeClass,
    mut is_a_type: GType,
) -> gboolean {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut iface: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut check: gboolean = 0;
    if type_class.is_null() {
        return 0 as gboolean;
    }
    node = lookup_type_node_I((*type_class).g_type);
    iface = lookup_type_node_I(is_a_type);
    check = (!node.is_null()
        && (*node).is_classed() as ::core::ffi::c_int != 0
        && !iface.is_null()
        && type_node_conforms_to_U(node, iface, 0 as gboolean, 0 as gboolean) != 0)
        as ::core::ffi::c_int as gboolean;
    return check;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_check_instance_cast(
    mut type_instance: *mut GTypeInstance,
    mut iface_type: GType,
) -> *mut GTypeInstance {
    if !type_instance.is_null() {
        if !(*type_instance).g_class.is_null() {
            let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
            let mut iface: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
            let mut is_instantiatable: gboolean = 0;
            let mut check: gboolean = 0;
            node = lookup_type_node_I((*(*type_instance).g_class).g_type);
            is_instantiatable = (!node.is_null()
                && (*node).is_instantiatable() as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int as gboolean;
            iface = lookup_type_node_I(iface_type);
            check = (is_instantiatable != 0
                && !iface.is_null()
                && type_node_conforms_to_U(
                    node,
                    iface,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                    0 as gboolean,
                ) != 0) as ::core::ffi::c_int as gboolean;
            if check != 0 {
                return type_instance;
            }
            if is_instantiatable != 0 {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"invalid cast from '%s' to '%s'\0" as *const u8 as *const gchar,
                    type_descriptive_name_I((*(*type_instance).g_class).g_type),
                    type_descriptive_name_I(iface_type),
                );
            } else {
                g_log(
                    b"GLib-GObject\0" as *const u8 as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"invalid uninstantiatable type '%s' in cast to '%s'\0" as *const u8
                        as *const gchar,
                    type_descriptive_name_I((*(*type_instance).g_class).g_type),
                    type_descriptive_name_I(iface_type),
                );
            }
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"invalid unclassed pointer in cast to '%s'\0" as *const u8 as *const gchar,
                type_descriptive_name_I(iface_type),
            );
        }
    }
    return type_instance;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_check_class_cast(
    mut type_class: *mut GTypeClass,
    mut is_a_type: GType,
) -> *mut GTypeClass {
    if !type_class.is_null() {
        let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
        let mut iface: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
        let mut is_classed: gboolean = 0;
        let mut check: gboolean = 0;
        node = lookup_type_node_I((*type_class).g_type);
        is_classed = (!node.is_null() && (*node).is_classed() as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as gboolean;
        iface = lookup_type_node_I(is_a_type);
        check = (is_classed != 0
            && !iface.is_null()
            && type_node_conforms_to_U(node, iface, 0 as gboolean, 0 as gboolean) != 0)
            as ::core::ffi::c_int as gboolean;
        if check != 0 {
            return type_class;
        }
        if is_classed != 0 {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"invalid class cast from '%s' to '%s'\0" as *const u8 as *const gchar,
                type_descriptive_name_I((*type_class).g_type),
                type_descriptive_name_I(is_a_type),
            );
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"invalid unclassed type '%s' in class cast to '%s'\0" as *const u8 as *const gchar,
                type_descriptive_name_I((*type_class).g_type),
                type_descriptive_name_I(is_a_type),
            );
        }
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"invalid class cast from (NULL) pointer to '%s'\0" as *const u8 as *const gchar,
            type_descriptive_name_I(is_a_type),
        );
    }
    return type_class;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_check_instance(mut type_instance: *mut GTypeInstance) -> gboolean {
    if !type_instance.is_null() {
        if !(*type_instance).g_class.is_null() {
            let mut node: *mut TypeNode = lookup_type_node_I((*(*type_instance).g_class).g_type);
            if !node.is_null() && (*node).is_instantiatable() as ::core::ffi::c_int != 0 {
                return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
            }
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"instance of invalid non-instantiatable type '%s'\0" as *const u8 as *const gchar,
                type_descriptive_name_I((*(*type_instance).g_class).g_type),
            );
        } else {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"instance with invalid (NULL) class pointer\0" as *const u8 as *const gchar,
            );
        }
    } else {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"invalid (NULL) pointer instance\0" as *const u8 as *const gchar,
        );
    }
    return 0 as gboolean;
}
#[inline]
unsafe extern "C" fn type_check_is_value_type_U(mut type_0: GType) -> gboolean {
    let mut tflags: GTypeFlags = G_TYPE_FLAG_VALUE_ABSTRACT;
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    node = lookup_type_node_I(type_0);
    if !node.is_null() && (*node).mutatable_check_cache() as ::core::ffi::c_int != 0 {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    g_rw_lock_reader_lock(&raw mut type_rw_lock);
    '_restart_check: while !node.is_null() {
        if !(*node).data.is_null()
            && ({
                let mut gaig_temp: gint = 0;
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                    *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                } else {
                };
                *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                    &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
                );
                gaig_temp
            }) as guint
                > 0 as guint
            && (*(*(*node).data).common.value_table).value_init.is_some()
        {
            tflags =
                type_get_qdata_L(node, static_quark_type_flags) as gulong as guint as GTypeFlags;
            break;
        } else {
            if !(*(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
                == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
            {
                break;
            }
            let mut i: guint = 0;
            i = 0 as guint;
            loop {
                if !(i < (*node).n_prerequisites()) {
                    break '_restart_check;
                }
                let mut prtype: GType = *(*node).prerequisites.offset(i as isize);
                let mut prnode: *mut TypeNode = lookup_type_node_I(prtype);
                if (*prnode).is_instantiatable() != 0 {
                    type_0 = prtype;
                    node = lookup_type_node_I(type_0);
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        }
    }
    g_rw_lock_reader_unlock(&raw mut type_rw_lock);
    return (tflags as ::core::ffi::c_uint
        & G_TYPE_FLAG_VALUE_ABSTRACT as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_check_is_value_type(mut type_0: GType) -> gboolean {
    return type_check_is_value_type_U(type_0);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_check_value(mut value: *const GValue) -> gboolean {
    return (!value.is_null() && type_check_is_value_type_U((*value).g_type) != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_check_value_holds(
    mut value: *const GValue,
    mut type_0: GType,
) -> gboolean {
    return (!value.is_null()
        && type_check_is_value_type_U((*value).g_type) != 0
        && ((*value).g_type == type_0 || g_type_is_a((*value).g_type, type_0) != 0))
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_value_table_peek(mut type_0: GType) -> *mut GTypeValueTable {
    let mut vtable: *mut GTypeValueTable = ::core::ptr::null_mut::<GTypeValueTable>();
    let mut node: *mut TypeNode = lookup_type_node_I(type_0);
    let mut has_refed_data: gboolean = 0;
    let mut has_table: gboolean = 0;
    if !node.is_null()
        && ({
            let mut gaig_temp: gint = 0;
            if 0 as ::core::ffi::c_int != 0 {
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
            } else {
            };
            *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
            );
            gaig_temp
        }) as guint
            != 0
        && (*node).mutatable_check_cache() as ::core::ffi::c_int != 0
    {
        return (*(*node).data).common.value_table;
    }
    g_rw_lock_reader_lock(&raw mut type_rw_lock);
    '_restart_table_peek: loop {
        has_refed_data = (!node.is_null()
            && !(*node).data.is_null()
            && ({
                let mut gaig_temp: gint = 0;
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                    *(&raw mut (*node).ref_count as *mut ::core::ffi::c_int);
                } else {
                };
                *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                    &raw mut (*node).ref_count as *mut ::core::ffi::c_int as *mut gint,
                );
                gaig_temp
            }) as guint
                > 0 as guint) as ::core::ffi::c_int as gboolean;
        has_table = (has_refed_data != 0
            && (*(*(*node).data).common.value_table).value_init.is_some())
            as ::core::ffi::c_int as gboolean;
        if !(has_refed_data != 0) {
            break;
        }
        if has_table != 0 {
            vtable = (*(*node).data).common.value_table;
            break;
        } else {
            if !(*(&raw mut (*node).supers as *mut GType).offset((*node).n_supers() as isize)
                == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType)
            {
                break;
            }
            let mut i: guint = 0;
            i = 0 as guint;
            loop {
                if !(i < (*node).n_prerequisites()) {
                    break '_restart_table_peek;
                }
                let mut prtype: GType = *(*node).prerequisites.offset(i as isize);
                let mut prnode: *mut TypeNode = lookup_type_node_I(prtype);
                if (*prnode).is_instantiatable() != 0 {
                    type_0 = prtype;
                    node = lookup_type_node_I(type_0);
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        }
    }
    g_rw_lock_reader_unlock(&raw mut type_rw_lock);
    if !vtable.is_null() {
        return vtable;
    }
    if node.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c:4393: type id '%lu' is invalid\0"
                as *const u8 as *const gchar,
            type_0,
        );
    }
    if has_refed_data == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"can't peek value table for type '%s' which is not currently referenced\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(type_0),
        );
    }
    return ::core::ptr::null_mut::<GTypeValueTable>();
}
#[no_mangle]
pub unsafe extern "C" fn g_type_name_from_instance(
    mut instance: *mut GTypeInstance,
) -> *const gchar {
    if instance.is_null() {
        return b"<NULL-instance>\0" as *const u8 as *const gchar;
    } else {
        return g_type_name_from_class((*instance).g_class);
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_name_from_class(mut g_class: *mut GTypeClass) -> *const gchar {
    if g_class.is_null() {
        return b"<NULL-class>\0" as *const u8 as *const gchar;
    } else {
        return g_type_name((*g_class).g_type);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _g_type_boxed_copy(mut type_0: GType, mut value: gpointer) -> gpointer {
    let mut node: *mut TypeNode = lookup_type_node_I(type_0);
    return (*(*node).data)
        .boxed
        .copy_func
        .expect("non-null function pointer")(value);
}
#[no_mangle]
pub unsafe extern "C" fn _g_type_boxed_free(mut type_0: GType, mut value: gpointer) {
    let mut node: *mut TypeNode = lookup_type_node_I(type_0);
    (*(*node).data)
        .boxed
        .free_func
        .expect("non-null function pointer")(value);
}
#[no_mangle]
pub unsafe extern "C" fn _g_type_boxed_init(
    mut type_0: GType,
    mut copy_func: GBoxedCopyFunc,
    mut free_func: GBoxedFreeFunc,
) {
    let mut node: *mut TypeNode = lookup_type_node_I(type_0);
    (*(*node).data).boxed.copy_func = copy_func;
    (*(*node).data).boxed.free_func = free_func;
}
pub unsafe fn bootstrap_type_system() {
    TYPE_SYSTEM_INIT.call_once(|| unsafe {
        let mut info: GTypeInfo = ::core::mem::zeroed();
        let mut node: *mut TypeNode;

        g_rw_lock_writer_lock(&raw mut type_rw_lock);

        static_quark_type_flags = g_quark_from_static_string(
            b"-g-type-private--GTypeFlags\0" as *const u8 as *const gchar,
        );
        static_quark_iface_holder = g_quark_from_static_string(
            b"-g-type-private--IFaceHolder\0" as *const u8 as *const gchar,
        );
        static_quark_dependants_array = g_quark_from_static_string(
            b"-g-type-private--dependants-array\0" as *const u8 as *const gchar,
        );

        static_type_nodes_ht = g_hash_table_new(Some(g_str_hash), Some(g_str_equal));
        static_fundamental_type_nodes[0] = ::core::ptr::null_mut::<TypeNode>();

        node = type_node_fundamental_new_W(
            ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            g_intern_static_string(b"void\0" as *const u8 as *const gchar),
            0 as GTypeFundamentalFlags,
        );
        debug_assert_eq!(
            *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
            ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        );

        node = type_node_fundamental_new_W(
            ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
            g_intern_static_string(b"GInterface\0" as *const u8 as *const gchar),
            G_TYPE_FLAG_DERIVABLE,
        );
        type_data_make_W(node, &raw const info, ::core::ptr::null::<GTypeValueTable>());
        debug_assert_eq!(
            *(&raw mut (*node).supers as *mut GType).offset(0 as ::core::ffi::c_int as isize),
            ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        );

        g_rw_lock_writer_unlock(&raw mut type_rw_lock);

        crate::translated::original::gobject::gvalue::_g_value_c_init();
        let _ = crate::translated::original::gobject::gtypeplugin::g_type_plugin_get_type();
        crate::translated::original::gobject::gvaluetypes::_g_value_types_init();
        crate::translated::original::gobject::genums::_g_enum_types_init();
        crate::translated::original::gobject::gboxed::_g_boxed_type_init();
        crate::translated::original::gobject::gparam::_g_param_type_init();
        crate::translated::original::gobject::gobject::_g_object_type_init();
        crate::translated::original::gobject::gparamspecs::_g_param_spec_types_init();
        crate::translated::original::gobject::gvaluetransform::_g_value_transforms_init();
        crate::translated::original::gobject::gsignal::_g_signal_init();
    });
}
#[no_mangle]
pub unsafe extern "C" fn g_type_init_with_debug_flags(mut debug_flags: GTypeDebugFlags) {
    bootstrap_type_system();
    if static_quark_type_flags != 0 {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4467 as ::core::ffi::c_int,
            b"g_type_init_with_debug_flags\0" as *const u8 as *const ::core::ffi::c_char,
            b"static_quark_type_flags\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if debug_flags as u64 != 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_MESSAGE,
            b"g_type_init_with_debug_flags() is no longer supported.  Use the GOBJECT_DEBUG environment variable.\0"
                as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_type_init() {
    bootstrap_type_system();
    if static_quark_type_flags != 0 {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4486 as ::core::ffi::c_int,
            b"g_type_init\0" as *const u8 as *const ::core::ffi::c_char,
            b"static_quark_type_flags\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_type_class_add_private(mut g_class: gpointer, mut private_size: gsize) {
    let mut instance_type: GType = (*(g_class as *mut GTypeClass)).g_type;
    let mut node: *mut TypeNode = lookup_type_node_I(instance_type);
    if private_size > 0 as gsize {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_class_add_private\0" as *const u8 as *const ::core::ffi::c_char,
            b"private_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if private_size <= 0xffff as gsize {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_class_add_private\0" as *const u8 as *const ::core::ffi::c_char,
            b"private_size <= 0xffff\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if node.is_null()
        || (*node).is_instantiatable() == 0
        || (*node).data.is_null()
        || (*(*node).data).class.class != g_class
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot add private field to invalid (non-instantiatable) type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(instance_type),
        );
        return;
    }
    if *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize) != 0 {
        let mut pnode: *mut TypeNode = lookup_type_node_I(
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
        if (*(*node).data).instance.private_size as ::core::ffi::c_int
            != (*(*pnode).data).instance.private_size as ::core::ffi::c_int
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_type_class_add_private() called multiple times for the same type\0" as *const u8
                    as *const gchar,
            );
            return;
        }
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    private_size = (((*(*node).data).instance.private_size as usize)
        .wrapping_add(private_size as usize)
        .wrapping_add(
            (2 as usize)
                .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                .wrapping_sub(1 as usize),
        )
        & (2 as usize)
            .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
            .wrapping_neg()) as gsize;
    if private_size <= 0xffff as gsize {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4778 as ::core::ffi::c_int,
            b"g_type_class_add_private\0" as *const u8 as *const ::core::ffi::c_char,
            b"private_size <= 0xffff\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(*node).data).instance.private_size = private_size as guint16;
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_add_instance_private(
    mut class_gtype: GType,
    mut private_size: gsize,
) -> gint {
    let mut node: *mut TypeNode = lookup_type_node_I(class_gtype);
    if private_size > 0 as gsize {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_add_instance_private\0" as *const u8 as *const ::core::ffi::c_char,
            b"private_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if private_size <= 0xffff as gsize {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_add_instance_private\0" as *const u8 as *const ::core::ffi::c_char,
            b"private_size <= 0xffff\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if node.is_null()
        || (*node).is_classed() == 0
        || (*node).is_instantiatable() == 0
        || (*node).data.is_null()
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot add private field to invalid (non-instantiatable) type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(class_gtype),
        );
        return 0 as gint;
    }
    if !(*node).plugin.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot use g_type_add_instance_private() with dynamic type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(class_gtype),
        );
        return 0 as gint;
    }
    return private_size as gint;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_class_adjust_private_offset(
    mut g_class: gpointer,
    mut private_size_or_offset: *mut gint,
) {
    let mut class_gtype: GType = (*(g_class as *mut GTypeClass)).g_type;
    let mut node: *mut TypeNode = lookup_type_node_I(class_gtype);
    let mut private_size: gssize = 0;
    if !private_size_or_offset.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_class_adjust_private_offset\0" as *const u8 as *const ::core::ffi::c_char,
            b"private_size_or_offset != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if *private_size_or_offset > 0 as ::core::ffi::c_int {
        if *private_size_or_offset <= 0xffff as ::core::ffi::c_int {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_type_class_adjust_private_offset\0" as *const u8 as *const ::core::ffi::c_char,
                b"*private_size_or_offset <= 0xffff\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
    } else {
        return;
    }
    if node.is_null()
        || (*node).is_classed() == 0
        || (*node).is_instantiatable() == 0
        || (*node).data.is_null()
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot add private field to invalid (non-instantiatable) type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(class_gtype),
        );
        *private_size_or_offset = 0 as ::core::ffi::c_int as gint;
        return;
    }
    if *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize) != 0 {
        let mut pnode: *mut TypeNode = lookup_type_node_I(
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
        if (*(*node).data).instance.private_size as ::core::ffi::c_int
            != (*(*pnode).data).instance.private_size as ::core::ffi::c_int
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_type_add_instance_private() called multiple times for the same type\0"
                    as *const u8 as *const gchar,
            );
            *private_size_or_offset = 0 as ::core::ffi::c_int as gint;
            return;
        }
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    private_size = ((((*(*node).data).instance.private_size as gint + *private_size_or_offset)
        as usize)
        .wrapping_add(
            (2 as usize)
                .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                .wrapping_sub(1 as usize),
        )
        & (2 as usize)
            .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
            .wrapping_neg()) as gssize;
    if private_size <= 0xffff as gssize {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4883 as ::core::ffi::c_int,
            b"g_type_class_adjust_private_offset\0" as *const u8 as *const ::core::ffi::c_char,
            b"private_size <= 0xffff\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*(*node).data).instance.private_size = private_size as guint16;
    *private_size_or_offset = -((*(*node).data).instance.private_size as gint);
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_instance_get_private(
    mut instance: *mut GTypeInstance,
    mut private_type: GType,
) -> gpointer {
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if !instance.is_null() && !(*instance).g_class.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_instance_get_private\0" as *const u8 as *const ::core::ffi::c_char,
            b"instance != NULL && instance->g_class != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    node = lookup_type_node_I(private_type);
    if node.is_null() || (*node).is_instantiatable() == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"instance of invalid non-instantiatable type '%s'\0" as *const u8 as *const gchar,
            type_descriptive_name_I((*(*instance).g_class).g_type),
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (instance as *mut gchar)
        .offset(-((*(*node).data).instance.private_size as ::core::ffi::c_int as isize))
        as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_class_get_instance_private_offset(mut g_class: gpointer) -> gint {
    let mut instance_type: GType = 0;
    let mut parent_size: guint16 = 0;
    let mut node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    if !g_class.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4934 as ::core::ffi::c_int,
            b"g_type_class_get_instance_private_offset\0" as *const u8
                as *const ::core::ffi::c_char,
            b"g_class != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    instance_type = (*(g_class as *mut GTypeClass)).g_type;
    node = lookup_type_node_I(instance_type);
    if !node.is_null() {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4939 as ::core::ffi::c_int,
            b"g_type_class_get_instance_private_offset\0" as *const u8
                as *const ::core::ffi::c_char,
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*node).is_instantiatable() != 0 {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                as *const ::core::ffi::c_char,
            4940 as ::core::ffi::c_int,
            b"g_type_class_get_instance_private_offset\0" as *const u8
                as *const ::core::ffi::c_char,
            b"node->is_instantiatable\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize) != 0 {
        let mut pnode: *mut TypeNode = lookup_type_node_I(
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
        parent_size = (*(*pnode).data).instance.private_size;
    } else {
        parent_size = 0 as guint16;
    }
    if (*(*node).data).instance.private_size as ::core::ffi::c_int
        == parent_size as ::core::ffi::c_int
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"g_type_class_get_instance_private_offset() called on class %s but it has no private data\0"
                as *const u8 as *const gchar,
            g_type_name(instance_type),
        );
        loop {}
    }
    return -((*(*node).data).instance.private_size as gint);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_add_class_private(mut class_type: GType, mut private_size: gsize) {
    let mut node: *mut TypeNode = lookup_type_node_I(class_type);
    let mut offset: gsize = 0;
    if private_size > 0 as gsize {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_add_class_private\0" as *const u8 as *const ::core::ffi::c_char,
            b"private_size > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if node.is_null() || (*node).is_classed() == 0 || (*node).data.is_null() {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"cannot add class private field to invalid type '%s'\0" as *const u8 as *const gchar,
            type_descriptive_name_I(class_type),
        );
        return;
    }
    if *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize) != 0 {
        let mut pnode: *mut TypeNode = lookup_type_node_I(
            *(&raw mut (*node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize),
        );
        if (*(*node).data).class.class_private_size as ::core::ffi::c_int
            != (*(*pnode).data).class.class_private_size as ::core::ffi::c_int
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_type_add_class_private() called multiple times for the same type\0" as *const u8
                    as *const gchar,
            );
            return;
        }
    }
    g_rw_lock_writer_lock(&raw mut type_rw_lock);
    offset = (((*(*node).data).class.class_private_size as usize).wrapping_add(
        (2 as usize)
            .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
            .wrapping_sub(1 as usize),
    ) & (2 as usize)
        .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
        .wrapping_neg()) as gsize;
    (*(*node).data).class.class_private_size = offset.wrapping_add(private_size) as guint16;
    g_rw_lock_writer_unlock(&raw mut type_rw_lock);
}
#[no_mangle]
pub unsafe extern "C" fn g_type_class_get_private(
    mut klass: *mut GTypeClass,
    mut private_type: GType,
) -> gpointer {
    let mut class_node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut private_node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut parent_node: *mut TypeNode = ::core::ptr::null_mut::<TypeNode>();
    let mut offset: gsize = 0;
    if !klass.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_type_class_get_private\0" as *const u8 as *const ::core::ffi::c_char,
            b"klass != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    class_node = lookup_type_node_I((*klass).g_type);
    if class_node.is_null() || (*class_node).is_classed() == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"class of invalid type '%s'\0" as *const u8 as *const gchar,
            type_descriptive_name_I((*klass).g_type),
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    private_node = lookup_type_node_I(private_type);
    if private_node.is_null()
        || !((*private_node).n_supers() as ::core::ffi::c_int
            <= (*class_node).n_supers() as ::core::ffi::c_int
            && *(&raw mut (*class_node).supers as *mut GType).offset(
                ((*class_node).n_supers() as ::core::ffi::c_int
                    - (*private_node).n_supers() as ::core::ffi::c_int) as isize,
            ) == *(&raw mut (*private_node).supers as *mut GType)
                .offset(0 as ::core::ffi::c_int as isize))
    {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"attempt to retrieve private data for invalid type '%s'\0" as *const u8
                as *const gchar,
            type_descriptive_name_I(private_type),
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    offset = (((*(*class_node).data).class.class_size as usize).wrapping_add(
        (2 as usize)
            .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
            .wrapping_sub(1 as usize),
    ) & (2 as usize)
        .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
        .wrapping_neg()) as gsize;
    if *(&raw mut (*private_node).supers as *mut GType).offset(1 as ::core::ffi::c_int as isize)
        != 0
    {
        parent_node = lookup_type_node_I(
            *(&raw mut (*private_node).supers as *mut GType)
                .offset(1 as ::core::ffi::c_int as isize),
        );
        if !(*parent_node).data.is_null()
            && ({
                let mut gaig_temp: gint = 0;
                if 0 as ::core::ffi::c_int != 0 {
                    *(&raw mut (*parent_node).ref_count as *mut ::core::ffi::c_int);
                    *(&raw mut (*parent_node).ref_count as *mut ::core::ffi::c_int);
                } else {
                };
                *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
                    &raw mut (*parent_node).ref_count as *mut ::core::ffi::c_int as *mut gint,
                );
                gaig_temp
            }) as guint
                > 0 as guint
        {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gtype.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                5042 as ::core::ffi::c_int,
                b"g_type_class_get_private\0" as *const u8 as *const ::core::ffi::c_char,
                b"parent_node->data && NODE_REFCOUNT (parent_node) > 0\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if (*(*private_node).data).class.class_private_size as ::core::ffi::c_int
            == (*(*parent_node).data).class.class_private_size as ::core::ffi::c_int
        {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"g_type_instance_get_class_private() requires a prior call to g_type_add_class_private()\0"
                    as *const u8 as *const gchar,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_void>();
        }
        offset = (offset as ::core::ffi::c_ulong).wrapping_add(
            (((*(*parent_node).data).class.class_private_size as usize).wrapping_add(
                (2 as usize)
                    .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                    .wrapping_sub(1 as usize),
            ) & (2 as usize)
                .wrapping_mul(::core::mem::size_of::<gsize>() as usize)
                .wrapping_neg()) as ::core::ffi::c_ulong,
        ) as gsize as gsize;
    }
    return (klass as *mut guint8).offset(offset as glong as isize) as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn g_type_ensure(mut type_0: GType) {
    if type_0 == -(1 as ::core::ffi::c_int) as GType {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_ERROR,
            b"can't happen\0" as *const u8 as *const gchar,
        );
        loop {}
    }
}
