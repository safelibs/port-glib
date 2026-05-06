extern "C" {
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GVariant;
    pub type _GCancellable;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GFileAttributeMatcher;
    pub type _GFileAttributeInfoList;
    pub type _GIcon;
    pub type _GWakeup;
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
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_rw_lock_writer_lock(rw_lock: *mut GRWLock);
    fn g_rw_lock_writer_unlock(rw_lock: *mut GRWLock);
    fn g_rw_lock_reader_lock(rw_lock: *mut GRWLock);
    fn g_rw_lock_reader_unlock(rw_lock: *mut GRWLock);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
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
    fn g_uri_parse_scheme(uri: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    fn glib__private__() -> *const GLibPrivateVTable;
    fn _g_local_vfs_new() -> *mut GVfs;
    fn _g_resource_file_new(uri: *const ::core::ffi::c_char) -> *mut GFile;
    fn _g_io_module_get_default(
        extension_point: *const gchar,
        envvar: *const gchar,
        verify_func: GIOModuleVerifyFunc,
    ) -> gpointer;
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
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
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRWLock {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GRWLock = _GRWLock;
pub type GData = _GData;
pub type GDir = _GDir;
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
pub type GVariant = _GVariant;
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
pub type GFileQueryInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS: GFileQueryInfoFlags = 1;
pub const G_FILE_QUERY_INFO_NONE: GFileQueryInfoFlags = 0;
pub type GCancellable = _GCancellable;
pub type GFile = _GFile;
pub type GFileInfo = _GFileInfo;
pub type GFileAttributeMatcher = _GFileAttributeMatcher;
pub type GFileAttributeInfoList = _GFileAttributeInfoList;
pub type GIcon = _GIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVfs {
    pub parent_instance: GObject,
}
pub type GVfs = _GVfs;
pub type GVfsFileLookupFunc =
    Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char, gpointer) -> *mut GFile>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVfsClass {
    pub parent_class: GObjectClass,
    pub is_active: Option<unsafe extern "C" fn(*mut GVfs) -> gboolean>,
    pub get_file_for_path:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>,
    pub get_file_for_uri:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>,
    pub get_supported_uri_schemes: Option<unsafe extern "C" fn(*mut GVfs) -> *const *const gchar>,
    pub parse_name:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> *mut GFile>,
    pub local_file_add_info: Option<
        unsafe extern "C" fn(
            *mut GVfs,
            *const ::core::ffi::c_char,
            guint64,
            *mut GFileAttributeMatcher,
            *mut GFileInfo,
            *mut GCancellable,
            *mut gpointer,
            *mut GDestroyNotify,
        ) -> (),
    >,
    pub add_writable_namespaces:
        Option<unsafe extern "C" fn(*mut GVfs, *mut GFileAttributeInfoList) -> ()>,
    pub local_file_set_attributes: Option<
        unsafe extern "C" fn(
            *mut GVfs,
            *const ::core::ffi::c_char,
            *mut GFileInfo,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub local_file_removed:
        Option<unsafe extern "C" fn(*mut GVfs, *const ::core::ffi::c_char) -> ()>,
    pub local_file_moved: Option<
        unsafe extern "C" fn(
            *mut GVfs,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> (),
    >,
    pub deserialize_icon: Option<unsafe extern "C" fn(*mut GVfs, *mut GVariant) -> *mut GIcon>,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
}
pub type GVfsClass = _GVfsClass;
pub type GVfsPrivate = _GVfsPrivate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVfsPrivate {
    pub additional_schemes: *mut GHashTable,
    pub supported_schemes: *mut *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GVfsURISchemeData {
    pub uri_func: GVfsFileLookupFunc,
    pub uri_data: gpointer,
    pub uri_destroy: GDestroyNotify,
    pub parse_name_func: GVfsFileLookupFunc,
    pub parse_name_data: gpointer,
    pub parse_name_destroy: GDestroyNotify,
}
pub type GIOModuleVerifyFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLibPrivateVTable {
    pub g_wakeup_new: Option<unsafe extern "C" fn() -> *mut GWakeup>,
    pub g_wakeup_free: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_get_pollfd: Option<unsafe extern "C" fn(*mut GWakeup, *mut GPollFD) -> ()>,
    pub g_wakeup_signal: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_wakeup_acknowledge: Option<unsafe extern "C" fn(*mut GWakeup) -> ()>,
    pub g_get_worker_context: Option<unsafe extern "C" fn() -> *mut GMainContext>,
    pub g_check_setuid: Option<unsafe extern "C" fn() -> gboolean>,
    pub g_main_context_new_with_next_id: Option<unsafe extern "C" fn(guint) -> *mut GMainContext>,
    pub g_dir_open_with_errno: Option<unsafe extern "C" fn(*const gchar, guint) -> *mut GDir>,
    pub g_dir_new_from_dirp: Option<unsafe extern "C" fn(gpointer) -> *mut GDir>,
    pub glib_init: Option<unsafe extern "C" fn() -> ()>,
    pub g_win32_push_empty_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_win32_pop_invalid_parameter_handler:
        Option<unsafe extern "C" fn(*mut GWin32InvalidParameterHandler) -> ()>,
    pub g_find_program_for_path: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub g_uri_get_default_scheme_port:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub g_set_prgname_once: Option<unsafe extern "C" fn(*const gchar) -> gboolean>,
    pub g_datalist_id_update_atomic: Option<
        unsafe extern "C" fn(
            *mut *mut GData,
            GQuark,
            GDataListUpdateAtomicFunc,
            gpointer,
        ) -> gpointer,
    >,
}
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWakeup = _GWakeup;
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
pub const G_VFS_EXTENSION_POINT_NAME: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"gio-vfs\0") };
static mut safe_c2rust_additional_schemes_lock: GRWLock = _GRWLock {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    i: [0; 2],
};
static mut safe_c2rust_GVfs_private_offset: gint = 0;
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_vfs_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GVfs\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GVfsClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_vfs_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GVfs>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVfs) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_vfs_init as unsafe extern "C" fn(*mut GVfs) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    safe_c2rust_GVfs_private_offset = g_type_add_instance_private(
        g_define_type_id,
        ::core::mem::size_of::<GVfsPrivate>() as gsize,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_vfs_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_vfs_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_vfs_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GVfs_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GVfs_private_offset);
    }
    safe_c2rust_g_vfs_class_init(klass as *mut GVfsClass);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfs_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_vfs_get_type_once();
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
#[inline]
unsafe extern "C" fn safe_c2rust_g_vfs_get_instance_private(mut self_0: *mut GVfs) -> gpointer {
    return (self_0 as *mut guint8).offset(safe_c2rust_GVfs_private_offset as glong as isize)
        as gpointer;
}
unsafe extern "C" fn safe_c2rust_g_vfs_dispose(mut object: *mut GObject) {
    let mut vfs: *mut GVfs = object as *mut ::core::ffi::c_void as *mut GVfs;
    let mut priv_0: *mut GVfsPrivate =
        safe_c2rust_g_vfs_get_instance_private(vfs) as *mut GVfsPrivate;
    let mut _pp: *mut *mut GHashTable = &raw mut (*priv_0).additional_schemes;
    let mut _ptr: *mut GHashTable = *_pp;
    *_pp = ::core::ptr::null_mut::<GHashTable>();
    if !_ptr.is_null() {
        g_hash_table_destroy(_ptr as *mut GHashTable);
    }
    let mut _pp_0: *mut *mut *const ::core::ffi::c_char = &raw mut (*priv_0).supported_schemes;
    let mut _ptr_0: *mut *const ::core::ffi::c_char = *_pp_0;
    *_pp_0 = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    if !_ptr_0.is_null() {
        g_free(_ptr_0 as gpointer);
    }
    (*(safe_c2rust_g_vfs_parent_class as *mut GObjectClass))
        .dispose
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_vfs_class_init(mut klass: *mut GVfsClass) {
    let mut object_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    (*object_class).dispose =
        Some(safe_c2rust_g_vfs_dispose as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
}
unsafe extern "C" fn safe_c2rust_resource_parse_name(
    mut vfs: *mut GVfs,
    mut parse_name: *const ::core::ffi::c_char,
    mut user_data: gpointer,
) -> *mut GFile {
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = parse_name;
            let __prefix: *const ::core::ffi::c_char =
                b"resource:\0" as *const u8 as *const ::core::ffi::c_char;
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
        g_str_has_prefix(
            parse_name as *const gchar,
            b"resource:\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        return _g_resource_file_new(parse_name);
    }
    return ::core::ptr::null_mut::<GFile>();
}
unsafe extern "C" fn safe_c2rust_resource_get_file_for_uri(
    mut vfs: *mut GVfs,
    mut uri: *const ::core::ffi::c_char,
    mut user_data: gpointer,
) -> *mut GFile {
    return _g_resource_file_new(uri);
}
unsafe extern "C" fn safe_c2rust_g_vfs_uri_lookup_func_closure_free(mut data: gpointer) {
    let mut closure: *mut GVfsURISchemeData = data as *mut GVfsURISchemeData;
    if (*closure).uri_destroy.is_some() {
        (*closure).uri_destroy.expect("non-null function pointer")((*closure).uri_data);
    }
    if (*closure).parse_name_destroy.is_some() {
        (*closure)
            .parse_name_destroy
            .expect("non-null function pointer")((*closure).parse_name_data);
    }
    g_free(closure as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_vfs_init(mut vfs: *mut GVfs) {
    let mut priv_0: *mut GVfsPrivate =
        safe_c2rust_g_vfs_get_instance_private(vfs) as *mut GVfsPrivate;
    (*priv_0).additional_schemes = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        Some(
            safe_c2rust_g_vfs_uri_lookup_func_closure_free as unsafe extern "C" fn(gpointer) -> (),
        ),
    );
    safe_c2rust_g_vfs_register_uri_scheme(
        vfs,
        b"resource\0" as *const u8 as *const ::core::ffi::c_char,
        Some(
            safe_c2rust_resource_get_file_for_uri
                as unsafe extern "C" fn(
                    *mut GVfs,
                    *const ::core::ffi::c_char,
                    gpointer,
                ) -> *mut GFile,
        ),
        NULL_0,
        None,
        Some(
            safe_c2rust_resource_parse_name
                as unsafe extern "C" fn(
                    *mut GVfs,
                    *const ::core::ffi::c_char,
                    gpointer,
                ) -> *mut GFile,
        ),
        NULL_0,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfs_is_active(mut vfs: *mut GVfs) -> gboolean {
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = vfs as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_vfs_get_type();
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
            b"G_IS_VFS (vfs)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    return Some((*class).is_active.expect("non-null function pointer"))
        .expect("non-null function pointer")(vfs);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfs_get_file_for_path(
    mut vfs: *mut GVfs,
    mut path: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = vfs as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_vfs_get_type();
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
            b"G_IS_VFS (vfs)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !path.is_null() {
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
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    return Some(
        (*class)
            .get_file_for_path
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(vfs, path);
}
unsafe extern "C" fn safe_c2rust_parse_name_internal(
    mut vfs: *mut GVfs,
    mut parse_name: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut priv_0: *mut GVfsPrivate =
        safe_c2rust_g_vfs_get_instance_private(vfs) as *mut GVfsPrivate;
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut closure: *mut GVfsURISchemeData = ::core::ptr::null_mut::<GVfsURISchemeData>();
    let mut ret: *mut GFile = ::core::ptr::null_mut::<GFile>();
    g_rw_lock_reader_lock(&raw mut safe_c2rust_additional_schemes_lock);
    g_hash_table_iter_init(&raw mut iter, (*priv_0).additional_schemes);
    while g_hash_table_iter_next(
        &raw mut iter,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut closure as *mut gpointer,
    ) != 0
    {
        ret = (*closure)
            .parse_name_func
            .expect("non-null function pointer")(
            vfs, parse_name, (*closure).parse_name_data
        );
        if !ret.is_null() {
            break;
        }
    }
    g_rw_lock_reader_unlock(&raw mut safe_c2rust_additional_schemes_lock);
    return ret;
}
unsafe extern "C" fn safe_c2rust_get_file_for_uri_internal(
    mut vfs: *mut GVfs,
    mut uri: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut priv_0: *mut GVfsPrivate =
        safe_c2rust_g_vfs_get_instance_private(vfs) as *mut GVfsPrivate;
    let mut ret: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut scheme: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut closure: *mut GVfsURISchemeData = ::core::ptr::null_mut::<GVfsURISchemeData>();
    scheme = g_uri_parse_scheme(uri);
    if scheme.is_null() {
        return ::core::ptr::null_mut::<GFile>();
    }
    g_rw_lock_reader_lock(&raw mut safe_c2rust_additional_schemes_lock);
    closure = g_hash_table_lookup((*priv_0).additional_schemes, scheme as gconstpointer)
        as *mut GVfsURISchemeData;
    if !closure.is_null() {
        ret =
            (*closure).uri_func.expect("non-null function pointer")(vfs, uri, (*closure).uri_data);
    }
    g_rw_lock_reader_unlock(&raw mut safe_c2rust_additional_schemes_lock);
    g_free(scheme as gpointer);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfs_get_file_for_uri(
    mut vfs: *mut GVfs,
    mut uri: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    let mut ret: *mut GFile = ::core::ptr::null_mut::<GFile>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = vfs as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_vfs_get_type();
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
            b"G_IS_VFS (vfs)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !uri.is_null() {
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
            b"uri != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    ret = safe_c2rust_get_file_for_uri_internal(vfs, uri);
    if ret.is_null() {
        ret = Some(
            (*class)
                .get_file_for_uri
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(vfs, uri);
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !ret.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gvfs.c\0" as *const u8
                as *const ::core::ffi::c_char,
            249 as ::core::ffi::c_int,
            G_STRFUNC,
            b"ret != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return safe_c2rust_g_steal_pointer(&raw mut ret as gpointer) as *mut GFile;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfs_get_supported_uri_schemes(
    mut vfs: *mut GVfs,
) -> *const *const gchar {
    let mut priv_0: *mut GVfsPrivate = ::core::ptr::null_mut::<GVfsPrivate>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = vfs as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_vfs_get_type();
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
            b"G_IS_VFS (vfs)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<*const gchar>();
    }
    priv_0 = safe_c2rust_g_vfs_get_instance_private(vfs) as *mut GVfsPrivate;
    if (*priv_0).supported_schemes.is_null() {
        let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
        let mut default_schemes: *const *const ::core::ffi::c_char =
            ::core::ptr::null::<*const ::core::ffi::c_char>();
        let mut additional_scheme: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        let mut supported_schemes: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
        let mut iter: GHashTableIter = _GHashTableIter {
            dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            dummy4: 0,
            dummy5: 0,
            dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
        default_schemes = Some(
            (*class)
                .get_supported_uri_schemes
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(vfs)
            as *const *const ::core::ffi::c_char;
        supported_schemes = g_ptr_array_new();
        while !default_schemes.is_null() && !(*default_schemes).is_null() {
            g_ptr_array_add(supported_schemes, *default_schemes as gpointer);
            default_schemes = default_schemes.offset(1);
        }
        g_rw_lock_reader_lock(&raw mut safe_c2rust_additional_schemes_lock);
        g_hash_table_iter_init(&raw mut iter, (*priv_0).additional_schemes);
        while g_hash_table_iter_next(
            &raw mut iter,
            &raw mut additional_scheme as *mut gpointer,
            ::core::ptr::null_mut::<gpointer>(),
        ) != 0
        {
            g_ptr_array_add(supported_schemes, additional_scheme as gpointer);
        }
        g_rw_lock_reader_unlock(&raw mut safe_c2rust_additional_schemes_lock);
        g_ptr_array_add(supported_schemes, NULL_0);
        g_free((*priv_0).supported_schemes as gpointer);
        (*priv_0).supported_schemes =
            g_ptr_array_free(supported_schemes, FALSE) as *mut *const ::core::ffi::c_char;
    }
    return (*priv_0).supported_schemes as *const *const gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfs_parse_name(
    mut vfs: *mut GVfs,
    mut parse_name: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    let mut ret: *mut GFile = ::core::ptr::null_mut::<GFile>();
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = vfs as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_vfs_get_type();
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
            b"G_IS_VFS (vfs)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !parse_name.is_null() {
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
            b"parse_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    ret = safe_c2rust_parse_name_internal(vfs, parse_name);
    if !ret.is_null() {
        return ret;
    }
    return Some((*class).parse_name.expect("non-null function pointer"))
        .expect("non-null function pointer")(vfs, parse_name);
}
static mut safe_c2rust_vfs_default_singleton: *mut GVfs = ::core::ptr::null::<GVfs>() as *mut GVfs;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfs_get_default() -> *mut GVfs {
    if (*glib__private__())
        .g_check_setuid
        .expect("non-null function pointer")()
        != 0
    {
        return safe_c2rust_g_vfs_get_local();
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_vfs_default_singleton;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
            let mut gapg_temp_atomic: *mut *mut GVfs = &raw mut safe_c2rust_vfs_default_singleton;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_vfs_default_singleton as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut singleton: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
        singleton = _g_io_module_get_default(
            G_VFS_EXTENSION_POINT_NAME.as_ptr() as *const gchar,
            b"GIO_USE_VFS\0" as *const u8 as *const gchar,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GVfs) -> gboolean>,
                GIOModuleVerifyFunc,
            >(Some(
                safe_c2rust_g_vfs_is_active as unsafe extern "C" fn(*mut GVfs) -> gboolean,
            )),
        ) as *mut GVfs;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_vfs_default_singleton = singleton;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_vfs_default_singleton as *mut ::core::ffi::c_void,
            singleton as guintptr as gpointer,
        );
    }
    return safe_c2rust_vfs_default_singleton;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfs_get_local() -> *mut GVfs {
    static mut safe_c2rust_vfs: *mut GVfs = ::core::ptr::null::<GVfs>() as *mut GVfs;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_vfs;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
            let mut gapg_temp_atomic: *mut *mut GVfs = &raw mut safe_c2rust_vfs;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(&raw mut safe_c2rust_vfs as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_vfs = _g_local_vfs_new();
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_vfs as *mut ::core::ffi::c_void,
            _g_local_vfs_new() as guintptr as gpointer,
        );
    }
    return safe_c2rust_vfs;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfs_register_uri_scheme(
    mut vfs: *mut GVfs,
    mut scheme: *const ::core::ffi::c_char,
    mut uri_func: GVfsFileLookupFunc,
    mut uri_data: gpointer,
    mut uri_destroy: GDestroyNotify,
    mut parse_name_func: GVfsFileLookupFunc,
    mut parse_name_data: gpointer,
    mut parse_name_destroy: GDestroyNotify,
) -> gboolean {
    let mut priv_0: *mut GVfsPrivate = ::core::ptr::null_mut::<GVfsPrivate>();
    let mut closure: *mut GVfsURISchemeData = ::core::ptr::null_mut::<GVfsURISchemeData>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = vfs as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_vfs_get_type();
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
            b"G_IS_VFS (vfs)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !scheme.is_null() {
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
            b"scheme != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    priv_0 = safe_c2rust_g_vfs_get_instance_private(vfs) as *mut GVfsPrivate;
    g_rw_lock_reader_lock(&raw mut safe_c2rust_additional_schemes_lock);
    closure = g_hash_table_lookup((*priv_0).additional_schemes, scheme as gconstpointer)
        as *mut GVfsURISchemeData;
    g_rw_lock_reader_unlock(&raw mut safe_c2rust_additional_schemes_lock);
    if !closure.is_null() {
        return FALSE;
    }
    closure = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GVfsURISchemeData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GVfsURISchemeData;
    (*closure).uri_func = uri_func;
    (*closure).uri_data = uri_data;
    (*closure).uri_destroy = uri_destroy;
    (*closure).parse_name_func = parse_name_func;
    (*closure).parse_name_data = parse_name_data;
    (*closure).parse_name_destroy = parse_name_destroy;
    g_rw_lock_writer_lock(&raw mut safe_c2rust_additional_schemes_lock);
    g_hash_table_insert(
        (*priv_0).additional_schemes,
        safe_c2rust_g_strdup_inline(scheme) as gpointer,
        closure as gpointer,
    );
    g_rw_lock_writer_unlock(&raw mut safe_c2rust_additional_schemes_lock);
    let mut _pp: *mut *mut *const ::core::ffi::c_char = &raw mut (*priv_0).supported_schemes;
    let mut _ptr: *mut *const ::core::ffi::c_char = *_pp;
    *_pp = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    if !_ptr.is_null() {
        g_free(_ptr as gpointer);
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_vfs_unregister_uri_scheme(
    mut vfs: *mut GVfs,
    mut scheme: *const ::core::ffi::c_char,
) -> gboolean {
    let mut priv_0: *mut GVfsPrivate = ::core::ptr::null_mut::<GVfsPrivate>();
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if ({
            let mut __inst: *mut GTypeInstance = vfs as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust_g_vfs_get_type();
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
            b"G_IS_VFS (vfs)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !scheme.is_null() {
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
            b"scheme != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    priv_0 = safe_c2rust_g_vfs_get_instance_private(vfs) as *mut GVfsPrivate;
    g_rw_lock_writer_lock(&raw mut safe_c2rust_additional_schemes_lock);
    res = g_hash_table_remove((*priv_0).additional_schemes, scheme as gconstpointer);
    g_rw_lock_writer_unlock(&raw mut safe_c2rust_additional_schemes_lock);
    if res != 0 {
        let mut _pp: *mut *mut *const ::core::ffi::c_char = &raw mut (*priv_0).supported_schemes;
        let mut _ptr: *mut *const ::core::ffi::c_char = *_pp;
        *_pp = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        if !_ptr.is_null() {
            g_free(_ptr as gpointer);
        }
        return TRUE;
    }
    return FALSE;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
