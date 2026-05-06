extern "C" {
    pub type _GBytes;
    pub type _GHashTable;
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_error_free(error: *mut GError);
    fn g_error_matches(error: *const GError, domain: GQuark, code: gint) -> gboolean;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_get_user_data_dir() -> *const gchar;
    fn g_get_system_data_dirs() -> *const *const gchar;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn g_locale_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_file_error_quark() -> GQuark;
    fn g_file_error_from_errno(err_no: gint) -> GFileError;
    fn g_file_set_contents(
        filename: *const gchar,
        contents: *const gchar,
        length: gssize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_free(mem: gpointer);
    fn g_free_sized(mem: gpointer, size: size_t);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free_1(list: *mut GList);
    fn g_list_free_full(list: *mut GList, free_func: GDestroyNotify);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_insert(list: *mut GList, data: gpointer, position: gint) -> *mut GList;
    fn g_list_insert_before(list: *mut GList, sibling: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_remove(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_list_remove_link(list: *mut GList, llink: *mut GList) -> *mut GList;
    fn g_list_find(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_list_last(list: *mut GList) -> *mut GList;
    fn g_list_length(list: *mut GList) -> guint;
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_replace(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn bind_textdomain_codeset(
        __domainname: *const ::core::ffi::c_char,
        __codeset: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn g_slist_free(list: *mut GSList);
    fn g_slist_free_full(list: *mut GSList, free_func: GDestroyNotify);
    fn g_slist_prepend(list: *mut GSList, data: gpointer) -> *mut GSList;
    fn g_slist_reverse(list: *mut GSList) -> *mut GSList;
    fn g_slist_length(list: *mut GSList) -> guint;
    fn g_unichar_isalnum(c: gunichar) -> gboolean;
    fn g_utf8_get_char_validated(p: *const gchar, max_len: gssize) -> gunichar;
    fn g_utf8_find_next_char(p: *const gchar, end: *const gchar) -> *mut gchar;
    fn g_utf8_strchr(p: *const gchar, len: gssize, c: gunichar) -> *mut gchar;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_utf8_make_valid(str: *const gchar, len: gssize) -> *mut gchar;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strstr_len(
        haystack: *const gchar,
        haystack_len: gssize,
        needle: *const gchar,
    ) -> *mut gchar;
    fn g_strrstr(haystack: *const gchar, needle: *const gchar) -> *mut gchar;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_ascii_strtod(nptr: *const gchar, endptr: *mut *mut gchar) -> gdouble;
    fn g_ascii_strtoull(nptr: *const gchar, endptr: *mut *mut gchar, base: guint) -> guint64;
    fn g_ascii_strtoll(nptr: *const gchar, endptr: *mut *mut gchar, base: guint) -> gint64;
    fn g_ascii_dtostr(buffer: *mut gchar, buf_len: gint, d: gdouble) -> *mut gchar;
    fn g_ascii_strncasecmp(s1: *const gchar, s2: *const gchar, n: gsize) -> gint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
    fn g_strfreev(str_array: *mut *mut gchar);
    fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    fn g_strv_length(str_array: *mut *mut gchar) -> guint;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
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
    fn g_string_erase(string: *mut GString, pos: gssize, len: gssize) -> *mut GString;
    fn g_string_append_printf(string: *mut GString, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_warn_message(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        warnexpr: *const ::core::ffi::c_char,
    );
    fn g_get_language_names() -> *const *const gchar;
    fn g_get_locale_variants(locale: *const gchar) -> *mut *mut gchar;
    fn g_dgettext(domain: *const gchar, msgid: *const gchar) -> *const gchar;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
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
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
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
pub type GBytes = _GBytes;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_KEY_FILE_ERROR_INVALID_VALUE: C2RustUnnamed = 5;
pub const G_KEY_FILE_ERROR_GROUP_NOT_FOUND: C2RustUnnamed = 4;
pub const G_KEY_FILE_ERROR_KEY_NOT_FOUND: C2RustUnnamed = 3;
pub const G_KEY_FILE_ERROR_NOT_FOUND: C2RustUnnamed = 2;
pub const G_KEY_FILE_ERROR_PARSE: C2RustUnnamed = 1;
pub const G_KEY_FILE_ERROR_UNKNOWN_ENCODING: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GKeyFile {
    pub groups: *mut GList,
    pub group_hash: *mut GHashTable,
    pub start_group: *mut GKeyFileGroup,
    pub current_group: *mut GKeyFileGroup,
    pub parse_buffer: *mut GString,
    pub list_separator: gchar,
    pub flags: GKeyFileFlags,
    pub checked_locales: gboolean,
    pub locales: *mut *mut gchar,
    pub gettext_domain: *mut gchar,
    pub ref_count: gint,
}
pub type GKeyFileFlags = ::core::ffi::c_uint;
pub const G_KEY_FILE_KEEP_TRANSLATIONS: GKeyFileFlags = 2;
pub const G_KEY_FILE_KEEP_COMMENTS: GKeyFileFlags = 1;
pub const G_KEY_FILE_NONE: GKeyFileFlags = 0;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GKeyFileGroup = _GKeyFileGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GKeyFileGroup {
    pub name: *const gchar,
    pub key_value_pairs: *mut GList,
    pub lookup_map: *mut GHashTable,
}
pub type GHashTable = _GHashTable;
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GKeyFile = _GKeyFile;
pub type GKeyFileKeyValuePair = _GKeyFileKeyValuePair;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GKeyFileKeyValuePair {
    pub key: *mut gchar,
    pub value: *mut gchar,
}
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub const G_ASCII_SPACE: C2RustUnnamed_0 = 256;
pub type gunichar = guint32;
pub const G_ASCII_CNTRL: C2RustUnnamed_0 = 4;
pub type GFileError = ::core::ffi::c_uint;
pub const G_FILE_ERROR_FAILED: GFileError = 24;
pub const G_FILE_ERROR_NOSYS: GFileError = 23;
pub const G_FILE_ERROR_PERM: GFileError = 22;
pub const G_FILE_ERROR_IO: GFileError = 21;
pub const G_FILE_ERROR_INTR: GFileError = 20;
pub const G_FILE_ERROR_AGAIN: GFileError = 19;
pub const G_FILE_ERROR_PIPE: GFileError = 18;
pub const G_FILE_ERROR_INVAL: GFileError = 17;
pub const G_FILE_ERROR_BADF: GFileError = 16;
pub const G_FILE_ERROR_NFILE: GFileError = 15;
pub const G_FILE_ERROR_MFILE: GFileError = 14;
pub const G_FILE_ERROR_NOMEM: GFileError = 13;
pub const G_FILE_ERROR_NOSPC: GFileError = 12;
pub const G_FILE_ERROR_LOOP: GFileError = 11;
pub const G_FILE_ERROR_FAULT: GFileError = 10;
pub const G_FILE_ERROR_TXTBSY: GFileError = 9;
pub const G_FILE_ERROR_ROFS: GFileError = 8;
pub const G_FILE_ERROR_NODEV: GFileError = 7;
pub const G_FILE_ERROR_NXIO: GFileError = 6;
pub const G_FILE_ERROR_NOTDIR: GFileError = 5;
pub const G_FILE_ERROR_NOENT: GFileError = 4;
pub const G_FILE_ERROR_NAMETOOLONG: GFileError = 3;
pub const G_FILE_ERROR_ACCES: GFileError = 2;
pub const G_FILE_ERROR_ISDIR: GFileError = 1;
pub const G_FILE_ERROR_EXIST: GFileError = 0;
pub type ssize_t = isize;
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const G_ASCII_XDIGIT: C2RustUnnamed_0 = 1024;
pub const G_ASCII_UPPER: C2RustUnnamed_0 = 512;
pub const G_ASCII_PUNCT: C2RustUnnamed_0 = 128;
pub const G_ASCII_PRINT: C2RustUnnamed_0 = 64;
pub const G_ASCII_LOWER: C2RustUnnamed_0 = 32;
pub const G_ASCII_GRAPH: C2RustUnnamed_0 = 16;
pub const G_ASCII_DIGIT: C2RustUnnamed_0 = 8;
pub const G_ASCII_ALPHA: C2RustUnnamed_0 = 2;
pub const G_ASCII_ALNUM: C2RustUnnamed_0 = 1;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_DIR_SEPARATOR: ::core::ffi::c_int = '/' as i32;
pub const NULL_1: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_KEY_FILE_DESKTOP_GROUP: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"Desktop Entry\0") };
pub const G_KEY_FILE_DESKTOP_KEY_NAME: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Name\0") };
pub const G_KEY_FILE_DESKTOP_KEY_GENERIC_NAME: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"GenericName\0") };
pub const G_KEY_FILE_DESKTOP_KEY_COMMENT: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"Comment\0") };
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL_1 as gpointer;
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
pub const G_KEY_FILE_DESKTOP_KEY_GETTEXT_DOMAIN: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"X-GNOME-Gettext-Domain\0")
};
pub const G_KEY_FILE_DESKTOP_KEY_FULLNAME: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"X-GNOME-FullName\0")
};
pub const G_KEY_FILE_DESKTOP_KEY_KEYWORDS: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"Keywords\0") };
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-key-file-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
unsafe extern "C" fn safe_c2rust_g_key_file_init(mut key_file: *mut GKeyFile) {
    (*key_file).current_group = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GKeyFileGroup>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GKeyFileGroup;
    (*key_file).groups = g_list_prepend(
        ::core::ptr::null_mut::<GList>(),
        (*key_file).current_group as gpointer,
    );
    (*key_file).group_hash = ::core::ptr::null_mut::<GHashTable>();
    (*key_file).start_group = ::core::ptr::null_mut::<GKeyFileGroup>();
    (*key_file).parse_buffer = ::core::ptr::null_mut::<GString>();
    (*key_file).list_separator = ';' as i32 as gchar;
    (*key_file).flags = G_KEY_FILE_NONE;
    (*key_file).gettext_domain = ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn safe_c2rust_g_key_file_clear(mut key_file: *mut GKeyFile) {
    let mut tmp: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut group_node: *mut GList = ::core::ptr::null_mut::<GList>();
    if !(*key_file).locales.is_null() {
        g_strfreev((*key_file).locales);
        (*key_file).locales = ::core::ptr::null_mut::<*mut gchar>();
    }
    (*key_file).checked_locales = FALSE as gboolean;
    if !(*key_file).parse_buffer.is_null() {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(
                    (*key_file).parse_buffer,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            } else {
                g_string_free_and_steal((*key_file).parse_buffer);
            };
        } else {
            g_string_free(
                (*key_file).parse_buffer,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        };
        (*key_file).parse_buffer = ::core::ptr::null_mut::<GString>();
    }
    if !(*key_file).gettext_domain.is_null() {
        g_free((*key_file).gettext_domain as gpointer);
        (*key_file).gettext_domain = ::core::ptr::null_mut::<gchar>();
    }
    tmp = (*key_file).groups;
    while !tmp.is_null() {
        group_node = tmp;
        tmp = (*tmp).next;
        safe_c2rust_g_key_file_remove_group_node(key_file, group_node);
    }
    if !(*key_file).group_hash.is_null() {
        g_hash_table_destroy((*key_file).group_hash);
        (*key_file).group_hash = ::core::ptr::null_mut::<GHashTable>();
    }
    if !(({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if (*key_file).groups.is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            692 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key_file->groups == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_new() -> *mut GKeyFile {
    let mut key_file: *mut GKeyFile = ::core::ptr::null_mut::<GKeyFile>();
    key_file = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GKeyFile>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GKeyFile;
    (*key_file).ref_count = 1 as ::core::ffi::c_int as gint;
    safe_c2rust_g_key_file_init(key_file);
    return key_file;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_list_separator(
    mut key_file: *mut GKeyFile,
    mut separator: gchar,
) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*key_file).list_separator = separator;
}
unsafe extern "C" fn safe_c2rust_find_file_in_data_dirs(
    mut file: *const gchar,
    mut dirs: *mut *const gchar,
    mut output_file: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gint {
    let mut data_dirs: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut data_dir: *const gchar = ::core::ptr::null::<gchar>();
    let mut path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut fd: gint = 0;
    path = ::core::ptr::null_mut::<gchar>();
    fd = -(1 as ::core::ffi::c_int) as gint;
    if dirs.is_null() {
        return fd;
    }
    data_dirs = dirs;
    while !data_dirs.is_null()
        && {
            data_dir = *data_dirs;
            !data_dir.is_null()
        }
        && fd == -(1 as ::core::ffi::c_int)
    {
        let mut candidate_file: *const gchar = ::core::ptr::null::<gchar>();
        let mut sub_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
        candidate_file = file;
        sub_dir = safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
        while !candidate_file.is_null() && fd == -(1 as ::core::ffi::c_int) {
            let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
            path = g_build_filename(data_dir, sub_dir, candidate_file, NULL);
            fd = open(path, O_RDONLY | O_CLOEXEC, 0 as ::core::ffi::c_int) as gint;
            if fd == -(1 as ::core::ffi::c_int) {
                g_free(path as gpointer);
                path = ::core::ptr::null_mut::<gchar>();
            }
            candidate_file = strchr(candidate_file as *const ::core::ffi::c_char, '-' as i32);
            if candidate_file.is_null() {
                break;
            }
            candidate_file = candidate_file.offset(1);
            g_free(sub_dir as gpointer);
            sub_dir = g_strndup(
                file,
                (candidate_file.offset_from(file) as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                    as gsize,
            );
            p = sub_dir;
            while *p as ::core::ffi::c_int != '\0' as i32 {
                if *p as ::core::ffi::c_int == '-' as i32 {
                    *p = G_DIR_SEPARATOR as gchar;
                }
                p = p.offset(1);
            }
        }
        g_free(sub_dir as gpointer);
        data_dirs = data_dirs.offset(1);
    }
    if fd == -(1 as ::core::ffi::c_int) {
        g_set_error_literal(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Valid key file could not be found in search dirs\0" as *const u8 as *const gchar,
            ),
        );
    }
    if !output_file.is_null() && fd != -(1 as ::core::ffi::c_int) {
        *output_file = safe_c2rust_g_strdup_inline(path) as *mut gchar;
    }
    g_free(path as gpointer);
    return fd;
}
unsafe extern "C" fn safe_c2rust_g_key_file_load_from_fd(
    mut key_file: *mut GKeyFile,
    mut fd: gint,
    mut flags: GKeyFileFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut bytes_read: gssize = 0;
    let mut stat_buf: stat = stat {
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
    let mut read_buf: [gchar; 4096] = [0; 4096];
    let mut list_separator: gchar = 0;
    if fstat(fd as ::core::ffi::c_int, &raw mut stat_buf) < 0 as ::core::ffi::c_int {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error_literal(
            error,
            g_file_error_quark(),
            g_file_error_from_errno(errsv as gint) as gint,
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    if !(stat_buf.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
        g_set_error_literal(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_PARSE as ::core::ffi::c_int as gint,
            glib_gettext(b"Not a regular file\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    list_separator = (*key_file).list_separator;
    safe_c2rust_g_key_file_clear(key_file);
    safe_c2rust_g_key_file_init(key_file);
    (*key_file).list_separator = list_separator;
    (*key_file).flags = flags;
    loop {
        let mut errsv_0: ::core::ffi::c_int = 0;
        bytes_read = read(
            fd as ::core::ffi::c_int,
            &raw mut read_buf as *mut gchar as *mut ::core::ffi::c_void,
            4096 as size_t,
        ) as gssize;
        errsv_0 = *__errno_location();
        if bytes_read == 0 as gssize {
            break;
        }
        if bytes_read < 0 as gssize {
            if !(errsv_0 == EINTR || errsv_0 == EAGAIN) {
                g_set_error_literal(
                    error,
                    g_file_error_quark(),
                    g_file_error_from_errno(errsv_0 as gint) as gint,
                    g_strerror(errsv_0 as gint),
                );
                return FALSE;
            }
        } else {
            safe_c2rust_g_key_file_parse_data(
                key_file,
                &raw mut read_buf as *mut gchar,
                bytes_read as gsize,
                &raw mut key_file_error,
            );
        }
        if !key_file_error.is_null() {
            break;
        }
    }
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
        return FALSE;
    }
    safe_c2rust_g_key_file_flush_parse_buffer(key_file, &raw mut key_file_error);
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
        return FALSE;
    }
    (*key_file).gettext_domain = safe_c2rust_g_key_file_get_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_GETTEXT_DOMAIN.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if (*key_file).gettext_domain.is_null() {
        (*key_file).gettext_domain = safe_c2rust_g_key_file_get_string(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            b"X-Ubuntu-Gettext-Domain\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if (*key_file).gettext_domain.is_null() {
        (*key_file).gettext_domain = safe_c2rust_g_key_file_get_string(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            b"X-Debian-Gettext-Domain\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_load_from_file(
    mut key_file: *mut GKeyFile,
    mut file: *const gchar,
    mut flags: GKeyFileFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut fd: gint = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !file.is_null() {
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
            b"file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    fd = open(
        file as *const ::core::ffi::c_char,
        O_RDONLY | O_CLOEXEC,
        0 as ::core::ffi::c_int,
    ) as gint;
    errsv = *__errno_location();
    if fd == -(1 as ::core::ffi::c_int) {
        g_set_error_literal(
            error,
            g_file_error_quark(),
            g_file_error_from_errno(errsv as gint) as gint,
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    safe_c2rust_g_key_file_load_from_fd(key_file, fd, flags, &raw mut key_file_error);
    close(fd as ::core::ffi::c_int);
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_load_from_data(
    mut key_file: *mut GKeyFile,
    mut data: *const gchar,
    mut length: gsize,
    mut flags: GKeyFileFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut list_separator: gchar = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !data.is_null() || length == 0 as gsize {
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
            b"data != NULL || length == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if length == -(1 as ::core::ffi::c_int) as gsize {
        length = strlen(data as *const ::core::ffi::c_char) as gsize;
    }
    list_separator = (*key_file).list_separator;
    safe_c2rust_g_key_file_clear(key_file);
    safe_c2rust_g_key_file_init(key_file);
    (*key_file).list_separator = list_separator;
    (*key_file).flags = flags;
    safe_c2rust_g_key_file_parse_data(key_file, data, length, &raw mut key_file_error);
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
        return FALSE;
    }
    safe_c2rust_g_key_file_flush_parse_buffer(key_file, &raw mut key_file_error);
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
        return FALSE;
    }
    (*key_file).gettext_domain = safe_c2rust_g_key_file_get_string(
        key_file,
        G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
        G_KEY_FILE_DESKTOP_KEY_GETTEXT_DOMAIN.as_ptr() as *const gchar,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if (*key_file).gettext_domain.is_null() {
        (*key_file).gettext_domain = safe_c2rust_g_key_file_get_string(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            b"X-Ubuntu-Gettext-Domain\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    if (*key_file).gettext_domain.is_null() {
        (*key_file).gettext_domain = safe_c2rust_g_key_file_get_string(
            key_file,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr() as *const gchar,
            b"X-Debian-Gettext-Domain\0" as *const u8 as *const gchar,
            ::core::ptr::null_mut::<*mut GError>(),
        );
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_load_from_bytes(
    mut key_file: *mut GKeyFile,
    mut bytes: *mut GBytes,
    mut flags: GKeyFileFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut data: *const guchar = ::core::ptr::null::<guchar>();
    let mut size: gsize = 0;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !bytes.is_null() {
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
            b"bytes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    data = g_bytes_get_data(bytes, &raw mut size) as *const guchar;
    return safe_c2rust_g_key_file_load_from_data(
        key_file,
        data as *const gchar,
        size,
        flags,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_load_from_dirs(
    mut key_file: *mut GKeyFile,
    mut file: *const gchar,
    mut search_dirs: *mut *const gchar,
    mut full_path: *mut *mut gchar,
    mut flags: GKeyFileFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut data_dirs: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut output_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut fd: gint = 0;
    let mut found_file: gboolean = 0;
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if g_path_is_absolute(file) == 0 {
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
            b"!g_path_is_absolute (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !search_dirs.is_null() {
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
            b"search_dirs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    found_file = FALSE as gboolean;
    data_dirs = search_dirs;
    output_path = ::core::ptr::null_mut::<gchar>();
    while !(*data_dirs).is_null() && found_file == 0 {
        g_free(output_path as gpointer);
        output_path = ::core::ptr::null_mut::<gchar>();
        fd = safe_c2rust_find_file_in_data_dirs(
            file,
            data_dirs,
            &raw mut output_path,
            &raw mut key_file_error,
        );
        if fd == -(1 as ::core::ffi::c_int) {
            if !key_file_error.is_null() {
                g_propagate_error(error, key_file_error);
            }
            break;
        } else {
            found_file =
                safe_c2rust_g_key_file_load_from_fd(key_file, fd, flags, &raw mut key_file_error);
            close(fd as ::core::ffi::c_int);
            if key_file_error.is_null() {
                continue;
            }
            g_propagate_error(error, key_file_error);
            break;
        }
    }
    if found_file != 0 && !full_path.is_null() {
        *full_path = output_path;
    } else {
        g_free(output_path as gpointer);
    }
    return found_file;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_load_from_data_dirs(
    mut key_file: *mut GKeyFile,
    mut file: *const gchar,
    mut full_path: *mut *mut gchar,
    mut flags: GKeyFileFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut all_data_dirs: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut user_data_dir: *const gchar = ::core::ptr::null::<gchar>();
    let mut system_data_dirs: *const *const gchar = ::core::ptr::null::<*const gchar>();
    let mut i: gsize = 0;
    let mut j: gsize = 0;
    let mut found_file: gboolean = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if g_path_is_absolute(file) == 0 {
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
            b"!g_path_is_absolute (file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    user_data_dir = g_get_user_data_dir();
    system_data_dirs = g_get_system_data_dirs();
    all_data_dirs = ({
        let mut __n: gsize =
            g_strv_length(system_data_dirs as *mut *mut gchar).wrapping_add(2 as guint) as gsize;
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
    let fresh1 = i;
    i = i.wrapping_add(1);
    let ref mut fresh2 = *all_data_dirs.offset(fresh1 as isize);
    *fresh2 =
        safe_c2rust_g_strdup_inline(user_data_dir as *const ::core::ffi::c_char) as *mut gchar;
    j = 0 as gsize;
    while !(*system_data_dirs.offset(j as isize)).is_null() {
        let fresh3 = j;
        j = j.wrapping_add(1);
        let fresh4 = i;
        i = i.wrapping_add(1);
        let ref mut fresh5 = *all_data_dirs.offset(fresh4 as isize);
        *fresh5 = safe_c2rust_g_strdup_inline(
            *system_data_dirs.offset(fresh3 as isize) as *const ::core::ffi::c_char
        ) as *mut gchar;
    }
    let ref mut fresh6 = *all_data_dirs.offset(i as isize);
    *fresh6 = ::core::ptr::null_mut::<gchar>();
    found_file = safe_c2rust_g_key_file_load_from_dirs(
        key_file,
        file,
        all_data_dirs as *mut *const gchar,
        full_path,
        flags,
        error,
    );
    g_strfreev(all_data_dirs);
    return found_file;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_ref(mut key_file: *mut GKeyFile) -> *mut GKeyFile {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GKeyFile>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*key_file).ref_count;
        (*key_file).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*key_file).ref_count, 1 as ::core::ffi::c_int);
    return key_file;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_free(mut key_file: *mut GKeyFile) {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_key_file_clear(key_file);
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*key_file).ref_count;
            (*key_file).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*key_file).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        g_free_sized(
            key_file as gpointer,
            ::core::mem::size_of::<GKeyFile>() as size_t,
        );
    } else {
        safe_c2rust_g_key_file_init(key_file);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_unref(mut key_file: *mut GKeyFile) {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*key_file).ref_count;
            (*key_file).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*key_file).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        safe_c2rust_g_key_file_clear(key_file);
        g_free_sized(
            key_file as gpointer,
            ::core::mem::size_of::<GKeyFile>() as size_t,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_key_file_locale_is_interesting(
    mut key_file: *mut GKeyFile,
    mut locale: *const gchar,
    mut locale_len: gsize,
) -> gboolean {
    let mut i: gsize = 0;
    if (*key_file).flags as ::core::ffi::c_uint
        & G_KEY_FILE_KEEP_TRANSLATIONS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        return TRUE;
    }
    if (*key_file).checked_locales == 0 {
        if ({
            let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
            if (*key_file).locales.is_null() {
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
                b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                1289 as ::core::ffi::c_int,
                G_STRFUNC,
                b"key_file->locales == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*key_file).locales = g_strdupv(g_get_language_names() as *mut *mut gchar);
        (*key_file).checked_locales = TRUE as gboolean;
    }
    i = 0 as gsize;
    while !(*(*key_file).locales.offset(i as isize)).is_null() {
        if g_ascii_strncasecmp(*(*key_file).locales.offset(i as isize), locale, locale_len)
            == 0 as ::core::ffi::c_int
            && *(*(*key_file).locales.offset(i as isize)).offset(locale_len as isize)
                as ::core::ffi::c_int
                == '\0' as i32
        {
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_line(
    mut key_file: *mut GKeyFile,
    mut line: *const gchar,
    mut length: gsize,
    mut error: *mut *mut GError,
) {
    let mut parse_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut line_start: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !line.is_null() {
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
            b"line != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    line_start = line;
    while *safe_c2rust_g_ascii_table.offset(*line_start as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_SPACE as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        line_start = line_start.offset(1);
    }
    if safe_c2rust_g_key_file_line_is_comment(line_start) != 0 {
        safe_c2rust_g_key_file_parse_comment(key_file, line, length, &raw mut parse_error);
    } else if safe_c2rust_g_key_file_line_is_group(line_start) != 0 {
        safe_c2rust_g_key_file_parse_group(
            key_file,
            line_start,
            length.wrapping_sub(line_start.offset_from(line) as ::core::ffi::c_long as gsize),
            &raw mut parse_error,
        );
    } else if safe_c2rust_g_key_file_line_is_key_value_pair(line_start) != 0 {
        safe_c2rust_g_key_file_parse_key_value_pair(
            key_file,
            line_start,
            length.wrapping_sub(line_start.offset_from(line) as ::core::ffi::c_long as gsize),
            &raw mut parse_error,
        );
    } else {
        let mut line_utf8: *mut gchar = g_utf8_make_valid(line, length as gssize);
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_PARSE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file contains line \xE2\x80\x9C%s\xE2\x80\x9D which is not a key-value pair, group, or comment\0"
                    as *const u8 as *const gchar,
            ),
            line_utf8,
        );
        g_free(line_utf8 as gpointer);
        return;
    }
    if !parse_error.is_null() {
        g_propagate_error(error, parse_error);
    }
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_comment(
    mut key_file: *mut GKeyFile,
    mut line: *const gchar,
    mut length: gsize,
    mut error: *mut *mut GError,
) {
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    if (*key_file).flags as ::core::ffi::c_uint
        & G_KEY_FILE_KEEP_COMMENTS as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
    {
        return;
    }
    if !(({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !(*key_file).current_group.is_null() {
            _g_boolean_var_28 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_28 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_28
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1358 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key_file->current_group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pair = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GKeyFileKeyValuePair>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GKeyFileKeyValuePair;
    (*pair).key = ::core::ptr::null_mut::<gchar>();
    (*pair).value = g_strndup(line, length);
    (*(*key_file).current_group).key_value_pairs = g_list_prepend(
        (*(*key_file).current_group).key_value_pairs,
        pair as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_group(
    mut key_file: *mut GKeyFile,
    mut line: *const gchar,
    mut length: gsize,
    mut error: *mut *mut GError,
) {
    let mut group_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut group_name_start: *const gchar = ::core::ptr::null::<gchar>();
    let mut group_name_end: *const gchar = ::core::ptr::null::<gchar>();
    group_name_start = line.offset(1 as ::core::ffi::c_int as isize);
    group_name_end = line
        .offset(length as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    while *group_name_end as ::core::ffi::c_int != ']' as i32 {
        group_name_end = group_name_end.offset(-1);
    }
    group_name = g_strndup(
        group_name_start,
        group_name_end.offset_from(group_name_start) as ::core::ffi::c_long as gsize,
    );
    if safe_c2rust_g_key_file_is_group_name(group_name) == 0 {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_PARSE as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid group name: %s\0" as *const u8 as *const gchar),
            group_name,
        );
        g_free(group_name as gpointer);
        return;
    }
    safe_c2rust_g_key_file_add_group(key_file, group_name, FALSE);
    g_free(group_name as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_key_value_pair(
    mut key_file: *mut GKeyFile,
    mut line: *const gchar,
    mut length: gsize,
    mut error: *mut *mut GError,
) {
    let mut key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut key_end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut value_start: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut locale: *const gchar = ::core::ptr::null::<gchar>();
    let mut locale_len: gsize = 0;
    let mut key_len: gsize = 0;
    let mut value_len: gsize = 0;
    if (*key_file).current_group.is_null() || (*(*key_file).current_group).name.is_null() {
        g_set_error_literal(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_GROUP_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(b"Key file does not start with a group\0" as *const u8 as *const gchar),
        );
        return;
    }
    value_start = strchr(line as *const ::core::ffi::c_char, '=' as i32) as *mut gchar;
    key_end = value_start;
    if !(({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !key_end.is_null() {
            _g_boolean_var_29 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_29 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_29
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1422 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key_end != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    key_end = key_end.offset(-1);
    value_start = value_start.offset(1);
    while *safe_c2rust_g_ascii_table.offset(*key_end as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_SPACE as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        key_end = key_end.offset(-1);
    }
    key_len =
        (key_end.offset_from(line) as ::core::ffi::c_long + 2 as ::core::ffi::c_long) as gsize;
    if !(({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if key_len <= length {
            _g_boolean_var_30 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_30 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_30
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1434 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key_len <= length\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if safe_c2rust_g_key_file_is_key_name(line, key_len.wrapping_sub(1 as gsize)) == 0 {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_PARSE as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid key name: %.*s\0" as *const u8 as *const gchar),
            key_len as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
            line,
        );
        return;
    }
    key = g_strndup(line, key_len.wrapping_sub(1 as gsize));
    while *safe_c2rust_g_ascii_table.offset(*value_start as guchar as isize) as ::core::ffi::c_int
        & G_ASCII_SPACE as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        value_start = value_start.offset(1);
    }
    value_len =
        line.offset(length as isize).offset_from(value_start) as ::core::ffi::c_long as gsize;
    if !(({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !(*key_file).start_group.is_null() {
            _g_boolean_var_31 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_31 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_31
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1453 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key_file->start_group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !(*key_file).current_group.is_null() {
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
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1456 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key_file->current_group != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !(*(*key_file).current_group).name.is_null() {
            _g_boolean_var_33 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_33 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_33
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            1457 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key_file->current_group->name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*key_file).start_group == (*key_file).current_group
        && strcmp(
            key,
            b"Encoding\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        if value_len as size_t != strlen(b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char)
            || g_ascii_strncasecmp(
                value_start,
                b"UTF-8\0" as *const u8 as *const gchar,
                value_len,
            ) != 0 as ::core::ffi::c_int
        {
            let mut value_utf8: *mut gchar = g_utf8_make_valid(value_start, value_len as gssize);
            g_set_error(
                error,
                safe_c2rust_g_key_file_error_quark(),
                G_KEY_FILE_ERROR_UNKNOWN_ENCODING as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Key file contains unsupported encoding \xE2\x80\x9C%s\xE2\x80\x9D\0"
                        as *const u8 as *const gchar,
                ),
                value_utf8,
            );
            g_free(value_utf8 as gpointer);
            g_free(key as gpointer);
            return;
        }
    }
    locale = safe_c2rust_key_get_locale(key, &raw mut locale_len);
    if locale.is_null()
        || safe_c2rust_g_key_file_locale_is_interesting(key_file, locale, locale_len) != 0
    {
        let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
        pair = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<GKeyFileKeyValuePair>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc_n(__n, __s);
            }
            __p
        }) as *mut GKeyFileKeyValuePair;
        (*pair).key =
            safe_c2rust_g_steal_pointer(&raw mut key as gpointer) as *mut gchar as *mut gchar;
        (*pair).value = g_strndup(value_start, value_len);
        safe_c2rust_g_key_file_add_key_value_pair(
            key_file,
            (*key_file).current_group,
            pair,
            (*(*key_file).current_group).key_value_pairs,
        );
    }
    g_free(key as gpointer);
}
unsafe extern "C" fn safe_c2rust_key_get_locale(
    mut key: *const gchar,
    mut len_out: *mut gsize,
) -> *const gchar {
    let mut locale: *const gchar = ::core::ptr::null::<gchar>();
    let mut locale_len: gsize = 0;
    locale = g_strrstr(key, b"[\0" as *const u8 as *const gchar);
    if !locale.is_null() {
        locale_len = strlen(locale as *const ::core::ffi::c_char) as gsize;
    } else {
        locale_len = 0 as gsize;
    }
    if locale_len > 2 as gsize {
        locale = locale.offset(1);
        locale_len = locale_len.wrapping_sub(2 as gsize);
    } else {
        locale = ::core::ptr::null::<gchar>();
        locale_len = 0 as gsize;
    }
    *len_out = locale_len;
    return locale;
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_data(
    mut key_file: *mut GKeyFile,
    mut data: *const gchar,
    mut length: gsize,
    mut error: *mut *mut GError,
) {
    let mut parse_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !data.is_null() || length == 0 as gsize {
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
            b"data != NULL || length == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    parse_error = ::core::ptr::null_mut::<GError>();
    if (*key_file).parse_buffer.is_null() {
        (*key_file).parse_buffer = g_string_sized_new(128 as gsize);
    }
    i = 0 as gsize;
    while i < length {
        if *data.offset(i as isize) as ::core::ffi::c_int == '\n' as i32 {
            if (*(*key_file).parse_buffer).len > 0 as gsize
                && *(*(*key_file).parse_buffer)
                    .str_0
                    .offset((*(*key_file).parse_buffer).len.wrapping_sub(1 as gsize) as isize)
                    as ::core::ffi::c_int
                    == '\r' as i32
            {
                g_string_erase(
                    (*key_file).parse_buffer,
                    (*(*key_file).parse_buffer).len.wrapping_sub(1 as gsize) as gssize,
                    1 as gssize,
                );
            }
            if (*(*key_file).parse_buffer).len > 0 as gsize {
                safe_c2rust_g_key_file_flush_parse_buffer(key_file, &raw mut parse_error);
            } else {
                safe_c2rust_g_key_file_parse_comment(
                    key_file,
                    b"\0" as *const u8 as *const gchar,
                    1 as gsize,
                    &raw mut parse_error,
                );
            }
            if !parse_error.is_null() {
                g_propagate_error(error, parse_error);
                return;
            }
            i = i.wrapping_add(1);
        } else {
            let mut start_of_line: *const gchar = ::core::ptr::null::<gchar>();
            let mut end_of_line: *const gchar = ::core::ptr::null::<gchar>();
            let mut line_length: gsize = 0;
            start_of_line = data.offset(i as isize);
            end_of_line = memchr(
                start_of_line as *const ::core::ffi::c_void,
                '\n' as i32,
                (length as size_t).wrapping_sub(i as size_t),
            ) as *const gchar;
            if end_of_line.is_null() {
                end_of_line = data.offset(length as isize);
            }
            line_length = end_of_line.offset_from(start_of_line) as ::core::ffi::c_long as gsize;
            safe_c2rust_g_string_append_len_inline(
                (*key_file).parse_buffer,
                start_of_line as *const ::core::ffi::c_char,
                line_length as gssize,
            );
            i = i.wrapping_add(line_length);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_key_file_flush_parse_buffer(
    mut key_file: *mut GKeyFile,
    mut error: *mut *mut GError,
) {
    let mut file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*key_file).parse_buffer.is_null() {
        return;
    }
    file_error = ::core::ptr::null_mut::<GError>();
    if (*(*key_file).parse_buffer).len > 0 as gsize {
        safe_c2rust_g_key_file_parse_line(
            key_file,
            (*(*key_file).parse_buffer).str_0,
            (*(*key_file).parse_buffer).len,
            &raw mut file_error,
        );
        g_string_erase(
            (*key_file).parse_buffer,
            0 as gssize,
            -(1 as ::core::ffi::c_int) as gssize,
        );
        if !file_error.is_null() {
            g_propagate_error(error, file_error);
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_to_data(
    mut key_file: *mut GKeyFile,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut data_string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut group_node: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut key_file_node: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    data_string = g_string_new(::core::ptr::null::<gchar>());
    group_node = g_list_last((*key_file).groups);
    while !group_node.is_null() {
        let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
        group = (*group_node).data as *mut GKeyFileGroup;
        if !(*group).name.is_null() {
            g_string_append_printf(
                data_string,
                b"[%s]\n\0" as *const u8 as *const gchar,
                (*group).name,
            );
        }
        key_file_node = g_list_last((*group).key_value_pairs);
        while !key_file_node.is_null() {
            let mut pair: *mut GKeyFileKeyValuePair =
                ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
            pair = (*key_file_node).data as *mut GKeyFileKeyValuePair;
            if !(*pair).key.is_null() {
                g_string_append_printf(
                    data_string,
                    b"%s=%s\n\0" as *const u8 as *const gchar,
                    (*pair).key,
                    (*pair).value,
                );
            } else {
                g_string_append_printf(
                    data_string,
                    b"%s\n\0" as *const u8 as *const gchar,
                    (*pair).value,
                );
            }
            key_file_node = (*key_file_node).prev;
        }
        group_node = (*group_node).prev;
    }
    if !length.is_null() {
        *length = (*data_string).len;
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(data_string, 0 as gboolean)
        } else {
            g_string_free_and_steal(data_string)
        }
    } else {
        g_string_free(data_string, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_keys(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut *mut gchar {
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    let mut tmp: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut keys: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gsize = 0;
    let mut num_keys: gsize = 0;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if group.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_GROUP_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file does not have group \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            group_name,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    num_keys = 0 as gsize;
    tmp = (*group).key_value_pairs;
    while !tmp.is_null() {
        let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
        pair = (*tmp).data as *mut GKeyFileKeyValuePair;
        if !(*pair).key.is_null() {
            num_keys = num_keys.wrapping_add(1);
        }
        tmp = (*tmp).next;
    }
    keys = ({
        let mut __n: gsize = num_keys.wrapping_add(1 as gsize);
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
    i = num_keys.wrapping_sub(1 as gsize);
    tmp = (*group).key_value_pairs;
    while !tmp.is_null() {
        let mut pair_0: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
        pair_0 = (*tmp).data as *mut GKeyFileKeyValuePair;
        if !(*pair_0).key.is_null() {
            let ref mut fresh10 = *keys.offset(i as isize);
            *fresh10 = safe_c2rust_g_strdup_inline((*pair_0).key) as *mut gchar;
            i = i.wrapping_sub(1);
        }
        tmp = (*tmp).next;
    }
    let ref mut fresh11 = *keys.offset(num_keys as isize);
    *fresh11 = ::core::ptr::null_mut::<gchar>();
    if !length.is_null() {
        *length = num_keys;
    }
    return keys;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_start_group(
    mut key_file: *mut GKeyFile,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if !(*key_file).start_group.is_null() {
        return safe_c2rust_g_strdup_inline(
            (*(*key_file).start_group).name as *const ::core::ffi::c_char,
        ) as *mut gchar;
    }
    return ::core::ptr::null_mut::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_groups(
    mut key_file: *mut GKeyFile,
    mut length: *mut gsize,
) -> *mut *mut gchar {
    let mut group_node: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut groups: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gsize = 0;
    let mut num_groups: gsize = 0;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    num_groups = g_list_length((*key_file).groups) as gsize;
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if num_groups > 0 as gsize {
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
            b"num_groups > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    group_node = g_list_last((*key_file).groups);
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if (*((*group_node).data as *mut GKeyFileGroup)).name.is_null() {
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
            b"((GKeyFileGroup *) group_node->data)->name == NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    groups = ({
        let mut __n: gsize = num_groups;
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
    group_node = (*group_node).prev;
    while !group_node.is_null() {
        let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
        group = (*group_node).data as *mut GKeyFileGroup;
        if !(({
            let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
            if !(*group).name.is_null() {
                _g_boolean_var_44 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_44 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_44
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                1825 as ::core::ffi::c_int,
                G_STRFUNC,
                b"group->name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        let fresh7 = i;
        i = i.wrapping_add(1);
        let ref mut fresh8 = *groups.offset(fresh7 as isize);
        *fresh8 =
            safe_c2rust_g_strdup_inline((*group).name as *const ::core::ffi::c_char) as *mut gchar;
        group_node = (*group_node).prev;
    }
    let ref mut fresh9 = *groups.offset(i as isize);
    *fresh9 = ::core::ptr::null_mut::<gchar>();
    if !length.is_null() {
        *length = i;
    }
    return groups;
}
unsafe extern "C" fn safe_c2rust_set_not_found_key_error(
    mut group_name: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) {
    g_set_error(
        error,
        safe_c2rust_g_key_file_error_quark(),
        G_KEY_FILE_ERROR_KEY_NOT_FOUND as ::core::ffi::c_int as gint,
        glib_gettext(
            b"Key file does not have key \xE2\x80\x9C%s\xE2\x80\x9D in group \xE2\x80\x9C%s\xE2\x80\x9D\0"
                as *const u8 as *const gchar,
        ),
        key,
        group_name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_value(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if group.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_GROUP_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file does not have group \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            group_name,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    pair = safe_c2rust_g_key_file_lookup_key_value_pair(key_file, group, key);
    if !pair.is_null() {
        value = safe_c2rust_g_strdup_inline((*pair).value) as *mut gchar;
    } else {
        safe_c2rust_set_not_found_key_error(
            group_name as *const ::core::ffi::c_char,
            key as *const ::core::ffi::c_char,
            error,
        );
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_value(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut value: *const gchar,
) {
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if !group_name.is_null() && safe_c2rust_g_key_file_is_group_name(group_name) != 0 {
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
            b"group_name != NULL && g_key_file_is_group_name (group_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !key.is_null()
            && safe_c2rust_g_key_file_is_key_name(
                key,
                strlen(key as *const ::core::ffi::c_char) as gsize,
            ) != 0
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
            b"key != NULL && g_key_file_is_key_name (key, strlen (key))\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
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
        return;
    }
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if group.is_null() {
        safe_c2rust_g_key_file_add_group(key_file, group_name, TRUE);
        group = (*(*key_file).groups).data as *mut GKeyFileGroup;
        safe_c2rust_g_key_file_add_key(key_file, group, key, value);
    } else {
        pair = safe_c2rust_g_key_file_lookup_key_value_pair(key_file, group, key);
        if pair.is_null() {
            safe_c2rust_g_key_file_add_key(key_file, group, key, value);
        } else {
            g_free((*pair).value as gpointer);
            (*pair).value =
                safe_c2rust_g_strdup_inline(value as *const ::core::ffi::c_char) as *mut gchar;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_string(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut string_value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    key_file_error = ::core::ptr::null_mut::<GError>();
    value = safe_c2rust_g_key_file_get_value(key_file, group_name, key, &raw mut key_file_error);
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
        return ::core::ptr::null_mut::<gchar>();
    }
    if g_utf8_validate(
        value,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<*const gchar>(),
    ) == 0
    {
        let mut value_utf8: *mut gchar =
            g_utf8_make_valid(value, -(1 as ::core::ffi::c_int) as gssize);
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_UNKNOWN_ENCODING as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file contains key \xE2\x80\x9C%s\xE2\x80\x9D with value \xE2\x80\x9C%s\xE2\x80\x9D which is not UTF-8\0"
                    as *const u8 as *const gchar,
            ),
            key,
            value_utf8,
        );
        g_free(value_utf8 as gpointer);
        g_free(value as gpointer);
        return ::core::ptr::null_mut::<gchar>();
    }
    string_value = safe_c2rust_g_key_file_parse_value_as_string(
        key_file,
        value,
        ::core::ptr::null_mut::<*mut GSList>(),
        &raw mut key_file_error,
    );
    g_free(value as gpointer);
    if !key_file_error.is_null() {
        if g_error_matches(
            key_file_error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
        ) != 0
        {
            g_set_error(
                error,
                safe_c2rust_g_key_file_error_quark(),
                G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Key file contains key \xE2\x80\x9C%s\xE2\x80\x9D which has a value that cannot be interpreted.\0"
                        as *const u8 as *const gchar,
                ),
                key,
            );
            g_error_free(key_file_error);
        } else {
            g_propagate_error(error, key_file_error);
        }
    }
    return string_value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_string(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut string: *const gchar,
) {
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value = safe_c2rust_g_key_file_parse_string_as_value(key_file, string, FALSE);
    safe_c2rust_g_key_file_set_value(key_file, group_name, key, value);
    g_free(value as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_string_list(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut *mut gchar {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut string_value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut values: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gint = 0;
    let mut len: gint = 0;
    let mut p: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut pieces: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if !length.is_null() {
        *length = 0 as gsize;
    }
    value = safe_c2rust_g_key_file_get_value(key_file, group_name, key, &raw mut key_file_error);
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if g_utf8_validate(
        value,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<*const gchar>(),
    ) == 0
    {
        let mut value_utf8: *mut gchar =
            g_utf8_make_valid(value, -(1 as ::core::ffi::c_int) as gssize);
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_UNKNOWN_ENCODING as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file contains key \xE2\x80\x9C%s\xE2\x80\x9D with value \xE2\x80\x9C%s\xE2\x80\x9D which is not UTF-8\0"
                    as *const u8 as *const gchar,
            ),
            key,
            value_utf8,
        );
        g_free(value_utf8 as gpointer);
        g_free(value as gpointer);
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    string_value = safe_c2rust_g_key_file_parse_value_as_string(
        key_file,
        value,
        &raw mut pieces,
        &raw mut key_file_error,
    );
    g_free(value as gpointer);
    g_free(string_value as gpointer);
    if !key_file_error.is_null() {
        if g_error_matches(
            key_file_error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
        ) != 0
        {
            g_set_error(
                error,
                safe_c2rust_g_key_file_error_quark(),
                G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Key file contains key \xE2\x80\x9C%s\xE2\x80\x9D which has a value that cannot be interpreted.\0"
                        as *const u8 as *const gchar,
                ),
                key,
            );
            g_error_free(key_file_error);
        } else {
            g_propagate_error(error, key_file_error);
        }
        g_slist_free_full(pieces, Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    len = g_slist_length(pieces) as gint;
    values = ({
        let mut __n: gsize = (len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gsize;
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
    p = pieces;
    i = 0 as ::core::ffi::c_int as gint;
    while !p.is_null() {
        let fresh12 = i;
        i = i + 1;
        let ref mut fresh13 = *values.offset(fresh12 as isize);
        *fresh13 = (*p).data as *mut gchar;
        p = (*p).next;
    }
    let ref mut fresh14 = *values.offset(len as isize);
    *fresh14 = ::core::ptr::null_mut::<gchar>();
    g_slist_free(pieces);
    if !length.is_null() {
        *length = len as gsize;
    }
    return values;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_string_list(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut list: *const *const gchar,
    mut length: gsize,
) {
    let mut value_list: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !list.is_null() || length == 0 as gsize {
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
            b"list != NULL || length == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value_list = g_string_sized_new(length.wrapping_mul(128 as gsize));
    i = 0 as gsize;
    while i < length && !(*list.offset(i as isize)).is_null() {
        let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
        value =
            safe_c2rust_g_key_file_parse_string_as_value(key_file, *list.offset(i as isize), TRUE);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = value;
                safe_c2rust_g_string_append_len_inline(
                    value_list,
                    __val,
                    if ({
                        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_62
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
                value_list,
                value,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        safe_c2rust_g_string_append_c_inline(value_list, (*key_file).list_separator);
        g_free(value as gpointer);
        i = i.wrapping_add(1);
    }
    safe_c2rust_g_key_file_set_value(key_file, group_name, key, (*value_list).str_0);
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                value_list,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(value_list);
        };
    } else {
        g_string_free(
            value_list,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_locale_string(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut locale: *const gchar,
    mut string: *const gchar,
) {
    let mut full_key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if !locale.is_null() {
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
            b"locale != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if !string.is_null() {
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
            b"string != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value = safe_c2rust_g_key_file_parse_string_as_value(key_file, string, FALSE);
    full_key = g_strdup_printf(b"%s[%s]\0" as *const u8 as *const gchar, key, locale);
    safe_c2rust_g_key_file_set_value(key_file, group_name, full_key, value);
    g_free(full_key as gpointer);
    g_free(value as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_locale_string(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut locale: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut candidate_key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut translated_value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut languages: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut free_languages: gboolean = FALSE;
    let mut try_gettext: gboolean = FALSE;
    let mut msg_locale: *const gchar = ::core::ptr::null::<gchar>();
    let mut i: gint = 0;
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    candidate_key = ::core::ptr::null_mut::<gchar>();
    translated_value = ::core::ptr::null_mut::<gchar>();
    key_file_error = ::core::ptr::null_mut::<GError>();
    if !locale.is_null() {
        languages = g_get_locale_variants(locale);
        free_languages = TRUE as gboolean;
    } else {
        languages = g_get_language_names() as *mut *mut gchar;
        free_languages = FALSE as gboolean;
    }
    msg_locale = setlocale(LC_MESSAGES, ::core::ptr::null::<::core::ffi::c_char>());
    try_gettext = (!msg_locale.is_null()
        && !(*key_file).gettext_domain.is_null()
        && (strcmp(
            group_name as *const ::core::ffi::c_char,
            G_KEY_FILE_DESKTOP_GROUP.as_ptr(),
        ) == 0 as ::core::ffi::c_int
            || (if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char =
                        group_name as *const ::core::ffi::c_char;
                    let __prefix: *const ::core::ffi::c_char =
                        b"Desktop Action\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = FALSE;
                    if ({
                        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_70
                    }) as ::core::ffi::c_long
                        != 0
                    {
                        __result =
                            g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                    } else {
                        let __str_len: size_t =
                            strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                                as size_t;
                        let __prefix_len: size_t = strlen(
                            __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                        ) as size_t;
                        if __str_len >= __prefix_len {
                            __result = (memcmp(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize)
                                    as *const ::core::ffi::c_void,
                                __prefix_len,
                            ) == 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                                as gboolean;
                        }
                    }
                    __result
                })
            } else {
                g_str_has_prefix(group_name, b"Desktop Action\0" as *const u8 as *const gchar)
            }) != 0)
        && (strcmp(
            key as *const ::core::ffi::c_char,
            G_KEY_FILE_DESKTOP_KEY_NAME.as_ptr(),
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                key as *const ::core::ffi::c_char,
                G_KEY_FILE_DESKTOP_KEY_FULLNAME.as_ptr(),
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                key as *const ::core::ffi::c_char,
                G_KEY_FILE_DESKTOP_KEY_GENERIC_NAME.as_ptr(),
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                key as *const ::core::ffi::c_char,
                G_KEY_FILE_DESKTOP_KEY_KEYWORDS.as_ptr(),
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                key as *const ::core::ffi::c_char,
                G_KEY_FILE_DESKTOP_KEY_COMMENT.as_ptr(),
            ) == 0 as ::core::ffi::c_int)) as ::core::ffi::c_int as gboolean;
    i = 0 as ::core::ffi::c_int as gint;
    while !(*languages.offset(i as isize)).is_null() {
        candidate_key = g_strdup_printf(
            b"%s[%s]\0" as *const u8 as *const gchar,
            key,
            *languages.offset(i as isize),
        );
        translated_value = safe_c2rust_g_key_file_get_string(
            key_file,
            group_name,
            candidate_key,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_free(candidate_key as gpointer);
        if !translated_value.is_null() {
            break;
        }
        i += 1;
    }
    if try_gettext != 0 && translated_value.is_null() {
        let mut orig_value: *mut gchar = safe_c2rust_g_key_file_get_string(
            key_file,
            group_name,
            key,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        if !orig_value.is_null() {
            let mut codeset_set: gboolean = 0;
            let mut translated: *const gchar = ::core::ptr::null::<gchar>();
            let mut has_gettext: gboolean = 0;
            codeset_set = (bind_textdomain_codeset(
                (*key_file).gettext_domain,
                b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
            ) != NULL as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                as gboolean;
            translated = ::core::ptr::null::<gchar>();
            translated = g_dgettext((*key_file).gettext_domain, orig_value);
            has_gettext =
                (translated != orig_value as *const gchar) as ::core::ffi::c_int as gboolean;
            g_free(orig_value as gpointer);
            if has_gettext != 0 {
                if codeset_set != 0 {
                    translated_value =
                        safe_c2rust_g_strdup_inline(translated as *const ::core::ffi::c_char)
                            as *mut gchar;
                } else {
                    translated_value = g_locale_to_utf8(
                        translated,
                        -(1 as ::core::ffi::c_int) as gssize,
                        ::core::ptr::null_mut::<gsize>(),
                        ::core::ptr::null_mut::<gsize>(),
                        ::core::ptr::null_mut::<*mut GError>(),
                    );
                }
            } else {
                translated_value = ::core::ptr::null_mut::<gchar>();
            }
        }
    }
    if translated_value.is_null() {
        translated_value =
            safe_c2rust_g_key_file_get_string(key_file, group_name, key, &raw mut key_file_error);
        if translated_value.is_null() {
            g_propagate_error(error, key_file_error);
        }
    }
    if free_languages != 0 {
        g_strfreev(languages);
    }
    return translated_value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_locale_for_key(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut locale: *const gchar,
) -> *mut gchar {
    let mut languages_allocated: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut languages: *const *const gchar = ::core::ptr::null::<*const gchar>();
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_72: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_73: ::core::ffi::c_int = 0;
        if !key.is_null() {
            _g_boolean_var_73 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_73 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_73
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if !locale.is_null() {
        languages_allocated = g_get_locale_variants(locale);
        languages = languages_allocated as *const *const gchar;
    } else {
        languages = g_get_language_names();
    }
    i = 0 as gsize;
    while !(*languages.offset(i as isize)).is_null() {
        let mut candidate_key: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut translated_value: *mut gchar = ::core::ptr::null_mut::<gchar>();
        candidate_key = g_strdup_printf(
            b"%s[%s]\0" as *const u8 as *const gchar,
            key,
            *languages.offset(i as isize),
        );
        translated_value = safe_c2rust_g_key_file_get_string(
            key_file,
            group_name,
            candidate_key,
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_free(translated_value as gpointer);
        g_free(candidate_key as gpointer);
        if !translated_value.is_null() {
            break;
        }
        i = i.wrapping_add(1);
    }
    result = safe_c2rust_g_strdup_inline(*languages.offset(i as isize) as *const ::core::ffi::c_char)
        as *mut gchar;
    g_strfreev(languages_allocated);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_locale_string_list(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut locale: *const gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut *mut gchar {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut values: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut list_separator: [::core::ffi::c_char; 2] = [0; 2];
    let mut len: gsize = 0;
    if ({
        let mut _g_boolean_var_74: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
            _g_boolean_var_74 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_74 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_74
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_75: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
            _g_boolean_var_75 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_75 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_75
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    if ({
        let mut _g_boolean_var_76: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    key_file_error = ::core::ptr::null_mut::<GError>();
    value = safe_c2rust_g_key_file_get_locale_string(
        key_file,
        group_name,
        key,
        locale,
        &raw mut key_file_error,
    );
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
    }
    if value.is_null() {
        if !length.is_null() {
            *length = 0 as gsize;
        }
        return ::core::ptr::null_mut::<*mut gchar>();
    }
    len = strlen(value) as gsize;
    if *value.offset(len.wrapping_sub(1 as gsize) as isize) as ::core::ffi::c_int
        == (*key_file).list_separator as ::core::ffi::c_int
    {
        *value.offset(len.wrapping_sub(1 as gsize) as isize) = '\0' as i32 as gchar;
    }
    list_separator[0 as ::core::ffi::c_int as usize] =
        (*key_file).list_separator as ::core::ffi::c_char;
    list_separator[1 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    values = g_strsplit(
        value,
        &raw mut list_separator as *mut ::core::ffi::c_char,
        0 as gint,
    );
    g_free(value as gpointer);
    if !length.is_null() {
        *length = g_strv_length(values) as gsize;
    }
    return values;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_locale_string_list(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut locale: *const gchar,
    mut list: *const *const gchar,
    mut length: gsize,
) {
    let mut value_list: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut full_key: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_77: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_78: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_79: ::core::ffi::c_int = 0;
        if !locale.is_null() {
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
            b"locale != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_80: ::core::ffi::c_int = 0;
        if length != 0 as gsize {
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
            b"length != 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value_list = g_string_sized_new(length.wrapping_mul(128 as gsize));
    i = 0 as gsize;
    while i < length && !(*list.offset(i as isize)).is_null() {
        let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
        value =
            safe_c2rust_g_key_file_parse_string_as_value(key_file, *list.offset(i as isize), TRUE);
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = value;
                safe_c2rust_g_string_append_len_inline(
                    value_list,
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
                value_list,
                value,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        safe_c2rust_g_string_append_c_inline(value_list, (*key_file).list_separator);
        g_free(value as gpointer);
        i = i.wrapping_add(1);
    }
    full_key = g_strdup_printf(b"%s[%s]\0" as *const u8 as *const gchar, key, locale);
    safe_c2rust_g_key_file_set_value(key_file, group_name, full_key, (*value_list).str_0);
    g_free(full_key as gpointer);
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                value_list,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(value_list);
        };
    } else {
        g_string_free(
            value_list,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_boolean(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut bool_value: gboolean = 0;
    if ({
        let mut _g_boolean_var_82: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_83: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_84: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    value = safe_c2rust_g_key_file_get_value(key_file, group_name, key, &raw mut key_file_error);
    if value.is_null() {
        g_propagate_error(error, key_file_error);
        return FALSE;
    }
    bool_value =
        safe_c2rust_g_key_file_parse_value_as_boolean(key_file, value, &raw mut key_file_error);
    g_free(value as gpointer);
    if !key_file_error.is_null() {
        if g_error_matches(
            key_file_error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
        ) != 0
        {
            g_set_error(
                error,
                safe_c2rust_g_key_file_error_quark(),
                G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Key file contains key \xE2\x80\x9C%s\xE2\x80\x9D which has a value that cannot be interpreted.\0"
                        as *const u8 as *const gchar,
                ),
                key,
            );
            g_error_free(key_file_error);
        } else {
            g_propagate_error(error, key_file_error);
        }
    }
    return bool_value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_boolean(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut value: gboolean,
) {
    let mut result: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_85: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    result = safe_c2rust_g_key_file_parse_boolean_as_value(key_file, value);
    safe_c2rust_g_key_file_set_value(key_file, group_name, key, result);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_boolean_list(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gboolean {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut values: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut bool_values: *mut gboolean = ::core::ptr::null_mut::<gboolean>();
    let mut i: gsize = 0;
    let mut num_bools: gsize = 0;
    if ({
        let mut _g_boolean_var_86: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gboolean>();
    }
    if ({
        let mut _g_boolean_var_87: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gboolean>();
    }
    if ({
        let mut _g_boolean_var_88: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gboolean>();
    }
    if !length.is_null() {
        *length = 0 as gsize;
    }
    key_file_error = ::core::ptr::null_mut::<GError>();
    values = safe_c2rust_g_key_file_get_string_list(
        key_file,
        group_name,
        key,
        &raw mut num_bools,
        &raw mut key_file_error,
    );
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
    }
    if values.is_null() {
        return ::core::ptr::null_mut::<gboolean>();
    }
    bool_values = ({
        let mut __n: gsize = num_bools;
        let mut __s: gsize = ::core::mem::size_of::<gboolean>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut gboolean;
    i = 0 as gsize;
    while i < num_bools {
        *bool_values.offset(i as isize) = safe_c2rust_g_key_file_parse_value_as_boolean(
            key_file,
            *values.offset(i as isize),
            &raw mut key_file_error,
        );
        if !key_file_error.is_null() {
            g_propagate_error(error, key_file_error);
            g_strfreev(values);
            g_free(bool_values as gpointer);
            return ::core::ptr::null_mut::<gboolean>();
        }
        i = i.wrapping_add(1);
    }
    g_strfreev(values);
    if !length.is_null() {
        *length = num_bools;
    }
    return bool_values;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_boolean_list(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut list: *mut gboolean,
    mut length: gsize,
) {
    let mut value_list: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_89: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_90: ::core::ffi::c_int = 0;
        if !list.is_null() {
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
            b"list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    value_list = g_string_sized_new(length.wrapping_mul(8 as gsize));
    i = 0 as gsize;
    while i < length {
        let mut value: *const gchar = ::core::ptr::null::<gchar>();
        value = safe_c2rust_g_key_file_parse_boolean_as_value(key_file, *list.offset(i as isize));
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = value as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    value_list,
                    __val,
                    if ({
                        let mut _g_boolean_var_91: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_91 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_91 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_91
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
                value_list,
                value as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        safe_c2rust_g_string_append_c_inline(value_list, (*key_file).list_separator);
        i = i.wrapping_add(1);
    }
    safe_c2rust_g_key_file_set_value(key_file, group_name, key, (*value_list).str_0);
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(
                value_list,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        } else {
            g_string_free_and_steal(value_list);
        };
    } else {
        g_string_free(
            value_list,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_integer(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> gint {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut int_value: gint = 0;
    if ({
        let mut _g_boolean_var_92: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_93: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_94: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    key_file_error = ::core::ptr::null_mut::<GError>();
    value = safe_c2rust_g_key_file_get_value(key_file, group_name, key, &raw mut key_file_error);
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
        return 0 as gint;
    }
    int_value =
        safe_c2rust_g_key_file_parse_value_as_integer(key_file, value, &raw mut key_file_error);
    g_free(value as gpointer);
    if !key_file_error.is_null() {
        if g_error_matches(
            key_file_error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
        ) != 0
        {
            g_set_error(
                error,
                safe_c2rust_g_key_file_error_quark(),
                G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Key file contains key \xE2\x80\x9C%s\xE2\x80\x9D in group \xE2\x80\x9C%s\xE2\x80\x9D which has a value that cannot be interpreted.\0"
                        as *const u8 as *const gchar,
                ),
                key,
                group_name,
            );
            g_error_free(key_file_error);
        } else {
            g_propagate_error(error, key_file_error);
        }
    }
    return int_value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_integer(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut value: gint,
) {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_95: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    result = safe_c2rust_g_key_file_parse_integer_as_value(key_file, value);
    safe_c2rust_g_key_file_set_value(key_file, group_name, key, result);
    g_free(result as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_int64(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> gint64 {
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut v: gint64 = 0;
    if ({
        let mut _g_boolean_var_96: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gint64;
    }
    if ({
        let mut _g_boolean_var_97: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gint64;
    }
    if ({
        let mut _g_boolean_var_98: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gint64;
    }
    s = safe_c2rust_g_key_file_get_value(key_file, group_name, key, error);
    if s.is_null() {
        return 0 as gint64;
    }
    v = g_ascii_strtoll(s, &raw mut end, 10 as guint);
    if *s as ::core::ffi::c_int == '\0' as i32 || *end as ::core::ffi::c_int != '\0' as i32 {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key \xE2\x80\x9C%s\xE2\x80\x9D in group \xE2\x80\x9C%s\xE2\x80\x9D has value \xE2\x80\x9C%s\xE2\x80\x9D where %s was expected\0"
                    as *const u8 as *const gchar,
            ),
            key,
            group_name,
            s,
            b"int64\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_free(s as gpointer);
        return 0 as gint64;
    }
    g_free(s as gpointer);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_int64(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut value: gint64,
) {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_99: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    result = g_strdup_printf(b"%li\0" as *const u8 as *const gchar, value);
    safe_c2rust_g_key_file_set_value(key_file, group_name, key, result);
    g_free(result as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_uint64(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> guint64 {
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut end: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut v: guint64 = 0;
    if ({
        let mut _g_boolean_var_100: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as guint64;
    }
    if ({
        let mut _g_boolean_var_101: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as guint64;
    }
    if ({
        let mut _g_boolean_var_102: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as guint64;
    }
    s = safe_c2rust_g_key_file_get_value(key_file, group_name, key, error);
    if s.is_null() {
        return 0 as guint64;
    }
    v = g_ascii_strtoull(s, &raw mut end, 10 as guint);
    if *s as ::core::ffi::c_int == '\0' as i32 || *end as ::core::ffi::c_int != '\0' as i32 {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key \xE2\x80\x9C%s\xE2\x80\x9D in group \xE2\x80\x9C%s\xE2\x80\x9D has value \xE2\x80\x9C%s\xE2\x80\x9D where %s was expected\0"
                    as *const u8 as *const gchar,
            ),
            key,
            group_name,
            s,
            b"uint64\0" as *const u8 as *const ::core::ffi::c_char,
        );
        g_free(s as gpointer);
        return 0 as guint64;
    }
    g_free(s as gpointer);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_uint64(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut value: guint64,
) {
    let mut result: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_103: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    result = g_strdup_printf(b"%lu\0" as *const u8 as *const gchar, value);
    safe_c2rust_g_key_file_set_value(key_file, group_name, key, result);
    g_free(result as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_integer_list(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gint {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut values: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut int_values: *mut gint = ::core::ptr::null_mut::<gint>();
    let mut i: gsize = 0;
    let mut num_ints: gsize = 0;
    if ({
        let mut _g_boolean_var_104: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gint>();
    }
    if ({
        let mut _g_boolean_var_105: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gint>();
    }
    if ({
        let mut _g_boolean_var_106: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gint>();
    }
    if !length.is_null() {
        *length = 0 as gsize;
    }
    values = safe_c2rust_g_key_file_get_string_list(
        key_file,
        group_name,
        key,
        &raw mut num_ints,
        &raw mut key_file_error,
    );
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
    }
    if values.is_null() {
        return ::core::ptr::null_mut::<gint>();
    }
    int_values = ({
        let mut __n: gsize = num_ints;
        let mut __s: gsize = ::core::mem::size_of::<gint>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut gint;
    i = 0 as gsize;
    while i < num_ints {
        *int_values.offset(i as isize) = safe_c2rust_g_key_file_parse_value_as_integer(
            key_file,
            *values.offset(i as isize),
            &raw mut key_file_error,
        );
        if !key_file_error.is_null() {
            g_propagate_error(error, key_file_error);
            g_strfreev(values);
            g_free(int_values as gpointer);
            return ::core::ptr::null_mut::<gint>();
        }
        i = i.wrapping_add(1);
    }
    g_strfreev(values);
    if !length.is_null() {
        *length = num_ints;
    }
    return int_values;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_integer_list(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut list: *mut gint,
    mut length: gsize,
) {
    let mut values: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_107: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_108: ::core::ffi::c_int = 0;
        if !list.is_null() {
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
            b"list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    values = g_string_sized_new(length.wrapping_mul(16 as gsize));
    i = 0 as gsize;
    while i < length {
        let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
        value = safe_c2rust_g_key_file_parse_integer_as_value(key_file, *list.offset(i as isize));
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = value;
                safe_c2rust_g_string_append_len_inline(
                    values,
                    __val,
                    if ({
                        let mut _g_boolean_var_109: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_109 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_109 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_109
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
                values,
                value,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        safe_c2rust_g_string_append_c_inline(values, (*key_file).list_separator);
        g_free(value as gpointer);
        i = i.wrapping_add(1);
    }
    safe_c2rust_g_key_file_set_value(key_file, group_name, key, (*values).str_0);
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(values, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        } else {
            g_string_free_and_steal(values);
        };
    } else {
        g_string_free(values, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_double(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> gdouble {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut double_value: gdouble = 0.;
    if ({
        let mut _g_boolean_var_110: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gdouble;
    }
    if ({
        let mut _g_boolean_var_111: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gdouble;
    }
    if ({
        let mut _g_boolean_var_112: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as gdouble;
    }
    key_file_error = ::core::ptr::null_mut::<GError>();
    value = safe_c2rust_g_key_file_get_value(key_file, group_name, key, &raw mut key_file_error);
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
        return 0 as ::core::ffi::c_int as gdouble;
    }
    double_value =
        safe_c2rust_g_key_file_parse_value_as_double(key_file, value, &raw mut key_file_error);
    g_free(value as gpointer);
    if !key_file_error.is_null() {
        if g_error_matches(
            key_file_error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
        ) != 0
        {
            g_set_error(
                error,
                safe_c2rust_g_key_file_error_quark(),
                G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Key file contains key \xE2\x80\x9C%s\xE2\x80\x9D in group \xE2\x80\x9C%s\xE2\x80\x9D which has a value that cannot be interpreted.\0"
                        as *const u8 as *const gchar,
                ),
                key,
                group_name,
            );
            g_error_free(key_file_error);
        } else {
            g_propagate_error(error, key_file_error);
        }
    }
    return double_value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_double(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut value: gdouble,
) {
    let mut result: [gchar; 39] = [0; 39];
    if ({
        let mut _g_boolean_var_113: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_ascii_dtostr(
        &raw mut result as *mut gchar,
        ::core::mem::size_of::<[gchar; 39]>() as gint,
        value,
    );
    safe_c2rust_g_key_file_set_value(key_file, group_name, key, &raw mut result as *mut gchar);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_double_list(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> *mut gdouble {
    let mut key_file_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut values: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut double_values: *mut gdouble = ::core::ptr::null_mut::<gdouble>();
    let mut i: gsize = 0;
    let mut num_doubles: gsize = 0;
    if ({
        let mut _g_boolean_var_114: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gdouble>();
    }
    if ({
        let mut _g_boolean_var_115: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gdouble>();
    }
    if ({
        let mut _g_boolean_var_116: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gdouble>();
    }
    if !length.is_null() {
        *length = 0 as gsize;
    }
    values = safe_c2rust_g_key_file_get_string_list(
        key_file,
        group_name,
        key,
        &raw mut num_doubles,
        &raw mut key_file_error,
    );
    if !key_file_error.is_null() {
        g_propagate_error(error, key_file_error);
    }
    if values.is_null() {
        return ::core::ptr::null_mut::<gdouble>();
    }
    double_values = ({
        let mut __n: gsize = num_doubles;
        let mut __s: gsize = ::core::mem::size_of::<gdouble>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut gdouble;
    i = 0 as gsize;
    while i < num_doubles {
        *double_values.offset(i as isize) = safe_c2rust_g_key_file_parse_value_as_double(
            key_file,
            *values.offset(i as isize),
            &raw mut key_file_error,
        );
        if !key_file_error.is_null() {
            g_propagate_error(error, key_file_error);
            g_strfreev(values);
            g_free(double_values as gpointer);
            return ::core::ptr::null_mut::<gdouble>();
        }
        i = i.wrapping_add(1);
    }
    g_strfreev(values);
    if !length.is_null() {
        *length = num_doubles;
    }
    return double_values;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_double_list(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut list: *mut gdouble,
    mut length: gsize,
) {
    let mut values: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut i: gsize = 0;
    if ({
        let mut _g_boolean_var_117: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_118: ::core::ffi::c_int = 0;
        if !list.is_null() {
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
            b"list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    values = g_string_sized_new(length.wrapping_mul(16 as gsize));
    i = 0 as gsize;
    while i < length {
        let mut result: [gchar; 39] = [0; 39];
        g_ascii_dtostr(
            &raw mut result as *mut gchar,
            ::core::mem::size_of::<[gchar; 39]>() as gint,
            *list.offset(i as isize),
        );
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = &raw mut result as *mut gchar;
                safe_c2rust_g_string_append_len_inline(
                    values,
                    __val,
                    if ({
                        let mut _g_boolean_var_119: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_119 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_119 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_119
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
                values,
                &raw mut result as *mut gchar,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        safe_c2rust_g_string_append_c_inline(values, (*key_file).list_separator);
        i = i.wrapping_add(1);
    }
    safe_c2rust_g_key_file_set_value(key_file, group_name, key, (*values).str_0);
    if 0 != 0 {
        if 0 as ::core::ffi::c_int == 0 {
            g_string_free(values, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        } else {
            g_string_free_and_steal(values);
        };
    } else {
        g_string_free(values, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
    };
}
unsafe extern "C" fn safe_c2rust_g_key_file_set_key_comment(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut comment: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    let mut key_node: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut comment_node: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut tmp: *mut GList = ::core::ptr::null_mut::<GList>();
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if group.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_GROUP_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file does not have group \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            if !group_name.is_null() {
                group_name as *const ::core::ffi::c_char
            } else {
                b"(null)\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        return FALSE;
    }
    key_node = safe_c2rust_g_key_file_lookup_key_value_pair_node(key_file, group, key);
    if key_node.is_null() {
        safe_c2rust_set_not_found_key_error(
            (*group).name as *const ::core::ffi::c_char,
            key as *const ::core::ffi::c_char,
            error,
        );
        return FALSE;
    }
    tmp = (*key_node).next;
    while !tmp.is_null() {
        pair = (*tmp).data as *mut GKeyFileKeyValuePair;
        if !(*pair).key.is_null() {
            break;
        }
        comment_node = tmp;
        tmp = (*tmp).next;
        safe_c2rust_g_key_file_remove_key_value_pair_node(key_file, group, comment_node);
    }
    if comment.is_null() {
        return TRUE;
    }
    pair = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GKeyFileKeyValuePair>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GKeyFileKeyValuePair;
    (*pair).key = ::core::ptr::null_mut::<gchar>();
    (*pair).value = safe_c2rust_g_key_file_parse_comment_as_value(key_file, comment);
    key_node = g_list_insert(key_node, pair as gpointer, 1 as gint);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_set_top_comment(
    mut key_file: *mut GKeyFile,
    mut comment: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut group_node: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    if !(({
        let mut _g_boolean_var_120: ::core::ffi::c_int = 0;
        if !(*key_file).groups.is_null() {
            _g_boolean_var_120 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_120 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_120
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            3451 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key_file->groups != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    group_node = g_list_last((*key_file).groups);
    group = (*group_node).data as *mut GKeyFileGroup;
    if !(({
        let mut _g_boolean_var_121: ::core::ffi::c_int = 0;
        if (*group).name.is_null() {
            _g_boolean_var_121 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_121 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_121
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            3454 as ::core::ffi::c_int,
            G_STRFUNC,
            b"group->name == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_list_free_full(
        (*group).key_value_pairs,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GKeyFileKeyValuePair) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_g_key_file_key_value_pair_free
                as unsafe extern "C" fn(*mut GKeyFileKeyValuePair) -> (),
        )),
    );
    (*group).key_value_pairs = ::core::ptr::null_mut::<GList>();
    if comment.is_null() {
        return TRUE;
    }
    pair = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GKeyFileKeyValuePair>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GKeyFileKeyValuePair;
    (*pair).key = ::core::ptr::null_mut::<gchar>();
    (*pair).value = safe_c2rust_g_key_file_parse_comment_as_value(key_file, comment);
    (*group).key_value_pairs = g_list_prepend((*group).key_value_pairs, pair as gpointer);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_set_group_comment(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut comment: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    let mut group_node: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    if ({
        let mut _g_boolean_var_122: ::core::ffi::c_int = 0;
        if !group_name.is_null() && safe_c2rust_g_key_file_is_group_name(group_name) != 0 {
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
            b"group_name != NULL && g_key_file_is_group_name (group_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if group.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_GROUP_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file does not have group \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            group_name,
        );
        return FALSE;
    }
    if group == (*key_file).start_group {
        return safe_c2rust_g_key_file_set_top_comment(key_file, comment, error);
    }
    group_node = safe_c2rust_g_key_file_lookup_group_node(key_file, group_name);
    group = (*(*group_node).next).data as *mut GKeyFileGroup;
    let mut lp: *mut GList = (*group).key_value_pairs;
    while !lp.is_null() {
        let mut lnext: *mut GList = (*lp).next;
        pair = (*lp).data as *mut GKeyFileKeyValuePair;
        if !(*pair).key.is_null() {
            break;
        }
        safe_c2rust_g_key_file_remove_key_value_pair_node(key_file, group, lp);
        lp = lnext;
    }
    if comment.is_null() {
        return TRUE;
    }
    pair = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GKeyFileKeyValuePair>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GKeyFileKeyValuePair;
    (*pair).key = ::core::ptr::null_mut::<gchar>();
    (*pair).value = safe_c2rust_g_key_file_parse_comment_as_value(key_file, comment);
    (*group).key_value_pairs = g_list_prepend((*group).key_value_pairs, pair as gpointer);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_set_comment(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut comment: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_123: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !group_name.is_null() && !key.is_null() {
        if safe_c2rust_g_key_file_set_key_comment(key_file, group_name, key, comment, error) == 0 {
            return FALSE;
        }
    } else if !group_name.is_null() {
        if safe_c2rust_g_key_file_set_group_comment(key_file, group_name, comment, error) == 0 {
            return FALSE;
        }
    } else if safe_c2rust_g_key_file_set_top_comment(key_file, comment, error) == 0 {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_get_key_comment(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    let mut key_node: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut tmp: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut comment: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_124: ::core::ffi::c_int = 0;
        if !group_name.is_null() && safe_c2rust_g_key_file_is_group_name(group_name) != 0 {
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
            b"group_name != NULL && g_key_file_is_group_name (group_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if group.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_GROUP_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file does not have group \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            if !group_name.is_null() {
                group_name as *const ::core::ffi::c_char
            } else {
                b"(null)\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    key_node = safe_c2rust_g_key_file_lookup_key_value_pair_node(key_file, group, key);
    if key_node.is_null() {
        safe_c2rust_set_not_found_key_error(
            (*group).name as *const ::core::ffi::c_char,
            key as *const ::core::ffi::c_char,
            error,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    string = ::core::ptr::null_mut::<GString>();
    tmp = (*key_node).next;
    if (*key_node).next.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    pair = (*tmp).data as *mut GKeyFileKeyValuePair;
    if !(*pair).key.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    while !(*tmp).next.is_null() {
        pair = (*(*tmp).next).data as *mut GKeyFileKeyValuePair;
        if !(*pair).key.is_null() {
            break;
        }
        tmp = (*tmp).next;
    }
    while tmp != key_node {
        pair = (*tmp).data as *mut GKeyFileKeyValuePair;
        if string.is_null() {
            string = g_string_sized_new(512 as gsize);
        }
        comment = safe_c2rust_g_key_file_parse_value_as_comment(
            key_file,
            (*pair).value,
            ((*tmp).prev == key_node) as ::core::ffi::c_int,
        );
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = comment;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_125: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_125 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_125 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_125
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
                comment,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(comment as gpointer);
        tmp = (*tmp).prev;
    }
    if !string.is_null() {
        comment = g_string_free_and_steal(
            safe_c2rust_g_steal_pointer(&raw mut string as gpointer) as *mut GString
        );
    } else {
        comment = ::core::ptr::null_mut::<gchar>();
    }
    return comment;
}
unsafe extern "C" fn safe_c2rust_get_group_comment(
    mut key_file: *mut GKeyFile,
    mut group: *mut GKeyFileGroup,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut tmp: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut comment: *mut gchar = ::core::ptr::null_mut::<gchar>();
    string = ::core::ptr::null_mut::<GString>();
    tmp = (*group).key_value_pairs;
    while !tmp.is_null() {
        let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
        pair = (*tmp).data as *mut GKeyFileKeyValuePair;
        if !(*pair).key.is_null() {
            tmp = (*tmp).prev;
            break;
        } else {
            if (*tmp).next.is_null() {
                break;
            }
            tmp = (*tmp).next;
        }
    }
    while !tmp.is_null() {
        let mut pair_0: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
        pair_0 = (*tmp).data as *mut GKeyFileKeyValuePair;
        if string.is_null() {
            string = g_string_sized_new(512 as gsize);
        }
        comment = safe_c2rust_g_key_file_parse_value_as_comment(
            key_file,
            (*pair_0).value,
            ((*tmp).prev == NULL as *mut GList) as ::core::ffi::c_int,
        );
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = comment;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_126: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_126 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_126 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_126
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
                comment,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        g_free(comment as gpointer);
        tmp = (*tmp).prev;
    }
    if !string.is_null() {
        return if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(string, 0 as gboolean)
            } else {
                g_string_free_and_steal(string)
            }
        } else {
            g_string_free(string, 0 as gboolean)
        };
    }
    return ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn safe_c2rust_g_key_file_get_group_comment(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut group_node: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if group.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_GROUP_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file does not have group \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            if !group_name.is_null() {
                group_name as *const ::core::ffi::c_char
            } else {
                b"(null)\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    group_node = safe_c2rust_g_key_file_lookup_group_node(key_file, group_name);
    group_node = (*group_node).next;
    group = (*group_node).data as *mut GKeyFileGroup;
    return safe_c2rust_get_group_comment(key_file, group, error);
}
unsafe extern "C" fn safe_c2rust_g_key_file_get_top_comment(
    mut key_file: *mut GKeyFile,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut group_node: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    if !(({
        let mut _g_boolean_var_127: ::core::ffi::c_int = 0;
        if !(*key_file).groups.is_null() {
            _g_boolean_var_127 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_127 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_127
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            3748 as ::core::ffi::c_int,
            G_STRFUNC,
            b"key_file->groups != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    group_node = g_list_last((*key_file).groups);
    group = (*group_node).data as *mut GKeyFileGroup;
    if !(({
        let mut _g_boolean_var_128: ::core::ffi::c_int = 0;
        if (*group).name.is_null() {
            _g_boolean_var_128 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_128 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_128
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            3751 as ::core::ffi::c_int,
            G_STRFUNC,
            b"group->name == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return safe_c2rust_get_group_comment(key_file, group, error);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_get_comment(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_129: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if !group_name.is_null() && !key.is_null() {
        return safe_c2rust_g_key_file_get_key_comment(key_file, group_name, key, error);
    } else if !group_name.is_null() {
        return safe_c2rust_g_key_file_get_group_comment(key_file, group_name, error);
    } else {
        return safe_c2rust_g_key_file_get_top_comment(key_file, error);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_remove_comment(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_130: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if !group_name.is_null() && !key.is_null() {
        return safe_c2rust_g_key_file_set_key_comment(
            key_file,
            group_name,
            key,
            ::core::ptr::null::<gchar>(),
            error,
        );
    } else if !group_name.is_null() {
        return safe_c2rust_g_key_file_set_group_comment(
            key_file,
            group_name,
            ::core::ptr::null::<gchar>(),
            error,
        );
    } else {
        return safe_c2rust_g_key_file_set_top_comment(
            key_file,
            ::core::ptr::null::<gchar>(),
            error,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_has_group(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
) -> gboolean {
    if ({
        let mut _g_boolean_var_131: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_132: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (safe_c2rust_g_key_file_lookup_group(key_file, group_name) != NULL as *mut GKeyFileGroup)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_key_file_has_key_full(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut has_key: *mut gboolean,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    if ({
        let mut _g_boolean_var_133: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_134: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
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
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_135: ::core::ffi::c_int = 0;
        if !key.is_null() {
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
            b"key != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if group.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_GROUP_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file does not have group \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            group_name,
        );
        return FALSE;
    }
    pair = safe_c2rust_g_key_file_lookup_key_value_pair(key_file, group, key);
    if !has_key.is_null() {
        *has_key = (pair != NULL as *mut GKeyFileKeyValuePair) as ::core::ffi::c_int as gboolean;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_has_key(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut temp_error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut has_key: gboolean = 0;
    if safe_c2rust_g_key_file_has_key_full(
        key_file,
        group_name,
        key,
        &raw mut has_key,
        &raw mut temp_error,
    ) != 0
    {
        return has_key;
    } else {
        g_propagate_error(error, temp_error);
        return FALSE;
    };
}
unsafe extern "C" fn safe_c2rust_g_key_file_add_group(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut created: gboolean,
) {
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    if ({
        let mut _g_boolean_var_136: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
            _g_boolean_var_136 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_136 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_136
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_137: ::core::ffi::c_int = 0;
        if !group_name.is_null() && safe_c2rust_g_key_file_is_group_name(group_name) != 0 {
            _g_boolean_var_137 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_137 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_137
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"group_name != NULL && g_key_file_is_group_name (group_name)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if !group.is_null() {
        (*key_file).current_group = group;
        return;
    }
    group = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GKeyFileGroup>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GKeyFileGroup;
    (*group).name = safe_c2rust_g_strdup_inline(group_name as *const ::core::ffi::c_char);
    (*group).lookup_map = g_hash_table_new(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    (*key_file).groups = g_list_prepend((*key_file).groups, group as gpointer);
    (*key_file).current_group = group;
    if (*key_file).start_group.is_null() {
        (*key_file).start_group = group;
    } else if (*key_file).flags as ::core::ffi::c_uint
        & G_KEY_FILE_KEEP_COMMENTS as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0
        || created != 0
    {
        let mut next_group: *mut GKeyFileGroup =
            (*(*(*key_file).groups).next).data as *mut GKeyFileGroup;
        let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
        if !(*next_group).key_value_pairs.is_null() {
            pair = (*(*next_group).key_value_pairs).data as *mut GKeyFileKeyValuePair;
        }
        if (*next_group).key_value_pairs.is_null()
            || !(*pair).key.is_null()
                && g_strstr_len(
                    (*pair).value,
                    -(1 as ::core::ffi::c_int) as gssize,
                    b"\n\0" as *const u8 as *const gchar,
                )
                .is_null()
        {
            let mut pair_0: *mut GKeyFileKeyValuePair = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<GKeyFileKeyValuePair>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc_n(__n, __s);
                }
                __p
            }) as *mut GKeyFileKeyValuePair;
            (*pair_0).key = ::core::ptr::null_mut::<gchar>();
            (*pair_0).value =
                safe_c2rust_g_strdup_inline(b"\0" as *const u8 as *const ::core::ffi::c_char)
                    as *mut gchar;
            (*next_group).key_value_pairs =
                g_list_prepend((*next_group).key_value_pairs, pair_0 as gpointer);
        }
    }
    if (*key_file).group_hash.is_null() {
        (*key_file).group_hash = g_hash_table_new(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
        );
    }
    g_hash_table_insert(
        (*key_file).group_hash,
        (*group).name as gpointer,
        group as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_g_key_file_key_value_pair_free(
    mut pair: *mut GKeyFileKeyValuePair,
) {
    if !pair.is_null() {
        g_free((*pair).key as gpointer);
        g_free((*pair).value as gpointer);
        g_free_sized(
            pair as gpointer,
            ::core::mem::size_of::<GKeyFileKeyValuePair>() as size_t,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_key_file_remove_key_value_pair_node(
    mut key_file: *mut GKeyFile,
    mut group: *mut GKeyFileGroup,
    mut pair_node: *mut GList,
) {
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    pair = (*pair_node).data as *mut GKeyFileKeyValuePair;
    (*group).key_value_pairs = g_list_remove_link((*group).key_value_pairs, pair_node);
    if !(({
        let mut _g_boolean_var_138: ::core::ffi::c_int = 0;
        if !(*pair).value.is_null() {
            _g_boolean_var_138 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_138 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_138
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            4007 as ::core::ffi::c_int,
            G_STRFUNC,
            b"pair->value != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    safe_c2rust_g_key_file_key_value_pair_free(pair);
    g_list_free_1(pair_node);
}
unsafe extern "C" fn safe_c2rust_g_key_file_remove_group_node(
    mut key_file: *mut GKeyFile,
    mut group_node: *mut GList,
) {
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    let mut tmp: *mut GList = ::core::ptr::null_mut::<GList>();
    group = (*group_node).data as *mut GKeyFileGroup;
    if !(*group).name.is_null() {
        if ({
            let mut _g_boolean_var_139: ::core::ffi::c_int = 0;
            if !(*key_file).group_hash.is_null() {
                _g_boolean_var_139 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_139 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_139
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
                4025 as ::core::ffi::c_int,
                G_STRFUNC,
                b"key_file->group_hash\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        g_hash_table_remove((*key_file).group_hash, (*group).name as gconstpointer);
    }
    if (*key_file).current_group == group {
        if !(*key_file).groups.is_null() {
            (*key_file).current_group = (*(*key_file).groups).data as *mut GKeyFileGroup;
        } else {
            (*key_file).current_group = ::core::ptr::null_mut::<GKeyFileGroup>();
        }
    }
    if (*key_file).start_group == group {
        tmp = g_list_last((*key_file).groups);
        while !tmp.is_null() {
            if tmp != group_node && !(*((*tmp).data as *mut GKeyFileGroup)).name.is_null() {
                break;
            }
            tmp = (*tmp).prev;
        }
        if !tmp.is_null() {
            (*key_file).start_group = (*tmp).data as *mut GKeyFileGroup;
        } else {
            (*key_file).start_group = ::core::ptr::null_mut::<GKeyFileGroup>();
        }
    }
    (*key_file).groups = g_list_remove_link((*key_file).groups, group_node);
    tmp = (*group).key_value_pairs;
    while !tmp.is_null() {
        let mut pair_node: *mut GList = ::core::ptr::null_mut::<GList>();
        pair_node = tmp;
        tmp = (*tmp).next;
        safe_c2rust_g_key_file_remove_key_value_pair_node(key_file, group, pair_node);
    }
    if !(({
        let mut _g_boolean_var_140: ::core::ffi::c_int = 0;
        if (*group).key_value_pairs.is_null() {
            _g_boolean_var_140 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_140 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_140
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            4076 as ::core::ffi::c_int,
            G_STRFUNC,
            b"group->key_value_pairs == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*group).lookup_map.is_null() {
        g_hash_table_destroy((*group).lookup_map);
        (*group).lookup_map = ::core::ptr::null_mut::<GHashTable>();
    }
    g_free((*group).name as *mut gchar as gpointer);
    g_free_sized(
        group as gpointer,
        ::core::mem::size_of::<GKeyFileGroup>() as size_t,
    );
    g_list_free_1(group_node);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_remove_group(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut group_node: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_141: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
            _g_boolean_var_141 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_141 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_141
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_142: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
            _g_boolean_var_142 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_142 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_142
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    group_node = safe_c2rust_g_key_file_lookup_group_node(key_file, group_name);
    if group_node.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_GROUP_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file does not have group \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            group_name,
        );
        return FALSE;
    }
    safe_c2rust_g_key_file_remove_group_node(key_file, group_node);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_add_key_value_pair(
    mut key_file: *mut GKeyFile,
    mut group: *mut GKeyFileGroup,
    mut pair: *mut GKeyFileKeyValuePair,
    mut sibling: *mut GList,
) {
    g_hash_table_replace(
        (*group).lookup_map,
        (*pair).key as gpointer,
        pair as gpointer,
    );
    (*group).key_value_pairs =
        g_list_insert_before((*group).key_value_pairs, sibling, pair as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_key_file_add_key(
    mut key_file: *mut GKeyFile,
    mut group: *mut GKeyFileGroup,
    mut key: *const gchar,
    mut value: *const gchar,
) {
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    let mut lp: *mut GList = ::core::ptr::null_mut::<GList>();
    pair = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GKeyFileKeyValuePair>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GKeyFileKeyValuePair;
    (*pair).key = safe_c2rust_g_strdup_inline(key as *const ::core::ffi::c_char) as *mut gchar;
    (*pair).value = safe_c2rust_g_strdup_inline(value as *const ::core::ffi::c_char) as *mut gchar;
    lp = (*group).key_value_pairs;
    while !lp.is_null() && (*((*lp).data as *mut GKeyFileKeyValuePair)).key.is_null() {
        lp = (*lp).next;
    }
    safe_c2rust_g_key_file_add_key_value_pair(key_file, group, pair, lp);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_remove_key(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
    mut key: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    if ({
        let mut _g_boolean_var_143: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
            _g_boolean_var_143 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_143 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_143
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_144: ::core::ffi::c_int = 0;
        if !group_name.is_null() {
            _g_boolean_var_144 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_144 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_144
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"group_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_145: ::core::ffi::c_int = 0;
        if !key.is_null() {
            _g_boolean_var_145 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_145 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_145
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
    pair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if group.is_null() {
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_GROUP_NOT_FOUND as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Key file does not have group \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                    as *const gchar,
            ),
            group_name,
        );
        return FALSE;
    }
    pair = safe_c2rust_g_key_file_lookup_key_value_pair(key_file, group, key);
    if pair.is_null() {
        safe_c2rust_set_not_found_key_error(
            (*group).name as *const ::core::ffi::c_char,
            key as *const ::core::ffi::c_char,
            error,
        );
        return FALSE;
    }
    (*group).key_value_pairs = g_list_remove((*group).key_value_pairs, pair as gconstpointer);
    g_hash_table_remove((*group).lookup_map, (*pair).key as gconstpointer);
    safe_c2rust_g_key_file_key_value_pair_free(pair);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_lookup_group_node(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
) -> *mut GList {
    let mut group: *mut GKeyFileGroup = ::core::ptr::null_mut::<GKeyFileGroup>();
    group = safe_c2rust_g_key_file_lookup_group(key_file, group_name);
    if group.is_null() {
        return ::core::ptr::null_mut::<GList>();
    }
    return g_list_find((*key_file).groups, group as gconstpointer);
}
unsafe extern "C" fn safe_c2rust_g_key_file_lookup_group(
    mut key_file: *mut GKeyFile,
    mut group_name: *const gchar,
) -> *mut GKeyFileGroup {
    if (*key_file).group_hash.is_null() {
        return ::core::ptr::null_mut::<GKeyFileGroup>();
    }
    return g_hash_table_lookup((*key_file).group_hash, group_name as gconstpointer)
        as *mut GKeyFileGroup;
}
unsafe extern "C" fn safe_c2rust_g_key_file_lookup_key_value_pair_node(
    mut key_file: *mut GKeyFile,
    mut group: *mut GKeyFileGroup,
    mut key: *const gchar,
) -> *mut GList {
    let mut key_node: *mut GList = ::core::ptr::null_mut::<GList>();
    key_node = (*group).key_value_pairs;
    while !key_node.is_null() {
        let mut pair: *mut GKeyFileKeyValuePair = ::core::ptr::null_mut::<GKeyFileKeyValuePair>();
        pair = (*key_node).data as *mut GKeyFileKeyValuePair;
        if !(*pair).key.is_null()
            && strcmp((*pair).key, key as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
        {
            break;
        }
        key_node = (*key_node).next;
    }
    return key_node;
}
unsafe extern "C" fn safe_c2rust_g_key_file_lookup_key_value_pair(
    mut key_file: *mut GKeyFile,
    mut group: *mut GKeyFileGroup,
    mut key: *const gchar,
) -> *mut GKeyFileKeyValuePair {
    return g_hash_table_lookup((*group).lookup_map, key as gconstpointer)
        as *mut GKeyFileKeyValuePair;
}
unsafe extern "C" fn safe_c2rust_g_key_file_line_is_comment(mut line: *const gchar) -> gboolean {
    return (*line as ::core::ffi::c_int == '#' as i32
        || *line as ::core::ffi::c_int == '\0' as i32
        || *line as ::core::ffi::c_int == '\n' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_key_file_is_group_name(mut name: *const gchar) -> gboolean {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut q: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_146: ::core::ffi::c_int = 0;
        if !name.is_null() {
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
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            4280 as ::core::ffi::c_int,
            G_STRFUNC,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    q = name;
    p = q;
    while *q as ::core::ffi::c_int != 0
        && *q as ::core::ffi::c_int != ']' as i32
        && *q as ::core::ffi::c_int != '[' as i32
        && !(*safe_c2rust_g_ascii_table.offset(*q as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_CNTRL as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int)
    {
        q = g_utf8_find_next_char(q, ::core::ptr::null::<gchar>());
    }
    if *q as ::core::ffi::c_int != '\0' as i32 || q == p {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_is_key_name(
    mut name: *const gchar,
    mut len: gsize,
) -> gboolean {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut q: *const gchar = ::core::ptr::null::<gchar>();
    let mut end: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_147: ::core::ffi::c_int = 0;
        if !name.is_null() {
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
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            4298 as ::core::ffi::c_int,
            G_STRFUNC,
            b"name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    q = name;
    p = q;
    end = name.offset(len as isize);
    while q < end
        && *q as ::core::ffi::c_int != 0
        && *q as ::core::ffi::c_int != '=' as i32
        && *q as ::core::ffi::c_int != '[' as i32
        && *q as ::core::ffi::c_int != ']' as i32
    {
        q = g_utf8_find_next_char(q, end);
        if q.is_null() {
            q = end;
        }
    }
    if q == p {
        return FALSE;
    }
    if *p as ::core::ffi::c_int == ' ' as i32
        || *q.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int == ' ' as i32
    {
        return FALSE;
    }
    if *q as ::core::ffi::c_int == '[' as i32 {
        q = q.offset(1);
        while q < end
            && *q as ::core::ffi::c_int != '\0' as i32
            && (g_unichar_isalnum(g_utf8_get_char_validated(q, end.offset_from(q) as gssize)) != 0
                || *q as ::core::ffi::c_int == '-' as i32
                || *q as ::core::ffi::c_int == '_' as i32
                || *q as ::core::ffi::c_int == '.' as i32
                || *q as ::core::ffi::c_int == '@' as i32)
        {
            q = g_utf8_find_next_char(q, end);
            if !q.is_null() {
                continue;
            }
            q = end;
            break;
        }
        if *q as ::core::ffi::c_int != ']' as i32 {
            return FALSE;
        }
        q = q.offset(1);
    }
    if q < end {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_line_is_group(mut line: *const gchar) -> gboolean {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    p = line;
    if *p as ::core::ffi::c_int != '[' as i32 {
        return FALSE;
    }
    p = p.offset(1);
    while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != ']' as i32 {
        p = g_utf8_find_next_char(p, ::core::ptr::null::<gchar>());
    }
    if *p as ::core::ffi::c_int != ']' as i32 {
        return FALSE;
    }
    p = g_utf8_find_next_char(p, ::core::ptr::null::<gchar>());
    while *p as ::core::ffi::c_int == ' ' as i32 || *p as ::core::ffi::c_int == '\t' as i32 {
        p = g_utf8_find_next_char(p, ::core::ptr::null::<gchar>());
    }
    if *p != 0 {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_line_is_key_value_pair(
    mut line: *const gchar,
) -> gboolean {
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    p = g_utf8_strchr(
        line,
        -(1 as ::core::ffi::c_int) as gssize,
        '=' as i32 as gunichar,
    );
    if p.is_null() {
        return FALSE;
    }
    if *p as ::core::ffi::c_int
        == *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
    {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_value_as_string(
    mut key_file: *mut GKeyFile,
    mut value: *const gchar,
    mut pieces: *mut *mut GSList,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut current_block: u64;
    let mut string_value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut q0: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut q: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut tmp_pieces: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    if ({
        let mut _g_boolean_var_148: ::core::ffi::c_int = 0;
        if pieces.is_null() || (*pieces).is_null() {
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
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            4411 as ::core::ffi::c_int,
            G_STRFUNC,
            b"pieces == NULL || *pieces == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    string_value = ({
        let mut __n: gsize =
            strlen(value as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as gsize;
        let mut __s: gsize = ::core::mem::size_of::<gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut gchar;
    p = value;
    q = string_value;
    q0 = q;
    loop {
        if !(*p != 0) {
            current_block = 2873832966593178012;
            break;
        }
        if *p as ::core::ffi::c_int == '\\' as i32 {
            p = p.offset(1);
            match *p as ::core::ffi::c_int {
                115 => {
                    *q = ' ' as i32 as gchar;
                }
                110 => {
                    *q = '\n' as i32 as gchar;
                }
                116 => {
                    *q = '\t' as i32 as gchar;
                }
                114 => {
                    *q = '\r' as i32 as gchar;
                }
                92 => {
                    *q = '\\' as i32 as gchar;
                }
                0 => {
                    g_set_error_literal(
                        error,
                        safe_c2rust_g_key_file_error_quark(),
                        G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Key file contains escape character at end of line\0" as *const u8
                                as *const gchar,
                        ),
                    );
                    current_block = 10194951743515834651;
                    break;
                }
                _ => {
                    if !pieces.is_null()
                        && *p as ::core::ffi::c_int
                            == (*key_file).list_separator as ::core::ffi::c_int
                    {
                        *q = (*key_file).list_separator;
                    } else {
                        let fresh0 = q;
                        q = q.offset(1);
                        *fresh0 = '\\' as i32 as gchar;
                        *q = *p;
                        if (*error).is_null() {
                            let mut sequence: [gchar; 3] = [0; 3];
                            sequence[0 as ::core::ffi::c_int as usize] = '\\' as i32 as gchar;
                            sequence[1 as ::core::ffi::c_int as usize] = *p;
                            sequence[2 as ::core::ffi::c_int as usize] = '\0' as i32 as gchar;
                            g_set_error(
                                error,
                                safe_c2rust_g_key_file_error_quark(),
                                G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int
                                    as gint,
                                glib_gettext(
                                    b"Key file contains invalid escape sequence \xE2\x80\x9C%s\xE2\x80\x9D\0"
                                        as *const u8 as *const gchar,
                                ),
                                &raw mut sequence as *mut gchar,
                            );
                            current_block = 10194951743515834651;
                            break;
                        }
                    }
                }
            }
        } else {
            *q = *p;
            if !pieces.is_null()
                && *p as ::core::ffi::c_int == (*key_file).list_separator as ::core::ffi::c_int
            {
                tmp_pieces = g_slist_prepend(
                    tmp_pieces,
                    g_strndup(q0, q.offset_from(q0) as ::core::ffi::c_long as gsize) as gpointer,
                );
                q0 = q.offset(1 as ::core::ffi::c_int as isize);
            }
        }
        if *p as ::core::ffi::c_int == '\0' as i32 {
            current_block = 2873832966593178012;
            break;
        }
        q = q.offset(1);
        p = p.offset(1);
    }
    match current_block {
        10194951743515834651 => {
            g_free(string_value as gpointer);
            g_slist_free_full(
                tmp_pieces,
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            );
            return ::core::ptr::null_mut::<gchar>();
        }
        _ => {
            *q = '\0' as i32 as gchar;
            if !pieces.is_null() {
                if q0 < q {
                    tmp_pieces = g_slist_prepend(
                        tmp_pieces,
                        g_strndup(q0, q.offset_from(q0) as ::core::ffi::c_long as gsize)
                            as gpointer,
                    );
                }
                *pieces = g_slist_reverse(tmp_pieces);
            }
            return string_value;
        }
    };
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_string_as_value(
    mut key_file: *mut GKeyFile,
    mut string: *const gchar,
    mut escape_separator: gboolean,
) -> *mut gchar {
    let mut value: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut q: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *const gchar = ::core::ptr::null::<gchar>();
    let mut length: gsize = 0;
    let mut parsing_leading_space: gboolean = 0;
    length = strlen(string as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as gsize;
    value = ({
        let mut __n: gsize = (2 as gsize).wrapping_mul(length);
        let mut __s: gsize = ::core::mem::size_of::<gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut gchar;
    p = string;
    q = value;
    parsing_leading_space = TRUE as gboolean;
    while p < string
        .offset(length as isize)
        .offset(-(1 as ::core::ffi::c_int as isize))
    {
        let mut escaped_character: [gchar; 3] = [
            '\\' as i32 as gchar,
            0 as ::core::ffi::c_int as gchar,
            0 as ::core::ffi::c_int as gchar,
        ];
        match *p as ::core::ffi::c_int {
            32 => {
                if parsing_leading_space != 0 {
                    escaped_character[1 as ::core::ffi::c_int as usize] = 's' as i32 as gchar;
                    strcpy(
                        q as *mut ::core::ffi::c_char,
                        &raw mut escaped_character as *mut gchar,
                    );
                    q = q.offset(2 as ::core::ffi::c_int as isize);
                } else {
                    *q = *p;
                    q = q.offset(1);
                }
            }
            9 => {
                if parsing_leading_space != 0 {
                    escaped_character[1 as ::core::ffi::c_int as usize] = 't' as i32 as gchar;
                    strcpy(
                        q as *mut ::core::ffi::c_char,
                        &raw mut escaped_character as *mut gchar,
                    );
                    q = q.offset(2 as ::core::ffi::c_int as isize);
                } else {
                    *q = *p;
                    q = q.offset(1);
                }
            }
            10 => {
                escaped_character[1 as ::core::ffi::c_int as usize] = 'n' as i32 as gchar;
                strcpy(
                    q as *mut ::core::ffi::c_char,
                    &raw mut escaped_character as *mut gchar,
                );
                q = q.offset(2 as ::core::ffi::c_int as isize);
            }
            13 => {
                escaped_character[1 as ::core::ffi::c_int as usize] = 'r' as i32 as gchar;
                strcpy(
                    q as *mut ::core::ffi::c_char,
                    &raw mut escaped_character as *mut gchar,
                );
                q = q.offset(2 as ::core::ffi::c_int as isize);
            }
            92 => {
                escaped_character[1 as ::core::ffi::c_int as usize] = '\\' as i32 as gchar;
                strcpy(
                    q as *mut ::core::ffi::c_char,
                    &raw mut escaped_character as *mut gchar,
                );
                q = q.offset(2 as ::core::ffi::c_int as isize);
                parsing_leading_space = FALSE as gboolean;
            }
            _ => {
                if escape_separator != 0
                    && *p as ::core::ffi::c_int == (*key_file).list_separator as ::core::ffi::c_int
                {
                    escaped_character[1 as ::core::ffi::c_int as usize] =
                        (*key_file).list_separator;
                    strcpy(
                        q as *mut ::core::ffi::c_char,
                        &raw mut escaped_character as *mut gchar,
                    );
                    q = q.offset(2 as ::core::ffi::c_int as isize);
                    parsing_leading_space = TRUE as gboolean;
                } else {
                    *q = *p;
                    q = q.offset(1);
                    parsing_leading_space = FALSE as gboolean;
                }
            }
        }
        p = p.offset(1);
    }
    *q = '\0' as i32 as gchar;
    return value;
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_value_as_integer(
    mut key_file: *mut GKeyFile,
    mut value: *const gchar,
    mut error: *mut *mut GError,
) -> gint {
    let mut eof_int: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut long_value: glong = 0;
    let mut int_value: gint = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    *__errno_location() = 0 as ::core::ffi::c_int;
    long_value = strtol(
        value as *const ::core::ffi::c_char,
        &raw mut eof_int,
        10 as ::core::ffi::c_int,
    ) as glong;
    errsv = *__errno_location();
    if *value as ::core::ffi::c_int == '\0' as i32
        || *eof_int as ::core::ffi::c_int != '\0' as i32
            && !(*safe_c2rust_g_ascii_table.offset(*eof_int as guchar as isize)
                as ::core::ffi::c_int
                & G_ASCII_SPACE as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
    {
        let mut value_utf8: *mut gchar =
            g_utf8_make_valid(value, -(1 as ::core::ffi::c_int) as gssize);
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Value \xE2\x80\x9C%s\xE2\x80\x9D cannot be interpreted as a number.\0"
                    as *const u8 as *const gchar,
            ),
            value_utf8,
        );
        g_free(value_utf8 as gpointer);
        return 0 as gint;
    }
    int_value = long_value as gint;
    if int_value as glong != long_value || errsv == ERANGE {
        let mut value_utf8_0: *mut gchar =
            g_utf8_make_valid(value, -(1 as ::core::ffi::c_int) as gssize);
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Integer value \xE2\x80\x9C%s\xE2\x80\x9D out of range\0" as *const u8
                    as *const gchar,
            ),
            value_utf8_0,
        );
        g_free(value_utf8_0 as gpointer);
        return 0 as gint;
    }
    return int_value;
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_integer_as_value(
    mut key_file: *mut GKeyFile,
    mut value: gint,
) -> *mut gchar {
    return g_strdup_printf(b"%d\0" as *const u8 as *const gchar, value);
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_value_as_double(
    mut key_file: *mut GKeyFile,
    mut value: *const gchar,
    mut error: *mut *mut GError,
) -> gdouble {
    let mut end_of_valid_d: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut double_value: gdouble = 0 as ::core::ffi::c_int as gdouble;
    double_value = g_ascii_strtod(value, &raw mut end_of_valid_d);
    if *end_of_valid_d as ::core::ffi::c_int != '\0' as i32 || end_of_valid_d == value as *mut gchar
    {
        let mut value_utf8: *mut gchar =
            g_utf8_make_valid(value, -(1 as ::core::ffi::c_int) as gssize);
        g_set_error(
            error,
            safe_c2rust_g_key_file_error_quark(),
            G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Value \xE2\x80\x9C%s\xE2\x80\x9D cannot be interpreted as a float number.\0"
                    as *const u8 as *const gchar,
            ),
            value_utf8,
        );
        g_free(value_utf8 as gpointer);
        double_value = 0 as ::core::ffi::c_int as gdouble;
    }
    return double_value;
}
unsafe extern "C" fn safe_c2rust_strcmp_sized(
    mut s1: *const gchar,
    mut len1: size_t,
    mut s2: *const gchar,
) -> gint {
    let mut len2: size_t = strlen(s2 as *const ::core::ffi::c_char);
    return strncmp(
        s1 as *const ::core::ffi::c_char,
        s2 as *const ::core::ffi::c_char,
        if len1 > len2 { len1 } else { len2 },
    ) as gint;
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_value_as_boolean(
    mut key_file: *mut GKeyFile,
    mut value: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut value_utf8: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut i: gint = 0;
    let mut length: gint = 0 as gint;
    i = 0 as ::core::ffi::c_int as gint;
    while *value.offset(i as isize) != 0 {
        if !(*safe_c2rust_g_ascii_table.offset(*value.offset(i as isize) as guchar as isize)
            as ::core::ffi::c_int
            & G_ASCII_SPACE as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int)
        {
            length = (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint;
        }
        i += 1;
    }
    if safe_c2rust_strcmp_sized(
        value,
        length as size_t,
        b"true\0" as *const u8 as *const gchar,
    ) == 0 as ::core::ffi::c_int
        || safe_c2rust_strcmp_sized(value, length as size_t, b"1\0" as *const u8 as *const gchar)
            == 0 as ::core::ffi::c_int
    {
        return TRUE;
    } else if safe_c2rust_strcmp_sized(
        value,
        length as size_t,
        b"false\0" as *const u8 as *const gchar,
    ) == 0 as ::core::ffi::c_int
        || safe_c2rust_strcmp_sized(value, length as size_t, b"0\0" as *const u8 as *const gchar)
            == 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    value_utf8 = g_utf8_make_valid(value, -(1 as ::core::ffi::c_int) as gssize);
    g_set_error(
        error,
        safe_c2rust_g_key_file_error_quark(),
        G_KEY_FILE_ERROR_INVALID_VALUE as ::core::ffi::c_int as gint,
        glib_gettext(
            b"Value \xE2\x80\x9C%s\xE2\x80\x9D cannot be interpreted as a boolean.\0" as *const u8
                as *const gchar,
        ),
        value_utf8,
    );
    g_free(value_utf8 as gpointer);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_boolean_as_value(
    mut key_file: *mut GKeyFile,
    mut value: gboolean,
) -> *const gchar {
    if value != 0 {
        return b"true\0" as *const u8 as *const gchar;
    } else {
        return b"false\0" as *const u8 as *const gchar;
    };
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_value_as_comment(
    mut key_file: *mut GKeyFile,
    mut value: *const gchar,
    mut is_final_line: gboolean,
) -> *mut gchar {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut lines: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gsize = 0;
    string = g_string_sized_new(512 as gsize);
    lines = g_strsplit(value, b"\n\0" as *const u8 as *const gchar, 0 as gint);
    i = 0 as gsize;
    while !(*lines.offset(i as isize)).is_null() {
        let mut line: *const gchar = *lines.offset(i as isize);
        if i != 0 as gsize {
            safe_c2rust_g_string_append_c_inline(string, '\n' as i32 as gchar);
        }
        if *line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32 {
            line = line.offset(1);
        }
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char = line as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_149: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_149 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_149 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_149
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
                line as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        i = i.wrapping_add(1);
    }
    g_strfreev(lines);
    if is_final_line == 0 {
        safe_c2rust_g_string_append_c_inline(string, '\n' as i32 as gchar);
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(string, 0 as gboolean)
        } else {
            g_string_free_and_steal(string)
        }
    } else {
        g_string_free(string, 0 as gboolean)
    };
}
unsafe extern "C" fn safe_c2rust_g_key_file_parse_comment_as_value(
    mut key_file: *mut GKeyFile,
    mut comment: *const gchar,
) -> *mut gchar {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut lines: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
    let mut i: gsize = 0;
    string = g_string_sized_new(512 as gsize);
    lines = g_strsplit(comment, b"\n\0" as *const u8 as *const gchar, 0 as gint);
    i = 0 as gsize;
    while !(*lines.offset(i as isize)).is_null() {
        g_string_append_printf(
            string,
            b"#%s%s\0" as *const u8 as *const gchar,
            *lines.offset(i as isize),
            if (*lines.offset(i.wrapping_add(1 as gsize) as isize)).is_null() {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\n\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        i = i.wrapping_add(1);
    }
    g_strfreev(lines);
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(string, 0 as gboolean)
        } else {
            g_string_free_and_steal(string)
        }
    } else {
        g_string_free(string, 0 as gboolean)
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_key_file_save_to_file(
    mut key_file: *mut GKeyFile,
    mut filename: *const gchar,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut success: gboolean = 0;
    let mut length: gsize = 0;
    if ({
        let mut _g_boolean_var_150: ::core::ffi::c_int = 0;
        if !key_file.is_null() {
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
            b"key_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_151: ::core::ffi::c_int = 0;
        if !filename.is_null() {
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
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_152: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    contents = safe_c2rust_g_key_file_to_data(
        key_file,
        &raw mut length,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if ({
        let mut _g_boolean_var_153: ::core::ffi::c_int = 0;
        if !contents.is_null() {
            _g_boolean_var_153 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_153 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_153
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gkeyfile.c\0" as *const u8 as *const ::core::ffi::c_char,
            4811 as ::core::ffi::c_int,
            G_STRFUNC,
            b"contents != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    success = g_file_set_contents(filename, contents, length as gssize, error);
    g_free(contents as gpointer);
    return success;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_key_file_ref\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
