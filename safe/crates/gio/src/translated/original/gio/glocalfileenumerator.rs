extern "C" {
    pub type _GData;
    pub type __dirstream;
    pub type _GAsyncResult;
    pub type _GCancellablePrivate;
    pub type _GFileEnumeratorPrivate;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GFileAttributeMatcher;
    pub type _GLocalFile;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_filename_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_file_enumerator_get_type() -> GType;
    fn g_file_info_set_file_type(info: *mut GFileInfo, type_0: GFileType);
    fn g_file_info_set_is_symlink(info: *mut GFileInfo, is_symlink: gboolean);
    fn g_file_attribute_matcher_new(
        attributes: *const ::core::ffi::c_char,
    ) -> *mut GFileAttributeMatcher;
    fn g_file_attribute_matcher_unref(matcher: *mut GFileAttributeMatcher);
    fn g_file_attribute_matcher_subtract(
        matcher: *mut GFileAttributeMatcher,
        subtract: *mut GFileAttributeMatcher,
    ) -> *mut GFileAttributeMatcher;
    fn g_file_get_path(file: *mut GFile) -> *mut ::core::ffi::c_char;
    fn _g_local_file_info_get_parent_info(
        dir: *const ::core::ffi::c_char,
        attribute_matcher: *mut GFileAttributeMatcher,
        parent_info: *mut GLocalParentFileInfo,
    );
    fn _g_local_file_info_free_parent_info(parent_info: *mut GLocalParentFileInfo);
    fn _g_local_file_info_get_nostat(
        info: *mut GFileInfo,
        basename: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        attribute_matcher: *mut GFileAttributeMatcher,
    );
    fn _g_local_file_info_get(
        basename: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        attribute_matcher: *mut GFileAttributeMatcher,
        flags: GFileQueryInfoFlags,
        parent_info: *mut GLocalParentFileInfo,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __off64_t = ::core::ffi::c_long;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const DT_WHT: C2RustUnnamed = 14;
pub const DT_SOCK: C2RustUnnamed = 12;
pub const DT_LNK: C2RustUnnamed = 10;
pub const DT_REG: C2RustUnnamed = 8;
pub const DT_BLK: C2RustUnnamed = 6;
pub const DT_DIR: C2RustUnnamed = 4;
pub const DT_CHR: C2RustUnnamed = 2;
pub const DT_FIFO: C2RustUnnamed = 1;
pub const DT_UNKNOWN: C2RustUnnamed = 0;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
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
pub type GFileType = ::core::ffi::c_uint;
pub const G_FILE_TYPE_MOUNTABLE: GFileType = 6;
pub const G_FILE_TYPE_SHORTCUT: GFileType = 5;
pub const G_FILE_TYPE_SPECIAL: GFileType = 4;
pub const G_FILE_TYPE_SYMBOLIC_LINK: GFileType = 3;
pub const G_FILE_TYPE_DIRECTORY: GFileType = 2;
pub const G_FILE_TYPE_REGULAR: GFileType = 1;
pub const G_FILE_TYPE_UNKNOWN: GFileType = 0;
pub type GIOErrorEnum = ::core::ffi::c_uint;
pub const G_IO_ERROR_DESTINATION_UNSET: GIOErrorEnum = 48;
pub const G_IO_ERROR_NO_SUCH_DEVICE: GIOErrorEnum = 47;
pub const G_IO_ERROR_MESSAGE_TOO_LARGE: GIOErrorEnum = 46;
pub const G_IO_ERROR_NOT_CONNECTED: GIOErrorEnum = 45;
pub const G_IO_ERROR_CONNECTION_CLOSED: GIOErrorEnum = 44;
pub const G_IO_ERROR_BROKEN_PIPE: GIOErrorEnum = 44;
pub const G_IO_ERROR_PROXY_NOT_ALLOWED: GIOErrorEnum = 43;
pub const G_IO_ERROR_PROXY_NEED_AUTH: GIOErrorEnum = 42;
pub const G_IO_ERROR_PROXY_AUTH_FAILED: GIOErrorEnum = 41;
pub const G_IO_ERROR_PROXY_FAILED: GIOErrorEnum = 40;
pub const G_IO_ERROR_CONNECTION_REFUSED: GIOErrorEnum = 39;
pub const G_IO_ERROR_NETWORK_UNREACHABLE: GIOErrorEnum = 38;
pub const G_IO_ERROR_HOST_UNREACHABLE: GIOErrorEnum = 37;
pub const G_IO_ERROR_DBUS_ERROR: GIOErrorEnum = 36;
pub const G_IO_ERROR_INVALID_DATA: GIOErrorEnum = 35;
pub const G_IO_ERROR_PARTIAL_INPUT: GIOErrorEnum = 34;
pub const G_IO_ERROR_ADDRESS_IN_USE: GIOErrorEnum = 33;
pub const G_IO_ERROR_NOT_INITIALIZED: GIOErrorEnum = 32;
pub const G_IO_ERROR_TOO_MANY_OPEN_FILES: GIOErrorEnum = 31;
pub const G_IO_ERROR_FAILED_HANDLED: GIOErrorEnum = 30;
pub const G_IO_ERROR_WOULD_MERGE: GIOErrorEnum = 29;
pub const G_IO_ERROR_HOST_NOT_FOUND: GIOErrorEnum = 28;
pub const G_IO_ERROR_WOULD_BLOCK: GIOErrorEnum = 27;
pub const G_IO_ERROR_BUSY: GIOErrorEnum = 26;
pub const G_IO_ERROR_WOULD_RECURSE: GIOErrorEnum = 25;
pub const G_IO_ERROR_TIMED_OUT: GIOErrorEnum = 24;
pub const G_IO_ERROR_WRONG_ETAG: GIOErrorEnum = 23;
pub const G_IO_ERROR_CANT_CREATE_BACKUP: GIOErrorEnum = 22;
pub const G_IO_ERROR_READ_ONLY: GIOErrorEnum = 21;
pub const G_IO_ERROR_PENDING: GIOErrorEnum = 20;
pub const G_IO_ERROR_CANCELLED: GIOErrorEnum = 19;
pub const G_IO_ERROR_CLOSED: GIOErrorEnum = 18;
pub const G_IO_ERROR_ALREADY_MOUNTED: GIOErrorEnum = 17;
pub const G_IO_ERROR_NOT_MOUNTED: GIOErrorEnum = 16;
pub const G_IO_ERROR_NOT_SUPPORTED: GIOErrorEnum = 15;
pub const G_IO_ERROR_PERMISSION_DENIED: GIOErrorEnum = 14;
pub const G_IO_ERROR_INVALID_ARGUMENT: GIOErrorEnum = 13;
pub const G_IO_ERROR_NO_SPACE: GIOErrorEnum = 12;
pub const G_IO_ERROR_TOO_MANY_LINKS: GIOErrorEnum = 11;
pub const G_IO_ERROR_INVALID_FILENAME: GIOErrorEnum = 10;
pub const G_IO_ERROR_FILENAME_TOO_LONG: GIOErrorEnum = 9;
pub const G_IO_ERROR_NOT_MOUNTABLE_FILE: GIOErrorEnum = 8;
pub const G_IO_ERROR_NOT_SYMBOLIC_LINK: GIOErrorEnum = 7;
pub const G_IO_ERROR_NOT_REGULAR_FILE: GIOErrorEnum = 6;
pub const G_IO_ERROR_NOT_EMPTY: GIOErrorEnum = 5;
pub const G_IO_ERROR_NOT_DIRECTORY: GIOErrorEnum = 4;
pub const G_IO_ERROR_IS_DIRECTORY: GIOErrorEnum = 3;
pub const G_IO_ERROR_EXISTS: GIOErrorEnum = 2;
pub const G_IO_ERROR_NOT_FOUND: GIOErrorEnum = 1;
pub const G_IO_ERROR_FAILED: GIOErrorEnum = 0;
pub type GAsyncResult = _GAsyncResult;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCancellable {
    pub parent_instance: GObject,
    pub priv_0: *mut GCancellablePrivate,
}
pub type GCancellablePrivate = _GCancellablePrivate;
pub type GCancellable = _GCancellable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileEnumerator {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileEnumeratorPrivate,
}
pub type GFileEnumeratorPrivate = _GFileEnumeratorPrivate;
pub type GFileEnumerator = _GFileEnumerator;
pub type GFile = _GFile;
pub type GFileInfo = _GFileInfo;
pub type GFileAttributeMatcher = _GFileAttributeMatcher;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileEnumeratorClass {
    pub parent_class: GObjectClass,
    pub next_file: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub close_fn: Option<
        unsafe extern "C" fn(*mut GFileEnumerator, *mut GCancellable, *mut *mut GError) -> gboolean,
    >,
    pub next_files_async: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub next_files_finish: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GList,
    >,
    pub close_async: Option<
        unsafe extern "C" fn(
            *mut GFileEnumerator,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub close_finish: Option<
        unsafe extern "C" fn(*mut GFileEnumerator, *mut GAsyncResult, *mut *mut GError) -> gboolean,
    >,
    pub _g_reserved1: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved2: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved3: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved4: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved5: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved6: Option<unsafe extern "C" fn() -> ()>,
    pub _g_reserved7: Option<unsafe extern "C" fn() -> ()>,
}
pub type GFileEnumeratorClass = _GFileEnumeratorClass;
pub type GLocalFile = _GLocalFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileEnumerator {
    pub parent: GFileEnumerator,
    pub matcher: *mut GFileAttributeMatcher,
    pub reduced_matcher: *mut GFileAttributeMatcher,
    pub filename: *mut ::core::ffi::c_char,
    pub attributes: *mut ::core::ffi::c_char,
    pub flags: GFileQueryInfoFlags,
    pub got_parent_info: gboolean,
    pub parent_info: GLocalParentFileInfo,
    pub dir: *mut DIR,
    pub entries: *mut DirEntry,
    pub entries_pos: ::core::ffi::c_int,
    pub at_end: gboolean,
    pub follow_symlinks: gboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DirEntry {
    pub name: *mut ::core::ffi::c_char,
    pub inode: ::core::ffi::c_long,
    pub type_0: GFileType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLocalParentFileInfo {
    pub writable: gboolean,
    pub is_sticky: gboolean,
    pub has_trash_dir: gboolean,
    pub owner: ::core::ffi::c_int,
    pub device: dev_t,
    pub inode: ino_t,
    pub extra_data: gpointer,
    pub free_extra_data: GDestroyNotify,
}
pub type GLocalFileEnumerator = _GLocalFileEnumerator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileEnumeratorClass {
    pub parent_class: GFileEnumeratorClass,
}
pub type GLocalFileEnumeratorClass = _GLocalFileEnumeratorClass;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const DT_UNKNOWN_0: ::core::ffi::c_int = 0;
pub const DT_FIFO_0: ::core::ffi::c_int = 1;
pub const DT_CHR_0: ::core::ffi::c_int = 2;
pub const DT_DIR_0: ::core::ffi::c_int = 4;
pub const DT_BLK_0: ::core::ffi::c_int = 6;
pub const DT_REG_0: ::core::ffi::c_int = 8;
pub const DT_LNK_0: ::core::ffi::c_int = 10;
pub const DT_SOCK_0: ::core::ffi::c_int = 12;
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
pub const CHUNK_SIZE: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_local_file_enumerator_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_local_file_enumerator_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GLocalFileEnumerator_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(
            klass,
            &raw mut safe_c2rust_GLocalFileEnumerator_private_offset,
        );
    }
    safe_c2rust_g_local_file_enumerator_class_init(klass as *mut GLocalFileEnumeratorClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_local_file_enumerator_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        g_file_enumerator_get_type(),
        g_intern_static_string(b"GLocalFileEnumerator\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GLocalFileEnumeratorClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_enumerator_class_intern_init
                    as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GLocalFileEnumerator>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GLocalFileEnumerator) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_enumerator_init
                    as unsafe extern "C" fn(*mut GLocalFileEnumerator) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    return g_define_type_id;
}
static mut safe_c2rust_g_local_file_enumerator_parent_class: gpointer = NULL_0;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_enumerator_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_local_file_enumerator_get_type_once();
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
static mut safe_c2rust_GLocalFileEnumerator_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_free_entries(mut local: *mut GLocalFileEnumerator) {
    let mut i: ::core::ffi::c_int = 0;
    if !(*local).entries.is_null() {
        i = 0 as ::core::ffi::c_int;
        while !(*(*local).entries.offset(i as isize)).name.is_null() {
            g_free((*(*local).entries.offset(i as isize)).name as gpointer);
            i += 1;
        }
        g_free((*local).entries as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_g_local_file_enumerator_finalize(mut object: *mut GObject) {
    let mut local: *mut GLocalFileEnumerator = ::core::ptr::null_mut::<GLocalFileEnumerator>();
    local = object as *mut ::core::ffi::c_void as *mut GLocalFileEnumerator;
    if (*local).got_parent_info != 0 {
        _g_local_file_info_free_parent_info(&raw mut (*local).parent_info);
    }
    g_free((*local).filename as gpointer);
    g_file_attribute_matcher_unref((*local).matcher);
    g_file_attribute_matcher_unref((*local).reduced_matcher);
    if !(*local).dir.is_null() {
        closedir((*local).dir);
        (*local).dir = ::core::ptr::null_mut::<DIR>();
    }
    safe_c2rust_free_entries(local);
    (*(safe_c2rust_g_local_file_enumerator_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_local_file_enumerator_class_init(
    mut klass: *mut GLocalFileEnumeratorClass,
) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut enumerator_class: *mut GFileEnumeratorClass =
        klass as *mut ::core::ffi::c_void as *mut GFileEnumeratorClass;
    (*gobject_class).finalize = Some(
        safe_c2rust_g_local_file_enumerator_finalize as unsafe extern "C" fn(*mut GObject) -> (),
    ) as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    (*enumerator_class).next_file = Some(
        safe_c2rust_g_local_file_enumerator_next_file
            as unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
    (*enumerator_class).close_fn = Some(
        safe_c2rust_g_local_file_enumerator_close
            as unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFileEnumerator,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
}
unsafe extern "C" fn safe_c2rust_g_local_file_enumerator_init(
    mut local: *mut GLocalFileEnumerator,
) {
}
unsafe extern "C" fn safe_c2rust_g_file_attribute_matcher_subtract_attributes(
    mut matcher: *mut GFileAttributeMatcher,
    mut attributes: *const ::core::ffi::c_char,
) -> *mut GFileAttributeMatcher {
    let mut result: *mut GFileAttributeMatcher = ::core::ptr::null_mut::<GFileAttributeMatcher>();
    let mut tmp: *mut GFileAttributeMatcher = ::core::ptr::null_mut::<GFileAttributeMatcher>();
    tmp = g_file_attribute_matcher_new(attributes);
    result = g_file_attribute_matcher_subtract(matcher, tmp);
    g_file_attribute_matcher_unref(tmp);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_enumerator_new(
    mut file: *mut GLocalFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileEnumerator {
    let mut local: *mut GLocalFileEnumerator = ::core::ptr::null_mut::<GLocalFileEnumerator>();
    let mut filename: *mut ::core::ffi::c_char =
        g_file_get_path(file as *mut ::core::ffi::c_void as *mut GFile);
    let mut dir: *mut DIR = ::core::ptr::null_mut::<DIR>();
    let mut errsv: ::core::ffi::c_int = 0;
    dir = opendir(filename);
    if dir.is_null() {
        let mut utf8_filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
        errsv = *__errno_location();
        utf8_filename = g_filename_to_utf8(
            filename,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            b"Error opening directory '%s': %s\0" as *const u8 as *const gchar,
            utf8_filename,
            g_strerror(errsv as gint),
        );
        g_free(utf8_filename as gpointer);
        g_free(filename as gpointer);
        return ::core::ptr::null_mut::<GFileEnumerator>();
    }
    local = g_object_new(
        safe_c2rust__g_local_file_enumerator_get_type(),
        b"container\0" as *const u8 as *const gchar,
        file,
        NULL_0,
    ) as *mut GLocalFileEnumerator;
    (*local).dir = dir;
    (*local).filename = filename;
    (*local).matcher = g_file_attribute_matcher_new(attributes);
    (*local).reduced_matcher = safe_c2rust_g_file_attribute_matcher_subtract_attributes(
        (*local).matcher,
        b"standard::name,standard::display-name,standard::edit-name,standard::copy-name,standard::type\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    (*local).flags = flags;
    return local as *mut ::core::ffi::c_void as *mut GFileEnumerator;
}
unsafe extern "C" fn safe_c2rust_sort_by_inode(
    mut _a: *const ::core::ffi::c_void,
    mut _b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut a: *const DirEntry = ::core::ptr::null::<DirEntry>();
    let mut b: *const DirEntry = ::core::ptr::null::<DirEntry>();
    a = _a as *const DirEntry;
    b = _b as *const DirEntry;
    return ((*a).inode - (*b).inode) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_file_type_from_dirent(
    mut d_type: ::core::ffi::c_char,
) -> GFileType {
    match d_type as ::core::ffi::c_int {
        DT_BLK_0 | DT_CHR_0 | DT_FIFO_0 | DT_SOCK_0 => return G_FILE_TYPE_SPECIAL,
        DT_DIR_0 => return G_FILE_TYPE_DIRECTORY,
        DT_LNK_0 => return G_FILE_TYPE_SYMBOLIC_LINK,
        DT_REG_0 => return G_FILE_TYPE_REGULAR,
        DT_UNKNOWN_0 | _ => return G_FILE_TYPE_UNKNOWN,
    };
}
unsafe extern "C" fn safe_c2rust_next_file_helper(
    mut local: *mut GLocalFileEnumerator,
    mut file_type: *mut GFileType,
) -> *const ::core::ffi::c_char {
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut filename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if (*local).at_end != 0 {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if (*local).entries.is_null()
        || (*(*local).entries.offset((*local).entries_pos as isize))
            .name
            .is_null()
    {
        if (*local).entries.is_null() {
            (*local).entries = ({
                let mut __n: gsize =
                    (1000 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
                let mut __s: gsize = ::core::mem::size_of::<DirEntry>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc_n(__n, __s);
                }
                __p
            }) as *mut DirEntry;
        } else {
            i = 0 as ::core::ffi::c_int;
            while !(*(*local).entries.offset(i as isize)).name.is_null() {
                g_free((*(*local).entries.offset(i as isize)).name as gpointer);
                i += 1;
            }
        }
        i = 0 as ::core::ffi::c_int;
        while i < CHUNK_SIZE {
            entry = readdir((*local).dir);
            while !entry.is_null()
                && (0 as ::core::ffi::c_int
                    == strcmp(
                        &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                        b".\0" as *const u8 as *const ::core::ffi::c_char,
                    )
                    || 0 as ::core::ffi::c_int
                        == strcmp(
                            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
                            b"..\0" as *const u8 as *const ::core::ffi::c_char,
                        ))
            {
                entry = readdir((*local).dir);
            }
            if entry.is_null() {
                break;
            }
            let ref mut fresh0 = (*(*local).entries.offset(i as isize)).name;
            *fresh0 =
                safe_c2rust_g_strdup_inline(&raw mut (*entry).d_name as *mut ::core::ffi::c_char);
            (*(*local).entries.offset(i as isize)).inode = (*entry).d_ino as ::core::ffi::c_long;
            (*(*local).entries.offset(i as isize)).type_0 =
                safe_c2rust_file_type_from_dirent((*entry).d_type as ::core::ffi::c_char);
            i += 1;
        }
        let ref mut fresh1 = (*(*local).entries.offset(i as isize)).name;
        *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*local).entries_pos = 0 as ::core::ffi::c_int;
        qsort(
            (*local).entries as *mut ::core::ffi::c_void,
            i as size_t,
            ::core::mem::size_of::<DirEntry>() as size_t,
            Some(
                safe_c2rust_sort_by_inode
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
    filename = (*(*local).entries.offset((*local).entries_pos as isize)).name;
    if filename.is_null() {
        (*local).at_end = TRUE as gboolean;
    }
    *file_type = (*(*local).entries.offset((*local).entries_pos as isize)).type_0;
    (*local).entries_pos += 1;
    return filename;
}
unsafe extern "C" fn safe_c2rust_g_local_file_enumerator_next_file(
    mut enumerator: *mut GFileEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut local: *mut GLocalFileEnumerator =
        enumerator as *mut ::core::ffi::c_void as *mut GLocalFileEnumerator;
    let mut filename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut my_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut file_type: GFileType = G_FILE_TYPE_UNKNOWN;
    if (*local).got_parent_info == 0 {
        _g_local_file_info_get_parent_info(
            (*local).filename,
            (*local).matcher,
            &raw mut (*local).parent_info,
        );
        (*local).got_parent_info = TRUE as gboolean;
    }
    loop {
        if g_cancellable_set_error_if_cancelled(cancellable, error) != 0 {
            return ::core::ptr::null_mut::<GFileInfo>();
        }
        filename = safe_c2rust_next_file_helper(local, &raw mut file_type);
        if filename.is_null() {
            return ::core::ptr::null_mut::<GFileInfo>();
        }
        my_error = ::core::ptr::null_mut::<GError>();
        path = g_build_filename((*local).filename, filename, NULL_0) as *mut ::core::ffi::c_char;
        if file_type as ::core::ffi::c_uint
            == G_FILE_TYPE_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
            || file_type as ::core::ffi::c_uint
                == G_FILE_TYPE_SYMBOLIC_LINK as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*local).flags as ::core::ffi::c_uint
                    & G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    == 0
        {
            info = _g_local_file_info_get(
                filename,
                path,
                (*local).matcher,
                (*local).flags,
                &raw mut (*local).parent_info,
                &raw mut my_error,
            );
        } else {
            info = _g_local_file_info_get(
                filename,
                path,
                (*local).reduced_matcher,
                (*local).flags,
                &raw mut (*local).parent_info,
                &raw mut my_error,
            );
            if !info.is_null() {
                _g_local_file_info_get_nostat(info, filename, path, (*local).matcher);
                g_file_info_set_file_type(info, file_type);
                if file_type as ::core::ffi::c_uint
                    == G_FILE_TYPE_SYMBOLIC_LINK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    g_file_info_set_is_symlink(info, TRUE);
                }
            }
        }
        g_free(path as gpointer);
        if !info.is_null() {
            break;
        }
        if g_error_matches(
            my_error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
        ) != 0
        {
            g_error_free(my_error);
        } else {
            g_propagate_error(error, my_error);
            break;
        }
    }
    return info;
}
unsafe extern "C" fn safe_c2rust_g_local_file_enumerator_close(
    mut enumerator: *mut GFileEnumerator,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut local: *mut GLocalFileEnumerator =
        enumerator as *mut ::core::ffi::c_void as *mut GLocalFileEnumerator;
    if !(*local).dir.is_null() {
        closedir((*local).dir);
        (*local).dir = ::core::ptr::null_mut::<DIR>();
    }
    return TRUE;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
