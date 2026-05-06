use ::c2rust_bitfields;
extern "C" {
    pub type _GData;
    pub type __dirstream;
    pub type _GHashTable;
    pub type _GMarkupParseContext;
    pub type _GCancellable;
    pub type _GFileEnumeratorPrivate;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GIcon;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn g_ptr_array_new() -> *mut GPtrArray;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_free(array: *mut GPtrArray, free_seg: gboolean) -> *mut gpointer;
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_get_user_data_dir() -> *const gchar;
    fn g_get_system_data_dirs() -> *const *const gchar;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_get_language_names() -> *const *const gchar;
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
    fn g_file_test(filename: *const gchar, test: GFileTest) -> gboolean;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_insert_sorted(list: *mut GList, data: gpointer, func: GCompareFunc) -> *mut GList;
    fn g_list_last(list: *mut GList) -> *mut GList;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_replace(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_iter_steal(iter: *mut GHashTableIter);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_utf8_casefold(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_utf8_collate_key(str: *const gchar, len: gssize) -> *mut gchar;
    fn g_utf8_make_valid(str: *const gchar, len: gssize) -> *mut gchar;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_markup_parse_context_new(
        parser: *const GMarkupParser,
        flags: GMarkupParseFlags,
        user_data: gpointer,
        user_data_dnotify: GDestroyNotify,
    ) -> *mut GMarkupParseContext;
    fn g_markup_parse_context_free(context: *mut GMarkupParseContext);
    fn g_markup_parse_context_parse(
        context: *mut GMarkupParseContext,
        text: *const gchar,
        text_len: gssize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_themed_icon_new_from_names(
        iconnames: *mut *mut ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> *mut GIcon;
    fn g_file_get_child(file: *mut GFile, name: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_query_info(
        file: *mut GFile,
        attributes: *const ::core::ffi::c_char,
        flags: GFileQueryInfoFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_file_enumerate_children(
        file: *mut GFile,
        attributes: *const ::core::ffi::c_char,
        flags: GFileQueryInfoFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileEnumerator;
    fn g_file_enumerator_next_file(
        enumerator: *mut GFileEnumerator,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn g_file_info_get_attribute_boolean(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn g_file_info_get_file_type(info: *mut GFileInfo) -> GFileType;
    fn g_file_info_get_name(info: *mut GFileInfo) -> *const ::core::ffi::c_char;
    fn g_file_info_get_content_type(info: *mut GFileInfo) -> *const ::core::ffi::c_char;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn __lsan_enable();
    fn __lsan_disable();
    static safe_c2rust__gio_xdg_type_unknown: [::core::ffi::c_char; 0];
    fn _gio_xdg_get_mime_type_for_data(
        data: *const ::core::ffi::c_void,
        len: size_t,
        result_prio: *mut ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn _gio_xdg_get_mime_types_from_file_name(
        file_name: *const ::core::ffi::c_char,
        mime_types: *mut *const ::core::ffi::c_char,
        n_mime_types: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn _gio_xdg_mime_type_equal(
        mime_a: *const ::core::ffi::c_char,
        mime_b: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _gio_xdg_mime_type_subclass(
        mime_a: *const ::core::ffi::c_char,
        mime_b: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _gio_xdg_list_mime_parents(
        mime: *const ::core::ffi::c_char,
    ) -> *mut *mut ::core::ffi::c_char;
    fn _gio_xdg_unalias_mime_type(mime: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn _gio_xdg_get_icon(mime: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn _gio_xdg_get_generic_icon(mime: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn _gio_xdg_get_max_buffer_extents() -> ::core::ffi::c_int;
    fn _gio_xdg_register_reload_callback(
        callback: XdgMimeCallback,
        data: *mut ::core::ffi::c_void,
        destroy: XdgMimeDestroy,
    ) -> ::core::ffi::c_int;
    fn xdg_mime_set_dirs(dirs: *const *const ::core::ffi::c_char);
}
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
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
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
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
pub type DIR = __dirstream;
pub type GFileTest = ::core::ffi::c_uint;
pub const G_FILE_TEST_EXISTS: GFileTest = 16;
pub const G_FILE_TEST_IS_EXECUTABLE: GFileTest = 8;
pub const G_FILE_TEST_IS_DIR: GFileTest = 4;
pub const G_FILE_TEST_IS_SYMLINK: GFileTest = 2;
pub const G_FILE_TEST_IS_REGULAR: GFileTest = 1;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed = 512;
pub const G_ASCII_SPACE: C2RustUnnamed = 256;
pub const G_ASCII_PUNCT: C2RustUnnamed = 128;
pub const G_ASCII_PRINT: C2RustUnnamed = 64;
pub const G_ASCII_LOWER: C2RustUnnamed = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed = 8;
pub const G_ASCII_CNTRL: C2RustUnnamed = 4;
pub const G_ASCII_ALPHA: C2RustUnnamed = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed = 1;
pub type GMarkupParseFlags = ::core::ffi::c_uint;
pub const G_MARKUP_IGNORE_QUALIFIED: GMarkupParseFlags = 8;
pub const G_MARKUP_PREFIX_ERROR_POSITION: GMarkupParseFlags = 4;
pub const G_MARKUP_TREAT_CDATA_AS_TEXT: GMarkupParseFlags = 2;
pub const G_MARKUP_DO_NOT_USE_THIS_UNSUPPORTED_FLAG: GMarkupParseFlags = 1;
pub const G_MARKUP_DEFAULT_FLAGS: GMarkupParseFlags = 0;
pub type GMarkupParseContext = _GMarkupParseContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMarkupParser {
    pub start_element: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            *mut *const gchar,
            *mut *const gchar,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub end_element: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub text: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            gsize,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub passthrough: Option<
        unsafe extern "C" fn(
            *mut GMarkupParseContext,
            *const gchar,
            gsize,
            gpointer,
            *mut *mut GError,
        ) -> (),
    >,
    pub error: Option<unsafe extern "C" fn(*mut GMarkupParseContext, *mut GError, gpointer) -> ()>,
}
pub type GMarkupParser = _GMarkupParser;
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
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}
pub type GObject = _GObject;
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
pub type GIcon = _GIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MimeParser {
    pub current_type: ::core::ffi::c_int,
    pub current_lang_level: ::core::ffi::c_int,
    pub comment_lang_level: ::core::ffi::c_int,
    pub comment: *mut ::core::ffi::c_char,
}
pub const MIME_TAG_TYPE_COMMENT: C2RustUnnamed_0 = 1;
pub const MIME_TAG_TYPE_OTHER: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TreeMatch {
    pub contenttype: *mut gchar,
    pub priority: gint,
    pub matches: *mut GList,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct TreeMatchlet {
    pub path: *mut gchar,
    pub type_0: GFileType,
    #[bitfield(name = "match_case", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "executable", ty = "guint", bits = "1..=1")]
    #[bitfield(name = "non_empty", ty = "guint", bits = "2..=2")]
    #[bitfield(name = "on_disc", ty = "guint", bits = "3..=3")]
    pub match_case_executable_non_empty_on_disc: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub mimetype: *mut gchar,
    pub matches: *mut GList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Enumerator {
    pub path: *mut gchar,
    pub depth: gint,
    pub ignore_case: gboolean,
    pub components: *mut *mut gchar,
    pub case_components: *mut *mut gchar,
    pub enumerators: *mut *mut GFileEnumerator,
    pub children: *mut *mut GFile,
}
pub type XdgMimeDestroy = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XdgMimeCallback = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_atoi(
    mut __nptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_DIR_SEPARATOR_S: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0") };
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL_0 as gpointer;
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
pub const G_FILE_ATTRIBUTE_STANDARD_NAME: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"standard::name\0") };
pub const G_FILE_ATTRIBUTE_ACCESS_CAN_EXECUTE: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"access::can-execute\0")
};
#[inline]
unsafe extern "C" fn safe_c2rust_g_begin_ignore_leaks() {
    if Some(__lsan_disable as unsafe extern "C" fn() -> ()).is_some() {
        __lsan_disable();
    }
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_end_ignore_leaks() {
    if Some(__lsan_enable as unsafe extern "C" fn() -> ()).is_some() {
        __lsan_enable();
    }
}
static mut safe_c2rust_g__gio_xdgmime_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_content_type_get_sniff_len() -> gsize {
    let mut size: gsize = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    safe_c2rust_g_begin_ignore_leaks();
    size = _gio_xdg_get_max_buffer_extents() as gsize;
    safe_c2rust_g_end_ignore_leaks();
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_content_type_unalias(
    mut type_0: *const gchar,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut gchar = ::core::ptr::null_mut::<gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    safe_c2rust_g_begin_ignore_leaks();
    res = safe_c2rust_g_strdup_inline(_gio_xdg_unalias_mime_type(
        type_0 as *const ::core::ffi::c_char,
    )) as *mut gchar;
    safe_c2rust_g_end_ignore_leaks();
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    return res as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_unix_content_type_get_parents(
    mut type_0: *const gchar,
) -> *mut *mut ::core::ffi::c_char {
    let mut umime: *const gchar = ::core::ptr::null::<gchar>();
    let mut parents: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut array: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut i: ::core::ffi::c_int = 0;
    array = g_ptr_array_new();
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    safe_c2rust_g_begin_ignore_leaks();
    umime = _gio_xdg_unalias_mime_type(type_0 as *const ::core::ffi::c_char) as *const gchar;
    g_ptr_array_add(
        array,
        safe_c2rust_g_strdup_inline(umime as *const ::core::ffi::c_char) as gpointer,
    );
    parents = _gio_xdg_list_mime_parents(umime as *const ::core::ffi::c_char) as *mut *mut gchar;
    i = 0 as ::core::ffi::c_int;
    while !parents.is_null() && !(*parents.offset(i as isize)).is_null() {
        g_ptr_array_add(
            array,
            safe_c2rust_g_strdup_inline(*parents.offset(i as isize)) as gpointer,
        );
        i += 1;
    }
    free(parents as *mut ::core::ffi::c_void);
    safe_c2rust_g_end_ignore_leaks();
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    g_ptr_array_add(array, NULL_1);
    return g_ptr_array_free(array, FALSE) as *mut *mut ::core::ffi::c_char;
}
static mut safe_c2rust_g__global_mime_dirs_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_global_mime_dirs: *mut *mut gchar =
    ::core::ptr::null::<*mut gchar>() as *mut *mut gchar;
unsafe extern "C" fn safe_c2rust__g_content_type_set_mime_dirs_locked(
    mut dirs: *const *const ::core::ffi::c_char,
) {
    let mut _pp: *mut *mut *mut gchar = &raw mut safe_c2rust_global_mime_dirs;
    let mut _ptr: *mut *mut gchar = *_pp;
    *_pp = ::core::ptr::null_mut::<*mut gchar>();
    if !_ptr.is_null() {
        g_strfreev(_ptr as *mut *mut gchar);
    }
    if !dirs.is_null() {
        safe_c2rust_global_mime_dirs = g_strdupv(dirs as *mut *mut gchar);
    } else {
        let mut mime_dirs: *mut GPtrArray =
            g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
        let mut system_dirs: *const *const gchar = g_get_system_data_dirs();
        g_ptr_array_add(
            mime_dirs,
            g_build_filename(
                g_get_user_data_dir(),
                b"mime\0" as *const u8 as *const ::core::ffi::c_char,
                NULL_1,
            ) as gpointer,
        );
        while !(*system_dirs).is_null() {
            g_ptr_array_add(
                mime_dirs,
                g_build_filename(
                    *system_dirs,
                    b"mime\0" as *const u8 as *const ::core::ffi::c_char,
                    NULL_1,
                ) as gpointer,
            );
            system_dirs = system_dirs.offset(1);
        }
        g_ptr_array_add(mime_dirs, NULL_1);
        safe_c2rust_global_mime_dirs = g_ptr_array_free(mime_dirs, FALSE) as *mut *mut gchar;
    }
    xdg_mime_set_dirs(safe_c2rust_global_mime_dirs as *const *const ::core::ffi::c_char);
    safe_c2rust_tree_magic_schedule_reload();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_set_mime_dirs(mut dirs: *const *const gchar) {
    g_mutex_lock(&raw mut safe_c2rust_g__global_mime_dirs_lock);
    safe_c2rust__g_content_type_set_mime_dirs_locked(dirs as *const *const ::core::ffi::c_char);
    g_mutex_unlock(&raw mut safe_c2rust_g__global_mime_dirs_lock);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_get_mime_dirs() -> *const *const gchar {
    let mut mime_dirs: *const *const gchar = ::core::ptr::null::<*const gchar>();
    g_mutex_lock(&raw mut safe_c2rust_g__global_mime_dirs_lock);
    if safe_c2rust_global_mime_dirs.is_null() {
        safe_c2rust__g_content_type_set_mime_dirs_locked(::core::ptr::null::<
            *const ::core::ffi::c_char,
        >());
    }
    mime_dirs = safe_c2rust_global_mime_dirs as *const *const gchar;
    g_mutex_unlock(&raw mut safe_c2rust_g__global_mime_dirs_lock);
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !mime_dirs.is_null() {
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
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/gcontenttype.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            224 as ::core::ffi::c_int,
            G_STRFUNC,
            b"mime_dirs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return mime_dirs;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_equals(
    mut type1: *const gchar,
    mut type2: *const gchar,
) -> gboolean {
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !type1.is_null() {
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
            b"type1 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !type2.is_null() {
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
            b"type2 != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    safe_c2rust_g_begin_ignore_leaks();
    res = _gio_xdg_mime_type_equal(
        type1 as *const ::core::ffi::c_char,
        type2 as *const ::core::ffi::c_char,
    ) as gboolean;
    safe_c2rust_g_end_ignore_leaks();
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_is_a(
    mut type_0: *const gchar,
    mut supertype: *const gchar,
) -> gboolean {
    let mut res: gboolean = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !type_0.is_null() {
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
            b"type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !supertype.is_null() {
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
            b"supertype != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    safe_c2rust_g_begin_ignore_leaks();
    res = _gio_xdg_mime_type_subclass(
        type_0 as *const ::core::ffi::c_char,
        supertype as *const ::core::ffi::c_char,
    ) as gboolean;
    safe_c2rust_g_end_ignore_leaks();
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_is_mime_type(
    mut type_0: *const gchar,
    mut mime_type: *const gchar,
) -> gboolean {
    return safe_c2rust_g_content_type_is_a(type_0, mime_type);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_is_unknown(
    mut type_0: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !type_0.is_null() {
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
            b"type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (strcmp(
        &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char,
        type_0 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_language_level(
    mut lang: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut lang_list: *const *const ::core::ffi::c_char =
        ::core::ptr::null::<*const ::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    lang_list = g_get_language_names() as *const *const ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while !(*lang_list.offset(i as isize)).is_null() {
        if strcmp(*lang_list.offset(i as isize), lang) == 0 as ::core::ffi::c_int {
            return 1000 as ::core::ffi::c_int - i;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_mime_info_start_element(
    mut context: *mut GMarkupParseContext,
    mut element_name: *const gchar,
    mut attribute_names: *mut *const gchar,
    mut attribute_values: *mut *const gchar,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut lang: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut parser: *mut MimeParser = user_data as *mut MimeParser;
    if strcmp(
        element_name as *const ::core::ffi::c_char,
        b"comment\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        lang = b"C\0" as *const u8 as *const ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while !(*attribute_names.offset(i as isize)).is_null() {
            if strcmp(
                *attribute_names.offset(i as isize) as *const ::core::ffi::c_char,
                b"xml:lang\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                lang = *attribute_values.offset(i as isize) as *const ::core::ffi::c_char;
                break;
            } else {
                i += 1;
            }
        }
        (*parser).current_lang_level = safe_c2rust_language_level(lang);
        (*parser).current_type = MIME_TAG_TYPE_COMMENT as ::core::ffi::c_int;
    } else {
        (*parser).current_type = MIME_TAG_TYPE_OTHER as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn safe_c2rust_mime_info_end_element(
    mut context: *mut GMarkupParseContext,
    mut element_name: *const gchar,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut parser: *mut MimeParser = user_data as *mut MimeParser;
    (*parser).current_type = MIME_TAG_TYPE_OTHER as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_mime_info_text(
    mut context: *mut GMarkupParseContext,
    mut text: *const gchar,
    mut text_len: gsize,
    mut user_data: gpointer,
    mut error: *mut *mut GError,
) {
    let mut parser: *mut MimeParser = user_data as *mut MimeParser;
    if (*parser).current_type == MIME_TAG_TYPE_COMMENT as ::core::ffi::c_int
        && (*parser).current_lang_level > (*parser).comment_lang_level
    {
        g_free((*parser).comment as gpointer);
        (*parser).comment = g_strndup(text, text_len) as *mut ::core::ffi::c_char;
        (*parser).comment_lang_level = (*parser).current_lang_level;
    }
}
unsafe extern "C" fn safe_c2rust_load_comment_for_mime_helper(
    mut dir: *const ::core::ffi::c_char,
    mut basename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut context: *mut GMarkupParseContext = ::core::ptr::null_mut::<GMarkupParseContext>();
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: gsize = 0;
    let mut res: gboolean = 0;
    let mut parse_data: MimeParser = MimeParser {
        current_type: 0 as ::core::ffi::c_int,
        current_lang_level: 0,
        comment_lang_level: 0,
        comment: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut parser: GMarkupParser = _GMarkupParser {
        start_element: Some(
            safe_c2rust_mime_info_start_element
                as unsafe extern "C" fn(
                    *mut GMarkupParseContext,
                    *const gchar,
                    *mut *const gchar,
                    *mut *const gchar,
                    gpointer,
                    *mut *mut GError,
                ) -> (),
        ),
        end_element: Some(
            safe_c2rust_mime_info_end_element
                as unsafe extern "C" fn(
                    *mut GMarkupParseContext,
                    *const gchar,
                    gpointer,
                    *mut *mut GError,
                ) -> (),
        ),
        text: Some(
            safe_c2rust_mime_info_text
                as unsafe extern "C" fn(
                    *mut GMarkupParseContext,
                    *const gchar,
                    gsize,
                    gpointer,
                    *mut *mut GError,
                ) -> (),
        ),
        passthrough: None,
        error: None,
    };
    filename = g_build_filename(dir as *const gchar, basename, NULL_1) as *mut ::core::ffi::c_char;
    res = g_file_get_contents(
        filename,
        &raw mut data,
        &raw mut len,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_free(filename as gpointer);
    if res == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    context = g_markup_parse_context_new(
        &raw mut parser,
        G_MARKUP_DEFAULT_FLAGS,
        &raw mut parse_data as gpointer,
        None,
    );
    res = g_markup_parse_context_parse(
        context,
        data,
        len as gssize,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_free(data as gpointer);
    g_markup_parse_context_free(context);
    if res == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return parse_data.comment;
}
unsafe extern "C" fn safe_c2rust_load_comment_for_mime(
    mut mimetype: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut dirs: *const *const ::core::ffi::c_char =
        ::core::ptr::null::<*const ::core::ffi::c_char>();
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut comment: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: gsize = 0;
    basename = g_strdup_printf(b"%s.xml\0" as *const u8 as *const gchar, mimetype)
        as *mut ::core::ffi::c_char;
    dirs = safe_c2rust_g_content_type_get_mime_dirs() as *const *const ::core::ffi::c_char;
    i = 0 as gsize;
    while !(*dirs.offset(i as isize)).is_null() {
        comment = safe_c2rust_load_comment_for_mime_helper(*dirs.offset(i as isize), basename);
        if !comment.is_null() {
            g_free(basename as gpointer);
            return comment;
        }
        i = i.wrapping_add(1);
    }
    g_free(basename as gpointer);
    return g_strdup_printf(
        glib_gettext(b"%s type\0" as *const u8 as *const gchar),
        mimetype,
    ) as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_get_description(
    mut type_0: *const gchar,
) -> *mut gchar {
    static mut safe_c2rust_type_comment_cache: *mut GHashTable =
        ::core::ptr::null::<GHashTable>() as *mut GHashTable;
    let mut type_copy: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut comment: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !type_0.is_null() {
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
            b"type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    safe_c2rust_g_begin_ignore_leaks();
    type_0 = _gio_xdg_unalias_mime_type(type_0 as *const ::core::ffi::c_char) as *const gchar;
    safe_c2rust_g_end_ignore_leaks();
    if safe_c2rust_type_comment_cache.is_null() {
        safe_c2rust_type_comment_cache = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    comment =
        g_hash_table_lookup(safe_c2rust_type_comment_cache, type_0 as gconstpointer) as *mut gchar;
    comment = safe_c2rust_g_strdup_inline(comment) as *mut gchar;
    if !comment.is_null() {
        g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
        return safe_c2rust_g_steal_pointer(&raw mut comment as gpointer) as *mut gchar;
    }
    type_copy = safe_c2rust_g_strdup_inline(type_0 as *const ::core::ffi::c_char) as *mut gchar;
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    comment = safe_c2rust_load_comment_for_mime(type_copy) as *mut gchar;
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    g_hash_table_insert(
        safe_c2rust_type_comment_cache,
        safe_c2rust_g_steal_pointer(&raw mut type_copy as gpointer) as *mut gchar as gpointer,
        safe_c2rust_g_strdup_inline(comment) as gpointer,
    );
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    return safe_c2rust_g_steal_pointer(&raw mut comment as gpointer) as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_get_mime_type(
    mut type_0: *const ::core::ffi::c_char,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !type_0.is_null() {
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
            b"type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_strdup_inline(type_0) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_g_content_type_get_icon_internal(
    mut type_0: *const gchar,
    mut symbolic: gboolean,
) -> *mut GIcon {
    let mut mimetype_icon: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut generic_mimetype_icon: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut icon_names: [*mut ::core::ffi::c_char; 6] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 6];
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut themed_icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    let mut xdg_icon: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !type_0.is_null() {
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
            b"type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GIcon>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    safe_c2rust_g_begin_ignore_leaks();
    xdg_icon = _gio_xdg_get_icon(type_0 as *const ::core::ffi::c_char);
    safe_c2rust_g_end_ignore_leaks();
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    if !xdg_icon.is_null() {
        let fresh0 = n;
        n = n + 1;
        icon_names[fresh0 as usize] = safe_c2rust_g_strdup_inline(xdg_icon);
    }
    mimetype_icon = safe_c2rust_g_strdup_inline(type_0 as *const ::core::ffi::c_char);
    loop {
        q = strchr(mimetype_icon, '/' as i32);
        if q.is_null() {
            break;
        }
        *q = '-' as i32 as ::core::ffi::c_char;
    }
    let fresh1 = n;
    n = n + 1;
    icon_names[fresh1 as usize] = mimetype_icon;
    generic_mimetype_icon =
        safe_c2rust_g_content_type_get_generic_icon_name(type_0) as *mut ::core::ffi::c_char;
    if !generic_mimetype_icon.is_null() {
        let fresh2 = n;
        n = n + 1;
        icon_names[fresh2 as usize] = generic_mimetype_icon;
    }
    if symbolic != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < n {
            icon_names[(n + i) as usize] = icon_names[i as usize];
            icon_names[i as usize] = g_strconcat(
                icon_names[i as usize],
                b"-symbolic\0" as *const u8 as *const ::core::ffi::c_char,
                NULL_1,
            ) as *mut ::core::ffi::c_char;
            i += 1;
        }
        n += n;
    }
    themed_icon =
        g_themed_icon_new_from_names(&raw mut icon_names as *mut *mut ::core::ffi::c_char, n);
    i = 0 as ::core::ffi::c_int;
    while i < n {
        g_free(icon_names[i as usize] as gpointer);
        i += 1;
    }
    return themed_icon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_get_icon(
    mut type_0: *const gchar,
) -> *mut GIcon {
    return safe_c2rust_g_content_type_get_icon_internal(type_0, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_get_symbolic_icon(
    mut type_0: *const gchar,
) -> *mut GIcon {
    return safe_c2rust_g_content_type_get_icon_internal(type_0, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_get_generic_icon_name(
    mut type_0: *const gchar,
) -> *mut gchar {
    let mut xdg_icon_name: *const gchar = ::core::ptr::null::<gchar>();
    let mut icon_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !type_0.is_null() {
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
            b"type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    safe_c2rust_g_begin_ignore_leaks();
    xdg_icon_name = _gio_xdg_get_generic_icon(type_0 as *const ::core::ffi::c_char) as *const gchar;
    safe_c2rust_g_end_ignore_leaks();
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    if xdg_icon_name.is_null() {
        let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut suffix: *const ::core::ffi::c_char =
            b"-x-generic\0" as *const u8 as *const ::core::ffi::c_char;
        p = strchr(type_0 as *const ::core::ffi::c_char, '/' as i32);
        if p.is_null() {
            p = type_0.offset(strlen(type_0 as *const ::core::ffi::c_char) as isize)
                as *const ::core::ffi::c_char;
        }
        icon_name = g_malloc(
            (p.offset_from(type_0) as ::core::ffi::c_long as gsize)
                .wrapping_add(strlen(suffix) as gsize)
                .wrapping_add(1 as gsize),
        ) as *mut gchar;
        memcpy(
            icon_name as *mut ::core::ffi::c_void,
            type_0 as *const ::core::ffi::c_void,
            p.offset_from(type_0) as ::core::ffi::c_long as size_t,
        );
        memcpy(
            icon_name.offset(p.offset_from(type_0) as ::core::ffi::c_long as isize)
                as *mut ::core::ffi::c_void,
            suffix as *const ::core::ffi::c_void,
            strlen(suffix),
        );
        *icon_name.offset(
            (p.offset_from(type_0) as ::core::ffi::c_long as size_t).wrapping_add(strlen(suffix))
                as isize,
        ) = 0 as gchar;
    } else {
        icon_name =
            safe_c2rust_g_strdup_inline(xdg_icon_name as *const ::core::ffi::c_char) as *mut gchar;
    }
    return icon_name;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_can_be_executable(
    mut type_0: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !type_0.is_null() {
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
            b"type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if safe_c2rust_g_content_type_is_a(
        type_0,
        b"application/x-executable\0" as *const u8 as *const gchar,
    ) != 0
        || safe_c2rust_g_content_type_is_a(type_0, b"text/plain\0" as *const u8 as *const gchar)
            != 0
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_looks_like_text(
    mut data: *const guchar,
    mut data_size: gsize,
) -> gboolean {
    let mut i: gsize = 0;
    let mut c: ::core::ffi::c_char = 0;
    i = 0 as gsize;
    while i < data_size {
        c = *data.offset(i as isize) as ::core::ffi::c_char;
        if *safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_CNTRL as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
            && !(*safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
                & G_ASCII_SPACE as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
            && c as ::core::ffi::c_int != '\u{8}' as i32
        {
            return FALSE;
        }
        i = i.wrapping_add(1);
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_from_mime_type(
    mut mime_type: *const gchar,
) -> *mut gchar {
    let mut umime: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !mime_type.is_null() {
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
            b"mime_type != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    safe_c2rust_g_begin_ignore_leaks();
    umime = safe_c2rust_g_strdup_inline(_gio_xdg_unalias_mime_type(
        mime_type as *const ::core::ffi::c_char,
    ));
    safe_c2rust_g_end_ignore_leaks();
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    return umime as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_guess(
    mut filename: *const gchar,
    mut data: *const guchar,
    mut data_size: gsize,
    mut result_uncertain: *mut gboolean,
) -> *mut gchar {
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut name_mimetypes: [*const ::core::ffi::c_char; 10] =
        [::core::ptr::null::<::core::ffi::c_char>(); 10];
    let mut sniffed_mimetype: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut mimetype: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut n_name_mimetypes: ::core::ffi::c_int = 0;
    let mut sniffed_prio: ::core::ffi::c_int = 0;
    sniffed_prio = 0 as ::core::ffi::c_int;
    n_name_mimetypes = 0 as ::core::ffi::c_int;
    sniffed_mimetype = &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char;
    if !result_uncertain.is_null() {
        *result_uncertain = FALSE as gboolean;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if data_size != -(1 as ::core::ffi::c_int) as gsize {
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
            b"data_size != (gsize) -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return safe_c2rust_g_strdup_inline(
            &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char,
        ) as *mut gchar;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    safe_c2rust_g_begin_ignore_leaks();
    if !filename.is_null() {
        i = strlen(filename as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        if i > 0 as ::core::ffi::c_int
            && *filename.offset((i - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == '/' as i32
        {
            name_mimetypes[0 as ::core::ffi::c_int as usize] =
                b"inode/directory\0" as *const u8 as *const ::core::ffi::c_char;
            name_mimetypes[1 as ::core::ffi::c_int as usize] =
                ::core::ptr::null::<::core::ffi::c_char>();
            n_name_mimetypes = 1 as ::core::ffi::c_int;
            if !result_uncertain.is_null() {
                *result_uncertain = TRUE as gboolean;
            }
        } else {
            basename = g_path_get_basename(filename) as *mut ::core::ffi::c_char;
            n_name_mimetypes = _gio_xdg_get_mime_types_from_file_name(
                basename,
                &raw mut name_mimetypes as *mut *const ::core::ffi::c_char,
                10 as ::core::ffi::c_int,
            );
            g_free(basename as gpointer);
        }
    }
    if n_name_mimetypes == 1 as ::core::ffi::c_int {
        let mut s: *mut gchar =
            safe_c2rust_g_strdup_inline(name_mimetypes[0 as ::core::ffi::c_int as usize])
                as *mut gchar;
        safe_c2rust_g_end_ignore_leaks();
        g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
        return s;
    }
    if !data.is_null() {
        sniffed_mimetype = _gio_xdg_get_mime_type_for_data(
            data as *const ::core::ffi::c_void,
            data_size as size_t,
            &raw mut sniffed_prio,
        );
        if sniffed_mimetype
            == &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char
            && !data.is_null()
            && safe_c2rust_looks_like_text(data, data_size) != 0
        {
            sniffed_mimetype = b"text/plain\0" as *const u8 as *const ::core::ffi::c_char;
        }
        if !filename.is_null()
            && strcmp(
                sniffed_mimetype,
                b"application/x-desktop\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            sniffed_mimetype = b"text/plain\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    if n_name_mimetypes == 0 as ::core::ffi::c_int {
        if sniffed_mimetype
            == &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char
            && !result_uncertain.is_null()
        {
            *result_uncertain = TRUE as gboolean;
        }
        mimetype = safe_c2rust_g_strdup_inline(sniffed_mimetype);
    } else {
        mimetype = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if sniffed_mimetype
            != &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char
        {
            if sniffed_prio >= 80 as ::core::ffi::c_int {
                mimetype = safe_c2rust_g_strdup_inline(sniffed_mimetype);
            } else {
                i = 0 as ::core::ffi::c_int;
                while i < n_name_mimetypes {
                    if _gio_xdg_mime_type_subclass(name_mimetypes[i as usize], sniffed_mimetype)
                        != 0
                    {
                        mimetype = safe_c2rust_g_strdup_inline(name_mimetypes[i as usize]);
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        if mimetype.is_null() {
            mimetype =
                safe_c2rust_g_strdup_inline(name_mimetypes[0 as ::core::ffi::c_int as usize]);
            if !result_uncertain.is_null() {
                *result_uncertain = TRUE as gboolean;
            }
        }
    }
    safe_c2rust_g_end_ignore_leaks();
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_xdgmime_lock);
    return mimetype as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_enumerate_mimetypes_subdir(
    mut dir: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
    mut mimetypes: *mut GHashTable,
) {
    let mut d: *mut DIR = ::core::ptr::null_mut::<DIR>();
    let mut ent: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut mimetype: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    d = opendir(dir);
    if !d.is_null() {
        loop {
            ent = readdir(d);
            if ent.is_null() {
                break;
            }
            if if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char =
                        &raw mut (*ent).d_name as *mut ::core::ffi::c_char;
                    let __suffix: *const ::core::ffi::c_char =
                        b".xml\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                        if __str.is_null() || __suffix.is_null() {
                            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_23
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
                g_str_has_suffix(
                    &raw mut (*ent).d_name as *mut ::core::ffi::c_char,
                    b".xml\0" as *const u8 as *const gchar,
                )
            } != 0
            {
                mimetype = g_strdup_printf(
                    b"%s/%.*s\0" as *const u8 as *const gchar,
                    prefix,
                    strlen(&raw mut (*ent).d_name as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_int
                        - 4 as ::core::ffi::c_int,
                    &raw mut (*ent).d_name as *mut ::core::ffi::c_char,
                ) as *mut ::core::ffi::c_char;
                g_hash_table_replace(mimetypes, mimetype as gpointer, NULL_1);
            }
        }
        closedir(d);
    }
}
unsafe extern "C" fn safe_c2rust_enumerate_mimetypes_dir(
    mut dir: *const ::core::ffi::c_char,
    mut mimetypes: *mut GHashTable,
) {
    let mut d: *mut DIR = ::core::ptr::null_mut::<DIR>();
    let mut ent: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut mimedir: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    mimedir = dir;
    d = opendir(mimedir);
    if !d.is_null() {
        loop {
            ent = readdir(d);
            if ent.is_null() {
                break;
            }
            if strcmp(
                &raw mut (*ent).d_name as *mut ::core::ffi::c_char,
                b"packages\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                name = g_build_filename(
                    mimedir as *const gchar,
                    &raw mut (*ent).d_name as *mut ::core::ffi::c_char,
                    NULL_1,
                ) as *mut ::core::ffi::c_char;
                if g_file_test(name, G_FILE_TEST_IS_DIR) != 0 {
                    safe_c2rust_enumerate_mimetypes_subdir(
                        name,
                        &raw mut (*ent).d_name as *mut ::core::ffi::c_char,
                        mimetypes,
                    );
                }
                g_free(name as gpointer);
            }
        }
        closedir(d);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_types_get_registered() -> *mut GList {
    let mut dirs: *const *const ::core::ffi::c_char =
        ::core::ptr::null::<*const ::core::ffi::c_char>();
    let mut mimetypes: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut key: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut i: gsize = 0;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    mimetypes = g_hash_table_new_full(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        None,
    );
    dirs = safe_c2rust_g_content_type_get_mime_dirs() as *const *const ::core::ffi::c_char;
    i = 0 as gsize;
    while !(*dirs.offset(i as isize)).is_null() {
        safe_c2rust_enumerate_mimetypes_dir(*dirs.offset(i as isize), mimetypes);
        i = i.wrapping_add(1);
    }
    l = ::core::ptr::null_mut::<GList>();
    g_hash_table_iter_init(&raw mut iter, mimetypes);
    while g_hash_table_iter_next(
        &raw mut iter,
        &raw mut key,
        ::core::ptr::null_mut::<gpointer>(),
    ) != 0
    {
        l = g_list_prepend(l, key);
        g_hash_table_iter_steal(&raw mut iter);
    }
    g_hash_table_destroy(mimetypes);
    return l;
}
static mut safe_c2rust_tree_matches: *mut GList = ::core::ptr::null::<GList>() as *mut GList;
static mut safe_c2rust_need_reload: gboolean = FALSE;
static mut safe_c2rust_g__gio_treemagic_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
unsafe extern "C" fn safe_c2rust_tree_matchlet_free(mut matchlet: *mut TreeMatchlet) {
    g_list_free_full(
        (*matchlet).matches,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut TreeMatchlet) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_tree_matchlet_free as unsafe extern "C" fn(*mut TreeMatchlet) -> (),
        )),
    );
    g_free((*matchlet).path as gpointer);
    g_free((*matchlet).mimetype as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<TreeMatchlet>() as gsize,
        matchlet as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_tree_match_free(mut match_0: *mut TreeMatch) {
    g_list_free_full(
        (*match_0).matches,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut TreeMatchlet) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_tree_matchlet_free as unsafe extern "C" fn(*mut TreeMatchlet) -> (),
        )),
    );
    g_free((*match_0).contenttype as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<TreeMatch>() as gsize,
        match_0 as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_parse_header(mut line: *mut gchar) -> *mut TreeMatch {
    let mut len: size_t = 0;
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut match_0: *mut TreeMatch = ::core::ptr::null_mut::<TreeMatch>();
    len = strlen(line);
    if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '[' as i32
        || *line.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int != ']' as i32
    {
        return ::core::ptr::null_mut::<TreeMatch>();
    }
    *line.offset(len.wrapping_sub(1 as size_t) as isize) = 0 as gchar;
    s = strchr(line, ':' as i32) as *mut gchar;
    if s.is_null() {
        return ::core::ptr::null_mut::<TreeMatch>();
    }
    match_0 = ({
        let mut __s: gsize = ::core::mem::size_of::<TreeMatch>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut TreeMatch;
    (*match_0).priority = safe_c2rust_atoi(line.offset(1 as ::core::ffi::c_int as isize)) as gint;
    (*match_0).contenttype =
        safe_c2rust_g_strdup_inline(s.offset(1 as ::core::ffi::c_int as isize)) as *mut gchar;
    return match_0;
}
unsafe extern "C" fn safe_c2rust_parse_match_line(
    mut line: *mut gchar,
    mut depth: *mut gint,
) -> *mut TreeMatchlet {
    let mut current_block: u64;
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut matchlet: *mut TreeMatchlet = ::core::ptr::null_mut::<TreeMatchlet>();
    let mut parts: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gint = 0;
    matchlet = ({
        let mut __s: gsize = ::core::mem::size_of::<TreeMatchlet>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut TreeMatchlet;
    if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32 {
        *depth = 0 as ::core::ffi::c_int as gint;
        s = line;
        current_block = 6937071982253665452;
    } else {
        *depth = safe_c2rust_atoi(line) as gint;
        s = strchr(line, '>' as i32) as *mut gchar;
        if s.is_null() {
            current_block = 10245483827264291712;
        } else {
            current_block = 6937071982253665452;
        }
    }
    match current_block {
        6937071982253665452 => {
            s = s.offset(2 as ::core::ffi::c_int as isize);
            p = strchr(s, '"' as i32) as *mut gchar;
            if !p.is_null() {
                *p = 0 as gchar;
                (*matchlet).path = safe_c2rust_g_strdup_inline(s) as *mut gchar;
                s = p.offset(1 as ::core::ffi::c_int as isize);
                parts = g_strsplit(s, b",\0" as *const u8 as *const gchar, 0 as gint);
                if strcmp(
                    *parts.offset(0 as ::core::ffi::c_int as isize),
                    b"=file\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    (*matchlet).type_0 = G_FILE_TYPE_REGULAR;
                } else if strcmp(
                    *parts.offset(0 as ::core::ffi::c_int as isize),
                    b"=directory\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    (*matchlet).type_0 = G_FILE_TYPE_DIRECTORY;
                } else if strcmp(
                    *parts.offset(0 as ::core::ffi::c_int as isize),
                    b"=link\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    (*matchlet).type_0 = G_FILE_TYPE_SYMBOLIC_LINK;
                } else {
                    (*matchlet).type_0 = G_FILE_TYPE_UNKNOWN;
                }
                i = 1 as ::core::ffi::c_int as gint;
                while !(*parts.offset(i as isize)).is_null() {
                    if strcmp(
                        *parts.offset(i as isize),
                        b"executable\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*matchlet).set_executable(1 as guint as guint);
                    } else if strcmp(
                        *parts.offset(i as isize),
                        b"match-case\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*matchlet).set_match_case(1 as guint as guint);
                    } else if strcmp(
                        *parts.offset(i as isize),
                        b"non-empty\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*matchlet).set_non_empty(1 as guint as guint);
                    } else if strcmp(
                        *parts.offset(i as isize),
                        b"on-disc\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*matchlet).set_on_disc(1 as guint as guint);
                    } else {
                        (*matchlet).mimetype =
                            safe_c2rust_g_strdup_inline(*parts.offset(i as isize)) as *mut gchar;
                    }
                    i += 1;
                }
                g_strfreev(parts);
                return matchlet;
            }
        }
        _ => {}
    }
    g_slice_free1(
        ::core::mem::size_of::<TreeMatchlet>() as gsize,
        matchlet as gpointer,
    );
    return ::core::ptr::null_mut::<TreeMatchlet>();
}
unsafe extern "C" fn safe_c2rust_cmp_match(mut a: gconstpointer, mut b: gconstpointer) -> gint {
    let mut aa: *const TreeMatch = a as *const TreeMatch;
    let mut bb: *const TreeMatch = b as *const TreeMatch;
    return (*bb).priority - (*aa).priority;
}
unsafe extern "C" fn safe_c2rust_insert_match(mut match_0: *mut TreeMatch) {
    safe_c2rust_tree_matches = g_list_insert_sorted(
        safe_c2rust_tree_matches,
        match_0 as gpointer,
        Some(safe_c2rust_cmp_match as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint),
    );
}
unsafe extern "C" fn safe_c2rust_insert_matchlet(
    mut match_0: *mut TreeMatch,
    mut matchlet: *mut TreeMatchlet,
    mut depth: gint,
) {
    if depth == 0 as ::core::ffi::c_int {
        (*match_0).matches = g_list_append((*match_0).matches, matchlet as gpointer);
    } else {
        let mut last: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut m: *mut TreeMatchlet = ::core::ptr::null_mut::<TreeMatchlet>();
        last = g_list_last((*match_0).matches);
        if last.is_null() {
            safe_c2rust_tree_matchlet_free(matchlet);
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"can't insert tree matchlet at depth %d\0" as *const u8 as *const gchar,
                depth,
            );
            return;
        }
        m = (*last).data as *mut TreeMatchlet;
        loop {
            depth -= 1;
            if !(depth > 0 as ::core::ffi::c_int) {
                break;
            }
            last = g_list_last((*m).matches);
            if last.is_null() {
                safe_c2rust_tree_matchlet_free(matchlet);
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"can't insert tree matchlet at depth %d\0" as *const u8 as *const gchar,
                    depth,
                );
                return;
            }
            m = (*last).data as *mut TreeMatchlet;
        }
        (*m).matches = g_list_append((*m).matches, matchlet as gpointer);
    };
}
unsafe extern "C" fn safe_c2rust_read_tree_magic_from_directory(mut prefix: *const gchar) {
    let mut filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut text: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut len: gsize = 0;
    let mut lines: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gsize = 0;
    let mut match_0: *mut TreeMatch = ::core::ptr::null_mut::<TreeMatch>();
    let mut matchlet: *mut TreeMatchlet = ::core::ptr::null_mut::<TreeMatchlet>();
    let mut depth: gint = 0;
    filename = g_build_filename(
        prefix,
        b"treemagic\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_1,
    );
    if g_file_get_contents(
        filename,
        &raw mut text,
        &raw mut len,
        ::core::ptr::null_mut::<*mut GError>(),
    ) != 0
    {
        if strcmp(
            text,
            b"MIME-TreeMagic\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            lines = g_strsplit(
                text.offset(
                    strlen(b"MIME-TreeMagic\0" as *const u8 as *const ::core::ffi::c_char) as isize,
                )
                .offset(2 as ::core::ffi::c_int as isize),
                b"\n\0" as *const u8 as *const gchar,
                0 as gint,
            );
            match_0 = ::core::ptr::null_mut::<TreeMatch>();
            i = 0 as gsize;
            while !(*lines.offset(i as isize)).is_null()
                && *(*lines.offset(i as isize)).offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    != 0
            {
                if *(*lines.offset(i as isize)).offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '[' as i32
                    && {
                        match_0 = safe_c2rust_parse_header(*lines.offset(i as isize));
                        !match_0.is_null()
                    }
                {
                    safe_c2rust_insert_match(match_0);
                } else if !match_0.is_null() {
                    matchlet =
                        safe_c2rust_parse_match_line(*lines.offset(i as isize), &raw mut depth);
                    if matchlet.is_null() {
                        g_log(
                            G_LOG_DOMAIN.as_ptr() as *const gchar,
                            G_LOG_LEVEL_WARNING,
                            b"%s: body corrupt; skipping\0" as *const u8 as *const gchar,
                            filename,
                        );
                        break;
                    } else {
                        safe_c2rust_insert_matchlet(match_0, matchlet, depth);
                    }
                } else {
                    g_log(
                        G_LOG_DOMAIN.as_ptr() as *const gchar,
                        G_LOG_LEVEL_WARNING,
                        b"%s: header corrupt; skipping\0" as *const u8 as *const gchar,
                        filename,
                    );
                    break;
                }
                i = i.wrapping_add(1);
            }
            g_strfreev(lines);
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"%s: header not found, skipping\0" as *const u8 as *const gchar,
                filename,
            );
        }
        g_free(text as gpointer);
    }
    g_free(filename as gpointer);
}
unsafe extern "C" fn safe_c2rust_tree_magic_schedule_reload() {
    safe_c2rust_need_reload = TRUE as gboolean;
}
unsafe extern "C" fn safe_c2rust_xdg_mime_reload(mut user_data: *mut ::core::ffi::c_void) {
    safe_c2rust_tree_magic_schedule_reload();
}
unsafe extern "C" fn safe_c2rust_tree_magic_shutdown() {
    g_list_free_full(
        safe_c2rust_tree_matches,
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut TreeMatch) -> ()>, GDestroyNotify>(
            Some(safe_c2rust_tree_match_free as unsafe extern "C" fn(*mut TreeMatch) -> ()),
        ),
    );
    safe_c2rust_tree_matches = ::core::ptr::null_mut::<GList>();
}
unsafe extern "C" fn safe_c2rust_tree_magic_init() {
    static mut safe_c2rust_initialized: gboolean = FALSE;
    let mut i: gsize = 0;
    if safe_c2rust_initialized == 0 {
        safe_c2rust_initialized = TRUE as gboolean;
        _gio_xdg_register_reload_callback(
            Some(
                safe_c2rust_xdg_mime_reload as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
            NULL_1,
            None,
        );
        safe_c2rust_need_reload = TRUE as gboolean;
    }
    if safe_c2rust_need_reload != 0 {
        let mut dirs: *const *const ::core::ffi::c_char =
            ::core::ptr::null::<*const ::core::ffi::c_char>();
        safe_c2rust_need_reload = FALSE as gboolean;
        safe_c2rust_tree_magic_shutdown();
        dirs = safe_c2rust_g_content_type_get_mime_dirs() as *const *const ::core::ffi::c_char;
        i = 0 as gsize;
        while !(*dirs.offset(i as isize)).is_null() {
            safe_c2rust_read_tree_magic_from_directory(*dirs.offset(i as isize) as *const gchar);
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn safe_c2rust_component_match(
    mut e: *mut Enumerator,
    mut depth: gint,
    mut name: *const gchar,
) -> gboolean {
    let mut case_folded: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut utf8_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut found: gboolean = 0;
    if strcmp(
        name as *const ::core::ffi::c_char,
        *(*e).components.offset(depth as isize),
    ) == 0 as ::core::ffi::c_int
    {
        return TRUE;
    }
    if (*e).ignore_case == 0 {
        return FALSE;
    }
    utf8_name = g_filename_to_utf8(
        name,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if utf8_name.is_null() {
        utf8_name = g_utf8_make_valid(name, -(1 as ::core::ffi::c_int) as gssize);
    }
    case_folded = g_utf8_casefold(utf8_name, -(1 as ::core::ffi::c_int) as gssize);
    key = g_utf8_collate_key(case_folded, -(1 as ::core::ffi::c_int) as gssize);
    found = (strcmp(key, *(*e).case_components.offset(depth as isize)) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as gboolean;
    g_free(utf8_name as gpointer);
    g_free(case_folded as gpointer);
    g_free(key as gpointer);
    return found;
}
unsafe extern "C" fn safe_c2rust_next_match_recurse(
    mut e: *mut Enumerator,
    mut depth: gint,
) -> *mut GFile {
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    while FALSE == 0 {
        if (*(*e).enumerators.offset(depth as isize)).is_null() {
            if depth > 0 as ::core::ffi::c_int {
                file = safe_c2rust_next_match_recurse(e, depth - 1 as gint);
                if !file.is_null() {
                    let ref mut fresh3 = *(*e).children.offset(depth as isize);
                    *fresh3 = file;
                    let ref mut fresh4 = *(*e).enumerators.offset(depth as isize);
                    *fresh4 = g_file_enumerate_children(
                        file,
                        G_FILE_ATTRIBUTE_STANDARD_NAME.as_ptr(),
                        G_FILE_QUERY_INFO_NONE,
                        ::core::ptr::null_mut::<GCancellable>(),
                        ::core::ptr::null_mut::<*mut GError>(),
                    );
                }
            }
            if (*(*e).enumerators.offset(depth as isize)).is_null() {
                return ::core::ptr::null_mut::<GFile>();
            }
        }
        loop {
            info = g_file_enumerator_next_file(
                *(*e).enumerators.offset(depth as isize),
                ::core::ptr::null_mut::<GCancellable>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            if info.is_null() {
                break;
            }
            name = g_file_info_get_name(info) as *const gchar;
            if safe_c2rust_component_match(e, depth, name) != 0 {
                file = g_file_get_child(
                    *(*e).children.offset(depth as isize),
                    name as *const ::core::ffi::c_char,
                );
                g_object_unref(info as gpointer);
                return file;
            }
            g_object_unref(info as gpointer);
        }
        g_object_unref(*(*e).enumerators.offset(depth as isize) as gpointer);
        let ref mut fresh5 = *(*e).enumerators.offset(depth as isize);
        *fresh5 = ::core::ptr::null_mut::<GFileEnumerator>();
        g_object_unref(*(*e).children.offset(depth as isize) as gpointer);
        let ref mut fresh6 = *(*e).children.offset(depth as isize);
        *fresh6 = ::core::ptr::null_mut::<GFile>();
    }
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn safe_c2rust_enumerator_next(mut e: *mut Enumerator) -> *mut GFile {
    return safe_c2rust_next_match_recurse(e, (*e).depth - 1 as gint);
}
unsafe extern "C" fn safe_c2rust_enumerator_new(
    mut root: *mut GFile,
    mut path: *const ::core::ffi::c_char,
    mut ignore_case: gboolean,
) -> *mut Enumerator {
    let mut e: *mut Enumerator = ::core::ptr::null_mut::<Enumerator>();
    let mut i: gint = 0;
    let mut case_folded: *mut gchar = ::core::ptr::null_mut::<gchar>();
    e = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<Enumerator>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut Enumerator;
    (*e).path = safe_c2rust_g_strdup_inline(path) as *mut gchar;
    (*e).ignore_case = ignore_case;
    (*e).components = g_strsplit(
        (*e).path,
        G_DIR_SEPARATOR_S.as_ptr() as *const gchar,
        -(1 as gint),
    );
    (*e).depth = g_strv_length((*e).components) as gint;
    if (*e).ignore_case != 0 {
        (*e).case_components = ({
            let mut __n: gsize =
                ((*e).depth as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
            let mut __s: gsize = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut *mut ::core::ffi::c_char as *mut *mut gchar;
        i = 0 as ::core::ffi::c_int as gint;
        while !(*(*e).components.offset(i as isize)).is_null() {
            case_folded = g_utf8_casefold(
                *(*e).components.offset(i as isize),
                -(1 as ::core::ffi::c_int) as gssize,
            );
            let ref mut fresh7 = *(*e).case_components.offset(i as isize);
            *fresh7 = g_utf8_collate_key(case_folded, -(1 as ::core::ffi::c_int) as gssize);
            g_free(case_folded as gpointer);
            i += 1;
        }
    }
    (*e).children = ({
        let mut __n: gsize = (*e).depth as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut GFile>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut *mut GFile;
    let ref mut fresh8 = *(*e).children.offset(0 as ::core::ffi::c_int as isize);
    *fresh8 = g_object_ref(root as gpointer) as *mut GFile as *mut GFile;
    (*e).enumerators = ({
        let mut __n: gsize = (*e).depth as gsize;
        let mut __s: gsize = ::core::mem::size_of::<*mut GFileEnumerator>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut *mut GFileEnumerator;
    let ref mut fresh9 = *(*e).enumerators.offset(0 as ::core::ffi::c_int as isize);
    *fresh9 = g_file_enumerate_children(
        root,
        G_FILE_ATTRIBUTE_STANDARD_NAME.as_ptr(),
        G_FILE_QUERY_INFO_NONE,
        ::core::ptr::null_mut::<GCancellable>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    return e;
}
unsafe extern "C" fn safe_c2rust_enumerator_free(mut e: *mut Enumerator) {
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while i < (*e).depth {
        if !(*(*e).enumerators.offset(i as isize)).is_null() {
            g_object_unref(*(*e).enumerators.offset(i as isize) as gpointer);
        }
        if !(*(*e).children.offset(i as isize)).is_null() {
            g_object_unref(*(*e).children.offset(i as isize) as gpointer);
        }
        i += 1;
    }
    g_free((*e).enumerators as gpointer);
    g_free((*e).children as gpointer);
    g_strfreev((*e).components);
    if !(*e).case_components.is_null() {
        g_strfreev((*e).case_components);
    }
    g_free((*e).path as gpointer);
    g_free(e as gpointer);
}
unsafe extern "C" fn safe_c2rust_matchlet_match(
    mut matchlet: *mut TreeMatchlet,
    mut root: *mut GFile,
) -> gboolean {
    let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut result: gboolean = 0;
    let mut attrs: *const gchar = ::core::ptr::null::<gchar>();
    let mut e: *mut Enumerator = ::core::ptr::null_mut::<Enumerator>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    e = safe_c2rust_enumerator_new(
        root,
        (*matchlet).path,
        ((*matchlet).match_case() == 0) as ::core::ffi::c_int,
    );
    loop {
        file = safe_c2rust_enumerator_next(e);
        if file.is_null() {
            safe_c2rust_enumerator_free(e);
            return FALSE;
        }
        if !(*matchlet).mimetype.is_null() {
            attrs = b"standard::type,access::can-execute,standard::content-type\0" as *const u8
                as *const ::core::ffi::c_char as *const gchar;
        } else {
            attrs = b"standard::type,access::can-execute\0" as *const u8
                as *const ::core::ffi::c_char as *const gchar;
        }
        info = g_file_query_info(
            file,
            attrs as *const ::core::ffi::c_char,
            G_FILE_QUERY_INFO_NONE,
            ::core::ptr::null_mut::<GCancellable>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !info.is_null() {
            result = TRUE as gboolean;
            if (*matchlet).type_0 as ::core::ffi::c_uint
                != G_FILE_TYPE_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
                && g_file_info_get_file_type(info) as ::core::ffi::c_uint
                    != (*matchlet).type_0 as ::core::ffi::c_uint
            {
                result = FALSE as gboolean;
            }
            if (*matchlet).executable() as ::core::ffi::c_int != 0
                && g_file_info_get_attribute_boolean(
                    info,
                    G_FILE_ATTRIBUTE_ACCESS_CAN_EXECUTE.as_ptr(),
                ) == 0
            {
                result = FALSE as gboolean;
            }
        } else {
            result = FALSE as gboolean;
        }
        if result != 0 && (*matchlet).non_empty() as ::core::ffi::c_int != 0 {
            let mut child_enum: *mut GFileEnumerator = ::core::ptr::null_mut::<GFileEnumerator>();
            let mut child_info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
            child_enum = g_file_enumerate_children(
                file,
                G_FILE_ATTRIBUTE_STANDARD_NAME.as_ptr(),
                G_FILE_QUERY_INFO_NONE,
                ::core::ptr::null_mut::<GCancellable>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            if !child_enum.is_null() {
                child_info = g_file_enumerator_next_file(
                    child_enum,
                    ::core::ptr::null_mut::<GCancellable>(),
                    ::core::ptr::null_mut::<*mut GError>(),
                );
                if !child_info.is_null() {
                    g_object_unref(child_info as gpointer);
                } else {
                    result = FALSE as gboolean;
                }
                g_object_unref(child_enum as gpointer);
            } else {
                result = FALSE as gboolean;
            }
        }
        if result != 0 && !(*matchlet).mimetype.is_null() {
            if strcmp((*matchlet).mimetype, g_file_info_get_content_type(info))
                != 0 as ::core::ffi::c_int
            {
                result = FALSE as gboolean;
            }
        }
        if !info.is_null() {
            g_object_unref(info as gpointer);
        }
        g_object_unref(file as gpointer);
        if !(result == 0) {
            break;
        }
    }
    safe_c2rust_enumerator_free(e);
    if (*matchlet).matches.is_null() {
        return TRUE;
    }
    l = (*matchlet).matches;
    while !l.is_null() {
        let mut submatchlet: *mut TreeMatchlet = ::core::ptr::null_mut::<TreeMatchlet>();
        submatchlet = (*l).data as *mut TreeMatchlet;
        if safe_c2rust_matchlet_match(submatchlet, root) != 0 {
            return TRUE;
        }
        l = (*l).next;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_match_match(
    mut match_0: *mut TreeMatch,
    mut root: *mut GFile,
    mut types: *mut GPtrArray,
) {
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    l = (*match_0).matches;
    while !l.is_null() {
        let mut matchlet: *mut TreeMatchlet = (*l).data as *mut TreeMatchlet;
        if safe_c2rust_matchlet_match(matchlet, root) != 0 {
            g_ptr_array_add(
                types,
                safe_c2rust_g_strdup_inline((*match_0).contenttype) as gpointer,
            );
            break;
        } else {
            l = (*l).next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_content_type_guess_for_tree(
    mut root: *mut GFile,
) -> *mut *mut gchar {
    let mut types: *mut GPtrArray = ::core::ptr::null_mut::<GPtrArray>();
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    types = g_ptr_array_new();
    g_mutex_lock(&raw mut safe_c2rust_g__gio_treemagic_lock);
    safe_c2rust_tree_magic_init();
    l = safe_c2rust_tree_matches;
    while !l.is_null() {
        let mut match_0: *mut TreeMatch = (*l).data as *mut TreeMatch;
        safe_c2rust_match_match(match_0, root, types);
        l = (*l).next;
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__gio_treemagic_lock);
    g_ptr_array_add(types, NULL_1);
    return g_ptr_array_free(types, FALSE) as *mut *mut gchar;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
